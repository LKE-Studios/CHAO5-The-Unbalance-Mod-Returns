use crate::imports::BuildImports::*;

#[status_script( agent = "toonlink", status = FIGHTER_TOONLINK_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_toonlink_SpecialHiEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    fighter.sub_shift_status_main(L2CValue::Ptr(toonlink_SpecialHiEnd_Main_Loop as *const () as _))
}

unsafe fn toonlink_SpecialHiEnd_handler(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        return 1.into();
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                return 0.into();
            }
        }
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                return 1.into();
            }
        }
    }
    0.into();
}

unsafe extern "C" fn toonlink_SpecialHiEnd_Main_Loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    else {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            toonlink_SpecialHiEnd_handler(fighter);

        }
    }
}

