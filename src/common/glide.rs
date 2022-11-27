use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue, L2CAgent}
    },
    smash_script::*,
    smashline::*
};

pub struct GlideParams {
    pub angle_max_up : f32, //#0 Max Upward Angle
    pub angle_max_down : f32, //#1 Max Downward Angle
    pub v_glide_start : f32, //#2 V Speed added on GlideStart
    pub gravity_start : f32, //#3 Gravity multiplier on GlideStart
    pub speed_mul_start : f32, //#4 H speed multiplier on GlideStart
    pub base_speed : f32, //#5 Base Power/Speed
    pub speed_change : f32, //#6 Power Rate
    pub max_speed : f32, //#7 Maximum Speed
    pub end_speed : f32, //#8 End Speed
    pub gravity_accel : f32, //#9 Gravity Acceleration
    pub gravity_speed : f32, //#10 Gravity Max Speed
    pub angle_extra : f32, //#11 Angle stuff but unknown what this is for
    pub angle_more_speed : f32, //#12 Angle to gain more speed
    pub down_speed_add : f32, //#13 Max added H speed gained aiming downward
    pub unknown : f32, //#14 Unknown
    pub radial_stick : f32, //#15 Radial Stick Sensitivity
    pub up_angle_accel : f32, //#16 Upward angular acceleration
    pub down_angle_accel : f32, //#17 Downward angular acceleration
    pub max_angle_speed : f32, //#18 Maximum angular speed
    pub add_angle_speed : f32 //#19 Added angular speed for when stick is center
}

impl GlideParams {
    pub fn get(fighter: &mut L2CFighterCommon) -> GlideParams {
        let kind = fighter.global_table[0x2].get_i32();
        if kind == *FIGHTER_KIND_METAKNIGHT {
            GlideParams {
                angle_max_up : 80.0,
                angle_max_down : -70.0,
                v_glide_start : 0.75,
                gravity_start : 1.0,
                speed_mul_start : 1.0,
                base_speed : 1.7,
                speed_change : 0.04,
                max_speed : 2.2,
                end_speed : 0.7,
                gravity_accel : 0.03,
                gravity_speed : 0.6,
                angle_extra : 15.0,
                angle_more_speed : -25.0,
                down_speed_add : 0.03,
                unknown : 0.15,
                radial_stick : 0.25,
                up_angle_accel : 0.55,
                down_angle_accel : 0.75,
                max_angle_speed : 7.0,
                add_angle_speed : 1.0
            }
        }
        else {
            // if fighter kind not defined, just use Meta Knight's params.
            GlideParams {
                angle_max_up : 80.0,
                angle_max_down : -70.0,
                v_glide_start : 0.75,
                gravity_start : 1.0,
                speed_mul_start : 1.0,
                base_speed : 1.7,
                speed_change : 0.04,
                max_speed : 2.2,
                end_speed : 0.7,
                gravity_accel : 0.03,
                gravity_speed : 0.6,
                angle_extra : 15.0,
                angle_more_speed : -25.0,
                down_speed_add : 0.03,
                unknown : 0.15,
                radial_stick : 0.25,
                up_angle_accel : 0.55,
                down_angle_accel : 0.75,
                max_angle_speed : 7.0,
                add_angle_speed : 1.0
            }
        }
    }
}

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

#[skyline::hook(replace = L2CFighterCommon_status_GlideStart)]
pub unsafe fn status_glidestart(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE_LANDING);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_start"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_GlideStart_Main as *const () as _))
}

#[common_status_script( status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn status_init_glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let params = GlideParams::get(fighter);
    WorkModule::set_float(fighter.module_accessor, params.base_speed, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
    WorkModule::set_float(fighter.module_accessor, -sum_speed_y, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);
    
    let initial_speed = params.base_speed * lr;
    kinetic_utility::reset_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_FREE, Vector2f{x: initial_speed, y: 0.0}, Vector3f{x: initial_speed, y: 0.0, z: 0.0} /*What is the Vector 3f for?*/);
    kinetic_utility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    kinetic_utility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    kinetic_utility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_Glide)]
pub unsafe fn status_glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_GLIDE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE_LANDING);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GLIDE_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_direction"), 90.0, 0.0, true, 0.0, false, false);
    MotionModule::add_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, Hash40::new("glide_wing"), 0.0, 1.0, true, false, 0.0, false, true, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Glide_Main as *const () as _))
}

#[common_status_script( status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn status_exec_glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    let params = GlideParams::get(fighter);
    let lr = PostureModule::lr(fighter.module_accessor);
    let _energy_stop = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let mut angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
    let mut angle_speed = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
    let mut stick_angle = ControlModule::get_stick_angle(fighter.module_accessor);

    if lr <= 0.0 {
        let mut above_or_below = -1.0;
        if stick_angle > 0.0 {
            above_or_below = 1.0;
        }
        stick_angle = (180.0 * above_or_below) - (stick_angle * 180.0 / std::f32::consts::PI);
    }
    else {
        stick_angle = stick_angle * 180.0 / std::f32::consts::PI;
    }

    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let stick_magnitude = (stick_x * stick_x + stick_y * stick_y).sqrt();

    if stick_magnitude > params.radial_stick {
        let angle_accel = if stick_angle < 0.0 {
            if stick_angle >= -135.0 {
                -params.down_angle_accel //What is angle_accel here?
            }
            else {
                params.up_angle_accel
            }   
        }
        else {
            if stick_angle >= 45.0 {
                params.up_angle_accel
            }
            else {
                -params.down_angle_accel
            }
        };

        let scaled_angle_accel = angle_accel * (stick_magnitude - params.radial_stick) / (1.0 - params.radial_stick);

        if angle_speed * scaled_angle_accel < 0.0 {
            angle_speed = 0.0;
        }
        
        let mut new_angle_speed = angle_speed + scaled_angle_accel;

        new_angle_speed = new_angle_speed.clamp(-params.max_angle_speed, params.max_angle_speed);
        WorkModule::set_float(fighter.module_accessor, new_angle_speed, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
        angle += new_angle_speed;
    }
    
    angle = angle.clamp(params.angle_max_down, params.angle_max_up);
    
    let mut power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
    power -= angle * params.speed_change / 90.0;
    // instead of setting the status flag for touching a wall,
    // we can just check it directly in this code
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
        power -= 0.01;
    }
    if power < 0.0 {
        power = 0.0
    }

    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_RAPID_FALL) {
        if angle < params.angle_more_speed {
            power += params.down_speed_add * (params.angle_more_speed - angle) / (params.angle_more_speed - params.angle_max_down);
        }
    }
    else if angle > 0.0 {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_RAPID_FALL);
    }

    let gravity = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);
    let mut new_gravity = gravity + params.gravity_accel;
    if new_gravity > params.gravity_speed {
        new_gravity = params.gravity_speed;
    }
    WorkModule::set_float(fighter.module_accessor, new_gravity, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);

    //let unrotated = Vector2f { x: power * lr, y: 0.0 };
    // TODO: probably want to make a new function for this, it doesn't seem like
    // the vec2_rot function from the game does what we want
    //let mut angled = smash::app::sv_math::vec2_rot(angle * lr * std::f32::consts::PI / 180.0, unrotated, 0.0 /*There's 3rd arg here*/);
    let mut angled = Vector2f {x: power * angle.to_radians().cos() * lr, y: power * angle.to_radians().sin()};
    angled.y -= new_gravity;

    let speed = (angled.x * angled.x + angled.y * angled.y).sqrt();
    let ratio = params.max_speed / speed;

    if speed > params.max_speed {
        angled.x *= ratio;
        angled.y *= ratio;
    }
    if speed < params.end_speed || power <= 0.0 {
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
    WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_Glide)]
pub unsafe fn bind_address_call_status_end_glide(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.status_end_Glide()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_Glide)]
pub unsafe fn status_end_glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, false);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_glidestart,
            status_glide, bind_address_call_status_end_glide, status_end_glide
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    install_status_scripts!(
        status_init_glide,
        status_exec_glide
    );
}