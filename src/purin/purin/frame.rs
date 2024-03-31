use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_purin_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW,
    *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HIT_END, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_END].contains(&status_kind) {
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

pub unsafe extern "C" fn frame_purin_Exec(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if [*FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD_MAX].contains(&status_kind) {
        fighter.sub_wait_ground_check_common(false.into());
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
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), true.into());
            }
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), true.into());
            }
        }
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_TURN,
        *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL_AIR].contains(&status_kind) {
        fighter.sub_wait_ground_check_common(false.into());
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
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                fighter.change_status(FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_END.into(), true.into());
            }
        }
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), true.into());
            }
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), true.into());
            }
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                fighter.change_status(FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_END.into(), true.into());
            }
        }
    };
}

pub fn install() {
    Agent::new("purin")
    .on_line(Main, frame_purin_Main)
    .on_line(Exec, frame_purin_Exec)
    .install();
}