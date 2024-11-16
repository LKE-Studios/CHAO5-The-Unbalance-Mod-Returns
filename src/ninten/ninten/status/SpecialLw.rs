use crate::imports::BuildImports::*;

unsafe extern "C" fn status_ninten_SpecialLw_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {
        AbsorberModule::clear_all(fighter.module_accessor);
        0.into()
    }
    else {
        0.into()
    }
}

pub fn install() {
    Agent::new("ness")
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_ninten_SpecialLw_Exec)
    .install();
}