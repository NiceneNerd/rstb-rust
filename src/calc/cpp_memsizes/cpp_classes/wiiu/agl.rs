use super::super::IParamType;

#[repr(C)]
pub struct ParameterList {
    vfptr: u32, // vfptr*
    mpChildObjHead: u32, // IParameterObj*
    mpChildObjTail: u32, // IParameterObj*
    mpChildListHead: u32, // IParameterList*
    mpChildListTail: u32, // IParameterList*
    mNameHash: u32, // u32
    mNext: u32, // IParameterList*
    mParent: u32, // IParameterList*
    mName: u32 // char*
}

#[repr(C)]
pub struct ParameterObj {
    vfptr: u32, // vfptr*
    mParamListHead: u32, // ParameterBase*
    mParamListTail: u32, // ParameterBase*
    mParamListSize: u32, // u32
    mNameHash: u32, // u32
    mNext: u32, // IParameterObj*
    mName: u32 // char*
}

#[repr(C)]
pub struct ParameterBase {
    vfptr: u32,
    mNameHash: u32,
    mNext: u32,
}

#[repr(C)]
pub struct Parameter<T: IParamType> {
    base: ParameterBase,
    mValue: T
}
