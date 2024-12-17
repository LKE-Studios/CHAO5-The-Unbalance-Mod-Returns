use crate::imports::BuildImports::*;

unsafe extern "C" fn status_maskedman_SpecialHi_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32, 0);
    }
    else {
        original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
    }
}

unsafe extern "C" fn maskedman_SpecialHi_status_helper(fighter: &mut L2CFighterCommon, is_start: bool, status: i32) {
    let motion_g;
    let motion_a;
    if status == FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_HI_HOLD {
        motion_g = Hash40::new("special_hi_hold");
        motion_a = Hash40::new("special_hi_hold");
    }
    else if status == FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_HI_END {
        motion_g = Hash40::new("special_hi_end");
        motion_a = Hash40::new("special_hi_end");
    }
    else {
        motion_g = Hash40::new("special_hi_start");
        motion_a = Hash40::new("special_air_hi_start");
    }
    if is_start && false {
        let motion = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {motion_g} else {motion_a};
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        fighter.sub_change_motion_by_situation(motion_g.into(), motion_a.into(), (!is_start).into());
    }
    let correct = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK} else {*GROUND_CORRECT_KIND_AIR};
    GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(correct));
    if status == *FIGHTER_STATUS_KIND_SPECIAL_HI || !is_start {
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    }
}

unsafe extern "C" fn status_maskedman_SpecialHi_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {
        maskedman_SpecialHi_status_helper(fighter, true, *FIGHTER_STATUS_KIND_SPECIAL_HI);
        fighter.sub_shift_status_main(L2CValue::Ptr(maskedman_SpecialHi_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
    }
}

unsafe extern "C" fn maskedman_SpecialHi_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_HI_HOLD.into(),false.into());
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        maskedman_SpecialHi_status_helper(fighter, false, *FIGHTER_STATUS_KIND_SPECIAL_HI);
    }
    0.into()
}

unsafe extern "C" fn status_maskedman_SpecialHi_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {	
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
    }
}

pub fn install() {
    Agent::new("lucas")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_maskedman_SpecialHi_Pre)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_maskedman_SpecialHi_Main)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_maskedman_SpecialHi_End)
    .install();
}