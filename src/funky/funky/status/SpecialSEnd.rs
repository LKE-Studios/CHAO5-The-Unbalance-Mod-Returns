use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_funky_SpecialSEnd_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
        0.into()
    }
    else {
        0.into()
    }
}

pub unsafe extern "C" fn status_funky_SpecialSEnd_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        let motion = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut smash::app::KineticEnergy;
        let lr = PostureModule::lr(fighter.module_accessor);
        KineticUtility::reset_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_FREE, Vector2f{x: 0.0, y: 0.0}, Vector3f{x: 0.0, y: 0.0, z: 0.0});
        KineticEnergy::reset_energy(motion, *ENERGY_STOP_RESET_TYPE_AIR, &Vector2f{x: 0.0 * lr, y: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, fighter.module_accessor);
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        0.into()
    }
    else {
        0.into()
    }
}

pub unsafe extern "C" fn status_funky_SpecialSEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_end"), 0.0, 1.0, false, 0.0, false, false);
        if !StopModule::is_stop(fighter.module_accessor) {
            fighter.sub_fall_common_uniq(false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
        fighter.sub_shift_status_main(L2CValue::Ptr(funky_SpecialSEnd_Main_loop as *const () as _))
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn funky_SpecialSEnd_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_air_landing().get_bool() {
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        let jump_count_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        let status = if jump_count >= jump_count_max {
            FIGHTER_STATUS_KIND_FALL_AERIAL
        }
        else {
            FIGHTER_STATUS_KIND_FALL_AERIAL
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

pub unsafe extern "C" fn status_funky_SpecialSEnd_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        0.into()
    }
    else {
        0.into()
    }
}

pub fn install() {
    Agent::new("donkey")
    .status(Pre, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_S_END, status_funky_SpecialSEnd_Pre)
    .status(Init, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_S_END, status_funky_SpecialSEnd_Init)
    .status(Main, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_S_END, status_funky_SpecialSEnd_Main)
    .status(End, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_S_END, status_funky_SpecialSEnd_End)
    .install();
}