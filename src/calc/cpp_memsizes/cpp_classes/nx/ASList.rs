use super::SafeString;
use super::SeadBuffer;
use super::agl::*;
use super::super::{Bool32, Float};

#[repr(C)]
pub struct ASDefine {
    name: Parameter<SafeString>,
    file_name: Parameter<SafeString>,
    obj: ParameterObj,
    animseq: u64,
}

#[repr(C)]
pub struct CFPost {
    name: Parameter<SafeString>,
    frame: Parameter<Float>,
    start_frame_rate: Parameter<Float>,
    obj: ParameterObj
}

#[repr(C)]
pub struct CFExcept {
    name: Parameter<SafeString>,
}

#[repr(C)]
pub struct CFDefine {
    name: Parameter<SafeString>,
    pre_obj: ParameterObj,
    posts: SeadBuffer,
    posts_list: ParameterList,
    excepts: SeadBuffer,
    excepts_obj: ParameterObj,
    list: ParameterList,
}

#[repr(C)]
pub struct AddRes {
    anim: Parameter<SafeString>,
    retarget_model: Parameter<SafeString>,
    retarget_no_correct: Parameter<Bool32>,
    obj: ParameterObj,
}

#[repr(C)]
pub struct Common {
    rate_all: Parameter<Float>,
    obj: ParameterObj,
}
