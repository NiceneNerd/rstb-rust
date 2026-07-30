use std::mem::size_of;

use roead::aamp::ParameterIO;

use super::cpp_classes::Chemical::*;
use crate::Endian;

const CLASS_SIZE_WIIU: usize = std::mem::size_of::<Chemical<u32>>();
const CLASS_SIZE_NX: usize = std::mem::size_of::<Chemical<u64>>();

// TODO: Figure out why this list iter takes 0x20 instead of 0x10 for WiiU
const OVERHEAD_WIIU: usize = 0x0;
const OVERHEAD_NX: usize = 0x0;
const HEADER_OVERHEAD_WIIU: usize = 0x10;
const HEADER_OVERHEAD_NX: usize = 0x18;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX + OVERHEAD_NX,
    };

    let a = ParameterIO::from_binary(bytes).ok()?;
    let (
        header_overhead,
        iter_overhead,
        size_t,
        shape_size,
        rigid_size,
    );
    match endian {
        Endian::Big => {
            header_overhead = HEADER_OVERHEAD_WIIU;
            iter_overhead = super::ITER_CONST_WIIU;
            size_t = 0; // size_of::<u32>(); // Why does the WiiU version not care about non-trivially-destructible types?
            shape_size = size_of::<Shape<u32>>();
            rigid_size = size_of::<Rigid<u32>>();
        }
        Endian::Little => {
            header_overhead = HEADER_OVERHEAD_NX;
            iter_overhead = super::ITER_CONST_NX;
            size_t = size_of::<u64>();
            shape_size = size_of::<Shape<u64>>();
            rigid_size = size_of::<Rigid<u64>>();
        }
    }

    if let Some(root) = a.param_root.lists.get("chemical_root") {
        if let Some(header) = root.objects.get("chemical_header") {
            total_size += header_overhead;
            if let Some(res_shape_num) = header.get("res_shape_num") {
                let num_sets: usize = usize::try_from(res_shape_num.as_u32().ok()?).ok()?;
                if num_sets > 0 {
                    total_size += iter_overhead + size_t + num_sets * shape_size;
                    total_size += iter_overhead + size_t + num_sets * rigid_size;
                }
            }
        }
    }
    Some(total_size as u32)
}
