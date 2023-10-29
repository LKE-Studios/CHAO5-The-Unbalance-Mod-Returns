use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_PICKEL )]
pub fn frame_pickel(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor); 
        if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD].contains(&status_kind){
            if WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_REACTION_FRAME) > 0.1 {
                FLASH(fighter, 2.00, 0.12, 0.0, 0.55);
            }
            else {
                COL_NORMAL(fighter);
            }
        }
        if status_kind == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_GLIDING {
            fighter.sub_air_check_fall_common();
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_pickel
    );
}