use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*
};

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_JUMP_AERIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn sonic_jumpaerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ROCKETBELT) {
        let energy = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_ROCKETBELT_BURNER_ENERGY_VALUE);
        if 0.0 < energy {
            ItemModule::set_attach_item_action(fighter.module_accessor, ItemKind(*ITEM_KIND_ROCKETBELT), *ITEM_ROCKETBELT_ACTION_JUMP_JET_FIRE, 1.0);
        }
    }
    fighter.status_JumpAerialSub(false.into(), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_JumpAerial_Main as *const () as _))
}

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP_AERIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn sonic_screwjumpaerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_ItemScrewJumpAerialSub();
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_ItemScrewJumpAerial_Main as *const () as _))
}

pub fn install() {
    install_status_scripts!(
        sonic_jumpaerial_main,
        sonic_screwjumpaerial_main
    );
}