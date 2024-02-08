use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
pub fn frame_kirby(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SWALLOW {
            DamageModule::heal(fighter.module_accessor, -15.0, 0);
        }
        if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2 && frame <= 2.0 {
            fighter.set_situation(SITUATION_KIND_AIR.into());
        }
        if status_kind == *FIGHTER_KIRBY_STATUS_KIND_GAMEWATCH_SPECIAL_N {
            if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING, false);
            }
            if StatusModule::is_changing(fighter.module_accessor) {
                KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: 0.9, y: 1.0, z: 1.0}, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            if situation_kind == *SITUATION_KIND_AIR {
                if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                }
            }
        }
        if [*FIGHTER_KIRBY_STATUS_KIND_FOX_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_FALCO_SPECIAL_N].contains(&status_kind) {
            if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING, false);
            }
            if situation_kind == *SITUATION_KIND_AIR {
                if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                }
            }
        }
        if status_kind == *FIGHTER_KIRBY_STATUS_KIND_WOLF_SPECIAL_N {
            fighter.sub_air_check_fall_common();
            if frame > 16.0 {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
            }
            if situation_kind == *SITUATION_KIND_AIR {
                if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                }
            }
        }
        if [*FIGHTER_KIRBY_STATUS_KIND_METAKNIGHT_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_METAKNIGHT_SPECIAL_N_SPIN,
        *FIGHTER_KIRBY_STATUS_KIND_METAKNIGHT_SPECIAL_N_END].contains(&status_kind) {
            KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
            KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
        }
        
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_kirby
    );
}