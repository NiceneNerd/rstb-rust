use super::*;
use super::agl::*;

#[repr(C)]
struct GParamListObj<T> {
    vfptr: T,
    mObj: ParameterObj<T>,
}
#[repr(C)]
pub struct GParamListObjectAirWall<T> {
    base: GParamListObj<T>,
    mLayer: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectAnimalFollowOffset<T> {
    base: GParamListObj<T>,
    mLayer: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectAnimalUnit<T> {
    base: GParamListObj<T>,
    mSpeedActorName: Parameter<T, SafeString<T>>,
    mBasePlayRate: Parameter<T, F32>,
    mGearMaxNum: Parameter<T, S32>,
    mIsSetWaitASAtGear0: Parameter<T, Bool32>,
    mStressFramesMin: Parameter<T, F32>,
    mStressFramesMax: Parameter<T, F32>,
    mSteeringOutputKp: Parameter<T, F32>,
    mSteeringOutputKi: Parameter<T, F32>,
    mSteeringOutputKd: Parameter<T, F32>,
    mSteeringOutputIClamp: Parameter<T, F32>,
    mSteeringOutputIReduceRatio: Parameter<T, F32>,
    mSteeringOutputDLerpRatio: Parameter<T, F32>,
    mSteeringOutputAvoidanceLerpRatio: Parameter<T, F32>,
    mSteeringOutputIIRLerpRatio: Parameter<T, F32>,
    mOverrideSteeringOutputKp: Parameter<T, F32>,
    mOverrideSteeringOutputKi: Parameter<T, F32>,
    mOverrideSteeringOutputKd: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectArmor<T> {
    base: GParamListObj<T>,
    mStarNum: Parameter<T, S32>,
    mDefenceAddLevel: Parameter<T, S32>,
    mWindScaleMesh: Parameter<T, SafeString<T>>,
    mWindScale: Parameter<T, F32>,
    mNextRankName: Parameter<T, SafeString<T>>,
    mAffectTransOffsetShield: Parameter<T, Vector3f>,
    mAffectRotOffsetShield: Parameter<T, Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectArmorEffect<T> {
    base: GParamListObj<T>,
    mEffectType: Parameter<T, SafeString<T>>,
    mEffectLevel: Parameter<T, S32>,
    mAncientPowUp: Parameter<T, Bool32>,
    mEnableClimbWaterfall: Parameter<T, Bool32>,
    mEnableSpinAttack: Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectArmorHead<T> {
    base: GParamListObj<T>,
    mEarRotate: Parameter<T, Vector3f>,
    mMantleType: Parameter<T, S32>,
    mMaskType: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectArmorUpper<T> {
    base: GParamListObj<T>,
    mIsDispOffPorch: Parameter<T, Bool32>,
    mShiekerStoneTransOffset: Parameter<T, Vector3f>,
    mShiekerStoneRotOffset: Parameter<T, Vector3f>,
    mDisableSelfMantle: Parameter<T, Bool32>,
    mUseMantleType: Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectArrow<T> {
    base: GParamListObj<T>,
    mArrowNum: Parameter<T, S32>,
    mDeleteTime: Parameter<T, S32>,
    mDeleteTimeWithChemical: Parameter<T, S32>,
    mEnemyShootNumForDelete: Parameter<T, S32>,
    mArrowDeletePer: Parameter<T, S32>,
    mExtraDamage: Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectAttack<T> {
    base: GParamListObj<T>,
    mPower: Parameter<T, S32>,
    mImpulse: Parameter<T, S32>,
    mImpulseLarge: Parameter<T, S32>,
    mRange: Parameter<T, F32>,
    mGuardBreakPower: Parameter<T, S32>,
    mSpHitActor: Parameter<T, SafeString<T>>,
    mSpHitTag: Parameter<T, SafeString<T>>,
    mSpHitRatio: Parameter<T, F32>,
    mSpWeakHitActor: Parameter<T, SafeString<T>>,
    mPowerForPlayer: Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectAttackInterval<T> {
    base: GParamListObj<T>,
    mShortAttackTimeMin: Parameter<T, S32>,
    mShortAttackTimeMax: Parameter<T, S32>,
    mMiddleAttackTimeMin: Parameter<T, S32>,
    mMiddleAttackTimeMax: Parameter<T, S32>,
    mLongAttackTimeMin: Parameter<T, S32>,
    mLongAttackTimeMax: Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectAutoGen<T> {
    base: GParamListObj<T>,
    mSetName: Parameter<T, SafeString<T>>,
    mKeyActorName: Parameter<T, SafeString<T>>,
    mSetRadius: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectBeam<T> {
    base: GParamListObj<T>,
    mBeamLevel: Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectBindActor<T> {
    base: GParamListObj<T>,
    mBindActorName: Parameter<T, SafeString<T>>,
    mIsKeepSleep: Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectBindBone<T> {
    base: GParamListObj<T>,
    mBoneName: Parameter<T, SafeString<T>>,
    mBoneOffset: Parameter<T, Vector3f>,
    mBoneRotate: Parameter<T, Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectBow<T> {
    base: GParamListObj<T>,
    mQuiverName: Parameter<T, SafeString<T>>,
    mArrowName: Parameter<T, SafeString<T>>,
    mIsGuardPierce: Parameter<T, Bool32>,
    mExtraDamageRatio: Parameter<T, F32>,
    mBaseAttackPowerRatio: Parameter<T, F32>,
    mIsLeadShot: Parameter<T, Bool32>,
    mLeadShotNum: Parameter<T, S32>,
    mLeadShotAng: Parameter<T, F32>,
    mLeadShotInterval: Parameter<T, S32>,
    mIsRapidFire: Parameter<T, Bool32>,
    mRapidFireNum: Parameter<T, S32>,
    mRapidFireInterval: Parameter<T, S32>,
    mIsLongRange: Parameter<T, Bool32>,
    mArrowFirstSpeed: Parameter<T, F32>,
    mArrowAcceleration: Parameter<T, F32>,
    mArrowStabilitySpeed: Parameter<T, F32>,
    mArrowFallAcceleration: Parameter<T, F32>,
    mArrowFallStabilitySpeed: Parameter<T, F32>,
    mArrowGravity: Parameter<T, F32>,
    mPlayerHoldTransOffset: Parameter<T, Vector3f>,
    mPlayerHoldRotOffset: Parameter<T, Vector3f>,
    mPlayerEquipTransOffset: Parameter<T, Vector3f>,
    mPlayerEquipRotOffset: Parameter<T, Vector3f>,
    mSquatPlayerHoldTransAddOffset: Parameter<T, Vector3f>,
    mSquatPlayerHoldRotAddOffset: Parameter<T, Vector3f>,
    mNPCHoldTransOffset: Parameter<T, Vector3f>,
    mNPCHoldRotOffset: Parameter<T, Vector3f>,
    mNPCEquipTransOffset: Parameter<T, Vector3f>,
    mNPCEquipRotOffset: Parameter<T, Vector3f>,
    mEnemyEquipTransOffset: Parameter<T, Vector3f>,
    mEnemyEquipRotOffset: Parameter<T, Vector3f>,
    mStandEquipTransOffset: Parameter<T, Vector3f>,
    mStandEquipRotOffset: Parameter<T, Vector3f>,
    mArrowChargeRate: Parameter<T, F32>,
    mArrowReloadRate: Parameter<T, F32>,
    mWeaponSubType: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectBullet<T> {
    base: GParamListObj<T>,
    mNoHitParent: Parameter<T, Bool32>,
    mIsLimitCount: Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectCamera<T> {
    base: GParamListObj<T>,
    mDefaultConnectScaleAfterEvent: Parameter<T, F32>,
    mLatConnectRateAfterEvent: Parameter<T, F32>,
    mLngConnectRateAfterEvent: Parameter<T, F32>,
    mDistConnectRateAfterEvent: Parameter<T, F32>,
    mFovyConnectRateAfterEvent: Parameter<T, F32>,
    mConnectAfterEventMin: Parameter<T, F32>,
    mConnectAfterEventMax: Parameter<T, F32>,
    mRoofGradientNearHighWeight: Parameter<T, F32>,
    mRoofGradientFarHighWeight: Parameter<T, F32>,
    mRoofGradientNearLowWeight: Parameter<T, F32>,
    mRoofGradientFarLowWeight: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectChemicalType<T> {
    base: GParamListObj<T>,
    mChemicalType: Parameter<T, SafeString<T>>,
    mEmitChemicalActor: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectClothReaction<T> {
    base: GParamListObj<T>,
    mAtkCollidableName: Parameter<T, SafeString<T>>,
    mAtkCollidableBoneName: Parameter<T, SafeString<T>>,
    mAtkCollidableSpeed: Parameter<T, F32>,
    mAtkCollidableActiveTime: Parameter<T, F32>,
    mAtkCollidableResetPos: Parameter<T, Vector3f>,
    mGroundCollidableName: Parameter<T, SafeString<T>>,
    mGroundCollidableBoneName: Parameter<T, SafeString<T>>,
    mGroundCollidableOffset: Parameter<T, F32>,
    mUseGroundCollidableResetPos: Parameter<T, Bool32>,
    mGroundCollidableResetPos: Parameter<T, Vector3f>,
    mGroundCollidableMoveSpeed: Parameter<T, F32>,
    mWallCollidableName: Parameter<T, SafeString<T>>,
    mWallCollidableBoneName: Parameter<T, SafeString<T>>,
    mWallCollidableOffset: Parameter<T, F32>,
    mPlayerCollidableName: Parameter<T, SafeString<T>>,
    mPlayerCollidableBoneName: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectCookSpice<T> {
    base: GParamListObj<T>,
    mBoostHitPointRecover: Parameter<T, S32>,
    mBoostEffectiveTime: Parameter<T, S32>,
    mBoostSuccessRate: Parameter<T, S32>,
    mBoostMaxHeartLevel: Parameter<T, S32>,
    mBoostStaminaLevel: Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectCureItem<T> {
    base: GParamListObj<T>,
    mHitPointRecover: Parameter<T, S32>,
    mEffectType: Parameter<T, SafeString<T>>,
    mEffectLevel: Parameter<T, S32>,
    mEffectiveTime: Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectEatTarget<T> {
    base: GParamListObj<T>,
    mFavoriteEatActorNames: Parameter<T, SafeString<T>>,
    mFavoriteEatActorTags: Parameter<T, SafeString<T>>,
    mEatActorNames: Parameter<T, SafeString<T>>,
    mEatActorNames2: Parameter<T, SafeString<T>>,
    mEatActorNames3: Parameter<T, SafeString<T>>,
    mEatActorTags: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectEnemy<T> {
    base: GParamListObj<T>,
    mRank: Parameter<T, S32>,
    mPower: Parameter<T, S32>,
    mDropLife: Parameter<T, S32>,
    mDyingLife: Parameter<T, S32>,
    mLostDist: Parameter<T, F32>,
    mLostHeightMax: Parameter<T, F32>,
    mLostHeightMin: Parameter<T, F32>,
    mLostRayLength: Parameter<T, F32>,
    mLODLostDist: Parameter<T, F32>,
    mLODLostHeightMax: Parameter<T, F32>,
    mLODLostHeightMin: Parameter<T, F32>,
    mIntelligenceLevel: Parameter<T, F32>,
    mEmotionalLevel: Parameter<T, F32>,
    mHeroismLevel: Parameter<T, F32>,
    mPartActorName: Parameter<T, SafeString<T>>,
    mIsMindFriend: Parameter<T, Bool32>,
    mStatusChangeFlag: Parameter<T, SafeString<T>>,
    mChangeLife: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectEnemyLevel<T> {
    base: GParamListObj<T>,
    mIsAvoidDanger: Parameter<T, Bool32>,
    mIsGuardArrow: Parameter<T, Bool32>,
    mIsHideArrowAttack: Parameter<T, Bool32>,
    mIsSwiftAttack: Parameter<T, Bool32>,
    mIsBackSwiftAttack: Parameter<T, Bool32>,
    mIsCounterAttack: Parameter<T, Bool32>,
    mIsEscapeBomb: Parameter<T, Bool32>,
    mIsKickBomb: Parameter<T, Bool32>,
    mIsShootBomb: Parameter<T, Bool32>,
    mIsThrowWeapon: Parameter<T, Bool32>,
    mGuardPer: Parameter<T, S32>,
    mIsJustGuard: Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectEnemyRace<T> {
    base: GParamListObj<T>,
    mEquipableWeapon: Parameter<T, SafeString<T>>,
    mIsFitGroundByAnimalUnit: Parameter<T, Bool32>,
    mIsUpdateSupportNormalInAir: Parameter<T, Bool32>,
    mBowAttackRangeRatio: Parameter<T, F32>,
    mWeaponScaleSmallSword: Parameter<T, F32>,
    mWeaponTransOffsetSmallSword: Parameter<T, Vector3f>,
    mWeaponRotOffsetSmallSword: Parameter<T, Vector3f>,
    mWeaponHoldTransOffsetSmallSword: Parameter<T, Vector3f>,
    mWeaponHoldRotOffsetSmallSword: Parameter<T, Vector3f>,
    mWeaponScaleLargeSword: Parameter<T, F32>,
    mWeaponTransOffsetLargeSword: Parameter<T, Vector3f>,
    mWeaponRotOffsetLargeSword: Parameter<T, Vector3f>,
    mWeaponHoldTransOffsetLargeSword: Parameter<T, Vector3f>,
    mWeaponHoldRotOffsetLargeSword: Parameter<T, Vector3f>,
    mWeaponScaleSpear: Parameter<T, F32>,
    mWeaponTransOffsetSpear: Parameter<T, Vector3f>,
    mWeaponRotOffsetSpear: Parameter<T, Vector3f>,
    mWeaponHoldTransOffsetSpear: Parameter<T, Vector3f>,
    mWeaponHoldRotOffsetSpear: Parameter<T, Vector3f>,
    mWeaponScaleBow: Parameter<T, F32>,
    mWeaponTransOffsetBow: Parameter<T, Vector3f>,
    mWeaponRotOffsetBow: Parameter<T, Vector3f>,
    mWeaponHoldTransOffsetBow: Parameter<T, Vector3f>,
    mWeaponHoldRotOffsetBow: Parameter<T, Vector3f>,
    mWeaponScaleShield: Parameter<T, F32>,
    mWeaponTransOffsetShield: Parameter<T, Vector3f>,
    mWeaponRotOffsetShield: Parameter<T, Vector3f>,
    mWeaponHoldTransOffsetShield: Parameter<T, Vector3f>,
    mWeaponHoldRotOffsetShield: Parameter<T, Vector3f>,
    mIsUseTargetTag: Parameter<T, Bool32>,
    mTargetActorType: Parameter<T, SafeString<T>>,
    mEscapeAttackedActorType: Parameter<T, SafeString<T>>,
    mReactionBalloon: Parameter<T, Bool32>,
    mSmallRagdollTime: Parameter<T, S32>,
    mSmallRagdollRecoverTime: Parameter<T, S32>,
    mSmallLargeRagdollTime: Parameter<T, S32>,
    mSmallLargeRagdollRecoverTime: Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectEnemyShown<T> {
    base: GParamListObj<T>,
    mIsHappy: Parameter<T, Bool32>,
    mIsCasebyCase: Parameter<T, Bool32>,
    mIsSit: Parameter<T, Bool32>,
    mIsNoise: Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectEvent<T> {
    base: GParamListObj<T>,
    mVisibleOffActor1: Parameter<T, SafeString<T>>,
    mVisibleOffActor2: Parameter<T, SafeString<T>>,
    mVisibleOffActor3: Parameter<T, SafeString<T>>,
    mVisibleOffActor4: Parameter<T, SafeString<T>>,
    mVisibleOffActor5: Parameter<T, SafeString<T>>,
    mVisibleOffActor6: Parameter<T, SafeString<T>>,
    mVisibleOffActor7: Parameter<T, SafeString<T>>,
    mVisibleOffActor8: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectExtendedEntity<T> {
    base: GParamListObj<T>,
    mIsUsePivotAdjustRange: Parameter<T, Bool32>,
    mPivotAdjustRange: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectFish<T> {
    base: GParamListObj<T>,
    mRestoreSpeedRate: Parameter<T, F32>,
    mRestoreSpeedRateAdd: Parameter<T, F32>,
    mLimitAngle: Parameter<T, F32>,
    mLimitAngleAdd: Parameter<T, F32>,
    mPrevSpeedRate: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectGelEnemy<T> {
    base: GParamListObj<T>,
    mMoveBoneName: Parameter<T, SafeString<T>>,
    mBodyRadius: Parameter<T, F32>,
    mClothBoneNumForEyeCalc: Parameter<T, S32>,
    mBodyRootBoneName: Parameter<T, SafeString<T>>,
    mLeftEyeBoneName: Parameter<T, SafeString<T>>,
    mRightEyeBoneName: Parameter<T, SafeString<T>>,
    mEyeSpaceHalf: Parameter<T, F32>,
    mEyeDir: Parameter<T, Vector3f>,
    mEyeOffset: Parameter<T, Vector3f>,
    mEyeUpMoveRate: Parameter<T, F32>,
    mEyeDownMoveRate: Parameter<T, F32>,
    mIsAverageEyePos: Parameter<T, Bool32>,
    mEyeDelayAccRate: Parameter<T, F32>,
    mEyeYMoveTheta: Parameter<T, F32>,
    mEyeYMoveFrequency: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectGeneral<T> {
    base: GParamListObj<T>,
    mSpeed: Parameter<T, F32>,
    mLife: Parameter<T, S32>,
    mIsLifeInfinite: Parameter<T, Bool32>,
    mElectricalDischarge: Parameter<T, F32>,
    mIsBurnOutBorn: Parameter<T, Bool32>,
    mBurnOutBornName: Parameter<T, SafeString<T>>,
    mIsBurnOutBornIdent: Parameter<T, Bool32>,
    mChangeDropTableName: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectGiantArmor<T> {
    base: GParamListObj<T>,
    mDamageScale: Parameter<T, F32>,
    mRotOffset: Parameter<T, Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectGiantArmorSlot<T> {
    base: GParamListObj<T>,
    mSlot0Node: Parameter<T, SafeString<T>>,
    mSlot0RigidBody: Parameter<T, SafeString<T>>,
    mSlot0DefaultActorName: Parameter<T, SafeString<T>>,
    mSlot1Node: Parameter<T, SafeString<T>>,
    mSlot1RigidBody: Parameter<T, SafeString<T>>,
    mSlot1DefaultActorName: Parameter<T, SafeString<T>>,
    mSlot2Node: Parameter<T, SafeString<T>>,
    mSlot2RigidBody: Parameter<T, SafeString<T>>,
    mSlot2DefaultActorName: Parameter<T, SafeString<T>>,
    mSlot3Node: Parameter<T, SafeString<T>>,
    mSlot3RigidBody: Parameter<T, SafeString<T>>,
    mSlot3DefaultActorName: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectGlobal<T> {
    base: GParamListObj<T>,
    mEnemyLifeGageDist: Parameter<T, F32>,
    mEnemyNoSkitDist: Parameter<T, F32>,
    mEnemyWeaponPickAllowDist: Parameter<T, F32>,
    mEnemyWeaponPickForbidTime: Parameter<T, S32>,
    mEnemyAnimalNoDamageDist: Parameter<T, F32>,
    mEnemyNearCraeteIDDelay: Parameter<T, F32>,
    mEnemyForceTiredLODCount: Parameter<T, S32>,
    mEnemyForceTiredNoSightLODCount: Parameter<T, S32>,
    mEnemyForceWarpReturnLODCount: Parameter<T, S32>,
    mSilentAttackAng: Parameter<T, F32>,
    mSilentAttackRatio: Parameter<T, F32>,
    mBlownOffPlayerAtkDelay: Parameter<T, S32>,
    mJustAvoidAcceptWpRangeSS: Parameter<T, F32>,
    mJustAvoidAcceptWpRangeLS: Parameter<T, F32>,
    mJustAvoidAcceptWpRangeSP: Parameter<T, F32>,
    mForceNoticeEnemyCount: Parameter<T, S32>,
    mForceNoticeEnemyDist: Parameter<T, F32>,
    mWeaponRickeyLife: Parameter<T, S32>,
    mWeaponDropRotSpd: Parameter<T, F32>,
    mShieldRideBaseFrame: Parameter<T, S32>,
    mShieldRideHitBaseDamage: Parameter<T, S32>,
    mShieldDamageratio: Parameter<T, F32>,
    mShieldSurfMasterFrictionRatio: Parameter<T, F32>,
    mLoudNoiseRadius: Parameter<T, F32>,
    mImpulse2DamageRatio: Parameter<T, F32>,
    mIceMeltSpeedOnContactFire: Parameter<T, F32>,
    mCriticalAttackRatio: Parameter<T, F32>,
    mBooerangAttackRatio: Parameter<T, F32>,
    mHitImpulseClampMax: Parameter<T, F32>,
    mDropItemVelXZFromBomb: Parameter<T, F32>,
    mDropItemVelYFromBomb: Parameter<T, F32>,
    mDropItemVelRandomFromBomb: Parameter<T, F32>,
    mDropItemAngVelFromBomb: Parameter<T, F32>,
    mDropItemAngVelRandomFromBomb: Parameter<T, F32>,
    mDropItemVelXZSmall: Parameter<T, F32>,
    mDropItemVelYSmall: Parameter<T, F32>,
    mDropItemVelRandomSmall: Parameter<T, F32>,
    mDropItemAngVelSmall: Parameter<T, F32>,
    mDropItemAngVelRandomSmall: Parameter<T, F32>,
    mDropItemVelXZLarge: Parameter<T, F32>,
    mDropItemVelYLarge: Parameter<T, F32>,
    mDropItemVelRandomLarge: Parameter<T, F32>,
    mDropItemAngVelLarge: Parameter<T, F32>,
    mDropItemAngVelRandomLarge: Parameter<T, F32>,
    mDropItemVelXZRupeeRabbit: Parameter<T, F32>,
    mDropItemVelYRupeeRabbit: Parameter<T, F32>,
    mDropItemVelRandomRupeeRabbit: Parameter<T, F32>,
    mDropItemVelXZItemRupeeOnly: Parameter<T, F32>,
    mDropItemVelYItemRupeeOnly: Parameter<T, F32>,
    mDropItemVelRandomItemRupeeOnly: Parameter<T, F32>,
    mDropItemInvincibleTime: Parameter<T, F32>,
    mTreeWeaponEquipTransOffset: Parameter<T, Vector3f>,
    mTreeWeaponEquipRotOffset: Parameter<T, Vector3f>,
    mWetRatioToDie: Parameter<T, F32>,
    mEnvWetRatioToDie: Parameter<T, F32>,
    mNPCTurnAngleDiff: Parameter<T, F32>,
    mNPCWaitFrameAfterEvent: Parameter<T, S32>,
    mNPCIgnorePlayerTime: Parameter<T, F32>,
    mNPCCancelIgnorePlayerTime: Parameter<T, F32>,
    mNPCOpenDoorDistance: Parameter<T, F32>,
    mNPCWalkRateOnSandAndSnow: Parameter<T, F32>,
    mNPCDownVerticallyAngle: Parameter<T, F32>,
    mGerudoQueenSafetyAreaRadius: Parameter<T, F32>,
    mCreateFairyLimitCount: Parameter<T, S32>,
    mTerrorRegistSpeed: Parameter<T, F32>,
    mTerrorUnregistSpeed: Parameter<T, F32>,
    mTerrorRegistTimer: Parameter<T, S32>,
    mTerrorRadiusOffset: Parameter<T, F32>,
    mSpeedTerrorLevel: Parameter<T, S32>,
    mSpeedTerrorLevelHuge: Parameter<T, S32>,
    mSpeedTerrorLevelCheckRadius: Parameter<T, F32>,
    mAtDirTypeAffectRatio: Parameter<T, F32>,
    mRainyAwnHearingLevel: Parameter<T, F32>,
    mHorseBindOffsetYOfMaleUMii: Parameter<T, F32>,
    mHorseBindOffsetYOfFemaleUMii: Parameter<T, F32>,
    mHorseFamiliarityIncreasePerFrame: Parameter<T, Vector3f>,
    mHorseFamiliarityIncreaseSootheAtFirstRun: Parameter<T, Vector3f>,
    mHorseFamiliarityIncreaseSootheAfterRun: Parameter<T, Vector3f>,
    mHorseFamiliarityIncreaseSootheAfterGearTop: Parameter<T, Vector3f>,
    mHorseFamiliarityIncreaseSootheAfterJump: Parameter<T, Vector3f>,
    mHorseFamiliarityIncreaseSootheWhileResisting: Parameter<T, Vector3f>,
    mHorseFamiliarityIncreaseEat: Parameter<T, Vector3f>,
    mHorseAlertProbability: Parameter<T, Vector3f>,
    mHorseAlertFramesMin: Parameter<T, Vector3f>,
    mHorseAlertFramesMax: Parameter<T, Vector3f>,
    mHorseExtraChargeNum: Parameter<T, S32>,
    mPlayerGrabThrowDiffRate: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectGolem<T> {
    base: GParamListObj<T>,
    mUpperArmRActor: Parameter<T, SafeString<T>>,
    mLowerArmRActor: Parameter<T, SafeString<T>>,
    mUpperArmLActor: Parameter<T, SafeString<T>>,
    mLowerArmLActor: Parameter<T, SafeString<T>>,
    mDefaultWeakPointActor: Parameter<T, SafeString<T>>,
    mIsDefaultChemicalOn: Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectGolemIK<T> {
    base: GParamListObj<T>,
    mFootRayCheckDist: Parameter<T, F32>,
    mFootDownRatio: Parameter<T, F32>,
    mFootUpRatio: Parameter<T, F32>,
    mKneeExtendL: Parameter<T, F32>,
    mKneeShrinkL: Parameter<T, F32>,
    mFootExtendL: Parameter<T, F32>,
    mFootShrinkL: Parameter<T, F32>,
    mKneeExtendR: Parameter<T, F32>,
    mKneeShrinkR: Parameter<T, F32>,
    mFootExtendR: Parameter<T, F32>,
    mFootShrinkR: Parameter<T, F32>,
    mArmRayCheckDist: Parameter<T, F32>,
    mArmDownRatio: Parameter<T, F32>,
    mArmUpRatio: Parameter<T, F32>,
    mElbowExtendL: Parameter<T, F32>,
    mElbowShrinkL: Parameter<T, F32>,
    mWristExtendL: Parameter<T, F32>,
    mWristShrinkL: Parameter<T, F32>,
    mElbowExtendR: Parameter<T, F32>,
    mElbowShrinkR: Parameter<T, F32>,
    mWristExtendR: Parameter<T, F32>,
    mWristShrinkR: Parameter<T, F32>,
    mWaistRotateRatio: Parameter<T, F32>,
    mWaistMorphRatio: Parameter<T, F32>,
    mWaistResetMorphRatio: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectGrab<T> {
    base: GParamListObj<T>,
    mSlot0Node: Parameter<T, SafeString<T>>,
    mSlot1Node: Parameter<T, SafeString<T>>,
    mSlot2Node: Parameter<T, SafeString<T>>,
    mSlot3Node: Parameter<T, SafeString<T>>,
    mSlot4Node: Parameter<T, SafeString<T>>,
    mSlot5Node: Parameter<T, SafeString<T>>,
    mSlot0PodNode: Parameter<T, SafeString<T>>,
    mSlot1PodNode: Parameter<T, SafeString<T>>,
    mSlot2PodNode: Parameter<T, SafeString<T>>,
    mSlot3PodNode: Parameter<T, SafeString<T>>,
    mSlot4PodNode: Parameter<T, SafeString<T>>,
    mSlot5PodNode: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectGuardian<T> {
    base: GParamListObj<T>,
    mGuardianModelType: Parameter<T, S32>,
    mGuardianControllerType: Parameter<T, S32>,
    mHeadLimitMax: Parameter<T, F32>,
    mHeadLimitMin: Parameter<T, F32>,
    mSightLimitMax: Parameter<T, F32>,
    mSightLimitMin: Parameter<T, F32>,
    mMaxSpeed: Parameter<T, F32>,
    mCannonBoneName: Parameter<T, SafeString<T>>,
    mRapidFireDistance: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectGuardianMini<T> {
    base: GParamListObj<T>,
    mColorType: Parameter<T, S32>,
    mBodyMatName: Parameter<T, SafeString<T>>,
    mGuardJustActor: Parameter<T, SafeString<T>>,
    mBeamName: Parameter<T, SafeString<T>>,
    mLineBeamName: Parameter<T, SafeString<T>>,
    mFinalBeamName: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectGuardianMiniWeapon<T> {
    base: GParamListObj<T>,
    mBindMyNodeName: Parameter<T, SafeString<T>>,
    mVisibleMatNameR: Parameter<T, SafeString<T>>,
    mVisibleMatNameL: Parameter<T, SafeString<T>>,
    mVisibleMatNameB: Parameter<T, SafeString<T>>,
    mVisibleOffMatName: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectHorse<T> {
    base: GParamListObj<T>,
    mIsDecoy: Parameter<T, Bool32>,
    mHasMane: Parameter<T, Bool32>,
    mASVariation: Parameter<T, SafeString<T>>,
    mNature: Parameter<T, S32>,
    mAttackPowerMultiplierGear2: Parameter<T, F32>,
    mAttackPowerMultiplierGear3: Parameter<T, F32>,
    mAttackPowerMultiplierGearTop: Parameter<T, F32>,
    mRunnableFramesAtGearTop: Parameter<T, F32>,
    mGearTopInterval: Parameter<T, F32>,
    mGearTopChargeNum: Parameter<T, S32>,
    mEatActorNames: Parameter<T, SafeString<T>>,
    mEatActorNamesForExtraCharge: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectHorseCreator<T> {
    base: GParamListObj<T>,
    mHorseNames: Parameter<T, SafeString<T>>,
    mLeaderHorseNames: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectHorseObject<T> {
    base: GParamListObj<T>,
    mHideHorseMane: Parameter<T, Bool32>,
    mIsHorseClothDisable: Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectHorseRider<T> {
    base: GParamListObj<T>,
    mRootNode: Parameter<T, SafeString<T>>,
    mSpineNode: Parameter<T, SafeString<T>>,
    mRotBaseNode: Parameter<T, SafeString<T>>,
    mRotAxis: Parameter<T, Vector3f>,
    mRotLimit: Parameter<T, F32>,
    mWeaponTransOffsetSmallSword: Parameter<T, Vector3f>,
    mWeaponTransOffsetLargeSword: Parameter<T, Vector3f>,
    mWeaponTransOffsetSpear: Parameter<T, Vector3f>,
    mWeaponTransOffsetBow: Parameter<T, Vector3f>,
    mWeaponTransOffsetShield: Parameter<T, Vector3f>,
    mLeftFootNode: Parameter<T, SafeString<T>>,
    mLeftFootRotAxis: Parameter<T, Vector3f>,
    mRightFootNode: Parameter<T, SafeString<T>>,
    mRightFootRotAxis: Parameter<T, Vector3f>,
    mFootRotRatio: Parameter<T, F32>,
    mFootRetRotRatio: Parameter<T, F32>,
    mFootRotAngleForKuma: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectHorseTargetedInfo<T> {
    base: GParamListObj<T>,
    mHorseMoveRadius: Parameter<T, F32>,
    mHorseAvoidOffset: Parameter<T, F32>,
    mIsCircularMoveAlways: Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectHorseUnit<T> {
    base: GParamListObj<T>,
    mRiddenAnimalType: Parameter<T, S32>,
    mCalmDownNum: Parameter<T, S32>,
    mRideonAboveASHeight: Parameter<T, F32>,
    mRideonAboveASRadius: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectInsect<T> {
    base: GParamListObj<T>,
    mFireResistanceLevel: Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectItem<T> {
    base: GParamListObj<T>,
    mPlayerUseItem: Parameter<T, Bool32>,
    mSellingPrice: Parameter<T, S32>,
    mBuyingPrice: Parameter<T, S32>,
    mCreatingPrice: Parameter<T, S32>,
    mStainColor: Parameter<T, S32>,
    mSaleRevivalCount: Parameter<T, S32>,
    mUseIconActorName: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectLargeSword<T> {
    base: GParamListObj<T>,
    mPodName: Parameter<T, SafeString<T>>,
    mPlayerHoldTransOffset: Parameter<T, Vector3f>,
    mPlayerHoldRotOffset: Parameter<T, Vector3f>,
    mPlayerEquipTransOffset: Parameter<T, Vector3f>,
    mPlayerEquipRotOffset: Parameter<T, Vector3f>,
    mRideHorsePlayerHoldTransOffset: Parameter<T, Vector3f>,
    mRideHorsePlayerHoldRotOffset: Parameter<T, Vector3f>,
    mAffectTransOffsetShield: Parameter<T, Vector3f>,
    mAffectRotOffsetShield: Parameter<T, Vector3f>,
    mAffectTransOffsetBow: Parameter<T, Vector3f>,
    mAffectRotOffsetBow: Parameter<T, Vector3f>,
    mSquatPlayerHoldTransAddOffset: Parameter<T, Vector3f>,
    mSquatPlayerHoldRotAddOffset: Parameter<T, Vector3f>,
    mNPCHoldTransOffset: Parameter<T, Vector3f>,
    mNPCHoldRotOffset: Parameter<T, Vector3f>,
    mNPCEquipTransOffset: Parameter<T, Vector3f>,
    mNPCEquipRotOffset: Parameter<T, Vector3f>,
    mEnemyEquipTransOffset: Parameter<T, Vector3f>,
    mEnemyEquipRotOffset: Parameter<T, Vector3f>,
    mStandEquipTransOffset: Parameter<T, Vector3f>,
    mStandEquipRotOffset: Parameter<T, Vector3f>,
    mWeaponSubType: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectLiftable<T> {
    base: GParamListObj<T>,
    mLiftType: Parameter<T, SafeString<T>>,
    mThrownMass: Parameter<T, S32>,
    mThrownRotSpd: Parameter<T, Vector3f>,
    mLiftPosOffset: Parameter<T, Vector3f>,
    mLiftRotOffset: Parameter<T, Vector3f>,
    mLiftRotFrame: Parameter<T, S32>,
    mAddLiftRotOffsetList: Parameter<T, SafeString<T>>,
    mChaseLiftRotOffset: Parameter<T, Bool32>,
    mLiftCenterOffset: Parameter<T, Vector3f>,
    mPutPosOffset: Parameter<T, Vector3f>,
    mPutRotOffset: Parameter<T, Vector3f>,
    mPutRotFrame: Parameter<T, S32>,
    mAddPutRotOffsetList: Parameter<T, SafeString<T>>,
    mIsUpdateOffsetEachFrame: Parameter<T, Bool32>,
    mIsUse2MassConstraintMode: Parameter<T, Bool32>,
    mIsSetChemicalParent: Parameter<T, Bool32>,
    mDisableFreezeLift: Parameter<T, Bool32>,
    mDisableBurnLift: Parameter<T, Bool32>,
    mThrowReactionLevel: Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectLumberjackTree<T> {
    base: GParamListObj<T>,
    mImpulseThreshold: Parameter<T, F32>,
    mIsValid: Parameter<T, Bool32>,
    mStumpName: Parameter<T, SafeString<T>>,
    mTrunkName: Parameter<T, SafeString<T>>,
    mWeaponWoodName: Parameter<T, SafeString<T>>,
    mBranchName: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectMasterSword<T> {
    base: GParamListObj<T>,
    mIsMasterSword: Parameter<T, Bool32>,
    mTrueFormAttackPower: Parameter<T, S32>,
    mTrueFormMagicPower: Parameter<T, S32>,
    mTrueFormBreakRatio: Parameter<T, F32>,
    mSearchEvilDist: Parameter<T, F32>,
    mRecoverTime: Parameter<T, S32>,
    mSleepActorName: Parameter<T, SafeString<T>>,
    mTrueFormActorName: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectMonsterShop<T> {
    base: GParamListObj<T>,
    mBuyMamo: Parameter<T, S32>,
    mSellMamo: Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectMotorcycle<T> {
    base: GParamListObj<T>,
    mPitchDampingCoefficient: Parameter<T, F32>,
    mDriftAllowSpeedKPH: Parameter<T, F32>,
    mDriftAbortSpeedKPH: Parameter<T, F32>,
    mDriftAllowSteerRate: Parameter<T, F32>,
    mDriftAbortSteerRate: Parameter<T, F32>,
    mDriftRearAngleRate: Parameter<T, F32>,
    mDriftSpeedRate: Parameter<T, F32>,
    mManualWheelieAllowAngleFront: Parameter<T, F32>,
    mManualWheelieAllowAngleRear: Parameter<T, F32>,
    mManualWheelieLastSec: Parameter<T, F32>,
    mWheelieLastSecInMidAir: Parameter<T, F32>,
    mManualControlProhibitSecAfterWheelie: Parameter<T, F32>,
    mWheelieRevertPower: Parameter<T, F32>,
    mWheelieRevertPowerSec: Parameter<T, F32>,
    mManualWheelieRiseDegDelta: Parameter<T, F32>,
    mWheelieLaunchRiseDegDelta: Parameter<T, F32>,
    mEngineBrakeMaxPower: Parameter<T, F32>,
    mBackwardEngineBrakePower: Parameter<T, F32>,
    mSlipStartAngle: Parameter<T, F32>,
    mSlipThresholdPower: Parameter<T, F32>,
    mSlipPowerMax: Parameter<T, F32>,
    mWristBindRotation: Parameter<T, Vector3f>,
    mWristBindTranslation: Parameter<T, Vector3f>,
    mPostureLimitAngle: Parameter<T, F32>,
    mInvalidPostureLimitSec: Parameter<T, F32>,
    mFallOverThresholdAngle: Parameter<T, F32>,
    mJumpIntervalSec: Parameter<T, F32>,
    mFullEnergyLastSec: Parameter<T, F32>,
    mWheelieLaunchJumpProhibitSec: Parameter<T, F32>,
    mSlowModeTargetSpeedKPH2: Parameter<T, F32>,
    mSlowDriftTargetSpeedKPH2: Parameter<T, F32>,
    mSlowModeTransitionSec: Parameter<T, F32>,
    mSlowSlipThresholdKPH: Parameter<T, F32>,
    mSlowSlipThresholdPower: Parameter<T, F32>,
    mSlowSlipThresholdSec: Parameter<T, F32>,
    mJumpRearWheelRotateRadPerSec: Parameter<T, F32>,
    mWeaponThrowModeSpeedKPH2: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectNest<T> {
    base: GParamListObj<T>,
    mCreateActor: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectNpc<T> {
    base: GParamListObj<T>,
    mIsReactNakedPlayer: Parameter<T, Bool32>,
    mUseAutoLabel: Parameter<T, Bool32>,
    mIsOffPodFromWeapon: Parameter<T, Bool32>,
    mIsRunRainWhenGoToSleep: Parameter<T, Bool32>,
    mIsWalkUnderShelterFromRain: Parameter<T, Bool32>,
    mIsSlowWalkOnSandAndSnow: Parameter<T, Bool32>,
    mIsAlwaysCounterPlayerAttack: Parameter<T, Bool32>,
    mIsNotTurnDetect: Parameter<T, Bool32>,
    mIsNotEscapeFromTerror: Parameter<T, Bool32>,
    mTolerantTime: Parameter<T, S32>,
    mTolerantCount: Parameter<T, S32>,
    mCounterRate: Parameter<T, S32>,
    mChangeSearchModeFlagName: Parameter<T, SafeString<T>>,
    mOnFlagWhenDelete: Parameter<T, SafeString<T>>,
    mOffFlagWhenDelete: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectNpcEquipment<T> {
    base: GParamListObj<T>,
    mIsSetWeaponTypeWhenEquip: Parameter<T, Bool32>,
    mEquipName1: Parameter<T, SafeString<T>>,
    mScale1: Parameter<T, F32>,
    mHoldTransOffset1: Parameter<T, Vector3f>,
    mHoldRotOffset1: Parameter<T, Vector3f>,
    mEquipTransOffset1: Parameter<T, Vector3f>,
    mEquipRotOffset1: Parameter<T, Vector3f>,
    mEquipName2: Parameter<T, SafeString<T>>,
    mScale2: Parameter<T, F32>,
    mHoldTransOffset2: Parameter<T, Vector3f>,
    mHoldRotOffset2: Parameter<T, Vector3f>,
    mEquipTransOffset2: Parameter<T, Vector3f>,
    mEquipRotOffset2: Parameter<T, Vector3f>,
    mEquipName3: Parameter<T, SafeString<T>>,
    mScale3: Parameter<T, F32>,
    mHoldTransOffset3: Parameter<T, Vector3f>,
    mHoldRotOffset3: Parameter<T, Vector3f>,
    mEquipTransOffset3: Parameter<T, Vector3f>,
    mEquipRotOffset3: Parameter<T, Vector3f>,
    mEquipName4: Parameter<T, SafeString<T>>,
    mScale4: Parameter<T, F32>,
    mHoldTransOffset4: Parameter<T, Vector3f>,
    mHoldRotOffset4: Parameter<T, Vector3f>,
    mEquipTransOffset4: Parameter<T, Vector3f>,
    mEquipRotOffset4: Parameter<T, Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectPictureBook<T> {
    base: GParamListObj<T>,
    mLiveSpot1: Parameter<T, S32>,
    mLiveSpot2: Parameter<T, S32>,
    mSpecialDrop: Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectPlayer<T> {
    base: GParamListObj<T>,
    mBombReloadTime1: Parameter<T, F32>,
    mBombReloadTime2: Parameter<T, F32>,
    mStopTimerReloadTime: Parameter<T, F32>,
    mStopTimerBlowAngle: Parameter<T, F32>,
    mStopTimerBlowSpeedLimit: Parameter<T, F32>,
    mStopTimerImpluseMaxCountSmallSword: Parameter<T, S32>,
    mStopTimerImpluseMaxCountLargeSword: Parameter<T, S32>,
    mStopTimerImpluseMaxCountSpear: Parameter<T, S32>,
    mStopTimerCancelDeleteWaitTime: Parameter<T, F32>,
    mStopTimerLongTime: Parameter<T, F32>,
    mStopTimerMiddleTime: Parameter<T, F32>,
    mStopTimerShortTime: Parameter<T, F32>,
    mEnergyTiredValue: Parameter<T, F32>,
    mEnergyBowSlow: Parameter<T, F32>,
    mEnergyPush: Parameter<T, F32>,
    mEnergyCharge: Parameter<T, F32>,
    mEnergyAutoRecover: Parameter<T, F32>,
    mEnergyAutoRecoverInAir: Parameter<T, F32>,
    mEnergyAutoRecoverInvalidTime1: Parameter<T, F32>,
    mEnergyAutoRecoverInvalidTime2: Parameter<T, F32>,
    mColdTempDamageAdd: Parameter<T, F32>,
    mHotTempDamageAdd: Parameter<T, F32>,
    mTempDamage: Parameter<T, F32>,
    mTempEnergyDecDiamAdd: Parameter<T, F32>,
    mTempEnergyDecDegAdd: Parameter<T, F32>,
    mVelDiamSand: Parameter<T, F32>,
    mVelDiamTired: Parameter<T, F32>,
    mStickDiamTired: Parameter<T, F32>,
    mAutoRecoverNum: Parameter<T, F32>,
    mAutoRecoverIntervalMin: Parameter<T, F32>,
    mAutoRecoverIntervalMax: Parameter<T, F32>,
    mAutoRecoverInvalidTime: Parameter<T, F32>,
    mBowSubjectContTime: Parameter<T, F32>,
    mLNGStickScale: Parameter<T, F32>,
    mLATStickScale: Parameter<T, F32>,
    mLNGGyroScale: Parameter<T, F32>,
    mLATGyroScale: Parameter<T, F32>,
    mBowSlowShootNum: Parameter<T, S32>,
    mBowSlowRateDiam: Parameter<T, F32>,
    mBowSlowMaxTime: Parameter<T, F32>,
    mDiveBowSlowMaxTime: Parameter<T, F32>,
    mBowSlowInvalidTime: Parameter<T, F32>,
    mBowSlowInvalidHeight: Parameter<T, F32>,
    mBowSlowInvalidHeightOnShield: Parameter<T, F32>,
    mBowSlowInvalidHeightWeaponChange: Parameter<T, F32>,
    mGuardJustForceSlowTime: Parameter<T, F32>,
    mMoveMaxDecRateByWater: Parameter<T, F32>,
    mMoveIgnoreWaterHeight: Parameter<T, F32>,
    mMoveDecRateByBog: Parameter<T, F32>,
    mMoveDecRateMaxHeight: Parameter<T, F32>,
    mMaxForce: Parameter<T, F32>,
    mMinForce: Parameter<T, F32>,
    mAddForce: Parameter<T, F32>,
    mSnowBallAddForce: Parameter<T, F32>,
    mLogPushF: Parameter<T, F32>,
    mRockPushF: Parameter<T, F32>,
    mRockPushSpeed: Parameter<T, F32>,
    mWaistAngleUpperMax: Parameter<T, F32>,
    mWaistAngleLowerMax: Parameter<T, F32>,
    mWaistAngleSideMax: Parameter<T, F32>,
    mNoSquatWaterHeight: Parameter<T, F32>,
    mInvalidReloadTime: Parameter<T, F32>,
    mWeaponThrowSpeedY: Parameter<T, F32>,
    mWeaponThrowSpeedF: Parameter<T, F32>,
    mWeaponThrowSpeedFSquat: Parameter<T, F32>,
    mDashUpEnableAngle: Parameter<T, F32>,
    mShockTime: Parameter<T, F32>,
    mIceInvalidTime: Parameter<T, F32>,
    mMaxSpeedInAir: Parameter<T, F32>,
    mTurnEnableSpeed: Parameter<T, F32>,
    mTurnEnableStickSub: Parameter<T, F32>,
    mTurnEnableDirSub: Parameter<T, F32>,
    mShortDashImpulse: Parameter<T, S32>,
    mShortDashDamage: Parameter<T, S32>,
    mSwordTerrorScope: Parameter<T, F32>,
    mArrowTerrorScope: Parameter<T, F32>,
    mTorchTerrorScope: Parameter<T, F32>,
    mTorchTerrorOffsetY: Parameter<T, F32>,
    mTorchTerrorOffsetZ: Parameter<T, F32>,
    mDashNoise: Parameter<T, F32>,
    mWhistleNoise: Parameter<T, F32>,
    mClimbEnableAngle: Parameter<T, F32>,
    mClimbEnableSpeedMinAngle: Parameter<T, F32>,
    mClimbEnableSpeedMaxAngle: Parameter<T, F32>,
    mSlipEnableSpeed: Parameter<T, F32>,
    mSlipSpeedAddMin: Parameter<T, F32>,
    mSlipSpeedAddMax: Parameter<T, F32>,
    mSlipSpeedAddDiamByRain: Parameter<T, F32>,
    mMagnetAim2DPosOffsetY: Parameter<T, F32>,
    mLookPosOffsetXZ: Parameter<T, F32>,
    mLookPosOffsetY: Parameter<T, F32>,
    mLookPosOffsetYSquat: Parameter<T, F32>,
    mLookPosOffsetYSwim: Parameter<T, F32>,
    mLookPosOffsetYHorse: Parameter<T, F32>,
    mLookEnableAngle: Parameter<T, F32>,
    mHitSlowTimeS: Parameter<T, F32>,
    mHitSlowTimeM: Parameter<T, F32>,
    mHitSlowTimeL: Parameter<T, F32>,
    mHitSlowRate: Parameter<T, F32>,
    mHitStopTimeS: Parameter<T, F32>,
    mHitStopTimeL: Parameter<T, F32>,
    mHitStopRate: Parameter<T, F32>,
    mAtnPosInterPolationRate: Parameter<T, F32>,
    mAtnPosInterPolationMin: Parameter<T, F32>,
    mAtnPosInterPolationMax: Parameter<T, F32>,
    mPredictDiffAngleMax: Parameter<T, F32>,
    mDashToRunStickValueDec: Parameter<T, F32>,
    mWindSupportReuseTime: Parameter<T, F32>,
    mFireSupportReuseTime: Parameter<T, F32>,
    mElectricSupportReuseTime: Parameter<T, F32>,
    mWaterSupportReuseTime: Parameter<T, F32>,
    mWindSupportTimerRate: Parameter<T, F32>,
    mFireSupportTimerRate: Parameter<T, F32>,
    mElectricSupportTimerRate: Parameter<T, F32>,
    mWaterSupportTimerRate: Parameter<T, F32>,
    mChemicalInvalidTime: Parameter<T, F32>,
    mAutoDashUpTime: Parameter<T, F32>,
    mAutoDashUpAngle: Parameter<T, F32>,
    mClimbRestartHeight: Parameter<T, F32>,
    mClimbRestartTime: Parameter<T, F32>,
    mPushNoticeLookTime: Parameter<T, F32>,
    mEnergyUseSmall: Parameter<T, F32>,
    mEnergyUseLarge: Parameter<T, F32>,
    mNoEnergyDashInterval: Parameter<T, F32>,
    mGuardableAngle: Parameter<T, F32>,
    mStickMaxInStore: Parameter<T, F32>,
    mLookContinueTime: Parameter<T, F32>,
    mPostureContinueTime: Parameter<T, F32>,
    mItemUseModelAlpha: Parameter<T, F32>,
    mLadderCheckSide: Parameter<T, F32>,
    mLadderCheckDist: Parameter<T, F32>,
    mNoDeathDamageBase: Parameter<T, S32>,
    mNoDeathDamageAdd: Parameter<T, S32>,
    mArmorCompSwimEnergyRate: Parameter<T, F32>,
    mArmorCompRegistElecFrame: Parameter<T, F32>,
    mArmorCompNightSpeedRate: Parameter<T, F32>,
    mArmorCompClimbJumpEnergyRate: Parameter<T, F32>,
    mArmorCompPlusDropRate: Parameter<T, F32>,
    mArmorCompWeaponBrakeRate: Parameter<T, F32>,
    mArmorCompSwordBeamAttackRate: Parameter<T, F32>,
    mArmorCompAncientAttackRate: Parameter<T, F32>,
    mArmorCompBoneAttackRate: Parameter<T, F32>,
    mArmorCompTerrorLevel: Parameter<T, F32>,
    mArmorCompTerrorRadius: Parameter<T, F32>,
    mArmorCompNakedSwimSpeedRate: Parameter<T, F32>,
    mArmorCompNakedSwimAnimeRate: Parameter<T, F32>,
    mArmorCompNakedSwimEnergyRate: Parameter<T, F32>,
    mArmorAncientAttackRate: Parameter<T, F32>,
    mSupportWindNum: Parameter<T, S32>,
    mSupportElectricNum: Parameter<T, S32>,
    mSupportElectricEnergy: Parameter<T, F32>,
    mSupportFireNum: Parameter<T, S32>,
    mSupportWaterLifeAdd: Parameter<T, S32>,
    mSupportWaterEnergyAdd: Parameter<T, F32>,
    mStickRInputFrame: Parameter<T, F32>,
    mDiffAngleFromLookVec: Parameter<T, F32>,
    mLookPosOffset: Parameter<T, F32>,
    mLookFixAngle: Parameter<T, F32>,
    mLookContinueTimeToCamera: Parameter<T, F32>,
    mCutKnockBackNoCrrDist: Parameter<T, F32>,
    mWaitUnsteadyApplyVel: Parameter<T, F32>,
    mCurseAddWeight: Parameter<T, F32>,
    mRoofCrashVel: Parameter<T, F32>,
    mCutJumpInvalidTime: Parameter<T, F32>,
    mWaterDepthInGrudge: Parameter<T, F32>,
    mLargeHorseLegBendAngY: Parameter<T, F32>,
    mLargeHorseLegBendAngX: Parameter<T, F32>,
    mLargeHorseLegBendFrame: Parameter<T, F32>,
    mNoMaskPauseWaterHeight: Parameter<T, F32>,
    mLookAtThreshold: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectPrey<T> {
    base: GParamListObj<T>,
    mEatActorFindRadius: Parameter<T, F32>,
    mEatActorFindRotDegree: Parameter<T, F32>,
    mWaitTimeForStartEat: Parameter<T, F32>,
    mIsEnableGroupEscape: Parameter<T, Bool32>,
    mAimEscapeOffsetRate: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectRod<T> {
    base: GParamListObj<T>,
    mMagicName: Parameter<T, SafeString<T>>,
    mChargeMagicNum: Parameter<T, S32>,
    mChargeMagicInterval: Parameter<T, S32>,
    mMagicPower: Parameter<T, S32>,
    mMagicSpeed: Parameter<T, F32>,
    mMagicSpeedByThrow: Parameter<T, F32>,
    mMagicGravity: Parameter<T, F32>,
    mMagicRadius: Parameter<T, F32>,
    mScaleTime: Parameter<T, S32>,
    mMagicRange: Parameter<T, F32>,
    mMagicSpeedByEnemy: Parameter<T, F32>,
    mMagicGravityByEnemy: Parameter<T, F32>,
    mMagicRadiusByEnemy: Parameter<T, F32>,
    mScaleTimeByEnemy: Parameter<T, S32>,
    mMagicRangeByEnemy: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectRope<T> {
    base: GParamListObj<T>,
    mIsAllowCutting: Parameter<T, Bool32>,
    mIsSetupKeyframed: Parameter<T, Bool32>,
    mBoneEffectiveLength: Parameter<T, F32>,
    mIsInterpolateEdge: Parameter<T, Bool32>,
    mIsDeformable: Parameter<T, Bool32>,
    mIsOneBoneOneShape: Parameter<T, Bool32>,
    mSplineOffsetRateA: Parameter<T, F32>,
    mSplineOffsetRateB: Parameter<T, F32>,
    mSplineOffsetRateC: Parameter<T, F32>,
    mMtxEndPosOffsetLength: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectRupee<T> {
    base: GParamListObj<T>,
    mRupeeValue: Parameter<T, S32>,
}

#[repr(C)]
pub struct GParamListObjectSandworm<T> {
    base: GParamListObj<T>,
    mSandWidth: Parameter<T, F32>,
    mSandLength: Parameter<T, F32>,
    mSandHeight: Parameter<T, F32>,
    mSandCombSpan: Parameter<T, F32>,
    mSandCombHeight: Parameter<T, F32>,
    mSnakeModelOffsetZ: Parameter<T, F32>,
    mSnakeBaseNode: Parameter<T, SafeString<T>>,
    mSnakeNode1: Parameter<T, SafeString<T>>,
    mSnakeNode2: Parameter<T, SafeString<T>>,
    mSnakeNode3: Parameter<T, SafeString<T>>,
    mSnakeNode4: Parameter<T, SafeString<T>>,
    mSnakeNode5: Parameter<T, SafeString<T>>,
    mSnakeNode6: Parameter<T, SafeString<T>>,
    mSnakeNode7: Parameter<T, SafeString<T>>,
    mSnakeNode8: Parameter<T, SafeString<T>>,
    mSnakeNode9: Parameter<T, SafeString<T>>,
    mSnakeNode10: Parameter<T, SafeString<T>>,
    mSnakeNode11: Parameter<T, SafeString<T>>,
    mSnakeNode12: Parameter<T, SafeString<T>>,
    mSnakeNodeRotOffset: Parameter<T, Vector3f>,
    mSnakeNodeChaseInterval: Parameter<T, F32>,
    mShowLifeGageDist: Parameter<T, F32>,
    mShowLifeGageOffset: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectSeriesArmor<T> {
    base: GParamListObj<T>,
    mSeriesType: Parameter<T, SafeString<T>>,
    mEnableCompBonus: Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectShiekerStone<T> {
    base: GParamListObj<T>,
    mNodeNameWithWaist: Parameter<T, SafeString<T>>,
    mTransOffsetWithWaist: Parameter<T, Vector3f>,
    mRotOffsetWithWaist: Parameter<T, Vector3f>,
    mNodeNameWithHand: Parameter<T, SafeString<T>>,
    mTransOffsetWithHand: Parameter<T, Vector3f>,
    mRotOffsetWithHand: Parameter<T, Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectShield<T> {
    base: GParamListObj<T>,
    mPlayerHoldTransOffset: Parameter<T, Vector3f>,
    mPlayerHoldRotOffset: Parameter<T, Vector3f>,
    mPlayerEquipTransOffset: Parameter<T, Vector3f>,
    mPlayerEquipRotOffset: Parameter<T, Vector3f>,
    mSquatPlayerHoldTransAddOffset: Parameter<T, Vector3f>,
    mSquatPlayerHoldRotAddOffset: Parameter<T, Vector3f>,
    mNPCHoldTransOffset: Parameter<T, Vector3f>,
    mNPCHoldRotOffset: Parameter<T, Vector3f>,
    mNPCEquipTransOffset: Parameter<T, Vector3f>,
    mNPCEquipRotOffset: Parameter<T, Vector3f>,
    mEnemyEquipTransOffset: Parameter<T, Vector3f>,
    mEnemyEquipRotOffset: Parameter<T, Vector3f>,
    mStandEquipTransOffset: Parameter<T, Vector3f>,
    mStandEquipRotOffset: Parameter<T, Vector3f>,
    mRideBreakRatio: Parameter<T, F32>,
    mMirrorLevel: Parameter<T, S32>,
    mWeaponSubType: Parameter<T, SafeString<T>>,
    mSurfingFriction: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectSmallSword<T> {
    base: GParamListObj<T>,
    mPodName: Parameter<T, SafeString<T>>,
    mPlayerHoldTransOffset: Parameter<T, Vector3f>,
    mPlayerHoldRotOffset: Parameter<T, Vector3f>,
    mPlayerEquipTransOffset: Parameter<T, Vector3f>,
    mPlayerEquipRotOffset: Parameter<T, Vector3f>,
    mRideHorsePlayerHoldTransOffset: Parameter<T, Vector3f>,
    mRideHorsePlayerHoldRotOffset: Parameter<T, Vector3f>,
    mAffectTransOffsetShield: Parameter<T, Vector3f>,
    mAffectRotOffsetShield: Parameter<T, Vector3f>,
    mAffectTransOffsetBow: Parameter<T, Vector3f>,
    mAffectRotOffsetBow: Parameter<T, Vector3f>,
    mSquatPlayerHoldTransAddOffset: Parameter<T, Vector3f>,
    mSquatPlayerHoldRotAddOffset: Parameter<T, Vector3f>,
    mNPCHoldTransOffset: Parameter<T, Vector3f>,
    mNPCHoldRotOffset: Parameter<T, Vector3f>,
    mNPCEquipTransOffset: Parameter<T, Vector3f>,
    mNPCEquipRotOffset: Parameter<T, Vector3f>,
    mEnemyEquipTransOffset: Parameter<T, Vector3f>,
    mEnemyEquipRotOffset: Parameter<T, Vector3f>,
    mStandEquipTransOffset: Parameter<T, Vector3f>,
    mStandEquipRotOffset: Parameter<T, Vector3f>,
    mWeaponSubType: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectSpear<T> {
    base: GParamListObj<T>,
    mPodName: Parameter<T, SafeString<T>>,
    mPlayerHoldTransOffset: Parameter<T, Vector3f>,
    mPlayerHoldRotOffset: Parameter<T, Vector3f>,
    mPlayerEquipTransOffset: Parameter<T, Vector3f>,
    mPlayerEquipRotOffset: Parameter<T, Vector3f>,
    mRideHorsePlayerHoldTransOffset: Parameter<T, Vector3f>,
    mRideHorsePlayerHoldRotOffset: Parameter<T, Vector3f>,
    mAffectTransOffsetShield: Parameter<T, Vector3f>,
    mAffectRotOffsetShield: Parameter<T, Vector3f>,
    mAffectTransOffsetBow: Parameter<T, Vector3f>,
    mAffectRotOffsetBow: Parameter<T, Vector3f>,
    mGrabPlayerHoldTransOffset: Parameter<T, Vector3f>,
    mGrabPlayerHoldRotOffset: Parameter<T, Vector3f>,
    mGrabAffectTransOffsetShield: Parameter<T, Vector3f>,
    mGrabAffectRotOffsetShield: Parameter<T, Vector3f>,
    mSquatPlayerHoldTransAddOffset: Parameter<T, Vector3f>,
    mSquatPlayerHoldRotAddOffset: Parameter<T, Vector3f>,
    mNPCHoldTransOffset: Parameter<T, Vector3f>,
    mNPCHoldRotOffset: Parameter<T, Vector3f>,
    mNPCEquipTransOffset: Parameter<T, Vector3f>,
    mNPCEquipRotOffset: Parameter<T, Vector3f>,
    mEnemyEquipTransOffset: Parameter<T, Vector3f>,
    mEnemyEquipRotOffset: Parameter<T, Vector3f>,
    mStandEquipTransOffset: Parameter<T, Vector3f>,
    mStandEquipRotOffset: Parameter<T, Vector3f>,
    mWeaponSubType: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectStalEnemy<T> {
    base: GParamListObj<T>,
    mHeadActorName: Parameter<T, SafeString<T>>,
    mLeftArmActorName: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectSwarm<T> {
    base: GParamListObj<T>,
    mSwarmSubActorNum: Parameter<T, S32>,
    mSwarmPattern: Parameter<T, S32>,
    mDeadActorName: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectSystem<T> {
    base: GParamListObj<T>,
    mSameGroupActorName: Parameter<T, SafeString<T>>,
    mIsGetItemSelf: Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectTraveler<T> {
    base: GParamListObj<T>,
    mAppearGameDataName: Parameter<T, SafeString<T>>,
    mDeleteGameDataName: Parameter<T, SafeString<T>>,
    mRouteType: Parameter<T, SafeString<T>>,
    mRideHorseName: Parameter<T, SafeString<T>>,
    mIsLeadHorse: Parameter<T, Bool32>,
    mHorseGearLevel: Parameter<T, S32>,
    mRoutePoints: RoutePoints<T>,
    mRoutePoint29Name: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectWeaponCommon<T> {
    base: GParamListObj<T>,
    mPlayerEqScale: Parameter<T, F32>,
    mEnemyEqScale: Parameter<T, F32>,
    mGuardPower: Parameter<T, S32>,
    mRank: Parameter<T, S32>,
    mIsHammer: Parameter<T, Bool32>,
    mIsWeakBreaker: Parameter<T, Bool32>,
    mIsBoomerang: Parameter<T, Bool32>,
    mIsBlunt: Parameter<T, Bool32>,
    mIsLuckyWeapon: Parameter<T, Bool32>,
    mIsPikohan: Parameter<T, Bool32>,
    mIsThrowingWeapon: Parameter<T, Bool32>,
    mIsThrowingBreakWeapon: Parameter<T, Bool32>,
    mThrowRange: Parameter<T, F32>,
    mDreadActor: Parameter<T, SafeString<T>>,
    mThroughActor: Parameter<T, SafeString<T>>,
    mNPCWeaponType: Parameter<T, SafeString<T>>,
    mIsNotOnTerrorHold: Parameter<T, Bool32>,
    mIsAsOffUnEquiped: Parameter<T, Bool32>,
    mChemicalEnergyMax: Parameter<T, S32>,
    mChemicalEnergyAmountUsed: Parameter<T, S32>,
    mChemicalEnergyRecoverRate: Parameter<T, F32>,
    mChemicalEnergyRecoverInterval: Parameter<T, S32>,
    mStickDamage: Parameter<T, S32>,
    mShootBeam: Parameter<T, SafeString<T>>,
    mDropFromPorchRot: Parameter<T, Vector3f>,
    mSharpWeaponPer: Parameter<T, F32>,
    mSharpWeaponAddAtkMin: Parameter<T, S32>,
    mSharpWeaponAddAtkMax: Parameter<T, S32>,
    mSharpWeaponAddLifeMin: Parameter<T, S32>,
    mSharpWeaponAddLifeMax: Parameter<T, S32>,
    mSharpWeaponAddCrit: Parameter<T, Bool32>,
    mSharpWeaponAddGuardMin: Parameter<T, S32>,
    mSharpWeaponAddGuardMax: Parameter<T, S32>,
    mPoweredSharpAddAtkMin: Parameter<T, S32>,
    mPoweredSharpAddAtkMax: Parameter<T, S32>,
    mPoweredSharpAddLifeMin: Parameter<T, S32>,
    mPoweredSharpAddLifeMax: Parameter<T, S32>,
    mPoweredSharpWeaponAddGuardMin: Parameter<T, S32>,
    mPoweredSharpWeaponAddGuardMax: Parameter<T, S32>,
    mPoweredSharpAddThrowMin: Parameter<T, F32>,
    mPoweredSharpAddThrowMax: Parameter<T, F32>,
    mPoweredSharpAddSpreadFire: Parameter<T, Bool32>,
    mPoweredSharpAddZoomRapid: Parameter<T, Bool32>,
    mPoweredSharpAddRapidFireMin: Parameter<T, F32>,
    mPoweredSharpAddRapidFireMax: Parameter<T, F32>,
    mPoweredSharpAddSurfMaster: Parameter<T, Bool32>,
}

#[repr(C)]
pub struct GParamListObjectWeaponOption<T> {
    base: GParamListObj<T>,
    mPlayerHoldTransOffset: Parameter<T, Vector3f>,
    mPlayerHoldRotOffset: Parameter<T, Vector3f>,
    mNPCHoldTransOffset: Parameter<T, Vector3f>,
    mNPCHoldRotOffset: Parameter<T, Vector3f>,
    mNPCEquipTransOffset: Parameter<T, Vector3f>,
    mNPCEquipRotOffset: Parameter<T, Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectWeaponThrow<T> {
    base: GParamListObj<T>,
    mThrowSpeed: Parameter<T, F32>,
    mThrowRotSpeed: Parameter<T, F32>,
    mThrowDist: Parameter<T, F32>,
    mThrowRigidBodyBaseAxis: Parameter<T, Vector3f>,
}

#[repr(C)]
pub struct GParamListObjectWizzrobe<T> {
    base: GParamListObj<T>,
    mMagicWeatherType: Parameter<T, S32>,
    mMagicFallActorName: Parameter<T, SafeString<T>>,
    mMagicFallIgniteRotSpd: Parameter<T, F32>,
    mMagicFallOffsetY: Parameter<T, F32>,
    mMagicFallCenterOffsetXZ: Parameter<T, F32>,
    mMagicFallRandRadius: Parameter<T, F32>,
    mMagicFallIntervalMax: Parameter<T, F32>,
    mMagicFallIntervalMin: Parameter<T, F32>,
    mSummonActorName: Parameter<T, SafeString<T>>,
}

#[repr(C)]
pub struct GParamListObjectWolfLink<T> {
    base: GParamListObj<T>,
    mNeckSpeedWait: Parameter<T, F32>,
    mNeckRateWait: Parameter<T, F32>,
    mNeckSpeedShiekSensor: Parameter<T, F32>,
    mNeckRateShiekSensor: Parameter<T, F32>,
    mNeckSpeedFollow: Parameter<T, F32>,
    mNeckRateFollow: Parameter<T, F32>,
    mNeckSpeedBattle: Parameter<T, F32>,
    mNeckRateBattle: Parameter<T, F32>,
    mNeckSpeedHeal: Parameter<T, F32>,
    mNeckRateHeal: Parameter<T, F32>,
    mBattleRange: Parameter<T, F32>,
    mHealRange: Parameter<T, F32>,
    mHuntRange: Parameter<T, F32>,
    mHowlRange: Parameter<T, F32>,
    mMaxHeightAttackable: Parameter<T, F32>,
    mMaxHeightHealable: Parameter<T, F32>,
    mNavMeshSearchRadius: Parameter<T, F32>,
    mCanReachPlayerNavMeshSearchRadius: Parameter<T, F32>,
    mSubmergedDepth: Parameter<T, F32>,
    mUtilityLifeToHunt: Parameter<T, F32>,
    mUtilityDangerDistMin: Parameter<T, F32>,
    mUtilityDangerDistMax: Parameter<T, F32>,
    mUtilityConstant: Parameter<T, F32>,
    mChainAttackChargeMin: Parameter<T, F32>,
    mChainAttackChargeMax: Parameter<T, F32>,
    mLookAtCooldownWait: Parameter<T, F32>,
    mLookAtCooldownWaitRand: Parameter<T, F32>,
    mLookAtCounterWait: Parameter<T, F32>,
    mLookAtCounterWaitRand: Parameter<T, F32>,
    mLookAtCooldownRun: Parameter<T, F32>,
    mLookAtCooldownRunRand: Parameter<T, F32>,
    mLookAtCounterRun: Parameter<T, F32>,
    mLookAtCounterRunRand: Parameter<T, F32>,
    mAttackCounterLength: Parameter<T, F32>,
    mAttackCounterRand: Parameter<T, F32>,
    mHowlCooldownCounterLength: Parameter<T, F32>,
    mHowlCooldownCounterRand: Parameter<T, F32>,
    mHealCooldownCounterLength: Parameter<T, F32>,
    mHealCooldownCounterRand: Parameter<T, F32>,
    mFailPathCooldownCounterLength: Parameter<T, F32>,
    mFailPathCooldownCounterRand: Parameter<T, F32>,
    mRetargetCooldownCounterLength: Parameter<T, F32>,
    mRetargetCooldownCounterRand: Parameter<T, F32>,
    mAfterTargetDeathCounterLength: Parameter<T, F32>,
    mAfterTargetDeathCounterRand: Parameter<T, F32>,
    mLostTargetCounterLength: Parameter<T, F32>,
    mLostTargetCounterRand: Parameter<T, F32>,
    mInvinceableCounterLength: Parameter<T, F32>,
    mInvinceableCounterRand: Parameter<T, F32>,
    mCallDelayMinLength: Parameter<T, F32>,
    mCallOverrideCounterLength: Parameter<T, F32>,
    mGiveUpShiekSensorLength: Parameter<T, F32>,
    mRetryShiekSensorLength: Parameter<T, F32>,
    mBattleWallHitLength: Parameter<T, F32>,
    mFollowRetryLength: Parameter<T, F32>,
    mPowerUpFoodLength: Parameter<T, F32>,
    mSafePosFailCounter: Parameter<T, F32>,
    mRestrictedTargetTimeNormal: Parameter<T, F32>,
    mRestrictedTargetTimeSpecial: Parameter<T, F32>,
    mPowerUpFoodAttackMod: Parameter<T, S32>,
    mPowerUpFoodChainAttackCharge: Parameter<T, F32>,
    mVSStalfosCritChance: Parameter<T, S32>,
    mAttackBase: Parameter<T, F32>,
    mAttackHeartMod: Parameter<T, F32>,
    mDefenseBase: Parameter<T, F32>,
    mDefenseHeartMod: Parameter<T, F32>,
}

#[repr(C)]
pub struct GParamListObjectZora<T> {
    base: GParamListObj<T>,
    mInWaterDepth: Parameter<T, F32>,
    mFloatDepth: Parameter<T, F32>,
    mFloatRadius: Parameter<T, F32>,
    mFloatCycleTime: Parameter<T, F32>,
    mChangeDepthSpeed: Parameter<T, F32>,
}

#[repr(C)]
struct DirectionInfo<T> {
    mEntryPoint: Parameter<T, SafeString<T>>,
    mWaitFrame: Parameter<T, F32>,
    mSchedule: Parameter<T, SafeString<T>>,
    mMoveAS: Parameter<T, SafeString<T>>,
    mWaitAS: Parameter<T, SafeString<T>>,
}

#[repr(C)]
struct RoutePoint<T> {
    mName: Parameter<T, SafeString<T>>,
    mForward: DirectionInfo<T>,
    mBackward: DirectionInfo<T>,
}

#[repr(C)]
struct RoutePoints<T> {
    mStorage: [RoutePoint<T>; 29]
}
