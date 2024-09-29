use crate::imports::BuildImports::*;

static mut ATTACK_LW_EFFECT : [i32; 8] = [0; 8];

unsafe extern "C" fn frame_krystal_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(fighter.module_accessor);
    let KRYSTAL = color >= 64 && color <= 71; 
    if KRYSTAL {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_ENABLE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_INPUT);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_INPUT_BACK);
        if ModelModule::scale(fighter.module_accessor) == WorkModule::get_param_float(fighter.module_accessor, hash40("scale"), 0) {
            ModelModule::set_scale(fighter.module_accessor, 0.89);
            AttackModule::set_attack_scale(fighter.module_accessor, 0.89, true);
            GrabModule::set_size_mul(fighter.module_accessor, 0.89);
        };
        if ![*FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_LW4_IS_CHARGED);
        };
        if ![*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END].contains(&status_kind) {
            STOP_SE(fighter, Hash40::new("se_pitb_special_h02"));
        }
        if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_HOLD {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -4.0, 0);
                PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
                
            }
        }
        if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
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
}

pub fn install() {
    Agent::new("pitb")
    .on_line(Main, frame_krystal_Main)
    .install();
}
