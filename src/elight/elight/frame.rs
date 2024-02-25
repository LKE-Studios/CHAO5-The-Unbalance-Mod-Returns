use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_elight_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_N_END].contains(&status_kind) {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(fighter.module_accessor, -4.0, 0);
        }
    };
    if status_kind == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END {
        if MotionModule::frame(fighter.module_accessor) > 21.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            if is_grounded(fighter.module_accessor) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
            }
        }
    }
    if status_kind == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let lr = PostureModule::lr(fighter.module_accessor);
        if stick_x * lr < -0.75 {
            PostureModule::reverse_lr(fighter.module_accessor);
        }
    }
}

pub fn install() {
    Agent::new("elight")
    .on_line(Main, frame_elight_Main)
    .install();
}