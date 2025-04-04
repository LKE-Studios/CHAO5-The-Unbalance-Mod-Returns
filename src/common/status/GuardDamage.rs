use crate::imports::BuildImports::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_GuardDamage)]
unsafe extern "C" fn status_pre_GuardDamage(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_DAMAGE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_DAMAGE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_DAMAGE_FLOAT, *FS_SUCCEEDS_KEEP_VISIBILITY);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32, 0, 0);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_initStatus)]
unsafe extern "C" fn sub_ftStatusUniqProcessGuardDamage_initstatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardDamage_initStatus_Inner();
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        let prev_shield_scale_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_PREV_SHIELD_SCALE_FRAME);
        let shield_hp_const = if 0 < prev_shield_scale_frame {
            *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_PREV_SHIELD
        }
        else {
            *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD
        };
        let shield_hp = WorkModule::get_float(fighter.module_accessor, shield_hp_const);
        let scale = fighter.FighterStatusGuard__calc_shield_scale(shield_hp.into()).get_f32();
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: scale, y: scale, z: scale});
    }
    else {
        if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_GUARD_ON {
            let prev_shield_scale_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_PREV_SHIELD_SCALE_FRAME);
            let shield_hp_const = if 0 < prev_shield_scale_frame {
                *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_PREV_SHIELD
            }
            else {
                *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD
            };
            let shield_hp = WorkModule::get_float(fighter.module_accessor, shield_hp_const);
            let scale = fighter.FighterStatusGuard__calc_shield_scale(shield_hp.into()).get_f32();
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: scale, y: scale, z: scale});
        }
        else {
            fighter.FighterStatusGuard__set_just_shield_scale();
        }
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_initStatus_Inner)]
unsafe fn sub_ftStatusUniqProcessGuardDamage_initStatus_Inner(fighter: &mut L2CFighterCommon) {
    let shield_power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_POWER);
    let shield_setoff_mul_status = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_SETOFF_MUL);
    let mut shield_stiff_frame = shield_power * shield_setoff_mul_status;
    shield_stiff_frame *= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_setoff_mul"));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        shield_stiff_frame *= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("just_shield_setoff_mul"));
    }
    shield_stiff_frame += WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_setoff_add"));
    let shield_stiff_frame_max = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_stiff_frame_max"));
    if shield_stiff_frame_max < shield_stiff_frame {
        shield_stiff_frame = shield_stiff_frame_max;
    }
    let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_OBJECT_ID);
    if object_id != *BATTLE_OBJECT_ID_INVALID {
        capture!(fighter, MA_MSC_CMD_CAPTURE_SET_IGNORE_OBJECT_ID, object_id);
        let mut invalid_capture_frame = shield_stiff_frame;
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
            invalid_capture_frame += WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_cancel_frame")) as f32;
        }
        invalid_capture_frame *= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_ignore_capture_rate"));
        WorkModule::set_int(fighter.module_accessor, invalid_capture_frame as i32, *FIGHTER_INSTANCE_WORK_ID_INT_GUARD_INVALID_CAPTURE_FRAME);
    }
    WorkModule::set_int(fighter.module_accessor, shield_stiff_frame as i32, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_STIFF_FRAME);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        let shield_setoff_catch_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("shield_setoff_catch_frame"));
        if shield_setoff_catch_frame > 0 {
            WorkModule::set_int(fighter.module_accessor, shield_setoff_catch_frame, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME);
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        fighter.clear_lua_stack();
        let mot = hash40("guard_damage");
        lua_args!(fighter, mot);
        let motion_rate = sv_fighter_util::get_guard_damage_motion_rate(fighter.lua_state_agent, Hash40::new_raw(mot));
        let weight = MotionModule::weight(fighter.module_accessor);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, motion_rate, false, 0.0, false, false);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("guard"), 0.0, 1.0, false, 1.0);
            MotionModule::set_rate_2nd(fighter.module_accessor, 0.0);
            MotionModule::set_weight(fighter.module_accessor, weight, true);
            let prev_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_PREV_X);
            let prev_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_PREV_Y);
            fighter.FighterStatusGuard__set_guard_blend_motion_angle(prev_x.into(), prev_y.into());
        }
    }
    else {
        if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_GUARD_ON {
            let mut cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("guard_on"), true);
            if cancel_frame == 0.0 {
                cancel_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("guard_on"));
            }
            let motion_rate = cancel_frame / shield_stiff_frame;
            let weight = MotionModule::weight(fighter.module_accessor);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("guard_on"), -1.0, motion_rate, 0.0, false, false);
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
                MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("guard"), 0.0, 1.0, false, 1.0);
                MotionModule::set_rate_2nd(fighter.module_accessor, 0.0);
                MotionModule::set_weight(fighter.module_accessor, weight, true);
                let prev_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_PREV_X);
                let prev_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_PREV_Y);
                fighter.FighterStatusGuard__set_guard_blend_motion_angle(prev_x.into(), prev_y.into());
            }
            
        }
        else { 
            let mut cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("just_shield_off"), true);
            if cancel_frame == 0.0 {
                cancel_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("just_shield_off"));
            }
            let motion_rate = cancel_frame / shield_stiff_frame;
            let just_shield_motion = WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("just_shield_motion"));
            if just_shield_motion == 0 {
                let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("just_shield_off"));
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("just_shield_off"), end_frame, motion_rate, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("just_shield_off"), 0.0, motion_rate, false, 0.0, false, false);
            }
            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(0x1a29f56bfb), -1);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_SHEILD_COUNT);
        if fighter.FighterStatusGuard__is_continue_just_shield_count().get_bool() == false {
            CancelModule::enable_cancel(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_DISABLE_HIT_STOP_DELAY_STICK);
        }
        else {
            ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
            ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(*SHIELD_TYPE_JUST_SHIELD), *FIGHTER_SHIELD_KIND_GUARD, 0);
            let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut smash::app::BattleObjectModuleAccessor;
            if FighterUtil::is_valid_just_shield_reflector(module_accessor) {
                ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
            }
        }
    }
    else {
        ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
        ControlModule::clear_command(fighter.module_accessor, false);
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        let shield_setoff_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_setoff_speed_mul"));
        let shield_setoff_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_setoff_speed_max"));
        let shield_lr = -WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_LR);
        let setoff_speed = (shield_setoff_speed_mul * shield_stiff_frame * shield_lr).clamp(-shield_setoff_speed_max, shield_setoff_speed_max);
        let setoff_mul_indirect = if sv_battle_object::category(object_id as u32) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
            0.5
        }
        else {
            1.0
        };
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE, ENERGY_STOP_RESET_TYPE_GUARD_DAMAGE, setoff_speed * setoff_mul_indirect, 0.0, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    }
    let mut hit_stop_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_HIT_STOP_FRAME);
    hit_stop_frame *= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x2434ca61df);
    WorkModule::set_int(fighter.module_accessor, hit_stop_frame as i32, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_PREV_SHIELD_SCALE_FRAME);
    let hit_stop_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x20d241cd64);
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_mul);
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardDamage_common)]
unsafe extern "C" fn status_GuardDamage_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    ControlModule::reset_flick_x(fighter.module_accessor);
    ControlModule::reset_flick_sub_x(fighter.module_accessor);
    fighter.global_table[STICK_X].assign(&0xFE.into());
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_ESCAPE_FRAME);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_HIT_XLU);
        let just_shield_precede_extension = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("just_shield_precede_extension"));
        ControlModule::set_command_life_extend(fighter.module_accessor, just_shield_precede_extension as u8);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_JUST_SHIELD);
        let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut smash::app::BattleObjectModuleAccessor;
        FighterUtil::flash_eye_info(module_accessor);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL) {
            ModelModule::enable_gold_eye(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE);
        }
        EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_just_shield"), Hash40::new("throw"), &VECTOR_ZERO, &VECTOR_ZERO, 1.0, &VECTOR_ZERO, &VECTOR_ZERO, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, *EFFECT_FLIP_NONE, 1);
        let shield_lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_LR);
        ColorBlendModule::set_last_attack_direction(fighter.module_accessor, &Vector3f{x: -shield_lr, y: 0.0, z: 0.0});
        EffectModule::req_common(fighter.module_accessor, Hash40::new("just_shield"), 0.0);
        let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
        if prev_status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF || prev_status_kind == *FIGHTER_STATUS_KIND_GUARD_ON {
            EffectModule::req_screen(fighter.module_accessor, Hash40::new("just_shield_screen"), false, false, false);
        }
        let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
        let just_shield_se = FighterUtil::get_just_shield_se(fighter_kind);
        if prev_status_kind == *FIGHTER_STATUS_KIND_GUARD_ON {
            SoundModule::play_se(fighter.module_accessor, just_shield_se, true, false, false, false, enSEType(0)) as i32;
        }
        else {
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_shieldparry"), true, false, false, false, enSEType(0)) as i32;
            if [*FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN].contains(&fighter_kind) {
                SoundModule::play_se(fighter.module_accessor, just_shield_se, true, false, false, false, enSEType(0)) as i32;
            }
        }
    }
    else {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_IS_DONE_GUARD_DAMAGE_NUM) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_GUARD_DAMAGE_NUM);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_IS_DONE_GUARD_DAMAGE_NUM);
        }
        let prev_shield = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_PREV_SHIELD);
        let prev_shield_scale = fighter.FighterStatusGuard__calc_shield_scale(prev_shield.into()).get_f32();
        let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        let shield_scale = fighter.FighterStatusGuard__calc_shield_scale(shield_hp.into()).get_f32();
        let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
        if fighter_kind != *FIGHTER_KIND_YOSHI {
            let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut smash::app::BattleObjectModuleAccessor;
            let team_color = FighterUtil::get_team_color(module_accessor);
            let effect_team_color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("shield_effect_color"));
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_shield_damage3"), Hash40::new("throw"), &VECTOR_ZERO, &VECTOR_ZERO, 0.1, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, 0, -1, *EFFECT_FLIP_NONE, 1, false, false);
            EffectModule::set_rgb_partial_last(fighter.module_accessor, effect_team_color.value[0], effect_team_color.value[1], effect_team_color.value[2]);
            let effect_glass_handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("shield_damage_glass"), Hash40::new("throw"), &VECTOR_ZERO, &VECTOR_ZERO, 0.1, false, 0, 0, 0, *EFFECT_FLIP_NONE, 0, false, false);
            EffectModule::set_rgb(module_accessor, effect_glass_handle as u32, 0.4 + effect_team_color.value[0], 0.4 + effect_team_color.value[1], 0.4 + effect_team_color.value[2]);
            let effect4_handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("shield_damage4"), Hash40::new("throw"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &VECTOR_ZERO, 0.16, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, 0, -1, *EFFECT_FLIP_NONE, 0, false, false);
            EffectModule::set_rgb(module_accessor, effect4_handle as u32, 0.25 + effect_team_color.value[0], 0.25 + effect_team_color.value[1], 0.25 + effect_team_color.value[2]);
            let effect2_handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_shield_damage2"), Hash40::new("throw"), &VECTOR_ZERO, &VECTOR_ZERO, 0.1, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, 0, 0, *EFFECT_FLIP_NONE, 0, false, false) as u32;
            EffectModule::set_rgb_partial_last(fighter.module_accessor, effect_team_color.value[0], effect_team_color.value[1], effect_team_color.value[2]);
            WorkModule::set_int(fighter.module_accessor, effect2_handle as i32, *FIGHTER_STATUS_GUARD_ON_WORK_INT_SHIELD_DAMAGE2_EFFECT_HANDLE);
            let effect_handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_shield_damage"), Hash40::new("throw"), &VECTOR_ZERO, &VECTOR_ZERO, 1.0, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, 0, -1, *EFFECT_FLIP_NONE, 1, false, false) as u32;
            EffectModule::set_rgb_partial_last(fighter.module_accessor, effect_team_color.value[0], effect_team_color.value[1], effect_team_color.value[2]);
            WorkModule::set_int(fighter.module_accessor, effect_handle as i32, *FIGHTER_STATUS_GUARD_ON_WORK_INT_SHIELD_DAMAGE_EFFECT_HANDLE);
            if effect_handle != 0 {
                let effect_scale = 0.1 * (shield_scale / prev_shield_scale);
                EffectModule::set_scale(fighter.module_accessor, effect_handle, &(Vector3f {x: effect_scale, y: effect_scale, z: effect_scale}));
            }
        }
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_GuardDamageUniq(false.into());    
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_GuardDamageUniq as *const () as _));
}

#[skyline::hook(replace = L2CFighterCommon_sub_GuardDamageUniq)]
unsafe fn sub_GuardDamageUniq(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        fighter.FighterStatusGuard__landing_effect_control();
        return 0.into();
    }
    let mut min_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
    if 0 < min_frame {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        min_frame -= 1;
    }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) && min_frame <= 0 {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
    }
    let just_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) && 0 < just_frame {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
        if just_frame - 1 == 0 {
            ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NONE), 0);
            let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
            let type_of_guard = FighterUtil::get_shield_type_of_guard(fighter_kind) as i32;
            ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(type_of_guard), *FIGHTER_SHIELD_KIND_GUARD, 0);
            ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
        }
    }
    if !WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_STIFF_FRAME, 0) {
        return 0.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL)  {
                ModelModule::disable_gold_eye(fighter.module_accessor);
            }
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_HIT_XLU) {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_HIT_XLU);
        }
        EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_windwave"), Hash40::new("top"), &VECTOR_ZERO, &VECTOR_ZERO, 1.0, &VECTOR_ZERO, &VECTOR_ZERO, false, 0, 0, 0);
    }
    else {
        let guard_damage_just_shield_disable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_damage_just_shield_disable_frame"));
        WorkModule::set_int(fighter.module_accessor, guard_damage_just_shield_disable_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_JUST_SHIELD_FRAME);
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardDamage_Main)]
unsafe extern "C" fn status_GuardDamage_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_guard_damage_main_common_air().get_bool() {
        return 0.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        let is_hit = StopModule::is_hit(fighter.module_accessor);
        if is_hit {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_TRANSITION_STATUS_STOP);
        }
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool()
            && is_hit {
                StopModule::cancel_hit_stop(fighter.module_accessor);
                return 0.into();
            }
        }
        if is_hit {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_TRANSITION_STATUS_STOP);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD)
    && fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_GUARD_ON {
        fighter.change_status(FIGHTER_STATUS_KIND_GUARD.into(), false.into());
    }
    fighter.status_guard_damage_main_common();
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_execStatus_common)]
unsafe extern "C" fn sub_ftStatusUniqProcessGuardDamage_execStatus_common(fighter: &mut L2CFighterCommon) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        let scale = fighter.FighterStatusGuard__calc_shield_scale(shield_hp.into()).get_f32();
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: scale, y: scale, z: scale});
    }
    else {
        fighter.FighterStatusGuard__set_just_shield_scale();
    }
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_exitStatus_common)]
unsafe extern "C" fn sub_ftStatusUniqProcessGuardDamage_exitStatus_common(fighter: &mut L2CFighterCommon) {
    ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NONE), 0);
    let type_of_guard = FighterUtil::get_shield_type_of_guard(fighter.global_table[FIGHTER_KIND].get_i32()) as i32;
    ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(type_of_guard), *FIGHTER_SHIELD_KIND_GUARD, 0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_HIT_XLU) {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            HitModule::set_xlu_frame_global(fighter.module_accessor, 8, 0);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_HIT_XLU);
        }
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("just_shield"));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL) {
                ModelModule::disable_gold_eye(fighter.module_accessor);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE);
            }
        }
        ControlModule::set_command_life_extend(fighter.module_accessor, 0);
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_GuardDamage,
            sub_ftStatusUniqProcessGuardDamage_initstatus,
            sub_ftStatusUniqProcessGuardDamage_initStatus_Inner,
            status_GuardDamage_common,
            sub_GuardDamageUniq,
            status_GuardDamage_Main,
            sub_ftStatusUniqProcessGuardDamage_execStatus_common,
            sub_ftStatusUniqProcessGuardDamage_exitStatus_common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}