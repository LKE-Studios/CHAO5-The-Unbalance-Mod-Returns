use crate::imports::BuildImports::*;
use crate::kamek::kamek::status::SpecialNHold::*;

//Regular
unsafe extern "C" fn game_kamek_beam_Regular(fighter: &mut L2CAgentBase) {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let float_charge = WorkModule::get_float(owner_module_accessor, *FIGHTER_KAMEK_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let sound_level = if float_charge < charge_frame { *ATTACK_SOUND_LEVEL_M } else { *ATTACK_SOUND_LEVEL_LL };
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 45, 50, 0, 10, 5.0, 0.0, 6.0, -4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -25.0, 0.0, 40, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), sound_level, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 30.0, /*Unk*/ false);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("ness_pinkmagic")
    .game_acmd("game_shoot", game_kamek_beam_Regular, Low)
    .install();
}