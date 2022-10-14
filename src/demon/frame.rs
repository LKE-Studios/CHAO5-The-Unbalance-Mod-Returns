use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use crate::utils::*;
use smashline::*;
use smash::app::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lua2cpp::L2CFighterBase;
use smash::phx::Vector3f;
use smash_script::*;
use smash::phx::Hash40;

static mut FLOAT : [i32; 8] = [0; 8]; //Logs Float Time
static mut START_FLOAT : [bool; 8] = [false; 8];
static mut CHECK_FLOAT : [i32; 8] = [0; 8];
static mut CHECK_FLOAT_MAX : i32 = 16; //Frames where jump needs to be held to start floating
static mut X : [f32; 8] = [0.0; 8]; //Logs speed
static mut Y : [f32; 8] = [0.0; 8]; //Logs speed
static mut FLOAT_MAX : i32 = 900; //Frames this bitch can float (In frames, 300 = 5 seconds)
static mut X_MAX : f32 = 2.2; //Max Horizontal movespeed
static mut X_ACCEL_MUL : f32 = 0.09; //Air Accel Mul
static mut Y_MAX : f32 = 2.15; //Max Vertical movespeed

#[fighter_frame( agent = FIGHTER_KIND_DEMON )]
pub fn demon_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let kind = smash::app::utility::get_kind(boma); 
        if kind == *FIGHTER_KIND_DEMON {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
        }
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if StatusModule::situation_kind(fighter.module_accessor) != SITUATION_KIND_AIR
        || !sv_information::is_ready_go()
        || [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status_kind) {
            FLOAT[ENTRY_ID] = 0;
            START_FLOAT[ENTRY_ID] = false;
            CHECK_FLOAT[ENTRY_ID] = 0;
        };
        if FLOAT[ENTRY_ID] == 1{
            if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_MOTION_AIR
            && [
                *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S,
                *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END
            ].contains(&status_kind) == false {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            };
        };
        if StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_AIR {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                CHECK_FLOAT[ENTRY_ID] += 1;
            } else {
                CHECK_FLOAT[ENTRY_ID] = 0;
            };
            if CHECK_FLOAT[ENTRY_ID] >= CHECK_FLOAT_MAX && FLOAT[ENTRY_ID] == 0 {
                START_FLOAT[ENTRY_ID] = true;
            };
        };
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind)
        && FLOAT[ENTRY_ID] > 1{
            FLOAT[ENTRY_ID] = 1;
        };
        if FLOAT[ENTRY_ID] > 1{
            FLOAT[ENTRY_ID] -= 1;
            if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            };
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
                FLOAT[ENTRY_ID] = 1;
            };
            if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                FLOAT[ENTRY_ID] = 1;
            };
            let mut y_add;
            let mut x_add;
            x_add = (stick_x)*X_ACCEL_MUL;
            y_add = (stick_y)*X_ACCEL_MUL;
            if x_add > 0.0 && X[ENTRY_ID] > X_MAX {
                x_add = 0.0;
            };
            if x_add < 0.0 && X[ENTRY_ID] < X_MAX*-1.0 {
                x_add = 0.0;
            };
            if y_add > 0.0 && Y[ENTRY_ID] > Y_MAX {
                y_add = 0.0;
            };
            if y_add < 0.0 && Y[ENTRY_ID] < Y_MAX*-1.0 {
                y_add = 0.0;
            };
            println!("x{}, y{}", X[ENTRY_ID], Y[ENTRY_ID]);
            println!("x_add{}, y_add{}", x_add, y_add);
            X[ENTRY_ID] += x_add;
            Y[ENTRY_ID] += y_add;
            macros::SET_SPEED_EX(fighter, X[ENTRY_ID], Y[ENTRY_ID], *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        } else {
            X[ENTRY_ID] = 0.0;
            Y[ENTRY_ID] = 0.0;
        };
        if START_FLOAT[ENTRY_ID] == true {
            FLOAT[ENTRY_ID] = FLOAT_MAX;
            START_FLOAT[ENTRY_ID] = false;
            if status_kind == *FIGHTER_STATUS_KIND_JUMP {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            };
            if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
            };
            if [*FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && FLOAT[ENTRY_ID] > 1{
                FLOAT[ENTRY_ID] = 1;
            };
        };
        if status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S || status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2F || status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L {
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                COUNTER[ENTRY_ID] += 1;
                IS_CRIT[ENTRY_ID] = true;
                if COUNTER[ENTRY_ID] < 2 {
                    EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40::new("sys_bg_criticalhit"), smash::phx::Hash40::new("haver"), &Vector3f{x: 0.0, y: 8.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                    CURRENTFRAME[ENTRY_ID] = MotionModule::frame(fighter.module_accessor);
                    SlowModule::set_whole(fighter.module_accessor, 2, 0);
                    macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
                    macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
                    macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 4.0,/*no*/ 0.0,/*zoom*/ 2.1,/*yrot*/ 0.0,/*xrot*/ 0.0 * PostureModule::lr(boma));
                }
            }
            if MotionModule::frame(fighter.module_accessor) >= (CURRENTFRAME[ENTRY_ID] + 1.0) && IS_CRIT[ENTRY_ID] {
                COUNTER[ENTRY_ID] = 0;
                SlowModule::clear_whole(fighter.module_accessor);
                CameraModule::reset_all(fighter.module_accessor);
                EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
                HitModule::set_status_all(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                if StatusModule::status_kind(fighter.module_accessor) != 510 {
                    macros::CAM_ZOOM_OUT(fighter);
                }
            }
            if IS_CRIT[ENTRY_ID] && MotionModule::frame(fighter.module_accessor) < 2.0 {
                macros::CAM_ZOOM_OUT(fighter);
                IS_CRIT[ENTRY_ID] = false;
                EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
                HitModule::set_status_all(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                SlowModule::clear_whole(fighter.module_accessor);
            };
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        demon_opff
    );
}