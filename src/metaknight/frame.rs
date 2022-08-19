use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use crate::utils::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;

static mut ANGLE : [f32; 8] = [0.0; 8];
static ANGLE_MAX : f32 = 80.0; //Max Ascent Angle for Glide (degrees)
static ANGLE_LOW_MAX : f32 = -80.0; //Max Descent Angle for Glide (degrees)
static STICK_ANGLE_MUL : f32 = 7.0; //Controls how much Meta Knight's body rotates according to the control stick (higher value = higher sensitivity)
static mut HOLD_TIME : [f32; 8] = [0.0; 8]; //Allows Meta Knight to enter the glide state when holding the jump button
static mut COUNTER: [i32; 8] = [0; 8];
static mut CURRENTFRAME: [f32; 8] = [0.0; 8];
static mut IS_CRIT: [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_METAKNIGHT )]
fn metaknight_opff(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        //let situation_kind = StatusModule::situation_kind(boma);
        let kind = smash::app::utility::get_kind(boma);
        if kind == *FIGHTER_KIND_METAKNIGHT {
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{x:1.14, y:1.14, z:1.14});
        };
        if ![*FIGHTER_STATUS_KIND_GLIDE_START, *FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP){
                HOLD_TIME[ENTRY_ID] +=1.0;
            }
            if HOLD_TIME[ENTRY_ID] == 20.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_START.into(), true.into());
            }
        }
        else {
            HOLD_TIME[ENTRY_ID] = 0.0;
        }
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
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_START {
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        }
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_ATTACK {
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        }
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_END {
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        };
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_END {
            if MotionModule::frame(fighter.module_accessor) > 29.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                if is_grounded(fighter.module_accessor) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                }
            }
        };
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH {
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -1.0, 0);
            }
        };
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            fighter.sub_air_check_fall_common();
            if MotionModule::frame(fighter.module_accessor) >= 1.0 && MotionModule::frame(fighter.module_accessor) < 2.0 { //SFX Stuff added to prevent them from looping along with the animation
                macros::PLAY_STATUS(fighter, Hash40::new("vc_metaknight_special_h01"));
                macros::PLAY_SE(fighter, Hash40::new("se_metaknight_smash_h03"));
                macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
            }
            if MotionModule::frame(fighter.module_accessor) >= 5.0 && MotionModule::frame(fighter.module_accessor) < 6.0 {
                macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
            if MotionModule::frame(fighter.module_accessor) >= 6.0 && MotionModule::frame(fighter.module_accessor) < 7.0 {
                macros::PLAY_STATUS(fighter, Hash40::new("se_metaknight_special_h02"));
            }
            if MotionModule::frame(fighter.module_accessor) >= 8.0 && MotionModule::frame(fighter.module_accessor) < 9.0 {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_shuttleloop1"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.2, true);
            }
            if MotionModule::frame(fighter.module_accessor) > 25.0 {
                macros::SET_SPEED_EX(fighter, 1.8, -0.42, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            if MotionModule::frame(fighter.module_accessor) > 28.0 {
                static Y_ACCEL_ADD : f32 = 0.0425; //Ascent/Descent Speed Multiplier
                static X_ACCEL_MUL_UP : f32 = 0.03334; //Horizontal Air Acceleration multiplier when ascending in between lower angle values
                static X_DECEL_MUL_UP_PRE : f32 = -0.0473;
                static X_DECEL_MUL_UP : f32 = -0.013125; //Horizontal Air Deceleration multiplier when ascending in between higher angle values
                static X_ACCEL_MUL_DOWN : f32 = -0.02; //Horizontal Air Acceleration multiplier when descending in between lower angle values
                static X_DECEL_MUL_DOWN_PRE : f32 = 0.0676; 
                static X_DECEL_MUL_DOWN : f32 = 0.0127; //Horizontal Air Deceleration multiplier when descending in between higher angle values
                let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
                if stick_y >= 0.1 || stick_y <= -0.1 { //Used to prevent having a stick_y in the middle from changing flight angle
                    ANGLE[ENTRY_ID] += STICK_ANGLE_MUL*stick_y;
                    if ANGLE[ENTRY_ID] > ANGLE_MAX {
                        ANGLE[ENTRY_ID] = ANGLE_MAX; //Caps the max upward value at 80 and prevents it from going beyond. 
                    };
                    if ANGLE[ENTRY_ID] < ANGLE_LOW_MAX {
                        ANGLE[ENTRY_ID] = ANGLE_LOW_MAX; //Caps the max downward value at -80 and prevents it from going beyond. 
                    };
                };
                let y = ANGLE[ENTRY_ID] * Y_ACCEL_ADD; //Applies the ascent/descent speed multiplier when angling the glide
                macros::SET_SPEED_EX(fighter, 1.8, -0.42 + y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if ANGLE[ENTRY_ID] >= -80.0 && ANGLE[ENTRY_ID] <= -40.1 { //Applies the H Air decel. multilplier when descending when angle is between -80 and 40.1
                    macros::SET_SPEED_EX(fighter, 1.8, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE[ENTRY_ID] * X_DECEL_MUL_DOWN, y:0.0, z:0.0});
                };
                if ANGLE[ENTRY_ID] >= -40.0 && ANGLE[ENTRY_ID] <= -25.1 {
                    macros::SET_SPEED_EX(fighter, 4.0, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE[ENTRY_ID] * X_DECEL_MUL_DOWN_PRE, y:0.0, z:0.0});
                };
                if ANGLE[ENTRY_ID] >= -25.0 && ANGLE[ENTRY_ID] <= -0.1 { //Applies the H Air accel. multilplier when descending when angle is between -25 and 0.1
                    macros::SET_SPEED_EX(fighter, 1.8, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE[ENTRY_ID] * X_ACCEL_MUL_DOWN, y:0.0, z:0.0});
                };
                if ANGLE[ENTRY_ID] <= 80.0 && ANGLE[ENTRY_ID] >= 35.1 { //Applies the H Air decel. multilplier when ascending when angle is between 35.1 and 80
                    macros::SET_SPEED_EX(fighter, 1.8, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE [ENTRY_ID] * X_DECEL_MUL_UP, y:0.0, z:0.0});
                };
                if ANGLE[ENTRY_ID] <= 35.0 && ANGLE[ENTRY_ID] >= 15.1 { //Applies the H Air decel. multilplier when ascending when angle is between 15.1 and 35
                    macros::SET_SPEED_EX(fighter, 3.0, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE [ENTRY_ID] * X_DECEL_MUL_UP_PRE, y:0.0, z:0.0});
                };
                if ANGLE[ENTRY_ID] <= 15.0 && ANGLE[ENTRY_ID] >= 0.1 { //Applies the H Air accel. multilplier when ascending when angle is between 0.1 and 15
                    macros::SET_SPEED_EX(fighter, 1.8, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE [ENTRY_ID] * X_ACCEL_MUL_UP, y:0.0, z:0.0});
                };
                let rotation = Vector3f{x: ANGLE[ENTRY_ID]*-1.0, y: 0.0 , z: 0.0 }; //Controls body rotation & model/bone movement when angling the glide
                let rotation2 = Vector3f{x: ANGLE[ENTRY_ID]*-0.31, y: ANGLE[ENTRY_ID]*0.18, z: ANGLE[ENTRY_ID]*-0.4 };
                let rotation3 = Vector3f{x: ANGLE[ENTRY_ID]*0.06, y: ANGLE[ENTRY_ID]*0.11, z: ANGLE[ENTRY_ID]*-0.24 };
                let rotation4 = Vector3f{x: ANGLE[ENTRY_ID]*-0.05, y: ANGLE[ENTRY_ID]*-0.042, z: ANGLE[ENTRY_ID]*-0.11 };
                let rotation5 = Vector3f{x: ANGLE[ENTRY_ID]*0.035, y: ANGLE[ENTRY_ID]*-0.006, z: ANGLE[ENTRY_ID]*-0.04 };
                let rotation6 = Vector3f{x: ANGLE[ENTRY_ID]*0.0, y: ANGLE[ENTRY_ID]*-0.27, z: ANGLE[ENTRY_ID]*0.0 };
                let rotation7 = Vector3f{x: ANGLE[ENTRY_ID]*-0.26, y: ANGLE[ENTRY_ID]*0.08, z: ANGLE[ENTRY_ID]*0.1 };
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("shoulderr"), &rotation2,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("footr"), &rotation3,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("footl"), &rotation4,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});  
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("handr"), &rotation5,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("haver"), &rotation6,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("shoulderl"), &rotation7,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                //Cancel Stuff
                WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
                WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
                WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_ATTACK.into(), true.into());
                }
            } else {
                ANGLE[ENTRY_ID] = 0.0;
                macros::SET_SPEED_EX(fighter, 1.8, -0.42, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            };
            if MotionModule::frame(fighter.module_accessor) >= 31.0 && MotionModule::frame(fighter.module_accessor) < 32.0 {
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
                macros::PLAY_SE(fighter, Hash40::new("se_metaknight_special_h01"));
            }
            if MotionModule::frame(fighter.module_accessor) > 55.0 {
                if is_grounded(fighter.module_accessor) {
                    fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_LANDING.into(), true.into());
                }
            }
            if MotionModule::frame(fighter.module_accessor) >= 469.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 307.0, 1.0, false, 0.0, false, false);
            };
            if [*FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
            if [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
            if [*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
            if [*FIGHTER_STATUS_KIND_JUMP].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
            if [*FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP {
            fighter.sub_air_check_fall_common();
            if MotionModule::frame(fighter.module_accessor) >= 1.0 && MotionModule::frame(fighter.module_accessor) < 2.0 {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
                macros::PLAY_SE(fighter, Hash40::new("se_metaknight_smash_h03"));
            }
            if MotionModule::frame(fighter.module_accessor) >= 2.0 && MotionModule::frame(fighter.module_accessor) < 3.0 {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_shuttleloop1"), Hash40::new("top"), 0, -5, 2.5, 4, 0, 0, 1, true);
                EffectModule::set_disable_render_offset_last(fighter.module_accessor);
                macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 20, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                macros::PLAY_STATUS(fighter, Hash40::new("vc_metaknight_special_h01"));
                macros::PLAY_SE(fighter, Hash40::new("se_metaknight_special_h02"));
            }
            if MotionModule::frame(fighter.module_accessor) > 25.0 {
                macros::SET_SPEED_EX(fighter, 1.8, -0.42, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }  
            if MotionModule::frame(fighter.module_accessor) > 28.0 {
                static Y_ACCEL_ADD : f32 = 0.0425; //Ascent/Descent Speed Multiplier
                static X_ACCEL_MUL_UP : f32 = 0.03334; //Horizontal Air Acceleration multiplier when ascending in between lower angle values
                static X_DECEL_MUL_UP_PRE : f32 = -0.0473;
                static X_DECEL_MUL_UP : f32 = -0.013125; //Horizontal Air Deceleration multiplier when ascending in between higher angle values
                static X_ACCEL_MUL_DOWN : f32 = -0.02; //Horizontal Air Acceleration multiplier when descending in between lower angle values
                static X_DECEL_MUL_DOWN_PRE : f32 = 0.0676; 
                static X_DECEL_MUL_DOWN : f32 = 0.0127; //Horizontal Air Deceleration multiplier when descending in between higher angle values
                let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
                if stick_y >= 0.1 || stick_y <= -0.1 { //Used to prevent having a stick_y in the middle from changing flight angle
                    ANGLE[ENTRY_ID] += STICK_ANGLE_MUL*stick_y;
                    if ANGLE[ENTRY_ID] > ANGLE_MAX {
                        ANGLE[ENTRY_ID] = ANGLE_MAX; //Caps the max upward value at 80 and prevents it from going beyond. 
                    };
                    if ANGLE[ENTRY_ID] < ANGLE_LOW_MAX {
                        ANGLE[ENTRY_ID] = ANGLE_LOW_MAX; //Caps the max downward value at -80 and prevents it from going beyond. 
                    };
                };
                let y = ANGLE[ENTRY_ID] * Y_ACCEL_ADD; //Applies the ascent/descent speed multiplier when angling the glide
                macros::SET_SPEED_EX(fighter, 1.8, -0.42 + y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if ANGLE[ENTRY_ID] >= -80.0 && ANGLE[ENTRY_ID] <= -40.1 { //Applies the H Air decel. multilplier when descending when angle is between -80 and 40.1
                    macros::SET_SPEED_EX(fighter, 1.8, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE[ENTRY_ID] * X_DECEL_MUL_DOWN, y:0.0, z:0.0});
                };
                if ANGLE[ENTRY_ID] >= -40.0 && ANGLE[ENTRY_ID] <= -25.1 {
                    macros::SET_SPEED_EX(fighter, 4.0, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE[ENTRY_ID] * X_DECEL_MUL_DOWN_PRE, y:0.0, z:0.0});
                };
                if ANGLE[ENTRY_ID] >= -25.0 && ANGLE[ENTRY_ID] <= -0.1 { //Applies the H Air accel. multilplier when descending when angle is between -25 and 0.1
                    macros::SET_SPEED_EX(fighter, 1.8, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE[ENTRY_ID] * X_ACCEL_MUL_DOWN, y:0.0, z:0.0});
                };
                if ANGLE[ENTRY_ID] <= 80.0 && ANGLE[ENTRY_ID] >= 35.1 { //Applies the H Air decel. multilplier when ascending when angle is between 35.1 and 80
                    macros::SET_SPEED_EX(fighter, 1.8, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE [ENTRY_ID] * X_DECEL_MUL_UP, y:0.0, z:0.0});
                };
                if ANGLE[ENTRY_ID] <= 35.0 && ANGLE[ENTRY_ID] >= 15.1 { //Applies the H Air decel. multilplier when ascending when angle is between 15.1 and 35
                    macros::SET_SPEED_EX(fighter, 3.0, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE [ENTRY_ID] * X_DECEL_MUL_UP_PRE, y:0.0, z:0.0});
                };
                if ANGLE[ENTRY_ID] <= 15.0 && ANGLE[ENTRY_ID] >= 0.1 { //Applies the H Air accel. multilplier when ascending when angle is between 0.1 and 15
                    macros::SET_SPEED_EX(fighter, 1.8, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE [ENTRY_ID] * X_ACCEL_MUL_UP, y:0.0, z:0.0});
                };
                let rotation = Vector3f{x: ANGLE[ENTRY_ID]*-1.0, y: 0.0 , z: 0.0 }; //Controls body rotation & model/bone movement when angling the glide
                let rotation2 = Vector3f{x: ANGLE[ENTRY_ID]*-0.31, y: ANGLE[ENTRY_ID]*0.18, z: ANGLE[ENTRY_ID]*-0.4 };
                let rotation3 = Vector3f{x: ANGLE[ENTRY_ID]*0.06, y: ANGLE[ENTRY_ID]*0.11, z: ANGLE[ENTRY_ID]*-0.24 };
                let rotation4 = Vector3f{x: ANGLE[ENTRY_ID]*-0.05, y: ANGLE[ENTRY_ID]*-0.042, z: ANGLE[ENTRY_ID]*-0.11 };
                let rotation5 = Vector3f{x: ANGLE[ENTRY_ID]*0.035, y: ANGLE[ENTRY_ID]*-0.006, z: ANGLE[ENTRY_ID]*-0.04 };
                let rotation6 = Vector3f{x: ANGLE[ENTRY_ID]*0.0, y: ANGLE[ENTRY_ID]*-0.27, z: ANGLE[ENTRY_ID]*0.0 };
                let rotation7 = Vector3f{x: ANGLE[ENTRY_ID]*-0.26, y: ANGLE[ENTRY_ID]*0.08, z: ANGLE[ENTRY_ID]*0.1 };
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("shoulderr"), &rotation2,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("footr"), &rotation3,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("footl"), &rotation4,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});  
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("handr"), &rotation5,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("haver"), &rotation6,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("shoulderl"), &rotation7,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                //Cancel Stuff
                WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
                WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
                WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_ATTACK.into(), true.into());
                }
            } else {
                ANGLE[ENTRY_ID] = 0.0;
                macros::SET_SPEED_EX(fighter, 1.8, -0.42, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            };
            if MotionModule::frame(fighter.module_accessor) >= 31.0 && MotionModule::frame(fighter.module_accessor) < 32.0 {
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
                macros::PLAY_SE(fighter, Hash40::new("se_metaknight_special_h01"));
            }
            if MotionModule::frame(fighter.module_accessor) > 55.0 {
                if is_grounded(fighter.module_accessor) {
                    fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_LANDING.into(), true.into());
                }
            }
            if MotionModule::frame(fighter.module_accessor) >= 469.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_loop"), 307.0, 1.0, false, 0.0, false, false);
            };
            if [*FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
            if [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
            if [*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
            if [*FIGHTER_STATUS_KIND_JUMP].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
            if [*FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if MotionModule::frame(fighter.module_accessor) > 10.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END.into(), true.into());
                }
                else if fighter.global_table[0x1F].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
                    fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK.into(), true.into());
                }
            }
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK {
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                COUNTER[ENTRY_ID] += 1;
                IS_CRIT[ENTRY_ID] = true;
                if COUNTER[ENTRY_ID] < 2 {
                    EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40::new("sys_bg_criticalhit"), smash::phx::Hash40::new("haver"), &Vector3f{x: 0.0, y: 8.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                    CURRENTFRAME[ENTRY_ID] = MotionModule::frame(fighter.module_accessor);
                    SlowModule::set_whole(fighter.module_accessor, 2, 0);
                    macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
                    macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
                    macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 4.0,/*no*/ 0.0,/*zoom*/ 2.1,/*yrot*/ 0.0,/*xrot*/ 0.0 * PostureModule::lr(boma));
                }
                if MotionModule::frame(fighter.module_accessor) > 25.0 {
                    CancelModule::enable_cancel(fighter.module_accessor);
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                }
            }
            if MotionModule::frame(fighter.module_accessor) >= (CURRENTFRAME[ENTRY_ID] + 1.0) && IS_CRIT[ENTRY_ID] {
                COUNTER[ENTRY_ID] = 0;
                SlowModule::clear_whole(fighter.module_accessor);
                CameraModule::reset_all(fighter.module_accessor);
                EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
                HitModule::set_status_all(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                if StatusModule::status_kind(fighter.module_accessor) != 510 {
                    macros::CAM_ZOOM_OUT(fighter);
                }
            }
            if IS_CRIT[ENTRY_ID] && MotionModule::frame(fighter.module_accessor) < 2.0 {
                macros::CAM_ZOOM_OUT(fighter);
                IS_CRIT[ENTRY_ID] = false;
                EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
                HitModule::set_status_all(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                SlowModule::clear_whole(fighter.module_accessor);
            };
        }
    }
}        

pub fn install() {
    smashline::install_agent_frames!(
        metaknight_opff
    );
}