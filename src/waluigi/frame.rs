use crate::imports::BuildImports::*;

pub static mut RANDOM_NUM: [i32; 8] = [0; 8];
pub static mut RANDOM_NUM_SELECTED: [bool; 8] = [false; 8];
pub static mut ALREADY_BLOCK: [bool; 8] = [false; 8];
pub static mut COPTER_DIR: [i32; 8] = [0; 8];
pub static mut STICK_DIRECTION : [f32; 8] = [0.0; 8];

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

pub unsafe fn special_s_waluigi(fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    if situation_kind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK || situation_kind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
    };
    if motion_kind == hash40("special_air_f_start") {
        GroundModule::pass_floor(fighter.module_accessor);
        if ray_check_pos(fighter.module_accessor, 0.0, -0.3, false) == 1 {
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
            KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        };
        if MotionModule::frame(fighter.module_accessor) >= 19.0 && MotionModule::frame(fighter.module_accessor) <= 63.0 {
            if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_f_end"), 0.0, 1.0, true, 0.0, false, false);
            };
        };
        if MotionModule::frame(fighter.module_accessor) >= 88.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
        };
        if GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) && MotionModule::frame(fighter.module_accessor) > 8.0 && MotionModule::frame(fighter.module_accessor) < 90.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP, true);
        };
    };
    if motion_kind == hash40("special_air_f_end") {
        GroundModule::pass_floor(fighter.module_accessor);
        if ray_check_pos(fighter.module_accessor, 0.0, -0.3, false) == 1 {
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
            KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_CLIFF_GROUND);
            SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        };
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
        if MotionModule::frame(fighter.module_accessor) >= 2.0 && MotionModule::frame(fighter.module_accessor) <= 19.0 {
            SET_SPEED_EX(fighter, 0.4, 1.62, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        };
        if MotionModule::frame(fighter.module_accessor) >= 20.0 && MotionModule::frame(fighter.module_accessor) <= 25.0 {
            SET_SPEED_EX(fighter, 0.2, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        };
        if MotionModule::frame(fighter.module_accessor) >= 33.0 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_f_end2"), 0.0, 1.0, true, 0.0, false, false);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
        };
    };
    if motion_kind == hash40("special_air_f_end2") {
        if stick_y <= -0.5 {
            GroundModule::pass_floor(fighter.module_accessor);
            if ray_check_pos(fighter.module_accessor, 0.0, -0.3, false) == 1 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
                SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            };
        } 
        else {
            GroundModule::clear_pass_floor(fighter.module_accessor);
            if ray_check_pos(fighter.module_accessor, 0.0, -0.3, true) == 1 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
                SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            };
        };
        if MotionModule::frame(fighter.module_accessor) >= 0.0 {
            KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            SET_SPEED_EX(fighter, 0.25, -3.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        };
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            MotionModule::set_frame(fighter.module_accessor, 176.0, true);
            SET_SPEED_EX(fighter, 0.0, 0.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
        };
    };
    if motion_kind == hash40("special_f_start") {
        if MotionModule::frame(fighter.module_accessor) >= 57.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
        };
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
    STICK_DIRECTION[ENTRY_ID] = ControlModule::get_stick_dir(fighter.module_accessor) * (180.0 / PI);
    if [hash40("special_lw_start")].contains(&motion_kind) { //Grounded Mischief Step
        SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if MotionModule::frame(fighter.module_accessor) >= 12.0 && MotionModule::frame(fighter.module_accessor) <= 22.0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) && situation_kind == *SITUATION_KIND_GROUND { //grounded shield (moonwalk)
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_shield"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            };
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && situation_kind == *SITUATION_KIND_GROUND { //grounded attack (destruction dance)
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack1"), 0.0, 1.0, false, 0.0, false, false);
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
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && situation_kind == *SITUATION_KIND_AIR { //Aerial Attack (Trick Kick)
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_air_special"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
            };
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && situation_kind == *SITUATION_KIND_AIR { //Aerial Jump (copter)
                if stick_x >= -0.2 && stick_x <= 0.2 && stick_y >= -0.2 && stick_y <= 0.2 {
                    STICK_DIRECTION[ENTRY_ID] = 361.0;
                } else if STICK_DIRECTION[ENTRY_ID] <= -67.5 {
                    STICK_DIRECTION[ENTRY_ID] *= -1.0;
                };
                if STICK_DIRECTION[ENTRY_ID] >= -67.5 && STICK_DIRECTION[ENTRY_ID] < -22.5 && stick_x < 0.0 {
                    COPTER_DIR[ENTRY_ID] = 1;
                }
                else if STICK_DIRECTION[ENTRY_ID] >= 67.5 && STICK_DIRECTION[ENTRY_ID] <= 90.0 && stick_y < 0.0 {
                    COPTER_DIR[ENTRY_ID] = 2;
                }
                else if STICK_DIRECTION[ENTRY_ID] >= -67.5 && STICK_DIRECTION[ENTRY_ID] < -22.5 && stick_x > 0.0 {
                    COPTER_DIR[ENTRY_ID] = 3;
                }
                else if STICK_DIRECTION[ENTRY_ID] >= -22.5 && STICK_DIRECTION[ENTRY_ID] <= 22.5 && stick_x < 0.0 {
                    COPTER_DIR[ENTRY_ID] = 4;
                }
                else if STICK_DIRECTION[ENTRY_ID] == 361.0 {
                    COPTER_DIR[ENTRY_ID] = 5;
                }
                else if STICK_DIRECTION[ENTRY_ID] >= -22.5 && STICK_DIRECTION[ENTRY_ID] <= 22.5 && stick_x > 0.0 {
                    COPTER_DIR[ENTRY_ID] = 6;
                }
                else if STICK_DIRECTION[ENTRY_ID] > 22.5 && STICK_DIRECTION[ENTRY_ID] <= 67.5 && stick_x < 0.0 {
                    COPTER_DIR[ENTRY_ID] = 7;
                }
                else if STICK_DIRECTION[ENTRY_ID] > 67.5 && STICK_DIRECTION[ENTRY_ID] <= 90.0 && stick_y > 0.0 {
                    COPTER_DIR[ENTRY_ID] = 8;
                }
                else  {
                    COPTER_DIR[ENTRY_ID] = 9;
                };
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
    if [hash40("special_lw_attack1")].contains(&motion_kind) {
        SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if MotionModule::frame(fighter.module_accessor) >= 28.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
        };
        if MotionModule::frame(fighter.module_accessor) >= 20.0 && MotionModule::frame(fighter.module_accessor) < 28.0 {
            if (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack2"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            };
            if (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack_special2"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            };
        };
    };
    if [hash40("special_lw_attack2")].contains(&motion_kind) {
        SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if MotionModule::frame(fighter.module_accessor) >= 48.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
        };
        if MotionModule::frame(fighter.module_accessor) >= 29.0 && MotionModule::frame(fighter.module_accessor) < 48.0 {
            if (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack3"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            };
            if (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack_special1"), 0.0, 1.0, false, 0.0, false, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            };
        };
    };
    if [hash40("special_lw_attack3")].contains(&motion_kind) {
        SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if MotionModule::frame(fighter.module_accessor) >= 42.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
        };
    };
    if [hash40("special_lw_attack_special1")].contains(&motion_kind) {
        SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if MotionModule::frame(fighter.module_accessor) >= 37.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
        };
    };
    if [hash40("special_lw_attack_special2")].contains(&motion_kind) {
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
        if COPTER_DIR[ENTRY_ID] == 1 {
            SET_SPEED_EX(fighter, -1.5, -1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if COPTER_DIR[ENTRY_ID] == 2 {
            SET_SPEED_EX(fighter, 0, -1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if COPTER_DIR[ENTRY_ID] == 3 {
            SET_SPEED_EX(fighter, 1.5, -1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if COPTER_DIR[ENTRY_ID] == 4 {
            SET_SPEED_EX(fighter, -1.5, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); 
        }
        else if COPTER_DIR[ENTRY_ID] == 5 {
            SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); //No direction
        }
        else if COPTER_DIR[ENTRY_ID] == 6 {
            SET_SPEED_EX(fighter, 1.5, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if COPTER_DIR[ENTRY_ID] == 7 {
            SET_SPEED_EX(fighter, -1.5, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if COPTER_DIR[ENTRY_ID] == 8 {
            SET_SPEED_EX(fighter, 0, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if COPTER_DIR[ENTRY_ID] == 9 {
            SET_SPEED_EX(fighter, 1.5, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        if MotionModule::frame(fighter.module_accessor) >= 26.0 {
            SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        };
        if MotionModule::frame(fighter.module_accessor) >= 28.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
        };
    };
}

pub unsafe fn slap_cancel_waluigi(fighter: &mut L2CFighterCommon) {
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

pub unsafe fn final_start_waluigi(fighter: &mut L2CFighterCommon) {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("final_start") {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            SlowModule::set_whole(module_accessor, 0, 1);
        };
        if status_kind == *FIGHTER_STATUS_KIND_DEAD {
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_shield"), false, false);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_bg_vortex2"), false, false);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_special_all_up"), false, false);
        }
    }
}
