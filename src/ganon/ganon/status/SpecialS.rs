use crate::imports::BuildImports::*;

unsafe extern "C" fn status_ganon_SpecialS_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let explosion_speed_coef = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("explosion_speed_coef"));
    WorkModule::set_float(fighter.module_accessor, explosion_speed_coef, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        if !StopModule::is_stop(fighter.module_accessor) {
            ganon_SpecialS_Sub_Status(fighter, false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ganon_SpecialS_Sub_Status as *const () as _));
    }
    else {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);  
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_SpecialS_Main_Sub as *const () as _))
}

unsafe extern "C" fn ganon_SpecialS_Sub_Status(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_GRAVITY_ONOFF) {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        else {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    0.into()
}

unsafe extern "C" fn ganon_SpecialS_Main_Sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let mut int_val: i32 = 0;
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            if GroundModule::is_ottotto(fighter.module_accessor, 1.5) {
                if GrabModule::is_grab(fighter.module_accessor, 0) {
                    MotionModule::set_frame(fighter.module_accessor, 30.0, true);
                    KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                }
                GrabModule::clear_all(fighter.module_accessor);
                AttackModule::clear_all(fighter.module_accessor);
            }
            let explosion_start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_START_SITUATION);
            if explosion_start_situation != *SITUATION_KIND_GROUND {
                if !MotionModule::is_end(fighter.module_accessor) {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_GRAVITY_ONOFF) {
                            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                        }
                    }
                }
            }
            if MotionModule::is_end(fighter.module_accessor) { 
                if situation_kind != *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                }
                else { 
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                }
            }
            else {
                int_val = 0;
            }
        }
        else {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                if fighter.sub_air_check_fall_common().get_bool() {
                    int_val = 1;
                }
            }
        }
    }
    int_val.into()
}

pub fn install() {
    Agent::new("ganon")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, status_ganon_SpecialS_Main)
    .install();
}