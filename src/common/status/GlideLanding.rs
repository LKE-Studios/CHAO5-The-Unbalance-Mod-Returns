use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_GlideLanding_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GLIDE_LANDING_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, true, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, 0, 0, 0, 0);
    0.into()
}

pub unsafe extern "C" fn status_GlideLanding_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new("glide_landing")) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN_STAND);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_landing"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN_WAIT);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(GlideLanding_Main_Sub as *const () as _))
}

unsafe extern "C" fn GlideLanding_Main_Sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        if !MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new("glide_landing")) {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN_STAND)
            || fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_DOWN_STAND.into(), false.into());
            }
            else {
                return 0.into();
            }
        }
        else {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN_WAIT)
            && MotionModule::is_end(fighter.module_accessor)
            && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_DOWN_WAIT.into(), false.into());
            }
            else {
                return 0.into();
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

pub unsafe extern "C" fn status_GlideLanding_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("fighter")
    .status(Pre, *FIGHTER_STATUS_KIND_GLIDE_LANDING, status_GlideLanding_Pre)
    .status(Main, *FIGHTER_STATUS_KIND_GLIDE_LANDING, status_GlideLanding_Main)
    .status(End, *FIGHTER_STATUS_KIND_GLIDE_LANDING, status_GlideLanding_End)
    .install();
}