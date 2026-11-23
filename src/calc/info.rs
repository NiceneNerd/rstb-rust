use crate::calc::cpp_memsizes::{
    baiprog,
    baniminfo,
    baslist,
    bchemical,
    bdrop,
    bgparamlist,
    blifecondition,
    bmodellist,
    bphysics,
    bphyssb,
    brecipe,
    brgconfiglist,
    bshop,
    bxml
};
use crate::calc::cpp_memsizes::cpp_classes::{
    ActorLink::ActorLink,
    AIProgram::AIProgram,
    AnimationInfo::AnimInfo,
    ASList::ASList,
    Chemical::Chemical,
    DropTable::Drop,
    GParamList::GParamList,
    LifeCondition::LifeCondition,
    ModelList::ModelList,
    Physics::Physics,
    RagdollConfigList::RagdollConfigList,
    Recipe::Recipe,
    ShopData::Shop,
    SupportBone::SupportBoneResource,
};
use crate::Endian;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParseSize {
    Simple(u32),
    Complex(Option<u32>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FactoryType {
    ActorCapture,
    ActorLink,
    AIProgram,
    AISchedule,
    AnimInfo,
    AnimSeq,
    AnimSeqList,
    AnimSeqSetting,
    AttentionClient,
    AttentionClientList,
    AudioResource,
    AwarenessParams,
    Batpl,
    Base,
    BinaryYAML,
    BoneControl,
    Byaml,
    CafeResource,
    CafeResourceTexture,
    CamAnim,
    ChemicalParams,
    DamageParams,
    Demo,
    DropTable,
    EffectSettingList,
    EventFlow,
    EventPack,
    GameData,
    GeneralParamList,
    HavokCloth,
    HavokNavMesh,
    HavokRagdoll,
    HavokRigidBody,
    HavokStaticCompound,
    HavokTmrb,
    ItemIcon,
    LifeCondition,
    Lua,
    DoILookLikeIKnowWhatAJPEGIs,
    LevelOfDetail,
    Map,
    MapTexture,
    MiscTexture,
    ModelList,
    MusicDefinition,
    PhysicsParams,
    PhysicsSupportBone,
    Placement,
    RagdollBlendWeight,
    RagdollConfig,
    RagdollConfigList,
    RagdollControl,
    Recipe,
    Sarc,
    SaveData,
    ShopTable,
    Timeline,
    UMii,
}

impl From<&str> for FactoryType {
    fn from(value: &str) -> Self {
        match value {
            "sarc" | "pack" | "bactorpack" | "bmodelsh" | "stats" | "stera" => Self::Sarc,
            "Tex.bfres" | "Tex1.bfres" | "Tex2.bfres" => Self::CafeResourceTexture,
            "bactcapt" => Self::ActorCapture,
            "baiprog" => Self::AIProgram,
            "baischedule" => Self::AISchedule,
            "baniminfo" => Self::AnimInfo,
            "bars" => Self::AudioResource,
            "bas" => Self::AnimSeq,
            "baslist" => Self::AnimSeqList,
            "bassetting" => Self::AnimSeqSetting,
            "batcl" => Self::AttentionClient,
            "batcllist" => Self::AttentionClientList,
            "batpl" | "bnfprl" => Self::Batpl,
            "bawareness" => Self::AwarenessParams,
            "bbonectrl" => Self::BoneControl,
            "bcamanim" => Self::CamAnim,
            "bchemical" => Self::ChemicalParams,
            "bdemo" => Self::Demo,
            "bdmgparam" => Self::DamageParams,
            "bdrop" => Self::DropTable,
            "beventpack" => Self::EventPack,
            "bfevfl" => Self::EventFlow,
            "bfevtm" => Self::Timeline,
            "bfres" => Self::CafeResource,
            "bgdata" => Self::GameData,
            "bgparamlist" => Self::GeneralParamList,
            "bgsvdata" => Self::SaveData,
            "bitemico" => Self::ItemIcon,
            "blifecondition" => Self::LifeCondition,
            "blod" => Self::LevelOfDetail,
            "bmaptex" => Self::MapTexture,
            "bmodellist" => Self::ModelList,
            "bmscdef" => Self::MusicDefinition,
            "bphysics" => Self::PhysicsParams,
            "bphyssb" => Self::PhysicsSupportBone,
            "bplacement" => Self::Placement,
            "brecipe" => Self::Recipe,
            "brgbw" => Self::RagdollBlendWeight,
            "brgconfig" => Self::RagdollConfig,
            "brgconfiglist" => Self::RagdollConfigList,
            "brgcon" => Self::RagdollControl,
            "bshop" => Self::ShopTable,
            "bstftex" | "bmapopen" | "breviewtex" => Self::MiscTexture,
            "bumii" => Self::UMii,
            "bxml" => Self::ActorLink,
            "byaml" => Self::Byaml,
            "byml" => Self::BinaryYAML,
            "esetlist" => Self::EffectSettingList,
            "hkcl" => Self::HavokCloth,
            "hknm2" => Self::HavokNavMesh,
            "hkrb" => Self::HavokRigidBody,
            "hkrg" => Self::HavokRagdoll,
            "hksc" => Self::HavokStaticCompound,
            "hks" | "lua" => Self::Lua,
            "hktmrb" => Self::HavokTmrb,
            "jpg" => Self::DoILookLikeIKnowWhatAJPEGIs,
            "mubin" => Self::Map,
            _ => Self::Base,
        }
    }
}

pub(crate) fn get_factory_info(factory_type: FactoryType, endian: Endian, bytes: &[u8]) -> (u32, isize, ParseSize) {
    match factory_type {
        FactoryType::Sarc | FactoryType::EventPack => {
            match endian {
                Endian::Little => (0x68, 0x80, ParseSize::Simple(0)),
                Endian::Big => (0x3c, 0x80, ParseSize::Simple(0)),
            }
        }
        FactoryType::CafeResourceTexture => {
            match endian {
                Endian::Little => (0x38, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x20, 0x4, ParseSize::Simple(0)),
            }
        }
        FactoryType::CafeResource => {
            match endian {
                Endian::Little => (0x1a8, 0x1000, ParseSize::Complex(None)),
                Endian::Big => (0x13c, 0x1000, ParseSize::Complex(None)),
            }
        }
        FactoryType::CamAnim => {
            match endian {
                Endian::Little => (0x50, 0x2000, ParseSize::Complex(None)),
                Endian::Big => (0x2c, 0x2000, ParseSize::Complex(None)),
            }
        }
        FactoryType::Batpl => {
            match endian {
                Endian::Little => (0x40, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x24, 0x4, ParseSize::Simple(0)),
            }
        }
        FactoryType::Placement => {
            match endian {
                Endian::Little => (0x48, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0)),
            }
        }
        FactoryType::Lua => {
            match endian {
                Endian::Little => (0x38, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0)),
            }
        }
        FactoryType::ActorCapture => {
            match endian {
                Endian::Little => (0x538, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x3b4, 0x4, ParseSize::Simple(0)),
            }
        }
        FactoryType::ItemIcon => {
            match endian {
                Endian::Little => (0x60, 0x2000, ParseSize::Simple(0)),
                Endian::Big => (0xd0, 0x2000, ParseSize::Simple(0)),
            }
        }
        FactoryType::DoILookLikeIKnowWhatAJPEGIs => {
            match endian {
                Endian::Little => (0x80, 0x2000, ParseSize::Simple(0)),
                Endian::Big => (0x174, 0x2000, ParseSize::Simple(0)),
            }
        }
        FactoryType::MapTexture => {
            match endian {
                Endian::Little => (0x60, 0x2000, ParseSize::Simple(0)),
                Endian::Big => (0xd0, 0x2000, ParseSize::Simple(0)),
            }
        }
        FactoryType::MiscTexture => {
            match endian {
                Endian::Little => (0x60, 0x2000, ParseSize::Simple(0)),
                Endian::Big => (0xd0, 0x2000, ParseSize::Simple(0)),
            }
        }
        FactoryType::GameData => {
            match endian {
                Endian::Little => (0x140, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0xcc, 0x4, ParseSize::Simple(0)),
            }
        }
        FactoryType::SaveData => {
            match endian {
                Endian::Little => (0x38, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0)),
            }
        }
        FactoryType::HavokNavMesh => {
            match endian {
                Endian::Little => (0x48, 0x8, ParseSize::Complex(Some(0x248))),
                Endian::Big => (0x28, 0x4, ParseSize::Complex(Some(0x174))),
            }
        }
        FactoryType::MusicDefinition => {
            match endian {
                Endian::Little => (0x2a8, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x1fc, 0x4, ParseSize::Complex(None)),
            }
        }
        FactoryType::AudioResource => {
            match endian {
                Endian::Little => (0xb0, 0x80, ParseSize::Complex(None)),
                Endian::Big => (0x84, 0x80, ParseSize::Complex(None)),
            }
        }
        FactoryType::ActorLink => {
            match endian {
                Endian::Little => (
                    size_of::<ActorLink<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(bxml::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<ActorLink<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(bxml::parse_size(bytes, endian))
                ),
            }
        }
        FactoryType::GeneralParamList => {
            match endian {
                Endian::Little => (
                    size_of::<GParamList<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(bgparamlist::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<GParamList<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(bgparamlist::parse_size(bytes, endian))
                ),
            }
        }
        FactoryType::ModelList => {
            match endian {
                Endian::Little => (
                    size_of::<ModelList<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(bmodellist::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<ModelList<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(bmodellist::parse_size(bytes, endian))
                ),
            }
        }
        FactoryType::AnimSeqList => {
            match endian {
                Endian::Little => (
                    size_of::<ASList<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(baslist::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<ASList<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(baslist::parse_size(bytes, endian))
                ),
            }
        }
        FactoryType::AIProgram => {
            match endian {
                Endian::Little => (
                    size_of::<AIProgram<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(baiprog::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<AIProgram<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(baiprog::parse_size(bytes, endian))
                ),
            }
        }
        FactoryType::PhysicsParams => {
            match endian {
                Endian::Little => (
                    size_of::<Physics<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(bphysics::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<Physics<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(bphysics::parse_size(bytes, endian))
                ),
            }
        }
        FactoryType::ChemicalParams => {
            match endian {
                Endian::Little => (
                    size_of::<Chemical<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(bchemical::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<Chemical<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(bchemical::parse_size(bytes, endian))
                ),
            }
        }
        FactoryType::AnimSeq => {
            match endian {
                Endian::Little => (0x3c8, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x2d0, 0x4, ParseSize::Complex(None)),
            }
        }
        FactoryType::AttentionClientList => {
            match endian {
                Endian::Little => (0x3f0, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x2e4, 0x4, ParseSize::Complex(None)),
            }
        }
        FactoryType::AttentionClient => {
            match endian {
                Endian::Little => (0x428, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x344, 0x4, ParseSize::Complex(None)),
            }
        }
        FactoryType::AISchedule => {
            match endian {
                Endian::Little => (0x2b8, 0x8, ParseSize::Simple(0x20)),
                Endian::Big => (0x244, 0x4, ParseSize::Simple(0x18)),
            }
        }
        FactoryType::DamageParams => {
            match endian {
                Endian::Little => (0x11d0, 0x8, ParseSize::Simple(0x7f8)),
                Endian::Big => (0x9f0, 0x4, ParseSize::Simple(0x40c)),
            }
        }
        FactoryType::RagdollConfigList => {
            match endian {
                Endian::Little => (
                    size_of::<RagdollConfigList<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(brgconfiglist::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<RagdollConfigList<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(brgconfiglist::parse_size(bytes, endian))
                ),
            }
        }
        FactoryType::RagdollConfig => {
            match endian {
                Endian::Little => (0x42d8, 0x8, ParseSize::Simple(0x20)),
                Endian::Big => (0x2acc, 0x4, ParseSize::Simple(0x10)),
            }
        }
        FactoryType::RagdollBlendWeight => {
            match endian {
                Endian::Little => (0x2c0, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x248, 0x4, ParseSize::Complex(None)),
            }
        }
        FactoryType::AwarenessParams => {
            match endian {
                Endian::Little => (0xb38, 0x8, ParseSize::Simple(0x20)),
                Endian::Big => (0x70c, 0x4, ParseSize::Simple(0x10)),
            }
        }
        FactoryType::DropTable => {
            match endian {
                Endian::Little => (
                    size_of::<Drop<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(bdrop::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<Drop<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(bdrop::parse_size(bytes, endian))
                ),
            }
        }
        FactoryType::ShopTable => {
            match endian {
                Endian::Little => (
                    size_of::<Shop<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(bshop::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<Shop<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(bshop::parse_size(bytes, endian))
                ),
            }
        }
        FactoryType::Recipe => {
            match endian {
                Endian::Little => (
                    size_of::<Recipe<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(brecipe::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<Recipe<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(brecipe::parse_size(bytes, endian))
                ),
            }
        }
        FactoryType::LevelOfDetail => {
            match endian {
                Endian::Little => (0x3c0, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x2cc, 0x4, ParseSize::Simple(0x10)),
            }
        }
        FactoryType::BoneControl => {
            match endian {
                Endian::Little => (0x8d0, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x564, 0x4, ParseSize::Complex(None)),
            }
        }
        FactoryType::LifeCondition => {
            match endian {
                Endian::Little => (
                    size_of::<LifeCondition<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(blifecondition::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<LifeCondition<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(blifecondition::parse_size(bytes, endian))
                ),
            }
        }
        FactoryType::UMii => {
            match endian {
                Endian::Little => (0x2b8, 0x8, ParseSize::Simple(0x20)),
                Endian::Big => (0x244, 0x4, ParseSize::Simple(0x18)),
            }
        }
        FactoryType::AnimInfo => {
            match endian {
                Endian::Little => (
                    size_of::<AnimInfo<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(baniminfo::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<AnimInfo<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(baniminfo::parse_size(bytes, endian))
                ),
            }
        }
        FactoryType::Byaml => {
            match endian {
                Endian::Little => (0x20, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0)),
            }
        }
        FactoryType::BinaryYAML => {
            match endian {
                Endian::Little => (0x38, 0x8, ParseSize::Simple(0x28)),
                Endian::Big => (0x20, 0x4, ParseSize::Simple(0x3c)),
            }
        }
        FactoryType::AnimSeqSetting => {
            match endian {
                Endian::Little => (0x260, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x1d8, 0x4, ParseSize::Complex(None)),
            }
        }
        FactoryType::HavokRigidBody => {
            match endian {
                Endian::Little => (0x20, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0x8)),
            }
        }
        FactoryType::HavokRagdoll => {
            match endian {
                Endian::Little => (0x20, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0x8)),
            }
        }
        FactoryType::PhysicsSupportBone => {
            match endian {
                Endian::Little => (
                    size_of::<SupportBoneResource<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(bphyssb::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<SupportBoneResource<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(bphyssb::parse_size(bytes, endian))
                ),
            }
        }
        FactoryType::HavokCloth => {
            match endian {
                Endian::Little => (0xe8, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0xb8, 0x4, ParseSize::Complex(None)),
            }
        }
        FactoryType::HavokStaticCompound => {
            match endian {
                Endian::Little => (0x140, 0x8, ParseSize::Complex(Some(0x9ac0))),
                Endian::Big => (0xe8, 0x4, ParseSize::Complex(Some(0x73e4))),
            }
        }
        FactoryType::HavokTmrb => {
            match endian {
                Endian::Little => (0x48, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x28, 0x4, ParseSize::Complex(None)),
            }
        }
        FactoryType::RagdollControl => {
            match endian {
                Endian::Little => (0x48, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x28, 0x4, ParseSize::Complex(None)),
            }
        }
        FactoryType::EffectSettingList => {
            match endian {
                Endian::Little => (0x38, 0x4000, ParseSize::Simple(0)),
                Endian::Big => (0x20, 0x4000, ParseSize::Simple(0)),
            }
        }
        FactoryType::Demo => {
            match endian {
                Endian::Little => (0xb20, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x6cc, 0x4, ParseSize::Simple(0x10)),
            }
        }
        FactoryType::EventFlow => {
            match endian {
                Endian::Little => (0x40, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x24, 0x4, ParseSize::Simple(0x38)),
            }
        }
        FactoryType::Timeline => {
            match endian {
                Endian::Little => (0x40, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x24, 0x4, ParseSize::Simple(0x38)),
            }
        }
        FactoryType::Map => {
            match endian {
                Endian::Little => (0x38, 0x8, ParseSize::Simple(0x20)),
                Endian::Big => (0x20, 0x4, ParseSize::Simple(0x1C)),
            }
        }
        FactoryType::Base => {
            match endian {
                Endian::Little => (0x38, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x20, 0x4, ParseSize::Simple(0)),
            }
        }
    }
}
