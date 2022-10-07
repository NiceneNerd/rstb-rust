#![allow(dead_code)]
#![allow(non_snake_case)]

pub mod agl;
pub mod AIProgram;
pub mod ASList;
pub mod DropTable;
pub mod GParamList;
pub mod ModelList;
pub mod Physics;
pub mod Recipe;
pub mod ShopData;

pub struct Bool32 { v: u32 }
pub struct Int { v: i32 }
pub struct S32 { v: i32 }
pub struct U32 { v: u32 }
pub struct Float { v: f32 }
pub struct F32 { v: f32 }
pub struct Vector2f { x: f32, y: f32 }
pub struct Vector3f { x: f32, y: f32, z: f32 }
pub struct Vector4f { x: f32, y: f32, z: f32, w: f32 }

#[repr(C)]
pub struct SafeString<T> { ptr: T, v: [u8; 8] }

#[repr(C)]
pub struct FixedSafeString32<T> { ptr: T, v: [u8; 32] }

#[repr(C)]
pub struct FixedSafeString64<T> { ptr: T, v: [u8; 64] }

#[repr(C)]
pub struct FixedSafeString256<T> { ptr: T, v: [u8; 256] }

#[repr(C)]
pub struct SeadBuffer<T> {
    mSize: i32, // s32
    mBuffer: T // T*
}
