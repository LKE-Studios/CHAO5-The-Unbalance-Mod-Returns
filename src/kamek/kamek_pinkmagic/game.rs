use crate::imports::BuildImports::*;

//Pillar
unsafe extern "C" fn game_ninten_pkhypnosis_Bang(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        let power = WorkModule::get_int(fighter.module_accessor,*WEAPON_NESS_PK_FLASH_INSTANCE_WORK_ID_INT_POWER) as f32;
        AttackModule::set_power(fighter.module_accessor, 1, power, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 6, 0, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep_ex"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("top"), power, 361, 3, 0, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_LEFT, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep_ex"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PSI);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x27936db96d));
    }
}

pub fn install() {
    Agent::new("ness_pkflash")
    .game_acmd("game_bang_ninten", game_ninten_pkhypnosis_Bang, Low)
    .install();
}