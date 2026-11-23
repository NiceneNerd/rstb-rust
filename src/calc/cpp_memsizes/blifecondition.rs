use roead::aamp::ParameterIO;

use crate::Endian;
use super::cpp_classes::{agl, sead};

// TODO: Why does this still have unaccounted-for overhead?
const OVERHEAD_WIIU: usize = 0x40;
const OVERHEAD_NX: usize = 0x28;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    if bytes.len()  == 0 { return None; }

    let mut total_size: usize = match endian {
        Endian::Big => OVERHEAD_WIIU,
        Endian::Little => OVERHEAD_NX,
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
