use crate::imports::BuildImports::*;

/*//FinalAttack
unsafe extern "C" fn game_gekkouga_gekkougas_FinalAttack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter, );
    }
    frame(fighter.lua_state_agent, 2.0);
    get_fov();
    if(0x141380(1, 25.0)){
        if is_excute(fighter) {
            get_fov(5, 0);
            0x141380(0, 60);
            CAM_ZOOM_IN_arg5(fighter, 0.1, 0);
        }
    }
    frame(fighter.lua_state_agent, 3.0);
    get_fov();
    if(0x141380(1, 5.0)){
        if is_excute(fighter) {
            get_fov(5, 0);
            0x141380(0, 90);
            CAM_ZOOM_IN_arg5(fighter, 0.1, 0);
        }
    }
    frame(fighter.lua_state_agent, 4.0);
    get_fov();
    if(fighter.module_accessor, methodlib::L2CValue::operator<=(lib::L2CValueconst&)const(1, 5.0)){
        if is_excute(fighter) {
            CAM_ZOOM_IN_arg5(fighter, 5, 0, 10, 0, 0);
        }
    }
    camera(*MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 15);
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        methodlib::L2CValue::as_hash(fighter.module_accessor)const(Hash40::new("rbkind_slashs"), 19, false);
        ControlModule::set_rumble(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ Hash40::new("no"), /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *WEAPON_GEKKOUGA_GEKKOUGAS_INSTANCE_WORK_ID_FLAG_REQUEST_LOOP_DAMAGE_MOTION);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *WEAPON_GEKKOUGA_GEKKOUGAS_GENERATE_ARTICLE_BUNSHIN, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ Hash40::new("no"), /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        methodlib::L2CValue::as_hash(fighter.module_accessor)const(Hash40::new("rbkind_attacks"), 5, false);
        ControlModule::set_rumble(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ Hash40::new("no"), /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        methodlib::L2CValue::as_hash(fighter.module_accessor)const(Hash40::new("rbkind_slashs"), 16, false);
        ControlModule::set_rumble(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 43.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *WEAPON_GEKKOUGA_GEKKOUGAS_GENERATE_ARTICLE_BUNSHIN, false);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *WEAPON_GEKKOUGA_GEKKOUGAS_GENERATE_ARTICLE_BUNSHIN, true);
    }
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ Hash40::new("no"), /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        VisibilityModule::set_whole(fighter.module_accessor, false);
        methodlib::L2CValue::as_hash(fighter.module_accessor)const(Hash40::new("rbkind_attackm"), 5, false);
        ControlModule::set_rumble(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 51.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        methodlib::L2CValue::as_hash(fighter.module_accessor)const(Hash40::new("rbkind_attacks"), 9, false);
        ControlModule::set_rumble(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 56.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ Hash40::new("no"), /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 57.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *WEAPON_GEKKOUGA_GEKKOUGAS_GENERATE_ARTICLE_BUNSHIN, false);
    }
    frame(fighter.lua_state_agent, 59.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *WEAPON_GEKKOUGA_GEKKOUGAS_GENERATE_ARTICLE_BUNSHIN, true);
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ Hash40::new("no"), /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        methodlib::L2CValue::as_hash(fighter.module_accessor)const(Hash40::new("rbkind_slashm"), 14, false);
        ControlModule::set_rumble(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 66.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 73.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 77.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 80.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ Hash40::new("no"), /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        methodlib::L2CValue::as_hash(fighter.module_accessor)const(Hash40::new("rbkind_attacks"), 11, false);
        ControlModule::set_rumble(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 81.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 82.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *WEAPON_GEKKOUGA_GEKKOUGAS_GENERATE_ARTICLE_BUNSHIN, false);
    }
    frame(fighter.lua_state_agent, 85.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *WEAPON_GEKKOUGA_GEKKOUGAS_GENERATE_ARTICLE_BUNSHIN, true);
    }
    frame(fighter.lua_state_agent, 88.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 92.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ Hash40::new("no"), /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        methodlib::L2CValue::as_hash(fighter.module_accessor)const(Hash40::new("rbkind_slashm"), 4, false);
        ControlModule::set_rumble(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 93.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 95.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 97.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ Hash40::new("no"), /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        methodlib::L2CValue::as_hash(fighter.module_accessor)const(Hash40::new("rbkind_attacks"), 17, false);
        ControlModule::set_rumble(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 98.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 105.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 106.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *WEAPON_GEKKOUGA_GEKKOUGAS_GENERATE_ARTICLE_BUNSHIN, false);
    }
    frame(fighter.lua_state_agent, 107.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 112.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ Hash40::new("no"), /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        methodlib::L2CValue::as_hash(fighter.module_accessor)const(Hash40::new("rbkind_slashm"), 5, false);
        ControlModule::set_rumble(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 113.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 115.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *WEAPON_GEKKOUGA_GEKKOUGAS_GENERATE_ARTICLE_BUNSHIN, true);
    }
    frame(fighter.lua_state_agent, 118.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ Hash40::new("no"), /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        methodlib::L2CValue::as_hash(fighter.module_accessor)const(Hash40::new("rbkind_attacks"), 17, false);
        ControlModule::set_rumble(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 119.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 125.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *WEAPON_GEKKOUGA_GEKKOUGAS_GENERATE_ARTICLE_BUNSHIN, false);
    }
    frame(fighter.lua_state_agent, 128.3);
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 133.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 136.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ Hash40::new("no"), /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        methodlib::L2CValue::as_hash(fighter.module_accessor)const(Hash40::new("rbkind_attackm"), 8, false);
        ControlModule::set_rumble(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 137.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 139.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *WEAPON_GEKKOUGA_GEKKOUGAS_GENERATE_ARTICLE_BUNSHIN, true);
    }
    frame(fighter.lua_state_agent, 144.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 145.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ Hash40::new("no"), /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        methodlib::L2CValue::as_hash(fighter.module_accessor)const(Hash40::new("rbkind_attacks"), 6, false);
        ControlModule::set_rumble(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 146.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 150.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 155.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *WEAPON_GEKKOUGA_GEKKOUGAS_GENERATE_ARTICLE_BUNSHIN, false);
    }
    frame(fighter.lua_state_agent, 157.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ Hash40::new("no"), /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        methodlib::L2CValue::as_hash(fighter.module_accessor)const(Hash40::new("rbkind_slashs"), 9, false);
        ControlModule::set_rumble(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 158.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 161.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *WEAPON_GEKKOUGA_GEKKOUGAS_GENERATE_ARTICLE_BUNSHIN, true);
    }
    frame(fighter.lua_state_agent, 167.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ Hash40::new("no"), /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        methodlib::L2CValue::as_hash(fighter.module_accessor)const(Hash40::new("rbkind_slashs"), 0, false);
        ControlModule::set_rumble(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_GEKKOUGA_GEKKOUGAS_INSTANCE_WORK_ID_FLAG_ATTACK_END);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_GEKKOUGA_GEKKOUGAS_INSTANCE_WORK_ID_FLAG_REQUEST_LOOP_DAMAGE_MOTION);
        WorkModule::set_int64(fighter.module_accessor, Hash40::new("fall_damage"), *WEAPON_GEKKOUGA_GEKKOUGAS_INSTANCE_WORK_ID_INT_REQUEST_LOOP_DAMAGE_MOTION);
    }
    frame(fighter.lua_state_agent, 168.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 172.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 184.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 188.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *WEAPON_GEKKOUGA_GEKKOUGAS_GENERATE_ARTICLE_BUNSHIN, false);
    }
    frame(fighter.lua_state_agent, 190.0);
    if is_excute(fighter) {
        FILL_SCREEN_MODEL_COLOR(fighter, 0, 30, 0.65, 0.65, 0.65, 0, 0, 0, 0.6, 2.7, *EffectScreenLayer:CHAR, *EFFECT_SCREEN_PRIO_FINAL);
    }
    frame(fighter.lua_state_agent, 242.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_GEKKOUGA_GEKKOUGAS_INSTANCE_WORK_ID_FLAG_TARGET_NOREAC_OFF);
    }
    frame(fighter.lua_state_agent, 243.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 100.0, /*Angle*/ 275, /*KBG*/ 125, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 30.0, /*X*/ 0.0, /*Y*/ -10.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ Hash40::new("no"), /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        methodlib::L2CValue::as_hash(fighter.module_accessor)const(Hash40::new("rbkind_attack_finish"), 0, false);
        ControlModule::set_rumble(fighter.module_accessor);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, 93, false);
    }
    frame(fighter.lua_state_agent, 244.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *WEAPON_GEKKOUGA_GEKKOUGAS_INSTANCE_WORK_ID_FLAG_ATTACK_END);
    }
    frame(fighter.lua_state_agent, 255.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *WEAPON_GEKKOUGA_GEKKOUGAS_GENERATE_ARTICLE_BUNSHIN, true);
    }
}*/

pub fn install() {
    Agent::new("gekkouga_gekkougas")
    .game_acmd("game_finalattack", game_gekkouga_gekkougas_FinalAttack, Low)
    .install();
}