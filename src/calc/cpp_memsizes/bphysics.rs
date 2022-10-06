// TODO: Most of the WiiU classes have arbitrary values added to them
// that make the calculations work better. Test/have someone test the
// Switch version to see if it needs values added as well.

use roead::aamp::ParameterIO;
use crate::Endian;

use std::mem::size_of;

use super::cpp_align;
use super::cpp_classes::{
    wiiu,
    nx,
    Bool32,
    Int,
    S32,
    F32,
    Vector3f,
};

const PARAMSET_OVERHEAD: u32 = 0x70; // 0x1A8;

pub fn parse_size(bytes: &[u8], endian: Endian) -> u32 {
    match endian {
        Endian::Big => parse_size_wiiu(bytes),
        Endian::Little => parse_size_nx(bytes),
    }
}

const CHARACTERCONTROLLERPARAM_FORM_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::ParameterList>() as u32,
    size_of::<wiiu::agl::ParameterObj>() as u32,
    size_of::<wiiu::agl::Parameter<Int>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::FixedSafeString32>>() as u32,
    size_of::<wiiu::SeadBuffer>() as u32,
], &4) + 0x30;
const CHARACTERCONTROLLERPARAM_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::ParameterList>() as u32,
    size_of::<u32>() as u32, // from ICharacterControllerParam's vftable. it has no members
    size_of::<wiiu::agl::ParameterObj>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<Int>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::FixedSafeString32>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::FixedSafeString32>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::FixedSafeString32>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::FixedSafeString32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::FixedSafeString32>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::FixedSafeString32>>() as u32,
    size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<S32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
    size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::SeadBuffer>() as u32,
], &4) + 0x20;
const CLOTHPARAM_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::ParameterObj>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
    size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
], &4) + 0x10;
const CLOTHSUBWINDPARAM_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::ParameterObj>() as u32,
    size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
], &4) + 0x20;
const CLOTHSETPARAM_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::ParameterList>() as u32,
    size_of::<wiiu::agl::ParameterObj>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<Int>>() as u32,
    size_of::<wiiu::FixedSafeString64>() as u32,
    CLOTHSUBWINDPARAM_SIZE_WIIU,
    size_of::<wiiu::SeadBuffer>() as u32,
], &4);
const CONTACTINFOPARAM_CONTACTPOINTINFOPARAM_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::ParameterObj>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::FixedSafeString32>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::FixedSafeString32>>() as u32,
    size_of::<wiiu::agl::Parameter<Int>>() as u32,
], &4) + 0x10;
const CONTACTINFOPARAM_COLLISIONINFOPARAM_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::ParameterObj>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::FixedSafeString32>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::FixedSafeString32>>() as u32,
], &4) + 0x10;
const CONTACTINFOPARAM_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::ParameterList>() as u32,
    size_of::<wiiu::SeadBuffer>() as u32,
    size_of::<wiiu::SeadBuffer>() as u32,
    size_of::<wiiu::agl::ParameterObj>() as u32,
    size_of::<wiiu::agl::Parameter<Int>>() as u32,
    size_of::<wiiu::agl::Parameter<Int>>() as u32,
], &4) + 0x10;
const EDGERIGIDBODYPARAM_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::ParameterObj>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
], &4);
const EDGERIGIDBODYSETPARAM_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::ParameterList>() as u32,
    size_of::<wiiu::SeadBuffer>() as u32,
], &4);
const RAGDOLLPARAM_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::ParameterObj>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::FixedSafeString32>() as u32,
], &4);
const RIGIDBODYPARAM_INFO_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::ParameterObj>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
    size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
    size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
    size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
    size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
    size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
    size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
    size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
    size_of::<wiiu::agl::Parameter<S32>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
    size_of::<wiiu::agl::Parameter<S32>>() as u32,
    size_of::<u32>() as u32, // enum
    size_of::<u32>() as u32, // enum
    size_of::<u32>() as u32, // u32
], &4);
const RIGIDBODYPARAM_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::ParameterList>() as u32,
    RIGIDBODYPARAM_INFO_SIZE_WIIU,
    size_of::<wiiu::SeadBuffer>() as u32,
], &4);
const RIGIDBODYSETPARAM_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::ParameterList>() as u32,
    size_of::<u32>() as u32, // enum
    size_of::<wiiu::agl::ParameterObj>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::agl::Parameter<S32>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
    size_of::<wiiu::SeadBuffer>() as u32,
], &4);
const SHAPEPARAMOBJ_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::ParameterObj>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::FixedSafeString32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<F32>>() as u32,
    size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
    size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
    size_of::<wiiu::agl::Parameter<Vector3f>>() as u32,
    size_of::<wiiu::agl::Parameter<S32>>() as u32,
    size_of::<wiiu::SeadBuffer>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::FixedSafeString32>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::FixedSafeString32>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::FixedSafeString32>>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::FixedSafeString32>>() as u32,
    size_of::<wiiu::agl::Parameter<Bool32>>() as u32,
], &4) + 0x30;
const SUPPORTBONEPARAM_SIZE_WIIU: u32 = cpp_align(&[
    size_of::<wiiu::agl::ParameterObj>() as u32,
    size_of::<wiiu::agl::Parameter<wiiu::SafeString>>() as u32,
], &4);
const VERTEX_SIZE_WIIU: u32 = size_of::<wiiu::agl::Parameter<Vector3f>>() as u32;

fn parse_size_wiiu(b: &[u8]) -> u32 {
    let mut total_size = PARAMSET_OVERHEAD;
    let a = ParameterIO::from_binary(b).unwrap();

    if let Some(paramset) = a.param_root.lists.get("ParamSet") {
        if let Some(paramsetheader) = paramset.objects.get("ParamSetHeader") {
            let num_rigid_body_sets = paramsetheader.get("use_rigid_body_set_num").unwrap().as_int().unwrap() as u32;
            if num_rigid_body_sets > 0 {
                total_size += num_rigid_body_sets * RIGIDBODYSETPARAM_SIZE_WIIU;
                for i in 0..num_rigid_body_sets {
                    if let Some(rigidbodysetlist) = paramset.lists.get("RigidBodySet") {
                        if let Some(rigidbodyset) = rigidbodysetlist.lists.get(format!("RigidBodySet_{}", i)) {
                            if let Some(rigidbodysetheader) = rigidbodyset.objects.get("RigidBodySetHeader") {
                                let num_rigid_bodies = rigidbodysetheader.get("num").unwrap().as_int().unwrap() as u32;
                                total_size += num_rigid_bodies * RIGIDBODYPARAM_SIZE_WIIU;
                                for j in 0..num_rigid_bodies {
                                    if let Some(rigidbody) = rigidbodyset.lists.get(format!("RigidBody_{}", j)) {
                                        if let Some(rigidbodyparam) = rigidbody.objects.get("RigidBodyParam") {
                                            if let Some(shape_num) = rigidbodyparam.get("shape_num") {
                                                let num_shapes = shape_num.as_int().unwrap() as u32;
                                                total_size += num_shapes * SHAPEPARAMOBJ_SIZE_WIIU;
                                                for k in 0..num_shapes {
                                                    if let Some(shapeparam) = rigidbody.objects.get(format!("ShapeParam_{}", k)) {
                                                        if let Some(vertex_num) = shapeparam.get("vertex_num") {
                                                            let num_vertices = vertex_num.as_int().unwrap() as u32;
                                                            total_size += num_vertices * VERTEX_SIZE_WIIU;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if paramsetheader.get("use_character_controller").unwrap().as_bool().unwrap() {
                total_size += CHARACTERCONTROLLERPARAM_SIZE_WIIU;
                if let Some(charactercontroller) = paramset.lists.get("CharacterController") {
                    if let Some(charactercontrollerparam) = charactercontroller.objects.get("CharacterControllerParam") {
                        let num_forms = charactercontrollerparam.get("form_num").unwrap().as_int().unwrap() as u32;
                        total_size += num_forms * CHARACTERCONTROLLERPARAM_FORM_SIZE_WIIU;
                        for i in 0..num_forms {
                            if let Some(form) = charactercontroller.lists.get(format!("Form_{}", i)) {
                                if let Some(formheader) = form.objects.get("FormHeader") {
                                    let num_shapes = formheader.get("shape_num").unwrap().as_int().unwrap() as u32;
                                    total_size += num_shapes * SHAPEPARAMOBJ_SIZE_WIIU;
                                    for j in 0..num_shapes {
                                        if let Some(shapeparam) = form.objects.get(format!("ShapeParam_{}", j)) {
                                            if let Some(vertex_num) = shapeparam.get("vertex_num") {
                                                let num_vertices = vertex_num.as_int().unwrap() as u32;
                                                total_size += num_vertices * VERTEX_SIZE_WIIU;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if paramsetheader.get("use_contact_info").unwrap().as_bool().unwrap() {
                total_size += CONTACTINFOPARAM_SIZE_WIIU;
                if let Some(rigidcontactinfo) = paramset.lists.get("RigidContactInfo") {
                    if let Some(rigidcontactinfoheader) = rigidcontactinfo.objects.get("RigidContactInfoHeader") {
                        let num_contact_point_info = rigidcontactinfoheader.get("contact_point_info_num").unwrap().as_int().unwrap() as u32;
                        total_size += num_contact_point_info * CONTACTINFOPARAM_CONTACTPOINTINFOPARAM_SIZE_WIIU;
                        let num_collision_info = rigidcontactinfoheader.get("collision_info_num").unwrap().as_int().unwrap() as u32;
                        total_size += num_collision_info * CONTACTINFOPARAM_COLLISIONINFOPARAM_SIZE_WIIU;
                    }
                }
            }
            if paramsetheader.get("use_support_bone").unwrap().as_bool().unwrap() {
                total_size += SUPPORTBONEPARAM_SIZE_WIIU;
            }
            if paramsetheader.get("use_ragdoll").unwrap().as_bool().unwrap() {
                total_size += RAGDOLLPARAM_SIZE_WIIU;
            }
            if paramsetheader.get("use_cloth").unwrap().as_bool().unwrap() {
                total_size += CLOTHSETPARAM_SIZE_WIIU;
                if let Some(clothlist) = paramset.lists.get("Cloth") {
                    if let Some(clothheader) = clothlist.objects.get("ClothHeader") {
                        let num_cloth = clothheader.get("cloth_num").unwrap().as_int().unwrap() as u32;
                        total_size += num_cloth * CLOTHPARAM_SIZE_WIIU;
                    }
                }
            }
            let num_edge_rigid_bodies = paramsetheader.get("use_edge_rigid_body_num").unwrap().as_int().unwrap() as u32;
            if num_edge_rigid_bodies > 0 {
                total_size += EDGERIGIDBODYSETPARAM_SIZE_WIIU;
                total_size += num_edge_rigid_bodies * EDGERIGIDBODYPARAM_SIZE_WIIU;
            }
        }
    }
    total_size
}

const CHARACTERCONTROLLERPARAM_FORM_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::ParameterList>() as u32,
    size_of::<nx::agl::ParameterObj>() as u32,
    size_of::<nx::agl::Parameter<Int>>() as u32,
    size_of::<nx::agl::Parameter<nx::FixedSafeString32>>() as u32,
    size_of::<nx::SeadBuffer>() as u32,
], &4);
const CHARACTERCONTROLLERPARAM_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::ParameterList>() as u32,
    size_of::<u32>() as u32, // from ICharacterControllerParam's vftable. it has no members
    size_of::<nx::agl::ParameterObj>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<Int>>() as u32,
    size_of::<nx::agl::Parameter<nx::FixedSafeString32>>() as u32,
    size_of::<nx::agl::Parameter<nx::FixedSafeString32>>() as u32,
    size_of::<nx::agl::Parameter<nx::FixedSafeString32>>() as u32,
    size_of::<nx::agl::Parameter<nx::FixedSafeString32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<nx::FixedSafeString32>>() as u32,
    size_of::<nx::agl::Parameter<nx::FixedSafeString32>>() as u32,
    size_of::<nx::agl::Parameter<Bool32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<S32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<Bool32>>() as u32,
    size_of::<nx::agl::Parameter<Bool32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::SeadBuffer>() as u32,
], &4);
const CLOTHPARAM_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::ParameterObj>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<Bool32>>() as u32,
    size_of::<nx::agl::Parameter<Bool32>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
], &4);
const CLOTHSUBWINDPARAM_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::ParameterObj>() as u32,
    size_of::<nx::agl::Parameter<Vector3f>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
], &4);
const CLOTHSETPARAM_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::ParameterList>() as u32,
    size_of::<nx::agl::ParameterObj>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<Int>>() as u32,
    size_of::<nx::FixedSafeString64>() as u32,
    CLOTHSUBWINDPARAM_SIZE_NX,
    size_of::<nx::SeadBuffer>() as u32,
], &4);
const CONTACTINFOPARAM_CONTACTPOINTINFOPARAM_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::ParameterObj>() as u32,
    size_of::<nx::agl::Parameter<nx::FixedSafeString32>>() as u32,
    size_of::<nx::agl::Parameter<nx::FixedSafeString32>>() as u32,
    size_of::<nx::agl::Parameter<Int>>() as u32,
], &4);
const CONTACTINFOPARAM_COLLISIONINFOPARAM_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::ParameterObj>() as u32,
    size_of::<nx::agl::Parameter<nx::FixedSafeString32>>() as u32,
    size_of::<nx::agl::Parameter<nx::FixedSafeString32>>() as u32,
], &4);
const CONTACTINFOPARAM_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::ParameterList>() as u32,
    size_of::<nx::SeadBuffer>() as u32,
    size_of::<nx::SeadBuffer>() as u32,
    size_of::<nx::agl::ParameterObj>() as u32,
    size_of::<nx::agl::Parameter<Int>>() as u32,
    size_of::<nx::agl::Parameter<Int>>() as u32,
], &4);
const EDGERIGIDBODYPARAM_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::ParameterObj>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
], &4);
const EDGERIGIDBODYSETPARAM_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::ParameterList>() as u32,
    size_of::<nx::SeadBuffer>() as u32,
], &4);
const RAGDOLLPARAM_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::ParameterObj>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::FixedSafeString32>() as u32,
], &4);
const RIGIDBODYPARAM_INFO_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::ParameterObj>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<Bool32>>() as u32,
    size_of::<nx::agl::Parameter<Bool32>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<Vector3f>>() as u32,
    size_of::<nx::agl::Parameter<Vector3f>>() as u32,
    size_of::<nx::agl::Parameter<Vector3f>>() as u32,
    size_of::<nx::agl::Parameter<Vector3f>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<Bool32>>() as u32,
    size_of::<nx::agl::Parameter<Bool32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<Bool32>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<Bool32>>() as u32,
    size_of::<nx::agl::Parameter<Bool32>>() as u32,
    size_of::<nx::agl::Parameter<Bool32>>() as u32,
    size_of::<nx::agl::Parameter<S32>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<Bool32>>() as u32,
    size_of::<nx::agl::Parameter<S32>>() as u32,
    size_of::<u32>() as u32, // enum
    size_of::<u32>() as u32, // enum
    size_of::<u32>() as u32, // u32
], &4) - 0x38;
// TODO: NX version might have fewer params...? This normalizes the results by a lot,
// when compared to vanilla parse sizes, but also might drop the lower numbers by too much
// The numbers balance out to 2 primitive Parameters and one string/vector
const RIGIDBODYPARAM_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::ParameterList>() as u32,
    RIGIDBODYPARAM_INFO_SIZE_NX,
    size_of::<nx::SeadBuffer>() as u32,
], &4);
const RIGIDBODYSETPARAM_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::ParameterList>() as u32,
    size_of::<u32>() as u32, // enum
    size_of::<nx::agl::ParameterObj>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::agl::Parameter<S32>>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
    size_of::<nx::SeadBuffer>() as u32,
], &4);
const SHAPEPARAMOBJ_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::ParameterObj>() as u32,
    size_of::<nx::agl::Parameter<nx::FixedSafeString32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<F32>>() as u32,
    size_of::<nx::agl::Parameter<Vector3f>>() as u32,
    size_of::<nx::agl::Parameter<Vector3f>>() as u32,
    size_of::<nx::agl::Parameter<Vector3f>>() as u32,
    size_of::<nx::agl::Parameter<S32>>() as u32,
    size_of::<nx::SeadBuffer>() as u32,
    size_of::<nx::agl::Parameter<nx::FixedSafeString32>>() as u32,
    size_of::<nx::agl::Parameter<nx::FixedSafeString32>>() as u32,
    size_of::<nx::agl::Parameter<nx::FixedSafeString32>>() as u32,
    size_of::<nx::agl::Parameter<nx::FixedSafeString32>>() as u32,
    size_of::<nx::agl::Parameter<Bool32>>() as u32,
], &4);
const SUPPORTBONEPARAM_SIZE_NX: u32 = cpp_align(&[
    size_of::<nx::agl::ParameterObj>() as u32,
    size_of::<nx::agl::Parameter<nx::SafeString>>() as u32,
], &4);
const VERTEX_SIZE_NX: u32 = size_of::<nx::agl::Parameter<Vector3f>>() as u32;

pub fn parse_size_nx(b: &[u8]) -> u32 {
    let mut total_size = PARAMSET_OVERHEAD;
    let a = ParameterIO::from_binary(b).unwrap();

    if let Some(paramset) = a.param_root.lists.get("ParamSet") {
        if let Some(paramsetheader) = paramset.objects.get("ParamSetHeader") {
            let num_rigid_body_sets = paramsetheader.get("use_rigid_body_set_num").unwrap().as_int().unwrap() as u32;
            if num_rigid_body_sets > 0 {
                total_size += num_rigid_body_sets * RIGIDBODYSETPARAM_SIZE_NX;
                for i in 0..num_rigid_body_sets {
                    if let Some(rigidbodysetlist) = paramset.lists.get("RigidBodySet") {
                        if let Some(rigidbodyset) = rigidbodysetlist.lists.get(format!("RigidBodySet_{}", i)) {
                            if let Some(rigidbodysetheader) = rigidbodyset.objects.get("RigidBodySetHeader") {
                                let num_rigid_bodies = rigidbodysetheader.get("num").unwrap().as_int().unwrap() as u32;
                                total_size += num_rigid_bodies * RIGIDBODYPARAM_SIZE_NX;
                                for j in 0..num_rigid_bodies {
                                    if let Some(rigidbody) = rigidbodyset.lists.get(format!("RigidBody_{}", j)) {
                                        if let Some(rigidbodyparam) = rigidbody.objects.get("RigidBodyParam") {
                                            if let Some(shape_num) = rigidbodyparam.get("shape_num") {
                                                let num_shapes = shape_num.as_int().unwrap() as u32;
                                                total_size += num_shapes * SHAPEPARAMOBJ_SIZE_NX;
                                                for k in 0..num_shapes {
                                                    if let Some(shapeparam) = rigidbody.objects.get(format!("ShapeParam_{}", k)) {
                                                        if let Some(vertex_num) = shapeparam.get("vertex_num") {
                                                            let num_vertices = vertex_num.as_int().unwrap() as u32;
                                                            total_size += num_vertices * VERTEX_SIZE_NX;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if paramsetheader.get("use_character_controller").unwrap().as_bool().unwrap() {
                total_size += CHARACTERCONTROLLERPARAM_SIZE_NX;
                if let Some(charactercontroller) = paramset.lists.get("CharacterController") {
                    if let Some(charactercontrollerparam) = charactercontroller.objects.get("CharacterControllerParam") {
                        let num_forms = charactercontrollerparam.get("form_num").unwrap().as_int().unwrap() as u32;
                        total_size += num_forms * CHARACTERCONTROLLERPARAM_FORM_SIZE_NX;
                        for i in 0..num_forms {
                            if let Some(form) = charactercontroller.lists.get(format!("Form_{}", i)) {
                                if let Some(formheader) = form.objects.get("FormHeader") {
                                    let num_shapes = formheader.get("shape_num").unwrap().as_int().unwrap() as u32;
                                    total_size += num_shapes * SHAPEPARAMOBJ_SIZE_NX;
                                    for j in 0..num_shapes {
                                        if let Some(shapeparam) = form.objects.get(format!("ShapeParam_{}", j)) {
                                            if let Some(vertex_num) = shapeparam.get("vertex_num") {
                                                let num_vertices = vertex_num.as_int().unwrap() as u32;
                                                total_size += num_vertices * VERTEX_SIZE_NX;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if paramsetheader.get("use_contact_info").unwrap().as_bool().unwrap() {
                total_size += CONTACTINFOPARAM_SIZE_NX;
                if let Some(rigidcontactinfo) = paramset.lists.get("RigidContactInfo") {
                    if let Some(rigidcontactinfoheader) = rigidcontactinfo.objects.get("RigidContactInfoHeader") {
                        let num_contact_point_info = rigidcontactinfoheader.get("contact_point_info_num").unwrap().as_int().unwrap() as u32;
                        total_size += num_contact_point_info * CONTACTINFOPARAM_CONTACTPOINTINFOPARAM_SIZE_NX;
                        let num_collision_info = rigidcontactinfoheader.get("collision_info_num").unwrap().as_int().unwrap() as u32;
                        total_size += num_collision_info * CONTACTINFOPARAM_COLLISIONINFOPARAM_SIZE_NX;
                    }
                }
            }
            if paramsetheader.get("use_support_bone").unwrap().as_bool().unwrap() {
                total_size += SUPPORTBONEPARAM_SIZE_NX;
            }
            if paramsetheader.get("use_ragdoll").unwrap().as_bool().unwrap() {
                total_size += RAGDOLLPARAM_SIZE_NX;
            }
            if paramsetheader.get("use_cloth").unwrap().as_bool().unwrap() {
                total_size += CLOTHSETPARAM_SIZE_NX;
                if let Some(clothlist) = paramset.lists.get("Cloth") {
                    if let Some(clothheader) = clothlist.objects.get("ClothHeader") {
                        let num_cloth = clothheader.get("cloth_num").unwrap().as_int().unwrap() as u32;
                        total_size += num_cloth * CLOTHPARAM_SIZE_NX;
                    }
                }
            }
            let num_edge_rigid_bodies = paramsetheader.get("use_edge_rigid_body_num").unwrap().as_int().unwrap() as u32;
            if num_edge_rigid_bodies > 0 {
                total_size += EDGERIGIDBODYSETPARAM_SIZE_NX;
                total_size += num_edge_rigid_bodies * EDGERIGIDBODYPARAM_SIZE_NX;
            }
        }
    }
    total_size
}
