use roead::aamp::ParameterIO;

const BXML_OVERHEAD: usize = 0x64;
const TAG_SIZE: usize = std::mem::size_of::<u32>();

pub fn parse_size(b: &[u8]) -> u32 {
    let mut total_size: usize = BXML_OVERHEAD;
    let a: ParameterIO = ParameterIO::from_binary(b).unwrap();

    if let Some(tags) = a.param_root.objects.get("Tags") {
        total_size += TAG_SIZE * tags.len();
    }

    total_size as u32
}
