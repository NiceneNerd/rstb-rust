use super::{agl, sead};

#[repr(C)]
pub struct SupportBoneResource<T> {
    base:                       sead::DirectResource<T>,    // sead::DirectResource
    base2:                      agl::IParameterIO<T>,       // agl::utl::IParameterIO
    mSupportBoneHeader:         agl::ParameterObj<T>,       // agl::utl::ParameterObj
    mSupportBoneData:           agl::ParameterList<T>,      // agl::utl::ParameterList
    mBoneNum:                   agl::Parameter<T, i32>,     // agl::utl::Parameter<int>
    mBoneList:                  agl::ParameterList<T>,      // agl::utl::ParameterList
    boneBuffer:                 sead::Buffer<T>,            // sead::Buffer<SupportBoneResource::Bone>
    mConnectionLinearNum:       agl::Parameter<T, i32>,     // agl::utl::Parameter<int>
    mConnectionLinearList:      agl::ParameterList<T>,      // agl::utl::ParameterList
    mConnectionLinearBuffer:    sead::Buffer<T>,            // sead::Buffer<SupportBoneResource::ConnectionLinear>
    mConnectionCurveNum:        agl::Parameter<T, i32>,     // agl::utl::Parameter<int>
    mConnectionCurveList:       agl::ParameterList<T>,      // agl::utl::ParameterList
    mConnectionCurveBuffer:     sead::Buffer<T>,            // sead::Buffer<SupportBoneResource::ConnectionCurve>
    mOutputSingleNum:           agl::Parameter<T, i32>,     // agl::utl::Parameter<int>
    mOutputSingleList:          agl::ParameterList<T>,      // agl::utl::ParameterList
    mOutputSingleBuffer:        sead::Buffer<T>,            // sead::Buffer<SupportBoneResource::OutputSingle>
    mOutputDoubleNum:           agl::Parameter<T, i32>,     // agl::utl::Parameter<int>
    mOutputDoubleList:          agl::ParameterList<T>,      // agl::utl::ParameterList
    mOutputDoubleBuffer:        sead::Buffer<T>,            // sead::Buffer<SupportBoneResource::OutputDouble>
    mMainBoneNum:               agl::Parameter<T, i32>,     // agl::utl::Parameter<int>
    mMainBoneList:              agl::ParameterList<T>,      // agl::utl::ParameterList
    mMainBoneBuffer:            sead::Buffer<T>,            // sead::Buffer<SupportBoneResource::MainBone>
    mSupportBoneNum:            agl::Parameter<T, i32>,     // agl::utl::Parameter<int>
    mSupportBoneList:           agl::ParameterList<T>,      // agl::utl::ParameterList
    mSupportBoneBuffer:         sead::Buffer<T>,            // sead::Buffer<SupportBoneResource::SupportBone>
}

#[repr(C)]
pub struct Bone<T> {
    base:   agl::ParameterObj<T>,                               // agl::utl::ParameterObj
    name:   agl::Parameter<T, sead::FixedSafeString<T, 64>>,    // agl::utl::Parameter<sead::FixedSafeString<64>>
}

#[repr(C)]
pub struct ConnectionLinear<T> {
    base:               agl::ParameterObj<T>,               // agl::utl::ParameterObj
    bone_attribute:     agl::Parameter<T, i32>,             // agl::utl::Parameter<int>
    slope_intercept:    agl::Parameter<T, sead::Vector2f>,  // agl::utl::Parameter<sead::Vector2f>
}

#[repr(C)]
pub struct ConnectionCurve<T> {
    base:           agl::ParameterObj<T>,               // agl::utl::ParameterObj
    bone_attribute: agl::Parameter<T, i32>,             // agl::utl::Parameter<int>
    constant_in:    agl::Parameter<T, bool>,            // agl::utl::Parameter<bool>
    constant_out:   agl::Parameter<T, bool>,            // agl::utl::Parameter<bool>
    key0:           agl::Parameter<T, sead::Vector4f>,  // agl::utl::Parameter<sead::Vector4f>
    key1:           agl::Parameter<T, sead::Vector4f>,  // agl::utl::Parameter<sead::Vector4f>
    key2:           agl::Parameter<T, sead::Vector4f>,  // agl::utl::Parameter<sead::Vector4f>
}

#[repr(C)]
pub struct OutputSingle<T> {
    base:       agl::ParameterObj<T>,   // agl::utl::ParameterObj
    connection: agl::Parameter<T, i32>, // agl::utl::Parameter<int>
    weight:     agl::Parameter<T, f32>, // agl::utl::Parameter<float>
}

#[repr(C)]
pub struct OutputDouble<T> {
    base:           agl::ParameterObj<T>,   // agl::utl::ParameterObj
    connection_0:   agl::Parameter<T, i32>, // agl::utl::Parameter<int>
    weight_0:       agl::Parameter<T, f32>, // agl::utl::Parameter<float>
    connection_1:   agl::Parameter<T, i32>, // agl::utl::Parameter<int>
    weight_1:       agl::Parameter<T, f32>, // agl::utl::Parameter<float>
}

#[repr(C)]
pub struct BaseBone<T> {
    base:           agl::ParameterObj<T>,               // agl::utl::ParameterObj
    bone:           agl::Parameter<T, i32>,             // agl::utl::Parameter<int>
    aim:            agl::Parameter<T, sead::Vector3f>,  // agl::utl::Parameter<sead::Vector3f>
    up:             agl::Parameter<T, sead::Vector3f>,  // agl::utl::Parameter<sead::Vector3f>
    space:          agl::Parameter<T, i32>,             // agl::utl::Parameter<int>
    base_rotate:    agl::Parameter<T, sead::Quatf>,     // agl::utl::Parameter<sead::Quatf>
    base_translate: agl::Parameter<T, sead::Vector3f>,  // agl::utl::Parameter<sead::Vector3f>
    reverse_rotate: sead::Quatf,                        // sead::Quatf
    side:           sead::Vector3f,                     // sead::Vector3<float>
}

#[repr(C)]
pub struct MainBone<T> {
    base:   BaseBone<T>,    // BaseBone
}

#[repr(C)]
pub struct SupportBone<T> {
    base:       BaseBone<T>,            // BaseBone
    bendH:      agl::Parameter<T, i32>, // agl::utl::Parameter<int>
    bendV:      agl::Parameter<T, i32>, // agl::utl::Parameter<int>
    roll:       agl::Parameter<T, i32>, // agl::utl::Parameter<int>
    translateX: agl::Parameter<T, i32>, // agl::utl::Parameter<int>
    translateY: agl::Parameter<T, i32>, // agl::utl::Parameter<int>
    translateZ: agl::Parameter<T, i32>, // agl::utl::Parameter<int>
}
