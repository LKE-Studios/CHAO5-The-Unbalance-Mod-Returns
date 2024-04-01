use crate::imports::BuildImports::*;
use crate::pit::pit::status::SpecialHiFly::*;

pub unsafe extern "C" fn status_pit_SpecialHiFlyTurn_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0 as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHiFlyTurn_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_x_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("speed_x_max"));
    let speed_y_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("speed_y_max"));
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x_max, speed_y_max);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x_max, speed_y_max);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, speed_y_max);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, speed_y_max);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHiFlyTurn_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_fly_turn"), 0.0, 1.0, false, 0.0, false, false);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    PostureModule::reverse_lr(fighter.module_accessor);
    fighter.sub_shift_status_main(L2CValue::Ptr(pit_SpecialHiFlyTurn_Main_loop as *const () as _))
}

unsafe extern "C" fn pit_SpecialHiFlyTurn_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY.into(), true.into());
    }
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHiFlyTurn_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_accel_x"));
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_accel_y"));
    let speed_x = air_accel_x * stick_x;
    let speed_y = air_accel_y * stick_y;
    let gravity_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("gravity_speed"));
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -gravity_speed);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x * lr, speed_y);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, speed_y);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    WorkModule::dec_int(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_TIME);
    println!("speed_x{}", speed_x);
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHiFlyTurn_End(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("pit")
    .status(Pre, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN, status_pit_SpecialHiFlyTurn_Pre)
    .status(Init, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN, status_pit_SpecialHiFlyTurn_Init)
    .status(Main, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN, status_pit_SpecialHiFlyTurn_Main)
    .status(Exec, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN, status_pit_SpecialHiFlyTurn_Exec)
    .status(End, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN, status_pit_SpecialHiFlyTurn_End)
    .install();
}