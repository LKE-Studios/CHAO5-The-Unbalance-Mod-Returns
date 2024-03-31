use crate::imports::BuildImports::*;

unsafe extern "C" fn status_ridley_SpecialHiChargeF_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_CHARGE_DECCEL);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_COUNT);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_charge_f"), 0.0, 1.0, false, 0.0, false, false);
    let charge_frame_f = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("charge_frame_f"));
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    MotionModule::set_rate(fighter.module_accessor, end_frame / charge_frame_f as f32);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let charge_degree_f = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("charge_degree_f"));
    let charge_speed_f = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("charge_speed_f"));
    let lr = PostureModule::lr(fighter.module_accessor);
    let rad = charge_degree_f.to_radians();
    let speed_x = rad.cos() * charge_speed_f * lr;
    let speed_y = rad.sin() * charge_speed_f;
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, speed_x, speed_y, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, speed_y);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, speed_y);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticUtility::reset_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_AIR_TRANS, Vector2f{x: 0.0, y:0.0}, Vector3f{x: 0.0, y: 0.0, z: 0.0});
    fighter.sub_shift_status_main(L2CValue::Ptr(ridley_SpecialHiChargeF_Main_loop as *const () as _))
}

unsafe extern "C" fn ridley_SpecialHiChargeF_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_COUNT);
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
        let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
        FighterSpecializer_Ridley::request_special_hi_wall_effect(module_accessor);
        fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_WALL.into(), false.into());
        return true.into()
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_LANDING_F);
        fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_LANDING.into(), false.into());
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("ridley")
    .status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F, status_ridley_SpecialHiChargeF_Main)
    .install();
}