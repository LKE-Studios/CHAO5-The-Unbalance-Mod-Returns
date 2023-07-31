use crate::imports::BuildImports::*;

pub static mut PFUSHIGISOU_SOLAR_BEAM_TIMER : [i32; 8] = [0; 8];
pub static SOLAR_BEAM_MAX_CHARGE : i32 = 240;
pub static mut SFX_COUNTER : [i32; 8] = [0; 8];

//FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N2_CHARGE
#[status_script(agent = "pfushigisou", status = 0x1D3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn status_pre_pfushigisou_special_n2(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

#[status_script(agent = "pfushigisou", status = 0x1D3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_main_pfushigisou_special_n2(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air_special_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("guard_special_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(pfushigisou_special_n2_loop as *const () as _))
}

unsafe extern "C" fn pfushigisou_special_n2_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("guard_special_start"), -1.0, 1.0, 0.0, false, false);
        }
        if MotionModule::is_end(fighter.module_accessor) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air_special_charge"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if MotionModule::is_end(fighter.module_accessor) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("guard_special_charge"), 0.0, 1.0, false, 0.0, false, false);
        }
    } 
    PFUSHIGISOU_SOLAR_BEAM_TIMER[ENTRY_ID] += 1;
    if PFUSHIGISOU_SOLAR_BEAM_TIMER[ENTRY_ID] >= SOLAR_BEAM_MAX_CHARGE {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N2_SHOOT.into(), false.into());
        }
    }
    if PFUSHIGISOU_SOLAR_BEAM_TIMER[ENTRY_ID] == SOLAR_BEAM_MAX_CHARGE {
        SFX_COUNTER[ENTRY_ID] += 1;
        if SFX_COUNTER[ENTRY_ID] < 2 {
            PLAY_SE(fighter, Hash40::new("se_pfushigisou_appeal_l03"));
            PLAY_SE(fighter, Hash40::new("se_pfushigisou_special_n01"));
        }
        if GFX_COUNTER[ENTRY_ID] >= 4 {
            GFX_COUNTER[ENTRY_ID] += 1;
            EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 6.5, 4.0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter, /*R*/ 3.0, /*G*/ 1.5, /*B*/ 0.0);
            GFX_COUNTER[ENTRY_ID] = 0;
        }
    }
    if SFX_COUNTER[ENTRY_ID] >= 100 {
        SFX_COUNTER[ENTRY_ID] = 2;
    }
    0.into()
}

#[status_script(agent = "pfushigisou", status = 0x1D3, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn status_exec_fushigisou_special_n2(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("escape_air_special_charge") {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                if frame >= 120.0 {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air_special_charge"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            if ControlModule::check_button_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                fighter.change_status(FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N2_END.into(), false.into());
            }
        }
    }
    else {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("guard_special_charge") {
            if lr * stick_x >= 0.5 {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), false.into());
            }
            if lr * stick_x <= -0.5 {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), false.into());
            }
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                if frame >= 120.0 {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("guard_special_charge"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            if ControlModule::check_button_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || 
            ControlModule::check_button_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                fighter.change_status(FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N2_END.into(), false.into());
            }
        }    
    }
    0.into()
}  

#[status_script(agent = "pfushigisou", status = 0x1D3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_end_pfushigisou_special_n2(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    STOP_SE(fighter, Hash40::new("se_pfushigisou_recovery"));
    SFX_COUNTER[ENTRY_ID] = 0;
    0.into()
}

//FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N2_SHOOT
#[status_script(agent = "pfushigisou", status = 0x1DA, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn status_pre_pfushigisou_special_n2_shoot(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

#[status_script(agent = "pfushigisou", status = 0x1DA, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_main_pfushigisou_special_n2_shoot(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air_special_shoot"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("guard_special_shoot"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(pfushigisou_special_n2_shoot_loop as *const () as _))
}

unsafe extern "C" fn pfushigisou_special_n2_shoot_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_KEEP));
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("guard_special_shoot"), -1.0, 1.0, 0.0, false, false);
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            PFUSHIGISOU_SOLAR_BEAM_TIMER[ENTRY_ID] = 0;
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            PFUSHIGISOU_SOLAR_BEAM_TIMER[ENTRY_ID] = 0;
        }
    }    
    0.into()
}

#[status_script(agent = "pfushigisou", status = 0x1DA, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_end_pfushigisou_special_n2_shoot(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    PFUSHIGISOU_SOLAR_BEAM_TIMER[ENTRY_ID] = 0;
    EFFECT_OFF_KIND(fighter, Hash40::new("finptrainer_solar_beam"), false, false);
    0.into()
}

//FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N2_END
#[status_script(agent = "pfushigisou", status = 0x1DB, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn status_pre_pfushigisou_special_n2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

#[status_script(agent = "pfushigisou", status = 0x1DB, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_main_pfushigisou_special_n2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air_special_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("guard_special_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(pfushigisou_special_n2_end_loop as *const () as _))
}

unsafe extern "C" fn pfushigisou_special_n2_end_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    } 
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "pfushigisou", status = 0x1DB, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_end_pfushigisou_special_n2_end(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    install_status_scripts!(
        status_pre_pfushigisou_special_n2,
        status_main_pfushigisou_special_n2,
        status_exec_fushigisou_special_n2,
        status_end_pfushigisou_special_n2,
        status_pre_pfushigisou_special_n2_shoot,
        status_main_pfushigisou_special_n2_shoot,
        status_end_pfushigisou_special_n2_shoot,
        status_pre_pfushigisou_special_n2_end,
        status_main_pfushigisou_special_n2_end,
        status_end_pfushigisou_special_n2_end
    );
}