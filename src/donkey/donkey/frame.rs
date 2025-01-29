use crate::imports::BuildImports::*;

pub static button_on_frame : f32 = 30.0;

pub unsafe extern "C" fn frame_donkey(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if status_kind == *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_APPEAL_SPECIAL) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_r_2"), 0.0, 1.0, false, 0.0, false, false);
    };
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL && [hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind) {
        let frame_float = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_BUTTON_ON_FRAME);
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            WorkModule::add_float(fighter.module_accessor, 1.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_BUTTON_ON_FRAME);
        }
        if frame_float == button_on_frame {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_APPEAL_SPECIAL);
        }
    }
    else {
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_BUTTON_ON_FRAME);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_APPEAL_SPECIAL);
    };
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
        if ItemModule::is_have_item(fighter.module_accessor, 0) 
        && ItemModule::get_have_item_kind(fighter.module_accessor, 0) == *ITEM_KIND_BARREL {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(),false.into());
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && motion_kind == hash40("special_hi") {
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        if frame <= 10.0 && stick_y <= -0.5 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_2"), -1.0, 1.0, 0.0, false, false);
        }
    }
    SpecialHi2_Function(fighter);
    AppealLw2_Loop_Function(fighter);
    if [*FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_WAIT, *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_WALK,
        *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_TURN, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WAIT,
        *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WALK, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_TURN].contains(&status_kind) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
}     

unsafe fn SpecialHi2_Function(fighter: &mut L2CFighterCommon) {
    let frame = MotionModule::frame(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let x_acl_ground = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("x_acl_ground"));
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_hi_2") {
        if ControlModule::get_stick_x(fighter.module_accessor) * lr < 0.0 {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: -x_acl_ground, y: 0.0, z: 0.0});
        }
    }
}

unsafe fn AppealLw2_Loop_Function(fighter: &mut L2CFighterCommon) {
    let frame = MotionModule::frame(fighter.module_accessor);
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_lw_r_2") {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            if frame >= 134.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_r_2"), 33.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_lw_l_2") {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            if frame >= 134.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_l_2"), 33.0, 1.0, false, 0.0, false, false);
            }
        }
    }
}

pub fn install() {
    Agent::new("donkey")
    .on_line(Main, frame_donkey)
    .install();
}