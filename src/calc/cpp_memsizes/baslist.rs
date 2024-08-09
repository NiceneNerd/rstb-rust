use std::mem::size_of;

use roead::aamp::ParameterIO;

use super::cpp_classes::ASList::*;
use crate::Endian;

const CLASS_SIZE_WIIU: usize = std::mem::size_of::<ASList<u32>>();
const CLASS_SIZE_NX: usize = std::mem::size_of::<ASList<u64>>();

const OVERHEAD_WIIU: usize = 0x50;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX,
    };

    let a = ParameterIO::from_binary(bytes).ok()?;
    let (asdefine_size, cfdefine_size, cfpost_size, cfexcept_size, addres_size): (
        usize,
        usize,
        usize,
        usize,
        usize,
    );
    match endian {
        Endian::Big => {
            asdefine_size = size_of::<ASDefine<u32>>();
            cfdefine_size = size_of::<CFDefine<u32>>();
            cfpost_size = size_of::<CFPost<u32>>();
            cfexcept_size = size_of::<CFExcept<u32>>();
            addres_size = size_of::<AddRes<u32>>();
        }
        Endian::Little => {
            asdefine_size = size_of::<ASDefine<u64>>();
            cfdefine_size = size_of::<CFDefine<u64>>();
            cfpost_size = size_of::<CFPost<u64>>();
            cfexcept_size = size_of::<CFExcept<u64>>();
            addres_size = size_of::<AddRes<u64>>();
        }
    }

    if let Some(asdefine_list) = a.param_root.lists.get("ASDefines") {
        let num_asdefines = asdefine_list.objects.len();
        if num_asdefines > 0 {
            total_size += num_asdefines * asdefine_size;
            if let Some(cfdefine_list) = a.param_root.lists.get("CFDefines") {
                let num_cfdefines = cfdefine_list.lists.len();
                if num_cfdefines > 0 {
                    total_size += num_cfdefines * cfdefine_size;
                    for i in 0..num_cfdefines {
                        let cfdefine = cfdefine_list.lists.get(format!("CFDefine_{}", i))?;
                        if let Some(cfpost_list) = cfdefine.lists.get("CFPosts") {
                            let num_cfposts = cfpost_list.objects.len();
                            total_size += num_cfposts * cfpost_size;
                        }
                        if let Some(cfexcept_obj) = cfdefine.objects.get("CFExcepts") {
                            let num_cfexcepts = cfexcept_obj.len();
                            total_size += num_cfexcepts * cfexcept_size;
                        }
                    }
                }
            }
        }
    }
    if let Some(addreses_list) = a.param_root.lists.get("AddReses") {
        let num_addreses = addreses_list.objects.len();
        if num_addreses > 0 {
            total_size += num_addreses * addres_size;
        }
    }

    Some(total_size as u32)
}
