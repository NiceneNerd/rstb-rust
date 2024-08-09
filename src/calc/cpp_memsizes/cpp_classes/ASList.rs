use super::{agl, sead, Bool32, Float, ParamIO, Resource};

#[repr(C)]
pub struct ASList<T> {
    base:           ParamIO<T>,             // ParamIO
    base2:          Resource<T>,            // Resource
    _2b0:           u8,                     // u8
    _2b1:           u8,                     // u8
    _2b2:           u8,                     // u8
    _2b3:           u8,                     // u8
    mASDefines:     sead::Buffer<T>,        // sead::Buffer<ASDefine>
    mCFDefines:     sead::Buffer<T>,        // sead::Buffer<CFDefine>
    mAddReses:      sead::Buffer<T>,        // sead::Buffer<AddRes>
    mCFDefinesList: agl::ParameterList<T>,  // agl::utl::ParameterList
    mASDefinesList: agl::ParameterList<T>,  // agl::utl::ParameterList
    mAddResesList:  agl::ParameterList<T>,  // agl::utl::ParameterList
    mCommon:        Common<T>,              // Common
}

#[repr(C)]
pub struct ASDefine<T> {
    name:       agl::Parameter<T, sead::SafeString<T>>,
    file_name:  agl::Parameter<T, sead::SafeString<T>>,
    obj:        agl::ParameterObj<T>,
    animseq:    u64,
}

#[repr(C)]
pub struct CFPost<T> {
    name:               agl::Parameter<T, sead::SafeString<T>>,
    frame:              agl::Parameter<T, Float>,
    start_frame_rate:   agl::Parameter<T, Float>,
    obj:                agl::ParameterObj<T>,
}

#[repr(C)]
pub struct CFExcept<T> {
    name:   agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct CFDefine<T> {
    name:           agl::Parameter<T, sead::SafeString<T>>,
    pre_obj:        agl::ParameterObj<T>,
    posts:          sead::Buffer<T>,
    posts_list:     agl::ParameterList<T>,
    excepts:        sead::Buffer<T>,
    excepts_obj:    agl::ParameterObj<T>,
    list:           agl::ParameterList<T>,
}

#[repr(C)]
pub struct AddRes<T> {
    anim:                   agl::Parameter<T, sead::SafeString<T>>,
    retarget_model:         agl::Parameter<T, sead::SafeString<T>>,
    retarget_no_correct:    agl::Parameter<T, Bool32>,
    obj:                    agl::ParameterObj<T>,
}

#[repr(C)]
pub struct Common<T> {
    rate_all:   agl::Parameter<T, Float>,
    obj:        agl::ParameterObj<T>,
}
