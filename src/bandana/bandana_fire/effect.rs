use crate::imports::BuildImports::*;

//SpecialN1
unsafe extern "C" fn effect_bandana_fire_SpecialN1(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ness_pkfl_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, true);
    }
}

//BurstS
unsafe extern "C" fn effect_bandana_fire_BurstS(fighter: &mut L2CAgentBase) {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let store_frame = WorkModule::get_int(owner_module_accessor, *FIGHTER_BANDANA_INSTANCE_WORK_ID_INT_SPECIAL_S_STORE_FRAME);
    if is_excute(fighter) {
        if store_frame == 30 {
            EFFECT(fighter, Hash40::new("ness_pkfl_bomb"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else {
            EFFECT(fighter, Hash40::new("ness_pkfl_bullet_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

pub fn install() {
    Agent::new("edge_fire")
    .effect_acmd("effect_specialn1_bandana", effect_bandana_fire_SpecialN1, Low)
    .effect_acmd("effect_bursts_bandana", effect_bandana_fire_BurstS, Low)
    .install();
}