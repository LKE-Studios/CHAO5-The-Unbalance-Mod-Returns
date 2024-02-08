use crate::imports::BuildImports::*;

#[acmd_script(//GuardSpecial
    agent = "pzenigame", 
    script = "sound_guardspecial", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pzenigame_guardspecial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_pzenigame_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pzenigame_bubble"));
    }
}

#[acmd_script(//EscapeAirSpecial
    agent = "pzenigame", 
    script = "sound_escapeairspecial", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pzenigame_escapeairspecial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_pzenigame_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pzenigame_bubble"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_pzenigame_guardspecial,
        sound_pzenigame_escapeairspecial
    );
}