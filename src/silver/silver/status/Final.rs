use crate::imports::BuildImports::*;

unsafe extern "C" fn status_silver_Final_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SILVER = color >= 120 && color <= 127;
    if SILVER {
        fighter.sub_status_pre_FinalCommon();
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, (*FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | *FIGHTER_STATUS_ATTR_FINAL) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
        0.into()
    }
    else {
        original_status(Pre, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter)
    }
}

unsafe extern "C" fn status_silver_Final_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SILVER = color >= 120 && color <= 127;
    if SILVER {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x201bc9217c));
        fighter.pop_lua_stack(1);
        AreaModule::set_whole(fighter.module_accessor, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("final_air"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("final"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(silver_Final_Main_loop as *const () as _))
    }    
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter)
    }
}

unsafe extern "C" fn silver_Final_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("final_air"), -1.0, 1.0, 0.0, false, false);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("final"), -1.0, 1.0, 0.0, false, false);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn status_silver_Final_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SILVER = color >= 120 && color <= 127;
    if SILVER {
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter)
    }
}

pub fn install() {
    Agent::new("mewtwo")
    .status(Pre, *FIGHTER_STATUS_KIND_FINAL, status_silver_Final_Pre)    
    .status(Main, *FIGHTER_STATUS_KIND_FINAL, status_silver_Final_Main)
    .status(End, *FIGHTER_STATUS_KIND_FINAL, status_silver_Final_End)
    .install();
}