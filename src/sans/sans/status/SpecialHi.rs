use crate::imports::BuildImports::*;

unsafe extern "C" fn status_sans_SpecialHi_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SANS = color >= 120 && color <= 127;
    if SANS {
        let special_hi_move_time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_move_time"));
        WorkModule::set_int(fighter.module_accessor, special_hi_move_time, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_WORK_INT_MOVE_XLU);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01 - 1);
        fighter.sub_shift_status_main(L2CValue::Ptr(sans_SpecialHi_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
    } 
}

unsafe extern "C" fn sans_SpecialHi_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if situation_kind != *SITUATION_KIND_GROUND || situation_kind != *SITUATION_KIND_GROUND && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_PALUTENA_SPECIAL_HI_AIR);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_start"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE);
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND || situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn status_sans_SpecialHi_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SANS = color >= 120 && color <= 127;
    if SANS {
        let cliff_check = GroundModule::cliff_check(fighter.module_accessor) as i32;
        WorkModule::set_int(fighter.module_accessor, cliff_check, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_WORK_INT_CLIFF_CHECK);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("rockman_hardknuckle"), false, false);
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
    } 
}

pub fn install() {
    Agent::new("palutena")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_sans_SpecialHi_Main)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_sans_SpecialHi_End)
    .install();
}