use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smashline::*;
use smash_script::*;
use smash::phx::Vector3f;

static mut COUNTER: [i32; 8] = [0; 8];
static mut CURRENTFRAME: [f32; 8] = [0.0; 8];
static mut IS_CRIT: [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
fn frame_ganon(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);

        if motion_kind == hash40("appeal_lw_r") || motion_kind == hash40("appeal_lw_l") {
            if frame > 25.0 && frame < 59.0 {
                if FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) {
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
                        StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
                    }
                }
            }
        }

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            COUNTER[ENTRY_ID] += 1;
            IS_CRIT[ENTRY_ID] = true;
            if COUNTER[ENTRY_ID] < 2 {
                EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_bg_criticalhit"), smash::phx::Hash40::new("haver"), &Vector3f{x: 0.0, y: 8.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                CURRENTFRAME[ENTRY_ID] = MotionModule::frame(module_accessor);
                SlowModule::set_whole(module_accessor, 2, 0);
                macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
                macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
                macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 4.0,/*no*/ 0.0,/*zoom*/ 2.1,/*yrot*/ 0.0,/*xrot*/ 0.0 * PostureModule::lr(module_accessor));
            }
        }
        if MotionModule::frame(module_accessor) >= (CURRENTFRAME[ENTRY_ID] + 1.0) && IS_CRIT[ENTRY_ID] {
            COUNTER[ENTRY_ID] = 0;
            SlowModule::clear_whole(module_accessor);
            CameraModule::reset_all(module_accessor);
            EffectModule::kill_kind(module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
            HitModule::set_status_all(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            if StatusModule::status_kind(module_accessor) != 510 {
                macros::CAM_ZOOM_OUT(fighter);
            }
        }
        if IS_CRIT[ENTRY_ID] && MotionModule::frame(module_accessor) < 2.0 {
            macros::CAM_ZOOM_OUT(fighter);
            IS_CRIT[ENTRY_ID] = false;
            EffectModule::kill_kind(module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
            HitModule::set_status_all(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            SlowModule::clear_whole(module_accessor);
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            fighter.sub_air_check_fall_common();
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
                }
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
                }
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
                }
            }
        };
    }
}


pub fn install() {
    smashline::install_agent_frames!(
        frame_ganon
    );
}
