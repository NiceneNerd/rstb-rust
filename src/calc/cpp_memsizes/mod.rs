pub mod baslist;
pub mod bdrop;
pub mod bgparamlist;
pub mod bmodellist;
pub mod bphysics;
pub mod brecipe;
pub mod bshop;
pub mod bxml;
mod cpp_classes;

const fn cpp_align(list: &[u32], alignment: &u32) -> u32 {
    let mut size: u32 = 0;
    let a: u32 = if *alignment > 0 { *alignment } else {
        let mut i: usize = 0;
        let mut tmp: u32 = 0;
        while i < list.len() {
            if tmp < list[i] {
                tmp = list[i];
            }
            i += 1;
        }
        tmp
    };
    let mut temp_sum: u32 = 0;
    let mut i: usize = 0;
    let mut tmp: u32;
    while i < list.len() {
        tmp = list[i];
        if tmp > a {
            if temp_sum > 0 {
                size += 1;
            }
            size += tmp / a;
            temp_sum = tmp % a;
        }
        else if temp_sum + tmp > a {
            size += 1;
            temp_sum = tmp;
        }
        else {
            temp_sum += tmp;
        }
        i += 1;
    }
    if temp_sum > 0 {
        size += 1;
    }
    size * a
}
