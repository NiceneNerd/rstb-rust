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
use cpp_memsizes::{
    baiprog, baslist, bdrop, bgparamlist, bmodellist, bphysics, brecipe, bshop, bxml,
};
use info::{get_factory_info, ParseSize};

use crate::{Endian, Result};

#[inline]
fn round_32(size: usize) -> u32 {
    ((size as isize + 31) & -32) as u32
}

/// Infallibly calculate an RSTB value from a file on disk, returning `None` if
/// the type is not supported.
pub fn calc_from_file<P: AsRef<Path>>(file: P, endian: Endian) -> Result<Option<u32>> {
    Ok(calc_from_slice_and_name(
        &std::fs::read(file.as_ref())?,
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
        let (size, parse_size) = get_factory_info(ext, endian);
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
        let (size, parse_size) = get_factory_info(ext, endian);
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
                                "bdmgparam" => {
                                    let rounded = rounded as f32;
                                    return Some(
                                        ((((-0.0018 * rounded) + 6.6273) * rounded) + 500.0) as u32,
                                    );
                                }
                                _ => 0,
                            }
                    }
                    Endian::Little => {
                        rounded
                            + 0x168
                            + size
                            + parse_size
                            + match ext {
                                "bdmgparam" => {
                                    let rounded = rounded as f32;
                                    return Some(
                                        (((((-0.0018 * rounded) + 6.6273) * rounded) + 500.0) * 1.5)
                                            as u32,
                                    );
                                }
                                _ => 0,
                            }
                    }
                })
            }
            ParseSize::Complex => {
                if estimate {
                    match ext {
                        #[cfg(feature = "complex")]
                        "baiprog" => Some(rounded + baiprog::parse_size(bytes, endian)),
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
                        "baslist" => Some(rounded + baslist::parse_size(bytes, endian)),
                        #[cfg(feature = "complex")]
                        "bdrop" => Some(rounded + bdrop::parse_size(bytes, endian)),
                        "bfres" => Some(estimate_bfres(filesize, endian)),
                        #[cfg(feature = "complex")]
                        "bgparamlist" => Some(rounded + bgparamlist::parse_size(bytes, endian)),
                        #[cfg(feature = "complex")]
                        "bmodellist" => Some(rounded + bmodellist::parse_size(bytes, endian)),
                        #[cfg(feature = "complex")]
                        "bphysics" => Some(rounded + bphysics::parse_size(bytes, endian)),
                        #[cfg(feature = "complex")]
                        "brecipe" => Some(rounded + brecipe::parse_size(bytes, endian)),
                        #[cfg(feature = "complex")]
                        "bshop" => Some(rounded + bshop::parse_size(bytes, endian)),
                        #[cfg(feature = "complex")]
                        "bxml" => Some(rounded + bxml::parse_size(bytes, endian)),
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
    use std::fs::read;

    use all_asserts::assert_ge;

    use crate::Endian;

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

    #[cfg(feature = "complex")]
    #[test]
    fn estimate_sizes_complex() {
        assert_eq!(
            super::estimate_from_slice_and_name(
                &std::fs::read("test/Animal_Bass.Tex1.sbfres").unwrap(),
                "Model/Animal_Bass.Tex1.sbfres",
                Endian::Big
            ),
            Some(42756)
        );
        assert_ge!(
            super::estimate_from_slice_and_name(
                &std::fs::read("test/DgnMrgPrt_Dungeon061.sbfres").unwrap(),
                "Model/DgnMrgPrt_Dungeon061.sbfres",
                Endian::Big,
            ),
            Some(8671688)
        );
        assert_ge!(
            super::estimate_from_slice_and_name(
                &std::fs::read("test/NpcGerudoQueenBattle.baiprog").unwrap(),
                "Actor/AIProgram/NpcGerudoQueenBattle.baiprog",
                Endian::Big,
            ),
            Some(9444)
        );
        assert_eq!(
            super::estimate_from_slice_and_name(
                &std::fs::read("test/Player_Link.bgparamlist").unwrap(),
                "Actor/GeneralParamList/Player_Link.bgparamlist",
                Endian::Big,
            ),
            Some(7076)
        );
        assert_eq!(
            super::estimate_from_slice_and_name(
                &std::fs::read("test/Armor_001_Upper.bmodellist").unwrap(),
                "Actor/ModelList/Armor_001_Upper.bmodellist",
                Endian::Big,
            ),
            Some(2636)
        );
        assert_ge!(
            super::estimate_from_slice_and_name(
                &std::fs::read("test/Armor_002_Upper.brecipe").unwrap(),
                "Actor/ModelList/Armor_002_Upper.brecipe",
                Endian::Big,
            ),
            Some(1276)
        );
        assert_ge!(
            super::estimate_from_slice_and_name(
                &std::fs::read("test/Assassin_Senior.bdrop").unwrap(),
                "Actor/DropTable/Assassin_Senior.bdrop",
                Endian::Big,
            ),
            Some(1132)
        );
        assert_eq!(
            super::estimate_from_slice_and_name(
                &std::fs::read("test/Assassin_Senior.bxml").unwrap(),
                "Actor/ActorLink/Assassin_Senior.bxml",
                Endian::Big,
            ),
            Some(2116)
        );
        assert_eq!(
            super::estimate_from_slice_and_name(
                &std::fs::read("test/Npc_TripMaster_08.bshop").unwrap(),
                "Actor/ShopData/Npc_TripMaster_08.bshop",
                Endian::Big,
            ),
            Some(2588)
        );
        assert_ge!(
            super::estimate_from_slice_and_name(
                &std::fs::read("test/Player_Link.bphysics").unwrap(),
                "Actor/Physics/Player_Link.bphysics",
                Endian::Big,
            ),
            Some(38940)
        );
    }
    #[cfg(feature = "complex")]
    #[test]
    fn agl_size_tests() {
        use std::mem::size_of;

        use crate::calc::cpp_memsizes::cpp_classes::{agl::*, *};
        assert_eq!(size_of::<ParameterList<u32>>(), 0x24);
        assert_eq!(size_of::<ParameterObj<u32>>(), 0x1c);
        assert_eq!(size_of::<ParameterBase<u32>>(), 0xc);
        assert_eq!(size_of::<Parameter<u32, Bool32>>(), 0x10);
        assert_eq!(size_of::<Parameter<u32, Int>>(), 0x10);
        assert_eq!(size_of::<Parameter<u32, S32>>(), 0x10);
        assert_eq!(size_of::<Parameter<u32, U32>>(), 0x10);
        assert_eq!(size_of::<Parameter<u32, Float>>(), 0x10);
        assert_eq!(size_of::<Parameter<u32, F32>>(), 0x10);
        assert_eq!(size_of::<Parameter<u32, Vector2f>>(), 0x14);
        assert_eq!(size_of::<Parameter<u32, Vector3f>>(), 0x18);
        assert_eq!(size_of::<Parameter<u32, Vector4f>>(), 0x1c);
        assert_eq!(size_of::<Parameter<u32, SafeString<u32>>>(), 0x18);
        assert_eq!(size_of::<Parameter<u32, FixedSafeString32<u32>>>(), 0x30);
        assert_eq!(size_of::<Parameter<u32, FixedSafeString64<u32>>>(), 0x50);
        assert_eq!(size_of::<Parameter<u32, FixedSafeString256<u32>>>(), 0x110);
        assert_eq!(size_of::<SafeString<u32>>(), 0xc);
        assert_eq!(size_of::<FixedSafeString32<u32>>(), 0x24);
        assert_eq!(size_of::<FixedSafeString64<u32>>(), 0x44);
        assert_eq!(size_of::<FixedSafeString256<u32>>(), 0x104);
        assert_eq!(size_of::<SeadBuffer<u32>>(), 0x8);
        assert_eq!(size_of::<ParameterList<u64>>(), 0x48);
        assert_eq!(size_of::<ParameterObj<u64>>(), 0x30);
        assert_eq!(size_of::<ParameterBase<u64>>(), 0x18);
        assert_eq!(size_of::<Parameter<u64, Bool32>>(), 0x20);
        assert_eq!(size_of::<Parameter<u64, Int>>(), 0x20);
        assert_eq!(size_of::<Parameter<u64, S32>>(), 0x20);
        assert_eq!(size_of::<Parameter<u64, U32>>(), 0x20);
        assert_eq!(size_of::<Parameter<u64, Float>>(), 0x20);
        assert_eq!(size_of::<Parameter<u64, F32>>(), 0x20);
        assert_eq!(size_of::<Parameter<u64, Vector2f>>(), 0x20);
        assert_eq!(size_of::<Parameter<u64, Vector3f>>(), 0x28);
        assert_eq!(size_of::<Parameter<u64, Vector4f>>(), 0x28);
        assert_eq!(size_of::<Parameter<u64, SafeString<u64>>>(), 0x28);
        assert_eq!(size_of::<Parameter<u64, FixedSafeString32<u64>>>(), 0x40);
        assert_eq!(size_of::<Parameter<u64, FixedSafeString64<u64>>>(), 0x60);
        assert_eq!(size_of::<Parameter<u64, FixedSafeString256<u64>>>(), 0x120);
        assert_eq!(size_of::<SafeString<u64>>(), 0x10);
        assert_eq!(size_of::<FixedSafeString32<u64>>(), 0x28);
        assert_eq!(size_of::<FixedSafeString64<u64>>(), 0x48);
        assert_eq!(size_of::<FixedSafeString256<u64>>(), 0x108);
        assert_eq!(size_of::<SeadBuffer<u64>>(), 0x10);
    }
    #[cfg(feature = "complex")]
    #[test]
    fn baiprog_size_tests() {
        use std::mem::size_of;

        use crate::calc::cpp_memsizes::cpp_classes::AIProgram::*;
        assert_eq!(size_of::<AIActionDef<u32>>(), 0x6c);
        assert_eq!(size_of::<BehaviorDef<u32>>(), 0x54);
        assert_eq!(size_of::<QueryDef<u32>>(), 0x50);
        assert_eq!(size_of::<AIActionDef<u64>>(), 0xc8);
        assert_eq!(size_of::<BehaviorDef<u64>>(), 0xa0);
        assert_eq!(size_of::<QueryDef<u64>>(), 0x98);
    }
    #[cfg(feature = "complex")]
    #[test]
    fn baslist_size_tests() {
        use std::mem::size_of;

        use crate::calc::cpp_memsizes::cpp_classes::ASList::*;
        assert_eq!(size_of::<ASDefine<u32>>(), 0x58);
        assert_eq!(size_of::<CFPost<u32>>(), 0x54);
        assert_eq!(size_of::<CFExcept<u32>>(), 0x18);
        assert_eq!(size_of::<CFDefine<u32>>(), 0xa8);
        assert_eq!(size_of::<AddRes<u32>>(), 0x5c);
        assert_eq!(size_of::<Common<u32>>(), 0x2c);
        assert_eq!(size_of::<ASDefine<u64>>(), 0x88);
        assert_eq!(size_of::<CFPost<u64>>(), 0x98);
        assert_eq!(size_of::<CFExcept<u64>>(), 0x28);
        assert_eq!(size_of::<CFDefine<u64>>(), 0x138);
        assert_eq!(size_of::<AddRes<u64>>(), 0xa0);
        assert_eq!(size_of::<Common<u64>>(), 0x50);
    }
    #[cfg(feature = "complex")]
    #[test]
    fn bdrop_size_tests() {
        use std::mem::size_of;

        use crate::calc::cpp_memsizes::cpp_classes::DropTable::*;
        assert_eq!(size_of::<Table<u32>>(), 0x8c);
        assert_eq!(size_of::<Item<u32>>(), 0x28);
        assert_eq!(size_of::<Table<u64>>(), 0x108);
        assert_eq!(size_of::<Item<u64>>(), 0x48);
    }
    #[cfg(feature = "complex")]
    #[test]
    fn bgplobj_size_tests() {
        use std::mem::size_of;

        use crate::calc::cpp_memsizes::cpp_classes::GParamList::*;
        assert_eq!(size_of::<GParamListObjectAirWall<u32>>(), 0x38);
        assert_eq!(size_of::<GParamListObjectAnimalFollowOffset<u32>>(), 0x38);
        assert_eq!(size_of::<GParamListObjectAnimalUnit<u32>>(), 0x138);
        assert_eq!(size_of::<GParamListObjectArmor<u32>>(), 0xb0);
        assert_eq!(size_of::<GParamListObjectArmorEffect<u32>>(), 0x78);
        assert_eq!(size_of::<GParamListObjectArmorHead<u32>>(), 0x60);
        assert_eq!(size_of::<GParamListObjectArmorUpper<u32>>(), 0x80);
        assert_eq!(size_of::<GParamListObjectArrow<u32>>(), 0x80);
        assert_eq!(size_of::<GParamListObjectAttack<u32>>(), 0xd8);
        assert_eq!(size_of::<GParamListObjectAttackInterval<u32>>(), 0x80);
        assert_eq!(size_of::<GParamListObjectAutoGen<u32>>(), 0x60);
        assert_eq!(size_of::<GParamListObjectBeam<u32>>(), 0x30);
        assert_eq!(size_of::<GParamListObjectBindActor<u32>>(), 0x48);
        assert_eq!(size_of::<GParamListObjectBindBone<u32>>(), 0x68);
        assert_eq!(size_of::<GParamListObjectBow<u32>>(), 0x2e8);
        assert_eq!(size_of::<GParamListObjectBullet<u32>>(), 0x40);
        assert_eq!(size_of::<GParamListObjectCamera<u32>>(), 0xd0);
        assert_eq!(size_of::<GParamListObjectChemicalType<u32>>(), 0x50);
        assert_eq!(size_of::<GParamListObjectClothReaction<u32>>(), 0x170);
        assert_eq!(size_of::<GParamListObjectCookSpice<u32>>(), 0x70);
        assert_eq!(size_of::<GParamListObjectCureItem<u32>>(), 0x68);
        assert_eq!(size_of::<GParamListObjectEatTarget<u32>>(), 0xb0);
        assert_eq!(size_of::<GParamListObjectEnemy<u32>>(), 0x150);
        assert_eq!(size_of::<GParamListObjectEnemyLevel<u32>>(), 0xe0);
        assert_eq!(size_of::<GParamListObjectEnemyRace<u32>>(), 0x328);
        assert_eq!(size_of::<GParamListObjectEnemyShown<u32>>(), 0x60);
        assert_eq!(size_of::<GParamListObjectEvent<u32>>(), 0xe0);
        assert_eq!(size_of::<GParamListObjectExtendedEntity<u32>>(), 0x40);
        assert_eq!(size_of::<GParamListObjectFish<u32>>(), 0x70);
        assert_eq!(size_of::<GParamListObjectGelEnemy<u32>>(), 0x140);
        assert_eq!(size_of::<GParamListObjectGeneral<u32>>(), 0xb0);
        assert_eq!(size_of::<GParamListObjectGiantArmor<u32>>(), 0x48);
        assert_eq!(size_of::<GParamListObjectGiantArmorSlot<u32>>(), 0x140);
        assert_eq!(size_of::<GParamListObjectGlobal<u32>>(), 0x5f0);
        assert_eq!(size_of::<GParamListObjectGolem<u32>>(), 0xa8);
        assert_eq!(size_of::<GParamListObjectGolemIK<u32>>(), 0x1b0);
        assert_eq!(size_of::<GParamListObjectGrab<u32>>(), 0x140);
        assert_eq!(size_of::<GParamListObjectGuardian<u32>>(), 0xb8);
        assert_eq!(size_of::<GParamListObjectGuardianMini<u32>>(), 0xa8);
        assert_eq!(size_of::<GParamListObjectGuardianMiniWeapon<u32>>(), 0x98);
        assert_eq!(size_of::<GParamListObjectHorse<u32>>(), 0xf8);
        assert_eq!(size_of::<GParamListObjectHorseCreator<u32>>(), 0x50);
        assert_eq!(size_of::<GParamListObjectHorseObject<u32>>(), 0x40);
        assert_eq!(size_of::<GParamListObjectHorseRider<u32>>(), 0x198);
        assert_eq!(size_of::<GParamListObjectHorseTargetedInfo<u32>>(), 0x50);
        assert_eq!(size_of::<GParamListObjectHorseUnit<u32>>(), 0x60);
        assert_eq!(size_of::<GParamListObjectInsect<u32>>(), 0x30);
        assert_eq!(size_of::<GParamListObjectItem<u32>>(), 0x98);
        assert_eq!(size_of::<GParamListObjectLargeSword<u32>>(), 0x230);
        assert_eq!(size_of::<GParamListObjectLiftable<u32>>(), 0x198);
        assert_eq!(size_of::<GParamListObjectLumberjackTree<u32>>(), 0xa0);
        assert_eq!(size_of::<GParamListObjectMasterSword<u32>>(), 0xb0);
        assert_eq!(size_of::<GParamListObjectMonsterShop<u32>>(), 0x40);
        assert_eq!(size_of::<GParamListObjectMotorcycle<u32>>(), 0x280);
        assert_eq!(size_of::<GParamListObjectNest<u32>>(), 0x38);
        assert_eq!(size_of::<GParamListObjectNpc<u32>>(), 0x128);
        assert_eq!(size_of::<GParamListObjectNpcEquipment<u32>>(), 0x250);
        assert_eq!(size_of::<GParamListObjectPictureBook<u32>>(), 0x50);
        assert_eq!(size_of::<GParamListObjectPlayer<u32>>(), 0xac0);
        assert_eq!(size_of::<GParamListObjectPrey<u32>>(), 0x70);
        assert_eq!(size_of::<GParamListObjectRod<u32>>(), 0x118);
        assert_eq!(size_of::<GParamListObjectRope<u32>>(), 0xc0);
        assert_eq!(size_of::<GParamListObjectRupee<u32>>(), 0x30);
        assert_eq!(size_of::<GParamListObjectSandworm<u32>>(), 0x200);
        assert_eq!(size_of::<GParamListObjectSeriesArmor<u32>>(), 0x48);
        assert_eq!(size_of::<GParamListObjectShiekerStone<u32>>(), 0xb0);
        assert_eq!(size_of::<GParamListObjectShield<u32>>(), 0x1b8);
        assert_eq!(size_of::<GParamListObjectSmallSword<u32>>(), 0x230);
        assert_eq!(size_of::<GParamListObjectSpear<u32>>(), 0x290);
        assert_eq!(size_of::<GParamListObjectStalEnemy<u32>>(), 0x50);
        assert_eq!(size_of::<GParamListObjectSwarm<u32>>(), 0x58);
        assert_eq!(size_of::<GParamListObjectSystem<u32>>(), 0x48);
        assert_eq!(size_of::<GParamListObjectTraveler<u32>>(), 0x1cd0);
        assert_eq!(size_of::<GParamListObjectWeaponCommon<u32>>(), 0x328);
        assert_eq!(size_of::<GParamListObjectWeaponOption<u32>>(), 0xb0);
        assert_eq!(size_of::<GParamListObjectWeaponThrow<u32>>(), 0x68);
        assert_eq!(size_of::<GParamListObjectWizzrobe<u32>>(), 0xc0);
        assert_eq!(size_of::<GParamListObjectWolfLink<u32>>(), 0x440);
        assert_eq!(size_of::<GParamListObjectZora<u32>>(), 0x70);
        assert_eq!(size_of::<GParamListObjectAirWall<u64>>(), 0x60);
        assert_eq!(size_of::<GParamListObjectAnimalFollowOffset<u64>>(), 0x60);
        assert_eq!(size_of::<GParamListObjectAnimalUnit<u64>>(), 0x260);
        assert_eq!(size_of::<GParamListObjectArmor<u64>>(), 0x138);
        assert_eq!(size_of::<GParamListObjectArmorEffect<u64>>(), 0xe0);
        assert_eq!(size_of::<GParamListObjectArmorHead<u64>>(), 0xa8);
        assert_eq!(size_of::<GParamListObjectArmorUpper<u64>>(), 0xe8);
        assert_eq!(size_of::<GParamListObjectArrow<u64>>(), 0xf8);
        assert_eq!(size_of::<GParamListObjectAttack<u64>>(), 0x190);
        assert_eq!(size_of::<GParamListObjectAttackInterval<u64>>(), 0xf8);
        assert_eq!(size_of::<GParamListObjectAutoGen<u64>>(), 0xa8);
        assert_eq!(size_of::<GParamListObjectBeam<u64>>(), 0x58);
        assert_eq!(size_of::<GParamListObjectBindBone<u64>>(), 0xb0);
        assert_eq!(size_of::<GParamListObjectBow<u64>>(), 0x540);
        assert_eq!(size_of::<GParamListObjectBullet<u64>>(), 0x78);
        assert_eq!(size_of::<GParamListObjectCamera<u64>>(), 0x198);
        assert_eq!(size_of::<GParamListObjectChemicalType<u64>>(), 0x88);
        assert_eq!(size_of::<GParamListObjectClothReaction<u64>>(), 0x288);
        assert_eq!(size_of::<GParamListObjectCookSpice<u64>>(), 0xd8);
        assert_eq!(size_of::<GParamListObjectCureItem<u64>>(), 0xc0);
        assert_eq!(size_of::<GParamListObjectEatTarget<u64>>(), 0x128);
        assert_eq!(size_of::<GParamListObjectEnemy<u64>>(), 0x288);
        assert_eq!(size_of::<GParamListObjectEnemyLevel<u64>>(), 0x1b8);
        assert_eq!(size_of::<GParamListObjectEnemyRace<u64>>(), 0x590);
        assert_eq!(size_of::<GParamListObjectEnemyShown<u64>>(), 0xb8);
        assert_eq!(size_of::<GParamListObjectEvent<u64>>(), 0x178);
        assert_eq!(size_of::<GParamListObjectExtendedEntity<u64>>(), 0x78);
        assert_eq!(size_of::<GParamListObjectFish<u64>>(), 0xd8);
        assert_eq!(size_of::<GParamListObjectGelEnemy<u64>>(), 0x248);
        assert_eq!(size_of::<GParamListObjectGeneral<u64>>(), 0x148);
        assert_eq!(size_of::<GParamListObjectGiantArmor<u64>>(), 0x80);
        assert_eq!(size_of::<GParamListObjectGiantArmorSlot<u64>>(), 0x218);
        assert_eq!(size_of::<GParamListObjectGlobal<u64>>(), 0xb78);
        assert_eq!(size_of::<GParamListObjectGolem<u64>>(), 0x120);
        assert_eq!(size_of::<GParamListObjectGolemIK<u64>>(), 0x358);
        assert_eq!(size_of::<GParamListObjectGrab<u64>>(), 0x218);
        assert_eq!(size_of::<GParamListObjectGuardian<u64>>(), 0x160);
        assert_eq!(size_of::<GParamListObjectGuardianMini<u64>>(), 0x120);
        assert_eq!(size_of::<GParamListObjectGuardianMiniWeapon<u64>>(), 0x100);
        assert_eq!(size_of::<GParamListObjectHorse<u64>>(), 0x1d0);
        assert_eq!(size_of::<GParamListObjectHorseCreator<u64>>(), 0x88);
        assert_eq!(size_of::<GParamListObjectHorseRider<u64>>(), 0x2c0);
        assert_eq!(size_of::<GParamListObjectHorseTargetedInfo<u64>>(), 0x98);
        assert_eq!(size_of::<GParamListObjectHorseUnit<u64>>(), 0xb8);
        assert_eq!(size_of::<GParamListObjectInsect<u64>>(), 0x58);
        assert_eq!(size_of::<GParamListObjectItem<u64>>(), 0x120);
        assert_eq!(size_of::<GParamListObjectLargeSword<u64>>(), 0x3a8);
        assert_eq!(size_of::<GParamListObjectLiftable<u64>>(), 0x2e0);
        assert_eq!(size_of::<GParamListObjectLumberjackTree<u64>>(), 0x118);
        assert_eq!(size_of::<GParamListObjectMasterSword<u64>>(), 0x148);
        assert_eq!(size_of::<GParamListObjectMonsterShop<u64>>(), 0x78);
        assert_eq!(size_of::<GParamListObjectMotorcycle<u64>>(), 0x4e8);
        assert_eq!(size_of::<GParamListObjectNest<u64>>(), 0x60);
        assert_eq!(size_of::<GParamListObjectNpc<u64>>(), 0x230);
        assert_eq!(size_of::<GParamListObjectNpcEquipment<u64>>(), 0x3f8);
        assert_eq!(size_of::<GParamListObjectPictureBook<u64>>(), 0x98);
        assert_eq!(size_of::<GParamListObjectPlayer<u64>>(), 0x1578);
        assert_eq!(size_of::<GParamListObjectPrey<u64>>(), 0xd8);
        assert_eq!(size_of::<GParamListObjectRod<u64>>(), 0x220);
        assert_eq!(size_of::<GParamListObjectRope<u64>>(), 0x178);
        assert_eq!(size_of::<GParamListObjectRupee<u64>>(), 0x58);
        assert_eq!(size_of::<GParamListObjectSandworm<u64>>(), 0x388);
        assert_eq!(size_of::<GParamListObjectSeriesArmor<u64>>(), 0x80);
        assert_eq!(size_of::<GParamListObjectShiekerStone<u64>>(), 0x128);
        assert_eq!(size_of::<GParamListObjectShield<u64>>(), 0x2f0);
        assert_eq!(size_of::<GParamListObjectSmallSword<u64>>(), 0x3a8);
        assert_eq!(size_of::<GParamListObjectSpear<u64>>(), 0x448);
        assert_eq!(size_of::<GParamListObjectStalEnemy<u64>>(), 0x88);
        assert_eq!(size_of::<GParamListObjectSwarm<u64>>(), 0xa0);
        assert_eq!(size_of::<GParamListObjectSystem<u64>>(), 0x80);
        assert_eq!(size_of::<GParamListObjectTraveler<u64>>(), 0x3148);
        assert_eq!(size_of::<GParamListObjectWeaponCommon<u64>>(), 0x620);
        assert_eq!(size_of::<GParamListObjectWeaponOption<u64>>(), 0x128);
        assert_eq!(size_of::<GParamListObjectWeaponThrow<u64>>(), 0xc0);
        assert_eq!(size_of::<GParamListObjectWizzrobe<u64>>(), 0x168);
        assert_eq!(size_of::<GParamListObjectWolfLink<u64>>(), 0x878);
        assert_eq!(size_of::<GParamListObjectZora<u64>>(), 0xd8);
    }
    #[cfg(feature = "complex")]
    #[test]
    fn bmodellist_size_tests() {
        use std::mem::size_of;

        use crate::calc::cpp_memsizes::cpp_classes::ModelList::*;
        assert_eq!(size_of::<Unit<u32>>(), 0x4c);
        assert_eq!(size_of::<ModelData<u32>>(), 0x84);
        assert_eq!(size_of::<Partial<u32>>(), 0x54);
        assert_eq!(size_of::<AnmTarget<u32>>(), 0x9c);
        assert_eq!(size_of::<Unit<u64>>(), 0x80);
        assert_eq!(size_of::<ModelData<u64>>(), 0xf8);
        assert_eq!(size_of::<Partial<u64>>(), 0x98);
        assert_eq!(size_of::<AnmTarget<u64>>(), 0x130);
    }
    #[cfg(feature = "complex")]
    #[test]
    fn bphysics_size_tests() {
        use std::mem::size_of;

        use crate::calc::cpp_memsizes::cpp_classes::Physics::*;
        assert_eq!(size_of::<RigidBodySetParam<u32>>(), 0xa4);
        assert_eq!(size_of::<CharacterControllerParam<u32>>(), 0x2d4);
        assert_eq!(size_of::<ClothSetParam<u32>>(), 0x108);
        assert_eq!(size_of::<RagdollParam<u32>>(), 0x88);
        assert_eq!(size_of::<SupportBoneParam<u32>>(), 0x34);
        assert_eq!(size_of::<ContactInfoParam<u32>>(), 0x70);
        assert_eq!(size_of::<EdgeRigidBodySetParam<u32>>(), 0x2c);
        assert_eq!(size_of::<RigidBodyParam<u32>>(), 0x36c);
        assert_eq!(size_of::<Form<u32>>(), 0x88);
        assert_eq!(size_of::<ClothParam<u32>>(), 0xcc);
        assert_eq!(size_of::<ContactPointInfoParam<u32>>(), 0x8c);
        assert_eq!(size_of::<CollisionInfoParam<u32>>(), 0x7c);
        assert_eq!(size_of::<EdgeRigidBodyParam<u32>>(), 0x64);
        assert_eq!(size_of::<ShapeParamObj<u32>>(), 0x19c);
        assert_eq!(size_of::<RigidBodySetParam<u64>>(), 0x128);
        assert_eq!(size_of::<CharacterControllerParam<u64>>(), 0x4d8);
        assert_eq!(size_of::<ClothSetParam<u64>>(), 0x1b0);
        assert_eq!(size_of::<RagdollParam<u64>>(), 0xd0);
        assert_eq!(size_of::<SupportBoneParam<u64>>(), 0x58);
        assert_eq!(size_of::<ContactInfoParam<u64>>(), 0xd8);
        assert_eq!(size_of::<EdgeRigidBodySetParam<u64>>(), 0x58);
        assert_eq!(size_of::<RigidBodyParam<u64>>(), 0x640);
        assert_eq!(size_of::<Form<u64>>(), 0xe8);
        assert_eq!(size_of::<ClothParam<u64>>(), 0x180);
        assert_eq!(size_of::<ContactPointInfoParam<u64>>(), 0xd0);
        assert_eq!(size_of::<CollisionInfoParam<u64>>(), 0xb0);
        assert_eq!(size_of::<EdgeRigidBodyParam<u64>>(), 0xa8);
        assert_eq!(size_of::<ShapeParamObj<u64>>(), 0x278);
    }
    #[cfg(feature = "complex")]
    #[test]
    fn brecipe_size_tests() {
        use std::mem::size_of;

        use crate::calc::cpp_memsizes::cpp_classes::Recipe::*;
        assert_eq!(size_of::<Table<u32>>(), 0x4c);
        assert_eq!(size_of::<Item<u32>>(), 0x28);
        assert_eq!(size_of::<Table<u64>>(), 0x88);
        assert_eq!(size_of::<Item<u64>>(), 0x48);
    }
    #[cfg(feature = "complex")]
    #[test]
    fn bshop_size_tests() {
        use std::mem::size_of;

        use crate::calc::cpp_memsizes::cpp_classes::ShopData::*;
        assert_eq!(size_of::<Table<u32>>(), 0x4c);
        assert_eq!(size_of::<Item<u32>>(), 0x68);
        assert_eq!(size_of::<Table<u64>>(), 0x88);
        assert_eq!(size_of::<Item<u64>>(), 0xc8);
    }
    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_baiprog() {
        use std::collections::HashSet;

        use glob::glob;
        use roead::{aamp::ParameterIO, sarc};

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let root = "E:/Users/chodn/Documents/ISOs - WiiU/The Legend of Zelda Breath of the Wild \
                    (UPDATE DATA) (v208) (USA)/content";
        let rstb_path = root.to_owned() + "/System/Resource/ResourceSizeTable.product.srsizetable";
        let rstable = ResourceSizeTable::from_binary(std::fs::read(rstb_path).unwrap()).unwrap();
        for entry in glob(&(root.to_owned() + "/Actor/Pack/*.sbactorpack")).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let bxml = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                            .unwrap()
                            .unwrap(),
                    )
                    .unwrap();
                    let user = bxml
                        .param_root
                        .objects
                        .get("LinkTarget")
                        .unwrap()
                        .get("AIProgramUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/AIProgram/{}.baiprog", user);
                    if param_name.contains("Dummy") | result.contains(&param_name) {
                        continue;
                    }
                    if let Some(o_file) = sarc.get_data(&param_name).unwrap() {
                        if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                o_file,
                                &param_name,
                                Endian::Big,
                            )
                            .unwrap();
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
    }
    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_baslist() {
        use std::collections::HashSet;

        use glob::glob;
        use roead::{aamp::ParameterIO, sarc};

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let root = "E:/Users/chodn/Documents/ISOs - WiiU/The Legend of Zelda Breath of the Wild \
                    (UPDATE DATA) (v208) (USA)/content";
        let rstb_path = root.to_owned() + "/System/Resource/ResourceSizeTable.product.srsizetable";
        let rstable = ResourceSizeTable::from_binary(std::fs::read(rstb_path).unwrap()).unwrap();
        for entry in glob(&(root.to_owned() + "/Actor/Pack/*.sbactorpack")).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let bxml = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                            .unwrap()
                            .unwrap(),
                    )
                    .unwrap();
                    let user = bxml
                        .param_root
                        .objects
                        .get("LinkTarget")
                        .unwrap()
                        .get("ASUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/ASList/{}.baslist", user);
                    if param_name.contains("Dummy") | result.contains(&param_name) {
                        continue;
                    }
                    if let Some(o_file) = sarc.get_data(&param_name).unwrap() {
                        if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                o_file,
                                &param_name,
                                Endian::Big,
                            )
                            .unwrap();
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
    }
    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bdrop() {
        use std::collections::HashSet;

        use glob::glob;
        use roead::{aamp::ParameterIO, sarc};

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let root = "E:/Users/chodn/Documents/ISOs - WiiU/The Legend of Zelda Breath of the Wild \
                    (UPDATE DATA) (v208) (USA)/content";
        let rstb_path = root.to_owned() + "/System/Resource/ResourceSizeTable.product.srsizetable";
        let rstable = ResourceSizeTable::from_binary(std::fs::read(rstb_path).unwrap()).unwrap();
        for entry in glob(&(root.to_owned() + "/Actor/Pack/*.sbactorpack")).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let bxml = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                            .unwrap()
                            .unwrap(),
                    )
                    .unwrap();
                    let user = bxml
                        .param_root
                        .objects
                        .get("LinkTarget")
                        .unwrap()
                        .get("DropTableUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/DropTable/{}.bdrop", user);
                    if param_name.contains("Dummy") | result.contains(&param_name) {
                        continue;
                    }
                    if let Some(o_file) = sarc.get_data(&param_name).unwrap() {
                        if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                o_file,
                                &param_name,
                                Endian::Big,
                            )
                            .unwrap();
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
    }
    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bgparamlist() {
        use std::collections::HashSet;

        use glob::glob;
        use roead::{aamp::ParameterIO, sarc};

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let root = "E:/Users/chodn/Documents/ISOs - WiiU/The Legend of Zelda Breath of the Wild \
                    (UPDATE DATA) (v208) (USA)/content";
        let rstb_path = root.to_owned() + "/System/Resource/ResourceSizeTable.product.srsizetable";
        let rstable = ResourceSizeTable::from_binary(std::fs::read(rstb_path).unwrap()).unwrap();
        for entry in glob(&(root.to_owned() + "/Actor/Pack/*.sbactorpack")).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let bxml = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                            .unwrap()
                            .unwrap(),
                    )
                    .unwrap();
                    let user = bxml
                        .param_root
                        .objects
                        .get("LinkTarget")
                        .unwrap()
                        .get("GParamUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/GeneralParamList/{}.bgparamlist", user);
                    if param_name.contains("Dummy") | result.contains(&param_name) {
                        continue;
                    }
                    if let Some(o_file) = sarc.get_data(&param_name).unwrap() {
                        if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                o_file,
                                &param_name,
                                Endian::Big,
                            )
                            .unwrap();
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
    }
    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bmodellist() {
        use std::collections::HashSet;

        use glob::glob;
        use roead::{aamp::ParameterIO, sarc};

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let root = "E:/Users/chodn/Documents/ISOs - WiiU/The Legend of Zelda Breath of the Wild \
                    (UPDATE DATA) (v208) (USA)/content";
        let rstb_path = root.to_owned() + "/System/Resource/ResourceSizeTable.product.srsizetable";
        let rstable = ResourceSizeTable::from_binary(std::fs::read(rstb_path).unwrap()).unwrap();
        for entry in glob(&(root.to_owned() + "/Actor/Pack/*.sbactorpack")).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let bxml = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                            .unwrap()
                            .unwrap(),
                    )
                    .unwrap();
                    let user = bxml
                        .param_root
                        .objects
                        .get("LinkTarget")
                        .unwrap()
                        .get("ModelUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/ModelList/{}.bmodellist", user);
                    if param_name.contains("Dummy") | result.contains(&param_name) {
                        continue;
                    }
                    if let Some(o_file) = sarc.get_data(&param_name).unwrap() {
                        if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                o_file,
                                &param_name,
                                Endian::Big,
                            )
                            .unwrap();
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
    }
    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bphysics() {
        use std::collections::HashSet;

        use glob::glob;
        use roead::{aamp::ParameterIO, sarc};

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let root = "E:/Users/chodn/Documents/ISOs - WiiU/The Legend of Zelda Breath of the Wild \
                    (UPDATE DATA) (v208) (USA)/content";
        let rstb_path = root.to_owned() + "/System/Resource/ResourceSizeTable.product.srsizetable";
        let rstable = ResourceSizeTable::from_binary(std::fs::read(rstb_path).unwrap()).unwrap();
        for entry in glob(&(root.to_owned() + "/Actor/Pack/*.sbactorpack")).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let bxml = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                            .unwrap()
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
                    let param_name = format!("Actor/Physics/{}.bphysics", user);
                    if param_name.contains("Dummy") | result.contains(&param_name) {
                        continue;
                    }
                    if let Some(o_file) = sarc.get_data(&param_name).unwrap() {
                        if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                o_file,
                                &param_name,
                                Endian::Big,
                            )
                            .unwrap();
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
    }
    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_brecipe() {
        use std::collections::HashSet;

        use glob::glob;
        use roead::{aamp::ParameterIO, sarc};

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let root = "E:/Users/chodn/Documents/ISOs - WiiU/The Legend of Zelda Breath of the Wild \
                    (UPDATE DATA) (v208) (USA)/content";
        let rstb_path = root.to_owned() + "/System/Resource/ResourceSizeTable.product.srsizetable";
        let rstable = ResourceSizeTable::from_binary(std::fs::read(rstb_path).unwrap()).unwrap();
        for entry in glob(&(root.to_owned() + "/Actor/Pack/*.sbactorpack")).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let bxml = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                            .unwrap()
                            .unwrap(),
                    )
                    .unwrap();
                    let user = bxml
                        .param_root
                        .objects
                        .get("LinkTarget")
                        .unwrap()
                        .get("RecipeUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/Recipe/{}.brecipe", user);
                    if param_name.contains("Dummy") | result.contains(&param_name) {
                        continue;
                    }
                    if let Some(o_file) = sarc.get_data(&param_name).unwrap() {
                        if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                o_file,
                                &param_name,
                                Endian::Big,
                            )
                            .unwrap();
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
    }
    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bshop() {
        use std::collections::HashSet;

        use glob::glob;
        use roead::{aamp::ParameterIO, sarc};

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let root = "E:/Users/chodn/Documents/ISOs - WiiU/The Legend of Zelda Breath of the Wild \
                    (UPDATE DATA) (v208) (USA)/content";
        let rstb_path = root.to_owned() + "/System/Resource/ResourceSizeTable.product.srsizetable";
        let rstable = ResourceSizeTable::from_binary(std::fs::read(rstb_path).unwrap()).unwrap();
        for entry in glob(&(root.to_owned() + "/Actor/Pack/*.sbactorpack")).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let bxml = ParameterIO::from_binary(
                        sarc.get_data(&format!("Actor/ActorLink/{}.bxml", actorname))
                            .unwrap()
                            .unwrap(),
                    )
                    .unwrap();
                    let user = bxml
                        .param_root
                        .objects
                        .get("LinkTarget")
                        .unwrap()
                        .get("ShopDataUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/ShopData/{}.bshop", user);
                    if param_name.contains("Dummy") | result.contains(&param_name) {
                        continue;
                    }
                    if let Some(o_file) = sarc.get_data(&param_name).unwrap() {
                        if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                            let calc_size = super::estimate_from_bytes_and_name(
                                o_file,
                                &param_name,
                                Endian::Big,
                            )
                            .unwrap();
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
    }
    #[cfg(feature = "complex_testing")]
    #[test]
    fn test_all_bxml() {
        use std::collections::HashSet;

        use glob::glob;
        use roead::sarc;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let root = "E:/Users/chodn/Documents/ISOs - WiiU/The Legend of Zelda Breath of the Wild \
                    (UPDATE DATA) (v208) (USA)/content";
        let rstb_path = root.to_owned() + "/System/Resource/ResourceSizeTable.product.srsizetable";
        let rstable = ResourceSizeTable::from_binary(std::fs::read(rstb_path).unwrap()).unwrap();
        for entry in glob(&(root.to_owned() + "/Actor/Pack/*.sbactorpack")).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let param_name = format!("Actor/ActorLink/{}.bxml", actorname);
                    if param_name.contains("Dummy") | result.contains(&param_name) {
                        continue;
                    }
                    let bxml_bytes = sarc.get_data(&param_name).unwrap().unwrap();
                    if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                        let calc_size = super::estimate_from_bytes_and_name(
                            bxml_bytes,
                            &param_name,
                            Endian::Big,
                        )
                        .unwrap();
                        assert_ge!(calc_size, rstb_entry);
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
    fn test_all_bxml_nx() {
        use std::collections::HashSet;

        use glob::glob;
        use roead::sarc;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let root = "E:/Users/chodn/Documents/ISOs - Switch/LoZBOTW/content";
        let rstb_path = root.to_owned() + "/System/Resource/ResourceSizeTable.product.srsizetable";
        let rstable = ResourceSizeTable::from_binary(std::fs::read(rstb_path).unwrap()).unwrap();
        for entry in glob(&(root.to_owned() + "/Actor/Pack/*.sbactorpack")).unwrap() {
            match entry {
                Ok(path) => {
                    let actorname = path.file_stem().unwrap().to_str().unwrap();
                    let sarc = sarc::Sarc::new(std::fs::read(&path).unwrap()).unwrap();
                    let param_name = format!("Actor/ActorLink/{}.bxml", actorname);
                    if param_name.contains("Dummy") | result.contains(&param_name) {
                        continue;
                    }
                    let bxml_bytes = sarc.get_data(&param_name).unwrap().unwrap();
                    if let Some(rstb_entry) = rstable.get(param_name.as_str()) {
                        let calc_size = super::estimate_from_bytes_and_name(
                            bxml_bytes,
                            &param_name,
                            Endian::Little,
                        )
                        .unwrap();
                        assert_ge!(calc_size, rstb_entry);
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
    fn write_graphic_pack_rstb_from_formulas_only() {
        use std::{collections::HashSet, fs, path::Path};

        use glob::glob;
        use roead::{sarc, yaz0};

        use crate::ResourceSizeTable;
        let mut parsed: HashSet<String> = HashSet::new();

        let root = "D:/Program Files/cemu_1.16.1/graphicPacks/BreathOfTheWild_BCML/content";
        let rstb_str = &format!(
            "{}/System/Resource/ResourceSizeTable.product.srsizetable",
            root
        );
        let rstb_path = Path::new(rstb_str);

        let rstb_bak_str = &format!(
            "{}/System/Resource/ResourceSizeTable.product.srsizetable.bak",
            root
        );
        let rstb_backup = Path::new(rstb_bak_str);
        if !rstb_backup.exists() {
            println!("RSTB not backed up. Backing up...");
            fs::copy(rstb_path, rstb_backup).expect("Failed to back up RSTB");
        }
        let mut rstable = ResourceSizeTable::from_binary(fs::read(rstb_backup).unwrap()).unwrap();

        let titlebg =
            sarc::Sarc::new(fs::read(&format!("{}/Pack/TitleBG.pack", root)).unwrap()).unwrap();
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
                            if !rstable.contains(s_name) || parsed.contains(s_name) {
                                continue;
                            }
                            match Path::new(s_name).extension().unwrap().to_str().unwrap() {
                                "baiprog"
                                | "baslist"
                                | "bdrop"
                                | "bgparamlist"
                                | "bmodellist"
                                | "bphysics"
                                | "brecipe"
                                | "bshop"
                                | "bxml"
                                | "nonexistent_so_i_can_comment_out_bxml" => {
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

        for entry in glob(&(root.to_owned() + "/Model/*.Tex*.sbfres")).unwrap() {
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

        for entry in glob(&(root.to_owned() + "/Actor/Pack/*.sbactorpack")).unwrap() {
            match entry {
                Ok(path) => {
                    let param_name = format!(
                        "Actor/Pack/{}.bactorpack",
                        path.file_stem().unwrap().to_str().unwrap()
                    );
                    let data = fs::read(&path).unwrap();
                    if rstable.contains(param_name.as_str()) && !parsed.contains(&param_name) {
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
                        if !rstable.contains(s_name) || parsed.contains(s_name) {
                            continue;
                        }
                        match Path::new(s_name).extension().unwrap().to_str().unwrap() {
                            "baiprog"
                            | "baslist"
                            | "bdrop"
                            | "bgparamlist"
                            | "bmodellist"
                            | "bphysics"
                            | "brecipe"
                            | "bshop"
                            | "bxml"
                            | "nonexistent_so_i_can_comment_out_bxml" => {
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
