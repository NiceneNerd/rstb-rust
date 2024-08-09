// Commented properties are only compiled in the C++ libs
// with compiler directive SEAD_DEBUG, so presumably we don't
// care about them for production code

#[repr(C)]
struct SafeStringBase<T> {
    vfptr:      T,  // vtable*
    mStringTop: T,  // T*
}

#[repr(C)]
struct BufferedSafeStringBase<T> {
    base:           SafeStringBase<T>,  // SafeStringBase<T>
    mBufferSize:    i32,                // s32
}

#[repr(C)]
struct FixedSafeStringBase<T, U: Sized, const L: usize> {
    base:       BufferedSafeStringBase<T>,  // BufferedSafeStringBase<T>
    mBuffer:    [U; L],                     // T mBuffer[L]
}

#[repr(C)]
pub struct FixedSafeString<T, const L: usize> {
    base:   FixedSafeStringBase<T, u8, L>,    // FixedSafeStringBase<char, L>
}

#[repr(C)]
pub struct SafeString<T> {
    ptr: T,
    v:   [u8; 8],
}

#[repr(C)]
pub struct Buffer<T> {
    mSize:   i32, // s32
    mBuffer: T,   // T*
}

#[repr(C)]
pub struct BitFlag<T> {
    mBits:  T,  // T
}

#[repr(C)]
pub struct DirectResource<T> {
    vfptr:          T,              // vtable*
    mRawData:       T,              // u8*
    mRawSize:       u32,            // u32
    mBufferSize:    u32,            // u32
    mSettingFlag:   BitFlag<u32>,   // BitFlag32
}

#[repr(C)]
struct Resource<T> {
    vfptr:  T,  // vtable*
}

#[repr(C)]
pub struct Node<T> {
    base:       Reflexible<T>,  // Reflexible
    /*
    mTreeNode:  TTreeNode<T>,   // TTreeNode<Node*>
    */
}

#[repr(C)]
struct Reflexible<T> {
    base:                               NodeEventListener/*<T>*/,   // NodeEventListener
    vfptr:                              T,                          // vftable*
    /*
    mName:                              T,                          // char*
    mMeta:                              T,                          // char*
    mIsGenerated:                       bool,                       // bool
    mAllocFlg:                          BitFlag<u8>,                // BitFlag8
    sApplyEventDataToMemoryCallback:    T,                          // ApplyEventDataToMemoryCallback
    */
}

#[repr(C)]
struct NodeEventListener/*<T>*/ {
    base:   PropertyEventListener/*<T>*/,   // PropertyEventListener
    /*
    vfptr:  T,                          // vtable*
    */
}

#[repr(C)]
struct PropertyEventListener/*<T>*/ {
    base:   LifeCheckable/*<T>*/,   // LifeCheckable
    /*
    vfptr:  T,                          // vtable*
    */
}

#[repr(C)]
struct LifeCheckable/*<T>*/ {
    /*
    vfptr:          T,                                  // vtable*
    mCreateID:      u32,                                // u32
    mPrev:          T,                                  // LifeCheckable*
    mNext:          T,                                  // LifeCheckable*
    mDisposer:      T,                                  // DisposeHostIOCaller*
    mDisposerBuf:   StorageFor<DisposeHostIOCaller>,    // StorageFor<DisposeHostIOCaller>
    */
}

#[repr(C)]
pub struct Color3f {
    r: f32,
    g: f32,
    b: f32,
}

#[repr(C)]
pub struct Color4f {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[repr(C)]
pub struct Vector2f {
    x: f32,
    y: f32,
}

#[repr(C)]
pub struct Vector3f {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
pub struct Vector4f {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}
