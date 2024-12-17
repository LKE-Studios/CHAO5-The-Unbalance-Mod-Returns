use crate::imports::BuildImports::*;

unsafe extern "C" fn status_maskedman_SpecialNEnd_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {	
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_maskedman_SpecialNEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_end"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_end"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(maskedman_SpecialNEnd_Main_loop as *const () as _))
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn maskedman_SpecialNEnd_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_end"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_end"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if stick_y <= -0.5 {
            GroundModule::pass_floor(fighter.module_accessor);
            if ray_check_pos(module_accessor, 0.0, -0.6, false) == 1 {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), true.into());
                fighter.set_situation(SITUATION_KIND_GROUND.into());
                KineticModule::suspend_energy_all(fighter.module_accessor);
            }
        }
        else {
            GroundModule::clear_pass_floor(fighter.module_accessor);
            if ray_check_pos(module_accessor, 0.0, -0.6, true) == 1 {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), true.into());
                fighter.set_situation(SITUATION_KIND_GROUND.into());
                KineticModule::suspend_energy_all(fighter.module_accessor);
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

unsafe extern "C" fn status_maskedman_SpecialNEnd_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("lucas")
    .status(Pre, FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_N_END, status_maskedman_SpecialNEnd_Pre)
    .status(Main, FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_N_END, status_maskedman_SpecialNEnd_Main)
    .status(End, FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_N_END, status_maskedman_SpecialNEnd_End)
    .install();
}