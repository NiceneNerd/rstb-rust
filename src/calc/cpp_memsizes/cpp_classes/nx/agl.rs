use super::super::IParamType;

#[repr(C)]
pub struct ParameterList {
    vfptr: u64, // vfptr*
    mpChildObjHead: u64, // IParameterObj*
    mpChildObjTail: u64, // IParameterObj*
    mpChildListHead: u64, // IParameterList*
    mpChildListTail: u64, // IParameterList*
    mNameHash: u32, // u32
    mNext: u64, // IParameterList*
    mParent: u64, // IParameterList*
    mName: u64 // char*
}

#[repr(C)]
pub struct ParameterObj {
    vfptr: u64, // vfptr*
    mParamListHead: u64, // ParameterBase*
    mParamListTail: u64, // ParameterBase*
    mParamListSize: u32, // u32
    mNameHash: u32, // u32
    mNext: u64, // IParameterObj*
    mName: u64 // char*
}

#[repr(C)]
pub struct ParameterBase {
    vfptr: u64,
    mNameHash: u64,
    mNext: u64
}

#[repr(C)]
pub struct Parameter<T: IParamType> {
    base: ParameterBase,
    mValue: T
}
