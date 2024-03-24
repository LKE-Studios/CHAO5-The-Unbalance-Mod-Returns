use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_murabito_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_POCKET, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH,
    *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_TAKE_OUT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_JUMP, 
    *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH,
    *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_AIR, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT_FAIL, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP].contains(&status_kind) {
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
    if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_AIR, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_DASH_B, 
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_DASH_F, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP_SQUAT,  *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_LANDING,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WAIT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_B,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_BRAKE_B, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_BRAKE_F,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_F].contains(&status_kind) {
        if frame > 12.0 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            if situation_kind == *SITUATION_KIND_AIR {
                WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            }
            if situation_kind == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
            }
        }
    }
}

pub unsafe extern "C" fn frame_murabito_Exec(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);   
    if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_DEFOREST {
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("havel"), &Vector3f{x: 2.0, y: 2.0, z: 2.0});
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{x: 2.0, y: 2.0, z: 2.0});
    }
}

pub fn install() {
    Agent::new("murabito")
    .on_line(Main, frame_murabito_Main)
    .on_line(Exec, frame_murabito_Exec)
    .install();
}