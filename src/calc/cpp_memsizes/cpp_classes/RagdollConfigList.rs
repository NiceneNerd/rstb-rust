use super::{agl, sead, ParamIO, Resource};

#[repr(C)]
pub struct RagdollConfigList<T> {
    base:               ParamIO<T>,             // ParamIO
    base2:              Resource<T>,            // Resource
    mImpulseParamList:  agl::ParameterList<T>,  // agl::utl::ParameterList
    mImpulseParams:     sead::Buffer<T>,        // sead::Buffer<ImpulseParam>
    mCommonData:        agl::ParameterObj<T>,   // agl::utl::ParameterObj
    mUpperLimitHeight:  agl::Parameter<T, f32>, // agl::utl::Parameter<f32>
    mLowerLimitHeight:  agl::Parameter<T, f32>, // agl::utl::Parameter<f32>
    mBodyParamList:     agl::ParameterList<T>,  // agl::utl::ParameterList
    mBodyParams:        sead::Buffer<T>,        // sead::Buffer<BodyParam>
}

#[repr(C)]
pub struct ImpulseParam<T> {
    file_name:  agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    obj:        agl::ParameterObj<T>,                   // agl::utl::ParameterObj
    config:     T,                                      // RagdollConfig*
}

#[repr(C)]
pub struct BodyParam<T> {
    base:           agl::ParameterObj<T>,                   // agl::utl::ParameterObj
    file_name:      agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    friction_scale: agl::Parameter<T, f32>,                 // agl::utl::Parameter<f32>
    buoyancy_scale: agl::Parameter<T, f32>,                 // agl::utl::Parameter<f32>
}
