use crate::imports::BuildImports::*;
use crate::pit::pit::status::SpecialHiFly::*;

pub unsafe extern "C" fn status_pit_SpecialHiFlyTurn_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0 as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHiFlyTurn_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_fly_turn"), 0.0, 1.0, false, 0.0, false, false);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    PostureModule::reverse_lr(fighter.module_accessor);
    fighter.sub_shift_status_main(L2CValue::Ptr(pit_SpecialHiFlyTurn_Main_loop as *const () as _))
}

unsafe extern "C" fn pit_SpecialHiFlyTurn_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY.into(), true.into());
    }
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHiFlyTurn_End(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("pit")
    .status(Pre, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN, status_pit_SpecialHiFlyTurn_Pre)
    .status(Main, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN, status_pit_SpecialHiFlyTurn_Main)
    .status(End, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN, status_pit_SpecialHiFlyTurn_End)
    .install();
}