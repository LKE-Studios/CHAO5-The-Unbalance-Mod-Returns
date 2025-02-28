use crate::imports::BuildImports::*;

unsafe extern "C" fn status_ninten_AttackHi4_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {	
        fighter.sub_status_pre_SpecialNCommon();
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
        0.into()
    }
    else {
        original_status(Pre, fighter, *FIGHTER_STATUS_KIND_ATTACK_HI4)(fighter)
    }
}

unsafe extern "C" fn status_ninten_AttackHi4_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {
        if motion_kind != hash40("attack_hi4") {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_hi4"), -1.0, 1.0, false, 0.0, false, false);
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        0.into()
        //fighter.sub_shift_status_main(L2CValue::Ptr(ninten_AttackHi4_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_ATTACK_HI4)(fighter)
    }
}

unsafe extern "C" fn ninten_AttackHi4_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackHi4_Main();
    0.into()
}

pub fn install() {
    Agent::new("ness")
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_HI4, status_ninten_AttackHi4_Pre)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI4, status_ninten_AttackHi4_Main)
    .install();
}