use crate::imports::BuildImports::*;

unsafe extern "C" fn status_link_bowarrow_HitStick_End(weapon: &mut L2CFighterBase) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT) {
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor(owner_id);
        let owner_kind = utility::get_kind(&mut *owner_module_accessor);
        let team_no = if owner_kind == *FIGHTER_KIND_KIRBY {
            WorkModule::get_int(owner_module_accessor, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_TEAM_NO)
        }
        else if [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&owner_kind) {
            WorkModule::get_int(owner_module_accessor, FIGHTER_MURABTIO_INSTANCE_WORK_ID_INT_TEAM_NO)
        }
        else {
            WorkModule::get_int(owner_module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_INT_TEAM_NO)
        };
        TeamModule::set_team(weapon.module_accessor, team_no, true);
        TeamModule::set_team_owner_id(weapon.module_accessor, (*(owner_module_accessor)).battle_object_id);
        WorkModule::set_flag(weapon.module_accessor, false, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
    }
    0.into()
}

pub fn install() {
    Agent::new("link_bowarrow")
    .status(End, *WN_LINK_BOWARROW_STATUS_KIND_HIT_STICK, status_link_bowarrow_HitStick_End)
    .install();
}