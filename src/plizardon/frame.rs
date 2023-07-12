use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_PLIZARDON )]
fn frame_plizardon(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        //SFX Controllers
        if [
            *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_GLIDE_LANDING,
            *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_MISS_FOOT, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_CLIFF_CATCH,
            *FIGHTER_STATUS_KIND_GLIDE_ATTACK, *FIGHTER_STATUS_KIND_GLIDE_END
        ].contains(&status_kind) { 
            STOP_SE(fighter, Hash40::new("se_plizardon_glide_start"));
            STOP_SE(fighter, Hash40::new("se_plizardon_glide_loop"));
            STOP_SE(fighter, Hash40::new("se_plizardon_special_h02_02"));
        };
        if [hash40("jump_aerial_f2"), hash40("jump_aerial_f3"), hash40("jump_aerial_f4"), 
        hash40("jump_aerial_f5")].contains(&motion_kind) && MotionModule::frame(fighter.module_accessor) == 44.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if MotionModule::frame(fighter.module_accessor) > 50.0 &&
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_HI2, false);
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_plizardon
    );
}
