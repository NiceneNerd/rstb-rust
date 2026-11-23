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
    v: SafeStringBase<T>,
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
pub struct BaseVector2<U> {
    x:  U,
    y:  U,
}

#[repr(C)]
pub struct BaseVector3<U> {
    x:  U,
    y:  U,
    z:  U,
}

#[repr(C)]
pub struct BaseVector4<U> {
    x:  U,
    y:  U,
    z:  U,
    w:  U,
}

#[repr(C)]
pub struct BaseColor3<U> {
    r:  U,
    g:  U,
    b:  U,
}

#[repr(C)]
pub struct BaseColor4<U> {
    r:  U,
    g:  U,
    b:  U,
}

#[repr(C)]
pub struct Color3f {
    base:   BaseColor3<f32>,
}

#[repr(C)]
pub struct Color4f {
    base:   BaseColor4<f32>,
}

#[repr(C)]
pub struct Vector2f {
    base:   BaseVector2<f32>
}

#[repr(C)]
pub struct Vector3f {
    base:   BaseVector3<f32>
}

#[repr(C)]
pub struct Vector4f {
    base: BaseVector4<f32>
}

#[repr(C)]
pub struct Quatf {
    base: BaseVector4<f32>
}

#[repr(C)]
pub struct BaseMtx22<T> {
    v:  [[T; 2]; 2]
}

#[repr(C)]
pub struct BaseMtx33<T> {
    v:  [[T; 3]; 3]
}

#[repr(C)]
pub struct BaseMtx34<T> {
    v:  [[T; 3]; 4]
}

#[repr(C)]
pub struct BaseMtx44<T> {
    v:  [[T; 4]; 4]
}

#[repr(C)]
pub struct Matrix22f {
    base:   BaseMtx22<f32>
}

#[repr(C)]
pub struct Matrix33f {
    base:   BaseMtx33<f32>
}

#[repr(C)]
pub struct Matrix34f {
    base:   BaseMtx34<f32>
}

#[repr(C)]
pub struct Matrix44f {
    base:   BaseMtx44<f32>
}
