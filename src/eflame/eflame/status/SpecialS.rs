use crate::imports::BuildImports::*;

unsafe extern "C" fn status_eflame_SpecialS_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[CMD_CAT3].get_i32() != *FIGHTER_PAD_CMD_CAT3_FLAG_SPECIAL_S_SMASH_DASH {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_SPECIAL_S_FLICK);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_SPECIAL_S_FLICK);
    }
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S as u64 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK as u64 | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON as u64 | *FIGHTER_LOG_MASK_FLAG_SHOOT as u64), *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn status_eflame_SpecialS_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_s").into());
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_SPECIAL_S_FLICK) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_s_flick").into(), Hash40::new("special_air_s_flick").into(), false.into());
    }
    else {
        fighter.sub_change_motion_by_situation(Hash40::new("special_s").into(), Hash40::new("special_air_s").into(), false.into());
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(eflame_SpecialS_Main_loop as *const () as _))
}

unsafe extern "C" fn eflame_SpecialS_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_air_check_fall_common().get_bool() || fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::Hash40s("param_special_s"));
        if MotionModule::is_end(fighter.module_accessor) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_SPECIAL_S_FLICK) {
                fighter.sub_change_motion_by_situation(Hash40::new("special_s_flick").into(), Hash40::new("special_air_s_flick").into(), false.into());
            }
            else {
                fighter.sub_change_motion_by_situation(Hash40::new("special_s").into(), Hash40::new("special_air_s").into(), false.into());
            }
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
        else {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("eflame")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, status_eflame_SpecialS_Pre)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, status_eflame_SpecialS_Main)
    .install();
}