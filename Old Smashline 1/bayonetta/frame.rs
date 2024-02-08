use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_BAYONETTA )]
pub fn frame_bayonetta(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
        if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE {
            if situation_kind == *SITUATION_KIND_AIR {
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE != 0  {
                    ControlModule::reset_trigger(fighter.module_accessor);
                    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                    ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
                }
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.04);
                sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.005);
                sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.4, 0.0);
            } 
            else {
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                if fighter.global_table[STICK_Y].get_f32() <= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"))
                && fighter.global_table[FLICK_Y].get_i32() < WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("pass_flick_y"))
                && GroundModule::is_passable_ground(fighter.module_accessor) {
                    GroundModule::pass_floor(fighter.module_accessor);
                    ControlModule::clear_command(fighter.module_accessor, false);
                }
            }
        }
        if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_FIRE,
        *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_END, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_HIT,
        *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_WALL_END].contains(&status_kind) {
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
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME_SUCCESS) {
                DamageModule::heal(fighter.module_accessor, -0.8, 0);
            }
        };
        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME_SUCCESS);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_bayonetta
    );
}