use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_shizue_pot_Burst_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let weapon_kind = weapon.global_table[FIGHTER_KIND].get_i32();
    if weapon_kind == *WEAPON_KIND_MURABITO_FLOWERPOT {
        SoundModule::play_se(weapon.module_accessor, Hash40::new("se_murabito_attackdash02"), true, true, false, false, enSEType(0));
    }
    else if weapon_kind == *WEAPON_KIND_SHIZUE_POT {
        SoundModule::play_se(weapon.module_accessor, Hash40::new("se_shizue_attackdash_03"), true, true, false, false, enSEType(0));
    }
    VisibilityModule::set_whole(weapon.module_accessor, false);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("burst"), 0.0, 1.0, false, 0.0, false, false);
    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_MURABITO_FLOWERPOT_INSTANCE_WORK_ID_FLAG_BOUND) {
        WorkModule::off_flag(weapon.module_accessor, *WEAPON_MURABITO_FLOWERPOT_INSTANCE_WORK_ID_FLAG_BOUND);
    }
    weapon.fastshift(L2CValue::Ptr(shizue_pot_Burst_Main_loop as *const () as _))
}

unsafe extern "C" fn shizue_pot_Burst_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    weapon.pop_lua_stack(1);
    0.into()
}

pub fn install() {
    Agent::new("shizue_pot")
    .status(Main, *WEAPON_MURABITO_FLOWERPOT_STATUS_KIND_BURST, status_shizue_pot_Burst_Main)
    .install();
}