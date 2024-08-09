// TODO: Most of the WiiU classes have arbitrary values added to them
// that make the calculations work better. Test/have someone test the
// Switch version to see if it needs values added as well.

use std::mem::size_of;

use roead::aamp::ParameterIO;

use super::cpp_classes::{agl::Parameter, Physics::*, sead};
use crate::Endian;

const CLASS_SIZE_WIIU: usize = std::mem::size_of::<Physics<u32>>();
const CLASS_SIZE_NX: usize = std::mem::size_of::<Physics<u64>>();

//const OVERHEAD_WIIU: usize = 0xD4; // Might be necessary for mod files?
const OVERHEAD_WIIU: usize = 0x68;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX,
    };

    let a = ParameterIO::from_binary(bytes).ok()?;
    let (rigidbodysetparam_size, rigidbodyparam_size, shapeparamobj_size, vertex_size): (
        usize,
        usize,
        usize,
        usize,
    );
    let (charactercontrollerparam_size, form_size, ragdollparam_size, edgerigidbodyparam_size): (
        usize,
        usize,
        usize,
        usize,
    );
    let (contactinfoparam_size, contactpointinfoparam_size, collisioninfoparam_size): (
        usize,
        usize,
        usize,
    );
    let (clothsetparam_size, clothparam_size, edgerigidbodysetparam_size, supportboneparam_size): (
        usize,
        usize,
        usize,
        usize,
    );
    match endian {
        Endian::Big => {
            rigidbodysetparam_size = size_of::<RigidBodySetParam<u32>>();
            rigidbodyparam_size = size_of::<RigidBodyParam<u32>>();
            shapeparamobj_size = size_of::<ShapeParamObj<u32>>() + 0x30;
            vertex_size = size_of::<Parameter<u32, sead::Vector3f>>();
            charactercontrollerparam_size =
                size_of::<CharacterControllerParam<u32>>() + 0x20;
            form_size = size_of::<Form<u32>>() + 0x30;
            ragdollparam_size = size_of::<RagdollParam<u32>>();
            contactinfoparam_size = size_of::<ContactInfoParam<u32>>() + 0x10;
            contactpointinfoparam_size = size_of::<ContactPointInfoParam<u32>>() + 0x10;
            collisioninfoparam_size = size_of::<CollisionInfoParam<u32>>() + 0x10;
            clothsetparam_size = size_of::<ClothSetParam<u32>>() + 0x20;
            clothparam_size = size_of::<ClothParam<u32>>() + 0x10;
            edgerigidbodysetparam_size = size_of::<EdgeRigidBodySetParam<u32>>();
            edgerigidbodyparam_size = size_of::<EdgeRigidBodyParam<u32>>();
            supportboneparam_size = size_of::<SupportBoneParam<u32>>();
        }
        Endian::Little => {
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

    if let Some(paramset) = a.param_root.lists.get("ParamSet") {
        if let Some(paramsetheader) = paramset.objects.get("ParamSetHeader") {
            let num_rigid_body_sets: usize = paramsetheader
                .get("use_rigid_body_set_num")?
                .as_int()
                .ok()?;
            if num_rigid_body_sets > 0 {
                total_size += num_rigid_body_sets * rigidbodysetparam_size;
                for i in 0..num_rigid_body_sets {
                    if let Some(rigidbodysetlist) = paramset.lists.get("RigidBodySet") {
                        if let Some(rigidbodyset) =
                            rigidbodysetlist.lists.get(format!("RigidBodySet_{}", i))
                        {
                            if let Some(rigidbodysetheader) =
                                rigidbodyset.objects.get("RigidBodySetHeader")
                            {
                                let num_rigid_bodies: usize =
                                    rigidbodysetheader.get("num")?.as_int().ok()?;
                                total_size += num_rigid_bodies * rigidbodyparam_size;
                                for j in 0..num_rigid_bodies {
                                    if let Some(rigidbody) =
                                        rigidbodyset.lists.get(format!("RigidBody_{}", j))
                                    {
                                        if let Some(rigidbodyparam) =
                                            rigidbody.objects.get("RigidBodyParam")
                                        {
                                            if let Some(shape_num) = rigidbodyparam.get("shape_num")
                                            {
                                                let num_shapes: usize = shape_num.as_int().ok()?;
                                                total_size += num_shapes * shapeparamobj_size;
                                                for k in 0..num_shapes {
                                                    if let Some(shapeparam) = rigidbody
                                                        .objects
                                                        .get(format!("ShapeParam_{}", k))
                                                    {
                                                        if let Some(vertex_num) =
                                                            shapeparam.get("vertex_num")
                                                        {
                                                            let num_vertices: usize =
                                                                vertex_num.as_int().ok()?;
                                                            total_size +=
                                                                num_vertices * vertex_size;
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
            if paramsetheader
                .get("use_character_controller")?
                .as_bool()
                .ok()?
            {
                total_size += charactercontrollerparam_size;
                if let Some(charactercontroller) = paramset.lists.get("CharacterController") {
                    if let Some(charactercontrollerparam) =
                        charactercontroller.objects.get("CharacterControllerParam")
                    {
                        let num_forms: usize =
                            charactercontrollerparam.get("form_num")?.as_int().ok()?;
                        total_size += num_forms * form_size;
                        for i in 0..num_forms {
                            if let Some(form) = charactercontroller.lists.get(format!("Form_{}", i))
                            {
                                if let Some(formheader) = form.objects.get("FormHeader") {
                                    let num_shapes: usize =
                                        formheader.get("shape_num")?.as_int().ok()?;
                                    total_size += num_shapes * shapeparamobj_size;
                                    for j in 0..num_shapes {
                                        if let Some(shapeparam) =
                                            form.objects.get(format!("ShapeParam_{}", j))
                                        {
                                            if let Some(vertex_num) = shapeparam.get("vertex_num") {
                                                let num_vertices: usize = vertex_num.as_int().ok()?;
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
            if paramsetheader.get("use_contact_info")?.as_bool().ok()? {
                total_size += contactinfoparam_size;
                if let Some(rigidcontactinfo) = paramset.lists.get("RigidContactInfo") {
                    if let Some(rigidcontactinfoheader) =
                        rigidcontactinfo.objects.get("RigidContactInfoHeader")
                    {
                        let num_contact_point_info: usize = rigidcontactinfoheader
                            .get("contact_point_info_num")?
                            .as_int()
                            .ok()?;
                        total_size += num_contact_point_info * contactpointinfoparam_size;
                        let num_collision_info: usize = rigidcontactinfoheader
                            .get("collision_info_num")?
                            .as_int()
                            .ok()?;
                        total_size += num_collision_info * collisioninfoparam_size;
                    }
                }
            }
            if paramsetheader.get("use_support_bone")?.as_bool().ok()? {
                total_size += supportboneparam_size;
            }
            if paramsetheader.get("use_ragdoll")?.as_bool().ok()? {
                total_size += ragdollparam_size;
            }
            if paramsetheader.get("use_cloth")?.as_bool().ok()? {
                total_size += clothsetparam_size;
                if let Some(clothlist) = paramset.lists.get("Cloth") {
                    if let Some(clothheader) = clothlist.objects.get("ClothHeader") {
                        let num_cloth: usize = clothheader.get("cloth_num")?.as_int().ok()?;
                        total_size += num_cloth * clothparam_size;
                    }
                }
            }
            let num_edge_rigid_bodies: usize = paramsetheader
                .get("use_edge_rigid_body_num")?
                .as_int()
                .ok()?;
            if num_edge_rigid_bodies > 0 {
                total_size += edgerigidbodysetparam_size;
                total_size += num_edge_rigid_bodies * edgerigidbodyparam_size;
            }
        }
    }
    Some(total_size as u32)
}
