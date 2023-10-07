use crate::imports::BuildImports::*;
use crate::silver::frame::*;

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

#[acmd_script(//Attack11
    agent = "mewtwo", 
    script = "effect_attack11_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attack11(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackDash
    agent = "mewtwo", 
    script = "effect_attackdash_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attackdash(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackS3Hi
    agent = "mewtwo", 
    script = "effect_attacks3hi_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attacks3hi(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackS3 
    agent = "mewtwo", 
    script = "effect_attacks3_silver",
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attacks3(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackS3Lw 
    agent = "mewtwo", 
    script = "effect_attacks3lw_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attacks3lw(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackHi3
    agent = "mewtwo", 
    script = "effect_attackhi3_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attackhi3(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackLw3
    agent = "mewtwo", 
    script = "effect_attacklw3_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attacklw3(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackS4Hi
    agent = "mewtwo", 
    script = "effect_attacks4hi_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attacks4hi(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackS4Charge 
    agent = "mewtwo", 
    script = "effect_attacks4charge_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attacks4charge(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackHi4Charge
    agent = "mewtwo", 
    script = "effect_attackhi4charge_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attackhi4charge(fighter: &mut L2CAgentBase) {
    for _ in 0..25 {
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 10, 0, 5, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, 5, 5, 5, 0, 0, 0, true, *EF_FLIP_YZ);
    }
}

#[acmd_script(//AttackLw4Charge
    agent = "mewtwo", 
    script = "effect_attacklw4charge_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attacklw4charge(fighter: &mut L2CAgentBase) {
    for _ in 0..25 {
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 4, 0, 10, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 5.0);
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("havel"), 0, 0, 2, 0, 0, 0, 1, 10, 10, 10, 0, 0, 0, true, *EF_FLIP_YZ);
    }
}

#[acmd_script(//AttackS4
    agent = "mewtwo", 
    script = "effect_attacks4_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attacks4(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackS4Lw
    agent = "mewtwo", 
    script = "effect_attacks4lw_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attacks4lw(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackHi4
    agent = "mewtwo", 
    script = "effect_attackhi4_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attackhi4(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackLw4
    agent = "mewtwo", 
    script = "effect_attacklw4_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attacklw4(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackAirN 
    agent = "mewtwo", 
    script = "effect_attackairn_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attackairn(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackAirF 
    agent = "mewtwo", 
    script = "effect_attackairf_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attackairf(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//LandingAirF 
    agent = "mewtwo", 
    script = "effect_landingairf_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_landingairf(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("mewtwo_shadowball_bomb"), Hash40::new("top"), 10, 3, 0, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script(//AttackAirB 
    agent = "mewtwo", 
    script = "effect_attackairb_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attackairb(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackAirHi 
    agent = "mewtwo", 
    script = "effect_attackairhi_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attackairhi(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackAirLw 
    agent = "mewtwo", 
    script = "effect_attackairlw_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_attackairlw(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//CatchWait 
    agent = "mewtwo", 
    script = "effect_catchwait_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_catchwait(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
}

#[acmd_script(//CatchAttack 
    agent = "mewtwo", 
    script = "effect_catchattack_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_catchattack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
}

#[acmd_script(//CatchPull 
    agent = "mewtwo", 
    script = "effect_catchpull_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_catchpull(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
}

#[acmd_script(//ThrowF 
    agent = "mewtwo", 
    script = "effect_throwf_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_throwf(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//ThrowB 
    agent = "mewtwo", 
    script = "effect_throwb_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_throwb(fighter: &mut L2CAgentBase) { 
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

#[acmd_script(//ThrowHi 
    agent = "mewtwo", 
    script = "effect_throwhi_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_throwhi(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//ThrowLw 
    agent = "mewtwo", 
    script = "effect_throwlw_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_throwlw(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//CliffAttack 
    agent = "mewtwo", 
    script = "effect_cliffattack_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_cliffattack(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//DownAttackU 
    agent = "mewtwo", 
    script = "effect_downattacku_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_downattacku(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//DownAttackD
    agent = "mewtwo", 
    script = "effect_downattackd_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_downattackd(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialNShoot 
    agent = "mewtwo", 
    script = "effect_specialnshoot_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_specialnshoot(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialAirNShoot 
    agent = "mewtwo", 
    script = "effect_specialairnshoot_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_specialairnshoot(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//Shoot 
    agent = "mewtwo_shadowball", 
    script = "effect_shoot_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_beam_shoot(fighter: &mut L2CAgentBase) {
    let mut shoot : u32 = 0;
    let mut trail : u32 = 0;
    let mut trail2 : u32 = 0;
    let mut trail3 : u32 = 0;
    let mut trail4 : u32 = 0;
    let shadowball_boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(shadowball_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	let ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(module_accessor);
    COUNT[ENTRY_ID] = 0;
    SPECIAL_N_WALL[ENTRY_ID] = true;
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_shadowball_max_hand"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_shadowball"), true, true);
        shoot = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_hit_cut_shock"), Hash40::new("top"), &BASE, &ROT, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rate(fighter.module_accessor, shoot, 2.4); 
        EffectModule::set_rgb(fighter.module_accessor, shoot, 0.0, 1.0, 1.0);
        let scale = Vector3f { x: 1.0, y: 0.35, z: 0.6};
        EffectModule::set_scale(fighter.module_accessor, shoot, &scale);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_flyroll_aura"), Hash40::new("sys_flyroll_aura"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        LAST_EFFECT_SET_ALPHA(fighter, 0.1);
        LAST_EFFECT_SET_RATE(fighter, 0.25);
    } 
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EffectModule::set_rate(fighter.module_accessor, shoot, 0.0); 
    }
    frame(fighter.lua_state_agent, 7.0);
    for _ in 0..5 {
        if COUNT[ENTRY_ID] == 0 {
            trail = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_hit_cut_shock"), Hash40::new("top"), &BASE_TRAIL, &ROT, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rate(fighter.module_accessor, trail, 2.4); 
            EffectModule::set_alpha(fighter.module_accessor, trail, 0.8); 
            EffectModule::set_rgb(fighter.module_accessor, trail, 0.2, 1.0, 1.0);
            let scale2 = Vector3f { x: 0.9, y: 0.35, z: 0.55};
            EffectModule::set_scale(fighter.module_accessor, trail, &scale2);
            wait(fighter.lua_state_agent, 4.0);
            if is_excute(fighter) {
                EffectModule::set_rate(fighter.module_accessor, trail, 0.0); 
                COUNT[ENTRY_ID] += 1;
            }
            wait(fighter.lua_state_agent, 6.0);
        }
        else if COUNT[ENTRY_ID] == 1 {
            trail2 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_hit_cut_shock"), Hash40::new("top"), &BASE_TRAIL2, &ROT, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rate(fighter.module_accessor, trail2, 2.4); 
            EffectModule::set_alpha(fighter.module_accessor, trail2, 0.6); 
            EffectModule::set_rgb(fighter.module_accessor, trail2, 0.4, 1.0, 1.0);
            let scale3 = Vector3f { x: 0.85, y: 0.35, z: 0.5};
            EffectModule::set_scale(fighter.module_accessor, trail2, &scale3);

            wait(fighter.lua_state_agent, 4.0);
            if is_excute(fighter) {
                EffectModule::set_rate(fighter.module_accessor, trail2, 0.0); 
                COUNT[ENTRY_ID] += 1;
            }
            wait(fighter.lua_state_agent, 6.0);
        }
        else if COUNT[ENTRY_ID] == 2 {
            trail3 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_hit_cut_shock"), Hash40::new("top"), &BASE_TRAIL3, &ROT, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rate(fighter.module_accessor, trail3, 2.4); 
            EffectModule::set_alpha(fighter.module_accessor, trail3, 0.4); 
            EffectModule::set_rgb(fighter.module_accessor, trail3, 0.6, 1.0, 1.0);
            let scale4 = Vector3f { x: 0.8, y: 0.35, z: 0.45};
            EffectModule::set_scale(fighter.module_accessor, trail3, &scale4);

            wait(fighter.lua_state_agent, 4.0);
            if is_excute(fighter) {
                EffectModule::set_rate(fighter.module_accessor, trail3, 0.0); 
                COUNT[ENTRY_ID] += 1;
            }
            wait(fighter.lua_state_agent, 6.0);
        }
        else if COUNT[ENTRY_ID] == 3 {
            trail4 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_hit_cut_shock"), Hash40::new("top"), &BASE_TRAIL4, &ROT, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rate(fighter.module_accessor, trail4, 2.4); 
            EffectModule::set_alpha(fighter.module_accessor, trail4, 0.2); 
            EffectModule::set_rgb(fighter.module_accessor, trail4, 0.8, 1.0, 1.0);
            let scale5 = Vector3f { x: 0.75, y: 0.35, z: 0.4};
            EffectModule::set_scale(fighter.module_accessor, trail4, &scale5);
            wait(fighter.lua_state_agent, 4.0);
            if is_excute(fighter) {
                EffectModule::set_rate(fighter.module_accessor, trail4, 0.0); 
                COUNT[ENTRY_ID] += 1;
            }
            wait(fighter.lua_state_agent, 6.0);
        }
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        SPECIAL_N_WALL[ENTRY_ID] = false;
        EFFECT(fighter, Hash40::new("mewtwo_shadowball_bomb"), Hash40::new("top"), 0, 0, -1, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        let scale = Vector3f { x: 0.001, y: 0.001, z: 0.001};
        EffectModule::set_scale(fighter.module_accessor, shoot, &scale);
        EffectModule::set_scale(fighter.module_accessor, trail, &scale);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_flyroll_aura"), true, true);
        let scale = Vector3f { x: 0.001, y: 0.001, z: 0.001};
        EffectModule::set_scale(fighter.module_accessor, trail2, &scale);
    }
    frame(fighter.lua_state_agent, 53.0);
    if is_excute(fighter) {
        let scale = Vector3f { x: 0.001, y: 0.001, z: 0.001};
        EffectModule::set_scale(fighter.module_accessor, trail3, &scale);
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        let scale = Vector3f { x: 0.001, y: 0.001, z: 0.001};
        EffectModule::set_scale(fighter.module_accessor, trail4, &scale);
    }
}

#[acmd_script(//HitGround 
    agent = "mewtwo_shadowball", 
    script = "effect_hitground_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_beam_hitground(fighter: &mut L2CAgentBase) {
    let shadowball_boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(shadowball_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	let ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_shadowball"), true, true);
        if SPECIAL_N_WALL[ENTRY_ID] {
            EFFECT(fighter, Hash40::new("mewtwo_shadowball_bomb"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

#[acmd_script(//SpecialSStart 
    agent = "mewtwo", 
    script = "effect_specialsstart_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_specialsstart(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialS 
    agent = "mewtwo", 
    script = "effect_specials_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_specials(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialAirSStart 
    agent = "mewtwo", 
    script = "effect_specialairsstart_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_specialairsstart(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialAirS 
    agent = "mewtwo", 
    script = "effect_specialairs_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_specialairs(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialHiStart 
    agent = "mewtwo", 
    script = "effect_specialhistart_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_specialhistart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent,4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_just_shield"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1, true);
    }
}

#[acmd_script(//SpecialHi 
    agent = "mewtwo", 
    script = "effect_specialhi_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_specialhi(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialLw 
    agent = "mewtwo", 
    script = "effect_speciallw_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_speciallw(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialAirLw 
    agent = "mewtwo", 
    script = "effect_specialairlw_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_specialairlw(fighter: &mut L2CAgentBase) {
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

//Misc Scripts
#[acmd_script(//Dash 
    agent = "mewtwo", 
    script = "effect_dash_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_dash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//Run 
    agent = "mewtwo", 
    script = "effect_run_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_run(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//RunBrakeR 
    agent = "mewtwo", 
    script = "effect_runbraker_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_runbraker(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 9, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 8.0);
}

#[acmd_script(//RunBrakeL 
    agent = "mewtwo", 
    script = "effect_runbrakel_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_runbrakel(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 9, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 8.0);
}

#[acmd_script(//TurnRun 
    agent = "mewtwo", 
    script = "effect_turnrun_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_turnrun(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "mewtwo", script = "effect_escapeairslidesilver", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_silver_escapeairslide(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//EscapeN 
    agent = "mewtwo", 
    script = "effect_escapen_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_escapen(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//EscapeF 
    agent = "mewtwo", 
    script = "effect_escapef_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_escapef(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//EscapeB 
    agent = "mewtwo", 
    script = "effect_escapeb_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_escapeb(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//Wait2 
    agent = "mewtwo", 
    script = "effect_wait2_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_wait2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 185.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.36, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//Wait3 
    agent = "mewtwo", 
    script = "effect_wait3_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_wait3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 185.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.36, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//AppealHiL
    agent = "mewtwo", 
    script = "effect_appealhil_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_appealhil(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AppealHiR 
    agent = "mewtwo", 
    script = "effect_appealhir_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_appealhir(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AppealSL 
    agent = "mewtwo", 
    script = "effect_appealsl_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_appealsl(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(fighter.lua_state_agent, 117.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//AppealSR 
    agent = "mewtwo", 
    script = "effect_appealsr_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_appealsr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(fighter.lua_state_agent, 117.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//AppealLwL 
    agent = "mewtwo", 
    script = "effect_appeallwl_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_appeallwl(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 59.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_appeal_aura"), Hash40::new("top"), 0, -1.5, 0, 0, 0, 0, 0.8, true);
    }
    frame(fighter.lua_state_agent, 157.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//AppealLwR 
    agent = "mewtwo", 
    script = "effect_appeallwr_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_appeallwr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 59.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_appeal_aura"), Hash40::new("top"), 0, -1.5, 0, 0, 0, 0, 0.8, true);
    }
    frame(fighter.lua_state_agent, 157.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//Win1 
    agent = "mewtwo", 
    script = "effect_win1_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_win1(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_entry"), Hash40::new("hip"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(fighter.lua_state_agent, 128.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, true);
    }
}

#[acmd_script(//WinWait1 
    agent = "mewtwo", 
    script = "effect_win1wait_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_win1wait(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, true);
    }
}

#[acmd_script(//Win3 
    agent = "mewtwo", 
    script = "effect_win3_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_win3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 115.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, true);
    }
}

#[acmd_script(//Win3Wait 
    agent = "mewtwo", 
    script = "effect_win3wait_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_win3wait(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0, 0, 0, 0.4, true);
    }
}

#[acmd_script(//Final, FinalAir 
    agent = "mewtwo", 
    scripts = ["effect_final_silver","effect_finalair_silver"], 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_final(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EffectModule::req_screen(fighter.module_accessor, Hash40::new("bg_mewtwo_final"), false, false, false);
        FILL_SCREEN_MODEL_COLOR(fighter, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1.2, *GROUND, *EFFECT_SCREEN_PRIO_FINAL);
    }
}

#[acmd_script(//Hit 
    agent = "mewtwo_psychobreak", 
    script = "effect_hit_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_psychobreak_hit(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_final_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.45, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
    frame(fighter.lua_state_agent, 67.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("mewtwo_final_hit"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
    }
}

#[acmd_script(//Final 
    agent = "mewtwo_mewtwom", 
    script = "effect_final_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_mewtwom_final_eff(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_final_aura"), Hash40::new("head"), 0, -3, 0, 0, 0, 0, 0.6, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_final_aura"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.7, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
}

#[acmd_script(//FinalEnd 
    agent = "mewtwo_mewtwom", 
    script = "effect_finalend_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_mewtwom_finalend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_final_shot"), false, false);
        BURN_COLOR(fighter, 1, 0.0, 1.0, 1.0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        BURN_COLOR_FRAME(fighter, 5, 1, 1.0, 1.0, 1);
    }
    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        BURN_COLOR_NORMAL(fighter);
    }
}

#[acmd_script(//FinalShoot
    agent = "mewtwo_mewtwom", 
    script = "effect_finalshoot_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_mewtwom_finalshoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_final_shot_hold"), Hash40::new("arml"), 4, 0, -2, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_final_shot_hold"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("0x161efe0679"), false, false);
        EFFECT(fighter, Hash40::new("mewtwo_final_shot"), Hash40::new("top"), 0, 13, 8, 0, 0, 0, 0.6, 0, 0, 0, 50, 360, 50, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
    for _ in 0..90 {
        wait(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        }
    }
}

#[acmd_script(//Move 
    agent = "mewtwo_search", 
    script = "effect_move_silver", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_search_move(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_final_ball"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.49, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
}

#[acmd_script(//EntryL, EntryR 
    agent = "mewtwo", 
    scripts = ["effect_entryr_silver","effect_entryl_silver"], 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_silver_entry(fighter: &mut L2CAgentBase) {
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

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_silver_attack11,
        effect_silver_attackdash,
        effect_silver_attacks3hi,
        effect_silver_attacks3,
        effect_silver_attacks3lw,
        effect_silver_attackhi3,
        effect_silver_attacklw3,
        effect_silver_attacks4charge,
        effect_silver_attackhi4charge,
        effect_silver_attacklw4charge,
        effect_silver_attacks4hi,
        effect_silver_attacks4,
        effect_silver_attacks4lw,
        effect_silver_attackhi4,
        effect_silver_attacklw4,
        effect_silver_attackairn,
        effect_silver_attackairf,
        effect_silver_landingairf,
        effect_silver_attackairb,
        effect_silver_attackairhi,
        effect_silver_attackairlw,
        effect_silver_catchwait,
        effect_silver_catchattack,
        effect_silver_catchpull,
        effect_silver_throwf,
        effect_silver_throwb,
        effect_silver_throwhi,
        effect_silver_throwlw,
        effect_silver_cliffattack,
        effect_silver_downattacku,
        effect_silver_downattackd,
        effect_silver_specialnshoot,
        effect_silver_specialairnshoot,
        effect_silver_beam_shoot,
        effect_silver_beam_hitground,
        effect_silver_specialsstart,
        effect_silver_specials,
        effect_silver_specialairsstart,
        effect_silver_specialairs,
        effect_silver_specialhistart,
        effect_silver_specialhi,
        effect_silver_speciallw,
        effect_silver_specialairlw,
        effect_silver_turnrun,
        effect_silver_escapeairslide,
        effect_silver_escapen,
        effect_silver_escapef,
        effect_silver_escapeb,
        effect_silver_wait2,
        effect_silver_wait3,
        effect_silver_appealhil,
        effect_silver_appealhir,
        effect_silver_appealsl,
        effect_silver_appealsr,
        effect_silver_appeallwl,
        effect_silver_appeallwr,
        effect_silver_win1,
        effect_silver_win1wait,
        effect_silver_entry
    );
}