use std::collections::HashMap;

use lazy_static::lazy_static;
use serde_json::Value;

use crate::{Endian, ResourceSizeTable, CRC32};

const SWITCH_RSTB_JSON: &str = include_str!("../data/switch.json");
const WIIU_RSTB_JSON: &str = include_str!("../data/wiiu.json");

impl ResourceSizeTable {
    /// *Requires the `botw-data` feature.*
    /// Creates a new copy of a stock BOTW RSTB. Passing `Endian::Big` will
    /// return the RSTB from the 1.5.0 Wii U version, and passing
    /// `Endian::Little` will return the RSTB from the 1.6.0 Switch version.
    #[cfg(feature = "botw-data")]
    pub fn new_from_stock(endian: Endian) -> Self {
        match endian {
            Endian::Big => WIIU_RSTB.clone(),
            Endian::Little => SWITCH_RSTB.clone(),
        }
    }
}

lazy_static! {
    pub(crate) static ref SWITCH_RSTB: ResourceSizeTable =
        serde_json::from_str(SWITCH_RSTB_JSON).unwrap();
    pub(crate) static ref WIIU_RSTB: ResourceSizeTable =
        serde_json::from_str(WIIU_RSTB_JSON).unwrap();
    pub(crate) static ref FILE_HASHES: HashMap<u32, String> = {
        let nx: Value = serde_json::from_str(SWITCH_RSTB_JSON).unwrap();
        let u: Value = serde_json::from_str(WIIU_RSTB_JSON).unwrap();
        nx["crc_map"]
            .as_object()
            .unwrap()
            .keys()
            .chain(u["crc_map"].as_object().unwrap().keys())
            .map(|s| (CRC32.checksum(s.as_bytes()), s.to_owned()))
            .collect()
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_resources() {
        assert!(!super::SWITCH_RSTB.is_empty());
        assert_eq!(
            super::SWITCH_RSTB.get("Map/MainField/A-1/A-1_Dynamic.mubin"),
            Some(48960)
        );
        assert!(!super::WIIU_RSTB.is_empty());
        assert_eq!(
            super::WIIU_RSTB.get("Map/MainField/A-1/A-1_Dynamic.mubin"),
            Some(48800)
        );
    }
}
