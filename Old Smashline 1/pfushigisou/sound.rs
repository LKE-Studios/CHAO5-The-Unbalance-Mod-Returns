use crate::imports::BuildImports::*;

#[acmd_script(//GuardSpecialStart
    agent = "pfushigisou", 
    script = "sound_guardspecialstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pfushigisou_guardspecialstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_appeal_h01"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_heal_start")); //Index 80
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_special_n01"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_recovery")); //Index 81
    }
}

#[acmd_script(//GuardSpecialCharge
    agent = "pfushigisou", 
    script = "sound_guardspecialcharge", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pfushigisou_guardspecialcharge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
    }
    for _ in 0..8 {
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_appeal_l02"));
        }
        wait(fighter.lua_state_agent, 15.0);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
    }
    frame(fighter.lua_state_agent, 80.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
    }
}

#[acmd_script(//GuardSpecialShoot
    agent = "pfushigisou", 
    script = "sound_guardspecialshoot", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pfushigisou_guardspecialshoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_special_beam"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_smash_h02"));
    }
}

#[acmd_script(//GuardSpecialEnd
    agent = "pfushigisou", 
    script = "sound_guardspecialend", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pfushigisou_guardspecialend(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pfushigisou_appeal_l01"));
        STOP_SE(fighter, Hash40::new("se_pfushigisou_recovery"));
    }
}

#[acmd_script(//EscapeAirSpecialStart
    agent = "pfushigisou", 
    script = "sound_escapeairspecialstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pfushigisou_escapeairspecialstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_appeal_h01"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_heal_start")); //Index 80
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_special_n01"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_recovery")); //Index 81
    }
}

#[acmd_script(//EscapeAirSpecialCharge
    agent = "pfushigisou", 
    script = "sound_escapeairspecialcharge", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pfushigisou_escapeairspecialcharge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
    }
    for _ in 0..8 {
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_appeal_l02"));
        }
        wait(fighter.lua_state_agent, 15.0);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
    }
    frame(fighter.lua_state_agent, 80.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
    }
}

#[acmd_script(//EscapeAirSpecialShoot
    agent = "pfushigisou", 
    script = "sound_escapeairspecialshoot", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pfushigisou_escapeairspecialshoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_special_beam"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_smash_h02"));
    }
}

#[acmd_script(//EscapeAirSpecialEnd
    agent = "pfushigisou", 
    script = "sound_escapeairspecialend", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pfushigisou_escapeairspecialend(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pfushigisou_appeal_l01"));
        STOP_SE(fighter, Hash40::new("se_pfushigisou_recovery"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_pfushigisou_guardspecialstart,
        sound_pfushigisou_guardspecialcharge,
        sound_pfushigisou_guardspecialshoot,
        sound_pfushigisou_guardspecialend,
        sound_pfushigisou_escapeairspecialstart,
        sound_pfushigisou_escapeairspecialcharge,
        sound_pfushigisou_escapeairspecialshoot,
        sound_pfushigisou_escapeairspecialend
    );
}
