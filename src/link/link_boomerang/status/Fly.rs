use crate::imports::BuildImports::*;

unsafe extern "C" fn status_link_boomerang_Fly_End(weapon: &mut L2CFighterBase) -> L2CValue {
    let status_kind_next = StatusModule::status_kind_next(weapon.module_accessor);
    let item_id = WorkModule::get_int64(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
    let item_module_accessor = smash::app::sv_battle_object::module_accessor(item_id);
    if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED)
    && !WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT)
    && AttackModule::is_infliction_status(weapon.module_accessor,*COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        LinkModule::remove_model_constraint(item_module_accessor, true);
        if LinkModule::is_link(item_module_accessor, *ITEM_LINK_NO_HAVE) {
            LinkModule::unlink_all(item_module_accessor);
            let status = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
            StatusModule::change_status_request(item_module_accessor, status, false);
        }
    }
    else if ![*WN_LINK_BOOMERANG_STATUS_KIND_TURN, *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED].contains(&status_kind_next)
    && WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        LinkModule::remove_model_constraint(item_module_accessor, true);
        StatusModule::change_status_request(item_module_accessor, *ITEM_STATUS_KIND_FALL, false);
    }
    0.into()
}

pub fn install() {
    Agent::new("link_boomerang")
    .status(End, *WN_LINK_BOOMERANG_STATUS_KIND_FLY, status_link_boomerang_Fly_End)
    .install();
}