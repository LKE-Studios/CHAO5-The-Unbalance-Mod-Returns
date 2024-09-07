use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_gekkouga_Main(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if status_kind == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_ATTACK {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -20.0, 0);
            }
        };
    }
}

pub fn install() {
    Agent::new("gekkouga")
    .on_line(Main, frame_gekkouga_Main)
    .install();
}