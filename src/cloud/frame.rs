use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::{Vector3f, Hash40};

#[fighter_frame( agent = FIGHTER_KIND_CLOUD )]
pub fn cloud_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if MotionModule::frame(fighter.module_accessor) > 1.0 {
                ModelModule::set_joint_scale(fighter.module_accessor,Hash40::new("haver"), &Vector3f{x:2.0, y:2.0, z:2.0});
                AttackModule::set_attack_scale(fighter.module_accessor, 1.8, true);
            };
            if MotionModule::frame(fighter.module_accessor) > 26.0 {
                ModelModule::set_joint_scale(fighter.module_accessor,Hash40::new("haver"), &Vector3f{x:1.0, y: 1.0,z: 1.0});         
                AttackModule::set_attack_scale(fighter.module_accessor, 1.0, true);
            };
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        cloud_opff
    );
}