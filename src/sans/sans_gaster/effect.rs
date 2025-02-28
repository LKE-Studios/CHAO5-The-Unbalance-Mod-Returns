use crate::imports::BuildImports::*;

//Fire
unsafe extern "C" fn effect_sans_gaster_Fire(fighter: &mut L2CAgentBase) {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(owner_module_accessor, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_GASTER_ANGLE) {
            EFFECT_FOLLOW(fighter, Hash40::new("demon_final_blaster_wing"), Hash40::new("top"), 1, 7, 13.25, 0, 0, 0, 1.1, true);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("demon_final_blaster_wing"), Hash40::new("top"), 1, 7, 15, 25, 0, 0, 1.1, true);
        }
        LAST_EFFECT_SET_COLOR(fighter, 10.0, 10.0, 10.0);
    }
    frame(fighter.lua_state_agent, 58.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("demon_final_blaster_wing"), false, false);
    }
    frame(fighter.lua_state_agent, 75.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 6, 13.25, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 1.8);
    }
}

//Final
unsafe extern "C" fn effect_sans_gaster_Final(fighter: &mut L2CAgentBase) {}

//FinalFire
unsafe extern "C" fn effect_sans_gaster_FinalFire(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("demon_final_blaster_wing"), Hash40::new("top"), 1, 4, 2.25, 0, 0, 0, 1.8, true);
        LAST_EFFECT_SET_COLOR(fighter, 10.0, 10.0, 10.0);
    }
    frame(fighter.lua_state_agent, 58.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("demon_final_blaster_wing"), false, false);
    }
    frame(fighter.lua_state_agent, 75.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 3, 2.25, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 1.8);
    }
}

pub fn install() {
    Agent::new("palutena_gaster")
    .effect_acmd("effect_fire", effect_sans_gaster_Fire, Low)
    .effect_acmd("effect_final", effect_sans_gaster_Final, Low)
    .effect_acmd("effect_finalfire", effect_sans_gaster_FinalFire, Low)
    .install();
}