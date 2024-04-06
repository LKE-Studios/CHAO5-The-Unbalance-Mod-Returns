use crate::imports::BuildImports::*;

unsafe extern "C" fn status_silver_SpecialHi_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SILVER = color >= 120 && color <= 127;
    if SILVER {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        KineticModule::clear_speed_all(fighter.module_accessor);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        if !StopModule::is_stop(fighter.module_accessor) {
            silver_SpecialHi_Sub_Status(fighter, false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(silver_SpecialHi_Sub_Status as *const () as _));
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01 - 1);
        fighter.sub_shift_status_main(L2CValue::Ptr(silver_SpecialHi_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
    } 
}

unsafe extern "C" fn silver_SpecialHi_Sub_Status(fighter: &mut L2CFighterCommon, param_3: L2CValue) -> L2CValue {
    if !param_3.get_bool() {
        WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_MOVE_WAIT_FRAME);
    }
    0.into()
}

unsafe extern "C" fn silver_SpecialHi_function(fighter: &mut L2CFighterCommon) {
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    VisibilityModule::set_whole(fighter.module_accessor, false);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
    WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    GroundModule::set_passable_check(fighter.module_accessor, true);
    let int_cliff_check = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_CLIFF_CHECK);
    fighter.sub_fighter_cliff_check(int_cliff_check.into());
}

unsafe extern "C" fn silver_SpecialHi_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //For Silver
    fighter.change_status(FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3.into(), true.into());
    //Vanilla Main_loop for Mewtwo
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_FLAG_MOVE_WAIT) {
            let wait_frame = WorkModule::get_int(fighter.module_accessor, FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_MOVE_WAIT_FRAME);
            if wait_frame >= 0 {
                fighter.change_status(FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2.into(), false.into());
            }
        }
        else {
            if MotionModule::is_end(fighter.module_accessor) {
                let move_wait_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("move_wait_frame"));
                let cliff_check = GroundModule::cliff_check(fighter.module_accessor);
                WorkModule::set_int(fighter.module_accessor, move_wait_frame, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_MOVE_WAIT_FRAME);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_FLAG_MOVE_WAIT);
                WorkModule::set_int(fighter.module_accessor, cliff_check, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_CLIFF_CHECK);
                silver_SpecialHi_function(fighter);
            }
        }
        return 0.into();
    }
    else {
        return 1.into();
    }
    0.into()
}

pub fn install() {
    Agent::new("mewtwo")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_silver_SpecialHi_Main)
    .install();
}
