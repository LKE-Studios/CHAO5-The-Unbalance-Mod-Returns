use crate::imports::BuildImports::*;

//Regular
unsafe extern "C" fn effect_waluigi_diceblock_Regular(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.4, /*G*/ 0.0, /*B*/ 2.0);
    }
}

//Bound
unsafe extern "C" fn effect_waluigi_diceblock_Bound(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, -4, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
	}
}

//Break
unsafe extern "C" fn effect_waluigi_diceblock_Break(fighter: &mut L2CAgentBase) {
    let dice_num = WorkModule::get_int(fighter.module_accessor, *WEAPON_WALUIGI_DICEBLOCK_INSTANCE_WORK_ID_INT_NUMBER);
    if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_assist"), Hash40::new("top"), 0, -4, 1, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
		EFFECT(fighter, Hash40::new("sys_assist"), Hash40::new("top"), 0, 4, -1, 0, 0, 180, 0.6, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_pokemon_out"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        if dice_num == 0 {
            EFFECT(fighter, Hash40::new("waluigi_diceblock_num_1"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 1 {
            EFFECT(fighter, Hash40::new("waluigi_diceblock_num_2"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 2 {
            EFFECT(fighter, Hash40::new("waluigi_diceblock_num_3"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 3 {
            EFFECT(fighter, Hash40::new("waluigi_diceblock_num_4"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 4 {
            EFFECT(fighter, Hash40::new("waluigi_diceblock_num_5"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 5 {
            EFFECT(fighter, Hash40::new("waluigi_diceblock_num_6"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 6 {
            EFFECT(fighter, Hash40::new("waluigi_diceblock_num_7"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 7 {
            EFFECT(fighter, Hash40::new("waluigi_diceblock_num_8"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 8 {
            EFFECT(fighter, Hash40::new("waluigi_diceblock_num_9"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 9 {
            EFFECT(fighter, Hash40::new("overlay_num_10"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

pub fn install() {
    Agent::new("dolly_diceblock")
    .effect_acmd("effect_regular", effect_waluigi_diceblock_Regular, Low)
    .effect_acmd("effect_bound", effect_waluigi_diceblock_Bound, Low)
    .effect_acmd("effect_break", effect_waluigi_diceblock_Break, Low)
    .install();
}