use crate::imports::BuildImports::*;

unsafe extern "C" fn status_gamewatch_SpecialLwWaitStart_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_BUTTON_RELEASE);
    let lr = PostureModule::lr(fighter.module_accessor);
    WorkModule::set_float(fighter.module_accessor, lr, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_WORK_FLOAT_LR);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_WORK_INT_REFLECT_COUNT);
    fighter.sub_shift_status_main(L2CValue::Ptr(gamewatch_SpecialLwWaitStart_Main_Loop as *const () as _))
}

unsafe extern "C" fn gamewatch_special_lw_bucket_function(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_BUCKET) {
        gamewatch_special_lw_gauge_function(fighter);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_BUCKET);
    }
}

unsafe extern "C" fn gamewatch_special_lw_gauge_function(fighter: &mut L2CFighterCommon) {
    let gauge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_GAUGE);
    let gauge_level = match gauge {
        1.0 => hash40("oil_1"), 
        2.0 => hash40("oil_2"),
        3.0 => hash40("oil_3"),
        _ => hash40("oil_none"),
    };
    VisibilityModule::set_int64(fighter.module_accessor, Hash40::new("oil").hash as i64, Hash40::new_raw(gauge_level).hash as i64);
}

unsafe extern "C" fn gamewatch_SpecialLwWaitStart_Main_Loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if !StatusModule::is_changing(fighter.module_accessor) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_WAIT.into(), false.into());
            return 0.into()
        }
    }
    if situation_kind != *SITUATION_KIND_GROUND || situation_kind != *SITUATION_KIND_GROUND && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GAMEWATCH_SPECIAL_AIR_LW);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE);
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND || situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_start"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE);
        }
    }
    gamewatch_special_lw_bucket_function(fighter);
    return 0.into()
}

pub fn install() {
    Agent::new("gamewatch")
    .status(Main, *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_WAIT_START, status_gamewatch_SpecialLwWaitStart_Main)
    .install();
}