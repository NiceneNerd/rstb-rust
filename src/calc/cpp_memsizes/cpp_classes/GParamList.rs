use super::*;

#[repr(C)]
pub struct GParamList<T> {
    base:       ParamIO<T>,         // ParamIO
    base2:      Resource<T>,        // Resource
    mObjects:   sead::Buffer<T>,    // sead::Buffer<GParamListObject*>
}

#[repr(C)]
pub struct GParamListObj<T> {
    vfptr: T,
    mObj:  agl::ParameterObj<T>,
}

#[repr(C)]
pub struct GParamListObjectAirWall<T> {
    base:   GParamListObj<T>,
    mLayer: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectAnimalFollowOffset<T> {
    base:   GParamListObj<T>,
    mEatLocalOffset: agl::Parameter<T, sead::Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectAnimalUnit<T> {
    base: GParamListObj<T>,
    mSpeedActorName: agl::Parameter<T, sead::SafeString<T>>,
    mBasePlayRate: agl::Parameter<T, f32>,
    mGearMaxNum: agl::Parameter<T, i32>,
    mIsSetWaitASAtGear0: agl::Parameter<T, bool>,
    mStressFramesMin: agl::Parameter<T, f32>,
    mStressFramesMax: agl::Parameter<T, f32>,
    mSteeringOutputKp: agl::Parameter<T, f32>,
    mSteeringOutputKi: agl::Parameter<T, f32>,
    mSteeringOutputKd: agl::Parameter<T, f32>,
    mSteeringOutputIClamp: agl::Parameter<T, f32>,
    mSteeringOutputIReduceRatio: agl::Parameter<T, f32>,
    mSteeringOutputDLerpRatio: agl::Parameter<T, f32>,
    mSteeringOutputAvoidanceLerpRatio: agl::Parameter<T, f32>,
    mSteeringOutputIIRLerpRatio: agl::Parameter<T, f32>,
    mOverrideSteeringOutputKp: agl::Parameter<T, f32>,
    mOverrideSteeringOutputKi: agl::Parameter<T, f32>,
    mOverrideSteeringOutputKd: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectArmor<T> {
    base: GParamListObj<T>,
    mStarNum: agl::Parameter<T, i32>,
    mDefenceAddLevel: agl::Parameter<T, i32>,
    mWindScaleMesh: agl::Parameter<T, sead::SafeString<T>>,
    mWindScale: agl::Parameter<T, f32>,
    mNextRankName: agl::Parameter<T, sead::SafeString<T>>,
    mAffectTransOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mAffectRotOffsetShield: agl::Parameter<T, sead::Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectArmorEffect<T> {
    base: GParamListObj<T>,
    mEffectType: agl::Parameter<T, sead::SafeString<T>>,
    mEffectLevel: agl::Parameter<T, i32>,
    mAncientPowUp: agl::Parameter<T, bool>,
    mEnableClimbWaterfall: agl::Parameter<T, bool>,
    mEnableSpinAttack: agl::Parameter<T, bool>,
}

#[repr(C)]
pub struct GParamListObjectArmorHead<T> {
    base: GParamListObj<T>,
    mEarRotate: agl::Parameter<T, sead::Vector3f>,
    mMantleType: agl::Parameter<T, i32>,
    mMaskType: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectArmorUpper<T> {
    base: GParamListObj<T>,
    mIsDispOffPorch: agl::Parameter<T, bool>,
    mShiekerStoneTransOffset: agl::Parameter<T, sead::Vector3f>,
    mShiekerStoneRotOffset: agl::Parameter<T, sead::Vector3f>,
    mDisableSelfMantle: agl::Parameter<T, bool>,
    mUseMantleType: agl::Parameter<T, i32>,
}

#[repr(C)]
pub struct GParamListObjectArrow<T> {
    base: GParamListObj<T>,
    mArrowNum: agl::Parameter<T, i32>,
    mDeleteTime: agl::Parameter<T, i32>,
    mDeleteTimeWithChemical: agl::Parameter<T, i32>,
    mEnemyShootNumForDelete: agl::Parameter<T, i32>,
    mArrowDeletePer: agl::Parameter<T, i32>,
    mExtraDamage: agl::Parameter<T, i32>,
}

#[repr(C)]
pub struct GParamListObjectAttack<T> {
    base: GParamListObj<T>,
    mPower: agl::Parameter<T, i32>,
    mImpulse: agl::Parameter<T, i32>,
    mImpulseLarge: agl::Parameter<T, i32>,
    mRange: agl::Parameter<T, f32>,
    mGuardBreakPower: agl::Parameter<T, i32>,
    mSpHitActor: agl::Parameter<T, sead::SafeString<T>>,
    mSpHitTag: agl::Parameter<T, sead::SafeString<T>>,
    mSpHitRatio: agl::Parameter<T, f32>,
    mSpWeakHitActor: agl::Parameter<T, sead::SafeString<T>>,
    mPowerForPlayer: agl::Parameter<T, i32>,
}

#[repr(C)]
pub struct GParamListObjectAttackInterval<T> {
    base: GParamListObj<T>,
    mShortAttackTimeMin: agl::Parameter<T, i32>,
    mShortAttackTimeMax: agl::Parameter<T, i32>,
    mMiddleAttackTimeMin: agl::Parameter<T, i32>,
    mMiddleAttackTimeMax: agl::Parameter<T, i32>,
    mLongAttackTimeMin: agl::Parameter<T, i32>,
    mLongAttackTimeMax: agl::Parameter<T, i32>,
}

#[repr(C)]
pub struct GParamListObjectAutoGen<T> {
    base: GParamListObj<T>,
    mSetName: agl::Parameter<T, sead::SafeString<T>>,
    mKeyActorName: agl::Parameter<T, sead::SafeString<T>>,
    mSetRadius: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectBeam<T> {
    base: GParamListObj<T>,
    mBeamLevel: agl::Parameter<T, i32>,
}

#[repr(C)]
pub struct GParamListObjectBindActor<T> {
    base: GParamListObj<T>,
    mBindActorName: agl::Parameter<T, sead::SafeString<T>>,
    mIsKeepSleep: agl::Parameter<T, bool>,
}

#[repr(C)]
pub struct GParamListObjectBindBone<T> {
    base: GParamListObj<T>,
    mBoneName: agl::Parameter<T, sead::SafeString<T>>,
    mBoneOffset: agl::Parameter<T, sead::Vector3f>,
    mBoneRotate: agl::Parameter<T, sead::Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectBow<T> {
    base: GParamListObj<T>,
    mQuiverName: agl::Parameter<T, sead::SafeString<T>>,
    mArrowName: agl::Parameter<T, sead::SafeString<T>>,
    mIsGuardPierce: agl::Parameter<T, bool>,
    mExtraDamageRatio: agl::Parameter<T, f32>,
    mBaseAttackPowerRatio: agl::Parameter<T, f32>,
    mIsLeadShot: agl::Parameter<T, bool>,
    mLeadShotNum: agl::Parameter<T, i32>,
    mLeadShotAng: agl::Parameter<T, f32>,
    mLeadShotInterval: agl::Parameter<T, i32>,
    mIsRapidFire: agl::Parameter<T, bool>,
    mRapidFireNum: agl::Parameter<T, i32>,
    mRapidFireInterval: agl::Parameter<T, i32>,
    mIsLongRange: agl::Parameter<T, bool>,
    mArrowFirstSpeed: agl::Parameter<T, f32>,
    mArrowAcceleration: agl::Parameter<T, f32>,
    mArrowStabilitySpeed: agl::Parameter<T, f32>,
    mArrowFallAcceleration: agl::Parameter<T, f32>,
    mArrowFallStabilitySpeed: agl::Parameter<T, f32>,
    mArrowGravity: agl::Parameter<T, f32>,
    mPlayerHoldTransOffset: agl::Parameter<T, sead::Vector3f>,
    mPlayerHoldRotOffset: agl::Parameter<T, sead::Vector3f>,
    mPlayerEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mPlayerEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mSquatPlayerHoldTransAddOffset: agl::Parameter<T, sead::Vector3f>,
    mSquatPlayerHoldRotAddOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCHoldTransOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCHoldRotOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mEnemyEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mEnemyEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mStandEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mStandEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mArrowChargeRate: agl::Parameter<T, f32>,
    mArrowReloadRate: agl::Parameter<T, f32>,
    mWeaponSubType: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectBullet<T> {
    base: GParamListObj<T>,
    mNoHitParent: agl::Parameter<T, bool>,
    mIsLimitCount: agl::Parameter<T, bool>,
}

#[repr(C)]
pub struct GParamListObjectCamera<T> {
    base: GParamListObj<T>,
    mDefaultConnectScaleAfterEvent: agl::Parameter<T, f32>,
    mLatConnectRateAfterEvent: agl::Parameter<T, f32>,
    mLngConnectRateAfterEvent: agl::Parameter<T, f32>,
    mDistConnectRateAfterEvent: agl::Parameter<T, f32>,
    mFovyConnectRateAfterEvent: agl::Parameter<T, f32>,
    mConnectAfterEventMin: agl::Parameter<T, f32>,
    mConnectAfterEventMax: agl::Parameter<T, f32>,
    mRoofGradientNearHighWeight: agl::Parameter<T, f32>,
    mRoofGradientFarHighWeight: agl::Parameter<T, f32>,
    mRoofGradientNearLowWeight: agl::Parameter<T, f32>,
    mRoofGradientFarLowWeight: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectChemicalType<T> {
    base: GParamListObj<T>,
    mChemicalType: agl::Parameter<T, sead::SafeString<T>>,
    mEmitChemicalActor: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectClothReaction<T> {
    base: GParamListObj<T>,
    mAtkCollidableName: agl::Parameter<T, sead::SafeString<T>>,
    mAtkCollidableBoneName: agl::Parameter<T, sead::SafeString<T>>,
    mAtkCollidableSpeed: agl::Parameter<T, f32>,
    mAtkCollidableActiveTime: agl::Parameter<T, f32>,
    mAtkCollidableResetPos: agl::Parameter<T, sead::Vector3f>,
    mGroundCollidableName: agl::Parameter<T, sead::SafeString<T>>,
    mGroundCollidableBoneName: agl::Parameter<T, sead::SafeString<T>>,
    mGroundCollidableOffset: agl::Parameter<T, f32>,
    mUseGroundCollidableResetPos: agl::Parameter<T, bool>,
    mGroundCollidableResetPos: agl::Parameter<T, sead::Vector3f>,
    mGroundCollidableMoveSpeed: agl::Parameter<T, f32>,
    mWallCollidableName: agl::Parameter<T, sead::SafeString<T>>,
    mWallCollidableBoneName: agl::Parameter<T, sead::SafeString<T>>,
    mWallCollidableOffset: agl::Parameter<T, f32>,
    mPlayerCollidableName: agl::Parameter<T, sead::SafeString<T>>,
    mPlayerCollidableBoneName: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectCookSpice<T> {
    base: GParamListObj<T>,
    mBoostHitPointRecover: agl::Parameter<T, i32>,
    mBoostEffectiveTime: agl::Parameter<T, i32>,
    mBoostSuccessRate: agl::Parameter<T, i32>,
    mBoostMaxHeartLevel: agl::Parameter<T, i32>,
    mBoostStaminaLevel: agl::Parameter<T, i32>,
}

#[repr(C)]
pub struct GParamListObjectCureItem<T> {
    base: GParamListObj<T>,
    mHitPointRecover: agl::Parameter<T, i32>,
    mEffectType: agl::Parameter<T, sead::SafeString<T>>,
    mEffectLevel: agl::Parameter<T, i32>,
    mEffectiveTime: agl::Parameter<T, i32>,
}

#[repr(C)]
pub struct GParamListObjectEatTarget<T> {
    base: GParamListObj<T>,
    mFavoriteEatActorNames: agl::Parameter<T, sead::SafeString<T>>,
    mFavoriteEatActorTags: agl::Parameter<T, sead::SafeString<T>>,
    mEatActorNames: agl::Parameter<T, sead::SafeString<T>>,
    mEatActorNames2: agl::Parameter<T, sead::SafeString<T>>,
    mEatActorNames3: agl::Parameter<T, sead::SafeString<T>>,
    mEatActorTags: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectEnemy<T> {
    base: GParamListObj<T>,
    mRank: agl::Parameter<T, i32>,
    mPower: agl::Parameter<T, i32>,
    mDropLife: agl::Parameter<T, i32>,
    mDyingLife: agl::Parameter<T, i32>,
    mLostDist: agl::Parameter<T, f32>,
    mLostHeightMax: agl::Parameter<T, f32>,
    mLostHeightMin: agl::Parameter<T, f32>,
    mLostRayLength: agl::Parameter<T, f32>,
    mLODLostDist: agl::Parameter<T, f32>,
    mLODLostHeightMax: agl::Parameter<T, f32>,
    mLODLostHeightMin: agl::Parameter<T, f32>,
    mIntelligenceLevel: agl::Parameter<T, f32>,
    mEmotionalLevel: agl::Parameter<T, f32>,
    mHeroismLevel: agl::Parameter<T, f32>,
    mPartActorName: agl::Parameter<T, sead::SafeString<T>>,
    mIsMindFriend: agl::Parameter<T, bool>,
    mStatusChangeFlag: agl::Parameter<T, sead::SafeString<T>>,
    mChangeLife: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectEnemyLevel<T> {
    base: GParamListObj<T>,
    mIsAvoidDanger: agl::Parameter<T, bool>,
    mIsGuardArrow: agl::Parameter<T, bool>,
    mIsHideArrowAttack: agl::Parameter<T, bool>,
    mIsSwiftAttack: agl::Parameter<T, bool>,
    mIsBackSwiftAttack: agl::Parameter<T, bool>,
    mIsCounterAttack: agl::Parameter<T, bool>,
    mIsEscapeBomb: agl::Parameter<T, bool>,
    mIsKickBomb: agl::Parameter<T, bool>,
    mIsShootBomb: agl::Parameter<T, bool>,
    mIsThrowWeapon: agl::Parameter<T, bool>,
    mGuardPer: agl::Parameter<T, i32>,
    mIsJustGuard: agl::Parameter<T, bool>,
}

#[repr(C)]
pub struct GParamListObjectEnemyRace<T> {
    base: GParamListObj<T>,
    mEquipableWeapon: agl::Parameter<T, sead::SafeString<T>>,
    mIsFitGroundByAnimalUnit: agl::Parameter<T, bool>,
    mIsUpdateSupportNormalInAir: agl::Parameter<T, bool>,
    mBowAttackRangeRatio: agl::Parameter<T, f32>,
    mWeaponScaleSmallSword: agl::Parameter<T, f32>,
    mWeaponTransOffsetSmallSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponRotOffsetSmallSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldTransOffsetSmallSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldRotOffsetSmallSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponScaleLargeSword: agl::Parameter<T, f32>,
    mWeaponTransOffsetLargeSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponRotOffsetLargeSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldTransOffsetLargeSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldRotOffsetLargeSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponScaleSpear: agl::Parameter<T, f32>,
    mWeaponTransOffsetSpear: agl::Parameter<T, sead::Vector3f>,
    mWeaponRotOffsetSpear: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldTransOffsetSpear: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldRotOffsetSpear: agl::Parameter<T, sead::Vector3f>,
    mWeaponScaleBow: agl::Parameter<T, f32>,
    mWeaponTransOffsetBow: agl::Parameter<T, sead::Vector3f>,
    mWeaponRotOffsetBow: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldTransOffsetBow: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldRotOffsetBow: agl::Parameter<T, sead::Vector3f>,
    mWeaponScaleShield: agl::Parameter<T, f32>,
    mWeaponTransOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mWeaponRotOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldTransOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldRotOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mIsUseTargetTag: agl::Parameter<T, bool>,
    mTargetActorType: agl::Parameter<T, sead::SafeString<T>>,
    mEscapeAttackedActorType: agl::Parameter<T, sead::SafeString<T>>,
    mReactionBalloon: agl::Parameter<T, bool>,
    mSmallRagdollTime: agl::Parameter<T, i32>,
    mSmallRagdollRecoverTime: agl::Parameter<T, i32>,
    mSmallLargeRagdollTime: agl::Parameter<T, i32>,
    mSmallLargeRagdollRecoverTime: agl::Parameter<T, i32>,
}

#[repr(C)]
pub struct GParamListObjectEnemyShown<T> {
    base: GParamListObj<T>,
    mIsHappy: agl::Parameter<T, bool>,
    mIsCasebyCase: agl::Parameter<T, bool>,
    mIsSit: agl::Parameter<T, bool>,
    mIsNoise: agl::Parameter<T, bool>,
}

#[repr(C)]
pub struct GParamListObjectEvent<T> {
    base: GParamListObj<T>,
    mVisibleOffActor1: agl::Parameter<T, sead::SafeString<T>>,
    mVisibleOffActor2: agl::Parameter<T, sead::SafeString<T>>,
    mVisibleOffActor3: agl::Parameter<T, sead::SafeString<T>>,
    mVisibleOffActor4: agl::Parameter<T, sead::SafeString<T>>,
    mVisibleOffActor5: agl::Parameter<T, sead::SafeString<T>>,
    mVisibleOffActor6: agl::Parameter<T, sead::SafeString<T>>,
    mVisibleOffActor7: agl::Parameter<T, sead::SafeString<T>>,
    mVisibleOffActor8: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectExtendedEntity<T> {
    base: GParamListObj<T>,
    mIsUsePivotAdjustRange: agl::Parameter<T, bool>,
    mPivotAdjustRange: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectFish<T> {
    base: GParamListObj<T>,
    mRestoreSpeedRate: agl::Parameter<T, f32>,
    mRestoreSpeedRateAdd: agl::Parameter<T, f32>,
    mLimitAngle: agl::Parameter<T, f32>,
    mLimitAngleAdd: agl::Parameter<T, f32>,
    mPrevSpeedRate: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectGelEnemy<T> {
    base: GParamListObj<T>,
    mMoveBoneName: agl::Parameter<T, sead::SafeString<T>>,
    mBodyRadius: agl::Parameter<T, f32>,
    mClothBoneNumForEyeCalc: agl::Parameter<T, i32>,
    mBodyRootBoneName: agl::Parameter<T, sead::SafeString<T>>,
    mLeftEyeBoneName: agl::Parameter<T, sead::SafeString<T>>,
    mRightEyeBoneName: agl::Parameter<T, sead::SafeString<T>>,
    mEyeSpaceHalf: agl::Parameter<T, f32>,
    mEyeDir: agl::Parameter<T, sead::Vector3f>,
    mEyeOffset: agl::Parameter<T, sead::Vector3f>,
    mEyeUpMoveRate: agl::Parameter<T, f32>,
    mEyeDownMoveRate: agl::Parameter<T, f32>,
    mIsAverageEyePos: agl::Parameter<T, bool>,
    mEyeDelayAccRate: agl::Parameter<T, f32>,
    mEyeYMoveTheta: agl::Parameter<T, f32>,
    mEyeYMoveFrequency: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectGeneral<T> {
    base: GParamListObj<T>,
    mSpeed: agl::Parameter<T, f32>,
    mLife: agl::Parameter<T, i32>,
    mIsLifeInfinite: agl::Parameter<T, bool>,
    mElectricalDischarge: agl::Parameter<T, f32>,
    mIsBurnOutBorn: agl::Parameter<T, bool>,
    mBurnOutBornName: agl::Parameter<T, sead::SafeString<T>>,
    mIsBurnOutBornIdent: agl::Parameter<T, bool>,
    mChangeDropTableName: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectGiantArmor<T> {
    base: GParamListObj<T>,
    mDamageScale: agl::Parameter<T, f32>,
    mRotOffset: agl::Parameter<T, sead::Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectGiantArmorSlot<T> {
    base: GParamListObj<T>,
    mSlot0Node: agl::Parameter<T, sead::SafeString<T>>,
    mSlot0RigidBody: agl::Parameter<T, sead::SafeString<T>>,
    mSlot0DefaultActorName: agl::Parameter<T, sead::SafeString<T>>,
    mSlot1Node: agl::Parameter<T, sead::SafeString<T>>,
    mSlot1RigidBody: agl::Parameter<T, sead::SafeString<T>>,
    mSlot1DefaultActorName: agl::Parameter<T, sead::SafeString<T>>,
    mSlot2Node: agl::Parameter<T, sead::SafeString<T>>,
    mSlot2RigidBody: agl::Parameter<T, sead::SafeString<T>>,
    mSlot2DefaultActorName: agl::Parameter<T, sead::SafeString<T>>,
    mSlot3Node: agl::Parameter<T, sead::SafeString<T>>,
    mSlot3RigidBody: agl::Parameter<T, sead::SafeString<T>>,
    mSlot3DefaultActorName: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectGlobal<T> {
    base: GParamListObj<T>,
    mEnemyLifeGageDist: agl::Parameter<T, f32>,
    mEnemyNoSkitDist: agl::Parameter<T, f32>,
    mEnemyWeaponPickAllowDist: agl::Parameter<T, f32>,
    mEnemyWeaponPickForbidTime: agl::Parameter<T, i32>,
    mEnemyAnimalNoDamageDist: agl::Parameter<T, f32>,
    mEnemyNearCraeteIDDelay: agl::Parameter<T, f32>,
    mEnemyForceTiredLODCount: agl::Parameter<T, i32>,
    mEnemyForceTiredNoSightLODCount: agl::Parameter<T, i32>,
    mEnemyForceWarpReturnLODCount: agl::Parameter<T, i32>,
    mSilentAttackAng: agl::Parameter<T, f32>,
    mSilentAttackRatio: agl::Parameter<T, f32>,
    mBlownOffPlayerAtkDelay: agl::Parameter<T, i32>,
    mJustAvoidAcceptWpRangeSS: agl::Parameter<T, f32>,
    mJustAvoidAcceptWpRangeLS: agl::Parameter<T, f32>,
    mJustAvoidAcceptWpRangeSP: agl::Parameter<T, f32>,
    mForceNoticeEnemyCount: agl::Parameter<T, i32>,
    mForceNoticeEnemyDist: agl::Parameter<T, f32>,
    mWeaponRickeyLife: agl::Parameter<T, i32>,
    mWeaponDropRotSpd: agl::Parameter<T, f32>,
    mShieldRideBaseFrame: agl::Parameter<T, i32>,
    mShieldRideHitBaseDamage: agl::Parameter<T, i32>,
    mShieldDamageratio: agl::Parameter<T, f32>,
    mShieldSurfMasterFrictionRatio: agl::Parameter<T, f32>,
    mLoudNoiseRadius: agl::Parameter<T, f32>,
    mImpulse2DamageRatio: agl::Parameter<T, f32>,
    mIceMeltSpeedOnContactFire: agl::Parameter<T, f32>,
    mCriticalAttackRatio: agl::Parameter<T, f32>,
    mBooerangAttackRatio: agl::Parameter<T, f32>,
    mHitImpulseClampMax: agl::Parameter<T, f32>,
    mDropItemVelXZFromBomb: agl::Parameter<T, f32>,
    mDropItemVelYFromBomb: agl::Parameter<T, f32>,
    mDropItemVelRandomFromBomb: agl::Parameter<T, f32>,
    mDropItemAngVelFromBomb: agl::Parameter<T, f32>,
    mDropItemAngVelRandomFromBomb: agl::Parameter<T, f32>,
    mDropItemVelXZSmall: agl::Parameter<T, f32>,
    mDropItemVelYSmall: agl::Parameter<T, f32>,
    mDropItemVelRandomSmall: agl::Parameter<T, f32>,
    mDropItemAngVelSmall: agl::Parameter<T, f32>,
    mDropItemAngVelRandomSmall: agl::Parameter<T, f32>,
    mDropItemVelXZLarge: agl::Parameter<T, f32>,
    mDropItemVelYLarge: agl::Parameter<T, f32>,
    mDropItemVelRandomLarge: agl::Parameter<T, f32>,
    mDropItemAngVelLarge: agl::Parameter<T, f32>,
    mDropItemAngVelRandomLarge: agl::Parameter<T, f32>,
    mDropItemVelXZRupeeRabbit: agl::Parameter<T, f32>,
    mDropItemVelYRupeeRabbit: agl::Parameter<T, f32>,
    mDropItemVelRandomRupeeRabbit: agl::Parameter<T, f32>,
    mDropItemVelXZItemRupeeOnly: agl::Parameter<T, f32>,
    mDropItemVelYItemRupeeOnly: agl::Parameter<T, f32>,
    mDropItemVelRandomItemRupeeOnly: agl::Parameter<T, f32>,
    mDropItemInvincibleTime: agl::Parameter<T, f32>,
    mTreeWeaponEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mTreeWeaponEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mWetRatioToDie: agl::Parameter<T, f32>,
    mEnvWetRatioToDie: agl::Parameter<T, f32>,
    mNPCTurnAngleDiff: agl::Parameter<T, f32>,
    mNPCWaitFrameAfterEvent: agl::Parameter<T, i32>,
    mNPCIgnorePlayerTime: agl::Parameter<T, f32>,
    mNPCCancelIgnorePlayerTime: agl::Parameter<T, f32>,
    mNPCOpenDoorDistance: agl::Parameter<T, f32>,
    mNPCWalkRateOnSandAndSnow: agl::Parameter<T, f32>,
    mNPCDownVerticallyAngle: agl::Parameter<T, f32>,
    mGerudoQueenSafetyAreaRadius: agl::Parameter<T, f32>,
    mCreateFairyLimitCount: agl::Parameter<T, i32>,
    mTerrorRegistSpeed: agl::Parameter<T, f32>,
    mTerrorUnregistSpeed: agl::Parameter<T, f32>,
    mTerrorRegistTimer: agl::Parameter<T, i32>,
    mTerrorRadiusOffset: agl::Parameter<T, f32>,
    mSpeedTerrorLevel: agl::Parameter<T, i32>,
    mSpeedTerrorLevelHuge: agl::Parameter<T, i32>,
    mSpeedTerrorLevelCheckRadius: agl::Parameter<T, f32>,
    mAtDirTypeAffectRatio: agl::Parameter<T, f32>,
    mRainyAwnHearingLevel: agl::Parameter<T, f32>,
    mHorseBindOffsetYOfMaleUMii: agl::Parameter<T, f32>,
    mHorseBindOffsetYOfFemaleUMii: agl::Parameter<T, f32>,
    mHorseFamiliarityIncreasePerFrame: agl::Parameter<T, sead::Vector3f>,
    mHorseFamiliarityIncreaseSootheAtFirstRun: agl::Parameter<T, sead::Vector3f>,
    mHorseFamiliarityIncreaseSootheAfterRun: agl::Parameter<T, sead::Vector3f>,
    mHorseFamiliarityIncreaseSootheAfterGearTop: agl::Parameter<T, sead::Vector3f>,
    mHorseFamiliarityIncreaseSootheAfterJump: agl::Parameter<T, sead::Vector3f>,
    mHorseFamiliarityIncreaseSootheWhileResisting: agl::Parameter<T, sead::Vector3f>,
    mHorseFamiliarityIncreaseEat: agl::Parameter<T, sead::Vector3f>,
    mHorseAlertProbability: agl::Parameter<T, sead::Vector3f>,
    mHorseAlertFramesMin: agl::Parameter<T, sead::Vector3f>,
    mHorseAlertFramesMax: agl::Parameter<T, sead::Vector3f>,
    mHorseExtraChargeNum: agl::Parameter<T, i32>,
    mPlayerGrabThrowDiffRate: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectGolem<T> {
    base: GParamListObj<T>,
    mUpperArmRActor: agl::Parameter<T, sead::SafeString<T>>,
    mLowerArmRActor: agl::Parameter<T, sead::SafeString<T>>,
    mUpperArmLActor: agl::Parameter<T, sead::SafeString<T>>,
    mLowerArmLActor: agl::Parameter<T, sead::SafeString<T>>,
    mDefaultWeakPointActor: agl::Parameter<T, sead::SafeString<T>>,
    mIsDefaultChemicalOn: agl::Parameter<T, bool>,
}

#[repr(C)]
pub struct GParamListObjectGolemIK<T> {
    base: GParamListObj<T>,
    mFootRayCheckDist: agl::Parameter<T, f32>,
    mFootDownRatio: agl::Parameter<T, f32>,
    mFootUpRatio: agl::Parameter<T, f32>,
    mKneeExtendL: agl::Parameter<T, f32>,
    mKneeShrinkL: agl::Parameter<T, f32>,
    mFootExtendL: agl::Parameter<T, f32>,
    mFootShrinkL: agl::Parameter<T, f32>,
    mKneeExtendR: agl::Parameter<T, f32>,
    mKneeShrinkR: agl::Parameter<T, f32>,
    mFootExtendR: agl::Parameter<T, f32>,
    mFootShrinkR: agl::Parameter<T, f32>,
    mArmRayCheckDist: agl::Parameter<T, f32>,
    mArmDownRatio: agl::Parameter<T, f32>,
    mArmUpRatio: agl::Parameter<T, f32>,
    mElbowExtendL: agl::Parameter<T, f32>,
    mElbowShrinkL: agl::Parameter<T, f32>,
    mWristExtendL: agl::Parameter<T, f32>,
    mWristShrinkL: agl::Parameter<T, f32>,
    mElbowExtendR: agl::Parameter<T, f32>,
    mElbowShrinkR: agl::Parameter<T, f32>,
    mWristExtendR: agl::Parameter<T, f32>,
    mWristShrinkR: agl::Parameter<T, f32>,
    mWaistRotateRatio: agl::Parameter<T, f32>,
    mWaistMorphRatio: agl::Parameter<T, f32>,
    mWaistResetMorphRatio: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectGrab<T> {
    base: GParamListObj<T>,
    mSlot0Node: agl::Parameter<T, sead::SafeString<T>>,
    mSlot1Node: agl::Parameter<T, sead::SafeString<T>>,
    mSlot2Node: agl::Parameter<T, sead::SafeString<T>>,
    mSlot3Node: agl::Parameter<T, sead::SafeString<T>>,
    mSlot4Node: agl::Parameter<T, sead::SafeString<T>>,
    mSlot5Node: agl::Parameter<T, sead::SafeString<T>>,
    mSlot0PodNode: agl::Parameter<T, sead::SafeString<T>>,
    mSlot1PodNode: agl::Parameter<T, sead::SafeString<T>>,
    mSlot2PodNode: agl::Parameter<T, sead::SafeString<T>>,
    mSlot3PodNode: agl::Parameter<T, sead::SafeString<T>>,
    mSlot4PodNode: agl::Parameter<T, sead::SafeString<T>>,
    mSlot5PodNode: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectGuardian<T> {
    base: GParamListObj<T>,
    mGuardianModelType: agl::Parameter<T, i32>,
    mGuardianControllerType: agl::Parameter<T, i32>,
    mHeadLimitMax: agl::Parameter<T, f32>,
    mHeadLimitMin: agl::Parameter<T, f32>,
    mSightLimitMax: agl::Parameter<T, f32>,
    mSightLimitMin: agl::Parameter<T, f32>,
    mMaxSpeed: agl::Parameter<T, f32>,
    mCannonBoneName: agl::Parameter<T, sead::SafeString<T>>,
    mRapidFireDistance: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectGuardianMini<T> {
    base: GParamListObj<T>,
    mColorType: agl::Parameter<T, i32>,
    mBodyMatName: agl::Parameter<T, sead::SafeString<T>>,
    mGuardJustActor: agl::Parameter<T, sead::SafeString<T>>,
    mBeamName: agl::Parameter<T, sead::SafeString<T>>,
    mLineBeamName: agl::Parameter<T, sead::SafeString<T>>,
    mFinalBeamName: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectGuardianMiniWeapon<T> {
    base: GParamListObj<T>,
    mBindMyNodeName: agl::Parameter<T, sead::SafeString<T>>,
    mVisibleMatNameR: agl::Parameter<T, sead::SafeString<T>>,
    mVisibleMatNameL: agl::Parameter<T, sead::SafeString<T>>,
    mVisibleMatNameB: agl::Parameter<T, sead::SafeString<T>>,
    mVisibleOffMatName: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectHorse<T> {
    base: GParamListObj<T>,
    mIsDecoy: agl::Parameter<T, bool>,
    mHasMane: agl::Parameter<T, bool>,
    mASVariation: agl::Parameter<T, sead::SafeString<T>>,
    mNature: agl::Parameter<T, i32>,
    mAttackPowerMultiplierGear2: agl::Parameter<T, f32>,
    mAttackPowerMultiplierGear3: agl::Parameter<T, f32>,
    mAttackPowerMultiplierGearTop: agl::Parameter<T, f32>,
    mRunnableFramesAtGearTop: agl::Parameter<T, f32>,
    mGearTopInterval: agl::Parameter<T, f32>,
    mGearTopChargeNum: agl::Parameter<T, i32>,
    mEatActorNames: agl::Parameter<T, sead::SafeString<T>>,
    mEatActorNamesForExtraCharge: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectHorseCreator<T> {
    base: GParamListObj<T>,
    mHorseNames: agl::Parameter<T, sead::SafeString<T>>,
    mLeaderHorseNames: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectHorseObject<T> {
    base: GParamListObj<T>,
    mHideHorseMane: agl::Parameter<T, bool>,
    mIsHorseClothDisable: agl::Parameter<T, bool>,
}

#[repr(C)]
pub struct GParamListObjectHorseRider<T> {
    base: GParamListObj<T>,
    mRootNode: agl::Parameter<T, sead::SafeString<T>>,
    mSpineNode: agl::Parameter<T, sead::SafeString<T>>,
    mRotBaseNode: agl::Parameter<T, sead::SafeString<T>>,
    mRotAxis: agl::Parameter<T, sead::Vector3f>,
    mRotLimit: agl::Parameter<T, f32>,
    mWeaponTransOffsetSmallSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponTransOffsetLargeSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponTransOffsetSpear: agl::Parameter<T, sead::Vector3f>,
    mWeaponTransOffsetBow: agl::Parameter<T, sead::Vector3f>,
    mWeaponTransOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mLeftFootNode: agl::Parameter<T, sead::SafeString<T>>,
    mLeftFootRotAxis: agl::Parameter<T, sead::Vector3f>,
    mRightFootNode: agl::Parameter<T, sead::SafeString<T>>,
    mRightFootRotAxis: agl::Parameter<T, sead::Vector3f>,
    mFootRotRatio: agl::Parameter<T, f32>,
    mFootRetRotRatio: agl::Parameter<T, f32>,
    mFootRotAngleForKuma: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectHorseTargetedInfo<T> {
    base: GParamListObj<T>,
    mHorseMoveRadius: agl::Parameter<T, f32>,
    mHorseAvoidOffset: agl::Parameter<T, f32>,
    mIsCircularMoveAlways: agl::Parameter<T, bool>,
}

#[repr(C)]
pub struct GParamListObjectHorseUnit<T> {
    base: GParamListObj<T>,
    mRiddenAnimalType: agl::Parameter<T, i32>,
    mCalmDownNum: agl::Parameter<T, i32>,
    mRideonAboveASHeight: agl::Parameter<T, f32>,
    mRideonAboveASRadius: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectInsect<T> {
    base: GParamListObj<T>,
    mFireResistanceLevel: agl::Parameter<T, i32>,
}

#[repr(C)]
pub struct GParamListObjectItem<T> {
    base: GParamListObj<T>,
    mPlayerUseItem: agl::Parameter<T, bool>,
    mSellingPrice: agl::Parameter<T, i32>,
    mBuyingPrice: agl::Parameter<T, i32>,
    mCreatingPrice: agl::Parameter<T, i32>,
    mStainColor: agl::Parameter<T, i32>,
    mSaleRevivalCount: agl::Parameter<T, i32>,
    mUseIconActorName: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectLargeSword<T> {
    base: GParamListObj<T>,
    mPodName: agl::Parameter<T, sead::SafeString<T>>,
    mPlayerHoldTransOffset: agl::Parameter<T, sead::Vector3f>,
    mPlayerHoldRotOffset: agl::Parameter<T, sead::Vector3f>,
    mPlayerEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mPlayerEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mRideHorsePlayerHoldTransOffset: agl::Parameter<T, sead::Vector3f>,
    mRideHorsePlayerHoldRotOffset: agl::Parameter<T, sead::Vector3f>,
    mAffectTransOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mAffectRotOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mAffectTransOffsetBow: agl::Parameter<T, sead::Vector3f>,
    mAffectRotOffsetBow: agl::Parameter<T, sead::Vector3f>,
    mSquatPlayerHoldTransAddOffset: agl::Parameter<T, sead::Vector3f>,
    mSquatPlayerHoldRotAddOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCHoldTransOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCHoldRotOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mEnemyEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mEnemyEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mStandEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mStandEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mWeaponSubType: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectLiftable<T> {
    base: GParamListObj<T>,
    mLiftType: agl::Parameter<T, sead::SafeString<T>>,
    mThrownMass: agl::Parameter<T, i32>,
    mThrownRotSpd: agl::Parameter<T, sead::Vector3f>,
    mLiftPosOffset: agl::Parameter<T, sead::Vector3f>,
    mLiftRotOffset: agl::Parameter<T, sead::Vector3f>,
    mLiftRotFrame: agl::Parameter<T, i32>,
    mAddLiftRotOffsetList: agl::Parameter<T, sead::SafeString<T>>,
    mChaseLiftRotOffset: agl::Parameter<T, bool>,
    mLiftCenterOffset: agl::Parameter<T, sead::Vector3f>,
    mPutPosOffset: agl::Parameter<T, sead::Vector3f>,
    mPutRotOffset: agl::Parameter<T, sead::Vector3f>,
    mPutRotFrame: agl::Parameter<T, i32>,
    mAddPutRotOffsetList: agl::Parameter<T, sead::SafeString<T>>,
    mIsUpdateOffsetEachFrame: agl::Parameter<T, bool>,
    mIsUse2MassConstraintMode: agl::Parameter<T, bool>,
    mIsSetChemicalParent: agl::Parameter<T, bool>,
    mDisableFreezeLift: agl::Parameter<T, bool>,
    mDisableBurnLift: agl::Parameter<T, bool>,
    mThrowReactionLevel: agl::Parameter<T, i32>,
}

#[repr(C)]
pub struct GParamListObjectLumberjackTree<T> {
    base: GParamListObj<T>,
    mImpulseThreshold: agl::Parameter<T, f32>,
    mIsValid: agl::Parameter<T, bool>,
    mStumpName: agl::Parameter<T, sead::SafeString<T>>,
    mTrunkName: agl::Parameter<T, sead::SafeString<T>>,
    mWeaponWoodName: agl::Parameter<T, sead::SafeString<T>>,
    mBranchName: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectMasterSword<T> {
    base: GParamListObj<T>,
    mIsMasterSword: agl::Parameter<T, bool>,
    mTrueFormAttackPower: agl::Parameter<T, i32>,
    mTrueFormMagicPower: agl::Parameter<T, i32>,
    mTrueFormBreakRatio: agl::Parameter<T, f32>,
    mSearchEvilDist: agl::Parameter<T, f32>,
    mRecoverTime: agl::Parameter<T, i32>,
    mSleepActorName: agl::Parameter<T, sead::SafeString<T>>,
    mTrueFormActorName: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectMonsterShop<T> {
    base: GParamListObj<T>,
    mBuyMamo: agl::Parameter<T, i32>,
    mSellMamo: agl::Parameter<T, i32>,
}

#[repr(C)]
pub struct GParamListObjectMotorcycle<T> {
    base: GParamListObj<T>,
    mPitchDampingCoefficient: agl::Parameter<T, f32>,
    mDriftAllowSpeedKPH: agl::Parameter<T, f32>,
    mDriftAbortSpeedKPH: agl::Parameter<T, f32>,
    mDriftAllowSteerRate: agl::Parameter<T, f32>,
    mDriftAbortSteerRate: agl::Parameter<T, f32>,
    mDriftRearAngleRate: agl::Parameter<T, f32>,
    mDriftSpeedRate: agl::Parameter<T, f32>,
    mManualWheelieAllowAngleFront: agl::Parameter<T, f32>,
    mManualWheelieAllowAngleRear: agl::Parameter<T, f32>,
    mManualWheelieLastSec: agl::Parameter<T, f32>,
    mWheelieLastSecInMidAir: agl::Parameter<T, f32>,
    mManualControlProhibitSecAfterWheelie: agl::Parameter<T, f32>,
    mWheelieRevertPower: agl::Parameter<T, f32>,
    mWheelieRevertPowerSec: agl::Parameter<T, f32>,
    mManualWheelieRiseDegDelta: agl::Parameter<T, f32>,
    mWheelieLaunchRiseDegDelta: agl::Parameter<T, f32>,
    mEngineBrakeMaxPower: agl::Parameter<T, f32>,
    mBackwardEngineBrakePower: agl::Parameter<T, f32>,
    mSlipStartAngle: agl::Parameter<T, f32>,
    mSlipThresholdPower: agl::Parameter<T, f32>,
    mSlipPowerMax: agl::Parameter<T, f32>,
    mWristBindRotation: agl::Parameter<T, sead::Vector3f>,
    mWristBindTranslation: agl::Parameter<T, sead::Vector3f>,
    mPostureLimitAngle: agl::Parameter<T, f32>,
    mInvalidPostureLimitSec: agl::Parameter<T, f32>,
    mFallOverThresholdAngle: agl::Parameter<T, f32>,
    mJumpIntervalSec: agl::Parameter<T, f32>,
    mFullEnergyLastSec: agl::Parameter<T, f32>,
    mWheelieLaunchJumpProhibitSec: agl::Parameter<T, f32>,
    mSlowModeTargetSpeedKPH2: agl::Parameter<T, f32>,
    mSlowDriftTargetSpeedKPH2: agl::Parameter<T, f32>,
    mSlowModeTransitionSec: agl::Parameter<T, f32>,
    mSlowSlipThresholdKPH: agl::Parameter<T, f32>,
    mSlowSlipThresholdPower: agl::Parameter<T, f32>,
    mSlowSlipThresholdSec: agl::Parameter<T, f32>,
    mJumpRearWheelRotateRadPerSec: agl::Parameter<T, f32>,
    mWeaponThrowModeSpeedKPH2: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectNest<T> {
    base: GParamListObj<T>,
    mCreateActor: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectNpc<T> {
    base: GParamListObj<T>,
    mIsReactNakedPlayer: agl::Parameter<T, bool>,
    mUseAutoLabel: agl::Parameter<T, bool>,
    mIsOffPodFromWeapon: agl::Parameter<T, bool>,
    mIsRunRainWhenGoToSleep: agl::Parameter<T, bool>,
    mIsWalkUnderShelterFromRain: agl::Parameter<T, bool>,
    mIsSlowWalkOnSandAndSnow: agl::Parameter<T, bool>,
    mIsAlwaysCounterPlayerAttack: agl::Parameter<T, bool>,
    mIsNotTurnDetect: agl::Parameter<T, bool>,
    mIsNotEscapeFromTerror: agl::Parameter<T, bool>,
    mTolerantTime: agl::Parameter<T, i32>,
    mTolerantCount: agl::Parameter<T, i32>,
    mCounterRate: agl::Parameter<T, i32>,
    mChangeSearchModeFlagName: agl::Parameter<T, sead::SafeString<T>>,
    mOnFlagWhenDelete: agl::Parameter<T, sead::SafeString<T>>,
    mOffFlagWhenDelete: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectNpcEquipment<T> {
    base: GParamListObj<T>,
    mIsSetWeaponTypeWhenEquip: agl::Parameter<T, bool>,
    mEquipName1: agl::Parameter<T, sead::SafeString<T>>,
    mScale1: agl::Parameter<T, f32>,
    mHoldTransOffset1: agl::Parameter<T, sead::Vector3f>,
    mHoldRotOffset1: agl::Parameter<T, sead::Vector3f>,
    mEquipTransOffset1: agl::Parameter<T, sead::Vector3f>,
    mEquipRotOffset1: agl::Parameter<T, sead::Vector3f>,
    mEquipName2: agl::Parameter<T, sead::SafeString<T>>,
    mScale2: agl::Parameter<T, f32>,
    mHoldTransOffset2: agl::Parameter<T, sead::Vector3f>,
    mHoldRotOffset2: agl::Parameter<T, sead::Vector3f>,
    mEquipTransOffset2: agl::Parameter<T, sead::Vector3f>,
    mEquipRotOffset2: agl::Parameter<T, sead::Vector3f>,
    mEquipName3: agl::Parameter<T, sead::SafeString<T>>,
    mScale3: agl::Parameter<T, f32>,
    mHoldTransOffset3: agl::Parameter<T, sead::Vector3f>,
    mHoldRotOffset3: agl::Parameter<T, sead::Vector3f>,
    mEquipTransOffset3: agl::Parameter<T, sead::Vector3f>,
    mEquipRotOffset3: agl::Parameter<T, sead::Vector3f>,
    mEquipName4: agl::Parameter<T, sead::SafeString<T>>,
    mScale4: agl::Parameter<T, f32>,
    mHoldTransOffset4: agl::Parameter<T, sead::Vector3f>,
    mHoldRotOffset4: agl::Parameter<T, sead::Vector3f>,
    mEquipTransOffset4: agl::Parameter<T, sead::Vector3f>,
    mEquipRotOffset4: agl::Parameter<T, sead::Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectPictureBook<T> {
    base: GParamListObj<T>,
    mLiveSpot1: agl::Parameter<T, i32>,
    mLiveSpot2: agl::Parameter<T, i32>,
    mSpecialDrop: agl::Parameter<T, i32>,
}

#[repr(C)]
pub struct GParamListObjectPlayer<T> {
    base: GParamListObj<T>,
    mBombReloadTime1: agl::Parameter<T, f32>,
    mBombReloadTime2: agl::Parameter<T, f32>,
    mStopTimerReloadTime: agl::Parameter<T, f32>,
    mStopTimerBlowAngle: agl::Parameter<T, f32>,
    mStopTimerBlowSpeedLimit: agl::Parameter<T, f32>,
    mStopTimerImpluseMaxCountSmallSword: agl::Parameter<T, i32>,
    mStopTimerImpluseMaxCountLargeSword: agl::Parameter<T, i32>,
    mStopTimerImpluseMaxCountSpear: agl::Parameter<T, i32>,
    mStopTimerCancelDeleteWaitTime: agl::Parameter<T, f32>,
    mStopTimerLongTime: agl::Parameter<T, f32>,
    mStopTimerMiddleTime: agl::Parameter<T, f32>,
    mStopTimerShortTime: agl::Parameter<T, f32>,
    mEnergyTiredValue: agl::Parameter<T, f32>,
    mEnergyBowSlow: agl::Parameter<T, f32>,
    mEnergyPush: agl::Parameter<T, f32>,
    mEnergyCharge: agl::Parameter<T, f32>,
    mEnergyAutoRecover: agl::Parameter<T, f32>,
    mEnergyAutoRecoverInAir: agl::Parameter<T, f32>,
    mEnergyAutoRecoverInvalidTime1: agl::Parameter<T, f32>,
    mEnergyAutoRecoverInvalidTime2: agl::Parameter<T, f32>,
    mColdTempDamageAdd: agl::Parameter<T, f32>,
    mHotTempDamageAdd: agl::Parameter<T, f32>,
    mTempDamage: agl::Parameter<T, f32>,
    mTempEnergyDecDiamAdd: agl::Parameter<T, f32>,
    mTempEnergyDecDegAdd: agl::Parameter<T, f32>,
    mVelDiamSand: agl::Parameter<T, f32>,
    mVelDiamTired: agl::Parameter<T, f32>,
    mStickDiamTired: agl::Parameter<T, f32>,
    mAutoRecoverNum: agl::Parameter<T, f32>,
    mAutoRecoverIntervalMin: agl::Parameter<T, f32>,
    mAutoRecoverIntervalMax: agl::Parameter<T, f32>,
    mAutoRecoverInvalidTime: agl::Parameter<T, f32>,
    mBowSubjectContTime: agl::Parameter<T, f32>,
    mLNGStickScale: agl::Parameter<T, f32>,
    mLATStickScale: agl::Parameter<T, f32>,
    mLNGGyroScale: agl::Parameter<T, f32>,
    mLATGyroScale: agl::Parameter<T, f32>,
    mBowSlowShootNum: agl::Parameter<T, i32>,
    mBowSlowRateDiam: agl::Parameter<T, f32>,
    mBowSlowMaxTime: agl::Parameter<T, f32>,
    mDiveBowSlowMaxTime: agl::Parameter<T, f32>,
    mBowSlowInvalidTime: agl::Parameter<T, f32>,
    mBowSlowInvalidHeight: agl::Parameter<T, f32>,
    mBowSlowInvalidHeightOnShield: agl::Parameter<T, f32>,
    mBowSlowInvalidHeightWeaponChange: agl::Parameter<T, f32>,
    mGuardJustForceSlowTime: agl::Parameter<T, f32>,
    mMoveMaxDecRateByWater: agl::Parameter<T, f32>,
    mMoveIgnoreWaterHeight: agl::Parameter<T, f32>,
    mMoveDecRateByBog: agl::Parameter<T, f32>,
    mMoveDecRateMaxHeight: agl::Parameter<T, f32>,
    mMaxForce: agl::Parameter<T, f32>,
    mMinForce: agl::Parameter<T, f32>,
    mAddForce: agl::Parameter<T, f32>,
    mSnowBallAddForce: agl::Parameter<T, f32>,
    mLogPushF: agl::Parameter<T, f32>,
    mRockPushF: agl::Parameter<T, f32>,
    mRockPushSpeed: agl::Parameter<T, f32>,
    mWaistAngleUpperMax: agl::Parameter<T, f32>,
    mWaistAngleLowerMax: agl::Parameter<T, f32>,
    mWaistAngleSideMax: agl::Parameter<T, f32>,
    mNoSquatWaterHeight: agl::Parameter<T, f32>,
    mInvalidReloadTime: agl::Parameter<T, f32>,
    mWeaponThrowSpeedY: agl::Parameter<T, f32>,
    mWeaponThrowSpeedF: agl::Parameter<T, f32>,
    mWeaponThrowSpeedFSquat: agl::Parameter<T, f32>,
    mDashUpEnableAngle: agl::Parameter<T, f32>,
    mShockTime: agl::Parameter<T, f32>,
    mIceInvalidTime: agl::Parameter<T, f32>,
    mMaxSpeedInAir: agl::Parameter<T, f32>,
    mTurnEnableSpeed: agl::Parameter<T, f32>,
    mTurnEnableStickSub: agl::Parameter<T, f32>,
    mTurnEnableDirSub: agl::Parameter<T, f32>,
    mShortDashImpulse: agl::Parameter<T, i32>,
    mShortDashDamage: agl::Parameter<T, i32>,
    mSwordTerrorScope: agl::Parameter<T, f32>,
    mArrowTerrorScope: agl::Parameter<T, f32>,
    mTorchTerrorScope: agl::Parameter<T, f32>,
    mTorchTerrorOffsetY: agl::Parameter<T, f32>,
    mTorchTerrorOffsetZ: agl::Parameter<T, f32>,
    mDashNoise: agl::Parameter<T, f32>,
    mWhistleNoise: agl::Parameter<T, f32>,
    mClimbEnableAngle: agl::Parameter<T, f32>,
    mClimbEnableSpeedMinAngle: agl::Parameter<T, f32>,
    mClimbEnableSpeedMaxAngle: agl::Parameter<T, f32>,
    mSlipEnableSpeed: agl::Parameter<T, f32>,
    mSlipSpeedAddMin: agl::Parameter<T, f32>,
    mSlipSpeedAddMax: agl::Parameter<T, f32>,
    mSlipSpeedAddDiamByRain: agl::Parameter<T, f32>,
    mMagnetAim2DPosOffsetY: agl::Parameter<T, f32>,
    mLookPosOffsetXZ: agl::Parameter<T, f32>,
    mLookPosOffsetY: agl::Parameter<T, f32>,
    mLookPosOffsetYSquat: agl::Parameter<T, f32>,
    mLookPosOffsetYSwim: agl::Parameter<T, f32>,
    mLookPosOffsetYHorse: agl::Parameter<T, f32>,
    mLookEnableAngle: agl::Parameter<T, f32>,
    mHitSlowTimeS: agl::Parameter<T, f32>,
    mHitSlowTimeM: agl::Parameter<T, f32>,
    mHitSlowTimeL: agl::Parameter<T, f32>,
    mHitSlowRate: agl::Parameter<T, f32>,
    mHitStopTimeS: agl::Parameter<T, f32>,
    mHitStopTimeL: agl::Parameter<T, f32>,
    mHitStopRate: agl::Parameter<T, f32>,
    mAtnPosInterPolationRate: agl::Parameter<T, f32>,
    mAtnPosInterPolationMin: agl::Parameter<T, f32>,
    mAtnPosInterPolationMax: agl::Parameter<T, f32>,
    mPredictDiffAngleMax: agl::Parameter<T, f32>,
    mDashToRunStickValueDec: agl::Parameter<T, f32>,
    mWindSupportReuseTime: agl::Parameter<T, f32>,
    mFireSupportReuseTime: agl::Parameter<T, f32>,
    mElectricSupportReuseTime: agl::Parameter<T, f32>,
    mWaterSupportReuseTime: agl::Parameter<T, f32>,
    mWindSupportTimerRate: agl::Parameter<T, f32>,
    mFireSupportTimerRate: agl::Parameter<T, f32>,
    mElectricSupportTimerRate: agl::Parameter<T, f32>,
    mWaterSupportTimerRate: agl::Parameter<T, f32>,
    mChemicalInvalidTime: agl::Parameter<T, f32>,
    mAutoDashUpTime: agl::Parameter<T, f32>,
    mAutoDashUpAngle: agl::Parameter<T, f32>,
    mClimbRestartHeight: agl::Parameter<T, f32>,
    mClimbRestartTime: agl::Parameter<T, f32>,
    mPushNoticeLookTime: agl::Parameter<T, f32>,
    mEnergyUseSmall: agl::Parameter<T, f32>,
    mEnergyUseLarge: agl::Parameter<T, f32>,
    mNoEnergyDashInterval: agl::Parameter<T, f32>,
    mGuardableAngle: agl::Parameter<T, f32>,
    mStickMaxInStore: agl::Parameter<T, f32>,
    mLookContinueTime: agl::Parameter<T, f32>,
    mPostureContinueTime: agl::Parameter<T, f32>,
    mItemUseModelAlpha: agl::Parameter<T, f32>,
    mLadderCheckSide: agl::Parameter<T, f32>,
    mLadderCheckDist: agl::Parameter<T, f32>,
    mNoDeathDamageBase: agl::Parameter<T, i32>,
    mNoDeathDamageAdd: agl::Parameter<T, i32>,
    mArmorCompSwimEnergyRate: agl::Parameter<T, f32>,
    mArmorCompRegistElecFrame: agl::Parameter<T, f32>,
    mArmorCompNightSpeedRate: agl::Parameter<T, f32>,
    mArmorCompClimbJumpEnergyRate: agl::Parameter<T, f32>,
    mArmorCompPlusDropRate: agl::Parameter<T, f32>,
    mArmorCompWeaponBrakeRate: agl::Parameter<T, f32>,
    mArmorCompSwordBeamAttackRate: agl::Parameter<T, f32>,
    mArmorCompAncientAttackRate: agl::Parameter<T, f32>,
    mArmorCompBoneAttackRate: agl::Parameter<T, f32>,
    mArmorCompTerrorLevel: agl::Parameter<T, f32>,
    mArmorCompTerrorRadius: agl::Parameter<T, f32>,
    mArmorCompNakedSwimSpeedRate: agl::Parameter<T, f32>,
    mArmorCompNakedSwimAnimeRate: agl::Parameter<T, f32>,
    mArmorCompNakedSwimEnergyRate: agl::Parameter<T, f32>,
    mArmorAncientAttackRate: agl::Parameter<T, f32>,
    mSupportWindNum: agl::Parameter<T, i32>,
    mSupportElectricNum: agl::Parameter<T, i32>,
    mSupportElectricEnergy: agl::Parameter<T, f32>,
    mSupportFireNum: agl::Parameter<T, i32>,
    mSupportWaterLifeAdd: agl::Parameter<T, i32>,
    mSupportWaterEnergyAdd: agl::Parameter<T, f32>,
    mStickRInputFrame: agl::Parameter<T, f32>,
    mDiffAngleFromLookVec: agl::Parameter<T, f32>,
    mLookPosOffset: agl::Parameter<T, f32>,
    mLookFixAngle: agl::Parameter<T, f32>,
    mLookContinueTimeToCamera: agl::Parameter<T, f32>,
    mCutKnockBackNoCrrDist: agl::Parameter<T, f32>,
    mWaitUnsteadyApplyVel: agl::Parameter<T, f32>,
    mCurseAddWeight: agl::Parameter<T, f32>,
    mRoofCrashVel: agl::Parameter<T, f32>,
    mCutJumpInvalidTime: agl::Parameter<T, f32>,
    mWaterDepthInGrudge: agl::Parameter<T, f32>,
    mLargeHorseLegBendAngY: agl::Parameter<T, f32>,
    mLargeHorseLegBendAngX: agl::Parameter<T, f32>,
    mLargeHorseLegBendFrame: agl::Parameter<T, f32>,
    mNoMaskPauseWaterHeight: agl::Parameter<T, f32>,
    mLookAtThreshold: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectPrey<T> {
    base: GParamListObj<T>,
    mEatActorFindRadius: agl::Parameter<T, f32>,
    mEatActorFindRotDegree: agl::Parameter<T, f32>,
    mWaitTimeForStartEat: agl::Parameter<T, f32>,
    mIsEnableGroupEscape: agl::Parameter<T, bool>,
    mAimEscapeOffsetRate: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectRod<T> {
    base: GParamListObj<T>,
    mMagicName: agl::Parameter<T, sead::SafeString<T>>,
    mChargeMagicNum: agl::Parameter<T, i32>,
    mChargeMagicInterval: agl::Parameter<T, i32>,
    mMagicPower: agl::Parameter<T, i32>,
    mMagicSpeed: agl::Parameter<T, f32>,
    mMagicSpeedByThrow: agl::Parameter<T, f32>,
    mMagicGravity: agl::Parameter<T, f32>,
    mMagicRadius: agl::Parameter<T, f32>,
    mScaleTime: agl::Parameter<T, i32>,
    mMagicRange: agl::Parameter<T, f32>,
    mMagicSpeedByEnemy: agl::Parameter<T, f32>,
    mMagicGravityByEnemy: agl::Parameter<T, f32>,
    mMagicRadiusByEnemy: agl::Parameter<T, f32>,
    mScaleTimeByEnemy: agl::Parameter<T, i32>,
    mMagicRangeByEnemy: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectRope<T> {
    base: GParamListObj<T>,
    mIsAllowCutting: agl::Parameter<T, bool>,
    mIsSetupKeyframed: agl::Parameter<T, bool>,
    mBoneEffectiveLength: agl::Parameter<T, f32>,
    mIsInterpolateEdge: agl::Parameter<T, bool>,
    mIsDeformable: agl::Parameter<T, bool>,
    mIsOneBoneOneShape: agl::Parameter<T, bool>,
    mSplineOffsetRateA: agl::Parameter<T, f32>,
    mSplineOffsetRateB: agl::Parameter<T, f32>,
    mSplineOffsetRateC: agl::Parameter<T, f32>,
    mMtxEndPosOffsetLength: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectRupee<T> {
    base: GParamListObj<T>,
    mRupeeValue: agl::Parameter<T, i32>,
}

#[repr(C)]
pub struct GParamListObjectSandworm<T> {
    base: GParamListObj<T>,
    mSandWidth: agl::Parameter<T, f32>,
    mSandLength: agl::Parameter<T, f32>,
    mSandHeight: agl::Parameter<T, f32>,
    mSandCombSpan: agl::Parameter<T, f32>,
    mSandCombHeight: agl::Parameter<T, f32>,
    mSnakeModelOffsetZ: agl::Parameter<T, f32>,
    mSnakeBaseNode: agl::Parameter<T, sead::SafeString<T>>,
    mSnakeNode1: agl::Parameter<T, sead::SafeString<T>>,
    mSnakeNode2: agl::Parameter<T, sead::SafeString<T>>,
    mSnakeNode3: agl::Parameter<T, sead::SafeString<T>>,
    mSnakeNode4: agl::Parameter<T, sead::SafeString<T>>,
    mSnakeNode5: agl::Parameter<T, sead::SafeString<T>>,
    mSnakeNode6: agl::Parameter<T, sead::SafeString<T>>,
    mSnakeNode7: agl::Parameter<T, sead::SafeString<T>>,
    mSnakeNode8: agl::Parameter<T, sead::SafeString<T>>,
    mSnakeNode9: agl::Parameter<T, sead::SafeString<T>>,
    mSnakeNode10: agl::Parameter<T, sead::SafeString<T>>,
    mSnakeNode11: agl::Parameter<T, sead::SafeString<T>>,
    mSnakeNode12: agl::Parameter<T, sead::SafeString<T>>,
    mSnakeNodeRotOffset: agl::Parameter<T, sead::Vector3f>,
    mSnakeNodeChaseInterval: agl::Parameter<T, f32>,
    mShowLifeGageDist: agl::Parameter<T, f32>,
    mShowLifeGageOffset: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectSeriesArmor<T> {
    base: GParamListObj<T>,
    mSeriesType: agl::Parameter<T, sead::SafeString<T>>,
    mEnableCompBonus: agl::Parameter<T, bool>,
}

#[repr(C)]
pub struct GParamListObjectShiekerStone<T> {
    base: GParamListObj<T>,
    mNodeNameWithWaist: agl::Parameter<T, sead::SafeString<T>>,
    mTransOffsetWithWaist: agl::Parameter<T, sead::Vector3f>,
    mRotOffsetWithWaist: agl::Parameter<T, sead::Vector3f>,
    mNodeNameWithHand: agl::Parameter<T, sead::SafeString<T>>,
    mTransOffsetWithHand: agl::Parameter<T, sead::Vector3f>,
    mRotOffsetWithHand: agl::Parameter<T, sead::Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectShield<T> {
    base: GParamListObj<T>,
    mPlayerHoldTransOffset: agl::Parameter<T, sead::Vector3f>,
    mPlayerHoldRotOffset: agl::Parameter<T, sead::Vector3f>,
    mPlayerEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mPlayerEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mSquatPlayerHoldTransAddOffset: agl::Parameter<T, sead::Vector3f>,
    mSquatPlayerHoldRotAddOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCHoldTransOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCHoldRotOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mEnemyEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mEnemyEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mStandEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mStandEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mRideBreakRatio: agl::Parameter<T, f32>,
    mMirrorLevel: agl::Parameter<T, i32>,
    mWeaponSubType: agl::Parameter<T, sead::SafeString<T>>,
    mSurfingFriction: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectSmallSword<T> {
    base: GParamListObj<T>,
    mPodName: agl::Parameter<T, sead::SafeString<T>>,
    mPlayerHoldTransOffset: agl::Parameter<T, sead::Vector3f>,
    mPlayerHoldRotOffset: agl::Parameter<T, sead::Vector3f>,
    mPlayerEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mPlayerEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mRideHorsePlayerHoldTransOffset: agl::Parameter<T, sead::Vector3f>,
    mRideHorsePlayerHoldRotOffset: agl::Parameter<T, sead::Vector3f>,
    mAffectTransOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mAffectRotOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mAffectTransOffsetBow: agl::Parameter<T, sead::Vector3f>,
    mAffectRotOffsetBow: agl::Parameter<T, sead::Vector3f>,
    mSquatPlayerHoldTransAddOffset: agl::Parameter<T, sead::Vector3f>,
    mSquatPlayerHoldRotAddOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCHoldTransOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCHoldRotOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mEnemyEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mEnemyEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mStandEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mStandEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mWeaponSubType: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectSpear<T> {
    base: GParamListObj<T>,
    mPodName: agl::Parameter<T, sead::SafeString<T>>,
    mPlayerHoldTransOffset: agl::Parameter<T, sead::Vector3f>,
    mPlayerHoldRotOffset: agl::Parameter<T, sead::Vector3f>,
    mPlayerEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mPlayerEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mRideHorsePlayerHoldTransOffset: agl::Parameter<T, sead::Vector3f>,
    mRideHorsePlayerHoldRotOffset: agl::Parameter<T, sead::Vector3f>,
    mAffectTransOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mAffectRotOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mAffectTransOffsetBow: agl::Parameter<T, sead::Vector3f>,
    mAffectRotOffsetBow: agl::Parameter<T, sead::Vector3f>,
    mGrabPlayerHoldTransOffset: agl::Parameter<T, sead::Vector3f>,
    mGrabPlayerHoldRotOffset: agl::Parameter<T, sead::Vector3f>,
    mGrabAffectTransOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mGrabAffectRotOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mSquatPlayerHoldTransAddOffset: agl::Parameter<T, sead::Vector3f>,
    mSquatPlayerHoldRotAddOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCHoldTransOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCHoldRotOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mEnemyEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mEnemyEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mStandEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mStandEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mWeaponSubType: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectStalEnemy<T> {
    base: GParamListObj<T>,
    mHeadActorName: agl::Parameter<T, sead::SafeString<T>>,
    mLeftArmActorName: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectSwarm<T> {
    base: GParamListObj<T>,
    mSwarmSubActorNum: agl::Parameter<T, i32>,
    mSwarmPattern: agl::Parameter<T, i32>,
    mDeadActorName: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectSystem<T> {
    base: GParamListObj<T>,
    mSameGroupActorName: agl::Parameter<T, sead::SafeString<T>>,
    mIsGetItemSelf: agl::Parameter<T, bool>,
}

#[repr(C)]
pub struct GParamListObjectTraveler<T> {
    base: GParamListObj<T>,
    mAppearGameDataName: agl::Parameter<T, sead::SafeString<T>>,
    mDeleteGameDataName: agl::Parameter<T, sead::SafeString<T>>,
    mRouteType: agl::Parameter<T, sead::SafeString<T>>,
    mRideHorseName: agl::Parameter<T, sead::SafeString<T>>,
    mIsLeadHorse: agl::Parameter<T, bool>,
    mHorseGearLevel: agl::Parameter<T, i32>,
    mRoutePoints: RoutePoints<T>,
    mRoutePoint29Name: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectWeaponCommon<T> {
    base: GParamListObj<T>,
    mPlayerEqScale: agl::Parameter<T, f32>,
    mEnemyEqScale: agl::Parameter<T, f32>,
    mGuardPower: agl::Parameter<T, i32>,
    mRank: agl::Parameter<T, i32>,
    mIsHammer: agl::Parameter<T, bool>,
    mIsWeakBreaker: agl::Parameter<T, bool>,
    mIsBoomerang: agl::Parameter<T, bool>,
    mIsBlunt: agl::Parameter<T, bool>,
    mIsLuckyWeapon: agl::Parameter<T, bool>,
    mIsPikohan: agl::Parameter<T, bool>,
    mIsThrowingWeapon: agl::Parameter<T, bool>,
    mIsThrowingBreakWeapon: agl::Parameter<T, bool>,
    mThrowRange: agl::Parameter<T, f32>,
    mDreadActor: agl::Parameter<T, sead::SafeString<T>>,
    mThroughActor: agl::Parameter<T, sead::SafeString<T>>,
    mNPCWeaponType: agl::Parameter<T, sead::SafeString<T>>,
    mIsNotOnTerrorHold: agl::Parameter<T, bool>,
    mIsAsOffUnEquiped: agl::Parameter<T, bool>,
    mChemicalEnergyMax: agl::Parameter<T, i32>,
    mChemicalEnergyAmountUsed: agl::Parameter<T, i32>,
    mChemicalEnergyRecoverRate: agl::Parameter<T, f32>,
    mChemicalEnergyRecoverInterval: agl::Parameter<T, i32>,
    mStickDamage: agl::Parameter<T, i32>,
    mShootBeam: agl::Parameter<T, sead::SafeString<T>>,
    mDropFromPorchRot: agl::Parameter<T, sead::Vector3f>,
    mSharpWeaponPer: agl::Parameter<T, f32>,
    mSharpWeaponAddAtkMin: agl::Parameter<T, i32>,
    mSharpWeaponAddAtkMax: agl::Parameter<T, i32>,
    mSharpWeaponAddLifeMin: agl::Parameter<T, i32>,
    mSharpWeaponAddLifeMax: agl::Parameter<T, i32>,
    mSharpWeaponAddCrit: agl::Parameter<T, bool>,
    mSharpWeaponAddGuardMin: agl::Parameter<T, i32>,
    mSharpWeaponAddGuardMax: agl::Parameter<T, i32>,
    mPoweredSharpAddAtkMin: agl::Parameter<T, i32>,
    mPoweredSharpAddAtkMax: agl::Parameter<T, i32>,
    mPoweredSharpAddLifeMin: agl::Parameter<T, i32>,
    mPoweredSharpAddLifeMax: agl::Parameter<T, i32>,
    mPoweredSharpWeaponAddGuardMin: agl::Parameter<T, i32>,
    mPoweredSharpWeaponAddGuardMax: agl::Parameter<T, i32>,
    mPoweredSharpAddThrowMin: agl::Parameter<T, f32>,
    mPoweredSharpAddThrowMax: agl::Parameter<T, f32>,
    mPoweredSharpAddSpreadFire: agl::Parameter<T, bool>,
    mPoweredSharpAddZoomRapid: agl::Parameter<T, bool>,
    mPoweredSharpAddRapidFireMin: agl::Parameter<T, f32>,
    mPoweredSharpAddRapidFireMax: agl::Parameter<T, f32>,
    mPoweredSharpAddSurfMaster: agl::Parameter<T, bool>,
}

#[repr(C)]
pub struct GParamListObjectWeaponOption<T> {
    base: GParamListObj<T>,
    mPlayerHoldTransOffset: agl::Parameter<T, sead::Vector3f>,
    mPlayerHoldRotOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCHoldTransOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCHoldRotOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mNPCEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectWeaponThrow<T> {
    base: GParamListObj<T>,
    mThrowSpeed: agl::Parameter<T, f32>,
    mThrowRotSpeed: agl::Parameter<T, f32>,
    mThrowDist: agl::Parameter<T, f32>,
    mThrowRigidBodyBaseAxis: agl::Parameter<T, sead::Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectWizzrobe<T> {
    base: GParamListObj<T>,
    mMagicWeatherType: agl::Parameter<T, i32>,
    mMagicFallActorName: agl::Parameter<T, sead::SafeString<T>>,
    mMagicFallIgniteRotSpd: agl::Parameter<T, f32>,
    mMagicFallOffsetY: agl::Parameter<T, f32>,
    mMagicFallCenterOffsetXZ: agl::Parameter<T, f32>,
    mMagicFallRandRadius: agl::Parameter<T, f32>,
    mMagicFallIntervalMax: agl::Parameter<T, f32>,
    mMagicFallIntervalMin: agl::Parameter<T, f32>,
    mSummonActorName: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectWolfLink<T> {
    base: GParamListObj<T>,
    mNeckSpeedWait: agl::Parameter<T, f32>,
    mNeckRateWait: agl::Parameter<T, f32>,
    mNeckSpeedShiekSensor: agl::Parameter<T, f32>,
    mNeckRateShiekSensor: agl::Parameter<T, f32>,
    mNeckSpeedFollow: agl::Parameter<T, f32>,
    mNeckRateFollow: agl::Parameter<T, f32>,
    mNeckSpeedBattle: agl::Parameter<T, f32>,
    mNeckRateBattle: agl::Parameter<T, f32>,
    mNeckSpeedHeal: agl::Parameter<T, f32>,
    mNeckRateHeal: agl::Parameter<T, f32>,
    mBattleRange: agl::Parameter<T, f32>,
    mHealRange: agl::Parameter<T, f32>,
    mHuntRange: agl::Parameter<T, f32>,
    mHowlRange: agl::Parameter<T, f32>,
    mMaxHeightAttackable: agl::Parameter<T, f32>,
    mMaxHeightHealable: agl::Parameter<T, f32>,
    mNavMeshSearchRadius: agl::Parameter<T, f32>,
    mCanReachPlayerNavMeshSearchRadius: agl::Parameter<T, f32>,
    mSubmergedDepth: agl::Parameter<T, f32>,
    mUtilityLifeToHunt: agl::Parameter<T, f32>,
    mUtilityDangerDistMin: agl::Parameter<T, f32>,
    mUtilityDangerDistMax: agl::Parameter<T, f32>,
    mUtilityConstant: agl::Parameter<T, f32>,
    mChainAttackChargeMin: agl::Parameter<T, f32>,
    mChainAttackChargeMax: agl::Parameter<T, f32>,
    mLookAtCooldownWait: agl::Parameter<T, f32>,
    mLookAtCooldownWaitRand: agl::Parameter<T, f32>,
    mLookAtCounterWait: agl::Parameter<T, f32>,
    mLookAtCounterWaitRand: agl::Parameter<T, f32>,
    mLookAtCooldownRun: agl::Parameter<T, f32>,
    mLookAtCooldownRunRand: agl::Parameter<T, f32>,
    mLookAtCounterRun: agl::Parameter<T, f32>,
    mLookAtCounterRunRand: agl::Parameter<T, f32>,
    mAttackCounterLength: agl::Parameter<T, f32>,
    mAttackCounterRand: agl::Parameter<T, f32>,
    mHowlCooldownCounterLength: agl::Parameter<T, f32>,
    mHowlCooldownCounterRand: agl::Parameter<T, f32>,
    mHealCooldownCounterLength: agl::Parameter<T, f32>,
    mHealCooldownCounterRand: agl::Parameter<T, f32>,
    mFailPathCooldownCounterLength: agl::Parameter<T, f32>,
    mFailPathCooldownCounterRand: agl::Parameter<T, f32>,
    mRetargetCooldownCounterLength: agl::Parameter<T, f32>,
    mRetargetCooldownCounterRand: agl::Parameter<T, f32>,
    mAfterTargetDeathCounterLength: agl::Parameter<T, f32>,
    mAfterTargetDeathCounterRand: agl::Parameter<T, f32>,
    mLostTargetCounterLength: agl::Parameter<T, f32>,
    mLostTargetCounterRand: agl::Parameter<T, f32>,
    mInvinceableCounterLength: agl::Parameter<T, f32>,
    mInvinceableCounterRand: agl::Parameter<T, f32>,
    mCallDelayMinLength: agl::Parameter<T, f32>,
    mCallOverrideCounterLength: agl::Parameter<T, f32>,
    mGiveUpShiekSensorLength: agl::Parameter<T, f32>,
    mRetryShiekSensorLength: agl::Parameter<T, f32>,
    mBattleWallHitLength: agl::Parameter<T, f32>,
    mFollowRetryLength: agl::Parameter<T, f32>,
    mPowerUpFoodLength: agl::Parameter<T, f32>,
    mSafePosFailCounter: agl::Parameter<T, f32>,
    mRestrictedTargetTimeNormal: agl::Parameter<T, f32>,
    mRestrictedTargetTimeSpecial: agl::Parameter<T, f32>,
    mPowerUpFoodAttackMod: agl::Parameter<T, i32>,
    mPowerUpFoodChainAttackCharge: agl::Parameter<T, f32>,
    mVSStalfosCritChance: agl::Parameter<T, i32>,
    mAttackBase: agl::Parameter<T, f32>,
    mAttackHeartMod: agl::Parameter<T, f32>,
    mDefenseBase: agl::Parameter<T, f32>,
    mDefenseHeartMod: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct GParamListObjectZora<T> {
    base: GParamListObj<T>,
    mInWaterDepth: agl::Parameter<T, f32>,
    mFloatDepth: agl::Parameter<T, f32>,
    mFloatRadius: agl::Parameter<T, f32>,
    mFloatCycleTime: agl::Parameter<T, f32>,
    mChangeDepthSpeed: agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct DirectionInfo<T> {
    mEntryPoint: agl::Parameter<T, sead::SafeString<T>>,
    mWaitFrame:  agl::Parameter<T, f32>,
    mSchedule:   agl::Parameter<T, sead::SafeString<T>>,
    mMoveAS:     agl::Parameter<T, sead::SafeString<T>>,
    mWaitAS:     agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct RoutePoint<T> {
    mName:     agl::Parameter<T, sead::SafeString<T>>,
    mForward:  DirectionInfo<T>,
    mBackward: DirectionInfo<T>,
}

#[repr(C)]
pub struct RoutePoints<T> {
    mStorage: [RoutePoint<T>; 29],
}
