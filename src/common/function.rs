use crate::imports::BuildImports::*;

pub static mut GFX_COUNTER : [i32; 8] = [0; 8];
pub static mut SFX_COUNTER : [i32; 8] = [0; 8];
pub static mut COUNTER : [i32; 8] = [0; 8];
pub static mut CURRENT_ON_FRAME : [f32; 8] = [0.0; 8];
pub static mut IS_CRIT : [bool; 8] = [false; 8];

//Const Functions
pub unsafe fn common_attack_critical_flag (fighter: &mut L2CFighterCommon) {
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

#[smashline::fighter_init]
fn metaknight_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind == *FIGHTER_KIND_METAKNIGHT {
            fighter.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(metaknight_special_n_callback as *const () as _));
            fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(metaknight_special_s_callback as *const () as _));
            fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(metaknight_special_hi_callback as *const () as _));
            fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(metaknight_special_lw_callback as *const () as _));
            fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(metaknight_change_status_callback as *const () as _)); 
        }
    }
}

unsafe extern "C" fn metaknight_special_n_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_N) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn metaknight_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_S) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn metaknight_special_hi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_HI) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn metaknight_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_LW) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn metaknight_change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF, *SITUATION_KIND_WATER, *SITUATION_KIND_LADDER].contains(&situation_kind) || 
    [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_MISS_FOOT, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FALL, 
    *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_FALL, 
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, 
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD, 
    *FIGHTER_STATUS_KIND_ICE].contains(&status_kind) || sv_information::is_ready_go() == false {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_N);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_S);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_HI);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_LW);
    }
    true.into()
}

#[smashline::fighter_init]
fn lucario_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind == *FIGHTER_KIND_LUCARIO {
            fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(lucario_special_hi_callback as *const () as _));  
            fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(lucario_change_status_callback as *const () as _));   
        }
    }
}

unsafe extern "C" fn lucario_special_hi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_ENABLE_SPECIAL_HI) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn lucario_change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_ENABLE_SPECIAL_HI);
    }
    true.into()
}

//Custom Fighter Functions

pub static mut META_POWER : [bool; 8] = [false; 8];
pub static META_POWER_DAMAGE : f32 = 100.0;
pub static META_POWER_ATTACK_MUL : f32 = 1.25;
pub static META_POWER_REACTION_MUL : f32 = 0.5;
pub static META_POWER_DAMAGE_TAKEN_MUL : f32 = 0.5;
pub static mut GODDESS_POWER_UP : [bool; 8] = [false; 8];
pub static POWER_MUL : f32 = 1.1;

pub mod FighterSpecializer_MetaKnight {
    use crate::imports::BuildImports::*;
    pub unsafe fn meta_power (fighter: &mut L2CFighterCommon) {
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if META_POWER[ENTRY_ID] == true {
            GFX_COUNTER[ENTRY_ID] += 1;
            SFX_COUNTER[ENTRY_ID] += 1;
            AttackModule::set_power_up(fighter.module_accessor, META_POWER_ATTACK_MUL);
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, META_POWER_DAMAGE_TAKEN_MUL);
            DamageModule::set_reaction_mul(fighter.module_accessor, META_POWER_REACTION_MUL);
            if SFX_COUNTER[ENTRY_ID] < 2 {
                PLAY_SE(fighter, Hash40::new("se_metaknight_special_l01"));
                PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_final01"));
                PLAY_SE_REMAIN(fighter, Hash40::new("vc_metaknight_final03"));
            };
            if SFX_COUNTER[ENTRY_ID] >= 100 {
                SFX_COUNTER[ENTRY_ID] = 2;
            };
            if DamageModule::damage(fighter.module_accessor, 0) < META_POWER_DAMAGE {
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
        if DamageModule::damage(fighter.module_accessor, 0) >= META_POWER_DAMAGE {
            META_POWER[ENTRY_ID] = true;
        };   
        if DamageModule::damage(fighter.module_accessor, 0) < META_POWER_DAMAGE {
            META_POWER[ENTRY_ID] = false;
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
            AttackModule::set_power_up(fighter.module_accessor, 1.0);
        };     
        if [
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_MISS_FOOT
            ].contains(&status_kind) || sv_information::is_ready_go() == false {
            META_POWER[ENTRY_ID] = false;
            AttackModule::set_power_up(fighter.module_accessor, 1.0);
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
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
            STOP_SE(fighter, Hash40::new("vc_metaknight_final03"));
        }
    }
}

pub mod FighterSpecializer_Palutena {
    use crate::imports::BuildImports::*;
    pub unsafe fn goddess_power_up (fighter: &mut L2CFighterCommon) {
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if GODDESS_POWER_UP[ENTRY_ID] == true {
            GFX_COUNTER[ENTRY_ID] += 1;
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 0.7);
            DamageModule::set_reaction_mul(fighter.module_accessor, 0.7);
            AttackModule::set_power_up(fighter.module_accessor, POWER_MUL);
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

pub unsafe fn waluigi_dice_block_visible(fighter: &mut L2CFighterCommon) {
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_WALUIGI_STATUS_SPECIAL_DICE_BLOCK_WORK_ID_FLAG_VISIBLE) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"), false);
		ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"), true);
    }
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_WALUIGI_STATUS_SPECIAL_N_DICE_BLOCK_WORK_ID_FLAG_VISIBLE) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("dolly_Kart_Glider_VIS_O_OBJShape"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"), false);
    }
    else {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"), true);
    }
}

pub unsafe fn waluigi_dice_block_roll(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_WALUIGI_STATUS_SPECIAL_DICE_BLOCK_WORK_ID_FLAG_ROLL_ONE) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), true);
    }
    else {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_WALUIGI_STATUS_SPECIAL_DICE_BLOCK_WORK_ID_FLAG_ROLL_TWO) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), true);
    }
    else {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_WALUIGI_STATUS_SPECIAL_DICE_BLOCK_WORK_ID_FLAG_ROLL_THREE) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), true);
    }
    else {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_WALUIGI_STATUS_SPECIAL_DICE_BLOCK_WORK_ID_FLAG_ROLL_FOUR) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), true);
    }
    else {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_WALUIGI_STATUS_SPECIAL_DICE_BLOCK_WORK_ID_FLAG_ROLL_FIVE) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), true);
    }
    else {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_WALUIGI_STATUS_SPECIAL_DICE_BLOCK_WORK_ID_FLAG_ROLL_SIX) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), true);
    }
    else {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_WALUIGI_STATUS_SPECIAL_DICE_BLOCK_WORK_ID_FLAG_ROLL_SEVEN) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), true);
    }
    else {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_WALUIGI_STATUS_SPECIAL_DICE_BLOCK_WORK_ID_FLAG_ROLL_EIGHT) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), true);
    }
    else {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_WALUIGI_STATUS_SPECIAL_DICE_BLOCK_WORK_ID_FLAG_ROLL_NINE) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), true);
    }
    else {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_WALUIGI_STATUS_SPECIAL_DICE_BLOCK_WORK_ID_FLAG_ROLL_TEN) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), true);
    }
    else {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
    }
}

pub fn install() {
    smashline::install_agent_init_callbacks!(
        metaknight_init,
        lucario_init
    );
}