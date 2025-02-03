use crate::imports::BuildImports::*;

unsafe extern "C" fn status_kirby_SpecialHi2_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_METEOR_HIT_NUM);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi2") as i64, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_hi2") as i64, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_AIR_MOT);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        let air_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_AIR_MOT);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(air_mot), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(air_mot), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT);
    }
    else {
        let ground_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_GROUND_MOT);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(ground_mot), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(ground_mot), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_SpecialHi2_Main_loop as *const () as _))
}

unsafe extern "C" fn kirby_SpecialHi2_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_off_passive_opponent(FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_METEOR_TARGET_ID.into(), FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_METEOR_HIT_NUM.into(), false.into());
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_HI_FLAG_CANCEL) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), true.into());
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_CANCEL) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), true.into());
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3.into(), false.into());
        }
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if MotionModule::frame(fighter.module_accessor) > 1.0 {
                fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4.into(), false.into());
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("kirby")
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, status_kirby_SpecialHi2_Main)
    .install();
}