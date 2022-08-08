use smash::lib::lua_const::*;
use smash::phx::Hash40;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;

static mut HOLD_TIME : [f32; 8] = [0.0; 8];

#[fighter_frame( agent = FIGHTER_KIND_RIDLEY )]
fn ridley_opff(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if ![*FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR{
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP){
                HOLD_TIME[ENTRY_ID] +=1.0;
            }
            if HOLD_TIME[ENTRY_ID] == 24.0{
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE.into(), true.into());
            }
        }
        else{
            HOLD_TIME[ENTRY_ID] = 0.0;
        };
        if [*FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_JUMP].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_WAIT].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if status_kind == *FIGHTER_STATUS_KIND_LANDING { 
            macros::STOP_SE(fighter, Hash40::new("se_plizardon_special_h01_win02"));
        };
        if status_kind == *FIGHTER_STATUS_KIND_LANDING_LIGHT { 
            macros::STOP_SE(fighter, Hash40::new("se_plizardon_special_h01_win02"));
        };
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            macros::STOP_SE(fighter, Hash40::new("se_plizardon_special_h01_win02"));
        };
        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            macros::STOP_SE(fighter, Hash40::new("se_plizardon_special_h01_win02"));
        };
        if status_kind == *FIGHTER_STATUS_KIND_DEAD {
            macros::STOP_SE(fighter, Hash40::new("se_plizardon_special_h01_win02"));
        }; 
        if status_kind == *FIGHTER_STATUS_KIND_MISS_FOOT {
            macros::STOP_SE(fighter, Hash40::new("se_plizardon_special_h01_win02"));
        }; 
        if status_kind == *FIGHTER_STATUS_KIND_DAMAGE {
            macros::STOP_SE(fighter, Hash40::new("se_plizardon_special_h01_win02"));
        };
        if status_kind == *FIGHTER_STATUS_KIND_CLIFF_CATCH {
            macros::STOP_SE(fighter, Hash40::new("se_plizardon_special_h01_win02"));
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        ridley_opff
    );
}
