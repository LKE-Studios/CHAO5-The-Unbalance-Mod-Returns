use crate::imports::BuildImports::*;

pub static mut DONKEY_64_DANCE : [bool; 8] = [false; 8];
pub static mut HOLD_TIME : [f32; 8] = [0.0; 8];

pub unsafe extern "C" fn frame_donkey(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(fighter.module_accessor);
    if DONKEY_64_DANCE[ENTRY_ID] == true {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_r_2"), 0.0, 1.0, false, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    };
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL && [hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            HOLD_TIME[ENTRY_ID] += 1.0;
        }
        if HOLD_TIME[ENTRY_ID] == 30.0 {
            DONKEY_64_DANCE[ENTRY_ID] = true;
        }
    }
    else {
        HOLD_TIME[ENTRY_ID] = 0.0;
        DONKEY_64_DANCE[ENTRY_ID] = false;
    };
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
        if ItemModule::is_have_item(fighter.module_accessor, 0) && ItemModule::get_have_item_kind(fighter.module_accessor, 0) == *ITEM_KIND_BARREL {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(),false.into());
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && motion_kind == hash40("special_hi") {
        if frame <= 10.0 && ControlModule::get_stick_y(fighter.module_accessor) <= -0.5 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_2"), -1.0, 1.0, 0.0, false, false);
        }
    }
    SpecialHi2_Function(fighter);
    AppealLw2_Loop_Function(fighter);
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