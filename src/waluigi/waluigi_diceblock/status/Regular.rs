use crate::imports::BuildImports::*;

unsafe extern "C" fn status_waluigi_diceblock_Regular_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn status_waluigi_diceblock_Regular_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(waluigi_diceblock_Regular_Main_loop as *const () as _))
}

unsafe extern "C" fn waluigi_diceblock_Regular_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let remaining_life = life <= 0;
    if !remaining_life {
        if !GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
            return 0.into();
        }
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        weapon.pop_lua_stack(1);
        MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_EFFECT, Hash40::new("effect_bound"), -1);
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

unsafe extern "C" fn status_waluigi_diceblock_Regular_Exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
	WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
	let frame = MotionModule::frame(weapon.module_accessor);
	let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
	if frame <= 1.0 {
		WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
	}
	if life < 20 {
		StatusModule::change_status_force(weapon.module_accessor, WEAPON_WALUIGI_DICEBLOCK_STATUS_KIND_BREAK, false);
	}
	0.into()
}

pub fn install() {
    Agent::new("dolly_diceblock")
	.status(Pre, *WEAPON_MARIO_FIREBALL_STATUS_KIND_REGULAR, status_waluigi_diceblock_Regular_Pre)
	.status(Main, *WEAPON_MARIO_FIREBALL_STATUS_KIND_REGULAR, status_waluigi_diceblock_Regular_Main)
    .status(Exec, *WEAPON_MARIO_FIREBALL_STATUS_KIND_REGULAR, status_waluigi_diceblock_Regular_Exec)
    .install();
}
