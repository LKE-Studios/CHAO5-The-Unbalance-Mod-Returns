use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;
static mut FLOAT : [i32; 8] = [0; 8]; //Logs Float Time
static mut START_FLOAT : [bool; 8] = [false; 8];
static mut CHECK_FLOAT : [i32; 8] = [0; 8];
static mut CHECK_FLOAT_MAX : i32 = 10; //Frames where jump needs to be held to start floating
static mut X : [f32; 8] = [0.0; 8]; //Logs speed
static mut Y : [f32; 8] = [0.0; 8]; //Logs speed
static mut FLOAT_MAX : i32 = 90000; //Frames this bitch can float (In frames, 300 = 5 seconds)
static mut X_MAX : f32 = 2.6; //Max Horizontal movespeed
// static mut X_ACCEL_ADD : f32 = 0.06; //Air Accel Add
static mut X_ACCEL_MUL : f32 = 0.12; //Air Accel Mul
static mut Y_MAX : f32 = 1.94; //Max Vertical movespeed
// static mut Y_ACCEL_ADD : f32 = 0.06;
// static mut Y_ACCEL_MUL : f32 = 0.06;

#[fighter_frame( agent = FIGHTER_KIND_METAKNIGHT )]
fn metaknight_opff(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        // let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        // let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if StatusModule::situation_kind(fighter.module_accessor) != SITUATION_KIND_AIR
        || !sv_information::is_ready_go()
        || [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status_kind) {
            FLOAT[ENTRY_ID] = 0;
            START_FLOAT[ENTRY_ID] = false;
            CHECK_FLOAT[ENTRY_ID] = 0;
        };
        if FLOAT[ENTRY_ID] == 1{
            if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_MOTION_AIR
            && [
                *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S,
                *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END, 
            ].contains(&status_kind) == false {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            };
        };
        if StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_AIR {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                CHECK_FLOAT[ENTRY_ID] += 1;
            } else {
                CHECK_FLOAT[ENTRY_ID] = 0;
            };
            if CHECK_FLOAT[ENTRY_ID] >= CHECK_FLOAT_MAX && FLOAT[ENTRY_ID] == 0 {
                START_FLOAT[ENTRY_ID] = true;
            };
        };
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind)
        && FLOAT[ENTRY_ID] > 1{
            FLOAT[ENTRY_ID] = 1;
        };
        if FLOAT[ENTRY_ID] > 1{
            FLOAT[ENTRY_ID] -= 1;
            if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            };
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
                FLOAT[ENTRY_ID] = 1;
            };
            if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                FLOAT[ENTRY_ID] = 1;
            };
            let mut y_add;
            let mut x_add;
            x_add = (stick_x)*X_ACCEL_MUL;
            y_add = (stick_y)*X_ACCEL_MUL;
            if x_add > 0.0 && X[ENTRY_ID] > X_MAX {
                x_add = 0.0;
            };
            if x_add < 0.0 && X[ENTRY_ID] < X_MAX*-1.0 {
                x_add = 0.0;
            };
            if y_add > 0.0 && Y[ENTRY_ID] > Y_MAX {
                y_add = 0.0;
            };
            if y_add < 0.0 && Y[ENTRY_ID] < Y_MAX*-1.0 {
                y_add = 0.0;
            };
            println!("x{}, y{}", X[ENTRY_ID], Y[ENTRY_ID]);
            println!("x_add{}, y_add{}", x_add, y_add);
            X[ENTRY_ID] += x_add;
            Y[ENTRY_ID] += y_add;
            macros::SET_SPEED_EX(fighter, X[ENTRY_ID], Y[ENTRY_ID], *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        } else {
            X[ENTRY_ID] = 0.0;
            Y[ENTRY_ID] = 0.0;
        };
        if START_FLOAT[ENTRY_ID] == true {
            FLOAT[ENTRY_ID] = FLOAT_MAX;
            START_FLOAT[ENTRY_ID] = false;
            if status_kind == *FIGHTER_STATUS_KIND_JUMP {
                StatusModule::change_status_request_from_script(
                    fighter.module_accessor,
                    *FIGHTER_STATUS_KIND_FALL,
                    true
                );
            };
            if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                StatusModule::change_status_request_from_script(
                    fighter.module_accessor,
                    *FIGHTER_STATUS_KIND_FALL_AERIAL,
                    true
                );
            };
            if [*FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && FLOAT[ENTRY_ID] > 1{
                FLOAT[ENTRY_ID] = 1;
            };
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
            if MotionModule::frame(fighter.module_accessor) > 25.0 {
                macros::SET_SPEED_EX(fighter, 2.6, -0.6, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            if MotionModule::frame(fighter.module_accessor) > 27.0 {
                static mut Y_ACCEL_ADD : f32 = 4.1;
                static mut X_ACCEL_ADD : f32 = 0.0;
                let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
                let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
                let mut y_add;
                let mut x_add;
                x_add = (stick_x)*X_ACCEL_ADD;
                y_add = (stick_y)*Y_ACCEL_ADD;
                if x_add > 2.6 && X[ENTRY_ID] > X_MAX {
                    x_add = 2.6;
                };
                if x_add < 2.6 && X[ENTRY_ID] < X_MAX*2.6 {
                    x_add = 2.6;
                };
                if y_add > -0.9 && Y[ENTRY_ID] > Y_MAX {
                    y_add = -0.9;
                };
                if y_add < -0.9 && Y[ENTRY_ID] < Y_MAX*-0.9 {
                    y_add = -0.9;
                };
                println!("x{}, y{}", X[ENTRY_ID], Y[ENTRY_ID]);
                println!("x_add{}, y_add{}", x_add, y_add);
                X[ENTRY_ID] += x_add;
                Y[ENTRY_ID] += y_add;
                macros::SET_SPEED_EX(fighter, X[ENTRY_ID], Y[ENTRY_ID], *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.02 {
                    let rotation = Vector3f{x: 2.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.06 {
                    let rotation = Vector3f{x: 5.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.1 {
                    let rotation = Vector3f{x: 7.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.14 {
                    let rotation = Vector3f{x: 10.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.18 {
                    let rotation = Vector3f{x: 12.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.22 {
                    let rotation = Vector3f{x: 15.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.26 {
                    let rotation = Vector3f{x: 17.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.3 {
                    let rotation = Vector3f{x: 20.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.34 {
                    let rotation = Vector3f{x: 22.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.38 {
                    let rotation = Vector3f{x: 25.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.42 {
                    let rotation = Vector3f{x: 27.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.46 {
                    let rotation = Vector3f{x: 30.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.5 {
                    let rotation = Vector3f{x: 32.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.55 {
                    let rotation = Vector3f{x: 33.75, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.6 {
                    let rotation = Vector3f{x: 35.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.65 {
                    let rotation = Vector3f{x: 36.25, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.7 {
                    let rotation = Vector3f{x: 37.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.75 {
                    let rotation = Vector3f{x: 38.75, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.8 {
                    let rotation = Vector3f{x: 40.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.85 {
                    let rotation = Vector3f{x: 41.25, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.9 {
                    let rotation = Vector3f{x: 42.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.95 {
                    let rotation = Vector3f{x: 43.75, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -1.0 {
                    let rotation = Vector3f{x: 45.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
            }
            if MotionModule::frame(fighter.module_accessor) > 28.0 {
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.025 {
                    let rotation = Vector3f{x: -2.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.05 {
                    let rotation = Vector3f{x: -5.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.075 {
                    let rotation = Vector3f{x: -7.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.1 {
                    let rotation = Vector3f{x: -10.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.125 {
                    let rotation = Vector3f{x: -12.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.15 {
                    let rotation = Vector3f{x: -15.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.175 {
                    let rotation = Vector3f{x: -17.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.2 {
                    let rotation = Vector3f{x: -20.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.225 {
                    let rotation = Vector3f{x: -22.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.25 {
                    let rotation = Vector3f{x: -25.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.275 {
                    let rotation = Vector3f{x: -27.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.3 {
                    let rotation = Vector3f{x: -30.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.325 {
                    let rotation = Vector3f{x: -32.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.35 {
                    let rotation = Vector3f{x: -35.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.375 {
                    let rotation = Vector3f{x: -37.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.4 {
                    let rotation = Vector3f{x: -40.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.425 {
                    let rotation = Vector3f{x: -42.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.45 {
                    let rotation = Vector3f{x: -45.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.475 {
                    let rotation = Vector3f{x: -47.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.5 {
                    let rotation = Vector3f{x: -50.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.52 {
                    let rotation = Vector3f{x: -52.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.54 {
                    let rotation = Vector3f{x: -55.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.57 {
                    let rotation = Vector3f{x: -57.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.6 {
                    let rotation = Vector3f{x: -60.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.63 {
                    let rotation = Vector3f{x: -62.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.67 {
                    let rotation = Vector3f{x: -65.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.72 {
                    let rotation = Vector3f{x: -67.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.77 {
                    let rotation = Vector3f{x: -70.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.82 {
                    let rotation = Vector3f{x: -72.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.88 {
                    let rotation = Vector3f{x: -75.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.94 {
                    let rotation = Vector3f{x: -77.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 1.0 {
                    let rotation = Vector3f{x: -80.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
            }
            if MotionModule::frame(fighter.module_accessor) > 749.0 {
                MotionModule::set_frame(fighter.module_accessor, 60.0, true);
            };
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP {
            fighter.sub_air_check_fall_common();
            if MotionModule::frame(fighter.module_accessor) > 25.0 {
                macros::SET_SPEED_EX(fighter, 2.6, -0.6, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            if MotionModule::frame(fighter.module_accessor) > 27.0 {
                static mut Y_ACCEL_ADD : f32 = 4.1;
                static mut X_ACCEL_ADD : f32 = 0.0;
                let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
                let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
                let mut y_add;
                let mut x_add;
                x_add = (stick_x)*X_ACCEL_ADD;
                y_add = (stick_y)*Y_ACCEL_ADD;
                if x_add > 2.6 && X[ENTRY_ID] > X_MAX {
                    x_add = 2.6;
                };
                if x_add < 2.6 && X[ENTRY_ID] < X_MAX*2.6 {
                    x_add = 2.6;
                };
                if y_add > -0.9 && Y[ENTRY_ID] > Y_MAX {
                    y_add = -0.9;
                };
                if y_add < -0.9 && Y[ENTRY_ID] < Y_MAX*-0.9 {
                    y_add = -0.9;
                };
                println!("x{}, y{}", X[ENTRY_ID], Y[ENTRY_ID]);
                println!("x_add{}, y_add{}", x_add, y_add);
                X[ENTRY_ID] += x_add;
                Y[ENTRY_ID] += y_add;
                macros::SET_SPEED_EX(fighter, X[ENTRY_ID], Y[ENTRY_ID], *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.02 {
                    let rotation = Vector3f{x: 2.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.06 {
                    let rotation = Vector3f{x: 5.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.1 {
                    let rotation = Vector3f{x: 7.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.14 {
                    let rotation = Vector3f{x: 10.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.18 {
                    let rotation = Vector3f{x: 12.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.22 {
                    let rotation = Vector3f{x: 15.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.26 {
                    let rotation = Vector3f{x: 17.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.3 {
                    let rotation = Vector3f{x: 20.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.34 {
                    let rotation = Vector3f{x: 22.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.38 {
                    let rotation = Vector3f{x: 25.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.42 {
                    let rotation = Vector3f{x: 27.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.46 {
                    let rotation = Vector3f{x: 30.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.5 {
                    let rotation = Vector3f{x: 32.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.55 {
                    let rotation = Vector3f{x: 33.75, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.6 {
                    let rotation = Vector3f{x: 35.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.65 {
                    let rotation = Vector3f{x: 36.25, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.7 {
                    let rotation = Vector3f{x: 37.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.75 {
                    let rotation = Vector3f{x: 38.75, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.8 {
                    let rotation = Vector3f{x: 40.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.85 {
                    let rotation = Vector3f{x: 41.25, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.9 {
                    let rotation = Vector3f{x: 42.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -0.95 {
                    let rotation = Vector3f{x: 43.75, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) <= -1.0 {
                    let rotation = Vector3f{x: 45.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
            }
            if MotionModule::frame(fighter.module_accessor) > 28.0 {
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.025 {
                    let rotation = Vector3f{x: -2.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.05 {
                    let rotation = Vector3f{x: -5.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.075 {
                    let rotation = Vector3f{x: -7.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.1 {
                    let rotation = Vector3f{x: -10.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.125 {
                    let rotation = Vector3f{x: -12.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.15 {
                    let rotation = Vector3f{x: -15.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.175 {
                    let rotation = Vector3f{x: -17.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.2 {
                    let rotation = Vector3f{x: -20.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.225 {
                    let rotation = Vector3f{x: -22.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.25 {
                    let rotation = Vector3f{x: -25.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.275 {
                    let rotation = Vector3f{x: -27.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.3 {
                    let rotation = Vector3f{x: -30.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.325 {
                    let rotation = Vector3f{x: -32.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.35 {
                    let rotation = Vector3f{x: -35.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.375 {
                    let rotation = Vector3f{x: -37.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.4 {
                    let rotation = Vector3f{x: -40.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.425 {
                    let rotation = Vector3f{x: -42.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.45 {
                    let rotation = Vector3f{x: -45.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.475 {
                    let rotation = Vector3f{x: -47.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.5 {
                    let rotation = Vector3f{x: -50.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.52 {
                    let rotation = Vector3f{x: -52.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.54 {
                    let rotation = Vector3f{x: -55.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.57 {
                    let rotation = Vector3f{x: -57.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.6 {
                    let rotation = Vector3f{x: -60.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.63 {
                    let rotation = Vector3f{x: -62.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.67 {
                    let rotation = Vector3f{x: -65.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.72 {
                    let rotation = Vector3f{x: -67.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.77 {
                    let rotation = Vector3f{x: -70.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.82 {
                    let rotation = Vector3f{x: -72.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.88 {
                    let rotation = Vector3f{x: -75.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 0.94 {
                    let rotation = Vector3f{x: -77.5, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
                if ControlModule::get_stick_y(fighter.module_accessor) >= 1.0 {
                    let rotation = Vector3f{x: -80.0, y: 0.0, z: 0.0}; ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                }
            }
            if MotionModule::frame(fighter.module_accessor) > 749.0 {
                MotionModule::set_frame(fighter.module_accessor, 60.0, true);
            };
        }
    }
}        

pub fn install() {
    smashline::install_agent_frames!(
        metaknight_opff
    );
}