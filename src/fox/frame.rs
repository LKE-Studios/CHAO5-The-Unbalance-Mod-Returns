use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;

#[fighter_frame( agent = FIGHTER_KIND_FOX )]
fn frame_fox(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
            && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING, true);
            }
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                if ControlModule::get_command_flag_cat(fighter.module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP != 0
                    && ControlModule::get_stick_y(fighter.module_accessor) < -0.66
                    && KineticModule::get_sum_speed_y(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                    WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            fighter.sub_transition_group_check_air_cliff();
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -5.0, 0);
            }
        }
        if status_kind == *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH || status_kind == *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -50.0, 0);
            }
        }
        if motion_kind == hash40("appeal_hi_r") || motion_kind == hash40("appeal_hi_l") {
            if frame > 41.0 && frame < 44.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, false);
                }
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_fox
    );
}