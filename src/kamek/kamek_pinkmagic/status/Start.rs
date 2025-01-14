use crate::imports::BuildImports::*;

unsafe extern "C" fn status_kamek_pinkmagic_Start_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn status_kamek_pinkmagic_Start_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.fastshift(L2CValue::Ptr(kamek_pinkmagic_Start_Main_loop as *const () as _))
}

unsafe extern "C" fn kamek_pinkmagic_Start_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.change_status(WEAPON_KAMEK_PINKMAGIC_STATUS_KIND_SHOOT.into(), false.into());
    0.into()
}

unsafe extern "C" fn status_kamek_pinkmagic_Start_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
	0.into()
}

pub fn install() {
    Agent::new("ness_pinkmagic")
	.status(Pre, *WEAPON_KAMEK_PINKMAGIC_STATUS_KIND_START, status_kamek_pinkmagic_Start_Pre)
	.status(Main, *WEAPON_KAMEK_PINKMAGIC_STATUS_KIND_START, status_kamek_pinkmagic_Start_Main)
    .status(End, *WEAPON_KAMEK_PINKMAGIC_STATUS_KIND_START, status_kamek_pinkmagic_Start_End)
    .install();
}