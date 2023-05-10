use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;
use crate::utils::*;
use smashline::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
//use smash::app::sv_battle_object::module_accessor;
use smash::hash40;
use smash::phx::{Vector3f, Hash40};
use smash_script::*;

static mut ANGLE : [f32; 8] = [0.0; 8];
static GLIDE_START_H_SPEED : f32 = 0.4; //H speed multiplier on GlideStart
static ANGLE_MAX_UP : f32 = 60.0; //Max Upward Angle for Glide (degrees)
static ANGLE_MAX_DOWN : f32 = -60.0; //Max Downward Angle for Glide (degrees)
static mut MOMENTUM : [f32; 8] = [0.0; 8];
static ANGLE_MAX_SPEED : f32 = -20.0;
static BASE_H_SPEED : f32 = 1.7; //Base H Speed forward on glide
static GRAVITY : f32 = -0.55; //Default descent speed when unangled
static DOWN_H_SPEED : f32 = 2.35; //Sorta Emulates how Brawl handled gaining additional forward speed
static Y_ACCEL_ADD : f32 = 0.0434; //Vertical Speed Multiplier
static Y_ACCEL_UP : f32 = -0.01;
static X_DECEL_MUL_UP : f32 = -0.01833;
static X_ACCEL_MUL_DOWN : f32 = -0.025;
static X_DECEL_MUL_DOWN : f32 = 0.027583;
static ANGLE_SPEED_MAX : f32 = 5.0; //Max Angular Speed
static mut UP_ANGLE_ACCEL_MIN : [f32; 8] = [0.55; 8]; //Min Upward angular acceleration 
static UP_ANGLE_ACCEL_MAX : f32 = 0.75; //Max Upward angular acceleration
static mut DOWN_ANGLE_ACCEL_MIN : [f32; 8] = [0.45; 8]; //Min Downward angular acceleration 
static DOWN_ANGLE_ACCEL_MAX : f32 = 1.0; //Max Downward angular acceleration

#[status_script(agent = "ridley", status = FIGHTER_STATUS_KIND_GLIDE_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_main_ridley_glidestart(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_all(fighter.module_accessor);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GLIDE_START);
    if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_GLIDE_START {
        KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: GLIDE_START_H_SPEED, y:0.0, z:0.0}, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_start"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_main_ridley_glidestart_sub as *const () as _))
}

unsafe extern "C" fn status_main_ridley_glidestart_sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_start") && MotionModule::is_end(fighter.module_accessor) {   
        fighter.change_status(FIGHTER_STATUS_KIND_GLIDE.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "ridley", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_main_ridley_glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_direction"), 90.0, 0.0, true, 0.0, false, false);
    MotionModule::add_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, Hash40::new("glide_wing"), 0.0, 1.0, true, false, 0.0, false, true, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_main_ridley_glide_sub as *const () as _))
}

unsafe extern "C" fn status_main_ridley_glide_sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    fighter.sub_air_check_fall_common();
    if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR || KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_GLIDE_START {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    };
    macros::SET_SPEED_EX(fighter, BASE_H_SPEED, GRAVITY, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); //Base horizontal air mobility and normal descent speed.
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);  
    let y = ANGLE[ENTRY_ID] * Y_ACCEL_ADD; //Applies the ascent/descent speed multiplier when angling the glide
    let x = MOMENTUM[ENTRY_ID] * X_ACCEL_MUL_DOWN;
    if stick_y > 0.0 { //Used to prevent having a stick_y in the middle from changing flight angle
        UP_ANGLE_ACCEL_MIN[ENTRY_ID] += (ANGLE_SPEED_MAX) * stick_y;
        ANGLE[ENTRY_ID] += (ANGLE_SPEED_MAX*UP_ANGLE_ACCEL_MAX) * stick_y;
        if ANGLE[ENTRY_ID] > ANGLE_MAX_UP { //Caps the max upward value at 60 and prevents it from going beyond. 
            ANGLE[ENTRY_ID] = ANGLE_MAX_UP;
            MOMENTUM[ENTRY_ID] = ANGLE_MAX_SPEED;
            UP_ANGLE_ACCEL_MIN[ENTRY_ID] = UP_ANGLE_ACCEL_MAX;
        };
    };
    if stick_y < 0.0 {
        DOWN_ANGLE_ACCEL_MIN[ENTRY_ID] += (ANGLE_SPEED_MAX) * stick_y;
        ANGLE[ENTRY_ID] += (ANGLE_SPEED_MAX*DOWN_ANGLE_ACCEL_MAX) * stick_y;
        if ANGLE[ENTRY_ID] < ANGLE_MAX_DOWN {
            ANGLE[ENTRY_ID] = ANGLE_MAX_DOWN; //Caps the max downward value at -60 and prevents it from going beyond. 
            MOMENTUM[ENTRY_ID] = ANGLE_MAX_SPEED;
            DOWN_ANGLE_ACCEL_MIN[ENTRY_ID] = DOWN_ANGLE_ACCEL_MAX;
        };
    };
    if ANGLE[ENTRY_ID] <= ANGLE_MAX_UP && ANGLE[ENTRY_ID] >= ANGLE_MAX_DOWN {
        MotionModule::set_frame(fighter.module_accessor, 90.0 - ANGLE[ENTRY_ID], false);
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_ridley_glide_loop"), 0.8 + ANGLE[ENTRY_ID] * -0.005);
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_ridley_jump02"), 1.0 + ANGLE[ENTRY_ID] * 0.003);
    }
    //Forward Speed Stuff
    if ANGLE[ENTRY_ID] >= ANGLE_MAX_DOWN && ANGLE[ENTRY_ID] < ANGLE_MAX_SPEED { //Applies the H Air decel. multilplier when descending when angle is between -60 and -20
        macros::SET_SPEED_EX(fighter, DOWN_H_SPEED + x, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE[ENTRY_ID] * X_DECEL_MUL_DOWN, y:0.0, z:0.0});
        MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, 1.0 + ANGLE[ENTRY_ID] * 0.005);
    };
    if ANGLE[ENTRY_ID] >= ANGLE_MAX_SPEED && ANGLE[ENTRY_ID] < 0.0 { //Applies the H Air accel. multilplier when descending when angle is between -20 and 0
        macros::SET_SPEED_EX(fighter, BASE_H_SPEED + x, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, 1.0 + ANGLE[ENTRY_ID] * 0.005);
    };
    if ANGLE[ENTRY_ID] <= ANGLE_MAX_UP && ANGLE[ENTRY_ID] > 0.0 { //Applies the H Air decel. multilplier when descending when angle is between 0 and 60
        macros::SET_SPEED_EX(fighter, BASE_H_SPEED + x, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE[ENTRY_ID] * X_DECEL_MUL_UP, y:ANGLE[ENTRY_ID] * Y_ACCEL_UP, z:0.0});
        MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, 1.0 + ANGLE[ENTRY_ID] * 0.01);
    };
    if speed_x * PostureModule::lr(fighter.module_accessor) < 1.1 {
        fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_END.into(), true.into());
    }
    //Cancel Stuff
    WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_ATTACK.into(), true.into());
    }
    if is_grounded(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_LANDING.into(), true.into());
    }
    0.into()
}

#[status_script(agent = "ridley", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_end_ridley_glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    ANGLE[ENTRY_ID] = 0.0;
    MOMENTUM[ENTRY_ID] = 0.0;
    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, false);
    L2CValue::I32(0)
}

#[status_script(agent = "ridley", status = FIGHTER_STATUS_KIND_GLIDE_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_main_ridley_glideattack(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_attack"), -1.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_main_ridley_glideattack_sub as *const () as _))
}

unsafe extern "C" fn status_main_ridley_glideattack_sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_fall_common();
    WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_JUMP_FLY_NEXT);
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "ridley", status = FIGHTER_STATUS_KIND_GLIDE_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_main_ridley_glideend(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_end"), -1.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_main_ridley_glideend_sub as *const () as _))
}

unsafe extern "C" fn status_main_ridley_glideend_sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_fall_common();
    WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_end") && MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "ridley", status = FIGHTER_STATUS_KIND_GLIDE_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_main_ridley_glidelanding(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_landing"), -1.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_main_ridley_glidelanding_sub as *const () as _))
}

unsafe extern "C" fn status_main_ridley_glidelanding_sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_landing") && MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN_WAIT.into(), false.into());
    }
    L2CValue::I32(0)
}

pub fn install() {
    smashline::install_status_scripts!(
        status_main_ridley_glidestart, 
        status_main_ridley_glide,
        status_end_ridley_glide,
        status_main_ridley_glideattack,
        status_main_ridley_glideend,
        status_main_ridley_glidelanding
    );
}