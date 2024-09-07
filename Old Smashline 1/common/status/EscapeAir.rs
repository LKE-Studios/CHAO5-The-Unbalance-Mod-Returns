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
            if WorkModule::is_flag(module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH) {
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
            if WorkModule::is_flag(module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH) {
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

#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE, symbol = "_ZN7lua2cpp16L2CFighterCommon20status_pre_EscapeAirEv")]
pub unsafe fn status_pre_escape_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Automatically forces you to the ground if you're buffering Wavedashes during the startup of Airdodge
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH) && stick_y < 0.5 && 
    ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) {
        GroundModule::attach_ground(fighter.module_accessor, true);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }
    call_original!(fighter)
}

pub fn install() {
    skyline::install_hooks!(
        change_status_hook,
        change_status_request_hook
    );
    install_status_scripts!(
        status_pre_escape_air
    );
}