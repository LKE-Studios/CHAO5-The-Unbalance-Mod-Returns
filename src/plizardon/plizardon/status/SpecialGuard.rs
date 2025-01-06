use crate::imports::BuildImports::*;

unsafe extern "C" fn status_plizardon_SpecialGuard_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, *FIGHTER_STATUS_ATTR_START_TURN as u32, (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u32, 0);
    0.into()
}

unsafe extern "C" fn status_plizardon_SpecialGuard_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let y_acl_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_guard"), hash40("y_acl_mul"));
        let x_speed_mul_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_guard"), hash40("x_speed_mul_air"));
        let y_speed_mul_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_guard"), hash40("y_speed_mul_air"));
        let speed_x = sum_speed_x * x_speed_mul_air;
        let speed_y = sum_speed_y * y_speed_mul_air;
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, speed_x, speed_y, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -y_acl_mul);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    else {
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let x_speed_mul_ground = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_guard"), hash40("x_speed_mul_ground"));
        let y_speed_mul_ground = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_guard"), hash40("y_speed_mul_ground"));
        let speed_x = sum_speed_x * x_speed_mul_ground;
        let speed_y = sum_speed_y * y_speed_mul_ground;
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, speed_x, speed_y, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    0.into()
}

unsafe extern "C" fn status_plizardon_SpecialGuard_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_z").into(), Hash40::new("special_air_z").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(plizardon_SpecialGuard_Main_loop as *const () as _))
}

unsafe extern "C" fn plizardon_SpecialGuard_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_set_ground_correct_by_situation(false.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
        fighter.sub_change_motion_by_situation(Hash40::new("special_z").into(), Hash40::new("special_air_z").into(), true.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        let speed_x = {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)
        };
        if speed_x.abs() < 0.1 {
            KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    }
    0.into()
}

unsafe extern "C" fn status_plizardon_SpecialGuard_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let interval_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PLIZARDON_STATUS_ROCKSTONE_WORK_INT_INTERVAL_FRAME_COUNT);
    if interval_count == 0 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PLIZARDON_STATUS_ROCKSTONE_FLAG_GENE_ROCK);
    }
    let rock_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_rockstone"), hash40("rock_frame"));
    WorkModule::set_int(fighter.module_accessor, rock_frame, *FIGHTER_PLIZARDON_STATUS_ROCKSTONE_WORK_INT_INTERVAL_FRAME_COUNT);
    0.into()
}

unsafe extern "C" fn status_plizardon_SpecialGuard_Exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PLIZARDON_GENERATE_ARTICLE_ROCK, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

unsafe extern "C" fn status_plizardon_SpecialGuard_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("plizardon")
    .status(Pre, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_GUARD, status_plizardon_SpecialGuard_Pre)
    .status(Init, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_GUARD, status_plizardon_SpecialGuard_Init)
    .status(Main, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_GUARD, status_plizardon_SpecialGuard_Main)
    .status(Exec, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_GUARD, status_plizardon_SpecialGuard_Exec)
    .status(Exit, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_GUARD, status_plizardon_SpecialGuard_Exit)
    .status(End, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_GUARD, status_plizardon_SpecialGuard_End)
    .install();
}