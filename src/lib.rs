//! A quick and easy library for manipulating the resource size table (RSTB)
//! from *The Legend of Zelda: Breath of the Wild* in Rust. Can edit an RSTB
//! directly or convert to and from a JSON representation.
//!
//! Basic usage to manipulate an RSTB file:
//! ```rust
//! # fn main() -> rstb::Result<()> {
//! use rstb::{Endian, ResourceSizeTable};
//! let buf: Vec<u8> = std::fs::read("test/ResourceSizeTable.product.rsizetable")?;
//! // Read RSTB from data, can automatically decompress if yaz0 compressed
//! // (requires `yaz0` feature)
//! let mut table: ResourceSizeTable = ResourceSizeTable::from_binary(buf)?;
//! // Set the size for a resource
//! table.set("Map/MainField/A-1/A-1_Dynamic.mubin", 777);
//! // Check the size
//! assert_eq!(
//!     table.get("Map/MainField/A-1/A-1_Dynamic.mubin").unwrap(),
//!     777
//! );
//! // Dump to JSON, if `json` feature enabled
//! #[cfg(feature = "json")]
//! {
//!     let json_table = table.to_text();
//!     // From JSON back to RSTB
//!     let new_table = ResourceSizeTable::from_text(&json_table)?;
//! }
//! // Write new binary copy, and we'll yaz0 compress it
//! #[cfg(feature = "yaz0")]
//! let out_buf: Vec<u8> = table.to_compressed_binary(Endian::Big);
//! # Ok(())
//! # }
//! ```
//!
//! There are three important optional features: `yaz0`, `json`, and `botw-data`.
//! - **`yaz0`**: Enables parsing yaz0 compressed RSTB files automatically and
//!   the method [`ResourceSizeTable::to_compressed_binary()`]. Note that the
//!   yaz0 crate used is really slow so you're better off handling it some other
//!   way if your use case permits (e.g. by using [roead](https://github.com/NiceneNerd/roead)).
//! - **`json`**: Enables serializing/deserializing an RSTB file as JSON, using the
//!   [`to_text()`](ResourceSizeTable::to_text()) and
//!   [`from_text()`](ResourceSizeTable::from_text()) methods on [`ResourceSizeTable`]. Note that
//!   filenames can only be serialized if their CRC is known, which requires the `botw-data`
//!   feature.
//! - **`botw-data`**: Enables access to actual RSTB data from BOTW. This will enable filename
//!   serialization in the [`to_text()`](ResourceSizeTable::to_text()) method if `json` is enabled,
//!   and more importantly enables the [`new_from_stock()`](ResourceSizeTable::new_from_stock)
//!   method to create a copy of the original BOTW RSTB (1.5.0 Wii U or 1.6.0 Switch).
#![feature(exclusive_range_pattern)]
#![cfg_attr(feature = "botw-data", feature(once_cell))]
#![deny(clippy::unwrap_used)]

mod bin;
#[cfg(feature = "botw-data")]
mod botw;
pub mod calc;
#[cfg(feature = "json")]
mod json;
mod str;

use std::{borrow::Borrow, collections::BTreeMap};

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use crate::str::FixedString;

pub type Result<T> = std::result::Result<T, RstbError>;
const CRC32: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

/// An enum representing RSTB endianness. Note that it also serves as shorthand
/// for Wii U (`Big`) or Switch (`Little`) more broadly. So if you are
/// performing an RSTB value calculation or generating a stock RSTB, always use
/// `Endian::Big` for Wii U and `Endian::Little` for Switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    Big,
    Little,
}

/// An enum representing the possible keys into an RSTB, whether as CRC hashes
/// or resource names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceKey<'a> {
    Name(&'a str),
    Hash(u32),
}

impl<'a> From<&'a str> for ResourceKey<'a> {
    fn from(name: &'a str) -> Self {
        Self::Name(name)
    }
}

impl From<u32> for ResourceKey<'_> {
    fn from(hash: u32) -> Self {
        Self::Hash(hash)
    }
}

impl From<ResourceKey<'_>> for u32 {
    fn from(k: ResourceKey<'_>) -> Self {
        match k {
            ResourceKey::Hash(h) => h,
            ResourceKey::Name(n) => CRC32.checksum(n.as_bytes()),
        }
    }
}

/// Represents an error when serializing or deserializing an RSTB
#[derive(Debug, Error)]
pub enum RstbError {
    #[error("Bad RSTB magic: {0}, expected \"RSTB\"")]
    BadMagic(String),
    #[error("Unexpected end of data")]
    InsufficientData(#[from] core::array::TryFromSliceError),
    #[error("Unexpected end of data when reading name entry")]
    InsufficientNameData,
    #[error("Invalid RSTB JSON: {0}")]
    #[cfg(feature = "json")]
    InvalidJson(#[from] serde_json::Error),
    #[error("Filename '{0}' for calc has no extension")]
    MissingExtension(String),
    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),
    #[cfg(feature = "yaz0")]
    #[error("Error compressing RSTB: {0}")]
    Yaz0Error(#[from] yaz0::Error),
    #[error("Needed feature {0} not enabled")]
    FeatureError(String),
}

/// Represents a *Breath of the Wild* resource size table
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct ResourceSizeTable {
    #[cfg_attr(feature = "json", serde(with = "json"))]
    pub crc_map:  BTreeMap<u32, u32>,
    pub name_map: BTreeMap<FixedString, u32>,
}

impl ResourceSizeTable {
    /// Returns true if the RSTB contains a size value for the specified hash or resource name.
    /// Checks the CRC table first and then the name table.
    pub fn contains<'k, K: Into<ResourceKey<'k>>>(&self, entry: K) -> bool {
        match entry.into() {
            ResourceKey::Hash(h) => self.crc_map.contains_key(&h),
            ResourceKey::Name(n) => {
                self.name_map.keys().any(|k| k.as_str() == n)
                    || self.crc_map.contains_key(&CRC32.checksum(n.as_bytes()))
            }
        }
    }

    /// Returns the RSTB value for the specified hash or resource name if present.
    /// Checks the CRC table first and then the name table.
    pub fn get<'k, K: Into<ResourceKey<'k>>>(&self, entry: K) -> Option<u32> {
        match entry.into() {
            ResourceKey::Hash(h) => self.crc_map.get(&h).copied(),
            ResourceKey::Name(n) => {
                let hash = CRC32.checksum(n.as_bytes());
                self.crc_map
                    .get(&hash)
                    .or_else(|| {
                        self.name_map
                            .iter()
                            .find_map(|(k, v)| if k.as_str() == n { Some(v) } else { None })
                    })
                    .copied()
            }
        }
    }

    /// Sets the RSTB value for the specified hash or resource name in the CRC table.
    pub fn set<'k, K: Into<ResourceKey<'k>>>(&mut self, entry: K, value: u32) {
        self.crc_map.insert(
            match entry.into() {
                ResourceKey::Hash(h) => h,
                ResourceKey::Name(n) => CRC32.checksum(n.as_bytes()),
            },
            value,
        );
    }

    /// Sets the RSTB value for the specified resource name by calculating it
    /// from its data as a byte slice. If the new value cannot be determined,
    /// checks for an existing entry and removes it.
    pub fn set_from_slice<S: AsRef<str>, B: AsRef<[u8]>>(
        &mut self,
        entry: S,
        data: B,
        endian: Endian,
    ) {
        let name = entry.as_ref();
        match calc::calc_from_slice_and_name(data.as_ref(), name, endian) {
            Some(value) => self.crc_map.insert(CRC32.checksum(name.as_bytes()), value),
            None => self.remove(name),
        };
    }

    /// Sets the RSTB value for the specified resource name by calculating it
    /// from its data as a byte slice. If the new value cannot be determined,
    /// checks for an existing entry and removes it. Applies estimated values
    /// for supported AAMP and BFRES files if applicable.
    pub fn set_from_slice_with_estimate<S: AsRef<str>, B: AsRef<[u8]>>(
        &mut self,
        entry: S,
        data: B,
        endian: Endian,
    ) {
        let name = entry.as_ref();
        match calc::estimate_from_slice_and_name(data.as_ref(), name, endian) {
            Some(value) => self.crc_map.insert(CRC32.checksum(name.as_bytes()), value),
            None => self.remove(name),
        };
    }

    /// Sets the RSTB value for the specified hash or resource name in the name table.
    pub fn set_in_name_table<B: Borrow<str>>(&mut self, name: B, value: u32) {
        self.name_map.insert(FixedString::new(name.borrow()), value);
    }

    /// Returns the number of entries in the RSTB, including both the CRC and name tables.
    pub fn len(&self) -> usize {
        self.crc_map.len() + self.name_map.len()
    }

    /// Returns true if there are no entries in the RSTB, in either the CRC or name table.
    pub fn is_empty(&self) -> bool {
        self.crc_map.is_empty() && self.name_map.is_empty()
    }

    /// Gets an interator over all RSTB entries across both the CRC and name tables.
    pub fn iter(&self) -> impl Iterator<Item = (ResourceKey, &u32)> {
        self.crc_map
            .iter()
            .map(|(k, v)| (ResourceKey::Hash(*k), v))
            .chain(
                self.name_map
                    .iter()
                    .map(|(k, v)| (ResourceKey::Name(k.as_str()), v)),
            )
    }

    /// Gets a mutable interator over all RSTB entries across both the CRC and name tables.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (ResourceKey, &mut u32)> {
        self.crc_map
            .iter_mut()
            .map(|(k, v)| (ResourceKey::Hash(*k), v))
            .chain(
                self.name_map
                    .iter_mut()
                    .map(|(k, v)| (ResourceKey::Name(k.as_str()), v)),
            )
    }

    /// Removes an entry from the RSTB, trying the CRC table first and then the name table.
    /// Returns the old value if the entry already exists.
    pub fn remove<'k, K: Into<ResourceKey<'k>>>(&mut self, entry: K) -> Option<u32> {
        match entry.into() {
            ResourceKey::Hash(h) => self.crc_map.remove(&h),
            ResourceKey::Name(n) => {
                let hash = CRC32.checksum(n.as_bytes());
                self.crc_map.remove(&hash).or_else(|| {
                    let fixed_string = FixedString::new(n);
                    self.name_map.remove(&fixed_string)
                })
            }
        }
    }
}
