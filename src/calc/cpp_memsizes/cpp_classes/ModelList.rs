use super::*;

pub struct ModelList<T> {
    base:               ParamIO<T>,             // ParamIO
    base2:              Resource<T>,            // Resource
    mControllerInfo:    ControllerInfo<T>,      // ControllerInfo
    mAttention:         Attention<T>,           // Attention
    mRawData:           T,                      // u8*
    mModelData:         sead::Buffer<T>,        // sead::Buffer<ModelData>
    mModelDataList:     agl::ParameterList<T>,  // agl::ParameterList
    mAnmTargets:        sead::Buffer<T>,        // sead::Buffer<ModelData>
    mAnmTargetsList:    agl::ParameterList<T>,  // agl::ParameterList
    mIsDummy:           bool,                   // bool
}

#[repr(C)]
pub struct ControllerInfo<T> {
    mAddColor:                  agl::Parameter<T, sead::Color4f>,       // agl::util::Parameter<sead::Color4f>
    mMulColor:                  agl::Parameter<T, sead::Color4f>,       // agl::util::Parameter<sead::Color4f>
    mObj:                       agl::ParameterObj<T>,                   // agl::util::ParameterObj
    mBaseScale:                 agl::Parameter<T, sead::Vector3f>,      // agl::util::Parameter<sead::Vector3f>
    mVariationMatAnim:          agl::Parameter<T, sead::SafeString<T>>, // agl::util::Parameter<sead::SafeString>
    mVariationMatAnimFrame:     agl::Parameter<T, S32>,                 // agl::util::Parameter<s32>
    mVariationShaderAnim:       agl::Parameter<T, sead::SafeString<T>>, // agl::util::Parameter<sead::SafeString>
    mVariationShaderAnimFrame:  agl::Parameter<T, S32>,                 // agl::util::Parameter<s32>
    mCalcAABBASKey:             agl::Parameter<T, sead::SafeString<T>>, // agl::util::Parameter<sead::SafeString>
}

#[repr(C)]
pub struct Attention<T> {
    mIsEnableAttention:         agl::Parameter<T, Bool32>,              // agl::utl::Parameter<bool>
    mLookAtBone:                agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    mLookAtOffset:              agl::Parameter<T, sead::Vector3f>,      // agl::utl::Parameter<sead::Vector3f>
    mCursorOffsetY:             agl::Parameter<T, F32>,                 // agl::utl::Parameter<f32>
    mAIInfoOffsetY:             agl::Parameter<T, F32>,                 // agl::utl::Parameter<f32>
    mCutTargetBone:             agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    mCutTargetOffset:           agl::Parameter<T, sead::Vector3f>,      // agl::utl::Parameter<sead::Vector3f>
    mGameCameraBone:            agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    mGameCameraOffset:          agl::Parameter<T, sead::Vector3f>,      // agl::utl::Parameter<sead::Vector3f>
    mBowCameraBone:             agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    mBowCameraOffset:           agl::Parameter<T, sead::Vector3f>,      // agl::utl::Parameter<sead::Vector3f>
    mAttackTargetBone:          agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    mAttackTargetOffset:        agl::Parameter<T, sead::Vector3f>,      // agl::utl::Parameter<sead::Vector3f>
    mAttackTargetOffsetBack:    agl::Parameter<T, F32>,                 // agl::utl::Parameter<f32>
    mAtObstacleChkOffsetBone:   agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    mAtObstacleChkOffset:       agl::Parameter<T, sead::Vector3f>,      // agl::utl::Parameter<sead::Vector3f>
    mAtObstacleChkUseLookAtPos: agl::Parameter<T, Bool32>,              // agl::utl::Parameter<bool>
    mCursorAIInfoBaseBone:      agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    mCursorAIInfoBaseOffset:    agl::Parameter<T, sead::Vector3f>,      // agl::utl::Parameter<sead::Vector3f>
    mObj:                       agl::ParameterObj<T>,                   // agl::utl::ParameterObj
}

#[repr(C)]
pub struct Unit<T> {
    unit_name:  agl::Parameter<T, sead::SafeString<T>>,
    bind_bone:  agl::Parameter<T, sead::SafeString<T>>,
    obj:        agl::ParameterObj<T>,
}

#[repr(C)]
pub struct ModelData<T> {
    folder:     agl::Parameter<T, sead::SafeString<T>>,
    base_obj:   agl::ParameterObj<T>,
    units:      sead::Buffer<T>,
    unit_list:  agl::ParameterList<T>,
    list:       agl::ParameterList<T>,
}

#[repr(C)]
pub struct Partial<T> {
    bone:       agl::Parameter<T, sead::SafeString<T>>,
    bind_flag:  agl::Parameter<T, S32>,
    recursible: agl::Parameter<T, Bool32>,
    obj:        agl::ParameterObj<T>,
}

#[repr(C)]
pub struct AnmTarget<T> {
    num_as_slot:        agl::Parameter<T, S32>,
    is_partical_enable: agl::Parameter<T, Bool32>,
    target_type:        agl::Parameter<T, S32>,
    base_obj:           agl::ParameterObj<T>,
    partials:           sead::Buffer<T>,
    partial_list:       agl::ParameterList<T>,
    list:               agl::ParameterList<T>,
}

#[repr(C)]
pub struct ModelDataInfo<T> {
    unit_names:         [[T; 8]; 1],    // std::array<std::array<const char*, 8>, 1>
    unit_bind_bones:    [[T; 8]; 1],    // std::array<std::array<const char*, 8>, 1>
    num_units:          [i32; 1],       // std::array<int, 1>
    folder_name:        [T; 1],         // std::array<const char*, 1>
    num_model_data:     i32,            // int
    base_scale:         sead::Vector3f  // sead::Vector3f
}

#[repr(C)]
pub struct AttentionInfo<T> {
    look_at_bone:               T,              // const char*
    look_at_offset:             sead::Vector3f, // sead::Vector3f
    cursor_offset_y:            f32,            // float
    ai_info_offset_y:           f32,            // float
    cut_target_bone:            T,              // const char*
    cut_target_offset:          sead::Vector3f, // sead::Vector3f
    game_camera_bone:           T,              // const char*
    game_camera_offset:         sead::Vector3f, // sead::Vector3f
    bow_camera_bone:            T,              // const char*
    bow_camera_offset:          sead::Vector3f, // sead::Vector3f
    attack_target_bone:         T,              // const char*
    attack_target_offset:       sead::Vector3f, // sead::Vector3f
    attack_target_offset_back:  f32,            // float
    at_obstacle_chk_bone:       T,              // const char*
    at_obstacle_chk_offset:     sead::Vector3f, // sead::Vector3f
    cursor_ai_info_base_bone:   T,              // const char*
    cursor_ai_info_base_offset: sead::Vector3f, // sead::Vector3f
}

#[repr(C)]
pub struct PartialInfo<T> {
    bone:       sead::SafeString<T>,    // sead::SafeString
    bind_flag:  i32,                    // int
    recursible: bool,                   // bool
}
