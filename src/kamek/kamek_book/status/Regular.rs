use crate::imports::BuildImports::*;

pub static life : i32 = 200;
pub static speed : f32 = 3.5;
pub static scale : f32 = 2.0;

unsafe extern "C" fn status_kamek_book_Regular_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn status_kamek_book_Regular_Init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -speed);
    sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -1.0, -1.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    0.into()
}

unsafe extern "C" fn status_kamek_book_Regular_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    ModelModule::set_scale(weapon.module_accessor, scale);
    AttackModule::set_attack_scale(weapon.module_accessor, scale, true);
    if !StopModule::is_stop(weapon.module_accessor) {
        kamek_book_Regular_Sub_status(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(kamek_book_Regular_Sub_status as *const () as _));
    weapon.fastshift(L2CValue::Ptr(kamek_book_Regular_Main_loop as *const () as _))
}

unsafe extern "C" fn kamek_book_Regular_Sub_status(weapon: &mut L2CWeaponCommon, life_dec: L2CValue) -> L2CValue {
    if life_dec.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn kamek_book_Regular_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let int_life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let remaining_life = int_life <= 0;
    if !remaining_life {
        if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
            return 0.into();
        }
        weapon.change_status(WEAPON_ROCKMAN_HARDKNUCKLE_STATUS_KIND_STICK.into(), false.into());
    }
    else {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    0.into()
}

unsafe extern "C" fn status_kamek_book_Regular_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
	0.into()
}

pub fn install() {
    Agent::new("ness_book")
	.status(Pre, *WEAPON_ROCKMAN_HARDKNUCKLE_STATUS_KIND_REGULAR, status_kamek_book_Regular_Pre)
    .status(Init, *WEAPON_ROCKMAN_HARDKNUCKLE_STATUS_KIND_REGULAR, status_kamek_book_Regular_Init)
	.status(Main, *WEAPON_ROCKMAN_HARDKNUCKLE_STATUS_KIND_REGULAR, status_kamek_book_Regular_Main)
    .status(End, *WEAPON_ROCKMAN_HARDKNUCKLE_STATUS_KIND_REGULAR, status_kamek_book_Regular_End)
    .install();
}