use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
//use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;

#[fighter_frame( agent = FIGHTER_KIND_PURIN )]
pub fn frame_purin(fighter : &mut L2CFighterCommon) {
    unsafe {

        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);

        if [
            *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD,
            *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD_MAX
        ].contains(&status_kind) {
            fighter.sub_wait_ground_check_common(false.into());
            fighter.sub_air_check_fall_common();
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
                }
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
                }
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
                }
            }
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), true.into());
                }
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), true.into());
                }
            }
        }
        if [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL,
            *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_TURN,
            *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL_AIR
        ].contains(&status_kind) {
            fighter.sub_wait_ground_check_common(false.into());
            fighter.sub_air_check_fall_common();
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
                }
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
                }
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
                }
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    fighter.change_status(FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_END.into(), true.into());
                }
            }
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), true.into());
                }
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), true.into());
                }
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    fighter.change_status(FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_END.into(), true.into());
                }
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_purin
    );
}