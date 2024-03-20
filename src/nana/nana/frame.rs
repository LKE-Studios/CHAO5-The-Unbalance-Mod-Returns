use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_nana_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if [hash40("special_air_lw"), hash40("special_air_lw_nana")].contains(&motion_kind) {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) 
        && frame < 55.0 {
            MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 54.0, true, true, false);
            AttackModule::clear_all(fighter.module_accessor);
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                SET_SPEED_EX(fighter, 0, 1.7, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if fighter_kind == *FIGHTER_KIND_POPO {
                    PLAY_SE(fighter, Hash40::new("vc_popo_attack04"));
                } 
                else if fighter_kind == *FIGHTER_KIND_NANA {
                    PLAY_SE(fighter, Hash40::new("vc_nana_attack04"));
                } 
            } 
            else if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                SET_SPEED_EX(fighter, 0, 0.75, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if fighter_kind == *FIGHTER_KIND_POPO {
                    PLAY_SE(fighter, Hash40::new("vc_popo_attack04"));
                } 
                else if fighter_kind == *FIGHTER_KIND_NANA {
                    PLAY_SE(fighter, Hash40::new("vc_nana_attack04"));
                } 
            }
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if [hash40("special_air_s2")].contains(&motion_kind) && situation_kind == *SITUATION_KIND_AIR {
            if frame > 82.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
        if [hash40("special_air_s1")].contains(&motion_kind) && situation_kind == *SITUATION_KIND_AIR {
            if frame > 94.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        } 
    }
    if status_kind == *FIGHTER_POPO_STATUS_KIND_SPECIAL_S_PARTNER {
        if [hash40("special_air_s2_nana")].contains(&motion_kind) && situation_kind == *SITUATION_KIND_AIR {
            if frame > 82.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        } 
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_FAIL,
    *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP_PARTNER].contains(&status_kind) {
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

pub fn install() {
    Agent::new("nana")
    .on_line(Main, frame_nana_Main)
    .install();
}