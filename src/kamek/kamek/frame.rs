use crate::imports::BuildImports::*;

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

pub unsafe extern "C" fn frame_kamek_Main(fighter: &mut L2CFighterCommon) {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);    
    let status_kind = StatusModule::status_kind(fighter.module_accessor); 
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let KAMEK = color >= 96 && color <= 103;
    if KAMEK {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            fighter.change_status(FIGHTER_KAMEK_STATUS_KIND_SPECIAL_N_START.into(), true.into());
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            fighter.change_status(FIGHTER_KAMEK_STATUS_KIND_SPECIAL_S.into(), true.into());
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            fighter.change_status(FIGHTER_KAMEK_STATUS_KIND_SPECIAL_HI_START.into(), true.into());
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            fighter.change_status(FIGHTER_KAMEK_STATUS_KIND_SPECIAL_LW.into(), true.into());
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4 {
            let effect_counter = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KAMEK_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
            if frame > 17.0 && frame < 32.0 {
				if effect_counter == 0 {
					AttackLw4_Function(fighter);
				}
                WorkModule::add_int(fighter.module_accessor, 1, *FIGHTER_KAMEK_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
				if effect_counter > 4 {
                    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KAMEK_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
				}
			} 
            else {
				WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KAMEK_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
			}
        };
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            if lr == -1.0 {
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("kamek_box"), &Vector3f{x: 0.0, y: 180.0 , z: 0.0 },  MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            }
        }
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let cstick_x = ControlModule::get_attack_air_stick_x(fighter.module_accessor);
        let cstick_y = ControlModule::get_attack_air_stick_y(fighter.module_accessor);
        let mut cstick_direction = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CSTICK_DIRECTION);
        cstick_direction = ControlModule::get_attack_air_stick_dir(fighter.module_accessor) * (180.0 / PI);
        if motion_kind == hash40("attack_air_f") {
            if frame <= 1.0 {
                if cstick_x >= -0.2 && cstick_x <= 0.2 && cstick_y >= -0.2 && cstick_y <= 0.2 {
                    WorkModule::set_float(fighter.module_accessor, 361.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CSTICK_DIRECTION);
                } 
                else if cstick_direction <= -67.5 {
                    WorkModule::mul_float(fighter.module_accessor, -1.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CSTICK_DIRECTION);
                }
                if cstick_direction >= -67.5 && cstick_direction < -20.0 && cstick_x * lr > 0.0 { //3 (Angled Down)
                    param_config::update_float(-*WEAPON_KIND_NESS_PK_FIRE, vec![96,97,98,99,100,101,102,103], (hash40("param_pkfire"),hash40("angle_air")), -20.0);
                }
                else if cstick_direction >= -20.0 && cstick_direction <= 20.0 && cstick_x * lr > 0.0 { //6 (No Angle)
                    param_config::update_float(-*WEAPON_KIND_NESS_PK_FIRE, vec![96,97,98,99,100,101,102,103], (hash40("param_pkfire"),hash40("angle_air")), 0.0);
                }
                else if cstick_direction > 20.0 && cstick_direction <= 67.5 && cstick_x * lr > 0.0 { //9 (Angled Up)
                    param_config::update_float(-*WEAPON_KIND_NESS_PK_FIRE, vec![96,97,98,99,100,101,102,103], (hash40("param_pkfire"),hash40("angle_air")), 20.0);
                }
            }
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
    }
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
}