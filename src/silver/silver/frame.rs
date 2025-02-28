use crate::imports::BuildImports::*;
use crate::silver::silver::effect::*;
use crate::mewtwo::mewtwo::frame::*;

pub static mut ESCAPE_AIR_DIR : [i32; 8] = [0; 8];
static mut BAN_UPB : [bool; 8] = [false; 8];
static mut WAS_UPB : [bool; 8] = [false; 8];
pub static mut SPECIAL_N_HAS_STALL : [bool; 8] = [true; 8];
static mut ATTACK_AIR_WINDOW : [i32; 8] = [0; 8];
static mut MAX_ATTACK_AIR_WINDOW : i32 = 75;
static mut HAS_ATTACK_AIR: [bool; 8] = [false; 8];
static mut HAS_ALREADY_TELECANCEL: [bool; 8] = [false; 8];
pub static mut SPECIAL_HI_X: [f32; 8] = [0.0; 8];
pub static mut SPECIAL_HI_DIR: [i32; 8] = [0; 8];
pub static mut SPECIAL_N_ANGLE : [f32; 65544] = [0.0; 65544];
pub static mut SPECIAL_N_DIR: [i32; 65544] = [0; 65544];
pub static mut SPECIAL_N_GET_ANGLE: [bool; 65544] = [false; 65544];
pub static mut STICK_DIRECTION : [f32; 8] = [0.0; 8];
pub static mut KILL_EFFECTS : [bool; 8] = [false; 8];
pub static mut BOX_HIT : [bool; 8] = [false; 8];
static mut DID_EFF : [bool; 8] = [false; 8];
pub static mut CLOUD_COUNT : [f32; 8] = [0.0; 8];
pub static mut CLOUD_EFF : [u32; 8] = [0; 8];
pub static mut CLOUD_EFF2 : [u32; 8] = [0; 8];
pub static mut CLOUD_POS : Vector3f = Vector3f { x: 0.0, y: 10.0, z: 10.0 };
pub static mut CLOUD_POS2 : Vector3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
pub static mut CLOUD_ROT : Vector3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };

pub unsafe extern "C" fn frame_silver_Main(fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let mut stick_dir = ControlModule::get_stick_dir(fighter.module_accessor) * (180.0 / PI);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SILVER = color >= 120 && color <= 127;
    if SILVER {
        ModelModule::set_scale(fighter.module_accessor, 0.865);
        AttackModule::set_attack_scale(fighter.module_accessor, 0.865, true);
        GrabModule::set_size_mul(fighter.module_accessor, 0.865);
        if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR {
            BASE = Vector3f { x: 0.0, y: 2.8, z: -7.0 };
            BASE_TRAIL = Vector3f { x: 0.0, y: 2.8, z: -16.0 };
            BASE_TRAIL2 = Vector3f { x: 0.0, y: 2.8, z: -25.0 };
            BASE_TRAIL3 = Vector3f { x: 0.0, y: 2.8, z: -34.0 };
            BASE_TRAIL4 = Vector3f { x: 0.0, y: 2.8, z: -43.0 };
        };
        motion_main_silver(fighter);
        if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_SHOOT {
            if situation_kind == *SITUATION_KIND_AIR {
                if SPECIAL_N_HAS_STALL[ENTRY_ID] {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                    SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    SPECIAL_N_HAS_STALL[ENTRY_ID] = false;
                }
            }
        }
    }
}

pub unsafe extern "C" fn motion_main_silver(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if ![*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_AIR, 
        *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_MAX, 
        *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_SHOOT].contains(&status_kind) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MEWTWO_GENERATE_ARTICLE_SHADOWBALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_mewtwo_special_n01"), 0);
    };
    
}

pub unsafe extern "C" fn final_silver(fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SILVER_GENERATE_ARTICLE_BOX) && motion_kind != hash40("final") && motion_kind != hash40("final_air") {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_bg_black"), false, false);
    };
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SILVER_GENERATE_ARTICLE_BOX) {
        DID_EFF[ENTRY_ID] = false;
        BOX_HIT[ENTRY_ID] = false;
    };
    if [hash40("final"), hash40("final_air")].contains(&motion_kind) {
        if frame == 37.0 {
            CLOUD_EFF[ENTRY_ID] = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_misfire"), Hash40::new("top"), &CLOUD_POS, &CLOUD_ROT, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(fighter.module_accessor, CLOUD_EFF[ENTRY_ID], 0.0, 1.0, 1.0);
            EffectModule::set_alpha(fighter.module_accessor, CLOUD_EFF[ENTRY_ID], 0.5);
            EffectModule::set_rate(fighter.module_accessor, CLOUD_EFF[ENTRY_ID], 0.001); 
        }
        else if frame > 37.0 && frame < 67.0 {
            CLOUD_COUNT[ENTRY_ID] += 1.0;
            let scale = Vector3f { x: CLOUD_COUNT[ENTRY_ID]/2.0, y: CLOUD_COUNT[ENTRY_ID]/2.0, z: CLOUD_COUNT[ENTRY_ID]/2.0};
            EffectModule::set_scale(fighter.module_accessor, CLOUD_EFF[ENTRY_ID], &scale);
        }
        else {
            CLOUD_COUNT[ENTRY_ID] = 0.0;
            let scale = Vector3f { x: 0.00001, y: 0.00001, z: 0.00001};
            EffectModule::set_scale(fighter.module_accessor, CLOUD_EFF[ENTRY_ID], &scale);
        };
        if frame >= 100.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
        };
    };
}

pub unsafe extern "C" fn frame_silver_shadowball_Main(weapon: &mut L2CFighterBase) {
    let owner_target_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((owner_target_id) as u32);
    let ENTRY_ID = WorkModule::get_int(&mut *owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(weapon.module_accessor);
    let SILVER = (WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    let SILVER_WEAPON = (WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 137);
    if SILVER_WEAPON && [*FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) == true { //Silver
        if PostureModule::scale(weapon.module_accessor) != 1.75  && status_kind == *WEAPON_MEWTWO_SHADOWBALL_STATUS_KIND_CHARGE {
            PostureModule::set_scale(weapon.module_accessor, 1.75, false);
        };
        if PostureModule::scale(weapon.module_accessor) != 1.0  && status_kind == *WEAPON_MEWTWO_SHADOWBALL_STATUS_KIND_SHOOT {
            PostureModule::set_scale(weapon.module_accessor, 1.0, false);
        };
    };
    if SILVER_WEAPON && [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD].contains(&status_kind) == true {
        if MotionModule::frame(owner_module_accessor) <= 17.0 {
            if PostureModule::scale(weapon.module_accessor) != 1.75  && status_kind == *WEAPON_MEWTWO_SHADOWBALL_STATUS_KIND_CHARGE {
                PostureModule::set_scale(weapon.module_accessor, 1.75, false);
            };
            if PostureModule::scale(weapon.module_accessor) != 1.0  && status_kind == *WEAPON_MEWTWO_SHADOWBALL_STATUS_KIND_SHOOT {
                PostureModule::set_scale(weapon.module_accessor, 1.0, false);
            };
        };
    };
    if SILVER_WEAPON {
        if GroundModule::is_wall_touch_line(owner_module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) || AttackModule::is_infliction_status(owner_module_accessor, *COLLISION_KIND_MASK_ALL) {
            EffectModule::kill_kind(owner_module_accessor, Hash40::new("sys_hit_cut_shock"), false, false);
        };
    };
}

pub unsafe extern "C" fn frame_silver_box_Main(weapon: &mut L2CFighterBase) {
    let owner_target_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((owner_target_id) as u32);
    let ENTRY_ID = WorkModule::get_int(&mut *owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    BURN_COLOR(weapon, 0.0, 2.55, 2.55, 0.2);
    FLASH(weapon, 0.3, 0.7, 0.7, 0.1);
    if !DID_EFF[ENTRY_ID] {
        CLOUD_EFF2[ENTRY_ID] = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_misfire"), Hash40::new("top"), &CLOUD_POS2, &CLOUD_ROT, 15.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(weapon.module_accessor, CLOUD_EFF2[ENTRY_ID], 0.0, 1.0, 1.0);
        EffectModule::set_alpha(weapon.module_accessor, CLOUD_EFF2[ENTRY_ID], 0.1);
        EffectModule::set_rate(weapon.module_accessor, CLOUD_EFF2[ENTRY_ID], 0.001); 
        DID_EFF[ENTRY_ID] = true;
    };
}

pub fn install() {
    Agent::new("mewtwo")
    .on_line(Main, frame_silver_Main)
    .install();
    
    Agent::new("mewtwo_shadowball")
    .on_line(Main, frame_silver_shadowball_Main)
    .install();

    Agent::new("mewtwo_box")
    .on_line(Main, frame_silver_box_Main)
    .install();
}