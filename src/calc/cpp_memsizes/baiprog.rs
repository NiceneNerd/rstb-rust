use std::mem::size_of;

use roead::aamp::{ParameterIO, ParameterList, ParameterObject};

use super::cpp_classes::{
    agl::Parameter, AIProgram::*, Bool32, sead, F32, S32, U32,
};
use crate::Endian;

const CLASS_SIZE_WIIU: usize = std::mem::size_of::<AIProgram<u32>>();
const CLASS_SIZE_NX: usize = std::mem::size_of::<AIProgram<u64>>();

const OVERHEAD_WIIU: usize = 0xE2;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX,
    };

    let a = ParameterIO::from_binary(bytes).ok()?;
    let ai = a.param_root.lists.get("AI")?;
    let num_ai = ai.lists.len() as u32;
    if num_ai > 0 {
        for i in 0..num_ai {
            parse_aiaction(ai.lists.get(format!("AI_{}", i))?, &mut total_size, endian);
        }
    }
    let action = a.param_root.lists.get("Action")?;
    let num_action = action.lists.len() as u32;
    if num_action > 0 {
        for i in 0..num_action {
            parse_aiaction(
                action.lists.get(format!("Action_{}", i))?,
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
                    behavior.lists.get(format!("Behavior_{}", i))?,
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
                    query.lists.get(format!("Query_{}", i))?,
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

    Some(total_size as u32)
}

fn parse_aiactionidx(obj: &ParameterObject, size: &mut usize) {
    let num = obj.len();
    *size += num * size_of::<u16>();
}

fn parse_behavioridx(obj: &ParameterObject, size: &mut usize) {
    let num = obj.len();
    *size += num * size_of::<u8>();
}

fn parse_aiaction(list: &ParameterList, size: &mut usize, endian: Endian) {
    let aiactiondef_size = match endian {
        Endian::Big => size_of::<AIActionDef<u32>>(),
        Endian::Little => size_of::<AIActionDef<u64>>(),
    };
    *size += aiactiondef_size;

    if let Some(child_idx_obj) = list.objects.get("ChildIdx") {
        *size += child_idx_obj.len() * size_of::<u16>();
    }

    if let Some(behavior_idx_obj) = list.objects.get("BehaviorIdx") {
        *size += behavior_idx_obj.len() * size_of::<u8>();
    }

    if let Some(sinst_obj) = list.objects.get("SInst") {
        parse_defparams(sinst_obj, size, endian);
    }
}

fn parse_behavior(list: &ParameterList, size: &mut usize, endian: Endian) {
    let behaviordef_size = match endian {
        Endian::Big => size_of::<BehaviorDef<u32>>(),
        Endian::Little => size_of::<BehaviorDef<u64>>(),
    };
    *size += behaviordef_size;

    if let Some(sinst_obj) = list.objects.get("SInst") {
        parse_defparams(sinst_obj, size, endian);
    }
}

fn parse_query(list: &ParameterList, size: &mut usize, endian: Endian) {
    let querydef_size = match endian {
        Endian::Big => size_of::<QueryDef<u32>>(),
        Endian::Little => size_of::<QueryDef<u64>>(),
    };
    *size += querydef_size;

    if let Some(sinst_obj) = list.objects.get("SInst") {
        parse_defparams(sinst_obj, size, endian);
    }
}

fn parse_defparams(obj: &ParameterObject, size: &mut usize, endian: Endian) {
    let sinst_num_params = obj.len();
    if sinst_num_params > 0 {
        let ptr_size = match endian {
            Endian::Big => size_of::<u32>(),
            Endian::Little => size_of::<u64>(),
        };
        *size += sinst_num_params * ptr_size;
        for (_, p) in obj.iter() {
            if p.as_bool().is_ok() {
                *size += match endian {
                    Endian::Big => size_of::<Parameter<u32, Bool32>>(),
                    Endian::Little => size_of::<Parameter<u64, Bool32>>(),
                };
            } else if p.as_u32().is_ok() {
                *size += match endian {
                    Endian::Big => size_of::<Parameter<u32, U32>>(),
                    Endian::Little => size_of::<Parameter<u64, U32>>(),
                };
            } else if p.as_i32().is_ok() {
                *size += match endian {
                    Endian::Big => size_of::<Parameter<u32, S32>>(),
                    Endian::Little => size_of::<Parameter<u64, S32>>(),
                };
            } else if p.as_f32().is_ok() {
                *size += match endian {
                    Endian::Big => size_of::<Parameter<u32, F32>>(),
                    Endian::Little => size_of::<Parameter<u64, F32>>(),
                };
            } else if p.as_str().is_ok() {
                *size += match endian {
                    Endian::Big => size_of::<Parameter<u32, sead::SafeString<u32>>>(),
                    Endian::Little => size_of::<Parameter<u64, sead::SafeString<u64>>>(),
                };
            } else if p.as_vec3().is_ok() {
                *size += match endian {
                    Endian::Big => size_of::<Parameter<u32, sead::Vector3f>>(),
                    Endian::Little => size_of::<Parameter<u64, sead::Vector3f>>(),
                };
            }
        }
    }
}
