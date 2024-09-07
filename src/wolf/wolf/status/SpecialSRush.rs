use crate::imports::BuildImports::*; 

unsafe extern "C" fn status_wolf_SpecialSRush_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn status_wolf_SpecialSRush_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    0.into()
}

unsafe extern "C" fn status_wolf_SpecialSRush_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
        GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_FOX_CLIFF_HANG_DATA_SPECIAL_S as u32);
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS.into());
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
        let speed_mul = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_mul);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(wolf_SpecialSRush_Main_Loop as *const () as _))
}

unsafe extern "C" fn wolf_SpecialSRush_Main_Loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) && MotionModule::is_end(fighter.module_accessor) || StatusModule::is_situation_changed(fighter.module_accessor) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_END.into(), false.into());
            return 0.into();
        }
        else if StatusModule::is_situation_changed(fighter.module_accessor) {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), true.into());
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                let speed_mul = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
                sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_mul);
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_FOX_CLIFF_HANG_DATA_SPECIAL_S as u32);
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS.into());
            }
        }
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        }
    }
    let max_deg = WorkModule::get_float(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MAX_RUSH_DEGREE);
    PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: -max_deg, y: 0.0, z: 0.0}, 0);
    0.into()
}

unsafe extern "C" fn status_wolf_SpecialS_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        return 0.into();
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME) == 0 
    && WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_stop_y_frame")) != 0 {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let illusion_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_accel_y"));
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -illusion_accel_y);
    }
    0.into()
}

unsafe extern "C" fn status_wolf_SpecialSRush_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0);
    0.into()
}

pub fn install() {
    Agent::new("wolf")
    .status(Pre, FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_RUSH, status_wolf_SpecialSRush_Pre)
    .status(Init, FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_RUSH, status_wolf_SpecialSRush_Init)
    .status(Main, FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_RUSH, status_wolf_SpecialSRush_Main)
    .status(End, FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_RUSH, status_wolf_SpecialSRush_End)
    .install();
}