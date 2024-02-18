use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_GlideAttack_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_AIR_STOP, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK as u64, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn status_GlideAttack_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_attack"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(GlideAttack_Main_Sub as *const () as _))
}

unsafe extern "C" fn GlideAttack_Main_Sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_landing().get_bool() {
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        let jump_count_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        let status = if jump_count >= jump_count_max {
            FIGHTER_STATUS_KIND_FALL_SPECIAL
        }
        else {
            FIGHTER_STATUS_KIND_FALL_AERIAL
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

pub unsafe extern "C" fn status_GlideAttack_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("fighter")
    .status(Pre, *FIGHTER_STATUS_KIND_GLIDE_ATTACK, status_GlideAttack_Pre)
    .status(Main, *FIGHTER_STATUS_KIND_GLIDE_ATTACK, status_GlideAttack_Main)
    .status(End, *FIGHTER_STATUS_KIND_GLIDE_ATTACK, status_GlideAttack_End)
    .install();
}