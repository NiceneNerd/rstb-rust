#[repr(C)]
pub struct ParameterList<T> {
    vfptr: T, // vfptr*
    mpChildObjHead: T, // IParameterObj*
    mpChildObjTail: T, // IParameterObj*
    mpChildListHead: T, // IParameterList*
    mpChildListTail: T, // IParameterList*
    mNameHash: u32, // u32
    mNext: T, // IParameterList*
    mParent: T, // IParameterList*
    mName: T // char*
}

#[repr(C)]
pub struct ParameterObj<T> {
    vfptr: T, // vfptr*
    mParamListHead: T, // ParameterBase*
    mParamListTail: T, // ParameterBase*
    mParamListSize: u32, // u32
    mNameHash: u32, // u32
    mNext: T, // IParameterObj*
    mName: T // char*
}

#[repr(C)]
pub struct ParameterBase<T> {
    vfptr: T, // vfptr*
    mNameHash: u32, // u32
    mNext: T // ParameterBase*
}

#[repr(C)]
pub struct Parameter<T, U> {
    base: ParameterBase<T>,
    mValue: U
}
