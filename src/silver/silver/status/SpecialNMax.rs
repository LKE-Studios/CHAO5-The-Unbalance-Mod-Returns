use crate::imports::BuildImports::*;

unsafe extern "C" fn status_silver_SpecialNMax_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let SILVER = color >= 120 && color <= 127;
	if SILVER {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_max"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_max"), 0.0, 1.0, false, 0.0, false, false);
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_FLAG_CHARGE_MAX);
        FighterUtil::flash_eye_info(fighter.module_accessor);
        fighter.sub_shift_status_main(L2CValue::Ptr(silver_SpecialNMax_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_MAX)(fighter)
    }
}

unsafe extern "C" fn silver_SpecialNMax_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_max"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_max"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("mewtwo")
    .status(Main, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_MAX, status_silver_SpecialNMax_Main)
    .install();
}