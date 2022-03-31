
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;
use crate::utils::*;
use smashline::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::phx::{Vector3f, Hash40};

pub const FIGHTER_PEACH_STATUS_KIND_KAMEHAMEHA_START: i32 = 0x1eb; //491
pub const FIGHTER_PEACH_STATUS_KIND_KAMEHAMEHA_CHARGE: i32 = 0x1ec; //492
pub const FIGHTER_PEACH_STATUS_KIND_KAMEHAMEHA_FIRE: i32 = 0x1ed; //493
pub static mut CHARGE_TIME:[f32;8] = [0.0;8];

#[fighter_frame(agent = FIGHTER_KIND_PEACH)]
pub fn peach_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(module_accessor);
        let entry_id = get_entry_id(module_accessor);
        if status_kind == FIGHTER_PEACH_STATUS_KIND_KAMEHAMEHA_CHARGE{
            CHARGE_TIME[entry_id] += 1.0;
        }
        if ![FIGHTER_PEACH_STATUS_KIND_KAMEHAMEHA_START, FIGHTER_PEACH_STATUS_KIND_KAMEHAMEHA_CHARGE, FIGHTER_PEACH_STATUS_KIND_KAMEHAMEHA_FIRE]
            .contains(&status_kind){
            CHARGE_TIME[entry_id] = 0.0;
        }
        if [*FIGHTER_STATUS_KIND_ATTACK_S4_START,*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind){
            ModelModule::set_joint_scale(module_accessor, Hash40::new("havel"), &Vector3f{
                x: 8.0,
                y: 8.0,
                z: 8.0
            });
            ModelModule::set_joint_scale(module_accessor, Hash40::new("haver"), &Vector3f{
                x: 8.0,
                y: 8.0,
                z: 8.0
            });
            AttackModule::set_attack_scale(module_accessor, 8.0, true);
        }
        else{
            AttackModule::set_attack_scale(module_accessor, 1.0, true);
        }
    }
}

#[status_script(agent = "peach", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn peach_specialn(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_PEACH_STATUS_KIND_KAMEHAMEHA_START.into(), false.into());
    L2CValue::I32(0)
}

//idk why I can't put in the constant as the status. Only the raw i32 or LuaConst works. Smashline issue
#[status_script(agent = "peach", status = 0x1eb, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn peach_kamehameha_start_pre(_fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CValue::I32(0)
}

#[status_script(agent = "peach", status = 0x1eb, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn peach_kamehameha_start(fighter: &mut L2CFighterCommon) -> L2CValue {
    change_motion(fighter.module_accessor, "kamehameha_start");
    fighter.sub_shift_status_main(L2CValue::Ptr(peach_kamehameha_start_main as *const () as _))
}

unsafe extern "C" fn peach_kamehameha_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = get_entry_id(fighter.module_accessor) as i32;
    set_position_lock(entry_id);
    if MotionModule::is_end(fighter.module_accessor){
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_KAMEHAMEHA_CHARGE.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "peach", status = 0x1eb, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn peach_kamehameha_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = get_entry_id(fighter.module_accessor) as i32;
    unset_position_lock(entry_id);
    L2CValue::I32(0)
}

#[status_script(agent = "peach", status = 0x1ec, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn peach_kamehameha_charge_pre(_fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CValue::I32(0)
}

#[status_script(agent = "peach", status = 0x1ec, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn peach_kamehameha_charge(fighter: &mut L2CFighterCommon) -> L2CValue {
    change_motion(fighter.module_accessor, "kamehameha_charge");
    fighter.sub_shift_status_main(L2CValue::Ptr(peach_kamehameha_charge_main as *const () as _))
}

unsafe extern "C" fn peach_kamehameha_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = get_entry_id(fighter.module_accessor);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
        MotionModule::set_rate(fighter.module_accessor, 0.1);
        set_position_lock(entry_id as i32);
    }
    else{
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_KAMEHAMEHA_FIRE.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "peach", status = 0x1ec, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn peach_kamehameha_charge_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = get_entry_id(fighter.module_accessor) as i32;
    unset_position_lock(entry_id);
    L2CValue::I32(0)
}

#[status_script(agent = "peach", status = 0x1ed, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn peach_kamehamehafire_pre(_fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CValue::I32(0)
}

#[status_script(agent = "peach", status = 0x1ed, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn peach_kamehamehafire(fighter: &mut L2CFighterCommon) -> L2CValue {
    change_motion(fighter.module_accessor, "kamehameha_fire");
    fighter.sub_shift_status_main(L2CValue::Ptr(peach_kamehamehafire_main as *const () as _))
}

unsafe extern "C" fn peach_kamehamehafire_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = get_entry_id(fighter.module_accessor);
    set_position_lock(entry_id as i32);
    if MotionModule::is_end(fighter.module_accessor){
        if is_grounded(fighter.module_accessor){
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else{
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "peach", status = 0x1ed, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn peach_kamehamehafire_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = get_entry_id(fighter.module_accessor) as i32;
    unset_position_lock(entry_id);
    L2CValue::I32(0)
}


pub fn install() {
    smashline::install_status_scripts!(
        peach_specialn,
        peach_kamehameha_start_pre, peach_kamehameha_start, peach_kamehameha_start_end,
        peach_kamehameha_charge_pre, peach_kamehameha_charge, peach_kamehameha_charge_end,
        peach_kamehamehafire_pre, peach_kamehamehafire, peach_kamehamehafire_end);
    smashline::install_agent_frames!(peach_frame);
}