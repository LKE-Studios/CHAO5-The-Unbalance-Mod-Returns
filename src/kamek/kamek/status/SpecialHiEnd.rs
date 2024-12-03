use crate::imports::BuildImports::*;
use crate::kamek::kamek::status::SpecialHi::*;

unsafe extern "C" fn status_kamek_SpecialHiEnd_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32, 0);
        0.into()
    }
    else {
        original_status(Init, fighter, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END)(fighter)
    }
}

unsafe extern "C" fn status_kamek_SpecialHiEnd_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let warp_xy = 0.3;
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x * warp_xy, sum_speed_y * warp_xy);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        0.into()
    }
    else {
        original_status(Init, fighter, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END)(fighter)
    }
}

unsafe extern "C" fn status_kamek_SpecialHiEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        kamek_SpecialHi_status_helper(fighter, true, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END);
        fighter.sub_shift_status_main(L2CValue::Ptr(kamek_SpecialHiEnd_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END)(fighter)
    }
}

unsafe extern "C" fn kamek_SpecialHiEnd_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND 
        && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),false.into());
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        kamek_SpecialHi_status_helper(fighter, false, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND { 
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn status_kamek_SpecialHiEnd_End(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    .status(Pre, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, status_kamek_SpecialHiEnd_Pre)
    .status(Init, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, status_kamek_SpecialHiEnd_Init)
    .status(Main, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, status_kamek_SpecialHiEnd_Main)
    .status(End, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, status_kamek_SpecialHiEnd_End)
    .install();
}