use super::{S32, F32, SafeString, SeadBuffer};
use super::agl::{ParameterObj, Parameter};

#[repr(C)]
pub struct Table<T> {
    obj: ParameterObj<T>,
    name: Parameter<T, SafeString<T>>,
    repeat_num_min: Parameter<T, S32>,
    repeat_num_max: Parameter<T, S32>,
    approach_type: Parameter<T, S32>,
    occurrence_speed_type: Parameter<T, S32>,
    column_num: Parameter<T, S32>,
    items: SeadBuffer<T>,
}

#[repr(C)]
pub struct Item<T> {
    name: Parameter<T, SafeString<T>>,
    probability: Parameter<T, F32>,
}
