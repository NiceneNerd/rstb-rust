use roead::aamp::ParameterIO;
use crate::Endian;

use std::mem::size_of;

use super::cpp_classes::*;

const BASLIST_OVERHEAD: u32 = 0x80;

pub fn parse_size(bytes: &[u8], endian: Endian) -> u32 {
    let mut total_size = BASLIST_OVERHEAD;
    let a = ParameterIO::from_binary(bytes).unwrap();
    let (asdefine_size, cfdefine_size, cfpost_size, cfexcept_size, addres_size): (u32, u32, u32, u32, u32);
    match endian {
        Endian::Big => {
            asdefine_size = size_of::<ASList::ASDefine<u32>>() as u32;
            cfdefine_size = size_of::<ASList::CFDefine<u32>>() as u32;
            cfpost_size = size_of::<ASList::CFPost<u32>>() as u32;
            cfexcept_size = size_of::<ASList::CFExcept<u32>>() as u32;
            addres_size = size_of::<ASList::AddRes<u32>>() as u32;
        },
        Endian::Little => {
            asdefine_size = size_of::<ASList::ASDefine<u64>>() as u32;
            cfdefine_size = size_of::<ASList::CFDefine<u64>>() as u32;
            cfpost_size = size_of::<ASList::CFPost<u64>>() as u32;
            cfexcept_size = size_of::<ASList::CFExcept<u64>>() as u32;
            addres_size = size_of::<ASList::AddRes<u64>>() as u32;
        },
    }

    if let Some(asdefine_list) = a.param_root.lists.get("ASDefines") {
        let num_asdefines = asdefine_list.objects.len() as u32;
        if num_asdefines > 0 {
            total_size += num_asdefines * asdefine_size;
            if let Some(cfdefine_list) = a.param_root.lists.get("CFDefines") {
                let num_cfdefines = cfdefine_list.lists.len() as u32;
                if num_cfdefines > 0 {
                    total_size += num_cfdefines * cfdefine_size;
                    for i in 0..num_cfdefines {
                        let cfdefine = cfdefine_list.lists.get(format!("CFDefine_{}", i)).unwrap();
                        if let Some(cfpost_list) = cfdefine.lists.get("CFPosts") {
                            let num_cfposts = cfpost_list.objects.len() as u32;
                            total_size += num_cfposts * cfpost_size;
                        }
                        if let Some(cfexcept_obj) = cfdefine.objects.get("CFExcepts") {
                            let num_cfexcepts = cfexcept_obj.len() as u32;
                            total_size += num_cfexcepts * cfexcept_size;
                        }
                    }
                }
            }
        }
    }
    if let Some(addreses_list) = a.param_root.lists.get("AddReses") {
        let num_addreses = addreses_list.objects.len() as u32;
        if num_addreses > 0 {
            total_size += num_addreses * addres_size;
        }
    }

    total_size
}
