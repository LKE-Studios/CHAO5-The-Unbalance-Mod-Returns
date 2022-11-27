//use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lua2cpp::L2CFighterCommon;

pub static mut CLAUS_PK_BEAM : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_LUCAS )]
pub fn lucas_opff(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let kind = smash::app::utility::get_kind(boma);
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
        WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
            if CLAUS_PK_BEAM[ENTRY_ID] {
                if [*FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) == false {
                        CLAUS_PK_BEAM[ENTRY_ID] = false;
                        MotionModule::set_rate(fighter.module_accessor, 1.0);
                    }
                }
                else {
                    CLAUS_PK_BEAM[ENTRY_ID] = false;
                };
            }
        }
    }
}        

pub fn install() {
    smashline::install_agent_frames!(
        lucas_opff
    );
}