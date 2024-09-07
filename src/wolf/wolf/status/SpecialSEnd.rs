use crate::imports::BuildImports::*; 

unsafe extern "C" fn status_wolf_SpecialSEnd_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn status_wolf_SpecialSEnd_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let illusion_stop_y_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_stop_y_frame"));
    WorkModule::set_int(fighter.module_accessor, illusion_stop_y_frame, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    let illusion_end_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_end_brake_x"));
    let illusion_end_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_end_speed_x"));
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let illusion_end_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_end_brake_x"));
        let illusion_end_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_end_speed_x"));
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_end_brake_x, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_end_speed_x * lr, 0.0);
    }
    else {
        let illusion_end_air_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_end_air_brake_x"));
        let illusion_end_air_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_end_air_speed_x"));
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_end_air_brake_x, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_end_air_speed_x * lr, 0.0);
    }
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 10.0, 10.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    let illusion_end_c3_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_end_c3_speed_y"));
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, illusion_end_c3_speed_y);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 10.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    0.into()
}

unsafe extern "C" fn status_wolf_SpecialSEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_end"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(wolf_SpecialSEnd_Sub_status as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(wolf_SpecialSEnd_Main_Loop as *const () as _))
}

unsafe extern "C" fn wolf_SpecialSEnd_Sub_status(fighter: &mut L2CFighterCommon, param_3: L2CValue) -> L2CValue {
    if param_3.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_COUNT);
    }
    0.into()
}

unsafe extern "C" fn wolf_SpecialSEnd_Main_Loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                return 0.into();
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
                return 0.into();
            }
        }
        else if StatusModule::is_situation_changed(fighter.module_accessor) {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_end"), L2CValue::Hash40s("special_air_s_end"), true.into());
            fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
                return 0.into();
            }
            else {
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
            }
        }
    }
    let revert_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_COUNT);
    let revert_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_FRAME);
    let mut angle = 0.0;
    if revert_count < revert_frame && 0 < revert_frame {
        let rush_degree = WorkModule::get_float(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_RUSH_DEGREE);
        angle = rush_degree - (rush_degree * revert_count as f32 / revert_frame as f32);
    }
    PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: -angle, y: 0.0, z: 0.0}, 0);
    0.into()
}

unsafe extern "C" fn status_wolf_SpecialSEnd_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        return 0.into();
    }
    let illusion_end_air_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_end_air_brake_x"));
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    if illusion_end_air_brake_x != sv_kinetic_energy::get_brake_x(fighter.lua_state_agent) {
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_end_air_brake_x, 0.0);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME) == 0 {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let illusion_end_air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_end_air_accel_y"));
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -illusion_end_air_accel_y);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    0.into()
}

unsafe extern "C" fn status_wolf_SpecialSEnd_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("wolf")
    .status(Pre, FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_END, status_wolf_SpecialSEnd_Pre)
    .status(Init, FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_END, status_wolf_SpecialSEnd_Init)
    .status(Main, FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_END, status_wolf_SpecialSEnd_Main)
    .status(Exec, FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_END, status_wolf_SpecialSEnd_Exec)
    .status(End, FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_END, status_wolf_SpecialSEnd_End)
    .install();
}