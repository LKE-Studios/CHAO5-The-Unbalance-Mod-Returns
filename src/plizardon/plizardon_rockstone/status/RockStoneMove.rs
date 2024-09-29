use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_plizardon_RockStoneMove_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT, *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK as i32);
    0.into()
}

pub unsafe extern "C" fn status_plizardon_RockStoneMove_Init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let angle = WorkModule::get_float(weapon.module_accessor, WEAPON_PLIZARDON_ROCKSTONE_INSTANCE_WORK_ID_FLOAT_ANGLE);
    let lr = PostureModule::lr(weapon.module_accessor);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_rockstone"), hash40("speed"));
    let speed_x = angle.to_radians().cos() * speed;
    let speed_y = angle.to_radians().sin() * speed;
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x * lr, speed_y);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -speed_x * lr * 0.05, -speed_y * 0.05);
    0.into()
}

pub unsafe extern "C" fn status_plizardon_RockStoneMove_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_rockstone"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED);
    if StopModule::is_stop(weapon.module_accessor){
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(plizardon_RockStoneMove_Main_loop as *const () as _))
}

pub unsafe extern "C" fn RockStone_remove(weapon: &mut L2CWeaponCommon) {
    let pos = PostureModule::pos(weapon.module_accessor);
    let eff = EffectModule::req(weapon.module_accessor, Hash40::new("sys_misfire"), pos, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0) as u32;
    EffectModule::set_rgb(weapon.module_accessor, eff, 0.5, 0.5, 0.5);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}

unsafe extern "C" fn plizardon_RockStoneMove_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let rot = WorkModule::get_float(weapon.module_accessor, WEAPON_PLIZARDON_ROCKSTONE_INSTANCE_WORK_ID_FLOAT_ROT);
    let rotation = Vector3f{x: rot, y: rot, z: rot};
    ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("needle"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life < 0 {
        RockStone_remove(weapon);
        return 0.into();
    }
    let time_active = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE) - life;
    if time_active >= 4 {
        let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        PostureModule::set_lr(weapon.module_accessor, speed_x.signum());
    }
    let reflected = AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_REFLECTOR);
    let was_reflected = WorkModule::is_flag(weapon.module_accessor, *WEAPON_SHEIK_NEEDLE_STATUS_WORK_FLAG_INFLICT);
    if reflected && !was_reflected {
        let reflected_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_rockstone"), hash40("reflected_speed"));
        KineticModule::reflect_speed(weapon.module_accessor,  &Vector3f{x: reflected_speed, y: reflected_speed, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        KineticModule::mul_accel(weapon.module_accessor,  &Vector3f{x: 0.0, y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_SHEIK_NEEDLE_STATUS_WORK_FLAG_INFLICT);
        return 0.into();
    } 
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        RockStone_remove(weapon);
    }
    0.into()
}

unsafe extern "C" fn status_plizardon_RockStoneMove_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("plizardon_rockstone")
    .status(Pre, WEAPON_PLIZARDON_ROCKSTONE_STATUS_KIND_MOVE, status_plizardon_RockStoneMove_Pre)
    .status(Init, WEAPON_PLIZARDON_ROCKSTONE_STATUS_KIND_MOVE, status_plizardon_RockStoneMove_Init)
    .status(Main, WEAPON_PLIZARDON_ROCKSTONE_STATUS_KIND_MOVE, status_plizardon_RockStoneMove_Main)
    .status(End, WEAPON_PLIZARDON_ROCKSTONE_STATUS_KIND_MOVE, status_plizardon_RockStoneMove_End)
    .install();
}