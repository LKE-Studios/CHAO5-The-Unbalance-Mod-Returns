use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
//use crate::utils::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::{sv_information};

/*static mut ANGLE : [f32; 8] = [0.0; 8];
static ANGLE_MAX : f32 = 80.0; //Max Ascent Angle for Glide (degrees)
static ANGLE_LOW_MAX : f32 = -80.0; //Max Descent Angle for Glide (degrees)
static mut MOMENTUM : [f32; 8] = [0.0; 8];
static THRESHOLD_MAX : f32 = -25.0;
static STICK_ANGLE_MUL : f32 = 7.0;*/ //Controls how much Meta Knight's body rotates according to the control stick (higher value = higher sensitivity)
static mut HOLD_TIME : [f32; 8] = [0.0; 8]; //Allows Meta Knight to enter the glide state when holding the jump button
static mut COUNTER: [i32; 8] = [0; 8];
static mut CURRENTFRAME: [f32; 8] = [0.0; 8];
static mut IS_CRIT: [bool; 8] = [false; 8];
static mut META_POWER : [bool; 8] = [false; 8];
static mut GFX_COUNTER: [i32; 8] = [0; 8];
static mut SFX_COUNTER: [i32; 8] = [0; 8];

#[fighter_frame( agent = FIGHTER_KIND_METAKNIGHT )]
fn metaknight_opff(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        //let situation_kind = StatusModule::situation_kind(boma);
        let kind = smash::app::utility::get_kind(boma);
        let energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut smash::app::KineticEnergy;
        let anti_wind = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND) as *mut smash::app::KineticEnergy;
        let no_jostle = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE) as *mut smash::app::KineticEnergy;
        if kind == *FIGHTER_KIND_METAKNIGHT {
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{x:1.14, y:1.14, z:1.14});
        };
        if META_POWER[ENTRY_ID] == true {
            GFX_COUNTER[ENTRY_ID] += 1;
            SFX_COUNTER[ENTRY_ID] += 1;
            AttackModule::set_power_up(fighter.module_accessor, 1.2);
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 0.5);
            DamageModule::set_reaction_mul(fighter.module_accessor, 0.5);
            if MotionModule::frame(fighter.module_accessor) >= 0.0 && MotionModule::frame(fighter.module_accessor) < 1.0 {  
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 5.0, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.6, /*G*/ 0.57, /*B*/ 2.0);
            }
            if SFX_COUNTER[ENTRY_ID] < 2 {
                macros::PLAY_SE(fighter, Hash40::new("se_metaknight_special_l01"));
                macros::PLAY_SE(fighter, Hash40::new("se_metaknight_final_01"));
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("vc_metaknight_final03"));
            };
            if SFX_COUNTER[ENTRY_ID] >= 100 {
                SFX_COUNTER[ENTRY_ID] = 2;
            };
            if DamageModule::damage(fighter.module_accessor, 0) < 100.0 {
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
        if DamageModule::damage(fighter.module_accessor, 0) >= 100.0 {
            META_POWER[ENTRY_ID] = true;
        };   
        if DamageModule::damage(fighter.module_accessor, 0) < 100.0 {
            META_POWER[ENTRY_ID] = false;
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
            AttackModule::set_power_up(fighter.module_accessor, 1.0);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        };     
        if status_kind == *FIGHTER_STATUS_KIND_DEAD {
            META_POWER[ENTRY_ID] = false;
            AttackModule::set_power_up(fighter.module_accessor, 1.0);
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);

        };
        if sv_information::is_ready_go() == false {
            META_POWER[ENTRY_ID] = false;
            AttackModule::set_power_up(fighter.module_accessor, 1.0);
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        };
        if status_kind == *FIGHTER_STATUS_KIND_MISS_FOOT {
            META_POWER[ENTRY_ID] = false;
            AttackModule::set_power_up(fighter.module_accessor, 1.0);
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        };
        if ![*FIGHTER_STATUS_KIND_GLIDE_START, *FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP){
                HOLD_TIME[ENTRY_ID] +=1.0;
            }
            if HOLD_TIME[ENTRY_ID] == 20.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_START.into(), true.into());
            }
        }
        else {
            HOLD_TIME[ENTRY_ID] = 0.0;
        }
        if [*FIGHTER_STATUS_KIND_JUMP].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_WAIT].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_GLIDE].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_SPIN].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_END].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if status_kind == *FIGHTER_STATUS_KIND_LANDING { 
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_special_h01"));
        };
        if status_kind == *FIGHTER_STATUS_KIND_LANDING_LIGHT { 
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_special_h01"));
        };
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_LANDING { 
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_special_h01"));
        };
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_special_h01"));
        };
        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_special_h01"));
        };
        if status_kind == *FIGHTER_STATUS_KIND_DEAD {
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_special_h01"));
        }; 
        if status_kind == *FIGHTER_STATUS_KIND_MISS_FOOT {
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_special_h01"));
        }; 
        if status_kind == *FIGHTER_STATUS_KIND_DAMAGE {
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_special_h01"));
        };
        if status_kind == *FIGHTER_STATUS_KIND_CLIFF_CATCH {
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_special_h01"));
        };
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_START {
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        }
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_ATTACK {
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_special_h01"));
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        }
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_END {
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_special_h01"));
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        };
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
            /*if MotionModule::frame(fighter.module_accessor) > 29.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                if is_grounded(fighter.module_accessor) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                }
            }*/
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
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -1.0, 0);
            }
        };
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            fighter.sub_air_check_fall_common();
            smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            smash::app::lua_bind::KineticEnergy::clear_speed(anti_wind);
            if MotionModule::frame(fighter.module_accessor) >= 1.0 && MotionModule::frame(fighter.module_accessor) < 2.0 { //SFX Stuff added to prevent them from looping along with the animation
                macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
            }
            if MotionModule::frame(fighter.module_accessor) >= 5.0 && MotionModule::frame(fighter.module_accessor) < 6.0 {
                macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
            /*if MotionModule::frame(fighter.module_accessor) >= 6.0 && MotionModule::frame(fighter.module_accessor) < 7.0 {
                macros::PLAY_STATUS(fighter, Hash40::new("se_metaknight_special_h02"));
            }*/
            if MotionModule::frame(fighter.module_accessor) >= 8.0 && MotionModule::frame(fighter.module_accessor) < 9.0 {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_shuttleloop1"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.2, true);
            }          
            if MotionModule::frame(fighter.module_accessor) > 22.0 {
                WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            }
            if MotionModule::frame(fighter.module_accessor) > 25.0 {
                macros::SET_SPEED_EX(fighter, 1.8, -0.42, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            if MotionModule::frame(fighter.module_accessor) >= 29.0 && MotionModule::frame(fighter.module_accessor) < 30.0 {
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_h01"));
            }
            if MotionModule::frame(fighter.module_accessor) >= 31.0 && MotionModule::frame(fighter.module_accessor) < 32.0 {
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
            }
            if MotionModule::frame(fighter.module_accessor) > 32.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE.into(), true.into());
            };
            if [*FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
            if [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
            if [*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
            if [*FIGHTER_STATUS_KIND_JUMP].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
            if [*FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_AIR_HI_START {
            smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            smash::app::lua_bind::KineticEnergy::clear_speed(anti_wind);
        };
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP {
            fighter.sub_air_check_fall_common();
            smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            smash::app::lua_bind::KineticEnergy::clear_speed(anti_wind);
            if MotionModule::frame(fighter.module_accessor) >= 1.0 && MotionModule::frame(fighter.module_accessor) < 2.0 {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
            }
            if MotionModule::frame(fighter.module_accessor) >= 2.0 && MotionModule::frame(fighter.module_accessor) < 3.0 {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_shuttleloop1"), Hash40::new("top"), 0, -5, 2.5, 4, 0, 0, 1, true);
                EffectModule::set_disable_render_offset_last(fighter.module_accessor);
                macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 20, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
            if MotionModule::frame(fighter.module_accessor) > 22.0 {
                WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            }
            if MotionModule::frame(fighter.module_accessor) > 25.0 {
                macros::SET_SPEED_EX(fighter, 1.8, -0.42, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }  
            if MotionModule::frame(fighter.module_accessor) >= 29.0 && MotionModule::frame(fighter.module_accessor) < 30.0 {
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_h01"));
            }
            if MotionModule::frame(fighter.module_accessor) >= 31.0 && MotionModule::frame(fighter.module_accessor) < 32.0 {
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
            }
            if MotionModule::frame(fighter.module_accessor) > 32.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE.into(), true.into());
            };
            if [*FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
            if [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
            if [*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
            if [*FIGHTER_STATUS_KIND_JUMP].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
            if [*FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
                HOLD_TIME[ENTRY_ID] = 1.0;
            };
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
                if MotionModule::frame(fighter.module_accessor) > 10.0 {
                    CancelModule::enable_cancel(fighter.module_accessor);
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
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
        metaknight_opff
    );
}