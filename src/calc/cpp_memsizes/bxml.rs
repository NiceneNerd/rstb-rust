use roead::aamp::ParameterIO;

use crate::Endian;
use super::cpp_classes::ActorLink::ActorLink;

const CLASS_SIZE_WIIU: usize = std::mem::size_of::<ActorLink<u32>>();
const CLASS_SIZE_NX: usize = std::mem::size_of::<ActorLink<u64>>();

const TAG_SIZE: usize = std::mem::size_of::<u32>();

const BASE_OVERHEAD_WIIU: usize = 0x34;
const BASE_OVERHEAD_NX: usize = 0x20;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size: usize = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + BASE_OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX + BASE_OVERHEAD_NX,
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
