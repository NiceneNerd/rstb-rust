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

mod info;
#[cfg(feature = "complex")]
mod cpp_memsizes;

use crate::{Endian, Result};
use info::{get_factory_info, ParseSize};
#[cfg(feature = "complex")]
use cpp_memsizes::{baslist, bdrop, bgparamlist, bmodellist, bphysics, brecipe, bshop, bxml};
use std::path::Path;

#[inline]
fn round_32(size: usize) -> u32 {
    ((size as isize + 31) & -32) as u32
}

/// Infallibly calculate an RSTB value from a file on disk, returning `None` if
/// the type is not supported.
pub fn calc_from_file<P: AsRef<Path>>(file: P, endian: Endian) -> Result<Option<u32>> {
    Ok(calc_from_bytes_and_name(
        &std::fs::read(file.as_ref()).unwrap(),
        file.as_ref()
            .file_name()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Path not a file"))?
            .to_str()
            .unwrap(),
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
        calc_from_bytes_and_name(bytes, name, endian)
    }
}

/// Infallibly calculate an RSTB value from an uncompressed file size and
/// filename, returning `None` if the type is not supported.
pub fn calc_from_bytes_and_name(bytes: &[u8], name: &str, endian: Endian) -> Option<u32> {
    calc_or_estimate_from_bytes_and_name(bytes, name, endian, false)
}

/// Infallibly calculate *or* estimate an RSTB value from a file on disk,
/// returning `None` if the type is not supported.
pub fn estimate_from_file<P: AsRef<Path>>(file: P, endian: Endian) -> Result<Option<u32>> {
    Ok(estimate_from_bytes_and_name(
        &std::fs::read(file.as_ref()).unwrap(),
        file.as_ref()
            .file_name()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Path not a file"))?
            .to_str()
            .unwrap(),
        endian,
    ))
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
            b"Yaz0" => match endian {
                    Endian::Big => u32::from_be_bytes(bytes[4..8].try_into().unwrap()) as usize,
                    Endian::Little => u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize
                },
            _ => bytes.len()
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
            ParseSize::Simple(parse_size) => Some(match endian {
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
            }),
            ParseSize::Complex => {
                if estimate {
                    match ext {
                        "baniminfo" => Some(
                            ((rounded as f32 * (if filesize > 36864 { 1.5 } else { 4.0 })) as u32
                                + 0xe4
                                + 0x24c)
                                * match endian {
                                    Endian::Big => 1,
                                    Endian::Little => 2,
                                },
                        ),
                        "baslist" => Some(rounded + 0xe4 + 0x2f4 + baslist::parse_size(bytes, endian)),
                        "bdrop" => Some(rounded + 0xe4 + 0x27c + bdrop::parse_size(bytes, endian)),
                        "bfres" => Some(estimate_bfres(filesize, endian)),
                        "bgparamlist" => Some(rounded + 0xe4 + 0x248 + bgparamlist::parse_size(bytes, endian)),
                        "bmodellist" => Some(rounded + 0xe4 + 0x508 + bmodellist::parse_size(bytes, endian)),
                        "bphysics" => Some(rounded + 0xe4 + 0x324 + bphysics::parse_size(bytes, endian)),
                        "brecipe" => Some(rounded + 0xe4 + 0x27c + brecipe::parse_size(bytes, endian)),
                        "bshop" => Some(rounded + 0xe4 + 0x27c + bshop::parse_size(bytes, endian)),
                        "bxml" => Some(rounded + 0xe4 + 0x4a8 + bxml::parse_size(bytes)),
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
    let ext = &name[name.rfind('.').unwrap() + 1..];
    if ext == "bas" {
        size *= 1.05;
    };
    if ext == "bdmgparam" {
        size = (((-0.0018 * size) + 6.6273) * size) + 500.0
    } else if ext == "bphysics" {
        size = (round_32(size as usize) + 0x4E + 0x324) as f32
            * f32::max(4.0 * (size / 1388.0).floor(), 3.0)
    } else {
        size *= match ext {
            "baiprog" => match size as usize {
                (0..380) => 7.0,
                (380..400) => 6.0,
                (400..450) => 5.5,
                (450..600) => 5.0,
                (600..1_000) => 4.0,
                (1_000..1_750) => 3.5,
                _ => 3.0,
            },
            "bas" => match size as usize {
                (0..100) => 20.0,
                (100..200) => 12.5,
                (200..300) => 10.0,
                (300..600) => 8.0,
                (600..1_500) => 6.0,
                (1_500..2_000) => 5.5,
                (2_000..15_000) => 5.0,
                _ => 4.5,
            },
            "baslist" => match size as usize {
                (0..100) => 15.0,
                (100..200) => 10.0,
                (200..300) => 8.0,
                (300..500) => 6.0,
                (500..800) => 5.0,
                (800..4_000) => 4.0,
                _ => 3.5,
            },
            "bdrop" => match size as usize {
                (0..200) => 8.5,
                (200..250) => 7.0,
                (250..350) => 6.0,
                (350..450) => 5.25,
                (450..850) => 4.5,
                _ => 4.0,
            },
            "bgparamlist" => match size as usize {
                (0..100) => 20.0,
                (100..150) => 12.0,
                (150..250) => 10.0,
                (250..350) => 8.0,
                (350..450) => 7.0,
                _ => 6.0,
            },
            "brecipe" => match size as usize {
                (0..100) => 12.5,
                (100..160) => 8.5,
                (160..200) => 7.5,
                (200..215) => 7.0,
                _ => 6.5,
            },
            "bshop" => match size as usize {
                (0..200) => 7.25,
                (200..400) => 6.0,
                (400..500) => 5.0,
                _ => 4.05,
            },
            "bxml" => match size as usize {
                (0..350) => 6.0,
                (350..450) => 5.0,
                (450..550) => 4.5,
                (550..650) => 4.0,
                (650..800) => 3.5,
                _ => 3.0,
            },
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
    use all_asserts::assert_ge;

    use crate::Endian;
    use std::fs::read;

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
        assert_eq!(
            super::calc_from_slice_and_name(&buffer, "Enemy_Bokoblin_Gold.bdmgparam", Endian::Big),
            Some(5396)
        );
        let buffer: Vec<u8> = read("test/Obj_TreeWhiteBirch_A_01.hkrb").unwrap();
        assert_eq!(
            super::calc_from_slice_and_name(&buffer, "Obj_TreeWhiteBirch_A_01.hkrb", Endian::Big),
            Some(3560)
        );
        let buffer: Vec<u8> = read("test/savedataformat.ssarc").unwrap();
        assert_eq!(
            super::calc_from_slice_and_name(&buffer, "savedataformat.ssarc", Endian::Big),
            Some(2_801_216)
        );
    }

    #[test]
    fn estimate_sizes() {
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
        assert_eq!(
            super::estimate_from_slice_and_name(
                &std::fs::read("test/Armor_002_Upper.brecipe").unwrap(),
                "Actor/ModelList/Armor_002_Upper.brecipe",
                Endian::Big,
            ),
            Some(1276)
        );
        assert_eq!(
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
    #[test]
    fn random_tests() {
        use std::mem::size_of;
        use crate::calc::cpp_memsizes::cpp_classes::*;
        println!("size_of::<wiiu::agl::Parameter<S32>>(): {}", size_of::<agl::Parameter<u32, S32>>());
        println!("size_of::<nx::agl::Parameter<S32>>(): {}", size_of::<agl::Parameter<u64, S32>>());
    }
    #[test]
    fn bgplobj_size_tests() {
        use std::mem::size_of;
        use crate::calc::cpp_memsizes::cpp_classes::GParamList::*;
        println!("size_of::<GParamListObjectAirWall<u32>>(): {}", size_of::<GParamListObjectAirWall<u32>>());
        println!("size_of::<GParamListObjectAnimalFollowOffset<u32>>(): {}", size_of::<GParamListObjectAnimalFollowOffset<u32>>());
        println!("size_of::<GParamListObjectAnimalUnit<u32>>(): {}", size_of::<GParamListObjectAnimalUnit<u32>>());
        println!("size_of::<GParamListObjectArmor<u32>>(): {}", size_of::<GParamListObjectArmor<u32>>());
        println!("size_of::<GParamListObjectArmorEffect<u32>>(): {}", size_of::<GParamListObjectArmorEffect<u32>>());
        println!("size_of::<GParamListObjectArmorHead<u32>>(): {}", size_of::<GParamListObjectArmorHead<u32>>());
        println!("size_of::<GParamListObjectArmorUpper<u32>>(): {}", size_of::<GParamListObjectArmorUpper<u32>>());
        println!("size_of::<GParamListObjectArrow<u32>>(): {}", size_of::<GParamListObjectArrow<u32>>());
        println!("size_of::<GParamListObjectAttack<u32>>(): {}", size_of::<GParamListObjectAttack<u32>>());
        println!("size_of::<GParamListObjectAttackInterval<u32>>(): {}", size_of::<GParamListObjectAttackInterval<u32>>());
        println!("size_of::<GParamListObjectAutoGen<u32>>(): {}", size_of::<GParamListObjectAutoGen<u32>>());
        println!("size_of::<GParamListObjectBeam<u32>>(): {}", size_of::<GParamListObjectBeam<u32>>());
        println!("size_of::<GParamListObjectBindActor<u32>>(): {}", size_of::<GParamListObjectBindActor<u32>>());
        println!("size_of::<GParamListObjectBindBone<u32>>(): {}", size_of::<GParamListObjectBindBone<u32>>());
        println!("size_of::<GParamListObjectBow<u32>>(): {}", size_of::<GParamListObjectBow<u32>>());
        println!("size_of::<GParamListObjectBullet<u32>>(): {}", size_of::<GParamListObjectBullet<u32>>());
        println!("size_of::<GParamListObjectCamera<u32>>(): {}", size_of::<GParamListObjectCamera<u32>>());
        println!("size_of::<GParamListObjectChemicalType<u32>>(): {}", size_of::<GParamListObjectChemicalType<u32>>());
        println!("size_of::<GParamListObjectClothReaction<u32>>(): {}", size_of::<GParamListObjectClothReaction<u32>>());
        println!("size_of::<GParamListObjectCookSpice<u32>>(): {}", size_of::<GParamListObjectCookSpice<u32>>());
        println!("size_of::<GParamListObjectCureItem<u32>>(): {}", size_of::<GParamListObjectCureItem<u32>>());
        println!("size_of::<GParamListObjectEatTarget<u32>>(): {}", size_of::<GParamListObjectEatTarget<u32>>());
        println!("size_of::<GParamListObjectEnemy<u32>>(): {}", size_of::<GParamListObjectEnemy<u32>>());
        println!("size_of::<GParamListObjectEnemyLevel<u32>>(): {}", size_of::<GParamListObjectEnemyLevel<u32>>());
        println!("size_of::<GParamListObjectEnemyRace<u32>>(): {}", size_of::<GParamListObjectEnemyRace<u32>>());
        println!("size_of::<GParamListObjectEnemyShown<u32>>(): {}", size_of::<GParamListObjectEnemyShown<u32>>());
        println!("size_of::<GParamListObjectEvent<u32>>(): {}", size_of::<GParamListObjectEvent<u32>>());
        println!("size_of::<GParamListObjectExtendedEntity<u32>>(): {}", size_of::<GParamListObjectExtendedEntity<u32>>());
        println!("size_of::<GParamListObjectFish<u32>>(): {}", size_of::<GParamListObjectFish<u32>>());
        println!("size_of::<GParamListObjectGelEnemy<u32>>(): {}", size_of::<GParamListObjectGelEnemy<u32>>());
        println!("size_of::<GParamListObjectGeneral<u32>>(): {}", size_of::<GParamListObjectGeneral<u32>>());
        println!("size_of::<GParamListObjectGiantArmor<u32>>(): {}", size_of::<GParamListObjectGiantArmor<u32>>());
        println!("size_of::<GParamListObjectGiantArmorSlot<u32>>(): {}", size_of::<GParamListObjectGiantArmorSlot<u32>>());
        println!("size_of::<GParamListObjectGlobal<u32>>(): {}", size_of::<GParamListObjectGlobal<u32>>());
        println!("size_of::<GParamListObjectGolem<u32>>(): {}", size_of::<GParamListObjectGolem<u32>>());
        println!("size_of::<GParamListObjectGolemIK<u32>>(): {}", size_of::<GParamListObjectGolemIK<u32>>());
        println!("size_of::<GParamListObjectGrab<u32>>(): {}", size_of::<GParamListObjectGrab<u32>>());
        println!("size_of::<GParamListObjectGuardian<u32>>(): {}", size_of::<GParamListObjectGuardian<u32>>());
        println!("size_of::<GParamListObjectGuardianMini<u32>>(): {}", size_of::<GParamListObjectGuardianMini<u32>>());
        println!("size_of::<GParamListObjectGuardianMiniWeapon<u32>>(): {}", size_of::<GParamListObjectGuardianMiniWeapon<u32>>());
        println!("size_of::<GParamListObjectHorse<u32>>(): {}", size_of::<GParamListObjectHorse<u32>>());
        println!("size_of::<GParamListObjectHorseCreator<u32>>(): {}", size_of::<GParamListObjectHorseCreator<u32>>());
        println!("size_of::<GParamListObjectHorseObject<u32>>(): {}", size_of::<GParamListObjectHorseObject<u32>>());
        println!("size_of::<GParamListObjectHorseRider<u32>>(): {}", size_of::<GParamListObjectHorseRider<u32>>());
        println!("size_of::<GParamListObjectHorseTargetedInfo<u32>>(): {}", size_of::<GParamListObjectHorseTargetedInfo<u32>>());
        println!("size_of::<GParamListObjectHorseUnit<u32>>(): {}", size_of::<GParamListObjectHorseUnit<u32>>());
        println!("size_of::<GParamListObjectInsect<u32>>(): {}", size_of::<GParamListObjectInsect<u32>>());
        println!("size_of::<GParamListObjectItem<u32>>(): {}", size_of::<GParamListObjectItem<u32>>());
        println!("size_of::<GParamListObjectLargeSword<u32>>(): {}", size_of::<GParamListObjectLargeSword<u32>>());
        println!("size_of::<GParamListObjectLiftable<u32>>(): {}", size_of::<GParamListObjectLiftable<u32>>());
        println!("size_of::<GParamListObjectLumberjackTree<u32>>(): {}", size_of::<GParamListObjectLumberjackTree<u32>>());
        println!("size_of::<GParamListObjectMasterSword<u32>>(): {}", size_of::<GParamListObjectMasterSword<u32>>());
        println!("size_of::<GParamListObjectMonsterShop<u32>>(): {}", size_of::<GParamListObjectMonsterShop<u32>>());
        println!("size_of::<GParamListObjectMotorcycle<u32>>(): {}", size_of::<GParamListObjectMotorcycle<u32>>());
        println!("size_of::<GParamListObjectNest<u32>>(): {}", size_of::<GParamListObjectNest<u32>>());
        println!("size_of::<GParamListObjectNpc<u32>>(): {}", size_of::<GParamListObjectNpc<u32>>());
        println!("size_of::<GParamListObjectNpcEquipment<u32>>(): {}", size_of::<GParamListObjectNpcEquipment<u32>>());
        println!("size_of::<GParamListObjectPictureBook<u32>>(): {}", size_of::<GParamListObjectPictureBook<u32>>());
        println!("size_of::<GParamListObjectPlayer<u32>>(): {}", size_of::<GParamListObjectPlayer<u32>>());
        println!("size_of::<GParamListObjectPrey<u32>>(): {}", size_of::<GParamListObjectPrey<u32>>());
        println!("size_of::<GParamListObjectRod<u32>>(): {}", size_of::<GParamListObjectRod<u32>>());
        println!("size_of::<GParamListObjectRope<u32>>(): {}", size_of::<GParamListObjectRope<u32>>());
        println!("size_of::<GParamListObjectRupee<u32>>(): {}", size_of::<GParamListObjectRupee<u32>>());
        println!("size_of::<GParamListObjectSandworm<u32>>(): {}", size_of::<GParamListObjectSandworm<u32>>());
        println!("size_of::<GParamListObjectSeriesArmor<u32>>(): {}", size_of::<GParamListObjectSeriesArmor<u32>>());
        println!("size_of::<GParamListObjectShiekerStone<u32>>(): {}", size_of::<GParamListObjectShiekerStone<u32>>());
        println!("size_of::<GParamListObjectShield<u32>>(): {}", size_of::<GParamListObjectShield<u32>>());
        println!("size_of::<GParamListObjectSmallSword<u32>>(): {}", size_of::<GParamListObjectSmallSword<u32>>());
        println!("size_of::<GParamListObjectSpear<u32>>(): {}", size_of::<GParamListObjectSpear<u32>>());
        println!("size_of::<GParamListObjectStalEnemy<u32>>(): {}", size_of::<GParamListObjectStalEnemy<u32>>());
        println!("size_of::<GParamListObjectSwarm<u32>>(): {}", size_of::<GParamListObjectSwarm<u32>>());
        println!("size_of::<GParamListObjectSystem<u32>>(): {}", size_of::<GParamListObjectSystem<u32>>());
        println!("size_of::<GParamListObjectTraveler<u32>>(): {}", size_of::<GParamListObjectTraveler<u32>>());
        println!("size_of::<GParamListObjectWeaponCommon<u32>>(): {}", size_of::<GParamListObjectWeaponCommon<u32>>());
        println!("size_of::<GParamListObjectWeaponOption<u32>>(): {}", size_of::<GParamListObjectWeaponOption<u32>>());
        println!("size_of::<GParamListObjectWeaponThrow<u32>>(): {}", size_of::<GParamListObjectWeaponThrow<u32>>());
        println!("size_of::<GParamListObjectWizzrobe<u32>>(): {}", size_of::<GParamListObjectWizzrobe<u32>>());
        println!("size_of::<GParamListObjectWolfLink<u32>>(): {}", size_of::<GParamListObjectWolfLink<u32>>());
        println!("size_of::<GParamListObjectZora<u32>>(): {}", size_of::<GParamListObjectZora<u32>>());
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
}
