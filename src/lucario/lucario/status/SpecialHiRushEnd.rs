use crate::imports::BuildImports::*;

unsafe extern "C" fn status_lucario_SpecialHiRushEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if situation != *SITUATION_KIND_GROUND {
        FighterSpecializer_Lucario::set_mach_validity(fighter.module_accessor, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
        GroundModule::set_cliff_check(fighter.module_accessor, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        let end_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("end_brake_x"));
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, end_brake_x, 0.0);
        let end_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("end_accel_y"));
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -end_accel_y);
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y.clamp(-5.0, 2.0));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_end"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_end"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    }
    WorkModule::set_int(fighter.module_accessor, situation, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_RUSH_END_SITUATION);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_SpecialHiRushEnd_Main_loop as *const () as _))
}

unsafe extern "C" fn lucario_SpecialHiRushEnd_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let sit = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_RUSH_END_SITUATION);
    if sit != *SITUATION_KIND_GROUND
    && fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if sit != *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
            }
            return 0.into();
        }
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_AIR_END_CONTROL_X) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_AIR_END_CONTROL_X);
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into();
        }
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
            }
            return 0.into();
        }
    }
    let cont = if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) {
        lucario_SpecialHi_attach_wall(fighter, (1.0_f32).into()).get_bool()
    }
    else if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) {
        lucario_SpecialHi_attach_wall(fighter, (-1.0_f32).into()).get_bool()
    }
    else { false };
    if cont {
        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACH_WALL, false);
    }
    0.into()
}

pub unsafe extern "C" fn lucario_SpecialHi_attach_wall(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    let no_attach = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_ATTACH_WALL_FRAME);
    let wall_jump_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("wall_jump_stick_x"));
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    if no_attach <= 0 {
        if wall_jump_stick_x <= stick_x.abs() {
            let lr = param_1.get_f32();
            if lr * stick_x < 0.0 {
                let vector;
                let some = if 0.0 > lr {
                    vector = Vector2f {
                        x: GroundModule::get_touch_normal_x_consider_gravity(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32),
                        y: GroundModule::get_touch_normal_y_consider_gravity(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32)
                    };
                    1.0
                }
                else {
                    vector = Vector2f {
                        x: GroundModule::get_touch_normal_x_consider_gravity(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32),
                        y: GroundModule::get_touch_normal_y_consider_gravity(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32)
                    };
                    -1.0
                };
                let atan = vector.x.atan2(vector.y) * some;
                let deg = atan.to_degrees() + 90.0;
                let attach_wall_slant_max = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attach_wall_slant_max"));
                let attach_wall_slant_min = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attach_wall_slant_min"));
                if attach_wall_slant_max > deg
                && deg > attach_wall_slant_min {
                    if !GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) {
                        let attach_on = if 0.0 <= lr {
                            *GROUND_TOUCH_FLAG_LEFT
                        }
                        else { *GROUND_TOUCH_FLAG_RIGHT };
                        if GroundModule::is_attachable(fighter.module_accessor, GroundTouchFlag(attach_on)) {
                            return true.into();
                        }
                    }
                }
            }
        }
    }
    false.into()
}

pub fn install() {
    Agent::new("lucario")
    .status(Main, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END, status_lucario_SpecialHiRushEnd_Main)
    .install();
}