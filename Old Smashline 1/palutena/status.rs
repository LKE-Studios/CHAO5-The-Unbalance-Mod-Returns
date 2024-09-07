use crate::imports::BuildImports::*;

#[status_script(agent = "palutena", status = FIGHTER_STATUS_KIND_GLIDE_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_palutena_glide_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_start"), false, -1.0);
    fighter.status_GlideStart()
}

#[status_script(agent = "palutena", status = FIGHTER_STATUS_KIND_GLIDE_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_palutena_glide_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

#[status_script( agent = "palutena", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_palutena_glide_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_GLIDE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE_LANDING);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GLIDE_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_direction"), 90.0, 0.0, true, 0.0, false, false);
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_wing"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(palutena_glide_main_loop as *const () as _))
}

unsafe extern "C" fn palutena_glide_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let frame = MotionModule::frame(fighter.module_accessor);
        let glide_landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("glide_landing_frame"));
        if glide_landing_frame <= frame {
            let sum_speed_length = KineticModule::get_sum_speed_length(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let glide_landing_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("glide_landing_speed"));
            if glide_landing_speed <= sum_speed_length {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_LANDING.into(), false.into());
                return 0.into();
            }
        }
    }
    if fighter.sub_transition_group_check_air_landing().get_bool() {
        return 0.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
        || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP) {
            fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_END.into(), false.into());
            return 0.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GLIDE_ATTACK)
        && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_ATTACK.into(), true.into());
            return 0.into();
        }
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
            return 0.into();
        }
    }
    0.into()
}

#[status_script(agent = "palutena", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_palutena_glide_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

#[status_script(agent = "palutena", status = FIGHTER_STATUS_KIND_GLIDE_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_palutena_glide_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_attack"), false, -1.0);
    STOP_SE(fighter, Hash40::new("se_palutena_glide_loop"));
    fighter.status_GlideAttack()
}

#[status_script(agent = "palutena", status = FIGHTER_STATUS_KIND_GLIDE_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_palutena_glide_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

#[status_script(agent = "palutena", status = FIGHTER_STATUS_KIND_GLIDE_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_palutena_glide_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_end"), false, -1.0);
    STOP_SE(fighter, Hash40::new("se_palutena_glide_loop"));
    fighter.status_GlideEnd()
}

#[status_script(agent = "palutena", status = FIGHTER_STATUS_KIND_GLIDE_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_palutena_glide_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

#[status_script(agent = "palutena", status = FIGHTER_STATUS_KIND_GLIDE_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_palutena_glide_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_landing"), false, -1.0);
    STOP_SE(fighter, Hash40::new("se_palutena_glide_loop"));
    fighter.status_GlideLanding()
}

#[status_script(agent = "palutena", status = FIGHTER_STATUS_KIND_GLIDE_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_palutena_glide_landing_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

pub fn install() {
    install_status_scripts!(
        status_palutena_glide_start_main,
        status_palutena_glide_start_end,
        status_palutena_glide_main,
        status_palutena_glide_end,
        status_palutena_glide_attack_main,
        status_palutena_glide_attack_end,
        status_palutena_glide_end_main,
        status_palutena_glide_end_end,
        status_palutena_glide_landing_main,
        status_palutena_glide_landing_end
    );
}