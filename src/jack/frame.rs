use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
//use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;

#[fighter_frame( agent = FIGHTER_KIND_JACK )]
pub fn jack_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let kind = smash::app::utility::get_kind(boma); 
        if kind == *FIGHTER_KIND_JACK {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
                DamageModule::set_reaction_mul(fighter.module_accessor, 0.6);
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == false {
                DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
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
        jack_opff
    );
}