use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_funky_SpecialLwFlip_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
        0.into()
    }
    else {
        0.into()
    }
}

pub unsafe extern "C" fn status_funky_SpecialLwFlip_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_flip"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(funky_SpecialLwFlip_Main_loop as *const () as _))
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn funky_SpecialLwFlip_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool()
        && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
    }
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

pub unsafe extern "C" fn status_funky_SpecialLwFlip_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        0.into()
    }
    else {
        0.into()
    }
}

pub fn install() {
    Agent::new("donkey")
    .status(Pre, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_LW_FLIP, status_funky_SpecialLwFlip_Pre)
    .status(Main, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_LW_FLIP, status_funky_SpecialLwFlip_Main)
    .status(End, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_LW_FLIP, status_funky_SpecialLwFlip_End)    
    .install();
}