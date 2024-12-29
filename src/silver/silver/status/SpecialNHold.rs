use crate::imports::BuildImports::*;

unsafe extern "C" fn status_silver_SpecialNHold_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let SILVER = color >= 120 && color <= 127;
	if SILVER {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_MEWTWO_GENERATE_ARTICLE_SHADOWBALL, *WEAPON_MEWTWO_SHADOWBALL_STATUS_KIND_CHARGE, ArticleOperationTarget(0));
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_hold"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_hold"), 0.0, 1.0, false, 0.0, false, false);
        }
        WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME);
        fighter.sub_shift_status_main(L2CValue::Ptr(silver_SpecialNHold_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_HOLD)(fighter)
    }
}

unsafe extern "C" fn silver_SpecialNHold_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_SHOOT.into(), true.into());
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
        else if fighter.sub_transition_group_check_air_cliff().get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_hold"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_hold"), -1.0, 1.0, 0.0, false, false);
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("mewtwo")
    .status(Main, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_HOLD, status_silver_SpecialNHold_Main)
    .install();
}

