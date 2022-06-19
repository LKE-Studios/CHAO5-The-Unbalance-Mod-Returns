use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;
static mut X : [f32; 8] = [0.0; 8]; //Log speed for Shuttle Loop Glide
static mut Y : [f32; 8] = [0.0; 8]; //Log speed for Shuttle Loop Glide
static mut X2 : [f32; 8] = [0.0; 8]; //Log speed for Standard Glide 
static mut Y2 : [f32; 8] = [0.0; 8]; //Log speed for Standard Glide 
static mut X_MAX : f32 = 0.0; //Max Horizontal movespeed (is needed for both glide variations)
static mut Y_MAX : f32 = 1.94; //Max Vertical movespeed (is needed for both glide variations)
static mut ANGLE : [f32; 8] = [0.0; 8];
static ANGLE_MAX : f32 = 85.0; //Max Ascent Angle for Glide
static ANGLE_LOW_MAX : f32 = -41.0; //Max Descent Angle for Glide
static STICK_ANGLE_MUL : f32 = 12.0; //Controls how much Meta Knight's body rotates according to the control stick (higher value = higher sensitivity)

#[fighter_frame( agent = FIGHTER_KIND_METAKNIGHT )]
fn metaknight_opff(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        /*let jump_button_on_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_JUMP_WORK_INT_BUTTON_ON_FRAME);
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
            println!("jump button on frame: {}", jump_button_on_frame);
            if jump_button_on_frame > 20 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_start"), -1.0, 1.0, false, 0.0, false, false);
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_wing"), 1.0, 1.0, false, 0.0, false, false);
                fighter.sub_air_check_fall_common();
                    static Y_ACCEL_ADD2 : f32 = 0.0712;
                    static mut X_ACCEL_ADD2 : f32 = 0.0;
                    let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
                    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
                    let mut y_add;
                    let mut x_add;
                    x_add = (stick_x)*X_ACCEL_ADD2;
                    y_add = (stick_y)*Y_ACCEL_ADD2;
                    if x_add > 2.4 && X2[ENTRY_ID] > X_MAX {
                        x_add = 2.4;
                    };
                    if x_add < 2.4 && X2[ENTRY_ID] < X_MAX+2.4 {
                        x_add = 2.4;
                    };
                    if y_add > 0.0 && Y2[ENTRY_ID] > Y_MAX {
                        y_add = 0.0;
                    };
                    if y_add < 0.0 && Y2[ENTRY_ID] < Y_MAX*0.0 {
                        y_add = 0.0;
                    };
                    println!("x{}, y{}", X2[ENTRY_ID], Y2[ENTRY_ID]);
                    println!("x_add{}, y_add{}", x_add, y_add);
                    X[ENTRY_ID] += x_add;
                    Y[ENTRY_ID] += y_add;
                    if stick_y >= 0.1 || stick_y <= -0.1 { //Used to prevent having a stick_y in the middle from changing flight angle
                        ANGLE[ENTRY_ID] += STICK_ANGLE_MUL*stick_y;
                        if ANGLE[ENTRY_ID] > ANGLE_MAX {
                            ANGLE[ENTRY_ID] = ANGLE_MAX;
                        };
                        if ANGLE[ENTRY_ID] < ANGLE_LOW_MAX {
                            ANGLE[ENTRY_ID] = ANGLE_LOW_MAX;
                        };
                    }
                    let y = ANGLE[ENTRY_ID] * Y_ACCEL_ADD2;
                    macros::SET_SPEED_EX(fighter, X2[ENTRY_ID], -0.44 + Y2[ENTRY_ID] + y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    let rotation = Vector3f{x: ANGLE[ENTRY_ID]*-1.0, y: 0.0 , z: 0.0 };
                    ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                } else {
                    ANGLE[ENTRY_ID] = 0.0;
                };
            }
        }*/
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
                macros::SET_SPEED_EX(fighter, 2.5, -0.44, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            if MotionModule::frame(fighter.module_accessor) > 27.0 {
                static Y_ACCEL_ADD : f32 = 0.072;
                static mut X_ACCEL_ADD : f32 = 0.0;
                let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
                let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
                let mut y_add;
                let mut x_add;
                x_add = (stick_x)*X_ACCEL_ADD;
                y_add = (stick_y)*Y_ACCEL_ADD;
                if x_add > 2.5 && X[ENTRY_ID] > X_MAX {
                    x_add = 2.5;
                };
                if x_add < 2.5 && X[ENTRY_ID] < X_MAX+2.5 {
                    x_add = 2.5;
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
                    };
                    if ANGLE[ENTRY_ID] < ANGLE_LOW_MAX {
                        ANGLE[ENTRY_ID] = ANGLE_LOW_MAX;
                    };
                }
                let y = ANGLE[ENTRY_ID] * Y_ACCEL_ADD;
                macros::SET_SPEED_EX(fighter, X[ENTRY_ID], -0.44 + Y[ENTRY_ID] + y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                let rotation = Vector3f{x: ANGLE[ENTRY_ID]*-1.0, y: 0.0 , z: 0.0 };
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            } else {
                ANGLE[ENTRY_ID] = 0.0;
            }
            if MotionModule::frame(fighter.module_accessor) >= 740.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 120.0, 1.0, false, 0.0, false, false);
            };
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP {
            fighter.sub_air_check_fall_common();
            if MotionModule::frame(fighter.module_accessor) > 25.0 {
                macros::SET_SPEED_EX(fighter, 2.5, -0.44, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            if MotionModule::frame(fighter.module_accessor) > 27.0 {
                static Y_ACCEL_ADD : f32 = 0.072;
                static mut X_ACCEL_ADD : f32 = 0.0;
                let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
                let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
                let mut y_add;
                let mut x_add;
                x_add = (stick_x)*X_ACCEL_ADD;
                y_add = (stick_y)*Y_ACCEL_ADD;
                if x_add > 2.5 && X[ENTRY_ID] > X_MAX {
                    x_add = 2.5;
                };
                if x_add < 2.5 && X[ENTRY_ID] < X_MAX+2.5 {
                    x_add = 2.5;
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
                    };
                    if ANGLE[ENTRY_ID] < ANGLE_LOW_MAX {
                        ANGLE[ENTRY_ID] = ANGLE_LOW_MAX;
                    };
                };
                let y = ANGLE[ENTRY_ID] * Y_ACCEL_ADD;
                macros::SET_SPEED_EX(fighter, X[ENTRY_ID], -0.44 + Y[ENTRY_ID] + y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                let rotation = Vector3f{x: ANGLE[ENTRY_ID]*-1.0, y: 0.0 , z: 0.0 };
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            } else {
                ANGLE[ENTRY_ID] = 0.0;
            }
            if MotionModule::frame(fighter.module_accessor) >= 740.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_loop"), 120.0, 1.0, false, 0.0, false, false);
            };
        }
    }
}        

pub fn install() {
    smashline::install_agent_frames!(
        metaknight_opff
    );
}