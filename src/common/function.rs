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

pub unsafe fn metaknight_special_n_disable (fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind == *FIGHTER_KIND_METAKNIGHT {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_SPECIAL_N) {
            METAKNIGHT_DISABLE_SPECIAL_N[ENTRY_ID] = true;
        }
    }
}

//Custom Fighter Functions

pub static mut META_POWER : [bool; 8] = [false; 8];
pub static META_POWER_DAMAGE : f32 = 100.0;
pub static META_POWER_ATTACK_MUL : f32 = 1.25;
pub static META_POWER_REACTION_MUL : f32 = 0.5;
pub static META_POWER_DAMAGE_TAKEN_MUL : f32 = 0.5;

pub mod FighterSpecializer_MetaKnight {
    use crate::imports::BuildImports::*;
    pub unsafe fn meta_power (fighter: &mut L2CFighterCommon)  {
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