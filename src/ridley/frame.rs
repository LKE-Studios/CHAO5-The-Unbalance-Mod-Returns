//use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::phx::Hash40;
use smash::hash40;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;

#[fighter_frame( agent = FIGHTER_KIND_RIDLEY )]
fn frame_ridley(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if [
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_ESCAPE_AIR,
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_MISS_FOOT,
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_CLIFF_CATCH,
            *FIGHTER_STATUS_KIND_GLIDE_LANDING,
            *FIGHTER_STATUS_KIND_GLIDE_ATTACK,
            *FIGHTER_STATUS_KIND_GLIDE_END,
        ].contains(&status_kind) { 
            macros::STOP_SE(fighter, Hash40::new("se_ridley_glide_loop"));
        };
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE {
            if MotionModule::frame_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING) >= 25.0 && MotionModule::frame_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING) < 26.0 {
                macros::PLAY_SE(fighter, Hash40::new("se_ridley_jump02"));
            }
        }
        if status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_END {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_ridley
    );
}
