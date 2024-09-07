use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_IKE )]
pub fn frame_ike(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        if status_kind == *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK {
            if MotionModule::frame(fighter.module_accessor) > 11.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                if is_grounded(fighter.module_accessor) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                }
            }
        };
        if status_kind == *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        };
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2].contains(&status_kind) {
            if stick_x != 0.0 {
                KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f{x: 0.4 * stick_x.signum(), y: 0.0, z: 0.0});
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_ike
    );
}