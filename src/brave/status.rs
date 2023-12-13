use crate::imports::BuildImports::*;

#[status_script( agent = "brave", status = FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_STEEL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn status_brave_special_lw_steel_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_lw10").into(), Hash40::new("special_air_lw10").into(), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(brave_special_lw_steel_loop as *const () as _))
}

unsafe fn brave_special_lw_steel_kinetic_function(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            KineticModule::clear_speed_all(fighter.module_accessor);
        }
    }
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("air_speed_y_stable"));
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_CONTROL_ENERGY) {
                sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
                let common_start_control_accel_x_mul_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("common_start_control_accel_x_mul_air"));
                let common_start_control_max_speed_x_mul_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("common_start_control_max_speed_x_mul_air"));
                sv_kinetic_energy!(controller_set_accel_x_mul, fighter, common_start_control_accel_x_mul_air);
                sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, common_start_control_max_speed_x_mul_air, 0.0);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            }
            else {
                sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0);
            }
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, sum_speed_x, sum_speed_y);
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_accel_x_mul, air_accel_y);
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_STEEL) {
        let steel_gravity = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("steel_gravity"));
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, steel_gravity * air_accel_y);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable);
        sv_kinetic_energy!(unable, fighter, FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
        sv_kinetic_energy!(unable, fighter, FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
        sv_kinetic_energy!(unable, fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    }
    else {
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_accel_y);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable);
        sv_kinetic_energy!(enable, fighter, FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
        sv_kinetic_energy!(enable, fighter, FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
        sv_kinetic_energy!(enable, fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    }
}

unsafe extern "C" fn brave_special_lw_steel_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if situation_kind != prev_situation_kind {
        fighter.sub_change_motion_by_situation(Hash40::new("special_lw10").into(), Hash40::new("special_air_lw10").into(), false.into());
    }
    fighter.sub_set_ground_correct_by_situation(true.into());
    brave_special_lw_steel_kinetic_function(fighter);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_STEEL) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new_raw(0x1cf0b7fece), -1);
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_SOUND, Hash40::new_raw(0x1bb6d90083), -1);
            }
        }
        if !StopModule::is_stop(fighter.module_accessor) {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_INT_STEEL_FRAME);
            let steel_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_INT_STEEL_FRAME);
            if steel_frame >= 0 {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_STEEL);
                let FIGHTER_PTR = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
                FighterSpecializer_Brave::special_lw_deactive_command(FIGHTER_PTR);
                fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_STEEL_END.into(), false.into());
            }
        }
    }
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_STEEL);
        fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_STEEL_END.into(), false.into());
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        status_brave_special_lw_steel_main
    );
}

