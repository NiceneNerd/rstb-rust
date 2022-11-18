use crate::Endian;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParseSize {
    Simple(u32),
    Complex,
}

pub(crate) fn get_factory_info<S: AsRef<str>>(ext: S, endian: Endian) -> (u32, ParseSize) {
    match ext.as_ref() {
        "sarc" | "pack" | "bactorpack" | "bmodelsh" | "beventpack" | "stera" | "stats" => {
            match endian {
                Endian::Little => (0x68, ParseSize::Simple(0)),
                Endian::Big => (0x3c, ParseSize::Simple(0)),
            }
        }
        "Tex.bfres" | "Tex1.bfres" | "Tex2.bfres" => {
            match endian {
                Endian::Little => (0x38, ParseSize::Simple(0)),
                Endian::Big => (0x20, ParseSize::Simple(0)),
            }
        }
        "bfres" => {
            match endian {
                Endian::Little => (0x1a8, ParseSize::Complex),
                Endian::Big => (0x13c, ParseSize::Complex),
            }
        }
        "bcamanim" => {
            match endian {
                Endian::Little => (0x50, ParseSize::Complex),
                Endian::Big => (0x2c, ParseSize::Complex),
            }
        }
        "batpl" | "bnfprl" => {
            match endian {
                Endian::Little => (0x40, ParseSize::Simple(0)),
                Endian::Big => (0x24, ParseSize::Simple(0)),
            }
        }
        "bplacement" => {
            match endian {
                Endian::Little => (0x48, ParseSize::Simple(0)),
                Endian::Big => (0x14, ParseSize::Simple(0)),
            }
        }
        "hks" | "lua" => {
            match endian {
                Endian::Little => (0x38, ParseSize::Simple(0)),
                Endian::Big => (0x14, ParseSize::Simple(0)),
            }
        }
        "bactcapt" => {
            match endian {
                Endian::Little => (0x538, ParseSize::Simple(0)),
                Endian::Big => (0x3b4, ParseSize::Simple(0)),
            }
        }
        "bitemico" => {
            match endian {
                Endian::Little => (0x60, ParseSize::Simple(0)),
                Endian::Big => (0xd0, ParseSize::Simple(0)),
            }
        }
        "jpg" => {
            match endian {
                Endian::Little => (0x80, ParseSize::Simple(0)),
                Endian::Big => (0x174, ParseSize::Simple(0)),
            }
        }
        "bmaptex" => {
            match endian {
                Endian::Little => (0x60, ParseSize::Simple(0)),
                Endian::Big => (0xd0, ParseSize::Simple(0)),
            }
        }
        "bstftex" | "bmapopen" | "breviewtex" => {
            match endian {
                Endian::Little => (0x60, ParseSize::Simple(0)),
                Endian::Big => (0xd0, ParseSize::Simple(0)),
            }
        }
        "bgdata" => {
            match endian {
                Endian::Little => (0x140, ParseSize::Simple(0)),
                Endian::Big => (0xcc, ParseSize::Simple(0)),
            }
        }
        "bgsvdata" => {
            match endian {
                Endian::Little => (0x38, ParseSize::Simple(0)),
                Endian::Big => (0x14, ParseSize::Simple(0)),
            }
        }
        "hknm2" => {
            match endian {
                Endian::Little => (0x48, ParseSize::Complex),
                Endian::Big => (0x28, ParseSize::Complex),
            }
        }
        "bmscdef" => {
            match endian {
                Endian::Little => (0x2a8, ParseSize::Complex),
                Endian::Big => (0x1fc, ParseSize::Complex),
            }
        }
        "bars" => {
            match endian {
                Endian::Little => (0xb0, ParseSize::Complex),
                Endian::Big => (0x84, ParseSize::Complex),
            }
        }
        "bxml" => {
            match endian {
                Endian::Little => (0x778, ParseSize::Complex),
                Endian::Big => (0x4a8, ParseSize::Complex),
            }
        }
        "bgparamlist" => {
            match endian {
                Endian::Little => (0x2c0, ParseSize::Complex),
                Endian::Big => (0x248, ParseSize::Complex),
            }
        }
        "bmodellist" => {
            match endian {
                Endian::Little => (0x7d0, ParseSize::Complex),
                Endian::Big => (0x508, ParseSize::Complex),
            }
        }
        "baslist" => {
            match endian {
                Endian::Little => (0x410, ParseSize::Complex),
                Endian::Big => (0x2f4, ParseSize::Complex),
            }
        }
        "baiprog" => {
            match endian {
                Endian::Little => (0x448, ParseSize::Complex),
                Endian::Big => (0x30c, ParseSize::Complex),
            }
        }
        "bphysics" => {
            match endian {
                Endian::Little => (0x470, ParseSize::Complex),
                Endian::Big => (0x324, ParseSize::Complex),
            }
        }
        "bchemical" => {
            match endian {
                Endian::Little => (0x3c0, ParseSize::Complex),
                Endian::Big => (0x2cc, ParseSize::Complex),
            }
        }
        "bas" => {
            match endian {
                Endian::Little => (0x3c8, ParseSize::Complex),
                Endian::Big => (0x2d0, ParseSize::Complex),
            }
        }
        "batcllist" => {
            match endian {
                Endian::Little => (0x3f0, ParseSize::Complex),
                Endian::Big => (0x2e4, ParseSize::Complex),
            }
        }
        "batcl" => {
            match endian {
                Endian::Little => (0x428, ParseSize::Complex),
                Endian::Big => (0x344, ParseSize::Complex),
            }
        }
        "baischedule" => {
            match endian {
                Endian::Little => (0x2b8, ParseSize::Simple(0)),
                Endian::Big => (0x244, ParseSize::Simple(0)),
            }
        }
        "bdmgparam" => {
            match endian {
                Endian::Little => (0x11d0, ParseSize::Simple(0x790)),
                Endian::Big => (0x9f0, ParseSize::Simple(0x3c0)),
            }
        }
        "brgconfiglist" => {
            match endian {
                Endian::Little => (0x3d0, ParseSize::Complex),
                Endian::Big => (0x2d4, ParseSize::Complex),
            }
        }
        "brgconfig" => {
            match endian {
                Endian::Little => (0x42d8, ParseSize::Simple(0)),
                Endian::Big => (0x2acc, ParseSize::Simple(0)),
            }
        }
        "brgbw" => {
            match endian {
                Endian::Little => (0x2c0, ParseSize::Complex),
                Endian::Big => (0x248, ParseSize::Complex),
            }
        }
        "bawareness" => {
            match endian {
                Endian::Little => (0xb38, ParseSize::Simple(0)),
                Endian::Big => (0x70c, ParseSize::Simple(0)),
            }
        }
        "bdrop" => {
            match endian {
                Endian::Little => (0x320, ParseSize::Complex),
                Endian::Big => (0x27c, ParseSize::Complex),
            }
        }
        "bshop" => {
            match endian {
                Endian::Little => (0x320, ParseSize::Complex),
                Endian::Big => (0x27c, ParseSize::Complex),
            }
        }
        "brecipe" => {
            match endian {
                Endian::Little => (0x320, ParseSize::Complex),
                Endian::Big => (0x27c, ParseSize::Complex),
            }
        }
        "blod" => {
            match endian {
                Endian::Little => (0x3c0, ParseSize::Simple(0)),
                Endian::Big => (0x2cc, ParseSize::Simple(0)),
            }
        }
        "bbonectrl" => {
            match endian {
                Endian::Little => (0x8d0, ParseSize::Complex),
                Endian::Big => (0x564, ParseSize::Complex),
            }
        }
        "blifecondition" => {
            match endian {
                Endian::Little => (0x4b0, ParseSize::Complex),
                Endian::Big => (0x35c, ParseSize::Complex),
            }
        }
        "bumii" => {
            match endian {
                Endian::Little => (0x2b8, ParseSize::Simple(0)),
                Endian::Big => (0x244, ParseSize::Simple(0)),
            }
        }
        "baniminfo" => {
            match endian {
                Endian::Little => (0x2c8, ParseSize::Complex),
                Endian::Big => (0x24c, ParseSize::Complex),
            }
        }
        "byaml" => {
            match endian {
                Endian::Little => (0x20, ParseSize::Simple(0)),
                Endian::Big => (0x14, ParseSize::Simple(0)),
            }
        }
        "bassetting" => {
            match endian {
                Endian::Little => (0x260, ParseSize::Complex),
                Endian::Big => (0x1d8, ParseSize::Complex),
            }
        }
        "hkrb" => {
            match endian {
                Endian::Little => (0x20, ParseSize::Simple(0)),
                Endian::Big => (0x14, ParseSize::Simple(40)),
            }
        }
        "hkrg" => {
            match endian {
                Endian::Little => (0x20, ParseSize::Simple(0)),
                Endian::Big => (0x14, ParseSize::Simple(0)),
            }
        }
        "bphyssb" => {
            match endian {
                Endian::Little => (0x5b0, ParseSize::Complex),
                Endian::Big => (0x384, ParseSize::Complex),
            }
        }
        "hkcl" => {
            match endian {
                Endian::Little => (0xe8, ParseSize::Complex),
                Endian::Big => (0xb8, ParseSize::Complex),
            }
        }
        "hksc" => {
            match endian {
                Endian::Little => (0x140, ParseSize::Complex),
                Endian::Big => (0xe8, ParseSize::Complex),
            }
        }
        "hktmrb" => {
            match endian {
                Endian::Little => (0x48, ParseSize::Complex),
                Endian::Big => (0x28, ParseSize::Complex),
            }
        }
        "brgcon" => {
            match endian {
                Endian::Little => (0x48, ParseSize::Complex),
                Endian::Big => (0x28, ParseSize::Complex),
            }
        }
        "esetlist" => {
            match endian {
                Endian::Little => (0x38, ParseSize::Simple(0)),
                Endian::Big => (0x20, ParseSize::Simple(0)),
            }
        }
        "bdemo" => {
            match endian {
                Endian::Little => (0xb20, ParseSize::Simple(0)),
                Endian::Big => (0x6cc, ParseSize::Simple(0)),
            }
        }
        "bfevfl" => {
            match endian {
                Endian::Little => (0x40, ParseSize::Simple(0)),
                Endian::Big => (0x24, ParseSize::Simple(0)),
            }
        }
        "bfevtm" => {
            match endian {
                Endian::Little => (0x40, ParseSize::Simple(0)),
                Endian::Big => (0x24, ParseSize::Simple(0)),
            }
        }
        _ => {
            match endian {
                Endian::Little => (0x38, ParseSize::Simple(0)),
                Endian::Big => (0x20, ParseSize::Simple(0)),
            }
        }
    }
}
