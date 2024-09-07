use crate::imports::BuildImports::*; 

// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
        platform_frame(fighter);
        //If ground/platform is solid while shielding and taunting, run this instead
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
        //Infinite Ledge Regrabs and Invincibility
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CLIFF_XLU);
        //Flag Checks
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_ATTACK_WORK_FLAG_CRITICAL) {
            common_attack_critical_flag(fighter);
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_LOUPE) {
            SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_common_warning_out"), 0);
        }
    }
}

//Pseudo Platform Cancelling
pub unsafe fn platform_frame(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    //GLOBAL
    if status_kind == *FIGHTER_STATUS_KIND_LANDING && prev_status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR || prev_status_kind == *FIGHTER_STATUS_KIND_GUARD {
        if GroundModule::is_passable_ground(fighter.module_accessor) {
            if stick_y < -0.66 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
    //PIKACHU
    if fighter_kind == *FIGHTER_KIND_PIKACHU {
        if status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END || status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END {
            if situation_kind == *SITUATION_KIND_GROUND {
                if GroundModule::is_passable_ground(fighter.module_accessor) {
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
                    if stick_y < -0.66 {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                    }
                }
            }
        }
    }
    //NESS
    if fighter_kind == *FIGHTER_KIND_NESS {
        if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
            if situation_kind == *SITUATION_KIND_GROUND {
                if GroundModule::is_passable_ground(fighter.module_accessor) {
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
                    if stick_y < -0.66 {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                    }
                }
            }
        }
    }
    //FALCO
    if fighter_kind == *FIGHTER_KIND_FALCO {
        if [hash40("special_s_end"), hash40("special_air_s_end")].contains(&motion_kind) || 
        status_kind == *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH_END {
            if situation_kind == *SITUATION_KIND_GROUND {
                if GroundModule::is_passable_ground(fighter.module_accessor) {
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
                    if stick_y < -0.66 {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                    }
                }
            }
        }
    }
    //METAKNIGHT
    if fighter_kind == *FIGHTER_KIND_METAKNIGHT {
        if status_kind == *FIGHTER_STATUS_KIND_LANDING_LIGHT && prev_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || 
        prev_status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP {
            if GroundModule::is_passable_ground(fighter.module_accessor) {
                if stick_y < -0.66 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                }
            }
        }
        if motion_kind == hash40("special_s_end") {
            if situation_kind == *SITUATION_KIND_GROUND {
                if GroundModule::is_passable_ground(fighter.module_accessor) {
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
                    if stick_y < -0.66 {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                    }
                }
            }
        }
    }
    //GANON
    if fighter_kind == *FIGHTER_KIND_GANON {
        if [hash40("special_lw_end"), hash40("special_lw_end_air"), hash40("special_air_lw_end"), hash40("special_air_lw_end_air")].contains(&motion_kind) {
            if situation_kind == *SITUATION_KIND_GROUND {
                if GroundModule::is_passable_ground(fighter.module_accessor) {
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
                    if stick_y < -0.66 {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                    }
                }
            }
        }
    }
}

#[weapon_frame_callback]
pub fn global_weapon_frame(weapon: &mut L2CFighterBase) {
    unsafe {}
}

#[smashline::fighter_init]
fn global_fighter_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_LOUPE) {
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_warning_out"), true, false, false, false, enSEType(0));
        }
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame,
        //global_weapon_frame,
    );
    smashline::install_agent_init_callbacks!(
        global_fighter_init
    );
}