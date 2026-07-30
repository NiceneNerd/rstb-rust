//! This module handles calculating RSTB values. It can be used for exact
//! calculations on many filetypes or estimations on certain others.
//! The most important distinction is between the `calc` and `estimate`
//! functions. *The `calc` functions only attempt infallible calculations on
//! types where this is supported, and otherwise always return `None`.* By
//! contrast, *the `estimate` functions will perform an infallible calculation
//! on supported types or attempt estimates on supported BFRES and AAMP files*.
//!
//! A full list of supported infallible calculation types:
//! - `sarc` (`pack`, `bactorpack`, `bmodelsh`, `beventpack`, `stera`, `stats`)
//! - `batpl` (`bnfprl`)
//! - `bplacement`
//! - `hks` (`lua`)
//! - `bactcapt`
//! - `bitemico`
//! - `jpg`
//! - `bmaptex`
//! - `bstftex` (`bmapopen`, `breviewtex`)
//! - `bgdata`
//! - `bgsvdata`
//! - `baischedule`
//! - `bdmgparam`
//! - `brgconfig`
//! - `bawareness`
//! - `blod`
//! - `bumii`
//! - `byaml`
//! - `hkrb`
//! - `hkrg`
//! - `esetlist`
//! - `bdemo`
//! - `bfevfl`
//! - `bfevtm`
//!
//! A full list of supported estimation types:
//! - `baiprog`
//! - `bas`
//! - `bphysics`
//! - `baslist`
//! - `bdrop`
//! - `bgparamlist`
//! - `brecipe`
//! - `bshop`
//! - `bxml`
//! - `bfres`
//!
//! **To repeat:** the `calc` functions can only do infallible calculations,
//! and otherwise return `None`. The `estimate` functions will first try the
//! infallible calculation, then try and estimate, and return `None` only if
//! neither works.
//!
//! *A final note*: As mentioned elsewhere, the `endian` parameter should be
//! used as a shorthand for Wii U/Switch more generally. Pass `Endian::Big` for
//! Wii U files, even if they are actually in little endian, and pass
//! `Endian:Little` for Switch files, even if they are actually in big endian.

#[cfg(feature = "complex")]
mod cpp_memsizes;
mod info;

use std::path::Path;

#[cfg(feature = "complex")]
use cpp_memsizes::*;
use info::{get_factory_info, ParseSize};

use crate::{Endian, Result};

#[inline]
fn round_32(size: usize) -> u32 {
    ((size as isize + 31) & -32) as u32
}

#[inline]
fn round_64(size: usize) -> u32 {
    ((size as isize + 63) & -64) as u32
}

#[inline]
fn align_to(size: u32, alignment: isize) -> u32 {
    ((size as isize + (alignment-1)) & -alignment) as u32
}

/// Infallibly calculate an RSTB value from a file on disk, returning `None` if
/// the type is not supported.
pub fn calc_from_file<P: AsRef<Path>>(file: P, endian: Endian) -> Result<Option<u32>> {
    Ok(calc_from_slice_and_name(
        std::fs::read(file.as_ref())?,
        file.as_ref()
            .file_name()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Path not a file"))?
            .to_str()
            .unwrap_or_default(),
        endian,
    ))
}

/// Infallibly calculate an RSTB value from a byte slice and filename,
/// returning `None` if the type is not supported.
pub fn calc_from_slice_and_name<B: AsRef<[u8]>>(
    bytes: B,
    name: &str,
    endian: Endian,
) -> Option<u32> {
    let bytes = bytes.as_ref();
    if bytes.len() < 8 {
        None
    } else {
        calc_or_estimate_from_bytes_and_name(bytes, name, endian, false)
    }
}

/// Infallibly calculate an RSTB value from an uncompressed file size and
/// filename, returning `None` if the type is not supported.
pub fn calc_from_size_and_name(filesize: usize, name: &str, endian: Endian) -> Option<u32> {
    calc_or_estimate_from_size_and_name(filesize, name, endian, false)
}

fn calc_or_estimate_from_size_and_name(
    filesize: usize,
    name: &str,
    endian: Endian,
    estimate: bool,
) -> Option<u32> {
    if let Some(dot_pos) = name.find('.') {
        let rounded = round_32(filesize);
        let raw_ext = &name[dot_pos + 1..];
        let ext = match raw_ext {
            "sarc" => "sarc",
            _ => {
                if let Some(ext) = raw_ext.strip_prefix('s') {
                    ext
                } else {
                    raw_ext
                }
            }
        };
        let (size, _, parse_size) = get_factory_info(ext, endian);
        match parse_size {
            ParseSize::Simple(parse_size) => {
                Some(match endian {
                    Endian::Big => {
                        rounded
                            + 0xe4
                            + size
                            + parse_size
                            + match ext {
                                "beventpack" => 0xe0,
                                "bfevfl" => 0x58,
                                "hkrb" => 40,
                                "bdmgparam" => (rounded as f32 * 0.666) as u32,
                                _ => 0,
                            }
                    }
                    Endian::Little => {
                        rounded
                            + 0x168
                            + size
                            + parse_size
                            + match ext {
                                "bdmgparam" => (rounded as f32 * 0.666) as u32,
                                _ => 0,
                            }
                    }
                })
            }
            ParseSize::Complex => {
                if estimate {
                    match ext {
                        "baniminfo" => {
                            Some(
                                ((rounded as f32 * (if filesize > 36864 { 1.5 } else { 4.0 }))
                                    as u32
                                    + 0xe4
                                    + 0x24c)
                                    * match endian {
                                        Endian::Big => 1,
                                        Endian::Little => 2,
                                    },
                            )
                        }
                        "bfres" => Some(estimate_bfres(filesize, endian)),
                        _ => estimate_aamp(filesize, name, endian),
                    }
                } else {
                    None
                }
            }
        }
    } else {
        None
    }
}

/// Infallibly calculate *or* estimate an RSTB value from a file on disk,
/// returning `None` if the type is not supported.
pub fn estimate_from_file<P: AsRef<Path>>(file: P, endian: Endian) -> Result<Option<u32>> {
    Ok(estimate_from_bytes_and_name(
        &std::fs::read(file.as_ref())?,
        file.as_ref()
            .file_name()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Path not a file"))?
            .to_str()
            .unwrap_or_default(),
        endian,
    ))
}

/// Infallibly calculate *or* estimate an RSTB value from an uncompressed file size and
/// filename, returning `None` if the type is not supported.
pub fn estimate_from_size_and_name(filesize: usize, name: &str, endian: Endian) -> Option<u32> {
    calc_or_estimate_from_size_and_name(filesize, name, endian, true)
}

/// Infallibly calculate *or* estimate an RSTB value from a byte slice and
/// filename, returning `None` if the type is not supported.
pub fn estimate_from_slice_and_name<B: AsRef<[u8]>>(
    bytes: B,
    name: &str,
    endian: Endian,
) -> Option<u32> {
    let bytes = bytes.as_ref();
    if bytes.len() < 8 {
        None
    } else {
        estimate_from_bytes_and_name(bytes, name, endian)
    }
}

/// Infallibly calculate an RSTB value from an uncompressed file size and
/// filename, returning `None` if the type is not supported.
pub fn estimate_from_bytes_and_name(bytes: &[u8], name: &str, endian: Endian) -> Option<u32> {
    calc_or_estimate_from_bytes_and_name(bytes, name, endian, true)
}

fn calc_or_estimate_from_bytes_and_name(
    bytes: &[u8],
    name: &str,
    endian: Endian,
    estimate: bool,
) -> Option<u32> {
    if let Some(dot_pos) = name.find('.') {
        let filesize = match &bytes[0..4] {
            b"Yaz0" => u32::from_be_bytes(bytes[4..8].try_into().ok()?) as usize,
            _ => bytes.len(),
        };
        let rounded = match endian {
            Endian::Big => round_64(filesize),
            Endian::Little => round_32(filesize),
        };
        let raw_ext = &name[dot_pos + 1..];
        let ext = match raw_ext {
            "sarc" => "sarc",
            "Tex.sbfres" => "Tex.bfres",
            "Tex1.sbfres" => "Tex1.bfres",
            "Tex2.sbfres" => "Tex2.bfres",
            "product.byml" => "byml",
            "product.sbyml" => "byml",
            _ => {
                if let Some(ext) = raw_ext.strip_prefix('s') {
                    ext
                } else {
                    raw_ext
                }
            }
        };
        let (size, alignment, parse_size) = get_factory_info(ext, endian);
        match parse_size {
            ParseSize::Simple(parse_size) => {
                Some(match endian {
                    Endian::Big => {
                        match ext {
                            "esetlist" => round_32(filesize) + 
                                align_to(0xe4 + size + parse_size, alignment),
                            _ => rounded
                                + align_to(0xe4 + size + parse_size, alignment)
                                + match ext {
                                    "beventpack" => 0xe0,
                                    _ => 0,
                                }
                        }
                    }
                    Endian::Little => {
                        rounded + align_to(0x168 + size + parse_size, alignment)
                    }
                })
            }
            ParseSize::Complex => {
                if estimate {
                    match ext {
                        #[cfg(feature = "complex")]
                        "baiprog" => Some(
                            rounded + align_to(
                                baiprog::parse_size(bytes, endian)?,
                                alignment
                            )
                        ),
                        #[cfg(feature = "complex")]
                        "baniminfo" => Some(
                            rounded + align_to(
                                baniminfo::parse_size(bytes, endian)?,
                                alignment
                            )
                        ),
                        #[cfg(not(feature = "complex"))]
                        "baniminfo" => {
                            Some(
                                ((rounded as f32 * (if filesize > 36864 { 1.5 } else { 4.0 }))
                                    as u32
                                    + 0xe4
                                    + 0x24c)
                                    * match endian {
                                        Endian::Big => 1,
                                        Endian::Little => 2,
                                    },
                            )
                        }
                        #[cfg(feature = "complex")]
                        "baslist" => Some(
                            rounded + align_to(
                                baslist::parse_size(bytes, endian)?,
                                alignment
                            )
                        ),
                        #[cfg(feature = "complex")]
                        "bchemical" => Some(
                            rounded + align_to(
                                bchemical::parse_size(bytes, endian)?,
                                alignment
                            )
                        ),
                        #[cfg(feature = "complex")]
                        "bdrop" => Some(
                            rounded + align_to(
                                bdrop::parse_size(bytes, endian)?,
                                alignment
                            )
                        ),
                        "bfres" => Some(estimate_bfres(filesize, endian)),
                        #[cfg(feature = "complex")]
                        "bgparamlist" => Some(
                            rounded + align_to(
                                bgparamlist::parse_size(bytes, endian)?,
                                alignment
                            )
                        ),
                        #[cfg(feature = "complex")]
                        "blifecondition" => Some(
                            rounded + align_to(
                                blifecondition::parse_size(bytes, endian)?,
                                alignment
                            )
                        ),
                        #[cfg(feature = "complex")]
                        "bmodellist" => Some(
                            rounded + align_to(
                                bmodellist::parse_size(bytes, endian)?,
                                alignment
                            )
                        ),
                        #[cfg(feature = "complex")]
                        "bphysics" => Some(
                            rounded + align_to(
                                bphysics::parse_size(bytes, endian)?,
                                alignment
                            )
                        ),
                        #[cfg(feature = "complex")]
                        "bphyssb" => Some(
                            rounded + align_to(
                                bphyssb::parse_size(bytes, endian)?,
                                alignment
                            )
                        ),
                        #[cfg(feature = "complex")]
                        "brecipe" => Some(
                            rounded + align_to(
                                brecipe::parse_size(bytes, endian)?,
                                alignment
                            )
                        ),
                        #[cfg(feature = "complex")]
                        "brgconfiglist" => Some(
                            rounded + align_to(
                                brgconfiglist::parse_size(bytes, endian)?,
                                alignment
                            )
                        ),
                        #[cfg(feature = "complex")]
                        "bshop" => Some(
                            rounded + align_to(
                                bshop::parse_size(bytes, endian)?,
                                alignment
                            )
                        ),
                        #[cfg(feature = "complex")]
                        "bxml" => Some(
                            rounded + align_to(
                                bxml::parse_size(bytes, endian)?,
                                alignment
                            )
                        ),
                        "hknm2" => {
                            Some(
                                rounded
                                    + match endian {
                                        Endian::Big => 0x19c,
                                        Endian::Little => 0x290,
                                    },
                            )
                        }
                        "hksc" => {
                            Some(
                                rounded
                                    + match endian {
                                        Endian::Big => 0x74cc,
                                        Endian::Little => 0x9c00,
                                    },
                            )
                        }
                        _ => estimate_aamp(filesize, name, endian),
                    }
                } else {
                    None
                }
            }
        }
    } else {
        None
    }
}

fn estimate_aamp(filesize: usize, name: &str, endian: Endian) -> Option<u32> {
    let mut size = (filesize as f32) * 1.05;
    let ext = &name[name.rfind('.')? + 1..];
    if ext == "bas" {
        size *= 1.05;
    };
    if ext == "bassetting" {
        size = (round_32(size as usize)
            + match endian {
                Endian::Big => 0xe4 + 0x1d8,
                Endian::Little => 0x168 + 0x260,
            }) as f32
            + (size * 2.75)
    } else if ext == "bdmgparam" {
        size = (((-0.0018 * size) + 6.6273) * size) + 500.0
    } else if ext == "bphysics" {
        size = (round_32(size as usize) + 0x4E + 0x324) as f32
            * f32::max(4.0 * (size / 1388.0).floor(), 3.0)
    } else {
        size *= match ext {
            "baiprog" => {
                match size as usize {
                    (0..380) => 7.0,
                    (380..400) => 6.0,
                    (400..450) => 5.5,
                    (450..600) => 5.0,
                    (600..1_000) => 4.0,
                    (1_000..1_750) => 3.5,
                    _ => 3.0,
                }
            }
            "bas" => {
                match size as usize {
                    (0..100) => 20.0,
                    (100..200) => 12.5,
                    (200..300) => 10.0,
                    (300..600) => 8.0,
                    (600..1_500) => 6.0,
                    (1_500..2_000) => 5.5,
                    (2_000..15_000) => 5.0,
                    _ => 4.5,
                }
            }
            "baslist" => {
                match size as usize {
                    (0..100) => 15.0,
                    (100..200) => 10.0,
                    (200..300) => 8.0,
                    (300..500) => 6.0,
                    (500..800) => 5.0,
                    (800..4_000) => 4.0,
                    _ => 3.5,
                }
            }
            "bdrop" => {
                match size as usize {
                    (0..200) => 8.5,
                    (200..250) => 7.0,
                    (250..350) => 6.0,
                    (350..450) => 5.25,
                    (450..850) => 4.5,
                    _ => 4.0,
                }
            }
            "bgparamlist" => {
                match size as usize {
                    (0..100) => 20.0,
                    (100..150) => 12.0,
                    (150..250) => 10.0,
                    (250..350) => 8.0,
                    (350..450) => 7.0,
                    _ => 6.0,
                }
            }
            "brecipe" => {
                match size as usize {
                    (0..100) => 12.5,
                    (100..160) => 8.5,
                    (160..200) => 7.5,
                    (200..215) => 7.0,
                    _ => 6.5,
                }
            }
            "bshop" => {
                match size as usize {
                    (0..200) => 7.25,
                    (200..400) => 6.0,
                    (400..500) => 5.0,
                    _ => 4.05,
                }
            }
            "bxml" => {
                match size as usize {
                    (0..350) => 6.0,
                    (350..450) => 5.0,
                    (450..550) => 4.5,
                    (550..650) => 4.0,
                    (650..800) => 3.5,
                    _ => 3.0,
                }
            }
            _ => return None,
        };
    }
    Some(match endian {
        Endian::Big => size,
        Endian::Little => size * 1.5,
    } as u32)
}

fn estimate_bfres(filesize: usize, endian: Endian) -> u32 {
    (filesize as f32
        * match endian {
            Endian::Big => {
                match filesize {
                    (0..500) => 7.0,
                    (500..750) => 5.0,
                    (750..1_250) => 4.0,
                    (1_250..2_000) => 3.5,
                    (2_000..400_000) => 2.25,
                    (400_000..600_000) => 2.1,
                    (600_000..1_000_000) => 1.95,
                    (1_000_000..1_500_000) => 1.85,
                    (1_500_000..3_000_000) => 1.66,
                    _ => 1.45,
                }
            }
            Endian::Little => {
                match filesize {
                    (0..1_250) => 9.5,
                    (1_250..2_500) => 6.0,
                    (2_500..50_000) => 4.25,
                    (50_000..100_000) => 3.66,
                    (100_000..800_000) => 3.5,
                    (800_000..2_000_000) => 3.15,
                    (2_000_000..3_000_000) => 2.5,
                    (3_000_000..4_000_000) => 1.667,
                    _ => 1.6,
                }
            }
        }) as u32
}

#[cfg(test)]
mod tests {
    use std::{fs::read, path::PathBuf};

    use all_asserts::assert_ge;

    use crate::Endian;

    fn get_update_path() -> PathBuf {
        use ryml::Tree;
        use dirs2::config_dir;

        let settings_path = config_dir()
            .unwrap()
            .join("ukmm")
            .join("settings.yml");
        let settings = Tree::parse(
                std::fs::read_to_string(settings_path).unwrap()
            ).unwrap();
        let source_node = settings.root_ref()
            .unwrap()
            .get("wiiu_config")
            .unwrap()
            .get("dump")
            .unwrap()
            .get("source")
            .unwrap();

        assert_eq!(source_node.get("type").unwrap().val().unwrap(), "Unpacked");

        PathBuf::from(source_node.get("update_dir").unwrap().val().unwrap())
    }

    fn get_update_path_nx() -> PathBuf {
        use ryml::Tree;
        use dirs2::config_dir;

        let settings_path = config_dir()
            .unwrap()
            .join("ukmm")
            .join("settings.yml");
        let settings = Tree::parse(
                std::fs::read_to_string(settings_path).unwrap()
            ).unwrap();
        let source_node = settings.root_ref()
            .unwrap()
            .get("switch_config")
            .unwrap()
            .get("dump")
            .unwrap()
            .get("source")
            .unwrap();

        assert_eq!(source_node.get("type").unwrap().val().unwrap(), "Unpacked");

        PathBuf::from(source_node.get("content_dir").unwrap().val().unwrap())
    }

    #[test]
    fn calc_sizes() {
        assert_eq!(
            super::calc_from_file("test/A-1_Dynamic.smubin", Endian::Big).unwrap(),
            Some(48772)
        );
        assert_eq!(
            super::calc_from_file("test/AirOcta_Tag.sbactorpack", Endian::Little).unwrap(),
            Some(6192)
        );
        let buffer: Vec<u8> = read("test/Enemy_Bokoblin_Gold.bdmgparam").unwrap();
        assert_ge!(
            super::calc_from_slice_and_name(buffer, "Enemy_Bokoblin_Gold.bdmgparam", Endian::Big),
            Some(5396)
        );
        let buffer: Vec<u8> = read("test/Obj_TreeWhiteBirch_A_01.hkrb").unwrap();
        assert_eq!(
            super::calc_from_slice_and_name(buffer, "Obj_TreeWhiteBirch_A_01.hkrb", Endian::Big),
            Some(3560)
        );
        let buffer: Vec<u8> = read("test/savedataformat.ssarc").unwrap();
        assert_eq!(
            super::calc_from_slice_and_name(buffer, "savedataformat.ssarc", Endian::Big),
            Some(2_801_216)
        );
    }

    #[test]
    fn estimate_sizes() {
        assert_ge!(
            super::estimate_from_size_and_name(42496, "Model/Animal_Bass.Tex1.sbfres", Endian::Big),
            Some(42756) // Functional value lower than stock
        );
        assert_ge!(
            super::estimate_from_size_and_name(
                7735632,
                "Model/DgnMrgPrt_Dungeon061.bfres",
                Endian::Little,
            ),
            Some(8658192)
        );
        assert_ge!(
            super::estimate_from_size_and_name(
                7735632,
                "Model/DgnMrgPrt_Dungeon061.bfres",
                Endian::Little,
            ),
            Some(8658192)
        );
        assert_ge!(
            super::estimate_from_size_and_name(
                1408,
                "Actor/GeneralParamList/Weapon_Bow_071.bgparamlist",
                Endian::Little,
            ),
            Some(8272)
        );
        assert_ge!(
            super::estimate_from_size_and_name(
                3540,
                "Actor/AIProgram/NpcGerudoQueenBattle.baiprog",
                Endian::Big,
            ),
            Some(9444)
        );
        assert_ge!(
            super::estimate_from_size_and_name(27960, "Actor/ASSetting.bassetting", Endian::Big),
            Some(101452)
        );
        assert_ge!(
            super::estimate_from_size_and_name(27960, "Actor/ASSetting.bassetting", Endian::Little),
            Some(165864)
        );
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn estimate_sizes_complex() {
        assert_eq!(
            super::estimate_from_slice_and_name(
                std::fs::read("test/Animal_Bass.Tex1.sbfres").unwrap(),
                "Model/Animal_Bass.Tex1.sbfres",
                Endian::Big
            ),
            Some(42756)
        );
        assert_ge!(
            super::estimate_from_slice_and_name(
                std::fs::read("test/DgnMrgPrt_Dungeon061.sbfres").unwrap(),
                "Model/DgnMrgPrt_Dungeon061.sbfres",
                Endian::Big,
            ),
            Some(8671688)
        );
        assert_ge!(
            super::estimate_from_slice_and_name(
                std::fs::read("test/NpcGerudoQueenBattle.baiprog").unwrap(),
                "Actor/AIProgram/NpcGerudoQueenBattle.baiprog",
                Endian::Big,
            ),
            Some(9444)
        );
        assert_ge!(
            super::estimate_from_slice_and_name(
                std::fs::read("test/Player_Link.bgparamlist").unwrap(),
                "Actor/GeneralParamList/Player_Link.bgparamlist",
                Endian::Big,
            ),
            Some(6744)
        );
        assert_ge!(
            super::estimate_from_slice_and_name(
                &std::fs::read("test/Dummy.bgparamlist").unwrap(),
                "Actor/GeneralParamList/Dummy.bgparamlist",
                Endian::Big,
            ),
            Some(42824)
        );
        assert_ge!(
            super::estimate_from_slice_and_name(
                std::fs::read("test/Armor_001_Upper.bmodellist").unwrap(),
                "Actor/ModelList/Armor_001_Upper.bmodellist",
                Endian::Big,
            ),
            Some(2636)
        );
        assert_ge!(
            super::estimate_from_slice_and_name(
                std::fs::read("test/Armor_002_Upper.brecipe").unwrap(),
                "Actor/ModelList/Armor_002_Upper.brecipe",
                Endian::Big,
            ),
            Some(1276)
        );
        assert_ge!(
            super::estimate_from_slice_and_name(
                std::fs::read("test/Assassin_Senior.bdrop").unwrap(),
                "Actor/DropTable/Assassin_Senior.bdrop",
                Endian::Big,
            ),
            Some(1132)
        );
        assert_ge!(
            super::estimate_from_slice_and_name(
                std::fs::read("test/Assassin_Senior.bxml").unwrap(),
                "Actor/ActorLink/Assassin_Senior.bxml",
                Endian::Big,
            ),
            Some(2116)
        );
        assert_ge!(
            super::estimate_from_slice_and_name(
                std::fs::read("test/Npc_TripMaster_08.bshop").unwrap(),
                "Actor/ShopData/Npc_TripMaster_08.bshop",
                Endian::Big,
            ),
            Some(2588)
        );
        assert_ge!(
            super::estimate_from_slice_and_name(
                std::fs::read("test/Player_Link.bphysics").unwrap(),
                "Actor/Physics/Player_Link.bphysics",
                Endian::Big,
            ),
            Some(38940)
        );
    }

    #[cfg(feature = "complex_testing")]
    fn test_all_of_type_assert(link: &str, folder: &str, ext: &str) {
        use std::collections::HashSet;
        use roead::{aamp::ParameterIO, sarc};

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let update_path = get_update_path();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Actor")
                    .join("Pack")
                    .join("*.sbactorpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let bxml = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                            .unwrap(),
                    )
                    .unwrap();
                    let user = bxml
                        .param_root
                        .objects
                        .get("LinkTarget")
                        .unwrap()
                        .get(link)
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/{}/{}.{}", folder, user, ext);
                    if param_name.contains("Dummy") | result.contains(&param_name) {
                        continue;
                    }
                    if let Some(o_file) = sarc.get_data(&param_name) {
                        if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                o_file,
                                &param_name,
                                Endian::Big,
                            )
                            .unwrap();
                            assert_eq!(calc_size, rstb_entry);
                            result.insert(param_name);
                        } else {
                            println!("{} not in RSTB???", &param_name);
                            continue;
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
        let titlebg_path = update_path
            .join("Pack")
            .join("TitleBG.pack");
        let titlebg = sarc::Sarc::new(std::fs::read(&titlebg_path).unwrap()).unwrap();
        for file in titlebg.files() {
            if file.name.unwrap_or("").starts_with("Actor/Pack") {
                let actorname_as_pathbuf = PathBuf::from(file.name.unwrap());
                let actorname = actorname_as_pathbuf
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap();
                let sarc = sarc::Sarc::new(file.data).unwrap();
                let bxml = ParameterIO::from_binary(
                    sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                        .unwrap(),
                )
                .unwrap();
                let user = bxml
                    .param_root
                    .objects
                    .get("LinkTarget")
                    .unwrap()
                    .get(link)
                    .unwrap()
                    .as_str()
                    .unwrap();
                let param_name = format!("Actor/{}/{}.{}", folder, user, ext);
                if param_name.contains("Dummy") | result.contains(&param_name) {
                    continue;
                }
                if let Some(o_file) = sarc.get_data(&param_name) {
                    if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                        let calc_size = super::estimate_from_bytes_and_name(
                            o_file,
                            &param_name,
                            Endian::Big,
                        )
                        .unwrap();
                        assert_eq!(calc_size, rstb_entry);
                        result.insert(param_name);
                    } else {
                        println!("{} not in RSTB???", &param_name);
                        continue;
                    }
                }
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    fn test_all_of_type_print(link: &str, folder: &str, ext: &str) {
        use std::collections::HashSet;
        use roead::{aamp::ParameterIO, sarc};

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();
        let mut overshot: i32 = -0x300000;
        let mut undershot: i32 = 0x300000;

        let update_path = get_update_path();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Actor")
                    .join("Pack")
                    .join("*.sbactorpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let bxml = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                            .unwrap(),
                    )
                    .unwrap();
                    let user = bxml
                        .param_root
                        .objects
                        .get("LinkTarget")
                        .unwrap()
                        .get(link)
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/{}/{}.{}", folder, user, ext);
                    if param_name.contains("Dummy") | result.contains(&param_name) {
                        continue;
                    }
                    if let Some(o_file) = sarc.get_data(&param_name) {
                        if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                o_file,
                                &param_name,
                                Endian::Big,
                            )
                            .unwrap();
                            let current = calc_size as i32 - rstb_entry as i32;
                            if current > 0 {
                                println!("{}//{}: {}", actorname, param_name, current);
                            }
                            if overshot < current {
                                overshot = current;
                            }
                            if undershot > current {
                                undershot = current;
                            }
                            assert_ge!(calc_size, rstb_entry);
                            result.insert(param_name);
                        } else {
                            println!("{} not in RSTB???", &param_name);
                            continue;
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
        let titlebg_path = update_path
            .join("Pack")
            .join("TitleBG.pack");
        let titlebg = sarc::Sarc::new(std::fs::read(&titlebg_path).unwrap()).unwrap();
        for file in titlebg.files() {
            if file.name.unwrap_or("").starts_with("Actor/Pack") {
                let actorname_as_pathbuf = PathBuf::from(file.name.unwrap());
                let actorname = actorname_as_pathbuf
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap();
                let sarc = sarc::Sarc::new(file.data).unwrap();
                let bxml = ParameterIO::from_binary(
                    sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                        .unwrap(),
                )
                .unwrap();
                let user = bxml
                    .param_root
                    .objects
                    .get("LinkTarget")
                    .unwrap()
                    .get(link)
                    .unwrap()
                    .as_str()
                    .unwrap();
                let param_name = format!("Actor/{}/{}.{}", folder, user, ext);
                if param_name.contains("Dummy") | result.contains(&param_name) {
                    continue;
                }
                if let Some(o_file) = sarc.get_data(&param_name) {
                    if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                        let calc_size = super::estimate_from_bytes_and_name(
                            o_file,
                            &param_name,
                            Endian::Big,
                        )
                        .unwrap();
                        let current = calc_size as i32 - rstb_entry as i32;
                        if current > 0 {
                            println!("{}//{}: {}", actorname, param_name, current);
                        }
                        if overshot < current {
                            overshot = current;
                        }
                        if undershot > current {
                            undershot = current;
                        }
                        assert_ge!(calc_size, rstb_entry);
                        result.insert(param_name);
                    } else {
                        println!("{} not in RSTB???", &param_name);
                        continue;
                    }
                }
            }
        }
        println!("Range (max amount of memory wasted with the overhead): {}", overshot - undershot);
        println!("Biggest underguess (overhead must be increased by this much): {}", -undershot);
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_baiprog() {
        test_all_of_type_print("AIProgramUser", "AIProgram", "baiprog");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_baischedule() {
        test_all_of_type_assert("AIScheduleUser", "AISchedule", "baischedule");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_baniminfo() {
        test_all_of_type_print("AnimationInfo", "AnimationInfo", "baniminfo");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_baslist() {
        test_all_of_type_assert("ASUser", "ASList", "baslist");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bawareness() {
        test_all_of_type_assert("AwarenessUser", "Awareness", "bawareness");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bchemical() {
        test_all_of_type_assert("ChemicalUser", "Chemical", "bchemical");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bdemo() {
        use std::collections::HashSet;
        use roead::sarc;

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let update_path = get_update_path();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Event")
                    .join("*.sbeventpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    for file in sarc.files() {
                        let param_name = file.name.unwrap();
                        if !param_name.ends_with(".bdemo") | result.contains(param_name) {
                            continue;
                        }
                        if let Some(rstb_entry) = rstable.get(param_name) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                file.data,
                                param_name,
                                Endian::Big,
                            )
                            .unwrap();
                            assert_eq!(calc_size, rstb_entry);
                            result.insert(param_name.to_string());
                        } else {
                            println!("{} not in RSTB???", &param_name);
                            continue;
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bdmgparam() {
        test_all_of_type_assert("DamageParamUser", "DamageParam", "bdmgparam");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bdrop() {
        test_all_of_type_assert("DropTableUser", "DropTable", "bdrop");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bfevfl() {
        use std::collections::HashSet;
        use roead::sarc;

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();
        let mut overshot: i32 = -0x300000;
        let mut undershot: i32 = 0x300000;

        let update_path = get_update_path();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Event")
                    .join("*.sbeventpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    for file in sarc.files() {
                        let param_name = file.name.unwrap();
                        if !param_name.ends_with(".bfevfl") | result.contains(param_name) {
                            continue;
                        }
                        if let Some(rstb_entry) = rstable.get(param_name) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                file.data,
                                param_name,
                                Endian::Big,
                            )
                            .unwrap();
                            let current = calc_size as i32 - rstb_entry as i32;
                            if current > 0 {
                                println!("{}//{}: {}", actorname, param_name, current);
                            }
                            if overshot < current {
                                overshot = current;
                            }
                            if undershot > current {
                                undershot = current;
                            }
                            assert_ge!(calc_size, rstb_entry);
                            result.insert(param_name.to_string());
                        } else {
                            println!("{} not in RSTB???", &param_name);
                            continue;
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
        println!("Range (max amount of memory wasted with the overhead): {}", overshot - undershot);
        println!("Biggest underguess (overhead must be increased by this much): {}", -undershot);
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bfevtm() {
        use std::collections::HashSet;
        use roead::sarc;

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let update_path = get_update_path();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Event")
                    .join("*.sbeventpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    for file in sarc.files() {
                        let param_name = file.name.unwrap();
                        if !param_name.ends_with(".bfevtm") | result.contains(param_name) {
                            continue;
                        }
                        if let Some(rstb_entry) = rstable.get(param_name) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                file.data,
                                param_name,
                                Endian::Big,
                            )
                            .unwrap();
                            assert_eq!(calc_size, rstb_entry);
                            result.insert(param_name.to_string());
                        } else {
                            println!("{} not in RSTB???", &param_name);
                            continue;
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bgparamlist() {
        test_all_of_type_print("GParamUser", "GeneralParamList", "bgparamlist");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_blifecondition() {
        test_all_of_type_assert("LifeConditionUser", "LifeCondition", "blifecondition");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_blod() {
        test_all_of_type_assert("LODUser", "LOD", "blod");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bmodellist() {
        test_all_of_type_print("ModelUser", "ModelList", "bmodellist");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bphysics() {
        test_all_of_type_print("PhysicsUser", "Physics", "bphysics");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bphyssb() {
        use std::collections::HashSet;
        use roead::{aamp::ParameterIO, sarc};

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let update_path = get_update_path();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Actor")
                    .join("Pack")
                    .join("*.sbactorpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let bxml = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                            .unwrap(),
                    )
                    .unwrap();
                    let user = bxml
                        .param_root
                        .objects
                        .get("LinkTarget")
                        .unwrap()
                        .get("PhysicsUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    if user == "Dummy" {
                        continue;
                    }
                    let bphysics = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/Physics/{}.bphysics", user))
                            .unwrap()
                    )
                    .unwrap();
                    if let Some(paramset) = bphysics.param_root.lists.get("ParamSet") {
                        if let Some(paramsetheader) = paramset.objects.get("ParamSetHeader") {
                            if paramsetheader.get("use_support_bone").unwrap().as_bool().unwrap() {
                                let support_bone_path = paramset.objects
                                    .get("SupportBone")
                                    .unwrap()
                                    .get("support_bone_setup_file_path")
                                    .unwrap()
                                    .as_string256()
                                    .unwrap();
                                let param_name = format!("Physics/SupportBone/{}", support_bone_path);
                                if result.contains(&param_name) {
                                    continue;
                                }
                                if let Some(o_file) = sarc.get_data(&param_name) {
                                    if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                                        let calc_size = super::estimate_from_bytes_and_name(
                                            o_file,
                                            &param_name,
                                            Endian::Big,
                                        )
                                        .unwrap();
                                        assert_eq!(calc_size, rstb_entry);
                                        result.insert(param_name);
                                    } else {
                                        println!("{} not in RSTB???", &param_name);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_brecipe() {
        test_all_of_type_assert("RecipeUser", "Recipe", "brecipe");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_brgconfig() {
        use std::collections::HashSet;
        use roead::{aamp::ParameterIO, sarc};

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let update_path = get_update_path();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Actor")
                    .join("Pack")
                    .join("*.sbactorpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let bxml = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                            .unwrap(),
                    )
                    .unwrap();
                    let user = bxml
                        .param_root
                        .objects
                        .get("LinkTarget")
                        .unwrap()
                        .get("RgConfigListUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    if user == "Dummy" {
                        continue;
                    }
                    let brgconfiglist = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/RagdollConfigList/{}.brgconfiglist", user))
                            .unwrap()
                    )
                    .unwrap();
                    if let Some(impulseparams) = brgconfiglist.param_root.lists.get("ImpulseParamList") {
                        for filename in impulseparams.objects
                            .iter()
                            .map(|(_, v)| v.get("FileName")
                                .unwrap()
                                .as_str()
                                .unwrap()
                            ) {
                            let param_name = format!("Actor/RagdollConfig/{}.brgconfig", filename);
                            if result.contains(&param_name) {
                                continue;
                            }
                            if let Some(o_file) = sarc.get_data(&param_name) {
                                if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                                    //print!("{}//{}: ", path.to_string_lossy(), param_name);
                                    let calc_size = super::estimate_from_bytes_and_name(
                                        o_file,
                                        &param_name,
                                        Endian::Big,
                                    )
                                    .unwrap();
                                    assert_eq!(calc_size, rstb_entry);
                                    result.insert(param_name);
                                } else {
                                    println!("{} not in RSTB???", &param_name);
                                    continue;
                                }
                            }
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
        let titlebg_path = update_path
            .join("Pack")
            .join("TitleBG.pack");
        let titlebg = sarc::Sarc::new(std::fs::read(&titlebg_path).unwrap()).unwrap();
        for file in titlebg.files() {
            if file.name.unwrap_or("").starts_with("Actor/Pack") {
                let actorname_as_pathbuf = PathBuf::from(file.name.unwrap());
                let actorname = actorname_as_pathbuf
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap();
                let sarc = sarc::Sarc::new(file.data).unwrap();
                let bxml = ParameterIO::from_binary(
                    sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                        .unwrap(),
                )
                .unwrap();
                let user = bxml
                    .param_root
                    .objects
                    .get("LinkTarget")
                    .unwrap()
                    .get("RgConfigListUser")
                    .unwrap()
                    .as_str()
                    .unwrap();
                if user == "Dummy" {
                    continue;
                }
                let brgconfiglist = ParameterIO::from_binary(
                    sarc.get_data(&format!("Actor/RagdollConfigList/{}.brgconfiglist", user))
                        .unwrap()
                )
                .unwrap();
                if let Some(impulseparams) = brgconfiglist.param_root.lists.get("ImpulseParamList") {
                    for filename in impulseparams.objects
                        .iter()
                        .map(|(_, v)| v.get("FileName")
                            .unwrap()
                            .as_str()
                            .unwrap()
                        ) {
                        let param_name = format!("Actor/RagdollConfig/{}.brgconfig", filename);
                        if result.contains(&param_name) {
                            continue;
                        }
                        if let Some(o_file) = sarc.get_data(&param_name) {
                            if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                                let calc_size = super::estimate_from_bytes_and_name(
                                    o_file,
                                    &param_name,
                                    Endian::Big,
                                )
                                .unwrap();
                                assert_eq!(calc_size, rstb_entry);
                                result.insert(param_name);
                            } else {
                                println!("{} not in RSTB???", &param_name);
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_brgconfiglist() {
        test_all_of_type_assert("RgConfigListUser", "RagdollConfigList", "brgconfiglist");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bshop() {
        test_all_of_type_assert("ShopDataUser", "ShopData", "bshop");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bumii() {
        test_all_of_type_assert("UMiiUser", "UMii", "bumii");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bxml() {
        use std::collections::HashSet;

        use glob::glob;
        use roead::sarc;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let update_path = get_update_path();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Actor")
                    .join("Pack")
                    .join("*.sbactorpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let param_name = format!("Actor/ActorLink/{}.bxml", actorname);
                    if param_name.contains("Dummy") | result.contains(&param_name) {
                        continue;
                    }
                    let bxml_bytes = sarc.get_data(&param_name).unwrap();
                    if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                        let calc_size = super::estimate_from_bytes_and_name(
                            bxml_bytes,
                            &param_name,
                            Endian::Big,
                        )
                        .unwrap();
                        assert_eq!(calc_size, rstb_entry);
                        result.insert(param_name);
                    } else {
                        println!("{} not in RSTB???", &param_name);
                        continue;
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_esetlist() {
        use glob::glob;

        use crate::ResourceSizeTable;

        let update_path = get_update_path();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Effect")
                    .join("*.sesetlist")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let param_name = format!("Effect/{filename}.esetlist", filename = path.file_stem().unwrap().to_str().unwrap());
                    if let Some(rstb_entry) = rstable.get(param_name.as_ref()) {
                        let calc_size = super::estimate_from_bytes_and_name(
                            std::fs::read(&path).unwrap().as_ref(),
                            param_name.as_ref(),
                            Endian::Big,
                        )
                        .unwrap();
                        assert_eq!(calc_size, rstb_entry);
                    } else {
                        println!("{} not in RSTB???", &param_name);
                        continue;
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_hkrb() {
        use std::collections::HashSet;
        use roead::sarc;

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let update_path = get_update_path();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Actor")
                    .join("Pack")
                    .join("*.sbactorpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    for file in sarc.files() {
                        let param_name = file.name.unwrap();
                        if !param_name.ends_with(".hkrb") | result.contains(param_name) {
                            continue;
                        }
                        if let Some(rstb_entry) = rstable.get(param_name) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                file.data,
                                param_name,
                                Endian::Big,
                            )
                            .unwrap();
                            assert_eq!(calc_size, rstb_entry);
                            result.insert(param_name.to_string());
                        } else {
                            println!("{} not in RSTB???", &param_name);
                            continue;
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_hkrg() {
        use std::collections::HashSet;
        use roead::sarc;

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let update_path = get_update_path();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Actor")
                    .join("Pack")
                    .join("*.sbactorpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    for file in sarc.files() {
                        let param_name = file.name.unwrap();
                        if !param_name.ends_with(".hkrg") | result.contains(param_name) {
                            continue;
                        }
                        if let Some(rstb_entry) = rstable.get(param_name) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                file.data,
                                param_name,
                                Endian::Big,
                            )
                            .unwrap();
                            assert_eq!(calc_size, rstb_entry);
                            result.insert(param_name.to_string());
                        } else {
                            println!("{} not in RSTB???", &param_name);
                            continue;
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_hksc() {
        use std::collections::HashSet;

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();
        let mut overshot: i32 = -0x300000;
        let mut undershot: i32 = 0x300000;

        let update_path = get_update_path();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
            std::fs::read(rstb_path).unwrap()
        ).unwrap();
        for entry in glob(
            update_path.join("Physics")
                .join("StaticCompound")
                .join("MainField")
                .join("*.shksc")
                .to_string_lossy()
                .as_ref()
        ).unwrap() {
            match entry {
                Ok(path) => {
                    let canonical = path.strip_prefix(&update_path)
                        .unwrap()
                        .to_string_lossy()
                        .replace(".s", ".");
                    if let Some(rstb_entry) = rstable.get(canonical.as_str()) {
                        let calc_size = super::estimate_from_bytes_and_name(
                            &*read(&path).unwrap(),
                            path.to_str().unwrap(),
                            Endian::Big,
                        ).unwrap();
                        let current = calc_size as i32 - rstb_entry as i32;
                        if current > 0 {
                            println!("{}: {}", canonical, current);
                        }
                        if overshot < current {
                            overshot = current;
                        }
                        if undershot > current {
                            undershot = current;
                        }
                        assert_ge!(calc_size, rstb_entry);
                        result.insert(canonical);
                    } else {
                        println!("{} not in RSTB???", &canonical);
                        continue;
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
        println!("Range (max amount of memory wasted with the overhead): {}", overshot - undershot);
        println!("Biggest underguess (overhead must be increased by this much): {}", -undershot);
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_bootup_byml() {
        use roead::sarc;
        use crate::ResourceSizeTable;
        let mut overshot: i32 = -0x300000;
        let mut undershot: i32 = 0x300000;

        let update_path = get_update_path();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        let bootup_path = update_path
            .join("Pack")
            .join("Bootup.pack");
        let bootup = sarc::Sarc::new(std::fs::read(&bootup_path).unwrap()).unwrap();
        for file in bootup.files() {
            if file.name.unwrap_or("").ends_with("byml") {
                let name = file.name.unwrap().replace(".s", ".");
                if let Some(rstb_entry) = rstable.get(name.as_ref()) {
                    let calc_size = super::estimate_from_bytes_and_name(
                        file.data,
                        name.as_ref(),
                        Endian::Big,
                    )
                    .unwrap();
                    let current = calc_size as i32 - rstb_entry as i32;
                    if current > 0 {
                        println!("{}: {}", name, current);
                    }
                    if overshot < current {
                        overshot = current;
                    }
                    if undershot > current {
                        undershot = current;
                    }
                    assert_ge!(calc_size, rstb_entry);
                } else {
                    println!("{} not in RSTB???", name);
                    continue;
                }
            }
        }
        println!("Range (max amount of memory wasted with the overhead): {}", overshot - undershot);
        println!("Biggest underguess (overhead must be increased by this much): {}", -undershot);
    }

    #[cfg(feature = "complex_testing")]
    fn test_all_of_type_assert_nx(link: &str, folder: &str, ext: &str) {
        use std::collections::HashSet;
        use roead::{aamp::ParameterIO, sarc};

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let update_path = get_update_path_nx();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Actor")
                    .join("Pack")
                    .join("*.sbactorpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let bxml = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                            .unwrap(),
                    )
                    .unwrap();
                    let user = bxml
                        .param_root
                        .objects
                        .get("LinkTarget")
                        .unwrap()
                        .get(link)
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/{}/{}.{}", folder, user, ext);
                    if param_name.contains("Dummy") | result.contains(&param_name) {
                        continue;
                    }
                    if let Some(o_file) = sarc.get_data(&param_name) {
                        if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                o_file,
                                &param_name,
                                Endian::Little,
                            )
                            .unwrap();
                            assert_eq!(calc_size, rstb_entry);
                            result.insert(param_name);
                        } else {
                            println!("{} not in RSTB???", &param_name);
                            continue;
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
        let titlebg_path = update_path
            .join("Pack")
            .join("TitleBG.pack");
        let titlebg = sarc::Sarc::new(std::fs::read(&titlebg_path).unwrap()).unwrap();
        for file in titlebg.files() {
            if file.name.unwrap_or("").starts_with("Actor/Pack") {
                let actorname_as_pathbuf = PathBuf::from(file.name.unwrap());
                let actorname = actorname_as_pathbuf
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap();
                let sarc = sarc::Sarc::new(file.data).unwrap();
                let bxml = ParameterIO::from_binary(
                    sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                        .unwrap(),
                )
                .unwrap();
                let user = bxml
                    .param_root
                    .objects
                    .get("LinkTarget")
                    .unwrap()
                    .get(link)
                    .unwrap()
                    .as_str()
                    .unwrap();
                let param_name = format!("Actor/{}/{}.{}", folder, user, ext);
                if param_name.contains("Dummy") | result.contains(&param_name) {
                    continue;
                }
                if let Some(o_file) = sarc.get_data(&param_name) {
                    if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                        let calc_size = super::estimate_from_bytes_and_name(
                            o_file,
                            &param_name,
                            Endian::Little,
                        )
                        .unwrap();
                        assert_eq!(calc_size, rstb_entry);
                        result.insert(param_name);
                    } else {
                        println!("{} not in RSTB???", &param_name);
                        continue;
                    }
                }
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    fn test_all_of_type_print_nx(link: &str, folder: &str, ext: &str) {
        use std::collections::HashSet;
        use roead::{aamp::ParameterIO, sarc};

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();
        let mut overshot: i32 = -0x300000;
        let mut undershot: i32 = 0x300000;

        let update_path = get_update_path_nx();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Actor")
                    .join("Pack")
                    .join("*.sbactorpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let bxml = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                            .unwrap(),
                    )
                    .unwrap();
                    let user = bxml
                        .param_root
                        .objects
                        .get("LinkTarget")
                        .unwrap()
                        .get(link)
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/{}/{}.{}", folder, user, ext);
                    if param_name.contains("Dummy") | result.contains(&param_name) {
                        continue;
                    }
                    if let Some(o_file) = sarc.get_data(&param_name) {
                        if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                o_file,
                                &param_name,
                                Endian::Little,
                            )
                            .unwrap();
                            let current = calc_size as i32 - rstb_entry as i32;
                            if current > 0 {
                                println!("{}//{}: {}", actorname, param_name, current);
                            }
                            if overshot < current {
                                overshot = current;
                            }
                            if undershot > current {
                                undershot = current;
                            }
                            assert_ge!(calc_size, rstb_entry);
                            result.insert(param_name);
                        } else {
                            println!("{} not in RSTB???", &param_name);
                            continue;
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
        let titlebg_path = update_path
            .join("Pack")
            .join("TitleBG.pack");
        let titlebg = sarc::Sarc::new(std::fs::read(&titlebg_path).unwrap()).unwrap();
        for file in titlebg.files() {
            if file.name.unwrap_or("").starts_with("Actor/Pack") {
                let actorname_as_pathbuf = PathBuf::from(file.name.unwrap());
                let actorname = actorname_as_pathbuf
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap();
                let sarc = sarc::Sarc::new(file.data).unwrap();
                let bxml = ParameterIO::from_binary(
                    sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                        .unwrap(),
                )
                .unwrap();
                let user = bxml
                    .param_root
                    .objects
                    .get("LinkTarget")
                    .unwrap()
                    .get(link)
                    .unwrap()
                    .as_str()
                    .unwrap();
                let param_name = format!("Actor/{}/{}.{}", folder, user, ext);
                if param_name.contains("Dummy") | result.contains(&param_name) {
                    continue;
                }
                if let Some(o_file) = sarc.get_data(&param_name) {
                    if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                        let calc_size = super::estimate_from_bytes_and_name(
                            o_file,
                            &param_name,
                            Endian::Little,
                        )
                        .unwrap();
                        let current = calc_size as i32 - rstb_entry as i32;
                        if current > 0 {
                            println!("{}//{}: {}", actorname, param_name, current);
                        }
                        if overshot < current {
                            overshot = current;
                        }
                        if undershot > current {
                            undershot = current;
                        }
                        assert_ge!(calc_size, rstb_entry);
                        result.insert(param_name);
                    } else {
                        println!("{} not in RSTB???", &param_name);
                        continue;
                    }
                }
            }
        }
        println!("Range (max amount of memory wasted with the overhead): {}", overshot - undershot);
        println!("Biggest underguess (overhead must be increased by this much): {}", -undershot);
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_baiprog_nx() {
        test_all_of_type_print_nx("AIProgramUser", "AIProgram", "baiprog");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_baischedule_nx() {
        test_all_of_type_assert_nx("AIScheduleUser", "AISchedule", "baischedule");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_baniminfo_nx() {
        test_all_of_type_print_nx("AnimationInfo", "AnimationInfo", "baniminfo");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_baslist_nx() {
        test_all_of_type_assert_nx("ASUser", "ASList", "baslist");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bawareness_nx() {
        test_all_of_type_assert_nx("AwarenessUser", "Awareness", "bawareness");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bchemical_nx() {
        test_all_of_type_assert_nx("ChemicalUser", "Chemical", "bchemical");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bdemo_nx() {
        use std::collections::HashSet;
        use roead::sarc;

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let update_path = get_update_path_nx();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Event")
                    .join("*.sbeventpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    for file in sarc.files() {
                        let param_name = file.name.unwrap();
                        if !param_name.ends_with(".bdemo") | result.contains(param_name) {
                            continue;
                        }
                        if let Some(rstb_entry) = rstable.get(param_name) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                file.data,
                                param_name,
                                Endian::Little,
                            )
                            .unwrap();
                            assert_eq!(calc_size, rstb_entry);
                            result.insert(param_name.to_string());
                        } else {
                            println!("{} not in RSTB???", &param_name);
                            continue;
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bdmgparam_nx() {
        test_all_of_type_assert_nx("DamageParamUser", "DamageParam", "bdmgparam");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bdrop_nx() {
        test_all_of_type_assert_nx("DropTableUser", "DropTable", "bdrop");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bfevfl_nx() {
        use std::collections::HashSet;
        use roead::sarc;

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let update_path = get_update_path_nx();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Event")
                    .join("*.sbeventpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    for file in sarc.files() {
                        let param_name = file.name.unwrap();
                        if !param_name.ends_with(".bfevfl") | result.contains(param_name) {
                            continue;
                        }
                        if let Some(rstb_entry) = rstable.get(param_name) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                file.data,
                                param_name,
                                Endian::Little,
                            )
                            .unwrap();
                            assert_eq!(calc_size, rstb_entry);
                            result.insert(param_name.to_string());
                        } else {
                            println!("{} not in RSTB???", &param_name);
                            continue;
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bfevtm_nx() {
        use std::collections::HashSet;
        use roead::sarc;

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let update_path = get_update_path_nx();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Event")
                    .join("*.sbeventpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    for file in sarc.files() {
                        let param_name = file.name.unwrap();
                        if !param_name.ends_with(".bfevtm") | result.contains(param_name) {
                            continue;
                        }
                        if let Some(rstb_entry) = rstable.get(param_name) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                file.data,
                                param_name,
                                Endian::Little,
                            )
                            .unwrap();
                            assert_eq!(calc_size, rstb_entry);
                            result.insert(param_name.to_string());
                        } else {
                            println!("{} not in RSTB???", &param_name);
                            continue;
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bgparamlist_nx() {
        test_all_of_type_print_nx("GParamUser", "GeneralParamList", "bgparamlist");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_blifecondition_nx() {
        test_all_of_type_assert_nx("LifeConditionUser", "LifeCondition", "blifecondition");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_blod_nx() {
        test_all_of_type_assert_nx("LODUser", "LOD", "blod");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bmodellist_nx() {
        test_all_of_type_assert_nx("ModelUser", "ModelList", "bmodellist");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bphysics_nx() {
        test_all_of_type_print_nx("PhysicsUser", "Physics", "bphysics");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bphyssb_nx() {
        use std::collections::HashSet;
        use roead::{aamp::ParameterIO, sarc};

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let update_path = get_update_path_nx();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Actor")
                    .join("Pack")
                    .join("*.sbactorpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let bxml = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                            .unwrap(),
                    )
                    .unwrap();
                    let user = bxml
                        .param_root
                        .objects
                        .get("LinkTarget")
                        .unwrap()
                        .get("PhysicsUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    if user == "Dummy" {
                        continue;
                    }
                    let bphysics = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/Physics/{}.bphysics", user))
                            .unwrap()
                    )
                    .unwrap();
                    if let Some(paramset) = bphysics.param_root.lists.get("ParamSet") {
                        if let Some(paramsetheader) = paramset.objects.get("ParamSetHeader") {
                            if paramsetheader.get("use_support_bone").unwrap().as_bool().unwrap() {
                                let support_bone_path = paramset.objects
                                    .get("SupportBone")
                                    .unwrap()
                                    .get("support_bone_setup_file_path")
                                    .unwrap()
                                    .as_string256()
                                    .unwrap();
                                let param_name = format!("Physics/SupportBone/{}", support_bone_path);
                                if result.contains(&param_name) {
                                    continue;
                                }
                                if let Some(o_file) = sarc.get_data(&param_name) {
                                    if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                                        let calc_size = super::estimate_from_bytes_and_name(
                                            o_file,
                                            &param_name,
                                            Endian::Little,
                                        )
                                        .unwrap();
                                        assert_eq!(calc_size, rstb_entry);
                                        result.insert(param_name);
                                    } else {
                                        println!("{} not in RSTB???", &param_name);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_brecipe_nx() {
        test_all_of_type_assert_nx("RecipeUser", "Recipe", "brecipe");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_brgconfig_nx() {
        use std::collections::HashSet;
        use roead::{aamp::ParameterIO, sarc};

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let update_path = get_update_path_nx();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Actor")
                    .join("Pack")
                    .join("*.sbactorpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let bxml = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                            .unwrap(),
                    )
                    .unwrap();
                    let user = bxml
                        .param_root
                        .objects
                        .get("LinkTarget")
                        .unwrap()
                        .get("RgConfigListUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    if user == "Dummy" {
                        continue;
                    }
                    let brgconfiglist = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/RagdollConfigList/{}.brgconfiglist", user))
                            .unwrap()
                    )
                    .unwrap();
                    if let Some(impulseparams) = brgconfiglist.param_root.lists.get("ImpulseParamList") {
                        for filename in impulseparams.objects
                            .iter()
                            .map(|(_, v)| v.get("FileName")
                                .unwrap()
                                .as_str()
                                .unwrap()
                            ) {
                            let param_name = format!("Actor/RagdollConfig/{}.brgconfig", filename);
                            if result.contains(&param_name) {
                                continue;
                            }
                            if let Some(o_file) = sarc.get_data(&param_name) {
                                if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                                    let calc_size = super::estimate_from_bytes_and_name(
                                        o_file,
                                        &param_name,
                                        Endian::Little,
                                    )
                                    .unwrap();
                                    assert_eq!(calc_size, rstb_entry);
                                    result.insert(param_name);
                                } else {
                                    println!("{} not in RSTB???", &param_name);
                                    continue;
                                }
                            }
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
        let titlebg_path = update_path
            .join("Pack")
            .join("TitleBG.pack");
        let titlebg = sarc::Sarc::new(std::fs::read(&titlebg_path).unwrap()).unwrap();
        for file in titlebg.files() {
            if file.name.unwrap_or("").starts_with("Actor/Pack") {
                let actorname_as_pathbuf = PathBuf::from(file.name.unwrap());
                let actorname = actorname_as_pathbuf
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap();
                let sarc = sarc::Sarc::new(file.data).unwrap();
                let bxml = ParameterIO::from_binary(
                    sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                        .unwrap(),
                )
                .unwrap();
                let user = bxml
                    .param_root
                    .objects
                    .get("LinkTarget")
                    .unwrap()
                    .get("RgConfigListUser")
                    .unwrap()
                    .as_str()
                    .unwrap();
                if user == "Dummy" {
                    continue;
                }
                let brgconfiglist = ParameterIO::from_binary(
                    sarc.get_data(&format!("Actor/RagdollConfigList/{}.brgconfiglist", user))
                        .unwrap()
                )
                .unwrap();
                if let Some(impulseparams) = brgconfiglist.param_root.lists.get("ImpulseParamList") {
                    for filename in impulseparams.objects
                        .iter()
                        .map(|(_, v)| v.get("FileName")
                            .unwrap()
                            .as_str()
                            .unwrap()
                        ) {
                        let param_name = format!("Actor/RagdollConfig/{}.brgconfig", filename);
                        if result.contains(&param_name) {
                            continue;
                        }
                        if let Some(o_file) = sarc.get_data(&param_name) {
                            if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                                let calc_size = super::estimate_from_bytes_and_name(
                                    o_file,
                                    &param_name,
                                    Endian::Little,
                                )
                                .unwrap();
                                assert_eq!(calc_size, rstb_entry);
                                result.insert(param_name);
                            } else {
                                println!("{} not in RSTB???", &param_name);
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_brgconfiglist_nx() {
        test_all_of_type_assert_nx("RgConfigListUser", "RagdollConfigList", "brgconfiglist");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bshop_nx() {
        test_all_of_type_assert_nx("ShopDataUser", "ShopData", "bshop");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bumii_nx() {
        test_all_of_type_assert_nx("UMiiUser", "UMii", "bumii");
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bxml_nx() {
        use std::collections::HashSet;

        use glob::glob;
        use roead::sarc;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let update_path = get_update_path_nx();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Actor")
                    .join("Pack")
                    .join("*.sbactorpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let param_name = format!("Actor/ActorLink/{}.bxml", actorname);
                    if param_name.contains("Dummy") | result.contains(&param_name) {
                        continue;
                    }
                    let bxml_bytes = sarc.get_data(&param_name).unwrap();
                    if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                        let calc_size = super::estimate_from_bytes_and_name(
                            bxml_bytes,
                            &param_name,
                            Endian::Little,
                        )
                        .unwrap();
                        assert_eq!(calc_size, rstb_entry);
                        result.insert(param_name);
                    } else {
                        println!("{} not in RSTB???", &param_name);
                        continue;
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_esetlist_nx() {
        use glob::glob;

        use crate::ResourceSizeTable;

        let update_path = get_update_path_nx();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Effect")
                    .join("*.sesetlist")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let param_name = format!("Effect/{filename}.esetlist", filename = path.file_stem().unwrap().to_str().unwrap());
                    if let Some(rstb_entry) = rstable.get(param_name.as_ref()) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                std::fs::read(&path).unwrap().as_ref(),
                                param_name.as_ref(),
                                Endian::Little,
                            )
                            .unwrap();
                            assert_eq!(calc_size, rstb_entry);
                        } else {
                            println!("{} not in RSTB???", &param_name);
                            continue;
                        }
                }
                Err(_) => println!("File error...?"),
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_hkrb_nx() {
        use std::collections::HashSet;
        use roead::sarc;

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let update_path = get_update_path_nx();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Actor")
                    .join("Pack")
                    .join("*.sbactorpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    for file in sarc.files() {
                        let param_name = file.name.unwrap();
                        if !param_name.ends_with(".hkrb") | result.contains(param_name) {
                            continue;
                        }
                        if let Some(rstb_entry) = rstable.get(param_name) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                file.data,
                                param_name,
                                Endian::Little,
                            )
                            .unwrap();
                            assert_eq!(calc_size, rstb_entry);
                            result.insert(param_name.to_string());
                        } else {
                            println!("{} not in RSTB???", &param_name);
                            continue;
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_hkrg_nx() {
        use std::collections::HashSet;
        use roead::sarc;

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let update_path = get_update_path_nx();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                update_path.join("Actor")
                    .join("Pack")
                    .join("*.sbactorpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    for file in sarc.files() {
                        let param_name = file.name.unwrap();
                        if !param_name.ends_with(".hkrg") | result.contains(param_name) {
                            continue;
                        }
                        if let Some(rstb_entry) = rstable.get(param_name) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                file.data,
                                param_name,
                                Endian::Little,
                            )
                            .unwrap();
                            assert_eq!(calc_size, rstb_entry);
                            result.insert(param_name.to_string());
                        } else {
                            println!("{} not in RSTB???", &param_name);
                            continue;
                        }
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_bootup_byml_nx() {
        use roead::sarc;
        use crate::ResourceSizeTable;
        let mut overshot: i32 = -0x300000;
        let mut undershot: i32 = 0x300000;

        let update_path = get_update_path_nx();
        let rstb_path = update_path
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        let bootup_path = update_path
            .join("Pack")
            .join("Bootup.pack");
        let bootup = sarc::Sarc::new(std::fs::read(&bootup_path).unwrap()).unwrap();
        for file in bootup.files() {
            if file.name.unwrap_or("").ends_with("byml") {
                let name = file.name.unwrap().replace(".s", ".");
                if let Some(rstb_entry) = rstable.get(name.as_ref()) {
                    let calc_size = super::estimate_from_bytes_and_name(
                        file.data,
                        name.as_ref(),
                        Endian::Little,
                    )
                    .unwrap();
                    let current = calc_size as i32 - rstb_entry as i32;
                    if current > 0 {
                        println!("{}: {}", name, current);
                    }
                    if overshot < current {
                        overshot = current;
                    }
                    if undershot > current {
                        undershot = current;
                    }
                    assert_ge!(calc_size, rstb_entry);
                } else {
                    println!("{} not in RSTB???", name);
                    continue;
                }
            }
        }
        println!("Range (max amount of memory wasted with the overhead): {}", overshot - undershot);
        println!("Biggest underguess (overhead must be increased by this much): {}", -undershot);
    }

    #[cfg(feature = "complex_testing")]
    #[test]
    fn write_graphic_pack_rstb_from_formulas_only() {
        use std::{collections::HashSet, fs, path::Path};
        use dirs2;
        use ryml::Tree;

        use glob::glob;
        use roead::{sarc, yaz0};

        use crate::ResourceSizeTable;
        let mut parsed: HashSet<String> = HashSet::new();

        let settings_path = dirs2::data_dir()
            .unwrap()
            .join("ukmm")
            .join("settings.yml");
        let settings = Tree::parse(
                std::fs::read_to_string(settings_path).unwrap()
            ).unwrap();
        let profile_node = settings.root_ref()
            .unwrap()
            .get("wiiu_config")
            .unwrap()
            .get("profile")
            .unwrap();
        let profile = profile_node.val().unwrap();
        let root = dirs2::data_local_dir()
            .unwrap()
            .join("ukmm")
            .join("wiiu")
            .join("profiles")
            .join(profile)
            .join("merged")
            .join("content");

        let rstb_path = root
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");

        let rstb_backup = root
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable.bak");

        if !rstb_backup.exists() {
            println!("RSTB not backed up. Backing up...");
            fs::copy(&rstb_path, &rstb_backup).expect("Failed to back up RSTB");
        }
        let mut rstable = ResourceSizeTable::from_binary(fs::read(rstb_backup).unwrap()).unwrap();

        let bootup =
            sarc::Sarc::new(fs::read(root.join("Pack").join("Bootup.pack")).unwrap()).unwrap();
        for bootup_file in bootup.files() {
            if let Some(name) = bootup_file.name {
                if parsed.contains(name) {
                    continue;
                }
                let entry = name.replace(".s", ".");
                if let Some(ext) = Path::new(&entry).extension() {
                    if ext == "byml" {
                        if rstable.contains(entry.as_str()) {
                            rstable.set(
                                entry.as_str(),
                                super::estimate_from_bytes_and_name(
                                    bootup_file.data,
                                    &entry,
                                    Endian::Big,
                                )
                                .unwrap(),
                            );
                        }
                    }
                }
                parsed.insert(name.to_owned());
            }
        }

        let titlebg =
            sarc::Sarc::new(fs::read(root.join("Pack").join("TitleBG.pack")).unwrap()).unwrap();
        for bg_file in titlebg.files() {
            if let Some(name) = bg_file.name {
                if parsed.contains(name) {
                    continue;
                }
                let entry = name.replace(".s", ".");
                if let Some(ext) = Path::new(&entry).extension() {
                    if ext == "bfres" && name.contains("Tex") {
                        if rstable.contains(entry.as_str()) {
                            rstable.set(
                                entry.as_str(),
                                super::estimate_from_bytes_and_name(
                                    bg_file.data,
                                    &entry,
                                    Endian::Big,
                                )
                                .unwrap(),
                            );
                        }
                    } else if ext == "bdemo" {
                        if rstable.contains(entry.as_str()) {
                            rstable.set(
                                entry.as_str(),
                                super::estimate_from_bytes_and_name(
                                    bg_file.data,
                                    &entry,
                                    Endian::Big,
                                )
                                .unwrap(),
                            );
                        }
                    } else if ext == "bactorpack" {
                        if rstable.contains(entry.as_str()) {
                            rstable.set(
                                entry.as_str(),
                                super::estimate_from_bytes_and_name(
                                    bg_file.data,
                                    &entry,
                                    Endian::Big,
                                )
                                .unwrap(),
                            );
                        }
                        let pack = sarc::Sarc::new(bg_file.data).unwrap();
                        for s_file in pack.files() {
                            let s_name = s_file.name.unwrap();
                            if /* !rstable.contains(s_name) || */parsed.contains(s_name) {
                                continue;
                            }
                            match Path::new(s_name).extension().unwrap().to_str().unwrap() {
                                "baiprog"
                                | "baischedule"
                                | "baniminfo"
                                | "baslist"
                                | "bawareness"
                                | "bchemical"
                                | "bdmgparam"
                                | "bdrop"
                                | "bgparamlist"
                                | "blifecondition"
                                | "blod"
                                | "bmodellist"
                                | "bphysics"
                                | "bphyssb"
                                | "brecipe"
                                | "brgconfiglist"
                                | "bshop"
                                | "bumii"
                                | "bxml"
                                | "hkrb"
                                | "hkrg"
                                => {
                                    rstable.set(
                                        s_name,
                                        super::estimate_from_bytes_and_name(
                                            s_file.data,
                                            s_name,
                                            Endian::Big,
                                        )
                                        .unwrap(),
                                    );
                                }
                                _ => {}
                            }
                            parsed.insert(s_name.to_owned());
                        }
                    } else if ext == "beventpack" {
                        if rstable.contains(entry.as_str()) {
                            rstable.set(
                                entry.as_str(),
                                super::estimate_from_bytes_and_name(
                                    bg_file.data,
                                    &entry,
                                    Endian::Big,
                                )
                                .unwrap(),
                            );
                        }
                        let pack = sarc::Sarc::new(bg_file.data).unwrap();
                        for s_file in pack.files() {
                            let s_name = s_file.name.unwrap();
                            if /* !rstable.contains(s_name) || */parsed.contains(s_name) {
                                continue;
                            }
                            match Path::new(s_name).extension().unwrap().to_str().unwrap() {
                                "bdemo"
                                | "bfevfl"
                                | "bfevtm"
                                => {
                                    rstable.set(
                                        s_name,
                                        super::estimate_from_bytes_and_name(
                                            s_file.data,
                                            s_name,
                                            Endian::Big,
                                        )
                                        .unwrap(),
                                    );
                                }
                                _ => {}
                            }
                            parsed.insert(s_name.to_owned());
                        }
                    }
                }
                parsed.insert(name.to_owned());
            }
        }

        for entry in glob(
                root.join("Model").join("*.Tex*.sbfres").to_string_lossy().as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let param_name = format!(
                        "Model/{}.bfres",
                        path.file_stem().unwrap().to_str().unwrap()
                    );
                    if parsed.contains(&param_name) {
                        continue;
                    }
                    if rstable.contains(param_name.as_str()) {
                        rstable.set(
                            param_name.as_str(),
                            super::estimate_from_file(path, Endian::Big)
                                .unwrap()
                                .unwrap(),
                        );
                    }
                    parsed.insert(param_name);
                }
                Err(_) => println!("BFRES file error...?"),
            }
        }

        for entry in glob(
                root.join("Actor")
                    .join("Pack")
                    .join("*.sbactorpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let param_name = format!(
                        "Actor/Pack/{}.bactorpack",
                        path.file_stem().unwrap().to_str().unwrap()
                    );
                    let data = fs::read(&path).unwrap();
                    if /*rstable.contains(param_name.as_str()) && */!parsed.contains(&param_name) {
                        rstable.set(
                            param_name.as_str(),
                            super::estimate_from_bytes_and_name(&data, &param_name, Endian::Big)
                                .unwrap(),
                        );
                        parsed.insert(param_name);
                    }
                    let pack = sarc::Sarc::new(data).unwrap();
                    for s_file in pack.files() {
                        let s_name = s_file.name.unwrap();
                        if /* !rstable.contains(s_name) || */parsed.contains(s_name) {
                            continue;
                        }
                        match Path::new(s_name).extension().unwrap().to_str().unwrap() {
                            "baiprog"
                            | "baischedule"
                            | "baniminfo"
                            | "baslist"
                            | "bawareness"
                            | "bchemical"
                            | "bdmgparam"
                            | "bdrop"
                            | "bgparamlist"
                            | "blifecondition"
                            | "blod"
                            | "bmodellist"
                            | "bphysics"
                            | "bphyssb"
                            | "brecipe"
                            | "brgconfiglist"
                            | "bshop"
                            | "bumii"
                            | "bxml"
                            | "hkrb"
                            | "hkrg"
                            => {
                                rstable.set(
                                    s_name,
                                    super::estimate_from_bytes_and_name(
                                        s_file.data,
                                        s_name,
                                        Endian::Big,
                                    )
                                    .unwrap(),
                                );
                            }
                            _ => {}
                        }
                        parsed.insert(s_name.to_owned());
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }

        for entry in glob(
                root.join("Event")
                    .join("*.sbeventpack")
                    .to_string_lossy()
                    .as_ref()
            ).unwrap() {
            match entry {
                Ok(path) => {
                    let param_name = format!(
                        "Event/{}.beventpack",
                        path.file_stem().unwrap().to_str().unwrap()
                    );
                    let data = fs::read(&path).unwrap();
                    if /*rstable.contains(param_name.as_str()) && */!parsed.contains(&param_name) {
                        rstable.set(
                            param_name.as_str(),
                            super::estimate_from_bytes_and_name(&data, &param_name, Endian::Big)
                                .unwrap(),
                        );
                        parsed.insert(param_name);
                    }
                    let pack = sarc::Sarc::new(data).unwrap();
                    for s_file in pack.files() {
                        let s_name = s_file.name.unwrap();
                        if /* !rstable.contains(s_name) || */parsed.contains(s_name) {
                            continue;
                        }
                        match Path::new(s_name).extension().unwrap().to_str().unwrap() {
                            "bdemo"
                            | "bfevfl"
                            | "bfevtm"
                            => {
                                rstable.set(
                                    s_name,
                                    super::estimate_from_bytes_and_name(
                                        s_file.data,
                                        s_name,
                                        Endian::Big,
                                    )
                                    .unwrap(),
                                );
                            }
                            _ => {}
                        }
                        parsed.insert(s_name.to_owned());
                    }
                }
                Err(_) => println!("File error...?"),
            }
        }

        let mut buffer = Vec::<u8>::new();
        rstable
            .write(&mut buffer, Endian::Big)
            .expect("Couldn't write RSTB");
        std::fs::write(rstb_path, yaz0::compress(buffer)).expect("Couldn't write RSTB to disc");
    }
}
