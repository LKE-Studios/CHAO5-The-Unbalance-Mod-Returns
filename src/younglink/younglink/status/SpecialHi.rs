use crate::imports::BuildImports::*; 

pub static start_x_mul : f32 = 2.2;

pub unsafe extern "C" fn status_younglink_SpecialHi_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let start_speed = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, start_speed * start_x_mul);
    }
    let mask_flag = if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64
    } else {
        0 as u64
    };
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, 0, 0, 0, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, mask_flag, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("younglink")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_younglink_SpecialHi_Pre)
    .install();
}
