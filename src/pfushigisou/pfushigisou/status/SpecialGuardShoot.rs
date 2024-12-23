use crate::imports::BuildImports::*;
use crate::pfushigisou::pfushigisou::status::SpecialGuard::*;

pub unsafe extern "C" fn status_pfushigisou_SpecialGuardShoot_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

pub unsafe extern "C" fn status_pfushigisou_SpecialGuardShoot_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_z_shoot"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_z_shoot"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(pfushigisou_SpecialGuardShoot_Main_loop as *const () as _))
}

unsafe extern "C" fn pfushigisou_SpecialGuardShoot_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() || fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            SoundModule::play_landing_se(fighter.module_accessor, Hash40::new("se_pfushigisou_landing01"));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_z_shoot"), -1.0, 1.0, 0.0, false, false);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_z_shoot"), -1.0, 1.0, 0.0, false, false);
        }
    }    
    0.into()
}

pub unsafe extern "C" fn status_pfushigisou_SpecialGuardShoot_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_EFFECT_SCALE_MUL);
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_INSTANCE_WORK_ID_INT_CHARGE);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("finptrainer_solar_beam"), false, false);
    0.into()
}

pub fn install() {
    Agent::new("pfushigisou")
    .status(Pre, FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD_SHOOT, status_pfushigisou_SpecialGuardShoot_Pre)
    .status(Main, FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD_SHOOT, status_pfushigisou_SpecialGuardShoot_Main)
    .status(End, FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD_SHOOT, status_pfushigisou_SpecialGuardShoot_End)
    .install();
}