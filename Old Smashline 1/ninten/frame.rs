use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_NESS )]
pub fn ninten_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);    
        let status_kind = StatusModule::status_kind(fighter.module_accessor); 
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let NINTEN = color >= 120 && color <= 127;
        if NINTEN {
            if status_kind != *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD {
                for i in 0..12 {
                    let hit_status = if i == 11 {*HIT_STATUS_OFF} else {*HIT_STATUS_NORMAL};
                }
            }
            else {
                for i in 0..12 {
                    let hit_status = if i == 11 {*HIT_STATUS_NORMAL} else {*HIT_STATUS_OFF};
                }
            };
            if ![*FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, 
            *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) {
                STOP_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
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
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        ninten_frame,
    );
}