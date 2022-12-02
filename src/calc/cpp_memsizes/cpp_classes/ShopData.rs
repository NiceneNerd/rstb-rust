use super::{
    agl::{Parameter, ParameterObj},
    Bool32, SafeString, SeadBuffer, S32,
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
    sort_value: Parameter<T, S32>,
    name: Parameter<T, SafeString<T>>,
    num_stock: Parameter<T, S32>,
    price_adjustment: Parameter<T, S32>,
    demo_flag: Parameter<T, Bool32>,
    price: Parameter<T, S32>,
}
