use crate::imports::BuildImports::*;

//GlideStart
unsafe extern "C" fn expression_metaknight_GlideStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("mantle") as i64, hash40("mantle_wing") as i64);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_dash"), 0, false, 0);
    }
}

//GlideDirection
unsafe extern "C" fn expression_metaknight_GlideDirection(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("mantle") as i64, hash40("mantle_wing") as i64);
    }
}

//SpecialHi
unsafe extern "C" fn expression_metaknight_SpecialHi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("mantle") as i64, hash40("mantle_normal") as i64);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("mantle") as i64, hash40("mantle_wing") as i64);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//SpecialZ
unsafe extern "C" fn expression_metaknight_SpecialZ(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("mantle") as i64, hash40("mantle_normal") as i64);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//SpecialAirZ
unsafe extern "C" fn expression_metaknight_SpecialAirZ(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("mantle") as i64, hash40("mantle_wing") as i64);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("metaknight")
    .expression_acmd("expression_glidestart", expression_metaknight_GlideStart, Low)
    .expression_acmd("expression_glidedirection", expression_metaknight_GlideDirection, Low)
    .expression_acmd("expression_specialhi", expression_metaknight_SpecialHi, Low)
    .expression_acmd("expression_specialz", expression_metaknight_SpecialZ, Low)
    .expression_acmd("expression_specialairz", expression_metaknight_SpecialAirZ, Low)
    .install();
}