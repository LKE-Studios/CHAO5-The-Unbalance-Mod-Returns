use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
//use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;

#[fighter_frame( agent = FIGHTER_KIND_KROOL )]
pub fn krool_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        if status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI {
            fighter.sub_air_check_fall_common();
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
            }
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
            }
        }
        if status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_HIT {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -40.0, 0);
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        krool_opff
    );
}