use super::SafeString;
use super::SeadBuffer;
use super::agl::*;
use super::{Bool32, Float};

#[repr(C)]
pub struct ASDefine<T> {
    name: Parameter<T, SafeString<T>>,
    file_name: Parameter<T, SafeString<T>>,
    obj: ParameterObj<T>,
    animseq: u64,
}

#[repr(C)]
pub struct CFPost<T> {
    name: Parameter<T, SafeString<T>>,
    frame: Parameter<T, Float>,
    start_frame_rate: Parameter<T, Float>,
    obj: ParameterObj<T>
}

#[repr(C)]
pub struct CFExcept<T> {
    name: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct CFDefine<T> {
    name: Parameter<T, SafeString<T>>,
    pre_obj: ParameterObj<T>,
    posts: SeadBuffer<T>,
    posts_list: ParameterList<T>,
    excepts: SeadBuffer<T>,
    excepts_obj: ParameterObj<T>,
    list: ParameterList<T>,
}

#[repr(C)]
pub struct AddRes<T> {
    anim: Parameter<T, SafeString<T>>,
    retarget_model: Parameter<T, SafeString<T>>,
    retarget_no_correct: Parameter<T, Bool32>,
    obj: ParameterObj<T>,
}

#[repr(C)]
pub struct Common<T> {
    rate_all: Parameter<T, Float>,
    obj: ParameterObj<T>,
}
