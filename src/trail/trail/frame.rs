use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_trail_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_GLIDE {
        let mut angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
        let angle_se_pitch_ratio = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_se_pitch_ratio"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_trail_glide_loop"), 1.0 + angle * angle_se_pitch_ratio);
    }
    if ![*FIGHTER_STATUS_KIND_GLIDE_START, *FIGHTER_STATUS_KIND_GLIDE].contains(&status_kind) { 
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_trail_glide_loop"), 0);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_status_attack_up"), false, false);
    };
    if status_kind == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END {
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
        }
        if frame > 39.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        };
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        fighter.sub_air_check_fall_common();
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            if frame > 42.0 {
                CancelModule::enable_cancel(fighter.module_accessor);
            };
        }
    }
    if status_kind == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_LW_ATTACK {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(fighter.module_accessor, -35.0, 0);
        }
    }
}


pub fn install() {
    Agent::new("trail")
    .on_line(Main, frame_trail_Main)
    .install();
}