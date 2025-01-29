use crate::imports::BuildImports::*;

unsafe extern "C" fn status_metaknight_SpecialS_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_WORK_FLAG_HIT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_WORK_FLAG_GUARD_OR_INVINCIBLE);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_s_start") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s_start") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);  
    metaknight_special_s_motion_handler(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_SpecialS_Main_loop as *const () as _))
}

unsafe extern "C" fn metaknight_special_s_motion_handler(fighter: &mut L2CFighterCommon) {
    let mot_air_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
    let mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_AIR_S);
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_air_kind), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_air_kind), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_S);
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_kind), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_kind), 0.0, 1.0, false, 0.0, false, false);
    }
}

unsafe extern "C" fn metaknight_SpecialS_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut bool_set: bool = false;
    let mot_air_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
    let mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_air_kind), -1.0, 1.0, 0.0, false, false);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_kind), -1.0, 1.0, 0.0, false, false);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH.into(), false.into());
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                bool_set = true;
            }
        }
    }
    if bool_set {
        metaknight_special_s_motion_handler(fighter);
    }
    0.into()
}

pub fn install() {
    Agent::new("metaknight")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, status_metaknight_SpecialS_Main)
    .install();
}