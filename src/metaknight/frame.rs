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
static mut ANGLE : [f32; 8] = [0.0; 8];
static ANGLE_MAX : f32 = 85.0;
static ANGLE_LOW_MAX : f32 = -41.0;
static STICK_ANGLE_MUL : f32 = 12.0;

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
                if x_add < 2.5 && X[ENTRY_ID] < X_MAX*2.5 {
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
                if x_add < 2.5 && X[ENTRY_ID] < X_MAX*2.5 {
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