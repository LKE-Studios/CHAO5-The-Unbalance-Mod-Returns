use crate::imports::BuildImports::*;

unsafe extern "C" fn status_krystal_SpecialHiRushEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let KRYSTAL = color >= 64 && color <= 71;
    if KRYSTAL {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_end"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(krystal_SpecialHiRushEnd_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END)(fighter)
    }
}

unsafe extern "C" fn krystal_SpecialHiRushEnd_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
    }
    else if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return 0.into();
    }
    0.into()
}

pub fn install() {
    Agent::new("pitb")
    .status(Main, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, status_krystal_SpecialHiRushEnd_Main)
    .install();
}