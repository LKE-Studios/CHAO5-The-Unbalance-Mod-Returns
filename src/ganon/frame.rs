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
fn ganon_opff(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let kind = smash::app::utility::get_kind(module_accessor);
        let ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        if kind == *FIGHTER_KIND_GANON {
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
        }
    }
}


pub fn install() {
    smashline::install_agent_frames!(
        ganon_opff
    );
}
