use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_GAOGAEN )]
pub fn frame_gaogaen(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            fighter.sub_wait_ground_check_common(false.into());
        };
        if status_kind == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_HIT {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -60.0, 0);
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_gaogaen
    );
}