use crate::imports::BuildImports::*;

unsafe extern "C" fn status_gamewatch_SpecialLw_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    0.into()
}

unsafe extern "C" fn status_gamewatch_SpecialLw_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let gauge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_GAUGE);
    if gauge < 3.0 {
        fighter.change_status(FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_WAIT_START.into(), false.into());
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_CHARGE_MAX);
        effect!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_REMOVE_COMMON, Hash40::new_raw(0xaec2db62e));
        fighter.change_status(FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_SHOOT.into(), false.into());
    }
    1.into()
}

pub fn install() {
    Agent::new("gamewatch")
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_gamewatch_SpecialLw_Init)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_gamewatch_SpecialLw_Main)
    .install();
}
