use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_shizue_SpecialNSearch_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

pub unsafe extern "C" fn status_shizue_SpecialNSearch_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_MURABITO_STATUS_SPECIAL_N_INT_STATUS_ATTR);
    WorkModule::set_float(fighter.module_accessor, 0.3, *FIGHTER_MURABITO_STATUS_SPECIAL_N_FLOAT_PULL_SPEED);
    shizue_SpecialN_shot_helper(fighter);
    original_status(Main, fighter, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH)(fighter)
}

pub unsafe extern "C" fn shizue_SpecialN_shot_helper(fighter: &mut L2CFighterCommon) {
    let mut rand_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SHIZUE_INSTANCE_WORK_ID_INT_SHOT_KIND);
    rand_num = sv_math::rand(hash40("shizue"), 8);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SHIZUE_INSTANCE_WORK_ID_FLAG_RANDOM_SHOT) {
        if rand_num == 0 {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SHIZUE_INSTANCE_WORK_ID_INT_SHOT_KIND);
        }
        else if rand_num == 1 {
            WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_SHIZUE_INSTANCE_WORK_ID_INT_SHOT_KIND);
        }
        else if rand_num == 2 {
            WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_SHIZUE_INSTANCE_WORK_ID_INT_SHOT_KIND);
        }
        else if rand_num == 3 {
            WorkModule::set_int(fighter.module_accessor, 3, *FIGHTER_SHIZUE_INSTANCE_WORK_ID_INT_SHOT_KIND);
        }
        else if rand_num == 4 {
            WorkModule::set_int(fighter.module_accessor, 4, *FIGHTER_SHIZUE_INSTANCE_WORK_ID_INT_SHOT_KIND);
        }
        else if rand_num == 5 {
            WorkModule::set_int(fighter.module_accessor, 5, *FIGHTER_SHIZUE_INSTANCE_WORK_ID_INT_SHOT_KIND);
        }
        else if rand_num == 6 {
            WorkModule::set_int(fighter.module_accessor, 6, *FIGHTER_SHIZUE_INSTANCE_WORK_ID_INT_SHOT_KIND);
        }
        else if rand_num == 7 {
            WorkModule::set_int(fighter.module_accessor, 7, *FIGHTER_SHIZUE_INSTANCE_WORK_ID_INT_SHOT_KIND);
        }
    }
}

pub fn install() {
    Agent::new("shizue")
    .status(Pre, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH, status_shizue_SpecialNSearch_Pre)
    .status(Main, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH, status_shizue_SpecialNSearch_Main)
    .install();
}