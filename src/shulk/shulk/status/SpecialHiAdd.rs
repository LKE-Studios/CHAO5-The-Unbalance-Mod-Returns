use crate::imports::BuildImports::*;

unsafe extern "C" fn status_shulk_SpecialHiAdd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    let special_hi_fall_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_x_mul"));
    WorkModule::set_float(fighter.module_accessor, special_hi_fall_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_add"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(shulk_SpecialHiAdd_Main_loop as *const () as _))
}

unsafe extern "C" fn shulk_SpecialHiAdd_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SHULK_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL) {
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let special_hi_fall_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_fall_x_mul"));
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, special_hi_fall_x_mul, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, special_hi_fall_x_mul, 0.0);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SHULK_STATUS_SPECIAL_HI_ADD_FLAG_IS_MOTION_SPEED_Y_START) {
        if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) {
            let height_mul = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_HEIGHT_MUL);
            let add_hit_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("add_hit_speed_y"));
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, add_hit_speed_y, 0.0, 0.0, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            return 0.into();
        }
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    let frame = MotionModule::frame(fighter.module_accessor);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_x, 0.0);
    if frame > 27.0 { 
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        }
        return 0.into();
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            if sum_speed_y > 0.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
                return 0.into();
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("shulk")
    .status(Main, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD, status_shulk_SpecialHiAdd_Main)
    .install();
}