use crate::imports::BuildImports::*;

unsafe extern "C" fn status_metaknight_galaxiabeam_End_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn status_metaknight_galaxiabeam_End_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::set_int(weapon.module_accessor, 8, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, 8, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let pos = *PostureModule::pos(weapon.module_accessor);
    EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y, z: pos.z + 5.0}, &Vector3f {x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);
    EffectModule::req(weapon.module_accessor, Hash40::new("miiswordsman_hensoku_hit"), &Vector3f{x: pos.x, y: pos.y, z: pos.z + 5.0}, &Vector3f {x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);
    weapon.fastshift(L2CValue::Ptr(metaknight_galaxiabeam_End_Main_loop as *const () as _))
}

unsafe extern "C" fn metaknight_galaxiabeam_End_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let int_life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if int_life <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    0.into()
}

unsafe extern "C" fn status_metaknight_galaxiabeam_End_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("metaknight_galaxiabeam")
    .status(Pre, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_END, status_metaknight_galaxiabeam_End_Pre)
    .status(Main, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_END, status_metaknight_galaxiabeam_End_Main)
    .status(End, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_END, status_metaknight_galaxiabeam_End_End)
    .install();
}