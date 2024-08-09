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
    mLayer: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectAnimalUnit<T> {
    base: GParamListObj<T>,
    mSpeedActorName: agl::Parameter<T, sead::SafeString<T>>,
    mBasePlayRate: agl::Parameter<T, F32>,
    mGearMaxNum: agl::Parameter<T, S32>,
    mIsSetWaitASAtGear0: agl::Parameter<T, Bool32>,
    mStressFramesMin: agl::Parameter<T, F32>,
    mStressFramesMax: agl::Parameter<T, F32>,
    mSteeringOutputKp: agl::Parameter<T, F32>,
    mSteeringOutputKi: agl::Parameter<T, F32>,
    mSteeringOutputKd: agl::Parameter<T, F32>,
    mSteeringOutputIClamp: agl::Parameter<T, F32>,
    mSteeringOutputIReduceRatio: agl::Parameter<T, F32>,
    mSteeringOutputDLerpRatio: agl::Parameter<T, F32>,
    mSteeringOutputAvoidanceLerpRatio: agl::Parameter<T, F32>,
    mSteeringOutputIIRLerpRatio: agl::Parameter<T, F32>,
    mOverrideSteeringOutputKp: agl::Parameter<T, F32>,
    mOverrideSteeringOutputKi: agl::Parameter<T, F32>,
    mOverrideSteeringOutputKd: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectArmor<T> {
    base: GParamListObj<T>,
    mStarNum: agl::Parameter<T, S32>,
    mDefenceAddLevel: agl::Parameter<T, S32>,
    mWindScaleMesh: agl::Parameter<T, sead::SafeString<T>>,
    mWindScale: agl::Parameter<T, F32>,
    mNextRankName: agl::Parameter<T, sead::SafeString<T>>,
    mAffectTransOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mAffectRotOffsetShield: agl::Parameter<T, sead::Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectArmorEffect<T> {
    base: GParamListObj<T>,
    mEffectType: agl::Parameter<T, sead::SafeString<T>>,
    mEffectLevel: agl::Parameter<T, S32>,
    mAncientPowUp: agl::Parameter<T, Bool32>,
    mEnableClimbWaterfall: agl::Parameter<T, Bool32>,
    mEnableSpinAttack: agl::Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectArmorHead<T> {
    base: GParamListObj<T>,
    mEarRotate: agl::Parameter<T, sead::Vector3f>,
    mMantleType: agl::Parameter<T, S32>,
    mMaskType: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectArmorUpper<T> {
    base: GParamListObj<T>,
    mIsDispOffPorch: agl::Parameter<T, Bool32>,
    mShiekerStoneTransOffset: agl::Parameter<T, sead::Vector3f>,
    mShiekerStoneRotOffset: agl::Parameter<T, sead::Vector3f>,
    mDisableSelfMantle: agl::Parameter<T, Bool32>,
    mUseMantleType: agl::Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectArrow<T> {
    base: GParamListObj<T>,
    mArrowNum: agl::Parameter<T, S32>,
    mDeleteTime: agl::Parameter<T, S32>,
    mDeleteTimeWithChemical: agl::Parameter<T, S32>,
    mEnemyShootNumForDelete: agl::Parameter<T, S32>,
    mArrowDeletePer: agl::Parameter<T, S32>,
    mExtraDamage: agl::Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectAttack<T> {
    base: GParamListObj<T>,
    mPower: agl::Parameter<T, S32>,
    mImpulse: agl::Parameter<T, S32>,
    mImpulseLarge: agl::Parameter<T, S32>,
    mRange: agl::Parameter<T, F32>,
    mGuardBreakPower: agl::Parameter<T, S32>,
    mSpHitActor: agl::Parameter<T, sead::SafeString<T>>,
    mSpHitTag: agl::Parameter<T, sead::SafeString<T>>,
    mSpHitRatio: agl::Parameter<T, F32>,
    mSpWeakHitActor: agl::Parameter<T, sead::SafeString<T>>,
    mPowerForPlayer: agl::Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectAttackInterval<T> {
    base: GParamListObj<T>,
    mShortAttackTimeMin: agl::Parameter<T, S32>,
    mShortAttackTimeMax: agl::Parameter<T, S32>,
    mMiddleAttackTimeMin: agl::Parameter<T, S32>,
    mMiddleAttackTimeMax: agl::Parameter<T, S32>,
    mLongAttackTimeMin: agl::Parameter<T, S32>,
    mLongAttackTimeMax: agl::Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectAutoGen<T> {
    base: GParamListObj<T>,
    mSetName: agl::Parameter<T, sead::SafeString<T>>,
    mKeyActorName: agl::Parameter<T, sead::SafeString<T>>,
    mSetRadius: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectBeam<T> {
    base: GParamListObj<T>,
    mBeamLevel: agl::Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectBindActor<T> {
    base: GParamListObj<T>,
    mBindActorName: agl::Parameter<T, sead::SafeString<T>>,
    mIsKeepSleep: agl::Parameter<T, Bool32>,
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
    mIsGuardPierce: agl::Parameter<T, Bool32>,
    mExtraDamageRatio: agl::Parameter<T, F32>,
    mBaseAttackPowerRatio: agl::Parameter<T, F32>,
    mIsLeadShot: agl::Parameter<T, Bool32>,
    mLeadShotNum: agl::Parameter<T, S32>,
    mLeadShotAng: agl::Parameter<T, F32>,
    mLeadShotInterval: agl::Parameter<T, S32>,
    mIsRapidFire: agl::Parameter<T, Bool32>,
    mRapidFireNum: agl::Parameter<T, S32>,
    mRapidFireInterval: agl::Parameter<T, S32>,
    mIsLongRange: agl::Parameter<T, Bool32>,
    mArrowFirstSpeed: agl::Parameter<T, F32>,
    mArrowAcceleration: agl::Parameter<T, F32>,
    mArrowStabilitySpeed: agl::Parameter<T, F32>,
    mArrowFallAcceleration: agl::Parameter<T, F32>,
    mArrowFallStabilitySpeed: agl::Parameter<T, F32>,
    mArrowGravity: agl::Parameter<T, F32>,
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
    mArrowChargeRate: agl::Parameter<T, F32>,
    mArrowReloadRate: agl::Parameter<T, F32>,
    mWeaponSubType: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectBullet<T> {
    base: GParamListObj<T>,
    mNoHitParent: agl::Parameter<T, Bool32>,
    mIsLimitCount: agl::Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectCamera<T> {
    base: GParamListObj<T>,
    mDefaultConnectScaleAfterEvent: agl::Parameter<T, F32>,
    mLatConnectRateAfterEvent: agl::Parameter<T, F32>,
    mLngConnectRateAfterEvent: agl::Parameter<T, F32>,
    mDistConnectRateAfterEvent: agl::Parameter<T, F32>,
    mFovyConnectRateAfterEvent: agl::Parameter<T, F32>,
    mConnectAfterEventMin: agl::Parameter<T, F32>,
    mConnectAfterEventMax: agl::Parameter<T, F32>,
    mRoofGradientNearHighWeight: agl::Parameter<T, F32>,
    mRoofGradientFarHighWeight: agl::Parameter<T, F32>,
    mRoofGradientNearLowWeight: agl::Parameter<T, F32>,
    mRoofGradientFarLowWeight: agl::Parameter<T, F32>,
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
    mAtkCollidableSpeed: agl::Parameter<T, F32>,
    mAtkCollidableActiveTime: agl::Parameter<T, F32>,
    mAtkCollidableResetPos: agl::Parameter<T, sead::Vector3f>,
    mGroundCollidableName: agl::Parameter<T, sead::SafeString<T>>,
    mGroundCollidableBoneName: agl::Parameter<T, sead::SafeString<T>>,
    mGroundCollidableOffset: agl::Parameter<T, F32>,
    mUseGroundCollidableResetPos: agl::Parameter<T, Bool32>,
    mGroundCollidableResetPos: agl::Parameter<T, sead::Vector3f>,
    mGroundCollidableMoveSpeed: agl::Parameter<T, F32>,
    mWallCollidableName: agl::Parameter<T, sead::SafeString<T>>,
    mWallCollidableBoneName: agl::Parameter<T, sead::SafeString<T>>,
    mWallCollidableOffset: agl::Parameter<T, F32>,
    mPlayerCollidableName: agl::Parameter<T, sead::SafeString<T>>,
    mPlayerCollidableBoneName: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectCookSpice<T> {
    base: GParamListObj<T>,
    mBoostHitPointRecover: agl::Parameter<T, S32>,
    mBoostEffectiveTime: agl::Parameter<T, S32>,
    mBoostSuccessRate: agl::Parameter<T, S32>,
    mBoostMaxHeartLevel: agl::Parameter<T, S32>,
    mBoostStaminaLevel: agl::Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectCureItem<T> {
    base: GParamListObj<T>,
    mHitPointRecover: agl::Parameter<T, S32>,
    mEffectType: agl::Parameter<T, sead::SafeString<T>>,
    mEffectLevel: agl::Parameter<T, S32>,
    mEffectiveTime: agl::Parameter<T, S32>,
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
    mRank: agl::Parameter<T, S32>,
    mPower: agl::Parameter<T, S32>,
    mDropLife: agl::Parameter<T, S32>,
    mDyingLife: agl::Parameter<T, S32>,
    mLostDist: agl::Parameter<T, F32>,
    mLostHeightMax: agl::Parameter<T, F32>,
    mLostHeightMin: agl::Parameter<T, F32>,
    mLostRayLength: agl::Parameter<T, F32>,
    mLODLostDist: agl::Parameter<T, F32>,
    mLODLostHeightMax: agl::Parameter<T, F32>,
    mLODLostHeightMin: agl::Parameter<T, F32>,
    mIntelligenceLevel: agl::Parameter<T, F32>,
    mEmotionalLevel: agl::Parameter<T, F32>,
    mHeroismLevel: agl::Parameter<T, F32>,
    mPartActorName: agl::Parameter<T, sead::SafeString<T>>,
    mIsMindFriend: agl::Parameter<T, Bool32>,
    mStatusChangeFlag: agl::Parameter<T, sead::SafeString<T>>,
    mChangeLife: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectEnemyLevel<T> {
    base: GParamListObj<T>,
    mIsAvoidDanger: agl::Parameter<T, Bool32>,
    mIsGuardArrow: agl::Parameter<T, Bool32>,
    mIsHideArrowAttack: agl::Parameter<T, Bool32>,
    mIsSwiftAttack: agl::Parameter<T, Bool32>,
    mIsBackSwiftAttack: agl::Parameter<T, Bool32>,
    mIsCounterAttack: agl::Parameter<T, Bool32>,
    mIsEscapeBomb: agl::Parameter<T, Bool32>,
    mIsKickBomb: agl::Parameter<T, Bool32>,
    mIsShootBomb: agl::Parameter<T, Bool32>,
    mIsThrowWeapon: agl::Parameter<T, Bool32>,
    mGuardPer: agl::Parameter<T, S32>,
    mIsJustGuard: agl::Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectEnemyRace<T> {
    base: GParamListObj<T>,
    mEquipableWeapon: agl::Parameter<T, sead::SafeString<T>>,
    mIsFitGroundByAnimalUnit: agl::Parameter<T, Bool32>,
    mIsUpdateSupportNormalInAir: agl::Parameter<T, Bool32>,
    mBowAttackRangeRatio: agl::Parameter<T, F32>,
    mWeaponScaleSmallSword: agl::Parameter<T, F32>,
    mWeaponTransOffsetSmallSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponRotOffsetSmallSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldTransOffsetSmallSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldRotOffsetSmallSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponScaleLargeSword: agl::Parameter<T, F32>,
    mWeaponTransOffsetLargeSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponRotOffsetLargeSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldTransOffsetLargeSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldRotOffsetLargeSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponScaleSpear: agl::Parameter<T, F32>,
    mWeaponTransOffsetSpear: agl::Parameter<T, sead::Vector3f>,
    mWeaponRotOffsetSpear: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldTransOffsetSpear: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldRotOffsetSpear: agl::Parameter<T, sead::Vector3f>,
    mWeaponScaleBow: agl::Parameter<T, F32>,
    mWeaponTransOffsetBow: agl::Parameter<T, sead::Vector3f>,
    mWeaponRotOffsetBow: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldTransOffsetBow: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldRotOffsetBow: agl::Parameter<T, sead::Vector3f>,
    mWeaponScaleShield: agl::Parameter<T, F32>,
    mWeaponTransOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mWeaponRotOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldTransOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mWeaponHoldRotOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mIsUseTargetTag: agl::Parameter<T, Bool32>,
    mTargetActorType: agl::Parameter<T, sead::SafeString<T>>,
    mEscapeAttackedActorType: agl::Parameter<T, sead::SafeString<T>>,
    mReactionBalloon: agl::Parameter<T, Bool32>,
    mSmallRagdollTime: agl::Parameter<T, S32>,
    mSmallRagdollRecoverTime: agl::Parameter<T, S32>,
    mSmallLargeRagdollTime: agl::Parameter<T, S32>,
    mSmallLargeRagdollRecoverTime: agl::Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectEnemyShown<T> {
    base: GParamListObj<T>,
    mIsHappy: agl::Parameter<T, Bool32>,
    mIsCasebyCase: agl::Parameter<T, Bool32>,
    mIsSit: agl::Parameter<T, Bool32>,
    mIsNoise: agl::Parameter<T, Bool32>,
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
    mIsUsePivotAdjustRange: agl::Parameter<T, Bool32>,
    mPivotAdjustRange: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectFish<T> {
    base: GParamListObj<T>,
    mRestoreSpeedRate: agl::Parameter<T, F32>,
    mRestoreSpeedRateAdd: agl::Parameter<T, F32>,
    mLimitAngle: agl::Parameter<T, F32>,
    mLimitAngleAdd: agl::Parameter<T, F32>,
    mPrevSpeedRate: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectGelEnemy<T> {
    base: GParamListObj<T>,
    mMoveBoneName: agl::Parameter<T, sead::SafeString<T>>,
    mBodyRadius: agl::Parameter<T, F32>,
    mClothBoneNumForEyeCalc: agl::Parameter<T, S32>,
    mBodyRootBoneName: agl::Parameter<T, sead::SafeString<T>>,
    mLeftEyeBoneName: agl::Parameter<T, sead::SafeString<T>>,
    mRightEyeBoneName: agl::Parameter<T, sead::SafeString<T>>,
    mEyeSpaceHalf: agl::Parameter<T, F32>,
    mEyeDir: agl::Parameter<T, sead::Vector3f>,
    mEyeOffset: agl::Parameter<T, sead::Vector3f>,
    mEyeUpMoveRate: agl::Parameter<T, F32>,
    mEyeDownMoveRate: agl::Parameter<T, F32>,
    mIsAverageEyePos: agl::Parameter<T, Bool32>,
    mEyeDelayAccRate: agl::Parameter<T, F32>,
    mEyeYMoveTheta: agl::Parameter<T, F32>,
    mEyeYMoveFrequency: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectGeneral<T> {
    base: GParamListObj<T>,
    mSpeed: agl::Parameter<T, F32>,
    mLife: agl::Parameter<T, S32>,
    mIsLifeInfinite: agl::Parameter<T, Bool32>,
    mElectricalDischarge: agl::Parameter<T, F32>,
    mIsBurnOutBorn: agl::Parameter<T, Bool32>,
    mBurnOutBornName: agl::Parameter<T, sead::SafeString<T>>,
    mIsBurnOutBornIdent: agl::Parameter<T, Bool32>,
    mChangeDropTableName: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectGiantArmor<T> {
    base: GParamListObj<T>,
    mDamageScale: agl::Parameter<T, F32>,
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
    mEnemyLifeGageDist: agl::Parameter<T, F32>,
    mEnemyNoSkitDist: agl::Parameter<T, F32>,
    mEnemyWeaponPickAllowDist: agl::Parameter<T, F32>,
    mEnemyWeaponPickForbidTime: agl::Parameter<T, S32>,
    mEnemyAnimalNoDamageDist: agl::Parameter<T, F32>,
    mEnemyNearCraeteIDDelay: agl::Parameter<T, F32>,
    mEnemyForceTiredLODCount: agl::Parameter<T, S32>,
    mEnemyForceTiredNoSightLODCount: agl::Parameter<T, S32>,
    mEnemyForceWarpReturnLODCount: agl::Parameter<T, S32>,
    mSilentAttackAng: agl::Parameter<T, F32>,
    mSilentAttackRatio: agl::Parameter<T, F32>,
    mBlownOffPlayerAtkDelay: agl::Parameter<T, S32>,
    mJustAvoidAcceptWpRangeSS: agl::Parameter<T, F32>,
    mJustAvoidAcceptWpRangeLS: agl::Parameter<T, F32>,
    mJustAvoidAcceptWpRangeSP: agl::Parameter<T, F32>,
    mForceNoticeEnemyCount: agl::Parameter<T, S32>,
    mForceNoticeEnemyDist: agl::Parameter<T, F32>,
    mWeaponRickeyLife: agl::Parameter<T, S32>,
    mWeaponDropRotSpd: agl::Parameter<T, F32>,
    mShieldRideBaseFrame: agl::Parameter<T, S32>,
    mShieldRideHitBaseDamage: agl::Parameter<T, S32>,
    mShieldDamageratio: agl::Parameter<T, F32>,
    mShieldSurfMasterFrictionRatio: agl::Parameter<T, F32>,
    mLoudNoiseRadius: agl::Parameter<T, F32>,
    mImpulse2DamageRatio: agl::Parameter<T, F32>,
    mIceMeltSpeedOnContactFire: agl::Parameter<T, F32>,
    mCriticalAttackRatio: agl::Parameter<T, F32>,
    mBooerangAttackRatio: agl::Parameter<T, F32>,
    mHitImpulseClampMax: agl::Parameter<T, F32>,
    mDropItemVelXZFromBomb: agl::Parameter<T, F32>,
    mDropItemVelYFromBomb: agl::Parameter<T, F32>,
    mDropItemVelRandomFromBomb: agl::Parameter<T, F32>,
    mDropItemAngVelFromBomb: agl::Parameter<T, F32>,
    mDropItemAngVelRandomFromBomb: agl::Parameter<T, F32>,
    mDropItemVelXZSmall: agl::Parameter<T, F32>,
    mDropItemVelYSmall: agl::Parameter<T, F32>,
    mDropItemVelRandomSmall: agl::Parameter<T, F32>,
    mDropItemAngVelSmall: agl::Parameter<T, F32>,
    mDropItemAngVelRandomSmall: agl::Parameter<T, F32>,
    mDropItemVelXZLarge: agl::Parameter<T, F32>,
    mDropItemVelYLarge: agl::Parameter<T, F32>,
    mDropItemVelRandomLarge: agl::Parameter<T, F32>,
    mDropItemAngVelLarge: agl::Parameter<T, F32>,
    mDropItemAngVelRandomLarge: agl::Parameter<T, F32>,
    mDropItemVelXZRupeeRabbit: agl::Parameter<T, F32>,
    mDropItemVelYRupeeRabbit: agl::Parameter<T, F32>,
    mDropItemVelRandomRupeeRabbit: agl::Parameter<T, F32>,
    mDropItemVelXZItemRupeeOnly: agl::Parameter<T, F32>,
    mDropItemVelYItemRupeeOnly: agl::Parameter<T, F32>,
    mDropItemVelRandomItemRupeeOnly: agl::Parameter<T, F32>,
    mDropItemInvincibleTime: agl::Parameter<T, F32>,
    mTreeWeaponEquipTransOffset: agl::Parameter<T, sead::Vector3f>,
    mTreeWeaponEquipRotOffset: agl::Parameter<T, sead::Vector3f>,
    mWetRatioToDie: agl::Parameter<T, F32>,
    mEnvWetRatioToDie: agl::Parameter<T, F32>,
    mNPCTurnAngleDiff: agl::Parameter<T, F32>,
    mNPCWaitFrameAfterEvent: agl::Parameter<T, S32>,
    mNPCIgnorePlayerTime: agl::Parameter<T, F32>,
    mNPCCancelIgnorePlayerTime: agl::Parameter<T, F32>,
    mNPCOpenDoorDistance: agl::Parameter<T, F32>,
    mNPCWalkRateOnSandAndSnow: agl::Parameter<T, F32>,
    mNPCDownVerticallyAngle: agl::Parameter<T, F32>,
    mGerudoQueenSafetyAreaRadius: agl::Parameter<T, F32>,
    mCreateFairyLimitCount: agl::Parameter<T, S32>,
    mTerrorRegistSpeed: agl::Parameter<T, F32>,
    mTerrorUnregistSpeed: agl::Parameter<T, F32>,
    mTerrorRegistTimer: agl::Parameter<T, S32>,
    mTerrorRadiusOffset: agl::Parameter<T, F32>,
    mSpeedTerrorLevel: agl::Parameter<T, S32>,
    mSpeedTerrorLevelHuge: agl::Parameter<T, S32>,
    mSpeedTerrorLevelCheckRadius: agl::Parameter<T, F32>,
    mAtDirTypeAffectRatio: agl::Parameter<T, F32>,
    mRainyAwnHearingLevel: agl::Parameter<T, F32>,
    mHorseBindOffsetYOfMaleUMii: agl::Parameter<T, F32>,
    mHorseBindOffsetYOfFemaleUMii: agl::Parameter<T, F32>,
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
    mHorseExtraChargeNum: agl::Parameter<T, S32>,
    mPlayerGrabThrowDiffRate: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectGolem<T> {
    base: GParamListObj<T>,
    mUpperArmRActor: agl::Parameter<T, sead::SafeString<T>>,
    mLowerArmRActor: agl::Parameter<T, sead::SafeString<T>>,
    mUpperArmLActor: agl::Parameter<T, sead::SafeString<T>>,
    mLowerArmLActor: agl::Parameter<T, sead::SafeString<T>>,
    mDefaultWeakPointActor: agl::Parameter<T, sead::SafeString<T>>,
    mIsDefaultChemicalOn: agl::Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectGolemIK<T> {
    base: GParamListObj<T>,
    mFootRayCheckDist: agl::Parameter<T, F32>,
    mFootDownRatio: agl::Parameter<T, F32>,
    mFootUpRatio: agl::Parameter<T, F32>,
    mKneeExtendL: agl::Parameter<T, F32>,
    mKneeShrinkL: agl::Parameter<T, F32>,
    mFootExtendL: agl::Parameter<T, F32>,
    mFootShrinkL: agl::Parameter<T, F32>,
    mKneeExtendR: agl::Parameter<T, F32>,
    mKneeShrinkR: agl::Parameter<T, F32>,
    mFootExtendR: agl::Parameter<T, F32>,
    mFootShrinkR: agl::Parameter<T, F32>,
    mArmRayCheckDist: agl::Parameter<T, F32>,
    mArmDownRatio: agl::Parameter<T, F32>,
    mArmUpRatio: agl::Parameter<T, F32>,
    mElbowExtendL: agl::Parameter<T, F32>,
    mElbowShrinkL: agl::Parameter<T, F32>,
    mWristExtendL: agl::Parameter<T, F32>,
    mWristShrinkL: agl::Parameter<T, F32>,
    mElbowExtendR: agl::Parameter<T, F32>,
    mElbowShrinkR: agl::Parameter<T, F32>,
    mWristExtendR: agl::Parameter<T, F32>,
    mWristShrinkR: agl::Parameter<T, F32>,
    mWaistRotateRatio: agl::Parameter<T, F32>,
    mWaistMorphRatio: agl::Parameter<T, F32>,
    mWaistResetMorphRatio: agl::Parameter<T, F32>,
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
    mGuardianModelType: agl::Parameter<T, S32>,
    mGuardianControllerType: agl::Parameter<T, S32>,
    mHeadLimitMax: agl::Parameter<T, F32>,
    mHeadLimitMin: agl::Parameter<T, F32>,
    mSightLimitMax: agl::Parameter<T, F32>,
    mSightLimitMin: agl::Parameter<T, F32>,
    mMaxSpeed: agl::Parameter<T, F32>,
    mCannonBoneName: agl::Parameter<T, sead::SafeString<T>>,
    mRapidFireDistance: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectGuardianMini<T> {
    base: GParamListObj<T>,
    mColorType: agl::Parameter<T, S32>,
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
    mIsDecoy: agl::Parameter<T, Bool32>,
    mHasMane: agl::Parameter<T, Bool32>,
    mASVariation: agl::Parameter<T, sead::SafeString<T>>,
    mNature: agl::Parameter<T, S32>,
    mAttackPowerMultiplierGear2: agl::Parameter<T, F32>,
    mAttackPowerMultiplierGear3: agl::Parameter<T, F32>,
    mAttackPowerMultiplierGearTop: agl::Parameter<T, F32>,
    mRunnableFramesAtGearTop: agl::Parameter<T, F32>,
    mGearTopInterval: agl::Parameter<T, F32>,
    mGearTopChargeNum: agl::Parameter<T, S32>,
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
    mHideHorseMane: agl::Parameter<T, Bool32>,
    mIsHorseClothDisable: agl::Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectHorseRider<T> {
    base: GParamListObj<T>,
    mRootNode: agl::Parameter<T, sead::SafeString<T>>,
    mSpineNode: agl::Parameter<T, sead::SafeString<T>>,
    mRotBaseNode: agl::Parameter<T, sead::SafeString<T>>,
    mRotAxis: agl::Parameter<T, sead::Vector3f>,
    mRotLimit: agl::Parameter<T, F32>,
    mWeaponTransOffsetSmallSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponTransOffsetLargeSword: agl::Parameter<T, sead::Vector3f>,
    mWeaponTransOffsetSpear: agl::Parameter<T, sead::Vector3f>,
    mWeaponTransOffsetBow: agl::Parameter<T, sead::Vector3f>,
    mWeaponTransOffsetShield: agl::Parameter<T, sead::Vector3f>,
    mLeftFootNode: agl::Parameter<T, sead::SafeString<T>>,
    mLeftFootRotAxis: agl::Parameter<T, sead::Vector3f>,
    mRightFootNode: agl::Parameter<T, sead::SafeString<T>>,
    mRightFootRotAxis: agl::Parameter<T, sead::Vector3f>,
    mFootRotRatio: agl::Parameter<T, F32>,
    mFootRetRotRatio: agl::Parameter<T, F32>,
    mFootRotAngleForKuma: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectHorseTargetedInfo<T> {
    base: GParamListObj<T>,
    mHorseMoveRadius: agl::Parameter<T, F32>,
    mHorseAvoidOffset: agl::Parameter<T, F32>,
    mIsCircularMoveAlways: agl::Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectHorseUnit<T> {
    base: GParamListObj<T>,
    mRiddenAnimalType: agl::Parameter<T, S32>,
    mCalmDownNum: agl::Parameter<T, S32>,
    mRideonAboveASHeight: agl::Parameter<T, F32>,
    mRideonAboveASRadius: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectInsect<T> {
    base: GParamListObj<T>,
    mFireResistanceLevel: agl::Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectItem<T> {
    base: GParamListObj<T>,
    mPlayerUseItem: agl::Parameter<T, Bool32>,
    mSellingPrice: agl::Parameter<T, S32>,
    mBuyingPrice: agl::Parameter<T, S32>,
    mCreatingPrice: agl::Parameter<T, S32>,
    mStainColor: agl::Parameter<T, S32>,
    mSaleRevivalCount: agl::Parameter<T, S32>,
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
    mThrownMass: agl::Parameter<T, S32>,
    mThrownRotSpd: agl::Parameter<T, sead::Vector3f>,
    mLiftPosOffset: agl::Parameter<T, sead::Vector3f>,
    mLiftRotOffset: agl::Parameter<T, sead::Vector3f>,
    mLiftRotFrame: agl::Parameter<T, S32>,
    mAddLiftRotOffsetList: agl::Parameter<T, sead::SafeString<T>>,
    mChaseLiftRotOffset: agl::Parameter<T, Bool32>,
    mLiftCenterOffset: agl::Parameter<T, sead::Vector3f>,
    mPutPosOffset: agl::Parameter<T, sead::Vector3f>,
    mPutRotOffset: agl::Parameter<T, sead::Vector3f>,
    mPutRotFrame: agl::Parameter<T, S32>,
    mAddPutRotOffsetList: agl::Parameter<T, sead::SafeString<T>>,
    mIsUpdateOffsetEachFrame: agl::Parameter<T, Bool32>,
    mIsUse2MassConstraintMode: agl::Parameter<T, Bool32>,
    mIsSetChemicalParent: agl::Parameter<T, Bool32>,
    mDisableFreezeLift: agl::Parameter<T, Bool32>,
    mDisableBurnLift: agl::Parameter<T, Bool32>,
    mThrowReactionLevel: agl::Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectLumberjackTree<T> {
    base: GParamListObj<T>,
    mImpulseThreshold: agl::Parameter<T, F32>,
    mIsValid: agl::Parameter<T, Bool32>,
    mStumpName: agl::Parameter<T, sead::SafeString<T>>,
    mTrunkName: agl::Parameter<T, sead::SafeString<T>>,
    mWeaponWoodName: agl::Parameter<T, sead::SafeString<T>>,
    mBranchName: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectMasterSword<T> {
    base: GParamListObj<T>,
    mIsMasterSword: agl::Parameter<T, Bool32>,
    mTrueFormAttackPower: agl::Parameter<T, S32>,
    mTrueFormMagicPower: agl::Parameter<T, S32>,
    mTrueFormBreakRatio: agl::Parameter<T, F32>,
    mSearchEvilDist: agl::Parameter<T, F32>,
    mRecoverTime: agl::Parameter<T, S32>,
    mSleepActorName: agl::Parameter<T, sead::SafeString<T>>,
    mTrueFormActorName: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectMonsterShop<T> {
    base: GParamListObj<T>,
    mBuyMamo: agl::Parameter<T, S32>,
    mSellMamo: agl::Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectMotorcycle<T> {
    base: GParamListObj<T>,
    mPitchDampingCoefficient: agl::Parameter<T, F32>,
    mDriftAllowSpeedKPH: agl::Parameter<T, F32>,
    mDriftAbortSpeedKPH: agl::Parameter<T, F32>,
    mDriftAllowSteerRate: agl::Parameter<T, F32>,
    mDriftAbortSteerRate: agl::Parameter<T, F32>,
    mDriftRearAngleRate: agl::Parameter<T, F32>,
    mDriftSpeedRate: agl::Parameter<T, F32>,
    mManualWheelieAllowAngleFront: agl::Parameter<T, F32>,
    mManualWheelieAllowAngleRear: agl::Parameter<T, F32>,
    mManualWheelieLastSec: agl::Parameter<T, F32>,
    mWheelieLastSecInMidAir: agl::Parameter<T, F32>,
    mManualControlProhibitSecAfterWheelie: agl::Parameter<T, F32>,
    mWheelieRevertPower: agl::Parameter<T, F32>,
    mWheelieRevertPowerSec: agl::Parameter<T, F32>,
    mManualWheelieRiseDegDelta: agl::Parameter<T, F32>,
    mWheelieLaunchRiseDegDelta: agl::Parameter<T, F32>,
    mEngineBrakeMaxPower: agl::Parameter<T, F32>,
    mBackwardEngineBrakePower: agl::Parameter<T, F32>,
    mSlipStartAngle: agl::Parameter<T, F32>,
    mSlipThresholdPower: agl::Parameter<T, F32>,
    mSlipPowerMax: agl::Parameter<T, F32>,
    mWristBindRotation: agl::Parameter<T, sead::Vector3f>,
    mWristBindTranslation: agl::Parameter<T, sead::Vector3f>,
    mPostureLimitAngle: agl::Parameter<T, F32>,
    mInvalidPostureLimitSec: agl::Parameter<T, F32>,
    mFallOverThresholdAngle: agl::Parameter<T, F32>,
    mJumpIntervalSec: agl::Parameter<T, F32>,
    mFullEnergyLastSec: agl::Parameter<T, F32>,
    mWheelieLaunchJumpProhibitSec: agl::Parameter<T, F32>,
    mSlowModeTargetSpeedKPH2: agl::Parameter<T, F32>,
    mSlowDriftTargetSpeedKPH2: agl::Parameter<T, F32>,
    mSlowModeTransitionSec: agl::Parameter<T, F32>,
    mSlowSlipThresholdKPH: agl::Parameter<T, F32>,
    mSlowSlipThresholdPower: agl::Parameter<T, F32>,
    mSlowSlipThresholdSec: agl::Parameter<T, F32>,
    mJumpRearWheelRotateRadPerSec: agl::Parameter<T, F32>,
    mWeaponThrowModeSpeedKPH2: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectNest<T> {
    base: GParamListObj<T>,
    mCreateActor: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectNpc<T> {
    base: GParamListObj<T>,
    mIsReactNakedPlayer: agl::Parameter<T, Bool32>,
    mUseAutoLabel: agl::Parameter<T, Bool32>,
    mIsOffPodFromWeapon: agl::Parameter<T, Bool32>,
    mIsRunRainWhenGoToSleep: agl::Parameter<T, Bool32>,
    mIsWalkUnderShelterFromRain: agl::Parameter<T, Bool32>,
    mIsSlowWalkOnSandAndSnow: agl::Parameter<T, Bool32>,
    mIsAlwaysCounterPlayerAttack: agl::Parameter<T, Bool32>,
    mIsNotTurnDetect: agl::Parameter<T, Bool32>,
    mIsNotEscapeFromTerror: agl::Parameter<T, Bool32>,
    mTolerantTime: agl::Parameter<T, S32>,
    mTolerantCount: agl::Parameter<T, S32>,
    mCounterRate: agl::Parameter<T, S32>,
    mChangeSearchModeFlagName: agl::Parameter<T, sead::SafeString<T>>,
    mOnFlagWhenDelete: agl::Parameter<T, sead::SafeString<T>>,
    mOffFlagWhenDelete: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectNpcEquipment<T> {
    base: GParamListObj<T>,
    mIsSetWeaponTypeWhenEquip: agl::Parameter<T, Bool32>,
    mEquipName1: agl::Parameter<T, sead::SafeString<T>>,
    mScale1: agl::Parameter<T, F32>,
    mHoldTransOffset1: agl::Parameter<T, sead::Vector3f>,
    mHoldRotOffset1: agl::Parameter<T, sead::Vector3f>,
    mEquipTransOffset1: agl::Parameter<T, sead::Vector3f>,
    mEquipRotOffset1: agl::Parameter<T, sead::Vector3f>,
    mEquipName2: agl::Parameter<T, sead::SafeString<T>>,
    mScale2: agl::Parameter<T, F32>,
    mHoldTransOffset2: agl::Parameter<T, sead::Vector3f>,
    mHoldRotOffset2: agl::Parameter<T, sead::Vector3f>,
    mEquipTransOffset2: agl::Parameter<T, sead::Vector3f>,
    mEquipRotOffset2: agl::Parameter<T, sead::Vector3f>,
    mEquipName3: agl::Parameter<T, sead::SafeString<T>>,
    mScale3: agl::Parameter<T, F32>,
    mHoldTransOffset3: agl::Parameter<T, sead::Vector3f>,
    mHoldRotOffset3: agl::Parameter<T, sead::Vector3f>,
    mEquipTransOffset3: agl::Parameter<T, sead::Vector3f>,
    mEquipRotOffset3: agl::Parameter<T, sead::Vector3f>,
    mEquipName4: agl::Parameter<T, sead::SafeString<T>>,
    mScale4: agl::Parameter<T, F32>,
    mHoldTransOffset4: agl::Parameter<T, sead::Vector3f>,
    mHoldRotOffset4: agl::Parameter<T, sead::Vector3f>,
    mEquipTransOffset4: agl::Parameter<T, sead::Vector3f>,
    mEquipRotOffset4: agl::Parameter<T, sead::Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectPictureBook<T> {
    base: GParamListObj<T>,
    mLiveSpot1: agl::Parameter<T, S32>,
    mLiveSpot2: agl::Parameter<T, S32>,
    mSpecialDrop: agl::Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectPlayer<T> {
    base: GParamListObj<T>,
    mBombReloadTime1: agl::Parameter<T, F32>,
    mBombReloadTime2: agl::Parameter<T, F32>,
    mStopTimerReloadTime: agl::Parameter<T, F32>,
    mStopTimerBlowAngle: agl::Parameter<T, F32>,
    mStopTimerBlowSpeedLimit: agl::Parameter<T, F32>,
    mStopTimerImpluseMaxCountSmallSword: agl::Parameter<T, S32>,
    mStopTimerImpluseMaxCountLargeSword: agl::Parameter<T, S32>,
    mStopTimerImpluseMaxCountSpear: agl::Parameter<T, S32>,
    mStopTimerCancelDeleteWaitTime: agl::Parameter<T, F32>,
    mStopTimerLongTime: agl::Parameter<T, F32>,
    mStopTimerMiddleTime: agl::Parameter<T, F32>,
    mStopTimerShortTime: agl::Parameter<T, F32>,
    mEnergyTiredValue: agl::Parameter<T, F32>,
    mEnergyBowSlow: agl::Parameter<T, F32>,
    mEnergyPush: agl::Parameter<T, F32>,
    mEnergyCharge: agl::Parameter<T, F32>,
    mEnergyAutoRecover: agl::Parameter<T, F32>,
    mEnergyAutoRecoverInAir: agl::Parameter<T, F32>,
    mEnergyAutoRecoverInvalidTime1: agl::Parameter<T, F32>,
    mEnergyAutoRecoverInvalidTime2: agl::Parameter<T, F32>,
    mColdTempDamageAdd: agl::Parameter<T, F32>,
    mHotTempDamageAdd: agl::Parameter<T, F32>,
    mTempDamage: agl::Parameter<T, F32>,
    mTempEnergyDecDiamAdd: agl::Parameter<T, F32>,
    mTempEnergyDecDegAdd: agl::Parameter<T, F32>,
    mVelDiamSand: agl::Parameter<T, F32>,
    mVelDiamTired: agl::Parameter<T, F32>,
    mStickDiamTired: agl::Parameter<T, F32>,
    mAutoRecoverNum: agl::Parameter<T, F32>,
    mAutoRecoverIntervalMin: agl::Parameter<T, F32>,
    mAutoRecoverIntervalMax: agl::Parameter<T, F32>,
    mAutoRecoverInvalidTime: agl::Parameter<T, F32>,
    mBowSubjectContTime: agl::Parameter<T, F32>,
    mLNGStickScale: agl::Parameter<T, F32>,
    mLATStickScale: agl::Parameter<T, F32>,
    mLNGGyroScale: agl::Parameter<T, F32>,
    mLATGyroScale: agl::Parameter<T, F32>,
    mBowSlowShootNum: agl::Parameter<T, S32>,
    mBowSlowRateDiam: agl::Parameter<T, F32>,
    mBowSlowMaxTime: agl::Parameter<T, F32>,
    mDiveBowSlowMaxTime: agl::Parameter<T, F32>,
    mBowSlowInvalidTime: agl::Parameter<T, F32>,
    mBowSlowInvalidHeight: agl::Parameter<T, F32>,
    mBowSlowInvalidHeightOnShield: agl::Parameter<T, F32>,
    mBowSlowInvalidHeightWeaponChange: agl::Parameter<T, F32>,
    mGuardJustForceSlowTime: agl::Parameter<T, F32>,
    mMoveMaxDecRateByWater: agl::Parameter<T, F32>,
    mMoveIgnoreWaterHeight: agl::Parameter<T, F32>,
    mMoveDecRateByBog: agl::Parameter<T, F32>,
    mMoveDecRateMaxHeight: agl::Parameter<T, F32>,
    mMaxForce: agl::Parameter<T, F32>,
    mMinForce: agl::Parameter<T, F32>,
    mAddForce: agl::Parameter<T, F32>,
    mSnowBallAddForce: agl::Parameter<T, F32>,
    mLogPushF: agl::Parameter<T, F32>,
    mRockPushF: agl::Parameter<T, F32>,
    mRockPushSpeed: agl::Parameter<T, F32>,
    mWaistAngleUpperMax: agl::Parameter<T, F32>,
    mWaistAngleLowerMax: agl::Parameter<T, F32>,
    mWaistAngleSideMax: agl::Parameter<T, F32>,
    mNoSquatWaterHeight: agl::Parameter<T, F32>,
    mInvalidReloadTime: agl::Parameter<T, F32>,
    mWeaponThrowSpeedY: agl::Parameter<T, F32>,
    mWeaponThrowSpeedF: agl::Parameter<T, F32>,
    mWeaponThrowSpeedFSquat: agl::Parameter<T, F32>,
    mDashUpEnableAngle: agl::Parameter<T, F32>,
    mShockTime: agl::Parameter<T, F32>,
    mIceInvalidTime: agl::Parameter<T, F32>,
    mMaxSpeedInAir: agl::Parameter<T, F32>,
    mTurnEnableSpeed: agl::Parameter<T, F32>,
    mTurnEnableStickSub: agl::Parameter<T, F32>,
    mTurnEnableDirSub: agl::Parameter<T, F32>,
    mShortDashImpulse: agl::Parameter<T, S32>,
    mShortDashDamage: agl::Parameter<T, S32>,
    mSwordTerrorScope: agl::Parameter<T, F32>,
    mArrowTerrorScope: agl::Parameter<T, F32>,
    mTorchTerrorScope: agl::Parameter<T, F32>,
    mTorchTerrorOffsetY: agl::Parameter<T, F32>,
    mTorchTerrorOffsetZ: agl::Parameter<T, F32>,
    mDashNoise: agl::Parameter<T, F32>,
    mWhistleNoise: agl::Parameter<T, F32>,
    mClimbEnableAngle: agl::Parameter<T, F32>,
    mClimbEnableSpeedMinAngle: agl::Parameter<T, F32>,
    mClimbEnableSpeedMaxAngle: agl::Parameter<T, F32>,
    mSlipEnableSpeed: agl::Parameter<T, F32>,
    mSlipSpeedAddMin: agl::Parameter<T, F32>,
    mSlipSpeedAddMax: agl::Parameter<T, F32>,
    mSlipSpeedAddDiamByRain: agl::Parameter<T, F32>,
    mMagnetAim2DPosOffsetY: agl::Parameter<T, F32>,
    mLookPosOffsetXZ: agl::Parameter<T, F32>,
    mLookPosOffsetY: agl::Parameter<T, F32>,
    mLookPosOffsetYSquat: agl::Parameter<T, F32>,
    mLookPosOffsetYSwim: agl::Parameter<T, F32>,
    mLookPosOffsetYHorse: agl::Parameter<T, F32>,
    mLookEnableAngle: agl::Parameter<T, F32>,
    mHitSlowTimeS: agl::Parameter<T, F32>,
    mHitSlowTimeM: agl::Parameter<T, F32>,
    mHitSlowTimeL: agl::Parameter<T, F32>,
    mHitSlowRate: agl::Parameter<T, F32>,
    mHitStopTimeS: agl::Parameter<T, F32>,
    mHitStopTimeL: agl::Parameter<T, F32>,
    mHitStopRate: agl::Parameter<T, F32>,
    mAtnPosInterPolationRate: agl::Parameter<T, F32>,
    mAtnPosInterPolationMin: agl::Parameter<T, F32>,
    mAtnPosInterPolationMax: agl::Parameter<T, F32>,
    mPredictDiffAngleMax: agl::Parameter<T, F32>,
    mDashToRunStickValueDec: agl::Parameter<T, F32>,
    mWindSupportReuseTime: agl::Parameter<T, F32>,
    mFireSupportReuseTime: agl::Parameter<T, F32>,
    mElectricSupportReuseTime: agl::Parameter<T, F32>,
    mWaterSupportReuseTime: agl::Parameter<T, F32>,
    mWindSupportTimerRate: agl::Parameter<T, F32>,
    mFireSupportTimerRate: agl::Parameter<T, F32>,
    mElectricSupportTimerRate: agl::Parameter<T, F32>,
    mWaterSupportTimerRate: agl::Parameter<T, F32>,
    mChemicalInvalidTime: agl::Parameter<T, F32>,
    mAutoDashUpTime: agl::Parameter<T, F32>,
    mAutoDashUpAngle: agl::Parameter<T, F32>,
    mClimbRestartHeight: agl::Parameter<T, F32>,
    mClimbRestartTime: agl::Parameter<T, F32>,
    mPushNoticeLookTime: agl::Parameter<T, F32>,
    mEnergyUseSmall: agl::Parameter<T, F32>,
    mEnergyUseLarge: agl::Parameter<T, F32>,
    mNoEnergyDashInterval: agl::Parameter<T, F32>,
    mGuardableAngle: agl::Parameter<T, F32>,
    mStickMaxInStore: agl::Parameter<T, F32>,
    mLookContinueTime: agl::Parameter<T, F32>,
    mPostureContinueTime: agl::Parameter<T, F32>,
    mItemUseModelAlpha: agl::Parameter<T, F32>,
    mLadderCheckSide: agl::Parameter<T, F32>,
    mLadderCheckDist: agl::Parameter<T, F32>,
    mNoDeathDamageBase: agl::Parameter<T, S32>,
    mNoDeathDamageAdd: agl::Parameter<T, S32>,
    mArmorCompSwimEnergyRate: agl::Parameter<T, F32>,
    mArmorCompRegistElecFrame: agl::Parameter<T, F32>,
    mArmorCompNightSpeedRate: agl::Parameter<T, F32>,
    mArmorCompClimbJumpEnergyRate: agl::Parameter<T, F32>,
    mArmorCompPlusDropRate: agl::Parameter<T, F32>,
    mArmorCompWeaponBrakeRate: agl::Parameter<T, F32>,
    mArmorCompSwordBeamAttackRate: agl::Parameter<T, F32>,
    mArmorCompAncientAttackRate: agl::Parameter<T, F32>,
    mArmorCompBoneAttackRate: agl::Parameter<T, F32>,
    mArmorCompTerrorLevel: agl::Parameter<T, F32>,
    mArmorCompTerrorRadius: agl::Parameter<T, F32>,
    mArmorCompNakedSwimSpeedRate: agl::Parameter<T, F32>,
    mArmorCompNakedSwimAnimeRate: agl::Parameter<T, F32>,
    mArmorCompNakedSwimEnergyRate: agl::Parameter<T, F32>,
    mArmorAncientAttackRate: agl::Parameter<T, F32>,
    mSupportWindNum: agl::Parameter<T, S32>,
    mSupportElectricNum: agl::Parameter<T, S32>,
    mSupportElectricEnergy: agl::Parameter<T, F32>,
    mSupportFireNum: agl::Parameter<T, S32>,
    mSupportWaterLifeAdd: agl::Parameter<T, S32>,
    mSupportWaterEnergyAdd: agl::Parameter<T, F32>,
    mStickRInputFrame: agl::Parameter<T, F32>,
    mDiffAngleFromLookVec: agl::Parameter<T, F32>,
    mLookPosOffset: agl::Parameter<T, F32>,
    mLookFixAngle: agl::Parameter<T, F32>,
    mLookContinueTimeToCamera: agl::Parameter<T, F32>,
    mCutKnockBackNoCrrDist: agl::Parameter<T, F32>,
    mWaitUnsteadyApplyVel: agl::Parameter<T, F32>,
    mCurseAddWeight: agl::Parameter<T, F32>,
    mRoofCrashVel: agl::Parameter<T, F32>,
    mCutJumpInvalidTime: agl::Parameter<T, F32>,
    mWaterDepthInGrudge: agl::Parameter<T, F32>,
    mLargeHorseLegBendAngY: agl::Parameter<T, F32>,
    mLargeHorseLegBendAngX: agl::Parameter<T, F32>,
    mLargeHorseLegBendFrame: agl::Parameter<T, F32>,
    mNoMaskPauseWaterHeight: agl::Parameter<T, F32>,
    mLookAtThreshold: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectPrey<T> {
    base: GParamListObj<T>,
    mEatActorFindRadius: agl::Parameter<T, F32>,
    mEatActorFindRotDegree: agl::Parameter<T, F32>,
    mWaitTimeForStartEat: agl::Parameter<T, F32>,
    mIsEnableGroupEscape: agl::Parameter<T, Bool32>,
    mAimEscapeOffsetRate: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectRod<T> {
    base: GParamListObj<T>,
    mMagicName: agl::Parameter<T, sead::SafeString<T>>,
    mChargeMagicNum: agl::Parameter<T, S32>,
    mChargeMagicInterval: agl::Parameter<T, S32>,
    mMagicPower: agl::Parameter<T, S32>,
    mMagicSpeed: agl::Parameter<T, F32>,
    mMagicSpeedByThrow: agl::Parameter<T, F32>,
    mMagicGravity: agl::Parameter<T, F32>,
    mMagicRadius: agl::Parameter<T, F32>,
    mScaleTime: agl::Parameter<T, S32>,
    mMagicRange: agl::Parameter<T, F32>,
    mMagicSpeedByEnemy: agl::Parameter<T, F32>,
    mMagicGravityByEnemy: agl::Parameter<T, F32>,
    mMagicRadiusByEnemy: agl::Parameter<T, F32>,
    mScaleTimeByEnemy: agl::Parameter<T, S32>,
    mMagicRangeByEnemy: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectRope<T> {
    base: GParamListObj<T>,
    mIsAllowCutting: agl::Parameter<T, Bool32>,
    mIsSetupKeyframed: agl::Parameter<T, Bool32>,
    mBoneEffectiveLength: agl::Parameter<T, F32>,
    mIsInterpolateEdge: agl::Parameter<T, Bool32>,
    mIsDeformable: agl::Parameter<T, Bool32>,
    mIsOneBoneOneShape: agl::Parameter<T, Bool32>,
    mSplineOffsetRateA: agl::Parameter<T, F32>,
    mSplineOffsetRateB: agl::Parameter<T, F32>,
    mSplineOffsetRateC: agl::Parameter<T, F32>,
    mMtxEndPosOffsetLength: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectRupee<T> {
    base: GParamListObj<T>,
    mRupeeValue: agl::Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectSandworm<T> {
    base: GParamListObj<T>,
    mSandWidth: agl::Parameter<T, F32>,
    mSandLength: agl::Parameter<T, F32>,
    mSandHeight: agl::Parameter<T, F32>,
    mSandCombSpan: agl::Parameter<T, F32>,
    mSandCombHeight: agl::Parameter<T, F32>,
    mSnakeModelOffsetZ: agl::Parameter<T, F32>,
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
    mSnakeNodeChaseInterval: agl::Parameter<T, F32>,
    mShowLifeGageDist: agl::Parameter<T, F32>,
    mShowLifeGageOffset: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectSeriesArmor<T> {
    base: GParamListObj<T>,
    mSeriesType: agl::Parameter<T, sead::SafeString<T>>,
    mEnableCompBonus: agl::Parameter<T, Bool32>,
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
    mRideBreakRatio: agl::Parameter<T, F32>,
    mMirrorLevel: agl::Parameter<T, S32>,
    mWeaponSubType: agl::Parameter<T, sead::SafeString<T>>,
    mSurfingFriction: agl::Parameter<T, F32>,
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
    mSwarmSubActorNum: agl::Parameter<T, S32>,
    mSwarmPattern: agl::Parameter<T, S32>,
    mDeadActorName: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectSystem<T> {
    base: GParamListObj<T>,
    mSameGroupActorName: agl::Parameter<T, sead::SafeString<T>>,
    mIsGetItemSelf: agl::Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectTraveler<T> {
    base: GParamListObj<T>,
    mAppearGameDataName: agl::Parameter<T, sead::SafeString<T>>,
    mDeleteGameDataName: agl::Parameter<T, sead::SafeString<T>>,
    mRouteType: agl::Parameter<T, sead::SafeString<T>>,
    mRideHorseName: agl::Parameter<T, sead::SafeString<T>>,
    mIsLeadHorse: agl::Parameter<T, Bool32>,
    mHorseGearLevel: agl::Parameter<T, S32>,
    mRoutePoints: RoutePoints<T>,
    mRoutePoint29Name: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectWeaponCommon<T> {
    base: GParamListObj<T>,
    mPlayerEqScale: agl::Parameter<T, F32>,
    mEnemyEqScale: agl::Parameter<T, F32>,
    mGuardPower: agl::Parameter<T, S32>,
    mRank: agl::Parameter<T, S32>,
    mIsHammer: agl::Parameter<T, Bool32>,
    mIsWeakBreaker: agl::Parameter<T, Bool32>,
    mIsBoomerang: agl::Parameter<T, Bool32>,
    mIsBlunt: agl::Parameter<T, Bool32>,
    mIsLuckyWeapon: agl::Parameter<T, Bool32>,
    mIsPikohan: agl::Parameter<T, Bool32>,
    mIsThrowingWeapon: agl::Parameter<T, Bool32>,
    mIsThrowingBreakWeapon: agl::Parameter<T, Bool32>,
    mThrowRange: agl::Parameter<T, F32>,
    mDreadActor: agl::Parameter<T, sead::SafeString<T>>,
    mThroughActor: agl::Parameter<T, sead::SafeString<T>>,
    mNPCWeaponType: agl::Parameter<T, sead::SafeString<T>>,
    mIsNotOnTerrorHold: agl::Parameter<T, Bool32>,
    mIsAsOffUnEquiped: agl::Parameter<T, Bool32>,
    mChemicalEnergyMax: agl::Parameter<T, S32>,
    mChemicalEnergyAmountUsed: agl::Parameter<T, S32>,
    mChemicalEnergyRecoverRate: agl::Parameter<T, F32>,
    mChemicalEnergyRecoverInterval: agl::Parameter<T, S32>,
    mStickDamage: agl::Parameter<T, S32>,
    mShootBeam: agl::Parameter<T, sead::SafeString<T>>,
    mDropFromPorchRot: agl::Parameter<T, sead::Vector3f>,
    mSharpWeaponPer: agl::Parameter<T, F32>,
    mSharpWeaponAddAtkMin: agl::Parameter<T, S32>,
    mSharpWeaponAddAtkMax: agl::Parameter<T, S32>,
    mSharpWeaponAddLifeMin: agl::Parameter<T, S32>,
    mSharpWeaponAddLifeMax: agl::Parameter<T, S32>,
    mSharpWeaponAddCrit: agl::Parameter<T, Bool32>,
    mSharpWeaponAddGuardMin: agl::Parameter<T, S32>,
    mSharpWeaponAddGuardMax: agl::Parameter<T, S32>,
    mPoweredSharpAddAtkMin: agl::Parameter<T, S32>,
    mPoweredSharpAddAtkMax: agl::Parameter<T, S32>,
    mPoweredSharpAddLifeMin: agl::Parameter<T, S32>,
    mPoweredSharpAddLifeMax: agl::Parameter<T, S32>,
    mPoweredSharpWeaponAddGuardMin: agl::Parameter<T, S32>,
    mPoweredSharpWeaponAddGuardMax: agl::Parameter<T, S32>,
    mPoweredSharpAddThrowMin: agl::Parameter<T, F32>,
    mPoweredSharpAddThrowMax: agl::Parameter<T, F32>,
    mPoweredSharpAddSpreadFire: agl::Parameter<T, Bool32>,
    mPoweredSharpAddZoomRapid: agl::Parameter<T, Bool32>,
    mPoweredSharpAddRapidFireMin: agl::Parameter<T, F32>,
    mPoweredSharpAddRapidFireMax: agl::Parameter<T, F32>,
    mPoweredSharpAddSurfMaster: agl::Parameter<T, Bool32>,
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
    mThrowSpeed: agl::Parameter<T, F32>,
    mThrowRotSpeed: agl::Parameter<T, F32>,
    mThrowDist: agl::Parameter<T, F32>,
    mThrowRigidBodyBaseAxis: agl::Parameter<T, sead::Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectWizzrobe<T> {
    base: GParamListObj<T>,
    mMagicWeatherType: agl::Parameter<T, S32>,
    mMagicFallActorName: agl::Parameter<T, sead::SafeString<T>>,
    mMagicFallIgniteRotSpd: agl::Parameter<T, F32>,
    mMagicFallOffsetY: agl::Parameter<T, F32>,
    mMagicFallCenterOffsetXZ: agl::Parameter<T, F32>,
    mMagicFallRandRadius: agl::Parameter<T, F32>,
    mMagicFallIntervalMax: agl::Parameter<T, F32>,
    mMagicFallIntervalMin: agl::Parameter<T, F32>,
    mSummonActorName: agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectWolfLink<T> {
    base: GParamListObj<T>,
    mNeckSpeedWait: agl::Parameter<T, F32>,
    mNeckRateWait: agl::Parameter<T, F32>,
    mNeckSpeedShiekSensor: agl::Parameter<T, F32>,
    mNeckRateShiekSensor: agl::Parameter<T, F32>,
    mNeckSpeedFollow: agl::Parameter<T, F32>,
    mNeckRateFollow: agl::Parameter<T, F32>,
    mNeckSpeedBattle: agl::Parameter<T, F32>,
    mNeckRateBattle: agl::Parameter<T, F32>,
    mNeckSpeedHeal: agl::Parameter<T, F32>,
    mNeckRateHeal: agl::Parameter<T, F32>,
    mBattleRange: agl::Parameter<T, F32>,
    mHealRange: agl::Parameter<T, F32>,
    mHuntRange: agl::Parameter<T, F32>,
    mHowlRange: agl::Parameter<T, F32>,
    mMaxHeightAttackable: agl::Parameter<T, F32>,
    mMaxHeightHealable: agl::Parameter<T, F32>,
    mNavMeshSearchRadius: agl::Parameter<T, F32>,
    mCanReachPlayerNavMeshSearchRadius: agl::Parameter<T, F32>,
    mSubmergedDepth: agl::Parameter<T, F32>,
    mUtilityLifeToHunt: agl::Parameter<T, F32>,
    mUtilityDangerDistMin: agl::Parameter<T, F32>,
    mUtilityDangerDistMax: agl::Parameter<T, F32>,
    mUtilityConstant: agl::Parameter<T, F32>,
    mChainAttackChargeMin: agl::Parameter<T, F32>,
    mChainAttackChargeMax: agl::Parameter<T, F32>,
    mLookAtCooldownWait: agl::Parameter<T, F32>,
    mLookAtCooldownWaitRand: agl::Parameter<T, F32>,
    mLookAtCounterWait: agl::Parameter<T, F32>,
    mLookAtCounterWaitRand: agl::Parameter<T, F32>,
    mLookAtCooldownRun: agl::Parameter<T, F32>,
    mLookAtCooldownRunRand: agl::Parameter<T, F32>,
    mLookAtCounterRun: agl::Parameter<T, F32>,
    mLookAtCounterRunRand: agl::Parameter<T, F32>,
    mAttackCounterLength: agl::Parameter<T, F32>,
    mAttackCounterRand: agl::Parameter<T, F32>,
    mHowlCooldownCounterLength: agl::Parameter<T, F32>,
    mHowlCooldownCounterRand: agl::Parameter<T, F32>,
    mHealCooldownCounterLength: agl::Parameter<T, F32>,
    mHealCooldownCounterRand: agl::Parameter<T, F32>,
    mFailPathCooldownCounterLength: agl::Parameter<T, F32>,
    mFailPathCooldownCounterRand: agl::Parameter<T, F32>,
    mRetargetCooldownCounterLength: agl::Parameter<T, F32>,
    mRetargetCooldownCounterRand: agl::Parameter<T, F32>,
    mAfterTargetDeathCounterLength: agl::Parameter<T, F32>,
    mAfterTargetDeathCounterRand: agl::Parameter<T, F32>,
    mLostTargetCounterLength: agl::Parameter<T, F32>,
    mLostTargetCounterRand: agl::Parameter<T, F32>,
    mInvinceableCounterLength: agl::Parameter<T, F32>,
    mInvinceableCounterRand: agl::Parameter<T, F32>,
    mCallDelayMinLength: agl::Parameter<T, F32>,
    mCallOverrideCounterLength: agl::Parameter<T, F32>,
    mGiveUpShiekSensorLength: agl::Parameter<T, F32>,
    mRetryShiekSensorLength: agl::Parameter<T, F32>,
    mBattleWallHitLength: agl::Parameter<T, F32>,
    mFollowRetryLength: agl::Parameter<T, F32>,
    mPowerUpFoodLength: agl::Parameter<T, F32>,
    mSafePosFailCounter: agl::Parameter<T, F32>,
    mRestrictedTargetTimeNormal: agl::Parameter<T, F32>,
    mRestrictedTargetTimeSpecial: agl::Parameter<T, F32>,
    mPowerUpFoodAttackMod: agl::Parameter<T, S32>,
    mPowerUpFoodChainAttackCharge: agl::Parameter<T, F32>,
    mVSStalfosCritChance: agl::Parameter<T, S32>,
    mAttackBase: agl::Parameter<T, F32>,
    mAttackHeartMod: agl::Parameter<T, F32>,
    mDefenseBase: agl::Parameter<T, F32>,
    mDefenseHeartMod: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectZora<T> {
    base: GParamListObj<T>,
    mInWaterDepth: agl::Parameter<T, F32>,
    mFloatDepth: agl::Parameter<T, F32>,
    mFloatRadius: agl::Parameter<T, F32>,
    mFloatCycleTime: agl::Parameter<T, F32>,
    mChangeDepthSpeed: agl::Parameter<T, F32>,
}

#[repr(C)]
pub struct DirectionInfo<T> {
    mEntryPoint: agl::Parameter<T, sead::SafeString<T>>,
    mWaitFrame:  agl::Parameter<T, F32>,
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
