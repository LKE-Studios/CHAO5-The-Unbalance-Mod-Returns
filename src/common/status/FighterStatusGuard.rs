use crate::imports::BuildImports::*;

#[skyline::hook(replace = L2CFighterCommon_FighterStatusGuard__calc_shield_scale)]
pub unsafe fn FighterStatusGuard__calc_shield_scale(fighter: &mut L2CFighterCommon, shield_level: L2CValue) -> L2CValue {
    let shield_level = shield_level.get_f32();
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let shield_max = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
    let shield_size = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_size"));
    let shield_scale = WorkModule::get_param_float(fighter.module_accessor, hash40("shield_scale"), 0);
    let shield_scale_min = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_scale_min"));
    let shield_radius = WorkModule::get_param_float(fighter.module_accessor, hash40("shield_radius"), 0);
    let scale = (shield_level / shield_max) * (1.0 - shield_scale_min) + shield_scale_min;
    let updated_shield = scale * shield_radius;
    updated_shield.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            FighterStatusGuard__calc_shield_scale
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}