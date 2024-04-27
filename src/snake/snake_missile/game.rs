use crate::imports::BuildImports::*;

//Explosion
unsafe extern "C" fn game_snake_missile_Explosion(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 150.0, 55, 80, 0, 100, 23.0, 0.0, 0.0, -6.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, false);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("snake_missile")
    .game_acmd("game_explosion", game_snake_missile_Explosion, Low)
    .install();
}