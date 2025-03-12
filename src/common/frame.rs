use crate::imports::BuildImports::*; 

// Use this for general per-frame fighter-level hooks
pub unsafe extern "C" fn frame_common(fighter : &mut L2CFighterCommon) {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
    if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&status_kind) {
        if !GroundModule::is_passable_ground(fighter.module_accessor) {
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
    CustomModule::check_ultra_armor_flag(fighter.module_accessor);
    loupe_function(fighter);
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    if can_use_float(fighter_kind) {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_FLOAT) {
                sub_float_check(fighter);
            }
        } 
        if situation_kind != *SITUATION_KIND_AIR || conditional_statuses(status_kind) || !sv_information::is_ready_go() {
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_FLOAT);
        }  
    }
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL && motion_kind == 0 {
        println!("Motion Kind: {}", motion_kind);
        let rand_val = sv_math::rand(hash40("fighter"), 3);
        if rand_val == 0 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_r"), 0.0, 1.0, false, 0.0, false, false);
        }
        else if rand_val == 1 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_s_r"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_r"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
	println!("{}", shield_hp);
}

pub unsafe extern "C" fn loupe_function(fighter : &mut L2CFighterCommon) {
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let camzones = get_camera_range();
    let threshold_left = camzones.left();
    let threshold_right = camzones.right();
    let threshold_up = camzones.up();
    let threshold_down = camzones.down();
    let is_too_left = pos_x < threshold_left;
    let is_too_right = pos_x > threshold_right;
    let is_too_up = pos_y > threshold_up;
    let is_too_down = pos_y < threshold_down;
    let loupe_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_LOUPE_FRAME);
    if !StopModule::is_stop(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_LOUPE) 
        && (is_too_left || is_too_right) && (!is_too_up || !is_too_down) && !CustomModule::is_operation_cpu(fighter.module_accessor) {
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
        let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
        let status_kind = fighter.global_table[STATUS_KIND].get_i32();
        if [*FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_PLIZARDON].contains(&fighter_kind) {
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_common_warning_out"), 0);
                WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_LOUPE_FRAME);
            }
        }
    }
}

pub unsafe extern "C" fn sub_float_check(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    let mut on_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_BUTTON_ON_FRAME);
    if ![*FIGHTER_STATUS_KIND_FLOAT, *FIGHTER_STATUS_KIND_SLEEP, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR, 
        *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_MISS_FOOT, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_JUMP, 
        *FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) && situation_kind == *SITUATION_KIND_AIR {
        let jump_button_on_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_uniq_float"), hash40("jump_button_on_frame"));
        WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_FLOAT);
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_BUTTON_ON_FRAME);
        }
        if on_frame >= jump_button_on_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_FLOAT.into(), true.into());
        }
    }
    else {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_BUTTON_ON_FRAME);
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

pub fn can_use_float(fighter_kind: i32) -> bool {
    [
        *FIGHTER_KIND_EDGE,
        *FIGHTER_KIND_MEWTWO,
        *FIGHTER_KIND_REFLET,
        *FIGHTER_KIND_SAMUSD
    ].contains(&fighter_kind)
}

pub fn conditional_statuses(status_kind: i32) -> bool {
    [
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_MISS_FOOT, 
        *FIGHTER_STATUS_KIND_DAMAGE, 
        *FIGHTER_STATUS_KIND_DAMAGE_AIR, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY, 
        *FIGHTER_STATUS_KIND_DAMAGE_FALL, 
        *FIGHTER_STATUS_KIND_DAMAGE_SONG, 
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, 
        *FIGHTER_STATUS_KIND_DAMAGE_SONG_FALL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, 
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD, 
        *FIGHTER_STATUS_KIND_ICE
    ].contains(&status_kind)
}

pub fn install() {
    Agent::new("fighter")
    .on_line(Main, frame_common)
    .install();
}