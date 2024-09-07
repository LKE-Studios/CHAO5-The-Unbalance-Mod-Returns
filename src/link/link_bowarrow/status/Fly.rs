use crate::imports::BuildImports::*;

unsafe extern "C" fn status_link_bowarrow_Fly_Init(weapon: &mut L2CFighterBase) -> L2CValue {
    original_status(Init, weapon, *WN_LINK_BOWARROW_STATUS_KIND_FLY)(weapon);
    if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        if WorkModule::get_int(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG) == FuseType::POWER {
            let lr = PostureModule::lr(weapon.module_accessor);
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 12.0 * lr, 0.0);
            sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
            KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
            AttackModule::set_power_mul(weapon.module_accessor, 2.5);
        }
    }
    0.into()
}

unsafe extern "C" fn status_link_bowarrow_Fly_End(weapon: &mut L2CFighterBase) -> L2CValue {
    let status_kind_next = StatusModule::status_kind_next(weapon.module_accessor);
    let item_id = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
    let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
    if [*WN_LINK_BOWARROW_STATUS_KIND_STICK, *WN_LINK_BOWARROW_STATUS_KIND_HIT_STICK].contains(&status_kind_next) {
        if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            LinkModule::remove_model_constraint(item_boma, true);
            if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_boma);
                let status = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
                StatusModule::change_status_request(item_boma, status, false);
            }
        }
    }
    else {
        if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED)
        && sv_battle_object::is_active(item_id) {
            LinkModule::remove_model_constraint(item_boma, true);
            StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_FALL, false);
        }
    }
    EffectModule::detach_all(weapon.module_accessor, 5);
    0.into()
}

pub fn install() {
    Agent::new("link_bowarrow")
    .status(Init, *WN_LINK_BOWARROW_STATUS_KIND_FLY, status_link_bowarrow_Fly_Init)
    .status(End, *WN_LINK_BOWARROW_STATUS_KIND_FLY, status_link_bowarrow_Fly_End)
    .install();
}