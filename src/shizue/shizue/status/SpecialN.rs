use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_shizue_SpecialN_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_attr = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_N_INT_STATUS_ATTR);
    WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);
    WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MURABITO_STATUS_SPECIAL_N_INT_ATTACK_OBJECT_ID);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_ATTR_START_TURN, *FIGHTER_MURABITO_STATUS_SPECIAL_N_INT_STATUS_ATTR);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x29518b01d8));
    if status_attr == *FIGHTER_STATUS_ATTR_START_TURN {
        fighter.change_status(FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_POCKET.into(), false.into());
    }
    else {
        let object_category = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_OBJECT_CATEGORY);
        if object_category == *BATTLE_OBJECT_CATEGORY_INVALID {
            fighter.change_status(FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_TAKE_OUT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH.into(), false.into());
        }
    }
    1.into()
}

pub fn install() {
    Agent::new("shizue")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, status_shizue_SpecialN_Main)
    .install();
}