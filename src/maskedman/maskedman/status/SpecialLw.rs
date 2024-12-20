use crate::imports::BuildImports::*;

pub static attack_power : f32 = 36.0;
pub static attack_angle : u64 = 45;
pub static size : f32 = 20.0;

unsafe extern "C" fn status_maskedman_SpecialLw_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {
        KineticModule::clear_speed_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x32de6245ed), *FIGHTER_HAVE_ITEM_WORK_MAIN, true);
        let time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("time"));
        let stop_y_time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("stop_y_time"));
        WorkModule::set_int(fighter.module_accessor, time, *FIGHTER_LUCAS_STATUS_SPECIAL_LW_WORK_INT_TIME);
        WorkModule::set_int(fighter.module_accessor, stop_y_time, *FIGHTER_LUCAS_STATUS_SPECIAL_LW_WORK_INT_STOP_Y_TIME);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(maskedman_SpecialLw_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
    }
}

unsafe extern "C" fn maskedman_SpecialLw_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let int_time = WorkModule::get_int(fighter.module_accessor, FIGHTER_MASKEDMAN_STATUS_SPECIAL_LW_WORK_INT_TIME);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            SoundModule::play_landing_se(fighter.module_accessor, Hash40::new("se_lucas_landing01"));
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_start"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_start"), -1.0, 1.0, 0.0, false, false);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        }
    }
    if (frame > 25.0 && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) 
    || frame > 96.0 || int_time > 0 {
        MotionModule::set_rate(fighter.module_accessor, 0.0);
        StopModule::end_stop(fighter.module_accessor);
        WorkModule::add_int(fighter.module_accessor, 1, FIGHTER_MASKEDMAN_STATUS_SPECIAL_LW_WORK_INT_TIME);
        if int_time == 4 {
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_direction"), false, false);
            effect!(fighter, *MA_MSC_CMD_EFFECT_EFFECT, hash40("sys_bomb_a"), hash40("throw"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);
            ATTACK(fighter, 0, 0, Hash40::new("throw"), attack_power, attack_angle, 65, 0, 17, size, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_lucas_throw_l02"), 0);
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_bomb_ll"), true, false, false, false, enSEType(0));
        }
        if int_time > 6 {
            AttackModule::clear_all(fighter.module_accessor);
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_direction"), false, false);
        }
        if int_time > 10 {
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_direction"), false, false);
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
                return 0.into();
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(),false.into());
                return 0.into();
            }
        }
    }
    else if frame > 20.0 && (frame as i32 % 4 == 0) {
        effect!(fighter, *MA_MSC_CMD_EFFECT_EFFECT, hash40("sys_damage_fire"), hash40("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
            return 0.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(),false.into());
            return 0.into();
        }
    }
    0.into()
}

unsafe extern "C" fn status_maskedman_SpecialLw_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_lucas_throw_l02"), 0);
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_MASKEDMAN_STATUS_SPECIAL_LW_WORK_INT_TIME);
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
    }
}

pub fn install() {
    Agent::new("lucas")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_maskedman_SpecialLw_Main)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_maskedman_SpecialLw_End)
    .install();
}