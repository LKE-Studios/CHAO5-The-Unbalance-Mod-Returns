use crate::imports::BuildImports::*;

unsafe extern "C" fn status_silver_wave_Start_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn status_silver_wave_Start_Init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_module_accessor = sv_battle_object::module_accessor(owner);
	let lr = PostureModule::lr(owner_module_accessor);
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    let pos_z = PostureModule::pos_z(weapon.module_accessor);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: pos_x + (7.0 * lr), y: pos_y, z: pos_z});
    0.into()
}

unsafe extern "C" fn status_silver_wave_Start_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_wave"), hash40("scale"));
    ModelModule::set_scale(weapon.module_accessor, scale);
    AttackModule::set_attack_scale(weapon.module_accessor, scale, true);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(silver_wave_Start_Main_loop as *const () as _))
}

unsafe extern "C" fn silver_wave_Start_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let remaining_life = life <= 0;
    if !remaining_life {
        if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
            return 0.into();
        }
        else {
            effect!(weapon, *MA_MSC_CMD_EFFECT_EFFECT, hash40("mewtwo_shadowball_bomb"), hash40("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            weapon.pop_lua_stack(1);
        }
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        weapon.pop_lua_stack(1);
        if remaining_life {
            effect!(weapon, *MA_MSC_CMD_EFFECT_EFFECT, hash40("mewtwo_shadowball_bomb"), hash40("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            weapon.pop_lua_stack(1);
        }
    }
    else {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    0.into()
}

unsafe extern "C" fn status_silver_wave_Start_Exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
	WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
	0.into()
}

unsafe extern "C" fn status_silver_wave_Start_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("mewtwo_wave")
	.status(Pre, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, status_silver_wave_Start_Pre)
    .status(Init, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, status_silver_wave_Start_Init)
	.status(Main, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, status_silver_wave_Start_Main)
    .status(Exec, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, status_silver_wave_Start_Exec)
    .status(End, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, status_silver_wave_Start_End)
    .install();
}