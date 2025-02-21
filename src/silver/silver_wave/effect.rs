use crate::imports::BuildImports::*;
use crate::silver::silver::frame::*;
use crate::silver::silver::effect::*;

//Shoot 
unsafe extern "C" fn effect_silver_wave_Regular(fighter: &mut L2CAgentBase) {
    let mut shoot : u32 = 0;
    let mut trail : u32 = 0;
    let mut trail2 : u32 = 0;
    let mut trail3 : u32 = 0;
    let mut trail4 : u32 = 0;
    let shadowball_boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(shadowball_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	let ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(module_accessor);
    COUNT[ENTRY_ID] = 0;
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
}

//HitGround 
unsafe extern "C" fn effect_silver_wave_HitGround(fighter: &mut L2CAgentBase) {
    let shadowball_boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(shadowball_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	let ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_shadowball"), true, true);
        if SPECIAL_N_WALL[ENTRY_ID] {
            EFFECT(fighter, Hash40::new("mewtwo_shadowball_bomb"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

pub fn install() {
    Agent::new("mewtwo_wave")
    .effect_acmd("effect_regular", effect_silver_wave_Regular, Low)
    .effect_acmd("effect_hitground", effect_silver_wave_HitGround, Low)
    .install();
}