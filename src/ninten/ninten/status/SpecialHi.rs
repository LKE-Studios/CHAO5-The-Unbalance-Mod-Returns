use crate::imports::BuildImports::*;

unsafe extern "C" fn status_ninten_SpecialHi_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {	
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_ninten_SpecialHi_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::pass_floor(fighter.module_accessor);
        StatusModule::set_keep_situation_air(fighter.module_accessor, true);
        JostleModule::set_status(fighter.module_accessor, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(ninten_SpecialHi_Main_loop as *const () as _))
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn ninten_SpecialHi_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);    
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_transition_group_check_air_cliff().get_bool() {
            return 1.into();
        }
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() 
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_NINTEN_STATUS_KIND_SPECIAL_HI_HOLD.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn status_ninten_SpecialHi_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {
        JostleModule::set_status(fighter.module_accessor, true);
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_common_spirits_floor_ice_loop"), 0);
        0.into()
    }
    else {
        0.into()
    }
}

pub fn install() {
    Agent::new("ness")
    .status(Pre, *FIGHTER_NINTEN_STATUS_KIND_SPECIAL_HI_START, status_ninten_SpecialHi_Pre)
    .status(Main, *FIGHTER_NINTEN_STATUS_KIND_SPECIAL_HI_START, status_ninten_SpecialHi_Main)
    .status(End, *FIGHTER_NINTEN_STATUS_KIND_SPECIAL_HI_START, status_ninten_SpecialHi_End)
    .install();
}