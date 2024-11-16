use crate::imports::BuildImports::*;

unsafe extern "C" fn status_ninten_SpecialLwHold_CheckDamage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {
        HIT_NO(fighter, 11, *HIT_STATUS_OFF);
        SoundModule::play_se(fighter.module_accessor, Hash40::new("se_ness_special_l02"), true, false, false, false, enSEType(0));
        StopModule::set_hit_stop_frame(fighter.module_accessor, 5, true);
        DamageModule::end_damage_info_log(fighter.module_accessor);
        KineticModule::clear_speed_all(fighter.module_accessor);
        StatusModule::change_status_force(fighter.module_accessor, FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
        1.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_ninten_SpecialLwHold_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let NINTEN = color >= 120 && color <= 127;
	if NINTEN {
        DamageModule::set_damage_lock(fighter.module_accessor, true);
        let phy_magnet_offset_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("phy_magnet_offset_y"));
        let shield_pos = &Vector3f{x: 0.0, y: phy_magnet_offset_y - 4.0, z: 0.0};
        let time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("time"));
        let int_time = WorkModule::get_int(fighter.module_accessor,*FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_TIME);
        let ratio = 1.0;
        let vector_ratio = Vector3f{x: ratio, y: ratio, z:ratio};
        let effect =  WorkModule::get_int(fighter.module_accessor,*FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_EFFECT_HANDLE) as u32;
        EffectModule::set_scale(fighter.module_accessor, effect, &vector_ratio);
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &vector_ratio);  
        ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("throw"), shield_pos, false, false);  
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        && int_time <= time - 15 {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_TIME);
        }
        else if int_time <= 0 {
            fighter.change_status(FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
        }
        0.into()
    }
    else {
        0.into()
    }
}

pub fn install() {
    Agent::new("ness")
    .status(CheckDamage, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, status_ninten_SpecialLwHold_CheckDamage)
    .status(Exec, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, status_ninten_SpecialLwHold_Exec)
    .install();
}