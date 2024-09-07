use crate::imports::BuildImports::*;

pub static mut PFUSHIGISOU_SOLAR_BEAM_TIMER : [i32; 8] = [0; 8];

pub unsafe extern "C" fn status_pfushigisou_SpecialGuard_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_FALL, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

pub unsafe extern "C" fn status_pfushigisou_SpecialGuard_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_z_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_z_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(pfushigisou_SpecialGuard_Main_loop as *const () as _))
}

unsafe extern "C" fn pfushigisou_SpecialGuard_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() || fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            SoundModule::play_landing_se(fighter.module_accessor, Hash40::new("se_pfushigisou_landing01"));
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_z_start"), -1.0, 1.0, 0.0, false, false);
        }
        if MotionModule::is_end(fighter.module_accessor) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_z_charge"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_z_start"), -1.0, 1.0, 0.0, false, false);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        }
        if MotionModule::is_end(fighter.module_accessor) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_z_charge"), 0.0, 1.0, false, 0.0, false, false);
        }
    }     
    let charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_guard"), hash40("charge_frame"));
    PFUSHIGISOU_SOLAR_BEAM_TIMER[ENTRY_ID] += 1;
    if PFUSHIGISOU_SOLAR_BEAM_TIMER[ENTRY_ID] >= charge_frame {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD_SHOOT.into(), false.into());
        }
    }
    if PFUSHIGISOU_SOLAR_BEAM_TIMER[ENTRY_ID] == charge_frame {
        gimmick_flash(fighter);
        SoundModule::play_se(fighter.module_accessor, Hash40::new("se_pfushigisou_appeal_l03"), true, false, false, false, enSEType(0));
        SoundModule::play_se(fighter.module_accessor, Hash40::new("se_pfushigisou_special_n01"), true, false, false, false, enSEType(0));
    }
    0.into()
}

unsafe extern "C" fn status_pfushigisou_SpecialGuard_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_z_charge") {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                if frame >= 120.0 {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_z_charge"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            if ControlModule::check_button_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                fighter.change_status(FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD_END.into(), false.into());
            }
            if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_z_charge"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    else {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_z_charge") {
            if lr * stick_x >= 0.5 {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), false.into());
            }
            if lr * stick_x <= -0.5 {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), false.into());
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                if frame >= 120.0 {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_z_charge"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            if ControlModule::check_button_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || 
            ControlModule::check_button_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                fighter.change_status(FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD_END.into(), false.into());
            }
            if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_z_charge"), -1.0, 1.0, 0.0, false, false);
                sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            }
        }    
    }
    0.into()
}  

pub unsafe extern "C" fn status_pfushigisou_SpecialGuard_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_pfushigisou_special_n01"), 0);
    SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_pfushigisou_recovery"), 0);
    0.into()
}

pub fn install() {
    Agent::new("pfushigisou")
    .status(Pre, FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD, status_pfushigisou_SpecialGuard_Pre)
    .status(Main, FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD, status_pfushigisou_SpecialGuard_Main)
    .status(Exec, FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD, status_pfushigisou_SpecialGuard_Exec)
    .status(End, FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD, status_pfushigisou_SpecialGuard_End)
    .install();
}