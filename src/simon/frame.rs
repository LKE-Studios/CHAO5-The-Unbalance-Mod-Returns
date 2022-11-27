use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lua2cpp::L2CFighterCommon;
//use smash::phx::Hash40;
//use smash::hash40;
//use smash::phx::Vector3f;

#[fighter_frame( agent = FIGHTER_KIND_SIMON )]
pub fn simon_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        //let kind = smash::app::utility::get_kind(boma);   
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if MotionModule::frame(fighter.module_accessor) > 32.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            }
        };
        /*if kind == *FIGHTER_KIND_SIMON {
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_hi3") {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("hookshot1"), &Vector3f{x:3.0, y:1.0, z:1.0});
            };
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_s4_hi") {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("hookshot1"), &Vector3f{x:3.0, y:1.0, z:1.0});
            };
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_s4_s") {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("hookshot1"), &Vector3f{x:3.0, y:1.0, z:1.0});
            };
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_s4_lw") {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("hookshot1"), &Vector3f{x:3.0, y:1.0, z:1.0});
            };
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_hi4") {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("hookshot1"), &Vector3f{x:3.0, y:1.0, z:1.0});
            };   
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_lw4") {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("hookshot1"), &Vector3f{x:3.0, y:1.0, z:1.0});
            }; 
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_f_hi") {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("hookshot1"), &Vector3f{x:3.0, y:1.0, z:1.0});
            };     
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_f") {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("hookshot1"), &Vector3f{x:3.0, y:1.0, z:1.0});
            };    
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_f_lw") {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("hookshot1"), &Vector3f{x:3.0, y:1.0, z:1.0});
            };   
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_b_hi") {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("hookshot1"), &Vector3f{x:3.0, y:1.0, z:1.0});
            };   
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_b") {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("hookshot1"), &Vector3f{x:3.0, y:1.0, z:1.0});
            }; 
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_b_lw") {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("hookshot1"), &Vector3f{x:3.0, y:1.0, z:1.0});
            };   
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi") {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("hookshot1"), &Vector3f{x:3.0, y:1.0, z:1.0});
            };
        };*/
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        simon_opff
    );
}