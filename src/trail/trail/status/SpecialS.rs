use crate::imports::BuildImports::*;

unsafe extern "C" fn status_trail_SpecialS_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_s_start").into(), Hash40::new("special_air_s_start").into(), false.into());
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_s").into());
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    fighter.sub_shift_status_main(L2CValue::Ptr(trail_SpecialS_Main_loop as *const () as _))
}

unsafe extern "C" fn trail_SpecialS_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_ATTACK.into(), false.into());
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_s_start").into(), Hash40::new("special_air_s_start").into(), true.into());
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_s").into());
    }
    0.into()
}

pub fn install() {
    Agent::new("trail")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, status_trail_SpecialS_Main)
    .install();
}