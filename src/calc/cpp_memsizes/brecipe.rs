use roead::aamp::ParameterIO;
use crate::Endian;

use std::mem::size_of;

use super::cpp_align;
use super::cpp_classes::{
    wiiu,
    nx,
    S32,
};

const BRECIPE_OVERHEAD: u32 = 0x40;
const TABLE_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::ParameterObj>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<S32>>() as u32,
    size_of::<wiiu::SeadBuffer>() as u32,
], &4);
const ITEM_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<S32>>() as u32,
], &4);

const TABLE_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::ParameterObj>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<S32>>() as u32,
    size_of::<nx::SeadBuffer>() as u32,
], &4);
const ITEM_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<S32>>() as u32,
], &4);

pub fn parse_size(bytes: &[u8], endian: Endian) -> u32 {
    match endian {
        Endian::Big => parse_size_wiiu(bytes),
        Endian::Little => parse_size_nx(bytes),
    }
}

fn parse_size_wiiu(b: &[u8]) -> u32 {
    let mut total_size = BRECIPE_OVERHEAD;
    let a = ParameterIO::from_binary(b).unwrap();

    if let Some(header) = a.param_root.objects.get("Header") {
        if let Some(num_tables_param) = header.get("TableNum") {
            let num_tables = num_tables_param.as_int().unwrap() as u32;
            total_size += num_tables * TABLE_SIZE_WIIU;
            for i in 0..num_tables {
                let table_id = format!("Table{:02}", i+1);
                let table_name = header.get(table_id).unwrap().as_string64().unwrap();
                if let Some(table) = a.param_root.objects.get(table_name.as_str()) {
                    let num_items = table.get("ColumnNum").unwrap().as_int().unwrap() as u32;
                    total_size += num_items * ITEM_SIZE_WIIU;
                }
            }
        }
    }
    total_size
}

fn parse_size_nx(b: &[u8]) -> u32 {
    let mut total_size = BRECIPE_OVERHEAD;
    let a = ParameterIO::from_binary(b).unwrap();

    if let Some(header) = a.param_root.objects.get("Header") {
        if let Some(num_tables_param) = header.get("TableNum") {
            let num_tables = num_tables_param.as_int().unwrap() as u32;
            total_size += num_tables * TABLE_SIZE_NX;
            for i in 0..num_tables {
                let table_id = format!("Table{:02}", i+1);
                let table_name = header.get(table_id).unwrap().as_string_ref().unwrap();
                if let Some(table) = a.param_root.objects.get(table_name) {
                    let num_items = table.get("ColumnNum").unwrap().as_int().unwrap() as u32;
                    total_size += num_items * ITEM_SIZE_NX;
                }
            }
        }
    }
    total_size
}
