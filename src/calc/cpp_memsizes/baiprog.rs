use std::mem::size_of;

use roead::aamp::{ParameterIO, ParameterList, ParameterObject};
use roead::byml::Byml;

use super::cpp_classes::{agl::Parameter, AIProgram::*, sead};
use crate::botw::AIDEF_GAME;
use crate::Endian;

const CLASS_SIZE_WIIU: usize = std::mem::size_of::<AIProgram<u32>>();
const CLASS_SIZE_NX: usize = std::mem::size_of::<AIProgram<u64>>();

const OVERHEAD_WIIU: usize = 0x100;
const OVERHEAD_NX: usize = 0x1000;

struct BaiprogMeta {
    endian: Endian,
    //iter_size: usize,
    //size_t: usize,
}

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX + OVERHEAD_NX,
    };

    let meta = match endian {
        Endian::Big => BaiprogMeta {
            endian,
            //iter_size: super::ITER_CONST_WIIU,
            //size_t: 0, // size_of::<u32>(); // Why does the WiiU version not care about non-trivially-destructible types?
        },
        Endian::Little => BaiprogMeta {
            endian,
            //iter_size: super::ITER_CONST_NX,
            //size_t: size_of::<u64>(),
        },
    };

    let a = ParameterIO::from_binary(bytes).ok()?;

    meta.parse_aiactions(&a.param_root, &mut total_size, "AI");
    meta.parse_aiactions(&a.param_root, &mut total_size, "Action");
    meta.parse_behaviors(&a.param_root, &mut total_size);
    meta.parse_queries(&a.param_root, &mut total_size);

    if let Some(ai_idx_obj) = a.param_root.objects.get("DemoAIActionIdx") {
        meta.parse_aiactionidx(ai_idx_obj, &mut total_size);
    }
    if let Some(behavior_idx_obj) = a.param_root.objects.get("DemoBehaviorIdx") {
        meta.parse_behavioridx(behavior_idx_obj, &mut total_size);
    }

    Some(total_size as u32)
}

impl BaiprogMeta {
    #[inline]
    fn parse_aiactionidx(&self, obj: &ParameterObject, size: &mut usize) {
        *size += obj.len() * size_of::<u16>();
    }
    
    #[inline]
    fn parse_behavioridx(&self, obj: &ParameterObject, size: &mut usize) {
        *size += obj.len() * size_of::<u8>();
    }

    fn parse_aiactions(&self, root: &ParameterList, size: &mut usize, type_name: &str) {
        let list = root.lists.get(type_name).unwrap();
        let num = list.lists.len();
        if num > 0 {
            let aiactiondef_size = match self.endian {
                Endian::Big => size_of::<AIActionDef<u32>>(),
                Endian::Little => size_of::<AIActionDef<u64>>(),
            };
            *size += num * aiactiondef_size;

            for i in 0..num {
                let child = list.lists.get(format!("{type_name}_{i}")).unwrap();
                if let Some(child_idx_obj) = child.objects.get("ChildIdx") {
                    self.parse_aiactionidx(child_idx_obj, size);
                }
                
                if let Some(behavior_idx_obj) = child.objects.get("BehaviorIdx") {
                    self.parse_behavioridx(behavior_idx_obj, size);
                }
                
                let class_name = child.objects
                    .get("Def")
                    .unwrap()
                    .get("ClassName")
                    .unwrap()
                    .as_str()
                    .unwrap();
                self.parse_defparams(class_name, type_name, child, size);
            }
        }
    }

    fn parse_behaviors(&self, root: &ParameterList, size: &mut usize) {
        let list = root.lists.get("Behavior").unwrap();
        let num = list.lists.len();
        if num > 0 {
            let behaviordef_size = match self.endian {
                Endian::Big => size_of::<BehaviorDef<u32>>(),
                Endian::Little => size_of::<BehaviorDef<u64>>(),
            };
            *size += num * behaviordef_size;

            for i in 0..num {
                let child = list.lists.get(format!("Behavior_{i}")).unwrap();
                let class_name = child.objects
                    .get("Def")
                    .unwrap()
                    .get("ClassName")
                    .unwrap()
                    .as_str()
                    .unwrap();
                self.parse_defparams(class_name, "Behavior", child, size);
            }
        }
    }

    fn parse_queries(&self, root: &ParameterList, size: &mut usize) {
        let list = root.lists.get("Query").unwrap();
        let num = list.lists.len();
        if num > 0 {
            let querydef_size = match self.endian {
                Endian::Big => size_of::<QueryDef<u32>>(),
                Endian::Little => size_of::<QueryDef<u64>>(),
            };
            *size += num * querydef_size;

            for i in 0..num {
                let child = list.lists.get(format!("Query_{i}")).unwrap();
                let class_name = child.objects
                    .get("Def")
                    .unwrap()
                    .get("ClassName")
                    .unwrap()
                    .as_str()
                    .unwrap();
                self.parse_defparams(class_name, "Query", child, size);
            }
        }
    }

    fn parse_defparams(&self, class_name: &str, type_: &str, list: &ParameterList, size: &mut usize) {
        if let Some(sinst_obj) = list.objects.get("SInst") {
            let sinst_num_params = sinst_obj.len();

            // TODO: AIClassDef::instance()->getDef() requires any allocations? It had BETTER not...
            let ai_class_def = get_ai_class_def(class_name, type_);

            // This is a kind of weird section. It allocates a pointer for
            // each param in the object...
            // def->mSInstParams.tryAllocBuffer(sinst_num_params, heap)
            // ...even though it will only read the number of pointers that belong in the
            // AIDef, at most...
            // aidef.num_params < sinst_num_params ? aidef.num_params : sinst_num_params;
            // ...The strange thing is that the game seems to iterate the
            // *AIDef's* params up to the number of params in the *file* sometimes
            // so if the file is missing e.g. the third param out of five, then only
            // the first 4 params from the file would be parsed. We avoid this by
            // iterating over all the AIDef params and discarding the ones the file
            // doesn't have
            if sinst_num_params > 0 {
                let ptr_size = match self.endian {
                    Endian::Big => size_of::<u32>(),
                    Endian::Little => size_of::<u64>(),
                };
                *size += sinst_num_params * ptr_size; // no size_t because these are just pointers

                if let Some(static_inst_params) = ai_class_def
                    .as_map()
                    .unwrap()
                    .get("StaticInstParams") {
                    for byml in static_inst_params
                        .as_array()
                        .unwrap() {
                        let map = byml.as_map().unwrap();
                        let param_name = map.get("Name")
                            .unwrap_or_else(|| panic!("Name not found in AIDef {class_name} StaticInstParams"))
                            .as_string()
                            .unwrap()
                            .as_str();
                        let param_type = map.get("Type")
                            .unwrap_or_else(|| panic!("Type not found in AIDef {class_name} StaticInstParams"))
                            .as_string()
                            .unwrap()
                            .as_str();

                        // Inconsistency: AIDefs cannot have UInt, as doGetDef doesn't have
                        // behavior for setting a param_type as AIDefParamType::UInt, but
                        // the parser has behavior for them. Include it to match behavior
                        if let Some(_) = sinst_obj.get(param_name) {
                            *size += match (param_type, self.endian) {
                                ("Bool", Endian::Big) => size_of::<Parameter<u32, bool>>(),
                                ("Bool", Endian::Little) => size_of::<Parameter<u64, bool>>(),
                                ("UInt", Endian::Big) => size_of::<Parameter<u32, u32>>(),
                                ("UInt", Endian::Little) => size_of::<Parameter<u64, u32>>(),
                                ("Int", Endian::Big) => size_of::<Parameter<u32, i32>>(),
                                ("Int", Endian::Little) => size_of::<Parameter<u64, i32>>(),
                                ("Float", Endian::Big) => size_of::<Parameter<u32, f32>>(),
                                ("Float", Endian::Little) => size_of::<Parameter<u64, f32>>(),
                                ("String", Endian::Big) =>
                                    size_of::<Parameter<u32, sead::SafeString<u32>>>(),
                                ("String", Endian::Little) =>
                                    size_of::<Parameter<u64, sead::SafeString<u64>>>(),
                                ("Vec3", Endian::Big) =>
                                    size_of::<Parameter<u32, sead::Vector3f>>(),
                                ("Vec3", Endian::Little) =>
                                    size_of::<Parameter<u64, sead::Vector3f>>(),
                                _ => 0,
                            };
                        }
                    }
                }
            }
        }
    }
}

#[inline]
fn get_ai_class_def<'a>(class_name: &'a str, type_: &'a str) -> &'a Byml {
    AIDEF_GAME
        .as_map()
        .unwrap()
        .get::<str>(format!("{type_}s").as_str().as_ref())
        .unwrap_or_else(|| panic!("{type_} somehow missing from AIDef?"))
        .as_map()
        .unwrap()
        .get(class_name)
        .unwrap_or_else(|| panic!("Unknown AI Program ClassName {class_name}"))
}
