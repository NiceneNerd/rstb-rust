use std::{collections::HashMap, sync::LazyLock};

use include_flate::flate;
use serde_json::Value;

use crate::{Endian, ResourceSizeTable, CRC32};

flate!(static SWITCH_RSTB_JSON: str from "data/switch.json");
flate!(static WIIU_RSTB_JSON: str from "data/wiiu.json");

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

pub(crate) static SWITCH_RSTB: LazyLock<ResourceSizeTable> = LazyLock::new(|| {
    serde_json::from_str(SWITCH_RSTB_JSON.as_ref()).expect("Ref JSON is good, tho")
});
pub(crate) static WIIU_RSTB: LazyLock<ResourceSizeTable> =
    LazyLock::new(|| serde_json::from_str(WIIU_RSTB_JSON.as_ref()).expect("Ref JSON is good, tho"));
pub(crate) static FILE_HASHES: LazyLock<HashMap<u32, String>> = LazyLock::new(|| {
    let nx: Value = serde_json::from_str(SWITCH_RSTB_JSON.as_ref()).expect("Ref JSON is good, tho");
    let u: Value = serde_json::from_str(WIIU_RSTB_JSON.as_ref()).expect("Ref JSON is good, tho");
    nx["crc_map"]
        .as_object()
        .expect("Impossible")
        .keys()
        .chain(u["crc_map"].as_object().expect("Impossible").keys())
        .map(|s| (CRC32.checksum(s.as_bytes()), s.to_owned()))
        .collect()
});

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
