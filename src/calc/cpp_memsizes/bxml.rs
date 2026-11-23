use roead::aamp::ParameterIO;

use crate::Endian;

const TAG_SIZE: usize = size_of::<u32>();

const BASE_OVERHEAD_WIIU: usize = 0x34;
const BASE_OVERHEAD_NX: usize = 0x20;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    if bytes.len()  == 0 { return None; }

    let mut total_size: usize = match endian {
        Endian::Big => BASE_OVERHEAD_WIIU,
        Endian::Little => BASE_OVERHEAD_NX,
    };
    let a = ParameterIO::from_binary(bytes).ok()?;
    let iter_const = match endian {
        Endian::Big => super::ITER_CONST_WIIU,
        Endian::Little => super::ITER_CONST_NX,
    };

    if let Some(tags) = a.param_root.objects.get("Tags") {
        let tags_num = tags.len();
        if tags_num > 0 {
            total_size += iter_const;
        }
        total_size += TAG_SIZE * tags.len();
    }

    Some(total_size as u32)
}
