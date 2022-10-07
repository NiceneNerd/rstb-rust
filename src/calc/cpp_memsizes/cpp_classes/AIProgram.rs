use super::SeadBuffer;
use super::agl::*;

#[repr(C)]
struct Definition<T> {
    mList: ParameterList<T>,
    mClassName: T, // const char*
    mName: T, // const char*
    mSInstParams: SeadBuffer<T>, // sead::Buffer<agl::utl::ParameterBase*>
    mSInstObj: ParameterObj<T>,
}

#[repr(C)]
pub struct AIActionDef<T> {
    base: Definition<T>,
    mGroupName: T, // const char*
    mChildIndices: SeadBuffer<T>, // sead::Buffer<u16>
    mBehaviorIndices: SeadBuffer<T>, //sead::Buffer<u8>
    mTriggerAction: u16, // u16
    mDynamicParamChild: u16, // u16
    _c4: u16, // u16
}

#[repr(C)]
pub struct BehaviorDef<T> {
    base: Definition<T>,
    mCalcTiming: u16,
    mNoStop: u16,
}

#[repr(C)]
pub struct QueryDef<T> {
    base: Definition<T>,
}