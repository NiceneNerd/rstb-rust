pub mod bdrop;
pub mod bgparamlist;
pub mod bmodellist;
pub mod bphysics;
pub mod brecipe;
pub mod bshop;
pub mod bxml;

struct WiiUParameterList {
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
struct NXParameterList {
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
struct WiiUParameterObj {
    vfptr: u32, // vfptr*
    mParamListHead: u32, // ParameterBase*
    mParamListTail: u32, // ParameterBase*
    mParamListSize: u32, // u32
    mNameHash: u32, // u32
    mNext: u32, // IParameterObj*
    mName: u32 // char*
}
struct NXParameterObj {
    vfptr: u64, // vfptr*
    mParamListHead: u64, // ParameterBase*
    mParamListTail: u64, // ParameterBase*
    mParamListSize: u32, // u32
    mNameHash: u32, // u32
    mNext: u64, // IParameterObj*
    mName: u64 // char*
}
trait IParamType {}
struct WiiUParameterBase {
    vfptr: u32,
    mNameHash: u32,
    mNext: u32,
}
#[repr(C)]
struct WiiUParameter<T: IParamType> {
    base: WiiUParameterBase,
    mValue: T
}
struct NXParameterBase {
    vfptr: u64,
    mNameHash: u64,
    mNext: u64
}
#[repr(C)]
struct NXParameter<T: IParamType> {
    base: NXParameterBase,
    mValue: T
}
struct Bool32 { v: u32 }
impl IParamType for Bool32 {}
struct Int { v: i32 }
impl IParamType for Int {}
struct S32 { v: i32 }
impl IParamType for S32 {}
struct U32 { v: u32 }
impl IParamType for U32 {}
struct Float { v: f32 }
impl IParamType for Float {}
struct F32 { v: f32 }
impl IParamType for F32 {}
struct Vector2f { x: f32, y: f32 }
impl IParamType for Vector2f {}
struct Vector3f { x: f32, y: f32, z: f32 }
impl IParamType for Vector3f {}
struct Vector4f { x: f32, y: f32, z: f32, w: f32 }
impl IParamType for Vector4f {}
struct WiiUSafeString { ptr: u32, v: [u8; 8] }
impl IParamType for WiiUSafeString {}
struct WiiUFixedSafeString32 { ptr: u32, v: [u8; 32] }
impl IParamType for WiiUFixedSafeString32 {}
struct WiiUFixedSafeString64 { ptr: u32, v: [u8; 64] }
impl IParamType for WiiUFixedSafeString64 {}
struct WiiUFixedSafeString256 { ptr: u32, v: [u8; 256] }
impl IParamType for WiiUFixedSafeString256 {}
struct NXSafeString { ptr: u64, v: [u8; 8] }
impl IParamType for NXSafeString {}
struct NXFixedSafeString32 { ptr: u64, v: [u8; 32] }
impl IParamType for NXFixedSafeString32 {}
struct NXFixedSafeString64 { ptr: u64, v: [u8; 64] }
impl IParamType for NXFixedSafeString64 {}
struct NXFixedSafeString256 { ptr: u64, v: [u8; 256] }
impl IParamType for NXFixedSafeString256 {}
struct WiiUSeadBuffer {
    mSize: i32, // s32
    mBuffer: u32 // T*
}
struct NXSeadBuffer {
    mSize: i32, // s32
    mBuffer: u64 // T*
}

const fn cpp_align(list: &[u32], alignment: &u32) -> u32 {
    let mut size: u32 = 0;
    let a: u32 = if *alignment > 0 { *alignment } else {
        let mut i: usize = 0;
        let mut tmp: u32 = 0;
        while i < list.len() {
            if tmp < list[i] {
                tmp = list[i];
            }
            i += 1;
        }
        tmp
    };
    let mut temp_sum: u32 = 0;
    let mut i: usize = 0;
    let mut tmp: u32;
    while i < list.len() {
        tmp = list[i];
        if tmp > a {
            if temp_sum > 0 {
                size += 1;
            }
            size += tmp / a;
            temp_sum = tmp % a;
        }
        else if temp_sum + tmp > a {
            size += 1;
            temp_sum = tmp;
        }
        else {
            temp_sum += tmp;
        }
        i += 1;
    }
    if temp_sum > 0 {
        size += 1;
    }
    size * a
}
