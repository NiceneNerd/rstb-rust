use std::{
    borrow::Borrow,
    fmt::{Debug, Display},
};

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

/// A silly string class representing the key of an entry in an RSTB name map.
/// Used for parsing convenience. Can be easily converted to `&str` or `String`.
/// It is not recommended to ever use this type manually. There's no point, and
/// it may panic in unexpected conditions.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "json",
    derive(Serialize, Deserialize),
    serde(from = "&str", into = "String")
)]
pub struct FixedString {
    raw: [u8; 128],
    len: usize,
}

impl FixedString {
    pub fn new(string: &str) -> Self {
        let len = string.len();
        if len > 128 {
            panic!("RSTB strings are max 128 chars");
        }
        let mut raw: [u8; 128] = [0; 128];
        let (bufw, _) = raw.split_at_mut(len);
        bufw.copy_from_slice(string.as_bytes());
        Self { raw, len }
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        let len = slice
            .iter()
            .position(|c| *c == 0)
            .expect("String not terminated");
        if len == 0 {
            panic!("Empty string");
        }
        let mut raw: [u8; 128] = [0; 128];
        let (bufw, _) = raw.split_at_mut(len);
        bufw.copy_from_slice(&slice[0..len]);
        Self { raw, len }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_str(&self) -> &str {
        let (string, _) = self.raw.split_at(self.len);
        std::str::from_utf8(string).expect("Bad string")
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.raw.split_at(self.len).0
    }

    pub fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.raw)?;
        Ok(())
    }

    pub fn crc(&self) -> u32 {
        crate::CRC32.checksum(self.as_bytes())
    }
}

impl Borrow<str> for FixedString {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl From<&str> for FixedString {
    fn from(string: &str) -> Self {
        Self::new(string)
    }
}

impl From<FixedString> for String {
    fn from(string: FixedString) -> Self {
        string.as_str().to_owned()
    }
}

#[allow(clippy::derivable_impls)]
impl Default for FixedString {
    fn default() -> Self {
        Self {
            raw: [0; 128],
            len: 0,
        }
    }
}

impl Display for FixedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Debug for FixedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FixedString {{\n    raw: b\"{}\",\n    len: {},\n}}",
            self.as_str(),
            self.len
        )
    }
}
