use crate::imports::BuildImports::*;

unsafe extern "C" fn status_waluigi_SpecialS_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {	
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, true, *FIGHTER_TREADED_KIND_ENABLE, true, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
        0.into()
    }
    else {
        original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
    }
}

unsafe extern "C" fn status_waluigi_SpecialS2_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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

unsafe extern "C" fn status_waluigi_SpecialS_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {	
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            GroundModule::pass_floor(fighter.module_accessor);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_f_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_f_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(waluigi_SpecialS_Main_loop as *const () as _))
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn waluigi_SpecialS_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if motion_kind == hash40("special_air_f_start") {
        if ray_check_pos(module_accessor, 0.0, -0.3, false) == 1 {
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            fighter.change_status(FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_S_DIVE.into(), false.into());
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        };
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
        }
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if MotionModule::frame(fighter.module_accessor) >= 75.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_f_start"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if frame >= 19.0 && frame <= 63.0 {
            if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                fighter.change_status(FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_S_DIVE.into(), false.into());
            };
        }
        if GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) && frame > 8.0 && frame < 90.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP.into(), false.into());
        };
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        };
    }
    if motion_kind == hash40("special_f_start") {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        };
    }
    0.into()
}

unsafe extern "C" fn status_waluigi_SpecialS_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("dolly")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, status_waluigi_SpecialS_Pre)
    .status(Pre, *FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_S, status_waluigi_SpecialS2_Pre)
    .status(Main, *FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_S, status_waluigi_SpecialS_Main)
    .status(End, *FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_S, status_waluigi_SpecialS_End)
    .install();
}