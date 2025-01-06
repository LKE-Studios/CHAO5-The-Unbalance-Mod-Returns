use crate::imports::BuildImports::*;

pub unsafe extern "C" fn start_pfushigisou_Init(fighter : &mut L2CFighterCommon) {
    let charge_effect_scale_min = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_guard"), hash40("charge_effect_scale_min"));
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
    WorkModule::set_float(fighter.module_accessor, charge_effect_scale_min, *FIGHTER_INSTANCE_WORK_ID_FLOAT_EFFECT_SCALE_MUL);
}

pub fn install() {
    Agent::new("pfushigisou")
    .on_start(start_pfushigisou_Init)
    .install();
}