use roead::aamp::ParameterIO;
use crate::Endian;

use std::mem::size_of;

use super::cpp_align;
use super::cpp_classes::{
    wiiu,
    nx,
    Bool32,
    S32,
};

const BMODELLIST_OVERHEAD: u32 = 0x74;
const NUM_UNIT_MAX: u32 = 8;
const ANMTARGET_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::Parameter<S32>>() as u32,
    size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
    size_of::<wiiu::agl::Parameter<S32>>() as u32,
    size_of::<wiiu::agl::ParameterObj>() as u32,
    size_of::<wiiu::SeadBuffer>() as u32,
    size_of::<wiiu::agl::ParameterList>() as u32,
    size_of::<wiiu::agl::ParameterList>() as u32,
], &4);
const PARTIAL_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<S32>>() as u32,
    size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
    size_of::<wiiu::agl::ParameterObj>() as u32,
], &4);
const MODELDATA_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::ParameterObj>() as u32,
    size_of::<wiiu::SeadBuffer>() as u32,
    size_of::<wiiu::agl::ParameterList>() as u32,
    size_of::<wiiu::agl::ParameterList>() as u32,
], &4);
const UNIT_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::ParameterObj>() as u32,
], &4);

const ANMTARGET_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::Parameter<S32>>() as u32,
    size_of::<nx::agl::Parameter<Bool32>>() as u32,
    size_of::<nx::agl::Parameter<S32>>() as u32,
    size_of::<nx::agl::ParameterObj>() as u32,
    size_of::<nx::SeadBuffer>() as u32,
    size_of::<nx::agl::ParameterList>() as u32,
    size_of::<nx::agl::ParameterList>() as u32,
], &4);
const PARTIAL_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<S32>>() as u32,
    size_of::<nx::agl::Parameter<Bool32>>() as u32,
    size_of::<nx::agl::ParameterObj>() as u32,
], &4);
const MODELDATA_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::ParameterObj>() as u32,
    size_of::<nx::SeadBuffer>() as u32,
    size_of::<nx::agl::ParameterList>() as u32,
    size_of::<nx::agl::ParameterList>() as u32,
], &4);
const UNIT_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::ParameterObj>() as u32,
], &4);

pub fn parse_size(bytes: &[u8], endian: Endian) -> u32 {
    match endian {
        Endian::Big => parse_size_wiiu(bytes),
        Endian::Little => parse_size_nx(bytes),
    }
}

fn parse_size_wiiu(b: &[u8]) -> u32 {
    let mut total_size = BMODELLIST_OVERHEAD;
    let a = ParameterIO::from_binary(b).unwrap();


    if let Some(modeldata) = a.param_root.lists.get("ModelData") {
        let num_modeldata = modeldata.lists.len() as u32;
        total_size += num_modeldata * MODELDATA_SIZE_WIIU;
        for i in 0..num_modeldata {
            let modeldata_name = format!("ModelData_{}", i);
            if let Some(model) = modeldata.lists.get(modeldata_name) {
                if let Some(unit) = model.lists.get("Unit") {
                    let num_unit = unit.objects.len() as u32;
                    if num_unit > NUM_UNIT_MAX {
                        total_size += NUM_UNIT_MAX * UNIT_SIZE_WIIU;
                    } else {
                        total_size += num_unit * UNIT_SIZE_WIIU;
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
        total_size += num_anmtarget * ANMTARGET_SIZE_WIIU;
        for i in 0..num_anmtarget {
            let anmtarget_name = format!("AnmTarget_{}", i);
            if let Some(target) = anmtarget.lists.get(anmtarget_name) {
                if let Some(partial) = target.lists.get("Partial") {
                    total_size += partial.objects.len() as u32 * PARTIAL_SIZE_WIIU;
                }
            }
        }
    }
    total_size
}

pub fn parse_size_nx(b: &[u8]) -> u32 {
    let mut total_size = BMODELLIST_OVERHEAD;
    let a = ParameterIO::from_binary(b).unwrap();

    if let Some(modeldata) = a.param_root.lists.get("ModelData") {
        let num_modeldata = modeldata.lists.len() as u32;
        total_size += num_modeldata * MODELDATA_SIZE_NX;
        for i in 0..num_modeldata {
            let modeldata_name = format!("ModelData_{}", i);
            if let Some(model) = modeldata.lists.get(modeldata_name) {
                if let Some(unit) = model.lists.get("Unit") {
                    let num_unit = unit.objects.len() as u32;
                    if num_unit > NUM_UNIT_MAX {
                        total_size += num_unit * UNIT_SIZE_NX;
                    } else {
                        total_size += NUM_UNIT_MAX * UNIT_SIZE_NX;
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
        total_size += num_anmtarget * ANMTARGET_SIZE_NX;
        for i in 0..num_anmtarget {
            let anmtarget_name = format!("AnmTarget_{}", i);
            if let Some(target) = anmtarget.lists.get(anmtarget_name) {
                if let Some(partial) = target.lists.get("Partial") {
                    total_size += partial.objects.len() as u32 * PARTIAL_SIZE_NX;
                }
            }
        }
    }
    total_size
}
