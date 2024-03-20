use crate::imports::BuildImports::*;

//AttackCommon
unsafe extern "C" fn game_popo_iceberg_AttackCommon(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("needle1"), 11.0, 65, 144, 0, 70, 2.0, -1.0, -1.0, 0.0, Some(25.0), Some(-80.0), Some(20.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("needle1"), 11.0, 65, 144, 0, 70, 2.0, -1.0, -1.0, 0.0, Some(-30.0), Some(0.0), Some(-14.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 2, 0, Hash40::new("needle2"), 11.0, 65, 144, 0, 70, 2.0, -1.0, -1.0, 0.0, Some(0.0), Some(-50.0), Some(-10.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 3, 0, Hash40::new("needle2"), 11.0, 65, 144, 0, 70, 2.0, -1.0, -1.0, 0.0, Some(0.0), Some(30.0), Some(40.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 4, 0, Hash40::new("needle3"), 11.0, 65, 144, 0, 70, 2.0, 0.0, -1.0, 0.0, Some(-20.0), Some(-80.0), Some(10.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 5, 0, Hash40::new("needle3"), 11.0, 65, 144, 0, 70, 2.0, 0.0, -1.0, 0.0, Some(25.0), Some(30.0), Some(-25.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 3, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 4, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 5, true, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 1, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 2, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 3, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 4, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 5, true, true, -1.0, false);
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_POPO_ICEBERG_INSTANCE_WORK_ID_FLAG_CHANGE_ATTACK_UPDATE_BASE);
    }
}

//FinalRise
unsafe extern "C" fn game_popo_iceberg_FinalRise(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("needle1"), 10.0, 65, 144, 0, 70, 2.0, -1.0, -1.0, 0.0, Some(25.0), Some(-80.0), Some(20.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("needle1"), 10.0, 65, 144, 0, 70, 2.0, -1.0, -1.0, 0.0, Some(-30.0), Some(0.0), Some(-14.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 2, 0, Hash40::new("needle2"), 10.0, 65, 144, 0, 70, 2.0, -1.0, -1.0, 0.0, Some(0.0), Some(-50.0), Some(-10.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 3, 0, Hash40::new("needle2"), 10.0, 65, 144, 0, 70, 2.0, -1.0, -1.0, 0.0, Some(0.0), Some(30.0), Some(40.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 4, 0, Hash40::new("needle3"), 10.0, 65, 144, 0, 70, 2.0, 0.0, -1.0, 0.0, Some(-20.0), Some(-80.0), Some(10.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 5, 0, Hash40::new("needle3"), 10.0, 65, 144, 0, 70, 2.0, 0.0, -1.0, 0.0, Some(25.0), Some(30.0), Some(-25.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 3, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 4, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 5, true, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 1, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 2, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 3, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 4, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 5, true, true, -1.0, false);
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_POPO_ICEBERG_INSTANCE_WORK_ID_FLAG_CHANGE_ATTACK_UPDATE_BASE);
    }
}

//FinalDecline
unsafe extern "C" fn game_popo_iceberg_FinalDecline(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("needle1"), 11.0, 65, 144, 0, 70, 2.0, -1.0, -1.0, 0.0, Some(25.0), Some(-80.0), Some(20.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("needle1"), 11.0, 65, 144, 0, 70, 2.0, -1.0, -1.0, 0.0, Some(-30.0), Some(0.0), Some(-14.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 2, 0, Hash40::new("needle2"), 11.0, 65, 144, 0, 70, 2.0, -1.0, -1.0, 0.0, Some(0.0), Some(-50.0), Some(-10.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 3, 0, Hash40::new("needle2"), 11.0, 65, 144, 0, 70, 2.0, -1.0, -1.0, 0.0, Some(0.0), Some(30.0), Some(40.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 4, 0, Hash40::new("needle3"), 11.0, 65, 144, 0, 70, 2.0, 0.0, -1.0, 0.0, Some(-20.0), Some(-80.0), Some(10.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 5, 0, Hash40::new("needle3"), 11.0, 65, 144, 0, 70, 2.0, 0.0, -1.0, 0.0, Some(25.0), Some(30.0), Some(-25.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 3, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 4, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 5, true, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 1, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 2, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 3, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 4, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 5, true, true, -1.0, false);
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_POPO_ICEBERG_INSTANCE_WORK_ID_FLAG_CHANGE_ATTACK_UPDATE_BASE);
    }
}

//FinalWait
unsafe extern "C" fn game_popo_iceberg_FinalWait(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("needle1"), 9.0, 65, 144, 0, 70, 2.0, -1.0, -1.0, 0.0, Some(25.0), Some(-80.0), Some(20.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("needle1"), 9.0, 65, 144, 0, 70, 2.0, -1.0, -1.0, 0.0, Some(-30.0), Some(0.0), Some(-14.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 2, 0, Hash40::new("needle2"), 9.0, 65, 144, 0, 70, 2.0, -1.0, -1.0, 0.0, Some(0.0), Some(-50.0), Some(-10.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 3, 0, Hash40::new("needle2"), 9.0, 65, 144, 0, 70, 2.0, -1.0, -1.0, 0.0, Some(0.0), Some(30.0), Some(40.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 4, 0, Hash40::new("needle3"), 9.0, 65, 144, 0, 70, 2.0, 0.0, -1.0, 0.0, Some(-20.0), Some(-80.0), Some(10.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATTACK(fighter, 5, 0, Hash40::new("needle3"), 9.0, 65, 144, 0, 70, 2.0, 0.0, -1.0, 0.0, Some(25.0), Some(30.0), Some(-25.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 3, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 4, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 5, true, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 1, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 2, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 3, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 4, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 5, true, true, -1.0, false);
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_POPO_ICEBERG_INSTANCE_WORK_ID_FLAG_CHANGE_ATTACK_UPDATE_BASE);
    }
}

pub fn install() {
    Agent::new("popo_iceberg")
    .game_acmd("game_attackcommon", game_popo_iceberg_AttackCommon)
    .game_acmd("game_finalrise", game_popo_iceberg_FinalRise)
    .game_acmd("game_finaldecline", game_popo_iceberg_FinalDecline)
    .game_acmd("game_finalwait", game_popo_iceberg_FinalWait)
    .install();
}