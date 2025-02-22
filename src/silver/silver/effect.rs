use crate::imports::BuildImports::*;
use crate::silver::silver::frame::*;

pub static mut BASE :  Vector3f =  Vector3f { x: 0.0, y: 2.8, z: -7.0 };
pub static mut BASEDOWN :  Vector3f =  Vector3f { x: 0.0, y: 4.8, z: -7.0 };
pub static mut BASE_TRAIL :  Vector3f =  Vector3f { x: 0.0, y: 4.8, z: -16.0 };
pub static mut BASE_TRAIL2 :  Vector3f =  Vector3f { x: 0.0, y: 4.8, z: -25.0 };
pub static mut BASE_TRAIL3 :  Vector3f =  Vector3f { x: 0.0, y: 4.8, z: -34.0 };
pub static mut BASE_TRAIL4 :  Vector3f =  Vector3f { x: 0.0, y: 4.8, z: -43.0 };
pub static mut BASE_TRAILUP :  Vector3f =  Vector3f { x: 0.0, y: -2.7, z: -16.0 };
pub static mut BASE_TRAILUP2 :  Vector3f =  Vector3f { x: 0.0, y: -8.2, z: -25.0 };
pub static mut BASE_TRAILUP3 :  Vector3f =  Vector3f { x: 0.0, y: -13.7, z: -34.0 };
pub static mut BASE_TRAILUP4 :  Vector3f =  Vector3f { x: 0.0, y: -19.2, z: -43.0 };
pub static mut BASE_TRAILDOWN :  Vector3f =  Vector3f { x: 0.0, y: 8.3, z: -16.0 };
pub static mut BASE_TRAILDOWN2 :  Vector3f =  Vector3f { x: 0.0, y: 13.8, z: -25.0 };
pub static mut BASE_TRAILDOWN3 :  Vector3f =  Vector3f { x: 0.0, y: 19.3, z: -34.0 };
pub static mut BASE_TRAILDOWN4 :  Vector3f =  Vector3f { x: 0.0, y: 24.8, z: -43.0 };
pub static mut ROT :  Vector3f =  Vector3f { x: 90.0, y: 0.0, z: 0.0 };
pub static mut ROTUP :  Vector3f =  Vector3f { x: 90.0, y: 0.0, z: 30.0 };
pub static mut ROTDOWN :  Vector3f =  Vector3f { x: 90.0, y: 0.0, z: -30.0 };
pub static mut COUNT : [i32; 8] = [0; 8];
pub static mut SPECIAL_N_WALL : [bool; 8] = [true; 8];

//Attack11
unsafe extern "C" fn effect_silver_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_attack_c"), Hash40::new("mewtwo_pk_attack_c"), Hash40::new("top"), 0, 7, 10, 0, 0, 0, 0.45, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
    }
}

//AttackDash
unsafe extern "C" fn effect_silver_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 8.0, 3.5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("haver"), 5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
    }
}

//AttackS3Hi
unsafe extern "C" fn effect_silver_AttackS3Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 8.0, -3.0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
    }
}

//AttackS3 
unsafe extern "C" fn effect_silver_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 8.0, -3.0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
    }
}

//AttackS3Lw 
unsafe extern "C" fn effect_silver_AttackS3Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 8.0, -3.0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
    }
}

//AttackHi3
unsafe extern "C" fn effect_silver_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 11.5, 0, 7, 90, 92, 1.0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
    }
}

//AttackLw3
unsafe extern "C" fn effect_silver_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5.5, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_attack_d"), Hash40::new("mewtwo_pk_attack_d"), Hash40::new("top"), 0, 2, 12, 0, 0, 0, 0.25, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
    }
}

//AttackS4Hi
unsafe extern "C" fn effect_silver_AttackS4Hi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, 12, 0, 0, 0, 0.9, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.6, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_shadowball_hold"), false, false);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.3, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.6, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_shadowball_hold"), false, false);
        EFFECT(fighter, Hash40::new("mewtwo_shadowball_bomb"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
    }
}

//AttackS4Charge 
unsafe extern "C" fn effect_silver_AttackS4Charge(fighter: &mut L2CAgentBase) {
    for _ in 0..25 {
        frame(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 1, 0, 1.3, 0, 0, 0, 0.25, true, *EF_FLIP_YZ);
        }
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, 7, 7, 7, 0, 0, 0, true, *EF_FLIP_YZ);
    }
}

//AttackHi4Charge
unsafe extern "C" fn effect_silver_AttackHi4Charge(fighter: &mut L2CAgentBase) {
    for _ in 0..25 {
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 10, 0, 5, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, 5, 5, 5, 0, 0, 0, true, *EF_FLIP_YZ);
    }
}

//AttackLw4Charge
unsafe extern "C" fn effect_silver_AttackLw4Charge(fighter: &mut L2CAgentBase) {
    for _ in 0..25 {
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 4, 0, 10, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 5.0);
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("havel"), 0, 0, 2, 0, 0, 0, 1, 10, 10, 10, 0, 0, 0, true, *EF_FLIP_YZ);
    }
}

//AttackS4
unsafe extern "C" fn effect_silver_AttackS4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, 12, 0, 0, 0, 0.9, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.6, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_shadowball_hold"), false, false);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.3, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.6, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_shadowball_hold"), false, false);
        EFFECT(fighter, Hash40::new("mewtwo_shadowball_bomb"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
    }
}

//AttackS4Lw
unsafe extern "C" fn effect_silver_AttackS4Lw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, 12, 0, 0, 0, 0.9, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.6, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_shadowball_hold"), false, false);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.3, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.6, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_shadowball_hold"), false, false);
        EFFECT(fighter, Hash40::new("mewtwo_shadowball_bomb"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
    }
}

//AttackHi4
unsafe extern "C" fn effect_silver_AttackHi4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 14, 11, 0, 0, 0, 0.9, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 16, 0, 0, 30, 90, 1.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 16, 0, 0, 30, 90, 1.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 16, 0, 0, 110, 90, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
    frame(fighter.lua_state_agent, 53.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackLw4
unsafe extern "C" fn effect_silver_AttackLw4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_attack_c"), Hash40::new("mewtwo_pk_attack_c"), Hash40::new("top"), 0, 7, 12, 0, 0, 0, 0.45, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_attack_c"), Hash40::new("mewtwo_pk_attack_c"), Hash40::new("top"), 0, 7, -12, 0, 0, 0, 0.45, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
    }
}

//AttackAirN 
unsafe extern "C" fn effect_silver_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_nenriki"), Hash40::new("throw"), 0, 0, 0, 0, 90, 0, 0.35, true);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.85, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.85, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_nenriki"), false, false);
    }
}

//AttackAirF 
unsafe extern "C" fn effect_silver_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.74, true);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("mewtwo_shadowball_bomb"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_shadowball_hold"), false, false);
    }
}

//LandingAirF 
unsafe extern "C" fn effect_silver_LandingAirF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("mewtwo_shadowball_bomb"), Hash40::new("top"), 10, 3, 0, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 0, true);
    }
}

//AttackAirB 
unsafe extern "C" fn effect_silver_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_attack_d"), Hash40::new("mewtwo_pk_attack_d"), Hash40::new("havel"), -6, 0, 0.5, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
    }
}

//AttackAirHi 
unsafe extern "C" fn effect_silver_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 17.5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 3, 12, -4.5, 210, -11, 245, 0.5, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 1.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 3, 12, 4.5, -45, -11, -95, 0.5, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 1.0);
    }
}

//AttackAirLw 
unsafe extern "C" fn effect_silver_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true, *EF_FLIP_NONE);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true, *EF_FLIP_NONE);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("mewtwo_pk_attack_b"), Hash40::new("mewtwo_pk_attack_b"), Hash40::new("top"), 3, 0, 0, 90, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.75);
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
    }
}

//CatchWait 
unsafe extern "C" fn effect_silver_CatchWait(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
}

//CatchAttack 
unsafe extern "C" fn effect_silver_CatchAttack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
}

//CatchPull 
unsafe extern "C" fn effect_silver_CatchPull(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
}

//ThrowF 
unsafe extern "C" fn effect_silver_ThrowF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.45, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
        EFFECT_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 10.0, -1.5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("handl"), 2.0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
    }
    frame(fighter.lua_state_agent, 63.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
    }
}

//ThrowB 
unsafe extern "C" fn effect_silver_ThrowB(fighter: &mut L2CAgentBase) { 
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 52.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
    }
    frame(fighter.lua_state_agent, 68.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
    }
}

//ThrowHi 
unsafe extern "C" fn effect_silver_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.85);
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
    }
}

//ThrowLw 
unsafe extern "C" fn effect_silver_ThrowLw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.45, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_damage_paralysis"), Hash40::new("top"), 0, 2.5, 12.0, 0, 0, 0, 0.85, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 2.9);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0, 0, 12.0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.8, 0.9);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 52.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
    }
    frame(fighter.lua_state_agent, 73.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
    }
}

//CliffAttack 
unsafe extern "C" fn effect_silver_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -3, 13.5, -1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 7.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

//DownAttackU
unsafe extern "C" fn effect_silver_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -1, 5, 8, 185, 150, 5, 1.1, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_d"), false, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 5, -6.5, 185, -20, 8, 1.1, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_d"), false, false);
    }
}

//DownAttackD
unsafe extern "C" fn effect_silver_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -1, 5, 8, 185, 150, 5, 1.1, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_d"), false, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 5, -6.5, 185, -20, 8, 1.1, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_d"), false, false);
    }
}

//SpecialNShoot 
unsafe extern "C" fn effect_silver_SpecialNShoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_shadowball_max_hand"), false, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_passive"), Hash40::new("sys_passive"), Hash40::new("top"), 0, 12, 9, 0, 0, 0, 0.23, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.81);
        LAST_EFFECT_SET_COLOR(fighter, 0.8, 1.0, 1.0);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.45, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 47.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
    }
    frame(fighter.lua_state_agent, 64.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialAirNShoot 
unsafe extern "C" fn effect_silver_SpecialAirNShoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_shadowball_max_hand"), false, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_passive"), Hash40::new("sys_passive"), Hash40::new("top"), 0, 12, 9, 0, 0, 0, 0.23, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        LAST_EFFECT_SET_COLOR(fighter, 0.8, 1.0, 1.0);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.45, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 47.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
    }
}

//SpecialSStart 
unsafe extern "C" fn effect_silver_SpecialSStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, 8, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, -4.5, 0, 0, 0, 0.55, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 5, -3, 0, 0, 0, 0.4, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
}

//SpecialS 
unsafe extern "C" fn effect_silver_SpecialS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.0, 1.0, 1.0, 0.65);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_teamhealfield"), Hash40::new("top"), 0, 0, 15, 0, 0, 0, 1.25, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.45, 2.55);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_assist_out"), Hash40::new("throw"), 0, -2, 0, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 68.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 2.0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spirits_up_s"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 69.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
}

//SpecialAirSStart 
unsafe extern "C" fn effect_silver_SpecialAirSStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, 8, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, -4.5, 0, 0, 0, 0.55, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 5, -3, 0, 0, 0, 0.4, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
}

//SpecialAirS 
unsafe extern "C" fn effect_silver_SpecialAirS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.0, 1.0, 1.0, 0.65);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_teamhealfield"), Hash40::new("top"), 0, 0, 15, 0, 0, 0, 1.25, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.45, 2.55);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_assist_out"), Hash40::new("throw"), 0, -2, 0, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 68.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 2.0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spirits_up_s"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 69.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
}

//SpecialHiStart 
unsafe extern "C" fn effect_silver_SpecialHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent,4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_just_shield"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1, true);
    }
}

//SpecialHi 
unsafe extern "C" fn effect_silver_SpecialHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.0, 1.0, 1.0, 0.45);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_just_shield_hit"), Hash40::new("top"), 0, 14, 0, 0, 0, 0, 0.8, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_just_shield_hit"), Hash40::new("top"), 0, 14, 0, 90, 0, 0, 0.8, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_just_shield_hit"), Hash40::new("top"), 0, 14, 0, 180, 0, 0, 0.8, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_vector"), Hash40::new("top"), 0, 26, 0, 90, 0, 0, 4.0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 85.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_vector"), false, false);
    }
}

//SpecialAirHiStart 
unsafe extern "C" fn effect_silver_SpecialAirHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent,4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_just_shield"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1, true);
    }
}

//SpecialAirHi 
unsafe extern "C" fn effect_silver_SpecialAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.0, 1.0, 1.0, 0.45);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_just_shield_hit"), Hash40::new("top"), 0, 14, 0, 0, 0, 0, 0.8, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_just_shield_hit"), Hash40::new("top"), 0, 14, 0, 90, 0, 0, 0.8, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_just_shield_hit"), Hash40::new("top"), 0, 14, 0, 180, 0, 0, 0.8, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_vector"), Hash40::new("top"), 0, 26, 0, 90, 0, 0, 4.0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 85.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_vector"), false, false);
    }
}

//SpecialLw 
unsafe extern "C" fn effect_silver_SpecialLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.4, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.0, 1.0, 1.0, 0.65);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_nenriki"), Hash40::new("top"), 0, 9, 0, 0, 90, 0, 0.55, true);
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
}

//SpecialAirLw 
unsafe extern "C" fn effect_silver_SpecialAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.4, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.0, 1.0, 1.0, 0.65);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_nenriki"), Hash40::new("top"), 0, 9, 0, 0, 90, 0, 0.55, true);
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
}

//Dash
unsafe extern "C" fn effect_silver_Dash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//Run 
unsafe extern "C" fn effect_silver_Run(fighter: &mut L2CAgentBase) {
    for _ in 0..35 {
        wait(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

//RunBrakeR 
unsafe extern "C" fn effect_silver_RunBrakeR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 9, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 8.0);
}

//RunBrakeL 
unsafe extern "C" fn effect_silver_RunBrakeL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 9, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 8.0);
}

//TurnRun 
unsafe extern "C" fn effect_silver_TurnRun(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
    }
}

//EscapeAirSlide 
unsafe extern "C" fn effect_silver_EscapeAirSlide(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    STICK_DIRECTION[ENTRY_ID] = ControlModule::get_stick_dir(fighter.module_accessor) * (180.0 / PI);
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        if stick_x >= -0.2 && stick_x <= 0.2 && 
        stick_y >= -0.2 && stick_y <= 0.2 {
            STICK_DIRECTION[ENTRY_ID] = 361.0;
        } else if STICK_DIRECTION[ENTRY_ID] <= -67.5 {
            STICK_DIRECTION[ENTRY_ID] *= -1.0;
        };
        if STICK_DIRECTION[ENTRY_ID] >= -67.5 && STICK_DIRECTION[ENTRY_ID] < -22.5 && stick_x < 0.0 {
            ESCAPE_AIR_DIR[ENTRY_ID] = 1;
        }
        else if STICK_DIRECTION[ENTRY_ID] >= 67.5 && STICK_DIRECTION[ENTRY_ID] <= 90.0 && stick_y < 0.0 {
            ESCAPE_AIR_DIR[ENTRY_ID] = 2;
        }
        else if STICK_DIRECTION[ENTRY_ID] >= -67.5 && STICK_DIRECTION[ENTRY_ID] < -22.5 && stick_x > 0.0 {
            ESCAPE_AIR_DIR[ENTRY_ID] = 3;
        }
        else if STICK_DIRECTION[ENTRY_ID] >= -22.5 && STICK_DIRECTION[ENTRY_ID] <= 22.5 && stick_x < 0.0 {
            ESCAPE_AIR_DIR[ENTRY_ID] = 4;
        }
        else if STICK_DIRECTION[ENTRY_ID] == 361.0 {
            ESCAPE_AIR_DIR[ENTRY_ID] = 5;
        }
        else if STICK_DIRECTION[ENTRY_ID] >= -22.5 && STICK_DIRECTION[ENTRY_ID] <= 22.5 && stick_x > 0.0 {
            ESCAPE_AIR_DIR[ENTRY_ID] = 6;
        }
        else if STICK_DIRECTION[ENTRY_ID] > 22.5 && STICK_DIRECTION[ENTRY_ID] <= 67.5 && stick_x < 0.0 {
            ESCAPE_AIR_DIR[ENTRY_ID] = 7;
        }
        else if STICK_DIRECTION[ENTRY_ID] > 67.5 && STICK_DIRECTION[ENTRY_ID] <= 90.0 && stick_y > 0.0 {
            ESCAPE_AIR_DIR[ENTRY_ID] = 8;
        }
        else  {
            ESCAPE_AIR_DIR[ENTRY_ID] = 9;
        };
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.0, 1.0, 1.0, 0.65);
        if ESCAPE_AIR_DIR[ENTRY_ID] == 1 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 5.0, -2.0, -45, 0, 0, 1.5, true);
            LAST_EFFECT_SET_RATE(fighter, 0.7);
            LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        }
        else if ESCAPE_AIR_DIR[ENTRY_ID] == 2 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 28.0, 0.0, 90, 0, 0, 1.5, true);
            LAST_EFFECT_SET_RATE(fighter, 0.7);
            LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        }
        else if ESCAPE_AIR_DIR[ENTRY_ID] == 3 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 23, -17.0, 45, 0, 0, 1.5, true);
            LAST_EFFECT_SET_RATE(fighter, 0.7);
            LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        }
        else if ESCAPE_AIR_DIR[ENTRY_ID] == 4 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 9, 24.0, 0, 180, 0, 1.5, true);
            LAST_EFFECT_SET_RATE(fighter, 0.7);
            LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        }
        else if ESCAPE_AIR_DIR[ENTRY_ID] == 6 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 9, -24.0, 0, 0, 0, 1.5, true);
            LAST_EFFECT_SET_RATE(fighter, 0.7);
            LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        }
        else if ESCAPE_AIR_DIR[ENTRY_ID] == 7 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 8.0, -1.0, 45, 0, 0, 1.5, true);
            LAST_EFFECT_SET_RATE(fighter, 0.7);
            LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        }
        else if ESCAPE_AIR_DIR[ENTRY_ID] == 8 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, -12.0, 0.0, 270, 0, 0, 1.5, true);
            LAST_EFFECT_SET_RATE(fighter, 0.7);
            LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        }
        else if ESCAPE_AIR_DIR[ENTRY_ID] == 9 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, -7.0, -17.0, -45, 0, 0, 1.5, true);
            LAST_EFFECT_SET_RATE(fighter, 0.7);
            LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        }
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
}

//EscapeN 
unsafe extern "C" fn effect_silver_EscapeN(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
}

//EscapeF 
unsafe extern "C" fn effect_silver_EscapeF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FLASH(fighter, 0.0, 1.0, 1.0, 0.65);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 10.0, -24.0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
}

//EscapeB 
unsafe extern "C" fn effect_silver_EscapeB(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FLASH(fighter, 0.0, 1.0, 1.0, 0.65);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 10.0, 24.0, 0, 180, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
}

//Wait2 
unsafe extern "C" fn effect_silver_Wait2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 185.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.36, 0, 0, 0, 0, 0, 0, false);
    }
}

//Wait3 
unsafe extern "C" fn effect_silver_Wait3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 185.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.36, 0, 0, 0, 0, 0, 0, false);
    }
}

//AppealHiL
unsafe extern "C" fn effect_silver_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(fighter.lua_state_agent, 165.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

//AppealHiR 
unsafe extern "C" fn effect_silver_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(fighter.lua_state_agent, 165.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

//AppealSL 
unsafe extern "C" fn effect_silver_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(fighter.lua_state_agent, 117.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

//AppealSR 
unsafe extern "C" fn effect_silver_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(fighter.lua_state_agent, 117.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

//AppealLwL 
unsafe extern "C" fn effect_silver_AppealLwL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 59.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_appeal_aura"), Hash40::new("top"), 0, -1.5, 0, 0, 0, 0, 0.8, true);
    }
    frame(fighter.lua_state_agent, 157.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

//AppealLwR 
unsafe extern "C" fn effect_silver_AppealLwR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 59.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_appeal_aura"), Hash40::new("top"), 0, -1.5, 0, 0, 0, 0, 0.8, true);
    }
    frame(fighter.lua_state_agent, 157.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

//Win1 
unsafe extern "C" fn effect_silver_Win1(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_entry"), Hash40::new("hip"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(fighter.lua_state_agent, 128.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, true);
    }
}

//WinWait1 
unsafe extern "C" fn effect_silver_Win1Wait(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, true);
    }
}

//Win3 
unsafe extern "C" fn effect_silver_Win3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 115.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, true);
    }
}

//Win3Wait 
unsafe extern "C" fn effect_silver_Win3Wait(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, true);
    }
}

//Final
unsafe extern "C" fn effect_silver_Final(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bg_black"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

//FinalAir 
unsafe extern "C" fn effect_silver_FinalAir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bg_black"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

//EntryL
unsafe extern "C" fn effect_silver_EntryL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_entry"), Hash40::new("hip"), 0, 0, 0, 0, 0, -90, 1, true);
        FLASH(fighter, 1, 1, 1, 1);
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 3, 0.8, 0, 1, 0);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
}

//EntryR
unsafe extern "C" fn effect_silver_EntryR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_entry"), Hash40::new("hip"), 0, 0, 0, 0, 0, -90, 1, true);
        FLASH(fighter, 1, 1, 1, 1);
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 3, 0.8, 0, 1, 0);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
}

//WalkSlow 
unsafe extern "C" fn effect_silver_WalkSlow(fighter: &mut L2CAgentBase) {
    loop {
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("footl"), 3, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 30.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("footr"), 3, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        }
        wait_loop_clear(fighter);
    }
}

//WalkMiddle 
unsafe extern "C" fn effect_silver_WalkMiddle(fighter: &mut L2CAgentBase) {
    loop {
        frame(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("footl"), 3, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 18.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("footr"), 4, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        wait_loop_clear(fighter);
    }
}

//WalkFast 
unsafe extern "C" fn effect_silver_WalkFast(fighter: &mut L2CAgentBase) {
    loop {
        frame(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("footl"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 12.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("footr"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait_loop_clear(fighter);
    }
}

pub fn install() {
    Agent::new("mewtwo")
    .effect_acmd("effect_attack11_silver", effect_silver_Attack11, Low)
    .effect_acmd("effect_attackdash_silver", effect_silver_AttackDash, Low)
    .effect_acmd("effect_attacks3hi_silver", effect_silver_AttackS3Hi, Low)
    .effect_acmd("effect_attacks3_silver", effect_silver_AttackS3, Low)
    .effect_acmd("effect_attacks3lw_silver", effect_silver_AttackS3Lw, Low)
    .effect_acmd("effect_attackhi3_silver", effect_silver_AttackHi3, Low)
    .effect_acmd("effect_attacklw3_silver", effect_silver_AttackLw3, Low)
    .effect_acmd("effect_attacks4charge_silver", effect_silver_AttackS4Charge, Low)
    .effect_acmd("effect_attackhi4charge_silver", effect_silver_AttackHi4Charge, Low)
    .effect_acmd("effect_attacklw4charge_silver", effect_silver_AttackLw4Charge, Low)
    .effect_acmd("effect_attacks4hi_silver", effect_silver_AttackS4Hi, Low)
    .effect_acmd("effect_attacks4_silver", effect_silver_AttackS4, Low)
    .effect_acmd("effect_attacks4lw_silver", effect_silver_AttackS4Lw, Low)
    .effect_acmd("effect_attackhi4_silver", effect_silver_AttackHi4, Low)
    .effect_acmd("effect_attacklw4_silver", effect_silver_AttackLw4, Low)
    .effect_acmd("effect_attackairn_silver", effect_silver_AttackAirN, Low)
    .effect_acmd("effect_attackairf_silver", effect_silver_AttackAirF, Low) 
    .effect_acmd("effect_landingairf_silver", effect_silver_LandingAirF, Low)    
    .effect_acmd("effect_attackairb_silver", effect_silver_AttackAirB, Low)
    .effect_acmd("effect_attackairhi_silver", effect_silver_AttackAirHi, Low)
    .effect_acmd("effect_attackairlw_silver", effect_silver_AttackAirLw, Low)
    .effect_acmd("effect_catchwait_silver", effect_silver_CatchWait, Low)
    .effect_acmd("effect_catchattack_silver", effect_silver_CatchAttack, Low)
    .effect_acmd("effect_catchpull_silver", effect_silver_CatchPull, Low)
    .effect_acmd("effect_throwf_silver", effect_silver_ThrowF, Low)
    .effect_acmd("effect_throwb_silver", effect_silver_ThrowB, Low)
    .effect_acmd("effect_throwhi_silver", effect_silver_ThrowHi, Low)
    .effect_acmd("effect_throwlw_silver", effect_silver_ThrowLw, Low)
    .effect_acmd("effect_downattackd_silver", effect_silver_DownAttackD, Low)
    .effect_acmd("effect_downattacku_silver", effect_silver_DownAttackU, Low)
    .effect_acmd("effect_cliffattack_silver", effect_silver_CliffAttack, Low)
    .effect_acmd("effect_specialnshoot_silver", effect_silver_SpecialNShoot, Low)
    .effect_acmd("effect_specialairnshoot_silver", effect_silver_SpecialAirNShoot, Low)
    .effect_acmd("effect_specialsstart_silver", effect_silver_SpecialSStart, Low)
    .effect_acmd("effect_specialairsstart_silver", effect_silver_SpecialAirSStart, Low)
    .effect_acmd("effect_specials_silver", effect_silver_SpecialS, Low)
    .effect_acmd("effect_specialairs_silver", effect_silver_SpecialAirS, Low)
    .effect_acmd("effect_specialhistart_silver", effect_silver_SpecialHiStart, Low)
    .effect_acmd("effect_specialairhistart_silver", effect_silver_SpecialAirHiStart, Low)
    .effect_acmd("effect_specialhi_silver", effect_silver_SpecialHi, Low)
    .effect_acmd("effect_specialairhi_silver", effect_silver_SpecialAirHi, Low)
    .effect_acmd("effect_speciallw_silver", effect_silver_SpecialLw, Low)
    .effect_acmd("effect_specialairlw_silver", effect_silver_SpecialAirLw, Low)
    .effect_acmd("effect_entryl_silver", effect_silver_EntryL, Low)
    .effect_acmd("effect_entryr_silver", effect_silver_EntryR, Low)
    .effect_acmd("effect_dash_silver", effect_silver_Dash, Low)
    .effect_acmd("effect_run_silver", effect_silver_Run, Low)
    .effect_acmd("effect_walkslow_silver", effect_silver_WalkSlow, Low)
    .effect_acmd("effect_walkmiddle_silver", effect_silver_WalkMiddle, Low)
    .effect_acmd("effect_walkfast_silver", effect_silver_WalkFast, Low)
    .effect_acmd("effect_runbrakel_silver", effect_silver_RunBrakeL, Low)
    .effect_acmd("effect_runbraker_silver", effect_silver_RunBrakeR, Low)
    .effect_acmd("effect_turnrun_silver", effect_silver_TurnRun, Low)
    .effect_acmd("effect_escapeairslide_silver", effect_silver_EscapeAirSlide, Low)
    .effect_acmd("effect_escapen_silver", effect_silver_EscapeN, Low)
    .effect_acmd("effect_escapef_silver", effect_silver_EscapeF, Low)
    .effect_acmd("effect_escapeb_silver", effect_silver_EscapeB, Low)
    .effect_acmd("effect_wait2_silver", effect_silver_Wait2, Low)
    .effect_acmd("effect_wait3_silver", effect_silver_Wait3, Low)
    .effect_acmd("effect_win1_silver", effect_silver_Win1, Low)
    .effect_acmd("effect_win1wait_silver", effect_silver_Win1Wait, Low)
    .effect_acmd("effect_appealsr_silver", effect_silver_AppealSR, Low)
    .effect_acmd("effect_appealsl_silver", effect_silver_AppealSL, Low)
    .effect_acmd("effect_appealhir_silver", effect_silver_AppealHiR, Low)
    .effect_acmd("effect_appealhil_silver", effect_silver_AppealHiL, Low)
    .effect_acmd("effect_appeallwr_silver", effect_silver_AppealLwR, Low)
    .effect_acmd("effect_appeallwl_silver", effect_silver_AppealLwL, Low)
    //.effect_acmd("effect_final_silver", effect_silver_Final, Low)
    //.effect_acmd("effect_finalair_silver", effect_silver_FinalAir, Low)
    .install();
}