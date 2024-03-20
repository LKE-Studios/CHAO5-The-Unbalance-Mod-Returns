use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_pit_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    frame_common(fighter);
    //SFX Controllers
    if [
        *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_GLIDE_LANDING,
        *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_MISS_FOOT, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_CLIFF_CATCH,
        *FIGHTER_STATUS_KIND_GLIDE_ATTACK, *FIGHTER_STATUS_KIND_GLIDE_END
    ].contains(&status_kind) { 
        STOP_SE(fighter, Hash40::new("se_pit_glide_loop"));
    };
    if status_kind == *FIGHTER_STATUS_KIND_GLIDE {
        let mut angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
        let angle_se_pitch_ratio = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_se_pitch_ratio"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_pit_glide_loop"), 1.0 + angle * angle_se_pitch_ratio);
    }
    if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(fighter.module_accessor, -7.5, 0);
        }
    }
    if status_kind == FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY || status_kind == FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN {
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingl1"), &Vector3f{x: 1.5, y: 1.5, z: 1.5});
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingr1"), &Vector3f{x: 1.5, y: 1.5, z: 1.5});
    };
}

unsafe extern "C" fn frame_pit_Exec(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if [
        *FIGHTER_STATUS_KIND_GLIDE_START,
        *FIGHTER_STATUS_KIND_GLIDE
    ].contains(&status_kind) { 
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingl1"), &Vector3f{x:1.2, y:1.2, z:1.2});
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingr1"), &Vector3f{x:1.2, y:1.2, z:1.2});
    }
    else {
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingl1"), &Vector3f{x:1.0, y:1.0, z:1.0});
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingr1"), &Vector3f{x:1.0, y:1.0, z:1.0});
    };
}

pub fn install() {
    Agent::new("pit")
    .on_line(Main, frame_pit_Main)
    .on_line(Exec, frame_pit_Exec)
    .install();
}
