use crate::imports::BuildImports::*;

pub static special_hi_barrel_wait_timer : f32 = 180.0;
pub static special_hi_barrel_scale : f32 = 1.2;
pub static special_hi_barrel_pos : f32 = 8.6;
pub static special_hi_barrel_speed : f32 = 3.4;
pub static special_hi_barrel_speed_y_mul : f32 = 1.0;
pub static special_hi_barrel_x_control_mul : f32 = 1.25;
pub static special_hi_c2_frame : f32 = 50.0;

pub unsafe extern "C" fn status_funky_SpecialHi_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
        let kinetic = if situation_kind == *SITUATION_KIND_GROUND { *FIGHTER_KINETIC_TYPE_GROUND_STOP } else { *FIGHTER_KINETIC_TYPE_AIR_STOP };
        let correct = if situation_kind == *SITUATION_KIND_GROUND { *GROUND_CORRECT_KIND_KEEP } else { *GROUND_CORRECT_KIND_AIR };
        StatusModule::init_settings(fighter.module_accessor, SituationKind(situation_kind), kinetic, correct as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
        return false.into()
    }
    else {
        original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
    }
}

pub unsafe extern "C" fn status_funky_SpecialHi_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
        KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
        WorkModule::set_flag(fighter.module_accessor, situation_kind == *SITUATION_KIND_AIR, *FIGHTER_FUNKY_STATUS_SPECIAL_HI_FLAG_BARREL_START);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            let mut barrel_modules = fighter.module_accessor;
            VisibilityModule::set_model_visible(fighter.module_accessor, false);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL,true,0);
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL) {
                barrel_modules = CustomModule::get_article_module_accessor(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL);
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, Hash40::new("special_air_hi"), true, 0.0);
                PostureModule::set_lr(barrel_modules, 1.0);
                PostureModule::update_rot_y_lr(barrel_modules);
                let scale = PostureModule::scale(fighter.module_accessor);
                PostureModule::set_scale(barrel_modules, scale * special_hi_barrel_scale, false);
                PostureModule::add_pos(barrel_modules, &Vector3f{x: 0.0, y: special_hi_barrel_pos * scale, z: 0.0});
            }
        }
        else {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
        return false.into()
    }
    else {
        original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
    }
}

pub unsafe extern "C" fn status_funky_SpecialHi_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FUNKY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
        JostleModule::set_status(fighter.module_accessor, false);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            fighter.change_status(FIGHTER_FUNKY_STATUS_KIND_SPECIAL_HI_C2.into(), false.into());
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FUNKY_STATUS_SPECIAL_HI_FLAG_BARREL_START) {
            fighter.sub_shift_status_main(L2CValue::Ptr(funky_SpecialHi_Main_loop as *const () as _))
        }
        else {
            fighter.sub_shift_status_main(L2CValue::Ptr(funky_SpecialHi_Main_loop as *const () as _))
        }
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
    }
}

unsafe extern "C" fn funky_SpecialHi_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    let frame = MotionModule::frame(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if MotionModule::is_end(fighter.module_accessor) 
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FUNKY_STATUS_SPECIAL_HI_FLAG_GROUND_END) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 0.into();
    }
    let barrel_timer_float = WorkModule::get_float(fighter.module_accessor, *FIGHTER_FUNKY_INSTANCE_WORK_ID_FLOAT_BARREL_TIMER);
    WorkModule::add_float(fighter.module_accessor, 1.0, *FIGHTER_FUNKY_INSTANCE_WORK_ID_FLOAT_BARREL_TIMER);
    if barrel_timer_float > special_hi_barrel_wait_timer {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    if motion_kind == hash40("special_air_hi_launch") {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_FUNKY_STATUS_KIND_SPECIAL_HI_C2.into(), false.into());
        }
    }
    0.into()
}

pub unsafe extern "C" fn status_funky_SpecialHi_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        let frame = MotionModule::frame(fighter.module_accessor);                    
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let barrel_angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_FUNKY_STATUS_SPECIAL_HI_WORK_FLOAT_BARREL_ANGLE);
        println!("Barrel Angle{}", barrel_angle);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FUNKY_STATUS_SPECIAL_HI_FLAG_BARREL_START) {
            return 0.into();
        }
        ModelModule::set_visibility(fighter.module_accessor, false);
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            funky_SpecialHiLanding_function(fighter);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FUNKY_STATUS_SPECIAL_HI_FLAG_GROUND_END) {
            return 0.into();
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FUNKY_STATUS_SPECIAL_HI_FLAG_LAUNCH) {
                funky_SpecialHiLaunch_function(fighter);
            }
            else {
                funky_SpecialHiStart_function(fighter);
            }
        }
        return false.into()
    }
    else {
        0.into()
    }
}

unsafe fn funky_SpecialHiStart_function(fighter: &mut L2CFighterCommon) {
    let frame = MotionModule::frame(fighter.module_accessor);                    
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL) {
        if frame > 2.0 {
            fighter.change_status(FIGHTER_FUNKY_STATUS_KIND_SPECIAL_HI_C2.into(), false.into());
        }
        return;
    }
    let barrel_modules = CustomModule::get_article_module_accessor(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL);
    let barrel_motion = MotionModule::motion_kind(barrel_modules);
    let barrel_frame = MotionModule::frame(barrel_modules);
    let can_launch = barrel_motion == hash40("special_air_hi_aim");
    if frame < 10.0 {
        if can_launch {
            if MotionModule::is_end(barrel_modules) || barrel_frame <= 0.0 || barrel_frame >= 90.0 {
                let rate = MotionModule::rate(barrel_modules);
                ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, -rate);
            }
            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY != 0 {
                MotionModule::set_frame(fighter.module_accessor, 10.0, false);
                MotionModule::set_rate(fighter.module_accessor, 1.0);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, Hash40::new("special_air_hi_launch"), true, 0.0);
                ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, 1.0);
                let motion_angle = barrel_frame - 45.0;
                let mut barrel_angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_FUNKY_STATUS_SPECIAL_HI_WORK_FLOAT_BARREL_ANGLE);
                barrel_angle = motion_angle;
                let lr = PostureModule::lr(fighter.module_accessor);
                WorkModule::set_float(fighter.module_accessor, motion_angle, *FIGHTER_FUNKY_STATUS_SPECIAL_HI_WORK_FLOAT_BARREL_ANGLE);
                PostureModule::set_rot(barrel_modules, &Vector3f{x: motion_angle, y: 0.0, z: 0.0}, 0);
            }
        }
        else if MotionModule::is_end(barrel_modules) || barrel_frame >= 12.0 {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, Hash40::new("special_air_hi_aim"), true, 45.0);
            let lr = if (*PostureModule::pos(fighter.module_accessor)).x >= 0.0 {-1.0} else {1.0};
            ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, 3.0 * lr);
        }
    }
    else {
        let barrel_angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_FUNKY_STATUS_SPECIAL_HI_WORK_FLOAT_BARREL_ANGLE);
        if frame >= 28.0 {
            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_specialairhilaunch_funky"), -1);
            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_SOUND, Hash40::new("sound_specialairhilaunch_funky"), -1);
            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new("expression_specialairhilaunch_funky"), -1);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_launch"), 0.0, 1.0, false, 0.0, false, false);
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_flyroll_smoke"), Hash40::new("top"), &Vector3f{x: 0.0, y: 5.5, z: 0.0}, &VECTOR_ZERO, 0.75, true, 0, 0, 0, 0, 0, true, true);
            VisibilityModule::set_model_visible(fighter.module_accessor, true);
            ModelModule::set_visibility(fighter.module_accessor, true);
            if (barrel_angle.abs() > 1.0) {
                PostureModule::set_lr(fighter.module_accessor, barrel_angle.signum());
                PostureModule::update_rot_y_lr(fighter.module_accessor);
            }
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SUPER_JUMP_PUNCH_AIR);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed = special_hi_barrel_speed;
            let speed_x = (barrel_angle.to_radians()).sin() * speed;
            let speed_y = (barrel_angle.to_radians()).cos() * (speed * special_hi_barrel_speed_y_mul);
            let lr = PostureModule::lr(fighter.module_accessor);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, speed_y);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, speed_y);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FUNKY_STATUS_SPECIAL_HI_FLAG_LAUNCH);
        }
    }
}

unsafe fn funky_SpecialHiLaunch_function(fighter: &mut L2CFighterCommon) {
    let barrel_angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_FUNKY_STATUS_SPECIAL_HI_WORK_FLOAT_BARREL_ANGLE);
    let speed_x = (barrel_angle.to_radians()).sin() * special_hi_barrel_speed;
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let dive_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("dive_speed_y"), 0);
    let min_y = -2.0 * dive_speed_y;
    let lr = PostureModule::lr(fighter.module_accessor);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, (speed_y - 0.07).max(min_y));
    println!("X Speed{}", speed_x);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let air_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_mul"), 0);
    let air_speed_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_add"), 0);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let mut add_speed = Vector3f{x: stick_x * air_speed_x_mul * special_hi_barrel_x_control_mul, y: 0.0, z: 0.0};
    KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &add_speed);
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32){
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), false);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), true.into());
    }
}

unsafe fn funky_SpecialHiLanding_function(fighter: &mut L2CFighterCommon) {
    fighter.sub_set_ground_correct_by_situation(false.into());
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), true.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_UNIQ);
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_DASH, sum_speed_x, 0.0, 0.0, 0.0, 0.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FUNKY_STATUS_SPECIAL_HI_FLAG_GROUND_END);
    }
}

pub unsafe extern "C" fn status_funky_SpecialHi_Exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        VisibilityModule::set_model_visible(fighter.module_accessor, true);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_FUNKY_INSTANCE_WORK_ID_FLOAT_BARREL_TIMER);
        JostleModule::set_status(fighter.module_accessor, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_flyroll_smoke"), false, false);
        let start = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FUNKY_STATUS_SPECIAL_HI_FLAG_BARREL_START);
        let launch = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FUNKY_STATUS_SPECIAL_HI_FLAG_LAUNCH);
        if start && !launch {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("donkey_entry"), Hash40::new("top"), &Vector3f{x: 0.0, y: 2.0, z: 0.0}, &VECTOR_ZERO, 1.0, true, 0, 0, 0, 0, 0, true, true);
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_donkey_appear01"), true, false, false, false, enSEType(0));
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL) {
                ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }
        0.into()
    }
    else {
        original_status(Exit, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
    }
}

pub fn install() {
    Agent::new("donkey")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_funky_SpecialHi_Pre)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_funky_SpecialHi_Init)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_funky_SpecialHi_Main)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_funky_SpecialHi_Exec)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_funky_SpecialHi_Exit)
    .install();
}