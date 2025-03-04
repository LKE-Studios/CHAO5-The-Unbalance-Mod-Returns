use crate::imports::BuildImports::*;

pub static mut GFX_COUNTER : [i32; 8] = [0; 8];
pub static mut SFX_COUNTER : [i32; 8] = [0; 8];
pub static mut COUNTER : [i32; 8] = [0; 8];
pub static mut CURRENT_ON_FRAME : [f32; 8] = [0.0; 8];
pub static mut IS_CRIT : [bool; 8] = [false; 8];
pub static VECTOR_ZERO : Vector3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
const COMMON_WEAPON_ATTACK_CALLBACK : usize = 0x33BDC30;
const ITEM_CREATOR : usize = 0x15DB0B0;
const NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x67A7B0;
const SPECIAL_ZOOM_CRITICAL_OFFSET : usize = 0x696720;

pub mod KineticUtility {
    // Resets and enables the kinetic energy type.
    // Unknown why there are two vectors required by reset_energy
    pub unsafe fn reset_enable_energy(module_accessor: *mut smash::app::BattleObjectModuleAccessor, energy_id: i32, energy_reset_type: i32, speed_vec: smash::phx::Vector2f, other_vec: smash::phx::Vector3f) {
        let energy = smash::app::lua_bind::KineticModule::get_energy(module_accessor, energy_id) as *mut smash::app::KineticEnergy;
        smash::app::lua_bind::KineticEnergy::reset_energy(energy, energy_reset_type, &speed_vec, &other_vec, module_accessor);
        smash::app::lua_bind::KineticEnergy::enable(energy);
    }
    // Clears and disables the kinetic energy type
    pub unsafe fn clear_unable_energy(module_accessor: *mut smash::app::BattleObjectModuleAccessor, energy_id: i32) {
        let energy = smash::app::lua_bind::KineticModule::get_energy(module_accessor, energy_id) as *mut smash::app::KineticEnergy;
        smash::app::lua_bind::KineticEnergy::clear_speed(energy);
        smash::app::lua_bind::KineticEnergy::unable(energy);
    }
}

//Calls the Special Zoom function
#[skyline::from_offset(SPECIAL_ZOOM_CRITICAL_OFFSET)]
pub fn special_zoom_critical(module_accessor: *mut smash::app::BattleObjectModuleAccessor, collision_log: u64, fighter_kind: i32, vl_params: u64, param_5: i32, param_6: i32, param_7: i32, param_8: i32, param_9: i32) -> u64;

pub unsafe fn common_attack_critical_flag(fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let special_zoom_effect = WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_EFFECT);
    let counter = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CRITICAL_COUNTER);
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        WorkModule::add_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_CRITICAL_COUNTER);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CHECK_CRITICAL);
        if counter < 2 {
            EffectModule::req_screen(fighter.module_accessor, Hash40::new("bg_criticalhit"), false, true, true);
            CURRENT_ON_FRAME[ENTRY_ID] = MotionModule::frame(fighter.module_accessor);
            SlowModule::set_whole(fighter.module_accessor, 2, 0);
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_criticalhit"), true, false, false, false, enSEType(0));
            QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
            let lr = PostureModule::lr(fighter.module_accessor);
            CAM_ZOOM_IN_arg5(fighter, /*frames*/ 4.0,/*no*/ 0.0,/*zoom*/ 2.1,/*yrot*/ 0.0,/*xrot*/ 0.0 * lr);
        }
    }
    if MotionModule::frame(fighter.module_accessor) >= (CURRENT_ON_FRAME[ENTRY_ID] + 1.0) 
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CHECK_CRITICAL) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_CRITICAL_COUNTER);
        SlowModule::clear_whole(fighter.module_accessor);
        CameraModule::reset_all(fighter.module_accessor);
        EffectModule::remove_screen(fighter.module_accessor, Hash40::new("bg_criticalhit"), 0);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        if StatusModule::status_kind(fighter.module_accessor) != 510 {
            CAM_ZOOM_OUT(fighter);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CHECK_CRITICAL) 
    && MotionModule::frame(fighter.module_accessor) < 2.0 {
        CAM_ZOOM_OUT(fighter);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CHECK_CRITICAL);
        EffectModule::remove_screen(fighter.module_accessor, Hash40::new("bg_criticalhit"), 0);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        SlowModule::clear_whole(fighter.module_accessor);
    };
}

pub unsafe fn gimmick_flash(fighter: &mut L2CFighterCommon) {
    let lr = PostureModule::lr(fighter.module_accessor);
    let offset = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);
    if !sv_information::is_ready_go() {
        return
    }
    FighterUtil::flash_eye_info(fighter.module_accessor);
    if WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("flip")) != 0 {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, offset, 2, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
    }
    else {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -5.0 * lr, offset, 2, 0, 0, 0, 1.0, true);
    }
    LAST_EFFECT_SET_COLOR(fighter, 0.831, 0.686, 0.216);
}

unsafe extern "C" fn start_special_flag_checks_Init(fighter: &mut L2CFighterCommon) {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if fighter_kind == *FIGHTER_KIND_DONKEY {
        fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(donkey_SpecialS_callback as *const () as _));
        fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(donkey_SpecialHi_callback as *const () as _));
        fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(donkey_SpecialLw_callback as *const () as _));
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(donkey_change_status_callback as *const () as _)); 
    }
    if fighter_kind == *FIGHTER_KIND_METAKNIGHT {
        fighter.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(metaknight_SpecialN_callback as *const () as _));
        fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(metaknight_SpecialS_callback as *const () as _));
        fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(metaknight_SpecialHi_callback as *const () as _));
        fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(metaknight_SpecialLw_callback as *const () as _));
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(metaknight_change_status_callback as *const () as _)); 
    }
    if fighter_kind == *FIGHTER_KIND_LUCARIO {
        fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(lucario_SpecialHi_callback as *const () as _));  
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(lucario_change_status_callback as *const () as _));   
    }
    if fighter_kind == *FIGHTER_KIND_SIMON || fighter_kind == *FIGHTER_KIND_RICHTER {
        fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(simon_SpecialHi_callback as *const () as _));  
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(simon_change_status_callback as *const () as _));   
    }
    if fighter_kind == *FIGHTER_KIND_TRAIL {
        fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(trail_SpecialS_callback as *const () as _));
        fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(trail_SpecialHi_callback as *const () as _));  
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(trail_change_status_callback as *const () as _));   
    }
}

unsafe extern "C" fn donkey_SpecialS_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FUNKY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn donkey_SpecialHi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FUNKY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn donkey_SpecialLw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FUNKY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn donkey_change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_AIR || conditional_statuses(status_kind) 
    || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI)
    || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
    || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
    || !sv_information::is_ready_go() {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FUNKY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FUNKY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FUNKY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW);
    }
    true.into()
}

unsafe extern "C" fn metaknight_SpecialN_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_N) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn metaknight_SpecialS_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn metaknight_SpecialHi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn metaknight_SpecialLw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn metaknight_change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_AIR || conditional_statuses(status_kind) 
    || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI)
    || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
    || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
    || !sv_information::is_ready_go() {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_N);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW);
    }
    true.into()
}


unsafe extern "C" fn lucario_SpecialHi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_ENABLE_SPECIAL_HI) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn lucario_change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_ENABLE_SPECIAL_HI);
    }
    true.into()
}

unsafe extern "C" fn simon_SpecialHi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn simon_change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_AIR || conditional_statuses(status_kind) 
    || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI)
    || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
    || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
    || !sv_information::is_ready_go() {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI);
    }
    true.into()
}

unsafe extern "C" fn trail_SpecialS_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn trail_SpecialHi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn trail_change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_AIR || conditional_statuses(status_kind) 
    || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI)
    || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
    || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
    || !sv_information::is_ready_go() {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    }
    true.into()
}

unsafe extern "C" fn basyaamo_init(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(basyaamo_check_special_command as *const () as _));
}

unsafe extern "C" fn basyaamo_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let BASYAAMO = color >= 120 && color <= 127;
    if !BASYAAMO || fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
        return false.into();
    }
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND) {
        fighter.change_status(FIGHTER_BASYAAMO_STATUS_KIND_SPECIAL_HI_OVERHEAT.into(), false.into());
        return true.into();
    }
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND) {
        fighter.change_status(FIGHTER_BASYAAMO_STATUS_KIND_SPECIAL_N_OVERHEAT.into(), false.into());
        return true.into();
    }
    false.into()
}

//Custom Fighter Functions

pub mod FighterSpecializer_MetaKnight {
    use crate::imports::BuildImports::*;
    pub unsafe fn meta_power(module_accessor: *mut smash::app::BattleObjectModuleAccessor) {
        let fighter = get_fighter_common_from_accessor(&mut *module_accessor);
        let status_kind = StatusModule::status_kind(module_accessor);
        let meta_power_damage = WorkModule::get_param_float(module_accessor, hash40("param_meta_power"), hash40("meta_power_damage"));
        let attack_mul = WorkModule::get_param_float(module_accessor, hash40("param_meta_power"), hash40("attack_mul"));
        let reaction_mul = WorkModule::get_param_float(module_accessor, hash40("param_meta_power"), hash40("reaction_mul"));
        let hit_damage_mul = WorkModule::get_param_float(module_accessor, hash40("param_meta_power"), hash40("hit_damage_mul"));
        let effect_scale = WorkModule::get_param_float(module_accessor, hash40("param_meta_power"), hash40("effect_scale"));
        if WorkModule::is_flag(module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_META_POWER) {
            let effect_counter = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
            let sound_counter = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SOUND_COUNTER);
            WorkModule::add_int(module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
            WorkModule::add_int(module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_SOUND_COUNTER);
            AttackModule::set_power_up(module_accessor, attack_mul);
            DamageModule::set_damage_mul_2nd(module_accessor, hit_damage_mul);
            DamageModule::set_reaction_mul(module_accessor, reaction_mul);
            if sound_counter == 1 {
                SoundModule::play_se(module_accessor, Hash40::new("se_metaknight_special_l01"), true, false, false, false, enSEType(0));
                SoundModule::play_se(module_accessor, Hash40::new("se_metaknight_final01"), true, false, false, false, enSEType(0));
                PLAY_SE_REMAIN(fighter, Hash40::new("vc_metaknight_final03"));
                println!("{}", sound_counter);
            }
            if effect_counter >= 6 {
                let effect_a = EffectModule::req_follow(module_accessor, Hash40::new("sys_hit_purple"), Hash40::new("haver"), &VECTOR_ZERO, &VECTOR_ZERO, effect_scale, true, 0, 0, 0, 0, 0, true, true);
                EffectModule::set_rgb(module_accessor, effect_a as u32, 0.68, 0.87, 2.0);
                let effect_b = EffectModule::req_follow(module_accessor, Hash40::new("sys_hit_purple"), Hash40::new("havel"), &VECTOR_ZERO, &VECTOR_ZERO, effect_scale, true, 0, 0, 0, 0, 0, true, true);
                EffectModule::set_rgb(module_accessor, effect_b as u32, 0.68, 0.87, 2.0);
                WorkModule::set_int(module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
            }
        }
        else {
            WorkModule::set_int(module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SOUND_COUNTER);
        }
        if DamageModule::damage(module_accessor, 0) >= meta_power_damage {
            WorkModule::on_flag(module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_META_POWER);
        }
        else if DamageModule::damage(module_accessor, 0) < meta_power_damage || [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_MISS_FOOT].contains(&status_kind) 
        || !sv_information::is_ready_go() {
            WorkModule::off_flag(module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_META_POWER);
            DamageModule::set_damage_mul_2nd(module_accessor, 1.0);
            DamageModule::set_reaction_mul(module_accessor, 1.0);
            AttackModule::set_power_up(module_accessor, 1.0);
        };     
        if [
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_MISS_FOOT,
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        ].contains(&status_kind) {
            SoundModule::stop_se(module_accessor, Hash40::new("vc_metaknight_final03"), 0);
        }
    }
}

pub mod FighterSpecializer_Palutena {
    use crate::imports::BuildImports::*;
    pub unsafe fn goddess_power_up (module_accessor: *mut smash::app::BattleObjectModuleAccessor) {
        let status_kind = StatusModule::status_kind(module_accessor);
        let attack_mul = WorkModule::get_param_float(module_accessor, hash40("param_divine_power_up"), hash40("attack_mul"));
        let reaction_mul = WorkModule::get_param_float(module_accessor, hash40("param_divine_power_up"), hash40("reaction_mul"));
        let hit_damage_mul = WorkModule::get_param_float(module_accessor, hash40("param_divine_power_up"), hash40("hit_damage_mul"));
        if WorkModule::is_flag(module_accessor, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_DIVINE_POWER) {
            let effect_counter = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
            WorkModule::add_int(module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
            DamageModule::set_damage_mul_2nd(module_accessor, hit_damage_mul);
            DamageModule::set_reaction_mul(module_accessor, reaction_mul);
            AttackModule::set_power_up(module_accessor, attack_mul);
            if effect_counter >= 20 {
                EffectModule::kill_kind(module_accessor, Hash40::new("sys_aura_light"), false, false);
                let effect = EffectModule::req_follow(module_accessor, Hash40::new("sys_aura_light"), Hash40::new("waist"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 5.0, true, 0, 0, 0, 0, 0, true, true);
                EffectModule::set_rgb(module_accessor, effect as u32, 0.0, 2.55, 0.48);
                WorkModule::set_int(module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
            };
        };
        if status_kind == *FIGHTER_STATUS_KIND_DEAD || status_kind == *FIGHTER_STATUS_KIND_MISS_FOOT 
        || !sv_information::is_ready_go() {
            WorkModule::off_flag(module_accessor, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_DIVINE_POWER);
            AttackModule::set_power_up(module_accessor, 1.0);
            DamageModule::set_damage_mul_2nd(module_accessor, 1.0);
            DamageModule::set_reaction_mul(module_accessor, 1.0);
            EffectModule::kill_kind(module_accessor, Hash40::new("sys_aura_light"), false, false);
        };
    }
}

pub mod FighterSpecializer_Ridley {
    extern "C" {
        #[link_name = "_ZN3app25FighterSpecializer_Ridley30request_special_hi_wall_effectERNS_21FighterModuleAccessorE"]
        pub fn request_special_hi_wall_effect(module_accessor: *mut smash::app::FighterModuleAccessor);
    }
}

pub mod FighterSpecializer_Murabito {
    extern "C" {
        #[link_name = "_ZN3app27FighterSpecializer_Murabito22check_special_lw_plantERNS_21FighterModuleAccessorEN3phx8Vector2fEf"]
        pub fn check_special_lw_plant(module_accessor: *mut smash::app::FighterModuleAccessor, arg1: smash::phx::Vector2f, arg2: f32) -> bool;
    }
}

extern "C" {
    #[link_name = "_ZN3app6camera13get_dead_areaEv"]
    pub fn get_dead_area() -> Rect;
    #[link_name = "_ZN3app6camera16get_camera_rangeEv"]
    pub fn get_camera_range() -> Rect;
}
#[repr(simd)]
#[derive(Debug)]
pub struct Rect {
    //left: f32,
    //right: f32,
    //up: f32,
    //down: f32,
    pub vec: [f32; 4]
}

impl Rect {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        (self.vec[0] <= x && x <= self.vec[1]) && (self.vec[3] <= y && y <= self.vec[2])
    }
    pub fn left(&self) -> f32 {return self.vec[0];}
    pub fn right(&self) -> f32 {return self.vec[1];}
    pub fn up(&self) -> f32 {return self.vec[2];}
    pub fn down(&self) -> f32 {return self.vec[3];}
}

pub unsafe extern "C" fn is_cloned_article(object_boma: *mut smash::app::BattleObjectModuleAccessor) -> bool {
    let color = WorkModule::get_int(object_boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let ADDED_FIGHTER = color >= 120 && color <= 130;
    let ADDED_FIGHTER_2 = color >= 64 && color <= 71;
    let ADDED_FIGHTER_3 = color >= 96 && color <= 103;
    if utility::get_kind(&mut *object_boma) == *WEAPON_KIND_SHEIK_NEEDLE 
    || utility::get_kind(&mut *object_boma) == *WEAPON_KIND_LINK_BOOMERANG 
    || utility::get_kind(&mut *object_boma) == *WEAPON_KIND_PITB_BOWARROW 
    || utility::get_kind(&mut *object_boma) == *WEAPON_KIND_MARIO_FIREBALL
    || utility::get_kind(&mut *object_boma) == *WEAPON_KIND_LUIGI_FIREBALL
    || utility::get_kind(&mut *object_boma) == *WEAPON_KIND_KIRBY_FINALCUTTERSHOT
    || utility::get_kind(&mut *object_boma) == *WEAPON_KIND_ROCKMAN_HARDKNUCKLE
    || utility::get_kind(&mut *object_boma) == *WEAPON_KIND_KOOPAJR_CANNONBALL {
        let owner_id = WorkModule::get_int(object_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let owner_boma = sv_battle_object::module_accessor(owner_id);
        let owner_kind = utility::get_kind(&mut *owner_boma);
        if owner_kind == *FIGHTER_KIND_PLIZARDON {
            return true;
        }
        if owner_kind == *FIGHTER_KIND_METAKNIGHT {
            return true;
        }
        if owner_kind == *FIGHTER_KIND_PITB && ADDED_FIGHTER_2 { //KRYSTAL
            return true;
        }
        if owner_kind == *FIGHTER_KIND_DOLLY && ADDED_FIGHTER { //WALUIGI
            return true;
        }
        if owner_kind == *FIGHTER_KIND_NESS && ADDED_FIGHTER_3 { //KAMEK
            return true;
        }
        if owner_kind == *FIGHTER_KIND_DONKEY && ADDED_FIGHTER { //FUNKY
            return true;
        }
        if owner_kind == *FIGHTER_KIND_PALUTENA && ADDED_FIGHTER { //SANS
            return true;
        }
    }
    return false;
}

pub unsafe extern "C" fn ac_update(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH {
        let object_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID) as u32;
        if object_id == 0 || object_id == *BATTLE_OBJECT_ID_INVALID as u32 {return;}
        let object_boma = sv_battle_object::module_accessor(object_id);
        if is_cloned_article(object_boma) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE, false);
            WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);
            let weapon = get_fighter_common_from_accessor(&mut *object_boma);
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            let pos = *PostureModule::pos(object_boma);
            EffectModule::req(object_boma, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y + 2.0, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0.625, 0, -1, false, 0);
        }
    }
}

pub unsafe extern "C" fn set_arrow_fuse_params(module_accessor: *mut smash::app::BattleObjectModuleAccessor, item_kind: i32, fuse_kind: i32, trait_type: i32) {
    if (![*ITEM_KIND_NONE, *ITEM_KIND_ASSIST, *ITEM_KIND_LINKARROW].contains(&item_kind) && ![*ITEM_TRAIT_FLAG_NONE, *ITEM_TRAIT_FLAG_SHOOT, *ITEM_TRAIT_FLAG_SWING].contains(&trait_type)) 
    || [*ITEM_KIND_BANANAGUN, *ITEM_KIND_FIREFLOWER].contains(&item_kind) {
        WorkModule::on_flag(module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    else {
        WorkModule::off_flag(module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    if WorkModule::is_flag(module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let owner_kind = smash::app::utility::get_kind(&mut *owner_module_accessor);
        WorkModule::set_int(module_accessor, item_kind, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
        if fuse_kind == FuseKind::FUSE {
            if owner_kind == *FIGHTER_KIND_LINK {
                WorkModule::set_int(owner_module_accessor, item_kind, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
            }
            else if owner_kind == *FIGHTER_KIND_KIRBY {
                WorkModule::set_int(owner_module_accessor, item_kind, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
            }
            let item_id = ItemModule::get_have_item_id(owner_module_accessor, 0) as i32;
            WorkModule::set_int(module_accessor, item_id, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
        }
        else if fuse_kind == FuseKind::REFUSE {
            let pos_x = PostureModule::pos_x(module_accessor);
            let pos_y = PostureModule::pos_y(module_accessor);
            let pos_z = PostureModule::pos_z(module_accessor);
            let mut params = CreateItemParam {
                founder_pos: smash::Vector4f{x: pos_x, y: pos_y, z: pos_z, w: 0.0},
                item_pos: smash::Vector4f{x: pos_x, y: pos_y, z: pos_z, w: 0.0},
                item_kind: smash::app::ItemKind(item_kind),
                another_battle_object_id: *BATTLE_OBJECT_ID_INVALID as u32,
                variation_kind: *ITEM_VARIATION_NONE,
                lr_dir: PostureModule::lr(module_accessor),
                owner_id: (*(module_accessor)).battle_object_id,
                unk_20: 20,
                pokeball_or_assist_kind: *ITEM_KIND_NONE,
                unk_0: 0,
                weird_flag: 0x633F800000,
                unk_1_weird: 1,
                unk_approx_0: 0.0,
                unk_02: 0.0
            };
            let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
            let battle_object = create_item(item_manager, &mut params, false, false, false);
            let item_module_accessor = (*battle_object).module_accessor;
            if ![*ITEM_KIND_HEALBALL, *ITEM_KIND_CHEWING, *ITEM_KIND_BOOMERANG].contains(&item_kind) {
                StatusModule::change_status_request(item_module_accessor, *ITEM_STATUS_KIND_HAVE, false);
            }
            if item_kind == *ITEM_KIND_LINKBOMB {
                PostureModule::set_scale(item_module_accessor, 1.35, false);
            }
            let item_id = (*(item_module_accessor)).battle_object_id as i32;
            WorkModule::set_int(module_accessor, item_id, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
            if [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&owner_kind) {
                WorkModule::set_int(owner_module_accessor, *ITEM_KIND_NONE, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
            }
        }
        if item_kind == *ITEM_KIND_BOMBER {
            WorkModule::set_int(module_accessor, FuseType::NORMAL, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(module_accessor, *ITEM_BOMBER_STATUS_KIND_BORN2, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if [*ITEM_KIND_KILLER, *ITEM_KIND_BANANAGUN, *ITEM_KIND_DOLPHINBOMB].contains(&item_kind) {
            WorkModule::set_int(module_accessor, FuseType::POWER, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(module_accessor, *ITEM_STATUS_KIND_THROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_FIREFLOWER {
            WorkModule::set_int(module_accessor, FuseType::ELEMENTAL, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(module_accessor, *ITEM_STATUS_KIND_LOST, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_LINKBOMB {
            WorkModule::set_int(module_accessor, FuseType::NORMAL, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(module_accessor, *ITEM_STATUS_KIND_BORN, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else {
            WorkModule::set_int(module_accessor, FuseType::NORMAL, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(module_accessor, *ITEM_STATUS_KIND_THROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
    }
}

pub unsafe extern "C" fn set_elemental_fuse(weapon: &mut L2CFighterBase, element: i32, fuse_type: i32, end_status: i32) {
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_kind = smash::app::utility::get_kind(&mut *owner_module_accessor);
    if owner_kind == *FIGHTER_KIND_LINK {
        WorkModule::set_int(owner_module_accessor, element, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
    }
    else {
        WorkModule::set_int(owner_module_accessor, element, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
    }
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    let pos_z = PostureModule::pos_z(weapon.module_accessor);
    let mut params = CreateItemParam {
        founder_pos: smash::Vector4f{x: pos_x, y: pos_y, z: pos_z, w: 0.0},
        item_pos: smash::Vector4f{x: pos_x, y: pos_y, z: pos_z, w: 0.0},
        item_kind: smash::app::ItemKind(element),
        another_battle_object_id: *BATTLE_OBJECT_ID_INVALID as u32,
        variation_kind: *ITEM_VARIATION_NONE,
        lr_dir: PostureModule::lr(owner_module_accessor),
        owner_id: (*(owner_module_accessor)).battle_object_id,
        unk_20: 20,
        pokeball_or_assist_kind: *ITEM_KIND_NONE,
        unk_0: 0,
        weird_flag: 0x633F800000,
        unk_1_weird: 1,
        unk_approx_0: 0.0,
        unk_02: 0.0
    };
    let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
    let battle_object = create_item(item_manager, &mut params, false, false, false);
    WorkModule::set_int(weapon.module_accessor, element, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
    WorkModule::set_int(weapon.module_accessor, fuse_type, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
    WorkModule::set_int(weapon.module_accessor, end_status, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
    let item_module_accessor = (*battle_object).module_accessor;
    let item_id = (*item_module_accessor).battle_object_id;
    WorkModule::set_int64(weapon.module_accessor, item_id as i64, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
    StatusModule::change_status_request(item_module_accessor, *ITEM_STATUS_KIND_THROW, false);
    LinkModule::remove_model_constraint(item_module_accessor, true);
    if LinkModule::is_link(item_module_accessor, *ITEM_LINK_NO_HAVE) {
        LinkModule::unlink(item_module_accessor, *ITEM_LINK_NO_HAVE);
    }
    if !LinkModule::is_link(item_module_accessor, *ITEM_LINK_NO_HAVE) {
        VisibilityModule::set_whole(item_module_accessor, true);
        LinkModule::link(item_module_accessor, *ITEM_LINK_NO_HAVE, (*(weapon.module_accessor)).battle_object_id);
        LinkModule::set_model_constraint_pos_ort(item_module_accessor, *ITEM_LINK_NO_HAVE, Hash40::new("top"), Hash40::new("top"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, true);
    }
    WorkModule::on_flag(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
}

pub unsafe extern "C" fn set_boomerang_fuse_params(module_accessor: *mut smash::app::BattleObjectModuleAccessor, item_kind: i32, fuse_kind: i32, trait_type: i32) {
    if (![*ITEM_KIND_NONE,*ITEM_KIND_ASSIST,*ITEM_KIND_LINKARROW].contains(&item_kind) && ![*ITEM_TRAIT_FLAG_NONE,*ITEM_TRAIT_FLAG_SHOOT,*ITEM_TRAIT_FLAG_SWING].contains(&trait_type))
    || [*ITEM_KIND_FIREFLOWER].contains(&item_kind) {
        WorkModule::on_flag(module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    else {
        WorkModule::off_flag(module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    if WorkModule::is_flag(module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        WorkModule::set_int(module_accessor, item_kind, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
        let owner_kind = smash::app::utility::get_kind(&mut *owner_module_accessor);
        if fuse_kind == FuseKind::FUSE {
            WorkModule::set_int(owner_module_accessor,item_kind, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
            let item_id = ItemModule::get_have_item_id(owner_module_accessor, 0) as i32;
            WorkModule::set_int(module_accessor,item_id, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
            WorkModule::set_int(owner_module_accessor,item_id, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
        }
        else if fuse_kind == FuseKind::REFUSE {
            let mut params = CreateItemParam {
                founder_pos: smash::Vector4f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor), w: 0.0},
                item_pos: smash::Vector4f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor), w: 0.0},
                item_kind: smash::app::ItemKind(item_kind),
                another_battle_object_id: *BATTLE_OBJECT_ID_INVALID as u32,
                variation_kind: *ITEM_VARIATION_NONE,
                lr_dir: PostureModule::lr(module_accessor),
                owner_id: (*(module_accessor)).battle_object_id,
                unk_20: 20,
                pokeball_or_assist_kind: *ITEM_KIND_NONE,
                unk_0: 0,
                weird_flag: 0x633F800000,
                unk_1_weird: 1,
                unk_approx_0: 0.0,
                unk_02: 0.0
            };
            let item_manager = *(ItemManager() as *mut *mut smash::app::ItemManager);
            let battle_object = create_item(item_manager,&mut params,false,false,false);
            let item_module_accessor = (*battle_object).module_accessor;
            if ![*ITEM_KIND_HEALBALL, *ITEM_KIND_CHEWING, *ITEM_KIND_BOOMERANG].contains(&item_kind) {
                StatusModule::change_status_request(item_module_accessor, *ITEM_STATUS_KIND_HAVE,false);
            }
            if item_kind == *ITEM_KIND_LINKBOMB {
                PostureModule::set_scale(item_module_accessor, 1.3, false);
            }
            let item_id = (*(item_module_accessor)).battle_object_id as i32;
            WorkModule::set_int(module_accessor, item_id, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
            WorkModule::set_int(owner_module_accessor, item_id, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM_ID);
            if owner_kind == *FIGHTER_KIND_MURABITO
            || owner_kind == *FIGHTER_KIND_SHIZUE {
                WorkModule::set_int(owner_module_accessor, *ITEM_KIND_NONE,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM);
            }
        }
        if item_kind == *ITEM_KIND_BOMBER {
            WorkModule::set_int(module_accessor, *ITEM_BOMBER_STATUS_KIND_BORN2, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_FIREFLOWER {
            WorkModule::set_int(module_accessor, *ITEM_STATUS_KIND_LOST, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_LINKBOMB {
            WorkModule::set_int(module_accessor, *ITEM_STATUS_KIND_BORN, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else {
            WorkModule::set_int(module_accessor, *ITEM_STATUS_KIND_THROW, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
    }
}

//A hook regarding the generation/visiblity of articles. Used to allow entry articles to generate
/*#[skyline::hook(offset = 0x3a6670)]
unsafe fn get_article_use_type_mask(weapon_kind: i32, entry_id: i32) -> u8 {
    if weapon_kind == *WEAPON_KIND_LINK_PARASAIL {
        return 1;
    }
    original!()(weapon_kind, entry_id)
}*/

#[skyline::hook(offset = ITEM_CREATOR)]
pub unsafe fn create_item(item_manager: *mut smash::app::ItemManager, create_item_param: *mut CreateItemParam, param_3: bool, param_4: bool, param_5: bool) -> *mut BattleObject {
    if (*create_item_param).variation_kind > 7 {
        (*create_item_param).variation_kind = 0;
    }
    original!()(item_manager, create_item_param, param_3, param_4, param_5)
}

//VTable hook to disable the critical zoom-in on Neutral B
#[skyline::hook(offset = 0x8b8b90)]
unsafe extern "C" fn captain_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let module_accessor = fighter.battle_object.module_accessor;
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let BASYAAMO = color >= 120 && color <= 127;
    if BASYAAMO {
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_N ||
        (StatusModule::status_kind(module_accessor) == FIGHTER_BASYAAMO_STATUS_KIND_SPECIAL_N_OVERHEAT && MotionModule::frame(module_accessor) <= 40.0) {
            return 0;
        }
        if StatusModule::status_kind(module_accessor) == FIGHTER_BASYAAMO_STATUS_KIND_SPECIAL_N_OVERHEAT && MotionModule::frame(module_accessor) > 40.0 {
            special_zoom_critical(module_accessor, log, *FIGHTER_KIND_CAPTAIN, hash40("param_SpecialN"), 1, 0, 0, 0, 0);
            return 0;
        }
    }
    call_original!(vtable, fighter, log)
}

#[skyline::hook(offset = COMMON_WEAPON_ATTACK_CALLBACK)]
unsafe extern "C" fn common_weapon_attack_callback(vtable: u64, weapon: *mut smash::app::Weapon, log: u32) {
    let module_accessor = (*weapon).battle_object.module_accessor;
    let owner_object_id = WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor(owner_object_id as u32);
    let owner_kind = utility::get_kind(&mut *owner_module_accessor);
    let collision_log = *(log as *const u64).add(0x10/0x8);
    let collision_log = collision_log as *const CollisionLog;
    let opponent_id = (*collision_log).collider_id;
    let color = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let CUSTOM_FIGHTER_2 = color >= 64 && color <= 71;
    let CUSTOM_FIGHTER_3 = color >= 96 && color <= 103;
    let CUSTOM_FIGHTER = color >= 120 && color <= 127;
    if (*weapon).battle_object.kind == *WEAPON_KIND_KOOPAG_BREATH as u32 { 
        if owner_kind == *FIGHTER_KIND_KOOPAG {
            SoundModule::play_se(module_accessor, Hash40::new("se_koopag_fireball_impact01"), true, false, false, false, enSEType(0));
            *(weapon as *mut bool).add(0x90) = true;
        }
    }
    if (*weapon).battle_object.kind == *WEAPON_KIND_LUIGI_FIREBALL as u32 { 
        if CUSTOM_FIGHTER && owner_kind == *FIGHTER_KIND_MEWTWO {
            *(weapon as *mut bool).add(0x90) = true;
        }
    }
    if (*weapon).battle_object.kind == *WEAPON_KIND_KOOPAJR_CANNONBALL as u32 && CUSTOM_FIGHTER_3 && owner_kind == *FIGHTER_KIND_NESS {
        *(weapon as *mut bool).add(0x90) = true;
    }
    if (*weapon).battle_object.kind == *WEAPON_KIND_MARIOD_DRCAPSULE as u32 { 
        if CUSTOM_FIGHTER && owner_kind == *FIGHTER_KIND_PALUTENA {
            SoundModule::play_se(module_accessor, Hash40::new("se_palutena_attackair_b01"), true, false, false, false, enSEType(0));
            *(weapon as *mut bool).add(0x90) = true;
        }
    }
    if (*weapon).battle_object.kind == *WEAPON_KIND_ROCKMAN_HARDKNUCKLE as u32 { 
        if CUSTOM_FIGHTER && owner_kind == *FIGHTER_KIND_PALUTENA {
            SoundModule::play_se(module_accessor, Hash40::new("se_palutena_attackair_h01"), true, false, false, false, enSEType(0));
        }
    }
    if (*weapon).battle_object.kind == *WEAPON_KIND_ROCKMAN_ROCKBUSTER as u32 && owner_kind == *FIGHTER_KIND_PALUTENA {
        *(weapon as *mut bool).add(0x90) = true;
    }
    call_original!(vtable, weapon, log)
}

pub(crate) unsafe fn PLAY_ATTACK_VC(fighter: &mut L2CAgentBase) -> () {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let ADDED_FIGHTER = color >= 120 && color <= 130;
    let ADDED_FIGHTER_2 = color >= 64 && color <= 71;
    let ADDED_FIGHTER_3 = color >= 96 && color <= 103;
    if fighter_kind == *FIGHTER_KIND_NESS && ADDED_FIGHTER_3 {
        let rand_val = sv_math::rand(hash40("fighter"), 12);
        match rand_val {
            0 => PLAY_SE(fighter, Hash40::new("vc_ness_attack01")),
            1 => PLAY_SE(fighter, Hash40::new("vc_ness_attack02")),
            2 => PLAY_SE(fighter, Hash40::new("vc_ness_attack03")),
            3 => PLAY_SE(fighter, Hash40::new("vc_ness_attack04")),
            4 => PLAY_SE(fighter, Hash40::new("vc_ness_attack05")),
            5 => PLAY_SE(fighter, Hash40::new("vc_ness_attack06")),
            6 => PLAY_SE(fighter, Hash40::new("vc_ness_attack07")),
            _ => println!("Kamek is silent"),
        }
    }
    if fighter_kind == *FIGHTER_KIND_MEWTWO && ADDED_FIGHTER {
        let rand_val = sv_math::rand(hash40("fighter"), 12);
        match rand_val {
            0 => PLAY_SE(fighter, Hash40::new("vc_mewtwo_attack01")),
            1 => PLAY_SE(fighter, Hash40::new("vc_mewtwo_attack02")),
            2 => PLAY_SE(fighter, Hash40::new("vc_mewtwo_attack03")),
            3 => PLAY_SE(fighter, Hash40::new("vc_mewtwo_attack04")),
            4 => PLAY_SE(fighter, Hash40::new("vc_mewtwo_attack05")),
            5 => PLAY_SE(fighter, Hash40::new("vc_silver_attack06")),
            _ => println!("Silver is silent"),
        }
    }
    if fighter_kind == *FIGHTER_KIND_SHIZUE {
        let rand_val = sv_math::rand(hash40("fighter"), 8);
        let sound = match rand_val {
            0 => "vc_shizue_attack01",
            1 => "vc_shizue_attack02",
            2 => "vc_shizue_attack03",
            3 => "vc_shizue_attack04",
            4 => "vc_shizue_attack05",
            _ => "null",
        };
        let se_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new(sound), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, se_handle as i32, 0.7, 0);
    }
    if fighter_kind == *FIGHTER_KIND_KOOPAG {
        let rand_val = sv_math::rand(hash40("fighter"), 10);
        match rand_val {
            0 => PLAY_SE(fighter, Hash40::new("vc_koopag_attack01")),
            1 => PLAY_SE(fighter, Hash40::new("vc_koopag_attack02")),
            2 => PLAY_SE(fighter, Hash40::new("vc_koopag_attack03")),
            3 => PLAY_SE(fighter, Hash40::new("vc_koopag_attack04")),
            4 => PLAY_SE(fighter, Hash40::new("vc_koopag_attack05")),
            _ => println!("KoopaG is silent"),
        }
    }
}

pub(crate) unsafe fn PLAY_ATTACK_HEAVY_VC(fighter: &mut L2CAgentBase) -> () {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind == *FIGHTER_KIND_SHIZUE {
        let rand_val = sv_math::rand(hash40("fighter"), 7);
        let sound = match rand_val {
            0 => "vc_shizue_attack_heavy01",
            1 => "vc_shizue_attack_heavy02",
            2 => "vc_shizue_attack_heavy03",
            3 => "vc_shizue_attack_heavy04",
            _ => "null",
        };
        let se_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new(sound), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, se_handle as i32, 0.7, 0);
    }
}

pub(crate) unsafe fn PLAY_DAMAGE_VC(fighter: &mut L2CAgentBase) -> () {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind == *FIGHTER_KIND_SHIZUE {
        let rand_val = sv_math::rand(hash40("fighter"), 4);
        let sound = match rand_val {
            0 => "vc_shizue_damage01",
            1 => "vc_shizue_damage02",
            _ => "vc_shizue_damage03",
        };
        let se_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new(sound), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, se_handle as i32, 0.7, 0);
    }
    if fighter_kind == *FIGHTER_KIND_KOOPAG {
        let rand_val = sv_math::rand(hash40("fighter"), 4);
        let sound = match rand_val {
            0 => "vc_koopag_damage01",
            _ => "vc_koopag_damage02",
        };
        let se_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new(sound), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, se_handle as i32, 1.0, 0);
    }
}

pub(crate) unsafe fn PLAY_DAMAGEFLY_VC(fighter: &mut L2CAgentBase) -> () {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind == *FIGHTER_KIND_SHIZUE {
        let rand_val = sv_math::rand(hash40("fighter"), 3);
        let sound = match rand_val {
            0 => "vc_shizue_damagefly01",
            _ => "vc_shizue_damagefly02",
        };
        let se_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new(sound), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, se_handle as i32, 0.7, 0);
    }
    if fighter_kind == *FIGHTER_KIND_KOOPAG {
        let rand_val = sv_math::rand(hash40("fighter"), 3);
        let sound = match rand_val {
            0 => "vc_koopag_damagefly01",
            _ => "vc_koopag_damagefly02",
        };
        let se_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new(sound), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, se_handle as i32, 1.0, 0);
    }
}

pub unsafe fn get_table_value(table: *mut smash2::lib::L2CTable, key: &str) -> smash2::lib::L2CValue {
    let hash = if key.starts_with("0x") {
        smash2::phx::Hash40::from_hex_str(key).unwrap()
    } 
    else {
        smash2::phx::hash40(key)
    };
    (*table).get_map(hash).unwrap().clone()
}

extern "C" {
    #[link_name = "_ZN3lib9SingletonIN3app11ItemManagerEE9instance_E"]
    pub static ITEM_MANAGER: *mut smash::app::ItemManager;
}

#[skyline::hook(replace = WorkModule::is_enable_transition_term)]
pub unsafe fn disable_final_smash(module_accessor: &mut smash::app::BattleObjectModuleAccessor, term: i32) -> bool {
	let fighter_kind = smash::app::utility::get_kind(module_accessor);
	let ret = original!()(module_accessor, term);
	if fighter_kind == *FIGHTER_KIND_KOOPAG {
		if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL {
			return false;
		}
	}
	return ret;
}

#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(fighter_manager: *mut smash::app::FighterManager, attacker_object_id: u32, defender_object_id: u32, move_type: f32, arg5: i32, move_type_again: bool) -> u64 {
    let attacker_module_accessor = sv_battle_object::module_accessor(attacker_object_id);
    let defender_module_accessor = sv_battle_object::module_accessor(defender_object_id);
    let attacker_fighter_kind = sv_battle_object::kind(attacker_object_id);
    let defender_fighter_kind = sv_battle_object::kind(defender_object_id);
    let color = WorkModule::get_int(attacker_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let spell_type = WorkModule::get_int(attacker_module_accessor, *FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
    let CUSTOM_FIGHTER = color >= 64 && color <= 71;
    let CUSTOM_FIGHTER_2 = color >= 120 && color <= 127;
    let CUSTOM_FIGHTER_3 = color >= 96 && color <= 103;
    if attacker_fighter_kind == *FIGHTER_KIND_NESS && CUSTOM_FIGHTER_3 {
        if StatusModule::status_kind(attacker_module_accessor) == *FIGHTER_KAMEK_STATUS_KIND_SPECIAL_S {
            if spell_type == 8 {
                if AttackModule::is_infliction(attacker_module_accessor, *COLLISION_KIND_MASK_HIT) {
                    SlowModule::set(defender_module_accessor, 0, 25, 720, true, *FIGHTER_SLOW_KIND_NORMAL as u32);
                    EffectModule::req_time_follow(defender_module_accessor, Hash40::new("sys_timer"), Hash40::new("hip"), 720, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, false, 0);
                    SoundModule::play_se(defender_module_accessor, Hash40::new("se_timer_slow_all"), true, false, false, false, enSEType(0));
                }
            }
        }
    }
    let num_players = Fighter::get_fighter_entry_count();
    if attacker_fighter_kind == *FIGHTER_KIND_TRAIL {
        let motion_kind = MotionModule::motion_kind(attacker_module_accessor);
        if motion_kind == hash40("appeal_hi_l") || motion_kind == hash40("appeal_hi_r") { 
            if WorkModule::get_int(attacker_module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_APPEAL_HI_KIND) == 2 {
                if AttackModule::is_infliction(attacker_module_accessor, *COLLISION_KIND_MASK_HIT) {
                    for i in 1..num_players {
                        let opponent_module_accessor = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(i)); //All Opponents
                        SlowModule::set(opponent_module_accessor, 0, 20, 300, true, *FIGHTER_SLOW_KIND_NORMAL as u32);
                        EffectModule::req_time_follow(opponent_module_accessor, Hash40::new("sys_timer"), Hash40::new("hip"), 300, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, false, 0);
                        SoundModule::play_se(opponent_module_accessor, Hash40::new("se_timer_slow_all"), true, false, false, false, enSEType(0));
                    }
                }
            }
        }
    };
    original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

#[skyline::hook(offset = 0x1798ac8, inline)]
unsafe fn fix_chara_replace(ctx: &skyline::hooks::InlineCtx) {
    let ptr1 = *ctx.registers[0].x.as_ref() as *mut u64;
    let ptr2 = *ctx.registers[1].x.as_ref() as *mut u64;
    *ptr2.add(0x2) = *ptr1.add(0x2);
    *ptr2.add(0x3) = *ptr1.add(0x3);
    *ptr2.add(0x4) = *ptr1.add(0x4);
}

#[skyline::from_offset(0x3ac560)]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

pub mod CustomModule {
    use super::*;

    // A shortcut to reset i32 variables to 0.
    pub unsafe fn reset_i32(module_accessor: *mut smash::app::BattleObjectModuleAccessor, flag: i32) {
        WorkModule::set_int(module_accessor, 0, flag);
    }

    // A shortcut to reset f32 variables to 0.
    pub unsafe fn reset_f32(module_accessor: *mut smash::app::BattleObjectModuleAccessor, flag: i32) {
        WorkModule::set_float(module_accessor, 0.0, flag);
    }

    // A shortcut to add a value to an i32 variable.
    pub unsafe fn add_i32(module_accessor: *mut smash::app::BattleObjectModuleAccessor, flag: i32, amount: i32) {
        let counter = WorkModule::get_int(module_accessor, flag) + amount;
        WorkModule::set_int(module_accessor, counter, flag);
    }

    // A shortcut to add a value to an f32 variable.
    pub unsafe fn add_f32(module_accessor: *mut smash::app::BattleObjectModuleAccessor, flag: i32, amount: f32) {
        let counter = WorkModule::get_float(module_accessor, flag) + amount;
        WorkModule::set_float(module_accessor, counter, flag);
    }

    // A function for incrementing an f32 variable by an amount.
    // This function takes into account the effects of slowdown, such as from
    // Bayonetta's Witch Time or from the Timer item.
    pub unsafe fn count_down(module_accessor: *mut smash::app::BattleObjectModuleAccessor, flag: i32, amount: f32) {
        let slow_rate = SlowModule::rate(module_accessor);
        let global_slow_rate = sv_information::slow_rate();
        let counter = WorkModule::get_float(module_accessor, flag) - (amount * slow_rate * global_slow_rate);
        WorkModule::set_float(module_accessor, counter, flag);
    }

    pub unsafe fn is_operation_cpu(module_accessor: *mut smash::app::BattleObjectModuleAccessor) -> bool {
        if utility::get_category(&mut *module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
            return false;
        }
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32;
        let fighterentryid = smash::app::FighterEntryID(entry_id);
        let fighterinformation = smash::app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), fighterentryid);
        smash::app::lua_bind::FighterInformation::is_operation_cpu(fighterinformation)
    }

    pub unsafe extern "C" fn play_dedede_bgm(module_accessor: *mut smash::app::BattleObjectModuleAccessor) -> i32 {
        let fighter_kind = smash::app::utility::get_kind(&mut *module_accessor);
        let ui_sound_mgr = ((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 + 0x5329f38) as *const u64).read();
        let song = hash40("f43_dedede_waddledeearmy");
        play_bgm_override(ui_sound_mgr, song, -1)
    }

    pub unsafe extern "C" fn stop_dedede_bgm(module_accessor: *mut smash::app::BattleObjectModuleAccessor) {
        let fighter_kind = smash::app::utility::get_kind(&mut *module_accessor);
        let dedede_id = WorkModule::get_int(module_accessor, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_BGM_ID);
        let bgm_index = dedede_id;
        stop_status_bgm(((module_accessor as u64) + 0x148) as *const u64, bgm_index);
    }

    pub unsafe extern "C" fn get_article_module_accessor(modules: *mut smash::app::BattleObjectModuleAccessor, article_type: skyline::libc::c_int) -> *mut smash::app::BattleObjectModuleAccessor {
        let article = ArticleModule::get_article(modules, article_type);
        let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        return sv_battle_object::module_accessor(object_id);
    }
}

// 1: UiSoundManager(?) | 2: hash40 of song name | 3: priority - lowest value wins
// returns: index to be used in the stop function
#[skyline::from_offset(0x23edd60)]
pub unsafe extern "C" fn play_bgm_override(param1: u64, param2: u64, priority: i32) -> i32;

// sound_module: agent.module_accessor + 0x148
// param2: the index received from play_bgm_override earlier
#[skyline::from_offset(0x6e63d0)]
pub unsafe extern "C" fn stop_status_bgm(sound_module: *const u64, param2: i32);

//Sound pitch change values
pub static semitone_0_up : f32 = 1.0;
pub static semitone_1_up : f32 = 1.05946;
pub static semitone_2_up : f32 = 1.1225;
pub static semitone_3_up : f32 = 1.18921;
pub static semitone_4_up : f32 = 1.26;
pub static semitone_5_up : f32 = 1.33484;
pub static semitone_6_up : f32 = 1.41421;
pub static semitone_7_up : f32 = 1.49831;
pub static semitone_8_up : f32 = 1.58740;
pub static semitone_9_up : f32 = 1.68179;
pub static semitone_10_up : f32 = 1.78180;
pub static semitone_11_up : f32 = 1.88775;
pub static semitone_12_up : f32 = 2.0;
pub static semitone_13_up : f32 = 2.11893;
pub static semitone_14_up : f32 = 2.24492;
pub static semitone_15_up : f32 = 2.37841;
pub static semitone_16_up : f32 = 2.51984;

pub fn install() {
    Agent::new("fighter")
    .on_start(start_special_flag_checks_Init)
    .install();
    Agent::new("murabito")
    .on_line(Main, ac_update)
    .install();
    Agent::new("shizue")
    .on_line(Main, ac_update)
    .install();
    Agent::new("captain")
    .on_start(basyaamo_init)
    .install();
    #[cfg(not(feature = "dev"))]
    skyline::install_hooks!(
        create_item,
        captain_on_attack,
        common_weapon_attack_callback,
        notify_log_event_collision_hit_replace,
        fix_chara_replace,
        disable_final_smash
    );
    #[cfg(feature = "dev")]
    let _ = skyline::patching::Patch::in_text(0x8b8c88).nop();
}