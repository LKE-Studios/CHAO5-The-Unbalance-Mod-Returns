use crate::imports::BuildImports::*;

unsafe extern "C" fn status_bandana_apple_Haved_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn status_bandana_apple_Haved_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.change_status(WEAPON_MASTER_ARROW1_STATUS_KIND_FLY.into(), true.into());
    0.into()
}

unsafe extern "C" fn status_bandana_apple_Haved_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("edge_apple")
    .status(Pre, *WEAPON_MASTER_ARROW1_STATUS_KIND_HAVED, status_bandana_apple_Haved_Pre)
    .status(Main, *WEAPON_MASTER_ARROW1_STATUS_KIND_HAVED, status_bandana_apple_Haved_Main)
    .status(End, *WEAPON_MASTER_ARROW1_STATUS_KIND_HAVED, status_bandana_apple_Haved_End)
    .install();
}