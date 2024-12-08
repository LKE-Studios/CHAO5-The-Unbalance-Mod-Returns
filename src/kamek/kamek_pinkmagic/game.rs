use crate::imports::BuildImports::*;
use crate::kamek::kamek::frame::*;
use crate::kamek::kamek::status::SpecialNHold::*;

//Regular
unsafe extern "C" fn game_kamek_beam_Regular(fighter: &mut L2CAgentBase) {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let ENTRY_ID = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let damage = (FIGHTER_KAMEK_STATUS_SPECIAL_N_CHARGE[ENTRY_ID] * 0.5) + 1.0;
    let size = (FIGHTER_KAMEK_STATUS_SPECIAL_N_CHARGE[ENTRY_ID] * 0.05) + 0.1;
    let sound_level = if FIGHTER_KAMEK_STATUS_SPECIAL_N_CHARGE[ENTRY_ID] < full_charge_time { *ATTACK_SOUND_LEVEL_M } else { *ATTACK_SOUND_LEVEL_LL };
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0 + damage, 361, 60, 0, 30, 5.0 + size, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -25.0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), sound_level, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("ness_pinkmagic")
    .game_acmd("game_finalcutterregular", game_kamek_beam_Regular, Low)
    .install();
}