use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//SpecialHiL GFX
    agent = "purin", 
    script = "effect_specialhil", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn purin_upblgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("purin_utau"), Hash40::new("hip"), 2, 0, 0, 0, 0, 0, 2.25, true);
    }
}

#[acmd_script(//SpecialHiR GFX
    agent = "purin", 
    script = "effect_specialhir", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn purin_upbrgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("purin_utau"), Hash40::new("hip"), 2, 0, 0, 0, 0, 0, 2.25, true);
    }
}

#[acmd_script(//SpecialAirHiL GFX
    agent = "purin", 
    script = "effect_specialairhil", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn purin_upbairlgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("purin_utau"), Hash40::new("hip"), 2, 0, 0, 0, 0, 0, 2.25, true);
    }
}

#[acmd_script(//SpecialAirHiR GFX
    agent = "purin", 
    script = "effect_specialairhir", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn purin_upbairrgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("purin_utau"), Hash40::new("hip"), 2, 0, 0, 0, 0, 0, 2.25, true);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        purin_upblgfx,
        purin_upbrgfx,
        purin_upbairlgfx,
        purin_upbairrgfx,
    );
}
