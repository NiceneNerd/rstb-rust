use super::{agl, sead, ParamIO, Resource};

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
    name:       agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    file_name:  agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    obj:        agl::ParameterObj<T>,                   // agl::utl::ParameterObj
    animseq:    T,                                      // AS*
}

#[repr(C)]
pub struct CFPost<T> {
    name:               agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    frame:              agl::Parameter<T, f32>,                 // agl::utl::Parameter<float>
    start_frame_rate:   agl::Parameter<T, f32>,                 // agl::utl::Parameter<float>
    obj:                agl::ParameterObj<T>,                   // agl::utl::ParameterObj
}

#[repr(C)]
pub struct CFExcept<T> {
    name:   agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
}

#[repr(C)]
pub struct CFDefine<T> {
    name:           agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    pre_obj:        agl::ParameterObj<T>,                   // agl::utl::ParameterObj
    posts:          sead::Buffer<T>,                        // sead::Buffer<CFPost>
    posts_list:     agl::ParameterList<T>,                  // agl::utl::ParameterList
    excepts:        sead::Buffer<T>,                        // sead::Buffer<CFExcept>
    excepts_obj:    agl::ParameterObj<T>,                   // agl::utl::ParameterObj
    list:           agl::ParameterList<T>,                  // agl::utl::ParameterList
}

#[repr(C)]
pub struct AddRes<T> {
    anim:                   agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    retarget_model:         agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    retarget_no_correct:    agl::Parameter<T, bool>,                // agl::utl::Parameter<bool>
    obj:                    agl::ParameterObj<T>,                   // agl::utl::ParameterObj
}

#[repr(C)]
pub struct Common<T> {
    rate_all:   agl::Parameter<T, f32>, // agl::utl::Parameter<float>
    obj:        agl::ParameterObj<T>,   // agl::utl::ParameterObj
}
