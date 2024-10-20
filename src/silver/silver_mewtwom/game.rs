use crate::imports::BuildImports::*;

//Final
unsafe extern "C" fn game_silver_mewtwom_Final(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
    }
    frame(fighter.lua_state_agent, 3.0);
    FT_MOTION_RATE(fighter, 10.0);
    frame(fighter.lua_state_agent, 4.0);
    FT_MOTION_RATE(fighter, 1.0);
}

//FinalShoot
unsafe extern "C" fn game_silver_mewtwom_FinalShoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        CAM_ZOOM_IN_arg5(fighter, 10.0, 0.0, 1.0, 0.0, 0.0);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *WEAPON_MEWTWO_MEWTWOM_GENERATE_ARTICLE_SEARCH, false, -1);
    }
    for _ in 0..90 {
        if is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackss"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
}

//FinalEnd
unsafe extern "C" fn game_silver_mewtwom_FinalEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
    }
}

pub fn install() {
    Agent::new("mewtwo_mewtwom")
    .game_acmd("game_final_silver", game_silver_mewtwom_Final, Low)
    .game_acmd("game_finalshoot_silver", game_silver_mewtwom_FinalShoot, Low)
    .game_acmd("game_finalend_silver", game_silver_mewtwom_FinalEnd, Low)
    .install();
}