use super::{agl, sead, ParamIO, Resource, F32, S32};

#[repr(C)]
pub struct Drop<T> {
    base:       ParamIO<T>,             // ParamIO
    base2:      Resource<T>,            // Resource
    mObj:       agl::ParameterObj<T>,   // agl::utl::ParameterObj
    mTableNum:  agl::Parameter<T, u32>, // agl::utl::Parameter<u32>
    _300:       sead::Buffer<T>,        // sead::Buffer<void*>
    mTables:    sead::Buffer<T>,        // sead::Buffer<Table>
}

#[repr(C)]
pub struct Table<T> {
    obj:                    agl::ParameterObj<T>,
    name:                   agl::Parameter<T, sead::SafeString<T>>,
    repeat_num_min:         agl::Parameter<T, S32>,
    repeat_num_max:         agl::Parameter<T, S32>,
    approach_type:          agl::Parameter<T, S32>,
    occurrence_speed_type:  agl::Parameter<T, S32>,
    column_num:             agl::Parameter<T, S32>,
    items:                  sead::Buffer<T>,
}

#[repr(C)]
pub struct Item<T> {
    name:           agl::Parameter<T, sead::SafeString<T>>,
    probability:    agl::Parameter<T, F32>,
}
