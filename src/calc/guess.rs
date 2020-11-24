#![allow(clippy::unreadable_literal)]
use crate::{AnyError, Endian};
use std::fs::read;
use std::io::Cursor;
use std::path::Path;
use yaz0::Yaz0Header;

/// Estimates the resource size for complex resource types, excluding BFRES. Returns `None` if no
/// estimate is available.
pub fn guess_size(filesize: usize, endian: Endian, ext: &str) -> Option<u32> {
    let actual_ext = match ext.strip_prefix(".s") {
        Some(s) => s,
        None => match ext.strip_prefix('.') {
            Some(s) => s,
            None => ext,
        },
    };
    let size = match actual_ext {
        "baiprog" => guess_baiprog(filesize),
        "bgparamlist" => guess_bgparamlist(filesize),
        "bdrop" => guess_bdrop(filesize),
        "bxml" => guess_bxml(filesize),
        "brecipe" => guess_brecipe(filesize),
        "bshop" => guess_bshop(filesize),
        "bas" => guess_bas(filesize),
        "baslist" => guess_baslist(filesize),
        "bphysics" => guess_bphysics(filesize),
        _ => return None,
    };
    match endian {
        Endian::Big => Some(size as u32),
        Endian::Little => Some((size * 1.5) as u32),
    }
}

/// Estimates the resource size for BFRES files on the file system. Returns a `Result` with the
/// estimated resource size or an IO/file system error.
pub fn guess_bfres_value_from_file<P: AsRef<Path>>(
    file: P,
    endian: Endian,
) -> Result<u32, AnyError> {
    let data = read(&file)?;
    let filesize = match &data[0..4] {
        b"Yaz0" => {
            let mut view: Cursor<&[u8]> = Cursor::new(&data);
            {
                let header = Yaz0Header::parse(&mut view);
                header.unwrap().expected_size
            }
        }
        _ => data.len(),
    };
    drop(data);
    Ok(guess_bfres_value_from_size(
        filesize,
        endian,
        file.as_ref()
            .file_name()
            .ok_or("Problem parsing file name")?
            .to_str()
            .ok_or("Problem parsing file name")?,
    ))
}

/// Estimates the resource size for BFRES files from the uncompressed file size. Unlike other RSTB
/// calculators, this will always return a real value.
pub fn guess_bfres_value_from_size(filesize: usize, endian: Endian, name: &str) -> u32 {
    (match endian {
        Endian::Big => {
            if name.contains(".Tex") {
                guess_tex_u(filesize)
            } else {
                guess_model_u(filesize)
            }
        }
        Endian::Little => {
            if name.contains(".Tex") {
                guess_tex_nx(filesize)
            } else {
                guess_model_nx(filesize)
            }
        }
    }) as u32
}

fn guess_baiprog(filesize: usize) -> f32 {
    let mut value = filesize as f32;
    if filesize > 1750 {
        value *= 3.0;
    } else if filesize > 1000 {
        value *= 3.5
    } else if filesize > 600 {
        value *= 4.0
    } else if filesize > 450 {
        value *= 5.0
    } else if filesize > 400 {
        value *= 5.5
    } else if filesize > 380 {
        value *= 6.0
    } else {
        value *= 7.0
    }
    value
}

fn guess_bgparamlist(filesize: usize) -> f32 {
    let mut value = filesize as f32;
    if filesize > 450 {
        value *= 6.0
    } else if filesize > 350 {
        value *= 7.0
    } else if filesize > 250 {
        value *= 8.0
    } else if filesize > 150 {
        value *= 10.0
    } else if filesize > 100 {
        value *= 12.0
    } else {
        value *= 20.0
    }
    value
}

fn guess_bdrop(filesize: usize) -> f32 {
    let mut value = filesize as f32;
    if filesize > 850 {
        value *= 4.0
    } else if filesize > 450 {
        value *= 4.5
    } else if filesize > 350 {
        value *= 5.25
    } else if filesize > 250 {
        value *= 6.0
    } else if filesize > 200 {
        value *= 7.0
    } else {
        value *= 8.5
    }
    value
}

fn guess_bxml(filesize: usize) -> f32 {
    let mut value = filesize as f32;
    if filesize > 800 {
        value *= 3.0
    } else if filesize > 650 {
        value *= 3.5
    } else if filesize > 550 {
        value *= 4.0
    } else if filesize > 450 {
        value *= 4.5
    } else if filesize > 350 {
        value *= 5.0
    } else {
        value *= 6.0
    }
    value
}

fn guess_brecipe(filesize: usize) -> f32 {
    let mut value = filesize as f32;
    if filesize > 215 {
        value *= 6.5
    } else if filesize > 200 {
        value *= 7.0
    } else if filesize > 160 {
        value *= 7.5
    } else if filesize > 100 {
        value *= 8.5
    } else {
        value *= 12.5
    }
    value
}

fn guess_bshop(filesize: usize) -> f32 {
    let mut value = filesize as f32;
    if filesize > 500 {
        value *= 4.05
    } else if filesize > 400 {
        value *= 5.0
    } else if filesize > 200 {
        value *= 6.0
    } else {
        value *= 7.25
    }
    value
}

fn guess_bas(filesize: usize) -> f32 {
    let mut value = filesize as f32;
    if filesize > 15000 {
        value *= 4.5
    } else if filesize > 2000 {
        value *= 5.0
    } else if filesize > 1500 {
        value *= 5.5
    } else if filesize > 600 {
        value *= 6.0
    } else if filesize > 300 {
        value *= 8.0
    } else if filesize > 200 {
        value *= 10.0
    } else if filesize > 100 {
        value *= 12.5
    } else {
        value *= 20.0
    }
    value
}

fn guess_baslist(filesize: usize) -> f32 {
    let mut value = filesize as f32;
    if filesize > 4000 {
        value *= 3.5
    } else if filesize > 800 {
        value *= 4.0
    } else if filesize > 500 {
        value *= 5.0
    } else if filesize > 300 {
        value *= 6.0
    } else if filesize > 200 {
        value *= 8.0
    } else if filesize > 100 {
        value *= 10.0
    } else {
        value *= 15.0
    }
    value
}

fn guess_bphysics(filesize: usize) -> f32 {
    ((((filesize + 32) as isize & -32) + 0x4e + 0x324)
        * std::cmp::max(4 * (filesize as isize / 1388), 3)) as f32
}

fn guess_model_u(filesize: usize) -> f32 {
    let mut value = filesize as f32;
    if filesize > 3000000 {
        value *= 1.45
    } else if filesize > 1500000 {
        value *= 1.66
    } else if filesize > 1000000 {
        value *= 1.85
    } else if filesize > 600000 {
        value *= 1.95
    } else if filesize > 400000 {
        value *= 2.1
    } else if filesize > 2000 {
        value *= 2.25
    } else if filesize > 1250 {
        value *= 3.5
    } else if filesize > 750 {
        value *= 4.0
    } else if filesize > 500 {
        value *= 5.0
    } else {
        value *= 7.0
    }
    value
}

fn guess_model_nx(filesize: usize) -> f32 {
    let mut value = filesize as f32;
    if filesize > 4000000 {
        value *= 1.5
    } else if filesize > 3000000 {
        value *= 1.667
    } else if filesize > 2000000 {
        value *= 2.5
    } else if filesize > 800000 {
        value *= 3.15
    } else if filesize > 100000 {
        value *= 3.5
    } else if filesize > 50000 {
        value *= 3.66
    } else if filesize > 2500 {
        value *= 4.25
    } else if filesize > 1250 {
        value *= 6.0
    } else {
        value *= 9.5
    }
    value
}

fn guess_tex_u(filesize: usize) -> f32 {
    let mut value = filesize as f32;
    if filesize > 6000000 {
        value *= 1.0015
    } else if filesize > 4500000 {
        value *= 1.002
    } else if filesize > 1800000 {
        value *= 1.005
    } else if filesize > 1000000 {
        value *= 1.009
    } else if filesize > 600000 {
        value *= 1.015
    } else if filesize > 300000 {
        value *= 1.03
    } else if filesize > 250000 {
        value *= 1.035
    } else if filesize > 200000 {
        value *= 1.045
    } else if filesize > 150000 {
        value *= 1.07
    } else if filesize > 100000 {
        value *= 1.1
    } else if filesize > 45000 {
        value *= 1.2
    } else if filesize > 30000 {
        value *= 1.3
    } else if filesize > 17000 {
        value *= 1.5
    } else if filesize > 12000 {
        value *= 1.75
    } else if filesize > 8500 {
        value *= 2.0
    } else if filesize > 4000 {
        value *= 3.0
    } else if filesize > 3000 {
        value *= 4.0
    } else if filesize > 2000 {
        value *= 5.0
    } else if filesize > 100 {
        value *= 7.0
    } else {
        value *= 9.0
    }
    value
}

fn guess_tex_nx(filesize: usize) -> f32 {
    let mut value = filesize as f32;
    if filesize > 50000 {
        value *= 1.2
    } else if filesize > 30000 {
        value *= 1.3
    } else if filesize > 10000 {
        value *= 1.5
    } else {
        value *= 2.0
    }
    value
}
