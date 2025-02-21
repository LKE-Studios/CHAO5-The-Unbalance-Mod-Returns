use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_palutena_GlideStart_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_start"), false, -1.0);
    fighter.status_GlideStart()
}

pub unsafe extern "C" fn status_palutena_GlideStart_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

pub fn install() {
    Agent::new("palutena")
    .status(Main, *FIGHTER_STATUS_KIND_GLIDE_START, status_palutena_GlideStart_Main)
    .status(End, *FIGHTER_STATUS_KIND_GLIDE_START, status_palutena_GlideStart_End)
    .install();
}