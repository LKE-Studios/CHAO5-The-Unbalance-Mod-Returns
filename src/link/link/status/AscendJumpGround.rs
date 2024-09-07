use crate::imports::BuildImports::*;

unsafe extern "C" fn status_link_AscendJumpGround_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn status_link_AscendJumpGround_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_pass_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_ascend_revali"), hash40("air_pass_mul"));
    let pass_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_ascend_revali"), hash40("pass_mul"));
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("ascend_air_jump"), 0.0, 1.0, false, 0.0, false, false);
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, air_pass_mul);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("ascend_jump"), 0.0, 1.0, false, 0.0, false, false);
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, pass_mul);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(link_AscendJumpGround_Main_loop as *const () as _))
}

pub unsafe extern "C" fn find_ascendable_ground(module_accessor: *mut smash::app::BattleObjectModuleAccessor, pos_x: f32, min_pos_y: f32, pos_y: f32, height: f32) -> f32 {
    let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
    if GroundModule::ray_check_hit_pos(module_accessor, &Vector2f{x: pos_x, y: pos_y}, &Vector2f{x: 0.0, y: -100.0}, ground_hit_pos, true) {
        if ground_hit_pos.y < min_pos_y {
            return pos_y;
        }
        return find_ascendable_ground(module_accessor, pos_x, min_pos_y, ground_hit_pos.y - height, height);
    }
    else {
        return pos_y;
    }
}

unsafe extern "C" fn link_AscendJumpGround_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND 
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("ascend_jump"), -1.0, 1.0, 0.0, false, false);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR 
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("ascend_air_jump"), -1.0, 1.0, 0.0, false, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CAN_ASCEND) {
        let pos_x = PostureModule::pos_x(fighter.module_accessor);
        let pos_y = PostureModule::pos_y(fighter.module_accessor);     
        let height = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);  
        let mut min_pos_y = pos_y;
        let lr = PostureModule::lr(fighter.module_accessor);
        let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
        if GroundModule::ray_check_hit_pos(fighter.module_accessor, &Vector2f{x: pos_x, y: pos_y}, &Vector2f{x: 0.0, y: 100.0}, ground_hit_pos, true) {
            min_pos_y = ground_hit_pos.y;
        }
        let ground = find_ascendable_ground(fighter.module_accessor, pos_x, min_pos_y + height, pos_y + 100.0, height);
        if pos_y < ground && ground < pos_y + 100.0 {
            WorkModule::set_float(fighter.module_accessor, pos_y, FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_START_Y);
            WorkModule::set_float(fighter.module_accessor, ground + 5.0, FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
            fighter.change_status(FIGHTER_LINK_STATUS_KIND_ASCEND_START.into(), false.into());
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_REVALI_GLIDE.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn status_link_AscendJumpGround_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CAN_ASCEND);
    0.into()
}

pub fn install() {
    Agent::new("link")
    .status(Pre, FIGHTER_LINK_STATUS_KIND_ASCEND_JUMP_GROUND, status_link_AscendJumpGround_Pre)
    .status(Main, FIGHTER_LINK_STATUS_KIND_ASCEND_JUMP_GROUND, status_link_AscendJumpGround_Main)
    .status(End, FIGHTER_LINK_STATUS_KIND_ASCEND_JUMP_GROUND, status_link_AscendJumpGround_End)
    .install();
}