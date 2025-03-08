use crate::imports::BuildImports::*;

unsafe extern "C" fn status_koopag_GuardOn_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOn()
}

pub fn install() {
    Agent::new("koopag")
    .status(Main, *FIGHTER_STATUS_KIND_GUARD_ON, status_koopag_GuardOn_Main)
    .install();
}