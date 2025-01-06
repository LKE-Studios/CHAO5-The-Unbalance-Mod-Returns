use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_plizardon_RockStoneStart_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, *FS_SUCCEEDS_KEEP_ATTACK as i32);
    0.into()
}

pub unsafe extern "C" fn status_plizardon_RockStoneStart_Init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    //Angle
    let rock_count = WorkModule::get_param_int(weapon.module_accessor, hash40("param_rockstone"), hash40("rock_count"));
    let blast_angle = WorkModule::get_param_float(weapon.module_accessor, hash40("param_rockstone"), hash40("blast_angle"));
    let rand_angle = sv_math::rand(hash40("fighter"), 30);
    let mut angle = ((rock_count as f32 * blast_angle) - 45.0) + rand_angle as f32;
    //Prevent going too far behind
    while angle > 100.0 && angle < 260.0 {
        angle += 25.0;
    }
    WorkModule::set_float(weapon.module_accessor, angle, *WEAPON_PLIZARDON_ROCKSTONE_INSTANCE_WORK_ID_FLOAT_ANGLE);
    //Rot
    let rand_rot = sv_math::rand(hash40("fighter"), 360) as i32;
    WorkModule::set_float(weapon.module_accessor, angle, *WEAPON_PLIZARDON_ROCKSTONE_INSTANCE_WORK_ID_FLOAT_ROT);
    //Snap to throw position
    let mut owner_pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let mut article_pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let mut offset_add = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let lr = PostureModule::lr(owner_module_accessor);
    let owner_offset = ModelModule::joint_global_offset_from_top(owner_module_accessor, Hash40::new("throw"), &mut owner_pos);  
    let cap_offset = ModelModule::joint_global_offset_from_top(weapon.module_accessor, Hash40::new("have"), &mut article_pos);       
    let newPos = Vector3f{x: PostureModule::pos_x(owner_module_accessor) + owner_pos.x - article_pos.x + (offset_add.x * lr), y: PostureModule::pos_y(owner_module_accessor) + owner_pos.y - (article_pos.y)+ offset_add.y, z: PostureModule::pos_z(owner_module_accessor) + owner_pos.z - article_pos.z};
    PostureModule::set_pos(weapon.module_accessor, &newPos);
    0.into()
}

pub unsafe extern "C" fn status_plizardon_RockStoneStart_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let debris_life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_rockstone"), hash40("debris_life"));;
    WorkModule::set_int(weapon.module_accessor, debris_life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, debris_life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("stay"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(plizardon_RockStoneStart_Main_loop as *const () as _))
}

unsafe extern "C" fn plizardon_RockStoneStart_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life < 0 {
        StatusModule::change_status_force(weapon.module_accessor, *WEAPON_PLIZARDON_ROCKSTONE_STATUS_KIND_MOVE, false);
        return 0.into();
    }
    0.into()
}

pub fn install() {
    Agent::new("plizardon_rockstone")
    .status(Pre, *WEAPON_PLIZARDON_ROCKSTONE_STATUS_KIND_START, status_plizardon_RockStoneStart_Pre)
    .status(Init, *WEAPON_PLIZARDON_ROCKSTONE_STATUS_KIND_START, status_plizardon_RockStoneStart_Init)
    .status(Main, *WEAPON_PLIZARDON_ROCKSTONE_STATUS_KIND_START, status_plizardon_RockStoneStart_Main)
    .install();
}