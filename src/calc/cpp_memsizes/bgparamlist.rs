use roead::aamp::ParameterIO;
use crate::Endian;

use std::mem::size_of;
use phf::{Map, phf_map};

use super::cpp_classes::GParamList::*;

const BGPARAM_OVERHEAD: u32 = 0x2c0;
static OBJ_SIZES_WIIU: Map<&'static str, u32> = phf_map! {
    "AirWall" => size_of::<GParamListObjectAirWall<u32>>() as u32,
    "AnimalFollowOffset" => size_of::<GParamListObjectAnimalFollowOffset<u32>>() as u32,
    "AnimalUnit" => size_of::<GParamListObjectAnimalUnit<u32>>() as u32,
    "Armor" => size_of::<GParamListObjectArmor<u32>>() as u32,
    "ArmorEffect" => size_of::<GParamListObjectArmorEffect<u32>>() as u32,
    "ArmorHead" => size_of::<GParamListObjectArmorHead<u32>>() as u32,
    "ArmorUpper" => size_of::<GParamListObjectArmorUpper<u32>>() as u32,
    "Arrow" => size_of::<GParamListObjectArrow<u32>>() as u32,
    "Attack" => size_of::<GParamListObjectAttack<u32>>() as u32,
    "AttackInterval" => size_of::<GParamListObjectAttackInterval<u32>>() as u32,
    "AutoGen" => size_of::<GParamListObjectAutoGen<u32>>() as u32,
    "Beam" => size_of::<GParamListObjectBeam<u32>>() as u32,
    "BindActor" => size_of::<GParamListObjectBindActor<u32>>() as u32,
    "BindBone" => size_of::<GParamListObjectBindBone<u32>>() as u32,
    "Bow" => size_of::<GParamListObjectBow<u32>>() as u32,
    "Bullet" => size_of::<GParamListObjectBullet<u32>>() as u32,
    "Camera" => size_of::<GParamListObjectCamera<u32>>() as u32,
    "ChemicalType" => size_of::<GParamListObjectChemicalType<u32>>() as u32,
    "ClothReaction" => size_of::<GParamListObjectClothReaction<u32>>() as u32,
    "CookSpice" => size_of::<GParamListObjectCookSpice<u32>>() as u32,
    "CureItem" => size_of::<GParamListObjectCureItem<u32>>() as u32,
    "EatTarget" => size_of::<GParamListObjectEatTarget<u32>>() as u32,
    "Enemy" => size_of::<GParamListObjectEnemy<u32>>() as u32,
    "EnemyLevel" => size_of::<GParamListObjectEnemyLevel<u32>>() as u32,
    "EnemyRace" => size_of::<GParamListObjectEnemyRace<u32>>() as u32,
    "EnemyShown" => size_of::<GParamListObjectEnemyShown<u32>>() as u32,
    "Event" => size_of::<GParamListObjectEvent<u32>>() as u32,
    "ExtendedEntity" => size_of::<GParamListObjectExtendedEntity<u32>>() as u32,
    "Fish" => size_of::<GParamListObjectFish<u32>>() as u32,
    "GelEnemy" => size_of::<GParamListObjectGelEnemy<u32>>() as u32,
    "General" => size_of::<GParamListObjectGeneral<u32>>() as u32,
    "GiantArmor" => size_of::<GParamListObjectGiantArmor<u32>>() as u32,
    "GiantArmorSlot" => size_of::<GParamListObjectGiantArmorSlot<u32>>() as u32,
    "Global" => size_of::<GParamListObjectGlobal<u32>>() as u32,
    "Golem" => size_of::<GParamListObjectGolem<u32>>() as u32,
    "GolemIK" => size_of::<GParamListObjectGolemIK<u32>>() as u32,
    "Grab" => size_of::<GParamListObjectGrab<u32>>() as u32,
    "Guardian" => size_of::<GParamListObjectGuardian<u32>>() as u32,
    "GuardianMini" => size_of::<GParamListObjectGuardianMini<u32>>() as u32,
    "GuardianMiniWeapon" => size_of::<GParamListObjectGuardianMiniWeapon<u32>>() as u32,
    "Horse" => size_of::<GParamListObjectHorse<u32>>() as u32,
    "HorseCreator" => size_of::<GParamListObjectHorseCreator<u32>>() as u32,
    "HorseObject" => size_of::<GParamListObjectHorseObject<u32>>() as u32,
    "HorseRider" => size_of::<GParamListObjectHorseRider<u32>>() as u32,
    "HorseTargetedInfo" => size_of::<GParamListObjectHorseTargetedInfo<u32>>() as u32,
    "HorseUnit" => size_of::<GParamListObjectHorseUnit<u32>>() as u32,
    "Insect" => size_of::<GParamListObjectInsect<u32>>() as u32,
    "Item" => size_of::<GParamListObjectItem<u32>>() as u32,
    "LargeSword" => size_of::<GParamListObjectLargeSword<u32>>() as u32,
    "Liftable" => size_of::<GParamListObjectLiftable<u32>>() as u32,
    "LumberjackTree" => size_of::<GParamListObjectLumberjackTree<u32>>() as u32,
    "MasterSword" => size_of::<GParamListObjectMasterSword<u32>>() as u32,
    "MonsterShop" => size_of::<GParamListObjectMonsterShop<u32>>() as u32,
    "Motorcycle" => size_of::<GParamListObjectMotorcycle<u32>>() as u32,
    "Nest" => size_of::<GParamListObjectNest<u32>>() as u32,
    "Npc" => size_of::<GParamListObjectNpc<u32>>() as u32,
    "NpcEquipment" => size_of::<GParamListObjectNpcEquipment<u32>>() as u32,
    "PictureBook" => size_of::<GParamListObjectPictureBook<u32>>() as u32,
    "Player" => size_of::<GParamListObjectPlayer<u32>>() as u32,
    "Prey" => size_of::<GParamListObjectPrey<u32>>() as u32,
    "Rod" => size_of::<GParamListObjectRod<u32>>() as u32,
    "Rope" => size_of::<GParamListObjectRope<u32>>() as u32,
    "Rupee" => size_of::<GParamListObjectRupee<u32>>() as u32,
    "Sandworm" => size_of::<GParamListObjectSandworm<u32>>() as u32,
    "SeriesArmor" => size_of::<GParamListObjectSeriesArmor<u32>>() as u32,
    "ShiekerStone" => size_of::<GParamListObjectShiekerStone<u32>>() as u32,
    "Shield" => size_of::<GParamListObjectShield<u32>>() as u32,
    "SmallSword" => size_of::<GParamListObjectSmallSword<u32>>() as u32,
    "Spear" => size_of::<GParamListObjectSpear<u32>>() as u32,
    "StalEnemy" => size_of::<GParamListObjectStalEnemy<u32>>() as u32,
    "Swarm" => size_of::<GParamListObjectSwarm<u32>>() as u32,
    "System" => size_of::<GParamListObjectSystem<u32>>() as u32,
    "Traveler" => size_of::<GParamListObjectTraveler<u32>>() as u32,
    "WeaponCommon" => size_of::<GParamListObjectWeaponCommon<u32>>() as u32,
    "WeaponOption" => size_of::<GParamListObjectWeaponOption<u32>>() as u32,
    "WeaponThrow" => size_of::<GParamListObjectWeaponThrow<u32>>() as u32,
    "WizzRobe" => size_of::<GParamListObjectWizzrobe<u32>>() as u32,
    "WolfLink" => size_of::<GParamListObjectWolfLink<u32>>() as u32,
    "Zora" => size_of::<GParamListObjectZora<u32>>() as u32,
};
static OBJ_SIZES_NX: Map<&'static str, u32> = phf_map! {
    "AirWall" => size_of::<GParamListObjectAirWall<u64>>() as u32,
    "AnimalFollowOffset" => size_of::<GParamListObjectAnimalFollowOffset<u64>>() as u32,
    "AnimalUnit" => size_of::<GParamListObjectAnimalUnit<u64>>() as u32,
    "Armor" => size_of::<GParamListObjectArmor<u64>>() as u32,
    "ArmorEffect" => size_of::<GParamListObjectArmorEffect<u64>>() as u32,
    "ArmorHead" => size_of::<GParamListObjectArmorHead<u64>>() as u32,
    "ArmorUpper" => size_of::<GParamListObjectArmorUpper<u64>>() as u32,
    "Arrow" => size_of::<GParamListObjectArrow<u64>>() as u32,
    "Attack" => size_of::<GParamListObjectAttack<u64>>() as u32,
    "AttackInterval" => size_of::<GParamListObjectAttackInterval<u64>>() as u32,
    "AutoGen" => size_of::<GParamListObjectAutoGen<u64>>() as u32,
    "Beam" => size_of::<GParamListObjectBeam<u64>>() as u32,
    "BindActor" => size_of::<GParamListObjectBindActor<u64>>() as u32,
    "BindBone" => size_of::<GParamListObjectBindBone<u64>>() as u32,
    "Bow" => size_of::<GParamListObjectBow<u64>>() as u32,
    "Bullet" => size_of::<GParamListObjectBullet<u64>>() as u32,
    "Camera" => size_of::<GParamListObjectCamera<u64>>() as u32,
    "ChemicalType" => size_of::<GParamListObjectChemicalType<u64>>() as u32,
    "ClothReaction" => size_of::<GParamListObjectClothReaction<u64>>() as u32,
    "CookSpice" => size_of::<GParamListObjectCookSpice<u64>>() as u32,
    "CureItem" => size_of::<GParamListObjectCureItem<u64>>() as u32,
    "EatTarget" => size_of::<GParamListObjectEatTarget<u64>>() as u32,
    "Enemy" => size_of::<GParamListObjectEnemy<u64>>() as u32,
    "EnemyLevel" => size_of::<GParamListObjectEnemyLevel<u64>>() as u32,
    "EnemyRace" => size_of::<GParamListObjectEnemyRace<u64>>() as u32,
    "EnemyShown" => size_of::<GParamListObjectEnemyShown<u64>>() as u32,
    "Event" => size_of::<GParamListObjectEvent<u64>>() as u32,
    "ExtendedEntity" => size_of::<GParamListObjectExtendedEntity<u64>>() as u32,
    "Fish" => size_of::<GParamListObjectFish<u64>>() as u32,
    "GelEnemy" => size_of::<GParamListObjectGelEnemy<u64>>() as u32,
    "General" => size_of::<GParamListObjectGeneral<u64>>() as u32,
    "GiantArmor" => size_of::<GParamListObjectGiantArmor<u64>>() as u32,
    "GiantArmorSlot" => size_of::<GParamListObjectGiantArmorSlot<u64>>() as u32,
    "Global" => size_of::<GParamListObjectGlobal<u64>>() as u32,
    "Golem" => size_of::<GParamListObjectGolem<u64>>() as u32,
    "GolemIK" => size_of::<GParamListObjectGolemIK<u64>>() as u32,
    "Grab" => size_of::<GParamListObjectGrab<u64>>() as u32,
    "Guardian" => size_of::<GParamListObjectGuardian<u64>>() as u32,
    "GuardianMini" => size_of::<GParamListObjectGuardianMini<u64>>() as u32,
    "GuardianMiniWeapon" => size_of::<GParamListObjectGuardianMiniWeapon<u64>>() as u32,
    "Horse" => size_of::<GParamListObjectHorse<u64>>() as u32,
    "HorseCreator" => size_of::<GParamListObjectHorseCreator<u64>>() as u32,
    "HorseObject" => size_of::<GParamListObjectHorseObject<u64>>() as u32,
    "HorseRider" => size_of::<GParamListObjectHorseRider<u64>>() as u32,
    "HorseTargetedInfo" => size_of::<GParamListObjectHorseTargetedInfo<u64>>() as u32,
    "HorseUnit" => size_of::<GParamListObjectHorseUnit<u64>>() as u32,
    "Insect" => size_of::<GParamListObjectInsect<u64>>() as u32,
    "Item" => size_of::<GParamListObjectItem<u64>>() as u32,
    "LargeSword" => size_of::<GParamListObjectLargeSword<u64>>() as u32,
    "Liftable" => size_of::<GParamListObjectLiftable<u64>>() as u32,
    "LumberjackTree" => size_of::<GParamListObjectLumberjackTree<u64>>() as u32,
    "MasterSword" => size_of::<GParamListObjectMasterSword<u64>>() as u32,
    "MonsterShop" => size_of::<GParamListObjectMonsterShop<u64>>() as u32,
    "Motorcycle" => size_of::<GParamListObjectMotorcycle<u64>>() as u32,
    "Nest" => size_of::<GParamListObjectNest<u64>>() as u32,
    "Npc" => size_of::<GParamListObjectNpc<u64>>() as u32,
    "NpcEquipment" => size_of::<GParamListObjectNpcEquipment<u64>>() as u32,
    "PictureBook" => size_of::<GParamListObjectPictureBook<u64>>() as u32,
    "Player" => size_of::<GParamListObjectPlayer<u64>>() as u32,
    "Prey" => size_of::<GParamListObjectPrey<u64>>() as u32,
    "Rod" => size_of::<GParamListObjectRod<u64>>() as u32,
    "Rope" => size_of::<GParamListObjectRope<u64>>() as u32,
    "Rupee" => size_of::<GParamListObjectRupee<u64>>() as u32,
    "Sandworm" => size_of::<GParamListObjectSandworm<u64>>() as u32,
    "SeriesArmor" => size_of::<GParamListObjectSeriesArmor<u64>>() as u32,
    "ShiekerStone" => size_of::<GParamListObjectShiekerStone<u64>>() as u32,
    "Shield" => size_of::<GParamListObjectShield<u64>>() as u32,
    "SmallSword" => size_of::<GParamListObjectSmallSword<u64>>() as u32,
    "Spear" => size_of::<GParamListObjectSpear<u64>>() as u32,
    "StalEnemy" => size_of::<GParamListObjectStalEnemy<u64>>() as u32,
    "Swarm" => size_of::<GParamListObjectSwarm<u64>>() as u32,
    "System" => size_of::<GParamListObjectSystem<u64>>() as u32,
    "Traveler" => size_of::<GParamListObjectTraveler<u64>>() as u32,
    "WeaponCommon" => size_of::<GParamListObjectWeaponCommon<u64>>() as u32,
    "WeaponOption" => size_of::<GParamListObjectWeaponOption<u64>>() as u32,
    "WeaponThrow" => size_of::<GParamListObjectWeaponThrow<u64>>() as u32,
    "WizzRobe" => size_of::<GParamListObjectWizzrobe<u64>>() as u32,
    "WolfLink" => size_of::<GParamListObjectWolfLink<u64>>() as u32,
    "Zora" => size_of::<GParamListObjectZora<u64>>() as u32,
};

pub fn parse_size(bytes: &[u8], endian: Endian) -> u32 {
    let a = ParameterIO::from_binary(bytes).unwrap();
    let mut total_size = BGPARAM_OVERHEAD;
    let obj_map: &Map<&'static str, u32> = match endian {
        Endian::Big => &OBJ_SIZES_WIIU,
        Endian::Little => &OBJ_SIZES_NX,
    };
    for (name, size) in (*obj_map).into_iter() {
        if let Some(_) = a.param_root.objects.get(*name) {
            total_size += size;
        }
    }
    total_size
}
