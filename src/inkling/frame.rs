use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
//use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;

#[fighter_frame( agent = FIGHTER_KIND_INKLING )]
pub fn inkling_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_100 {
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -1.0, 0);
            }
        };
        if status_kind == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN {
            if MotionModule::frame(fighter.module_accessor) > 25.0 {
                if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                    fighter.sub_wait_ground_check_common(false.into());
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD, false);
                    }
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP, false);
                    }
                }
                if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
                    fighter.sub_air_check_fall_common();
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                    }
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                    }
                }
            }
        };
        if status_kind == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_END {
            if MotionModule::frame(fighter.module_accessor) > 10.0 {
                if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                    fighter.sub_wait_ground_check_common(false.into());
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD, false);
                    }
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP, false);
                    }
                }
                if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
                    fighter.sub_air_check_fall_common();
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                    }
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                    }
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                    }
                }
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        inkling_opff
    );
}