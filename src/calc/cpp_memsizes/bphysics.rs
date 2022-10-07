// TODO: Most of the WiiU classes have arbitrary values added to them
// that make the calculations work better. Test/have someone test the
// Switch version to see if it needs values added as well.

use roead::aamp::ParameterIO;
use crate::Endian;

use std::mem::size_of;

use super::cpp_classes::Vector3f;
use super::cpp_classes::agl::Parameter;
use super::cpp_classes::Physics::*;

const PARAMSET_OVERHEAD: u32 = 0x70; // 0x1A8;

pub fn parse_size(bytes: &[u8], endian: Endian) -> u32 {
    let mut total_size = PARAMSET_OVERHEAD;
    let a = ParameterIO::from_binary(bytes).unwrap();
    let (rigidbodysetparam_size, rigidbodyparam_size, shapeparamobj_size, vertex_size): (u32, u32, u32, u32);
    let (charactercontrollerparam_size, form_size, ragdollparam_size,edgerigidbodyparam_size): (u32, u32, u32, u32);
    let (contactinfoparam_size, contactpointinfoparam_size, collisioninfoparam_size): (u32, u32, u32);
    let (clothsetparam_size, clothparam_size, edgerigidbodysetparam_size, supportboneparam_size): (u32, u32, u32, u32);
    match endian {
        Endian::Big => {
            rigidbodysetparam_size = size_of::<RigidBodySetParam<u32>>() as u32;
            rigidbodyparam_size = size_of::<RigidBodyParam<u32>>() as u32;
            shapeparamobj_size = size_of::<ShapeParamObj<u32>>() as u32 + 0x30;
            vertex_size = size_of::<Parameter<u32, Vector3f>>() as u32;
            charactercontrollerparam_size = size_of::<CharacterControllerParam<u32>>() as u32 + 0x20;
            form_size = size_of::<Form<u32>>() as u32 + 0x30;
            ragdollparam_size = size_of::<RagdollParam<u32>>() as u32;
            contactinfoparam_size = size_of::<ContactInfoParam<u32>>() as u32 + 0x10;
            contactpointinfoparam_size = size_of::<ContactPointInfoParam<u32>>() as u32 + 0x10;
            collisioninfoparam_size = size_of::<CollisionInfoParam<u32>>() as u32 + 0x10;
            clothsetparam_size = size_of::<ClothSetParam<u32>>() as u32 + 0x20;
            clothparam_size = size_of::<ClothParam<u32>>() as u32 + 0x10;
            edgerigidbodysetparam_size = size_of::<EdgeRigidBodySetParam<u32>>() as u32;
            edgerigidbodyparam_size = size_of::<EdgeRigidBodyParam<u32>>() as u32;
            supportboneparam_size = size_of::<SupportBoneParam<u32>>() as u32;
        },
        Endian::Little => {
            rigidbodysetparam_size = size_of::<RigidBodySetParam<u64>>() as u32;
            rigidbodyparam_size = size_of::<RigidBodyParam<u64>>() as u32;
            shapeparamobj_size = size_of::<ShapeParamObj<u64>>() as u32;
            vertex_size = size_of::<Parameter<u64, Vector3f>>() as u32;
            charactercontrollerparam_size = size_of::<CharacterControllerParam<u64>>() as u32;
            form_size = size_of::<Form<u64>>() as u32;
            ragdollparam_size = size_of::<RagdollParam<u64>>() as u32;
            contactinfoparam_size = size_of::<ContactInfoParam<u64>>() as u32;
            contactpointinfoparam_size = size_of::<ContactPointInfoParam<u64>>() as u32;
            collisioninfoparam_size = size_of::<CollisionInfoParam<u64>>() as u32;
            clothsetparam_size = size_of::<ClothSetParam<u64>>() as u32;
            clothparam_size = size_of::<ClothParam<u64>>() as u32;
            edgerigidbodysetparam_size = size_of::<EdgeRigidBodySetParam<u64>>() as u32;
            edgerigidbodyparam_size = size_of::<EdgeRigidBodyParam<u64>>() as u32;
            supportboneparam_size = size_of::<SupportBoneParam<u64>>() as u32;
        },
    }

    if let Some(paramset) = a.param_root.lists.get("ParamSet") {
        if let Some(paramsetheader) = paramset.objects.get("ParamSetHeader") {
            let num_rigid_body_sets = paramsetheader.get("use_rigid_body_set_num").unwrap().as_int().unwrap() as u32;
            if num_rigid_body_sets > 0 {
                total_size += num_rigid_body_sets * rigidbodysetparam_size;
                for i in 0..num_rigid_body_sets {
                    if let Some(rigidbodysetlist) = paramset.lists.get("RigidBodySet") {
                        if let Some(rigidbodyset) = rigidbodysetlist.lists.get(format!("RigidBodySet_{}", i)) {
                            if let Some(rigidbodysetheader) = rigidbodyset.objects.get("RigidBodySetHeader") {
                                let num_rigid_bodies = rigidbodysetheader.get("num").unwrap().as_int().unwrap() as u32;
                                total_size += num_rigid_bodies * rigidbodyparam_size;
                                for j in 0..num_rigid_bodies {
                                    if let Some(rigidbody) = rigidbodyset.lists.get(format!("RigidBody_{}", j)) {
                                        if let Some(rigidbodyparam) = rigidbody.objects.get("RigidBodyParam") {
                                            if let Some(shape_num) = rigidbodyparam.get("shape_num") {
                                                let num_shapes = shape_num.as_int().unwrap() as u32;
                                                total_size += num_shapes * shapeparamobj_size;
                                                for k in 0..num_shapes {
                                                    if let Some(shapeparam) = rigidbody.objects.get(format!("ShapeParam_{}", k)) {
                                                        if let Some(vertex_num) = shapeparam.get("vertex_num") {
                                                            let num_vertices = vertex_num.as_int().unwrap() as u32;
                                                            total_size += num_vertices * vertex_size;
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
                total_size += charactercontrollerparam_size;
                if let Some(charactercontroller) = paramset.lists.get("CharacterController") {
                    if let Some(charactercontrollerparam) = charactercontroller.objects.get("CharacterControllerParam") {
                        let num_forms = charactercontrollerparam.get("form_num").unwrap().as_int().unwrap() as u32;
                        total_size += num_forms * form_size;
                        for i in 0..num_forms {
                            if let Some(form) = charactercontroller.lists.get(format!("Form_{}", i)) {
                                if let Some(formheader) = form.objects.get("FormHeader") {
                                    let num_shapes = formheader.get("shape_num").unwrap().as_int().unwrap() as u32;
                                    total_size += num_shapes * shapeparamobj_size;
                                    for j in 0..num_shapes {
                                        if let Some(shapeparam) = form.objects.get(format!("ShapeParam_{}", j)) {
                                            if let Some(vertex_num) = shapeparam.get("vertex_num") {
                                                let num_vertices = vertex_num.as_int().unwrap() as u32;
                                                total_size += num_vertices * vertex_size;
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
                total_size += contactinfoparam_size;
                if let Some(rigidcontactinfo) = paramset.lists.get("RigidContactInfo") {
                    if let Some(rigidcontactinfoheader) = rigidcontactinfo.objects.get("RigidContactInfoHeader") {
                        let num_contact_point_info = rigidcontactinfoheader.get("contact_point_info_num").unwrap().as_int().unwrap() as u32;
                        total_size += num_contact_point_info * contactpointinfoparam_size;
                        let num_collision_info = rigidcontactinfoheader.get("collision_info_num").unwrap().as_int().unwrap() as u32;
                        total_size += num_collision_info * collisioninfoparam_size;
                    }
                }
            }
            if paramsetheader.get("use_support_bone").unwrap().as_bool().unwrap() {
                total_size += supportboneparam_size;
            }
            if paramsetheader.get("use_ragdoll").unwrap().as_bool().unwrap() {
                total_size += ragdollparam_size;
            }
            if paramsetheader.get("use_cloth").unwrap().as_bool().unwrap() {
                total_size += clothsetparam_size;
                if let Some(clothlist) = paramset.lists.get("Cloth") {
                    if let Some(clothheader) = clothlist.objects.get("ClothHeader") {
                        let num_cloth = clothheader.get("cloth_num").unwrap().as_int().unwrap() as u32;
                        total_size += num_cloth * clothparam_size;
                    }
                }
            }
            let num_edge_rigid_bodies = paramsetheader.get("use_edge_rigid_body_num").unwrap().as_int().unwrap() as u32;
            if num_edge_rigid_bodies > 0 {
                total_size += edgerigidbodysetparam_size;
                total_size += num_edge_rigid_bodies * edgerigidbodyparam_size;
            }
        }
    }
    total_size
}
