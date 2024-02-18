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
    frame_common(fighter);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) == true {
        DamageModule::set_damage_mul_2nd(fighter.module_accessor, 0.65);
        DamageModule::set_reaction_mul(fighter.module_accessor, 0.65);
    };
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) == false {
        DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
        DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
    };
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if StatusModule::situation_kind(fighter.module_accessor) != SITUATION_KIND_AIR
    || !sv_information::is_ready_go()
    || [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status_kind) {
        FLOAT[ENTRY_ID] = 0;
        START_FLOAT[ENTRY_ID] = false;
        CHECK_FLOAT[ENTRY_ID] = 0;
    };
    if FLOAT[ENTRY_ID] == 1{
        if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_MOTION_AIR
        && [
            *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END
        ].contains(&status_kind) == false {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        };
    };
    if StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_AIR {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            CHECK_FLOAT[ENTRY_ID] += 1;
        } else {
            CHECK_FLOAT[ENTRY_ID] = 0;
        };
        if CHECK_FLOAT[ENTRY_ID] >= CHECK_FLOAT_MAX && FLOAT[ENTRY_ID] == 0 {
            START_FLOAT[ENTRY_ID] = true;
        };
    };
    if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind)
    && FLOAT[ENTRY_ID] > 1{
        FLOAT[ENTRY_ID] = 1;
    };
    if FLOAT[ENTRY_ID] > 1{
        FLOAT[ENTRY_ID] -= 1;
        if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        };
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
            FLOAT[ENTRY_ID] = 1;
        };
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            FLOAT[ENTRY_ID] = 1;
        };
        let mut y_add;
        let mut x_add;
        x_add = (stick_x)*X_ACCEL_MUL;
        y_add = (stick_y)*X_ACCEL_MUL;
        if x_add > 0.0 && X[ENTRY_ID] > X_MAX {
            x_add = 0.0;
        };
        if x_add < 0.0 && X[ENTRY_ID] < X_MAX*-1.0 {
            x_add = 0.0;
        };
        if y_add > 0.0 && Y[ENTRY_ID] > Y_MAX {
            y_add = 0.0;
        };
        if y_add < 0.0 && Y[ENTRY_ID] < Y_MAX*-1.0 {
            y_add = 0.0;
        };
        println!("x{}, y{}", X[ENTRY_ID], Y[ENTRY_ID]);
        println!("x_add{}, y_add{}", x_add, y_add);
        X[ENTRY_ID] += x_add;
        Y[ENTRY_ID] += y_add;
        macros::SET_SPEED_EX(fighter, X[ENTRY_ID], Y[ENTRY_ID], *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    } else {
        X[ENTRY_ID] = 0.0;
        Y[ENTRY_ID] = 0.0;
    };
    if START_FLOAT[ENTRY_ID] == true {
        FLOAT[ENTRY_ID] = FLOAT_MAX;
        START_FLOAT[ENTRY_ID] = false;
        if status_kind == *FIGHTER_STATUS_KIND_JUMP {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
        };
        if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
        };
        if [*FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && FLOAT[ENTRY_ID] > 1{
            FLOAT[ENTRY_ID] = 1;
        };
    };
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