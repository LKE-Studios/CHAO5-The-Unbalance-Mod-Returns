use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_buddy(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    //SFX Controllers
    if [
        *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_GLIDE_LANDING,
        *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_MISS_FOOT, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_CLIFF_CATCH,
        *FIGHTER_STATUS_KIND_GLIDE_ATTACK, *FIGHTER_STATUS_KIND_GLIDE_END
    ].contains(&status_kind)  { 
        STOP_SE(fighter, Hash40::new("se_buddy_glide_loop"));
    };
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
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_buddy_glide_loop"), 1.0 + angle * angle_se_pitch_ratio /*-0.005*/);
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_buddy_wing"), 1.0 + angle * angle_se_pitch_partial_ratio /*0.0048*/);
        let angle_up_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_up_max"));
        let angle_down_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_down_max"));
        let angle_up_set_motion_rate_partial =  WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_up_set_motion_rate_partial"));
        let angle_down_set_motion_rate_partial =  WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_down_set_motion_rate_partial"));
        if angle >= angle_down_max && angle < 0.0 {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, 1.0 + angle * angle_down_set_motion_rate_partial /*0.01*/);
        }
        if angle <= angle_up_max && angle > 0.0 {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, 1.0 + angle * angle_up_set_motion_rate_partial /*0.018*/);
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    };
    if status_kind == *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH {
        KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
        KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
        fighter.sub_air_check_fall_common();
        fighter.sub_wait_ground_check_common(false.into());
        WorkModule::set_int(fighter.module_accessor, 5, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(fighter.module_accessor, -10.0, 0);
        }
        if MotionModule::frame(fighter.module_accessor) > 15.0 {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
                }
            }
        }
    };
}

pub fn install() {
    Agent::new("buddy")
    .on_line(Main, frame_buddy)
    .install();
}
