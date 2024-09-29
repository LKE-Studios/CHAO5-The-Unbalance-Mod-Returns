use crate::imports::BuildImports::*;

//Shoot
unsafe extern "C" fn effect_metaknight_galaxiabeam_Shoot(fighter: &mut L2CAgentBase) {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    frame(fighter.lua_state_agent, 1.0);
    if WorkModule::is_flag(owner_module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_GUARD)
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_final_edge_yellow"), Hash40::new("rot"), 0, 3, 0, 0.0, 0, 0, 1.0, true);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("miiswordsman_final_edge_yellow"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1.0, true);
        }
    }
}

pub fn install() {
    Agent::new("metaknight_galaxiabeam")
    .effect_acmd("effect_shoot", effect_metaknight_galaxiabeam_Shoot, Low)
    .install();
}