use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_ryu_KamehamehaStart_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

pub unsafe extern "C" fn status_ryu_KamehamehaStart_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_all(fighter.module_accessor);
    KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("kamehameha_start"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_KamehamehaStart_Main_loop as *const () as _))
}

unsafe extern "C" fn ryu_KamehamehaStart_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() || fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor){
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_CHARGE.into(), false.into());
    }
    0.into()
}

pub unsafe extern "C" fn status_ryu_KamehamehaStart_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::resume_energy_all(fighter.module_accessor);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_hit_elec"), false, false);
    0.into()
}

pub fn install() {
    Agent::new("ryu")
    .status(Pre, FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_START, status_ryu_KamehamehaStart_Pre)
    .status(Main, FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_START, status_ryu_KamehamehaStart_Main)
    .status(End, FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_START, status_ryu_KamehamehaStart_End)
    .install();
}