use crate::imports::BuildImports::*;

#[status_script( agent = "gamewatch", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS )]
unsafe fn status_gamewatch_SpecialLw_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    0.into()
}

#[status_script( agent = "gamewatch", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_gamewatch_SpecialLw_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let gauge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_GAUGE);
    if gauge < 3.0 {
        fighter.change_status(FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_WAIT_START.into(), false.into());
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_CHARGE_MAX);
        effect!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_REMOVE_COMMON, Hash40::new_raw(0xaec2db62e));
        fighter.change_status(FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_SHOOT.into(), false.into());
    }
    1.into()
}

#[status_script( agent = "gamewatch", status = FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_WAIT_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_gamewatch_SpecialLwWaitStart_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_BUTTON_RELEASE);
    let lr = PostureModule::lr(fighter.module_accessor);
    WorkModule::set_float(fighter.module_accessor, lr, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_WORK_FLOAT_LR);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_WORK_INT_REFLECT_COUNT);
    fighter.sub_shift_status_main(L2CValue::Ptr(gamewatch_SpecialLwWaitStart_Main_Loop as *const () as _))
}

unsafe fn gamewatch_special_lw_bucket_function(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_BUCKET) {
        gamewatch_special_lw_gauge_function(fighter);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_BUCKET);
    }
}

unsafe fn gamewatch_special_lw_gauge_function(fighter: &mut L2CFighterCommon) {
    let gauge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_GAUGE);
    let gauge_level = match gauge {
        1.0 => hash40("oil_1"), 
        2.0 => hash40("oil_2"),
        3.0 => hash40("oil_3"),
        _ => hash40("oil_none"),
    };
    VisibilityModule::set_int64(fighter.module_accessor, Hash40::new("oil").hash as i64, Hash40::new_raw(gauge_level).hash as i64);
}

unsafe extern "C" fn gamewatch_SpecialLwWaitStart_Main_Loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if !StatusModule::is_changing(fighter.module_accessor) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_WAIT.into(), false.into());
            return 0.into()
        }
    }
    if situation_kind != *SITUATION_KIND_GROUND || situation_kind != *SITUATION_KIND_GROUND && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GAMEWATCH_SPECIAL_AIR_LW);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE);
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND || situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_start"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE);
        }
    }
    gamewatch_special_lw_bucket_function(fighter);
    return 0.into()
}

#[status_script( agent = "gamewatch", status = FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS )]
unsafe extern "C" fn status_gamewatch_SpecialLwWait_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    0.into()
}

#[status_script( agent = "gamewatch", status = FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS )]
unsafe fn status_gamewatch_SpecialLwShoot_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    let attack_min = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("attack_min"));
    let mut special_lw_attack = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_ATTACK);
    if special_lw_attack < attack_min {
        special_lw_attack = attack_min;
    }
    WorkModule::set_float(fighter.module_accessor, special_lw_attack, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_WORK_FLOAT_ATTACK);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        status_gamewatch_SpecialLw_Init,
        status_gamewatch_SpecialLw_Main,
        status_gamewatch_SpecialLwWaitStart_Main,
        status_gamewatch_SpecialLwWait_Exec,
        status_gamewatch_SpecialLwShoot_Init
    );
}
