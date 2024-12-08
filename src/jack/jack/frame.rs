use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_jack_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        DamageModule::set_reaction_mul(fighter.module_accessor, 0.6);
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_THROW,
    *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_END,
    *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_ATTACK, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_ENDURE, 
    *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW2_COUNTER, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW2_REFLECTOR].contains(&status_kind) {
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
    if status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_RUSH {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        }
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
        }
    };
    let frame = MotionModule::frame(fighter.module_accessor);
    if motion_kind == hash40("special_air_hi_throw") {
        if frame < 37.0 {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        if frame >= 37.0 {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    if status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW2_COUNTER {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(fighter.module_accessor, -66.0, 0);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_recovery"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
            PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
        }
    };
}

pub unsafe extern "C" fn frame_jack_Exec(fighter : &mut L2CFighterCommon) {
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("knife"), &Vector3f{x: 1.01, y: 1.25, z: 1.01});
}

pub fn install() {
    Agent::new("jack")
    .on_line(Main, frame_jack_Main)
    .on_line(Exec, frame_jack_Exec)
    .install();
}