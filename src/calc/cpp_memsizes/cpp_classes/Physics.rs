use super::*;
use super::agl::*;


#[repr(C)]
pub struct RigidBodySetParam<T> {
    base: ParameterList<T>,
    type_val: u32, // enum
    obj: ParameterObj<T>,
    set_name: Parameter<T, SafeString<T>>,
    set_type: Parameter<T, SafeString<T>>,
    num: Parameter<T, Int>,
    setup_file_path: Parameter<T, SafeString<T>>,
    rigid_bodies: SeadBuffer<T>,
}

#[repr(C)]
pub struct CharacterControllerParam<T> {
    base: ParameterList<T>,
    base2: ICharacterControllerParam<T>,
    obj: ParameterObj<T>,
    mass: Parameter<T, Float>,
    volume: Parameter<T, Float>,
    max_force: Parameter<T, Float>,
    form_num: Parameter<T, Int>,
    layer: Parameter<T, FixedSafeString32<T>>,
    groundhit: Parameter<T, FixedSafeString32<T>>,
    initial_state: Parameter<T, FixedSafeString32<T>>,
    initial_form: Parameter<T, FixedSafeString32<T>>,
    max_impulse: Parameter<T, Float>,
    contact_point_info: Parameter<T, FixedSafeString32<T>>,
    collision_info: Parameter<T, FixedSafeString32<T>>,
    use_nav_mesh_character: Parameter<T, Bool32>,
    nav_mesh_character_radius: Parameter<T, Float>,
    nav_mesh_character_height: Parameter<T, Float>,
    nav_mesh_character_avoidance_priority: Parameter<T, U32>,
    nav_mesh_character_max_speed: Parameter<T, Float>,
    nav_mesh_character_max_acceleration: Parameter<T, Float>,
    nav_mesh_character_max_angular_velocity: Parameter<T, Float>,
    nav_mesh_character_type: Parameter<T, SafeString<T>>,
    enable_water_effect: Parameter<T, Bool32>,
    enable_force_fall_cliff_edge: Parameter<T, Bool32>,
    water_effective_height: Parameter<T, Float>,
    water_flow_effective_rate: Parameter<T, Float>,
    water_attn_effective_rate: Parameter<T, Float>,
    max_force_scale_NPC: Parameter<T, Float>,
    water_buoyancy_scale: Parameter<T, Float>,
    magne_mass_scaling_factor: Parameter<T, Float>,
    height_enable_hitting_wall: Parameter<T, Float>,
    forms: SeadBuffer<T>,
}

#[repr(C)]
pub struct ClothSetParam<T> {
    base: ParameterList<T>,
    cloth_header_obj: ParameterObj<T>,
    cloth_setup_file_path: Parameter<T, SafeString<T>>,
    cloth_num: Parameter<T, Int>,
    cloth_setup_file_name: FixedSafeString32<T>,
    sub_wind: ClothSubWindParam<T>,
    cloths: SeadBuffer<T>,
}

#[repr(C)]
pub struct RagdollParam<T> {
    base: ParameterObj<T>,
    contact_point_info: Parameter<T, SafeString<T>>,
    collision_info: Parameter<T, SafeString<T>>,
    ragdoll_setup_file_path: Parameter<T, SafeString<T>>,
    ragdoll_setup_file_stem: FixedSafeString32<T>,
}

#[repr(C)]
pub struct SupportBoneParam<T> {
    base: ParameterObj<T>,
    support_bone_setup_file_path: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct ContactInfoParam<T> {
    base: ParameterList<T>,
    contact_point_info: SeadBuffer<T>,
    collision_info: SeadBuffer<T>,
    obj: ParameterObj<T>,
    contact_point_info_num: Parameter<T, Int>,
    collision_info_num: Parameter<T, Int>,
}

#[repr(C)]
pub struct EdgeRigidBodySetParam<T> {
    base: ParameterList<T>,
    edge_rigid_bodies: SeadBuffer<T>,
}

#[repr(C)]
struct Info<T> {
    base: ParameterObj<T>,
    rigid_body_name: Parameter<T, SafeString<T>>,
    mass: Parameter<T, Float>,
    volume: Parameter<T, Float>,
    toi: Parameter<T, Bool32>,
    mEnableAutoAddWorld: Parameter<T, Bool32>,
    navmesh: Parameter<T, SafeString<T>>,
    navmesh_sub_material: Parameter<T, SafeString<T>>,
    inertia: Parameter<T, Vector3f>,
    center_of_mass: Parameter<T, Vector3f>,
    bounding_center: Parameter<T, Vector3f>,
    bounding_extents: Parameter<T, Vector3f>,
    contact_point_info: Parameter<T, SafeString<T>>,
    collision_info: Parameter<T, SafeString<T>>,
    max_linear_velocity: Parameter<T, Float>,
    linear_damping: Parameter<T, Float>,
    max_angular_velocity_rad: Parameter<T, Float>,
    angular_damping: Parameter<T, Float>,
    max_impulse: Parameter<T, Float>,
    col_impulse_scale: Parameter<T, Float>,
    ignore_normal_for_impulse: Parameter<T, Bool32>,
    always_character_mass_scaling: Parameter<T, Bool32>,
    friction_scale: Parameter<T, Float>,
    restitution_scale: Parameter<T, Float>,
    water_buoyancy_scale: Parameter<T, Float>,
    water_flow_effective_rate: Parameter<T, Float>,
    magne_mass_scaling_factor: Parameter<T, Float>,
    motion_type: Parameter<T, SafeString<T>>,
    layer: Parameter<T, SafeString<T>>,
    groundhit: Parameter<T, SafeString<T>>,
    use_ground_hit_type_mask: Parameter<T, Bool32>,
    ground_hit_type_mask: Parameter<T, SafeString<T>>,
    receiver_type: Parameter<T, SafeString<T>>,
    no_hit_ground: Parameter<T, Bool32>,
    no_hit_water: Parameter<T, Bool32>,
    no_char_standing_on: Parameter<T, Bool32>,
    contact_mask: Parameter<T, U32>,
    link_matrix: Parameter<T, SafeString<T>>,
    link_entity_set: Parameter<T, SafeString<T>>,
    link_entity_body: Parameter<T, SafeString<T>>,
    use_entity_shape: Parameter<T, Bool32>,
    shape_num: Parameter<T, Int>,
    navmesh_val: u32, // enum
    navmesh_sub_material_val: u32, // enum
    ground_hit_mask: u32,
}

#[repr(C)]
pub struct RigidBodyParam<T> {
    base: ParameterList<T>,
    info: Info<T>,
    shapes: SeadBuffer<T>,
}

#[repr(C)]
struct ICharacterControllerParam<T> {
    vfptr: T, // vfptr*
}

#[repr(C)]
pub struct Form<T> {
    base: ParameterList<T>,
    form_header_obj: ParameterObj<T>,
    shape_num: Parameter<T, Int>,
    form_type: Parameter<T, FixedSafeString32<T>>,
    shape_params: SeadBuffer<T>
}

#[repr(C)]
struct ClothSubWindParam<T> {
    base: ParameterObj<T>,
    sub_wind_direction: Parameter<T, Vector3f>,
    sub_wind_frequency: Parameter<T, Float>,
    sub_wind_speed: Parameter<T, Float>,
}

#[repr(C)]
pub struct ClothParam<T> {
    base: ParameterObj<T>,
    wind_drag: Parameter<T, Float>,
    wind_frequency: Parameter<T, Float>,
    wind_min_speed: Parameter<T, Float>,
    wind_max_speed: Parameter<T, Float>,
    sub_wind_factor_main: Parameter<T, Float>,
    sub_wind_factor_add: Parameter<T, Float>,
    wind_enable: Parameter<T, Bool32>,
    writeback_to_local: Parameter<T, Bool32>,
    name: Parameter<T, SafeString<T>>,
    base_bone: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct ContactPointInfoParam<T> {
    base: ParameterObj<T>,
    name: Parameter<T, FixedSafeString32<T>>,
    contact_type: Parameter<T, FixedSafeString32<T>>,
    num: Parameter<T, Int>,
}

#[repr(C)]
pub struct CollisionInfoParam<T> {
    base: ParameterObj<T>,
    name: Parameter<T, FixedSafeString32<T>>,
    collision_type: Parameter<T, FixedSafeString32<T>>,
}

#[repr(C)]
pub struct EdgeRigidBodyParam<T> {
    base: ParameterObj<T>,
    set_name: Parameter<T, SafeString<T>>,
    body_name: Parameter<T, SafeString<T>>,
    edge_type: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct ShapeParamObj<T> {
    base: ParameterObj<T>,
    shape_type: Parameter<T, FixedSafeString32<T>>,
    radius: Parameter<T, Float>,
    convex_radius: Parameter<T, Float>,
    translate_0: Parameter<T, Vector3f>,
    translate_1: Parameter<T, Vector3f>,
    rotate: Parameter<T, Vector3f>,
    vertex_num: Parameter<T, Int>,
    vertices: SeadBuffer<T>,
    material: Parameter<T, FixedSafeString32<T>>,
    sub_material: Parameter<T, FixedSafeString32<T>>,
    wall_code: Parameter<T, FixedSafeString32<T>>,
    floor_code: Parameter<T, FixedSafeString32<T>>,
    item_code_disable_stick: Parameter<T, Bool32>,
}
