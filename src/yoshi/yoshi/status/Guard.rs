use crate::imports::BuildImports::*;

unsafe extern "C" fn status_yoshi_Guard_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuard_execStatus_common();
    yoshi_guard_exec_helper(fighter);
    0.into()
}

unsafe extern "C" fn status_yoshi_Guard_ExecStop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuard_execStop_common();
    yoshi_guard_exec_helper(fighter);
    0.into()
}

pub unsafe extern "C" fn yoshi_guard_exec_helper(fighter: &mut L2CFighterCommon) {
    let shield_scale = WorkModule::get_param_float(fighter.module_accessor, hash40("shield_radius"), 0);
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: shield_scale, y: shield_scale, z: shield_scale});
    let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let shield_max = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
    let model_color = (shield_hp / shield_max).clamp(0.2, 1.0);
    ModelModule::set_color_rgb(fighter.module_accessor, model_color, model_color, model_color, MODEL_COLOR_TYPE{_address: 0});
}

pub fn install() {
    Agent::new("yoshi")
    .status(Exec, *FIGHTER_STATUS_KIND_GUARD, status_yoshi_Guard_Exec)
    .status(ExecStop, *FIGHTER_STATUS_KIND_GUARD, status_yoshi_Guard_ExecStop)
    .install();
}