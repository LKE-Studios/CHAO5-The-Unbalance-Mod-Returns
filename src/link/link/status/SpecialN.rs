use crate::imports::BuildImports::*;

unsafe extern "C" fn status_link_SpecialN_Exit(fighter: &mut L2CFighterCommon) -> L2CValue { 
    let bow_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOW_ARTICLE_ID);
    ArticleModule::change_status_exist(fighter.module_accessor, bow_id, *WN_LINK_BOW_STATUS_KIND_BACK);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
        if ArticleModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            let item_id = ArticleModule::get_int(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_module_accessor = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_module_accessor, true);
            if LinkModule::is_link(item_module_accessor, *ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_module_accessor);
                StatusModule::change_status_request(item_module_accessor, *ITEM_STATUS_KIND_FALL, false);
            }
        }
    }
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(0));
    0.into()
}

pub fn install() {
    Agent::new("link")
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, status_link_SpecialN_Exit)
    .install();
}