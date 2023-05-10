use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Vector3f;

#[fighter_frame( agent = FIGHTER_KIND_CHROM )]
pub fn frame_chrom(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2 {
            if situation_kind == *SITUATION_KIND_AIR {
                if stick_x != 0.0 {
                    KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f{x: 0.2 * stick_x.signum(), y: 0.0, z: 0.0});
                }
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_chrom
    );
}