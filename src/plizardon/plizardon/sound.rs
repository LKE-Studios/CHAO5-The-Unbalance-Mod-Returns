use crate::imports::BuildImports::*;

//JumpAerialF3
unsafe extern "C" fn sound_plizardon_JumpAerialF3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_plizardon_wing"));
    }
}

//JumpAerialF4
unsafe extern "C" fn sound_plizardon_JumpAerialF4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_plizardon_wing"));
    }
}

//JumpAerialF5
unsafe extern "C" fn sound_plizardon_JumpAerialF5(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_plizardon_wing"));
    }
}

//JumpAerialF6
unsafe extern "C" fn sound_plizardon_JumpAerialF6(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_plizardon_wing"));
    }
}

//GlideStart
unsafe extern "C" fn sound_plizardon_GlideStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_plizardon_wing"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_plizardon_glide_start"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_plizardon_glide_loop"));        
    }
}

//GlideAttack
unsafe extern "C" fn sound_plizardon_GlideAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_plizardon_rnd_attack"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_plizardon_tailswing"));
    }
}

//GlideLanding
unsafe extern "C" fn sound_plizardon_GlideLanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_s"));
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_ss"));
    }
}

//GlideEnd
unsafe extern "C" fn sound_plizardon_GlideEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_plizardon_special_h01_win02"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_plizardon_wing"));
    }
}   

//SpecialAirHi2Start
unsafe extern "C" fn sound_plizardon_SpecialAirHi2Start(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_plizardon_special_h02_01"));
    }
}

//SpecialAirHi2
unsafe extern "C" fn sound_plizardon_SpecialAirHi2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_plizardon_special_h02_02"));
    }
}

//SpecialAirHi2Landing
unsafe extern "C" fn sound_plizardon_SpecialAirHi2Landing(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_plizardon_special_h02_02"));
        PLAY_SE(fighter, Hash40::new("se_plizardon_special_h02_03"));
    }
}

//SpecialZ
pub unsafe extern "C" fn sound_plizardon_SpecialZ(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_plizardon_squat"));
        PLAY_SE(fighter, Hash40::new("vc_plizardon_attack02"));
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_plizardon_special_z01"));//Index 47
    }
}

//SpecialAirZ
pub unsafe extern "C" fn sound_plizardon_SpecialAirZ(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_plizardon_squat"));
        PLAY_SE(fighter, Hash40::new("vc_plizardon_attack02"));
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_plizardon_special_z01"));
    }
}

//Win2
unsafe extern "C" fn sound_plizardon_Win2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_plizardon_jump02"));
        PLAY_SE_NO_3D(fighter, Hash40::new("se_plizardon_special_h01_win02"));
    }
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_plizardon_landing02"));
    }
    frame(fighter.lua_state_agent, 53.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_plizardon_win02"));
    }
    frame(fighter.lua_state_agent, 96.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_plizardon_landing02"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_plizardon_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_plizardon_rnd_futtobi01"), Hash40::new("seq_plizardon_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_plizardon_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_plizardon_rnd_futtobi01"), Hash40::new("seq_plizardon_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_plizardon_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_plizardon_rnd_futtobi01"), Hash40::new("seq_plizardon_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_plizardon_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_plizardon_rnd_futtobi01"), Hash40::new("seq_plizardon_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_plizardon_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_plizardon_rnd_futtobi01"), Hash40::new("seq_plizardon_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("plizardon")
    .sound_acmd("sound_jumpaerialf3", sound_plizardon_JumpAerialF3, Low)
    .sound_acmd("sound_jumpaerialf4", sound_plizardon_JumpAerialF4, Low)
    .sound_acmd("sound_jumpaerialf5", sound_plizardon_JumpAerialF5, Low)
    .sound_acmd("sound_jumpaerialf6", sound_plizardon_JumpAerialF6, Low)
    .sound_acmd("sound_glidestart", sound_plizardon_GlideStart, Low)
    .sound_acmd("sound_glideattack", sound_plizardon_GlideAttack, Low)
    .sound_acmd("sound_glidelanding", sound_plizardon_GlideLanding, Low)
    .sound_acmd("sound_glideend", sound_plizardon_GlideEnd, Low)
    .sound_acmd("sound_specialairhi2start", sound_plizardon_SpecialAirHi2Start, Low)
    .sound_acmd("sound_specialairhi2", sound_plizardon_SpecialAirHi2, Low)
    .sound_acmd("sound_specialairhi2landing", sound_plizardon_SpecialAirHi2Landing, Low)
    .sound_acmd("sound_specialz", sound_plizardon_SpecialZ, Low)
    .sound_acmd("sound_specialairz", sound_plizardon_SpecialAirZ, Low)
    .sound_acmd("sound_specialwin2", sound_plizardon_Win2, Low)
    .sound_acmd("sound_damageflyhi", sound_plizardon_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_plizardon_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_plizardon_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_plizardon_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_plizardon_DamageFlyRoll, Low)
    .install();
}