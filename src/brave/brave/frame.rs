use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_brave_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if status_kind != *FIGHTER_STATUS_KIND_CATCH_ATTACK {
        if MotionModule::frame(fighter.module_accessor) >= 0.0 && MotionModule::frame(fighter.module_accessor) < 1.0 {
            ATTACK(fighter, /*ID*/ 6, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.4, /*Angle*/ 0, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_rush"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_BODY);
        }
    }
    if status_kind == *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_STEEL {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_STEEL);
            fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_STEEL_END.into(), false.into());
        }
    }
}

unsafe extern "C" fn frame_brave_Exec(fighter : &mut L2CFighterCommon) {
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("sword1"), &Vector3f{x: 1.1, y: 1.05, z: 1.05});
}

pub fn install() {
    Agent::new("brave")
    .on_line(Main, frame_brave_Main)
    .on_line(Exec, frame_brave_Exec)
    .install();
}