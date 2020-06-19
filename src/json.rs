use crate::{AnyError, Endian, Header, ResourceSizeTable, DIGEST};
use crc::Hasher32;
use indexmap::IndexMap;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

static WIIU_DATA: &str = include_str!("../data/wiiu.json");
static SWITCH_DATA: &str = include_str!("../data/switch.json");

lazy_static::lazy_static! {
    static ref STOCK_NAMES: HashMap<u32, String> = {
        let wiiu: Value = serde_json::from_str(WIIU_DATA).unwrap();
        let switch: Value = serde_json::from_str(SWITCH_DATA).unwrap();
        wiiu.as_object()
            .unwrap()
            .get("hash_map")
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .map(|k| k.to_owned())
            .chain(
                switch.as_object()
                    .unwrap()
                    .get("hash_map")
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .keys()
                    .map(|k| k.to_owned())
            )
            .map(|f| (crate::hash(&f), f))
            .collect()
    };
}

fn hash_to_name(crc: &u32) -> String {
    match STOCK_NAMES.get(crc) {
        Some(s) => s.to_owned(),
        None => crc.to_string(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRstb {
    hash_map: IndexMap<String, u32>,
    name_map: IndexMap<String, u32>,
}

impl From<&ResourceSizeTable> for JsonRstb {
    fn from(rstb: &ResourceSizeTable) -> JsonRstb {
        JsonRstb {
            hash_map: rstb
                .crc_entries
                .iter()
                .map(|(k, v)| (hash_to_name(k), *v))
                .collect(),
            name_map: rstb.name_entries.clone(),
        }
    }
}

impl Into<ResourceSizeTable> for JsonRstb {
    fn into(self) -> ResourceSizeTable {
        let mut digest = DIGEST.lock().unwrap();
        ResourceSizeTable {
            header: Some(Header {
                crc_table_size: self.hash_map.len() as u32,
                name_table_size: self.name_map.len() as u32,
            }),
            crc_entries: self
                .hash_map
                .iter()
                .map(|(k, v)| {
                    let hash = match k.parse::<u32>() {
                        Ok(crc) => crc,
                        Err(_) => {
                            digest.write(k.as_bytes());
                            let h = digest.sum32();
                            digest.reset();
                            h
                        }
                    };
                    (hash, *v)
                })
                .collect(),
            name_entries: self.name_map,
        }
    }
}

impl ResourceSizeTable {
    /// Creates a new, blank RSTB. Probably not very useful, to be honest.
    pub fn new() -> ResourceSizeTable {
        ResourceSizeTable {
            header: Some(crate::Header {
                crc_table_size: 0,
                name_table_size: 0,
            }),
            crc_entries: IndexMap::new(),
            name_entries: IndexMap::new(),
        }
    }

    /// Creates a new copy of a stock BOTW RSTB. Passing `Endian::Big` will return the RSTB from
    /// the 1.5.0 Wii U version, and passing `Endian::Little` will return the RSTB from the 1.6.0
    /// Switch version.
    pub fn new_from_stock(endian: Endian) -> ResourceSizeTable {
        let json_rstb: JsonRstb = match endian {
            Endian::Big => serde_json::from_str(WIIU_DATA).unwrap(),
            Endian::Little => serde_json::from_str(SWITCH_DATA).unwrap(),
        };
        json_rstb.into()
    }

    /// Creates a text representation of an RSTB as a JSON string.
    pub fn to_text(&self) -> Result<String, AnyError> {
        let table = JsonRstb::from(self);
        Ok(serde_json::to_string_pretty(&table)?)
    }

    /// Reads an RSTB from a JSON representation.
    pub fn from_text(text: &str) -> Result<ResourceSizeTable, AnyError> {
        let table: JsonRstb = serde_json::from_str(text)?;
        Ok(table.into())
    }
}
