use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_LUCARIO )]
fn frame_lucario(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if DamageModule::damage(fighter.module_accessor, 0) >= 100.0 {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_AURA_POWER_INVINCIBLE);
        }
        else {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_AURA_POWER_INVINCIBLE);
        }
        if status_kind == FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END {
            fighter.sub_air_check_fall_common();
            if MotionModule::frame(fighter.module_accessor) < 1.0 {
                MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 0.0, true, true, false);
            }
        }
        if status_kind == FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND {
            fighter.sub_air_check_fall_common();
            if MotionModule::frame(fighter.module_accessor) > 25.0 {
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