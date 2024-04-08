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

pub unsafe extern "C" fn frame_silver_Exec(fighter: &mut L2CFighterCommon) {
    ModelModule::set_scale(fighter.module_accessor, 0.865);
    AttackModule::set_attack_scale(fighter.module_accessor, 0.865, true);
    GrabModule::set_size_mul(fighter.module_accessor, 0.865);
}

pub unsafe extern "C" fn frame_silver_Main(fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
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
        silver_float(fighter);
        misc_silver(fighter);
        motion_main_silver(fighter);
        special_hi_silver(fighter);
        if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_SHOOT {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR && SPECIAL_N_HAS_STALL[ENTRY_ID] {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                SPECIAL_N_HAS_STALL[ENTRY_ID] = false;
            };
            if frame >= 20.0 && frame <= 40.0 && !SPECIAL_N_GET_ANGLE[ENTRY_ID] {
                if stick_x >= -0.2 && stick_x <= 0.2 
                && stick_y >= -0.2 && stick_y <= 0.2 {
                    stick_dir = 361.0;
                } else if stick_dir <= -67.5 {
                    stick_dir *= -1.0;
                };
                if stick_dir >= -67.5 && stick_dir < -22.5 && stick_x < 0.0 {
                    SPECIAL_N_DIR[ENTRY_ID] = 1;
                }
                else if stick_dir >= 67.5 && stick_dir <= 90.0 && stick_y < 0.0 {
                    SPECIAL_N_DIR[ENTRY_ID] = 2;
                }
                else if stick_dir >= -67.5 && stick_dir < -22.5 && stick_x > 0.0 {
                    SPECIAL_N_DIR[ENTRY_ID] = 3;
                }
                else if stick_dir >= -22.5 && stick_dir <= 22.5 && stick_x < 0.0 {
                    SPECIAL_N_DIR[ENTRY_ID] = 4;
                }
                else if stick_dir == 361.0 {
                    SPECIAL_N_DIR[ENTRY_ID] = 5;
                }
                else if stick_dir >= -22.5 && stick_dir <= 22.5 && stick_x > 0.0 {
                    SPECIAL_N_DIR[ENTRY_ID] = 6;
                }
                else if stick_dir > 22.5 && stick_dir <= 67.5 && stick_x < 0.0 {
                    SPECIAL_N_DIR[ENTRY_ID] = 7;
                }
                else if stick_dir > 67.5 && stick_dir <= 90.0 && stick_y > 0.0 {
                    SPECIAL_N_DIR[ENTRY_ID] = 8;
                }
                else  {
                    SPECIAL_N_DIR[ENTRY_ID] = 9;
                };
                //SpecialN Drift
                if SPECIAL_N_DIR[ENTRY_ID] == 1 {
                    SPECIAL_N_ANGLE[ENTRY_ID] = -1.0;
                    SPECIAL_N_GET_ANGLE[ENTRY_ID] = true;
                }
                else if SPECIAL_N_DIR[ENTRY_ID] == 2 {
                    SPECIAL_N_ANGLE[ENTRY_ID] = -1.0;
                    SPECIAL_N_GET_ANGLE[ENTRY_ID] = true;
                }
                else if SPECIAL_N_DIR[ENTRY_ID] == 3 {
                    SPECIAL_N_ANGLE[ENTRY_ID] = -1.0;
                    SPECIAL_N_GET_ANGLE[ENTRY_ID] = true;
                }
                else if SPECIAL_N_DIR[ENTRY_ID] == 4 {
                    SPECIAL_N_ANGLE[ENTRY_ID] = 0.0;
                    SPECIAL_N_GET_ANGLE[ENTRY_ID] = true;
                }
                else if SPECIAL_N_DIR[ENTRY_ID] == 5 {
                    SPECIAL_N_ANGLE[ENTRY_ID] = 0.0;
                    SPECIAL_N_GET_ANGLE[ENTRY_ID] = true;
                }
                else if SPECIAL_N_DIR[ENTRY_ID] == 6 {
                    SPECIAL_N_ANGLE[ENTRY_ID] = 0.0;
                    SPECIAL_N_GET_ANGLE[ENTRY_ID] = true;
                }
                else if SPECIAL_N_DIR[ENTRY_ID] == 7 {
                    SPECIAL_N_ANGLE[ENTRY_ID] = 0.0;
                    SPECIAL_N_GET_ANGLE[ENTRY_ID] = true;
                }
                else if SPECIAL_N_DIR[ENTRY_ID] == 8 {
                    SPECIAL_N_ANGLE[ENTRY_ID] = 0.0;
                    SPECIAL_N_GET_ANGLE[ENTRY_ID] = true;
                }
                else if SPECIAL_N_DIR[ENTRY_ID] == 9 {
                    SPECIAL_N_ANGLE[ENTRY_ID] = 0.0;
                    SPECIAL_N_GET_ANGLE[ENTRY_ID] = true;
                }
            };
        } 
        if status_kind != *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_SHOOT {
            SPECIAL_N_GET_ANGLE[ENTRY_ID] = false;
        }   
    }
}

pub unsafe extern "C" fn motion_main_silver(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if [hash40("special_n_hold"),hash40("special_n_start")].contains(&motion_kind) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_SHOOT, true);
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
    };
    if [hash40("special_air_n_hold"),hash40("special_air_n_start")].contains(&motion_kind) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_SHOOT, true);
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
    };
    if [hash40("special_n_max"),hash40("special_n_cancel")].contains(&motion_kind) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
    };
    if [hash40("special_air_n_max"),hash40("special_air_n_cancel"),hash40("special_air_n_jump_cancel")].contains(&motion_kind) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
    };
    if [hash40("attack_100_start"), hash40("attack_100"), hash40("attack_100_end")].contains(&motion_kind) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
    };
    if ![*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_AIR, 
        *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_MAX, 
        *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_SHOOT].contains(&status_kind) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MEWTWO_GENERATE_ARTICLE_SHADOWBALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_mewtwo_special_n01"), 0);
    };
    
}

pub unsafe extern "C" fn special_hi_silver(fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * lr;
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3 && !HAS_ALREADY_TELECANCEL[ENTRY_ID] {
        ATTACK_AIR_WINDOW[ENTRY_ID] += 1;
    } 
    else {
        ATTACK_AIR_WINDOW[ENTRY_ID] = 0;
    };
    if ATTACK_AIR_WINDOW[ENTRY_ID] >= 15 && ATTACK_AIR_WINDOW[ENTRY_ID] <= MAX_ATTACK_AIR_WINDOW && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            HAS_ATTACK_AIR[ENTRY_ID] = true;
            HAS_ALREADY_TELECANCEL[ENTRY_ID] = true;
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
        };
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
            HAS_ATTACK_AIR[ENTRY_ID] = true;
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        };
    };
    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR {
        HAS_ALREADY_TELECANCEL[ENTRY_ID] = false;
        HAS_ATTACK_AIR[ENTRY_ID] = false;
    };
    if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) || !sv_information::is_ready_go() {
        HAS_ATTACK_AIR[ENTRY_ID] = false;
        ATTACK_AIR_WINDOW[ENTRY_ID] = 0;
    };
}

pub unsafe extern "C" fn misc_silver(fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR {
        SPECIAL_N_HAS_STALL[ENTRY_ID] = true;
        BASE = Vector3f { x: 0.0, y: 2.8, z: -7.0 };
        BASE_TRAIL = Vector3f { x: 0.0, y: 2.8, z: -16.0 };
        BASE_TRAIL2 = Vector3f { x: 0.0, y: 2.8, z: -25.0 };
        BASE_TRAIL3 = Vector3f { x: 0.0, y: 2.8, z: -34.0 };
        BASE_TRAIL4 = Vector3f { x: 0.0, y: 2.8, z: -43.0 };
    };
}

pub unsafe extern "C" fn silver_float(fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if FLOAT[ENTRY_ID] % 5 == 0 && FLOAT[ENTRY_ID] > 1 {
        BURN_COLOR(fighter, 0.0, 2.55, 2.55, 0.5);
        FLASH(fighter, 0.3, 0.7, 0.7, 0.3);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), -1, 0, 0, 0, 0, 0, 0.3, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 1, 0, 0, 0, 0, 0, 0.3, true, *EF_FLIP_YZ);
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_n09"));
        KILL_EFFECTS[ENTRY_ID] = true;
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

pub fn install() {
    Agent::new("mewtwo")
    .on_line(Main, frame_silver_Main)
    .install();
    
    Agent::new("mewtwo_shadowball")
    .on_line(Main, frame_silver_shadowball_Main)
    .install();
}