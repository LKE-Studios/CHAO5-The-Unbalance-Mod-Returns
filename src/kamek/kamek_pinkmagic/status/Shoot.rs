use crate::imports::BuildImports::*;
use crate::kamek::kamek::frame::*;

unsafe extern "C" fn status_kamek_pinkmagic_Shoot_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn status_kamek_pinkmagic_Shoot_Init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_pinkmagic"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    ModelModule::set_scale(weapon.module_accessor, 0.0);
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_pinkmagic"), hash40("speed_max"));
    let angle: f32 = 90.0;
    let lr = PostureModule::lr(weapon.module_accessor);
    let speed_x = angle.to_radians().sin() * speed_max * lr;
    let speed_y = angle.to_radians().cos() * speed_max;
    weapon.clear_lua_stack();
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_pos_x = PostureModule::pos_x(owner_module_accessor);
    let owner_pos_y = PostureModule::pos_y(owner_module_accessor);
    let owner_pos_z = PostureModule::pos_z(owner_module_accessor);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x + 4.25, y: owner_pos_y + 9.0, z: owner_pos_z});
    0.into()
}

unsafe extern "C" fn status_kamek_pinkmagic_Shoot_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_pinkmagic"), hash40("speed_max"));
    let angle: f32 = 90.0;
    let lr = PostureModule::lr(weapon.module_accessor);
    let speed_x = angle.to_radians().sin() * speed_max * lr;
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("shoot"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(kamek_pinkmagic_Shoot_Main_loop as *const () as _))
}

unsafe extern "C" fn kamek_pinkmagic_Shoot_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let ENTRY_ID = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let size = (FIGHTER_KAMEK_STATUS_SPECIAL_N_CHARGE[ENTRY_ID] * 0.025) + 0.1;
    if GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        SoundModule::play_se(weapon.module_accessor, Hash40::new("se_ness_special_n04_m"), true, false, false, false, enSEType(0));
        effect!(weapon, *MA_MSC_CMD_EFFECT_EFFECT, hash40("kamek_magicball_hit"), hash40("rot"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.5 + size, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 1.into();
    }
    if life <= 0 {
        effect!(weapon, *MA_MSC_CMD_EFFECT_EFFECT, hash40("sys_erace_smoke"), hash40("rot"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn status_kamek_pinkmagic_Shoot_Exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
	WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
	0.into()
}

pub fn install() {
    Agent::new("ness_pinkmagic")
	.status(Pre, WEAPON_KAMEK_PINKMAGIC_STATUS_KIND_SHOOT, status_kamek_pinkmagic_Shoot_Pre)
    .status(Init, WEAPON_KAMEK_PINKMAGIC_STATUS_KIND_SHOOT, status_kamek_pinkmagic_Shoot_Init)
	.status(Main, WEAPON_KAMEK_PINKMAGIC_STATUS_KIND_SHOOT, status_kamek_pinkmagic_Shoot_Main)
    .status(Exec, WEAPON_KAMEK_PINKMAGIC_STATUS_KIND_SHOOT, status_kamek_pinkmagic_Shoot_Exec)
    .install();
}