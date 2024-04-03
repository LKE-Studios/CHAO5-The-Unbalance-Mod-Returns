use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_ryu_SpecialN_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

pub unsafe extern "C" fn status_ryu_SpecialN_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_START.into(), false.into());
    0.into()
}

pub unsafe extern "C" fn status_ryu_SpecialN_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ryu")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, status_ryu_SpecialN_Pre)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, status_ryu_SpecialN_Main)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, status_ryu_SpecialN_End)
    .install();
}