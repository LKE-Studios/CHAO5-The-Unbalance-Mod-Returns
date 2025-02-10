use crate::imports::BuildImports::*;

pub static fly_landing_speed_x : f32 = 1.7;
pub static speed_x_stop_frame : f32 = 30.0;

pub unsafe extern "C" fn status_funky_SpecialSLanding_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
        0.into()
    }
    else {
        0.into()
    }
}

pub unsafe extern "C" fn status_funky_SpecialSLanding_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: fly_landing_speed_x, y: 0.0, z: 0.0});
        if !MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new("special_s_landing")) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN_STAND);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_landing"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN_WAIT);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(funky_SpecialSLanding_Main_loop as *const () as _))
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn funky_SpecialSLanding_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_air_landing().get_bool() {
        return 0.into();
    }
    let frame = MotionModule::frame(fighter.module_accessor);
    if frame > speed_x_stop_frame {
        KineticModule::clear_speed_all(fighter.module_accessor);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        let jump_count_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        let status = if jump_count >= jump_count_max {
            FIGHTER_STATUS_KIND_FALL_AERIAL
        }
        else {
            FIGHTER_STATUS_KIND_FALL_AERIAL
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

pub unsafe extern "C" fn status_funky_SpecialSLanding_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        0.into()
    }
    else {
        0.into()
    }
}

pub fn install() {
    Agent::new("donkey")
    .status(Pre, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_S_LANDING, status_funky_SpecialSLanding_Pre)
    .status(Main, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_S_LANDING, status_funky_SpecialSLanding_Main)
    .status(End, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_S_LANDING, status_funky_SpecialSLanding_End)
    .install();
}