use crate::imports::BuildImports::*;

unsafe extern "C" fn status_sans_gaster_Shoot_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn status_sans_gaster_Shoot_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if !WorkModule::is_flag(owner_module_accessor, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_GASTER_SIZE) {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("shoot"), 0.0, 1.0, false, 0.0, false, false);
        ModelModule::set_scale(weapon.module_accessor, 1.15);
        AttackModule::set_attack_scale(weapon.module_accessor, 1.15, true);
    }
    else {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("final"), 0.0, 1.0, false, 0.0, false, false);
        ModelModule::set_scale(weapon.module_accessor, 6.0);
        AttackModule::set_attack_scale(weapon.module_accessor, 6.0, true);
    }
    weapon.fastshift(L2CValue::Ptr(sans_gaster_Shoot_Main_loop as *const () as _))
}

unsafe extern "C" fn sans_gaster_Shoot_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let remaining_life = life <= 0;
    if life < 75 {
        StatusModule::change_status_force(weapon.module_accessor, *WEAPON_SANS_GASTER_STATUS_KIND_SHOT, false);
    }
    if !remaining_life {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        weapon.pop_lua_stack(1);
        if remaining_life {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            weapon.pop_lua_stack(1);
            return 0.into();
        }
    }
    else {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    0.into()
}

unsafe extern "C" fn status_sans_gaster_Shoot_Exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn status_sans_gaster_Shoot_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
	0.into()
}

pub fn install() {
    Agent::new("palutena_gaster")
	.status(Pre, *WEAPON_KOOPAJR_CANNONBALL_STATUS_KIND_SHOOT, status_sans_gaster_Shoot_Pre)
	.status(Main, *WEAPON_KOOPAJR_CANNONBALL_STATUS_KIND_SHOOT, status_sans_gaster_Shoot_Main)
    .status(Exec, *WEAPON_KOOPAJR_CANNONBALL_STATUS_KIND_SHOOT, status_sans_gaster_Shoot_Exec)
    .status(End, *WEAPON_KOOPAJR_CANNONBALL_STATUS_KIND_SHOOT, status_sans_gaster_Shoot_End)
    .install();
}