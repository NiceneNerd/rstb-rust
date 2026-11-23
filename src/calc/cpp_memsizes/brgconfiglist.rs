use std::mem::size_of;

use roead::aamp::ParameterIO;

use super::cpp_classes::RagdollConfigList::*;
use crate::Endian;

const C_NUM_IMPULSE_PARAMS: usize = 10;

const OVERHEAD_WIIU: usize = 0x8;
const OVERHEAD_NX: usize = 0x28;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    if bytes.len()  == 0 { return None; }

    let mut total_size = match endian {
        Endian::Big => OVERHEAD_WIIU,
        Endian::Little => OVERHEAD_NX,
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
