use super::{agl, sead, ParamIO, Resource};

#[repr(C)]
pub struct Chemical<T> {
    base:   ParamIO<T>,     // ParamIO
    base2:  Resource<T>,    // Resource
    mRoot:  Root<T>,        // chm::Root
}

#[repr(C)]
pub struct Root<T> {
    base:               agl::ParameterList<T>,  // agl::utl::ParameterList
    base2:              sead::Node<T>,          // sead::hostio::Node
    base3:              IRoot<T>,               // IRoot
    chemical_header:    agl::ParameterObj<T>,   // agl::utl::ParameterObj
    res_shape_num:      agl::Parameter<T, u32>, // agl::utl::Parameter<u32>
    chemical_body:      agl::ParameterList<T>,  // agl::utl::ParameterList
    shapes:             sead::Buffer<T>,        // sead::Buffer<Shape>
    rigids:             sead::Buffer<T>,        // sead::Buffer<Rigids>
}

#[repr(C)]
pub struct IRoot<T> {
    vfptr:  T,  // vtable*
}

#[repr(C)]
pub struct Shape<T> {
    base:               agl::IParameterObj<T>,                              // agl::utl::IParameterObj
    base2:              agl::IParameterIO<T>,                               // agl::utl::IParameterIO
    base3:              sead::Node<T>,                                      // sead::hostio::Node
    base4:              IShape<T>,                                          // IShape
    name:               agl::Parameter<T, sead::FixedSafeString<T, 32>>,    // agl::utl::Parameter<sead::FixedSafeString<32>>
    res_type_id:        agl::Parameter<T, sead::FixedSafeString<T, 32>>,    // agl::utl::Parameter<sead::FixedSafeString<32>>
    volume_occupancy:   agl::Parameter<T, f32>,                             // agl::utl::Parameter<float>
    element_occlusion:  agl::Parameter<T, f32>,                             // agl::utl::Parameter<float>
}

#[repr(C)]
struct IShape<T> {
    vfptr:  T,  // vtable*
}

#[repr(C)]
pub struct Rigid<T> {
    base:           agl::IParameterObj<T>,                              // agl::utl::IParameterObj
    attribute:      agl::Parameter<T, u32>,                             // agl::utl::Parameter<u32>
    rigid_set_name: agl::Parameter<T, sead::FixedSafeString<T, 64>>,    // agl::utl::Parameter<sead::FixedSafeString<64>>
    rigid_name:     agl::Parameter<T, sead::FixedSafeString<T, 64>>,    // agl::utl::Parameter<sead::FixedSafeString<64>>
    volume:         agl::Parameter<T, f32>,                             // agl::utl::Parameter<float>
    mass:           agl::Parameter<T, f32>,                             // agl::utl::Parameter<float>
    burn_out_time:  agl::Parameter<T, f32>,                             // agl::utl::Parameter<float>
}
