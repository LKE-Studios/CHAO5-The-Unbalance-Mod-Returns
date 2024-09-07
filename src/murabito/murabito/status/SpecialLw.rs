use crate::imports::BuildImports::*;

unsafe extern "C" fn status_murabito_SpecialLw_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_TREE) {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_DEFOREST);
    }
    else {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_SPROUT) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_AIR);
            }
            else {
                StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WAIT);
            }
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
            ItemModule::set_change_status_event(fighter.module_accessor, false);
        }
        else {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
                StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT);
            }
            else {
                StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_DEFOREST);
            }
        } 
    }
    1.into()
}

pub fn install() {
    Agent::new("murabito")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, status_murabito_SpecialLw_Pre)
    .install();
}