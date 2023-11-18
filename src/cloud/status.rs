use crate::imports::BuildImports::*;

#[status_script( agent = "cloud", status = FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2_FALL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_cloud_special_hi2_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi2_fall"), 0.0, 1.0, false, 0.0, false, false);
    if StopModule::is_stop(fighter.module_accessor) {
        
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
    KineticUtility::reset_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_STOP_RESET_TYPE_GRAVITY, Vector2f{x: 0.0, y: 0.0}, Vector3f{x: 0.0, y: 0.0, z: 0.0});
}