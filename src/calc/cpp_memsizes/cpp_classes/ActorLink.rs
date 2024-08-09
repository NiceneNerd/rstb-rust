use super::{agl, sead, F32, ParamIO, Resource};

#[repr(C)]
struct Users<T> {
    profile:            agl::Parameter<T, sead::SafeString<T>>,
    actor_capture:      agl::Parameter<T, sead::SafeString<T>>,
    as_:                agl::Parameter<T, sead::SafeString<T>>,
    model:              agl::Parameter<T, sead::SafeString<T>>,
    anim:               agl::Parameter<T, sead::SafeString<T>>,
    ai_program:         agl::Parameter<T, sead::SafeString<T>>,
    gparam:             agl::Parameter<T, sead::SafeString<T>>,
    damage_param:       agl::Parameter<T, sead::SafeString<T>>,
    rg_config_list:     agl::Parameter<T, sead::SafeString<T>>,
    rg_blend_weight:    agl::Parameter<T, sead::SafeString<T>>,
    awareness:          agl::Parameter<T, sead::SafeString<T>>,
    physics:            agl::Parameter<T, sead::SafeString<T>>,
    chemical:           agl::Parameter<T, sead::SafeString<T>>,
    attention:          agl::Parameter<T, sead::SafeString<T>>,
    elink:              agl::Parameter<T, sead::SafeString<T>>,
    slink:              agl::Parameter<T, sead::SafeString<T>>,
    xlink:              agl::Parameter<T, sead::SafeString<T>>,
    drop_table:         agl::Parameter<T, sead::SafeString<T>>,
    shop_data:          agl::Parameter<T, sead::SafeString<T>>,
    recipe:             agl::Parameter<T, sead::SafeString<T>>,
    lod:                agl::Parameter<T, sead::SafeString<T>>,
    bone_control:       agl::Parameter<T, sead::SafeString<T>>,
    ai_schedule:        agl::Parameter<T, sead::SafeString<T>>,
    life_condition:     agl::Parameter<T, sead::SafeString<T>>,
    umii:               agl::Parameter<T, sead::SafeString<T>>,
    animation_info:     agl::Parameter<T, sead::SafeString<T>>,
    obj:                agl::ParameterObj<T>,
}

#[repr(C)]
pub struct ActorLink<T> {
    base:           ParamIO<T>,                             // ParamIO
    base2:          Resource<T>,                            // Resource
    mUsers:         Users<T>,                               // Users
    mActorNameJpn:  agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    mPriority:      agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    mActorScale:    agl::Parameter<T, F32>,                 // agl::utl::Parameter<f32>
    mTags:          sead::Buffer<T>,                        // sead::Buffer<u32>
    mHeap:          T,                                      // sead::Heap*
}
