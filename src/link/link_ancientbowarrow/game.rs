use crate::imports::BuildImports::*;

//Stick
unsafe extern "C" fn game_link_ancientbowarrow_Stick(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        CAM_ZOOM_OUT_FINAL(fighter);
    }
    fighter.clear_lua_stack();
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
        if is_excute(fighter) {
            let object_id = WorkModule::get_int64(fighter.module_accessor, *WEAPON_LINK_ANCIENTBOWARROW_INSTANCE_WORK_ID_INT_HIT_OBJECT_ID) as u32;
            CAM_ZOOM_IN_FINAL_arg13(fighter, 3.0, 0.0, 2.0, -1, 0, 1, -10, 10, true, object_id, 0, -10, 0);
        }
    }
    else {
        if is_excute(fighter) {
            let scale = PostureModule::scale(fighter.module_accessor);
            let object_id = WorkModule::get_int64(fighter.module_accessor, *WEAPON_LINK_ANCIENTBOWARROW_INSTANCE_WORK_ID_INT_HIT_OBJECT_ID) as u32;
            CAM_ZOOM_IN_FINAL_arg13(fighter, 3.0, 0.0, 2.0 * scale, 1, 0, 1, -10, 10, true, object_id, 0, 10, 0);
        }
    }
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 350.0, /*Angle*/ 60, /*KBG*/ 46, /*FKB*/ 0, /*BKB*/ 102, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_BOMB, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        CAM_ZOOM_OUT_FINAL(fighter);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }
}

pub fn install() {
    Agent::new("link_ancientbowarrow")    
    .game_acmd("game_stick", game_link_ancientbowarrow_Stick, Low)
    .install();
}