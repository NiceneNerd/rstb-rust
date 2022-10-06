use roead::aamp::ParameterIO;
use crate::Endian;

use std::mem::size_of;

use super::{
    cpp_align,
    WiiUParameterObj,
    WiiUParameter,
    WiiUSeadBuffer,
    NXParameterObj,
    NXParameter,
    NXSeadBuffer,
    F32,
    S32,
    WiiUSafeString,
    NXSafeString
};

const BDROP_OVERHEAD: u32 = 0xAC;
const TABLE_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<WiiUParameterObj>() as u32,
    size_of::<WiiUParameter<WiiUSafeString>>() as u32,
    size_of::<WiiUParameter<S32>>() as u32,
    size_of::<WiiUParameter<S32>>() as u32,
    size_of::<WiiUParameter<S32>>() as u32,
    size_of::<WiiUParameter<S32>>() as u32,
    size_of::<WiiUParameter<S32>>() as u32,
    size_of::<WiiUSeadBuffer>() as u32,
], &4);
const ITEM_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<WiiUParameter<WiiUSafeString>>() as u32,
    size_of::<WiiUParameter<F32>>() as u32,
], &4);

const TABLE_SIZE_NX: u32 = cpp_align(&[
    size_of::<NXParameterObj>() as u32,
    size_of::<NXParameter<NXSafeString>>() as u32,
    size_of::<NXParameter<S32>>() as u32,
    size_of::<NXParameter<S32>>() as u32,
    size_of::<NXParameter<S32>>() as u32,
    size_of::<NXParameter<S32>>() as u32,
    size_of::<NXParameter<S32>>() as u32,
    size_of::<NXSeadBuffer>() as u32,
], &4);
const ITEM_SIZE_NX: u32 = cpp_align(&[
    size_of::<NXParameter<NXSafeString>>() as u32,
    size_of::<NXParameter<F32>>() as u32,
], &4);

pub fn parse_size(bytes: &[u8], endian: Endian) -> u32 {
    match endian {
        Endian::Big => parse_size_wiiu(bytes),
        Endian::Little => parse_size_nx(bytes),
    }
}

fn parse_size_wiiu(b: &[u8]) -> u32 {
    let mut total_size = BDROP_OVERHEAD;
    let a = ParameterIO::from_binary(b).unwrap();

    if let Some(header) = a.param_root.objects.get("Header") {
        if let Some(num_tables_param) = header.get("TableNum") {
            let num_tables = num_tables_param.as_int().unwrap() as u32;
            total_size += num_tables * TABLE_SIZE_WIIU;
            for i in 0..num_tables {
                let table_id = format!("Table{:02}", i+1);
                let table_name = header.get(table_id).unwrap().as_string_ref().unwrap();
                if let Some(table) = a.param_root.objects.get(table_name) {
                    let num_items = table.get("ColumnNum").unwrap().as_int().unwrap() as u32;
                    total_size += num_items * ITEM_SIZE_WIIU;
                }
            }
        }
    }
    total_size
}

fn parse_size_nx(b: &[u8]) -> u32 {
    let mut total_size = BDROP_OVERHEAD;
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
