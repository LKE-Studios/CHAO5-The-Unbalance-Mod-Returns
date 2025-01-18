use crate::imports::BuildImports::*;

pub unsafe extern "C" fn start_pfushigisou_Init(fighter : &mut L2CFighterCommon) {}

pub fn install() {
    Agent::new("pfushigisou")
    .on_start(start_pfushigisou_Init)
    .install();
}