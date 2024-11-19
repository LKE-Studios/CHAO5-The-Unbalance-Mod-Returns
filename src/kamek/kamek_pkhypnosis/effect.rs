use crate::imports::BuildImports::*;

//Move
unsafe extern "C" fn effect_ninten_pkhypnosis_Move(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("sys_special_all_up"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.7, 0.0, 1.0);
        EFFECT_OFF_KIND(fighter,Hash40::new("ness_pkfl_bullet"),false,false);
        EFFECT_OFF_KIND(fighter,Hash40::new("ness_pkfl_bullet_ed"),false,false);
    }
}

//Tame
unsafe extern "C" fn effect_ninten_pkhypnosis_Tame(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("ness_pkfl_bullet"),false,false);
        EFFECT_OFF_KIND(fighter,Hash40::new("ness_pkfl_bullet_ed"),false,false);
        EFFECT(fighter, Hash40::new("sys_explosion_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter,0.7,0.0,1.0);
    }
}

//Bang
unsafe extern "C" fn effect_ninten_pkhypnosis_Bang(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_level_up"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter,0.7,0.0,1.0);
        EFFECT_OFF_KIND(fighter,Hash40::new("ness_pkfl_bomb"), false, false);
        EFFECT_OFF_KIND(fighter,Hash40::new("ness_pkfl_bomb_max"), false, false);
        EFFECT_OFF_KIND(fighter,Hash40::new("ness_pkfl_bullet_ed"), false, false);
    }
}

pub fn install() {
    Agent::new("ness_pkflash")
    .effect_acmd("effect_move_ninten", effect_ninten_pkhypnosis_Move, Low)
    .effect_acmd("effect_tame_ninten", effect_ninten_pkhypnosis_Tame, Low)
    .effect_acmd("effect_bang_ninten", effect_ninten_pkhypnosis_Bang, Low)
    .install();
}