use std::mem::size_of;

use roead::byml::Byml;

use super::cpp_classes::{sead, AnimationInfo::*};
use crate::Endian;

const CLASS_SIZE_WIIU: usize = size_of::<AnimInfo<u32>>();
const CLASS_SIZE_NX: usize = size_of::<AnimInfo<u64>>();

const OVERHEAD_WIIU: usize = 0xAC;
const OVERHEAD_NX: usize = 0xD0;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX + OVERHEAD_NX,
    };

    let (
        iter_size,
        size_t,
        anim_size,
        sword_blur_size,
        sword_blur_info_size,
    );
    match endian {
        Endian::Big => {
            iter_size = super::ITER_CONST_WIIU;
            size_t = 0; // size_of::<u32>(); // Why does the WiiU version not care about non-trivially-destructible types?
            anim_size = size_of::<Anim<u32>>();
            sword_blur_size = size_of::<SwordBlur<u32>>();
            sword_blur_info_size = size_of::<SwordBlurInfo<u32>>();
        }
        Endian::Little => {
            iter_size = super::ITER_CONST_NX;
            size_t = size_of::<u64>();
            anim_size = size_of::<Anim<u64>>();
            sword_blur_size = size_of::<SwordBlur<u64>>();
            sword_blur_info_size = size_of::<SwordBlurInfo<u64>>();
        }
    }

    let b = Byml::from_binary(bytes).ok()?;
    let num_entries = b.as_map().ok()?.len();
    let mut num_anims = num_entries - 1;
    if num_entries < 1 {
        return Some(total_size as u32);
    }

    //let mut double_attack_anm_num: i32 = 0;
    let num: i32;
    let mut allocate_anims: bool = false;
    if let Some(sword_blur) = b.as_map().ok()?.get("_sword_blur") {
        let num_sword_blurs = sword_blur.as_map().ok()?.len() as i32;
        //if let Some(dbl_atk_anm_num) = sword_blur.as_map().ok()?.get("double_attack_anm_num") {
        //    double_attack_anm_num = dbl_atk_anm_num.as_i32().ok()?;
        //}
        num = num_sword_blurs - 1;
        if num_entries > 1 {
            allocate_anims = true;
        }
    } else {
        num = 0;
        num_anims = num_entries;
        allocate_anims = true;
    }

    if allocate_anims {
        total_size += size_t + num_anims * anim_size;
    }

    if num > 0 {
        total_size += sword_blur_info_size;

        // The following is not a port of the decomp. The decomp hasn't finished this parse
        // function. This is a guess, and may be inaccurate.

        // we had to check this to set num > 0, so this is fine
        let sword_blur = unsafe {
            b.as_map()
                .unwrap_unchecked()
                .get("_sword_blur")
                .unwrap_unchecked()
                .as_map()
                .unwrap_unchecked()
        };
        total_size += iter_size + sword_blur.iter()
            .filter(|(k, _)| *k != "double_attack_anm_num")
            .map(|(_, v)| {
                let num_iter: usize = if let Ok(_) = v.as_array() {
                    2
                } else if let Ok(_) = v.as_map() {
                    1                    
                } else {
                    0
                };
                (iter_size * num_iter * 2) + size_t + sword_blur_size +
                    (get_num_matrices(v) * size_of::<sead::Matrix34f>() * 2)
            })
            .sum::<usize>()
        
        // TODO: use double_attack_num? get_num_matrices might obviate the need for it
    }

    Some(total_size as u32)
}

fn get_num_matrices(byml: &Byml) -> usize {
    if let Ok(map) = byml.as_map() {
        map.get("mtx")
            .unwrap()
            .as_array()
            .unwrap()
            .len()
    } else if let Ok(array) = byml.as_array() {
        array.iter()
            .map(|b| { b.as_map()
                .unwrap()
                .get("mtx")
                .unwrap()
                .as_array()
                .unwrap()
                .len()
            })
            .sum::<usize>()
    } else {
        0
    }
}
