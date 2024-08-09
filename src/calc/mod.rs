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

#[cfg(feature = "complex_testing")]
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
            "Tex.sbfres" => "Tex.bfres",
            "Tex1.sbfres" => "Tex1.bfres",
            "Tex2.sbfres" => "Tex2.bfres",
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
                        "baiprog" => Some(rounded + baiprog::parse_size(bytes, endian)?),
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
                        "baslist" => Some(rounded + baslist::parse_size(bytes, endian)?),
                        #[cfg(feature = "complex")]
                        "bdrop" => Some(rounded + bdrop::parse_size(bytes, endian)?),
                        "bfres" => Some(estimate_bfres(filesize, endian)),
                        #[cfg(feature = "complex")]
                        "bgparamlist" => Some(rounded + bgparamlist::parse_size(bytes, endian)?),
                        #[cfg(feature = "complex")]
                        "bmodellist" => Some(rounded + bmodellist::parse_size(bytes, endian)?),
                        #[cfg(feature = "complex")]
                        "bphysics" => Some(rounded + bphysics::parse_size(bytes, endian)?),
                        #[cfg(feature = "complex")]
                        "brecipe" => Some(rounded + brecipe::parse_size(bytes, endian)?),
                        #[cfg(feature = "complex")]
                        "bshop" => Some(rounded + bshop::parse_size(bytes, endian)?),
                        #[cfg(feature = "complex")]
                        "bxml" => Some(rounded + bxml::parse_size(bytes, endian)?),
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
    #[test]
    fn test_all_baiprog() {
        use std::collections::HashSet;
        use dirs2;
        use ryml::Tree;
        use roead::{aamp::ParameterIO, sarc};

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

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
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                root.join("Actor")
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
                        .get("AIProgramUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/AIProgram/{}.baiprog", user);
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
        use dirs2;
        use ryml::Tree;
        use roead::{aamp::ParameterIO, sarc};

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

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
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                root.join("Actor")
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
                        .get("ASUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/ASList/{}.baslist", user);
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
        use dirs2;
        use ryml::Tree;
        use roead::{aamp::ParameterIO, sarc};

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

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
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                root.join("Actor")
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
                        .get("DropTableUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/DropTable/{}.bdrop", user);
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
        use dirs2;
        use ryml::Tree;
        use roead::{aamp::ParameterIO, sarc};

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

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
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                root.join("Actor")
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
                        .get("GParamUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/GeneralParamList/{}.bgparamlist", user);
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
        use dirs2;
        use ryml::Tree;
        use roead::{aamp::ParameterIO, sarc};

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

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
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                root.join("Actor")
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
                        .get("ModelUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/ModelList/{}.bmodellist", user);
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
        use dirs2;
        use ryml::Tree;
        use roead::{aamp::ParameterIO, sarc};

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

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
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                root.join("Actor")
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
                    let param_name = format!("Actor/Physics/{}.bphysics", user);
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
        use dirs2;
        use ryml::Tree;
        use roead::{aamp::ParameterIO, sarc};

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

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
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                root.join("Actor")
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
                        .get("RecipeUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/Recipe/{}.brecipe", user);
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
        use dirs2;
        use ryml::Tree;
        use roead::{aamp::ParameterIO, sarc};

        use glob::glob;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

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
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                root.join("Actor")
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
                        .get("ShopDataUser")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let param_name = format!("Actor/ShopData/{}.bshop", user);
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
        use dirs2;
        use ryml::Tree;

        use glob::glob;
        use roead::sarc;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

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
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                root.join("Actor")
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
        use dirs2;
        use ryml::Tree;

        use glob::glob;
        use roead::sarc;

        use crate::ResourceSizeTable;
        let mut result: HashSet<String> = HashSet::new();

        let settings_path = dirs2::data_dir()
            .unwrap()
            .join("ukmm")
            .join("settings.yml");
        let settings = Tree::parse(
                std::fs::read_to_string(settings_path).unwrap()
            ).unwrap();
        let profile_node = settings.root_ref()
            .unwrap()
            .get("switch_config")
            .unwrap()
            .get("profile")
            .unwrap();
        let profile = profile_node.val().unwrap();
        let root = dirs2::data_local_dir()
            .unwrap()
            .join("ukmm")
            .join("nx")
            .join("profiles")
            .join(profile)
            .join("merged")
            .join("01007EF00011E000")
            .join("romfs");
        let rstb_path = root
            .join("System")
            .join("Resource")
            .join("ResourceSizeTable.product.srsizetable");
        let rstable = ResourceSizeTable::from_binary(
                std::fs::read(rstb_path).unwrap()
            ).unwrap();
        for entry in glob(
                root.join("Actor")
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
                        println!("{} - vanilla: {} calc: {}", actorname, rstb_entry, calc_size);
                        //assert_ge!(calc_size, rstb_entry);
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
