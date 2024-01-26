use crate::imports::BuildImports::*;

pub static max_speed_x : f32 = 1.4;
pub static max_speed_y : f32 = 1.7;
pub static x_acl_air : f32 = 0.044;
pub static y_acl_air : f32 = 0.05;

#[status_script( agent = "ness", status = FIGHTER_STATUS_KIND_ATTACK_HI4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE )]
unsafe fn status_ninten_attack_hi4_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN && fighter_kind == FIGHTER_KIND_NESS {	
        fighter.sub_status_pre_SpecialNCommon();
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
        0.into()
    }
    else {
        original!(fighter)
    }
}

#[status_script( agent = "ness", status = FIGHTER_STATUS_KIND_ATTACK_HI4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_ninten_attack_hi4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN && fighter_kind == FIGHTER_KIND_NESS {
        //fighter.status_AttackHi4_common(L2CValue::Hash40s("attack_hi4"));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);	
        if motion_kind != hash40("attack_hi4") {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_hi4"), -1.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(ninten_attack_hi4_loop as *const () as _))
    }
    else {
        original!(fighter)
    }
}

unsafe extern "C" fn ninten_attack_hi4_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackHi4_Main();
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    }
    0.into()
}

#[status_script(agent = "ness", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn status_ninten_special_hi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN && fighter_kind == FIGHTER_KIND_NESS {
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, max_speed_x, max_speed_y);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, max_speed_x, max_speed_y);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, sum_speed_x, sum_speed_y);
        0.into()
    }
    else {
        0.into()
    }
}

#[status_script( agent = "ness", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_ninten_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN && fighter_kind == FIGHTER_KIND_NESS {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_UNIQ);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        GroundModule::pass_floor(fighter.module_accessor);
        StatusModule::set_keep_situation_air(fighter.module_accessor, true);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        fighter.sub_shift_status_main(L2CValue::Ptr(ninten_special_hi_main_loop as *const () as _))
    }
    else {
        original!(fighter)
    }
}

unsafe extern "C" fn ninten_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_transition_group_check_air_cliff().get_bool() {
            return 1.into();
        }
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    let lr = PostureModule::lr(fighter.module_accessor); 
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * lr;
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let speed_x = x_acl_air * stick_x;
    let speed_y = y_acl_air * stick_y;
    KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0 + speed_x, y: 0.0 + speed_y, z: 0.0});
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        fighter.change_status(FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK.into(), true.into());
        return 0.into();
    }
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
        return 0.into();
    }
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || 
    MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
    }
    0.into()
}

#[status_script( agent = "ness", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END )]
unsafe fn status_ninten_special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN && fighter_kind == FIGHTER_KIND_NESS {
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_common_spirits_floor_ice_loop"), 0);
        0.into()
    }
    else {
        0.into()
    }
}

#[status_script(agent = "ness", status = FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn status_ninten_special_hi_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN && fighter_kind == FIGHTER_KIND_NESS {
        0.into()
    }
    else {
        original!(fighter)
    }
}

#[status_script(agent = "ness", status = FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn status_ninten_special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN && fighter_kind == FIGHTER_KIND_NESS {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        StatusModule::set_keep_situation_air(fighter.module_accessor, true);
        KineticModule::clear_speed_all(fighter.module_accessor);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_end"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(ninten_special_hi_end_main_loop as *const () as _))
    }
    else {
        original!(fighter)
    }
}

unsafe extern "C" fn ninten_special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND { 
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
    }
    0.into()
}

#[status_script( agent = "ness", status = FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END )]
unsafe fn status_ninten_special_hi_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN && fighter_kind == FIGHTER_KIND_NESS {
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_common_spirits_floor_ice_loop"), 0);
        0.into()
    }
    else {
        0.into()
    }
}

#[status_script(agent = "ness", status = FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn status_ninten_special_hi_attack_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN && fighter_kind == FIGHTER_KIND_NESS {
        0.into()
    }
    else {
        original!(fighter)
    }
}

#[status_script(agent = "ness", status = FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn status_ninten_special_hi_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN && fighter_kind == FIGHTER_KIND_NESS {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        StatusModule::set_keep_situation_air(fighter.module_accessor, true);
        KineticModule::clear_speed_all(fighter.module_accessor);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_attack"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(ninten_special_hi_attack_main_loop as *const () as _))
    }
    else {
        original!(fighter)
    }
}

unsafe extern "C" fn ninten_special_hi_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND { 
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "ness", status = FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn status_ninten_special_hi_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN && fighter_kind == FIGHTER_KIND_NESS {
        0.into()
    }
    else {
        original!(fighter)
    }
}

#[status_script(agent = "ness", status = FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STOP)]
unsafe fn status_ninten_special_hi_attack_exec_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN && fighter_kind == FIGHTER_KIND_NESS {
        0.into()
    }
    else {
        original!(fighter)
    }
}

#[status_script(agent = "ness", status = FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn status_ninten_special_hi_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN && fighter_kind == FIGHTER_KIND_NESS {
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_common_spirits_floor_ice_loop"), 0);
        0.into()
    }
    else {
        original!(fighter)
    }
}

#[status_script( agent = "ness", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS )]
unsafe fn status_ninten_special_lw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN && fighter_kind == FIGHTER_KIND_NESS {
        AbsorberModule::clear_all(fighter.module_accessor);
        0.into()
    }
    else {
        0.into()
    }
}

#[status_script( agent = "ness", status = FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS )]
unsafe fn status_ninten_special_lw_hold_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN && fighter_kind == FIGHTER_KIND_NESS {
        DamageModule::set_damage_lock(fighter.module_accessor, true);
        let phy_magnet_offset_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("phy_magnet_offset_y"));
        let shield_pos = &Vector3f{x: 0.0, y: phy_magnet_offset_y - 4.0, z: 0.0};
        let time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("time"));
        let int_time = WorkModule::get_int(fighter.module_accessor,*FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_TIME);
        let ratio = 1.0;
        let vector_ratio = Vector3f{x: ratio, y: ratio, z:ratio};
        let effect =  WorkModule::get_int(fighter.module_accessor,*FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_EFFECT_HANDLE) as u32;
        EffectModule::set_scale(fighter.module_accessor, effect, &vector_ratio);
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &vector_ratio);  
        ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("throw"), shield_pos, false, false);  
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        && int_time <= time - 15 {
            WorkModule::set_int(fighter.module_accessor,0, *FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_TIME);
        }
        else if int_time <= 0 {
            fighter.change_status(FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
        }
        0.into()
    }
    else {
        0.into()
    }
}

#[status_script( agent = "ness", status = FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_CHECK_DAMAGE )]
unsafe fn status_ninten_special_lw_hold_check_damage(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN && fighter_kind == FIGHTER_KIND_NESS {
        HIT_NO(fighter, 11, *HIT_STATUS_OFF);
        SoundModule::play_se(fighter.module_accessor, Hash40::new("se_ness_special_l02"), true, false, false, false, enSEType(0));
        StopModule::set_hit_stop_frame(fighter.module_accessor, 5, true);
        DamageModule::end_damage_info_log(fighter.module_accessor);
        KineticModule::clear_speed_all(fighter.module_accessor);
        StatusModule::change_status_force(fighter.module_accessor, FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
        1.into()
    }
    else {
        0.into()
    }
}

#[status_script( agent = "ness", status = FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_ninten_special_lw_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN && fighter_kind == FIGHTER_KIND_NESS {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE);
        fighter.sub_change_motion_by_situation(Hash40::new("special_lw_hit").into(), Hash40::new("special_air_lw_hit").into(), false.into());
        fighter.sub_set_ground_correct_by_situation(true.into());
        if !StopModule::is_stop(fighter.module_accessor) {
            ninten_special_lw_hit_sub_status(fighter, false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ninten_special_lw_hit_sub_status as *const () as _));
        fighter.sub_shift_status_main(L2CValue::Ptr(ninten_special_lw_hit_main_loop as *const () as _))
    }
    else {
        original!(fighter)
    }
}

unsafe fn ninten_special_lw_hit_sub_status(fighter: &mut L2CFighterCommon, param_3: L2CValue) -> L2CValue {
    if param_3.get_bool() {
        let int_time = WorkModule::get_int(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_TIME);
        if int_time > 0 {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_TIME);
        }
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_STOP_Y_TIME);
        let int_stop_y_time = WorkModule::get_int(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_STOP_Y_TIME);
        if int_stop_y_time > 0 {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                if !StatusModule::is_changing(fighter.module_accessor) {
                    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_LW_FLAG_BUTTON_RELEASE);
                    }
                }
            }
        }
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    0.into()
}

unsafe extern "C" fn ninten_special_lw_hit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_set_ground_correct_by_situation(false.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        status_ninten_attack_hi4_pre,
        status_ninten_attack_hi4_main,
        status_ninten_special_hi_init,
        status_ninten_special_hi_main,
        status_ninten_special_hi_end,
        status_ninten_special_hi_end_init,
        status_ninten_special_hi_end_main,
        status_ninten_special_hi_end_end,
        status_ninten_special_hi_attack_init,
        status_ninten_special_hi_attack_main,
        status_ninten_special_hi_attack_exec,
        status_ninten_special_hi_attack_exec_stop,
        status_ninten_special_hi_attack_end,
        status_ninten_special_lw_exec,
        status_ninten_special_lw_hold_exec,
        status_ninten_special_lw_hold_check_damage,
        status_ninten_special_lw_hit_main
    );
}