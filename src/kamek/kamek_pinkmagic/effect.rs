use crate::imports::BuildImports::*;
use crate::kamek::kamek::frame::*;

//Move
unsafe extern "C" fn effect_kamek_beam_Regular(fighter: &mut L2CAgentBase) {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let float_charge = WorkModule::get_float(owner_module_accessor, *FIGHTER_KAMEK_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let size = float_charge * 0.015;
    loop {
        if is_excute(fighter) {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("kamek_magicball_flame"), Hash40::new("top"), &NONE, &NONE, 0.3 + size, true, 0, 0, 0, 0, 0, true, true);
        }
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("kamek_magicball_flame"), Hash40::new("top"), &NONE, &NONE, 0.4 + size, true, 0, 0, 0, 0, 0, true, true);
        }
        wait_loop_clear(fighter);
    }
}

pub fn install() {
    Agent::new("ness_pinkmagic")
    .effect_acmd("effect_shoot", effect_kamek_beam_Regular, Low)
    .install();
}