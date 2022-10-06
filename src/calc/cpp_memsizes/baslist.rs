use roead::aamp::ParameterIO;
use crate::Endian;

use std::mem::size_of;

use super::cpp_classes::{wiiu, nx};

const BASLIST_OVERHEAD: u32 = 0x80;

pub fn parse_size(bytes: &[u8], endian: Endian) -> u32 {
    match endian {
        Endian::Big => parse_size_wiiu(bytes),
        Endian::Little => parse_size_nx(bytes),
    }
}

fn parse_size_wiiu(b: &[u8]) -> u32 {
    let mut total_size = BASLIST_OVERHEAD;
    let a = ParameterIO::from_binary(b).unwrap();

    if let Some(asdefine_list) = a.param_root.lists.get("ASDefines") {
        let num_asdefines = asdefine_list.objects.len() as u32;
        if num_asdefines > 0 {
            total_size += num_asdefines * size_of::<wiiu::ASList::ASDefine>() as u32;
            if let Some(cfdefine_list) = a.param_root.lists.get("CFDefines") {
                let num_cfdefines = cfdefine_list.lists.len() as u32;
                if num_cfdefines > 0 {
                    total_size += num_cfdefines * size_of::<wiiu::ASList::CFDefine>() as u32;
                    for i in 0..num_cfdefines {
                        let cfdefine = cfdefine_list.lists.get(format!("CFDefine_{}", i)).unwrap();
                        if let Some(cfpost_list) = cfdefine.lists.get("CFPosts") {
                            let num_cfposts = cfpost_list.objects.len() as u32;
                            total_size += num_cfposts * size_of::<wiiu::ASList::CFPost>() as u32;
                        }
                        if let Some(cfexcept_obj) = cfdefine.objects.get("CFExcepts") {
                            let num_cfexcepts = cfexcept_obj.len() as u32;
                            total_size += num_cfexcepts * size_of::<wiiu::ASList::CFExcept>() as u32;
                        }
                    }
                }
            }
        }
    }
    if let Some(addreses_list) = a.param_root.lists.get("AddReses") {
        let num_addreses = addreses_list.objects.len() as u32;
        if num_addreses > 0 {
            total_size += num_addreses * size_of::<wiiu::ASList::AddRes>() as u32;
        }
    }

    total_size
}

fn parse_size_nx(b: &[u8]) -> u32 {
    let mut total_size = BASLIST_OVERHEAD;
    let a = ParameterIO::from_binary(b).unwrap();

    if let Some(asdefine_list) = a.param_root.lists.get("ASDefines") {
        let num_asdefines = asdefine_list.objects.len() as u32;
        if num_asdefines > 0 {
            total_size += num_asdefines * size_of::<nx::ASList::ASDefine>() as u32;
            if let Some(cfdefine_list) = a.param_root.lists.get("CFDefines") {
                let num_cfdefines = cfdefine_list.lists.len() as u32;
                if num_cfdefines > 0 {
                    total_size += num_cfdefines * size_of::<nx::ASList::CFDefine>() as u32;
                    for i in 0..num_cfdefines {
                        let cfdefine = cfdefine_list.lists.get(format!("CFDefine_{}", i)).unwrap();
                        if let Some(cfpost_list) = cfdefine.lists.get("CFPosts") {
                            let num_cfposts = cfpost_list.objects.len() as u32;
                            total_size += num_cfposts * size_of::<nx::ASList::CFPost>() as u32;
                        }
                        if let Some(cfexcept_obj) = cfdefine.objects.get("CFExcepts") {
                            let num_cfexcepts = cfexcept_obj.len() as u32;
                            total_size += num_cfexcepts * size_of::<nx::ASList::CFExcept>() as u32;
                        }
                    }
                }
            }
        }
    }
    if let Some(addreses_list) = a.param_root.lists.get("AddReses") {
        let num_addreses = addreses_list.objects.len() as u32;
        if num_addreses > 0 {
            total_size += num_addreses * size_of::<nx::ASList::AddRes>() as u32;
        }
    }

    total_size
}
