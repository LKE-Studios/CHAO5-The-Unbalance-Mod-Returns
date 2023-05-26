use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent}
    },
    smash_script::*,
    smashline::*,
    crate::globals::*,
};

#[skyline::hook(replace = L2CFighterCommon_status_Catch_Main)]
unsafe fn status_catch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Goes through a variety of checks to see if you transition into the heavy pickup status or light pickup status
    let heavy_item = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_IS_PICKABLE_ITEM_HEAVY); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    let light_item = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 || FighterUtil::is_valid_auto_catch_item(fighter.module_accessor, false) {
        if fighter.global_table[CMD_CAT1].get_i32() & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH)
            && ItemModule::get_pickable_item_size(fighter.module_accessor) == *ITEM_SIZE_HEAVY as u64
            && heavy_item 
            && !ItemModule::is_have_item(fighter.module_accessor, 0) {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
                return true.into();
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH)
            && ItemModule::get_pickable_item_size(fighter.module_accessor) == *ITEM_SIZE_LIGHT as u64
            && light_item
            && !ItemModule::is_have_item(fighter.module_accessor, 0) {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP.into(), true.into());
                return true.into();
            }
        }
    }
    call_original!(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_catch_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}