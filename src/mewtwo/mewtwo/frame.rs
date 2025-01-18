use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_mewtwo_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_FLOAT {
        let effect_counter = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
        let sound_counter = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SOUND_COUNTER);
        WorkModule::add_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
        WorkModule::add_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_SOUND_COUNTER);
        if effect_counter >= 4 {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), -1, 0, 0, 0, 0, 0, 0.3, true, *EF_FLIP_YZ);
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 1, 0, 0, 0, 0, 0, 0.3, true, *EF_FLIP_YZ);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
        }
        if sound_counter == 1 {
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_mewtwo_appeal_l01"), true, false, false, false, enSEType(0));
        }
    }
    else {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SOUND_COUNTER);
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_MAX, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_SHOOT,
    *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_CANCEL, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3].contains(&status_kind) {
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
    Agent::new("mewtwo")
    .on_line(Main, frame_mewtwo_Main)
    .install();
}