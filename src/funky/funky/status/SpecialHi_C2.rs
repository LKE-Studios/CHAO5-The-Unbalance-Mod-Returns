use crate::imports::BuildImports::*;

unsafe extern "C" fn status_funky_SpecialHi_C2_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, 0, 0, 0, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn status_funky_SpecialHi_C2_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let x_acl_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("x_acl_air"));
    let x_acl_ground = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("x_acl_ground"));
    let x_spd_max_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("x_spd_max_air"));
    let x_spd_max_ground = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("x_spd_max_ground"));
    let y_spd_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("y_spd_air"));
    let y_acl_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("y_acl_mul"));
    let mut accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    accel_y *= y_acl_mul;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, x_acl_air);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, x_acl_air);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, x_spd_max_air, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, x_spd_max_air, 0.0);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, y_spd_air, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, y_spd_air);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -accel_y);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_DASH, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, x_acl_ground);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, x_acl_ground);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, x_spd_max_ground, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, x_spd_max_ground, 0.0);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -accel_y);
    }
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, sum_speed_x, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    0.into()
}

unsafe extern "C" fn status_funky_SpecialHi_C2_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_YACL_DEFAULT);
    funky_SpecialHi_C2_motion_function(fighter);
    if !StopModule::is_stop(fighter.module_accessor) {
        funky_SpecialHi_C2_Sub_Status(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(funky_SpecialHi_C2_Sub_Status as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(funky_SpecialHi_C2_Main_loop as *const () as _))
}

unsafe extern "C" fn funky_SpecialHi_C2_motion_function(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi_c2"), -1.0, 1.0, 0.0, false, false);
            effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("sys_spin_wind"), false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_c2"), 0.0, 1.0, false, 0.0, false, false);
            let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
            WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_GROUND_MOT_FRAME) {
                AttackModule::clear_all(fighter.module_accessor);
                let ground_mot_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("ground_mot_frame"));
                MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_c2"), ground_mot_frame, 1.0, 0.0);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_GROUND_SPINEND);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_c2"), -1.0, 1.0, 0.0, false, false);
            }
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_c2"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE);
        }
    }
}

unsafe extern "C" fn funky_SpecialHi_C2_Sub_Status(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_CLIFF_CHECK) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_CLIFF_CHECK);
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        }
    }
    0.into()
}

unsafe extern "C" fn funky_SpecialHi_C2_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let frame = MotionModule::frame(fighter.module_accessor);
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_GROUND_MOT_FRAME) {
                AttackModule::clear_all(fighter.module_accessor);
                let ground_mot_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("ground_mot_frame"));
                let x_dcl_ground = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("x_dcl_ground"));
                MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_c2"), ground_mot_frame, 1.0, 0.0);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_GROUND_SPINEND);
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, sum_speed_x * x_dcl_ground, 0.0);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_c2"), -1.0, 1.0, 0.0, false, false);
                let x_dcl_ground = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("x_dcl_ground"));
                let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_c2"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE);
        }
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi_c2"), -1.0, 1.0, 0.0, false, false);
            effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("sys_spin_wind"), false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_c2"), 0.0, 1.0, false, 0.0, false, false);
            let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
            WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE);
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        let lr = PostureModule::lr(fighter.module_accessor);
        let x_acl_ground = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("x_acl_ground"));
        if ControlModule::get_stick_x(fighter.module_accessor) * lr < 0.0 {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: -x_acl_ground, y: 0.0, z: 0.0});
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn status_funky_SpecialHi_C2_Exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode {_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
    0.into()
}

unsafe extern "C" fn status_funky_SpecialHi_C2_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("donkey")
    .status(Pre, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_HI_C2, status_funky_SpecialHi_C2_Pre)
    .status(Init, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_HI_C2, status_funky_SpecialHi_C2_Init)
    .status(Main, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_HI_C2, status_funky_SpecialHi_C2_Main)
    .status(Exit, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_HI_C2, status_funky_SpecialHi_C2_Exit)
    .status(End, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_HI_C2, status_funky_SpecialHi_C2_End)
    .install();
}