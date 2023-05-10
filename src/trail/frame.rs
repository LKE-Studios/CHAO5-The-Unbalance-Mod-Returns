//use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::phx::Hash40;
use smash::hash40;
//use smash::phx::Vector3f;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;

#[fighter_frame( agent = FIGHTER_KIND_TRAIL )]
fn frame_trail(fighter: &mut L2CFighterCommon) {
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
            *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F,
            *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N
        ].contains(&status_kind) { 
            macros::STOP_SE(fighter, Hash40::new("se_trail_glide_loop"));
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_attack_up"), false, false);
        };
        if status_kind == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            fighter.sub_air_check_fall_common();
            if MotionModule::frame(fighter.module_accessor) > 50.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            };
        }
        if status_kind == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_LW_ATTACK {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -35.0, 0);
            }
        }
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_l") && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_APPEAL_HI_KIND) == 2 {
                if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                    macros::SLOW_OPPONENT(fighter, 10.0, 150.0);
                }
            }
        }
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_r") && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_APPEAL_HI_KIND) == 2 {
                if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                    macros::SLOW_OPPONENT(fighter, 10.0, 150.0);
                }
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_trail
    );
}
