use std::mem::size_of;

use roead::aamp::{ParameterIO, ParameterList, ParameterObject};

use super::cpp_classes::{
    agl::Parameter, AIProgram::*, Bool32, SafeString, Vector3f, F32, S32, U32,
};
use crate::Endian;

const CLASS_SIZE_WIIU: u32 = 0x30c;
const CLASS_SIZE_NX: u32 = 0x448;

const BAIPROG_OVERHEAD: u32 = 0xe6;

pub fn parse_size(bytes: &[u8], endian: Endian) -> u32 {
    let mut total_size = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX,
    };
    total_size += BAIPROG_OVERHEAD;
    let a = ParameterIO::from_binary(bytes).unwrap();

    let ai = a.param_root.lists.get("AI").unwrap();
    let num_ai = ai.lists.len() as u32;
    if num_ai > 0 {
        for i in 0..num_ai {
            parse_aiaction(
                ai.lists.get(format!("AI_{}", i)).unwrap(),
                &mut total_size,
                endian,
            );
        }
    }
    let action = a.param_root.lists.get("Action").unwrap();
    let num_action = action.lists.len() as u32;
    if num_action > 0 {
        for i in 0..num_action {
            parse_aiaction(
                action.lists.get(format!("Action_{}", i)).unwrap(),
                &mut total_size,
                endian,
            );
        }
    }
    if let Some(behavior) = a.param_root.lists.get("Behavior") {
        let num_behavior = behavior.lists.len() as u32;
        if num_behavior > 0 {
            for i in 0..num_behavior {
                parse_behavior(
                    behavior.lists.get(format!("Behavior_{}", i)).unwrap(),
                    &mut total_size,
                    endian,
                );
            }
        }
    }
    if let Some(query) = a.param_root.lists.get("Query") {
        let num_query = query.lists.len() as u32;
        if num_query > 0 {
            for i in 0..num_query {
                parse_query(
                    query.lists.get(format!("Query_{}", i)).unwrap(),
                    &mut total_size,
                    endian,
                );
            }
        }
    }
    if let Some(ai_idx_obj) = a.param_root.objects.get("DemoAIActionIdx") {
        parse_aiactionidx(ai_idx_obj, &mut total_size);
    }
    if let Some(behavior_idx_obj) = a.param_root.objects.get("DemoBehaviorIdx") {
        parse_behavioridx(behavior_idx_obj, &mut total_size);
    }

    total_size
}

fn parse_aiactionidx(obj: &ParameterObject, size: &mut u32) {
    let num = obj.len() as u32;
    *size += num * size_of::<u16>() as u32;
}

fn parse_behavioridx(obj: &ParameterObject, size: &mut u32) {
    let num = obj.len() as u32;
    *size += num * size_of::<u8>() as u32;
}

fn parse_aiaction(list: &ParameterList, size: &mut u32, endian: Endian) {
    let aiactiondef_size = match endian {
        Endian::Big => size_of::<AIActionDef<u32>>() as u32,
        Endian::Little => size_of::<AIActionDef<u64>>() as u32,
    };
    *size += aiactiondef_size;

    if let Some(child_idx_obj) = list.objects.get("ChildIdx") {
        *size += (child_idx_obj.len() * size_of::<u16>()) as u32;
    }

    if let Some(behavior_idx_obj) = list.objects.get("BehaviorIdx") {
        *size += (behavior_idx_obj.len() * size_of::<u8>()) as u32;
    }

    if let Some(sinst_obj) = list.objects.get("SInst") {
        parse_defparams(sinst_obj, size, endian);
    }
}

fn parse_behavior(list: &ParameterList, size: &mut u32, endian: Endian) {
    let behaviordef_size = match endian {
        Endian::Big => size_of::<BehaviorDef<u32>>() as u32,
        Endian::Little => size_of::<BehaviorDef<u64>>() as u32,
    };
    *size += behaviordef_size;

    if let Some(sinst_obj) = list.objects.get("SInst") {
        parse_defparams(sinst_obj, size, endian);
    }
}

fn parse_query(list: &ParameterList, size: &mut u32, endian: Endian) {
    let querydef_size = match endian {
        Endian::Big => size_of::<QueryDef<u32>>() as u32,
        Endian::Little => size_of::<QueryDef<u64>>() as u32,
    };
    *size += querydef_size;

    if let Some(sinst_obj) = list.objects.get("SInst") {
        parse_defparams(sinst_obj, size, endian);
    }
}

fn parse_defparams(obj: &ParameterObject, size: &mut u32, endian: Endian) {
    let sinst_num_params = obj.len() as u32;
    if sinst_num_params > 0 {
        let ptr_size: u32 = match endian {
            Endian::Big => size_of::<u32>() as u32,
            Endian::Little => size_of::<u64>() as u32,
        };
        *size += sinst_num_params * ptr_size;
        for (_, p) in obj.iter() {
            if p.as_bool().is_ok() {
                *size += match endian {
                    Endian::Big => size_of::<Parameter<u32, Bool32>>() as u32,
                    Endian::Little => size_of::<Parameter<u64, Bool32>>() as u32,
                };
            } else if p.as_u32().is_ok() {
                *size += match endian {
                    Endian::Big => size_of::<Parameter<u32, U32>>() as u32,
                    Endian::Little => size_of::<Parameter<u64, U32>>() as u32,
                };
            } else if p.as_i32().is_ok() {
                *size += match endian {
                    Endian::Big => size_of::<Parameter<u32, S32>>() as u32,
                    Endian::Little => size_of::<Parameter<u64, S32>>() as u32,
                };
            } else if p.as_f32().is_ok() {
                *size += match endian {
                    Endian::Big => size_of::<Parameter<u32, F32>>() as u32,
                    Endian::Little => size_of::<Parameter<u64, F32>>() as u32,
                };
            } else if p.as_str().is_ok() {
                *size += match endian {
                    Endian::Big => size_of::<Parameter<u32, SafeString<u32>>>() as u32,
                    Endian::Little => size_of::<Parameter<u64, SafeString<u64>>>() as u32,
                };
            } else if p.as_vec3().is_ok() {
                *size += match endian {
                    Endian::Big => size_of::<Parameter<u32, Vector3f>>() as u32,
                    Endian::Little => size_of::<Parameter<u64, Vector3f>>() as u32,
                };
            }
        }
    }
}
