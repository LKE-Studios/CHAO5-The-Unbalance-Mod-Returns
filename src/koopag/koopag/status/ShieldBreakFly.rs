use crate::imports::BuildImports::*;

unsafe extern "C" fn status_koopag_ShieldBreakFly_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_BIND, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, 0, (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DAMAGE | *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY) as u32, 0, 0);
    0.into()
}

unsafe extern "C" fn status_koopag_ShieldBreakFly_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_ShieldBreakFly()
}

pub fn install() {
    Agent::new("koopag")
    .status(Pre, *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY, status_koopag_ShieldBreakFly_Pre)
    .status(Main, *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY, status_koopag_ShieldBreakFly_Main)
    .install();
}