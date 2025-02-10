use crate::imports::BuildImports::*;

pub static special_lw_speed_x_max : f32 = 1.5;
pub static special_lw_gravity_speed : f32 = 0.35;

pub unsafe extern "C" fn status_funky_SpecialLw_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
        0.into()
    }
    else {
        original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
    }
}

pub unsafe extern "C" fn status_funky_SpecialLw_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, special_lw_speed_x_max, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, special_lw_speed_x_max, 0.0);
        let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut smash::app::KineticEnergy;
        let speed_y = KineticEnergy::get_speed_y(gravity);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_lw_gravity_speed);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_lw_gravity_speed);
        0.into()
    }
    else {
        0.into()
    }
}

unsafe fn funky_SpecialLw_motion_helper(fighter: &mut L2CFighterCommon, situation_change: bool) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !situation_change {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_start"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !situation_change {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_start"), -1.0, 1.0, 0.0, false, false);
        }
    }
}

pub unsafe extern "C" fn status_funky_SpecialLw_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        funky_SpecialLw_motion_helper(fighter, false);
        JostleModule::set_status(fighter.module_accessor, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(funky_SpecialLw_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
    }
}

unsafe extern "C" fn funky_SpecialLw_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let mut stick_direction = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_STICK_DIRECTION);
    stick_direction = ControlModule::get_stick_dir(fighter.module_accessor) * (180.0 / PI);
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && !fighter.sub_wait_ground_check_common(false.into()).get_bool()
    && fighter.sub_air_check_fall_common().get_bool() {
        return 0.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != fighter.global_table[PREV_SITUATION_KIND].get_i32() {
        funky_SpecialLw_motion_helper(fighter, true);
    }
    if frame == 2.0 {
        StatusModule::set_keep_situation_air(fighter.module_accessor, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("poke_mijumaeu_wave"), Hash40::new("top"), &Vector3f{x:0.0, y:0.0, z:-4.0}, &Vector3f{x:0.0, y:0.0, z:0.0}, 1.7, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("poke_mijumaeu_ripple"), Hash40::new("top"), &Vector3f{x:0.0, y:0.0, z:0.0}, &Vector3f{x:0.0, y:0.0, z:0.0}, 1.15, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("poke_mijumaeu_naminori_f"), Hash40::new("top"), &Vector3f{x:0.0, y:0.0, z:-5.0}, &Vector3f{x:0.0, y:0.0, z:0.0}, 1.9, true, 0, 0, 0, 0, 0, true, true) as u32;
    }
    if frame >= 15.0 && frame <= 45.0 {
        if stick_x >= -0.2 && stick_x <= 0.2 && stick_y >= -0.2 && stick_y <= 0.2 {
            WorkModule::set_float(fighter.module_accessor, 361.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_STICK_DIRECTION);
        }
        else if stick_direction <= -67.5 {
            WorkModule::mul_float(fighter.module_accessor, -1.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_STICK_DIRECTION);
        } 
        if stick_direction >= -67.5 && stick_direction < -22.5 && stick_x < 0.0 {
            //Nothing LOL            
        }
        else if (stick_direction >= 67.5 && stick_direction <= 90.0 && stick_y < 0.0) 
        && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_music"), 0.0, 1.0, false, 0.0, false, false);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        }
        else if stick_direction >= -67.5 && stick_direction < -22.5 && stick_x > 0.0 {
            //Nothing Again LOL    
        }
        else if (stick_direction >= -22.5 && stick_direction <= 22.5 && stick_x < 0.0) 
        && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_flip"), 0.0, 1.0, false, 0.0, false, false);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
            KineticModule::clear_speed_all(fighter.module_accessor);
        }
        else if stick_direction == 361.0 {
            //Nothing Again LOL    
        }
        else if (stick_direction >= -22.5 && stick_direction <= 22.5 && stick_x > 0.0) 
        && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_jump"), 0.0, 1.0, false, 0.0, false, false);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
            KineticModule::clear_speed_all(fighter.module_accessor);
        }
        else if stick_direction > 22.5 && stick_direction <= 67.5 && stick_x < 0.0 {
            //Nothing Again LOL
        }
        else if (stick_direction > 67.5 && stick_direction <= 90.0 && stick_y > 0.0) 
        && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_pose"), 0.0, 1.0, false, 0.0, false, false);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        }
        else {
            //Nothing for the last time LOL   
        };
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn status_funky_SpecialLw_CheckAttack(fighter: &mut L2CFighterCommon, param2: &L2CValue, param3: &L2CValue) -> L2CValue {
    let attacker_module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let table = param3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let color = WorkModule::get_int(attacker_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
    if utility::get_kind(attacker_module_accessor) == *FIGHTER_KIND_DONKEY && FUNKY && category == *BATTLE_OBJECT_CATEGORY_FIGHTER 
    && (MotionModule::motion_kind(attacker_module_accessor) == hash40("special_lw_music") 
    || MotionModule::motion_kind(attacker_module_accessor) == hash40("special_air_lw_music")) {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let module_accessor = sv_battle_object::module_accessor(object_id);
            let rand_val = sv_math::rand(hash40("fighter"), 3);
            if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), true.into());
            }
        }
    }
    
    0.into()
}

pub unsafe extern "C" fn status_funky_SpecialLw_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("poke_mijumaeu_wave"), false, false);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("poke_mijumaeu_ripple"), false, false);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("poke_mijumaeu_naminori_f"), false, false);
    0.into()
}

pub fn install() {
    Agent::new("donkey")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_funky_SpecialLw_Pre)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_funky_SpecialLw_Init)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_funky_SpecialLw_Main)
    .status(CheckAttack, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_funky_SpecialLw_CheckAttack)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_funky_SpecialLw_End)    
    .install();
}