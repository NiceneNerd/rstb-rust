use roead::aamp::ParameterIO;
use crate::Endian;

use std::mem::size_of;
use phf::{Map, phf_map};

use super::{
    cpp_align,
    WiiUParameterObj,
    WiiUParameter,
    NXParameterObj,
    NXParameter,
    Bool32,
    S32,
    F32,
    WiiUSafeString,
    NXSafeString,
    Vector3f
};

const BGPARAM_OVERHEAD_WIIU: u32 = 0x2c0;

pub fn parse_size(bytes: &[u8], endian: Endian) -> u32 {
    match endian {
        Endian::Big => parse_size_wiiu(bytes),
        Endian::Little => parse_size_nx(bytes),
    }
}

static OBJ_SIZES_WIIU: Map<&'static str, u32> = phf_map! {
    "AirWall" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "AnimalFollowOffset" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "AnimalUnit" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Armor" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "ArmorEffect" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "ArmorHead" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "ArmorUpper" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Arrow" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Attack" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "AttackInterval" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "AutoGen" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Beam" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "BindActor" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "BindBone" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Bow" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Bullet" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Camera" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "ChemicalType" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "ClothReaction" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "CookSpice" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "CureItem" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "EatTarget" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Enemy" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "EnemyLevel" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "EnemyRace" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "EnemyShown" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Event" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "ExtendedEntity" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Fish" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "GelEnemy" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "General" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "GiantArmor" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "GiantArmorSlot" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Global" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<F32>>() as u32, // mEnemyLifeGageDist
        size_of::<WiiUParameter<F32>>() as u32, // mEnemyNoSkitDist
        size_of::<WiiUParameter<F32>>() as u32, // mEnemyWeaponPickAllowDist
        size_of::<WiiUParameter<S32>>() as u32, // mEnemyWeaponPickForbidTime
        size_of::<WiiUParameter<F32>>() as u32, // mEnemyAnimalNoDamageDist
        size_of::<WiiUParameter<F32>>() as u32, // mEnemyNearCraeteIDDelay
        size_of::<WiiUParameter<S32>>() as u32, // mEnemyForceTiredLODCount
        size_of::<WiiUParameter<S32>>() as u32, // mEnemyForceTiredNoSightLODCount
        size_of::<WiiUParameter<S32>>() as u32, // mEnemyForceWarpReturnLODCount
        size_of::<WiiUParameter<F32>>() as u32, // mSilentAttackAng
        size_of::<WiiUParameter<F32>>() as u32, // mSilentAttackRatio
        size_of::<WiiUParameter<S32>>() as u32, // mBlownOffPlayerAtkDelay
        size_of::<WiiUParameter<F32>>() as u32, // mJustAvoidAcceptWpRangeSS
        size_of::<WiiUParameter<F32>>() as u32, // mJustAvoidAcceptWpRangeLS
        size_of::<WiiUParameter<F32>>() as u32, // mJustAvoidAcceptWpRangeSP
        size_of::<WiiUParameter<S32>>() as u32, // mForceNoticeEnemyCount
        size_of::<WiiUParameter<F32>>() as u32, // mForceNoticeEnemyDist
        size_of::<WiiUParameter<S32>>() as u32, // mWeaponRickeyLife
        size_of::<WiiUParameter<F32>>() as u32, // mWeaponDropRotSpd
        size_of::<WiiUParameter<S32>>() as u32, // mShieldRideBaseFrame
        size_of::<WiiUParameter<S32>>() as u32, // mShieldRideHitBaseDamage
        size_of::<WiiUParameter<F32>>() as u32, // mShieldDamageratio
        size_of::<WiiUParameter<F32>>() as u32, // mShieldSurfMasterFrictionRatio
        size_of::<WiiUParameter<F32>>() as u32, // mLoudNoiseRadius
        size_of::<WiiUParameter<F32>>() as u32, // mImpulse2DamageRatio
        size_of::<WiiUParameter<F32>>() as u32, // mIceMeltSpeedOnContactFire
        size_of::<WiiUParameter<F32>>() as u32, // mCriticalAttackRatio
        size_of::<WiiUParameter<F32>>() as u32, // mBooerangAttackRatio
        size_of::<WiiUParameter<F32>>() as u32, // mHitImpulseClampMax
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemVelXZFromBomb
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemVelYFromBomb
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemVelRandomFromBomb
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemAngVelFromBomb
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemAngVelRandomFromBomb
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemVelXZSmall
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemVelYSmall
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemVelRandomSmall
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemAngVelSmall
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemAngVelRandomSmall
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemVelXZLarge
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemVelYLarge
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemVelRandomLarge
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemAngVelLarge
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemAngVelRandomLarge
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemVelXZRupeeRabbit
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemVelYRupeeRabbit
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemVelRandomRupeeRabbit
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemVelXZItemRupeeOnly
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemVelYItemRupeeOnly
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemVelRandomItemRupeeOnly
        size_of::<WiiUParameter<F32>>() as u32, // mDropItemInvincibleTime
        size_of::<WiiUParameter<Vector3f>>() as u32, // mTreeWeaponEquipTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mTreeWeaponEquipRotOffset
        size_of::<WiiUParameter<F32>>() as u32, // mWetRatioToDie
        size_of::<WiiUParameter<F32>>() as u32, // mEnvWetRatioToDie
        size_of::<WiiUParameter<F32>>() as u32, // mNPCTurnAngleDiff
        size_of::<WiiUParameter<S32>>() as u32, // mNPCWaitFrameAfterEvent
        size_of::<WiiUParameter<F32>>() as u32, // mNPCIgnorePlayerTime
        size_of::<WiiUParameter<F32>>() as u32, // mNPCCancelIgnorePlayerTime
        size_of::<WiiUParameter<F32>>() as u32, // mNPCOpenDoorDistance
        size_of::<WiiUParameter<F32>>() as u32, // mNPCWalkRateOnSandAndSnow
        size_of::<WiiUParameter<F32>>() as u32, // mNPCDownVerticallyAngle
        size_of::<WiiUParameter<F32>>() as u32, // mGerudoQueenSafetyAreaRadius
        size_of::<WiiUParameter<S32>>() as u32, // mCreateFairyLimitCount
        size_of::<WiiUParameter<F32>>() as u32, // mTerrorRegistSpeed
        size_of::<WiiUParameter<F32>>() as u32, // mTerrorUnregistSpeed
        size_of::<WiiUParameter<S32>>() as u32, // mTerrorRegistTimer
        size_of::<WiiUParameter<F32>>() as u32, // mTerrorRadiusOffset
        size_of::<WiiUParameter<S32>>() as u32, // mSpeedTerrorLevel
        size_of::<WiiUParameter<S32>>() as u32, // mSpeedTerrorLevelHuge
        size_of::<WiiUParameter<F32>>() as u32, // mSpeedTerrorLevelCheckRadius
        size_of::<WiiUParameter<F32>>() as u32, // mAtDirTypeAffectRatio
        size_of::<WiiUParameter<F32>>() as u32, // mRainyAwnHearingLevel
        size_of::<WiiUParameter<F32>>() as u32, // mHorseBindOffsetYOfMaleUMii
        size_of::<WiiUParameter<F32>>() as u32, // mHorseBindOffsetYOfFemaleUMii
        size_of::<WiiUParameter<Vector3f>>() as u32, // mHorseFamiliarityIncreasePerFrame
        size_of::<WiiUParameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheAtFirstRun
        size_of::<WiiUParameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheAfterRun
        size_of::<WiiUParameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheAfterGearTop
        size_of::<WiiUParameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheAfterJump
        size_of::<WiiUParameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheWhileResisting
        size_of::<WiiUParameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseEat
        size_of::<WiiUParameter<Vector3f>>() as u32, // mHorseAlertProbability
        size_of::<WiiUParameter<Vector3f>>() as u32, // mHorseAlertFramesMin
        size_of::<WiiUParameter<Vector3f>>() as u32, // mHorseAlertFramesMax
        size_of::<WiiUParameter<S32>>() as u32, // mHorseExtraChargeNum
        size_of::<WiiUParameter<F32>>() as u32, // mPlayerGrabThrowDiffRate
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Golem" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "GolemIK" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Grab" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Guardian" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "GuardianMini" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "GuardianMiniWeapon" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Horse" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "HorseCreator" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "HorseObject" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "HorseRider" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "HorseTargetedInfo" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "HorseUnit" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Insect" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Item" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "LargeSword" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Liftable" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "LumberjackTree" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "MasterSword" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "MonsterShop" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Motorcycle" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<F32>>() as u32, // mPitchDampingCoefficient
        size_of::<WiiUParameter<F32>>() as u32, // mDriftAllowSpeedKPH
        size_of::<WiiUParameter<F32>>() as u32, // mDriftAbortSpeedKPH
        size_of::<WiiUParameter<F32>>() as u32, // mDriftAllowSteerRate
        size_of::<WiiUParameter<F32>>() as u32, // mDriftAbortSteerRate
        size_of::<WiiUParameter<F32>>() as u32, // mDriftRearAngleRate
        size_of::<WiiUParameter<F32>>() as u32, // mDriftSpeedRate
        size_of::<WiiUParameter<F32>>() as u32, // mManualWheelieAllowAngleFront
        size_of::<WiiUParameter<F32>>() as u32, // mManualWheelieAllowAngleRear
        size_of::<WiiUParameter<F32>>() as u32, // mManualWheelieLastSec
        size_of::<WiiUParameter<F32>>() as u32, // mWheelieLastSecInMidAir
        size_of::<WiiUParameter<F32>>() as u32, // mManualControlProhibitSecAfterWheelie
        size_of::<WiiUParameter<F32>>() as u32, // mWheelieRevertPower
        size_of::<WiiUParameter<F32>>() as u32, // mWheelieRevertPowerSec
        size_of::<WiiUParameter<F32>>() as u32, // mManualWheelieRiseDegDelta
        size_of::<WiiUParameter<F32>>() as u32, // mWheelieLaunchRiseDegDelta
        size_of::<WiiUParameter<F32>>() as u32, // mEngineBrakeMaxPower
        size_of::<WiiUParameter<F32>>() as u32, // mBackwardEngineBrakePower
        size_of::<WiiUParameter<F32>>() as u32, // mSlipStartAngle
        size_of::<WiiUParameter<F32>>() as u32, // mSlipThresholdPower
        size_of::<WiiUParameter<F32>>() as u32, // mSlipPowerMax
        size_of::<WiiUParameter<Vector3f>>() as u32, // mWristBindRotation
        size_of::<WiiUParameter<Vector3f>>() as u32, // mWristBindTranslation
        size_of::<WiiUParameter<F32>>() as u32, // mPostureLimitAngle
        size_of::<WiiUParameter<F32>>() as u32, // mInvalidPostureLimitSec
        size_of::<WiiUParameter<F32>>() as u32, // mFallOverThresholdAngle
        size_of::<WiiUParameter<F32>>() as u32, // mJumpIntervalSec
        size_of::<WiiUParameter<F32>>() as u32, // mFullEnergyLastSec
        size_of::<WiiUParameter<F32>>() as u32, // mWheelieLaunchJumpProhibitSec
        size_of::<WiiUParameter<F32>>() as u32, // mSlowModeTargetSpeedKPH2
        size_of::<WiiUParameter<F32>>() as u32, // mSlowDriftTargetSpeedKPH2
        size_of::<WiiUParameter<F32>>() as u32, // mSlowModeTransitionSec
        size_of::<WiiUParameter<F32>>() as u32, // mSlowSlipThresholdKPH
        size_of::<WiiUParameter<F32>>() as u32, // mSlowSlipThresholdPower
        size_of::<WiiUParameter<F32>>() as u32, // mSlowSlipThresholdSec
        size_of::<WiiUParameter<F32>>() as u32, // mJumpRearWheelRotateRadPerSec
        size_of::<WiiUParameter<F32>>() as u32, // mWeaponThrowModeSpeedKPH2
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Nest" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Npc" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "NpcEquipment" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "PictureBook" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Player" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<F32>>() as u32, // mBombReloadTime1
        size_of::<WiiUParameter<F32>>() as u32, // mBombReloadTime2
        size_of::<WiiUParameter<F32>>() as u32, // mStopTimerReloadTime
        size_of::<WiiUParameter<F32>>() as u32, // mStopTimerBlowAngle
        size_of::<WiiUParameter<F32>>() as u32, // mStopTimerBlowSpeedLimit
        size_of::<WiiUParameter<S32>>() as u32, // mStopTimerImpluseMaxCountSmallSword
        size_of::<WiiUParameter<S32>>() as u32, // mStopTimerImpluseMaxCountLargeSword
        size_of::<WiiUParameter<S32>>() as u32, // mStopTimerImpluseMaxCountSpear
        size_of::<WiiUParameter<F32>>() as u32, // mStopTimerCancelDeleteWaitTime
        size_of::<WiiUParameter<F32>>() as u32, // mStopTimerLongTime
        size_of::<WiiUParameter<F32>>() as u32, // mStopTimerMiddleTime
        size_of::<WiiUParameter<F32>>() as u32, // mStopTimerShortTime
        size_of::<WiiUParameter<F32>>() as u32, // mEnergyTiredValue
        size_of::<WiiUParameter<F32>>() as u32, // mEnergyBowSlow
        size_of::<WiiUParameter<F32>>() as u32, // mEnergyPush
        size_of::<WiiUParameter<F32>>() as u32, // mEnergyCharge
        size_of::<WiiUParameter<F32>>() as u32, // mEnergyAutoRecover
        size_of::<WiiUParameter<F32>>() as u32, // mEnergyAutoRecoverInAir
        size_of::<WiiUParameter<F32>>() as u32, // mEnergyAutoRecoverInvalidTime1
        size_of::<WiiUParameter<F32>>() as u32, // mEnergyAutoRecoverInvalidTime2
        size_of::<WiiUParameter<F32>>() as u32, // mColdTempDamageAdd
        size_of::<WiiUParameter<F32>>() as u32, // mHotTempDamageAdd
        size_of::<WiiUParameter<F32>>() as u32, // mTempDamage
        size_of::<WiiUParameter<F32>>() as u32, // mTempEnergyDecDiamAdd
        size_of::<WiiUParameter<F32>>() as u32, // mTempEnergyDecDegAdd
        size_of::<WiiUParameter<F32>>() as u32, // mVelDiamSand
        size_of::<WiiUParameter<F32>>() as u32, // mVelDiamTired
        size_of::<WiiUParameter<F32>>() as u32, // mStickDiamTired
        size_of::<WiiUParameter<F32>>() as u32, // mAutoRecoverNum
        size_of::<WiiUParameter<F32>>() as u32, // mAutoRecoverIntervalMin
        size_of::<WiiUParameter<F32>>() as u32, // mAutoRecoverIntervalMax
        size_of::<WiiUParameter<F32>>() as u32, // mAutoRecoverInvalidTime
        size_of::<WiiUParameter<F32>>() as u32, // mBowSubjectContTime
        size_of::<WiiUParameter<F32>>() as u32, // mLNGStickScale
        size_of::<WiiUParameter<F32>>() as u32, // mLATStickScale
        size_of::<WiiUParameter<F32>>() as u32, // mLNGGyroScale
        size_of::<WiiUParameter<F32>>() as u32, // mLATGyroScale
        size_of::<WiiUParameter<S32>>() as u32, // mBowSlowShootNum
        size_of::<WiiUParameter<F32>>() as u32, // mBowSlowRateDiam
        size_of::<WiiUParameter<F32>>() as u32, // mBowSlowMaxTime
        size_of::<WiiUParameter<F32>>() as u32, // mDiveBowSlowMaxTime
        size_of::<WiiUParameter<F32>>() as u32, // mBowSlowInvalidTime
        size_of::<WiiUParameter<F32>>() as u32, // mBowSlowInvalidHeight
        size_of::<WiiUParameter<F32>>() as u32, // mBowSlowInvalidHeightOnShield
        size_of::<WiiUParameter<F32>>() as u32, // mBowSlowInvalidHeightWeaponChange
        size_of::<WiiUParameter<F32>>() as u32, // mGuardJustForceSlowTime
        size_of::<WiiUParameter<F32>>() as u32, // mMoveMaxDecRateByWater
        size_of::<WiiUParameter<F32>>() as u32, // mMoveIgnoreWaterHeight
        size_of::<WiiUParameter<F32>>() as u32, // mMoveDecRateByBog
        size_of::<WiiUParameter<F32>>() as u32, // mMoveDecRateMaxHeight
        size_of::<WiiUParameter<F32>>() as u32, // mMaxForce
        size_of::<WiiUParameter<F32>>() as u32, // mMinForce
        size_of::<WiiUParameter<F32>>() as u32, // mAddForce
        size_of::<WiiUParameter<F32>>() as u32, // mSnowBallAddForce
        size_of::<WiiUParameter<F32>>() as u32, // mLogPushF
        size_of::<WiiUParameter<F32>>() as u32, // mRockPushF
        size_of::<WiiUParameter<F32>>() as u32, // mRockPushSpeed
        size_of::<WiiUParameter<F32>>() as u32, // mWaistAngleUpperMax
        size_of::<WiiUParameter<F32>>() as u32, // mWaistAngleLowerMax
        size_of::<WiiUParameter<F32>>() as u32, // mWaistAngleSideMax
        size_of::<WiiUParameter<F32>>() as u32, // mNoSquatWaterHeight
        size_of::<WiiUParameter<F32>>() as u32, // mInvalidReloadTime
        size_of::<WiiUParameter<F32>>() as u32, // mWeaponThrowSpeedY
        size_of::<WiiUParameter<F32>>() as u32, // mWeaponThrowSpeedF
        size_of::<WiiUParameter<F32>>() as u32, // mWeaponThrowSpeedFSquat
        size_of::<WiiUParameter<F32>>() as u32, // mDashUpEnableAngle
        size_of::<WiiUParameter<F32>>() as u32, // mShockTime
        size_of::<WiiUParameter<F32>>() as u32, // mIceInvalidTime
        size_of::<WiiUParameter<F32>>() as u32, // mMaxSpeedInAir
        size_of::<WiiUParameter<F32>>() as u32, // mTurnEnableSpeed
        size_of::<WiiUParameter<F32>>() as u32, // mTurnEnableStickSub
        size_of::<WiiUParameter<F32>>() as u32, // mTurnEnableDirSub
        size_of::<WiiUParameter<S32>>() as u32, // mShortDashImpulse
        size_of::<WiiUParameter<S32>>() as u32, // mShortDashDamage
        size_of::<WiiUParameter<F32>>() as u32, // mSwordTerrorScope
        size_of::<WiiUParameter<F32>>() as u32, // mArrowTerrorScope
        size_of::<WiiUParameter<F32>>() as u32, // mTorchTerrorScope
        size_of::<WiiUParameter<F32>>() as u32, // mTorchTerrorOffsetY
        size_of::<WiiUParameter<F32>>() as u32, // mTorchTerrorOffsetZ
        size_of::<WiiUParameter<F32>>() as u32, // mDashNoise
        size_of::<WiiUParameter<F32>>() as u32, // mWhistleNoise
        size_of::<WiiUParameter<F32>>() as u32, // mClimbEnableAngle
        size_of::<WiiUParameter<F32>>() as u32, // mClimbEnableSpeedMinAngle
        size_of::<WiiUParameter<F32>>() as u32, // mClimbEnableSpeedMaxAngle
        size_of::<WiiUParameter<F32>>() as u32, // mSlipEnableSpeed
        size_of::<WiiUParameter<F32>>() as u32, // mSlipSpeedAddMin
        size_of::<WiiUParameter<F32>>() as u32, // mSlipSpeedAddMax
        size_of::<WiiUParameter<F32>>() as u32, // mSlipSpeedAddDiamByRain
        size_of::<WiiUParameter<F32>>() as u32, // mMagnetAim2DPosOffsetY
        size_of::<WiiUParameter<F32>>() as u32, // mLookPosOffsetXZ
        size_of::<WiiUParameter<F32>>() as u32, // mLookPosOffsetY
        size_of::<WiiUParameter<F32>>() as u32, // mLookPosOffsetYSquat
        size_of::<WiiUParameter<F32>>() as u32, // mLookPosOffsetYSwim
        size_of::<WiiUParameter<F32>>() as u32, // mLookPosOffsetYHorse
        size_of::<WiiUParameter<F32>>() as u32, // mLookEnableAngle
        size_of::<WiiUParameter<F32>>() as u32, // mHitSlowTimeS
        size_of::<WiiUParameter<F32>>() as u32, // mHitSlowTimeM
        size_of::<WiiUParameter<F32>>() as u32, // mHitSlowTimeL
        size_of::<WiiUParameter<F32>>() as u32, // mHitSlowRate
        size_of::<WiiUParameter<F32>>() as u32, // mHitStopTimeS
        size_of::<WiiUParameter<F32>>() as u32, // mHitStopTimeL
        size_of::<WiiUParameter<F32>>() as u32, // mHitStopRate
        size_of::<WiiUParameter<F32>>() as u32, // mAtnPosInterPolationRate
        size_of::<WiiUParameter<F32>>() as u32, // mAtnPosInterPolationMin
        size_of::<WiiUParameter<F32>>() as u32, // mAtnPosInterPolationMax
        size_of::<WiiUParameter<F32>>() as u32, // mPredictDiffAngleMax
        size_of::<WiiUParameter<F32>>() as u32, // mDashToRunStickValueDec
        size_of::<WiiUParameter<F32>>() as u32, // mWindSupportReuseTime
        size_of::<WiiUParameter<F32>>() as u32, // mFireSupportReuseTime
        size_of::<WiiUParameter<F32>>() as u32, // mElectricSupportReuseTime
        size_of::<WiiUParameter<F32>>() as u32, // mWaterSupportReuseTime
        size_of::<WiiUParameter<F32>>() as u32, // mWindSupportTimerRate
        size_of::<WiiUParameter<F32>>() as u32, // mFireSupportTimerRate
        size_of::<WiiUParameter<F32>>() as u32, // mElectricSupportTimerRate
        size_of::<WiiUParameter<F32>>() as u32, // mWaterSupportTimerRate
        size_of::<WiiUParameter<F32>>() as u32, // mChemicalInvalidTime
        size_of::<WiiUParameter<F32>>() as u32, // mAutoDashUpTime
        size_of::<WiiUParameter<F32>>() as u32, // mAutoDashUpAngle
        size_of::<WiiUParameter<F32>>() as u32, // mClimbRestartHeight
        size_of::<WiiUParameter<F32>>() as u32, // mClimbRestartTime
        size_of::<WiiUParameter<F32>>() as u32, // mPushNoticeLookTime
        size_of::<WiiUParameter<F32>>() as u32, // mEnergyUseSmall
        size_of::<WiiUParameter<F32>>() as u32, // mEnergyUseLarge
        size_of::<WiiUParameter<F32>>() as u32, // mNoEnergyDashInterval
        size_of::<WiiUParameter<F32>>() as u32, // mGuardableAngle
        size_of::<WiiUParameter<F32>>() as u32, // mStickMaxInStore
        size_of::<WiiUParameter<F32>>() as u32, // mLookContinueTime
        size_of::<WiiUParameter<F32>>() as u32, // mPostureContinueTime
        size_of::<WiiUParameter<F32>>() as u32, // mItemUseModelAlpha
        size_of::<WiiUParameter<F32>>() as u32, // mLadderCheckSide
        size_of::<WiiUParameter<F32>>() as u32, // mLadderCheckDist
        size_of::<WiiUParameter<S32>>() as u32, // mNoDeathDamageBase
        size_of::<WiiUParameter<S32>>() as u32, // mNoDeathDamageAdd
        size_of::<WiiUParameter<F32>>() as u32, // mArmorCompSwimEnergyRate
        size_of::<WiiUParameter<F32>>() as u32, // mArmorCompRegistElecFrame
        size_of::<WiiUParameter<F32>>() as u32, // mArmorCompNightSpeedRate
        size_of::<WiiUParameter<F32>>() as u32, // mArmorCompClimbJumpEnergyRate
        size_of::<WiiUParameter<F32>>() as u32, // mArmorCompPlusDropRate
        size_of::<WiiUParameter<F32>>() as u32, // mArmorCompWeaponBrakeRate
        size_of::<WiiUParameter<F32>>() as u32, // mArmorCompSwordBeamAttackRate
        size_of::<WiiUParameter<F32>>() as u32, // mArmorCompAncientAttackRate
        size_of::<WiiUParameter<F32>>() as u32, // mArmorCompBoneAttackRate
        size_of::<WiiUParameter<F32>>() as u32, // mArmorCompTerrorLevel
        size_of::<WiiUParameter<F32>>() as u32, // mArmorCompTerrorRadius
        size_of::<WiiUParameter<F32>>() as u32, // mArmorCompNakedSwimSpeedRate
        size_of::<WiiUParameter<F32>>() as u32, // mArmorCompNakedSwimAnimeRate
        size_of::<WiiUParameter<F32>>() as u32, // mArmorCompNakedSwimEnergyRate
        size_of::<WiiUParameter<F32>>() as u32, // mArmorAncientAttackRate
        size_of::<WiiUParameter<S32>>() as u32, // mSupportWindNum
        size_of::<WiiUParameter<S32>>() as u32, // mSupportElectricNum
        size_of::<WiiUParameter<F32>>() as u32, // mSupportElectricEnergy
        size_of::<WiiUParameter<S32>>() as u32, // mSupportFireNum
        size_of::<WiiUParameter<S32>>() as u32, // mSupportWaterLifeAdd
        size_of::<WiiUParameter<F32>>() as u32, // mSupportWaterEnergyAdd
        size_of::<WiiUParameter<F32>>() as u32, // mStickRInputFrame
        size_of::<WiiUParameter<F32>>() as u32, // mDiffAngleFromLookVec
        size_of::<WiiUParameter<F32>>() as u32, // mLookPosOffset
        size_of::<WiiUParameter<F32>>() as u32, // mLookFixAngle
        size_of::<WiiUParameter<F32>>() as u32, // mLookContinueTimeToCamera
        size_of::<WiiUParameter<F32>>() as u32, // mCutKnockBackNoCrrDist
        size_of::<WiiUParameter<F32>>() as u32, // mWaitUnsteadyApplyVel
        size_of::<WiiUParameter<F32>>() as u32, // mCurseAddWeight
        size_of::<WiiUParameter<F32>>() as u32, // mRoofCrashVel
        size_of::<WiiUParameter<F32>>() as u32, // mCutJumpInvalidTime
        size_of::<WiiUParameter<F32>>() as u32, // mWaterDepthInGrudge
        size_of::<WiiUParameter<F32>>() as u32, // mLargeHorseLegBendAngY
        size_of::<WiiUParameter<F32>>() as u32, // mLargeHorseLegBendAngX
        size_of::<WiiUParameter<F32>>() as u32, // mLargeHorseLegBendFrame
        size_of::<WiiUParameter<F32>>() as u32, // mNoMaskPauseWaterHeight
        size_of::<WiiUParameter<F32>>() as u32, // mLookAtThreshold
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Prey" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Rod" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Rope" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Rupee" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Sandworm" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "SeriesArmor" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Bool32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "ShiekerStone" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Shield" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameter<S32>>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32,
        size_of::<WiiUParameter<F32>>() as u32,
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "SmallSword" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mPodName
        size_of::<WiiUParameter<Vector3f>>() as u32, // mPlayerHoldTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mPlayerHoldRotOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mPlayerEquipTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mPlayerEquipRotOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mRideHorsePlayerHoldTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mRideHorsePlayerHoldRotOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mAffectTransOffsetShield
        size_of::<WiiUParameter<Vector3f>>() as u32, // mAffectRotOffsetShield
        size_of::<WiiUParameter<Vector3f>>() as u32, // mAffectTransOffsetBow
        size_of::<WiiUParameter<Vector3f>>() as u32, // mAffectRotOffsetBow
        size_of::<WiiUParameter<Vector3f>>() as u32, // mSquatPlayerHoldTransAddOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mSquatPlayerHoldRotAddOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mNPCHoldTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mNPCHoldRotOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mNPCEquipTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mNPCEquipRotOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mEnemyEquipTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mEnemyEquipRotOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mStandEquipTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mStandEquipRotOffset
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWeaponSubType
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Spear" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mPodName
        size_of::<WiiUParameter<Vector3f>>() as u32, // mPlayerHoldTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mPlayerHoldRotOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mPlayerEquipTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mPlayerEquipRotOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mRideHorsePlayerHoldTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mRideHorsePlayerHoldRotOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mAffectTransOffsetShield
        size_of::<WiiUParameter<Vector3f>>() as u32, // mAffectRotOffsetShield
        size_of::<WiiUParameter<Vector3f>>() as u32, // mAffectTransOffsetBow
        size_of::<WiiUParameter<Vector3f>>() as u32, // mAffectRotOffsetBow
        size_of::<WiiUParameter<Vector3f>>() as u32, // mGrabPlayerHoldTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mGrabPlayerHoldRotOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mGrabAffectTransOffsetShield
        size_of::<WiiUParameter<Vector3f>>() as u32, // mGrabAffectRotOffsetShield
        size_of::<WiiUParameter<Vector3f>>() as u32, // mSquatPlayerHoldTransAddOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mSquatPlayerHoldRotAddOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mNPCHoldTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mNPCHoldRotOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mNPCEquipTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mNPCEquipRotOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mEnemyEquipTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mEnemyEquipRotOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mStandEquipTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mStandEquipRotOffset
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWeaponSubType
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "StalEnemy" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mHeadActorName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mLeftArmActorName
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Swarm" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32, // mSwarmSubActorNum
        size_of::<WiiUParameter<S32>>() as u32, // mSwarmPattern
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mDeadActorName
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "System" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSameGroupActorName
        size_of::<WiiUParameter<Bool32>>() as u32, // mIsGetItemSelf
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Traveler" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mAppearGameDataName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mDeleteGameDataName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mRouteType
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mRideHorseName
        size_of::<WiiUParameter<Bool32>>() as u32, // mIsLeadHorse
        size_of::<WiiUParameter<S32>>() as u32, // mHorseGearLevel
        // 1
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 2
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 3
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 4
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 5
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 6
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 7
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 8
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 9
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 10
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 11
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 12
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 13
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 14
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 15
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 16
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 17
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 18
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 19
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 20
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 21
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 22
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 23
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 24
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 25
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 26
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 27
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 28
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mEntryPoint
        size_of::<WiiUParameter<F32>>() as u32, // mWaitFrame
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSchedule
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMoveAS
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mWaitAS
        // 29
        // size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mName
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "WeaponCommon" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<F32>>() as u32, // mPlayerEqScale
        size_of::<WiiUParameter<F32>>() as u32, // mEnemyEqScale
        size_of::<WiiUParameter<S32>>() as u32, // mGuardPower
        size_of::<WiiUParameter<S32>>() as u32, // mRank
        size_of::<WiiUParameter<Bool32>>() as u32, // mIsHammer
        size_of::<WiiUParameter<Bool32>>() as u32, // mIsWeakBreaker
        size_of::<WiiUParameter<Bool32>>() as u32, // mIsBoomerang
        size_of::<WiiUParameter<Bool32>>() as u32, // mIsBlunt
        size_of::<WiiUParameter<Bool32>>() as u32, // mIsLuckyWeapon
        size_of::<WiiUParameter<Bool32>>() as u32, // mIsPikohan
        size_of::<WiiUParameter<Bool32>>() as u32, // mIsThrowingWeapon
        size_of::<WiiUParameter<Bool32>>() as u32, // mIsThrowingBreakWeapon
        size_of::<WiiUParameter<F32>>() as u32, // mThrowRange
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mDreadActor
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mThroughActor
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mNPCWeaponType
        size_of::<WiiUParameter<Bool32>>() as u32, // mIsNotOnTerrorHold
        size_of::<WiiUParameter<Bool32>>() as u32, // mIsAsOffUnEquiped
        size_of::<WiiUParameter<S32>>() as u32, // mChemicalEnergyMax
        size_of::<WiiUParameter<S32>>() as u32, // mChemicalEnergyAmountUsed
        size_of::<WiiUParameter<F32>>() as u32, // mChemicalEnergyRecoverRate
        size_of::<WiiUParameter<S32>>() as u32, // mChemicalEnergyRecoverInterval
        size_of::<WiiUParameter<S32>>() as u32, // mStickDamage
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mShootBeam
        size_of::<WiiUParameter<Vector3f>>() as u32, // mDropFromPorchRot
        size_of::<WiiUParameter<F32>>() as u32, // mSharpWeaponPer
        size_of::<WiiUParameter<S32>>() as u32, // mSharpWeaponAddAtkMin
        size_of::<WiiUParameter<S32>>() as u32, // mSharpWeaponAddAtkMax
        size_of::<WiiUParameter<S32>>() as u32, // mSharpWeaponAddLifeMin
        size_of::<WiiUParameter<S32>>() as u32, // mSharpWeaponAddLifeMax
        size_of::<WiiUParameter<Bool32>>() as u32, // mSharpWeaponAddCrit
        size_of::<WiiUParameter<S32>>() as u32, // mSharpWeaponAddGuardMin
        size_of::<WiiUParameter<S32>>() as u32, // mSharpWeaponAddGuardMax
        size_of::<WiiUParameter<S32>>() as u32, // mPoweredSharpAddAtkMin
        size_of::<WiiUParameter<S32>>() as u32, // mPoweredSharpAddAtkMax
        size_of::<WiiUParameter<S32>>() as u32, // mPoweredSharpAddLifeMin
        size_of::<WiiUParameter<S32>>() as u32, // mPoweredSharpAddLifeMax
        size_of::<WiiUParameter<S32>>() as u32, // mPoweredSharpWeaponAddGuardMin
        size_of::<WiiUParameter<S32>>() as u32, // mPoweredSharpWeaponAddGuardMax
        size_of::<WiiUParameter<F32>>() as u32, // mPoweredSharpAddThrowMin
        size_of::<WiiUParameter<F32>>() as u32, // mPoweredSharpAddThrowMax
        size_of::<WiiUParameter<Bool32>>() as u32, // mPoweredSharpAddSpreadFire
        size_of::<WiiUParameter<Bool32>>() as u32, // mPoweredSharpAddZoomRapid
        size_of::<WiiUParameter<F32>>() as u32, // mPoweredSharpAddRapidFireMin
        size_of::<WiiUParameter<F32>>() as u32, // mPoweredSharpAddRapidFireMax
        size_of::<WiiUParameter<Bool32>>() as u32, // mPoweredSharpAddSurfMaster
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "WeaponOption" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<Vector3f>>() as u32, // mPlayerHoldTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mPlayerHoldRotOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mNPCHoldTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mNPCHoldRotOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mNPCEquipTransOffset
        size_of::<WiiUParameter<Vector3f>>() as u32, // mNPCEquipRotOffset
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "WeaponThrow" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<F32>>() as u32, // mThrowSpeed
        size_of::<WiiUParameter<F32>>() as u32, // mThrowRotSpeed
        size_of::<WiiUParameter<F32>>() as u32, // mThrowDist
        size_of::<WiiUParameter<Vector3f>>() as u32, // mThrowRigidBodyBaseAxis
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "WizzRobe" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<S32>>() as u32, // mMagicWeatherType
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mMagicFallActorName
        size_of::<WiiUParameter<F32>>() as u32, // mMagicFallIgniteRotSpd
        size_of::<WiiUParameter<F32>>() as u32, // mMagicFallOffsetY
        size_of::<WiiUParameter<F32>>() as u32, // mMagicFallCenterOffsetXZ
        size_of::<WiiUParameter<F32>>() as u32, // mMagicFallRandRadius
        size_of::<WiiUParameter<F32>>() as u32, // mMagicFallIntervalMax
        size_of::<WiiUParameter<F32>>() as u32, // mMagicFallIntervalMin
        size_of::<WiiUParameter<WiiUSafeString>>() as u32, // mSummonActorName
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "WolfLink" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<F32>>() as u32, // mNeckSpeedWait
        size_of::<WiiUParameter<F32>>() as u32, // mNeckRateWait
        size_of::<WiiUParameter<F32>>() as u32, // mNeckSpeedShiekSensor
        size_of::<WiiUParameter<F32>>() as u32, // mNeckRateShiekSensor
        size_of::<WiiUParameter<F32>>() as u32, // mNeckSpeedFollow
        size_of::<WiiUParameter<F32>>() as u32, // mNeckRateFollow
        size_of::<WiiUParameter<F32>>() as u32, // mNeckSpeedBattle
        size_of::<WiiUParameter<F32>>() as u32, // mNeckRateBattle
        size_of::<WiiUParameter<F32>>() as u32, // mNeckSpeedHeal
        size_of::<WiiUParameter<F32>>() as u32, // mNeckRateHeal
        size_of::<WiiUParameter<F32>>() as u32, // mBattleRange
        size_of::<WiiUParameter<F32>>() as u32, // mHealRange
        size_of::<WiiUParameter<F32>>() as u32, // mHuntRange
        size_of::<WiiUParameter<F32>>() as u32, // mHowlRange
        size_of::<WiiUParameter<F32>>() as u32, // mMaxHeightAttackable
        size_of::<WiiUParameter<F32>>() as u32, // mMaxHeightHealable
        size_of::<WiiUParameter<F32>>() as u32, // mNavMeshSearchRadius
        size_of::<WiiUParameter<F32>>() as u32, // mCanReachPlayerNavMeshSearchRadius
        size_of::<WiiUParameter<F32>>() as u32, // mSubmergedDepth
        size_of::<WiiUParameter<F32>>() as u32, // mUtilityLifeToHunt
        size_of::<WiiUParameter<F32>>() as u32, // mUtilityDangerDistMin
        size_of::<WiiUParameter<F32>>() as u32, // mUtilityDangerDistMax
        size_of::<WiiUParameter<F32>>() as u32, // mUtilityConstant
        size_of::<WiiUParameter<F32>>() as u32, // mChainAttackChargeMin
        size_of::<WiiUParameter<F32>>() as u32, // mChainAttackChargeMax
        size_of::<WiiUParameter<F32>>() as u32, // mLookAtCooldownWait
        size_of::<WiiUParameter<F32>>() as u32, // mLookAtCooldownWaitRand
        size_of::<WiiUParameter<F32>>() as u32, // mLookAtCounterWait
        size_of::<WiiUParameter<F32>>() as u32, // mLookAtCounterWaitRand
        size_of::<WiiUParameter<F32>>() as u32, // mLookAtCooldownRun
        size_of::<WiiUParameter<F32>>() as u32, // mLookAtCooldownRunRand
        size_of::<WiiUParameter<F32>>() as u32, // mLookAtCounterRun
        size_of::<WiiUParameter<F32>>() as u32, // mLookAtCounterRunRand
        size_of::<WiiUParameter<F32>>() as u32, // mAttackCounterLength
        size_of::<WiiUParameter<F32>>() as u32, // mAttackCounterRand
        size_of::<WiiUParameter<F32>>() as u32, // mHowlCooldownCounterLength
        size_of::<WiiUParameter<F32>>() as u32, // mHowlCooldownCounterRand
        size_of::<WiiUParameter<F32>>() as u32, // mHealCooldownCounterLength
        size_of::<WiiUParameter<F32>>() as u32, // mHealCooldownCounterRand
        size_of::<WiiUParameter<F32>>() as u32, // mFailPathCooldownCounterLength
        size_of::<WiiUParameter<F32>>() as u32, // mFailPathCooldownCounterRand
        size_of::<WiiUParameter<F32>>() as u32, // mRetargetCooldownCounterLength
        size_of::<WiiUParameter<F32>>() as u32, // mRetargetCooldownCounterRand
        size_of::<WiiUParameter<F32>>() as u32, // mAfterTargetDeathCounterLength
        size_of::<WiiUParameter<F32>>() as u32, // mAfterTargetDeathCounterRand
        size_of::<WiiUParameter<F32>>() as u32, // mLostTargetCounterLength
        size_of::<WiiUParameter<F32>>() as u32, // mLostTargetCounterRand
        size_of::<WiiUParameter<F32>>() as u32, // mInvinceableCounterLength
        size_of::<WiiUParameter<F32>>() as u32, // mInvinceableCounterRand
        size_of::<WiiUParameter<F32>>() as u32, // mCallDelayMinLength
        size_of::<WiiUParameter<F32>>() as u32, // mCallOverrideCounterLength
        size_of::<WiiUParameter<F32>>() as u32, // mGiveUpShiekSensorLength
        size_of::<WiiUParameter<F32>>() as u32, // mRetryShiekSensorLength
        size_of::<WiiUParameter<F32>>() as u32, // mBattleWallHitLength
        size_of::<WiiUParameter<F32>>() as u32, // mFollowRetryLength
        size_of::<WiiUParameter<F32>>() as u32, // mPowerUpFoodLength
        size_of::<WiiUParameter<F32>>() as u32, // mSafePosFailCounter
        size_of::<WiiUParameter<F32>>() as u32, // mRestrictedTargetTimeNormal
        size_of::<WiiUParameter<F32>>() as u32, // mRestrictedTargetTimeSpecial
        size_of::<WiiUParameter<S32>>() as u32, // mPowerUpFoodAttackMod
        size_of::<WiiUParameter<F32>>() as u32, // mPowerUpFoodChainAttackCharge
        size_of::<WiiUParameter<S32>>() as u32, // mVSStalfosCritChance
        size_of::<WiiUParameter<F32>>() as u32, // mAttackBase
        size_of::<WiiUParameter<F32>>() as u32, // mAttackHeartMod
        size_of::<WiiUParameter<F32>>() as u32, // mDefenseBase
        size_of::<WiiUParameter<F32>>() as u32, // mDefenseHeartMod
        size_of::<WiiUParameterObj>() as u32
    ], &4),
    "Zora" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<WiiUParameter<F32>>() as u32, // mInWaterDepth
        size_of::<WiiUParameter<F32>>() as u32, // mFloatDepth
        size_of::<WiiUParameter<F32>>() as u32, // mFloatRadius
        size_of::<WiiUParameter<F32>>() as u32, // mFloatCycleTime
        size_of::<WiiUParameter<F32>>() as u32, // mChangeDepthSpeed
        size_of::<WiiUParameterObj>() as u32
    ], &4),
};

fn parse_size_wiiu(b: &[u8]) -> u32 {
    let a = ParameterIO::from_binary(b).unwrap();
    let mut total_size = BGPARAM_OVERHEAD_WIIU;
    for (name, size) in OBJ_SIZES_WIIU.into_iter() {
        if let Some(_) = a.param_root.objects.get(*name) {
            total_size += size;
        }
    }
    total_size
}

const BGPARAM_OVERHEAD_NX: u32 = 0x2c0; // 0x160
static OBJ_SIZES_NX: Map<&'static str, u32> = phf_map! {
    "AirWall" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "AnimalFollowOffset" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "AnimalUnit" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Armor" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "ArmorEffect" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "ArmorHead" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "ArmorUpper" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Arrow" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Attack" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "AttackInterval" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "AutoGen" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Beam" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "BindActor" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "BindBone" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Bow" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Bullet" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Camera" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "ChemicalType" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "ClothReaction" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "CookSpice" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "CureItem" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "EatTarget" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Enemy" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "EnemyLevel" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "EnemyRace" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "EnemyShown" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Event" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "ExtendedEntity" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Fish" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "GelEnemy" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "General" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "GiantArmor" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "GiantArmorSlot" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Global" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<F32>>() as u32, // mEnemyLifeGageDist
        size_of::<NXParameter<F32>>() as u32, // mEnemyNoSkitDist
        size_of::<NXParameter<F32>>() as u32, // mEnemyWeaponPickAllowDist
        size_of::<NXParameter<S32>>() as u32, // mEnemyWeaponPickForbidTime
        size_of::<NXParameter<F32>>() as u32, // mEnemyAnimalNoDamageDist
        size_of::<NXParameter<F32>>() as u32, // mEnemyNearCraeteIDDelay
        size_of::<NXParameter<S32>>() as u32, // mEnemyForceTiredLODCount
        size_of::<NXParameter<S32>>() as u32, // mEnemyForceTiredNoSightLODCount
        size_of::<NXParameter<S32>>() as u32, // mEnemyForceWarpReturnLODCount
        size_of::<NXParameter<F32>>() as u32, // mSilentAttackAng
        size_of::<NXParameter<F32>>() as u32, // mSilentAttackRatio
        size_of::<NXParameter<S32>>() as u32, // mBlownOffPlayerAtkDelay
        size_of::<NXParameter<F32>>() as u32, // mJustAvoidAcceptWpRangeSS
        size_of::<NXParameter<F32>>() as u32, // mJustAvoidAcceptWpRangeLS
        size_of::<NXParameter<F32>>() as u32, // mJustAvoidAcceptWpRangeSP
        size_of::<NXParameter<S32>>() as u32, // mForceNoticeEnemyCount
        size_of::<NXParameter<F32>>() as u32, // mForceNoticeEnemyDist
        size_of::<NXParameter<S32>>() as u32, // mWeaponRickeyLife
        size_of::<NXParameter<F32>>() as u32, // mWeaponDropRotSpd
        size_of::<NXParameter<S32>>() as u32, // mShieldRideBaseFrame
        size_of::<NXParameter<S32>>() as u32, // mShieldRideHitBaseDamage
        size_of::<NXParameter<F32>>() as u32, // mShieldDamageratio
        size_of::<NXParameter<F32>>() as u32, // mShieldSurfMasterFrictionRatio
        size_of::<NXParameter<F32>>() as u32, // mLoudNoiseRadius
        size_of::<NXParameter<F32>>() as u32, // mImpulse2DamageRatio
        size_of::<NXParameter<F32>>() as u32, // mIceMeltSpeedOnContactFire
        size_of::<NXParameter<F32>>() as u32, // mCriticalAttackRatio
        size_of::<NXParameter<F32>>() as u32, // mBooerangAttackRatio
        size_of::<NXParameter<F32>>() as u32, // mHitImpulseClampMax
        size_of::<NXParameter<F32>>() as u32, // mDropItemVelXZFromBomb
        size_of::<NXParameter<F32>>() as u32, // mDropItemVelYFromBomb
        size_of::<NXParameter<F32>>() as u32, // mDropItemVelRandomFromBomb
        size_of::<NXParameter<F32>>() as u32, // mDropItemAngVelFromBomb
        size_of::<NXParameter<F32>>() as u32, // mDropItemAngVelRandomFromBomb
        size_of::<NXParameter<F32>>() as u32, // mDropItemVelXZSmall
        size_of::<NXParameter<F32>>() as u32, // mDropItemVelYSmall
        size_of::<NXParameter<F32>>() as u32, // mDropItemVelRandomSmall
        size_of::<NXParameter<F32>>() as u32, // mDropItemAngVelSmall
        size_of::<NXParameter<F32>>() as u32, // mDropItemAngVelRandomSmall
        size_of::<NXParameter<F32>>() as u32, // mDropItemVelXZLarge
        size_of::<NXParameter<F32>>() as u32, // mDropItemVelYLarge
        size_of::<NXParameter<F32>>() as u32, // mDropItemVelRandomLarge
        size_of::<NXParameter<F32>>() as u32, // mDropItemAngVelLarge
        size_of::<NXParameter<F32>>() as u32, // mDropItemAngVelRandomLarge
        size_of::<NXParameter<F32>>() as u32, // mDropItemVelXZRupeeRabbit
        size_of::<NXParameter<F32>>() as u32, // mDropItemVelYRupeeRabbit
        size_of::<NXParameter<F32>>() as u32, // mDropItemVelRandomRupeeRabbit
        size_of::<NXParameter<F32>>() as u32, // mDropItemVelXZItemRupeeOnly
        size_of::<NXParameter<F32>>() as u32, // mDropItemVelYItemRupeeOnly
        size_of::<NXParameter<F32>>() as u32, // mDropItemVelRandomItemRupeeOnly
        size_of::<NXParameter<F32>>() as u32, // mDropItemInvincibleTime
        size_of::<NXParameter<Vector3f>>() as u32, // mTreeWeaponEquipTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mTreeWeaponEquipRotOffset
        size_of::<NXParameter<F32>>() as u32, // mWetRatioToDie
        size_of::<NXParameter<F32>>() as u32, // mEnvWetRatioToDie
        size_of::<NXParameter<F32>>() as u32, // mNPCTurnAngleDiff
        size_of::<NXParameter<S32>>() as u32, // mNPCWaitFrameAfterEvent
        size_of::<NXParameter<F32>>() as u32, // mNPCIgnorePlayerTime
        size_of::<NXParameter<F32>>() as u32, // mNPCCancelIgnorePlayerTime
        size_of::<NXParameter<F32>>() as u32, // mNPCOpenDoorDistance
        size_of::<NXParameter<F32>>() as u32, // mNPCWalkRateOnSandAndSnow
        size_of::<NXParameter<F32>>() as u32, // mNPCDownVerticallyAngle
        size_of::<NXParameter<F32>>() as u32, // mGerudoQueenSafetyAreaRadius
        size_of::<NXParameter<S32>>() as u32, // mCreateFairyLimitCount
        size_of::<NXParameter<F32>>() as u32, // mTerrorRegistSpeed
        size_of::<NXParameter<F32>>() as u32, // mTerrorUnregistSpeed
        size_of::<NXParameter<S32>>() as u32, // mTerrorRegistTimer
        size_of::<NXParameter<F32>>() as u32, // mTerrorRadiusOffset
        size_of::<NXParameter<S32>>() as u32, // mSpeedTerrorLevel
        size_of::<NXParameter<S32>>() as u32, // mSpeedTerrorLevelHuge
        size_of::<NXParameter<F32>>() as u32, // mSpeedTerrorLevelCheckRadius
        size_of::<NXParameter<F32>>() as u32, // mAtDirTypeAffectRatio
        size_of::<NXParameter<F32>>() as u32, // mRainyAwnHearingLevel
        size_of::<NXParameter<F32>>() as u32, // mHorseBindOffsetYOfMaleUMii
        size_of::<NXParameter<F32>>() as u32, // mHorseBindOffsetYOfFemaleUMii
        size_of::<NXParameter<Vector3f>>() as u32, // mHorseFamiliarityIncreasePerFrame
        size_of::<NXParameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheAtFirstRun
        size_of::<NXParameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheAfterRun
        size_of::<NXParameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheAfterGearTop
        size_of::<NXParameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheAfterJump
        size_of::<NXParameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheWhileResisting
        size_of::<NXParameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseEat
        size_of::<NXParameter<Vector3f>>() as u32, // mHorseAlertProbability
        size_of::<NXParameter<Vector3f>>() as u32, // mHorseAlertFramesMin
        size_of::<NXParameter<Vector3f>>() as u32, // mHorseAlertFramesMax
        size_of::<NXParameter<S32>>() as u32, // mHorseExtraChargeNum
        size_of::<NXParameter<F32>>() as u32, // mPlayerGrabThrowDiffRate
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Golem" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "GolemIK" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Grab" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Guardian" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "GuardianMini" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "GuardianMiniWeapon" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Horse" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "HorseCreator" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "HorseObject" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "HorseRider" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "HorseTargetedInfo" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "HorseUnit" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Insect" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Item" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "LargeSword" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Liftable" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "LumberjackTree" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "MasterSword" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "MonsterShop" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Motorcycle" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<F32>>() as u32, // mPitchDampingCoefficient
        size_of::<NXParameter<F32>>() as u32, // mDriftAllowSpeedKPH
        size_of::<NXParameter<F32>>() as u32, // mDriftAbortSpeedKPH
        size_of::<NXParameter<F32>>() as u32, // mDriftAllowSteerRate
        size_of::<NXParameter<F32>>() as u32, // mDriftAbortSteerRate
        size_of::<NXParameter<F32>>() as u32, // mDriftRearAngleRate
        size_of::<NXParameter<F32>>() as u32, // mDriftSpeedRate
        size_of::<NXParameter<F32>>() as u32, // mManualWheelieAllowAngleFront
        size_of::<NXParameter<F32>>() as u32, // mManualWheelieAllowAngleRear
        size_of::<NXParameter<F32>>() as u32, // mManualWheelieLastSec
        size_of::<NXParameter<F32>>() as u32, // mWheelieLastSecInMidAir
        size_of::<NXParameter<F32>>() as u32, // mManualControlProhibitSecAfterWheelie
        size_of::<NXParameter<F32>>() as u32, // mWheelieRevertPower
        size_of::<NXParameter<F32>>() as u32, // mWheelieRevertPowerSec
        size_of::<NXParameter<F32>>() as u32, // mManualWheelieRiseDegDelta
        size_of::<NXParameter<F32>>() as u32, // mWheelieLaunchRiseDegDelta
        size_of::<NXParameter<F32>>() as u32, // mEngineBrakeMaxPower
        size_of::<NXParameter<F32>>() as u32, // mBackwardEngineBrakePower
        size_of::<NXParameter<F32>>() as u32, // mSlipStartAngle
        size_of::<NXParameter<F32>>() as u32, // mSlipThresholdPower
        size_of::<NXParameter<F32>>() as u32, // mSlipPowerMax
        size_of::<NXParameter<Vector3f>>() as u32, // mWristBindRotation
        size_of::<NXParameter<Vector3f>>() as u32, // mWristBindTranslation
        size_of::<NXParameter<F32>>() as u32, // mPostureLimitAngle
        size_of::<NXParameter<F32>>() as u32, // mInvalidPostureLimitSec
        size_of::<NXParameter<F32>>() as u32, // mFallOverThresholdAngle
        size_of::<NXParameter<F32>>() as u32, // mJumpIntervalSec
        size_of::<NXParameter<F32>>() as u32, // mFullEnergyLastSec
        size_of::<NXParameter<F32>>() as u32, // mWheelieLaunchJumpProhibitSec
        size_of::<NXParameter<F32>>() as u32, // mSlowModeTargetSpeedKPH2
        size_of::<NXParameter<F32>>() as u32, // mSlowDriftTargetSpeedKPH2
        size_of::<NXParameter<F32>>() as u32, // mSlowModeTransitionSec
        size_of::<NXParameter<F32>>() as u32, // mSlowSlipThresholdKPH
        size_of::<NXParameter<F32>>() as u32, // mSlowSlipThresholdPower
        size_of::<NXParameter<F32>>() as u32, // mSlowSlipThresholdSec
        size_of::<NXParameter<F32>>() as u32, // mJumpRearWheelRotateRadPerSec
        size_of::<NXParameter<F32>>() as u32, // mWeaponThrowModeSpeedKPH2
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Nest" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Npc" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "NpcEquipment" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "PictureBook" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Player" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<F32>>() as u32, // mBombReloadTime1
        size_of::<NXParameter<F32>>() as u32, // mBombReloadTime2
        size_of::<NXParameter<F32>>() as u32, // mStopTimerReloadTime
        size_of::<NXParameter<F32>>() as u32, // mStopTimerBlowAngle
        size_of::<NXParameter<F32>>() as u32, // mStopTimerBlowSpeedLimit
        size_of::<NXParameter<S32>>() as u32, // mStopTimerImpluseMaxCountSmallSword
        size_of::<NXParameter<S32>>() as u32, // mStopTimerImpluseMaxCountLargeSword
        size_of::<NXParameter<S32>>() as u32, // mStopTimerImpluseMaxCountSpear
        size_of::<NXParameter<F32>>() as u32, // mStopTimerCancelDeleteWaitTime
        size_of::<NXParameter<F32>>() as u32, // mStopTimerLongTime
        size_of::<NXParameter<F32>>() as u32, // mStopTimerMiddleTime
        size_of::<NXParameter<F32>>() as u32, // mStopTimerShortTime
        size_of::<NXParameter<F32>>() as u32, // mEnergyTiredValue
        size_of::<NXParameter<F32>>() as u32, // mEnergyBowSlow
        size_of::<NXParameter<F32>>() as u32, // mEnergyPush
        size_of::<NXParameter<F32>>() as u32, // mEnergyCharge
        size_of::<NXParameter<F32>>() as u32, // mEnergyAutoRecover
        size_of::<NXParameter<F32>>() as u32, // mEnergyAutoRecoverInAir
        size_of::<NXParameter<F32>>() as u32, // mEnergyAutoRecoverInvalidTime1
        size_of::<NXParameter<F32>>() as u32, // mEnergyAutoRecoverInvalidTime2
        size_of::<NXParameter<F32>>() as u32, // mColdTempDamageAdd
        size_of::<NXParameter<F32>>() as u32, // mHotTempDamageAdd
        size_of::<NXParameter<F32>>() as u32, // mTempDamage
        size_of::<NXParameter<F32>>() as u32, // mTempEnergyDecDiamAdd
        size_of::<NXParameter<F32>>() as u32, // mTempEnergyDecDegAdd
        size_of::<NXParameter<F32>>() as u32, // mVelDiamSand
        size_of::<NXParameter<F32>>() as u32, // mVelDiamTired
        size_of::<NXParameter<F32>>() as u32, // mStickDiamTired
        size_of::<NXParameter<F32>>() as u32, // mAutoRecoverNum
        size_of::<NXParameter<F32>>() as u32, // mAutoRecoverIntervalMin
        size_of::<NXParameter<F32>>() as u32, // mAutoRecoverIntervalMax
        size_of::<NXParameter<F32>>() as u32, // mAutoRecoverInvalidTime
        size_of::<NXParameter<F32>>() as u32, // mBowSubjectContTime
        size_of::<NXParameter<F32>>() as u32, // mLNGStickScale
        size_of::<NXParameter<F32>>() as u32, // mLATStickScale
        size_of::<NXParameter<F32>>() as u32, // mLNGGyroScale
        size_of::<NXParameter<F32>>() as u32, // mLATGyroScale
        size_of::<NXParameter<S32>>() as u32, // mBowSlowShootNum
        size_of::<NXParameter<F32>>() as u32, // mBowSlowRateDiam
        size_of::<NXParameter<F32>>() as u32, // mBowSlowMaxTime
        size_of::<NXParameter<F32>>() as u32, // mDiveBowSlowMaxTime
        size_of::<NXParameter<F32>>() as u32, // mBowSlowInvalidTime
        size_of::<NXParameter<F32>>() as u32, // mBowSlowInvalidHeight
        size_of::<NXParameter<F32>>() as u32, // mBowSlowInvalidHeightOnShield
        size_of::<NXParameter<F32>>() as u32, // mBowSlowInvalidHeightWeaponChange
        size_of::<NXParameter<F32>>() as u32, // mGuardJustForceSlowTime
        size_of::<NXParameter<F32>>() as u32, // mMoveMaxDecRateByWater
        size_of::<NXParameter<F32>>() as u32, // mMoveIgnoreWaterHeight
        size_of::<NXParameter<F32>>() as u32, // mMoveDecRateByBog
        size_of::<NXParameter<F32>>() as u32, // mMoveDecRateMaxHeight
        size_of::<NXParameter<F32>>() as u32, // mMaxForce
        size_of::<NXParameter<F32>>() as u32, // mMinForce
        size_of::<NXParameter<F32>>() as u32, // mAddForce
        size_of::<NXParameter<F32>>() as u32, // mSnowBallAddForce
        size_of::<NXParameter<F32>>() as u32, // mLogPushF
        size_of::<NXParameter<F32>>() as u32, // mRockPushF
        size_of::<NXParameter<F32>>() as u32, // mRockPushSpeed
        size_of::<NXParameter<F32>>() as u32, // mWaistAngleUpperMax
        size_of::<NXParameter<F32>>() as u32, // mWaistAngleLowerMax
        size_of::<NXParameter<F32>>() as u32, // mWaistAngleSideMax
        size_of::<NXParameter<F32>>() as u32, // mNoSquatWaterHeight
        size_of::<NXParameter<F32>>() as u32, // mInvalidReloadTime
        size_of::<NXParameter<F32>>() as u32, // mWeaponThrowSpeedY
        size_of::<NXParameter<F32>>() as u32, // mWeaponThrowSpeedF
        size_of::<NXParameter<F32>>() as u32, // mWeaponThrowSpeedFSquat
        size_of::<NXParameter<F32>>() as u32, // mDashUpEnableAngle
        size_of::<NXParameter<F32>>() as u32, // mShockTime
        size_of::<NXParameter<F32>>() as u32, // mIceInvalidTime
        size_of::<NXParameter<F32>>() as u32, // mMaxSpeedInAir
        size_of::<NXParameter<F32>>() as u32, // mTurnEnableSpeed
        size_of::<NXParameter<F32>>() as u32, // mTurnEnableStickSub
        size_of::<NXParameter<F32>>() as u32, // mTurnEnableDirSub
        size_of::<NXParameter<S32>>() as u32, // mShortDashImpulse
        size_of::<NXParameter<S32>>() as u32, // mShortDashDamage
        size_of::<NXParameter<F32>>() as u32, // mSwordTerrorScope
        size_of::<NXParameter<F32>>() as u32, // mArrowTerrorScope
        size_of::<NXParameter<F32>>() as u32, // mTorchTerrorScope
        size_of::<NXParameter<F32>>() as u32, // mTorchTerrorOffsetY
        size_of::<NXParameter<F32>>() as u32, // mTorchTerrorOffsetZ
        size_of::<NXParameter<F32>>() as u32, // mDashNoise
        size_of::<NXParameter<F32>>() as u32, // mWhistleNoise
        size_of::<NXParameter<F32>>() as u32, // mClimbEnableAngle
        size_of::<NXParameter<F32>>() as u32, // mClimbEnableSpeedMinAngle
        size_of::<NXParameter<F32>>() as u32, // mClimbEnableSpeedMaxAngle
        size_of::<NXParameter<F32>>() as u32, // mSlipEnableSpeed
        size_of::<NXParameter<F32>>() as u32, // mSlipSpeedAddMin
        size_of::<NXParameter<F32>>() as u32, // mSlipSpeedAddMax
        size_of::<NXParameter<F32>>() as u32, // mSlipSpeedAddDiamByRain
        size_of::<NXParameter<F32>>() as u32, // mMagnetAim2DPosOffsetY
        size_of::<NXParameter<F32>>() as u32, // mLookPosOffsetXZ
        size_of::<NXParameter<F32>>() as u32, // mLookPosOffsetY
        size_of::<NXParameter<F32>>() as u32, // mLookPosOffsetYSquat
        size_of::<NXParameter<F32>>() as u32, // mLookPosOffsetYSwim
        size_of::<NXParameter<F32>>() as u32, // mLookPosOffsetYHorse
        size_of::<NXParameter<F32>>() as u32, // mLookEnableAngle
        size_of::<NXParameter<F32>>() as u32, // mHitSlowTimeS
        size_of::<NXParameter<F32>>() as u32, // mHitSlowTimeM
        size_of::<NXParameter<F32>>() as u32, // mHitSlowTimeL
        size_of::<NXParameter<F32>>() as u32, // mHitSlowRate
        size_of::<NXParameter<F32>>() as u32, // mHitStopTimeS
        size_of::<NXParameter<F32>>() as u32, // mHitStopTimeL
        size_of::<NXParameter<F32>>() as u32, // mHitStopRate
        size_of::<NXParameter<F32>>() as u32, // mAtnPosInterPolationRate
        size_of::<NXParameter<F32>>() as u32, // mAtnPosInterPolationMin
        size_of::<NXParameter<F32>>() as u32, // mAtnPosInterPolationMax
        size_of::<NXParameter<F32>>() as u32, // mPredictDiffAngleMax
        size_of::<NXParameter<F32>>() as u32, // mDashToRunStickValueDec
        size_of::<NXParameter<F32>>() as u32, // mWindSupportReuseTime
        size_of::<NXParameter<F32>>() as u32, // mFireSupportReuseTime
        size_of::<NXParameter<F32>>() as u32, // mElectricSupportReuseTime
        size_of::<NXParameter<F32>>() as u32, // mWaterSupportReuseTime
        size_of::<NXParameter<F32>>() as u32, // mWindSupportTimerRate
        size_of::<NXParameter<F32>>() as u32, // mFireSupportTimerRate
        size_of::<NXParameter<F32>>() as u32, // mElectricSupportTimerRate
        size_of::<NXParameter<F32>>() as u32, // mWaterSupportTimerRate
        size_of::<NXParameter<F32>>() as u32, // mChemicalInvalidTime
        size_of::<NXParameter<F32>>() as u32, // mAutoDashUpTime
        size_of::<NXParameter<F32>>() as u32, // mAutoDashUpAngle
        size_of::<NXParameter<F32>>() as u32, // mClimbRestartHeight
        size_of::<NXParameter<F32>>() as u32, // mClimbRestartTime
        size_of::<NXParameter<F32>>() as u32, // mPushNoticeLookTime
        size_of::<NXParameter<F32>>() as u32, // mEnergyUseSmall
        size_of::<NXParameter<F32>>() as u32, // mEnergyUseLarge
        size_of::<NXParameter<F32>>() as u32, // mNoEnergyDashInterval
        size_of::<NXParameter<F32>>() as u32, // mGuardableAngle
        size_of::<NXParameter<F32>>() as u32, // mStickMaxInStore
        size_of::<NXParameter<F32>>() as u32, // mLookContinueTime
        size_of::<NXParameter<F32>>() as u32, // mPostureContinueTime
        size_of::<NXParameter<F32>>() as u32, // mItemUseModelAlpha
        size_of::<NXParameter<F32>>() as u32, // mLadderCheckSide
        size_of::<NXParameter<F32>>() as u32, // mLadderCheckDist
        size_of::<NXParameter<S32>>() as u32, // mNoDeathDamageBase
        size_of::<NXParameter<S32>>() as u32, // mNoDeathDamageAdd
        size_of::<NXParameter<F32>>() as u32, // mArmorCompSwimEnergyRate
        size_of::<NXParameter<F32>>() as u32, // mArmorCompRegistElecFrame
        size_of::<NXParameter<F32>>() as u32, // mArmorCompNightSpeedRate
        size_of::<NXParameter<F32>>() as u32, // mArmorCompClimbJumpEnergyRate
        size_of::<NXParameter<F32>>() as u32, // mArmorCompPlusDropRate
        size_of::<NXParameter<F32>>() as u32, // mArmorCompWeaponBrakeRate
        size_of::<NXParameter<F32>>() as u32, // mArmorCompSwordBeamAttackRate
        size_of::<NXParameter<F32>>() as u32, // mArmorCompAncientAttackRate
        size_of::<NXParameter<F32>>() as u32, // mArmorCompBoneAttackRate
        size_of::<NXParameter<F32>>() as u32, // mArmorCompTerrorLevel
        size_of::<NXParameter<F32>>() as u32, // mArmorCompTerrorRadius
        size_of::<NXParameter<F32>>() as u32, // mArmorCompNakedSwimSpeedRate
        size_of::<NXParameter<F32>>() as u32, // mArmorCompNakedSwimAnimeRate
        size_of::<NXParameter<F32>>() as u32, // mArmorCompNakedSwimEnergyRate
        size_of::<NXParameter<F32>>() as u32, // mArmorAncientAttackRate
        size_of::<NXParameter<S32>>() as u32, // mSupportWindNum
        size_of::<NXParameter<S32>>() as u32, // mSupportElectricNum
        size_of::<NXParameter<F32>>() as u32, // mSupportElectricEnergy
        size_of::<NXParameter<S32>>() as u32, // mSupportFireNum
        size_of::<NXParameter<S32>>() as u32, // mSupportWaterLifeAdd
        size_of::<NXParameter<F32>>() as u32, // mSupportWaterEnergyAdd
        size_of::<NXParameter<F32>>() as u32, // mStickRInputFrame
        size_of::<NXParameter<F32>>() as u32, // mDiffAngleFromLookVec
        size_of::<NXParameter<F32>>() as u32, // mLookPosOffset
        size_of::<NXParameter<F32>>() as u32, // mLookFixAngle
        size_of::<NXParameter<F32>>() as u32, // mLookContinueTimeToCamera
        size_of::<NXParameter<F32>>() as u32, // mCutKnockBackNoCrrDist
        size_of::<NXParameter<F32>>() as u32, // mWaitUnsteadyApplyVel
        size_of::<NXParameter<F32>>() as u32, // mCurseAddWeight
        size_of::<NXParameter<F32>>() as u32, // mRoofCrashVel
        size_of::<NXParameter<F32>>() as u32, // mCutJumpInvalidTime
        size_of::<NXParameter<F32>>() as u32, // mWaterDepthInGrudge
        size_of::<NXParameter<F32>>() as u32, // mLargeHorseLegBendAngY
        size_of::<NXParameter<F32>>() as u32, // mLargeHorseLegBendAngX
        size_of::<NXParameter<F32>>() as u32, // mLargeHorseLegBendFrame
        size_of::<NXParameter<F32>>() as u32, // mNoMaskPauseWaterHeight
        size_of::<NXParameter<F32>>() as u32, // mLookAtThreshold
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Prey" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Rod" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Rope" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Rupee" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Sandworm" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "SeriesArmor" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Bool32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "ShiekerStone" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Shield" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameter<S32>>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32,
        size_of::<NXParameter<F32>>() as u32,
        size_of::<NXParameterObj>() as u32
    ], &4),
    "SmallSword" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32, // mPodName
        size_of::<NXParameter<Vector3f>>() as u32, // mPlayerHoldTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mPlayerHoldRotOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mPlayerEquipTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mPlayerEquipRotOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mRideHorsePlayerHoldTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mRideHorsePlayerHoldRotOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mAffectTransOffsetShield
        size_of::<NXParameter<Vector3f>>() as u32, // mAffectRotOffsetShield
        size_of::<NXParameter<Vector3f>>() as u32, // mAffectTransOffsetBow
        size_of::<NXParameter<Vector3f>>() as u32, // mAffectRotOffsetBow
        size_of::<NXParameter<Vector3f>>() as u32, // mSquatPlayerHoldTransAddOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mSquatPlayerHoldRotAddOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mNPCHoldTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mNPCHoldRotOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mNPCEquipTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mNPCEquipRotOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mEnemyEquipTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mEnemyEquipRotOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mStandEquipTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mStandEquipRotOffset
        size_of::<NXParameter<NXSafeString>>() as u32, // mWeaponSubType
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Spear" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32, // mPodName
        size_of::<NXParameter<Vector3f>>() as u32, // mPlayerHoldTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mPlayerHoldRotOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mPlayerEquipTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mPlayerEquipRotOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mRideHorsePlayerHoldTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mRideHorsePlayerHoldRotOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mAffectTransOffsetShield
        size_of::<NXParameter<Vector3f>>() as u32, // mAffectRotOffsetShield
        size_of::<NXParameter<Vector3f>>() as u32, // mAffectTransOffsetBow
        size_of::<NXParameter<Vector3f>>() as u32, // mAffectRotOffsetBow
        size_of::<NXParameter<Vector3f>>() as u32, // mGrabPlayerHoldTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mGrabPlayerHoldRotOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mGrabAffectTransOffsetShield
        size_of::<NXParameter<Vector3f>>() as u32, // mGrabAffectRotOffsetShield
        size_of::<NXParameter<Vector3f>>() as u32, // mSquatPlayerHoldTransAddOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mSquatPlayerHoldRotAddOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mNPCHoldTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mNPCHoldRotOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mNPCEquipTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mNPCEquipRotOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mEnemyEquipTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mEnemyEquipRotOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mStandEquipTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mStandEquipRotOffset
        size_of::<NXParameter<NXSafeString>>() as u32, // mWeaponSubType
        size_of::<NXParameterObj>() as u32
    ], &4),
    "StalEnemy" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32, // mHeadActorName
        size_of::<NXParameter<NXSafeString>>() as u32, // mLeftArmActorName
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Swarm" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32, // mSwarmSubActorNum
        size_of::<NXParameter<S32>>() as u32, // mSwarmPattern
        size_of::<NXParameter<NXSafeString>>() as u32, // mDeadActorName
        size_of::<NXParameterObj>() as u32
    ], &4),
    "System" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32, // mSameGroupActorName
        size_of::<NXParameter<Bool32>>() as u32, // mIsGetItemSelf
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Traveler" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<NXSafeString>>() as u32, // mAppearGameDataName
        size_of::<NXParameter<NXSafeString>>() as u32, // mDeleteGameDataName
        size_of::<NXParameter<NXSafeString>>() as u32, // mRouteType
        size_of::<NXParameter<NXSafeString>>() as u32, // mRideHorseName
        size_of::<NXParameter<Bool32>>() as u32, // mIsLeadHorse
        size_of::<NXParameter<S32>>() as u32, // mHorseGearLevel
        // 0
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 1
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 2
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 3
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 4
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 5
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 6
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 7
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 8
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 9
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 10
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 11
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 12
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 13
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 14
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 15
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 16
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 17
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 18
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 19
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 20
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 21
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 22
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 23
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 24
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 25
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 26
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 27
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 28
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mEntryPoint
        size_of::<NXParameter<F32>>() as u32, // mWaitFrame
        size_of::<NXParameter<NXSafeString>>() as u32, // mSchedule
        size_of::<NXParameter<NXSafeString>>() as u32, // mMoveAS
        size_of::<NXParameter<NXSafeString>>() as u32, // mWaitAS
        // 29
        size_of::<NXParameter<NXSafeString>>() as u32, // mName
        size_of::<NXParameterObj>() as u32
    ], &4),
    "WeaponCommon" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<F32>>() as u32, // mPlayerEqScale
        size_of::<NXParameter<F32>>() as u32, // mEnemyEqScale
        size_of::<NXParameter<S32>>() as u32, // mGuardPower
        size_of::<NXParameter<S32>>() as u32, // mRank
        size_of::<NXParameter<Bool32>>() as u32, // mIsHammer
        size_of::<NXParameter<Bool32>>() as u32, // mIsWeakBreaker
        size_of::<NXParameter<Bool32>>() as u32, // mIsBoomerang
        size_of::<NXParameter<Bool32>>() as u32, // mIsBlunt
        size_of::<NXParameter<Bool32>>() as u32, // mIsLuckyWeapon
        size_of::<NXParameter<Bool32>>() as u32, // mIsPikohan
        size_of::<NXParameter<Bool32>>() as u32, // mIsThrowingWeapon
        size_of::<NXParameter<Bool32>>() as u32, // mIsThrowingBreakWeapon
        size_of::<NXParameter<F32>>() as u32, // mThrowRange
        size_of::<NXParameter<NXSafeString>>() as u32, // mDreadActor
        size_of::<NXParameter<NXSafeString>>() as u32, // mThroughActor
        size_of::<NXParameter<NXSafeString>>() as u32, // mNPCWeaponType
        size_of::<NXParameter<Bool32>>() as u32, // mIsNotOnTerrorHold
        size_of::<NXParameter<Bool32>>() as u32, // mIsAsOffUnEquiped
        size_of::<NXParameter<S32>>() as u32, // mChemicalEnergyMax
        size_of::<NXParameter<S32>>() as u32, // mChemicalEnergyAmountUsed
        size_of::<NXParameter<F32>>() as u32, // mChemicalEnergyRecoverRate
        size_of::<NXParameter<S32>>() as u32, // mChemicalEnergyRecoverInterval
        size_of::<NXParameter<S32>>() as u32, // mStickDamage
        size_of::<NXParameter<NXSafeString>>() as u32, // mShootBeam
        size_of::<NXParameter<Vector3f>>() as u32, // mDropFromPorchRot
        size_of::<NXParameter<F32>>() as u32, // mSharpWeaponPer
        size_of::<NXParameter<S32>>() as u32, // mSharpWeaponAddAtkMin
        size_of::<NXParameter<S32>>() as u32, // mSharpWeaponAddAtkMax
        size_of::<NXParameter<S32>>() as u32, // mSharpWeaponAddLifeMin
        size_of::<NXParameter<S32>>() as u32, // mSharpWeaponAddLifeMax
        size_of::<NXParameter<Bool32>>() as u32, // mSharpWeaponAddCrit
        size_of::<NXParameter<S32>>() as u32, // mSharpWeaponAddGuardMin
        size_of::<NXParameter<S32>>() as u32, // mSharpWeaponAddGuardMax
        size_of::<NXParameter<S32>>() as u32, // mPoweredSharpAddAtkMin
        size_of::<NXParameter<S32>>() as u32, // mPoweredSharpAddAtkMax
        size_of::<NXParameter<S32>>() as u32, // mPoweredSharpAddLifeMin
        size_of::<NXParameter<S32>>() as u32, // mPoweredSharpAddLifeMax
        size_of::<NXParameter<S32>>() as u32, // mPoweredSharpWeaponAddGuardMin
        size_of::<NXParameter<S32>>() as u32, // mPoweredSharpWeaponAddGuardMax
        size_of::<NXParameter<F32>>() as u32, // mPoweredSharpAddThrowMin
        size_of::<NXParameter<F32>>() as u32, // mPoweredSharpAddThrowMax
        size_of::<NXParameter<Bool32>>() as u32, // mPoweredSharpAddSpreadFire
        size_of::<NXParameter<Bool32>>() as u32, // mPoweredSharpAddZoomRapid
        size_of::<NXParameter<F32>>() as u32, // mPoweredSharpAddRapidFireMin
        size_of::<NXParameter<F32>>() as u32, // mPoweredSharpAddRapidFireMax
        size_of::<NXParameter<Bool32>>() as u32, // mPoweredSharpAddSurfMaster
        size_of::<NXParameterObj>() as u32
    ], &4),
    "WeaponOption" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<Vector3f>>() as u32, // mPlayerHoldTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mPlayerHoldRotOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mNPCHoldTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mNPCHoldRotOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mNPCEquipTransOffset
        size_of::<NXParameter<Vector3f>>() as u32, // mNPCEquipRotOffset
        size_of::<NXParameterObj>() as u32
    ], &4),
    "WeaponThrow" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<F32>>() as u32, // mThrowSpeed
        size_of::<NXParameter<F32>>() as u32, // mThrowRotSpeed
        size_of::<NXParameter<F32>>() as u32, // mThrowDist
        size_of::<NXParameter<Vector3f>>() as u32, // mThrowRigidBodyBaseAxis
        size_of::<NXParameterObj>() as u32
    ], &4),
    "WizzRobe" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<S32>>() as u32, // mMagicWeatherType
        size_of::<NXParameter<NXSafeString>>() as u32, // mMagicFallActorName
        size_of::<NXParameter<F32>>() as u32, // mMagicFallIgniteRotSpd
        size_of::<NXParameter<F32>>() as u32, // mMagicFallOffsetY
        size_of::<NXParameter<F32>>() as u32, // mMagicFallCenterOffsetXZ
        size_of::<NXParameter<F32>>() as u32, // mMagicFallRandRadius
        size_of::<NXParameter<F32>>() as u32, // mMagicFallIntervalMax
        size_of::<NXParameter<F32>>() as u32, // mMagicFallIntervalMin
        size_of::<NXParameter<NXSafeString>>() as u32, // mSummonActorName
        size_of::<NXParameterObj>() as u32
    ], &4),
    "WolfLink" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<F32>>() as u32, // mNeckSpeedWait
        size_of::<NXParameter<F32>>() as u32, // mNeckRateWait
        size_of::<NXParameter<F32>>() as u32, // mNeckSpeedShiekSensor
        size_of::<NXParameter<F32>>() as u32, // mNeckRateShiekSensor
        size_of::<NXParameter<F32>>() as u32, // mNeckSpeedFollow
        size_of::<NXParameter<F32>>() as u32, // mNeckRateFollow
        size_of::<NXParameter<F32>>() as u32, // mNeckSpeedBattle
        size_of::<NXParameter<F32>>() as u32, // mNeckRateBattle
        size_of::<NXParameter<F32>>() as u32, // mNeckSpeedHeal
        size_of::<NXParameter<F32>>() as u32, // mNeckRateHeal
        size_of::<NXParameter<F32>>() as u32, // mBattleRange
        size_of::<NXParameter<F32>>() as u32, // mHealRange
        size_of::<NXParameter<F32>>() as u32, // mHuntRange
        size_of::<NXParameter<F32>>() as u32, // mHowlRange
        size_of::<NXParameter<F32>>() as u32, // mMaxHeightAttackable
        size_of::<NXParameter<F32>>() as u32, // mMaxHeightHealable
        size_of::<NXParameter<F32>>() as u32, // mNavMeshSearchRadius
        size_of::<NXParameter<F32>>() as u32, // mCanReachPlayerNavMeshSearchRadius
        size_of::<NXParameter<F32>>() as u32, // mSubmergedDepth
        size_of::<NXParameter<F32>>() as u32, // mUtilityLifeToHunt
        size_of::<NXParameter<F32>>() as u32, // mUtilityDangerDistMin
        size_of::<NXParameter<F32>>() as u32, // mUtilityDangerDistMax
        size_of::<NXParameter<F32>>() as u32, // mUtilityConstant
        size_of::<NXParameter<F32>>() as u32, // mChainAttackChargeMin
        size_of::<NXParameter<F32>>() as u32, // mChainAttackChargeMax
        size_of::<NXParameter<F32>>() as u32, // mLookAtCooldownWait
        size_of::<NXParameter<F32>>() as u32, // mLookAtCooldownWaitRand
        size_of::<NXParameter<F32>>() as u32, // mLookAtCounterWait
        size_of::<NXParameter<F32>>() as u32, // mLookAtCounterWaitRand
        size_of::<NXParameter<F32>>() as u32, // mLookAtCooldownRun
        size_of::<NXParameter<F32>>() as u32, // mLookAtCooldownRunRand
        size_of::<NXParameter<F32>>() as u32, // mLookAtCounterRun
        size_of::<NXParameter<F32>>() as u32, // mLookAtCounterRunRand
        size_of::<NXParameter<F32>>() as u32, // mAttackCounterLength
        size_of::<NXParameter<F32>>() as u32, // mAttackCounterRand
        size_of::<NXParameter<F32>>() as u32, // mHowlCooldownCounterLength
        size_of::<NXParameter<F32>>() as u32, // mHowlCooldownCounterRand
        size_of::<NXParameter<F32>>() as u32, // mHealCooldownCounterLength
        size_of::<NXParameter<F32>>() as u32, // mHealCooldownCounterRand
        size_of::<NXParameter<F32>>() as u32, // mFailPathCooldownCounterLength
        size_of::<NXParameter<F32>>() as u32, // mFailPathCooldownCounterRand
        size_of::<NXParameter<F32>>() as u32, // mRetargetCooldownCounterLength
        size_of::<NXParameter<F32>>() as u32, // mRetargetCooldownCounterRand
        size_of::<NXParameter<F32>>() as u32, // mAfterTargetDeathCounterLength
        size_of::<NXParameter<F32>>() as u32, // mAfterTargetDeathCounterRand
        size_of::<NXParameter<F32>>() as u32, // mLostTargetCounterLength
        size_of::<NXParameter<F32>>() as u32, // mLostTargetCounterRand
        size_of::<NXParameter<F32>>() as u32, // mInvinceableCounterLength
        size_of::<NXParameter<F32>>() as u32, // mInvinceableCounterRand
        size_of::<NXParameter<F32>>() as u32, // mCallDelayMinLength
        size_of::<NXParameter<F32>>() as u32, // mCallOverrideCounterLength
        size_of::<NXParameter<F32>>() as u32, // mGiveUpShiekSensorLength
        size_of::<NXParameter<F32>>() as u32, // mRetryShiekSensorLength
        size_of::<NXParameter<F32>>() as u32, // mBattleWallHitLength
        size_of::<NXParameter<F32>>() as u32, // mFollowRetryLength
        size_of::<NXParameter<F32>>() as u32, // mPowerUpFoodLength
        size_of::<NXParameter<F32>>() as u32, // mSafePosFailCounter
        size_of::<NXParameter<F32>>() as u32, // mRestrictedTargetTimeNormal
        size_of::<NXParameter<F32>>() as u32, // mRestrictedTargetTimeSpecial
        size_of::<NXParameter<S32>>() as u32, // mPowerUpFoodAttackMod
        size_of::<NXParameter<F32>>() as u32, // mPowerUpFoodChainAttackCharge
        size_of::<NXParameter<S32>>() as u32, // mVSStalfosCritChance
        size_of::<NXParameter<F32>>() as u32, // mAttackBase
        size_of::<NXParameter<F32>>() as u32, // mAttackHeartMod
        size_of::<NXParameter<F32>>() as u32, // mDefenseBase
        size_of::<NXParameter<F32>>() as u32, // mDefenseHeartMod
        size_of::<NXParameterObj>() as u32
    ], &4),
    "Zora" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<NXParameter<F32>>() as u32, // mInWaterDepth
        size_of::<NXParameter<F32>>() as u32, // mFloatDepth
        size_of::<NXParameter<F32>>() as u32, // mFloatRadius
        size_of::<NXParameter<F32>>() as u32, // mFloatCycleTime
        size_of::<NXParameter<F32>>() as u32, // mChangeDepthSpeed
        size_of::<NXParameterObj>() as u32
    ], &4),
};

fn parse_size_nx(b: &[u8]) -> u32 {
    let a = ParameterIO::from_binary(b).unwrap();
    let mut total_size = BGPARAM_OVERHEAD_NX;
    for (name, size) in OBJ_SIZES_NX.into_iter() {
        if let Some(_) = a.param_root.objects.get(*name) {
            total_size += size;
        }
    }
    total_size
}
