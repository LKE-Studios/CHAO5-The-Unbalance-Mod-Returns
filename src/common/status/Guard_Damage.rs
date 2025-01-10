use crate::imports::BuildImports::*;

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
        SoundModule::play_se(fighter.module_accessor, just_shield_se, true, false, false, false, enSEType(0)) as i32;
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
        let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut smash::app::BattleObjectModuleAccessor;
        let team_color = FighterUtil::get_team_color(module_accessor);
        let effect_team_color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("shield_effect_color"));
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_shield_damage3"), Hash40::new("throw"), &VECTOR_ZERO, &VECTOR_ZERO, 0.1, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, 0, -1, *EFFECT_FLIP_NONE, 1, false, false);
        EffectModule::set_rgb_partial_last(fighter.module_accessor, effect_team_color.value[0], effect_team_color.value[1], effect_team_color.value[2]);
        let effect2_handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_shield_damage2"), Hash40::new("throw"), &VECTOR_ZERO, &VECTOR_ZERO, 0.1, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, 0, -1, *EFFECT_FLIP_NONE, 1, false, false) as u32;
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

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_GuardDamage_common,
            sub_GuardDamageUniq,
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}