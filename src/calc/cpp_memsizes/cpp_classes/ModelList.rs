use super::{agl::*, *};

#[repr(C)]
pub struct Unit<T> {
    unit_name: Parameter<T, SafeString<T>>,
    bind_bone: Parameter<T, SafeString<T>>,
    obj: ParameterObj<T>,
}

#[repr(C)]
pub struct ModelData<T> {
    folder: Parameter<T, SafeString<T>>,
    base_obj: ParameterObj<T>,
    units: SeadBuffer<T>,
    unit_list: ParameterList<T>,
    list: ParameterList<T>,
}

#[repr(C)]
pub struct Partial<T> {
    bone: Parameter<T, SafeString<T>>,
    bind_flag: Parameter<T, S32>,
    recursible: Parameter<T, Bool32>,
    obj: ParameterObj<T>,
}

#[repr(C)]
pub struct AnmTarget<T> {
    num_as_slot: Parameter<T, S32>,
    is_partical_enable: Parameter<T, Bool32>,
    target_type: Parameter<T, S32>,
    base_obj: ParameterObj<T>,
    partials: SeadBuffer<T>,
    partial_list: ParameterList<T>,
    list: ParameterList<T>,
}
