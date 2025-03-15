use crate::imports::BuildImports::*;

unsafe extern "C" fn status_bandana_fire_BurstS_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let color = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        WorkModule::set_int(owner_module_accessor, 0, *FIGHTER_BANDANA_INSTANCE_WORK_ID_INT_SPECIAL_S_STORE_FRAME);
        SoundModule::stop_se(weapon.module_accessor, Hash40::new("se_edge_special_n04_03"), 0);
        0.into()
    }
    else {
        original_status(End, weapon, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S)(weapon)
    }
}

pub fn install() {
    Agent::new("edge_fire")
    .status(End, *WEAPON_EDGE_FIRE_STATUS_KIND_BURST_S, status_bandana_fire_BurstS_End)
    .install();
}