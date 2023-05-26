use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent}
    },
    smash_script::*,
    smashline::*,
};

#[status_script( agent = "metaknight", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_metaknight_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    let energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut smash::app::KineticEnergy;
    let anti_wind = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND) as *mut smash::app::KineticEnergy;

    smash::app::lua_bind::KineticEnergy::clear_speed(energy);
    smash::app::lua_bind::KineticEnergy::clear_speed(anti_wind);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    ret
}

#[status_script( agent = "metaknight", status = FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_metaknight_special_hi_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    let energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut smash::app::KineticEnergy;
    let anti_wind = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND) as *mut smash::app::KineticEnergy;
    
    smash::app::lua_bind::KineticEnergy::clear_speed(energy);
    smash::app::lua_bind::KineticEnergy::clear_speed(anti_wind);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    ret
}

pub fn install() {
    install_status_scripts!(
        status_metaknight_special_hi_main,
        status_metaknight_special_hi_loop_main
    );
}


