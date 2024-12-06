use crate::imports::BuildImports::*;

pub static initial_speed_y : f32 = 0.3;
pub static gravity_speed : f32 = 0.25;
pub static gravity_accel_y : f32 = 0.19;
pub static landing_frame : f32 = 11.0;

unsafe extern "C" fn status_kamek_SpecialHi_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {	
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
        0.into()
    }
    else {
        original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
    }
}

unsafe extern "C" fn status_kamek_SpecialHi_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, initial_speed_y);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -gravity_accel_y);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -gravity_speed);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        0.into()
    }
    else {
        0.into()
    }
}

pub unsafe extern "C" fn kamek_SpecialHi_status_helper(fighter: &mut L2CFighterCommon, is_start: bool, status: i32) {
    let motion_g;
    let motion_a;
    if status == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD {
        motion_g = Hash40::new("special_hi_hold");
        motion_a = Hash40::new("special_hi_hold");
    }
    else if status == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END {
        motion_g = Hash40::new("special_hi_end");
        motion_a = Hash40::new("special_air_hi_end");
    }
    else {
        motion_g = Hash40::new("special_hi_start");
        motion_a = Hash40::new("special_air_hi_start");
    }
    if is_start && false {
        let motion = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {motion_g} else {motion_a};
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0,1.0, false, 0.0, false, false);
    }
    else {
        fighter.sub_change_motion_by_situation(motion_g.into(), motion_a.into(), (!is_start).into());
    }
    let correct = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {*GROUND_CORRECT_KIND_GROUND} else {*GROUND_CORRECT_KIND_AIR};
    GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(correct));
    if status == *FIGHTER_STATUS_KIND_SPECIAL_HI || !is_start {
        let air_kinetic = if status == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END {FIGHTER_KINETIC_TYPE_AIR_STOP} else {FIGHTER_KINETIC_TYPE_SHEIK_SPECIAL_HI_AIR};
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),air_kinetic.into());
    } 
    
}

unsafe extern "C" fn status_kamek_SpecialHi_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        kamek_SpecialHi_status_helper(fighter, true, *FIGHTER_STATUS_KIND_SPECIAL_HI);
        WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.sub_shift_status_main(L2CValue::Ptr(kamek_SpecialHi_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
    }
}

unsafe extern "C" fn kamek_SpecialHi_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        kamek_SpecialHi_status_helper(fighter, false, *FIGHTER_STATUS_KIND_SPECIAL_HI);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KAMEK_STATUS_KIND_SPECIAL_HI_WARP.into(),false.into());
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn status_kamek_SpecialHi_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        0.into()
    }
    else {
        0.into()
    }
}

pub fn install() {
    Agent::new("ness")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_kamek_SpecialHi_Pre)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_kamek_SpecialHi_Init)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_kamek_SpecialHi_Main)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_kamek_SpecialHi_End)
    .install();
}