use std::collections::BTreeMap;

use serde::*;

#[cfg(feature = "botw-data")]
use crate::botw::FILE_HASHES;
use crate::{ResourceSizeTable, CRC32};

impl ResourceSizeTable {
    /// *Requires the `json` feature.*
    /// Generate a JSON string representation of this RSTB.
    pub fn to_text(&self) -> String {
        serde_json::to_string(self).expect("RSTB should serialize without error")
    }

    /// *Requires the `json` feature.*
    /// Generate a JSON string representation of this RSTB, pretty-printed.
    pub fn to_text_pretty(&self) -> String {
        serde_json::to_string_pretty(self).expect("RSTB should serialize without error")
    }

    /// *Requires the `json` feature.*
    /// Parse a JSON representation of an RSTB file.
    pub fn from_text<S: AsRef<str>>(text: S) -> crate::Result<Self> {
        serde_json::from_str(text.as_ref()).map_err(|e| e.into())
    }
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<BTreeMap<u32, u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let map = BTreeMap::<String, u32>::deserialize(deserializer)?;
    Ok(map
        .into_iter()
        .map(|(k, v)| {
            (
                match k.parse::<u32>() {
                    Ok(h) => h,
                    Err(_) => CRC32.checksum(k.as_bytes()),
                },
                v,
            )
        })
        .collect())
}

#[cfg(feature = "botw-data")]
pub(crate) fn serialize<S>(tree: &BTreeMap<u32, u32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    tree.iter()
        .map(|(k, v)| {
            (
                FILE_HASHES.get(k).cloned().unwrap_or_else(|| k.to_string()),
                *v,
            )
        })
        .collect::<BTreeMap<String, u32>>()
        .serialize(serializer)
}

#[cfg(not(feature = "botw-data"))]
pub(crate) fn serialize<S>(tree: &BTreeMap<u32, u32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    BTreeMap::<u32, u32>::serialize(tree, serializer)
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "botw-data")]
    #[test]
    fn text_roundtrip() {
        let json_rstb = crate::botw::WIIU_RSTB.to_text();
        let copy_rstb = crate::ResourceSizeTable::from_text(json_rstb).unwrap();
        assert_eq!(
            copy_rstb.get("Map/MainField/A-1/A-1_Dynamic.mubin"),
            Some(48800)
        )
    }
}
