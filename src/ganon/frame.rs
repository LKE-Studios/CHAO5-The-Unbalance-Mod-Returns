use crate::imports::BuildImports::*;

static mut COUNTER: [i32; 8] = [0; 8];
static mut CURRENTFRAME: [f32; 8] = [0.0; 8];
static mut IS_CRIT: [bool; 8] = [false; 8];
pub static mut FIGHTER_STATUS_GANON_UNIQ_APPEAL_COUNTER: [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
fn frame_ganon(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        //let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        //let frame = MotionModule::frame(fighter.module_accessor);

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            COUNTER[ENTRY_ID] += 1;
            IS_CRIT[ENTRY_ID] = true;
            if COUNTER[ENTRY_ID] < 2 {
                EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_bg_criticalhit"), smash::phx::Hash40::new("haver"), &Vector3f{x: 0.0, y: 8.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                CURRENTFRAME[ENTRY_ID] = MotionModule::frame(module_accessor);
                SlowModule::set_whole(module_accessor, 2, 0);
                PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
                QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
                CAM_ZOOM_IN_arg5(fighter, /*frames*/ 4.0,/*no*/ 0.0,/*zoom*/ 2.1,/*yrot*/ 0.0,/*xrot*/ 0.0 * PostureModule::lr(module_accessor));
            }
        }
        if MotionModule::frame(module_accessor) >= (CURRENTFRAME[ENTRY_ID] + 1.0) && IS_CRIT[ENTRY_ID] {
            COUNTER[ENTRY_ID] = 0;
            SlowModule::clear_whole(module_accessor);
            CameraModule::reset_all(module_accessor);
            EffectModule::kill_kind(module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
            HitModule::set_status_all(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            if StatusModule::status_kind(module_accessor) != 510 {
                CAM_ZOOM_OUT(fighter);
            }
        }
        if IS_CRIT[ENTRY_ID] && MotionModule::frame(module_accessor) < 2.0 {
            CAM_ZOOM_OUT(fighter);
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
        if FIGHTER_STATUS_GANON_UNIQ_APPEAL_COUNTER[ENTRY_ID] {
            if StopModule::is_hit(fighter.module_accessor) {
                if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
                }
                else {
                    FIGHTER_STATUS_GANON_UNIQ_APPEAL_COUNTER[ENTRY_ID] = false;
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
