use crate::imports::BuildImports::*;

//AscendJump
unsafe extern "C" fn effect_link_parasail_AscendJump(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("link_entry"), Hash40::new("trans"), 9.5, 3.6, 1.3, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(fighter, Hash40::new("link_entry"), Hash40::new("trans"), -9.5, 3.6, 1.3, 0, 0, 0, 1, false);
    }
}

//Glide
unsafe extern "C" fn effect_link_parasail_Glide(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("link_entry"), Hash40::new("trans"), 9.5, 3.6, 1.3, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(fighter, Hash40::new("link_entry"), Hash40::new("trans"), -9.5, 3.6, 1.3, 0, 0, 0, 1, false);
    }
}

//GlideTurn
unsafe extern "C" fn effect_link_parasail_GlideTurn(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("link_entry"), Hash40::new("trans"), 9.5, 3.6, 1.3, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(fighter, Hash40::new("link_entry"), Hash40::new("trans"), -9.5, 3.6, 1.3, 0, 0, 0, 1, false);
    }
}

//GlideDrop
unsafe extern "C" fn effect_link_parasail_GlideDrop(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("link_entry"), Hash40::new("trans"), 9.5, 3.6, 1.3, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(fighter, Hash40::new("link_entry"), Hash40::new("trans"), -9.5, 3.6, 1.3, 0, 0, 0, 1, false);
    }
}

//GlideLanding
unsafe extern "C" fn effect_link_parasail_GlideLanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("link_entry"), Hash40::new("trans"), 9.5, 3.6, 1.3, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(fighter, Hash40::new("link_entry"), Hash40::new("trans"), -9.5, 3.6, 1.3, 0, 0, 0, 1, false);
    }
}

pub fn install() {
    Agent::new("link_parasail")    
    .effect_acmd("effect_ascendjump", effect_link_parasail_AscendJump, Low)
    .effect_acmd("effect_glide", effect_link_parasail_Glide, Low)
    .effect_acmd("effect_glideturn", effect_link_parasail_GlideTurn, Low)
    .effect_acmd("effect_glidedrop", effect_link_parasail_GlideLanding, Low)
    .install();
}