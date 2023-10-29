use crate::imports::BuildImports::*;
use crate::dolly::frame::*;

pub static mut RANDOM_NUM : [i32; 8] = [0; 8];
pub static mut RANDOM_NUM_SELECTED : [bool; 8] = [false; 8];
pub static mut ALREADY_BLOCK : [bool; 8] = [false; 8];
pub static mut COPTER_DIR : [f32; 8] = [0.0; 8];
pub static mut STICK_DIRECTION : [f32; 8] = [0.0; 8];
pub static max_speed_x : f32 = 1.5;
pub static x_acl_air : f32 = 0.05;
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
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
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
    if motion_kind == hash40("landing_fall_special") {
        if MotionModule::frame(fighter.module_accessor) >= 42.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
        };
    };
}

pub unsafe fn special_lw_waluigi(fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    if [hash40("special_lw_start")].contains(&motion_kind) { //Grounded Mischief Step
        SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if MotionModule::frame(fighter.module_accessor) >= 12.0 && MotionModule::frame(fighter.module_accessor) <= 22.0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) && situation_kind == *SITUATION_KIND_GROUND { //grounded shield (moonwalk)
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_shield"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            };
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && situation_kind == *SITUATION_KIND_GROUND { //grounded attack (destruction dance)
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack_1"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            };
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && situation_kind == *SITUATION_KIND_GROUND { //grounded jump (cartwheel)
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_jump"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            };
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && situation_kind == *SITUATION_KIND_GROUND { //grounded special (slap)
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_special"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            };
        };
        if MotionModule::frame(fighter.module_accessor) > 22.0 { //no input pressed
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
        };
    };
    if [hash40("special_air_lw_start")].contains(&motion_kind) { //Aerial Mischief Step
        SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if MotionModule::frame(fighter.module_accessor) >= 12.0 && MotionModule::frame(fighter.module_accessor) <= 22.0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) && situation_kind == *SITUATION_KIND_AIR { //Aerial Shield (Moonwalk)
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_shield"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
            };
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && situation_kind == *SITUATION_KIND_AIR { //Aerial Attack (Trick Kick)
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_air_attack"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
            };
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && situation_kind == *SITUATION_KIND_AIR { //Aerial Jump (copter)
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_air_special"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
            };
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && situation_kind == *SITUATION_KIND_AIR { 
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_air_jump"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
            };
        };
        if MotionModule::frame(fighter.module_accessor) > 22.0 { //No Input pressed
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
        };
    };
    //Moonwalk
    if [hash40("special_lw_shield")].contains(&motion_kind) {
        SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if MotionModule::frame(fighter.module_accessor) >= 58.0 {
            if (situation_kind == *SITUATION_KIND_GROUND) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            };
            if (situation_kind == *SITUATION_KIND_AIR) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
            };
        };
    };
    //Destruction Dance
    if [hash40("special_lw_attack_1")].contains(&motion_kind) {
        SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if MotionModule::frame(fighter.module_accessor) >= 28.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
        };
        if MotionModule::frame(fighter.module_accessor) >= 20.0 && MotionModule::frame(fighter.module_accessor) < 28.0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack_2"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            };
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack_special_2"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            };
        };
    };
    if [hash40("special_lw_attack_2")].contains(&motion_kind) {
        SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if MotionModule::frame(fighter.module_accessor) >= 48.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
        };
        if MotionModule::frame(fighter.module_accessor) >= 29.0 && MotionModule::frame(fighter.module_accessor) < 48.0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack_3"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            };
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack_special_1"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            };
        };
    };
    if [hash40("special_lw_attack_3")].contains(&motion_kind) {
        SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if MotionModule::frame(fighter.module_accessor) >= 42.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
        };
    };
    if [hash40("special_lw_attack_special_1")].contains(&motion_kind) {
        SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if MotionModule::frame(fighter.module_accessor) >= 37.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
        };
    };
    if [hash40("special_lw_attack_special_2")].contains(&motion_kind) {
        SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if MotionModule::frame(fighter.module_accessor) >= 37.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
        };
    };
    //Cartwheel
    if [hash40("special_lw_jump")].contains(&motion_kind) {
        SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if MotionModule::frame(fighter.module_accessor) >= 31.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
        };
    };
    //Slap
    if [hash40("special_lw_special")].contains(&motion_kind) {
        SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if MotionModule::frame(fighter.module_accessor) >= 35.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
        };
    };
    //Trick Kick
    if [hash40("special_lw_air_attack")].contains(&motion_kind) {
        if MotionModule::frame(fighter.module_accessor) >= 1.0 {
            SET_SPEED_EX(fighter, 0, 0.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        };
        if MotionModule::frame(fighter.module_accessor) >= 15.0 {
            SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        };
        if MotionModule::frame(fighter.module_accessor) >= 34.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
        };
    };
    //Trick Spin
    if [hash40("special_lw_air_special")].contains(&motion_kind) {
        SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if MotionModule::frame(fighter.module_accessor) >= 85.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
        };
    };
    //Wah Copter
    if [hash40("special_lw_air_jump")].contains(&motion_kind) {
        let lr = PostureModule::lr(fighter.module_accessor); 
        let speed_x = x_acl_air * stick_x;
        WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0 + speed_x, y: 0.0, z: 0.0});
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if MotionModule::frame(fighter.module_accessor) >= 28.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_air_jump"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if MotionModule::frame(fighter.module_accessor) >= 28.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
        };
    };
}

#[fighter_frame_callback]
pub fn slap_cancel_waluigi(fighter: &mut L2CFighterCommon) {
    unsafe {
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
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

#[fighter_frame_callback]
pub fn jump_cancel_waluigi(fighter : &mut L2CFighterCommon) {
    unsafe {	   
		let status_kind = StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let frame = MotionModule::frame(fighter.module_accessor);
        let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        if fighter_kind == *FIGHTER_KIND_DOLLY && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120{
            if is_jc(module_accessor, fighter_kind, status_kind, frame) && check_jump(module_accessor){
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                };
                if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                };
            };
        };
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        jump_cancel_waluigi,
        slap_cancel_waluigi
    );
}


