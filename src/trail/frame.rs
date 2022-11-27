//use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::phx::Hash40;
use smash::hash40;
//use smash::phx::Vector3f;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;

static mut HOLD_TIME : [f32; 8] = [0.0; 8];

#[fighter_frame( agent = FIGHTER_KIND_TRAIL )]
fn trail_opff(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let kind = smash::app::utility::get_kind(boma);
        if ![*FIGHTER_STATUS_KIND_GLIDE_START, *FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR{
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP){
                HOLD_TIME[ENTRY_ID] +=1.0;
            }
            if HOLD_TIME[ENTRY_ID] == 24.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_START.into(), true.into());
            }
        }
        else {
            HOLD_TIME[ENTRY_ID] = 0.0;
        };
        if [*FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_JUMP].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_WAIT].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_GLIDE].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if status_kind == *FIGHTER_STATUS_KIND_LANDING || status_kind == *FIGHTER_STATUS_KIND_LANDING_LIGHT || status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR || status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR || status_kind == *FIGHTER_STATUS_KIND_DEAD ||
        status_kind == *FIGHTER_STATUS_KIND_MISS_FOOT || status_kind == *FIGHTER_STATUS_KIND_DAMAGE || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR ||
        status_kind == *FIGHTER_STATUS_KIND_CLIFF_CATCH || status_kind == *FIGHTER_STATUS_KIND_GLIDE_LANDING || status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F || status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N { 
            macros::STOP_SE(fighter, Hash40::new("se_trail_glide_loop"));
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_attack_up"), false, false);
        };
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_ATTACK {
            macros::STOP_SE(fighter, Hash40::new("se_trail_glide_loop"));
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_attack_up"), false, false);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        };
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_END {
            macros::STOP_SE(fighter, Hash40::new("se_trail_glide_loop"));
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_attack_up"), false, false);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        };
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_START {
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        }
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_ATTACK {
            macros::STOP_SE(fighter, Hash40::new("se_trail_glide_loop"));
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_attack_up"), false, false);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        }
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_END {
            macros::STOP_SE(fighter, Hash40::new("se_trail_glide_loop"));
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_attack_up"), false, false);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        };
        /*if status_kind == *FIGHTER_STATUS_KIND_GLIDE {
            if MotionModule::frame(fighter.module_accessor) >= 0.0 && MotionModule::frame(fighter.module_accessor) < 1.0 {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_status_attack_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.6, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.8, /*G*/ 0.13, /*B*/ 0.52);
            }
        }*/
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
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -35.0, 0);
            }
        }
        if kind == *FIGHTER_KIND_TRAIL {
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
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        trail_opff
    );
}
