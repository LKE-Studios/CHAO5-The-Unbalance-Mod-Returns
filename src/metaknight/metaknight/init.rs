use crate::imports::BuildImports::*;

unsafe extern "C" fn start_metaknight_Init(fighter : &mut L2CFighterCommon) {}

pub fn install() {
    Agent::new("metaknight")
    .on_start(start_metaknight_Init)
    .install();
}