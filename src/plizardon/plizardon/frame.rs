use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_plizardon_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    //SFX Controllers
    if [
        *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_GLIDE_LANDING,
        *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_MISS_FOOT, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_CLIFF_CATCH,
        *FIGHTER_STATUS_KIND_GLIDE_ATTACK, *FIGHTER_STATUS_KIND_GLIDE_END
    ].contains(&status_kind) { 
        STOP_SE(fighter, Hash40::new("se_plizardon_glide_start"));
        STOP_SE(fighter, Hash40::new("se_plizardon_glide_loop"));
        STOP_SE(fighter, Hash40::new("se_plizardon_special_h02_02"));
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_STATUS_ATTACK_WORK_FLAG_CRITICAL);
    };
    if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) && 
    ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_GUARD.into(), true.into());
    }
}

pub fn install() {
    Agent::new("plizardon")
    .on_line(Main, frame_plizardon_Main)
    .install();
}
