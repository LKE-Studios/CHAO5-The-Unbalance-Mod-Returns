use crate::imports::BuildImports::*;

pub static mut GFX_COUNTER : [i32; 8] = [0; 8];
pub static mut SFX_COUNTER : [i32; 8] = [0; 8];
pub static mut COUNTER : [i32; 8] = [0; 8];
pub static mut CURRENT_ON_FRAME : [f32; 8] = [0.0; 8];
pub static mut IS_CRIT : [bool; 8] = [false; 8];
const COMMON_WEAPON_ATTACK_CALLBACK: usize = 0x33bdc10;

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

//Const Functions
pub unsafe fn common_attack_critical_flag(fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        COUNTER[ENTRY_ID] += 1;
        IS_CRIT[ENTRY_ID] = true;
        if COUNTER[ENTRY_ID] < 2 {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), Hash40::new("haver"), &Vector3f{x: 0.0, y: 8.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
            CURRENT_ON_FRAME[ENTRY_ID] = MotionModule::frame(fighter.module_accessor);
            SlowModule::set_whole(fighter.module_accessor, 2, 0);
            PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
            QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
            let lr = PostureModule::lr(fighter.module_accessor);
            CAM_ZOOM_IN_arg5(fighter, /*frames*/ 4.0,/*no*/ 0.0,/*zoom*/ 2.1,/*yrot*/ 0.0,/*xrot*/ 0.0 * lr);
        }
    }
    if MotionModule::frame(fighter.module_accessor) >= (CURRENT_ON_FRAME[ENTRY_ID] + 1.0) && IS_CRIT[ENTRY_ID] {
        COUNTER[ENTRY_ID] = 0;
        SlowModule::clear_whole(fighter.module_accessor);
        CameraModule::reset_all(fighter.module_accessor);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        if StatusModule::status_kind(fighter.module_accessor) != 510 {
            CAM_ZOOM_OUT(fighter);
        }
    }
    if IS_CRIT[ENTRY_ID] && MotionModule::frame(fighter.module_accessor) < 2.0 {
        CAM_ZOOM_OUT(fighter);
        IS_CRIT[ENTRY_ID] = false;
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
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

unsafe extern "C" fn special_flag_checks_init(fighter: &mut L2CFighterCommon) {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind == *FIGHTER_KIND_METAKNIGHT {
        fighter.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(metaknight_special_n_callback as *const () as _));
        fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(metaknight_special_s_callback as *const () as _));
        fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(metaknight_special_hi_callback as *const () as _));
        fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(metaknight_special_lw_callback as *const () as _));
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(metaknight_change_status_callback as *const () as _)); 
    }
    if fighter_kind == *FIGHTER_KIND_LUCARIO {
        fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(lucario_special_hi_callback as *const () as _));  
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(lucario_change_status_callback as *const () as _));   
    }
    if fighter_kind == *FIGHTER_KIND_SIMON || fighter_kind == *FIGHTER_KIND_RICHTER {
        fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(simon_special_hi_callback as *const () as _));  
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(simon_change_status_callback as *const () as _));   
    }
    if fighter_kind == *FIGHTER_KIND_TRAIL {
        fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(trail_special_s_callback as *const () as _));  
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(trail_change_status_callback as *const () as _));   
    }
}

unsafe extern "C" fn metaknight_special_n_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_N) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn metaknight_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn metaknight_special_hi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn metaknight_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn metaknight_change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_AIR || [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_MISS_FOOT, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FALL, 
    *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_FALL, 
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, 
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD, 
    *FIGHTER_STATUS_KIND_ICE].contains(&status_kind) || sv_information::is_ready_go() == false {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_N);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW);
    }
    true.into()
}


unsafe extern "C" fn lucario_special_hi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_ENABLE_SPECIAL_HI) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn lucario_change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_ENABLE_SPECIAL_HI);
    }
    true.into()
}

unsafe extern "C" fn simon_special_hi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn simon_change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_AIR || [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_MISS_FOOT, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FALL, 
    *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_FALL, 
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, 
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD, 
    *FIGHTER_STATUS_KIND_ICE].contains(&status_kind) || sv_information::is_ready_go() == false {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI);
    }
    true.into()
}

unsafe extern "C" fn trail_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn trail_change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_AIR || [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_MISS_FOOT, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FALL, 
    *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_FALL, 
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, 
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD, 
    *FIGHTER_STATUS_KIND_ICE].contains(&status_kind) || sv_information::is_ready_go() == false {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
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

pub static mut META_POWER : [bool; 8] = [false; 8];
pub static mut GODDESS_POWER_UP : [bool; 8] = [false; 8];
pub static POWER_MUL : f32 = 1.1;

pub mod FighterSpecializer_MetaKnight {
    use crate::imports::BuildImports::*;
    pub unsafe fn meta_power(fighter: &mut L2CFighterCommon) {
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let meta_power_damage = WorkModule::get_param_float(fighter.module_accessor, hash40("param_meta_power"), hash40("meta_power_damage"));
        let attack_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_meta_power"), hash40("attack_mul"));
        let reaction_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_meta_power"), hash40("reaction_mul"));
        let hit_damage_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_meta_power"), hash40("hit_damage_mul"));
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_META_POWER) {
            GFX_COUNTER[ENTRY_ID] += 1;
            SFX_COUNTER[ENTRY_ID] += 1;
            AttackModule::set_power_up(fighter.module_accessor, attack_mul);
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, hit_damage_mul);
            DamageModule::set_reaction_mul(fighter.module_accessor, reaction_mul);
            if SFX_COUNTER[ENTRY_ID] < 2 {
                PLAY_SE(fighter, Hash40::new("se_metaknight_special_l01"));
                PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_final01"));
                PLAY_SE_REMAIN(fighter, Hash40::new("vc_metaknight_final03"));
            };
            if SFX_COUNTER[ENTRY_ID] >= 100 {
                SFX_COUNTER[ENTRY_ID] = 2;
            };
            if DamageModule::damage(fighter.module_accessor, 0) < meta_power_damage {
                SFX_COUNTER[ENTRY_ID] = 0;
            };
            if GFX_COUNTER[ENTRY_ID] >= 6 {
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_hit_purple"), Hash40::new("haver"), &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 0.15, true, 0, 0, 0, 0, 0, true, true);
                LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.68, /*G*/ 0.87, /*B*/ 2.0);
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_hit_purple"), Hash40::new("havel"), &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 0.15, true, 0, 0, 0, 0, 0, true, true);
                LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.68, /*G*/ 0.87, /*B*/ 2.0);
                GFX_COUNTER[ENTRY_ID] = 0;
            };
        };
        if DamageModule::damage(fighter.module_accessor, 0) >= meta_power_damage {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_META_POWER);
        };   
        if DamageModule::damage(fighter.module_accessor, 0) < meta_power_damage || [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_MISS_FOOT].contains(&status_kind) 
        || sv_information::is_ready_go() == false {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_META_POWER);
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
            AttackModule::set_power_up(fighter.module_accessor, 1.0);
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
            SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_metaknight_final03"), 0);
        }
    }
}

pub mod FighterSpecializer_Palutena {
    use crate::imports::BuildImports::*;
    pub unsafe fn goddess_power_up (fighter: &mut L2CFighterCommon) {
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let attack_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_divine_power_up"), hash40("attack_mul"));
        let reaction_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_divine_power_up"), hash40("reaction_mul"));
        let hit_damage_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_divine_power_up"), hash40("hit_damage_mul"));
        if GODDESS_POWER_UP[ENTRY_ID] == true {
            GFX_COUNTER[ENTRY_ID] += 1;
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, hit_damage_mul);
            DamageModule::set_reaction_mul(fighter.module_accessor, reaction_mul);
            AttackModule::set_power_up(fighter.module_accessor, attack_mul);
            if GFX_COUNTER[ENTRY_ID] >= 20 {
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_aura_light"), Hash40::new("waist"), &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 5.0, true, 0, 0, 0, 0, 0, true, true);
                LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 2.55, /*B*/ 0.48);
                GFX_COUNTER[ENTRY_ID] = 0;
            };
        };
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            GODDESS_POWER_UP[ENTRY_ID] = true;
        }
        if status_kind == *FIGHTER_STATUS_KIND_DEAD || status_kind == *FIGHTER_STATUS_KIND_MISS_FOOT || 
        sv_information::is_ready_go() == false {
            GODDESS_POWER_UP[ENTRY_ID] = false;
            AttackModule::set_power_up(fighter.module_accessor, 1.0);
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
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

pub unsafe extern "C" fn is_cloned_article(object_boma: *mut smash::app::BattleObjectModuleAccessor) -> bool {
    let color = WorkModule::get_int(object_boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let ADDED_FIGHTER = color >= 120 && color <= 130;
    let ADDED_FIGHTER_2 = color >= 64 && color <= 71;
    if utility::get_kind(&mut *object_boma) == *WEAPON_KIND_SHEIK_NEEDLE 
    || utility::get_kind(&mut *object_boma) == *WEAPON_KIND_LINK_BOOMERANG 
    || utility::get_kind(&mut *object_boma) == *WEAPON_KIND_PITB_BOWARROW {
        let owner_id = WorkModule::get_int(object_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
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
        if owner_kind == *FIGHTER_KIND_NESS && ADDED_FIGHTER_2 { //KAMEK
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
        if object_id == 0 || object_id == 0x50000000 {return;}
        let object_boma = sv_battle_object::module_accessor(object_id);
        if is_cloned_article(object_boma) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE, false);
            WorkModule::set_int(fighter.module_accessor, 0x50000000, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);
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
        WorkModule::on_flag(module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    else {
        WorkModule::off_flag(module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    if WorkModule::is_flag(module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let owner_kind = smash::app::utility::get_kind(&mut *owner_module_accessor);
        WorkModule::set_int(module_accessor, item_kind, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
        if fuse_kind == FuseKind::FUSE {
            if owner_kind == *FIGHTER_KIND_LINK {
                WorkModule::set_int(owner_module_accessor, item_kind, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
            }
            else if owner_kind == *FIGHTER_KIND_KIRBY {
                WorkModule::set_int(owner_module_accessor, item_kind, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
            }
            let item_id = ItemModule::get_have_item_id(owner_module_accessor, 0) as i32;
            WorkModule::set_int(module_accessor, item_id, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
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
            WorkModule::set_int(module_accessor, item_id, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
            if [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&owner_kind) {
                WorkModule::set_int(owner_module_accessor, *ITEM_KIND_NONE, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
            }
        }
        if item_kind == *ITEM_KIND_BOMBER {
            WorkModule::set_int(module_accessor, FuseType::NORMAL, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(module_accessor, *ITEM_BOMBER_STATUS_KIND_BORN2, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if [*ITEM_KIND_KILLER, *ITEM_KIND_BANANAGUN, *ITEM_KIND_DOLPHINBOMB].contains(&item_kind) {
            WorkModule::set_int(module_accessor, FuseType::POWER, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(module_accessor, *ITEM_STATUS_KIND_THROW, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_FIREFLOWER {
            WorkModule::set_int(module_accessor, FuseType::ELEMENTAL, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(module_accessor, *ITEM_STATUS_KIND_LOST, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_LINKBOMB {
            WorkModule::set_int(module_accessor, FuseType::NORMAL, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(module_accessor, *ITEM_STATUS_KIND_BORN, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else {
            WorkModule::set_int(module_accessor, FuseType::NORMAL, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(module_accessor, *ITEM_STATUS_KIND_THROW, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
    }
}

pub unsafe extern "C" fn set_elemental_fuse(weapon: &mut L2CFighterBase, element: i32, fuse_type: i32, end_status: i32) {
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_kind = smash::app::utility::get_kind(&mut *owner_module_accessor);
    if owner_kind == *FIGHTER_KIND_LINK {
        WorkModule::set_int(owner_module_accessor, element, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
    }
    else {
        WorkModule::set_int(owner_module_accessor, element, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
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
    WorkModule::set_int(weapon.module_accessor, element, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
    WorkModule::set_int(weapon.module_accessor, fuse_type, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
    WorkModule::set_int(weapon.module_accessor, end_status, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
    let item_module_accessor = (*battle_object).module_accessor;
    let item_id = (*item_module_accessor).battle_object_id;
    WorkModule::set_int64(weapon.module_accessor, item_id as i64, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
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
    WorkModule::on_flag(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
}

pub unsafe extern "C" fn set_boomerang_fuse_params(module_accessor: *mut smash::app::BattleObjectModuleAccessor, item_kind: i32, fuse_kind: i32, trait_type: i32) {
    if (![*ITEM_KIND_NONE,*ITEM_KIND_ASSIST,*ITEM_KIND_LINKARROW].contains(&item_kind) && ![*ITEM_TRAIT_FLAG_NONE,*ITEM_TRAIT_FLAG_SHOOT,*ITEM_TRAIT_FLAG_SWING].contains(&trait_type))
    || [*ITEM_KIND_FIREFLOWER].contains(&item_kind) {
        WorkModule::on_flag(module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    else {
        WorkModule::off_flag(module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    if WorkModule::is_flag(module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        WorkModule::set_int(module_accessor, item_kind, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
        let owner_kind = smash::app::utility::get_kind(&mut *owner_module_accessor);
        if fuse_kind == FuseKind::FUSE {
            WorkModule::set_int(owner_module_accessor,item_kind, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
            let item_id = ItemModule::get_have_item_id(owner_module_accessor, 0) as i32;
            WorkModule::set_int(module_accessor,item_id, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
            WorkModule::set_int(owner_module_accessor,item_id, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
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
            WorkModule::set_int(module_accessor, item_id, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
            WorkModule::set_int(owner_module_accessor, item_id, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM_ID);
            if owner_kind == *FIGHTER_KIND_MURABITO
            || owner_kind == *FIGHTER_KIND_SHIZUE {
                WorkModule::set_int(owner_module_accessor, *ITEM_KIND_NONE,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM);
            }
        }
        if item_kind == *ITEM_KIND_BOMBER {
            WorkModule::set_int(module_accessor, *ITEM_BOMBER_STATUS_KIND_BORN2, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_FIREFLOWER {
            WorkModule::set_int(module_accessor, *ITEM_STATUS_KIND_LOST, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_LINKBOMB {
            WorkModule::set_int(module_accessor, *ITEM_STATUS_KIND_BORN, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else {
            WorkModule::set_int(module_accessor, *ITEM_STATUS_KIND_THROW, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
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

#[skyline::hook(offset = 0x15db0b0)]
pub unsafe fn create_item(item_manager: *mut smash::app::ItemManager, create_item_param: *mut CreateItemParam, unk: bool, unk2: bool, unk3: bool) -> *mut BattleObject {
    if (*create_item_param).variation_kind > 7 {
        (*create_item_param).variation_kind = 0;
    }
    original!()(item_manager, create_item_param, unk, unk2, unk3)
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
            do_critical_zoom(module_accessor, log, 11, hash40("param_special_n"), 1, 0, 0, 0, 0);
            return 0;
        }
    }
    call_original!(vtable, fighter, log)
}

//The original critical zoom-in function
#[skyline::from_offset(0x696720)]
unsafe extern "C" fn do_critical_zoom(module_accessor: *mut smash::app::BattleObjectModuleAccessor, log: u64, unk1: u8, param_hash: u64, unk3: u64, unk4: u32, unk5: u32, unk6: i8, unk7: i8) -> u8;

#[skyline::hook(offset = COMMON_WEAPON_ATTACK_CALLBACK)]
unsafe extern "C" fn common_weapon_attack_callback(vtable: u64, weapon: *mut smash::app::Weapon, log: u32) {
    let module_accessor = (*weapon).battle_object.module_accessor;
    let owner_object_id = WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor(owner_object_id as u32);
    let owner_kind = utility::get_kind(&mut *owner_module_accessor);
    let collision_log = *(log as *const u64).add(0x10/0x8);
    let collision_log = collision_log as *const CollisionLog;
    let opponent_id = (*collision_log).collider_id;
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let CUSTOM_FIGHTER_2 = color >= 64 && color <= 71;
    let CUSTOM_FIGHTER = color >= 120 && color <= 127;
    if (*weapon).battle_object.kind == *WEAPON_KIND_LUIGI_FIREBALL as u32 && CUSTOM_FIGHTER && owner_kind == *FIGHTER_KIND_MEWTWO {
        *(weapon as *mut bool).add(0x90) = true;
    }
    call_original!(vtable, weapon, log)
}

pub fn install() {
    Agent::new("fighter")
    .on_start(special_flag_checks_init)
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
        common_weapon_attack_callback
    );
    #[cfg(feature = "dev")]
    let _ = skyline::patching::Patch::in_text(0x8b8c88).nop();
}