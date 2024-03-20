use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_lucina_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUCINA_STATUS_SPECIAL_LW_FLAG_ACTIVATE_SPECIAL_LW_HIT2) {
        if situation_kind == *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hit_2"), 0.0, 1.0, false, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_hit_2"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW,
    *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX].contains(&status_kind) {
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
    if status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(fighter.module_accessor, -50.0, 0);
        }
    };
}

pub unsafe extern "C" fn frame_lucina_Exec(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_LUCINA_STATUS_SPECIAL_LW_FLAG_ACTIVATE_SPECIAL_LW_HIT2);
        };
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUCINA_STATUS_SPECIAL_LW_FLAG_ACTIVATE_SPECIAL_LW_HIT2);
    }
}

pub fn install() {
    Agent::new("lucina")
    .on_line(Main, frame_lucina_Main)
    .on_line(Exec, frame_lucina_Exec)
    .install();
}