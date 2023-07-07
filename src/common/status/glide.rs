use crate::imports::BuildImports::*;

pub mod KineticUtility {
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

#[common_status_script( status = FIGHTER_STATUS_KIND_GLIDE_START, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn status_init_GlideStart(fighter: &mut L2CFighterCommon) -> L2CValue {
    let params = GlideParams::get(fighter);
    let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut smash::app::KineticEnergy;
    let motion = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut smash::app::KineticEnergy;
    let lr = PostureModule::lr(fighter.module_accessor);

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GLIDE_START);
    KineticEnergy::reset_energy(gravity, *ENERGY_GRAVITY_RESET_TYPE_GLIDE_START, &Vector2f{x: 0.0, y: -params.gravity_start}, &Vector3f{x: 0.0, y: -params.gravity_start, z: 0.0}, fighter.module_accessor);
    KineticEnergy::reset_energy(motion, *ENERGY_GRAVITY_RESET_TYPE_GLIDE_START, &Vector2f{x: params.speed_mul_start * lr, y: 0.0}, &Vector3f{x: params.speed_mul_start * lr, y: 0.0, z: 0.0}, fighter.module_accessor);
    KineticUtility::reset_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GLIDE_START, Vector2f{x: 0.0, y: params.v_glide_start}, Vector3f{x: 0.0, y: params.v_glide_start, z: 0.0});
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_GlideStart)]
pub unsafe fn status_GlideStart(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE_LANDING);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_start"), 0.0, 1.0, false, 0.0, false, false);
    if fighter.global_table[0x2].get_i32() == *FIGHTER_KIND_METAKNIGHT {
        let energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut smash::app::KineticEnergy;
        let anti_wind = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND) as *mut smash::app::KineticEnergy;
        KineticEnergy::clear_speed(energy);
        KineticEnergy::clear_speed(anti_wind);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_GlideStart_Main as *const () as _))
}

#[common_status_script( status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn status_init_Glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let params = GlideParams::get(fighter);
    WorkModule::set_float(fighter.module_accessor, params.base_speed, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
    WorkModule::set_float(fighter.module_accessor, -sum_speed_y, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);
    
    let initial_speed = params.base_speed * lr;
    KineticUtility::reset_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_FREE, Vector2f{x: initial_speed, y: 0.0}, Vector3f{x: initial_speed, y: 0.0, z: 0.0});
    KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_direction"), 90.0, 0.0, true, 0.0, false, false);
    MotionModule::add_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, Hash40::new("glide_wing"), 0.0, 1.0, true, false, 0.0, false, true, false);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_Glide)]
pub unsafe fn status_Glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_GLIDE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE_LANDING);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GLIDE_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    if fighter.global_table[0x2].get_i32() == *FIGHTER_KIND_PALUTENA {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_wing"), false, -1.0);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Glide_Main as *const () as _))
}

#[common_status_script( status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn status_exec_Glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    let params = GlideParams::get(fighter);
    let lr = PostureModule::lr(fighter.module_accessor);
    let _energy_stop = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let mut angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
    let mut angle_speed = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
    let mut stick_angle = ControlModule::get_stick_angle(fighter.module_accessor);

    fighter.sub_air_check_fall_common();
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
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
    let stick_magnitude = (stick_x * stick_x + stick_y * stick_y).sqrt(); //Square Root of Stick X^2 + Stick Y^2

    if stick_magnitude > params.radial_stick {
        let angle_accel = if stick_angle < 0.0 {
            if stick_angle >= -135.0 {
                -params.down_angle_accel
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
        power -= 0.0;
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

    /*Made a new function for this, it doesn't seem like the vec2_rot function in Ultimate does what we want*/
    let mut angled = Vector2f {x: power * angle.to_radians().cos() * lr, y: power * angle.to_radians().sin()};
    angled.y -= new_gravity;

    let speed = (angled.x * angled.x + angled.y * angled.y).sqrt(); //Square Root of angled X value + angled Y angle
    let ratio = params.max_speed / speed;

    if speed > params.max_speed {
        angled.x *= ratio;
        angled.y *= ratio;
    }
    if speed < params.end_speed || power <= 0.0 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
    }
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, angled.x, angled.y);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, angled.x, angled.y);
    WorkModule::set_float(fighter.module_accessor, power, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
    MotionModule::set_frame(fighter.module_accessor, 90.0 - angle, false);
    WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
    }
    //Fighter Specific
    let kind = fighter.global_table[0x2].get_i32();
    if kind == *FIGHTER_KIND_METAKNIGHT {
        let energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut smash::app::KineticEnergy;
        let anti_wind = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND) as *mut smash::app::KineticEnergy;
        let no_jostle = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE) as *mut smash::app::KineticEnergy;

        KineticEnergy::clear_speed(energy);
        KineticEnergy::clear_speed(anti_wind);
        KineticEnergy::clear_speed(no_jostle);
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_metaknight_glide_loop"), 1.0 + angle * -0.0035);
    }
    if kind == *FIGHTER_KIND_PIT {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_pit_glide_loop"), 1.0 + angle * -0.0047);
    }
    if kind == *FIGHTER_KIND_PITB {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_pitb_glide_loop"), 1.0 + angle * -0.0043);
    }
    if kind == *FIGHTER_KIND_PLIZARDON {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_plizardon_glide_loop"), 0.85 + angle * -0.006);
        SoundModule::set_se_vol(fighter.module_accessor, 0, 2.0 * (power * 0.5), 0);
    }
    if kind == *FIGHTER_KIND_RIDLEY {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_ridley_glide_loop"), 0.8 + angle * -0.005);
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_ridley_jump02"), 1.0 + angle * 0.003);
        if angle >= params.angle_max_down && angle < 0.0 {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, 1.0 + angle * 0.005);
        }
        if angle <= params.angle_max_up && angle > 0.0 {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, 1.0 + angle * 0.01);
        }
    }
    if kind == *FIGHTER_KIND_BUDDY {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_buddy_glide_loop"), 1.0 + angle * -0.005);
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_buddy_wing"), 1.0 + angle * 0.0048);
        if angle >= params.angle_max_down && angle < 0.0 {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, 1.0 + angle * 0.01);
        }
        if angle <= params.angle_max_up && angle > 0.0 {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, 1.0 + angle * 0.018);
        }
    }
    if kind == *FIGHTER_KIND_TRAIL {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_trail_glide_loop"), 1.1 + angle * -0.0071);
    }
    if kind == *FIGHTER_KIND_PALUTENA {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_palutena_glide_loop"), 1.0 + angle * -0.0043);
    }
    println!("x{}, y{}", angled.x, angled.y);
    println!("{}", angle);
    0.into()
}

#[common_status_script( status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
pub unsafe fn status_exit_Glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, false);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_Glide)]
pub unsafe fn bind_address_call_status_end_Glide(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.status_end_Glide()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_Glide)]
pub unsafe fn status_end_Glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, false);
    if fighter.global_table[0x2].get_i32() == *FIGHTER_KIND_PALUTENA {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}

#[common_status_script( status = FIGHTER_STATUS_KIND_GLIDE_END, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn status_init_GlideEnd(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut smash::app::KineticEnergy;
    let lr = PostureModule::lr(fighter.module_accessor);

    KineticUtility::reset_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_FREE, Vector2f{x: 0.0, y: 0.0}, Vector3f{x: 0.0, y: 0.0, z: 0.0});
    KineticEnergy::reset_energy(motion, *ENERGY_STOP_RESET_TYPE_AIR, &Vector2f{x: 0.0 * lr, y: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, fighter.module_accessor);
    KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_GlideStart,
            status_Glide, 
            bind_address_call_status_end_Glide, 
            status_end_Glide
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    install_status_scripts!(
        status_init_GlideStart,
        status_init_Glide,
        status_exec_Glide,
        status_exit_Glide,
        status_init_GlideEnd
    );
}