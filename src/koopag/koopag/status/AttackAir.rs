use crate::imports::BuildImports::*;

unsafe extern "C" fn status_koopag_AttackAir_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
	L2CFighterCommon_status_AttackAir(fighter)
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
    Agent::new("koopag")
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, status_koopag_AttackAir_Main)
    .install();
	skyline::install_hooks!(
		disable_final_smash
	);
}