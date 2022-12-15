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
static mut FLOAT_MAX : i32 = 1200; //Frames this bitch can float (In frames, 300 = 5 seconds)
static mut X_MAX : f32 = 1.45; //Max Horizontal movespeed
// static mut X_ACCEL_ADD : f32 = 0.06; //Air Accel Add
static mut X_ACCEL_MUL : f32 = 0.12; //Air Accel Mul
static mut Y_MAX : f32 = 1.3; //Max Vertical movespeed
// static mut Y_ACCEL_ADD : f32 = 0.06;
// static mut Y_ACCEL_MUL : f32 = 0.06;

#[fighter_frame( agent = FIGHTER_KIND_MEWTWO )]
fn mewtwo_frame(fighter: &mut L2CFighterCommon) {
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
                *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END
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
            // if stick_x > 0.2 {
            // 	x_add = ((stick_x-0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
            // 	if speed_x > X_MAX || speed_x < -X_MAX{
            // 		x_add = 0.0;
            // 	};
            // };
            // if stick_x < -0.2 {
            // 	x_add = ((stick_x+0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
            // 	if speed_x > X_MAX || speed_x < -X_MAX{
            // 		x_add = 0.0;
            // 	};
            // };
            // if stick_y > 0.2 {
            // 	y_add = ((stick_y-0.2)*Y_ACCEL_MUL) + Y_ACCEL_ADD;
            // 	if speed_y > Y_MAX || speed_y < -Y_MAX{
            // 		y_add = 0.0;
            // 	};
            // };
            // if stick_y < -0.2 {
            // 	y_add = ((stick_y+0.2)*Y_ACCEL_MUL) + Y_ACCEL_ADD;
            // 	if speed_y > Y_MAX || speed_y < -Y_MAX{
            // 		y_add = 0.0;
            // 	};
            // };
            // if stick_x > -0.2 && stick_x < 0.2 && stick_y > -0.2 && stick_y < 0.2 {
            // 	if speed_y > 0.0 {
            // 		y_add = -Y_ACCEL_MUL - Y_ACCEL_ADD;
            // 	} else if speed_y < 0.0{
            // 		y_add = Y_ACCEL_MUL + Y_ACCEL_ADD;
            // 	};
            // 	let mut x_add = 0.0;
            // 	if speed_x > 0.0 {
            // 		x_add = -X_ACCEL_MUL - X_ACCEL_ADD;
            // 	} else if speed_x < 0.0{
            // 		x_add = X_ACCEL_MUL + X_ACCEL_ADD;
            // 	};
            // };
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
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        mewtwo_frame
    );
}