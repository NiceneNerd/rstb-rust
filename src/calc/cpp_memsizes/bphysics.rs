// TODO: Most of the WiiU classes have arbitrary values added to them
// that make the calculations work better. Test/have someone test the
// Switch version to see if it needs values added as well.

use std::mem::size_of;

use roead::aamp::{ParameterIO, Parameter::I32};

use super::cpp_classes::{agl::Parameter, Physics::*, sead};
use crate::Endian;

const CLASS_SIZE_WIIU: usize = std::mem::size_of::<Physics<u32>>();
const CLASS_SIZE_NX: usize = std::mem::size_of::<Physics<u64>>();

const OVERHEAD_WIIU: usize = 0x120; // 0x8C;
const OVERHEAD_NX: usize = 0x100; // 0xD0;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX + OVERHEAD_NX,
    };

    let a = ParameterIO::from_binary(bytes).ok()?;
    let (
        iter_size,
        size_t,
        rigidbodysetparam_size,
        rigidbodyparam_size,
        shapeparamobj_size,
        vertex_size,
        charactercontrollerparam_size,
        form_size,
        ragdollparam_size,
        edgerigidbodyparam_size,
        contactinfoparam_size,
        contactpointinfoparam_size,
        collisioninfoparam_size,
        clothsetparam_size,
        clothparam_size,
        edgerigidbodysetparam_size,
        supportboneparam_size,
    );
    match endian {
        Endian::Big => {
            iter_size = super::ITER_CONST_WIIU;
            size_t = 0; // size_of::<u32>(); // Why does the WiiU version not care about non-trivially-destructible types?
            rigidbodysetparam_size = size_of::<RigidBodySetParam<u32>>();
            rigidbodyparam_size = size_of::<RigidBodyParam<u32>>();
            shapeparamobj_size = size_of::<ShapeParamObj<u32>>();
            vertex_size = size_of::<Parameter<u32, sead::Vector3f>>();
            charactercontrollerparam_size =
                size_of::<CharacterControllerParam<u32>>();
            form_size = size_of::<Form<u32>>();
            ragdollparam_size = size_of::<RagdollParam<u32>>();
            contactinfoparam_size = size_of::<ContactInfoParam<u32>>();
            contactpointinfoparam_size = size_of::<ContactPointInfoParam<u32>>();
            collisioninfoparam_size = size_of::<CollisionInfoParam<u32>>();
            clothsetparam_size = size_of::<ClothSetParam<u32>>();
            clothparam_size = size_of::<ClothParam<u32>>();
            edgerigidbodysetparam_size = size_of::<EdgeRigidBodySetParam<u32>>();
            edgerigidbodyparam_size = size_of::<EdgeRigidBodyParam<u32>>();
            supportboneparam_size = size_of::<SupportBoneParam<u32>>();
        }
        Endian::Little => {
            iter_size = super::ITER_CONST_NX;
            size_t = size_of::<u64>();
            rigidbodysetparam_size = size_of::<RigidBodySetParam<u64>>();
            rigidbodyparam_size = size_of::<RigidBodyParam<u64>>();
            shapeparamobj_size = size_of::<ShapeParamObj<u64>>();
            vertex_size = size_of::<Parameter<u64, sead::Vector3f>>();
            charactercontrollerparam_size = size_of::<CharacterControllerParam<u64>>();
            form_size = size_of::<Form<u64>>();
            ragdollparam_size = size_of::<RagdollParam<u64>>();
            contactinfoparam_size = size_of::<ContactInfoParam<u64>>();
            contactpointinfoparam_size = size_of::<ContactPointInfoParam<u64>>();
            collisioninfoparam_size = size_of::<CollisionInfoParam<u64>>();
            clothsetparam_size = size_of::<ClothSetParam<u64>>();
            clothparam_size = size_of::<ClothParam<u64>>();
            edgerigidbodysetparam_size = size_of::<EdgeRigidBodySetParam<u64>>();
            edgerigidbodyparam_size = size_of::<EdgeRigidBodyParam<u64>>();
            supportboneparam_size = size_of::<SupportBoneParam<u64>>();
        }
    }

    let param_set = a.param_root.lists.get("ParamSet").unwrap();
    let param_set_header = param_set.objects.get("ParamSetHeader").unwrap();
    let num_rigid_body_sets: usize = param_set_header
        .get("use_rigid_body_set_num")
        .unwrap()
        .as_int()
        .unwrap();
    if num_rigid_body_sets > 0 {
        total_size += iter_size + size_t +
            num_rigid_body_sets * rigidbodysetparam_size;
        let rigid_body_set_list = param_set.lists.get("RigidBodySet").unwrap();
        for i in 0..num_rigid_body_sets {
            let rigid_body_set = rigid_body_set_list
                .lists
                .get(format!("RigidBodySet_{}", i))
                .unwrap();
            let num_rigid_bodies: usize = rigid_body_set
                .objects
                .get("RigidBodySetHeader")
                .unwrap()
                .get("num")
                .unwrap()
                .as_int()
                .unwrap();
            if num_rigid_bodies > 0 {
                total_size += iter_size + size_t +
                    num_rigid_bodies * rigidbodyparam_size;
                for j in 0..num_rigid_bodies {
                    let rigid_body = rigid_body_set
                        .lists
                        .get(format!("RigidBody_{}", j))
                        .unwrap();
                    let num_shapes: usize = rigid_body
                        .objects
                        .get("RigidBodyParam")
                        .unwrap()
                        .get("shape_num")
                        .unwrap_or(&I32(0))
                        .as_int()
                        .unwrap();
                    if num_shapes > 0 {
                        total_size += iter_size + size_t +
                            num_shapes * shapeparamobj_size;
                        for k in 0..num_shapes {
                            let num_vertices: usize = rigid_body
                                .objects
                                .get(format!("ShapeParam_{}", k))
                                .unwrap()
                                .get("vertex_num")
                                .unwrap_or(&I32(0))
                                .as_int()
                                .unwrap();
                            if num_vertices > 0 {
                                total_size += iter_size + size_t +
                                    num_vertices * vertex_size;
                            }
                        }
                    }
                }
            }
        }
    }

    if param_set_header.get("use_character_controller")
        .unwrap()
        .as_bool()
        .unwrap() {
        total_size += charactercontrollerparam_size;
        let character_controller = param_set
            .lists
            .get("CharacterController")
            .unwrap();
        let num_forms: usize = character_controller
            .objects
            .get("CharacterControllerParam")
            .unwrap()
            .get("form_num")
            .unwrap()
            .as_int()
            .unwrap();
        if num_forms > 0 {
            total_size += iter_size + size_t + num_forms * form_size;
            for i in 0..num_forms {
                let form = character_controller
                    .lists
                    .get(format!("Form_{}", i))
                    .unwrap();
                let num_shapes: usize = form
                    .objects
                    .get("FormHeader")
                    .unwrap()
                    .get("shape_num")
                    .unwrap()
                    .as_int()
                    .unwrap();
                if num_shapes > 0 {
                    total_size += iter_size + size_t +
                        num_shapes * shapeparamobj_size;
                    for j in 0..num_shapes {
                        let num_vertices: usize = form
                            .objects
                            .get(format!("ShapeParam_{}", j))
                            .unwrap()
                            .get("vertex_num")
                            .unwrap_or(&I32(0))
                            .as_int()
                            .unwrap();
                        if num_vertices > 0 {
                            total_size += iter_size + size_t +
                                num_vertices * vertex_size;
                        }
                    }
                }
            }
        }
    }

    if param_set_header.get("use_contact_info").unwrap().as_bool().unwrap() {
        total_size += contactinfoparam_size;
        let rigid_contact_info_header = param_set
            .lists
            .get("RigidContactInfo")
            .unwrap()
            .objects
            .get("RigidContactInfoHeader")
            .unwrap();
        let num_contact_point_info: usize = rigid_contact_info_header
            .get("contact_point_info_num")
            .unwrap()
            .as_int()
            .unwrap();
        if num_contact_point_info > 0 {
            total_size += iter_size + size_t +
                num_contact_point_info * contactpointinfoparam_size;
        }
        let num_collision_info: usize = rigid_contact_info_header
            .get("collision_info_num")
            .unwrap()
            .as_int()
            .unwrap();
        if num_collision_info > 0 {
            total_size += iter_size + size_t +
                num_collision_info * collisioninfoparam_size;
        }
    }

    if param_set_header.get("use_support_bone").unwrap().as_bool().unwrap() {
        total_size += supportboneparam_size;
    }

    if param_set_header.get("use_ragdoll").unwrap().as_bool().unwrap() {
        total_size += ragdollparam_size;
    }

    if param_set_header.get("use_cloth").unwrap().as_bool().unwrap() {
        total_size += clothsetparam_size;
        let num_cloth: usize = param_set
            .lists
            .get("Cloth")
            .unwrap()
            .objects
            .get("ClothHeader")
            .unwrap()
            .get("cloth_num")
            .unwrap()
            .as_int()
            .unwrap();
        if num_cloth > 0 {
            total_size += iter_size + size_t +
                num_cloth * clothparam_size;
        }
    }

    let num_edge_rigid_bodies: usize = param_set_header
        .get("use_edge_rigid_body_num")
        .unwrap()
        .as_int()
        .unwrap();
    if num_edge_rigid_bodies > 0 {
        total_size += edgerigidbodysetparam_size;
        total_size += iter_size + size_t +
            num_edge_rigid_bodies * edgerigidbodyparam_size;
    }

    Some(total_size as u32)
}
