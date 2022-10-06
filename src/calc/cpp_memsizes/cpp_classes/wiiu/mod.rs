pub mod agl;
pub mod ASList;

use super::IParamType;

#[repr(C)]
pub struct SafeString { ptr: u32, v: [u8; 8] }
impl IParamType for SafeString {}

#[repr(C)]
pub struct FixedSafeString32 { ptr: u32, v: [u8; 32] }
impl IParamType for FixedSafeString32 {}

#[repr(C)]
pub struct FixedSafeString64 { ptr: u32, v: [u8; 64] }
impl IParamType for FixedSafeString64 {}

#[repr(C)]
pub struct FixedSafeString256 { ptr: u32, v: [u8; 256] }
impl IParamType for FixedSafeString256 {}

#[repr(C)]
pub struct SeadBuffer {
    mSize: i32, // s32
    mBuffer: u32 // T*
}
