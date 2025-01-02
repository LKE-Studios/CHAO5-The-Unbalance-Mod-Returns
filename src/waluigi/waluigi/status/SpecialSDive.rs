use crate::imports::BuildImports::*;

pub static dive_speed_y : f32 = 3.0;
pub static dive_speed_x : f32 = 0.2;

unsafe extern "C" fn status_waluigi_SpecialSDive_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {	
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, true, *FIGHTER_TREADED_KIND_ENABLE, true, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_waluigi_SpecialSDive_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {	
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        GroundModule::pass_floor(fighter.module_accessor);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_f_end"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(waluigi_SpecialSDive_Main_loop as *const () as _))
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn waluigi_SpecialSDive_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let frame = MotionModule::frame(fighter.module_accessor);
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
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
    }
    if ray_check_pos(module_accessor, 0.0, -0.3, false) == 1 {
        fighter.change_status(FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_S_LANDING.into(), false.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_CLIFF_GROUND);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    };
    if frame >= 2.0 && frame <= 19.0 {
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.2, 1.7);
    };
    if frame >= 20.0 && frame <= 25.0 {
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, dive_speed_x, 0.0);
    };
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_f_end_2"), 0.0, 1.0, true, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn status_waluigi_SpecialSDive_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {
        if motion_kind == hash40("special_air_f_end_2") {
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
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
            }
            if stick_y <= -0.5 {
                GroundModule::pass_floor(fighter.module_accessor);
                if ray_check_pos(module_accessor, 0.0, -0.3, false) == 1 {
                    fighter.change_status(FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_S_LANDING.into(), false.into());
                    StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
                    KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                };
            } 
            else {
                GroundModule::clear_pass_floor(fighter.module_accessor);
                if ray_check_pos(module_accessor, 0.0, -0.3, true) == 1 {
                    fighter.change_status(FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_S_LANDING.into(), false.into());
                    StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
                    KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                };
            };
            if frame >= 0.0 {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.25, -dive_speed_y);
                sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, -dive_speed_y);
            };
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                MotionModule::set_frame(fighter.module_accessor, 176.0, true);
                KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.8, z: 0.0});
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
            };
        }
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_waluigi_SpecialSDive_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("dolly")
    .status(Pre, *FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_S_DIVE, status_waluigi_SpecialSDive_Pre)
    .status(Main, *FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_S_DIVE, status_waluigi_SpecialSDive_Main)
    .status(Exec, *FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_S_DIVE, status_waluigi_SpecialSDive_Exec)
    .status(End, *FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_S_DIVE, status_waluigi_SpecialSDive_End)
    .install();
}