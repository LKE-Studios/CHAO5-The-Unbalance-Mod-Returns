use crate::imports::BuildImports::*;

unsafe extern "C" fn start_koopag_Init(fighter : &mut L2CFighterCommon) {
    let shield_max = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_max"));
    WorkModule::set_float(fighter.module_accessor, shield_max * 2.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
}

pub fn install() {
    Agent::new("koopag")
    .on_start(start_koopag_Init)
    .install();
}