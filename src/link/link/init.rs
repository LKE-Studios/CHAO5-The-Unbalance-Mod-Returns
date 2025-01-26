use crate::imports::BuildImports::*;

pub unsafe extern "C" fn start_link_Init(fighter : &mut L2CFighterCommon) {
    let team_no = TeamModule::team_no(fighter.module_accessor) as i32;
    WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CAN_ASCEND);
    WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
    WorkModule::set_int(fighter.module_accessor, team_no, FIGHTER_LINK_INSTANCE_WORK_ID_INT_TEAM_NO);
    WorkModule::set_int(fighter.module_accessor, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
    WorkModule::set_int(fighter.module_accessor, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
    WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ASCEND_FRAME);
    WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_START_Y);
    WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
}

unsafe extern "C" fn start_link_bowarrow_Init(weapon: &mut L2CFighterBase) {
    WorkModule::set_int(weapon.module_accessor, *ITEM_KIND_NONE, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
    WorkModule::set_int(weapon.module_accessor, 0, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
    WorkModule::set_int(weapon.module_accessor, 0, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
    WorkModule::set_int(weapon.module_accessor, *BATTLE_OBJECT_ID_INVALID, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
    WorkModule::set_flag(weapon.module_accessor, false, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    WorkModule::set_flag(weapon.module_accessor, false, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
    WorkModule::set_flag(weapon.module_accessor, false, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_DEDEDE_SWALLOW);
}

unsafe extern "C" fn start_link_boomerang_Init(weapon: &mut L2CFighterBase) {
    WorkModule::set_int(weapon.module_accessor, *ITEM_KIND_NONE, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
    WorkModule::set_int(weapon.module_accessor, 0, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
    WorkModule::set_int(weapon.module_accessor, *BATTLE_OBJECT_ID_INVALID, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
    WorkModule::set_flag(weapon.module_accessor, false, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    WorkModule::set_flag(weapon.module_accessor, false, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
}

pub fn install() {
    Agent::new("link")
    .on_start(start_link_Init)
    .install();

    Agent::new("link_bowarrow")
    .on_start(start_link_bowarrow_Init)
    .install();

    Agent::new("link_boomerang")
    .on_start(start_link_boomerang_Init)
    .install();
}