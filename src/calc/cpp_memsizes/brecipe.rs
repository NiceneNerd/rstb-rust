use std::mem::size_of;

use roead::aamp::ParameterIO;

use super::cpp_classes::Recipe::*;
use crate::Endian;

const CLASS_SIZE_WIIU: usize = std::mem::size_of::<Recipe<u32>>();
const CLASS_SIZE_NX: usize = std::mem::size_of::<Recipe<u64>>();

// TODO: Figure out why these overheads cost, when they don't for other types
const BASE_OVERHEAD_WIIU: usize = 0x0;
const BASE_OVERHEAD_NX: usize = 0x0;
const HEADER_OVERHEAD_WIIU: usize = 0x20;
const HEADER_OVERHEAD_NX: usize = 0x18;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + BASE_OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX + BASE_OVERHEAD_NX,
    };

    let a = ParameterIO::from_binary(bytes).ok()?;
    let (
        header_overhead,
        iter_size,
        size_t,
        table_size,
        item_size,
    );
    match endian {
        Endian::Big => {
            header_overhead = HEADER_OVERHEAD_WIIU;
            iter_size = super::ITER_CONST_WIIU;
            size_t = 0; // size_of::<u32>(); // Why does the WiiU version not care about non-trivially-destructible types?
            table_size = size_of::<Table<u32>>();
            item_size = size_of::<Item<u32>>();
        }
        Endian::Little => {
            header_overhead = HEADER_OVERHEAD_NX;
            iter_size = super::ITER_CONST_NX;
            size_t = size_of::<u64>();
            table_size = size_of::<Table<u64>>();
            item_size = size_of::<Item<u64>>();
        }
    }

    if let Some(header) = a.param_root.objects.get("Header") {
        total_size += header_overhead;
        if let Some(num_tables_param) = header.get("TableNum") {
            let num_tables: usize = num_tables_param.as_int().ok()?;
            if num_tables > 0 {
                total_size += iter_size + size_t + num_tables * table_size;
                for i in 0..num_tables {
                    let table_id = format!("Table{:02}", i + 1);
                    let table_name = header.get(table_id)?.as_string64().ok()?;
                    if let Some(table) = a.param_root.objects.get(table_name.as_str()) {
                        let num_items: usize = table.get("ColumnNum")?.as_int().ok()?;
                        if num_items > 0 {
                            total_size += iter_size + size_t + num_items * item_size;
                        }
                    }
                }
            }
        }
    }
    Some(total_size as u32)
}
