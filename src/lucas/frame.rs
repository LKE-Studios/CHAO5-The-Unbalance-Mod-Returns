use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lua2cpp::L2CFighterCommon;

static mut HOLD_TIME : [f32; 8] = [0.0; 8]; 

#[fighter_frame( agent = FIGHTER_KIND_LUCAS )]
fn lucas_opff(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let kind = smash::app::utility::get_kind(boma);
        if kind == *FIGHTER_KIND_LUCAS && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                HOLD_TIME[ENTRY_ID] +=1.0;
            }
            if HOLD_TIME[ENTRY_ID] == 30.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("wess_dance"), -1.0, 1.0, false, 0.0, false, false);
            }
            else {
                HOLD_TIME[ENTRY_ID] = 0.0;
            }
        }
    }
}        

pub fn install() {
    smashline::install_agent_frames!(
        lucas_opff
    );
}