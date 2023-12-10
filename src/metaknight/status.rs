use crate::imports::BuildImports::*;

#[status_script( agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_metaknight_glide_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_GLIDE);
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    fighter.status_GlideStart()
}

#[status_script( agent = "metaknight", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_metaknight_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_START_FRAME);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_start") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_start") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_start") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_start") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.set_situation(SITUATION_KIND_AIR.into());
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_BUTTON_ATTACK_COUNTER);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_N);
    ground_kinetic_function(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_special_n_loop as *const () as _))
}

pub unsafe fn ground_kinetic_function(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_AIR_N);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_N);
    }
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
}

unsafe extern "C" fn metaknight_special_n_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_SPIN.into(), false.into());
    }
    if StatusModule::is_changing(fighter.module_accessor) {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                ground_kinetic_function(fighter);
            }
            return true.into();
        }
        else {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
                ground_kinetic_function(fighter);
            }
            return true.into();
        }
    }
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let effect_start_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_START_FRAME);
    if current_frame == effect_start_frame as f32 {
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new_raw(0x133e36eb03), -1);
    }
    0.into()
}

#[status_script( agent = "metaknight", status = FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_SPIN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_metaknight_special_n_spin_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_special_n_spin_loop as *const () as _))
}

unsafe fn metaknight_special_n_spin_handler(fighter: &mut L2CFighterCommon) {
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

unsafe fn metaknight_special_n_spin_sound_handler(fighter: &mut L2CFighterCommon, param_3: L2CValue) -> L2CValue {
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

unsafe extern "C" fn metaknight_special_n_spin_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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

#[status_script( agent = "metaknight", status = FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_metaknight_special_n_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_end") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_end") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_TRANSITION_TERM_ID_MOT_END);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_FRAME);
    let remove_effect_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("remove_effect_frame"));
    WorkModule::set_int(fighter.module_accessor, remove_effect_frame, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_REMOVE_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_FLAG_EFFECT_REMOVE);
    metaknight_special_n_end_motion_handler(fighter);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_special_n_end_loop as *const () as _))
}

unsafe fn metaknight_special_n_end_motion_handler(fighter: &mut L2CFighterCommon) {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let sum_speed = KineticModule::get_sum_speed(fighter.module_accessor, -1);
    fighter.Vector2__create(sum_speed.into(), sum_speed.into());
    if situation_kind != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_end"), 0.0, 1.0, false, 0.0, false, false);
        let air_end_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("air_end_speed_x_mul"));
        let air_end_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("air_end_brake_x"));
        let air_end_max_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("air_end_max_speed_x"));
        let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, get_sum_speed_x * air_end_speed_x_mul, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_end_max_speed_x, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_end_brake_x, 0.0);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_end"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        let ground_end_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("ground_end_speed_x_mul"));
        let ground_end_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("ground_end_brake_x"));
        let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x * ground_end_speed_x_mul, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, -1.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ground_end_brake_x, 0.0);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_FLAG_MOTION_TRANSITION_TERM_ID_MOT_END);
    WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MTRANS);
}

unsafe extern "C" fn metaknight_special_n_end_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_end"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_end"), -1.0, 1.0, 0.0, false, false);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_FLAG_EFFECT_REMOVE) {
        let remove_effect_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("remove_effect_frame"));
        let end_frame = MotionModule::end_frame(fighter.module_accessor) as i32;
        if end_frame <= remove_effect_frame {
            WorkModule::set_int(fighter.module_accessor, end_frame, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_REMOVE_FRAME);
        }
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_FLAG_EFFECT_REMOVE);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    let spin_effect_handle = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_HANDLE);
    let spin_effect_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_FRAME);
    let spin_effect_remove_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_REMOVE_FRAME);
    if spin_effect_handle != 0 {
        if spin_effect_remove_frame <= spin_effect_frame {
            EffectModule::detach(fighter.module_accessor, 5.0 as u32, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_HANDLE);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_HANDLE);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            return 0.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            return 0.into();
        }
    }
    let mtrans = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MTRANS);
    let mut bool_set: bool = false;
    if !StatusModule::is_changing(fighter.module_accessor) {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != mtrans {
            if fighter.global_table[SITUATION_KIND].get_i32() == mtrans {
                bool_set = true;
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_TRANSITION_TERM_ID_MOT_END) {
            if MotionModule::is_end(fighter.module_accessor) {
                bool_set = true;
            }
        }
    }
    if bool_set {
        metaknight_special_n_end_motion_handler(fighter);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
    }
    0.into()
}

#[status_script( agent = "metaknight", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_metaknight_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_WORK_FLAG_HIT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_WORK_FLAG_GUARD_OR_INVINCIBLE);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_S);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_s_start") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s_start") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);  
    metaknight_special_s_motion_handler(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_special_s_main_loop as *const () as _))
}

unsafe fn metaknight_special_s_motion_handler(fighter: &mut L2CFighterCommon) {
    let mot_air_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
    let mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_AIR_S);
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_air_kind), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_air_kind), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_S);
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_kind), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_kind), 0.0, 1.0, false, 0.0, false, false);
    }
}

unsafe extern "C" fn metaknight_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut bool_set: bool = false;
    let mot_air_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
    let mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_air_kind), -1.0, 1.0, 0.0, false, false);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_kind), -1.0, 1.0, 0.0, false, false);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH.into(), false.into());
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                bool_set = true;
            }
        }
    }
    if bool_set {
        metaknight_special_s_motion_handler(fighter);
    }
    0.into()
}

#[status_script( agent = "metaknight", status = FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_metaknight_special_s_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let end_rot_comp_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("end_rot_comp_frame"));
    WorkModule::set_int(fighter.module_accessor, end_rot_comp_frame, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_END_WORK_INT_ROT_COMP_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_END_WORK_FLAG_SITUATION_AIR);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_s_end") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_WORK_FLAG_HIT) {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s_end") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s_finish") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s_finish") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        let mot_air_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_air_kind), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_WORK_FLAG_HIT) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        }
        let mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_kind), 0.0, 1.0, false, 0.0, false, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_WORK_FLAG_HIT) {
        KineticModule::clear_speed_all(fighter.module_accessor);
        let end_air_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("end_air_speed_x"));
        let end_air_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("end_air_speed_y"));
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: end_air_speed_x, y: end_air_speed_y, z: 0.0});
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_special_s_end_main_loop as *const () as _))
}

unsafe extern "C" fn metaknight_special_s_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_S);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_LW);
    }
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        DamageModule::heal(fighter.module_accessor, -5.0, 0);
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_END_WORK_FLAG_SITUATION_AIR);
        if MotionModule::is_end(fighter.module_accessor) {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return false.into();
            }
        }
    }
    else {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
            if motion_kind != hash40("special_air_s_end") {
                if MotionModule::is_end(fighter.module_accessor) {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                    return false.into();
                }
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_END_WORK_FLAG_SITUATION_AIR) {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                    return false.into();
                }
            }
            else {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_WORK_FLAG_HIT) {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                    return false.into();
                }
            }
        }
    }
    0.into()
}

#[status_script( agent = "metaknight", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_metaknight_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    let lr_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("lr_stick_x"));
    let dir_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("dir_mul"));
    let dir_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("dir_stick_x"));
    let pass_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("pass_mul"));
    let air_pass_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_pass_mul"));
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_start_x_mul"));
    let air_start_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_accel_y"));
    let fall_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_x_mul"));
    let trans_move_end_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("trans_move_end_speed_x_mul"));
    let trans_move_end_speed_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("trans_move_end_speed_y_mul"));
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi") as i64,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND);
    WorkModule::set_float(fighter.module_accessor, lr_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_LR_STICK_X);
    WorkModule::set_float(fighter.module_accessor, dir_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_STICK_X);
    WorkModule::set_float(fighter.module_accessor, dir_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_MUL);
    WorkModule::set_float(fighter.module_accessor, pass_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_PASS_MUL);
    WorkModule::set_float(fighter.module_accessor, air_pass_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_PASS_MUL);
    WorkModule::set_float(fighter.module_accessor, air_accel_y, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_ACCEL_Y);
    WorkModule::set_float(fighter.module_accessor, air_start_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_START_X_MUL);
    WorkModule::set_float(fighter.module_accessor, fall_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_FALL_X_MUL);
    WorkModule::set_int(fighter.module_accessor, landing_frame, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_CONST_LANDING_FRAME);
    WorkModule::set_float(fighter.module_accessor, trans_move_end_speed_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_MOVE_TRANS_END_SPEED_X_MUL);
    WorkModule::set_float(fighter.module_accessor, trans_move_end_speed_y_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_MOVE_TRANS_END_SPEED_Y_MUL);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_GLIDE);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_hi") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_hi_start") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_HI_FLAG_CONTINUE_MOT);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let metaknight_aerial_motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_HI_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(metaknight_aerial_motion_kind), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(metaknight_aerial_motion_kind), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_HI_FLAG_CONTINUE_MOT);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_special_hi_main_loop_2 as *const () as _))
    }
    else {
        fighter.super_jump_punch(L2CValue::Void());
        fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_special_hi_main_loop as *const () as _))
    }
}

unsafe extern "C" fn metaknight_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if frame > 23.0 {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_LIGHT.into(), true.into());
        }
    }
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if MotionModule::is_end(fighter.module_accessor) {
            let super_jump_punch_end_status = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
            fighter.change_status(super_jump_punch_end_status.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn metaknight_special_hi_main_loop_2(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    fighter.super_jump_punch_main();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if frame > 23.0 {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_LIGHT.into(), true.into());
        }
    }
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP.into(), true.into());
        }
    }
    0.into()
}

#[status_script( agent = "metaknight", status = FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_metaknight_special_hi_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    let lr_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("lr_stick_x"));
    let dir_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("dir_mul"));
    let dir_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("dir_stick_x"));
    let pass_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("pass_mul"));
    let air_pass_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_pass_mul"));
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_start_x_mul"));
    let air_start_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_accel_y"));
    let fall_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_x_mul"));
    let trans_move_end_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("trans_move_end_speed_x_mul"));
    let trans_move_end_speed_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("trans_move_end_speed_y_mul"));
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi_loop") as i64,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND_AIR);
    WorkModule::set_float(fighter.module_accessor, lr_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_LR_STICK_X);
    WorkModule::set_float(fighter.module_accessor, dir_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_STICK_X);
    WorkModule::set_float(fighter.module_accessor, dir_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_MUL);
    WorkModule::set_float(fighter.module_accessor, pass_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_PASS_MUL);
    WorkModule::set_float(fighter.module_accessor, air_accel_y, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_ACCEL_Y);
    WorkModule::set_float(fighter.module_accessor, air_start_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_START_X_MUL);
    WorkModule::set_float(fighter.module_accessor, air_pass_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_PASS_MUL);
    WorkModule::set_float(fighter.module_accessor, fall_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_FALL_X_MUL);
    WorkModule::set_int(fighter.module_accessor, landing_frame, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_CONST_LANDING_FRAME);
    WorkModule::set_float(fighter.module_accessor, trans_move_end_speed_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_MOVE_TRANS_END_SPEED_X_MUL);
    WorkModule::set_float(fighter.module_accessor, trans_move_end_speed_y_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_MOVE_TRANS_END_SPEED_Y_MUL);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_GLIDE);
    fighter.super_jump_punch(L2CValue::Void());
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_special_hi_main_loop as *const () as _))
}

#[status_script( agent = "metaknight", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_metaknight_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, false, -1);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_start") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_start") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_START_WORK_INT_FREE_MOVE_COUNT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_START_FLAG_ADVANCE_STATUS);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_FB);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_LW);
    if !StopModule::is_stop(fighter.module_accessor) {
        metaknight_special_lw_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(metaknight_special_lw_substatus as *const () as _));
    metaknight_special_lw_motion_handler(fighter);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01 - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01 - 1);
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_special_lw_main_loop as *const () as _))
}

unsafe fn metaknight_special_lw_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        let kinetic = KineticModule::get_kinetic_type(fighter.module_accessor);
        if kinetic == *FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_LW_FREE_MOVE {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_START_WORK_INT_FREE_MOVE_COUNT);
        }
    }
    0.into()
}

unsafe fn metaknight_special_lw_motion_handler(fighter: &mut L2CFighterCommon) {
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

unsafe fn metaknight_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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

#[status_script( agent = "metaknight", status = FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_metaknight_special_lw_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_end") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_end") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    metaknight_special_lw_end_motion_handler(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_special_lw_end_main_loop as *const () as _))
}

unsafe fn metaknight_special_lw_end_motion_handler(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("special_air_lw_end"), true, -1.0);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("special_lw_end"), true, -1.0);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
}

unsafe fn metaknight_special_lw_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut bool_set: bool = false;
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                bool_set = true;
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    if bool_set {
        metaknight_special_lw_end_motion_handler(fighter);
    }
    0.into()
}

#[status_script( agent = "metaknight", status = FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_metaknight_special_lw_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    let mot_air_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
    let mantle_mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_KIND);
    let mantle_mot_air_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_AIR_KIND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_STOP_X);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_FLOAT_MOVE_DISTANCE);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_FLOAT_RIGHT_EDGE_DISTANCE_X);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_FLOAT_LEFT_EDGE_DISTANCE_X);
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_i32();
    FighterSpecializer_Metaknight::check_edge_special_lw(fighter.module_accessor);
    if WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_air_neutral_x")) <= stick_x.abs() {
        if stick_x * lr > 0.0 {
            WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_b") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_b") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_b") as i64, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_KIND);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_b") as i64, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_AIR_KIND);
            if situation_kind == *SITUATION_KIND_GROUND {
                if GroundModule::is_ottotto_lr(fighter.module_accessor, lr, 1.5) {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_STOP_X);
                }
            }
        }
        else {
            WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_f") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_f") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_f") as i64, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_KIND);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_f") as i64, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_AIR_KIND);
            if situation_kind == *SITUATION_KIND_GROUND {
                if GroundModule::is_ottotto_lr(fighter.module_accessor, lr * -1.0, 1.5) {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_STOP_X);
                }
            }
        }
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_lw") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_lw") as i64, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw") as i64, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_AIR_KIND);
        if situation_kind == *SITUATION_KIND_GROUND {
            if GroundModule::is_ottotto(fighter.module_accessor, 1.5) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_STOP_X);
            }
        }
        fighter.set_situation(SITUATION_KIND_AIR.into());
    }
    let special_lw_situation_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_SITUATION_KIND);
    let special_lw_prev_situation_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_SITUATION_KIND_PREV);
    WorkModule::set_int(fighter.module_accessor, situation_kind, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_SITUATION_KIND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    if special_lw_prev_situation_kind != *SITUATION_KIND_GROUND {
        if special_lw_prev_situation_kind != *SITUATION_KIND_AIR {
            if special_lw_situation_kind == *SITUATION_KIND_AIR {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_air_kind), 0.0, 1.0, false, 0.0, false, false);   
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
                }
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new_raw(mantle_mot_air_kind), true, -1.0);          
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_air_kind), -1.0, 1.0, 0.0, false, false);
            }
        }
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_air_kind), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new_raw(mantle_mot_kind), true, -1.0);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_kind), -1.0, 1.0, 0.0, false, false);
    }
    else {
        if special_lw_situation_kind != *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_kind), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new_raw(mantle_mot_kind), true, -1.0);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_kind), -1.0, 1.0, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_special_lw_attack_main_loop as *const () as _))
}

unsafe extern "C" fn metaknight_special_lw_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_S);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_LW);
        if frame > 20.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 3.0);
        }
    }
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
        else {
            let mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
            let mantle_mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_KIND);
            let mot_air_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
            let mantle_mot_air_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_AIR_KIND);
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
                    if MotionModule::motion_kind(fighter.module_accessor) == mot_kind {
                        return 0.into();
                    }
                }
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_air_kind), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
                }
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new_raw(mantle_mot_air_kind), true, -1.0);          
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_air_kind), -1.0, 1.0, 0.0, false, false);
            }
            else {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_kind), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
                }
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new_raw(mantle_mot_kind), true, -1.0);          
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_kind), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        status_metaknight_glide_start_main,
        status_metaknight_special_n_main,
        status_metaknight_special_n_spin_main,
        status_metaknight_special_n_end_main,
        status_metaknight_special_s_main,
        status_metaknight_special_s_end_main,
        status_metaknight_special_hi_main,
        status_metaknight_special_hi_loop_main,
        status_metaknight_special_lw_main,
        status_metaknight_special_lw_end_main,
        status_metaknight_special_lw_attack_main,
    );
}


