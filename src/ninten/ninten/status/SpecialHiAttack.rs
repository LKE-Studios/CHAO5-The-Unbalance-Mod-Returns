use crate::imports::BuildImports::*;

unsafe extern "C" fn status_ninten_SpecialHiAttack_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_ninten_SpecialHiAttack_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        StatusModule::set_keep_situation_air(fighter.module_accessor, true);
        KineticModule::clear_speed_all(fighter.module_accessor);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_attack"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(ninten_SpecialHiAttack_Main_loop as *const () as _))
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn ninten_SpecialHiAttack_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND { 
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn status_ninten_SpecialHiAttack_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_ninten_SpecialHiAttack_ExecStop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_ninten_SpecialHiAttack_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_common_spirits_floor_ice_loop"), 0);
        0.into()
    }
    else {
        0.into()
    }
}

pub fn install() {
    Agent::new("ness")
    .status(Init, *FIGHTER_NINTEN_STATUS_KIND_SPECIAL_HI_ATTACK, status_ninten_SpecialHiAttack_Init)
    .status(Main, *FIGHTER_NINTEN_STATUS_KIND_SPECIAL_HI_ATTACK, status_ninten_SpecialHiAttack_Main)
    .status(Exec, *FIGHTER_NINTEN_STATUS_KIND_SPECIAL_HI_ATTACK, status_ninten_SpecialHiAttack_Exec)
    .status(ExecStop, *FIGHTER_NINTEN_STATUS_KIND_SPECIAL_HI_ATTACK, status_ninten_SpecialHiAttack_ExecStop)
    .status(End, *FIGHTER_NINTEN_STATUS_KIND_SPECIAL_HI_ATTACK, status_ninten_SpecialHiAttack_End)
    .install();
}