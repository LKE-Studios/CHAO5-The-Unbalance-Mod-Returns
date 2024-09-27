use crate::imports::BuildImports::*;

unsafe extern "C" fn status_krystal_SpecialHiRush_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let KRYSTAL = color >= 64 && color <= 71;
    if KRYSTAL {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_CONTINUE);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_INT_RUSH_FRAME);
        if !StopModule::is_stop(fighter.module_accessor) {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_INT_RUSH_FRAME);
            krystal_SpecialHiRush_Sub_Status(fighter, false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
        fighter.sub_shift_status_main(L2CValue::Ptr(krystal_SpecialHiRush_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH)(fighter)
    }
}

unsafe extern "C" fn krystal_SpecialHiRush_Sub_Status(fighter: &mut L2CFighterCommon, param_2: L2CValue) -> L2CValue {
    if param_2.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_INT_RUSH_FRAME);
    }
    0.into()
}

unsafe extern "C" fn krystal_SpecialHiRush_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let frame = MotionModule::frame(fighter.module_accessor);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_AIR);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi"), frame, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_AIR);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi"), frame, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END.into(), false.into());
        return 0.into();
    }
    0.into()
}

pub fn install() {
    Agent::new("pitb")
    .status(Main, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, status_krystal_SpecialHiRush_Main)
    .install();
}