use crate::imports::BuildImports::*;

//Fly
unsafe extern "C" fn effect_trail_fire_Fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("trail_fire_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 4.0, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
}

//Fly2
unsafe extern "C" fn effect_trail_fire_Fly2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("trail_fire_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.5, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("trail_fire")   
    .effect_acmd("effect_fly", effect_trail_fire_Fly) 
    .effect_acmd("effect_fly2", effect_trail_fire_Fly2)
    .install();
}