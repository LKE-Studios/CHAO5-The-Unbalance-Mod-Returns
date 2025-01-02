use crate::imports::BuildImports::*; 

const GANON_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xaa6510; //Ganon only
const GANON_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const GANON_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xaa6520; //Ganon only
const GANON_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x68d680; //Shared
const GANON_VTABLE_ON_ATTACK_OFFSET: usize = 0xaa6540; //Ganon only
const GANON_VTABLE_ON_DAMAGE_OFFSET: usize = 0x68d9e0; //Shared
const GANON_VTABLE_LINK_EVENT_OFFSET: usize = 0xaa6990; //Ganon only

#[skyline::hook(offset = GANON_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn vtable_ganon_OnAttack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let module_accessor = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(module_accessor);
    let situation_kind = StatusModule::situation_kind(module_accessor);
    if WorkModule::is_flag(module_accessor, *FIGHTER_GANON_STATUS_ATTACK_WORK_FLAG_CRITICAL) {
        special_zoom_critical(module_accessor, log, *FIGHTER_KIND_GANON, hash40("param_attack_air_lw"), 1, 0, 0, 0, 0);
    }
    original!()(vtable, fighter, log)
}

pub fn install() {
	skyline::install_hooks!(
        vtable_ganon_OnAttack
    );
}