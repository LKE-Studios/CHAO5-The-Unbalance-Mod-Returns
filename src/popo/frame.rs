use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_POPO )]
pub fn frame_popo(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -3.0, 0);
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_popo
    );
}