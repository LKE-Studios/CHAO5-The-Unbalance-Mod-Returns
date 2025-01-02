use crate::imports::BuildImports::*;

unsafe extern "C" fn status_metaknight_SpecialLw_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, false, -1);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_start") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_start") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_START_WORK_INT_FREE_MOVE_COUNT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_START_FLAG_ADVANCE_STATUS);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_FB);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW);
    if !StopModule::is_stop(fighter.module_accessor) {
        metaknight_SpecialLw_Sub_Status(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(metaknight_SpecialLw_Sub_Status as *const () as _));
    metaknight_special_lw_motion_handler(fighter);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01 - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01 - 1);
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_SpecialLw_Main_Sub as *const () as _))
}

unsafe extern "C" fn metaknight_SpecialLw_Sub_Status(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        let kinetic = KineticModule::get_kinetic_type(fighter.module_accessor);
        if kinetic == *FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_LW_FREE_MOVE {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_START_WORK_INT_FREE_MOVE_COUNT);
        }
    }
    0.into()
}

unsafe extern "C" fn metaknight_special_lw_motion_handler(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("special_air_lw_start"), true, -1.0);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
    }
    else {
        let move_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_START_WORK_INT_FREE_MOVE_COUNT);
        let cliff_stop_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("cliff_stop_frame"));
        if cliff_stop_frame < move_count {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
        let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_START_FLAG_VIS_OFF) {
                ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, true, ArticleOperationTarget(0));
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("special_lw_start"), true, -1.0);
            }
            VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("mantle") as i64, hash40("mantle_none") as i64);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
    }
}

unsafe extern "C" fn metaknight_SpecialLw_Main_Sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        metaknight_special_lw_motion_handler(fighter);
    }
    let move_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_START_WORK_INT_FREE_MOVE_COUNT);
    let cliff_stop_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("cliff_stop_frame"));
    if move_count == cliff_stop_frame
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    if frame > 10.0 {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) || 
        ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END.into(), true.into());
        }
        else if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0 {
            fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK.into(), true.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_START_FLAG_ADVANCE_STATUS) {
        let status = if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK
        }
        else {
            *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("metaknight")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_metaknight_SpecialLw_Main)
    .install();
}