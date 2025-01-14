use crate::imports::BuildImports::*;

#[skyline::hook(replace = StatusModule::change_status_request_from_script)]
pub unsafe fn change_status_hook(module_accessor: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, unk: bool) -> u64 {
    let next_status = status_kind;
    let get_kind = smash::app::utility::get_kind(&mut *module_accessor);
    if get_kind == *FIGHTER_KIND_ALL {
        if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&next_status) {
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) {
                original!()(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
            } 
            else {
                original!()(module_accessor, status_kind, unk);
            }
        }
        else if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&next_status) {
            if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH) {
                StatusModule::set_situation_kind(module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            }
            original!()(module_accessor, status_kind, unk);
        }
        else {
            original!()(module_accessor, status_kind, unk);
        }
    }
    return original!()(module_accessor, status_kind, unk);
}

#[skyline::hook(replace = smash::app::lua_bind::StatusModule::change_status_request)]
pub unsafe fn change_status_request_hook(module_accessor: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let next_status = status_kind;
    if smash::app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&next_status) {
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) {
                original!()(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false)
            } 
            else {
                original!()(module_accessor, status_kind, arg3)
            }
        } 
        else if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&next_status) {
            if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH) {
                StatusModule::set_situation_kind(module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            }
            original!()(module_accessor, status_kind, arg3)
        }
        else {
            original!()(module_accessor, status_kind, arg3)
        }
    } 
    else {
        original!()(module_accessor, status_kind, arg3)
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_EscapeAir)]
pub unsafe fn status_EscapeAir_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, 0, 0, 0, 0);
    //Automatically forces you to the ground if you're buffering Wavedashes during the startup of Airdodge
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH) && stick_y < 0.5 
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) 
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) {
        GroundModule::attach_ground(fighter.module_accessor, true);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_EscapeAir_Pre
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}