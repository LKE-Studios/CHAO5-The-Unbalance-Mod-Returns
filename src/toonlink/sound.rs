use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script(//AttackS4ChargeSFX
    agent = "toonlink", 
    script = "sound_attacks4charge", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_toonlink_attacks4charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
}

#[acmd_script(//AttackHi4ChargeSFX
    agent = "toonlink", 
    script = "sound_attackhi4charge", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_toonlink_attackhi4charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
}

#[acmd_script(//AttackLw4ChargeSFX
    agent = "toonlink", 
    script = "sound_attacklw4charge", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_toonlink_attacklw4charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_toonlink_attacks4charge,
        sound_toonlink_attackhi4charge,
        sound_toonlink_attacklw4charge
    );
}