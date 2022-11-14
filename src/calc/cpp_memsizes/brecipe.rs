use roead::aamp::ParameterIO;
use crate::Endian;

use std::mem::size_of;

use super::cpp_classes::Recipe::*;

const CLASS_SIZE_WIIU: u32 = 0x27c;
const CLASS_SIZE_NX: u32 = 0x320;

const BRECIPE_OVERHEAD: u32 = 0x58;

pub fn parse_size(bytes: &[u8], endian: Endian) -> u32 {
    let mut total_size = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX,
    };
    total_size += BRECIPE_OVERHEAD;
    let a = ParameterIO::from_binary(bytes).unwrap();
    let (table_size, item_size): (u32, u32);
    match endian {
        Endian::Big => {
            table_size = size_of::<Table<u32>>() as u32;
            item_size = size_of::<Item<u32>>() as u32;
        },
        Endian::Little => {
            table_size = size_of::<Table<u64>>() as u32;
            item_size = size_of::<Item<u64>>() as u32;
        },
    }

    if let Some(header) = a.param_root.objects.get("Header") {
        if let Some(num_tables_param) = header.get("TableNum") {
            let num_tables = num_tables_param.as_int().unwrap() as u32;
            total_size += num_tables * table_size;
            for i in 0..num_tables {
                let table_id = format!("Table{:02}", i+1);
                let table_name = header.get(table_id).unwrap().as_string64().unwrap();
                if let Some(table) = a.param_root.objects.get(table_name.as_str()) {
                    let num_items = table.get("ColumnNum").unwrap().as_int().unwrap() as u32;
                    total_size += num_items * item_size;
                }
            }
        }
    }
    total_size
}
