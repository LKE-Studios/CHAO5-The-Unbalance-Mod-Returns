use crate::imports::BuildImports::*;

pub static mut RANDOM_NUMBER : [i32; 8] = [0; 8];
pub static mut RANDOM_NUMBER_SELECTED : [bool; 8] = [false; 8];
pub static mut ALREADY_BLOCK : [bool; 8] = [false; 8];
pub static mut STICK_DIRECTION : [f32; 8] = [0.0; 8];
pub static mut DICEBLOCK_FRAME : [i32; 8] = [0; 8];

pub unsafe extern "C" fn frame_waluigi_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(fighter.module_accessor);
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let WALUIGI = color >= 120 && color <= 130; 
    if WALUIGI {
        scale_waluigi(fighter);
        attack_dash_waluigi(fighter);
        special_s_waluigi(fighter);
        final_start_waluigi(fighter);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
        if is_jc(module_accessor, fighter_kind, status_kind, frame) && check_jump(module_accessor) {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
            };
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            };
        }
    };
}

pub unsafe extern "C" fn scale_waluigi(fighter: &mut L2CFighterCommon) {
    ModelModule::set_scale(fighter.module_accessor, 0.9);
    AttackModule::set_attack_scale(fighter.module_accessor, 0.9, true);
    GrabModule::set_size_mul(fighter.module_accessor, 0.9);
}

pub unsafe extern "C" fn attack_dash_waluigi(fighter: &mut L2CFighterCommon) {
    let frame = MotionModule::frame(fighter.module_accessor);
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_dash") {
        if frame > 30.0 && frame < 36.0 {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                MotionModule::set_frame(fighter.module_accessor, 44.0, true);
            }
        }
    }
}

pub unsafe extern "C" fn special_s_waluigi(fighter: &mut L2CFighterCommon) {
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

pub unsafe extern "C" fn final_start_waluigi(fighter: &mut L2CFighterCommon) {
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
    Agent::new("dolly")
    .on_line(Main, frame_waluigi_Main)
    .install();
}


