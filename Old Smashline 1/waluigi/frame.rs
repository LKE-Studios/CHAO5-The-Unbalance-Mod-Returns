use crate::imports::BuildImports::*;
use crate::dolly::frame::*;

pub static mut RANDOM_NUM : [i32; 8] = [0; 8];
pub static mut RANDOM_NUM_SELECTED : [bool; 8] = [false; 8];
pub static mut ALREADY_BLOCK : [bool; 8] = [false; 8];
pub static mut COPTER_DIR : [f32; 8] = [0.0; 8];
pub static mut STICK_DIRECTION : [f32; 8] = [0.0; 8];
pub static max_speed_x : f32 = 1.5;
pub static x_acl_air : f32 = 0.05;
pub static y_acl_air : f32 = 0.7;
pub static ground_jump_x_dist : f32 = 1.0;
pub static init_speed_y : f32 = 1.7;
pub static dive_speed_y : f32 = 3.0;
pub static dive_speed_x : f32 = 0.2;

pub unsafe fn scale_waluigi(fighter: &mut L2CFighterCommon) {
    ModelModule::set_scale(fighter.module_accessor, 0.9);
    AttackModule::set_attack_scale(fighter.module_accessor, 0.9, true);
    GrabModule::set_size_mul(fighter.module_accessor, 0.9);
}

pub unsafe fn attack_dash_waluigi(fighter: &mut L2CFighterCommon) {
    let frame = MotionModule::frame(fighter.module_accessor);
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_dash") {
        if frame > 30.0 && frame < 36.0 {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                MotionModule::set_frame(fighter.module_accessor, 44.0, true);
            }
        }
    }
}

pub unsafe fn special_n_waluigi(fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if [hash40("special_n"),hash40("special_air_n")].contains(&motion_kind) && !RANDOM_NUM_SELECTED[ENTRY_ID] {
        if frame <= 2.0 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"), false);
        };
        RANDOM_NUM[ENTRY_ID] = smash::app::sv_math::rand(hash40("dolly"), 10);
        RANDOM_NUM_SELECTED[ENTRY_ID] = true;
    };
    if ![hash40("special_n"),hash40("special_air_n")].contains(&motion_kind) {
        RANDOM_NUM[ENTRY_ID] = 0;
        RANDOM_NUM_SELECTED[ENTRY_ID] = false;
        ALREADY_BLOCK[ENTRY_ID] = false;
    };
    ArticleModule::generate_article(fighter.module_accessor, *WEAPON_KIND_DOLLY_CAP, true, 1);
}

pub unsafe fn special_s_waluigi(fighter: &mut L2CFighterCommon) {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND].contains(&status_kind) 
    || motion_kind == hash40("special_air_b_start") {
        StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_S, true);
    }
    if status_kind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK || status_kind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
    };
    //Removes Up Throw loop sound effect if it doesnt reach the stop frame on the sound script
    if motion_kind != hash40("throw_hi") {
        STOP_SE(fighter, Hash40::new("se_dolly_special_sb03_command"));
    };
}

#[fighter_frame( agent = FIGHTER_KIND_DOLLY )]
pub fn waluigi_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let frame = MotionModule::frame(fighter.module_accessor);
        let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let WALUIGI = color >= 120 && color <= 130; 
        if WALUIGI {
            if [hash40("special_lw_special")].contains(&motion_kind) {
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                    if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 != 0 {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
                    } else if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0 {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
                    } else if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 != 0 {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
                    } else if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0 {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
                    } else if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0 {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
                    } else if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
                    };
                };
            };
            if is_jc(module_accessor, fighter_kind, status_kind, frame) && check_jump(module_accessor) {
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                };
                if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                };
            }
            if status_kind == FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_LW_JUMP {
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                    KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.6, z: 0.0});
                }
            }
        };
    }
}

pub unsafe fn final_start_waluigi(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("final_start") {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            SlowModule::set_whole(fighter.module_accessor, 0, 1);
        };
        if status_kind == *FIGHTER_STATUS_KIND_DEAD {
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_shield"), false, false);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_bg_vortex2"), false, false);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_special_all_up"), false, false);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        waluigi_frame,
    );
}


