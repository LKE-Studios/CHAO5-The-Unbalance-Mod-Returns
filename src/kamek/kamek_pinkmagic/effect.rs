use crate::imports::BuildImports::*;
use crate::kamek::kamek::frame::*;

//Move
unsafe extern "C" fn effect_kamek_beam_Regular(fighter: &mut L2CAgentBase) {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let ENTRY_ID = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let size = (FIGHTER_KAMEK_STATUS_SPECIAL_N_CHARGE[ENTRY_ID] * 0.04) + 0.1;
    let size_2 = (FIGHTER_KAMEK_STATUS_SPECIAL_N_CHARGE[ENTRY_ID] * 0.02) + 0.05;
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("lucario_hadoudan"), Hash40::new("top"), &N1, &NONE, 0.75 + size, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("stg_mariou_water_magic_bright2"), Hash40::new("top"), &NONE, &NONE, 0.4 + size_2, true, 0, 0, 0, 0, 0, true, true);
    }
    loop {
        if is_excute(fighter) {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("kamek_magicball_flame"), Hash40::new("top"), &NONE, &NONE, 0.3 + size_2, true, 0, 0, 0, 0, 0, true, true);
        }
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("kamek_magicball_flame"), Hash40::new("top"), &NONE, &NONE, 0.4 + size_2, true, 0, 0, 0, 0, 0, true, true);
        }
        wait_loop_clear(fighter);
    }
}

pub fn install() {
    Agent::new("ness_pinkmagic")
    .effect_acmd("effect_shoot", effect_kamek_beam_Regular, Low)
    .install();
}