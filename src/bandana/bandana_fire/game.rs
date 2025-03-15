use crate::imports::BuildImports::*;

//SpecialN1
unsafe extern "C" fn game_bandana_fire_SpecialN1(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::disable_tip(fighter.module_accessor);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 82, 74, 0, 50, 7.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, -2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
}

//BurstS
unsafe extern "C" fn game_bandana_fire_BurstS(fighter: &mut L2CAgentBase) {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let store_frame = WorkModule::get_int(owner_module_accessor, *FIGHTER_BANDANA_INSTANCE_WORK_ID_INT_SPECIAL_S_STORE_FRAME);
    if is_excute(fighter) {
        if store_frame == 30 {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_furafura"), 9, false, 0);
        }
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        if store_frame == 30 {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosionm"), 6, false, 0);
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            AttackModule::disable_tip(fighter.module_accessor);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 36.0, 62, 65, 0, 30, 20.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }
}

pub fn install() {
    Agent::new("edge_fire")
    .game_acmd("game_specialn1_bandana", game_bandana_fire_SpecialN1, Low)
    .game_acmd("game_bursts_bandana", game_bandana_fire_BurstS, Low)
    .install();
}