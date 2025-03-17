use crate::imports::BuildImports::*;

unsafe extern "C" fn status_bandana_SpecialSCharge_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        fighter.sub_change_motion_by_situation(Hash40::new("special_s_hold").into(), Hash40::new("special_s_air_hold").into(), false.into());
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_EDGE_STATUS_SPECIAL_S_INT_HOLD_FRAME);
        fighter.sub_shift_status_main(L2CValue::Ptr(bandana_SpecialSCharge_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CHARGE)(fighter)
    }
}

unsafe extern "C" fn bandana_kinetic_function(fighter: &mut L2CFighterCommon, param_1: bool) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if !param_1 && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            return 0.into();
        }
        sv_kinetic_energy!(set_needs_set_param, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, false);
        if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
            sv_kinetic_energy!(set_needs_set_param, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, false);
        }
    }
    0.into()
}

unsafe extern "C" fn bandana_SpecialS_reverse_function(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    let turn_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_stick_x"));
    if stick_x <= turn_stick_x {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }
    0.into()
}

unsafe extern "C" fn bandana_SpecialSCharge_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_s_hold").into(), Hash40::new("special_s_air_hold").into(), true.into());
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_s").into());
        bandana_kinetic_function(fighter, false);
    }
    WorkModule::add_int(fighter.module_accessor, 1, *FIGHTER_BANDANA_INSTANCE_WORK_ID_INT_SPECIAL_S_CHARGE);
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_BANDANA_INSTANCE_WORK_ID_INT_SPECIAL_S_STORE_FRAME);
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if fighter.global_table[CURRENT_FRAME].get_f32() > 0.0 {
            bandana_SpecialS_reverse_function(fighter);
            fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_SHOOT.into(), false.into());
        }
    }
    let hold_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("hold_max"));
    let hold_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_S_INT_HOLD_FRAME);
    if hold_max <= hold_frame {
        bandana_SpecialS_reverse_function(fighter);
        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_SHOOT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn status_bandana_SpecialSCharge_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_common_spirits_floor_ice_loop"), 0);
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_edge_special_n04_03"), 0);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_BANDANA_INSTANCE_WORK_ID_INT_SPECIAL_S_CHARGE);
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CHARGE)(fighter)
    }
}

pub fn install() {
    Agent::new("edge")
    .status(Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CHARGE, status_bandana_SpecialSCharge_Main)
    .status(End, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CHARGE, status_bandana_SpecialSCharge_End)
    .install();
}