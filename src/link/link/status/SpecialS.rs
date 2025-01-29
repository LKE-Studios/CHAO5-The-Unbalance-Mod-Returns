use crate::imports::BuildImports::*;

unsafe extern "C" fn status_link_SpecialS_Exit(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOOMERANG) {
        let item_id = ArticleModule::get_int(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_module_accessor = smash::app::sv_battle_object::module_accessor(item_id);
        LinkModule::remove_model_constraint(item_module_accessor, true);
        if LinkModule::is_link(item_module_accessor, *ITEM_LINK_NO_HAVE) {
            LinkModule::unlink_all(item_module_accessor);
            StatusModule::change_status_request(item_module_accessor, *ITEM_STATUS_KIND_FALL, false);
        }
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG, ArticleOperationTarget(0));
    }
    0.into()
}

pub fn install() {
    Agent::new("link")
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, status_link_SpecialS_Exit)
    .install();
}