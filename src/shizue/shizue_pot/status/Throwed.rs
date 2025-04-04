use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_shizue_pot_Throwed_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let rand_num = WorkModule::get_int(owner_module_accessor, *FIGHTER_SHIZUE_INSTANCE_WORK_ID_INT_SHOT_KIND);
    shizue_furniture_visibility_all_false(weapon);
    if rand_num == 0 {
        shizue_furniture_picture(weapon);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("item_1"), 0.0, 1.0, false, 0.0, false, false);
    }
    if rand_num == 1 {
        shizue_furniture_rug(weapon);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("item_2"), 0.0, 1.0, false, 0.0, false, false);
    }
    if rand_num == 2 {
        shizue_furniture_triforce(weapon);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("item_3"), 0.0, 1.0, false, 0.0, false, false);
    }
    if rand_num == 3 {
        shizue_furniture_table(weapon);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("item_4"), 0.0, 1.0, false, 0.0, false, false);
    }
    if rand_num == 4 {
        shizue_furniture_trophy(weapon);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("item_5"), 0.0, 1.0, false, 0.0, false, false);
    }
    if rand_num == 5 {
        shizue_furniture_froggy(weapon);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("item_6"), 0.0, 1.0, false, 0.0, false, false);
    }
    if rand_num == 6 {
        shizue_furniture_moyai(weapon);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("item_7"), 0.0, 1.0, false, 0.0, false, false);
    }
    if rand_num == 7 {
        shizue_furniture_couch(weapon);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("item_8"), 0.0, 1.0, false, 0.0, false, false);
    }
    original_status(Main, weapon, *WEAPON_MURABITO_FLOWERPOT_STATUS_KIND_THROWED)(weapon)
}

unsafe extern "C" fn shizue_furniture_visibility_all_false(weapon: &mut L2CAgentBase) {
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture02"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture03"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture06"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture08"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture10"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture12"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture13"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture15"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture16"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture17"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture18"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture19"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture21"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture22"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture23"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture24"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture25"), false);
}

unsafe extern "C" fn shizue_furniture_picture(weapon: &mut L2CAgentBase) {
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture01"), true); //Photo
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture05"), false);
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture04"), false);
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture07"), false);
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture09"), false);
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture11"), false);
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture20"), false);
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture14"), false);
}

unsafe extern "C" fn shizue_furniture_rug(weapon: &mut L2CAgentBase) {
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture05"), true); //Rug
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture01"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture04"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture07"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture09"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture11"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture20"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture14"), false);
}

unsafe extern "C" fn shizue_furniture_triforce(weapon: &mut L2CAgentBase) {
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture04"), true); //Triforce
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture01"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture05"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture07"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture09"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture11"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture20"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture14"), false);
}

unsafe extern "C" fn shizue_furniture_table(weapon: &mut L2CAgentBase) {
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture07"), true); //Table
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture01"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture05"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture04"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture09"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture11"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture20"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture14"), false);
}

unsafe extern "C" fn shizue_furniture_trophy(weapon: &mut L2CAgentBase) {
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture09"), true); //Trophy
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture01"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture05"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture04"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture07"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture11"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture20"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture14"), false);
}

unsafe extern "C" fn shizue_furniture_froggy(weapon: &mut L2CAgentBase) {
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture11"), true); //Froggy
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture01"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture05"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture04"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture07"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture09"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture20"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture14"), false);
}

unsafe extern "C" fn shizue_furniture_moyai(weapon: &mut L2CAgentBase) {
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture20"), true); //Moyai
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture01"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture05"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture04"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture07"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture09"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture11"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture14"), false);
}

unsafe extern "C" fn shizue_furniture_couch(weapon: &mut L2CAgentBase) {
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture14"), true); //Couch
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture01"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture05"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture04"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture07"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture09"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture11"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("furniture20"), false);
}

pub fn install() {
    Agent::new("shizue_pot")
    .status(Main, *WEAPON_MURABITO_FLOWERPOT_STATUS_KIND_THROWED, status_shizue_pot_Throwed_Main)
    .install();
}