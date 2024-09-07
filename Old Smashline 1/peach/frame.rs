use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_PEACH )]
fn frame_peach(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if [*FIGHTER_STATUS_KIND_ATTACK_S4_START,*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("havel"), &Vector3f{ x: 2.5, y: 2.5, z: 2.5 });
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{ x: 2.0, y: 2.0, z: 2.0 });
            AttackModule::set_attack_scale(fighter.module_accessor, 2.5, true);
        }
        else {
            AttackModule::set_attack_scale(fighter.module_accessor, 1.0, true);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_peach
    );
}