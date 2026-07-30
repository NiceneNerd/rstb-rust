use super::{agl, sead, ParamIO, Resource};

#[repr(C)]
pub struct Table<T> {
    obj:        agl::ParameterObj<T>,
    name:       agl::Parameter<T, sead::SafeString<T>>,
    column_num: agl::Parameter<T, i32>,
    items:      sead::Buffer<T>,
}

#[repr(C)]
pub struct Item<T> {
    sort_value:         agl::Parameter<T, i32>,
    name:               agl::Parameter<T, sead::SafeString<T>>,
    num_stock:          agl::Parameter<T, i32>,
    price_adjustment:   agl::Parameter<T, i32>,
    demo_flag:          agl::Parameter<T, bool>,
    price:              agl::Parameter<T, i32>,
}

#[repr(C)]
pub struct Shop<T> {
    base:       ParamIO<T>,             // ParamIO
    base2:      Resource<T>,            // Resource
    mObj:       agl::ParameterObj<T>,   // agl::utl::ParameterObj
    mTableNum:  agl::Parameter<T, i32>, // agl::utl::Parameter<int>
    _300:       sead::Buffer<T>,        // sead::Buffer<void*>
    mTables:    sead::Buffer<T>,        // sead::Buffer<Table>
}
