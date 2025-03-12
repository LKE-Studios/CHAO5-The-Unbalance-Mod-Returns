use crate::imports::BuildImports::*;

unsafe extern "C" fn status_donkey_SpecialHi_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR 
    && fighter.global_table[PREV_SITUATION_KIND] == *SITUATION_KIND_GROUND {
        GroundModule::set_cliff_check(fighter.module_accessor, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_YACL_DEFAULT);
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn status_donkey_SpecialHi_Exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode {_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
    0.into()
}

pub fn install() {
    Agent::new("donkey")
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_donkey_SpecialHi_Exec)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_donkey_SpecialHi_Exit)
    .install();
}