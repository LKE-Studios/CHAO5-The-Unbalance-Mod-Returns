use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_KOOPA )]
pub fn frame_koopa(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
        WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Midbus
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
                KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 3.5, y:0.0, z:0.0});
            }
            if status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G {
                if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw") && MotionModule::is_end(fighter.module_accessor) {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
                }
            };
            if status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_A {
                if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw") && MotionModule::is_end(fighter.module_accessor) {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
                }
            };
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_koopa
    );
}