use crate::imports::BuildImports::*;

pub static max_speed_x : f32 = 1.5;
pub static x_acl_air : f32 = 0.05;
pub static y_acl_air : f32 = 0.7;
pub static ground_jump_x_dist : f32 = 1.0;
pub static init_speed_y : f32 = 1.7;

unsafe extern "C" fn status_waluigi_SpecialLwJump_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {   
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, true, *FIGHTER_TREADED_KIND_ENABLE, true, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_waluigi_SpecialLwJump_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {   
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
            sv_kinetic_energy!(controller_set_accel_x_mul, fighter, x_acl_air);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, max_speed_x, 0.0);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, max_speed_x, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, sum_speed_x, 0.0);
            sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.0);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_waluigi_SpecialLwJump_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_air_jump"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            let lr = PostureModule::lr(fighter.module_accessor); 
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_jump"), 0.0, 1.0, false, 0.0, false, false);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, ground_jump_x_dist * lr, 0.0);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(waluigi_SpecialLwJump_Main_loop as *const () as _))
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn waluigi_SpecialLwJump_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    let lr = PostureModule::lr(fighter.module_accessor); 
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * lr;
    let speed_x = x_acl_air * stick_x;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0 + speed_x, y: 0.0, z: 0.0});
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0, -y_acl_air);
        if frame >= 28.0 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_air_jump"), 0.0, 1.0, false, 0.0, false, false);
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND &&
    MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn status_waluigi_SpecialLwJump_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {
        0.into()
    }
    else {
        0.into()
    }
}

pub fn install() {
    Agent::new("dolly")
    .status(Pre, FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_LW_JUMP, status_waluigi_SpecialLwJump_Pre)
    .status(Init, FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_LW_JUMP, status_waluigi_SpecialLwJump_Init)
    .status(Main, FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_LW_JUMP, status_waluigi_SpecialLwJump_Main)
    .status(End, FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_LW_JUMP, status_waluigi_SpecialLwJump_End)
    .install();
}