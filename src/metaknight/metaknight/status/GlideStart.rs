use crate::imports::BuildImports::*;

unsafe extern "C" fn status_metaknight_GlideStart_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_GLIDE);
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    fighter.status_GlideStart()
}

pub fn install() {
    Agent::new("metaknight")
    .status(Main, *FIGHTER_STATUS_KIND_GLIDE_START, status_metaknight_GlideStart_Main)
    .install();
}