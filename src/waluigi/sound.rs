use crate::imports::BuildImports::*;

#[acmd_script(//Attack11
    agent = "dolly",
    script =  "sound_attack11_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_swing_s"));
    }
}

#[acmd_script(//Attack12
    agent = "dolly",
    script =  "sound_attack12_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_swing_m"));
    }
}

#[acmd_script(//Attack13
    agent = "dolly",
    script =  "sound_attack13_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_attack13(fighter: &mut L2CAgentBase) {}

#[acmd_script(//AttackDash
    agent = "dolly",
    script =  "sound_attackdash_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_attackdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_attackdash01"));
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("seq_dolly_rnd_attackdash"));
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("se_dolly_attackair_h01"));
    }
}



pub fn install() {
    smashline::install_acmd_scripts!(
        sound_waluigi_attack11,
        sound_waluigi_attack12,
        sound_waluigi_attack13,
        sound_waluigi_attackdash,
        sound_waluigi_attacks3,
        sound_waluigi_attackhi3,
        sound_waluigi_attacklw3,
        sound_waluigi_attacks4,
        sound_waluigi_attackhi4,
        sound_waluigi_attacklw4,
        sound_waluigi_attackairn,
        sound_waluigi_attackairf,
        sound_waluigi_attackairb,
        sound_waluigi_attackairhi,
        sound_waluigi_attackairlw,
        sound_waluigi_catch,
        sound_waluigi_catchdash,
        sound_waluigi_catchturn,
        sound_waluigi_catchattack,
        sound_waluigi_throwf,
        sound_waluigi_throwb,
        sound_waluigi_throwhi,
        sound_waluigi_throwlw,
        sound_waluigi_downattacku,
        sound_waluigi_downattackd,
        sound_waluigi_cliffattack,
        sound_waluigi_specialn,
        sound_waluigi_specialairn,
        sound_waluigi_specialsbstart,
        sound_waluigi_specialsbattackw,
        sound_waluigi_specialsbattack,
        sound_waluigi_specialairsfend,
        sound_waluigi_specialairsfend2,
        sound_waluigi_specialairsfstart,
        sound_waluigi_specialsfattack,
        sound_waluigi_specialairsfattack,
        sound_waluigi_specialhi1,
        sound_waluigi_specialairhi1,
        sound_waluigi_speciallwstart,
        sound_waluigi_specialairlwstart,
        sound_waluigi_specialairlw,
        sound_waluigi_speciallwshield,
        sound_waluigi_speciallwattack1,
        sound_waluigi_speciallwattack2,
        sound_waluigi_speciallwattack3,
        sound_waluigi_speciallwattackspecial1,
        sound_waluigi_speciallwattackspecial2,
        sound_waluigi_speciallwjump,
        sound_waluigi_speciallwattackair,
        sound_waluigi_speciallwspecialair,
        sound_waluigi_speciallwjumpair,
        sound_waluigi_appealhil,
        sound_waluigi_appealhir,
        sound_waluigi_appeallwl,
        sound_waluigi_appeallwr,
        sound_waluigi_entryl,
        sound_waluigi_entryr,
        sound_waluigi_finalstart,
        sound_waluigi_finalairstart
    );
}