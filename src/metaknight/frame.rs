use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;
static mut X : [f32; 8] = [0.0; 8]; //Log speed for Shuttle Loop Glide
static mut Y : [f32; 8] = [0.0; 8]; //Log speed for Shuttle Loop Glide
//static mut X2 : [f32; 8] = [0.0; 8]; //Log speed for Standard Glide 
//static mut Y2 : [f32; 8] = [0.0; 8]; //Log speed for Standard Glide 
static mut X_MAX : f32 = 0.0; //Max Horizontal movespeed (is needed for both glide variations)
static mut Y_MAX : f32 = 1.94; //Max Vertical movespeed (is needed for both glide variations)
static mut ANGLE : [f32; 8] = [0.0; 8];
static ANGLE_MAX : f32 = 80.0; //Max Ascent Angle for Glide
static ANGLE_LOW_MAX : f32 = -80.0; //Max Descent Angle for Glide
static mut ANGLE2 : [f32; 8] = [0.0; 8];
static ANGLE_MAX2 : f32 = 80.0; 
static ANGLE_LOW_MAX2 : f32 = 80.0; 
static STICK_ANGLE_MUL : f32 = 9.0; //Controls how much Meta Knight's body rotates according to the control stick (higher value = higher sensitivity)
static mut HOLD_TIME : [f32; 8] = [0.0; 8];

#[fighter_frame( agent = FIGHTER_KIND_METAKNIGHT )]
fn metaknight_opff(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if ![*FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR{
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP){
                HOLD_TIME[ENTRY_ID] +=1.0;
            }
            if HOLD_TIME[ENTRY_ID] == 20.0{
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE.into(), true.into());
            }
        }
        else{
            HOLD_TIME[ENTRY_ID] = 0.0;
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH {
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -1.0, 0);
            }
        };
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
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            fighter.sub_air_check_fall_common();
            if MotionModule::frame(fighter.module_accessor) >= 1.0 && MotionModule::frame(fighter.module_accessor) < 2.0 { //SFX Stuff added to prevent them from looping along with the animation
                macros::PLAY_STATUS(fighter, Hash40::new("vc_metaknight_special_h01"));
                macros::PLAY_SE(fighter, Hash40::new("se_metaknight_special_h01"));
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
            if MotionModule::frame(fighter.module_accessor) >= 16.0 && MotionModule::frame(fighter.module_accessor) < 17.0 {
                macros::PLAY_STATUS(fighter, Hash40::new("se_metaknight_special_h03"));
            }
            if MotionModule::frame(fighter.module_accessor) > 25.0 {
                macros::SET_SPEED_EX(fighter, 2.0, -0.4, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            if MotionModule::frame(fighter.module_accessor) > 28.0 {
                static Y_ACCEL_ADD : f32 = 0.045;
                static X_ACCEL_ADD : f32 = 0.0;
                static X_DECEL : f32 = -0.0061; 
                let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
                let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
                let mut y_add;
                let mut x_add;
                x_add = (stick_x)*X_ACCEL_ADD;
                y_add = (stick_y)*Y_ACCEL_ADD + X_DECEL;
                if x_add > 2.0 && X[ENTRY_ID] > X_MAX {
                    x_add = 2.0;
                };
                if x_add < 2.0 && X[ENTRY_ID] < X_MAX+2.0 {
                    x_add = 2.0;
                };
                if y_add > 0.0 && Y[ENTRY_ID] > Y_MAX {
                    y_add = 0.0;
                };
                if y_add < 0.0 && Y[ENTRY_ID] < Y_MAX*0.0 {
                    y_add = 0.0;
                };
                println!("x{}, y{}", X[ENTRY_ID], Y[ENTRY_ID]);
                println!("x_add{}, y_add{}", x_add, y_add);
                X[ENTRY_ID] += x_add;
                Y[ENTRY_ID] += y_add;
                if stick_y >= 0.1 || stick_y <= -0.1 { //Used to prevent having a stick_y in the middle from changing flight angle
                    ANGLE[ENTRY_ID] += STICK_ANGLE_MUL*stick_y;
                    if ANGLE[ENTRY_ID] > ANGLE_MAX {
                        ANGLE[ENTRY_ID] = ANGLE_MAX;
                        ANGLE2[ENTRY_ID] = ANGLE_MAX2;
                    };
                    if ANGLE[ENTRY_ID] < ANGLE_LOW_MAX {
                        ANGLE[ENTRY_ID] = ANGLE_LOW_MAX;
                        ANGLE2[ENTRY_ID] = ANGLE_LOW_MAX2;
                    };
                };
                let y = ANGLE[ENTRY_ID] * Y_ACCEL_ADD;
                let x = ANGLE2[ENTRY_ID] * X_DECEL; //Some Horizontal Air mobility is sacrificed when ascending/descending
                macros::SET_SPEED_EX(fighter, X[ENTRY_ID] + x, -0.4 + Y[ENTRY_ID] + y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                let rotation = Vector3f{x: ANGLE[ENTRY_ID]*-1.0, y: 0.0 , z: 0.0 };
                let rotation2 = Vector3f{x: ANGLE[ENTRY_ID]*-0.31, y: ANGLE[ENTRY_ID]*0.18, z: ANGLE[ENTRY_ID]*-0.4 };
                let rotation3 = Vector3f{x: ANGLE[ENTRY_ID]*0.16, y: ANGLE[ENTRY_ID]*0.135, z: ANGLE[ENTRY_ID]*-0.24 };
                let rotation4 = Vector3f{x: ANGLE[ENTRY_ID]*-0.05, y: ANGLE[ENTRY_ID]*-0.042, z: ANGLE[ENTRY_ID]*-0.11 };
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("shoulderr"), &rotation2,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("footr"), &rotation3,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("footl"), &rotation4,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            } else {
                ANGLE[ENTRY_ID] = 0.0;
            };
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
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP {
            fighter.sub_air_check_fall_common();
            if MotionModule::frame(fighter.module_accessor) >= 2.0 && MotionModule::frame(fighter.module_accessor) < 3.0 {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_shuttleloop1"), Hash40::new("top"), 0, -5, 2.5, 4, 0, 0, 1, true);
                EffectModule::set_disable_render_offset_last(fighter.module_accessor);
                macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 20, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                macros::PLAY_STATUS(fighter, Hash40::new("vc_metaknight_special_h01"));
                macros::PLAY_SE(fighter, Hash40::new("se_metaknight_special_h02"));
            }
            if MotionModule::frame(fighter.module_accessor) >= 11.0 && MotionModule::frame(fighter.module_accessor) < 12.0 {
                macros::PLAY_STATUS(fighter, Hash40::new("se_metaknight_special_h03"));
            }
            if MotionModule::frame(fighter.module_accessor) > 25.0 {
                macros::SET_SPEED_EX(fighter, 2.0, -0.4, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }  
            if MotionModule::frame(fighter.module_accessor) > 28.0 {
                static Y_ACCEL_ADD : f32 = 0.045;
                static X_ACCEL_ADD : f32 = 0.0;
                static X_DECEL : f32 = -0.0061;
                let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
                let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
                let mut y_add;
                let mut x_add;
                x_add = (stick_x)*X_ACCEL_ADD;
                y_add = (stick_y)*Y_ACCEL_ADD + X_DECEL;
                if x_add > 2.0 && X[ENTRY_ID] > X_MAX {
                    x_add = 2.0;
                };
                if x_add < 2.0 && X[ENTRY_ID] < X_MAX+2.0 {
                    x_add = 2.0;
                };
                if y_add > 0.0 && Y[ENTRY_ID] > Y_MAX {
                    y_add = 0.0;
                };
                if y_add < 0.0 && Y[ENTRY_ID] < Y_MAX*0.0 {
                    y_add = 0.0;
                };
                println!("x{}, y{}", X[ENTRY_ID], Y[ENTRY_ID]);
                println!("x_add{}, y_add{}", x_add, y_add);
                X[ENTRY_ID] += x_add;
                Y[ENTRY_ID] += y_add;
                if stick_y >= 0.1 || stick_y <= -0.1 { //Used to prevent having a stick_y in the middle from changing flight angle
                    ANGLE[ENTRY_ID] += STICK_ANGLE_MUL*stick_y;
                    if ANGLE[ENTRY_ID] > ANGLE_MAX {
                        ANGLE[ENTRY_ID] = ANGLE_MAX;
                        ANGLE2[ENTRY_ID] = ANGLE_MAX2;
                    };
                    if ANGLE[ENTRY_ID] < ANGLE_LOW_MAX {
                        ANGLE[ENTRY_ID] = ANGLE_LOW_MAX;
                        ANGLE2[ENTRY_ID] = ANGLE_LOW_MAX2;
                    };
                };
                let y = ANGLE[ENTRY_ID] * Y_ACCEL_ADD;
                let x = ANGLE2[ENTRY_ID] * X_DECEL; //Some Horizontal Air mobility is sacrificed when ascending/descending
                macros::SET_SPEED_EX(fighter, X[ENTRY_ID] + x, -0.4 + Y[ENTRY_ID] + y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                let rotation = Vector3f{x: ANGLE[ENTRY_ID]*-1.0, y: 0.0 , z: 0.0 };
                let rotation2 = Vector3f{x: ANGLE[ENTRY_ID]*-0.31, y: ANGLE[ENTRY_ID]*0.18, z: ANGLE[ENTRY_ID]*-0.4 };
                let rotation3 = Vector3f{x: ANGLE[ENTRY_ID]*0.16, y: ANGLE[ENTRY_ID]*0.135, z: ANGLE[ENTRY_ID]*-0.24 };
                let rotation4 = Vector3f{x: ANGLE[ENTRY_ID]*-0.05, y: ANGLE[ENTRY_ID]*-0.042, z: ANGLE[ENTRY_ID]*-0.11 };
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("shoulderr"), &rotation2,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("footr"), &rotation3,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("footl"), &rotation4,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            } else {
                ANGLE[ENTRY_ID] = 0.0;
            };
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
        }
    }
}        

pub fn install() {
    smashline::install_agent_frames!(
        metaknight_opff
    );
}