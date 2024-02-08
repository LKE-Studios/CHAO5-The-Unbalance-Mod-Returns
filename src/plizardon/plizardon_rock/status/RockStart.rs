use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_plizardon_RockStart_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

pub unsafe extern "C" fn status_plizardon_RockStart_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_rock"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_STONES_MAX);
    WorkModule::set_int(weapon.module_accessor, life, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_STONES_REMAIN);
    WorkModule::set_int(weapon.module_accessor, 0, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_SPAWN_COOLDOWN);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("haved"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(plizardon_RockStart_Main_loop as *const () as _))
}

pub unsafe extern "C" fn snap_to_owner(weapon: &mut L2CWeaponCommon) {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let mut ownerPos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let mut capPos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let lr = PostureModule::lr(owner_module_accessor);
    let owner_offset = ModelModule::joint_global_offset_from_top(owner_module_accessor, Hash40{hash: hash40("throw")}, &mut ownerPos);  
    let cap_offset = ModelModule::joint_global_offset_from_top(weapon.module_accessor, Hash40{hash: hash40("have")}, &mut capPos);      
    let offset = Vector3f{x: -1.0 * lr, y: 1.0, z: 0.0};
    let newPos = Vector3f{x: PostureModule::pos_x(owner_module_accessor) + ownerPos.x - capPos.x + (offset.x), y: PostureModule::pos_y(owner_module_accessor) + ownerPos.y + offset.y, z: PostureModule::pos_z(owner_module_accessor) + ownerPos.z- capPos.z + offset.z};
    PostureModule::set_pos(weapon.module_accessor, &newPos);
    let mut vec = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let rot = Vector3f{x: vec.x, y: 0.0, z: 0.0};
    PostureModule::set_rot(weapon.module_accessor, &rot, 0);
}

unsafe extern "C" fn plizardon_RockStart_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    snap_to_owner(weapon);
    if WorkModule::is_flag(weapon.module_accessor,WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_FLAG_BREAK) {
        VisibilityModule::set_whole(weapon.module_accessor, false);
        WorkModule::dec_int(weapon.module_accessor, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_SPAWN_COOLDOWN);
        if WorkModule::get_int(weapon.module_accessor, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_SPAWN_COOLDOWN) <= 0 {
            WorkModule::set_int(weapon.module_accessor, 1, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_SPAWN_COOLDOWN);
            let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
            ArticleModule::generate_article(owner_module_accessor, FIGHTER_PLIZARDON_GENERATE_ARTICLE_ROCKSTONE, false, -1) as u32;
            WorkModule::inc_int(owner_module_accessor,FIGHTER_PLIZARDON_STATUS_ROCKSTONE_WORK_INT_INTERVAL_FRAME_COUNT);
            if WorkModule::count_down_int(weapon.module_accessor, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_STONES_REMAIN, 0) {
                notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            }
        }
    }
    0.into()
}

pub unsafe extern "C" fn status_plizardon_RockStart_Exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let mut should_delete = false;
    if !sv_battle_object::is_active(owner_id) {
        should_delete = true;
    }
    else {
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let status = StatusModule::status_kind(owner_module_accessor);
        if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_REBIRTH].contains(&status) {
            should_delete = true;
        }
    }
    if should_delete == true {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

pub unsafe extern "C" fn status_plizardon_RockStart_End(weapon: &mut L2CWeaponCommon) -> L2CValue {    
    0.into()
}

pub fn install() {
    Agent::new("plizardon_rock")
    .status(Pre, WEAPON_PLIZARDON_ROCK_STATUS_KIND_START, status_plizardon_RockStart_Pre)
    .status(Main, WEAPON_PLIZARDON_ROCK_STATUS_KIND_START, status_plizardon_RockStart_Main)
    .status(Exec, WEAPON_PLIZARDON_ROCK_STATUS_KIND_START, status_plizardon_RockStart_Exec)
    .status(End, WEAPON_PLIZARDON_ROCK_STATUS_KIND_START, status_plizardon_RockStart_End)
    .install();
}