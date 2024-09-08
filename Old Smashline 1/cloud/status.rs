use crate::imports::BuildImports::*;

#[status_script( agent = "cloud", status = FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2_FALL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_cloud_SpecialHi2Fall_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi2_fall"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        cloud_SpecialHi2Fall_Sub_Status(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
    let hi2_hit_fall_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("hi2_hit_fall_speed_y"));
    let hi2_hit_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("hi2_hit_accel_y"));
    let hi2_hit_speed_max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("hi2_hit_speed_max_y"));
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, hi2_hit_fall_speed_y, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, hi2_hit_accel_y);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, hi2_hit_speed_max_y);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let hi2_hit_fall_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("hi2_hit_fall_frame"));
    WorkModule::set_int(fighter.module_accessor, hi2_hit_fall_frame, *FIGHTER_CLOUD_STATUS_SPECIAL_HI2_INT_FALL_FRAME);
    fighter.sub_shift_status_main(L2CValue::Ptr(cloud_SpecialHi2Fall_Main_loop as *const () as _))
}

unsafe fn cloud_SpecialHi2Fall_Sub_Status(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    if bool_check {
        let fall_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI2_INT_FALL_FRAME);
        if 0 < fall_frame {
            WorkModule::set_int(fighter.module_accessor, fall_frame - 1, *FIGHTER_CLOUD_STATUS_SPECIAL_HI2_INT_FALL_FRAME);
        }
    }
    0.into()
}

unsafe extern "C" fn cloud_SpecialHi2Fall_handler(fighter: &mut L2CFighterCommon) -> L2CValue {
    let object_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI2_INT_HIT_OBJECT_NUM);
    let mut hit_object_num: i32 = -1;
    if -1 < (object_num - 1) {
        do {
            let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI2_INT_HIT_OBJECT_ID);
            if object_id != *BATTLE_OBJECT_ID_INVALID {
                if BattleObjectManager::is_active_find_battle_object(BattleObjectManager(), object_id) {
                    let module_accessor = app::sv_battle_object::module_accessor(object_id);
                    let boma = module_accessor.get_ptr() as *mut BattleObjectModuleAccessor;
                    let status_kind = StatusModule::status_kind(boma);
                    if ![*FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind) {
                        let transition_terms = [
                            *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL,
                            *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE
                        ];
                        for x in 0..transition_terms.len() {
                            WorkModule::enable_transition_term(boma, transition_terms[x]);
                        }
                    }
                }
            }
            hit_object_num += 1;
        }
        while (hit_object_num < object_num);
    }
    WorkModule::set_int(fighter.module_accessor, hit_object_num, *FIGHTER_CLOUD_STATUS_SPECIAL_HI2_INT_HIT_OBJECT_NUM)
    0.into()
}

unsafe extern "C" fn cloud_SpecialHi2Fall_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    cloud_SpecialHi2Fall_handler(fighter);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if !fighter.sub_fall().get_bool() {
            if situation_kind == *SITUATION_KIND_GROUND {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL) {
                    let mut status: i32 = 0;
                    if motion_kind != hash40("special_hi2_end") {
                        status = *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2_LANDING;
                    }
                    else {
                        status = *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL;
                    }
                    fighter.change_status(status.into(), false.into());
                }
            }
            let fall_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI2_INT_FALL_FRAME);
            if fall_frame == 0 {
                if motion_kind != hash40("special_hi2_fall") {
                    if MotionModule::is_end(fighter.module_accessor) {
                        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
                        return 1.into();
                    }
                }
                else {
                    let is_stop = fighter.global_table[IS_STOP].get_bool();
                    if !is_stop {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi2_end"), 0.0, 1.0, false, 0.0, false, false);
                    }
                }
            }
            return 0.into();
        }
    }
    0.into()
}