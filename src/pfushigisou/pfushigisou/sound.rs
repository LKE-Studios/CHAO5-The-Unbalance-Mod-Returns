use crate::imports::BuildImports::*;

//SpecialZStart
unsafe extern "C" fn sound_pfushigisou_SpecialZStart(fighter: &mut L2CAgentBase) {
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

//SpecialZCharge
unsafe extern "C" fn sound_pfushigisou_SpecialZCharge(fighter: &mut L2CAgentBase) {
    loop {
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_appeal_l02"));
            if DamageModule::damage(fighter.module_accessor, 0) > 0.0 {
                PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
            }
        }
        wait(fighter.lua_state_agent, 15.0);
    }
}

//SpecialZShoot
unsafe extern "C" fn sound_pfushigisou_SpecialZShoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_special_beam"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_smash_h02"));
    }
}

//SpecialZEnd
unsafe extern "C" fn sound_pfushigisou_SpecialZEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pfushigisou_appeal_l01"));
        STOP_SE(fighter, Hash40::new("se_pfushigisou_recovery"));
    }
}

//SpecialAirZStart
unsafe extern "C" fn sound_pfushigisou_SpecialAirZStart(fighter: &mut L2CAgentBase) {
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

//SpecialAirZCharge
unsafe extern "C" fn sound_pfushigisou_SpecialAirZCharge(fighter: &mut L2CAgentBase) {
    loop {
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_appeal_l02"));
            if DamageModule::damage(fighter.module_accessor, 0) > 0.0 {
                PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
            }
        }
        wait(fighter.lua_state_agent, 15.0);
    }
}

//SpecialAirZShoot
unsafe extern "C" fn sound_pfushigisou_SpecialAirZShoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_special_beam"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_smash_h02"));
    }
}

//SpecialAirZEnd
unsafe extern "C" fn sound_pfushigisou_SpecialAirZEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pfushigisou_appeal_l01"));
        STOP_SE(fighter, Hash40::new("se_pfushigisou_recovery"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_pfushigisou_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_pfushigisou_rnd_futtobi01"), Hash40::new("seq_pfushigisou_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_pfushigisou_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_pfushigisou_rnd_futtobi01"), Hash40::new("seq_pfushigisou_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_pfushigisou_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_pfushigisou_rnd_futtobi01"), Hash40::new("seq_pfushigisou_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_pfushigisou_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_pfushigisou_rnd_futtobi01"), Hash40::new("seq_pfushigisou_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_pfushigisou_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_pfushigisou_rnd_futtobi01"), Hash40::new("seq_pfushigisou_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("pfushigisou")
    .sound_acmd("sound_specialzstart", sound_pfushigisou_SpecialZStart, Low)
    .sound_acmd("sound_specialzcharge", sound_pfushigisou_SpecialZCharge, Low)
    .sound_acmd("sound_specialzend", sound_pfushigisou_SpecialZEnd, Low)
    .sound_acmd("sound_specialzshoot", sound_pfushigisou_SpecialZShoot, Low)
    .sound_acmd("sound_specialairzstart", sound_pfushigisou_SpecialAirZStart, Low)
    .sound_acmd("sound_specialairzcharge", sound_pfushigisou_SpecialAirZCharge, Low)
    .sound_acmd("sound_specialairzend", sound_pfushigisou_SpecialAirZEnd, Low)
    .sound_acmd("sound_specialairzshoot", sound_pfushigisou_SpecialAirZShoot, Low)
    .sound_acmd("sound_damageflyhi", sound_pfushigisou_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_pfushigisou_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_pfushigisou_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_pfushigisou_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_pfushigisou_DamageFlyRoll, Low)
    .install();
}
