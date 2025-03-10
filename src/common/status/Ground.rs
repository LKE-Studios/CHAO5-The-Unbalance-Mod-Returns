use crate::imports::BuildImports::*;

#[skyline::hook(replace=GroundModule::correct)]
pub unsafe fn replace_Ground_correct(module_accessor: &mut smash::app::BattleObjectModuleAccessor, kind: GroundCorrectKind) -> u64 {
    let situation_kind = StatusModule::situation_kind(module_accessor);
    if utility::get_category(module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER || situation_kind != *SITUATION_KIND_GROUND {
        return original!()(module_accessor, kind);
    }
    let status_kind = StatusModule::status_kind(module_accessor);
    if check_fighter_edge_slipoffs(module_accessor).get_bool() {
        return original!()(module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    original!()(module_accessor, kind)
}

unsafe fn check_fighter_edge_slipoffs(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> L2CValue {
    let fighter_kind = utility::get_kind(&mut *module_accessor);
    let status_kind = StatusModule::status_kind(module_accessor);
    let motion_kind = MotionModule::motion_kind(module_accessor);
    if fighter_kind == *FIGHTER_KIND_DAISY {
        if status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END { 
            return true.into(); 
        }
    }
    if fighter_kind == *FIGHTER_KIND_DONKEY { 
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if motion_kind == hash40("special_hi_2") {
                return true.into(); 
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_FOX {
        if [*FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP,
            *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT,
            *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) { 
            return true.into();
        }
    }
    if fighter_kind == *FIGHTER_KIND_WOLF {
        if [*FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP, 
            *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, 
            *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END
        ].contains(&status_kind) { 
            return true.into(); 
        }
    }
    return false.into();
}


pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            replace_Ground_correct
        );
    }
}
