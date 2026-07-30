use crate::Endian;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParseSize {
    Simple(u32),
    Complex,
}

pub(crate) fn get_factory_info<S: AsRef<str>>(ext: S, endian: Endian) -> (u32, isize, ParseSize) {
    match ext.as_ref() {
        "sarc" | "pack" | "bactorpack" | "bmodelsh" | "beventpack" | "stera" | "stats" => {
            match endian {
                Endian::Little => (0x68, 0x80, ParseSize::Simple(0)),
                Endian::Big => (0x3c, 0x80, ParseSize::Simple(0)),
            }
        }
        "Tex.bfres" | "Tex1.bfres" | "Tex2.bfres" => {
            match endian {
                Endian::Little => (0x38, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x20, 0x4, ParseSize::Simple(0)),
            }
        }
        "bfres" => {
            match endian {
                Endian::Little => (0x1a8, 0x1000, ParseSize::Complex),
                Endian::Big => (0x13c, 0x1000, ParseSize::Complex),
            }
        }
        "bcamanim" => {
            match endian {
                Endian::Little => (0x50, 0x2000, ParseSize::Complex),
                Endian::Big => (0x2c, 0x2000, ParseSize::Complex),
            }
        }
        "batpl" | "bnfprl" => {
            match endian {
                Endian::Little => (0x40, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x24, 0x4, ParseSize::Simple(0)),
            }
        }
        "bplacement" => {
            match endian {
                Endian::Little => (0x48, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0)),
            }
        }
        "hks" | "lua" => {
            match endian {
                Endian::Little => (0x38, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0)),
            }
        }
        "bactcapt" => {
            match endian {
                Endian::Little => (0x538, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x3b4, 0x4, ParseSize::Simple(0)),
            }
        }
        "bitemico" => {
            match endian {
                Endian::Little => (0x60, 0x2000, ParseSize::Simple(0)),
                Endian::Big => (0xd0, 0x2000, ParseSize::Simple(0)),
            }
        }
        "jpg" => {
            match endian {
                Endian::Little => (0x80, 0x2000, ParseSize::Simple(0)),
                Endian::Big => (0x174, 0x2000, ParseSize::Simple(0)),
            }
        }
        "bmaptex" => {
            match endian {
                Endian::Little => (0x60, 0x2000, ParseSize::Simple(0)),
                Endian::Big => (0xd0, 0x2000, ParseSize::Simple(0)),
            }
        }
        "bstftex" | "bmapopen" | "breviewtex" => {
            match endian {
                Endian::Little => (0x60, 0x2000, ParseSize::Simple(0)),
                Endian::Big => (0xd0, 0x2000, ParseSize::Simple(0)),
            }
        }
        "bgdata" => {
            match endian {
                Endian::Little => (0x140, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0xcc, 0x4, ParseSize::Simple(0)),
            }
        }
        "bgsvdata" => {
            match endian {
                Endian::Little => (0x38, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0)),
            }
        }
        "hknm2" => {
            match endian {
                Endian::Little => (0x48, 0x8, ParseSize::Complex),
                Endian::Big => (0x28, 0x4, ParseSize::Complex),
            }
        }
        "bmscdef" => {
            match endian {
                Endian::Little => (0x2a8, 0x8, ParseSize::Complex),
                Endian::Big => (0x1fc, 0x4, ParseSize::Complex),
            }
        }
        "bars" => {
            match endian {
                Endian::Little => (0xb0, 0x80, ParseSize::Complex),
                Endian::Big => (0x84, 0x80, ParseSize::Complex),
            }
        }
        "bxml" => {
            match endian {
                Endian::Little => (0x778, 0x8, ParseSize::Complex),
                Endian::Big => (0x4a8, 0x4, ParseSize::Complex),
            }
        }
        "bgparamlist" => {
            match endian {
                Endian::Little => (0x2c0, 0x8, ParseSize::Complex),
                Endian::Big => (0x248, 0x4, ParseSize::Complex),
            }
        }
        "bmodellist" => {
            match endian {
                Endian::Little => (0x7d0, 0x8, ParseSize::Complex),
                Endian::Big => (0x508, 0x4, ParseSize::Complex),
            }
        }
        "baslist" => {
            match endian {
                Endian::Little => (0x410, 0x8, ParseSize::Complex),
                Endian::Big => (0x2f4, 0x4, ParseSize::Complex),
            }
        }
        "baiprog" => {
            match endian {
                Endian::Little => (0x448, 0x8, ParseSize::Complex),
                Endian::Big => (0x30c, 0x4, ParseSize::Complex),
            }
        }
        "bphysics" => {
            match endian {
                Endian::Little => (0x470, 0x8, ParseSize::Complex),
                Endian::Big => (0x324, 0x4, ParseSize::Complex),
            }
        }
        "bchemical" => {
            match endian {
                Endian::Little => (0x3c0, 0x8, ParseSize::Complex),
                Endian::Big => (0x2cc, 0x4, ParseSize::Complex),
            }
        }
        "bas" => {
            match endian {
                Endian::Little => (0x3c8, 0x8, ParseSize::Complex),
                Endian::Big => (0x2d0, 0x4, ParseSize::Complex),
            }
        }
        "batcllist" => {
            match endian {
                Endian::Little => (0x3f0, 0x8, ParseSize::Complex),
                Endian::Big => (0x2e4, 0x4, ParseSize::Complex),
            }
        }
        "batcl" => {
            match endian {
                Endian::Little => (0x428, 0x8, ParseSize::Complex),
                Endian::Big => (0x344, 0x4, ParseSize::Complex),
            }
        }
        "baischedule" => {
            match endian {
                Endian::Little => (0x2b8, 0x8, ParseSize::Simple(0x20)),
                Endian::Big => (0x244, 0x4, ParseSize::Simple(0x18)),
            }
        }
        "bdmgparam" => {
            match endian {
                Endian::Little => (0x11d0, 0x8, ParseSize::Simple(0x7f8)),
                Endian::Big => (0x9f0, 0x4, ParseSize::Simple(0x40c)),
            }
        }
        "brgconfiglist" => {
            match endian {
                Endian::Little => (0x3d0, 0x8, ParseSize::Complex),
                Endian::Big => (0x2d4, 0x4, ParseSize::Complex),
            }
        }
        "brgconfig" => {
            match endian {
                Endian::Little => (0x42d8, 0x8, ParseSize::Simple(0x20)),
                Endian::Big => (0x2acc, 0x4, ParseSize::Simple(0x10)),
            }
        }
        "brgbw" => {
            match endian {
                Endian::Little => (0x2c0, 0x8, ParseSize::Complex),
                Endian::Big => (0x248, 0x4, ParseSize::Complex),
            }
        }
        "bawareness" => {
            match endian {
                Endian::Little => (0xb38, 0x8, ParseSize::Simple(0x20)),
                Endian::Big => (0x70c, 0x4, ParseSize::Simple(0x10)),
            }
        }
        "bdrop" => {
            match endian {
                Endian::Little => (0x320, 0x8, ParseSize::Complex),
                Endian::Big => (0x27c, 0x4, ParseSize::Complex),
            }
        }
        "bshop" => {
            match endian {
                Endian::Little => (0x320, 0x8, ParseSize::Complex),
                Endian::Big => (0x27c, 0x4, ParseSize::Complex),
            }
        }
        "brecipe" => {
            match endian {
                Endian::Little => (0x320, 0x8, ParseSize::Complex),
                Endian::Big => (0x27c, 0x4, ParseSize::Complex),
            }
        }
        "blod" => {
            match endian {
                Endian::Little => (0x3c0, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x2cc, 0x4, ParseSize::Simple(0x10)),
            }
        }
        "bbonectrl" => {
            match endian {
                Endian::Little => (0x8d0, 0x8, ParseSize::Complex),
                Endian::Big => (0x564, 0x4, ParseSize::Complex),
            }
        }
        "blifecondition" => {
            match endian {
                Endian::Little => (0x4b0, 0x8, ParseSize::Complex),
                Endian::Big => (0x35c, 0x4, ParseSize::Complex),
            }
        }
        "bumii" => {
            match endian {
                Endian::Little => (0x2b8, 0x8, ParseSize::Simple(0x20)),
                Endian::Big => (0x244, 0x4, ParseSize::Simple(0x18)),
            }
        }
        "baniminfo" => {
            match endian {
                Endian::Little => (0x2c8, 0x8, ParseSize::Complex),
                Endian::Big => (0x24c, 0x4, ParseSize::Complex),
            }
        }
        "byaml" => {
            match endian {
                Endian::Little => (0x20, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0)),
            }
        }
        "byml" => {
            match endian {
                Endian::Little => (0x38, 0x8, ParseSize::Simple(0x28)),
                Endian::Big => (0x20, 0x4, ParseSize::Simple(0x3c)),
            }
        }
        "bassetting" => {
            match endian {
                Endian::Little => (0x260, 0x8, ParseSize::Complex),
                Endian::Big => (0x1d8, 0x4, ParseSize::Complex),
            }
        }
        "hkrb" => {
            match endian {
                Endian::Little => (0x20, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0x8)),
            }
        }
        "hkrg" => {
            match endian {
                Endian::Little => (0x20, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0x8)),
            }
        }
        "bphyssb" => {
            match endian {
                Endian::Little => (0x5b0, 0x8, ParseSize::Complex),
                Endian::Big => (0x384, 0x4, ParseSize::Complex),
            }
        }
        "hkcl" => {
            match endian {
                Endian::Little => (0xe8, 0x8, ParseSize::Complex),
                Endian::Big => (0xb8, 0x4, ParseSize::Complex),
            }
        }
        "hksc" => {
            match endian {
                Endian::Little => (0x140, 0x8, ParseSize::Complex),
                Endian::Big => (0xe8, 0x4, ParseSize::Complex),
            }
        }
        "hktmrb" => {
            match endian {
                Endian::Little => (0x48, 0x8, ParseSize::Complex),
                Endian::Big => (0x28, 0x4, ParseSize::Complex),
            }
        }
        "brgcon" => {
            match endian {
                Endian::Little => (0x48, 0x8, ParseSize::Complex),
                Endian::Big => (0x28, 0x4, ParseSize::Complex),
            }
        }
        "esetlist" => {
            match endian {
                Endian::Little => (0x38, 0x4000, ParseSize::Simple(0)),
                Endian::Big => (0x20, 0x4000, ParseSize::Simple(0)),
            }
        }
        "bdemo" => {
            match endian {
                Endian::Little => (0xb20, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x6cc, 0x4, ParseSize::Simple(0x10)),
            }
        }
        "bfevfl" => {
            match endian {
                Endian::Little => (0x40, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x24, 0x4, ParseSize::Simple(0x38)),
            }
        }
        "bfevtm" => {
            match endian {
                Endian::Little => (0x40, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x24, 0x4, ParseSize::Simple(0x38)),
            }
        }
        _ => {
            match endian {
                Endian::Little => (0x38, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x20, 0x4, ParseSize::Simple(0)),
            }
        }
    }
}
