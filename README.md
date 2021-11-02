# BOTW resource size table (RSTB) library in Rust

[![crates.io](https://img.shields.io/crates/v/rstb)](https://crates.io/crates/rstb)
[![api](https://img.shields.io/badge/api-rustdoc-558b2f)](https://docs.rs/rstb/)
[![license](https://img.shields.io/crates/l/rstb)](https://spdx.org/licenses/MIT.html)

A quick and easy library for manipulating the resource size table (RSTB)
from *The Legend of Zelda: Breath of the Wild* in Rust. Can edit an RSTB
directly or convert to and from a JSON representation.
Basic usage to manipulate an RSTB file:

```rust
use rstb::{ResourceSizeTable, Endian};
let buf: Vec<u8> = std::fs::read("test/ResourceSizeTable.product.rsizetable")?;
// Read RSTB from data, can automatically decompress if yaz0 compressed
// (requires `yaz0` feature)
let mut table: ResourceSizeTable = ResourceSizeTable::from_binary(buf)?;
// Set the size for a resource
table.set("Map/MainField/A-1/A-1_Dynamic.mubin", 777);
// Check the size
assert_eq!(
    table.get("Map/MainField/A-1/A-1_Dynamic.mubin").unwrap(),
    777
);
// Dump to JSON, if `json` feature enabled
#[cfg(feature = "json")]
{
    let json_table = table.to_text();
    // From JSON back to RSTB
    let new_table = ResourceSizeTable::from_text(&json_table)?;
}
// Write new binary copy, and we'll yaz0 compress it
#[cfg(feature = "yaz0")]
let out_buf: Vec<u8> = table.to_compressed_binary(Endian::Big);
```
