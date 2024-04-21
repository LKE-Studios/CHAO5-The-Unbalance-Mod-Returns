use crate::imports::BuildImports::*;

unsafe extern "C" fn status_waluigi_SpecialFAttack_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {   
        0.into()
    }
    else {
        original_status(Main, fighter, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK)(fighter)
    }
}

pub fn install() {
    Agent::new("dolly")
    .status(Main, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK, status_waluigi_SpecialFAttack_Main)
    .install();
}