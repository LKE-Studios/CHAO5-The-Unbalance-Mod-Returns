use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;
use crate::utils::*;
use smashline::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::hash40;
use smash::phx::Vector2f;
use smash::phx::{Vector3f, Hash40};
use smash_script::*;

pub const PI : f32 = 3.14159265358979323846264338327950288;

static ANGLE_MAX_UP : f32 = 80.0; //#0 Max Upward Angle
static ANGLE_MAX_DOWN : f32 = -70.0; //#1 Max Downward Angle
static V_GLIDE_START : f32 = 0.75; //#2 V Speed added on GlideStart
static GRAVITY_START : f32 = 1.0; //#3 Gravity multiplier on GlideStart
static SPEED_MUL_START : f32 = 1.0; //#4 H speed multiplier on GlideStart
static BASE_SPEED : f32 = 1.7; //#5 Base Power/Speed
static SPEED_CHANGE : f32 = 0.04; //#6 Power Rate
static MAX_SPEED : f32 = 2.2; //#7 Maximum Speed
static END_SPEED : f32 = 0.7; //#8 End Speed
static GRAVITY_ACCEL : f32 = 0.03; //#9 Gravity Acceleration
static GRAVITY_SPEED : f32 = 0.6; //#10 Gravity Max Speed
static ANGLE_EXTRA : f32 = 15.0; //#11 Angle stuff but unknown what this is for
static ANGLE_MORE_SPEED : f32 = -25.0; //#12 Angle to gain more speed
static DOWN_SPEED_ADD : f32 = 0.03; //#13 Max added H speed gained aiming downward
static UNKNOWN : f32 = 0.15; //#14 Unknown
static RADIAL_STICK : f32 = 0.25; //#15 Radial Stick Sensitivity
static UP_ANGLE_ACCEL : f32 = 0.55; //#16 Upward angular acceleration
static DOWN_ANGLE_ACCEL : f32 = 0.75; //#17 Downward angular acceleration
static MAX_ANGLE_SPEED : f32 = 7.0; //#18 Maximum angular speed
static ADD_ANGLE_SPEED : f32 = 1.0; //#19 Added angular speed for when stick is center

mod kinetic_utility {
    // Resets and enables the kinetic energy type.
    // Unknown why there are two vectors required by reset_energy
    pub unsafe fn reset_enable_energy(module_accessor: *mut smash::app::BattleObjectModuleAccessor, energy_id: i32, energy_reset_type: i32, speed_vec: smash::phx::Vector2f, other_vec: smash::phx::Vector3f) {
        let energy = smash::app::lua_bind::KineticModule::get_energy(module_accessor, energy_id) as *mut smash::app::KineticEnergy;
        smash::app::lua_bind::KineticEnergy::reset_energy(energy, energy_reset_type, &speed_vec, &other_vec, module_accessor);
        smash::app::lua_bind::KineticEnergy::enable(energy);
    }

    // Clears and disables the kinetic energy type
    pub unsafe fn clear_unable_energy(module_accessor: *mut smash::app::BattleObjectModuleAccessor, energy_id: i32) {
        let energy = smash::app::lua_bind::KineticModule::get_energy(module_accessor, energy_id) as *mut smash::app::KineticEnergy;
        smash::app::lua_bind::KineticEnergy::clear_speed(energy);
        smash::app::lua_bind::KineticEnergy::unable(energy);
    }
}

#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_start_main_start(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_start"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GLIDE_START);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_start_main as *const () as _))
}

unsafe extern "C" fn glide_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_start") && MotionModule::is_end(fighter.module_accessor) {   
        fighter.change_status(FIGHTER_STATUS_KIND_GLIDE.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn glide_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    WorkModule::set_float(fighter.module_accessor, BASE_SPEED, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
    WorkModule::set_float(fighter.module_accessor, -sum_speed_y, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);
    
    let initial_speed = BASE_SPEED * lr;
    kinetic_utility::reset_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_FREE, Vector2f{x: initial_speed, y: 0.0}, Vector3f{x: initial_speed, y: 0.0, z: 0.0} /*What is the Vector 3f for?*/);
    kinetic_utility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    kinetic_utility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    kinetic_utility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    L2CValue::I32(0)
}

#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_main_start(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_direction"), 90.0, 0.0, true, 0.0, false, false);
    MotionModule::add_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, Hash40::new("glide_wing"), 0.0, 1.0, true, false, 0.0, false, true, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_main as *const () as _))
}

unsafe extern "C" fn glide_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if is_grounded(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_LANDING.into(), true.into());
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP) {
        fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_END.into(), true.into());
    }
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_ATTACK.into(), true.into());
    }
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_END.into(), true.into());
    }
    0.into()
}

#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn glide_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let energy_stop = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let mut angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
    let mut angle_speed = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
    let mut stick_angle = ControlModule::get_stick_angle(fighter.module_accessor);

    if lr <= 0.0 {
        let mut above_or_below = -1.0;
        if stick_angle > 0.0 {
            above_or_below = 1.0;
        }
        stick_angle = (180.0 * above_or_below) - (stick_angle * 180.0 / PI);
    }
    else {
        stick_angle = stick_angle * 180.0 / PI;
    }

    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let stick_magnitude = (stick_x * stick_x + stick_y * stick_y).sqrt();

    if stick_magnitude > RADIAL_STICK {
        let mut angle_accel = 0.0;
        if stick_angle < 0.0 {
            if stick_angle >= -135.0 {
                angle_accel = -DOWN_ANGLE_ACCEL; //What is angle_accel here?
            }
            else {
                angle_accel = UP_ANGLE_ACCEL;
            }   
        }
        else {
            if stick_angle >= 45.0 {
                angle_accel = UP_ANGLE_ACCEL;
            }
            else {
                angle_accel = -DOWN_ANGLE_ACCEL;
            }
        }

        let scaled_angle_accel = angle_accel * (stick_magnitude - RADIAL_STICK) / (1.0 - RADIAL_STICK);

        if angle_speed * scaled_angle_accel < 0.0 {
            angle_speed = 0.0;
        }
        
        let mut new_angle_speed = angle_speed + scaled_angle_accel;

        if new_angle_speed < -MAX_ANGLE_SPEED {
            new_angle_speed = -MAX_ANGLE_SPEED;
        }
        if new_angle_speed > MAX_ANGLE_SPEED {
            new_angle_speed = MAX_ANGLE_SPEED;
        }
        WorkModule::set_float(fighter.module_accessor, new_angle_speed, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
        angle = angle + new_angle_speed;
    }
    
    if angle < ANGLE_MAX_DOWN {
        angle = ANGLE_MAX_DOWN;
    }
    if angle > ANGLE_MAX_UP {
        angle = ANGLE_MAX_UP;
    }
    
    let mut power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
    power = power - (angle * SPEED_CHANGE / 90.0);
    // instead of setting the status flag for touching a wall,
    // we can just check it directly in this code
    if GroundModule::is_touch(fighter.module_accessor, 0x6) {
        power = power - 0.0;
    }
    if power < 0.0 {
        power = 0.0
    }

    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_RAPID_FALL) {
        if angle < ANGLE_MORE_SPEED {
            power = power + (DOWN_SPEED_ADD * (ANGLE_MORE_SPEED - angle) / (ANGLE_MORE_SPEED - ANGLE_MAX_DOWN));
        }
    }
    else if angle > 0.0 {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_RAPID_FALL);
    }

    let gravity = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);
    let mut new_gravity = gravity + GRAVITY_ACCEL;
    if new_gravity > GRAVITY_SPEED {
        new_gravity = GRAVITY_SPEED;
    }
    WorkModule::set_float(fighter.module_accessor, new_gravity, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);

    //let unrotated = Vector2f { x: power * lr, y: 0.0 };
    // TODO: probably want to make a new function for this, it doesn't seem like
    // the vec2_rot function from the game does what we want
    //let mut angled = smash::app::sv_math::vec2_rot(angle * lr * PI / 180.0, unrotated, 0.0 /*There's 3rd arg here*/);
    let mut angled = Vector2f {x: power * angle.to_radians().cos() * lr, y: power * angle.to_radians().sin()};
    angled.y = angled.y - new_gravity;

    let speed = (angled.x * angled.x + angled.y * angled.y).sqrt();
    let ratio = MAX_SPEED / speed;

    if speed > MAX_SPEED {
        angled.x = angled.x * ratio;
        angled.y = angled.y * ratio;
    }
    if speed < END_SPEED || power <= 0.0 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
    }
    // TODO: figure out how to set X and Y speed directly in an Energy
    //energy_stop.speed_x = angled.x;
    //energy_stop.speed_y = angled.y;
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, angled.x, angled.y);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, angled.x, angled.y);
    WorkModule::set_float(fighter.module_accessor, power, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
    MotionModule::set_frame(fighter.module_accessor, 90.0 - angle, false);
    SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_metaknight_jump05_win02"), 1.0 + angle * -0.0035);
    WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
    0.into()
}

#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn glide_finish(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, false);
    L2CValue::I32(0)
}

#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_attack_main_start(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_attack"), -1.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_attack_main as *const () as _))
}

unsafe extern "C" fn glide_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_fall_common();
    WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_JUMP_FLY_NEXT);
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_attack") && MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_end_main_start(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_end"), -1.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_end_main as *const () as _))
}

unsafe extern "C" fn glide_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_fall_common();
    WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_end") && MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_landing_main_start(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_landing"), -1.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_landing_main as *const () as _))
}

unsafe extern "C" fn glide_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_landing") && MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN_WAIT.into(), false.into());
    }
    L2CValue::I32(0)
}

pub fn install() {
    smashline::install_status_scripts!(
        glide_start_main_start, 
        glide_init,
        glide_main_start,
        glide_exec,
        glide_finish,
        glide_attack_main_start,
        glide_end_main_start,
        glide_landing_main_start
    );
}