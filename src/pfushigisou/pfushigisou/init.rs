use crate::imports::BuildImports::*;

pub unsafe extern "C" fn start_pfushigisou_Init(fighter : &mut L2CFighterCommon) {
    let float_charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHARGE_FRAME);
    WorkModule::set_float(fighter.module_accessor, float_charge, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHARGE_FRAME);
}

pub fn install() {
    Agent::new("pfushigisou")
    .on_start(start_pfushigisou_Init)
    .install();
}