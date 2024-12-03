use crate::imports::BuildImports::*;

pub static mut FIGHTER_KAMEK_STATUS_SPECIAL_N_CHARGE : [i32; 8] = [0; 8];
pub static mut FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_ID_EFFECT : [i32; 8] = [0; 8];
pub static mut ATTTACK_LW4_SPIN_EFFECT : [i32; 8] = [0; 8];
pub static mut N1 : Vector3f = Vector3f { x: 0.0, y: 3.0, z: -15.0 };
pub static mut N2 : Vector3f = Vector3f { x: 0.0, y: 8.0, z: -24.0 };
pub static mut F2 : [u32; 8] = [0; 8];
pub static mut F3 : [u32; 8] = [0; 8];
pub static mut F4 : [u32; 8] = [0; 8];	
pub static mut variance : [f32; 8] = [0.0; 8];
pub static mut SPIN1 :  Vector3f =  Vector3f { x: 0.0, y: 8.25, z: 0.0 };
pub static mut SPIN2 :  Vector3f =  Vector3f { x: 0.0, y: 8.2, z: 0.0 };
pub static mut SPIN3 :  Vector3f =  Vector3f { x: 0.0, y: 8.15, z: 0.0 };
pub static mut SPIN4 :  Vector3f =  Vector3f { x: 0.0, y: 8.3, z: 0.0 };
pub static mut SPIN5 :  Vector3f =  Vector3f { x: 0.0, y: 8.1, z: 0.0 };
pub static mut STAR1 :  Vector3f =  Vector3f { x: 0.0, y: 8.25, z: 0.0 };
pub static mut STAR2 :  Vector3f =  Vector3f { x: 0.0, y: 8.25, z: 11.0 };
pub static mut STAR3 :  Vector3f =  Vector3f { x: 0.0, y: 8.25, z: -11.0 };
pub static mut STAR4 :  Vector3f =  Vector3f { x: 0.0, y: 8.25, z: 6.5 };
pub static mut STAR5 :  Vector3f =  Vector3f { x: 0.0, y: 8.25, z: -6.5 };
pub static mut NO_SPIN :  Vector3f =  Vector3f { x: 0.0, y: 0.0, z: 0.0 };
pub static NONE : Vector3f = Vector3f { x: 0.0, y: 5.0, z: 0.0 };
pub static mut CSTICK_DIRECTION : [f32; 8] = [0.0; 8];

pub unsafe extern "C" fn frame_kamek_Main(fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);    
    let status_kind = StatusModule::status_kind(fighter.module_accessor); 
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let cstick_x = ControlModule::get_attack_air_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
    let cstick_y = ControlModule::get_attack_air_stick_y(fighter.module_accessor);
    let KAMEK = color >= 64 && color <= 71;
    if KAMEK {
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4 {
            if frame > 17.0 && frame < 32.0 {
				if ATTTACK_LW4_SPIN_EFFECT[ENTRY_ID] == 0 {
					AttackLw4_Function(fighter);
				};
				ATTTACK_LW4_SPIN_EFFECT[ENTRY_ID] += 1;
				if ATTTACK_LW4_SPIN_EFFECT[ENTRY_ID] > 4 {
					ATTTACK_LW4_SPIN_EFFECT[ENTRY_ID] = 0;
				};
			} 
            else {
				ATTTACK_LW4_SPIN_EFFECT[ENTRY_ID] = 0;
			};
        };
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("kamek_box"), &Vector3f{x: 0.0, y: 180.0 , z: 0.0 },  MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            }
        }
        if [hash40("attack_air_f")].contains(&MotionModule::motion_kind(fighter.module_accessor)) {
            if frame <= 1.0 {
                if ControlModule::get_attack_air_stick_x(fighter.module_accessor) >= -0.2 && ControlModule::get_attack_air_stick_x(fighter.module_accessor) <= 0.2 && ControlModule::get_attack_air_stick_y(fighter.module_accessor) >= -0.2 && ControlModule::get_attack_air_stick_y(fighter.module_accessor) <= 0.2 {
                    CSTICK_DIRECTION[ENTRY_ID] = 361.0;
                } 
                else if CSTICK_DIRECTION[ENTRY_ID] <= -67.5 {
                    CSTICK_DIRECTION[ENTRY_ID] *= -1.0;
                };
                if CSTICK_DIRECTION[ENTRY_ID] >= -67.5 && CSTICK_DIRECTION[ENTRY_ID] < -20.0 && cstick_x > 0.0 { //3 (Angled Down)
                    param_config::update_float(-*WEAPON_KIND_NESS_PK_FIRE, vec![64,65,66,67,68,69,70,71], (hash40("param_pkfire"),hash40("angle_air")), -20.0);
                }
                else if CSTICK_DIRECTION[ENTRY_ID] >= -20.0 && CSTICK_DIRECTION[ENTRY_ID] <= 20.0 && cstick_x > 0.0 { //6 (No Angle)
                    param_config::update_float(-*WEAPON_KIND_NESS_PK_FIRE, vec![64,65,66,67,68,69,70,71], (hash40("param_pkfire"),hash40("angle_air")), 0.0);
                }
                else if CSTICK_DIRECTION[ENTRY_ID] > 20.0 && CSTICK_DIRECTION[ENTRY_ID] <= 67.5 && cstick_x > 0.0 { //9 (Angled Up)
                    param_config::update_float(-*WEAPON_KIND_NESS_PK_FIRE, vec![64,65,66,67,68,69,70,71], (hash40("param_pkfire"),hash40("angle_air")), 20.0);
                };
            };
        };
        if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD].contains(&status_kind) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
            if !fighter.is_in_hitlag() && !StatusModule::is_changing(fighter.module_accessor) && situation_kind == *SITUATION_KIND_AIR {
                fighter.sub_air_check_dive();
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
                    if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_MOTION_AIR || 
                    KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE {
                        fighter.clear_lua_stack();
                        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                        let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
                        fighter.clear_lua_stack();
                        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                        fighter.clear_lua_stack();
                        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                        sv_kinetic_energy::enable(fighter.lua_state_agent);
                        KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                    }
                }
            }
        }
        if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) == *FIGHTER_PAD_CMD_CAT1_SPECIAL_HI {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
        }
        if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END, 
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD].contains(&status_kind) == true {
            AbsorberModule::clear_all(fighter.module_accessor);
        };
        if status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -75.0, 0);
            }
        };
    }
}

unsafe extern "C" fn frame_kamek_beam_Main(weapon : &mut L2CFighterBase) {
    let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let module_accessor = sv_battle_object::module_accessor(otarget_id);
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let ENTRY_ID = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(weapon.module_accessor) as i32;
    let charge_time = WorkModule::get_float(weapon.module_accessor, FIGHTER_KAMEK_STATUS_SPECIAL_N_HOLD_WORK_FLOAT_TIME);
    if frame < 15 {
        if frame >= 2 {
            if frame % 4 == 0 {
                variance[ENTRY_ID] = 8.0;
            } 
            else if (frame + 1) % 2 == 0 {
                variance[ENTRY_ID] = 4.0;
            } 
            else {
                variance[ENTRY_ID] = -2.0;
            };
        } 
        else {
            variance[ENTRY_ID] = 0.0;
        };
        if frame % 3 == 0 {
            let f1: u32 = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_fireflower_shot"), Hash40::new("top"), &NONE, &NONE, if charge_time < 42.0 {0.5} else if charge_time >= 42.0 && charge_time <= 64.0 {1.0} else {1.8}, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(weapon.module_accessor, f1, 255.0/255.0, 27.0/255.0, 172.0/255.0);
            EffectModule::set_alpha(weapon.module_accessor, f1, 0.7);
            EffectModule::set_rate(weapon.module_accessor, f1, 1.5);
            if frame >= 2 {
                let f2: u32 = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_fireflower_shot"), Hash40::new("top"), &N1, &NONE, if charge_time < 42.0 {0.2} else if charge_time >= 42.0 && charge_time <= 64.0 {0.4} else {0.7}, true, 0, 0, 0, 0, 0, true, true) as u32;
                EffectModule::set_rgb(weapon.module_accessor, f2, 255.0/255.0, 27.0/255.0, 172.0/255.0);
                EffectModule::set_alpha(weapon.module_accessor, f2, 0.7);
                EffectModule::set_rate(weapon.module_accessor, f2, 1.5);
            };
            if frame >= 5 {
                let f3: u32 = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_fireflower_shot"), Hash40::new("top"), &N2, &NONE, if charge_time < 42.0 {0.175} else if charge_time >= 42.0 && charge_time <= 64.0 {0.35} else {0.6}, true, 0, 0, 0, 0, 0, true, true) as u32;
                EffectModule::set_rgb(weapon.module_accessor, f3, 255.0/255.0, 27.0/255.0, 172.0/255.0);
                EffectModule::set_alpha(weapon.module_accessor, f3, 0.7);
                EffectModule::set_rate(weapon.module_accessor, f3, 1.5);
            };
        };
        if frame % 5 == 0 {
            let f2: u32 = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_smash_flash"), Hash40::new("top"), &NONE, &NONE, if charge_time < 42.0 {0.4} else if charge_time >= 42.0 && charge_time <= 64.0 {0.8} else {1.4}, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(weapon.module_accessor, f2, 255.0/255.0, 0.0/255.0, 60.0/255.0);
        };
        if frame % 20 == 0 {
            EffectModule::kill_kind(weapon.module_accessor, Hash40::new("sys_sscope_bullet"), false, true);
            F2[ENTRY_ID] = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_sscope_bullet"), Hash40::new("top"), &NONE, &NONE, if charge_time < 42.0 {1.05} else if charge_time >= 42.0 && charge_time <= 64.0 {2.1} else {3.9}, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(weapon.module_accessor, F2[ENTRY_ID], 255.0/255.0, 27.0/255.0, 172.0/255.0);
        };
        if frame == 2 {
            F3[ENTRY_ID] = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_sscope_bullet"), Hash40::new("top"), &N1, &NONE, if charge_time < 42.0 {0.42} else if charge_time >= 42.0 && charge_time <= 64.0 {0.84} else {1.38}, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(weapon.module_accessor, F3[ENTRY_ID], 255.0/255.0, 27.0/255.0, 172.0/255.0);
            EffectModule::set_alpha(weapon.module_accessor, F3[ENTRY_ID], 0.65);
        };
        if frame == 5 {
            F4[ENTRY_ID] = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_sscope_bullet"), Hash40::new("top"), &N2, &NONE, if charge_time < 42.0 {0.3675} else if charge_time >= 42.0 && charge_time <= 64.0 {0.735} else {1.2}, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(weapon.module_accessor, F4[ENTRY_ID], 255.0/255.0, 27.0/255.0, 172.0/255.0);
            EffectModule::set_alpha(weapon.module_accessor, F4[ENTRY_ID], 0.5);
        };
        if frame >= 2 {
            let n1 =  Vector3f { x: 0.0, y: 2.0 + variance[ENTRY_ID], z: -15.0 };
            EffectModule::set_pos(module_accessor, F3[ENTRY_ID], &n1);
        };
        if frame >= 5 {
            let n2 =  Vector3f { x: 0.0, y: 8.0 - variance[ENTRY_ID], z: -24.0 };
            EffectModule::set_pos(module_accessor, F4[ENTRY_ID], &n2);
        };
    } 
    else {
        EffectModule::kill_kind(weapon.module_accessor, Hash40::new("sys_sscope_bullet"), false, true);
        macros::EFFECT_OFF_KIND(weapon, Hash40::new("sys_sscope_bullet"), false, true);
        EffectModule::kill(weapon.module_accessor, F2[ENTRY_ID], false, true);
    };
}

unsafe fn AttackLw4_Function(fighter: &mut L2CFighterCommon) {
    let handbg1: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &SPIN2, &NO_SPIN, 1.45, true, 0, 0, 0, 0, 0, true, true) as u32;
    let handbg2: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &SPIN3, &NO_SPIN, 1.45, true, 0, 0, 0, 0, 0, true, true) as u32;
    let handbg3: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &SPIN4, &NO_SPIN, 1.45, true, 0, 0, 0, 0, 0, true, true) as u32;
    let handbg4: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &SPIN5, &NO_SPIN, 1.45, true, 0, 0, 0, 0, 0, true, true) as u32;
    let hand1: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &SPIN1, &NO_SPIN, 1.45, true, 0, 0, 0, 0, 0, true, true) as u32;
    let star1: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &STAR1, &NO_SPIN, 0.65, true, 0, 0, 0, 0, 0, true, true) as u32;
    let star2: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &STAR2, &NO_SPIN, 0.65, true, 0, 0, 0, 0, 0, true, true) as u32;
    let star3: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &STAR3, &NO_SPIN, 0.65, true, 0, 0, 0, 0, 0, true, true) as u32;
    let star4: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("handl"), &NO_SPIN, &NO_SPIN, 0.85, true, 0, 0, 0, 0, 0, true, true) as u32;
    let star5: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("handr"), &NO_SPIN, &NO_SPIN, 0.85, true, 0, 0, 0, 0, 0, true, true) as u32;
    let star6: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &STAR4, &NO_SPIN, 0.65, true, 0, 0, 0, 0, 0, true, true) as u32;
    let star7: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &STAR5, &NO_SPIN, 0.65, true, 0, 0, 0, 0, 0, true, true) as u32;
    EffectModule::set_rgb(fighter.module_accessor, hand1, 0.045, 0.345, 2.05);
    EffectModule::set_alpha(fighter.module_accessor, hand1, 0.3);
    EffectModule::set_rgb(fighter.module_accessor, handbg1, 0.045, 0.345, 2.05);
    EffectModule::set_alpha(fighter.module_accessor, handbg1, 0.25);
    EffectModule::set_rgb(fighter.module_accessor, handbg2, 0.045, 0.045, 2.05);
    EffectModule::set_alpha(fighter.module_accessor, handbg2, 0.25);
    EffectModule::set_rgb(fighter.module_accessor, handbg3, 0.045, 0.345, 2.05);
    EffectModule::set_alpha(fighter.module_accessor, handbg3, 0.25);
    EffectModule::set_rgb(fighter.module_accessor, handbg4, 0.045, 0.345, 2.05);
    EffectModule::set_alpha(fighter.module_accessor, handbg4, 0.25);
    EffectModule::set_rgb(fighter.module_accessor, star1, 1.65, 1.95, 1.85);
    EffectModule::set_rgb(fighter.module_accessor, star2, 1.65, 1.95, 1.85);
    EffectModule::set_rgb(fighter.module_accessor, star3, 1.65, 1.95, 1.85);
    EffectModule::set_rgb(fighter.module_accessor, star4, 1.65, 1.95, 1.85);
    EffectModule::set_rgb(fighter.module_accessor, star5, 1.65, 1.95, 1.85);
    EffectModule::set_rgb(fighter.module_accessor, star6, 1.65, 1.95, 1.85);
    EffectModule::set_rgb(fighter.module_accessor, star7, 1.65, 1.95, 1.85);
    EffectModule::set_alpha(fighter.module_accessor, star1, 0.6);
    EffectModule::set_alpha(fighter.module_accessor, star2, 0.6);
    EffectModule::set_alpha(fighter.module_accessor, star3, 0.6);
    EffectModule::set_alpha(fighter.module_accessor, star4, 0.6);
    EffectModule::set_alpha(fighter.module_accessor, star5, 0.6);
    EffectModule::set_alpha(fighter.module_accessor, star6, 0.6);
    EffectModule::set_alpha(fighter.module_accessor, star7, 0.6);
}

pub fn install() {
    Agent::new("ness")
    .on_line(Main, frame_kamek_Main)
    .install();

    Agent::new("ness_pinkmagic")
    .on_line(Main, frame_kamek_beam_Main)
    .install();
}