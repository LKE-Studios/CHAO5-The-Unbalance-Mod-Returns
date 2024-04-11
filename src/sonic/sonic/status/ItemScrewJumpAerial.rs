use crate::imports::BuildImports::*;

unsafe extern "C" fn status_sonic_ItemScrewJumpAerial_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_ItemScrewJumpAerialSub();
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_ItemScrewJumpAerial_Main as *const () as _))
}

pub fn install() {
    Agent::new("sonic")
    .status(Main, *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP_AERIAL, status_sonic_ItemScrewJumpAerial_Main)
    .install();
}