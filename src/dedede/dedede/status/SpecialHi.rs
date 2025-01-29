use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_dedede_SpecialHi_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let start_speed_max_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_speed_max_x"));
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, start_speed_max_x, 0.0);
    sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 1.0);
    sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.0);
    let start_set_speed_max_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_set_speed_max_x"));
    let set = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("set"));
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, start_set_speed_max_x * -1.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, set * -1.0);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    let damage_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut smash::app::KineticEnergy;
    lua_bind::KineticEnergy::mul_speed(damage_energy, &Vector3f{x: 0.5, y: 0.5, z: 0.0});
    0.into()
}

pub fn install() {
    Agent::new("dedede")
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_dedede_SpecialHi_Init)
    .install();
}