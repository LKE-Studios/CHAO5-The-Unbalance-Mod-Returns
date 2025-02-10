use crate::imports::BuildImports::*;

//Regular
unsafe extern "C" fn effect_funky_boot_Regular(fighter: &mut L2CAgentBase) {
    loop {   
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_misfire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 4.0);
    }
}

pub fn install() {
    Agent::new("donkey_boot")
    .effect_acmd("effect_regular", effect_funky_boot_Regular, Low)
    .install();
}