use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_palutena_GlideAttack_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_attack"), false, -1.0);
    STOP_SE(fighter, Hash40::new("se_palutena_glide_loop"));
    fighter.status_GlideAttack()
}

pub unsafe extern "C" fn status_palutena_GlideAttack_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

pub fn install() {
    Agent::new("palutena")
    .status(Main, *FIGHTER_STATUS_KIND_GLIDE_ATTACK, status_palutena_GlideAttack_Main)
    .status(End, *FIGHTER_STATUS_KIND_GLIDE_ATTACK, status_palutena_GlideAttack_End)
    .install();
}