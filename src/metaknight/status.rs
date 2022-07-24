use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;
use crate::utils::*;
use smashline::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::sv_battle_object::module_accessor;
use smash::hash40;
use smash::phx::{Vector3f, Hash40};
use smash_script::*;

static mut X : [f32; 8] = [0.0; 8]; //Log speed for Shuttle Loop Glide
static mut Y : [f32; 8] = [0.0; 8]; //Log speed for Shuttle Loop Glide
static mut X_MAX : f32 = 0.0; //Max Horizontal movespeed (is needed for both glide variations)
static mut Y_MAX : f32 = 1.94; //Max Vertical movespeed (is needed for both glide variations)
static mut ANGLE : [f32; 8] = [0.0; 8];
static ANGLE_MAX : f32 = 80.0; //Max Ascent Angle for Glide
static ANGLE_LOW_MAX : f32 = -80.0; //Max Descent Angle for Glide
static mut ANGLE2 : [f32; 8] = [0.0; 8];
static ANGLE_MAX2 : f32 = 80.0; 
static ANGLE_LOW_MAX2 : f32 = 80.0; 
static STICK_ANGLE_MUL : f32 = 9.0;

#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_start(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_start"), 1.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_main as *const () as _))
}

unsafe extern "C" fn glide_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(&mut *fighter.module_accessor);
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_start") && MotionModule::is_end(fighter.module_accessor){
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_wing"), 1.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_air_check_fall_common();
    macros::SET_SPEED_EX(fighter, 2.0, -0.4, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    static Y_ACCEL_ADD : f32 = 0.0456;
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
    if x_add < 2.0 && X[ENTRY_ID] < X_MAX + 2.0 {
        x_add = 2.0;
    };
    if y_add > 0.0 && Y[ENTRY_ID] > Y_MAX {
        y_add = 0.0;
    };
    if y_add < 0.0 && Y[ENTRY_ID] < Y_MAX * 0.0 {
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
    let rotation3 = Vector3f{x: ANGLE[ENTRY_ID]*0.06, y: ANGLE[ENTRY_ID]*0.11, z: ANGLE[ENTRY_ID]*-0.24 };
    let rotation4 = Vector3f{x: ANGLE[ENTRY_ID]*-0.05, y: ANGLE[ENTRY_ID]*-0.042, z: ANGLE[ENTRY_ID]*-0.11 };
    ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("shoulderr"), &rotation2,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("footr"), &rotation3,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("footl"), &rotation4,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});  
    0.into()
}

#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn glide_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    ANGLE[ENTRY_ID] = 0.0;
    L2CValue::I32(0)
}

pub fn install() {
    smashline::install_status_scripts!(glide_start, glide_end);
}