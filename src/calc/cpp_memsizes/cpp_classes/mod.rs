#![allow(dead_code)]
#![allow(non_snake_case)]

pub mod ActorLink;
pub mod AIProgram;
pub mod AnimationInfo;
pub mod ASList;
pub mod Chemical;
pub mod DropTable;
pub mod GParamList;
pub mod LifeCondition;
pub mod ModelList;
pub mod Physics;
pub mod RagdollConfigList;
pub mod Recipe;
pub mod ShopData;
pub mod SupportBone;
pub mod agl;
pub mod sead;

#[repr(C)]
pub struct ParamIO<T> {
    base:   agl::IParameterIO<T>,
    base2:  sead::Node<T>,
    mIdx:   u32,
    mPath:  sead::FixedSafeString<T, 128>,
}


#[repr(C)]
struct IResource<T> {
    vfptr:  T,  // vtable*
}

#[repr(C)]
struct Resource<T> {
    base:           sead::DirectResource<T>,    // sead::DirectResource
    base2:          IResource<T>,               // IResource
    mAllocSize:     u32,                        // u32
    mContext:       T,                          // Context*
}

#[cfg(feature = "complex")]
#[cfg(test)]
mod tests {
    use crate::calc::cpp_memsizes::cpp_classes::*;

    #[test]
    fn agl_size_tests() {
        assert_eq!(size_of::<agl::ParameterBase<u32>>(), 0xC);
        assert_eq!(size_of::<agl::Parameter<u32, bool>>(), 0x10);
        assert_eq!(size_of::<agl::Parameter<u32, i32>>(), 0x10);
        assert_eq!(size_of::<agl::Parameter<u32, i32>>(), 0x10);
        assert_eq!(size_of::<agl::Parameter<u32, u32>>(), 0x10);
        assert_eq!(size_of::<agl::Parameter<u32, f32>>(), 0x10);
        assert_eq!(size_of::<agl::Parameter<u32, sead::Vector2f>>(), 0x14);
        assert_eq!(size_of::<agl::Parameter<u32, sead::Vector3f>>(), 0x18);
        assert_eq!(size_of::<agl::Parameter<u32, sead::Vector4f>>(), 0x1c);
        assert_eq!(size_of::<agl::Parameter<u32, sead::SafeString<u32>>>(), 0x14);
        assert_eq!(size_of::<agl::Parameter<u32, sead::FixedSafeString<u32, 32>>>(), 0x38);
        assert_eq!(size_of::<agl::Parameter<u32, sead::FixedSafeString<u32, 64>>>(), 0x58);
        assert_eq!(size_of::<agl::Parameter<u32, sead::FixedSafeString<u32, 128>>>(), 0x98);
        assert_eq!(size_of::<agl::Parameter<u32, sead::FixedSafeString<u32, 256>>>(), 0x118);
        assert_eq!(size_of::<agl::ParameterObj<u32>>(), 0x1c);
        assert_eq!(size_of::<agl::ParameterList<u32>>(), 0x24);
        assert_eq!(size_of::<agl::IParameterIO<u32>>(), 0x18C);
        assert_eq!(size_of::<agl::ParameterBase<u64>>(), 0x18);
        assert_eq!(size_of::<agl::Parameter<u64, bool>>(), 0x20);
        assert_eq!(size_of::<agl::Parameter<u64, i32>>(), 0x20);
        assert_eq!(size_of::<agl::Parameter<u64, i32>>(), 0x20);
        assert_eq!(size_of::<agl::Parameter<u64, u32>>(), 0x20);
        assert_eq!(size_of::<agl::Parameter<u64, f32>>(), 0x20);
        assert_eq!(size_of::<agl::Parameter<u64, sead::Vector2f>>(), 0x20);
        assert_eq!(size_of::<agl::Parameter<u64, sead::Vector3f>>(), 0x28);
        assert_eq!(size_of::<agl::Parameter<u64, sead::Vector4f>>(), 0x28);
        assert_eq!(size_of::<agl::Parameter<u64, sead::SafeString<u64>>>(), 0x28);
        assert_eq!(size_of::<agl::Parameter<u64, sead::FixedSafeString<u64, 32>>>(), 0x50);
        assert_eq!(size_of::<agl::Parameter<u64, sead::FixedSafeString<u64, 64>>>(), 0x70);
        assert_eq!(size_of::<agl::Parameter<u64, sead::FixedSafeString<u64, 128>>>(), 0xB0);
        assert_eq!(size_of::<agl::Parameter<u64, sead::FixedSafeString<u64, 256>>>(), 0x130);
        assert_eq!(size_of::<agl::ParameterObj<u64>>(), 0x30);
        assert_eq!(size_of::<agl::ParameterList<u64>>(), 0x48);
        assert_eq!(size_of::<agl::IParameterIO<u64>>(), 0x1D0);
    }

    #[test]
    fn sead_size_tests() {
        assert_eq!(size_of::<sead::SafeString<u32>>(), 0x8);
        assert_eq!(size_of::<sead::SafeString<u64>>(), 0x10);
        assert_eq!(size_of::<sead::FixedSafeString<u32, 32>>(), 0x2C);
        assert_eq!(size_of::<sead::FixedSafeString<u64, 32>>(), 0x38);
        assert_eq!(size_of::<sead::FixedSafeString<u32, 64>>(), 0x4C);
        assert_eq!(size_of::<sead::FixedSafeString<u64, 64>>(), 0x58);
        assert_eq!(size_of::<sead::FixedSafeString<u32, 128>>(), 0x8C);
        assert_eq!(size_of::<sead::FixedSafeString<u64, 128>>(), 0x98);
        assert_eq!(size_of::<sead::FixedSafeString<u32, 256>>(), 0x10C);
        assert_eq!(size_of::<sead::FixedSafeString<u64, 256>>(), 0x118);
        assert_eq!(size_of::<sead::Buffer<u32>>(), 0x8);
        assert_eq!(size_of::<sead::Buffer<u64>>(), 0x10);
        assert_eq!(size_of::<sead::Node<u32>>(), 0x4);
        assert_eq!(size_of::<sead::Node<u64>>(), 0x8);
    }

    #[test]
    fn base_size_tests() {
        assert_eq!(size_of::<Resource<u32>>(), 0x20);
        assert_eq!(size_of::<Resource<u64>>(), 0x38);
        assert_eq!(size_of::<ParamIO<u32>>(), 0x220);
        assert_eq!(size_of::<ParamIO<u64>>(), 0x278);
    }

    #[test]
    fn actorlink_size_tests() {
        assert_eq!(size_of::<ActorLink::ActorLink<u32>>(), 0x4A8);
        assert_eq!(size_of::<ActorLink::ActorLink<u64>>(), 0x778);
    }

    #[test]
    fn aiprogram_size_tests() {
        assert_eq!(size_of::<AIProgram::Definition<u32>>(), 0x50);
        assert_eq!(size_of::<AIProgram::Definition<u64>>(), 0x98);
        assert_eq!(size_of::<AIProgram::AIActionDef<u32>>(), 0x6C);
        assert_eq!(size_of::<AIProgram::AIActionDef<u64>>(), 0xC8);
        assert_eq!(size_of::<AIProgram::BehaviorDef<u32>>(), 0x54);
        assert_eq!(size_of::<AIProgram::BehaviorDef<u64>>(), 0xA0);
        assert_eq!(size_of::<AIProgram::QueryDef<u32>>(), 0x50);
        assert_eq!(size_of::<AIProgram::QueryDef<u64>>(), 0x98);
        assert_eq!(size_of::<AIProgram::AIProgram<u32>>(), 0x30C);
        assert_eq!(size_of::<AIProgram::AIProgram<u64>>(), 0x448);
    }

    #[test]
    fn animinfo_size_tests() {
        assert_eq!(size_of::<AnimationInfo::Anim<u32>>(), 0x14);
        assert_eq!(size_of::<AnimationInfo::Anim<u64>>(), 0x20);
        assert_eq!(size_of::<AnimationInfo::SwordBlur<u32>>(), 0x1C);
        assert_eq!(size_of::<AnimationInfo::SwordBlur<u64>>(), 0x30);
        assert_eq!(size_of::<AnimationInfo::SwordBlurInfo<u32>>(), 0xC);
        assert_eq!(size_of::<AnimationInfo::SwordBlurInfo<u64>>(), 0x18);
        assert_eq!(size_of::<AnimationInfo::AnimInfo<u32>>(), 0x24C);
        assert_eq!(size_of::<AnimationInfo::AnimInfo<u64>>(), 0x2C8);
    }

    #[test]
    fn aslist_size_tests() {
        assert_eq!(size_of::<ASList::ASDefine<u32>>(), 0x48);
        assert_eq!(size_of::<ASList::ASDefine<u64>>(), 0x88);
        assert_eq!(size_of::<ASList::CFPost<u32>>(), 0x50);
        assert_eq!(size_of::<ASList::CFPost<u64>>(), 0x98);
        assert_eq!(size_of::<ASList::CFExcept<u32>>(), 0x14);
        assert_eq!(size_of::<ASList::CFExcept<u64>>(), 0x28);
        assert_eq!(size_of::<ASList::CFDefine<u32>>(), 0xA4);
        assert_eq!(size_of::<ASList::CFDefine<u64>>(), 0x138);
        assert_eq!(size_of::<ASList::AddRes<u32>>(), 0x54);
        assert_eq!(size_of::<ASList::AddRes<u64>>(), 0xA0);
        assert_eq!(size_of::<ASList::Common<u32>>(), 0x2C);
        assert_eq!(size_of::<ASList::Common<u64>>(), 0x50);
        assert_eq!(size_of::<ASList::ASList<u32>>(), 0x2F4);
        assert_eq!(size_of::<ASList::ASList<u64>>(), 0x410);
    }

    #[test]
    fn chemical_size_tests() {
        assert_eq!(size_of::<Chemical::Rigid<u32>>(), 0x10C);
        assert_eq!(size_of::<Chemical::Rigid<u64>>(), 0x190);
        assert_eq!(size_of::<Chemical::Shape<u32>>(), 0x240);
        assert_eq!(size_of::<Chemical::Shape<u64>>(), 0x2F0);
        assert_eq!(size_of::<Chemical::Root<u32>>(), 0x8C);
        assert_eq!(size_of::<Chemical::Root<u64>>(), 0x110);
        assert_eq!(size_of::<Chemical::Chemical<u32>>(), 0x2CC);
        assert_eq!(size_of::<Chemical::Chemical<u64>>(), 0x3C0);
    }

    #[test]
    fn droptable_size_tests() {
        assert_eq!(size_of::<DropTable::Item<u32>>(), 0x24);
        assert_eq!(size_of::<DropTable::Item<u64>>(), 0x48);
        assert_eq!(size_of::<DropTable::Table<u32>>(), 0x88);
        assert_eq!(size_of::<DropTable::Table<u64>>(), 0x108);
        assert_eq!(size_of::<DropTable::Drop<u32>>(), 0x27C);
        assert_eq!(size_of::<DropTable::Drop<u64>>(), 0x320);
    }

    #[test]
    fn gparamlist_size_tests() {
        assert_eq!(size_of::<GParamList::GParamListObj<u32>>(), 0x20);
        assert_eq!(size_of::<GParamList::GParamListObj<u64>>(), 0x38);
        assert_eq!(size_of::<GParamList::GParamListObjectAirWall<u32>>(), 0x34);
        assert_eq!(size_of::<GParamList::GParamListObjectAirWall<u64>>(), 0x60);
        assert_eq!(size_of::<GParamList::GParamListObjectAnimalFollowOffset<u32>>(), 0x38);
        assert_eq!(size_of::<GParamList::GParamListObjectAnimalFollowOffset<u64>>(), 0x60);
        assert_eq!(size_of::<GParamList::GParamListObjectAnimalUnit<u32>>(), 0x134);
        assert_eq!(size_of::<GParamList::GParamListObjectAnimalUnit<u64>>(), 0x260);
        assert_eq!(size_of::<GParamList::GParamListObjectArmor<u32>>(), 0xA8);
        assert_eq!(size_of::<GParamList::GParamListObjectArmor<u64>>(), 0x138);
        assert_eq!(size_of::<GParamList::GParamListObjectArmorEffect<u32>>(), 0x74);
        assert_eq!(size_of::<GParamList::GParamListObjectArmorEffect<u64>>(), 0xE0);
        assert_eq!(size_of::<GParamList::GParamListObjectArmorHead<u32>>(), 0x5C);
        assert_eq!(size_of::<GParamList::GParamListObjectArmorHead<u64>>(), 0xA8);
        assert_eq!(size_of::<GParamList::GParamListObjectArmorUpper<u32>>(), 0x80);
        assert_eq!(size_of::<GParamList::GParamListObjectArmorUpper<u64>>(), 0xE8);
        assert_eq!(size_of::<GParamList::GParamListObjectArrow<u32>>(), 0x80);
        assert_eq!(size_of::<GParamList::GParamListObjectArrow<u64>>(), 0xF8);
        assert_eq!(size_of::<GParamList::GParamListObjectAttack<u32>>(), 0xCC);
        assert_eq!(size_of::<GParamList::GParamListObjectAttack<u64>>(), 0x190);
        assert_eq!(size_of::<GParamList::GParamListObjectAttackInterval<u32>>(), 0x80);
        assert_eq!(size_of::<GParamList::GParamListObjectAttackInterval<u64>>(), 0xF8);
        assert_eq!(size_of::<GParamList::GParamListObjectAutoGen<u32>>(), 0x58);
        assert_eq!(size_of::<GParamList::GParamListObjectAutoGen<u64>>(), 0xA8);
        assert_eq!(size_of::<GParamList::GParamListObjectBeam<u32>>(), 0x30);
        assert_eq!(size_of::<GParamList::GParamListObjectBeam<u64>>(), 0x58);
        assert_eq!(size_of::<GParamList::GParamListObjectBindActor<u32>>(), 0x44);
        assert_eq!(size_of::<GParamList::GParamListObjectBindActor<u64>>(), 0x80); // *
        assert_eq!(size_of::<GParamList::GParamListObjectBindBone<u32>>(), 0x64);
        assert_eq!(size_of::<GParamList::GParamListObjectBindBone<u64>>(), 0xB0);
        assert_eq!(size_of::<GParamList::GParamListObjectBow<u32>>(), 0x2DC);
        assert_eq!(size_of::<GParamList::GParamListObjectBow<u64>>(), 0x540);
        assert_eq!(size_of::<GParamList::GParamListObjectBullet<u32>>(), 0x40);
        assert_eq!(size_of::<GParamList::GParamListObjectBullet<u64>>(), 0x78);
        assert_eq!(size_of::<GParamList::GParamListObjectCamera<u32>>(), 0xD0);
        assert_eq!(size_of::<GParamList::GParamListObjectCamera<u64>>(), 0x198);
        assert_eq!(size_of::<GParamList::GParamListObjectChemicalType<u32>>(), 0x48);
        assert_eq!(size_of::<GParamList::GParamListObjectChemicalType<u64>>(), 0x88);
        assert_eq!(size_of::<GParamList::GParamListObjectClothReaction<u32>>(), 0x150);
        assert_eq!(size_of::<GParamList::GParamListObjectClothReaction<u64>>(), 0x288);
        assert_eq!(size_of::<GParamList::GParamListObjectCookSpice<u32>>(), 0x70);
        assert_eq!(size_of::<GParamList::GParamListObjectCookSpice<u64>>(), 0xD8);
        assert_eq!(size_of::<GParamList::GParamListObjectCureItem<u32>>(), 0x64);
        assert_eq!(size_of::<GParamList::GParamListObjectCureItem<u64>>(), 0xC0);
        assert_eq!(size_of::<GParamList::GParamListObjectEatTarget<u32>>(), 0x98);
        assert_eq!(size_of::<GParamList::GParamListObjectEatTarget<u64>>(), 0x128);
        assert_eq!(size_of::<GParamList::GParamListObjectEnemy<u32>>(), 0x148);
        assert_eq!(size_of::<GParamList::GParamListObjectEnemy<u64>>(), 0x288);
        assert_eq!(size_of::<GParamList::GParamListObjectEnemyLevel<u32>>(), 0xE0);
        assert_eq!(size_of::<GParamList::GParamListObjectEnemyLevel<u64>>(), 0x1B8);
        assert_eq!(size_of::<GParamList::GParamListObjectEnemyRace<u32>>(), 0x31C);
        assert_eq!(size_of::<GParamList::GParamListObjectEnemyRace<u64>>(), 0x590);
        assert_eq!(size_of::<GParamList::GParamListObjectEnemyShown<u32>>(), 0x60);
        assert_eq!(size_of::<GParamList::GParamListObjectEnemyShown<u64>>(), 0xB8);
        assert_eq!(size_of::<GParamList::GParamListObjectEvent<u32>>(), 0xC0);
        assert_eq!(size_of::<GParamList::GParamListObjectEvent<u64>>(), 0x178);
        assert_eq!(size_of::<GParamList::GParamListObjectExtendedEntity<u32>>(), 0x40);
        assert_eq!(size_of::<GParamList::GParamListObjectExtendedEntity<u64>>(), 0x78);
        assert_eq!(size_of::<GParamList::GParamListObjectFish<u32>>(), 0x70);
        assert_eq!(size_of::<GParamList::GParamListObjectFish<u64>>(), 0xD8);
        assert_eq!(size_of::<GParamList::GParamListObjectGelEnemy<u32>>(), 0x130);
        assert_eq!(size_of::<GParamList::GParamListObjectGelEnemy<u64>>(), 0x248);
        assert_eq!(size_of::<GParamList::GParamListObjectGeneral<u32>>(), 0xA8);
        assert_eq!(size_of::<GParamList::GParamListObjectGeneral<u64>>(), 0x148);
        assert_eq!(size_of::<GParamList::GParamListObjectGiantArmor<u32>>(), 0x48);
        assert_eq!(size_of::<GParamList::GParamListObjectGiantArmor<u64>>(), 0x80);
        assert_eq!(size_of::<GParamList::GParamListObjectGiantArmorSlot<u32>>(), 0x110);
        assert_eq!(size_of::<GParamList::GParamListObjectGiantArmorSlot<u64>>(), 0x218);
        assert_eq!(size_of::<GParamList::GParamListObjectGlobal<u32>>(), 0x5F0);
        assert_eq!(size_of::<GParamList::GParamListObjectGlobal<u64>>(), 0xB78);
        assert_eq!(size_of::<GParamList::GParamListObjectGolem<u32>>(), 0x94);
        assert_eq!(size_of::<GParamList::GParamListObjectGolem<u64>>(), 0x120);
        assert_eq!(size_of::<GParamList::GParamListObjectGolemIK<u32>>(), 0x1B0);
        assert_eq!(size_of::<GParamList::GParamListObjectGolemIK<u64>>(), 0x358);
        assert_eq!(size_of::<GParamList::GParamListObjectGrab<u32>>(), 0x110);
        assert_eq!(size_of::<GParamList::GParamListObjectGrab<u64>>(), 0x218);
        assert_eq!(size_of::<GParamList::GParamListObjectGuardian<u32>>(), 0xB4);
        assert_eq!(size_of::<GParamList::GParamListObjectGuardian<u64>>(), 0x160);
        assert_eq!(size_of::<GParamList::GParamListObjectGuardianMini<u32>>(), 0x94);
        assert_eq!(size_of::<GParamList::GParamListObjectGuardianMini<u64>>(), 0x120);
        assert_eq!(size_of::<GParamList::GParamListObjectGuardianMiniWeapon<u32>>(), 0x84);
        assert_eq!(size_of::<GParamList::GParamListObjectGuardianMiniWeapon<u64>>(), 0x100);
        assert_eq!(size_of::<GParamList::GParamListObjectHorse<u32>>(), 0xEC);
        assert_eq!(size_of::<GParamList::GParamListObjectHorse<u64>>(), 0x1D0);
        assert_eq!(size_of::<GParamList::GParamListObjectHorseCreator<u32>>(), 0x48);
        assert_eq!(size_of::<GParamList::GParamListObjectHorseCreator<u64>>(), 0x88);
        assert_eq!(size_of::<GParamList::GParamListObjectHorseObject<u32>>(), 0x40);
        assert_eq!(size_of::<GParamList::GParamListObjectHorseObject<u64>>(), 0x78); // *
        assert_eq!(size_of::<GParamList::GParamListObjectHorseRider<u32>>(), 0x184);
        assert_eq!(size_of::<GParamList::GParamListObjectHorseRider<u64>>(), 0x2C0);
        assert_eq!(size_of::<GParamList::GParamListObjectHorseTargetedInfo<u32>>(), 0x50);
        assert_eq!(size_of::<GParamList::GParamListObjectHorseTargetedInfo<u64>>(), 0x98);
        assert_eq!(size_of::<GParamList::GParamListObjectHorseUnit<u32>>(), 0x60);
        assert_eq!(size_of::<GParamList::GParamListObjectHorseUnit<u64>>(), 0xB8);
        assert_eq!(size_of::<GParamList::GParamListObjectInsect<u32>>(), 0x30);
        assert_eq!(size_of::<GParamList::GParamListObjectInsect<u64>>(), 0x58);
        assert_eq!(size_of::<GParamList::GParamListObjectItem<u32>>(), 0x94);
        assert_eq!(size_of::<GParamList::GParamListObjectItem<u64>>(), 0x120);
        assert_eq!(size_of::<GParamList::GParamListObjectLargeSword<u32>>(), 0x228);
        assert_eq!(size_of::<GParamList::GParamListObjectLargeSword<u64>>(), 0x3A8);
        assert_eq!(size_of::<GParamList::GParamListObjectLiftable<u32>>(), 0x18C);
        assert_eq!(size_of::<GParamList::GParamListObjectLiftable<u64>>(), 0x2E0);
        assert_eq!(size_of::<GParamList::GParamListObjectLumberjackTree<u32>>(), 0x90);
        assert_eq!(size_of::<GParamList::GParamListObjectLumberjackTree<u64>>(), 0x118);
        assert_eq!(size_of::<GParamList::GParamListObjectMasterSword<u32>>(), 0xA8);
        assert_eq!(size_of::<GParamList::GParamListObjectMasterSword<u64>>(), 0x148);
        assert_eq!(size_of::<GParamList::GParamListObjectMonsterShop<u32>>(), 0x40);
        assert_eq!(size_of::<GParamList::GParamListObjectMonsterShop<u64>>(), 0x78);
        assert_eq!(size_of::<GParamList::GParamListObjectMotorcycle<u32>>(), 0x280);
        assert_eq!(size_of::<GParamList::GParamListObjectMotorcycle<u64>>(), 0x4E8);
        assert_eq!(size_of::<GParamList::GParamListObjectNest<u32>>(), 0x34);
        assert_eq!(size_of::<GParamList::GParamListObjectNest<u64>>(), 0x60);
        assert_eq!(size_of::<GParamList::GParamListObjectNpc<u32>>(), 0x11C);
        assert_eq!(size_of::<GParamList::GParamListObjectNpc<u64>>(), 0x230);
        assert_eq!(size_of::<GParamList::GParamListObjectNpcEquipment<u32>>(), 0x240);
        assert_eq!(size_of::<GParamList::GParamListObjectNpcEquipment<u64>>(), 0x3F8);
        assert_eq!(size_of::<GParamList::GParamListObjectPictureBook<u32>>(), 0x50);
        assert_eq!(size_of::<GParamList::GParamListObjectPictureBook<u64>>(), 0x98);
        assert_eq!(size_of::<GParamList::GParamListObjectPlayer<u32>>(), 0xAC0);
        assert_eq!(size_of::<GParamList::GParamListObjectPlayer<u64>>(), 0x1578);
        assert_eq!(size_of::<GParamList::GParamListObjectPrey<u32>>(), 0x70);
        assert_eq!(size_of::<GParamList::GParamListObjectPrey<u64>>(), 0xD8);
        assert_eq!(size_of::<GParamList::GParamListObjectRod<u32>>(), 0x114);
        assert_eq!(size_of::<GParamList::GParamListObjectRod<u64>>(), 0x220);
        assert_eq!(size_of::<GParamList::GParamListObjectRope<u32>>(), 0xC0);
        assert_eq!(size_of::<GParamList::GParamListObjectRope<u64>>(), 0x178);
        assert_eq!(size_of::<GParamList::GParamListObjectRupee<u32>>(), 0x30);
        assert_eq!(size_of::<GParamList::GParamListObjectRupee<u64>>(), 0x58);
        assert_eq!(size_of::<GParamList::GParamListObjectSandworm<u32>>(), 0x1CC);
        assert_eq!(size_of::<GParamList::GParamListObjectSandworm<u64>>(), 0x388);
        assert_eq!(size_of::<GParamList::GParamListObjectSeriesArmor<u32>>(), 0x44);
        assert_eq!(size_of::<GParamList::GParamListObjectSeriesArmor<u64>>(), 0x80);
        assert_eq!(size_of::<GParamList::GParamListObjectShiekerStone<u32>>(), 0xA8);
        assert_eq!(size_of::<GParamList::GParamListObjectShiekerStone<u64>>(), 0x128);
        assert_eq!(size_of::<GParamList::GParamListObjectShield<u32>>(), 0x1B4);
        assert_eq!(size_of::<GParamList::GParamListObjectShield<u64>>(), 0x2F0);
        assert_eq!(size_of::<GParamList::GParamListObjectSmallSword<u32>>(), 0x228);
        assert_eq!(size_of::<GParamList::GParamListObjectSmallSword<u64>>(), 0x3A8);
        assert_eq!(size_of::<GParamList::GParamListObjectSpear<u32>>(), 0x288);
        assert_eq!(size_of::<GParamList::GParamListObjectSpear<u64>>(), 0x448);
        assert_eq!(size_of::<GParamList::GParamListObjectStalEnemy<u32>>(), 0x48);
        assert_eq!(size_of::<GParamList::GParamListObjectStalEnemy<u64>>(), 0x88);
        assert_eq!(size_of::<GParamList::GParamListObjectSwarm<u32>>(), 0x54);
        assert_eq!(size_of::<GParamList::GParamListObjectSwarm<u64>>(), 0xA0);
        assert_eq!(size_of::<GParamList::GParamListObjectSystem<u32>>(), 0x44);
        assert_eq!(size_of::<GParamList::GParamListObjectSystem<u64>>(), 0x80);
        assert_eq!(size_of::<GParamList::GParamListObjectTraveler<u32>>(), 0x18A8);
        assert_eq!(size_of::<GParamList::GParamListObjectTraveler<u64>>(), 0x3148);
        assert_eq!(size_of::<GParamList::GParamListObjectWeaponCommon<u32>>(), 0x318);
        assert_eq!(size_of::<GParamList::GParamListObjectWeaponCommon<u64>>(), 0x620);
        assert_eq!(size_of::<GParamList::GParamListObjectWeaponOption<u32>>(), 0xB0);
        assert_eq!(size_of::<GParamList::GParamListObjectWeaponOption<u64>>(), 0x128);
        assert_eq!(size_of::<GParamList::GParamListObjectWeaponThrow<u32>>(), 0x68);
        assert_eq!(size_of::<GParamList::GParamListObjectWeaponThrow<u64>>(), 0xC0);
        assert_eq!(size_of::<GParamList::GParamListObjectWizzrobe<u32>>(), 0xB8);
        assert_eq!(size_of::<GParamList::GParamListObjectWizzrobe<u64>>(), 0x168);
        assert_eq!(size_of::<GParamList::GParamListObjectWolfLink<u32>>(), 0x440);
        assert_eq!(size_of::<GParamList::GParamListObjectWolfLink<u64>>(), 0x878);
        assert_eq!(size_of::<GParamList::GParamListObjectZora<u32>>(), 0x70);
        assert_eq!(size_of::<GParamList::GParamListObjectZora<u64>>(), 0xD8);
        assert_eq!(size_of::<GParamList::DirectionInfo<u32>>(), 0x60);
        assert_eq!(size_of::<GParamList::DirectionInfo<u64>>(), 0xC0);
        assert_eq!(size_of::<GParamList::RoutePoint<u32>>(), 0xD4);
        assert_eq!(size_of::<GParamList::RoutePoint<u64>>(), 0x1A8);
        assert_eq!(size_of::<GParamList::RoutePoints<u32>>(), 0x1804);
        assert_eq!(size_of::<GParamList::RoutePoints<u64>>(), 0x3008);
        assert_eq!(size_of::<GParamList::GParamList<u32>>(), 0x248);
        assert_eq!(size_of::<GParamList::GParamList<u64>>(), 0x2C0);
    }

    #[test]
    fn lifecondition_size_tests() {
        assert_eq!(size_of::<LifeCondition::LifeCondition<u32>>(), 0x35C);
        assert_eq!(size_of::<LifeCondition::LifeCondition<u64>>(), 0x4B0);
    }

    #[test]
    fn modellist_size_tests() {
        assert_eq!(size_of::<ModelList::ControllerInfo<u32>>(), 0xC0);
        assert_eq!(size_of::<ModelList::ControllerInfo<u64>>(), 0x160);
        assert_eq!(size_of::<ModelList::Attention<u32>>(), 0x1A0);
        assert_eq!(size_of::<ModelList::Attention<u64>>(), 0x300);
        assert_eq!(size_of::<ModelList::Unit<u32>>(), 0x44);
        assert_eq!(size_of::<ModelList::Unit<u64>>(), 0x80);
        assert_eq!(size_of::<ModelList::ModelData<u32>>(), 0x80);
        assert_eq!(size_of::<ModelList::ModelData<u64>>(), 0xF8);
        assert_eq!(size_of::<ModelList::Partial<u32>>(), 0x50);
        assert_eq!(size_of::<ModelList::Partial<u64>>(), 0x98);
        assert_eq!(size_of::<ModelList::AnmTarget<u32>>(), 0x9C);
        assert_eq!(size_of::<ModelList::AnmTarget<u64>>(), 0x130);
        assert_eq!(size_of::<ModelList::ModelDataInfo<u32>>(), 0x58);
        assert_eq!(size_of::<ModelList::ModelDataInfo<u64>>(), 0xA0);
        assert_eq!(size_of::<ModelList::AttentionInfo<u32>>(), 0x7C);
        assert_eq!(size_of::<ModelList::AttentionInfo<u64>>(), 0xB0);
        assert_eq!(size_of::<ModelList::PartialInfo<u32>>(), 0x10);
        assert_eq!(size_of::<ModelList::PartialInfo<u64>>(), 0x18);
        assert_eq!(size_of::<ModelList::ModelList<u32>>(), 0x500);
        assert_eq!(size_of::<ModelList::ModelList<u64>>(), 0x7D0);
    }

    #[test]
    fn physics_size_tests() {
        assert_eq!(size_of::<Physics::RigidBodySetParam<u32>>(), 0x98);
        assert_eq!(size_of::<Physics::RigidBodySetParam<u64>>(), 0x128);
        assert_eq!(size_of::<Physics::CharacterControllerParam<u32>>(), 0x300);
        assert_eq!(size_of::<Physics::CharacterControllerParam<u64>>(), 0x538);
        assert_eq!(size_of::<Physics::ICharacterControllerParam<u32>>(), 0x4);
        assert_eq!(size_of::<Physics::ICharacterControllerParam<u64>>(), 0x8);
        assert_eq!(size_of::<Physics::Form<u32>>(), 0x90);
        assert_eq!(size_of::<Physics::Form<u64>>(), 0xF8);
        assert_eq!(size_of::<Physics::ClothSetParam<u32>>(), 0x10C);
        assert_eq!(size_of::<Physics::ClothSetParam<u64>>(), 0x1C0);
        assert_eq!(size_of::<Physics::RagdollParam<u32>>(), 0x84);
        assert_eq!(size_of::<Physics::RagdollParam<u64>>(), 0xE0);
        assert_eq!(size_of::<Physics::SupportBoneParam<u32>>(), 0x30);
        assert_eq!(size_of::<Physics::SupportBoneParam<u64>>(), 0x58);
        assert_eq!(size_of::<Physics::ContactInfoParam<u32>>(), 0x70);
        assert_eq!(size_of::<Physics::ContactInfoParam<u64>>(), 0xD8);
        assert_eq!(size_of::<Physics::EdgeRigidBodySetParam<u32>>(), 0x2C);
        assert_eq!(size_of::<Physics::EdgeRigidBodySetParam<u64>>(), 0x58);
        assert_eq!(size_of::<Physics::Info<u32>>(), 0x30C);
        assert_eq!(size_of::<Physics::Info<u64>>(), 0x5E8);
        assert_eq!(size_of::<Physics::RigidBodyParam<u32>>(), 0x338);
        assert_eq!(size_of::<Physics::RigidBodyParam<u64>>(), 0x640);
        assert_eq!(size_of::<Physics::ClothSubWindParam<u32>>(), 0x54);
        assert_eq!(size_of::<Physics::ClothSubWindParam<u64>>(), 0x98);
        assert_eq!(size_of::<Physics::ClothParam<u32>>(), 0xC4);
        assert_eq!(size_of::<Physics::ClothParam<u64>>(), 0x180);
        assert_eq!(size_of::<Physics::ContactPointInfoParam<u32>>(), 0x9C);
        assert_eq!(size_of::<Physics::ContactPointInfoParam<u64>>(), 0xF0);
        assert_eq!(size_of::<Physics::CollisionInfoParam<u32>>(), 0x8C);
        assert_eq!(size_of::<Physics::CollisionInfoParam<u64>>(), 0xD0);
        assert_eq!(size_of::<Physics::EdgeRigidBodyParam<u32>>(), 0x58);
        assert_eq!(size_of::<Physics::EdgeRigidBodyParam<u64>>(), 0xA8);
        assert_eq!(size_of::<Physics::ShapeParamObj<u32>>(), 0x1C4);
        assert_eq!(size_of::<Physics::ShapeParamObj<u64>>(), 0x2C8);
        assert_eq!(size_of::<Physics::ParamSet<u32>>(), 0xE4);
        assert_eq!(size_of::<Physics::ParamSet<u64>>(), 0x1C0);
        assert_eq!(size_of::<Physics::Physics<u32>>(), 0x324);
        assert_eq!(size_of::<Physics::Physics<u64>>(), 0x470);
    }

    #[test]
    fn ragdollconfiglist_size_tests() {
        assert_eq!(size_of::<RagdollConfigList::ImpulseParam<u32>>(), 0x34);
        assert_eq!(size_of::<RagdollConfigList::ImpulseParam<u64>>(), 0x60);
        assert_eq!(size_of::<RagdollConfigList::BodyParam<u32>>(), 0x50);
        assert_eq!(size_of::<RagdollConfigList::BodyParam<u64>>(), 0x98);
        assert_eq!(size_of::<RagdollConfigList::RagdollConfigList<u32>>(), 0x2D4);
        assert_eq!(size_of::<RagdollConfigList::RagdollConfigList<u64>>(), 0x3D0);
    }

    #[test]
    fn recipe_size_tests() {
        assert_eq!(size_of::<Recipe::Item<u32>>(), 0x24);
        assert_eq!(size_of::<Recipe::Item<u64>>(), 0x48);
        assert_eq!(size_of::<Recipe::Table<u32>>(), 0x48);
        assert_eq!(size_of::<Recipe::Table<u64>>(), 0x88);
        assert_eq!(size_of::<Recipe::Recipe<u32>>(), 0x27C);
        assert_eq!(size_of::<Recipe::Recipe<u64>>(), 0x320);
    }

    #[test]
    fn shop_size_tests() {
        assert_eq!(size_of::<ShopData::Item<u32>>(), 0x64);
        assert_eq!(size_of::<ShopData::Item<u64>>(), 0xC8);
        assert_eq!(size_of::<ShopData::Table<u32>>(), 0x48);
        assert_eq!(size_of::<ShopData::Table<u64>>(), 0x88);
        assert_eq!(size_of::<ShopData::Shop<u32>>(), 0x27C);
        assert_eq!(size_of::<ShopData::Shop<u64>>(), 0x320);
    }

    #[test]
    fn supportbone_size_tests() {
        assert_eq!(size_of::<SupportBone::Bone<u32>>(), 0x74);
        assert_eq!(size_of::<SupportBone::Bone<u64>>(), 0xA0);
        assert_eq!(size_of::<SupportBone::ConnectionLinear<u32>>(), 0x40);
        assert_eq!(size_of::<SupportBone::ConnectionLinear<u64>>(), 0x70);
        assert_eq!(size_of::<SupportBone::ConnectionCurve<u32>>(), 0xA0);
        assert_eq!(size_of::<SupportBone::ConnectionCurve<u64>>(), 0x108);
        assert_eq!(size_of::<SupportBone::OutputSingle<u32>>(), 0x3C);
        assert_eq!(size_of::<SupportBone::OutputSingle<u64>>(), 0x70);
        assert_eq!(size_of::<SupportBone::OutputDouble<u32>>(), 0x5C);
        assert_eq!(size_of::<SupportBone::OutputDouble<u64>>(), 0xB0);
        assert_eq!(size_of::<SupportBone::BaseBone<u32>>(), 0xBC);
        assert_eq!(size_of::<SupportBone::BaseBone<u64>>(), 0x130);
        assert_eq!(size_of::<SupportBone::MainBone<u32>>(), 0xBC);
        assert_eq!(size_of::<SupportBone::MainBone<u64>>(), 0x130);
        assert_eq!(size_of::<SupportBone::SupportBone<u32>>(), 0x11C);
        assert_eq!(size_of::<SupportBone::SupportBone<u64>>(), 0x1F0);
        assert_eq!(size_of::<SupportBone::SupportBoneResource<u32>>(), 0x384);
        assert_eq!(size_of::<SupportBone::SupportBoneResource<u64>>(), 0x5B0);
    }
}