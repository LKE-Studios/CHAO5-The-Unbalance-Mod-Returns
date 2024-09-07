use crate::imports::BuildImports::*;

//Fly
unsafe extern "C" fn effect_trail_ice_Fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("trail_ice_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 4.5, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
}

//FlyLast
unsafe extern "C" fn effect_trail_ice_FlyLast(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("trail_ice_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 6.0, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("trail_ice")   
    .effect_acmd("effect_fly", effect_trail_ice_Fly, Low) 
    .effect_acmd("effect_flylast", effect_trail_ice_FlyLast, Low)
    .install();
}
