use crate::imports::BuildImports::*;

static mut DEFENCE_BOOST : [bool; 8] = [false; 8];
static mut GFX_COUNTER : [i32; 8] = [0; 8];

unsafe extern "C" fn frame_miifighter_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if DEFENCE_BOOST[ENTRY_ID] == true {
        GFX_COUNTER[ENTRY_ID] += 1;
        DamageModule::set_damage_mul_2nd(fighter.module_accessor, 0.75);
        DamageModule::set_reaction_mul(fighter.module_accessor, 0.75);
        if GFX_COUNTER[ENTRY_ID] >= 20 {
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_aura_light"), Hash40::new("waist"), &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 5.0, true, 0, 0, 0, 0, 0, true, true);
            LAST_EFFECT_SET_COLOR(fighter, /*R*/ 5.3, /*G*/ 0.13, /*B*/ 0.1);
            GFX_COUNTER[ENTRY_ID] = 0;
        };
    };
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
        DEFENCE_BOOST[ENTRY_ID] = true;
    }
    if status_kind == *FIGHTER_STATUS_KIND_DEAD || status_kind == *FIGHTER_STATUS_KIND_MISS_FOOT || 
    sv_information::is_ready_go() == false {
        DEFENCE_BOOST[ENTRY_ID] = false;
        DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
        DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    };
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let CUSTOMIZE_TO = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    if [*FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) 
    || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_1, 
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_1,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_1, 
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1].contains(&CUSTOMIZE_TO) 
    && [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END].contains(&status_kind) 
    || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_2,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_2,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_2,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_2].contains(&CUSTOMIZE_TO)) 
    && [*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_MISS, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_FINISH, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_FINISH_MISS, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI2_END].contains(&status_kind)) 
    || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_3,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_3,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_3,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_3].contains(&CUSTOMIZE_TO)) 
    && [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_TURN, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S3_THROW,].contains(&status_kind) {
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
    Agent::new("miifighter")
    .on_line(Main, frame_miifighter_Main)
    .install();
}