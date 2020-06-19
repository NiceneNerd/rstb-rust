# BOTW resource size table (RSTB) library in Rust

[![crates.io](https://img.shields.io/crates/v/rstb)](https://crates.io/crates/rstb)
[![api](https://img.shields.io/badge/api-rustdoc-757575)](https://docs.rs/rstb/)
[![license](https://img.shields.io/crates/l/rstb)](https://spdx.org/licenses/MIT.html)

A quick and easy library for manipulating the resource size table (RSTB) from *The Legend of Zelda:
Breath of the Wild* in Rust. Can edit an RSTB directly or convert to and from a JSON representation.

Basic usage to manipulate an RSTB file:
```rust
use std::fs::read;
use rstb::{ResourceSizeTable, Endian};

let buff: Vec<u8> = read("ResourceSizeTable.product.srsizetable").unwrap();
// Read RSTB from data, automatically decompressing if yaz0 compressed
let mut table: ResourceSizeTable = ResourceSizeTable::from_binary(buff, Endian::Big).unwrap();
// Set the size for a resource
table.set_size("Map/MainField/A-1/A-1_Dynamic.mubin", 777u32);
// Check the size
assert_eq!(
    table.get_size("Map/MainField/A-1/A-1_Dynamic.mubin").unwrap(),
    777
);
// Dump to JSON
let json_table = table.to_text().unwrap();
// From JSON back to RSTB
let new_table = ResourceSizeTable::from_text(&json_table).unwrap();
// Write new binary copy, and we'll yaz0 compress it
let out_buf: Vec<u8> = table.to_binary(Endian::Big, true).unwrap();
```

Also provides functions for calculating resource sizes:
```rust
use std::fs::read;
use rstb::calc::*;
// Calculate RSTB value for file on disk
assert_eq!(
    calculate_size(
        "A-1_Dynamic.mubin",
        Endian::Big,
        false
    ).unwrap()
        .unwrap(),
    48800
);

// Or we can calculate from a buffer if we provide the file extension
let buf: Vec<u8> = read("A-1_Dynamic.smubin").unwrap();
assert_eq!(
    calculate_size_with_ext(
        &buf,
        ".smubin",
        Endian::Big,
        false
    ).unwrap(),
    48800
);
```
