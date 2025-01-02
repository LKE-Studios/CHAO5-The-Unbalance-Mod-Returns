use crate::imports::BuildImports::*;

pub static mut FIGHTER_STATUS_GANON_UNIQ_APPEAL_COUNTER: [bool; 8] = [false; 8];

unsafe extern "C" fn frame_ganon_Main(fighter: &mut L2CFighterCommon) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
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
    };
}

pub fn install() {
    Agent::new("ganon")
    .on_line(Main, frame_ganon_Main)
    .install();
}
