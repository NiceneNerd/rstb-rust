#![allow(clippy::field_reassign_with_default)]
use binread::BinRead;
use binwrite::BinWrite;
use cached::proc_macro::cached;
use cached::UnboundCache;
use crc::{crc32, Hasher32};
use indexmap::IndexMap;
use std::convert::TryFrom;
use std::error::Error;
use std::io::{Cursor, Write};
use std::sync::Mutex;
use yaz0::inflate::Yaz0Archive;

pub mod calc;
mod json;

type AnyError = Box<dyn Error>;

/// Specifies endianness for RSTB operations. Note that, with the RSTB value calculator functions,
/// even files that only come in one endianness (e.g. AAMP files are always little endian) should
/// be specified as Big for Wii U and Little for Switch.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Endian {
    Big,
    Little,
}

impl Into<binread::Endian> for Endian {
    fn into(self) -> binread::Endian {
        match self {
            Self::Big => binread::Endian::Big,
            Self::Little => binread::Endian::Little,
        }
    }
}

impl TryFrom<binread::Endian> for Endian {
    type Error = &'static str;
    fn try_from(endian: binread::Endian) -> Result<Endian, Self::Error> {
        match endian {
            binread::Endian::Big => Ok(Endian::Big),
            binread::Endian::Little => Ok(Endian::Little),
            binread::Endian::Native => Err("RSTB endianness must be explicit"),
        }
    }
}

impl Into<binwrite::Endian> for Endian {
    fn into(self) -> binwrite::Endian {
        match self {
            Self::Big => binwrite::Endian::Big,
            Self::Little => binwrite::Endian::Little,
        }
    }
}

impl TryFrom<binwrite::Endian> for Endian {
    type Error = &'static str;
    fn try_from(endian: binwrite::Endian) -> Result<Endian, Self::Error> {
        match endian {
            binwrite::Endian::Big => Ok(Endian::Big),
            binwrite::Endian::Little => Ok(Endian::Little),
            binwrite::Endian::Native => Err("RSTB endianness must be explicit"),
        }
    }
}

#[derive(BinRead, Debug, PartialEq)]
#[binread(magic = b"RSTB")]
struct Header {
    crc_table_size: u32,
    name_table_size: u32,
}

#[derive(BinRead, Debug, BinWrite)]
struct Crc32Entry {
    crc32: u32,
    size: u32,
}

#[derive(BinRead, Debug)]
struct NameEntry {
    #[br(
        count = 128,
        map = |x: Vec<u8>|
            String::from_utf8(x).unwrap().trim_right_matches(char::from(0)).to_owned()
    )]
    name: String,
    size: u32,
}

#[derive(Debug, BinWrite)]
struct WriteNameEntry {
    #[binwrite(cstr, align_after(128))]
    name: String,
    size: u32,
}

#[derive(BinRead, Debug, PartialEq, Default)]
#[br(import(filesize: usize))]
/// A representation of a Breath of the Wild Resource Size Table (RSTB) file
pub struct ResourceSizeTable {
    #[br(try)]
    header: Option<Header>,
    #[br(
        count = match &header {
            Some(h) => h.crc_table_size as usize,
            None => (filesize / 8)
        },
        map = |x: Vec<Crc32Entry>| x.iter().map(|entry| (entry.crc32, entry.size)).collect()
    )]
    crc_entries: IndexMap<u32, u32>,
    #[br(
        count = match &header {
            Some(h) => h.name_table_size as usize,
            None => 0
        },
        map = |x: Vec<NameEntry>| x.iter().map(|entry| (entry.name.clone(), entry.size)).collect()
    )]
    name_entries: IndexMap<String, u32>,
}

#[derive(Debug, BinWrite)]
struct WriteRstb {
    magic: [u8; 4],
    crc_table_size: u32,
    name_table_size: u32,
    crc_entries: Vec<Crc32Entry>,
    name_entries: Vec<WriteNameEntry>,
}

impl From<&ResourceSizeTable> for WriteRstb {
    fn from(rstb: &ResourceSizeTable) -> WriteRstb {
        WriteRstb {
            magic: *b"RSTB",
            crc_table_size: rstb.crc_entries.len() as u32,
            name_table_size: rstb.name_entries.len() as u32,
            crc_entries: rstb
                .crc_entries
                .iter()
                .map(|(k, v)| Crc32Entry {
                    crc32: *k,
                    size: *v,
                })
                .collect(),
            name_entries: rstb
                .name_entries
                .iter()
                .map(|(k, v)| WriteNameEntry {
                    name: k.to_string(),
                    size: *v,
                })
                .collect(),
        }
    }
}

impl ResourceSizeTable {
    /// Parses an RSTB file from a buffer implementing `Into<Vec<u8>>` using the specified
    /// endianness. If the data is yaz0 compressed, it will be decompressed automatically.
    pub fn from_binary<B: AsRef<[u8]>>(
        data: B,
        endian: Endian,
    ) -> Result<ResourceSizeTable, AnyError> {
        let mut data = data.as_ref();
        let dec: Vec<u8>;
        if &data[..4] == b"Yaz0" {
            let mut reader: Cursor<Vec<u8>> = Cursor::new(data.into());
            let mut yaz = Yaz0Archive::new(&mut reader)?;
            dec = yaz.decompress()?;
            data = dec.as_ref();
        };
        let data_len = data.len();
        let mut reader = Cursor::new(data);
        let mut opts = binread::ReadOptions::default();
        opts.endian = endian.into();
        Ok(ResourceSizeTable::read_options(
            &mut reader,
            &opts,
            (data_len,),
        )?)
    }

    /// Writes the contents of an RSTB to a binary writer implementing `Write`. Does not yaz0
    /// compress, this is left to the user.
    pub fn write_binary<W: Write>(&self, writer: &mut W, endian: Endian) -> Result<(), AnyError> {
        let write_rstb: WriteRstb = WriteRstb::from(self);
        let mut opts = binwrite::WriterOption::default();
        opts.endian = endian.into();
        write_rstb.write_options(writer, &opts)?;
        Ok(())
    }

    /// Writes the binary content of an RSTB as `Vec<u8>`, optionally with yaz0 compression.
    pub fn to_binary(&self, endian: Endian, compress: bool) -> Result<Vec<u8>, AnyError> {
        let mut buf: Vec<u8> = vec![];
        self.write_binary(&mut buf, endian)?;
        if compress {
            let mut yaz_buf: Vec<u8> = vec![];
            let yaz = yaz0::deflate::Yaz0Writer::new(&mut yaz_buf);
            yaz.compress_and_write(&buf, yaz0::CompressionLevel::Naive { quality: 7 })?;
            buf = yaz_buf;
        }
        Ok(buf)
    }

    /// Attempts to retrieve the resource size of a file in the RSTB, returning None if there
    /// is no entry for the file. Checks the CRC table first and then the name table.
    pub fn get_size(&self, file: &str) -> Option<u32> {
        match self.crc_entries.get(&hash(file)).cloned() {
            Some(v) => Some(v),
            None => self.name_entries.get(&file.to_string()).cloned(),
        }
    }

    /// Checks whether a file has an entry in the RSTB. Checks the CRC table first and then the name
    /// table.
    pub fn is_in_table(&self, file: &str) -> bool {
        self.crc_entries.contains_key(&hash(file))
            || self.name_entries.contains_key(&file.to_string())
    }

    /// Sets the resource size of a file in the RSTB, adding an entry if one is not already present.
    /// Uses the CRC table only.
    pub fn set_size<I: Into<u32>>(&mut self, file: &str, size: I) {
        let old = self.crc_entries.insert(hash(file), size.into());
        if old.is_some() {
            if let Some(h) = self.header.as_mut() {
                h.crc_table_size += 1
            }
        }
    }

    /// Deletes an entry from the RSTB. Does nothing if the entry does not exist. Checks the CRC
    /// table first and then the name table.
    pub fn delete_entry(&mut self, file: &str) {
        match self.crc_entries.remove(&hash(file)) {
            Some(..) => {
                if let Some(h) = self.header.as_mut() {
                    h.crc_table_size -= 1
                }
            }
            None => {
                if let Some(..) = self.name_entries.remove(file) {
                    if let Some(h) = self.header.as_mut() {
                        h.crc_table_size -= 1
                    }
                };
            }
        };
    }
}

lazy_static::lazy_static! {
    static ref DIGEST: Mutex<crc32::Digest> = Mutex::new(crc32::Digest::new(crc32::IEEE));
}

#[cached(
    type = "UnboundCache<String, u32>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{}", name) }"#
)]
pub(crate) fn hash(name: &str) -> u32 {
    let mut digest = DIGEST.lock().unwrap();
    digest.write(name.as_bytes());
    let val = digest.sum32();
    digest.reset();
    val
}

#[cfg(test)]
mod tests {
    use crate::{calc, Endian, ResourceSizeTable};
    use std::fs::read;
    #[test]
    fn parse_rstb() {
        let buffer: Vec<u8> = read("test/ResourceSizeTable.product.srsizetable").unwrap();
        let rstb = ResourceSizeTable::from_binary(buffer, crate::Endian::Big).unwrap();
        assert_eq!(
            rstb.is_in_table("Map/MainField/A-1/A-1_Dynamic.mubin"),
            true
        );
        assert_eq!(
            rstb.get_size("Map/MainField/A-1/A-1_Dynamic.mubin")
                .unwrap(),
            48800
        )
    }

    #[test]
    fn edit_rstb() {
        let buffer: Vec<u8> = read("test/ResourceSizeTable.product.srsizetable").unwrap();
        let mut rstb = ResourceSizeTable::from_binary(buffer, crate::Endian::Big).unwrap();
        rstb.set_size("Map/MainField/A-1/A-1_Dynamic.mubin", 666u32);
        assert_eq!(
            rstb.get_size("Map/MainField/A-1/A-1_Dynamic.mubin")
                .unwrap(),
            666
        );
        rstb.delete_entry("Map/MainField/A-1/A-1_Dynamic.mubin");
        assert_eq!(
            rstb.is_in_table("Map/MainField/A-1/A-1_Dynamic.mubin"),
            false
        )
    }

    #[test]
    fn binary_roundtrip() {
        let buffer: Vec<u8> = read("test/ResourceSizeTable.product.srsizetable").unwrap();
        let rstb = ResourceSizeTable::from_binary(buffer, crate::Endian::Big).unwrap();
        let binary = rstb.to_binary(crate::Endian::Big, false).unwrap();
        let new_rstb = ResourceSizeTable::from_binary(binary, crate::Endian::Big).unwrap();
        assert_eq!(
            new_rstb.is_in_table("Map/MainField/A-1/A-1_Dynamic.mubin"),
            true
        );
        assert_eq!(
            new_rstb
                .get_size("Map/MainField/A-1/A-1_Dynamic.mubin")
                .unwrap(),
            48800
        )
    }

    #[test]
    fn text_roundtrip() {
        let buffer: Vec<u8> = read("test/ResourceSizeTable.product.srsizetable").unwrap();
        let rstb = ResourceSizeTable::from_binary(buffer, crate::Endian::Big).unwrap();
        let text = rstb.to_text().unwrap();
        let new_rstb: ResourceSizeTable = ResourceSizeTable::from_text(&text).unwrap();
        assert_eq!(
            new_rstb.is_in_table("Map/MainField/A-1/A-1_Dynamic.mubin"),
            true
        );
        assert_eq!(
            new_rstb
                .get_size("Map/MainField/A-1/A-1_Dynamic.mubin")
                .unwrap(),
            48800
        );
        assert_eq!(rstb, new_rstb);
    }

    #[test]
    fn test_stock() {
        let table = ResourceSizeTable::new_from_stock(Endian::Little);
        assert_eq!(
            table
                .get_size("Map/MainField/A-1/A-1_Dynamic.mubin")
                .unwrap(),
            48960
        )
    }

    #[test]
    fn calc_sizes() {
        let buffer: Vec<u8> = read("test/ActorInfo.product.sbyml").unwrap();
        assert_eq!(
            calc::calculate_size_with_ext(&buffer, ".sbyml", Endian::Big, false).unwrap(),
            1_964_004
        );
        assert_eq!(
            calc::calculate_size("test/AirOcta_Tag.sbactorpack", Endian::Big, false)
                .unwrap()
                .unwrap(),
            6016
        );
        let buffer: Vec<u8> = read("test/Enemy_Bokoblin_Gold.bdmgparam").unwrap();
        assert_eq!(
            calc::calculate_size_with_ext(&buffer, ".bdmgparam", Endian::Big, false).unwrap(),
            5396
        );
        let buffer: Vec<u8> = read("test/Obj_TreeWhiteBirch_A_01.hkrb").unwrap();
        assert_eq!(
            calc::calculate_size_with_ext(&buffer, ".hkrb", Endian::Big, false).unwrap(),
            3520
        );
        let buffer: Vec<u8> = read("test/savedataformat.ssarc").unwrap();
        assert_eq!(
            calc::calculate_size_with_ext(&buffer, ".sarc", Endian::Big, false).unwrap(),
            2_801_216
        );
    }

    #[test]
    fn guess_sizes() {
        let buffer: Vec<u8> = read("test/Armor.baiprog").unwrap();
        assert_eq!(
            calc::guess_size(buffer.len(), Endian::Little, ".baiprog").unwrap() >= 8216,
            true
        );
        assert_eq!(
            calc::guess_bfres_value_from_file(
                "test/FldObj_TreeRootTropical_A_Far.sbfres",
                Endian::Big
            )
            .unwrap()
                >= 137_816,
            true
        )
    }
}
