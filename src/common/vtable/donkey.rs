use crate::imports::BuildImports::*; 

const DONKEY_VTABLE_ON_ATTACK_OFFSET: usize = 0x994480; //Donkey only
const DONKEY_VTABLE_ON_ATTACK_2_OFFSET: usize = 0x994790; //Donkey only
const DONKEY_VTABLE_LINK_EVENT_OFFSET: usize = 0x993ee0; //Donkey only

#[skyline::hook(offset = DONKEY_VTABLE_LINK_EVENT_OFFSET)]
unsafe extern "C" fn vtable_donkey_LinkEvent(vtable: u64, fighter: &mut Fighter, event: u64) -> u64 {
    let module_accessor = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(module_accessor);
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
    if FUNKY && [*FIGHTER_STATUS_KIND_FINAL, *FIGHTER_DONKEY_STATUS_KIND_FINAL_ATTACK].contains(&status_kind) {
        return 1;
    }
    original!()(vtable, fighter, event)
}

#[skyline::hook(offset = DONKEY_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn vtable_donkey_OnAttack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let module_accessor = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(module_accessor);
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
    if FUNKY && [*FIGHTER_STATUS_KIND_FINAL, *FIGHTER_DONKEY_STATUS_KIND_FINAL_ATTACK].contains(&status_kind) {
        return 1;
    }
    original!()(vtable, fighter, log)
}

#[skyline::hook(offset = DONKEY_VTABLE_ON_ATTACK_2_OFFSET)]
unsafe extern "C" fn vtable_donkey_OnAttack2(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let module_accessor = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(module_accessor);
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
    if FUNKY && [*FIGHTER_STATUS_KIND_FINAL, *FIGHTER_DONKEY_STATUS_KIND_FINAL_ATTACK].contains(&status_kind) {
        return 1;
    }
    original!()(vtable, fighter, log)
}

pub fn install() {
	skyline::install_hooks!(
        vtable_donkey_LinkEvent,
        vtable_donkey_OnAttack,
        vtable_donkey_OnAttack2
    );
}