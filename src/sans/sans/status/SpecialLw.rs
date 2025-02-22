use crate::imports::BuildImports::*;

pub static attack_power : f32 = 50.0;
pub static size : f32 = 13.0;
pub static bonecage_frame : f32 = 20.0;

unsafe extern "C" fn status_sans_SpecialLw_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SANS = color >= 120 && color <= 127;
    if SANS {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_IS_SPECIAL_LW);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(sans_SpecialLw_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
    } 
}

unsafe extern "C" fn sans_SpecialLw_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
            }
        }
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
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
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_sans_SpecialLw_Main)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_sans_SpecialLw_End)
    .install();
}