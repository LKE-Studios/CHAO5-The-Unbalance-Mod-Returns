use crate::imports::BuildImports::*;

static mut SWORD_MUL : [f32; 8] = [1.0; 8];

pub unsafe extern "C" fn frame_miiswordsman_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let joint_scale = Vector3f { x: SWORD_MUL[ENTRY_ID], y: SWORD_MUL[ENTRY_ID], z: SWORD_MUL[ENTRY_ID]};
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &joint_scale);
    AttackModule::set_attack_scale(fighter.module_accessor, 1.0 * SWORD_MUL[ENTRY_ID], true);
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        SWORD_MUL[ENTRY_ID] += 0.05;
    }
    if !sv_information::is_ready_go() 
    || [*FIGHTER_STATUS_KIND_MISS_FOOT, 
    *FIGHTER_STATUS_KIND_DEAD, 
    *FIGHTER_STATUS_KIND_WIN, 
    *FIGHTER_STATUS_KIND_LOSE].contains(&status_kind) {
        SWORD_MUL[ENTRY_ID] = 1.0;
    }
    let CUSTOMIZE_TO = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) 
    || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_1, 
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_1,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_1, 
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1].contains(&CUSTOMIZE_TO) 
    && [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT].contains(&status_kind) 
    || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_2,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_2,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_2,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_2].contains(&CUSTOMIZE_TO)) 
    && [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK,
        *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH_END, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_BOUND].contains(&status_kind)) 
    || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_3,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_3,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_3,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_3].contains(&CUSTOMIZE_TO)) 
    && [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_END,
        *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_LOOP, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_END_MAX].contains(&status_kind) 
    || (motion_kind == hash40("special_air_hi3") && frame > 49.0) {
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

pub unsafe extern "C" fn frame_miiswordsman_Exec(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let joint_scale = Vector3f { x: SWORD_MUL[ENTRY_ID], y: SWORD_MUL[ENTRY_ID], z: SWORD_MUL[ENTRY_ID]};
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &joint_scale);
    AttackModule::set_attack_scale(fighter.module_accessor, 1.0 * SWORD_MUL[ENTRY_ID], true);
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        SWORD_MUL[ENTRY_ID] += 0.05;
    }
    if !sv_information::is_ready_go() 
    || [*FIGHTER_STATUS_KIND_MISS_FOOT, 
    *FIGHTER_STATUS_KIND_DEAD, 
    *FIGHTER_STATUS_KIND_WIN, 
    *FIGHTER_STATUS_KIND_LOSE].contains(&status_kind) {
        SWORD_MUL[ENTRY_ID] = 1.0;
    }
}

pub fn install() {
    Agent::new("miiswordsman")
    .on_line(Main, frame_miiswordsman_Main)
    .on_line(Exec, frame_miiswordsman_Exec)
    .install();
}