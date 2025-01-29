use crate::imports::BuildImports::*;

unsafe extern "C" fn status_link_boomerang_Haved_Main(weapon: &mut L2CFighterBase) -> L2CValue {
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_kind = smash::app::utility::get_kind(&mut *owner_module_accessor);
    if StatusModule::status_kind(owner_module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if ItemModule::is_have_item(owner_module_accessor, 0) {
            set_boomerang_fuse_params(weapon.module_accessor, ItemModule::get_have_item_kind(owner_module_accessor, 0), FuseKind::FUSE, ItemModule::get_have_item_trait(owner_module_accessor, 0) as i32);
        }
        else if [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&owner_kind) {
            let kind = WorkModule::get_int(owner_module_accessor, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM);
            if kind != *ITEM_KIND_NONE {
                set_boomerang_fuse_params(weapon.module_accessor, kind, FuseKind::REFUSE, i32::MAX);
            }
        }
        else {
            if owner_kind == *FIGHTER_KIND_LINK {
                WorkModule::set_int(owner_module_accessor, *ITEM_KIND_NONE, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
            }
        }
        if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            let item_id = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_module_accessor = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_module_accessor, true);
            if LinkModule::is_link(item_module_accessor, *ITEM_LINK_NO_HAVE) {
                LinkModule::unlink(item_module_accessor, *ITEM_LINK_NO_HAVE);
            }
            if !LinkModule::is_link(item_module_accessor, *ITEM_LINK_NO_HAVE) {
                VisibilityModule::set_whole(item_module_accessor, true);
                LinkModule::link(item_module_accessor, *ITEM_LINK_NO_HAVE, (*(weapon.module_accessor)).battle_object_id);
                LinkModule::set_model_constraint_pos_ort(item_module_accessor, *ITEM_LINK_NO_HAVE, Hash40::new("top"), Hash40::new("have"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
                let offset_pos = Vector3f{x: 0.0, y: 9.5, z: 0.0};
                LinkModule::set_constraint_translate_offset(item_module_accessor, &offset_pos);
            }
        }
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("haved"), 0.0, 1.0, false, 0.0, false, false);
        weapon.fastshift(L2CValue::Ptr(link_boomerang_Haved_Main_loop as *const () as _))
    }
    else {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("haved"), 0.0, 1.0, false, 0.0, false, false);
        weapon.fastshift(L2CValue::Ptr(link_boomerang_Haved_Main_loop as *const () as _))
    }
}

unsafe extern "C" fn link_boomerang_Haved_Main_loop(weapon: &mut L2CFighterBase) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_REMOVE_SELF) {
        notify_event_msc_cmd!(weapon,Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn status_link_boomerang_Haved_End(weapon: &mut L2CFighterBase) -> L2CValue {
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if StatusModule::status_kind(owner_module_accessor) == *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2 && WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let item_id = WorkModule::get_int64(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_module_accessor = smash::app::sv_battle_object::module_accessor(item_id);
        StatusModule::change_status_request(item_module_accessor, *ITEM_STATUS_KIND_FALL, false);
        if !ItemModule::is_have_item(owner_module_accessor, 0) {
            WorkModule::on_flag(owner_module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM);
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("link_boomerang")
    .status(Main, *WN_LINK_BOOMERANG_STATUS_KIND_HAVED, status_link_boomerang_Haved_Main)
    .status(End, *WN_LINK_BOOMERANG_STATUS_KIND_HAVED, status_link_boomerang_Haved_End)
    .install();
}