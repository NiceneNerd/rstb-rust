use super::{
    agl::{Parameter, ParameterObj},
    SafeString, SeadBuffer, S32,
};

#[repr(C)]
pub struct Table<T> {
    obj: ParameterObj<T>,
    name: Parameter<T, SafeString<T>>,
    column_num: Parameter<T, S32>,
    items: SeadBuffer<T>,
}

#[repr(C)]
pub struct Item<T> {
    name: Parameter<T, SafeString<T>>,
    num:  Parameter<T, S32>,
}
