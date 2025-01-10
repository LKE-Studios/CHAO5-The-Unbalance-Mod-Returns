use crate::imports::BuildImports::*; 

// Use this for general per-frame fighter-level hooks
pub unsafe extern "C" fn frame_common(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
    if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&status_kind) {
        if GroundModule::is_passable_ground(fighter.module_accessor) == false {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) || 
                ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) || 
                ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || 
                ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                EFFECT(fighter, Hash40::new("sys_kusudama"), Hash40::new("top"), 0, 28, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
            }
        }
    };
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CLIFF_XLU);
    //Flag Checks
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLAG_CRITICAL) {
        common_attack_critical_flag(fighter);
    }
    loupe_function(fighter);
}

pub unsafe extern "C" fn loupe_function(fighter : &mut L2CFighterCommon) {
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let camzones = get_camera_range();
    let threshold_left = camzones.left();
    let threshold_right = camzones.right();
    let threshold_up = camzones.up();
    let threshold_down = camzones.down();
    let is_too_left = pos_x < threshold_left;
    let is_too_right = pos_x > threshold_right;
    let loupe_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_LOUPE_FRAME);
    if !StopModule::is_stop(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_LOUPE) 
        && (is_too_left || is_too_right) && !CustomModule::is_operation_cpu(fighter.module_accessor) {
            WorkModule::add_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_LOUPE_FRAME);
            println!("{}", loupe_frame);
            if loupe_frame == 1 {
                SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_warning_out"), true, false, false, false, enSEType(0));
            }
        }
        else {
            SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_common_warning_out"), 0);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_LOUPE_FRAME);
        }
    }
}

pub unsafe extern "C" fn jump_cancel(fighter : &mut L2CFighterCommon) {
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    if is_jc(module_accessor, fighter_kind, status_kind, frame) && check_jump(module_accessor) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) 
        < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) 
        && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
        };
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
        };
    };
}

pub fn install() {
    Agent::new("fighter")
    .on_line(Main, frame_common)
    .install();
}