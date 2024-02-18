use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_gaogaen_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    frame_common(fighter);
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        fighter.sub_wait_ground_check_common(false.into());
    };
    if status_kind == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_HIT {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(fighter.module_accessor, -60.0, 0);
        }
    };
}

pub fn install() {
    Agent::new("gaogaen")
    .on_line(Main, frame_gaogaen_Main)
    .install();
}