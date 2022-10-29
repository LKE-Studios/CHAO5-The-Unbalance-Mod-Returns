use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon/*, L2CFighterBase*/};
use smash_script::*;
use smashline::*;
use smash::hash40;
use smash::phx::Hash40;

// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
        if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
        }
        if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&status_kind) {
            if GroundModule::is_passable_ground(fighter.module_accessor) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) && stick_y <= 0.2 {
                    GroundModule::pass_floor(fighter.module_accessor);
                }
            }
            else {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                    macros::EFFECT(fighter, Hash40::new("sys_kusudama"), Hash40::new("top"), 0, 28, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
                }
            }
        };
        if MotionModule::motion_kind(boma) == hash40("attack_dash") && MotionModule::frame(boma) <= (9.0) {
            if stick_y > 0.45 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }
        }; 
    }
}

#[skyline::hook(replace=smash::app::FighterUtil::is_valid_just_shield_reflector)]
unsafe fn is_valid_just_shield_reflector(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    return true;
}

// Use this for general per-frame weapon-level hooks
// #[weapon_frame_callback]
// pub fn global_weapon_frame(fighter_base : &mut L2CFighterBase) {
//     unsafe {
//         let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
//         let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;

//         if frame % 10 == 0 {
//             println!("[Weapon Hook] Frame : {}", frame);
//         }
//     }
// }

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame,
        // global_weapon_frame
    );
    skyline::install_hook!(
        is_valid_just_shield_reflector
    );
}