use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::app::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;

#[fighter_frame( agent = FIGHTER_KIND_LUCARIO )]
fn frame_lucario(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);

        if status_kind == FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
                fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END.into(), true.into())
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
            }
        }
        if status_kind == FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END {
            if MotionModule::frame(fighter.module_accessor) < 1.0 {
                MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 0.0, true, true, false);
            }
            if MotionModule::frame(fighter.module_accessor) > 30.0 && situation_kind == *SITUATION_KIND_AIR && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
            }
        }
        if status_kind == FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND {
            if MotionModule::frame(fighter.module_accessor) > 34.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_lucario
    );
}