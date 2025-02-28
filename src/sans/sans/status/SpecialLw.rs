use crate::imports::BuildImports::*;

pub static attack_power : f32 = 50.0;
pub static size : f32 = 13.0;
pub static bonecage_frame : f32 = 20.0;

unsafe extern "C" fn status_sans_SpecialLw_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SANS = color >= 120 && color <= 127;
    if SANS {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
        0.into()
    }
    else {
        original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
    } 
}

unsafe extern "C" fn status_sans_SpecialLw_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SANS = color >= 120 && color <= 127;
    if SANS {
        let lr = PostureModule::lr(fighter.module_accessor);
        WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_BONECAGE_EFFECT);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if lr >= 0.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_r"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_l"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            if lr >= 0.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_r"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_l"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(sans_SpecialLw_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
    } 
}

unsafe extern "C" fn sans_SpecialLw_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() 
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if lr >= 0.0 {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_r"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_l"), -1.0, 1.0, 0.0, false, false); 
                }
            }
        }
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                if lr >= 0.0 {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_r"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_l"), -1.0, 1.0, 0.0, false, false);
                }
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_BONECAGE_HIT) {
        ATTACK(fighter, 0, 0, Hash40::new("articlebone"), attack_power, 80, 40, 0, 20, size, -6.2, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        COL_NORMAL(fighter);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_smash_flash_s"), false, false);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_down_smoke"), false, false);
        MotionModule::set_rate(fighter.module_accessor, 0.0);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let bonecage_frame_float = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SANS_INSTANCE_WORK_ID_FLOAT_BONECAGE_FRAME);
        if bonecage_frame_float == bonecage_frame {
            let effect = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_smash_flash"), Hash40::new("articlebone"), &Vector3f{x: -5.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0.6, true, 0, 0, 0, 0, 0, true, true);
            EffectModule::set_rgb(fighter.module_accessor, effect as u32, 2.0, 2.0, 2.0);
            EffectModule::set_rate(fighter.module_accessor, effect as u32, 2.0);
        }
        if bonecage_frame_float > bonecage_frame {
            WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_BONECAGE_HIT);
            AttackModule::clear_all(fighter.module_accessor);
            ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("sans_bonecage"), false);
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            MotionModule::set_frame(fighter.module_accessor, 31.0, false);
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_smash_flash_s"), false, false);
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_down_smoke"), false, false);
        }
        WorkModule::add_float(fighter.module_accessor, 1.0, *FIGHTER_SANS_INSTANCE_WORK_ID_FLOAT_BONECAGE_FRAME);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn status_sans_SpecialLw_CheckAttack(fighter: &mut L2CFighterCommon, param2: &L2CValue, param3: &L2CValue) -> L2CValue {
    let attacker_module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let table = param3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let color = WorkModule::get_int(attacker_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SANS = color >= 120 && color <= 127;
    if collision_kind == *COLLISION_KIND_HIT && SANS {
        let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
        let module_accessor = sv_battle_object::module_accessor(object_id);
        WorkModule::set_flag(attacker_module_accessor, true, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_BONECAGE_HIT);
        WorkModule::set_flag(attacker_module_accessor, false, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_BONECAGE_EFFECT);
        AttackModule::clear(attacker_module_accessor, 1, false);
        ModelModule::set_mesh_visibility(attacker_module_accessor,Hash40::new("sans_bonecage"), true);
        DamageModule::heal(attacker_module_accessor, -15.0, 0);
        if DamageModule::damage(attacker_module_accessor, 0) > 0.0 {
            SoundModule::play_se(attacker_module_accessor, Hash40::new("se_common_lifeup"), true, false, false, false, enSEType(0));
        }
        EffectModule::req_follow(attacker_module_accessor, Hash40::new("sys_recovery"), Hash40::new("top"), &VECTOR_ZERO, &VECTOR_ZERO, 1.0, true, 0, 0, 0, 0, 0, true, true);
    }
    0.into()
}

unsafe extern "C" fn status_sans_SpecialLw_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SANS = color >= 120 && color <= 127;
    if SANS {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_bonecage"), false);
        WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_BONECAGE_HIT);
        WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_BONECAGE_EFFECT);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_SANS_INSTANCE_WORK_ID_FLOAT_BONECAGE_FRAME);
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
    } 
}

pub fn install() {
    Agent::new("palutena")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_sans_SpecialLw_Pre)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_sans_SpecialLw_Main)
    .status(CheckAttack, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_sans_SpecialLw_CheckAttack)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_sans_SpecialLw_End)
    .install();
}