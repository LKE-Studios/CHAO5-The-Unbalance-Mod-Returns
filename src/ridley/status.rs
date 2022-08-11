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
static ANGLE_MAX : f32 = 60.0; //Max Ascent Angle for Glide (degrees)
static ANGLE_LOW_MAX : f32 = -60.0; //Max Descent Angle for Glide (degrees)
static STICK_ANGLE_MUL : f32 = 5.0; //Controls how much Ridley's body rotates according to the control stick (higher value = higher sensitivity)

#[status_script(agent = "ridley", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_start(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_start"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_main as *const () as _))
}

unsafe extern "C" fn glide_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(&mut *fighter.module_accessor);
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_start") && MotionModule::is_end(fighter.module_accessor){
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_wing"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()    
}

#[fighter_frame( agent = FIGHTER_KIND_RIDLEY )]
fn ridley_glide(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE {
            fighter.sub_air_check_fall_common();
            macros::SET_SPEED_EX(fighter, 1.64, -0.485, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); //Base horizontal air mobility and normal descent speed.
            static Y_ACCEL_ADD : f32 = 0.064; //Ascent/Descent Speed Multiplier
            static X_DECEL_MUL_UP : f32 = -0.0082; //Horizontal Air Deceleration multiplier when ascending
            static X_DECEL_MUL_DOWN : f32 = 0.00725; //Horizontal Air Deceleration multiplier when descending
            let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
            if stick_y >= 0.1 || stick_y <= -0.1 { //Used to prevent having a stick_y in the middle from changing flight angle
                ANGLE[ENTRY_ID] += STICK_ANGLE_MUL*stick_y;
                if ANGLE[ENTRY_ID] > ANGLE_MAX {
                    ANGLE[ENTRY_ID] = ANGLE_MAX; //Caps the max upward value at 60 and prevents it from going beyond. 
                };
                if ANGLE[ENTRY_ID] < ANGLE_LOW_MAX {
                    ANGLE[ENTRY_ID] = ANGLE_LOW_MAX; //Caps the max downward value at -60 and prevents it from going beyond. 
                };
            };
            let y = ANGLE[ENTRY_ID] * Y_ACCEL_ADD; //Applies the ascent/descent speed multiplier when angling the glide
            macros::SET_SPEED_EX(fighter, 1.64, -0.485 + y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            if ANGLE[ENTRY_ID] >= -60.0 && ANGLE[ENTRY_ID] <= -0.1 { //Applies the H Air decel. multilplier when descending when angle is between -60 and 0.1
                KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE[ENTRY_ID] * X_DECEL_MUL_DOWN, y:0.0, z:0.0});
            };
            if ANGLE[ENTRY_ID] <= 60.0 && ANGLE[ENTRY_ID] >= 0.1 { //Applies the H Air accel. multilplier when descending when angle is between 0.1 and 60
                KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: ANGLE [ENTRY_ID] * X_DECEL_MUL_UP, y:0.0, z:0.0});
            };
            let rotation = Vector3f { x: ANGLE[ENTRY_ID] * -1.0, y: 0.0, z: 0.0 }; //Controls body rotation & model/bone movement when angling the glide
            let rotation2 = Vector3f{ x: ANGLE[ENTRY_ID]*-0.007, y: ANGLE[ENTRY_ID]*0.005, z: ANGLE[ENTRY_ID]*0.13 };
            let rotation3 = Vector3f{ x: ANGLE[ENTRY_ID]*0.06, y: ANGLE[ENTRY_ID]*-0.082, z: ANGLE[ENTRY_ID]*-0.28 };
            let rotation4 = Vector3f{ x: ANGLE[ENTRY_ID]*-0.05, y: ANGLE[ENTRY_ID]*-0.003, z: ANGLE[ENTRY_ID]*0.175 };
            let rotation5 = Vector3f{ x: ANGLE[ENTRY_ID]*-0.0027, y: ANGLE[ENTRY_ID]*0.0073, z: ANGLE[ENTRY_ID]*0.184 };
            let rotation6 = Vector3f{ x: ANGLE[ENTRY_ID]*0.0019, y: ANGLE[ENTRY_ID]*0.0013, z: ANGLE[ENTRY_ID]*0.11 };
            let rotation7 = Vector3f{ x: ANGLE[ENTRY_ID]*-0.22, y: ANGLE[ENTRY_ID]*-0.00063, z: ANGLE[ENTRY_ID]*0.228 };
            let rotation8 = Vector3f{ x: ANGLE[ENTRY_ID]*0.0, y: ANGLE[ENTRY_ID]*-0.0007, z: ANGLE[ENTRY_ID]*0.0061 };
            let rotation9 = Vector3f{ x: ANGLE[ENTRY_ID]*-0.065, y: ANGLE[ENTRY_ID]*-0.0048, z: ANGLE[ENTRY_ID]*0.35 };
            let rotation10 = Vector3f{ x: ANGLE[ENTRY_ID]*0.05, y: ANGLE[ENTRY_ID]*0.125, z: ANGLE[ENTRY_ID]*-0.094 };
            let rotation11 = Vector3f{ x: ANGLE[ENTRY_ID]*-0.01, y: ANGLE[ENTRY_ID]*0.22, z: ANGLE[ENTRY_ID]*-0.078 };
            let rotation12 = Vector3f{ x: ANGLE[ENTRY_ID]*0.11, y: ANGLE[ENTRY_ID]*0.122, z: ANGLE[ENTRY_ID]*0.064 };
            let rotation13 = Vector3f{ x: ANGLE[ENTRY_ID]*0.0054, y: ANGLE[ENTRY_ID]*0.073, z: ANGLE[ENTRY_ID]*-0.18 };
            let rotation14 = Vector3f{ x: ANGLE[ENTRY_ID]*0.0021, y: ANGLE[ENTRY_ID]*-0.0058, z: ANGLE[ENTRY_ID]*-0.194 };
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &rotation, smash::app::MotionNodeRotateCompose { _address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8 }, smash::app::MotionNodeRotateOrder { _address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8 });
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("bust"), &rotation2,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("neck"), &rotation3,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("head"), &rotation4,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("neck2"), &rotation5,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("shoulderl"), &rotation6,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("armr"), &rotation7,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("shoulderr"), &rotation8,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("tail1"), &rotation9,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("tail2"), &rotation10,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("tail3"), &rotation11,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("tail4"), &rotation12,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("legl"), &rotation13,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("legr"), &rotation14,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
    };
} 

#[status_script(agent = "ridley", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn glide_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    ANGLE[ENTRY_ID] = 0.0;
    macros::SET_SPEED_EX(fighter, 1.64, -0.485, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    L2CValue::I32(0)
}

pub fn install() {
    smashline::install_status_scripts!(
        glide_start, 
        glide_end
    );
    smashline::install_agent_frames!(
        ridley_glide
    );
}