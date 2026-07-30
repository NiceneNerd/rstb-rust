use super::{sead, ParamIO, Resource};

#[repr(C)]
pub struct AnimInfo<T> {
    base:           ParamIO<T>,         // ParamIO
    base2:          Resource<T>,        // Resource
    mAnims:         sead::Buffer<T>,    // sead::Buffer<Anim>
    mSwordBlurInfo: T,                  // SwordBlurInfo*
}

#[repr(C)]
pub struct Anim<T> {
    name:   sead::SafeString<T>,    // sead::SafeString
    scale:  sead::Vector3f,         // sead::Vector3f
}

#[repr(C)]
pub struct SwordBlur<T> {
    frame_num:  i32,                    // int
    start:      i32,                    // int
    end:        i32,                    // int
    frames:     T,                      // sead::Matrix34f*
    _18:        T,                      // float*
    name:       sead::SafeString<T>,    // sead::SafeString
}

#[repr(C)]
pub struct SwordBlurInfo<T> {
    heap:           T,      // sead::Heap*
    num_entries:    i32,    // int
    entries:        T,      // SwordBlur*
}
