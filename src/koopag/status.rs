use crate::imports::BuildImports::*;

#[status_script(agent = "koopag", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)] //Properly allow Giga Bowser to use Nair and Dair
unsafe fn status_koopag_attackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
	smash::lua2cpp::L2CFighterCommon_status_AttackAir(fighter)
}

pub unsafe fn dtilt_input(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) 
	&& ControlModule::get_stick_y(module_accessor) <= -0.25 
	&& (
		ControlModule::get_stick_y(module_accessor) > -0.72 || ControlModule::get_flick_y(module_accessor) > 4
	) 
	&& ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_GUARD) 
	&& ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
		return true;
	}
	else {
		return false;
	}
}

#[skyline::hook(replace = WorkModule::is_enable_transition_term)]
pub unsafe fn disable_final_smash(
    module_accessor: &mut smash::app::BattleObjectModuleAccessor,
    term: i32
) -> bool {
	let fighter_kind = smash::app::utility::get_kind(module_accessor);
	let ret = original!()(module_accessor, term);
	if fighter_kind == *FIGHTER_KIND_KOOPAG {
		if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL {
			return false;
		}
	}
	return ret;
}

pub fn install() {
    install_status_scripts!(
        status_koopag_attackair_main
    );
	skyline::install_hooks!(
		disable_final_smash
	);
}