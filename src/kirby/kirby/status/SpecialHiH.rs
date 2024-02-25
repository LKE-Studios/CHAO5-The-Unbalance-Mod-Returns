use crate::imports::BuildImports::*;

unsafe extern "C" fn status_kirby_SpecialHiH_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn status_kirby_SpecialHiH_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //let speed_x_mul_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_h"), hash40("speed_x_mul_air"));
    //let speed_x_mul_ground = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_h"), hash40("speed_x_mul_ground"));
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_h"), 0.0, 1.0, false, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 2.2);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_h"), 0.0, 1.0, false, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 2.2);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_SpecialHiH_Main_loop as *const () as _))
}

unsafe extern "C" fn kirby_SpecialHiH_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND && 
        fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn status_kirby_SpecialHiH_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

pub fn install() {
    Agent::new("kirby")
    .status(Pre, FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI_H, status_kirby_SpecialHiH_Pre)
    .status(Main, FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI_H, status_kirby_SpecialHiH_Main)
    .status(End, FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI_H, status_kirby_SpecialHiH_End)
    .install();
}