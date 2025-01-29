use crate::imports::BuildImports::*;

//Fly 
unsafe extern "C" fn game_link_bowarrow_Fly(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG) == FuseType::ELEMENTAL {
        let fuse_item_kind = WorkModule::get_int(fighter.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
        if fuse_item_kind == *ITEM_KIND_FIREFLOWER {
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 71, 0, 10, 1.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0.0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
                AttackModule::set_power_mul(fighter.module_accessor, 1.15);
                AttackModule::enable_safe_pos(fighter.module_accessor);
            }
        }
        else if fuse_item_kind == *ITEM_KIND_FREEZER {
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 71, 0, 10, 1.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0.0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
                AttackModule::set_power_mul(fighter.module_accessor, 0.5);
                AttackModule::enable_safe_pos(fighter.module_accessor);
            }
        }
        else if fuse_item_kind == *ITEM_KIND_CHEWING {
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 99.9, 361, 71, 0, 10, 1.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0.0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_OBJECT);
                AttackModule::set_power_mul(fighter.module_accessor, 10.0);
                AttackModule::enable_safe_pos(fighter.module_accessor);
            }
        }
        else {
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 71, 0, 10, 1.75, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0.0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralysis"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
                AttackModule::set_power_mul(fighter.module_accessor, 1.5);
                AttackModule::enable_safe_pos(fighter.module_accessor);
                QUAKE(fighter,*CAMERA_QUAKE_KIND_S);
            }
        }
    }
    else {
        if WorkModule::get_int(fighter.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_SHOOT_NUM) <= 0 {
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 71, 0, 10, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        }
        else {
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 88, 0, 25, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
                AttackModule::enable_safe_pos(fighter.module_accessor);
            }
        }
    }
}

pub fn install() {
    Agent::new("link_bowarrow")    
    .game_acmd("game_fly", game_link_bowarrow_Fly, Low)
    .install();
}