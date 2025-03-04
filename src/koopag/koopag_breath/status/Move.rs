use crate::imports::BuildImports::*;

unsafe extern "C" fn status_koopag_breath_Move_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let hit_frames = WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("hit_frames"));
    let life = WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("life"));
    let min_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("min_speed"));
    let max_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("max_speed"));
    WorkModule::set_int(weapon.module_accessor, hit_frames as i32, *WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_INT_HIT_FRAME);
    WorkModule::set_int(weapon.module_accessor, life as i32, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life as i32, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    let rand_val = sv_math::rand(hash40("weapon"), 10);
    let speed_mul = WorkModule::get_float(weapon.module_accessor, *WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_FLOAT_SPEED_MUL);
    let angle = WorkModule::get_float(weapon.module_accessor, *WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_FLOAT_ANGLE);
    let lr = PostureModule::lr(weapon.module_accessor);
    let speed_x = angle.to_radians().sin() * max_speed * lr;
    let speed_y = angle.to_radians().cos() * max_speed;
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    if !StopModule::is_stop(weapon.module_accessor) {
        koopag_breath_Move_Sub_Status(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(koopag_breath_Move_Sub_Status as *const () as _));
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(koopag_breath_Move_Main_loop as *const () as _))
}

unsafe extern "C" fn koopag_breath_Move_Sub_Status(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn koopag_breath_Move_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let pos = PostureModule::pos(weapon.module_accessor);
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ALL) {
        SoundModule::play_se(weapon.module_accessor, Hash40::new("se_koopag_fireball_impact01"), true, false, false, false, enSEType(0));
    }
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
        EffectModule::req(weapon.module_accessor, Hash40::new("brave_fire3_burst"), pos, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);
        SoundModule::play_se(weapon.module_accessor, Hash40::new("se_common_bomb_m"), true, false, false, false, enSEType(0));
        SoundModule::play_se(weapon.module_accessor, Hash40::new("se_koopag_fireball_impact01"), true, false, false, false, enSEType(0));
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    if life <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

pub fn install() {
    Agent::new("koopag_breath")
    .status(Main, *WEAPON_KOOPA_BREATH_STATUS_KIND_MOVE, status_koopag_breath_Move_Main)
    .install();
}