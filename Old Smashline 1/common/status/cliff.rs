use crate::imports::BuildImports::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CliffCatchMove)]
unsafe fn status_CliffCatchMove(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CLIFF_XLU) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    }
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffCatchMove)]
unsafe fn status_end_CliffCatchMove(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CLIFF_CATCH {
        HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
    }
    call_original!(fighter)
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_CliffCatchMove,
            status_end_CliffCatchMove
        );
    }
}