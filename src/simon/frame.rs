use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Hash40;
use smash::hash40;
use smash::phx::Vector3f;

#[fighter_frame( agent = FIGHTER_KIND_SIMON )]
pub fn simon_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{x:1.5, y:1.5, z:4.0});
        };
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{x:1.5, y:1.5, z:4.0});
        };
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{x:1.5, y:1.5, z:4.0});
        };   
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4 {
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{x:1.5, y:1.5, z:4.0});
        };   
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_f") {
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{x:1.5, y:1.5, z:4.0});
        };    
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_b") {
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{x:1.5, y:1.5, z:4.0});
        }; 
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi") {
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{x:1.5, y:1.5, z:4.0});
        };     
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if MotionModule::frame(fighter.module_accessor) > 32.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        simon_opff
    );
}