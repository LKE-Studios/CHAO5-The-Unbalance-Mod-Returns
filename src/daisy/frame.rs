use smash::lib::lua_const::*;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;

#[fighter_frame( agent = FIGHTER_KIND_DAISY )]
fn frame_daisy(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if [*FIGHTER_STATUS_KIND_ATTACK_S4_START,*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("havel"), &Vector3f{ x: 4.0, y: 4.0, z: 4.0 });
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{ x: 4.0, y: 4.0, z: 4.0 });
            AttackModule::set_attack_scale(fighter.module_accessor, 4.0, true);
        }
        else {
            AttackModule::set_attack_scale(fighter.module_accessor, 1.0, true);
        };
        if status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_HIT_END {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -15.0, 0);
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_daisy
    );
}