use crate::imports::BuildImports::*;

static mut FLOAT : [i32; 8] = [0; 8]; //Logs Float Time
static mut START_FLOAT : [bool; 8] = [false; 8];
static mut CHECK_FLOAT : [i32; 8] = [0; 8];
static mut CHECK_FLOAT_MAX : i32 = 10; //Frames where jump needs to be held to start floating
static mut X : [f32; 8] = [0.0; 8]; //Logs speed
static mut Y : [f32; 8] = [0.0; 8]; //Logs speed
static mut FLOAT_MAX : i32 = 1200; //Frames this bitch can float (In frames, 300 = 5 seconds)
static mut X_MAX : f32 = 1.81; //Max Horizontal movespeed
static mut X_ACCEL_MUL : f32 = 0.09; //Air Accel Mul
static mut Y_MAX : f32 = 1.24; //Max Vertical movespeed

pub unsafe extern "C" fn frame_edge_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        DamageModule::set_damage_mul_2nd(fighter.module_accessor, 0.65);
        DamageModule::set_reaction_mul(fighter.module_accessor, 0.65);
    }
    else {
        DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
        DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
    };
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(fighter.module_accessor, *STATUS_KIND_NONE, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    };
}

pub unsafe extern "C" fn frame_edge_Exec(fighter : &mut L2CFighterCommon) {
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("swordl1"), &Vector3f{x:1.15, y:1.0, z:1.0});
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("swordr1"), &Vector3f{x:1.15, y:1.0, z:1.0});
}

pub unsafe extern "C" fn frame_edge_flare(weapon : &mut L2CFighterBase) {
    let status = StatusModule::status_kind(weapon.module_accessor);
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if [*WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S].contains(&status) {
        if ControlModule::check_button_on(owner_module_accessor, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_on(owner_module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_EDGE_FIRE_STATUS_KIND_BURST_S, false);
        }
    }
    if [*WEAPON_EDGE_FIRE_STATUS_KIND_FLY_M].contains(&status) {
        if ControlModule::check_button_on(owner_module_accessor, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_on(owner_module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_EDGE_FIRE_STATUS_KIND_BURST_M, false);
        }
    }
    if [*WEAPON_EDGE_FIRE_STATUS_KIND_FLY_L].contains(&status) {
        if ControlModule::check_button_on(owner_module_accessor, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_on(owner_module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_EDGE_FIRE_STATUS_KIND_BURST_L, false);
        }
    }
}

pub fn install() {
    Agent::new("edge")
    .on_line(Main, frame_edge_Main)
    .on_line(Exec, frame_edge_Exec)
    .install();
    
    Agent::new("edge_fire")
    .on_line(Main, frame_edge_flare)
    .install();
}