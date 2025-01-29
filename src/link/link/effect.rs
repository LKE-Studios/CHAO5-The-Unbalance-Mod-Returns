use crate::imports::BuildImports::*;

//AscendJump
unsafe extern "C" fn effect_link_AscendJump(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 1.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 13, 0, -90, 0, 0, 0.625, true, *EF_FLIP_YZ);
        LAST_PARTICLE_SET_COLOR(fighter, 0.25, 1.00, 0.5);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_vector"), Hash40::new("sys_vector"), Hash40::new("top"), 0, 25, 0, -90, 0, 0, 2.0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0, 2, 0.5);
        LAST_EFFECT_SET_SCALE_W(fighter, 1.0, 3.25, 1);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("brave_tornado1_wind"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 25.0); 
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_vector"), false, false);
    }
}

//AscendAirJump
unsafe extern "C" fn effect_link_AscendAirJump(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 1.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 13, 0, -90, 0, 0, 0.625, true, *EF_FLIP_YZ);
        LAST_PARTICLE_SET_COLOR(fighter, 0.25, 1.00, 0.5);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_vector"), Hash40::new("sys_vector"), Hash40::new("top"), 0, 25, 0, -90, 0, 0, 2.0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0, 2, 0.5);
        LAST_EFFECT_SET_SCALE_W(fighter, 1.0, 3.25, 1);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("brave_tornado1_wind"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 25.0); 
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_vector"), false, false);
    }
}

//AscendStart
unsafe extern "C" fn effect_link_AscendStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {        
        let target_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
        let pos_y = PostureModule::pos_y(fighter.module_accessor);
        EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 0, target_y - pos_y + 5.0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.25, 1.0, 0.5);
        EFFECT(fighter, Hash40::new("sys_assist_out"), Hash40::new("top"), 0, -2.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.25, 1, 0.5);
        LAST_EFFECT_SET_RATE(fighter, 1.25);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("hip"), 0, 0.0, 0, 0, 0, 0, 3.0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {       
        EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.25, 1.0, 0.5);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
}

//Ascend
unsafe extern "C" fn effect_link_Ascend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {        
        let target_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
        let pos_y = PostureModule::pos_y(fighter.module_accessor);
        EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 0, target_y - pos_y + 5.0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.25, 1.0, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("hip"), 0, 0.0, 0, 0, 0, 0, 3.0, false);
    }
    for _ in 0..i32::MAX {
        wait(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, -4, 0, -90, 0, 0, 1.25, true, *EF_FLIP_YZ);
            LAST_PARTICLE_SET_COLOR(fighter, 0.25, 1.0, 0.5);
            FLASH(fighter, 0.25, 1.0, 0.5, 0.0);
        }
        wait(fighter.lua_state_agent, 3.0);
    }
}

//AscendEnd
unsafe extern "C" fn effect_link_AscendEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.25, 1.0, 0.5);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}

//RevaliGlideDrop
unsafe extern "C" fn effect_link_RevaliGlideDrop(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("link_entry"), Hash40::new("trans"), 9.5, 3.6, 1.3, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(fighter, Hash40::new("link_entry"), Hash40::new("trans"), -9.5, 3.6, 1.3, 0, 0, 0, 1, false);
    }
}

//RevaliGlideLanding
unsafe extern "C" fn effect_link_RevaliGlideLanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    Agent::new("link")
    .effect_acmd("effect_ascendjump", effect_link_AscendJump, Low)
    .effect_acmd("effect_ascendairjump", effect_link_AscendAirJump, Low)
    .effect_acmd("effect_ascendstart", effect_link_AscendStart, Low)
    .effect_acmd("effect_ascend", effect_link_Ascend, Low)
    .effect_acmd("effect_ascendend", effect_link_AscendEnd, Low)
    .effect_acmd("effect_revaliglidedrop", effect_link_RevaliGlideDrop, Low)
    .effect_acmd("effect_revaliglidelanding", effect_link_RevaliGlideLanding, Low)
    .install();
}