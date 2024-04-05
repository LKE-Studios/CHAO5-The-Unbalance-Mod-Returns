use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_pikachu_SpecialHiEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_SPECIALUPDUMMY, false, -1);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("special_air_hi_end"), false, -1.0);
    ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_SPECIALUPDUMMY, 0.0);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_end"), 0.0, 1.0, false, 0.0, false, false);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(pikachu_SpecialHiEnd_Main_loop as *const () as _))
}

unsafe extern "C" fn pikachu_SpecialHiEnd_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if MotionModule::is_end(fighter.module_accessor) {
        if motion_kind == hash40("special_hi_end") {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
        if frame > 16.0 && frame < 26.0 {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                fighter.change_status(FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP.into(), true.into());
                return 1.into();
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("pikachu")
    .status(Main, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END, status_pikachu_SpecialHiEnd_Main)
    .install();
}

