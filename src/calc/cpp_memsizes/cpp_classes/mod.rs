pub mod nx;
pub mod wiiu;

pub trait IParamType {}
pub struct Bool32 { v: u32 }
impl IParamType for Bool32 {}
pub struct Int { v: i32 }
impl IParamType for Int {}
pub struct S32 { v: i32 }
impl IParamType for S32 {}
pub struct U32 { v: u32 }
impl IParamType for U32 {}
pub struct Float { v: f32 }
impl IParamType for Float {}
pub struct F32 { v: f32 }
impl IParamType for F32 {}
pub struct Vector2f { x: f32, y: f32 }
impl IParamType for Vector2f {}
pub struct Vector3f { x: f32, y: f32, z: f32 }
impl IParamType for Vector3f {}
pub struct Vector4f { x: f32, y: f32, z: f32, w: f32 }
impl IParamType for Vector4f {}