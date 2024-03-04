use crate::imports::BuildImports::*;

unsafe extern "C" fn status_link_bowarrow_Haved_Main(weapon: &mut L2CFighterBase) -> L2CValue {
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_kind = utility::get_kind(&mut *owner_module_accessor);
    if ItemModule::is_have_item(owner_module_accessor, 0) {
        set_arrow_fuse_params(weapon.module_accessor, ItemModule::get_have_item_kind(owner_module_accessor, 0), FuseKind::FUSE, ItemModule::get_have_item_trait(owner_module_accessor, 0) as i32);
    }
    else if [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&owner_kind) {
        let kind = WorkModule::get_int(owner_module_accessor, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
        if kind != *ITEM_KIND_NONE {
            set_arrow_fuse_params(weapon.module_accessor, kind, FuseKind::REFUSE, i32::MAX);
        }
    }
    else {
        if owner_kind == *FIGHTER_KIND_LINK {
            WorkModule::set_int(owner_module_accessor, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
        }
        else if owner_kind == *FIGHTER_KIND_KIRBY {
            WorkModule::set_int(owner_module_accessor, *ITEM_KIND_NONE, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
        }
    }
    if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let item_id = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_module_accessor = sv_battle_object::module_accessor(item_id);
        LinkModule::remove_model_constraint(item_module_accessor, true);
        if LinkModule::is_link(item_module_accessor, *ITEM_LINK_NO_HAVE) {
            LinkModule::unlink(item_module_accessor, *ITEM_LINK_NO_HAVE);
        }
        if !LinkModule::is_link(item_module_accessor, *ITEM_LINK_NO_HAVE) {
            VisibilityModule::set_whole(item_module_accessor, true);
            LinkModule::link(item_module_accessor, *ITEM_LINK_NO_HAVE, (*(weapon.module_accessor)).battle_object_id);
            LinkModule::set_model_constraint_pos_ort(item_module_accessor, *ITEM_LINK_NO_HAVE, Hash40::new("top"), Hash40::new("top"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, true);
        }
    }
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("haved"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(link_bowarrow_Haved_Main_loop as *const () as _))
}

unsafe extern "C" fn link_bowarrow_Haved_Main_loop(weapon: &mut L2CFighterBase) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("link_bowarrow")
    .status(Main, *WN_LINK_BOWARROW_STATUS_KIND_HAVED, status_link_bowarrow_Haved_Main)
    .install();
}