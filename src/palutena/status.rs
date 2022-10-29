use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;
use crate::utils::*;
use smashline::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
//use smash::app::sv_battle_object::module_accessor;
use smash::hash40;
use smash::phx::{Vector3f, Hash40};
use smash_script::*;

static mut ANGLE : [f32; 8] = [0.0; 8];
static ANGLE_MAX_UP : f32 = 75.0; //Max Upward Angle
static ANGLE_MAX_DOWN : f32 = -75.0; //Max Downward Angle
static mut MOMENTUM : [f32; 8] = [0.0; 8];
static THRESHOLD_MAX : f32 = -25.0;
static mut ANGLE_FRAME : [f32; 8] = [90.0; 8]; //Sets GlideDirection Animation to Frame 90 by default
static DIRECTION_UP : f32 = 15.0;
static DIRECTION_DOWN : f32 = 165.0;
static STICK_ANGLE_MUL : f32 = 6.5; //Max Angular Speed
static UP_ANGLE_ACCEL : f32 = 0.86; //Upward angular acceleration
static DOWN_ANGLE_ACCEL : f32 = 0.95; //Downward angular acceleration

#[status_script(agent = "palutena", status = FIGHTER_STATUS_KIND_GLIDE_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_start_a(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = get_entry_id(fighter.module_accessor);
    set_position_lock(entry_id as i32);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_start"), 0.0, 1.0, false, 0.0, false, false);
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_start"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_start_b as *const () as _))
}

unsafe extern "C" fn glide_start_b(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_start") && MotionModule::is_end(fighter.module_accessor) {   
        let entry_id = get_entry_id(fighter.module_accessor) as i32;
        unset_position_lock(entry_id);
        fighter.change_status(FIGHTER_STATUS_KIND_GLIDE.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "palutena", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_direction"), ANGLE_FRAME[ENTRY_ID], 0.0, true, 0.0, false, false);
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_wing"), false, -1.0);
    //MotionModule::add_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, Hash40::new("glide_wing"), 0.0, 1.0, true, false, 0.0, false, true, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_core as *const () as _))
}

unsafe extern "C" fn glide_core(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    fighter.sub_air_check_fall_common();
    macros::SET_SPEED_EX(fighter, 1.7, -0.48, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); //Base horizontal air mobility and normal descent speed.
    static Y_ACCEL_ADD : f32 = 0.0334; //Ascent/Descent Speed Multiplier
    static X_DECEL_MUL_UP : f32 = -0.02666; //Horizontal Air Deceleration multiplier when ascending in between higher angle values
    static X_ACCEL_MUL_DOWN : f32 = -0.032; //Horizontal Air Acceleration multiplier when descending in between lower angle values
    static X_DECEL_MUL_DOWN : f32 = 0.04; //Horizontal Air Deceleration multiplier when descending
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let y = ANGLE[ENTRY_ID] * Y_ACCEL_ADD; //Applies the ascent/descent speed multiplier when angling the glide
    let x = MOMENTUM[ENTRY_ID] * X_ACCEL_MUL_DOWN;
    let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if stick_y > 0.0 { //Used to prevent having a stick_y in the middle from changing flight angle
        ANGLE[ENTRY_ID] += (STICK_ANGLE_MUL*UP_ANGLE_ACCEL) * stick_y;
        ANGLE_FRAME[ENTRY_ID] -= (STICK_ANGLE_MUL*UP_ANGLE_ACCEL) * stick_y;
        if ANGLE[ENTRY_ID] > ANGLE_MAX_UP { //Caps the max upward value at 75 and prevents it from going beyond. 
            ANGLE[ENTRY_ID] = ANGLE_MAX_UP;
            MOMENTUM[ENTRY_ID] = THRESHOLD_MAX;
            ANGLE_FRAME[ENTRY_ID] = DIRECTION_UP;
        };
    };
    if stick_y < 0.0 {
        ANGLE[ENTRY_ID] += (STICK_ANGLE_MUL*DOWN_ANGLE_ACCEL) * stick_y;
        ANGLE_FRAME[ENTRY_ID] -= (STICK_ANGLE_MUL*DOWN_ANGLE_ACCEL) * stick_y;
        if ANGLE[ENTRY_ID] < ANGLE_MAX_DOWN {
            ANGLE[ENTRY_ID] = ANGLE_MAX_DOWN; //Caps the max downward value at -75 and prevents it from going beyond. 
            MOMENTUM[ENTRY_ID] = THRESHOLD_MAX;
            ANGLE_FRAME[ENTRY_ID] = DIRECTION_DOWN;
        };
    };
    if ANGLE[ENTRY_ID] <= 75.0 && ANGLE[ENTRY_ID] >= -75.0 {
        MotionModule::set_frame(fighter.module_accessor, ANGLE_FRAME[ENTRY_ID], false);
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_palutena_glide_loop"), 1.0 + ANGLE[ENTRY_ID] * -0.0043);
    }
    //Forward Speed Stuff
    if ANGLE[ENTRY_ID] >= -75.0 && ANGLE[ENTRY_ID] < -25.0 { //Applies the H Air decel. multilplier when descending when angle is between -70 and -25.0
        macros::SET_SPEED_EX(fighter, 2.7 + x, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE[ENTRY_ID] * X_DECEL_MUL_DOWN, y:0.0, z:0.0});
    };
    if ANGLE[ENTRY_ID] >= -25.0 && ANGLE[ENTRY_ID] < 0.0 { //Applies the H Air accel. multilplier when descending when angle is between -25 and 0
        macros::SET_SPEED_EX(fighter, 1.7 + x, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    };
    if ANGLE[ENTRY_ID] <= 75.0 && ANGLE[ENTRY_ID] > 0.0 { //Applies the H Air decel. multilplier when descending when angle is between 0 and 70
        macros::SET_SPEED_EX(fighter, 1.7 + x, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE [ENTRY_ID] * X_DECEL_MUL_UP, y:0.0, z:0.0});
    };
    if speed_x * PostureModule::lr(fighter.module_accessor) < 1.0 {
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

#[status_script(agent = "palutena", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn glide_finish(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    ANGLE[ENTRY_ID] = 0.0;
    MOMENTUM[ENTRY_ID] = 0.0;
    ANGLE_FRAME[ENTRY_ID] = 90.0;
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    //MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, false);
    L2CValue::I32(0)
}

#[status_script(agent = "palutena", status = FIGHTER_STATUS_KIND_GLIDE_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_attack_a(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_attack"), -1.0, 1.0, false, 0.0, false, false);
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_attack"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_attack_b as *const () as _))
}

unsafe extern "C" fn glide_attack_b(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_fall_common();
    WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_JUMP_FLY_NEXT);
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_attack") && MotionModule::is_end(fighter.module_accessor) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        if is_grounded(fighter.module_accessor) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "palutena", status = FIGHTER_STATUS_KIND_GLIDE_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_end_a(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_end"), -1.0, 1.0, false, 0.0, false, false);
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_end"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_end_b as *const () as _))
}

unsafe extern "C" fn glide_end_b(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_fall_common();
    WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_end") && MotionModule::is_end(fighter.module_accessor) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        if is_grounded(fighter.module_accessor) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "palutena", status = FIGHTER_STATUS_KIND_GLIDE_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_landing_a(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_landing"), -1.0, 1.0, false, 0.0, false, false);
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_landing"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_landing_b as *const () as _))
}

unsafe extern "C" fn glide_landing_b(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_landing") && MotionModule::is_end(fighter.module_accessor) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN_WAIT.into(), false.into());
    }
    L2CValue::I32(0)
}

pub fn install() {
    smashline::install_status_scripts!(
        glide_start_a, 
        glide_main,
        glide_finish,
        glide_attack_a,
        glide_end_a,
        glide_landing_a
    );
}