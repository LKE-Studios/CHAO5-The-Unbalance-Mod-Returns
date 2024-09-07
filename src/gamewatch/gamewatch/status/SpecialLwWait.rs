use crate::imports::BuildImports::*;

unsafe extern "C" fn status_gamewatch_SpecialLwWait_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    0.into()
}

pub fn install() {
    Agent::new("gamewatch")
    .status(Exec, *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_WAIT, status_gamewatch_SpecialLwWait_Exec)
    .install();
}