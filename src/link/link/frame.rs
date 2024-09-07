use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_link_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    activate_ascend_revali(fighter);
    //Boomerang
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG) {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM) {
            WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
            WorkModule::set_int(fighter.module_accessor, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
        }
        else {
            let boomerang_fuse_item_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID) as u32;
            if sv_battle_object::is_active(boomerang_fuse_item_id) {
                let item_module_accessor = smash::app::sv_battle_object::module_accessor(boomerang_fuse_item_id);
                if StatusModule::status_kind(item_module_accessor) == *ITEM_STATUS_KIND_HAVE {
                    if smash::app::utility::get_kind(&mut *item_module_accessor) != *ITEM_KIND_LINKBOMB {
                        let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
                        smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, boomerang_fuse_item_id);
                    }
                    else {
                        StatusModule::change_status_request(item_module_accessor, *ITEM_STATUS_KIND_THROW, false);
                        WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
                        WorkModule::set_int(fighter.module_accessor, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
                    }
                }
            }
        }
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM);
        }
    }
    //Bomb
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED) {
        let item_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        if !sv_battle_object::is_active(item_id) {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("link_ken"), true);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED);
        }
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2,].contains(&status_kind) {
        if !fighter.is_in_hitlag() && !StatusModule::is_changing(fighter.module_accessor) && situation_kind == *SITUATION_KIND_AIR {
            fighter.sub_air_check_dive();
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
                if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_MOTION_AIR || 
                KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE {
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                    let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    sv_kinetic_energy::enable(fighter.lua_state_agent);
                    KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                }
            }
        }
    }
}

pub unsafe extern "C" fn activate_ascend_revali(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    let mut on_frame = WorkModule::get_int(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_INT_JUMP_BUTTON_ON_FRAME);
    if ![FIGHTER_LINK_STATUS_KIND_REVALI_GLIDE, FIGHTER_LINK_STATUS_KIND_REVALI_GLIDE_TURN, FIGHTER_LINK_STATUS_KIND_REVALI_GLIDE_DROP, 
    FIGHTER_LINK_STATUS_KIND_REVALI_GLIDE_LANDING, FIGHTER_LINK_STATUS_KIND_ASCEND_START, FIGHTER_LINK_STATUS_KIND_ASCEND, FIGHTER_LINK_STATUS_KIND_ASCEND_END, 
    FIGHTER_LINK_STATUS_KIND_ASCEND_JUMP_GROUND, FIGHTER_LINK_STATUS_KIND_ASCEND_JUMP_GROUND_END, *FIGHTER_STATUS_KIND_SLEEP, *FIGHTER_STATUS_KIND_ATTACK_AIR, 
    *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_MISS_FOOT, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_FLY,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_JUMP, 
    *FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) && situation_kind == *SITUATION_KIND_AIR {
        let jump_button_on_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_ascend_revali"), hash40("jump_button_on_frame"));
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            WorkModule::inc_int(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_INT_JUMP_BUTTON_ON_FRAME);
        }
        if on_frame >= jump_button_on_frame {
            fighter.change_status(FIGHTER_LINK_STATUS_KIND_ASCEND_JUMP_GROUND.into(), true.into());
        }
    }
    else {
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_LINK_INSTANCE_WORK_ID_INT_JUMP_BUTTON_ON_FRAME)
    }
}

pub unsafe extern "C" fn frame_link_bowarrow_Main(weapon : &mut L2CFighterBase) {
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR)
    && StatusModule::status_kind(weapon.module_accessor) == *WN_LINK_BOWARROW_STATUS_KIND_FLY
    && WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        WorkModule::set_flag(weapon.module_accessor, true, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
        let item_id = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_module_accessor = smash::app::sv_battle_object::module_accessor(item_id);
        let team_no = TeamModule::team_no(weapon.module_accessor) as i32;
        let team_owner_id = TeamModule::team_owner_id(weapon.module_accessor) as u32;
        TeamModule::set_team(item_module_accessor, team_no, true);
        TeamModule::set_team_owner_id(item_module_accessor, team_owner_id);
    }
}

pub unsafe extern "C" fn frame_link_boomerang_Main(weapon : &mut L2CFighterBase) {
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let status_kind = StatusModule::status_kind(weapon.module_accessor);
    let item_id = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
    let item_module_accessor = smash::app::sv_battle_object::module_accessor(item_id);
    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR)
    && status_kind == *WN_LINK_BOOMERANG_STATUS_KIND_FLY
    && WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        WorkModule::set_flag(weapon.module_accessor, true, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
        let team_no = TeamModule::team_no(weapon.module_accessor) as i32;
        let team_owner_id = TeamModule::team_owner_id(weapon.module_accessor) as u32;
        TeamModule::set_team(item_module_accessor, team_no, true);
        TeamModule::set_team_owner_id(item_module_accessor, team_owner_id);
    }
    if (AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_SHIELD))
    && [*WN_LINK_BOOMERANG_STATUS_KIND_TURN, *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED].contains(&status_kind)
    && WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT) {
            let team_no = WorkModule::get_int(owner_module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_INT_TEAM_NO);
            TeamModule::set_team(weapon.module_accessor, team_no, true);
            TeamModule::set_team_owner_id(weapon.module_accessor, (*(owner_module_accessor)).battle_object_id);
            WorkModule::set_flag(weapon.module_accessor, false, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
        }
        LinkModule::remove_model_constraint(item_module_accessor, true);
        if LinkModule::is_link(item_module_accessor, *ITEM_LINK_NO_HAVE) {
            LinkModule::unlink_all(item_module_accessor);
            let status = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
            StatusModule::change_status_request(item_module_accessor, status, false);
        }
    }
}

pub fn install() {
    Agent::new("link")
    .on_line(Main, frame_link_Main)
    .install();

    Agent::new("link_bowarrow")
    .on_line(Main, frame_link_bowarrow_Main)
    .install();

    Agent::new("link_boomerang")
    .on_line(Main, frame_link_boomerang_Main)
    .install();
}