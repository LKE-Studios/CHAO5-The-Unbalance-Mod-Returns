use crate::imports::BuildImports::*;

unsafe extern "C" fn status_waluigi_Wait_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
	let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {	
        fighter.status_pre_Wait()
    }
    else {
        original_status(Pre, fighter, *FIGHTER_STATUS_KIND_WAIT)(fighter)
    }	
}

unsafe extern "C" fn status_waluigi_Wait_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {
        fighter.status_Wait()
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_WAIT)(fighter)
    }	
}

#[skyline::hook(offset = 0x69a6e0)]
unsafe fn autoturn_handler(module_accessor: *mut smash::app::BattleObjectModuleAccessor, some_bool: bool, some_int: i32, some_uint: u32) -> f32 {
    let fighter_kind = smash::app::utility::get_kind(&mut *module_accessor);
	let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let WALUIGI = color >= 120 && color <= 130;
    if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {
        return 0.0;
   	}
    original!()(module_accessor, some_bool, some_int, some_uint)
}

pub fn install() {
    Agent::new("dolly")
    .status(Pre, *FIGHTER_STATUS_KIND_WAIT, status_waluigi_Wait_Pre)
    .status(Main, *FIGHTER_STATUS_KIND_WAIT, status_waluigi_Wait_Main)
    .install();
    skyline::install_hook!(autoturn_handler);
}

