use crate::imports::BuildImports::*;

pub static mut BANANA_EXIST : [bool; 8] = [false; 8];

pub unsafe extern "C" fn frame_diddy(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let frame = MotionModule::frame(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
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
    let opponent_boma_1 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
    let opponent_status_kind_1 = StatusModule::status_kind(opponent_boma_1);
    let opponent_boma_2 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(2));
    let opponent_status_kind_2 = StatusModule::status_kind(opponent_boma_2);
    let opponent_boma_3 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(3));
    let opponent_status_kind_3 = StatusModule::status_kind(opponent_boma_3);
    let opponent_boma_4 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(4));
    let opponent_status_kind_4 = StatusModule::status_kind(opponent_boma_4);
    let opponent_boma_5 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(5));
    let opponent_status_kind_5 = StatusModule::status_kind(opponent_boma_5);
    let opponent_boma_6 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(6));
    let opponent_status_kind_6 = StatusModule::status_kind(opponent_boma_6);
    let opponent_boma_7 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(7));
    let opponent_status_kind_7 = StatusModule::status_kind(opponent_boma_7);
    let banana_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_LW_WORK_INT_BANANA_ID);
    if prev_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW
    && BANANA_EXIST[ENTRY_ID] == false {
        BANANA_EXIST[ENTRY_ID] = true;
    }
    if (opponent_status_kind_1 == *FIGHTER_STATUS_KIND_SLIP || opponent_status_kind_2 == *FIGHTER_STATUS_KIND_SLIP || opponent_status_kind_3 == *FIGHTER_STATUS_KIND_SLIP || opponent_status_kind_4 == *FIGHTER_STATUS_KIND_SLIP 
    || opponent_status_kind_5 == *FIGHTER_STATUS_KIND_SLIP || opponent_status_kind_6 == *FIGHTER_STATUS_KIND_SLIP || opponent_status_kind_7 == *FIGHTER_STATUS_KIND_SLIP)
    && BANANA_EXIST[ENTRY_ID] == true {
        WorkModule::set_int(fighter.module_accessor, banana_id, *FIGHTER_DIDDY_STATUS_SPECIAL_LW_WORK_INT_BANANA_ID);
        BANANA_EXIST[ENTRY_ID] = false;
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_LW_LAUGH, false);
    }
    println!("Is Banana Exist {:?}", BANANA_EXIST[ENTRY_ID]);
}

pub fn install() {
    Agent::new("diddy")
    .on_line(Main, frame_diddy)
    .install();
}