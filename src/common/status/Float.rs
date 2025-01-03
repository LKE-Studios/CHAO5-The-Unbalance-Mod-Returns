use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_Float_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ATTACK_AIR {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_FLOAT_INHERIT_AERIAL);
    }
    let fs_succeeds = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_FLOAT_INHERIT_AERIAL) {
        *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_ATTACK | *FS_SUCCEEDS_KEEP_EFFECT
    }
    else {
        0
    };
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, 0, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32, 0);
    0.into()
}

pub unsafe extern "C" fn status_Float_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let pos_z = PostureModule::pos_z(fighter.module_accessor);
    let result = &mut Vector2f{x: 0.0, y: 0.0};
    let ray_check = GroundModule::ray_check_hit_pos(fighter.module_accessor, &Vector2f{x: pos_x, y: pos_y}, &Vector2f{x: 0.0, y: -6.0}, result, true);
    if ray_check {
        let ray_check_y = result.y;
        let uniq_float_start_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_uniq_float"), hash40("uniq_float_start_y"));
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos_x, y: ray_check_y + uniq_float_start_y, z: pos_z});
    }
    0.into()
}

pub unsafe extern "C" fn status_Float_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO);
    let float_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_uniq_float"), hash40("float_frame"));
    let motion_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("param_uniq_float"), hash40("motion_rate"));
    WorkModule::set_int(fighter.module_accessor, float_frame, *FIGHTER_STATUS_FLOAT_WORK_INT_TIME);
    WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_STATUS_FLOAT_WORK_INT_ENABLE_UNIQ);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_FLOAT_IS_FLOAT);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_FLOAT_INHERIT_AERIAL) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall"), 0.0, motion_rate, false, 0.0, false, false);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
    let shield_stiff_mul_attack_air = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_stiff_mul_attack_air"));
    AttackModule::set_shield_stiff_mul(fighter.module_accessor, shield_stiff_mul_attack_air);
    float_check_aerial(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(Float_Main_loop as *const () as _))
}

unsafe fn float_check_aerial(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
    || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_FLOAT_INHERIT_AERIAL) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_INT_MTRANS) != 1 {
            float_set_aerial(fighter);
            return;
        }
    }
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_NONE, FIGHTER_LOG_ACTION_KIND_NONE);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR);
    WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_STATUS_FLOAT_WORK_INT_MTRANS);
}

unsafe fn float_set_aerial(fighter: &mut L2CFighterCommon) {
    let reflet = fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_REFLET;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_FLOAT_INHERIT_AERIAL) {
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let log = match motion_kind {
            0xc3a4e2597 => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N),
            0xc3495ada5 => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F),
            0xc33f869bc => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B),
            0xdde67d935 => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI),
            0xd40042152 => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW),
            _ => None
        };
        if let Some(log) = log {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_KEEP, log);
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_FLOAT_INHERIT_AERIAL);
    }
    else {
        if reflet {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
        }
        let mot = fighter.sub_attack_air_kind_set_log_info();
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot.get_u64()), 0.0, 1.0, false, 0.0, false, false);
    }
    if reflet {
        if let Some(target) = smashline::api::get_target_function("lua2cpp_reflet.nrs", 0x3000) {
            let check_levin: fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(target);
            check_levin(fighter);
        }
        if let Some(target) = smashline::api::get_target_function("lua2cpp_reflet.nrs", 0x28220) {
            let set_levin: fn(&mut L2CFighterCommon) = std::mem::transmute(target);
            set_levin(fighter);
        }
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_CMD_CANCEL_UNABLE_CANCEL);
    sv_module_access::cancel(fighter.lua_state_agent);
    let _ = fighter.pop_lua_stack(1);
    ControlModule::clear_command(fighter.module_accessor, false);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR);
    WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_STATUS_FLOAT_WORK_INT_MTRANS);
}

unsafe extern "C" fn Float_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[CURRENT_FRAME].get_f32() == 3.0
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_OMNI_FLOAT) {
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x, 0.0);
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR.into(), false.into());
        return 0.into();
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_INT_MTRANS) == 2 
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        fighter.global_table[CMD_CAT1].assign(&L2CValue::I32(cat1));
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        return 0.into();
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_INT_MTRANS) == 1
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
        return 0.into();
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_INT_MTRANS) == 2
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_INT_TIME) <= 0 {
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        fighter.global_table[CMD_CAT1].assign(&L2CValue::I32(cat1));
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        return 0.into();
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_INT_MTRANS) == 1
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_INT_TIME) <= 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
        return 0.into();
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_INT_ENABLE_UNIQ) == 1 {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_INT_TIME) > 0 {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_INT_TIME);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_INT_TIME) == 0 {
                WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_FLOAT_WORK_INT_ENABLE_UNIQ);
            }
        }
        if WorkModule::is_enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING) {
            fighter.sub_transition_group_check_air_landing();
        }
        if WorkModule::is_enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL) {
            fighter.sub_transition_group_check_air_special();
        }
        let mut check_aerial = false;
        if !StatusModule::is_changing(fighter.module_accessor) {
            let mtrans = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_INT_MTRANS);
            if mtrans == 1 {
                if MotionModule::is_end(fighter.module_accessor) {
                    check_aerial = true;
                }
                if CancelModule::is_enable_cancel(fighter.module_accessor) {
                    if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
                        check_aerial = true;
                    }
                }
            }
            else if mtrans == 2 {
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
                    check_aerial = true;
                }
                if MotionModule::is_end(fighter.module_accessor) {
                    let motion_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("param_uniq_float"), hash40("motion_rate"));
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall"), 0.0, motion_rate, false, 0.0, false, false);
                }
            }
        }
        if check_aerial {
            if fighter.sub_transition_group_check_air_lasso().get_bool() {
                return 1.into();
            }
            float_check_aerial(fighter);
        }
    }
    0.into()
}

pub unsafe extern "C" fn status_Float_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        WorkModule::set_int64(fighter.module_accessor, motion_kind as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    }
    AttackModule::set_shield_stiff_mul(fighter.module_accessor, 1.0);
    0.into()
}

pub fn install() {
    Agent::new("fighter")
    .status(Pre, *FIGHTER_STATUS_KIND_FLOAT, status_Float_Pre)
    .status(Init, *FIGHTER_STATUS_KIND_FLOAT, status_Float_Init)
    .status(Main, *FIGHTER_STATUS_KIND_FLOAT, status_Float_Main)
    .status(End, *FIGHTER_STATUS_KIND_FLOAT, status_Float_End)
    .install();
}