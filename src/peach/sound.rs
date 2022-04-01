use smash::lua2cpp::L2CAgentBase;
use smash::hash40;
use smashline::*;

#[acmd_script(
agent = "peach",
script = "sound_kamehameha_start",
category = ACMD_SOUND,
low_priority )]
unsafe fn sound_peach_kamehameha_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            PLAY_SE(hash40("se_peach_special_n01"))
        }
    });
}


#[acmd_script(
agent = "peach",
script = "sound_kamehameha_charge",
category = ACMD_SOUND,
low_priority )]
unsafe fn sound_peach_kamehameha_charge(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
    if(is_excute){
        PLAY_SE(hash40("se_peach_special_n02"))
    }
    });
}


#[acmd_script(
agent = "peach",
script = "sound_kamehameha_fire",
category = ACMD_SOUND,
low_priority )]
unsafe fn sound_peach_kamehameha_fire(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
    frame(Frame=9)
    if(is_excute){
        PLAY_SE(hash40("se_peach_special_n04"))
    }
    frame(Frame=67)
    if(is_excute){
        STOP_SE(hash40("se_peach_special_n04"))
    }
    });
}


pub fn install() {
    smashline::install_acmd_scripts!(
        sound_peach_kamehameha_start,
        sound_peach_kamehameha_charge,
        sound_peach_kamehameha_fire
    );
}