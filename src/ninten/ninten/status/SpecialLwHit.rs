use crate::imports::BuildImports::*;

unsafe extern "C" fn status_ninten_SpecialLwHit_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE);
        fighter.sub_change_motion_by_situation(Hash40::new("special_lw_hit").into(), Hash40::new("special_air_lw_hit").into(), false.into());
        fighter.sub_set_ground_correct_by_situation(true.into());
        if !StopModule::is_stop(fighter.module_accessor) {
            ninten_SpecialLwHit_sub_status(fighter, false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ninten_SpecialLwHit_sub_status as *const () as _));
        fighter.sub_shift_status_main(L2CValue::Ptr(ninten_SpecialLwHit_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT)(fighter)
    }
}

unsafe extern "C" fn ninten_SpecialLwHit_sub_status(fighter: &mut L2CFighterCommon, param_3: L2CValue) -> L2CValue {
    if param_3.get_bool() {
        let int_time = WorkModule::get_int(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_TIME);
        if int_time > 0 {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_TIME);
        }
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_STOP_Y_TIME);
        let int_stop_y_time = WorkModule::get_int(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_STOP_Y_TIME);
        if int_stop_y_time > 0 {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                if !StatusModule::is_changing(fighter.module_accessor) {
                    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_LW_FLAG_BUTTON_RELEASE);
                    }
                }
            }
        }
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    0.into()
}

unsafe extern "C" fn ninten_SpecialLwHit_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_set_ground_correct_by_situation(false.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("ness")
    .status(Main, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, status_ninten_SpecialLwHit_Main)
    .install();
}