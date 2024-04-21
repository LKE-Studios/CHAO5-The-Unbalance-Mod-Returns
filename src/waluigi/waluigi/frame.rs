use crate::imports::BuildImports::*;

pub static mut RANDOM_NUMBER : [i32; 65544] = [0; 65544];
pub static mut RANDOM_NUMBER_SELECTED : [bool; 65544] = [false; 65544];
pub static mut ALREADY_BLOCK : [bool; 65544] = [false; 65544];
pub static mut STICK_DIRECTION : [f32; 8] = [0.0; 8];
pub static mut DICEBLOCK_FRAME : [i32; 65544] = [0; 65544];

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
        special_n_waluigi(fighter);
        special_s_waluigi(fighter);
        final_start_waluigi(fighter);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
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

pub unsafe extern "C" fn special_n_waluigi(fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if [hash40("special_n"),hash40("special_air_n")].contains(&motion_kind) && !RANDOM_NUMBER_SELECTED[ENTRY_ID] {
        if MotionModule::frame(fighter.module_accessor) <= 1.0 {
            DICEBLOCK_FRAME[ENTRY_ID] = 0;
            RANDOM_NUMBER[ENTRY_ID] = sv_math::rand(hash40("dolly"), 10);
        };
        if frame <= 2.0 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("dolly_Kart_Glider_VIS_O_OBJShape"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"), false);
        };
        if frame > 2.0 && frame < 18.0 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"), true);
            if RANDOM_NUMBER[ENTRY_ID] == 0  {
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), true);
                ALREADY_BLOCK[ENTRY_ID] = true;
            }
            else if RANDOM_NUMBER[ENTRY_ID] == 1 {
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), true);
                ALREADY_BLOCK[ENTRY_ID] = true;
            }
            else if RANDOM_NUMBER[ENTRY_ID] == 2  {
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), true);
                ALREADY_BLOCK[ENTRY_ID] = true;
            }
            else if RANDOM_NUMBER[ENTRY_ID] == 3  {
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), true);
                ALREADY_BLOCK[ENTRY_ID] = true;
            }
            else if RANDOM_NUMBER[ENTRY_ID] == 4  {
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), true);
                ALREADY_BLOCK[ENTRY_ID] = true;
            }
            else if RANDOM_NUMBER[ENTRY_ID] == 5  {
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), true);
                ALREADY_BLOCK[ENTRY_ID] = true;
            }
            else if RANDOM_NUMBER[ENTRY_ID] == 6 {
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), true);
                ALREADY_BLOCK[ENTRY_ID] = true;
            }
            else if RANDOM_NUMBER[ENTRY_ID] == 7  {
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), true);
                ALREADY_BLOCK[ENTRY_ID] = true;
            }
            else if RANDOM_NUMBER[ENTRY_ID] == 8  {
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), true);
                ALREADY_BLOCK[ENTRY_ID] = true;
            }
            else if RANDOM_NUMBER[ENTRY_ID] == 9  {
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), true);
                ALREADY_BLOCK[ENTRY_ID] = true;
            }
        };
        if frame >= 18.0 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("dolly_Kart_Glider_VIS_O_OBJShape"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10_trans"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"), false);
        }
    };
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

pub unsafe extern "C" fn frame_waluigi_diceblock_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(fighter.module_accessor);
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let WALUIGI = color >= 120 && color <= 130; 
    if [hash40("regular")].contains(&motion_kind) {
        let life = WorkModule::get_param_int(fighter.module_accessor, hash40("param_diceblock"), hash40("life"));
        if frame <= 1.0 {
            WorkModule::set_int(fighter.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        }
    }
    DICEBLOCK_FRAME[ENTRY_ID] += 1;
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"), true);
    if RANDOM_NUMBER[ENTRY_ID] == 0  {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if RANDOM_NUMBER[ENTRY_ID] == 1 {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if RANDOM_NUMBER[ENTRY_ID] == 2  {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if RANDOM_NUMBER[ENTRY_ID] == 3 {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if RANDOM_NUMBER[ENTRY_ID] == 4  {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if RANDOM_NUMBER[ENTRY_ID] == 5  {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if RANDOM_NUMBER[ENTRY_ID] == 6 {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if RANDOM_NUMBER[ENTRY_ID] == 7  {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if RANDOM_NUMBER[ENTRY_ID] == 8  {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if RANDOM_NUMBER[ENTRY_ID] == 9  {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), true);
    }
}

pub fn install() {
    Agent::new("dolly")
    .on_line(Main, frame_waluigi_Main)
    .install();

    Agent::new("dolly_diceblock")
    .on_line(Main, frame_waluigi_diceblock_Main)
    .install();
}


