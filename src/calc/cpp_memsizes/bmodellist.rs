use roead::aamp::ParameterIO;
use crate::Endian;

use std::mem::size_of;

use super::cpp_classes::ModelList::*;

const CLASS_SIZE_WIIU: u32 = 0x508;
const CLASS_SIZE_NX: u32 = 0x7d0;

const BMODELLIST_OVERHEAD: u32 = 0x74;
const NUM_UNIT_MAX: u32 = 8;

pub fn parse_size(bytes: &[u8], endian: Endian) -> u32 {
    let mut total_size = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX,
    };
    total_size += BMODELLIST_OVERHEAD;
    let a = ParameterIO::from_binary(bytes).unwrap();
    let (anmtarget_size, modeldata_size, partial_size, unit_size): (u32, u32, u32, u32);
    match endian {
        Endian::Big => {
            anmtarget_size = size_of::<AnmTarget<u32>>() as u32;
            modeldata_size = size_of::<ModelData<u32>>() as u32;
            partial_size = size_of::<Partial<u32>>() as u32;
            unit_size = size_of::<Unit<u32>>() as u32;
        },
        Endian::Little => {
            anmtarget_size = size_of::<AnmTarget<u64>>() as u32;
            modeldata_size = size_of::<ModelData<u64>>() as u32;
            partial_size = size_of::<Partial<u64>>() as u32;
            unit_size = size_of::<Unit<u64>>() as u32;
        },
    }


    if let Some(modeldata) = a.param_root.lists.get("ModelData") {
        let num_modeldata = modeldata.lists.len() as u32;
        total_size += num_modeldata * modeldata_size;
        for i in 0..num_modeldata {
            let modeldata_name = format!("ModelData_{}", i);
            if let Some(model) = modeldata.lists.get(modeldata_name) {
                if let Some(unit) = model.lists.get("Unit") {
                    let num_unit = unit.objects.len() as u32;
                    if num_unit > NUM_UNIT_MAX {
                        total_size += NUM_UNIT_MAX * unit_size;
                    } else {
                        total_size += num_unit * unit_size;
                    }
                } 
            }
        }
    }
    if let Some(anmtarget) = a.param_root.lists.get("AnmTarget") {
        let mut num_anmtarget = anmtarget.lists.len() as u32;
        if num_anmtarget > NUM_UNIT_MAX {
            num_anmtarget = NUM_UNIT_MAX;
        }
        total_size += num_anmtarget * anmtarget_size;
        for i in 0..num_anmtarget {
            let anmtarget_name = format!("AnmTarget_{}", i);
            if let Some(target) = anmtarget.lists.get(anmtarget_name) {
                if let Some(partial) = target.lists.get("Partial") {
                    total_size += partial.objects.len() as u32 * partial_size;
                }
            }
        }
    }
    total_size
}
