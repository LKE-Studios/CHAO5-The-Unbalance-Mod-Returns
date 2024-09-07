use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_cloud(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
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

pub fn install() {
    Agent::new("cloud")
    .on_line(Exec, frame_cloud)
    .install();
}