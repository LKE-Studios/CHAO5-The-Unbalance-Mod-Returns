use crate::imports::BuildImports::*;

#[status_script(agent = "buddy", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_buddy_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_s").into());
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN) <= 0 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
    }
    fighter.sub_change_motion_by_situation(Hash40::new("special_s").into(), Hash40::new("special_air_s").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(true.into());
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);
    fighter.sub_shift_status_main(L2CValue::Ptr(buddy_special_s_loop as *const () as _))
}

unsafe fn buddy_special_s_super_armor_function(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP) {
            HitModule::set_total_status_disguise(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP) {
                HitModule::set_total_status_disguise(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);
            }
        }
    }
}

unsafe extern "C" fn buddy_special_s_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into()
    }
    buddy_special_s_super_armor_function(fighter);
    fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_s").into());
    if MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL) {
            fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL.into(), false.into())
        }
        else {
            fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH.into(), false.into())
        }
    }
    else {
        fighter.sub_change_motion_by_situation(Hash40::new("special_s").into(), Hash40::new("special_air_s").into(), true.into());
        fighter.sub_set_ground_correct_by_situation(true.into());
    }
    0.into()
}

#[status_script(agent = "buddy", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_buddy_special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP) {
            HitModule::set_total_status_disguise(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);
        }
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        status_buddy_special_s_main,
        status_buddy_special_s_end
    );
}