use crate::imports::BuildImports::*;

unsafe extern "C" fn start_pit_Init(fighter : &mut L2CFighterCommon) {
    let fly_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("fly_frame_max"));
    WorkModule::set_float(fighter.module_accessor, fly_frame_min as f32, FIGHTER_PIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_FUEL);
}

pub fn install() {
    Agent::new("pit")
    .on_start(start_pit_Init)
    .install();
}