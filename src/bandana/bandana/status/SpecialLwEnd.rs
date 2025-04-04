use crate::imports::BuildImports::*;

unsafe extern "C" fn status_bandana_SpecialLwEnd_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
        0.into()
    }
    else {
        original_status(Pre, fighter, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_END)(fighter)
    }
}

unsafe extern "C" fn status_bandana_SpecialLwEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_end"), 0.0, 1.0, false, 0.0, false, false);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.set_situation(SITUATION_KIND_AIR.into());
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.set_situation(SITUATION_KIND_GROUND.into());
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(bandana_SpecialLwEnd_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_END)(fighter)
    }
}

unsafe extern "C" fn bandana_SpecialLwEnd_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND 
    || GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        SoundModule::play_landing_se(fighter.module_accessor, Hash40::new("se_edge_landing01"));
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn status_bandana_SpecialLwEnd_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("sys_warpstar_trace"), 0);
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_END)(fighter)
    }
}

pub fn install() {
    Agent::new("edge")
    .status(Pre, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_END, status_bandana_SpecialLwEnd_Pre)
    .status(Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_END, status_bandana_SpecialLwEnd_Main)
    .status(End, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_END, status_bandana_SpecialLwEnd_End)
    .install();
}