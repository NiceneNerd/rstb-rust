use roead::aamp::ParameterIO;

use crate::Endian;

const CLASS_SIZE_WIIU: usize = 0x4a8;
const CLASS_SIZE_NX: usize = 0x778;

const BXML_OVERHEAD: usize = 0x64;
const TAG_SIZE: usize = std::mem::size_of::<u32>();

pub fn parse_size(b: &[u8], endian: Endian) -> u32 {
    let mut total_size: usize = match endian {
        Endian::Big => super::PARSE_CONST_WIIU as usize + CLASS_SIZE_WIIU,
        Endian::Little => super::PARSE_CONST_NX as usize + CLASS_SIZE_NX,
    };
    total_size += BXML_OVERHEAD;
    let a: ParameterIO = ParameterIO::from_binary(b).unwrap();

    if let Some(tags) = a.param_root.objects.get("Tags") {
        total_size += TAG_SIZE * tags.len();
    }

    total_size as u32
}
