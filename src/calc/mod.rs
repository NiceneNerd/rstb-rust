//! Provides functions for calculating or estimating resource size values.
//!
//! Sample usage:
//! ```
//! use std::fs::read;
//! use rstb::calc::*;
//! // Calculate RSTB value for file on disk
//! assert_eq!(
//!     calculate_size(
//!         "A-1_Dynamic.smubin",
//!         Endian::Big,
//!         false
//!     ).unwrap()
//!         .unwrap(),
//!     48800
//! );
//!
//! // Or we can calculate from a buffer if we provide the file extension
//! let buf: Vec<u8> = read("A-1_Dynamic.smubin").unwrap();
//! assert_eq!(
//!     calculate_size_with_ext(
//!         &buf,
//!         ".smubin",
//!         Endian::Big,
//!         false
//!     ).unwrap(),
//!     48800
//! );
//! ```

use crate::{AnyError, Endian};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::read;
use std::io::Cursor;
use std::path::Path;
use yaz0::header::Yaz0Header;

mod guess;
pub use guess::{guess_bfres_value_from_file, guess_bfres_value_from_size, guess_size};

static FACTORY_DATA: &str = include_str!("../../data/factory_table.tsv");

#[derive(Debug, Deserialize)]
struct FactoryParseRow {
    name: String,
    size_nx: usize,
    size_wiiu: usize,
    alignment: usize,
    parse_size_nx: String,
    parse_size_wiiu: String,
    other_extensions: String,
    multiplier: f64,
    constant: usize,
    subsystem: String,
    details: String,
}

#[derive(Debug, Clone)]
struct FactoryInfo {
    size_nx: usize,
    size_wiiu: usize,
    alignment: usize,
    parse_size_nx: usize,
    parse_size_wiiu: usize,
    multiplier: f64,
    constant: usize,
    is_complex: bool,
}

/// Attempts to calculate the resource size for an existant file from its path, optionally applying
/// a statistical estimate for some complex resource types. Note that the `endian` parameter should
/// be specified as `Big` for Wii U and `Little` for Switch even if the file type itself always uses
/// the same endianness (e.g. AAMP files are always little endian).
///
/// Returns a Result of an Option containing the resource value or None if the size cannot be
/// calculated, or an IO/filesystem error.
pub fn calculate_size<P: AsRef<Path>>(
    file: P,
    endian: Endian,
    guess: bool,
) -> Result<Option<u32>, AnyError> {
    let data = read(&file)?;
    Ok(calculate_size_with_ext(
        &data,
        file.as_ref()
            .extension()
            .ok_or(format!("No file extension for {:?}", file.as_ref()))?
            .to_str()
            .ok_or("Extension couldn\'t be converted to string?")?,
        endian,
        guess,
    ))
}

/// Attempts to calculate the resource size for a file given its contents and file extension,
/// optionally applying a statistical estimate for some complex resource types. Note that the
/// `endian` parameter should be specified as `Big` for Wii U and `Little` for Switch even if the
/// file type itself always uses the same endianness (e.g. AAMP files are always little endian).
///
/// Returns `None` if no value can be calculated. **This function always returns `None` for BFRES
/// files**. Use `guess_bfres_size()` to handle BFRES files.
pub fn calculate_size_with_ext(data: &[u8], ext: &str, endian: Endian, guess: bool) -> Option<u32> {
    let actual_ext = match ext {
        ".sarc" | "sarc" => "sarc",
        _ => match ext.strip_prefix(".s") {
            Some(s) => s,
            None => {
                if ext.starts_with('.') || ext.starts_with('s') {
                    &ext[1..]
                } else {
                    &ext
                }
            }
        },
    };
    let info: &FactoryInfo = match FACTORY_INFO_TABLE.get(actual_ext) {
        Some(i) => i,
        None => FACTORY_INFO_TABLE.get("*").unwrap(),
    };
    if info.is_complex && !guess {
        return None;
    }
    let mut size: usize = match &data[0..4] {
        b"Yaz0" => {
            let mut view: Cursor<&[u8]> = Cursor::new(data);
            {
                let header = Yaz0Header::parse(&mut view);
                header.unwrap().expected_size
            }
        }
        _ => data.len(),
    };
    if info.is_complex && guess {
        return guess_size(size, endian, actual_ext);
    }
    size = ((size as isize + 31) & -32) as usize;
    size += match endian {
        Endian::Big => {
            0xe4 + info.size_wiiu
                + info.parse_size_wiiu
                + match actual_ext {
                    "beventpack" => 0xe0,
                    "bfevfl" => 0x58,
                    _ => 0,
                }
        }
        Endian::Little => 0x168 + info.size_nx + info.parse_size_nx,
    };
    Some(size as u32)
}

lazy_static::lazy_static! {
    static ref FACTORY_INFO_TABLE: HashMap<String, FactoryInfo> = {
        let mut map: HashMap<String, FactoryInfo> = HashMap::new();
        let mut tsv_reader = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(FACTORY_DATA.as_bytes());
        for result in tsv_reader.deserialize() {
            let row: FactoryParseRow = result.unwrap();
            let info = FactoryInfo {
                size_nx: row.size_nx,
                size_wiiu: row.size_wiiu,
                alignment: row.alignment,
                parse_size_nx: parse_int::parse::<usize>(&row.parse_size_nx).unwrap_or(0),
                parse_size_wiiu: parse_int::parse::<usize>(&row.parse_size_wiiu).unwrap_or(0),
                multiplier: row.multiplier,
                constant: row.constant,
                is_complex: matches!(row.parse_size_nx.as_str(), "COMPLEX")
            };
            for other_ext in row.other_extensions.split(", ") {
                map.insert(other_ext.to_string(), info.clone());
            }
            map.insert(row.name, info);
        }
        map
    };
}
