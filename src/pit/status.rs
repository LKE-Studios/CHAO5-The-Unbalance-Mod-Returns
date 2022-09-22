use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;
use crate::utils::*;
use smashline::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
//use smash::app::sv_battle_object::module_accessor;
use smash::hash40;
use smash::phx::{Vector3f, Hash40};
use smash_script::*;

static mut ANGLE : [f32; 8] = [0.0; 8];
static ANGLE_MAX : f32 = 65.0; //Max Ascent Angle for Glide (degrees)
static ANGLE_LOW_MAX : f32 = -65.0; //Max Descent Angle for Glide (degrees)
static mut MOMENTUM : [f32; 8] = [0.0; 8];
static THRESHOLD_MAX : f32 = -20.0;
/*static mut ANGLE_FRAME : [f32; 8] = [90.0; 8];
static DIRECTION_UP : f32 = 25.0;
static DIRECTION_DOWN : f32 = 155.0;*/
static STICK_ANGLE_MUL : f32 = 6.0; //Controls how much Pit's body rotates according to the control stick (higher value = higher sensitivity)

#[status_script(agent = "pit", status = FIGHTER_STATUS_KIND_GLIDE_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_start_a(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_start"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_start_b as *const () as _))
}

unsafe extern "C" fn glide_start_b(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_start") && MotionModule::is_end(fighter.module_accessor) {   
        fighter.change_status(FIGHTER_STATUS_KIND_GLIDE.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "pit", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_wing"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_core as *const () as _))
}

unsafe extern "C" fn glide_core(fighter: &mut L2CFighterCommon) -> L2CValue {
    if is_grounded(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_LANDING.into(), true.into());
    }
    0.into()
}

#[fighter_frame( agent = FIGHTER_KIND_PIT )]
fn pit_glide(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE {
            fighter.sub_air_check_fall_common();
            macros::SET_SPEED_EX(fighter, 1.9, -0.37, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); //Base horizontal air mobility and normal descent speed.
            static Y_ACCEL_ADD : f32 = 0.044; //Ascent/Descent Speed Multiplier
            static X_DECEL_MUL_UP : f32 = -0.023; //Horizontal Air Deceleration multiplier when ascending in between higher angle values
            static X_ACCEL_MUL_DOWN : f32 = -0.025; //Horizontal Air Acceleration multiplier when descending in between lower angle values
            static X_DECEL_MUL_DOWN : f32 = 0.0332; //Horizontal Air Deceleration multiplier when descending in between higher angle values
            let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
            let y = ANGLE[ENTRY_ID] * Y_ACCEL_ADD; //Applies the ascent/descent speed multiplier when angling the glide
            let x = MOMENTUM[ENTRY_ID] * X_ACCEL_MUL_DOWN;
            let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            if stick_y > 0.0 || stick_y <= 0.0 { //Used to prevent having a stick_y in the middle from changing flight angle
                ANGLE[ENTRY_ID] += STICK_ANGLE_MUL*stick_y;
                //ANGLE_FRAME[ENTRY_ID] -= STICK_ANGLE_MUL * stick_y;
                if ANGLE[ENTRY_ID] > ANGLE_MAX {
                    ANGLE[ENTRY_ID] = ANGLE_MAX; //Caps the max upward value at 65 and prevents it from going beyond. 
                    MOMENTUM[ENTRY_ID] = THRESHOLD_MAX;
                    //ANGLE_FRAME[ENTRY_ID] = DIRECTION_UP;
                };
                if ANGLE[ENTRY_ID] < ANGLE_LOW_MAX {
                    ANGLE[ENTRY_ID] = ANGLE_LOW_MAX; //Caps the max downward value at -65 and prevents it from going beyond. 
                    MOMENTUM[ENTRY_ID] = THRESHOLD_MAX;
                    //ANGLE_FRAME[ENTRY_ID] = DIRECTION_DOWN;
                };
            };
            /*if ANGLE[ENTRY_ID] <= 65.0 && ANGLE[ENTRY_ID] > 0.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_direction"), ANGLE_FRAME[ENTRY_ID], 1.0, true, 0.0, false, false);
                MotionModule::add_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, Hash40::new("glide_wing"), 1.0, 1.0, true, true, 0.0, false, false, true);
            };
            if ANGLE[ENTRY_ID] >= -65.0 && ANGLE[ENTRY_ID] < 0.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_direction"), ANGLE_FRAME[ENTRY_ID], 1.0, true, 0.0, false, false);
                MotionModule::add_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, Hash40::new("glide_wing"), 1.0, 1.0, true, true, 0.0, false, false, true);
            };*/
            //Forward Speed Stuff
            if ANGLE[ENTRY_ID] >= -65.0 && ANGLE[ENTRY_ID] < -20.0 { //Applies the H Air decel. multilplier when descending when angle is between -65 and -35.1
                macros::SET_SPEED_EX(fighter, 2.56 + x, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE[ENTRY_ID] * X_DECEL_MUL_DOWN, y:0.0, z:0.0});
            };
            if ANGLE[ENTRY_ID] >= -20.0 && ANGLE[ENTRY_ID] < 0.0 { //Applies the H Air accel. multilplier when descending when angle is between -20 and 0.1
                macros::SET_SPEED_EX(fighter, 1.9 + x, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            };
            if ANGLE[ENTRY_ID] <= 65.0 && ANGLE[ENTRY_ID] > 0.0 { //Applies the H Air decel. multilplier when descending when angle is between 30.1 and 65
                macros::SET_SPEED_EX(fighter, 1.9 + x, y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE [ENTRY_ID] * X_DECEL_MUL_UP, y:0.0, z:0.0});
            };
            if speed_x * PostureModule::lr(fighter.module_accessor) < 1.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_END.into(), true.into());
            }
            let rotation = Vector3f { x: ANGLE[ENTRY_ID] * -1.15, y: 0.0, z: 0.0 }; //Controls body rotation & model/bone movement when angling the glide
            let rotation1 = Vector3f{ x: ANGLE[ENTRY_ID] * -0.9, y: 0.0, z: 0.0 };
            let rotation2 = Vector3f{ x: ANGLE[ENTRY_ID]*-0.0, y: ANGLE[ENTRY_ID]*-0.0, z: ANGLE[ENTRY_ID]*0.22 };
            let rotation3 = Vector3f{ x: ANGLE[ENTRY_ID]*0.0, y: ANGLE[ENTRY_ID]*0.0, z: ANGLE[ENTRY_ID]*-0.81 };
            let rotation4 = Vector3f{ x: ANGLE[ENTRY_ID]*-0.01, y: ANGLE[ENTRY_ID]*-0.0425, z: ANGLE[ENTRY_ID]*0.55 };
            let rotation5 = Vector3f{ x: ANGLE[ENTRY_ID]*-0.13, y: ANGLE[ENTRY_ID]*0.055, z: ANGLE[ENTRY_ID]*0.26 };
            let rotation6 = Vector3f{ x: ANGLE[ENTRY_ID]*-0.0315, y: ANGLE[ENTRY_ID]*0.0875, z: ANGLE[ENTRY_ID]*-0.11 };
            let rotation7 = Vector3f{ x: ANGLE[ENTRY_ID]*0.0, y: ANGLE[ENTRY_ID]*0.00, z: ANGLE[ENTRY_ID]*0.65 };
            let rotation8 = Vector3f{ x: ANGLE[ENTRY_ID]*0.09, y: ANGLE[ENTRY_ID]*-0.11, z: ANGLE[ENTRY_ID]*0.122 };
            let rotation9 = Vector3f{ x: ANGLE[ENTRY_ID]*0.083, y: ANGLE[ENTRY_ID]*-0.064, z: ANGLE[ENTRY_ID]*-0.19 };
            let rotation10 = Vector3f{ x: ANGLE[ENTRY_ID]*-0.0, y: ANGLE[ENTRY_ID]*-0.0, z: ANGLE[ENTRY_ID]*-0.3 };
            let rotation11 = Vector3f{ x: ANGLE[ENTRY_ID]*0.0, y: ANGLE[ENTRY_ID]*0.0, z: ANGLE[ENTRY_ID]*0.15 };
            let rotation12 = Vector3f{ x: ANGLE[ENTRY_ID]*0.0, y: ANGLE[ENTRY_ID]*0.0, z: ANGLE[ENTRY_ID]*-0.39 };
            let rotation13 = Vector3f{ x: ANGLE[ENTRY_ID]*0.0, y: ANGLE[ENTRY_ID]*0.0, z: ANGLE[ENTRY_ID]*0.64};
            let rotation14 = Vector3f{ x: ANGLE[ENTRY_ID]*0.0, y: ANGLE[ENTRY_ID]*0.0, z: ANGLE[ENTRY_ID]*0.5 };
            let rotation15 = Vector3f{ x: ANGLE[ENTRY_ID]*0.0, y: ANGLE[ENTRY_ID]*0.0, z: ANGLE[ENTRY_ID]*-0.08 };
            let rotation16 = Vector3f{ x: ANGLE[ENTRY_ID]*0.0, y: ANGLE[ENTRY_ID]*0.0, z: ANGLE[ENTRY_ID]*0.1 };
            if ANGLE[ENTRY_ID] >= -65.0 && ANGLE[ENTRY_ID] <= -0.1 {
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation, smash::app::MotionNodeRotateCompose { _address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8 }, smash::app::MotionNodeRotateOrder { _address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8 });
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("waist"), &rotation2,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("legr"), &rotation6,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("kneer"), &rotation4,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("neck"), &rotation11,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("legl"), &rotation7,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("kneel"), &rotation3,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            }
            if ANGLE[ENTRY_ID] <= 65.0 && ANGLE[ENTRY_ID] >= 0.1 {
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation1, smash::app::MotionNodeRotateCompose { _address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8 }, smash::app::MotionNodeRotateOrder { _address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8 });
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("waist"), &rotation10,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("neck"), &rotation14,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("legr"), &rotation12,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("kneer"), &rotation13,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("legl"), &rotation15,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
                ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("kneel"), &rotation16,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            }
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("armr"), &rotation5,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});       
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("shoulderr"), &rotation8,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("arml"), &rotation9,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            //Cancel Stuff
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_ATTACK.into(), true.into());
            }
        }
    };
}

#[status_script(agent = "pit", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn glide_finish(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    ANGLE[ENTRY_ID] = 0.0;
    MOMENTUM[ENTRY_ID] = 0.0;
    //ANGLE_FRAME[ENTRY_ID] = 90.0;
    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, false);
    L2CValue::I32(0)
}

#[status_script(agent = "pit", status = FIGHTER_STATUS_KIND_GLIDE_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_attack_a(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_attack"), -1.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_attack_b as *const () as _))
}

unsafe extern "C" fn glide_attack_b(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_fall_common();
    WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "pit", status = FIGHTER_STATUS_KIND_GLIDE_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_end_a(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_end"), -1.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_end_b as *const () as _))
}

unsafe extern "C" fn glide_end_b(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_fall_common();
    WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_end") && MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "pit", status = FIGHTER_STATUS_KIND_GLIDE_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_landing_a(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_landing"), -1.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_landing_b as *const () as _))
}

unsafe extern "C" fn glide_landing_b(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_landing") && MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN_WAIT.into(), false.into());
    }
    L2CValue::I32(0)
}

pub fn install() {
    smashline::install_status_scripts!(
        glide_start_a, 
        glide_main,
        glide_finish,
        glide_attack_a,
        glide_end_a,
        glide_landing_a
    );
    smashline::install_agent_frames!(
        pit_glide
    );
}