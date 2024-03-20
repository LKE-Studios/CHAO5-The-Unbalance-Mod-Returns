use crate::imports::BuildImports::*;

//Main
unsafe extern "C" fn game_popo_whitebear_Main(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("body"), 999.0, 270, 77, 0, 60, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("head"), 999.0, 270, 77, 0, 60, 5.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_NONE);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 1, true, true, -1.0, false);
    }
}

pub fn install() {
    Agent::new("popo_whitebear")
    .game_acmd("game_main", game_popo_whitebear_Main)
    .install();
}