use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_PITB )]
fn frame_pitb(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        if [
            *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_GLIDE_LANDING,
            *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_MISS_FOOT, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_CLIFF_CATCH,
            *FIGHTER_STATUS_KIND_GLIDE_ATTACK, *FIGHTER_STATUS_KIND_GLIDE_END
        ].contains(&status_kind) { 
            STOP_SE(fighter, Hash40::new("se_pitb_glide_loop"));
        };
        if [
            *FIGHTER_STATUS_KIND_GLIDE_START,
            *FIGHTER_STATUS_KIND_GLIDE
        ].contains(&status_kind) { 
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingl1"), &Vector3f{x:1.25, y:1.25, z:1.25});
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingr1"), &Vector3f{x:1.25, y:1.25, z:1.25});
        }
        else {
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingl1"), &Vector3f{x:1.0, y:1.0, z:1.0});
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingr1"), &Vector3f{x:1.0, y:1.0, z:1.0});
        };
        if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH {
            if situation_kind == *SITUATION_KIND_AIR {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                }
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                }
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_pitb
    );
}
