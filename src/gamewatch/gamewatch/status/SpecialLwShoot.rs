use crate::imports::BuildImports::*;

unsafe extern "C" fn status_gamewatch_SpecialLwShoot_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    let attack_min = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("attack_min"));
    let mut special_lw_attack = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_ATTACK);
    if special_lw_attack < attack_min {
        special_lw_attack = attack_min;
    }
    WorkModule::set_float(fighter.module_accessor, special_lw_attack, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_WORK_FLOAT_ATTACK);
    0.into()
}

pub fn install() {
    Agent::new("gamewatch")
    .status(Init, *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_WAIT, status_gamewatch_SpecialLwShoot_Init)
    .install();
}