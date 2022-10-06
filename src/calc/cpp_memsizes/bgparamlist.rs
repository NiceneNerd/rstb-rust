use roead::aamp::ParameterIO;
use crate::Endian;

use std::mem::size_of;
use phf::{Map, phf_map};

use super::cpp_align;
use super::cpp_classes::{
    wiiu,
    nx,
    Bool32,
    S32,
    F32,
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
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "AnimalFollowOffset" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "AnimalUnit" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Armor" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "ArmorEffect" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "ArmorHead" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "ArmorUpper" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Arrow" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Attack" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "AttackInterval" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "AutoGen" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Beam" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "BindActor" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "BindBone" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Bow" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Bullet" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Camera" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "ChemicalType" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "ClothReaction" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "CookSpice" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "CureItem" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "EatTarget" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Enemy" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "EnemyLevel" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "EnemyRace" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "EnemyShown" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Event" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "ExtendedEntity" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Fish" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "GelEnemy" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "General" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "GiantArmor" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "GiantArmorSlot" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Global" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEnemyLifeGageDist
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEnemyNoSkitDist
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEnemyWeaponPickAllowDist
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mEnemyWeaponPickForbidTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEnemyAnimalNoDamageDist
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEnemyNearCraeteIDDelay
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mEnemyForceTiredLODCount
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mEnemyForceTiredNoSightLODCount
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mEnemyForceWarpReturnLODCount
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSilentAttackAng
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSilentAttackRatio
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mBlownOffPlayerAtkDelay
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mJustAvoidAcceptWpRangeSS
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mJustAvoidAcceptWpRangeLS
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mJustAvoidAcceptWpRangeSP
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mForceNoticeEnemyCount
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mForceNoticeEnemyDist
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mWeaponRickeyLife
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWeaponDropRotSpd
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mShieldRideBaseFrame
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mShieldRideHitBaseDamage
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mShieldDamageratio
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mShieldSurfMasterFrictionRatio
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLoudNoiseRadius
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mImpulse2DamageRatio
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mIceMeltSpeedOnContactFire
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mCriticalAttackRatio
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mBooerangAttackRatio
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHitImpulseClampMax
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemVelXZFromBomb
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemVelYFromBomb
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemVelRandomFromBomb
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemAngVelFromBomb
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemAngVelRandomFromBomb
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemVelXZSmall
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemVelYSmall
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemVelRandomSmall
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemAngVelSmall
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemAngVelRandomSmall
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemVelXZLarge
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemVelYLarge
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemVelRandomLarge
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemAngVelLarge
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemAngVelRandomLarge
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemVelXZRupeeRabbit
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemVelYRupeeRabbit
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemVelRandomRupeeRabbit
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemVelXZItemRupeeOnly
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemVelYItemRupeeOnly
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemVelRandomItemRupeeOnly
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDropItemInvincibleTime
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mTreeWeaponEquipTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mTreeWeaponEquipRotOffset
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWetRatioToDie
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEnvWetRatioToDie
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNPCTurnAngleDiff
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mNPCWaitFrameAfterEvent
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNPCIgnorePlayerTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNPCCancelIgnorePlayerTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNPCOpenDoorDistance
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNPCWalkRateOnSandAndSnow
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNPCDownVerticallyAngle
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mGerudoQueenSafetyAreaRadius
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mCreateFairyLimitCount
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mTerrorRegistSpeed
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mTerrorUnregistSpeed
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mTerrorRegistTimer
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mTerrorRadiusOffset
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mSpeedTerrorLevel
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mSpeedTerrorLevelHuge
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSpeedTerrorLevelCheckRadius
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mAtDirTypeAffectRatio
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mRainyAwnHearingLevel
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHorseBindOffsetYOfMaleUMii
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHorseBindOffsetYOfFemaleUMii
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mHorseFamiliarityIncreasePerFrame
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheAtFirstRun
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheAfterRun
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheAfterGearTop
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheAfterJump
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheWhileResisting
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseEat
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mHorseAlertProbability
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mHorseAlertFramesMin
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mHorseAlertFramesMax
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mHorseExtraChargeNum
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mPlayerGrabThrowDiffRate
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Golem" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "GolemIK" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Grab" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Guardian" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "GuardianMini" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "GuardianMiniWeapon" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Horse" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "HorseCreator" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "HorseObject" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "HorseRider" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "HorseTargetedInfo" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "HorseUnit" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Insect" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Item" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "LargeSword" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Liftable" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "LumberjackTree" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "MasterSword" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "MonsterShop" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Motorcycle" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mPitchDampingCoefficient
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDriftAllowSpeedKPH
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDriftAbortSpeedKPH
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDriftAllowSteerRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDriftAbortSteerRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDriftRearAngleRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDriftSpeedRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mManualWheelieAllowAngleFront
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mManualWheelieAllowAngleRear
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mManualWheelieLastSec
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWheelieLastSecInMidAir
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mManualControlProhibitSecAfterWheelie
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWheelieRevertPower
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWheelieRevertPowerSec
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mManualWheelieRiseDegDelta
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWheelieLaunchRiseDegDelta
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEngineBrakeMaxPower
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mBackwardEngineBrakePower
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSlipStartAngle
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSlipThresholdPower
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSlipPowerMax
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mWristBindRotation
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mWristBindTranslation
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mPostureLimitAngle
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mInvalidPostureLimitSec
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mFallOverThresholdAngle
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mJumpIntervalSec
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mFullEnergyLastSec
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWheelieLaunchJumpProhibitSec
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSlowModeTargetSpeedKPH2
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSlowDriftTargetSpeedKPH2
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSlowModeTransitionSec
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSlowSlipThresholdKPH
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSlowSlipThresholdPower
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSlowSlipThresholdSec
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mJumpRearWheelRotateRadPerSec
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWeaponThrowModeSpeedKPH2
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Nest" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Npc" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "NpcEquipment" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "PictureBook" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Player" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mBombReloadTime1
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mBombReloadTime2
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mStopTimerReloadTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mStopTimerBlowAngle
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mStopTimerBlowSpeedLimit
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mStopTimerImpluseMaxCountSmallSword
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mStopTimerImpluseMaxCountLargeSword
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mStopTimerImpluseMaxCountSpear
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mStopTimerCancelDeleteWaitTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mStopTimerLongTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mStopTimerMiddleTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mStopTimerShortTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEnergyTiredValue
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEnergyBowSlow
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEnergyPush
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEnergyCharge
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEnergyAutoRecover
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEnergyAutoRecoverInAir
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEnergyAutoRecoverInvalidTime1
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEnergyAutoRecoverInvalidTime2
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mColdTempDamageAdd
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHotTempDamageAdd
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mTempDamage
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mTempEnergyDecDiamAdd
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mTempEnergyDecDegAdd
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mVelDiamSand
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mVelDiamTired
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mStickDiamTired
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mAutoRecoverNum
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mAutoRecoverIntervalMin
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mAutoRecoverIntervalMax
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mAutoRecoverInvalidTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mBowSubjectContTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLNGStickScale
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLATStickScale
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLNGGyroScale
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLATGyroScale
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mBowSlowShootNum
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mBowSlowRateDiam
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mBowSlowMaxTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDiveBowSlowMaxTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mBowSlowInvalidTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mBowSlowInvalidHeight
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mBowSlowInvalidHeightOnShield
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mBowSlowInvalidHeightWeaponChange
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mGuardJustForceSlowTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mMoveMaxDecRateByWater
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mMoveIgnoreWaterHeight
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mMoveDecRateByBog
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mMoveDecRateMaxHeight
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mMaxForce
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mMinForce
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mAddForce
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSnowBallAddForce
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLogPushF
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mRockPushF
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mRockPushSpeed
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaistAngleUpperMax
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaistAngleLowerMax
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaistAngleSideMax
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNoSquatWaterHeight
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mInvalidReloadTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWeaponThrowSpeedY
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWeaponThrowSpeedF
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWeaponThrowSpeedFSquat
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDashUpEnableAngle
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mShockTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mIceInvalidTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mMaxSpeedInAir
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mTurnEnableSpeed
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mTurnEnableStickSub
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mTurnEnableDirSub
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mShortDashImpulse
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mShortDashDamage
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSwordTerrorScope
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mArrowTerrorScope
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mTorchTerrorScope
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mTorchTerrorOffsetY
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mTorchTerrorOffsetZ
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDashNoise
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWhistleNoise
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mClimbEnableAngle
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mClimbEnableSpeedMinAngle
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mClimbEnableSpeedMaxAngle
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSlipEnableSpeed
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSlipSpeedAddMin
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSlipSpeedAddMax
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSlipSpeedAddDiamByRain
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mMagnetAim2DPosOffsetY
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookPosOffsetXZ
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookPosOffsetY
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookPosOffsetYSquat
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookPosOffsetYSwim
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookPosOffsetYHorse
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookEnableAngle
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHitSlowTimeS
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHitSlowTimeM
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHitSlowTimeL
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHitSlowRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHitStopTimeS
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHitStopTimeL
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHitStopRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mAtnPosInterPolationRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mAtnPosInterPolationMin
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mAtnPosInterPolationMax
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mPredictDiffAngleMax
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDashToRunStickValueDec
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWindSupportReuseTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mFireSupportReuseTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mElectricSupportReuseTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaterSupportReuseTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWindSupportTimerRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mFireSupportTimerRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mElectricSupportTimerRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaterSupportTimerRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mChemicalInvalidTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mAutoDashUpTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mAutoDashUpAngle
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mClimbRestartHeight
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mClimbRestartTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mPushNoticeLookTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEnergyUseSmall
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEnergyUseLarge
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNoEnergyDashInterval
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mGuardableAngle
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mStickMaxInStore
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookContinueTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mPostureContinueTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mItemUseModelAlpha
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLadderCheckSide
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLadderCheckDist
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mNoDeathDamageBase
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mNoDeathDamageAdd
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mArmorCompSwimEnergyRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mArmorCompRegistElecFrame
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mArmorCompNightSpeedRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mArmorCompClimbJumpEnergyRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mArmorCompPlusDropRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mArmorCompWeaponBrakeRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mArmorCompSwordBeamAttackRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mArmorCompAncientAttackRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mArmorCompBoneAttackRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mArmorCompTerrorLevel
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mArmorCompTerrorRadius
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mArmorCompNakedSwimSpeedRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mArmorCompNakedSwimAnimeRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mArmorCompNakedSwimEnergyRate
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mArmorAncientAttackRate
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mSupportWindNum
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mSupportElectricNum
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSupportElectricEnergy
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mSupportFireNum
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mSupportWaterLifeAdd
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSupportWaterEnergyAdd
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mStickRInputFrame
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDiffAngleFromLookVec
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookPosOffset
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookFixAngle
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookContinueTimeToCamera
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mCutKnockBackNoCrrDist
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitUnsteadyApplyVel
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mCurseAddWeight
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mRoofCrashVel
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mCutJumpInvalidTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaterDepthInGrudge
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLargeHorseLegBendAngY
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLargeHorseLegBendAngX
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLargeHorseLegBendFrame
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNoMaskPauseWaterHeight
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookAtThreshold
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Prey" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Rod" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Rope" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Rupee" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Sandworm" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "SeriesArmor" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "ShiekerStone" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Shield" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32,
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "SmallSword" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mPodName
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mPlayerHoldTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mPlayerHoldRotOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mPlayerEquipTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mPlayerEquipRotOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mRideHorsePlayerHoldTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mRideHorsePlayerHoldRotOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mAffectTransOffsetShield
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mAffectRotOffsetShield
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mAffectTransOffsetBow
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mAffectRotOffsetBow
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mSquatPlayerHoldTransAddOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mSquatPlayerHoldRotAddOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mNPCHoldTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mNPCHoldRotOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mNPCEquipTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mNPCEquipRotOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mEnemyEquipTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mEnemyEquipRotOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mStandEquipTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mStandEquipRotOffset
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWeaponSubType
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Spear" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mPodName
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mPlayerHoldTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mPlayerHoldRotOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mPlayerEquipTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mPlayerEquipRotOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mRideHorsePlayerHoldTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mRideHorsePlayerHoldRotOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mAffectTransOffsetShield
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mAffectRotOffsetShield
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mAffectTransOffsetBow
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mAffectRotOffsetBow
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mGrabPlayerHoldTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mGrabPlayerHoldRotOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mGrabAffectTransOffsetShield
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mGrabAffectRotOffsetShield
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mSquatPlayerHoldTransAddOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mSquatPlayerHoldRotAddOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mNPCHoldTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mNPCHoldRotOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mNPCEquipTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mNPCEquipRotOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mEnemyEquipTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mEnemyEquipRotOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mStandEquipTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mStandEquipRotOffset
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWeaponSubType
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "StalEnemy" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mHeadActorName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mLeftArmActorName
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Swarm" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mSwarmSubActorNum
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mSwarmPattern
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mDeadActorName
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "System" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSameGroupActorName
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32, // mIsGetItemSelf
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Traveler" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mAppearGameDataName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mDeleteGameDataName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mRouteType
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mRideHorseName
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32, // mIsLeadHorse
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mHorseGearLevel
        // 1
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 2
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 3
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 4
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 5
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 6
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 7
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 8
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 9
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 10
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 11
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 12
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 13
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 14
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 15
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 16
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 17
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 18
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 19
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 20
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 21
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 22
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 23
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 24
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 25
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 26
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 27
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 28
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mEntryPoint
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSchedule
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMoveAS
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mWaitAS
        // 29
        // size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mName
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "WeaponCommon" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mPlayerEqScale
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mEnemyEqScale
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mGuardPower
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mRank
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32, // mIsHammer
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32, // mIsWeakBreaker
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32, // mIsBoomerang
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32, // mIsBlunt
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32, // mIsLuckyWeapon
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32, // mIsPikohan
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32, // mIsThrowingWeapon
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32, // mIsThrowingBreakWeapon
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mThrowRange
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mDreadActor
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mThroughActor
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mNPCWeaponType
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32, // mIsNotOnTerrorHold
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32, // mIsAsOffUnEquiped
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mChemicalEnergyMax
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mChemicalEnergyAmountUsed
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mChemicalEnergyRecoverRate
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mChemicalEnergyRecoverInterval
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mStickDamage
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mShootBeam
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mDropFromPorchRot
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSharpWeaponPer
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mSharpWeaponAddAtkMin
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mSharpWeaponAddAtkMax
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mSharpWeaponAddLifeMin
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mSharpWeaponAddLifeMax
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32, // mSharpWeaponAddCrit
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mSharpWeaponAddGuardMin
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mSharpWeaponAddGuardMax
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mPoweredSharpAddAtkMin
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mPoweredSharpAddAtkMax
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mPoweredSharpAddLifeMin
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mPoweredSharpAddLifeMax
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mPoweredSharpWeaponAddGuardMin
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mPoweredSharpWeaponAddGuardMax
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mPoweredSharpAddThrowMin
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mPoweredSharpAddThrowMax
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32, // mPoweredSharpAddSpreadFire
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32, // mPoweredSharpAddZoomRapid
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mPoweredSharpAddRapidFireMin
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mPoweredSharpAddRapidFireMax
        size_of::<wiiu::agl::Parameter<Bool32>>() as u32, // mPoweredSharpAddSurfMaster
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "WeaponOption" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mPlayerHoldTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mPlayerHoldRotOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mNPCHoldTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mNPCHoldRotOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mNPCEquipTransOffset
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mNPCEquipRotOffset
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "WeaponThrow" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mThrowSpeed
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mThrowRotSpeed
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mThrowDist
        size_of::<wiiu::agl::Parameter<Vector3f>>() as u32, // mThrowRigidBodyBaseAxis
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "WizzRobe" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mMagicWeatherType
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mMagicFallActorName
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mMagicFallIgniteRotSpd
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mMagicFallOffsetY
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mMagicFallCenterOffsetXZ
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mMagicFallRandRadius
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mMagicFallIntervalMax
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mMagicFallIntervalMin
        size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32, // mSummonActorName
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "WolfLink" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNeckSpeedWait
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNeckRateWait
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNeckSpeedShiekSensor
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNeckRateShiekSensor
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNeckSpeedFollow
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNeckRateFollow
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNeckSpeedBattle
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNeckRateBattle
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNeckSpeedHeal
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNeckRateHeal
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mBattleRange
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHealRange
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHuntRange
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHowlRange
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mMaxHeightAttackable
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mMaxHeightHealable
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mNavMeshSearchRadius
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mCanReachPlayerNavMeshSearchRadius
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSubmergedDepth
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mUtilityLifeToHunt
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mUtilityDangerDistMin
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mUtilityDangerDistMax
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mUtilityConstant
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mChainAttackChargeMin
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mChainAttackChargeMax
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookAtCooldownWait
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookAtCooldownWaitRand
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookAtCounterWait
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookAtCounterWaitRand
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookAtCooldownRun
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookAtCooldownRunRand
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookAtCounterRun
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLookAtCounterRunRand
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mAttackCounterLength
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mAttackCounterRand
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHowlCooldownCounterLength
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHowlCooldownCounterRand
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHealCooldownCounterLength
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mHealCooldownCounterRand
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mFailPathCooldownCounterLength
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mFailPathCooldownCounterRand
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mRetargetCooldownCounterLength
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mRetargetCooldownCounterRand
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mAfterTargetDeathCounterLength
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mAfterTargetDeathCounterRand
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLostTargetCounterLength
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mLostTargetCounterRand
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mInvinceableCounterLength
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mInvinceableCounterRand
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mCallDelayMinLength
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mCallOverrideCounterLength
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mGiveUpShiekSensorLength
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mRetryShiekSensorLength
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mBattleWallHitLength
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mFollowRetryLength
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mPowerUpFoodLength
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mSafePosFailCounter
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mRestrictedTargetTimeNormal
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mRestrictedTargetTimeSpecial
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mPowerUpFoodAttackMod
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mPowerUpFoodChainAttackCharge
        size_of::<wiiu::agl::Parameter<S32>>() as u32, // mVSStalfosCritChance
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mAttackBase
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mAttackHeartMod
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDefenseBase
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mDefenseHeartMod
        size_of::<wiiu::agl::ParameterObj>() as u32
    ], &4),
    "Zora" => cpp_align(&[
        size_of::<u32>() as u32,
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mInWaterDepth
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mFloatDepth
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mFloatRadius
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mFloatCycleTime
        size_of::<wiiu::agl::Parameter<F32>>() as u32, // mChangeDepthSpeed
        size_of::<wiiu::agl::ParameterObj>() as u32
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
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "AnimalFollowOffset" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "AnimalUnit" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Armor" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "ArmorEffect" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "ArmorHead" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "ArmorUpper" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Arrow" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Attack" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "AttackInterval" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "AutoGen" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Beam" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "BindActor" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "BindBone" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Bow" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Bullet" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Camera" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "ChemicalType" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "ClothReaction" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "CookSpice" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "CureItem" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "EatTarget" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Enemy" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "EnemyLevel" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "EnemyRace" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "EnemyShown" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Event" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "ExtendedEntity" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Fish" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "GelEnemy" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "General" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "GiantArmor" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "GiantArmorSlot" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Global" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEnemyLifeGageDist
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEnemyNoSkitDist
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEnemyWeaponPickAllowDist
        size_of::<nx::agl::Parameter<S32>>() as u32, // mEnemyWeaponPickForbidTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEnemyAnimalNoDamageDist
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEnemyNearCraeteIDDelay
        size_of::<nx::agl::Parameter<S32>>() as u32, // mEnemyForceTiredLODCount
        size_of::<nx::agl::Parameter<S32>>() as u32, // mEnemyForceTiredNoSightLODCount
        size_of::<nx::agl::Parameter<S32>>() as u32, // mEnemyForceWarpReturnLODCount
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSilentAttackAng
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSilentAttackRatio
        size_of::<nx::agl::Parameter<S32>>() as u32, // mBlownOffPlayerAtkDelay
        size_of::<nx::agl::Parameter<F32>>() as u32, // mJustAvoidAcceptWpRangeSS
        size_of::<nx::agl::Parameter<F32>>() as u32, // mJustAvoidAcceptWpRangeLS
        size_of::<nx::agl::Parameter<F32>>() as u32, // mJustAvoidAcceptWpRangeSP
        size_of::<nx::agl::Parameter<S32>>() as u32, // mForceNoticeEnemyCount
        size_of::<nx::agl::Parameter<F32>>() as u32, // mForceNoticeEnemyDist
        size_of::<nx::agl::Parameter<S32>>() as u32, // mWeaponRickeyLife
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWeaponDropRotSpd
        size_of::<nx::agl::Parameter<S32>>() as u32, // mShieldRideBaseFrame
        size_of::<nx::agl::Parameter<S32>>() as u32, // mShieldRideHitBaseDamage
        size_of::<nx::agl::Parameter<F32>>() as u32, // mShieldDamageratio
        size_of::<nx::agl::Parameter<F32>>() as u32, // mShieldSurfMasterFrictionRatio
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLoudNoiseRadius
        size_of::<nx::agl::Parameter<F32>>() as u32, // mImpulse2DamageRatio
        size_of::<nx::agl::Parameter<F32>>() as u32, // mIceMeltSpeedOnContactFire
        size_of::<nx::agl::Parameter<F32>>() as u32, // mCriticalAttackRatio
        size_of::<nx::agl::Parameter<F32>>() as u32, // mBooerangAttackRatio
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHitImpulseClampMax
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemVelXZFromBomb
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemVelYFromBomb
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemVelRandomFromBomb
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemAngVelFromBomb
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemAngVelRandomFromBomb
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemVelXZSmall
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemVelYSmall
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemVelRandomSmall
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemAngVelSmall
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemAngVelRandomSmall
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemVelXZLarge
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemVelYLarge
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemVelRandomLarge
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemAngVelLarge
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemAngVelRandomLarge
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemVelXZRupeeRabbit
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemVelYRupeeRabbit
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemVelRandomRupeeRabbit
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemVelXZItemRupeeOnly
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemVelYItemRupeeOnly
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemVelRandomItemRupeeOnly
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDropItemInvincibleTime
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mTreeWeaponEquipTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mTreeWeaponEquipRotOffset
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWetRatioToDie
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEnvWetRatioToDie
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNPCTurnAngleDiff
        size_of::<nx::agl::Parameter<S32>>() as u32, // mNPCWaitFrameAfterEvent
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNPCIgnorePlayerTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNPCCancelIgnorePlayerTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNPCOpenDoorDistance
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNPCWalkRateOnSandAndSnow
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNPCDownVerticallyAngle
        size_of::<nx::agl::Parameter<F32>>() as u32, // mGerudoQueenSafetyAreaRadius
        size_of::<nx::agl::Parameter<S32>>() as u32, // mCreateFairyLimitCount
        size_of::<nx::agl::Parameter<F32>>() as u32, // mTerrorRegistSpeed
        size_of::<nx::agl::Parameter<F32>>() as u32, // mTerrorUnregistSpeed
        size_of::<nx::agl::Parameter<S32>>() as u32, // mTerrorRegistTimer
        size_of::<nx::agl::Parameter<F32>>() as u32, // mTerrorRadiusOffset
        size_of::<nx::agl::Parameter<S32>>() as u32, // mSpeedTerrorLevel
        size_of::<nx::agl::Parameter<S32>>() as u32, // mSpeedTerrorLevelHuge
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSpeedTerrorLevelCheckRadius
        size_of::<nx::agl::Parameter<F32>>() as u32, // mAtDirTypeAffectRatio
        size_of::<nx::agl::Parameter<F32>>() as u32, // mRainyAwnHearingLevel
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHorseBindOffsetYOfMaleUMii
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHorseBindOffsetYOfFemaleUMii
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mHorseFamiliarityIncreasePerFrame
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheAtFirstRun
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheAfterRun
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheAfterGearTop
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheAfterJump
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseSootheWhileResisting
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mHorseFamiliarityIncreaseEat
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mHorseAlertProbability
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mHorseAlertFramesMin
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mHorseAlertFramesMax
        size_of::<nx::agl::Parameter<S32>>() as u32, // mHorseExtraChargeNum
        size_of::<nx::agl::Parameter<F32>>() as u32, // mPlayerGrabThrowDiffRate
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Golem" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "GolemIK" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Grab" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Guardian" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "GuardianMini" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "GuardianMiniWeapon" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Horse" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "HorseCreator" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "HorseObject" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "HorseRider" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "HorseTargetedInfo" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "HorseUnit" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Insect" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Item" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "LargeSword" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Liftable" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "LumberjackTree" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "MasterSword" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "MonsterShop" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Motorcycle" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32, // mPitchDampingCoefficient
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDriftAllowSpeedKPH
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDriftAbortSpeedKPH
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDriftAllowSteerRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDriftAbortSteerRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDriftRearAngleRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDriftSpeedRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mManualWheelieAllowAngleFront
        size_of::<nx::agl::Parameter<F32>>() as u32, // mManualWheelieAllowAngleRear
        size_of::<nx::agl::Parameter<F32>>() as u32, // mManualWheelieLastSec
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWheelieLastSecInMidAir
        size_of::<nx::agl::Parameter<F32>>() as u32, // mManualControlProhibitSecAfterWheelie
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWheelieRevertPower
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWheelieRevertPowerSec
        size_of::<nx::agl::Parameter<F32>>() as u32, // mManualWheelieRiseDegDelta
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWheelieLaunchRiseDegDelta
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEngineBrakeMaxPower
        size_of::<nx::agl::Parameter<F32>>() as u32, // mBackwardEngineBrakePower
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSlipStartAngle
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSlipThresholdPower
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSlipPowerMax
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mWristBindRotation
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mWristBindTranslation
        size_of::<nx::agl::Parameter<F32>>() as u32, // mPostureLimitAngle
        size_of::<nx::agl::Parameter<F32>>() as u32, // mInvalidPostureLimitSec
        size_of::<nx::agl::Parameter<F32>>() as u32, // mFallOverThresholdAngle
        size_of::<nx::agl::Parameter<F32>>() as u32, // mJumpIntervalSec
        size_of::<nx::agl::Parameter<F32>>() as u32, // mFullEnergyLastSec
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWheelieLaunchJumpProhibitSec
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSlowModeTargetSpeedKPH2
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSlowDriftTargetSpeedKPH2
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSlowModeTransitionSec
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSlowSlipThresholdKPH
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSlowSlipThresholdPower
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSlowSlipThresholdSec
        size_of::<nx::agl::Parameter<F32>>() as u32, // mJumpRearWheelRotateRadPerSec
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWeaponThrowModeSpeedKPH2
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Nest" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Npc" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "NpcEquipment" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "PictureBook" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Player" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32, // mBombReloadTime1
        size_of::<nx::agl::Parameter<F32>>() as u32, // mBombReloadTime2
        size_of::<nx::agl::Parameter<F32>>() as u32, // mStopTimerReloadTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mStopTimerBlowAngle
        size_of::<nx::agl::Parameter<F32>>() as u32, // mStopTimerBlowSpeedLimit
        size_of::<nx::agl::Parameter<S32>>() as u32, // mStopTimerImpluseMaxCountSmallSword
        size_of::<nx::agl::Parameter<S32>>() as u32, // mStopTimerImpluseMaxCountLargeSword
        size_of::<nx::agl::Parameter<S32>>() as u32, // mStopTimerImpluseMaxCountSpear
        size_of::<nx::agl::Parameter<F32>>() as u32, // mStopTimerCancelDeleteWaitTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mStopTimerLongTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mStopTimerMiddleTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mStopTimerShortTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEnergyTiredValue
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEnergyBowSlow
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEnergyPush
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEnergyCharge
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEnergyAutoRecover
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEnergyAutoRecoverInAir
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEnergyAutoRecoverInvalidTime1
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEnergyAutoRecoverInvalidTime2
        size_of::<nx::agl::Parameter<F32>>() as u32, // mColdTempDamageAdd
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHotTempDamageAdd
        size_of::<nx::agl::Parameter<F32>>() as u32, // mTempDamage
        size_of::<nx::agl::Parameter<F32>>() as u32, // mTempEnergyDecDiamAdd
        size_of::<nx::agl::Parameter<F32>>() as u32, // mTempEnergyDecDegAdd
        size_of::<nx::agl::Parameter<F32>>() as u32, // mVelDiamSand
        size_of::<nx::agl::Parameter<F32>>() as u32, // mVelDiamTired
        size_of::<nx::agl::Parameter<F32>>() as u32, // mStickDiamTired
        size_of::<nx::agl::Parameter<F32>>() as u32, // mAutoRecoverNum
        size_of::<nx::agl::Parameter<F32>>() as u32, // mAutoRecoverIntervalMin
        size_of::<nx::agl::Parameter<F32>>() as u32, // mAutoRecoverIntervalMax
        size_of::<nx::agl::Parameter<F32>>() as u32, // mAutoRecoverInvalidTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mBowSubjectContTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLNGStickScale
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLATStickScale
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLNGGyroScale
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLATGyroScale
        size_of::<nx::agl::Parameter<S32>>() as u32, // mBowSlowShootNum
        size_of::<nx::agl::Parameter<F32>>() as u32, // mBowSlowRateDiam
        size_of::<nx::agl::Parameter<F32>>() as u32, // mBowSlowMaxTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDiveBowSlowMaxTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mBowSlowInvalidTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mBowSlowInvalidHeight
        size_of::<nx::agl::Parameter<F32>>() as u32, // mBowSlowInvalidHeightOnShield
        size_of::<nx::agl::Parameter<F32>>() as u32, // mBowSlowInvalidHeightWeaponChange
        size_of::<nx::agl::Parameter<F32>>() as u32, // mGuardJustForceSlowTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mMoveMaxDecRateByWater
        size_of::<nx::agl::Parameter<F32>>() as u32, // mMoveIgnoreWaterHeight
        size_of::<nx::agl::Parameter<F32>>() as u32, // mMoveDecRateByBog
        size_of::<nx::agl::Parameter<F32>>() as u32, // mMoveDecRateMaxHeight
        size_of::<nx::agl::Parameter<F32>>() as u32, // mMaxForce
        size_of::<nx::agl::Parameter<F32>>() as u32, // mMinForce
        size_of::<nx::agl::Parameter<F32>>() as u32, // mAddForce
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSnowBallAddForce
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLogPushF
        size_of::<nx::agl::Parameter<F32>>() as u32, // mRockPushF
        size_of::<nx::agl::Parameter<F32>>() as u32, // mRockPushSpeed
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaistAngleUpperMax
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaistAngleLowerMax
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaistAngleSideMax
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNoSquatWaterHeight
        size_of::<nx::agl::Parameter<F32>>() as u32, // mInvalidReloadTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWeaponThrowSpeedY
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWeaponThrowSpeedF
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWeaponThrowSpeedFSquat
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDashUpEnableAngle
        size_of::<nx::agl::Parameter<F32>>() as u32, // mShockTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mIceInvalidTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mMaxSpeedInAir
        size_of::<nx::agl::Parameter<F32>>() as u32, // mTurnEnableSpeed
        size_of::<nx::agl::Parameter<F32>>() as u32, // mTurnEnableStickSub
        size_of::<nx::agl::Parameter<F32>>() as u32, // mTurnEnableDirSub
        size_of::<nx::agl::Parameter<S32>>() as u32, // mShortDashImpulse
        size_of::<nx::agl::Parameter<S32>>() as u32, // mShortDashDamage
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSwordTerrorScope
        size_of::<nx::agl::Parameter<F32>>() as u32, // mArrowTerrorScope
        size_of::<nx::agl::Parameter<F32>>() as u32, // mTorchTerrorScope
        size_of::<nx::agl::Parameter<F32>>() as u32, // mTorchTerrorOffsetY
        size_of::<nx::agl::Parameter<F32>>() as u32, // mTorchTerrorOffsetZ
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDashNoise
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWhistleNoise
        size_of::<nx::agl::Parameter<F32>>() as u32, // mClimbEnableAngle
        size_of::<nx::agl::Parameter<F32>>() as u32, // mClimbEnableSpeedMinAngle
        size_of::<nx::agl::Parameter<F32>>() as u32, // mClimbEnableSpeedMaxAngle
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSlipEnableSpeed
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSlipSpeedAddMin
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSlipSpeedAddMax
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSlipSpeedAddDiamByRain
        size_of::<nx::agl::Parameter<F32>>() as u32, // mMagnetAim2DPosOffsetY
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookPosOffsetXZ
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookPosOffsetY
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookPosOffsetYSquat
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookPosOffsetYSwim
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookPosOffsetYHorse
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookEnableAngle
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHitSlowTimeS
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHitSlowTimeM
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHitSlowTimeL
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHitSlowRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHitStopTimeS
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHitStopTimeL
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHitStopRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mAtnPosInterPolationRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mAtnPosInterPolationMin
        size_of::<nx::agl::Parameter<F32>>() as u32, // mAtnPosInterPolationMax
        size_of::<nx::agl::Parameter<F32>>() as u32, // mPredictDiffAngleMax
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDashToRunStickValueDec
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWindSupportReuseTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mFireSupportReuseTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mElectricSupportReuseTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaterSupportReuseTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWindSupportTimerRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mFireSupportTimerRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mElectricSupportTimerRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaterSupportTimerRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mChemicalInvalidTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mAutoDashUpTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mAutoDashUpAngle
        size_of::<nx::agl::Parameter<F32>>() as u32, // mClimbRestartHeight
        size_of::<nx::agl::Parameter<F32>>() as u32, // mClimbRestartTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mPushNoticeLookTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEnergyUseSmall
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEnergyUseLarge
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNoEnergyDashInterval
        size_of::<nx::agl::Parameter<F32>>() as u32, // mGuardableAngle
        size_of::<nx::agl::Parameter<F32>>() as u32, // mStickMaxInStore
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookContinueTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mPostureContinueTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mItemUseModelAlpha
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLadderCheckSide
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLadderCheckDist
        size_of::<nx::agl::Parameter<S32>>() as u32, // mNoDeathDamageBase
        size_of::<nx::agl::Parameter<S32>>() as u32, // mNoDeathDamageAdd
        size_of::<nx::agl::Parameter<F32>>() as u32, // mArmorCompSwimEnergyRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mArmorCompRegistElecFrame
        size_of::<nx::agl::Parameter<F32>>() as u32, // mArmorCompNightSpeedRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mArmorCompClimbJumpEnergyRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mArmorCompPlusDropRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mArmorCompWeaponBrakeRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mArmorCompSwordBeamAttackRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mArmorCompAncientAttackRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mArmorCompBoneAttackRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mArmorCompTerrorLevel
        size_of::<nx::agl::Parameter<F32>>() as u32, // mArmorCompTerrorRadius
        size_of::<nx::agl::Parameter<F32>>() as u32, // mArmorCompNakedSwimSpeedRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mArmorCompNakedSwimAnimeRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mArmorCompNakedSwimEnergyRate
        size_of::<nx::agl::Parameter<F32>>() as u32, // mArmorAncientAttackRate
        size_of::<nx::agl::Parameter<S32>>() as u32, // mSupportWindNum
        size_of::<nx::agl::Parameter<S32>>() as u32, // mSupportElectricNum
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSupportElectricEnergy
        size_of::<nx::agl::Parameter<S32>>() as u32, // mSupportFireNum
        size_of::<nx::agl::Parameter<S32>>() as u32, // mSupportWaterLifeAdd
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSupportWaterEnergyAdd
        size_of::<nx::agl::Parameter<F32>>() as u32, // mStickRInputFrame
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDiffAngleFromLookVec
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookPosOffset
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookFixAngle
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookContinueTimeToCamera
        size_of::<nx::agl::Parameter<F32>>() as u32, // mCutKnockBackNoCrrDist
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitUnsteadyApplyVel
        size_of::<nx::agl::Parameter<F32>>() as u32, // mCurseAddWeight
        size_of::<nx::agl::Parameter<F32>>() as u32, // mRoofCrashVel
        size_of::<nx::agl::Parameter<F32>>() as u32, // mCutJumpInvalidTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaterDepthInGrudge
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLargeHorseLegBendAngY
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLargeHorseLegBendAngX
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLargeHorseLegBendFrame
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNoMaskPauseWaterHeight
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookAtThreshold
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Prey" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Rod" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Rope" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Rupee" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Sandworm" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "SeriesArmor" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Bool32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "ShiekerStone" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Shield" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32,
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "SmallSword" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mPodName
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mPlayerHoldTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mPlayerHoldRotOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mPlayerEquipTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mPlayerEquipRotOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mRideHorsePlayerHoldTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mRideHorsePlayerHoldRotOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mAffectTransOffsetShield
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mAffectRotOffsetShield
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mAffectTransOffsetBow
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mAffectRotOffsetBow
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mSquatPlayerHoldTransAddOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mSquatPlayerHoldRotAddOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mNPCHoldTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mNPCHoldRotOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mNPCEquipTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mNPCEquipRotOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mEnemyEquipTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mEnemyEquipRotOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mStandEquipTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mStandEquipRotOffset
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWeaponSubType
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Spear" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mPodName
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mPlayerHoldTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mPlayerHoldRotOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mPlayerEquipTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mPlayerEquipRotOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mRideHorsePlayerHoldTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mRideHorsePlayerHoldRotOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mAffectTransOffsetShield
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mAffectRotOffsetShield
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mAffectTransOffsetBow
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mAffectRotOffsetBow
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mGrabPlayerHoldTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mGrabPlayerHoldRotOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mGrabAffectTransOffsetShield
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mGrabAffectRotOffsetShield
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mSquatPlayerHoldTransAddOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mSquatPlayerHoldRotAddOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mNPCHoldTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mNPCHoldRotOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mNPCEquipTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mNPCEquipRotOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mEnemyEquipTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mEnemyEquipRotOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mStandEquipTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mStandEquipRotOffset
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWeaponSubType
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "StalEnemy" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mHeadActorName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mLeftArmActorName
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Swarm" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32, // mSwarmSubActorNum
        size_of::<nx::agl::Parameter<S32>>() as u32, // mSwarmPattern
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mDeadActorName
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "System" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSameGroupActorName
        size_of::<nx::agl::Parameter<Bool32>>() as u32, // mIsGetItemSelf
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Traveler" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mAppearGameDataName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mDeleteGameDataName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mRouteType
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mRideHorseName
        size_of::<nx::agl::Parameter<Bool32>>() as u32, // mIsLeadHorse
        size_of::<nx::agl::Parameter<S32>>() as u32, // mHorseGearLevel
        // 0
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 1
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 2
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 3
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 4
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 5
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 6
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 7
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 8
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 9
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 10
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 11
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 12
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 13
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 14
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 15
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 16
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 17
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 18
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 19
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 20
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 21
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 22
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 23
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 24
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 25
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 26
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 27
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 28
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mEntryPoint
        size_of::<nx::agl::Parameter<F32>>() as u32, // mWaitFrame
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSchedule
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMoveAS
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mWaitAS
        // 29
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mName
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "WeaponCommon" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32, // mPlayerEqScale
        size_of::<nx::agl::Parameter<F32>>() as u32, // mEnemyEqScale
        size_of::<nx::agl::Parameter<S32>>() as u32, // mGuardPower
        size_of::<nx::agl::Parameter<S32>>() as u32, // mRank
        size_of::<nx::agl::Parameter<Bool32>>() as u32, // mIsHammer
        size_of::<nx::agl::Parameter<Bool32>>() as u32, // mIsWeakBreaker
        size_of::<nx::agl::Parameter<Bool32>>() as u32, // mIsBoomerang
        size_of::<nx::agl::Parameter<Bool32>>() as u32, // mIsBlunt
        size_of::<nx::agl::Parameter<Bool32>>() as u32, // mIsLuckyWeapon
        size_of::<nx::agl::Parameter<Bool32>>() as u32, // mIsPikohan
        size_of::<nx::agl::Parameter<Bool32>>() as u32, // mIsThrowingWeapon
        size_of::<nx::agl::Parameter<Bool32>>() as u32, // mIsThrowingBreakWeapon
        size_of::<nx::agl::Parameter<F32>>() as u32, // mThrowRange
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mDreadActor
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mThroughActor
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mNPCWeaponType
        size_of::<nx::agl::Parameter<Bool32>>() as u32, // mIsNotOnTerrorHold
        size_of::<nx::agl::Parameter<Bool32>>() as u32, // mIsAsOffUnEquiped
        size_of::<nx::agl::Parameter<S32>>() as u32, // mChemicalEnergyMax
        size_of::<nx::agl::Parameter<S32>>() as u32, // mChemicalEnergyAmountUsed
        size_of::<nx::agl::Parameter<F32>>() as u32, // mChemicalEnergyRecoverRate
        size_of::<nx::agl::Parameter<S32>>() as u32, // mChemicalEnergyRecoverInterval
        size_of::<nx::agl::Parameter<S32>>() as u32, // mStickDamage
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mShootBeam
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mDropFromPorchRot
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSharpWeaponPer
        size_of::<nx::agl::Parameter<S32>>() as u32, // mSharpWeaponAddAtkMin
        size_of::<nx::agl::Parameter<S32>>() as u32, // mSharpWeaponAddAtkMax
        size_of::<nx::agl::Parameter<S32>>() as u32, // mSharpWeaponAddLifeMin
        size_of::<nx::agl::Parameter<S32>>() as u32, // mSharpWeaponAddLifeMax
        size_of::<nx::agl::Parameter<Bool32>>() as u32, // mSharpWeaponAddCrit
        size_of::<nx::agl::Parameter<S32>>() as u32, // mSharpWeaponAddGuardMin
        size_of::<nx::agl::Parameter<S32>>() as u32, // mSharpWeaponAddGuardMax
        size_of::<nx::agl::Parameter<S32>>() as u32, // mPoweredSharpAddAtkMin
        size_of::<nx::agl::Parameter<S32>>() as u32, // mPoweredSharpAddAtkMax
        size_of::<nx::agl::Parameter<S32>>() as u32, // mPoweredSharpAddLifeMin
        size_of::<nx::agl::Parameter<S32>>() as u32, // mPoweredSharpAddLifeMax
        size_of::<nx::agl::Parameter<S32>>() as u32, // mPoweredSharpWeaponAddGuardMin
        size_of::<nx::agl::Parameter<S32>>() as u32, // mPoweredSharpWeaponAddGuardMax
        size_of::<nx::agl::Parameter<F32>>() as u32, // mPoweredSharpAddThrowMin
        size_of::<nx::agl::Parameter<F32>>() as u32, // mPoweredSharpAddThrowMax
        size_of::<nx::agl::Parameter<Bool32>>() as u32, // mPoweredSharpAddSpreadFire
        size_of::<nx::agl::Parameter<Bool32>>() as u32, // mPoweredSharpAddZoomRapid
        size_of::<nx::agl::Parameter<F32>>() as u32, // mPoweredSharpAddRapidFireMin
        size_of::<nx::agl::Parameter<F32>>() as u32, // mPoweredSharpAddRapidFireMax
        size_of::<nx::agl::Parameter<Bool32>>() as u32, // mPoweredSharpAddSurfMaster
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "WeaponOption" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mPlayerHoldTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mPlayerHoldRotOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mNPCHoldTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mNPCHoldRotOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mNPCEquipTransOffset
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mNPCEquipRotOffset
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "WeaponThrow" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32, // mThrowSpeed
        size_of::<nx::agl::Parameter<F32>>() as u32, // mThrowRotSpeed
        size_of::<nx::agl::Parameter<F32>>() as u32, // mThrowDist
        size_of::<nx::agl::Parameter<Vector3f>>() as u32, // mThrowRigidBodyBaseAxis
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "WizzRobe" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<S32>>() as u32, // mMagicWeatherType
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mMagicFallActorName
        size_of::<nx::agl::Parameter<F32>>() as u32, // mMagicFallIgniteRotSpd
        size_of::<nx::agl::Parameter<F32>>() as u32, // mMagicFallOffsetY
        size_of::<nx::agl::Parameter<F32>>() as u32, // mMagicFallCenterOffsetXZ
        size_of::<nx::agl::Parameter<F32>>() as u32, // mMagicFallRandRadius
        size_of::<nx::agl::Parameter<F32>>() as u32, // mMagicFallIntervalMax
        size_of::<nx::agl::Parameter<F32>>() as u32, // mMagicFallIntervalMin
        size_of::<nx::agl::Parameter<nx::SafeString>>() as u32, // mSummonActorName
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "WolfLink" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNeckSpeedWait
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNeckRateWait
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNeckSpeedShiekSensor
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNeckRateShiekSensor
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNeckSpeedFollow
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNeckRateFollow
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNeckSpeedBattle
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNeckRateBattle
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNeckSpeedHeal
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNeckRateHeal
        size_of::<nx::agl::Parameter<F32>>() as u32, // mBattleRange
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHealRange
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHuntRange
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHowlRange
        size_of::<nx::agl::Parameter<F32>>() as u32, // mMaxHeightAttackable
        size_of::<nx::agl::Parameter<F32>>() as u32, // mMaxHeightHealable
        size_of::<nx::agl::Parameter<F32>>() as u32, // mNavMeshSearchRadius
        size_of::<nx::agl::Parameter<F32>>() as u32, // mCanReachPlayerNavMeshSearchRadius
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSubmergedDepth
        size_of::<nx::agl::Parameter<F32>>() as u32, // mUtilityLifeToHunt
        size_of::<nx::agl::Parameter<F32>>() as u32, // mUtilityDangerDistMin
        size_of::<nx::agl::Parameter<F32>>() as u32, // mUtilityDangerDistMax
        size_of::<nx::agl::Parameter<F32>>() as u32, // mUtilityConstant
        size_of::<nx::agl::Parameter<F32>>() as u32, // mChainAttackChargeMin
        size_of::<nx::agl::Parameter<F32>>() as u32, // mChainAttackChargeMax
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookAtCooldownWait
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookAtCooldownWaitRand
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookAtCounterWait
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookAtCounterWaitRand
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookAtCooldownRun
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookAtCooldownRunRand
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookAtCounterRun
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLookAtCounterRunRand
        size_of::<nx::agl::Parameter<F32>>() as u32, // mAttackCounterLength
        size_of::<nx::agl::Parameter<F32>>() as u32, // mAttackCounterRand
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHowlCooldownCounterLength
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHowlCooldownCounterRand
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHealCooldownCounterLength
        size_of::<nx::agl::Parameter<F32>>() as u32, // mHealCooldownCounterRand
        size_of::<nx::agl::Parameter<F32>>() as u32, // mFailPathCooldownCounterLength
        size_of::<nx::agl::Parameter<F32>>() as u32, // mFailPathCooldownCounterRand
        size_of::<nx::agl::Parameter<F32>>() as u32, // mRetargetCooldownCounterLength
        size_of::<nx::agl::Parameter<F32>>() as u32, // mRetargetCooldownCounterRand
        size_of::<nx::agl::Parameter<F32>>() as u32, // mAfterTargetDeathCounterLength
        size_of::<nx::agl::Parameter<F32>>() as u32, // mAfterTargetDeathCounterRand
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLostTargetCounterLength
        size_of::<nx::agl::Parameter<F32>>() as u32, // mLostTargetCounterRand
        size_of::<nx::agl::Parameter<F32>>() as u32, // mInvinceableCounterLength
        size_of::<nx::agl::Parameter<F32>>() as u32, // mInvinceableCounterRand
        size_of::<nx::agl::Parameter<F32>>() as u32, // mCallDelayMinLength
        size_of::<nx::agl::Parameter<F32>>() as u32, // mCallOverrideCounterLength
        size_of::<nx::agl::Parameter<F32>>() as u32, // mGiveUpShiekSensorLength
        size_of::<nx::agl::Parameter<F32>>() as u32, // mRetryShiekSensorLength
        size_of::<nx::agl::Parameter<F32>>() as u32, // mBattleWallHitLength
        size_of::<nx::agl::Parameter<F32>>() as u32, // mFollowRetryLength
        size_of::<nx::agl::Parameter<F32>>() as u32, // mPowerUpFoodLength
        size_of::<nx::agl::Parameter<F32>>() as u32, // mSafePosFailCounter
        size_of::<nx::agl::Parameter<F32>>() as u32, // mRestrictedTargetTimeNormal
        size_of::<nx::agl::Parameter<F32>>() as u32, // mRestrictedTargetTimeSpecial
        size_of::<nx::agl::Parameter<S32>>() as u32, // mPowerUpFoodAttackMod
        size_of::<nx::agl::Parameter<F32>>() as u32, // mPowerUpFoodChainAttackCharge
        size_of::<nx::agl::Parameter<S32>>() as u32, // mVSStalfosCritChance
        size_of::<nx::agl::Parameter<F32>>() as u32, // mAttackBase
        size_of::<nx::agl::Parameter<F32>>() as u32, // mAttackHeartMod
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDefenseBase
        size_of::<nx::agl::Parameter<F32>>() as u32, // mDefenseHeartMod
        size_of::<nx::agl::ParameterObj>() as u32
    ], &4),
    "Zora" => cpp_align(&[
        size_of::<u64>() as u32,
        size_of::<nx::agl::Parameter<F32>>() as u32, // mInWaterDepth
        size_of::<nx::agl::Parameter<F32>>() as u32, // mFloatDepth
        size_of::<nx::agl::Parameter<F32>>() as u32, // mFloatRadius
        size_of::<nx::agl::Parameter<F32>>() as u32, // mFloatCycleTime
        size_of::<nx::agl::Parameter<F32>>() as u32, // mChangeDepthSpeed
        size_of::<nx::agl::ParameterObj>() as u32
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
