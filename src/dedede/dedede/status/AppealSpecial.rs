use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_dedede_AppealSpecial_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);
    0.into()
}

pub unsafe extern "C" fn status_dedede_AppealSpecial_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    VisibilityModule::set_int64(fighter.module_accessor, hash40("hammer") as i64, hash40("hammer_disp_off") as i64);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_r_2_start"), 0.0, 1.25, false, 0.0, false, false);
    let bgm_index = CustomModule::play_dedede_bgm(fighter.module_accessor);
    WorkModule::set_int(fighter.module_accessor, bgm_index, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_BGM_ID);
    fighter.sub_shift_status_main(L2CValue::Ptr(dedede_AppealSpecial_Main_loop as *const () as _))
}

unsafe extern "C" fn dedede_AppealSpecial_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if motion_kind == hash40("appeal_hi_r_2_start") {
        if MotionModule::is_end(fighter.module_accessor) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_r_2"), 0.0, 1.25, false, 0.0, false, false);
        }
    }
    let frame = MotionModule::frame(fighter.module_accessor);
    if motion_kind == hash40("appeal_hi_r_2") {
        if MotionModule::is_end(fighter.module_accessor) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_r_2_end"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    if motion_kind == hash40("appeal_hi_r_2_end") {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
        }
    }
    0.into()
}

pub unsafe extern "C" fn status_dedede_AppealSpecial_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    CustomModule::stop_dedede_bgm(fighter.module_accessor);
    VisibilityModule::set_int64(fighter.module_accessor, hash40("hammer") as i64, hash40("hammer_normal") as i64);
    0.into()
}

pub fn install() {
    Agent::new("dedede")
    .status(Pre, *FIGHTER_DEDEDE_STATUS_KIND_APPEAL_SPECIAL, status_dedede_AppealSpecial_Pre)
    .status(Main, *FIGHTER_DEDEDE_STATUS_KIND_APPEAL_SPECIAL, status_dedede_AppealSpecial_Main)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_APPEAL_SPECIAL, status_dedede_AppealSpecial_End)
    .install();
}
