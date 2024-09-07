use crate::imports::BuildImports::*;

unsafe extern "C" fn status_sonic_JumpAerial_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ROCKETBELT) {
        let energy = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_ROCKETBELT_BURNER_ENERGY_VALUE);
        if 0.0 < energy {
            ItemModule::set_attach_item_action(fighter.module_accessor, ItemKind(*ITEM_KIND_ROCKETBELT), *ITEM_ROCKETBELT_ACTION_JUMP_JET_FIRE, 1.0);
        }
    }
    fighter.status_JumpAerialSub(false.into(), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_JumpAerial_Main as *const () as _))
}

pub fn install() {
    Agent::new("sonic")
    .status(Main, *FIGHTER_STATUS_KIND_JUMP_AERIAL, status_sonic_JumpAerial_Main)
    .install();
}