use crate::imports::BuildImports::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_dead_uniq_process_init)]
pub unsafe fn sub_dead_uniq_process_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SILVER = color >= 120 && color <= 127;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if SILVER && fighter_kind == *FIGHTER_KIND_MEWTWO {
        let dead_reason = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_DEAD_REASON);
        let rand_sound = sv_math::rand(hash40("mewtwo"), 4);
        if rand_sound == 0 {
            SoundModule::play_se(fighter.module_accessor, Hash40::new("vc_silver_missfoot02"), false, false, false, false, enSEType(0));
        }
        else {
            SoundModule::play_se(fighter.module_accessor, Hash40::new("vc_mewtwo_missfoot01"), false, false, false, false, enSEType(0));
        }
        call_original!(fighter)
    }
    else {
        call_original!(fighter)
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_dead_uniq_process_init
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}
