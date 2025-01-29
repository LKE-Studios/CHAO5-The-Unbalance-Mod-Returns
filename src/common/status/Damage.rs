use crate::imports::BuildImports::*;

#[skyline::hook(replace = L2CFighterCommon_ftStatusUniqProcessDamage_init_common)]
unsafe fn ftstatusuniqprocessdamage_init_common(fighter: &mut L2CFighterCommon) {
    let reaction_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_REACTION_FRAME);
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("speed_vec_x"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let speed_vec_x = fighter.pop_lua_stack(1).get_f32();
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("speed_vec_y"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let speed_vec_y = fighter.pop_lua_stack(1).get_f32();
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("attr"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let attr = fighter.pop_lua_stack(1).get_u64();
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if !(0 < reaction_frame as i32) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION);
        WorkModule::set_float(fighter.module_accessor, reaction_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
        WorkModule::set_float(fighter.module_accessor, reaction_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
        }
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("angle"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let angle = fighter.pop_lua_stack(1).get_f32();
    let degrees = angle.to_degrees();
    let speed_vector = sv_math::vec2_length(speed_vec_x, speed_vec_y);
    fighterstatusdamage_init_damage_speed_up_by_speed(fighter, speed_vector.into(), degrees.into(), false.into());
    let damage_cliff_no_catch_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("damage_cliff_no_catch_frame"));
    WorkModule::set_int(fighter.module_accessor, damage_cliff_no_catch_frame, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME);
    let cursor_fly_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("cursor_fly_speed"));
    let pop1squared = speed_vec_x * speed_vec_x;
    let pop2squared = speed_vec_y * speed_vec_y;
    let combined = pop1squared + pop2squared;
    let cursor_fly_speed_squared = cursor_fly_speed * cursor_fly_speed;
    if cursor_fly_speed_squared < combined {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CURSOR);
        let cursor_fly_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cursor_fly_frame"));
        WorkModule::set_int(fighter.module_accessor, cursor_fly_frame, *FIGHTER_INSTANCE_WORK_ID_INT_CURSOR_FRAME);
    }
    let damage_fly_attack_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("damage_fly_attack_frame"));
    let damage_fly_escape_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("damage_fly_escape_frame"));
    WorkModule::set_int(fighter.module_accessor, damage_fly_attack_frame, *FIGHTER_STATUS_DAMAGE_WORK_INT_ATTACK_DISABLE_FRAME);
    WorkModule::set_int(fighter.module_accessor, damage_fly_escape_frame, *FIGHTER_STATUS_DAMAGE_WORK_INT_ESCAPE_DISABLE_FRAME);
    if [hash40("collision_attr_paralyze"), hash40("collision_attr_paralyze_ghost")].contains(&attr) {
        let invalid_paralyze_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("invalid_paralyze_frame"));
        WorkModule::set_float(fighter.module_accessor, invalid_paralyze_frame, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_PARALYZE_FRAME);
    }
}

unsafe extern "C" fn get_angle_factor(angle_threshold: f32, angle: f32) -> f32 {
    let angle_threshold = angle_threshold.to_radians();
    let angle = (90.0 - ((angle % 180.0).abs() - 90.0).abs()).to_radians();
    if angle <= angle_threshold { return 1.0; }
    let angle_factor = ((angle_threshold.cos().powf(2.0) / 640.0_f32.powf(2.0)) + (angle_threshold.sin().powf(2.0) / 360.0_f32.powf(2.0))).sqrt()
        / ((angle.cos().powf(2.0) / 640.0_f32.powf(2.0)) + (angle.sin().powf(2.0) / 360.0_f32.powf(2.0))).sqrt();
    return angle_factor;
}

unsafe extern "C" fn fighterstatusdamage_init_damage_speed_up_by_speed(fighter: &mut L2CFighterCommon, factor: L2CValue, angle: L2CValue, some_bool: L2CValue) {
    let angle = angle.get_f32();
    let angle_threshold = 29.358;
    let speed_start_horizontal: f32 = 4.65; // the start of scaling at angles below the angle_threshold
    let speed_start_vertical: f32 = 5.63; // the start of scaling at completely vertical angles
    let speed_end = 7.2; // the end of scaling
    let angle_factor = get_angle_factor(angle_threshold, angle); // the actual angle factor
    let ratio_base = get_angle_factor(angle_threshold, 90.0); // the max angle factor
    let ratio = (1.0 - angle_factor) / (1.0 - ratio_base);
    let speed_start = lerp(&speed_start_horizontal, &speed_start_vertical, &ratio);
    let speed = factor.get_f32();
    if check_damage_speed_up_fail(fighter) || speed <= speed_start {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_SPEED_UP_MAX_MAG);
        return;
    }
    let min_mul = 1.0;
    let max_mul = 1.65;
    let power = 1.0;
    let ratio = ((speed - speed_start) / (speed_end - speed_start));
    let speed_up_mul = if speed <= speed_end {
        nlerp(min_mul, max_mul, power, ratio)
    } 
    else {
        let dif = (speed_end * max_mul) - speed_end;
        let new_speed = speed + dif;
        new_speed / speed
    };
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP);
    WorkModule::set_float(fighter.module_accessor, speed_up_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_SPEED_UP_MAX_MAG);
}

unsafe extern "C" fn check_damage_speed_up_fail(fighter: &mut L2CFighterCommon) -> bool {
    let log = DamageModule::damage_log(fighter.module_accessor);
    if log == 0 {
        return true;
    }
    let log = log as *mut u8;
    return *log.add(0x8f) != 0 
        || *log.add(0x92) != 0
        || *log.add(0x93) != 0 
        || *log.add(0x98) != 0;
}

#[skyline::hook(replace = L2CFighterCommon_status_DamageFly_Main)]
unsafe fn status_DamageFly_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
        if CancelModule::is_enable_cancel(fighter.module_accessor)
        && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
        if MotionModule::frame(fighter.module_accessor) >= (MotionModule::end_frame(fighter.module_accessor) - 1.0) 
        && MotionModule::rate(fighter.module_accessor) != 0.0 {
            MotionModule::set_rate(fighter.module_accessor, 0.0);
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FALL) 
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FALL.into(), false.into());
            return 0.into();
        }
        if fighter.sub_DamageFlyCommon().get_bool() {
            return 0.into();
        }
        if !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) {
            if fighter.sub_AirChkDamageReflectWall().get_bool() || fighter.sub_AirChkDamageReflectCeil().get_bool()
            || fighter.sub_AirChkDamageReflectFloor().get_bool() {
                return 0.into();
            }
        }
        fighter.FighterStatusDamage__correctDamageVectorEffect(false.into());
    }
    else {
        if !fighter.status_DamageFinishCamera_exec().get_bool() {
            return 0.into();
        }
        fighter.status_DamageFly_Common();
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_ADJUST_VECTOR);
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_calc_damage_motion_rate)]
unsafe fn calc_damage_motion_rate(fighter: &mut L2CFighterCommon, motion_kind: L2CValue, start_frame: L2CValue, is_pierce: L2CValue) -> L2CValue {
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR]) && !is_pierce.get_bool() {
        WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_DAMAGE_MOTION_RATE);
        return L2CValue::F32(1.0);
    }
    original!()(fighter, motion_kind, start_frame, is_pierce)
}

#[skyline::hook(replace = L2CFighterCommon_sub_DamageFlyCommon)]
unsafe fn sub_DamageFlyCommon(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_AirChkPassiveWallJump().get_bool()
    || fighter.sub_AirChkPassiveWall().get_bool()
    || fighter.sub_AirChkPassiveCeil().get_bool() {
        return true.into();
    }
    if fighter.sub_transition_group_check_air_special().get_bool()
    || fighter.sub_transition_group_check_air_item_throw().get_bool()
    || fighter.sub_transition_group_check_air_lasso().get_bool()
    || fighter.sub_transition_group_check_air_escape().get_bool()
    || fighter.sub_transition_group_check_air_attack().get_bool() {
        return true.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
        if fighter.sub_transition_group_check_air_special().get_bool()
        || fighter.sub_transition_group_check_air_item_throw().get_bool()
        || fighter.sub_transition_group_check_air_lasso().get_bool()
        || fighter.sub_transition_group_check_air_escape().get_bool()
        || fighter.sub_transition_group_check_air_attack().get_bool()
        || fighter.sub_transition_group_check_air_tread_jump().get_bool()
        || fighter.sub_transition_group_check_air_wall_jump().get_bool()
        || fighter.sub_transition_group_check_air_jump_aerial().get_bool() {
            return true.into();
        }
        else {
            if !fighter.global_table[IS_STOP].get_bool()
            && fighter.sub_DamageFlyChkUniq().get_bool() {
                return true.into();
            }
            return false.into();
        }
    }
    else {
        if !fighter.global_table[IS_STOP].get_bool() {
            if fighter.sub_DamageFlyChkUniq().get_bool() {
                return true.into();
            }
        }
        return false.into();
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_DamageFlyChkUniq)]
unsafe fn sub_DamageFlyChkUniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_ENABLE_DOWN) {
            if fighter.sub_AirChkDown().get_bool() {
                return true.into();
            }
        }
        else {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
            let damage_speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            let damage_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_FRAME);
            if -1.0 <= damage_speed_y
            && WorkModule::get_param_int(fighter.module_accessor, hash40("common"), 0x1e7a52eb8a) <= damage_frame
            && fighter.sub_AirChkDown().get_bool() {
                return true.into();
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_ENABLE_DOWN);
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER)
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GENESISSET) {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
            let damage_speed_length = sv_kinetic_energy::get_speed_length(fighter.lua_state_agent);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_ATTACK_DISABLE_FRAME) <= 0
            && damage_speed_length <= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("damage_fly_attack_speed")) {
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            }
            if 1.0 < fighter.global_table[CURRENT_FRAME].get_f32()
            && WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_ESCAPE_DISABLE_FRAME) <= 0
            && damage_speed_length <= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("damage_fly_escape_speed")) {
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LASSO);
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_DAMAGE_REFLECT_ESCAPE_DISABLE_FRAME) <= 0 {
                WorkModule::enable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
            }
        }
        fighter.FighterStatusDamage__check_smoke_effect();
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_FighterStatusDamage__is_enable_damage_fly_effect)]
pub unsafe fn FighterStatusDamage__is_enable_damage_fly_effect(fighter: &mut L2CFighterCommon, arg2: L2CValue, arg3: L2CValue, arg4: L2CValue, arg5: L2CValue) -> L2CValue {
    let ret = call_original!(fighter, arg2, arg3, arg4, arg5);
    let sum_speed_x_main = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let sum_speed_x_damage = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_DAMAGE);
    let sum_speed_y_main = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let sum_speed_y_damage = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_DAMAGE);
    let speed = sv_math::vec2_length(sum_speed_x_main + sum_speed_x_damage, sum_speed_y_main + sum_speed_y_damage);
    let fly_effect_smoke_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("fly_effect_smoke_speed"));
    if ret.get_bool() {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_FRAME) < 3 {
            if speed > 0.0
            && speed < fly_effect_smoke_speed + 1.0 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_NO_SMOKE);
            }
            return false.into();
        }
        else if speed < fly_effect_smoke_speed {
            return false.into();
        }
    }
    ret
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            ftstatusuniqprocessdamage_init_common,
            status_DamageFly_Main,
            //calc_damage_motion_rate,
            sub_DamageFlyCommon,
            sub_DamageFlyChkUniq,
            FighterStatusDamage__is_enable_damage_fly_effect,
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}