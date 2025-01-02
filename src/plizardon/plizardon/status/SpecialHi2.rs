use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_plizardon_SpecialHi2_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

pub unsafe extern "C" fn status_plizardon_SpecialHi2_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_situation(SITUATION_KIND_AIR.into());
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_2_start"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.sub_shift_status_main(L2CValue::Ptr(plizardon_SpecialHi2_Main_loop as *const () as _))
}

unsafe extern "C" fn plizardon_SpecialHi2_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    } 
    else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        fighter.change_status(FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_HI2_LANDING.into(), false.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_2"), 0.0, 1.0, true, 0.0, false, true);
    }
    0.into()
}

unsafe extern "C" fn status_plizardon_SpecialHi2_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_2"), hash40("air_speed_x"));
    let dive_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_2"), hash40("dive_speed_y"));
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_hi_2") {
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: air_speed_x, y: -dive_speed_y, z: 0.0});
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
            KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
    }
    0.into()
}

pub unsafe extern "C" fn status_plizardon_SpecialHi2_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_plizardon_special_h02_02"), 0);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLAG_CRITICAL);
    0.into()
}

pub fn install() {
    Agent::new("plizardon")
    .status(Pre, FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_HI2, status_plizardon_SpecialHi2_Pre)
    .status(Main, FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_HI2, status_plizardon_SpecialHi2_Main)
    .status(Exec, FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_HI2, status_plizardon_SpecialHi2_Exec)
    .status(End, FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_HI2, status_plizardon_SpecialHi2_End)
    .install();
}