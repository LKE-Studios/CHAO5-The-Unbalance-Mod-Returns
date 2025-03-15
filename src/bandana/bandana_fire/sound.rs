use crate::imports::BuildImports::*;

//SpecialN1
unsafe extern "C" fn sound_bandana_fire_SpecialN1(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_special_n04_03"));
    }
}

//BurstS
unsafe extern "C" fn sound_bandana_fire_BurstS(fighter: &mut L2CAgentBase) {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let store_frame = WorkModule::get_int(owner_module_accessor, *FIGHTER_BANDANA_INSTANCE_WORK_ID_INT_SPECIAL_S_STORE_FRAME);
    if is_excute(fighter) {
        if store_frame == 30 {
            PLAY_SE(fighter, Hash40::new("se_edge_special_n03_03"));
        }
        else {
            PLAY_SE(fighter, Hash40::new("se_edge_special_n03_02"));
        }
    }
}

pub fn install() {
    Agent::new("edge_fire")
    .sound_acmd("sound_specialn1_bandana", sound_bandana_fire_SpecialN1, Low)
    .sound_acmd("sound_bursts_bandana", sound_bandana_fire_BurstS, Low)
    .install();
}