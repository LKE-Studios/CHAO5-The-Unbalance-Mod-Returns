use crate::imports::BuildImports::*;
use crate::kamek::kamek::status::SpecialHi::*;

pub static warp_speed_mul : f32 = 2.2;
pub static warp_speed_add : f32 = 1.5;
pub static max_frame : i32 = 40;
pub static button_add_warp_time : i32 = 5;

unsafe extern "C" fn status_kamek_SpecialHiHold_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, (*FIGHTER_STATUS_ATTR_DISABLE_DISSOLVE_CURSOR | *FIGHTER_STATUS_ATTR_HIDE_NAME_CURSOR) as u32, (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32, 0);
        0.into()
    }
    else {
        original_status(Pre, fighter, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD)(fighter)
    } 
}

unsafe extern "C" fn status_kamek_SpecialHiHold_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        JostleModule::set_status(fighter.module_accessor, false);
        let lr = PostureModule::lr(fighter.module_accessor);
        let mut stick_x =  ControlModule::get_stick_x(fighter.module_accessor) * lr;
        let mut stick_y =  ControlModule::get_stick_y(fighter.module_accessor);
        let warp_stick = 0.3;
        if (stick_x.abs() < warp_stick && stick_y.abs() < warp_stick) {
            stick_x = 0.0;
            stick_y = 1.0;
        }
        else if (stick_y.abs() < warp_stick) {
            stick_y = 0.0;
        }
        else if (stick_x.abs() < warp_stick) {
            stick_x = 0.0;
        }
        let normalized = sv_math::vec2_normalize(stick_x, stick_y);
        let mut speed_x= normalized.x * (warp_speed_mul + warp_speed_add) * lr;
        let mut speed_y= normalized.y * (warp_speed_mul + warp_speed_add);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            stick_y = stick_y.max(0.0);
            if stick_y > warp_stick {
                fighter.set_situation(SITUATION_KIND_AIR.into());
            }
            if stick_y < warp_stick {
                speed_y = 0.0;
            }
        }
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, speed_y);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, speed_y);
        0.into()
    }
    else {
        original_status(Init, fighter, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD)(fighter)
    }
}

unsafe extern "C" fn status_kamek_SpecialHiHold_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        WorkModule::set_int(fighter.module_accessor, max_frame, *FIGHTER_NESS_STATUS_SPECIAL_HI_WORK_INT_STOP_TIME);
        WorkModule::set_int(fighter.module_accessor, max_frame, *FIGHTER_NESS_STATUS_SPECIAL_HI_WORK_INT_TIME);
        GroundModule::set_passable_check(fighter.module_accessor, true);
        VisibilityModule::set_whole(fighter.module_accessor, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
        WorkModule::set_flag(fighter.module_accessor, false,*FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SHEIK_SPECIAL_HI_AIR);
        KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        let correct = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP
        } 
        else {
            *GROUND_CORRECT_KIND_AIR
        };
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(correct));
        fighter.sub_shift_status_main(L2CValue::Ptr(kamek_SpecialHiHold_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD)(fighter)
    }
}

unsafe extern "C" fn kamek_SpecialHiHold_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let max_frame_int = WorkModule::get_int(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_HI_WORK_INT_TIME) <= 0;
    let min_frame_int = WorkModule::get_int(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_HI_WORK_INT_TIME) <= WorkModule::get_int(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_HI_WORK_INT_STOP_TIME) - 8;
    let stop_cliff = fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND &&
    GroundModule::is_near_cliff(fighter.module_accessor, 5.0, 5.0) && min_frame_int;
    let stop_ground = fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND &&
    fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR && sum_speed_y < 0.0;
    let stop_air = fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR && sum_speed_y < 0.0 && (
        GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) ||
        GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) ||
        GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) ||
        GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
    );
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        WorkModule::add_int(fighter.module_accessor, button_add_warp_time, *FIGHTER_NESS_STATUS_SPECIAL_HI_WORK_INT_TIME);
    }
    if max_frame_int || stop_ground || stop_air || stop_cliff {
        fighter.change_status(FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END.into(),false.into());
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn status_kamek_SpecialHiHold_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_HI_WORK_INT_TIME);
        0.into()
    }
    else {
        original_status(Exec, fighter, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD)(fighter)
    }
}

unsafe extern "C" fn status_kamek_SpecialHiHold_Exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        WorkModule::set_flag(fighter.module_accessor, true,*FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        VisibilityModule::set_whole(fighter.module_accessor, true);
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD)(fighter)
    }
}

unsafe extern "C" fn status_kamek_SpecialHiHold_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD)(fighter)
    }
}

pub fn install() {
    Agent::new("ness")
    .status(Pre, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, status_kamek_SpecialHiHold_Pre)
    .status(Init, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, status_kamek_SpecialHiHold_Init)
    .status(Main, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, status_kamek_SpecialHiHold_Main)
    .status(Exec, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, status_kamek_SpecialHiHold_Exec)
    .status(Exit, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, status_kamek_SpecialHiHold_Exit)
    .status(End, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, status_kamek_SpecialHiHold_End)
    .install();
}