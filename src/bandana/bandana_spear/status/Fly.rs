use crate::imports::BuildImports::*;

pub static fly_speed_x : f32 = 1.7;
pub static fly_speed_y : f32 = 0.7;
pub static life : i32 = 360;
pub static air_accel_y : f32 = 0.075;
pub static air_accel_y_mul : f32 = 0.5;

unsafe extern "C" fn status_bandana_spear_Fly_Pre(weapon: &mut L2CFighterCommon) -> L2CValue {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let lr = PostureModule::lr(owner_module_accessor);
    let owner_pos_x = PostureModule::pos_x(owner_module_accessor);
    let owner_pos_y = PostureModule::pos_y(owner_module_accessor);
    let owner_pos_z = PostureModule::pos_z(owner_module_accessor);
    let kind = utility::get_kind(&mut *weapon.module_accessor);
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    if WorkModule::get_int(owner_module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND) != *FIGHTER_EDGE_SPECIAL_N_L {
        PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x + (25.0 * lr), y: owner_pos_y + 5.0, z: owner_pos_z});
    }
    else {
        if kind == *WEAPON_KIND_YOUNGLINK_BOWARROW {
            PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x, y: owner_pos_y + 10.0, z: owner_pos_z});
        }
        else if kind == *WEAPON_KIND_TOONLINK_BOWARROW {
            PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x, y: owner_pos_y + 10.0, z: owner_pos_z});
        }
        else {
            PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x + (12.5 * lr), y: owner_pos_y - 3.5, z: owner_pos_z});
        }
    }
    0.into()
}

unsafe extern "C" fn status_bandana_spear_Fly_Init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let lr = PostureModule::lr(owner_module_accessor);
    let kind = utility::get_kind(&mut *weapon.module_accessor);
    PostureModule::set_lr(weapon.module_accessor, lr);
    PostureModule::update_rot_y_lr(weapon.module_accessor);
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, fly_speed_x * lr, fly_speed_y);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -air_accel_y * air_accel_y_mul);
    0.into()
}

unsafe extern "C" fn status_bandana_spear_Fly_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(bandana_spear_Fly_Main_loop as *const () as _))
}


unsafe extern "C" fn bandana_spear_Fly_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        weapon.change_status(WN_LINK_BOWARROW_STATUS_KIND_STICK.into(), false.into());
    }
    if WorkModule::count_down_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE, 0) {
        let effect_pos = *PostureModule::pos(weapon.module_accessor);
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), &Vector3f{x: effect_pos.x, y: effect_pos.y, z: effect_pos.z - 10.0}, &Vector3f{x: 0.0, y: 300.0, z: 0.0}, 1.0, 0, -1, false, 0) as u32;
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
		return 0.into();
    }
    0.into()
}

unsafe extern "C" fn status_bandana_spear_Fly_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("edge_spear")
    .status(Pre, *WN_LINK_BOWARROW_STATUS_KIND_FLY, status_bandana_spear_Fly_Pre)
    .status(Init, *WN_LINK_BOWARROW_STATUS_KIND_FLY, status_bandana_spear_Fly_Init)
    .status(Main, *WN_LINK_BOWARROW_STATUS_KIND_FLY, status_bandana_spear_Fly_Main)
    .status(End, *WN_LINK_BOWARROW_STATUS_KIND_FLY, status_bandana_spear_Fly_End)
    .install();
}