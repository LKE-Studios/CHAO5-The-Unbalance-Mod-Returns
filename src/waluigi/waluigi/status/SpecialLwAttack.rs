use crate::imports::BuildImports::*;

unsafe extern "C" fn status_waluigi_SpecialLwAttack_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {   
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, true, *FIGHTER_TREADED_KIND_ENABLE, true, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_waluigi_SpecialLwAttack_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            GroundModule::pass_floor(fighter.module_accessor);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_air_attack"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            MotionModule::set_rate(fighter.module_accessor, 2.5);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack_1"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(waluigi_SpecialLwAttack_Main_loop as *const () as _))
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn waluigi_SpecialLwAttack_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
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

unsafe extern "C" fn status_waluigi_SpecialLwAttack_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {
        if motion_kind == hash40("special_lw_attack_1") {
            MotionModule::set_rate(fighter.module_accessor, 2.5);
            if frame >= 20.0 && frame < 28.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack_2"), 0.0, 1.0, false, 0.0, false, false);
                }
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack_special_2"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }
        if motion_kind == hash40("special_lw_attack_2") {
            MotionModule::set_rate(fighter.module_accessor, 2.5);
            if MotionModule::is_end(fighter.module_accessor) {
                if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                }
            }
            if frame >= 29.0 && frame < 48.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack_3"), 0.0, 1.0, false, 0.0, false, false);
                }
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack_special_1"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }
        if [hash40("special_lw_attack_3"), hash40("special_lw_attack_special_1"), hash40("special_lw_attack_special_2")].contains(&motion_kind) {
            MotionModule::set_rate(fighter.module_accessor, 2.5);
            if MotionModule::is_end(fighter.module_accessor) {
                if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                }
            }
        }
        if motion_kind == hash40("special_lw_air_attack") {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            if frame >= 1.0 {
                sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.8, 0.0);
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.8, 0.0);
            }
            if frame >= 15.0 {
                KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            }
            if MotionModule::is_end(fighter.module_accessor) {
                if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                }
                return 1.into()
            }
        }
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_waluigi_SpecialLwAttack_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {
        0.into()
    }
    else {
        0.into()
    }
}

pub fn install() {
    Agent::new("dolly")
    .status(Pre, FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_LW_ATTACK, status_waluigi_SpecialLwAttack_Pre)
    .status(Main, FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_LW_ATTACK, status_waluigi_SpecialLwAttack_Main)
    .status(Exec, FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_LW_ATTACK, status_waluigi_SpecialLwAttack_Exec)
    .status(End, FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_LW_ATTACK, status_waluigi_SpecialLwAttack_End)
    .install();
}