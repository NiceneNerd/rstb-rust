use std::mem::size_of;

use roead::aamp::ParameterIO;

use super::cpp_classes::ModelList::*;
use crate::Endian;

const CLASS_SIZE_WIIU: usize = std::mem::size_of::<ModelList<u32>>();
const CLASS_SIZE_NX: usize = std::mem::size_of::<ModelList<u64>>();

const OVERHEAD_WIIU: usize = 0x1C;
const OVERHEAD_NX: usize = 0x28;
const NUM_UNIT_MAX: usize = 8;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX + OVERHEAD_NX,
    };

    let a = ParameterIO::from_binary(bytes).ok()?;
    let (
        iter_const,
        size_t,
        anmtarget_size,
        modeldata_size,
        partial_size,
        unit_size,
    );
    match endian {
        Endian::Big => {
            iter_const = super::ITER_CONST_WIIU;
            size_t = 0; // size_of::<u32>(); // Why does the WiiU version not care about non-trivially-destructible types?
            anmtarget_size = size_of::<AnmTarget<u32>>();
            modeldata_size = size_of::<ModelData<u32>>();
            partial_size = size_of::<Partial<u32>>();
            unit_size = size_of::<Unit<u32>>();
        }
        Endian::Little => {
            iter_const = super::ITER_CONST_NX;
            size_t = size_of::<u64>();
            anmtarget_size = size_of::<AnmTarget<u64>>();
            modeldata_size = size_of::<ModelData<u64>>();
            partial_size = size_of::<Partial<u64>>();
            unit_size = size_of::<Unit<u64>>();
        }
    }

    if let Some(modeldata) = a.param_root.lists.get("ModelData") {
        let num_modeldata = modeldata.lists.len();
        if num_modeldata > 0 {
            total_size += iter_const + size_t + num_modeldata * modeldata_size;
            for i in 0..num_modeldata {
                let modeldata_name = format!("ModelData_{}", i);
                if let Some(model) = modeldata.lists.get(modeldata_name) {
                    if let Some(unit) = model.lists.get("Unit") {
                        let num_unit = if unit.objects.len() < NUM_UNIT_MAX {
                            unit.objects.len()
                        } else {
                            NUM_UNIT_MAX
                        };
                        if num_unit > 0 {
                            total_size += iter_const + size_t + num_unit * unit_size;
                        }
                    }
                }
            }
        }
    }

    if let Some(anmtarget) = a.param_root.lists.get("AnmTarget") {
        let num_anmtarget = if anmtarget.lists.len() < NUM_UNIT_MAX {
            anmtarget.lists.len()
        } else {
            NUM_UNIT_MAX
        };
        if num_anmtarget > 0 {
            total_size += iter_const + size_t + num_anmtarget * anmtarget_size;
            for i in 0..num_anmtarget {
                let anmtarget_name = format!("AnmTarget_{}", i);
                if let Some(target) = anmtarget.lists.get(anmtarget_name) {
                    if let Some(partial) = target.lists.get("Partial") {
                        let num_partial = partial.objects.len();
                        if num_partial > 0 {
                            total_size += iter_const + size_t + num_partial * partial_size;
                        }
                    }
                }
            }
        }
    }
    Some(total_size as u32)
}
