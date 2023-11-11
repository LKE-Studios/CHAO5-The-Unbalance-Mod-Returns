use crate::imports::BuildImports::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_dead_uniq_process_exec)]
pub unsafe fn sub_dead_uniq_process_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let SILVER = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 128 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 135;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if SILVER && fighter_kind == *FIGHTER_KIND_MEWTWO {
        let rand_sound = smash::app::sv_math::rand(hash40("mewtwo"), 4);
        if rand_sound == 0 {
            SoundModule::play_se(fighter.module_accessor, Hash40::new("vc_silver_missfoot02"), false, false, false, false, enSEType(0));
        }
        else {
            SoundModule::play_se(fighter.module_accessor, Hash40::new_raw(0x13846493d2), false, false, false, false, enSEType(0));
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
            sub_dead_uniq_process_exec
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}