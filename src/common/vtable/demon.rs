use crate::imports::BuildImports::*; 

const DEMON_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x930d60; //Kazuya only
const DEMON_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x930ff0; //Kazuya only
const DEMON_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x931680; //Kazuya only
const DEMON_VTABLE_ON_ATTACK_OFFSET: usize = 0x932f50; //Kazuya only
const DEMON_VTABLE_LINK_EVENT_OFFSET: usize = 0x933800; //Kazuya only
const DEMON_VTABLE_ON_GRAB_OFFSET: usize = 0x934310; //Kazuya only

#[skyline::hook(offset = DEMON_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn vtable_demon_OnAttack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let module_accessor = fighter.battle_object.module_accessor;
    if WorkModule::is_flag(module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_WORK_FLAG_CRITICAL) {
        special_zoom_critical(module_accessor, log, *FIGHTER_KIND_DEMON, hash40("param_attack_s4"), 1, 0, 0, 0, 0);
    }
    call_original!(vtable, fighter, log)
}

pub fn install() {
	skyline::install_hooks!(
        vtable_demon_OnAttack
    );
}