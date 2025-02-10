use crate::imports::BuildImports::*;

pub static special_s_start_air_accel_y_mul : f32 = 2.0;
pub static special_s_start_speed_x_mul : f32 = 1.0;
pub static special_s_start_speed_y_add : f32 = 1.0;

pub unsafe extern "C" fn status_funky_SpecialS_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_GLIDE_START, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
        0.into()
    }
    else {
        original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
    }
}

pub unsafe extern "C" fn status_funky_SpecialS_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut smash::app::KineticEnergy;
        let motion = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut smash::app::KineticEnergy;
        let lr = PostureModule::lr(fighter.module_accessor);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GLIDE_START);
        KineticEnergy::reset_energy(gravity, *ENERGY_GRAVITY_RESET_TYPE_GLIDE_START, &Vector2f{x: 0.0, y: -special_s_start_air_accel_y_mul}, &Vector3f{x: 0.0, y: -special_s_start_air_accel_y_mul, z: 0.0}, fighter.module_accessor);
        KineticEnergy::reset_energy(motion, *ENERGY_GRAVITY_RESET_TYPE_GLIDE_START, &Vector2f{x: special_s_start_speed_x_mul * lr, y: 0.0}, &Vector3f{x: special_s_start_speed_x_mul * lr, y: 0.0, z: 0.0}, fighter.module_accessor);
        KineticUtility::reset_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_GLIDE_START, Vector2f{x: 0.0, y: special_s_start_speed_y_add}, Vector3f{x: 0.0, y: special_s_start_speed_y_add, z: 0.0});
        0.into()
    }
    else {
        original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
    }
}

pub unsafe extern "C" fn status_funky_SpecialS_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        ControlModule::reset_trigger(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FUNKY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
        } 
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(funky_SpecialS_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
    }
}

unsafe extern "C" fn funky_SpecialS_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_FUNKY_STATUS_KIND_SPECIAL_S_LANDING.into(), false.into());
        return 0.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR 
    && MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_FUNKY_STATUS_KIND_SPECIAL_S_FLY.into(), false.into());
        return 0.into();
    }
    0.into()
}

pub unsafe extern "C" fn status_funky_SpecialS_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        0.into()
    }
    else {
        original_status(Exec, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
    }
}

pub unsafe extern "C" fn status_funky_SpecialS_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
    }
}

pub fn install() {
    Agent::new("donkey")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, status_funky_SpecialS_Pre)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, status_funky_SpecialS_Init)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, status_funky_SpecialS_Main)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, status_funky_SpecialS_Exec)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, status_funky_SpecialS_End)
    .install();
}