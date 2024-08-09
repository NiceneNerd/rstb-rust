use super::{agl, sead, ParamIO, Resource};

#[repr(C)]
pub struct AIProgram<T> {
    base:                   ParamIO<T>,             // ParamIO
    base2:                  Resource<T>,            // Resource
    mHeap:                  T,                      // sead::Heap*
    mStr:                   sead::SafeString<T>,    // sead::SafeString
    mAIs:                   sead::Buffer<T>,        // sead::Buffer<AIActionDef>
    mActions:               sead::Buffer<T>,        // sead::Buffer<AIActionDef>
    mBehaviors:             sead::Buffer<T>,        // sead::Buffer<BehaviorDef>
    mQueries:               sead::Buffer<T>,        // sead::Buffer<QueryDef>
    mParamListAI:           agl::ParameterList<T>,  // agl::utl::ParameterList
    mParamListAction:       agl::ParameterList<T>,  // agl::utl::ParameterList
    mParamListBehavior:     agl::ParameterList<T>,  // agl::utl::ParameterList
    mParamListQuery:        agl::ParameterList<T>,  // agl::utl::ParameterList
    mDemoAIActionIndices:   sead::Buffer<T>,        // sead::Buffer<u16>
    mDemoBehaviorIndices:   sead::Buffer<T>,        // sead::Buffer<u8>
}

#[repr(C)]
pub struct Definition<T> {
    mList:          agl::ParameterList<T>,  // agl::utl::ParameterList
    mClassName:     T,                      // const char*
    mName:          T,                      // const char*
    mSInstParams:   sead::Buffer<T>,        // sead::Buffer<agl::utl::ParameterBase*>
    mSInstObj:      agl::ParameterObj<T>,   // agl::utl::ParameterObj
}

#[repr(C)]
pub struct AIActionDef<T> {
    base: Definition<T>,
    mGroupName:         T,                  // const char*
    mChildIndices:      sead::Buffer<T>,    // sead::Buffer<u16>
    mBehaviorIndices:   sead::Buffer<T>,    //sead::Buffer<u8>
    mTriggerAction:     u16,                // u16
    mDynamicParamChild: u16,                // u16
    _c4:                u16,                // u16
}

#[repr(C)]
pub struct BehaviorDef<T> {
    base:           Definition<T>,
    mCalcTiming:    u16,
    mNoStop:        u16,
}

#[repr(C)]
pub struct QueryDef<T> {
    base:   Definition<T>,
}
