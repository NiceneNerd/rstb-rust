use super::{agl, sead, ParamIO, Resource};

#[repr(C)]
pub struct LifeCondition<T> {
    base:                   ParamIO<T>,                             // ParamIO
    base2:                  Resource<T>,                            // Resource
    mInvalidWeathersObj:    agl::ParameterObj<T>,                   // agl::utl::ParameterObj
    mInvalidTimesObj:       agl::ParameterObj<T>,                   // agl::utl::ParameterObj
    mDisplayDistanceObj:    agl::ParameterObj<T>,                   // agl::utl::ParameterObj
    mDeleteWeathersObj:     agl::ParameterObj<T>,                   // agl::utl::ParameterObj
    mDeleteTimesObj:        agl::ParameterObj<T>,                   // agl::utl::ParameterObj
    mBoundingYObj:          agl::ParameterObj<T>,                   // agl::utl::ParameterObj
    mYLimitAlgorithmObj:    agl::ParameterObj<T>,                   // agl::utl::ParameterObj
    mInvalidWeathersBuffer: sead::Buffer<T>,                        // sead::Buffer<agl::utl::Parameter<sead::SafeString>>
    mInvalidTimesBuffer:    sead::Buffer<T>,                        // sead::Buffer<agl::utl::Parameter<sead::SafeString>>
    mDisplayDistance:       agl::Parameter<T, f32>,                 // agl::utl::Parameter<f32>
    mBoundingY:             agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    mYLimitAlgorithm:       agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    mDeleteWeathersBuffer:  sead::Buffer<T>,                        // sead::Buffer<agl::utl::Parameter<sead::SafeString>>
    mDeleteTimesBuffer:     sead::Buffer<T>,                        // sead::Buffer<agl::utl::Parameter<sead::SafeString>>
}
