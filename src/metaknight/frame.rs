use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::phx::Vector2f;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::app::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::{sv_information};
use crate::common::status::glide::*;
use crate::common::status::glide::KineticUtility;
use smash::hash40;

static mut COUNTER : [i32; 8] = [0; 8];
static mut CURRENTFRAME : [f32; 8] = [0.0; 8];
static mut IS_CRIT : [bool; 8] = [false; 8];
pub static mut META_POWER : [bool; 8] = [false; 8];
static mut GFX_COUNTER : [i32; 8] = [0; 8];
static mut SFX_COUNTER : [i32; 8] = [0; 8];
pub static mut FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_METAKNIGHT )]
fn frame_metaknight(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut smash::app::KineticEnergy;
        let anti_wind = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND) as *mut smash::app::KineticEnergy;
        let no_jostle = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE) as *mut smash::app::KineticEnergy;
        let params = GlideParams::get(fighter);
        let lr = PostureModule::lr(fighter.module_accessor);

        if situation_kind == *SITUATION_KIND_GROUND || situation_kind == *SITUATION_KIND_CLIFF || 
        situation_kind == *SITUATION_KIND_WATER || situation_kind == *SITUATION_KIND_LADDER {
            FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N[ENTRY_ID] = false;
        }
        if [*FIGHTER_STATUS_KIND_DAMAGE, 
            *FIGHTER_STATUS_KIND_DAMAGE_AIR, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY, 
            *FIGHTER_STATUS_KIND_DAMAGE_FALL, 
            *FIGHTER_STATUS_KIND_DAMAGE_SONG, 
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, 
            *FIGHTER_STATUS_KIND_DAMAGE_SONG_FALL, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, 
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD, 
            *FIGHTER_STATUS_KIND_ICE
            ].contains(&status_kind) {
            FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N[ENTRY_ID] = false;
        }
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{x:1.1, y:1.1, z:1.1});
        if META_POWER[ENTRY_ID] == true {
            GFX_COUNTER[ENTRY_ID] += 1;
            SFX_COUNTER[ENTRY_ID] += 1;
            AttackModule::set_power_up(fighter.module_accessor, 1.25);
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 0.5);
            DamageModule::set_reaction_mul(fighter.module_accessor, 0.5);
            if SFX_COUNTER[ENTRY_ID] < 2 {
                macros::PLAY_SE(fighter, Hash40::new("se_metaknight_special_l01"));
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_final01"));
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("vc_metaknight_final03"));
            };
            if SFX_COUNTER[ENTRY_ID] >= 100 {
                SFX_COUNTER[ENTRY_ID] = 2;
            };
            if DamageModule::damage(fighter.module_accessor, 0) < 80.0 {
                SFX_COUNTER[ENTRY_ID] = 0;
            };
            if GFX_COUNTER[ENTRY_ID] >= 6 {
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_hit_purple"), Hash40::new("haver"), &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 0.15, true, 0, 0, 0, 0, 0, true, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.68, /*G*/ 0.87, /*B*/ 2.0);
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_hit_purple"), Hash40::new("havel"), &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 0.15, true, 0, 0, 0, 0, 0, true, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.68, /*G*/ 0.87, /*B*/ 2.0);
                GFX_COUNTER[ENTRY_ID] = 0;
            };
        };
        if DamageModule::damage(fighter.module_accessor, 0) >= 80.0 {
            META_POWER[ENTRY_ID] = true;
        };   
        if DamageModule::damage(fighter.module_accessor, 0) < 80.0 {
            META_POWER[ENTRY_ID] = false;
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
            AttackModule::set_power_up(fighter.module_accessor, 1.0);
        };     
        if [
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_MISS_FOOT
            ].contains(&status_kind) {
            META_POWER[ENTRY_ID] = false;
            AttackModule::set_power_up(fighter.module_accessor, 1.0);
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);

        };
        if sv_information::is_ready_go() == false {
            META_POWER[ENTRY_ID] = false;
            AttackModule::set_power_up(fighter.module_accessor, 1.0);
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
        };
        if [
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_GLIDE_LANDING,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_ESCAPE_AIR,
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_MISS_FOOT,
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_CLIFF_CATCH,
            *FIGHTER_STATUS_KIND_GLIDE_ATTACK,
            *FIGHTER_STATUS_KIND_GLIDE_END
        ].contains(&status_kind) { 
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_start"));
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_loop"));
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_special_h02"));
        };
        if [
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_MISS_FOOT,
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        ].contains(&status_kind) {
            macros::STOP_SE(fighter, Hash40::new("vc_metaknight_final03"));
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            smash::app::lua_bind::KineticEnergy::clear_speed(anti_wind);
            smash::app::lua_bind::KineticEnergy::clear_speed(no_jostle);
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_SPIN {
            fighter.sub_air_check_fall_common();
            smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            smash::app::lua_bind::KineticEnergy::clear_speed(anti_wind);
            smash::app::lua_bind::KineticEnergy::clear_speed(no_jostle);
            WorkModule::enable_transition_term(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH);
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_END {
            smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            smash::app::lua_bind::KineticEnergy::clear_speed(anti_wind);
            if MotionModule::frame(fighter.module_accessor) > 29.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                }
            }
        };
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            smash::app::lua_bind::KineticEnergy::clear_speed(anti_wind);
            smash::app::lua_bind::KineticEnergy::clear_speed(no_jostle);
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH {
            smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            smash::app::lua_bind::KineticEnergy::clear_speed(anti_wind);
            smash::app::lua_bind::KineticEnergy::clear_speed(no_jostle);
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -1.0, 0);
            }
        };
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
            }
            if MotionModule::frame(fighter.module_accessor) > 42.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            }
        };
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            fighter.sub_air_check_fall_common();
            if MotionModule::frame(fighter.module_accessor) > 22.0 {
                WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            }
            if MotionModule::frame(fighter.module_accessor) > 25.0 {
                KineticUtility::reset_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_FREE, Vector2f{x: params.base_speed * lr, y: 0.0}, Vector3f{x: params.base_speed * lr, y: 0.0, z: 0.0});
            }   
            if MotionModule::frame(fighter.module_accessor) >= 29.0 && MotionModule::frame(fighter.module_accessor) < 30.0 {
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_start"));
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_loop"));
            }
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_AIR_HI_START {
            smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            smash::app::lua_bind::KineticEnergy::clear_speed(anti_wind);
        };
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP {
            fighter.sub_air_check_fall_common();
            if MotionModule::frame(fighter.module_accessor) > 22.0 {
                WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            }
            if MotionModule::frame(fighter.module_accessor) > 25.0 {
                KineticUtility::reset_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_FREE, Vector2f{x: params.base_speed * lr, y: 0.0}, Vector3f{x: params.base_speed * lr, y: 0.0, z: 0.0});
            }    
            if MotionModule::frame(fighter.module_accessor) >= 29.0 && MotionModule::frame(fighter.module_accessor) < 30.0 {
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_start"));
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_loop"));
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            smash::app::lua_bind::KineticEnergy::clear_speed(anti_wind);
            if MotionModule::frame(fighter.module_accessor) > 10.0 {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END.into(), true.into());
                }
                else if fighter.global_table[0x1F].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
                    fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK.into(), true.into());
                }
            }
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK {
            smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            smash::app::lua_bind::KineticEnergy::clear_speed(anti_wind);
            fighter.sub_air_check_fall_common();
            fighter.sub_wait_ground_check_common(false.into());
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
            if MotionModule::frame(fighter.module_accessor) > 8.0 {
                HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            }
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                COUNTER[ENTRY_ID] += 1;
                IS_CRIT[ENTRY_ID] = true;
                if COUNTER[ENTRY_ID] < 2 {
                    EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40::new("sys_bg_criticalhit"), smash::phx::Hash40::new("haver"), &Vector3f{x: 0.0, y: 8.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                    CURRENTFRAME[ENTRY_ID] = MotionModule::frame(fighter.module_accessor);
                    SlowModule::set_whole(fighter.module_accessor, 2, 0);
                    macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
                    macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
                    let lr = PostureModule::lr(fighter.module_accessor);
                    macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 4.0,/*no*/ 0.0,/*zoom*/ 2.1,/*yrot*/ 0.0,/*xrot*/ 0.0 * lr);
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
            if MotionModule::frame(fighter.module_accessor) > 60.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                }
            }
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
            if MotionModule::frame(fighter.module_accessor) > 8.0 {
                HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            }
            if MotionModule::frame(fighter.module_accessor) > 32.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                }
            }
        };
    }
}        

pub fn install() {
    smashline::install_agent_frames!(
        frame_metaknight
    );
}