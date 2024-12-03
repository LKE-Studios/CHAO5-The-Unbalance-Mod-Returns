use crate::imports::BuildImports::*;

pub static life : i32 = 120;
pub static speed : f32 = 3.4;
pub static brake : f32 = 0.0;
pub static is_penetration : i32 = 1;
pub static angle : f32 = 0.0;
pub static ground_coll_offset_x : f32 = -0.5;
pub static ground_coll_offset_y : f32 = 0.5;

unsafe extern "C" fn status_kamek_pinkmagic_Regular_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn status_kamek_pinkmagic_Regular_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
    let float_charge = WorkModule::get_float(weapon.module_accessor, WEAPON_KAMEK_PINKMAGIC_INSTANCE_WORK_ID_FLOAT_CHARGE);
    AttackModule::set_lerp_ratio(weapon.module_accessor, float_charge, 0);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(kamek_pinkmagic_Regular_Main_loop as *const () as _))
}

unsafe extern "C" fn kamek_pinkmagic_Regular_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let mut bool_value = false;
    if weapon.global_table[IS_STOP].get_bool() {
        let sum_speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if sum_speed_x >= 0.00001 {
            if 0.0 >= sum_speed_x {
                if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) {
                    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
                }
            }
            else {
                if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) {
                    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
                }
            }
        }
        else {
            if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) {
                if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) {
                    return 0.into();
                }
            }
            else {
                notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            }
        }
    }
    else {
        let int_life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        if int_life <= 0 {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            return 1.into();
        }
    }
    if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR) {
        if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND) {
            if WorkModule::is_flag(weapon.module_accessor, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END) {
                bool_value = true;
            }
        }
        bool_value = false;
    }
    if !bool_value {
        if weapon.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            weapon.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            WorkModule::on_flag(weapon.module_accessor, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
        }
        else {
            weapon.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            WorkModule::on_flag(weapon.module_accessor, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
        }
    }
    if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR) {
        if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND) {
            if WorkModule::is_flag(weapon.module_accessor, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END) {
                bool_value = MotionModule::is_end(weapon.module_accessor);
            }
            else {
                bool_value = false;
            }
        }
    }
    else {
        bool_value = true;
    }
    if bool_value {
        WorkModule::off_flag(weapon.module_accessor, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
        WorkModule::off_flag(weapon.module_accessor, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
        WorkModule::off_flag(weapon.module_accessor, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
    }
    0.into()
}

unsafe extern "C" fn status_kamek_pinkmagic_Regular_Exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let current_frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let float_angle = WorkModule::get_float(weapon.module_accessor, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLOAT_ANGLE);
    let float_speed = WorkModule::get_float(weapon.module_accessor, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLOAT_SPEED);
    let lr = PostureModule::lr(weapon.module_accessor);
    let speed_x = float_angle.to_radians().cos() * float_speed * lr;
    let speed_y = float_angle.to_radians().sin() * float_speed;
    let mut new_speed = float_speed - brake;
    if current_frame < 1.0 {
        kamek_pinkmagic_Regular_function(weapon);
    }
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    if new_speed < 0.01 {
        new_speed = 0.01;
    }
    WorkModule::set_float(weapon.module_accessor, speed, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLOAT_SPEED);
    0.into()
}

unsafe extern "C" fn status_kamek_pinkmagic_Regular_MapCorrection(weapon: &mut L2CWeaponCommon) -> L2CValue {
    kamek_pinkmagic_Regular_function(weapon);
    0.into()
}

unsafe extern "C" fn kamek_pinkmagic_Regular_function(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let float_angle = WorkModule::get_float(weapon.module_accessor, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLOAT_ANGLE);
    let mut new_angle = float_angle;
    if weapon.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            let get_touch_normal = GroundModule::get_touch_normal(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
            let touch_x = get_touch_normal.x;
            let touch_y = get_touch_normal.y;
            let mut vector = weapon.Vector2__create(touch_x.into(), touch_y.into());
            let vec_x = vector["x"].get_f32();
            let vec_y = vector["y"].get_f32();
            vector["x"].assign(&L2CValue::F32(float_angle));
            let combined_value = (vec_x * vec_y).atan();
            let lr = PostureModule::lr(weapon.module_accessor);
            new_angle = combined_value * lr;
        }
    }
    let new_angle_deg = new_angle.to_degrees();
    let angle_deg = angle.to_degrees();
    if angle_deg != new_angle_deg {
        PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: new_angle_deg, y: 0.0, z: 0.0}, 0);
        WorkModule::set_float(weapon.module_accessor, new_angle_deg, *WEAPON_KIRBY_FINALCUTTERSHOT_INSTANCE_WORK_ID_FLOAT_ANGLE);
    }
    0.into()
}

unsafe extern "C" fn status_kamek_pinkmagic_Regular_FixCamera(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn status_kamek_pinkmagic_Regular_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ness_pinkmagic")
    .status(Pre, *WEAPON_KIRBY_FINALCUTTERSHOT_STATUS_KIND_REGULAR, status_kamek_pinkmagic_Regular_Pre)
    .status(Main, *WEAPON_KIRBY_FINALCUTTERSHOT_STATUS_KIND_REGULAR, status_kamek_pinkmagic_Regular_Main)
    .status(Exec, *WEAPON_KIRBY_FINALCUTTERSHOT_STATUS_KIND_REGULAR, status_kamek_pinkmagic_Regular_Exec)
    .status(MapCorrection, *WEAPON_KIRBY_FINALCUTTERSHOT_STATUS_KIND_REGULAR, status_kamek_pinkmagic_Regular_MapCorrection)
    .status(FixCamera, *WEAPON_KIRBY_FINALCUTTERSHOT_STATUS_KIND_REGULAR, status_kamek_pinkmagic_Regular_FixCamera)
    .status(End, *WEAPON_KIRBY_FINALCUTTERSHOT_STATUS_KIND_REGULAR, status_kamek_pinkmagic_Regular_End)
    .install();
}