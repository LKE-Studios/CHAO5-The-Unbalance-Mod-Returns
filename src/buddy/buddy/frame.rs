use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_buddy(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
        if MotionModule::frame(fighter.module_accessor) < 46.0 {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
                MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 45.0, true, true, false);
            }
        }
    }
    if [*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_AIR,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_AIR_TURN,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_END,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_FALL,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_JUMP,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_JUMP_AERIAL,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_JUMP_SQUAT,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_LANDING,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_START,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_TURN,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_B,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_F
    ].contains(&status_kind) { 
        MotionModule::set_rate(fighter.module_accessor, 5.0);
    }
    if status_kind == *FIGHTER_STATUS_KIND_GLIDE {
        let mut angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
        let mut power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
        let angle_se_pitch_ratio = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_se_pitch_ratio"));
        let angle_se_pitch_partial_ratio = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_se_pitch_partial_ratio"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_buddy_glide_loop"), 1.0 + angle * angle_se_pitch_ratio);
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_buddy_wing"), 1.0 + angle * angle_se_pitch_partial_ratio);
        let angle_up_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_up_max"));
        let angle_down_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_down_max"));
        let angle_up_set_motion_rate_partial =  WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_up_set_motion_rate_partial"));
        let angle_down_set_motion_rate_partial =  WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_down_set_motion_rate_partial"));
        if angle >= angle_down_max && angle < 0.0 {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, 1.0 + angle * angle_down_set_motion_rate_partial);
        }
        if angle <= angle_up_max && angle > 0.0 {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, 1.0 + angle * angle_up_set_motion_rate_partial);
        }
        let frame_partial = MotionModule::frame_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING);
        if frame_partial >= 4.0 && frame_partial < 5.0 {
            PLAY_SE(fighter, Hash40::new("se_buddy_wing"));
        }
    }
    if ![*FIGHTER_STATUS_KIND_GLIDE_START, *FIGHTER_STATUS_KIND_GLIDE].contains(&status_kind) { 
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_buddy_glide_loop"), 0);
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_buddy_glide_start"), 0);
    };
    if status_kind == *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(fighter.module_accessor, -10.0, 0);
        }
    };
}

pub fn install() {
    Agent::new("buddy")
    .on_line(Main, frame_buddy)
    .install();
}
