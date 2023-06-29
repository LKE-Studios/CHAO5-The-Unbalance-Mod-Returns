use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_JACK )]
pub fn frame_jack(fighter : &mut L2CFighterCommon) {
    unsafe { 
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
            DamageModule::set_reaction_mul(fighter.module_accessor, 0.6);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == false {
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
        }
        if status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_RUSH {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
            }
        };
        if status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW2_COUNTER {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -66.0, 0);
            }
        };

    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_jack
    );
}