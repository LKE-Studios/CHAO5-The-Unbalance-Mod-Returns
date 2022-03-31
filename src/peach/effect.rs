use smash::lua2cpp::L2CAgentBase;
use smash::hash40;
use smashline::*;

#[acmd_script(
agent = "peach",
script = "effect_kamehameha_fire",
category = ACMD_EFFECT,
low_priority )]
unsafe fn effect_peach_kamehameha_fire(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=8)
        if(is_execute){
            EFFECT(hash40("sys_genesis_beam"), hash40("top"), 10, 9, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=67)
        if(is_execute){
            EFFECT_OFF_KIND(hash40("sys_genesis_beam"), true, true)
        }
    });
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_peach_kamehameha_fire,
    //    effect_peach_kamehameha_charge,
     //   effect_peach_kamehameha_start
    );
}