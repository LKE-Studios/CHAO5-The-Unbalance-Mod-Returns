use crate::imports::BuildImports::*;
use crate::diddy::diddy::frame::*;

unsafe extern "C" fn status_diddy_SpecialLwLaugh_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    BANANA_EXIST[ENTRY_ID] = false;
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn status_diddy_SpecialLwLaugh_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    diddy_SpecialLwLaugh_helper(fighter);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.set_situation(SITUATION_KIND_AIR.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_laugh"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_laugh"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(diddy_SpecialLwLaugh_Main_loop as *const () as _))
}

unsafe extern "C" fn diddy_SpecialLwLaugh_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    let object_id = fighter.global_table[OBJECT_ID].get_u32();
    let itemmanager = smash2::app::ItemManager::instance().unwrap();
    let banana_count = smash2::app::ItemManager::get_num_of_ownered_item(itemmanager, object_id, smash2::app::ItemKind::Banana) as i32;
    let special_lw_banana_num_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_banana_num_max"));
    let interval_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DIDDY_INSTANCE_WORK_ID_INT_SPECIAL_LW_INTERVAL_FRAME);
    if banana_count >= special_lw_banana_num_max {
        if interval_frame == 0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_LW_FLAG_PUTOUT_CONDITION_OK);
        }
    }
    0.into()
}

pub unsafe extern "C" fn diddy_SpecialLwLaugh_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_laugh"), -1.0, 1.0, 0.0, false, false);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_laugh"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into()
    }
    0.into()
}

pub fn install() {
    Agent::new("diddy")
    .status(Pre, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_LW_LAUGH, status_diddy_SpecialLwLaugh_Pre)
    .status(Main, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_LW_LAUGH, status_diddy_SpecialLwLaugh_Main)
    .install();
}