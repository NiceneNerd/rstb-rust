use super::{agl, sead, Bool32, ParamIO, Resource, S32};

#[repr(C)]
pub struct Table<T> {
    obj:        agl::ParameterObj<T>,
    name:       agl::Parameter<T, sead::SafeString<T>>,
    column_num: agl::Parameter<T, S32>,
    items:      sead::Buffer<T>,
}

#[repr(C)]
pub struct Item<T> {
    sort_value:         agl::Parameter<T, S32>,
    name:               agl::Parameter<T, sead::SafeString<T>>,
    num_stock:          agl::Parameter<T, S32>,
    price_adjustment:   agl::Parameter<T, S32>,
    demo_flag:          agl::Parameter<T, Bool32>,
    price:              agl::Parameter<T, S32>,
}

#[repr(C)]
pub struct Shop<T> {
    base:       ParamIO<T>,             // ParamIO
    base2:      Resource<T>,            // Resource
    mObj:       agl::ParameterObj<T>,   // agl::utl::ParameterObj
    mTableNum:  agl::Parameter<T, S32>, // agl::utl::Parameter<s32>
    _300:       sead::Buffer<T>,        // sead::Buffer<void*>
    mTables:    sead::Buffer<T>,        // sead::Buffer<Table>
}
