use std::{borrow::Borrow, collections::BTreeMap, io::Write};

use crate::*;

impl ResourceSizeTable {
    /// Reads an RSTB from a byte slice. Will automatically decompress yaz0
    /// data if the `yaz0` feature is enabled.
    pub fn from_binary<B: Borrow<[u8]>>(bytes: B) -> Result<Self> {
        #[cfg(feature = "yaz0")]
        let bytes = if &bytes.borrow()[0..4] == b"Yaz0" {
            let mut reader = std::io::Cursor::new(bytes.borrow());
            std::borrow::Cow::Owned(yaz0::Yaz0Archive::new(&mut reader)?.decompress()?)
        } else {
            bytes.borrow().into()
        };
        #[cfg(not(feature = "yaz0"))]
        let bytes = if &bytes.borrow()[0..4] == b"Yaz0" {
            return Err(RstbError::FeatureError("yaz0".to_owned()));
        } else {
            bytes.borrow()
        };
        let mut endian = Endian::Big;
        let has_magic = &bytes[0..4] == b"RSTB";
        let crc_table_size = if has_magic {
            let size = read_u32(&bytes[8..12], endian)?;
            if size > 0x10000 {
                endian = Endian::Little;
                read_u32(&bytes[4..8], endian)?
            } else {
                read_u32(&bytes[4..8], endian)?
            }
        } else {
            (bytes.len() / 8) as u32
        };
        let crc_start = if has_magic { 12 } else { 0 };
        let crc_map: BTreeMap<u32, u32> = (0..crc_table_size)
            .into_iter()
            .map(|i| -> Result<(u32, u32)> {
                let offset = (crc_start + i * 8) as usize;
                Ok((
                    read_u32(&bytes[offset..offset + 4], endian)?,
                    read_u32(&bytes[offset + 4..offset + 8], endian)?,
                ))
            })
            .collect::<Result<BTreeMap<_, _>>>()?;
        let name_map: BTreeMap<FixedString, u32> = if has_magic {
            let name_map_size = read_u32(&bytes[8..12], endian)?;
            let name_map_start = 12 + crc_table_size * 8;
            (0..name_map_size)
                .into_iter()
                .map(|i| -> Result<(FixedString, u32)> {
                    let offset = (name_map_start + i * 132) as usize;
                    Ok((
                        read_name(&bytes[offset..offset + 128])?,
                        read_u32(&bytes[offset + 128..offset + 132], endian)?,
                    ))
                })
                .collect::<Result<BTreeMap<_, _>>>()?
        } else {
            BTreeMap::new()
        };
        Ok(Self { crc_map, name_map })
    }

    /// Writes the RSTB to a writer implementing `std::io::Write` with the
    /// specified endianness.
    pub fn write<W: Write>(&self, writer: &mut W, endian: Endian) -> std::io::Result<()> {
        write!(writer, "RSTB")?;
        write_u32(writer, self.crc_map.len() as u32, endian)?;
        write_u32(writer, self.name_map.len() as u32, endian)?;
        for (k, v) in &self.crc_map {
            write_u32(writer, k, endian)?;
            write_u32(writer, v, endian)?;
        }
        for (k, v) in &self.name_map {
            k.write(writer)?;
            write_u32(writer, v, endian)?;
        }
        Ok(())
    }

    /// Writes the RSTB to an in-memory buffer using the specified endianness.
    pub fn to_binary(&self, endian: Endian) -> Vec<u8> {
        let mut buf: Vec<u8> =
            Vec::with_capacity(12 + (self.crc_map.len() * 8) + (self.name_map.len() * 132));
        self.write(&mut buf, endian).unwrap();
        buf
    }

    /// *Requires the `yaz0` feature.*
    /// Writes the RSTB to an in-memory buffer using the specified endianness
    /// with yaz0 compression. **Note:** The yaz0 implementation used is
    /// actually really slow, so it's better to compress on your own if it
    /// suits your purpose.
    #[cfg(feature = "yaz0")]
    pub fn to_compressed_binary(&self, endian: Endian) -> Vec<u8> {
        let mut buf: Vec<u8> =
            Vec::with_capacity(12 + (self.crc_map.len() * 8) + (self.name_map.len() * 132));
        {
            let mut writer = std::io::BufWriter::new(&mut buf);
            yaz0::Yaz0Writer::new(&mut writer)
                .compress_and_write(&self.to_binary(endian), yaz0::CompressionLevel::Naive {
                    quality: 7,
                })
                .unwrap();
        }
        buf
    }
}

fn read_u32(slice: &[u8], endian: Endian) -> Result<u32> {
    Ok(match endian {
        Endian::Big => u32::from_be_bytes(slice[0..4].try_into()?),
        Endian::Little => u32::from_le_bytes(slice[0..4].try_into()?),
    })
}

fn write_u32<W: Write, U: Borrow<u32>>(
    writer: &mut W,
    value: U,
    endian: Endian,
) -> std::io::Result<()> {
    match endian {
        Endian::Big => writer.write(&value.borrow().to_be_bytes()),
        Endian::Little => writer.write(&value.borrow().to_le_bytes()),
    }?;
    Ok(())
}

fn read_name(slice: &[u8]) -> Result<FixedString> {
    if slice.len() < 128 {
        Err(RstbError::InsufficientNameData)
    } else {
        Ok(FixedString::from_slice(slice))
    }
}

#[cfg(feature = "yaz0")]
#[cfg(test)]
mod tests {
    use crate::ResourceSizeTable;

    #[test]
    fn parse_rstb() {
        let bytes = std::fs::read("test/ResourceSizeTable.product.rsizetable").unwrap();
        let rstb = ResourceSizeTable::from_binary(bytes).unwrap();
        assert_eq!(rstb.crc_map.len(), 67992);
        assert_eq!(rstb.get("Map/MainField/A-1/A-1_Dynamic.mubin"), Some(48800))
    }

    #[test]
    fn rstb_roundtrip() {
        let bytes = std::fs::read("test/ResourceSizeTable.product.rsizetable").unwrap();
        let rstb = ResourceSizeTable::from_binary(bytes.as_slice()).unwrap();
        let new_bytes = rstb.to_compressed_binary(crate::Endian::Big);
        assert_eq!(
            rstb,
            ResourceSizeTable::from_binary(new_bytes.as_slice()).unwrap()
        );
    }
}
