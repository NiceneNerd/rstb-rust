use roead::aamp::ParameterIO;

use crate::Endian;
use super::cpp_classes::{agl, sead, LifeCondition::LifeCondition};

const CLASS_SIZE_WIIU: usize = std::mem::size_of::<LifeCondition<u32>>();
const CLASS_SIZE_NX: usize = std::mem::size_of::<LifeCondition<u64>>();

// TODO: Why does this still have unaccounted-for overhead?
const OVERHEAD_WIIU: usize = 0x40;
const OVERHEAD_NX: usize = 0x28;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size: usize = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX + OVERHEAD_NX,
    };
    let (
        iter_size,
        size_t,
        safestring_size,
    );
    match endian {
        Endian::Big => {
            iter_size = super::ITER_CONST_WIIU;
            size_t = 0; // size_of::<u32>(); // Why does the WiiU version not care about non-trivially-destructible types?
            safestring_size = size_of::<agl::Parameter<u32, sead::SafeString<u32>>>();
        }
        Endian::Little => {
            iter_size = super::ITER_CONST_NX;
            size_t = size_of::<u64>();
            safestring_size = size_of::<agl::Parameter<u64, sead::SafeString<u64>>>();
        }
    }

    let a = ParameterIO::from_binary(bytes).ok()?;

    for key in ["InvalidWeathers", "InvalidTimes", "DeleteWeathers", "DeleteTimes"] {
        if let Some(object) = a.param_root.objects.get(key) {
            total_size += iter_size + size_t + object.len() * safestring_size;
        }
    }

    Some(total_size as u32)
}
