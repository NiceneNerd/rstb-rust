use super::{agl, sead, ParamIO, Resource, S32, U32};

#[repr(C)]
pub struct Table<T> {
    obj:        agl::ParameterObj<T>,
    name:       agl::Parameter<T, sead::SafeString<T>>,
    column_num: agl::Parameter<T, S32>,
    items:      sead::Buffer<T>,
}

#[repr(C)]
pub struct Item<T> {
    name: agl::Parameter<T, sead::SafeString<T>>,
    num:  agl::Parameter<T, S32>,
}

#[repr(C)]
pub struct Recipe<T> {
    base:       ParamIO<T>,             // ParamIO
    base2:      Resource<T>,            // Resource
    mObj:       agl::ParameterObj<T>,   // agl::ParameterObj
    mTableNum:  agl::Parameter<T, U32>, // agl::Parameter<u32>
    _300:       sead::Buffer<T>,        // sead::Buffer<void*>
    mTables:    sead::Buffer<T>,        // sead::Buffer<Table> but Buffer stores Table*
}
