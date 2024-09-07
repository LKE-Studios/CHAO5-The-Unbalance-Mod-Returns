use crate::imports::BuildImports::*;

unsafe extern "C" fn status_link_boomerang_Swallowed_Pre(weapon: &mut L2CFighterBase) -> L2CValue {
    let item_id = WorkModule::get_int64(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
    if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED)
    && sv_battle_object::is_active(item_id) {
        let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
        smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, item_id);
    }
    original_status(Pre, weapon, *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED)(weapon)
}

unsafe extern "C" fn status_link_boomerang_Swallowed_End(weapon: &mut L2CFighterBase) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let item_id = WorkModule::get_int64(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        if smash::app::sv_battle_object::is_active(item_id) {
            let item_module_accessor = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_module_accessor, true);
            if LinkModule::is_link(item_module_accessor, *ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_module_accessor);
                let status = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
                StatusModule::change_status_request(item_module_accessor, status, false);
            }
        }
    }
    original_status(End, weapon, *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED)(weapon)
}

pub fn install() {
    Agent::new("link_boomerang")
    .status(Pre, *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED, status_link_boomerang_Swallowed_Pre)
    .status(End, *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED, status_link_boomerang_Swallowed_End)
    .install();
}