use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lua2cpp::L2CFighterCommon;

#[fighter_frame( agent = FIGHTER_KIND_LITTLEMAC )]
pub fn littlemac_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        //let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let kind = smash::app::utility::get_kind(boma); 
        if kind == *FIGHTER_KIND_LITTLEMAC {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        littlemac_opff
    );
}