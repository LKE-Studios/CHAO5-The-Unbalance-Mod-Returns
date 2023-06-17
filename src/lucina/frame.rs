use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Hash40;
use smash::hash40;

static mut FIGHTER_LUCINA_STATUS_KIND_SPECIAL_LW_HIT2 : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_LUCINA )]
pub fn frame_lucina(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        if FIGHTER_LUCINA_STATUS_KIND_SPECIAL_LW_HIT2[ENTRY_ID] == true {
            if situation_kind == *SITUATION_KIND_GROUND {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hit_2"), 0.0, 1.0, false, 0.0, false, false);
            }
            if situation_kind == *SITUATION_KIND_AIR {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_hit_2"), 0.0, 1.0, false, 0.0, false, false);
            }
        };
        if status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -50.0, 0);
            }
        };
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                FIGHTER_LUCINA_STATUS_KIND_SPECIAL_LW_HIT2[ENTRY_ID] = true;
            };
        }
        else {
            FIGHTER_LUCINA_STATUS_KIND_SPECIAL_LW_HIT2[ENTRY_ID] = false;
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_lucina
    );
}