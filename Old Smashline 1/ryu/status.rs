use crate::imports::BuildImports::*;

pub const FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_START: i32 = 0x202;
pub const FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_CHARGE: i32 = 0x203;
pub const FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_FIRE: i32 = 0x204;
pub static mut CHARGE_TIME:[f32;8] = [0.0;8];


#[fighter_frame(agent = FIGHTER_KIND_RYU)]
pub fn ryu_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let entry_id = get_entry_id(fighter.module_accessor);
        if[FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_CHARGE, FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_START].contains(&status_kind) {
            CHARGE_TIME[entry_id] += 1.0;
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.3, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.6, 0.7);
        }
        if ![FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_START, FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_CHARGE, FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_FIRE]
            .contains(&status_kind) {
            CHARGE_TIME[entry_id] = 0.0;
        }
        if [
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD,
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_MISS_FOOT
        ].contains(&status_kind) {
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_genesis_beam"), false, false);
            STOP_SE(fighter, Hash40::new("se_item_genesis_shot02"));
        };
    }
}


#[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn ryu_specialn_pre(_fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CValue::I32(0)
}

#[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn ryu_specialn_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_START.into(), false.into());
    L2CValue::I32(0)
}

#[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn ryu_specialn_end(_fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CValue::I32(0)
}

//idk why I can't put in the constant as the status. Only the raw i32 or LuaConst works. Smashline issue
#[status_script(agent = "ryu", status = 0x202, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn ryu_kamehameha_start_pre(_fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CValue::I32(0)
}

#[status_script(agent = "ryu", status = 0x202, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn ryu_kamehameha_start(fighter: &mut L2CFighterCommon) -> L2CValue {
    change_motion(fighter.module_accessor, "kamehameha_start");
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_kamehameha_start_main as *const () as _))
}

unsafe extern "C" fn ryu_kamehameha_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = get_entry_id(fighter.module_accessor) as i32;
    set_position_lock(entry_id);
    if MotionModule::is_end(fighter.module_accessor){
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_CHARGE.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "ryu", status = 0x202, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn ryu_kamehameha_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = get_entry_id(fighter.module_accessor) as i32;
    unset_position_lock(entry_id);
    L2CValue::I32(0)
}

#[status_script(agent = "ryu", status = 0x203, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn ryu_kamehameha_charge_pre(_fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CValue::I32(0)
}

#[status_script(agent = "ryu", status = 0x203, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn ryu_kamehameha_charge(fighter: &mut L2CFighterCommon) -> L2CValue {
    change_motion(fighter.module_accessor, "kamehameha_charge");
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_kamehameha_charge_main as *const () as _))
}

unsafe extern "C" fn ryu_kamehameha_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = get_entry_id(fighter.module_accessor);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
        MotionModule::set_rate(fighter.module_accessor, 0.1);
        set_position_lock(entry_id as i32);
    }
    else{
        SoundModule::stop_all_sound(fighter.module_accessor);
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_FIRE.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "ryu", status = 0x203, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn ryu_kamehameha_charge_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = get_entry_id(fighter.module_accessor) as i32;
    unset_position_lock(entry_id);
    L2CValue::I32(0)
}

#[status_script(agent = "ryu", status = 0x204, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn ryu_kamehamehafire_pre(_fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CValue::I32(0)
}

#[status_script(agent = "ryu", status = 0x204, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn ryu_kamehamehafire(fighter: &mut L2CFighterCommon) -> L2CValue {
    change_motion(fighter.module_accessor, "kamehameha_fire");
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_kamehamehafire_main as *const () as _))
}

unsafe extern "C" fn ryu_kamehamehafire_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = get_entry_id(fighter.module_accessor);
    set_position_lock(entry_id as i32);
    if MotionModule::is_end(fighter.module_accessor){
        if is_grounded(fighter.module_accessor){
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "ryu", status = 0x204, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn ryu_kamehamehafire_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = get_entry_id(fighter.module_accessor) as i32;
    unset_position_lock(entry_id);
    L2CValue::I32(0)
}


pub fn install() {
    smashline::install_status_scripts!(
        ryu_specialn_pre, ryu_specialn_command, ryu_specialn_end,
        ryu_kamehameha_start_pre, ryu_kamehameha_start, ryu_kamehameha_start_end,
        ryu_kamehameha_charge_pre, ryu_kamehameha_charge, ryu_kamehameha_charge_end,
        ryu_kamehamehafire_pre, ryu_kamehamehafire, ryu_kamehamehafire_end);
    smashline::install_agent_frames!(ryu_frame);
}