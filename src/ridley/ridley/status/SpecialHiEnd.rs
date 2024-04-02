use crate::imports::BuildImports::*;

unsafe extern "C" fn status_ridley_SpecialHiEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut x_rot = 0.0;
    let mut speed_x = 0.0;
    let mut motion_kind = "invalid";
    let rot_x = PostureModule::rot_x(fighter.module_accessor, 0);
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    let charge_degree_f = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("charge_degree_f"));
    let charge_degree_b = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("charge_degree_b"));
    let charge_degree_hi = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("charge_degree_hi"));
    let charge_degree_lw = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("charge_degree_lw"));
    let charge_status = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_STATUS);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    if charge_status != *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F {
        if charge_status == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B {
            motion_kind = hash40("special_air_hi_charge_end_b");
        }
        if charge_status == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI {
            motion_kind = hash40("special_air_hi_charge_end_hi");
        }
        if charge_status == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_LW {
            motion_kind = hash40("special_air_hi_charge_end_lw");
        }
    }
    else {
        motion_kind = hash40("special_air_hi_charge_end_f");
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion_kind), 0.0, 1.0, false, 0.0, false, false);
    let mut charge_degree = 0.0;
    if status_kind != *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F {
        if status_kind != *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B {
            if status_kind != *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI {
                charge_degree = charge_degree_lw;
            }
            else {
                charge_degree = charge_degree_hi;
            }
        }
        else {
            charge_degree = charge_degree_b;
        }
    }
    else {
        charge_degree = charge_degree_f;
    }
    let deccel_on_end = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("deccel_on_end"));
    let brake_x = charge_degree.to_radians().sin() * deccel_on_end;
    let brake_y = charge_degree.to_radians().cos() * deccel_on_end;
    if brake_x < 0.0 {
        brake_x *= -1.0;
    }
    if brake_y < 0.0 {
        brake_y *= -1.0;
    }
    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, brake_x, brake_y);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    if rot_x >= 180.0 {
        if rot_x < -180.0 {
            x_rot += 360.0;
            speed_x = x_rot;
        }
    }
    else {
        x_rot -= 360.0;
        speed_x = x_rot;
    }
    WorkModule::set_float(fighter.module_accessor, speed_x / end_frame, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_FLOAT_DEGREE_SPEED_X);
    if !StopModule::is_stop(fighter.module_accessor) {
        ridley_SpecialHiEnd_Sub_Status(fighter, false);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ridley_SpecialHiEnd_Sub_Status as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ridley_SpecialHiEnd_Main_loop as *const () as _))
}

unsafe extern "C" fn ridley_SpecialHiEnd_Sub_Status(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        let rot_x = PostureModule::rot_x(fighter.module_accessor, 0);
        let degree_speed_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_FLOAT_DEGREE_SPEED_X);
        if degree_speed_x < 180.0 {
            
        }
        else {

        }
        ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }
    ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    0.into()
}

unsafe extern "C" fn ridley_SpecialHiEnd_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            let sum_speed = KineticModule::get_sum_speed(fighter.module_accessor, -1);
            fighter.Vector2__create(sum_speed.into(), sum_speed.into());
            let deccel_on_end = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("deccel_on_end"));
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, deccel_on_end.x);
            sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
            if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            }
            else {
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, deccel_on_end.y);
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_GRAVITY) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_GRAVITY);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            let sum_speed = KineticModule::get_sum_speed(fighter.module_accessor, -1);
            fighter.Vector2__create(sum_speed.into(), sum_speed.into());
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, sum_speed.y);
            sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
            if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            }
            else {
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0.x);
            }
        }
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            if !MotionModule::is_end(fighter.module_accessor) {
                let landing_fall_special_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_fall_special_frame"));
                let max_speed_x_on_fall_special = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("max_speed_x_on_fall_special"));
                WorkModule::set_float(fighter.module_accessor, landing_fall_special_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                WorkModule::set_float(fighter.module_accessor, max_speed_x_on_fall_special, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
                return 0.into()
            }
        }
        else {
            let landing_fall_special_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_fall_special_frame"));
            WorkModule::set_float(fighter.module_accessor, landing_fall_special_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 0.into()  
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("ridley")
    .status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_END, status_ridley_SpecialHiEnd_Main)
    .install();
}