use std::mem::size_of;

use roead::aamp::ParameterIO;

use super::cpp_classes::RagdollConfigList::*;
use crate::Endian;

const C_NUM_IMPULSE_PARAMS: usize = 10;

const CLASS_SIZE_WIIU: usize = std::mem::size_of::<RagdollConfigList<u32>>();
const CLASS_SIZE_NX: usize = std::mem::size_of::<RagdollConfigList<u64>>();

const OVERHEAD_WIIU: usize = 0x8;
const OVERHEAD_NX: usize = 0x28;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX + OVERHEAD_NX,
    };

    let (
        iter_size,
        size_t,
        impulseparam_size,
        bodyparam_size,
    );
    match endian {
        Endian::Big => {
            iter_size = super::ITER_CONST_WIIU;
            size_t = 0; // size_of::<u32>(); // Why doesn't blah blah we know
            impulseparam_size = size_of::<ImpulseParam<u32>>();
            bodyparam_size = size_of::<BodyParam<u32>>();
        }
        Endian::Little => {
            iter_size = super::ITER_CONST_NX;
            size_t = size_of::<u64>();
            impulseparam_size = size_of::<ImpulseParam<u64>>();
            bodyparam_size = size_of::<BodyParam<u64>>();
        }
    }

    let a = ParameterIO::from_binary(bytes).ok()?;

    // Yay game! It has this same test. Yay matching behavior!
    if C_NUM_IMPULSE_PARAMS > 0 {
        total_size += iter_size + size_t + impulseparam_size * C_NUM_IMPULSE_PARAMS;
    }

    let bodyparam_list = a.param_root.lists
        .get("BodyParamList")
        .unwrap();
    let num_params = bodyparam_list.objects.len();
    if num_params > 0 {
        total_size += iter_size + size_t + bodyparam_size * num_params;
    }

    Some(total_size as u32)
}
