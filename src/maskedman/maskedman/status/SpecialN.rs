use crate::imports::BuildImports::*;

pub static charge_frame_max : f32 = 60.0;

unsafe extern "C" fn status_maskedman_SpecialN_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        0.into()
    }
    else {
        original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
    }
}

unsafe extern "C" fn status_maskedman_SpecialN_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {
        let stop_y_time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("stop_y_time"));
        let float_charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_MASKEDMAN_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE);
        WorkModule::set_int(fighter.module_accessor, stop_y_time, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_STOP_Y_TIME);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MASKEDMAN_STATUS_SPECIAL_N_FLAG_CHARGE);
        WorkModule::set_float(fighter.module_accessor, float_charge, *FIGHTER_MASKEDMAN_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        if !StopModule::is_stop(fighter.module_accessor) {
            maskedman_SpecialN_Sub_Status(fighter, false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(maskedman_SpecialN_Sub_Status as *const () as _));
        fighter.sub_shift_status_main(L2CValue::Ptr(maskedman_SpecialN_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
    }
}

unsafe extern "C" fn maskedman_SpecialN_Sub_Status(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_STOP_Y_TIME);
            let stop_y_time_int = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_STOP_Y_TIME);
            if stop_y_time_int <= 0 {
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            else {
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn maskedman_SpecialN_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let float_charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_MASKEDMAN_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE);
    let effect_counter = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_start"), -1.0, 1.0, 0.0, false, false);
            SoundModule::play_landing_se(fighter.module_accessor, Hash40::new("se_lucas_landing01"));
        }
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_start"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if frame > 8.0 && frame < 10.0 {
        FLASH(fighter, 2.5, 2.5, 0.0, 0.25);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_assist_steam_max"), Hash40::new("top"), &Vector3f { x: 0.0, y: 3.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 2.0, true, 0, 0, 0, 0, 0, true, true);
    }
    if frame >= 13.0 {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
        && float_charge < charge_frame_max {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MASKEDMAN_STATUS_SPECIAL_N_FLAG_CHARGE) {
                MotionModule::set_rate(fighter.module_accessor, 0.0);
                WorkModule::add_float(fighter.module_accessor, 1.0, *FIGHTER_MASKEDMAN_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE);
                WorkModule::add_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
                if effect_counter >= 6 {
                    EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("top"), &Vector3f { x: 0.0, y: 3.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 1.5, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), &Vector3f { x: 0.0, y: 8.0, z: 4.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 1.0, true, 0, 0, 0, 0, 0, true, true);
                    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
                }
            }
        }
        else {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
    }
    if float_charge == charge_frame_max {
        gimmick_flash(fighter);
        SoundModule::play_se(fighter.module_accessor, Hash40::new("se_lucas_special_n06"), true, false, false, false, enSEType(0));
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_N_DASH.into(), false.into());
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn status_maskedman_SpecialN_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {	
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_lucas_special_n02"), 0);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_assist_steam_max"), false, false);
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
    }
}

pub fn install() {
    Agent::new("lucas")
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, status_maskedman_SpecialN_Init)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, status_maskedman_SpecialN_Main)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, status_maskedman_SpecialN_End)
    .install();
}