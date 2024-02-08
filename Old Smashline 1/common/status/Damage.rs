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
    let _status = StatusModule::status_kind(fighter.module_accessor);
    if 0 >= reaction_frame as i32 {
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
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let attacker_ids = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SUCCEED_ATTACKER_ENTRY_ID);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let BRAWL_META_KNIGHT = color >= 8 && color <= 10;
    if ![fighter_kind].contains(&*FIGHTER_KIND_METAKNIGHT) && !BRAWL_META_KNIGHT {
        fighter.FighterStatusDamage_init_damage_speed_up(reaction_frame.into(), degrees.into(), false.into());
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_SPEED_UP_MAX_MAG);
    }
    if [*FIGHTER_KIND_METAKNIGHT].contains(&fighter_kind) && BRAWL_META_KNIGHT {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_SPEED_UP_MAX_MAG);
    }
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
    WorkModule::set_int(fighter.module_accessor, damage_fly_attack_frame, *FIGHTER_STATUS_DAMAGE_WORK_INT_ATTACK_DISABLE_FRAME);
    let damage_fly_escape_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("damage_fly_escape_frame"));
    WorkModule::set_int(fighter.module_accessor, damage_fly_escape_frame, *FIGHTER_STATUS_DAMAGE_WORK_INT_ESCAPE_DISABLE_FRAME);
    if [
        hash40("collision_attr_paralyze"),
        hash40("collision_attr_paralyze_ghost")
    ].contains(&attr) {
        let invalid_paralyze_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("invalid_paralyze_frame"));
        WorkModule::set_float(fighter.module_accessor, invalid_paralyze_frame, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_PARALYZE_FRAME);
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_DamageFly_Main)]
unsafe fn status_DamageFly_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
        if CancelModule::is_enable_cancel(fighter.module_accessor)
        && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
        let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let BRAWL_META_KNIGHT = color >= 8 && color <= 10;
        if fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_METAKNIGHT && BRAWL_META_KNIGHT {
            if MotionModule::frame(fighter.module_accessor) >= (MotionModule::end_frame(fighter.module_accessor) - 1.0) && MotionModule::rate(fighter.module_accessor) != 0.0 {
                MotionModule::set_rate(fighter.module_accessor, 0.0);
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FALL) 
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION)
        {
            fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FALL.into(), false.into());
            return 0.into();
        }
        if fighter.sub_DamageFlyCommon().get_bool() {
            return 0.into();
        }
        if !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) {
            if fighter.sub_AirChkDamageReflectWall().get_bool()
            || fighter.sub_AirChkDamageReflectCeil().get_bool()
            || fighter.sub_AirChkDamageReflectFloor().get_bool()
            {
                return 0.into();
            }
        }
        fighter.FighterStatusDamage__correctDamageVectorEffect(L2CValue::Bool(false));
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

#[skyline::hook(replace = L2CFighterCommon_sub_DamageFlyChkUniq)]
unsafe fn subdamageflychkuniq(fighter: &mut L2CFighterCommon) -> L2CValue {
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
            && WorkModule::get_param_int(fighter.module_accessor, hash40("common"), 0x1e7a52eb8a) <=damage_frame
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
            let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
            let BRAWL_META_KNIGHT = color >= 8 && color <= 10;
            let brawl_metaknight = fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_METAKNIGHT && BRAWL_META_KNIGHT;
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_ATTACK_DISABLE_FRAME) <= 0
            && (
                brawl_metaknight
                || damage_speed_length <= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("damage_fly_attack_speed"))
            ) {
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            }
            if 1.0 < fighter.global_table[CURRENT_FRAME].get_f32()
            && WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_ESCAPE_DISABLE_FRAME) <= 0
            && (
                brawl_metaknight 
                || damage_speed_length <= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("damage_fly_escape_speed"))
            ) {
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

#[skyline::hook(replace = L2CFighterCommon_calc_damage_motion_rate)]
unsafe fn calc_damage_motion_rate(fighter: &mut L2CFighterCommon, motion_kind: L2CValue, start_frame: L2CValue, is_pierce: L2CValue) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let BRAWL_META_KNIGHT = color >= 8 && color <= 10;
    if fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_METAKNIGHT && BRAWL_META_KNIGHT {
        if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR]) && !is_pierce.get_bool() {
            WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_DAMAGE_MOTION_RATE);
            return L2CValue::F32(1.0);
        }
        original!()(fighter, motion_kind, start_frame, is_pierce)
    }
    else {
        original!()(fighter, motion_kind, start_frame, is_pierce)
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            ftstatusuniqprocessdamage_init_common,
            status_DamageFly_Main,
            subdamageflychkuniq,
            calc_damage_motion_rate,
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}