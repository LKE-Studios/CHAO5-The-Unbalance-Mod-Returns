use crate::imports::BuildImports::*;

pub static max_speed_x : f32 = 1.4;
pub static max_speed_y : f32 = 1.7;
pub static x_acl_air : f32 = 0.044;
pub static y_acl_air : f32 = 0.05;
pub static warp_time : i32 = 180;
pub static button_add_warp_time : i32 = 6;

unsafe extern "C" fn status_ninten_SpecialHiHold_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {	
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_FREE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
        0.into()
    }
    else {
        original_status(Pre, fighter, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD)(fighter)
    }
}

unsafe extern "C" fn status_ninten_SpecialHiHold_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_math::vec2_normalize(sum_speed_x, sum_speed_y);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, max_speed_y);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, max_speed_y);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, max_speed_x, max_speed_y);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, max_speed_x, max_speed_y);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, sum_speed_x, sum_speed_y, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        0.into()
    }
    else {
        original_status(Init, fighter, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD)(fighter)
    }
}

unsafe extern "C" fn status_ninten_SpecialHiHold_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::pass_floor(fighter.module_accessor);
        StatusModule::set_keep_situation_air(fighter.module_accessor, true);
        JostleModule::set_status(fighter.module_accessor, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_hold"), 0.0, 1.0, true, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_hold"), 0.0, 1.0, true, 0.0, false, false);
        }
        let int_time = WorkModule::get_int(fighter.module_accessor, FIGHTER_NINTEN_STATUS_SPECIAL_HI_HOLD_WORK_INT_TIME);
        WorkModule::set_int(fighter.module_accessor, warp_time, FIGHTER_NINTEN_STATUS_SPECIAL_HI_HOLD_WORK_INT_TIME);
        fighter.sub_shift_status_main(L2CValue::Ptr(ninten_SpecialHiHold_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD)(fighter)
    }
}

unsafe extern "C" fn ninten_SpecialHiHold_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);    
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_transition_group_check_air_cliff().get_bool() {
            return 1.into();
        }
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() 
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    let int_time = WorkModule::get_int(fighter.module_accessor, FIGHTER_NINTEN_STATUS_SPECIAL_HI_HOLD_WORK_INT_TIME);
    WorkModule::dec_int(fighter.module_accessor, FIGHTER_NINTEN_STATUS_SPECIAL_HI_HOLD_WORK_INT_TIME);
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        fighter.change_status(FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK.into(), true.into());
        return 0.into();
    }
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
        return 0.into();
    }
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        WorkModule::add_int(fighter.module_accessor, button_add_warp_time, FIGHTER_NINTEN_STATUS_SPECIAL_HI_HOLD_WORK_INT_TIME);
    }
    if MotionModule::is_end(fighter.module_accessor) || int_time == 0 {
        fighter.change_status(FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn status_ninten_SpecialHiHold_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {
        JostleModule::set_status(fighter.module_accessor, true);
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_common_spirits_floor_ice_loop"), 0);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_flash"), false, false);
        0.into()
    }
    else {
        0.into()
    }
}

pub fn install() {
    Agent::new("ness")
    .status(Pre, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, status_ninten_SpecialHiHold_Pre)
    .status(Init, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, status_ninten_SpecialHiHold_Init)
    .status(Main, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, status_ninten_SpecialHiHold_Main)
    .status(End, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, status_ninten_SpecialHiHold_End)
    .install();
}