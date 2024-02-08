use crate::imports::BuildImports::*;

unsafe extern "C" fn status_metaknight_SpecialNSpin_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let button_unable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("button_unable_frame"));
    let start_rot_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("start_rot_speed"));
    WorkModule::set_int(fighter.module_accessor, button_unable_frame, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_BUTTON_UNABLE_COUNTER);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_spin"), 0.0, 1.0, false, 0.0, false, false);
    MotionModule::set_rate(fighter.module_accessor, start_rot_speed);
    if !StopModule::is_stop(fighter.module_accessor) {
        metaknight_special_n_spin_sound_handler(fighter, false.into());
    }
    metaknight_special_n_spin_handler(fighter);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(metaknight_special_n_spin_sound_handler as *const () as _));
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let add_speed_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("add_speed_stick"));
    let start_stick_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("start_stick_speed"));
    if stick_x.abs() >= add_speed_stick {
        let lr = PostureModule::lr(fighter.module_accessor);
        let speed_x = start_stick_speed * stick_x * lr;
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: speed_x, y: 0.0, z: 0.0});
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_SpecialNSpin_Main_Sub as *const () as _))
}

unsafe extern "C" fn metaknight_special_n_spin_handler(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_AIR_N);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_N);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_FLOAT_GROUND_EFFECT_COUNTER);
    }
    return;
}

unsafe extern "C" fn metaknight_special_n_spin_sound_handler(fighter: &mut L2CFighterCommon, param_3: L2CValue) -> L2CValue {
    if param_3.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_BUTTON_HOP_COUNT);
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE) <= 0 {
            WorkModule::set_int(fighter.module_accessor, 5, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE);
            let start_se_counter = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE_COUNTER);
            let sound = match start_se_counter {
                0 => { if motion_kind == hash40("special_n_spin_c3") { 0x1896dcd23e } else { hash40("se_metaknight_swish07") } },
                1 => { if motion_kind == hash40("special_n_spin_c3") { 0x187603a50d } else { hash40("se_metaknight_swish09") } },
                2 => { if motion_kind == hash40("special_n_spin_c3") { 0x188ed7a452 } else { hash40("se_metaknight_swish11") } },
                3 => { if motion_kind == hash40("special_n_spin_c3") { 0x188ed7a452 } else { hash40("se_metaknight_swish11") } },
                4 => hash40("se_metaknight_swish10"),
                5 => { if motion_kind == hash40("special_n_spin_c3") { 0x187603a50d } else { hash40("se_metaknight_swish09") } },
                6 => { if motion_kind == hash40("special_n_spin_c3") { 0x1896dcd23e } else { hash40("se_metaknight_swish07") } },
                7 => hash40("se_metaknight_swish06"),
                8 => hash40("se_metaknight_swish06"),
                9 => hash40("se_metaknight_swish05"),
                _ => hash40("se_metaknight_swish05"),
            };
            SoundModule::play_se(fighter.module_accessor, Hash40::new_raw(sound), true, false, false, false, enSEType(0));
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE_COUNTER);                           
        }
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            let ground_effect_counter = WorkModule::get_float(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_FLOAT_GROUND_EFFECT_COUNTER);
            let rate = MotionModule::rate(fighter.module_accessor);
            let counter_value = ground_effect_counter - rate;
            WorkModule::set_float(fighter.module_accessor, counter_value, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_FLOAT_GROUND_EFFECT_COUNTER);
            if counter_value <= 0.0 {
                let FIGHTER_PTR = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
                FighterSpecializer_Metaknight::set_special_n_ground_effect(FIGHTER_PTR);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn metaknight_SpecialNSpin_Main_Sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    let end_rot_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("end_rot_speed"));
    if MotionModule::rate(fighter.module_accessor) <= end_rot_speed {
        fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_END.into(), false.into())
    }
    if !StatusModule::is_changing(fighter.module_accessor) && StatusModule::is_situation_changed(fighter.module_accessor) {
        metaknight_special_n_spin_handler(fighter);
        return 0.into();
    }
    return 0.into()
}

unsafe extern "C" fn status_metaknight_SpecialNSpin_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    JostleModule::set_status(fighter.module_accessor, true);
    DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_END {
        effect!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0x1230d89b8b), false, false);
    }
    0.into()
}

pub fn install() {
    Agent::new("metaknight")
    .status(Main, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_SPIN, status_metaknight_SpecialNSpin_Main)
    .status(End, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_SPIN, status_metaknight_SpecialNSpin_End)
    .install();
}