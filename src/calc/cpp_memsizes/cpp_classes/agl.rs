use super::sead;

#[repr(C)]
pub struct IParameterIO<T> {
    base:           IParameterList<T>,
    mType:          sead::FixedSafeString<T, 64>,   // sead::FixedSafeString<64>
    mVersion:       u32,                            // u32
    _a8:            T,                              // void*
    _b0:            sead::FixedSafeString<T, 256>,  // sead::FixedSafeString<256>
    mResFileSize:   u32,                            // u32
    _1cc:           u32,                            // u32
}

#[repr(C)]
struct IParameterList<T> {
    vfptr:              T,      // vtable*
    mpChildObjHead:     T,      // IParameterObj*
    mpChildObjTail:     T,      // IParameterObj*
    mpChildListHead:    T,      // IParameterList*
    mpChildListTail:    T,      // IParameterList*
    mNameHash:          u32,    // u32
    mNext:              T,      // IParameterList*
    mParent:            T,      // IParameterList*
    mName:              T,      // char*
}

#[repr(C)]
pub struct ParameterList<T> {
    base:   IParameterList<T>,
}

#[repr(C)]
struct IParameterObj<T> {
    vfptr:          T,      // vtable*
    mParamListHead: T,      // ParameterBase*
    mParamListTail: T,      // ParameterBase*
    mParamListSize: u32,    // u32
    mNameHash:      u32,    // u32
    mNext:          T,      // IParameterObj*
    mName:          T,      // char*
}

#[repr(C)]
pub struct ParameterObj<T> {
    base:   IParameterObj<T>,
}

#[repr(C)]
pub struct ParameterBase<T> {
    vfptr:     T,   // vtable*
    mNameHash: u32, // u32
    mNext:     T,   // ParameterBase*
}

#[repr(C)]
pub struct Parameter<T, U> {
    base:   ParameterBase<T>,
    mValue: U,
}
