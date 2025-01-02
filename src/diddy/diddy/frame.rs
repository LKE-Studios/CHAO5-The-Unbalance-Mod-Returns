use crate::imports::BuildImports::*;

pub static mut BANANA_EXIST : [bool; 8] = [false; 8];

pub unsafe extern "C" fn frame_diddy(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let frame = MotionModule::frame(fighter.module_accessor);
    if status_kind == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_SHOOT {
        if frame > 5.0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
            }
        }
    }
    if status_kind == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_STICK {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(fighter.module_accessor, -4.0, 0);
        }
    }
    let num_players = Fighter::get_fighter_entry_count();
    for i in 1..num_players {
        let opponent_module_accessor = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(i));
        let opponent_status_kind = StatusModule::status_kind(opponent_module_accessor);
        let banana_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_LW_WORK_INT_BANANA_ID);
        if prev_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DIDDY_INSTANCE_WORK_ID_FLAG_BANANA_EXIST) {
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_DIDDY_INSTANCE_WORK_ID_FLAG_BANANA_EXIST);
        }
        if opponent_status_kind == *FIGHTER_STATUS_KIND_SLIP
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DIDDY_INSTANCE_WORK_ID_FLAG_BANANA_EXIST) {
            WorkModule::set_int(fighter.module_accessor, banana_id, *FIGHTER_DIDDY_STATUS_SPECIAL_LW_WORK_INT_BANANA_ID);
            WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_DIDDY_INSTANCE_WORK_ID_FLAG_BANANA_EXIST);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_LW_LAUGH, false);
        }
    }
}

pub fn install() {
    Agent::new("diddy")
    .on_line(Main, frame_diddy)
    .install();
}