use crate::imports::BuildImports::*;

static mut ATTACK_LW_EFFECT : [i32; 8] = [0; 8];

unsafe extern "C" fn frame_krystal_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(fighter.module_accessor);
    let KRYSTAL = color >= 64 && color <= 71; 
    if KRYSTAL {
        if situation_kind == *SITUATION_KIND_AIR {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_ENABLE);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_INPUT);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_INPUT_BACK);
        }
        ModelModule::set_scale(fighter.module_accessor, 0.89);
        AttackModule::set_attack_scale(fighter.module_accessor, 0.89, true);
        GrabModule::set_size_mul(fighter.module_accessor, 0.89);
        if ![*FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_LW4_IS_CHARGED);
        };
        if ![*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END].contains(&status_kind) {
            STOP_SE(fighter, Hash40::new("se_pitb_special_h02"));
        }
        if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_HOLD {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -5.0, 0);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_recovery"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
                PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
            }
            if situation_kind != *SITUATION_KIND_GROUND {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                }
            }
            else {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP, false);
                }
            }
        }
    }
}

pub fn install() {
    Agent::new("pitb")
    .on_line(Main, frame_krystal_Main)
    .install();
}
