<<<<<<< HEAD
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::app::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Hash40;
use smash::app::{sv_information};
use crate::utils::*;

static mut HOLD_TIME : [f32; 8] = [0.0; 8];
static mut DEFENCE_BOOST : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_PALUTENA )]
fn palutena_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if ![*FIGHTER_STATUS_KIND_GLIDE_START, *FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP){
                HOLD_TIME[ENTRY_ID] +=1.0;
            }
            if HOLD_TIME[ENTRY_ID] == 18.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_START.into(), true.into());
            }
        }
        else {
            HOLD_TIME[ENTRY_ID] = 0.0;
        };
        if [*FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_JUMP].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_WAIT].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_GLIDE].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if status_kind == *FIGHTER_STATUS_KIND_LANDING || status_kind == *FIGHTER_STATUS_KIND_LANDING_LIGHT || status_kind == *FIGHTER_STATUS_KIND_GLIDE_LANDING || status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR || status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR ||
        status_kind == *FIGHTER_STATUS_KIND_DEAD || status_kind == *FIGHTER_STATUS_KIND_MISS_FOOT || status_kind == *FIGHTER_STATUS_KIND_DAMAGE || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL || 
        status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR || status_kind == *FIGHTER_STATUS_KIND_CLIFF_CATCH { 
            macros::STOP_SE(fighter, Hash40::new("se_palutena_glide_loop"));
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        };
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_START {
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        }
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_START {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_wing"), false, -1.0);
            KineticModule::clear_speed_all(fighter.module_accessor);
            macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_ATTACK {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_attack"), false, -1.0);
            macros::STOP_SE(fighter, Hash40::new("se_palutena_glide_loop"));
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
            if MotionModule::frame(fighter.module_accessor) >= 25.0 && MotionModule::frame(fighter.module_accessor) < 26.0 {  
                ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_END {
            macros::STOP_SE(fighter, Hash40::new("se_palutena_glide_loop"));
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        }
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_LANDING {
            if MotionModule::frame(fighter.module_accessor) >= 25.0 && MotionModule::frame(fighter.module_accessor) < 26.0 {  
                ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }
        if DEFENCE_BOOST[ENTRY_ID] == true {
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 0.7);
            DamageModule::set_reaction_mul(fighter.module_accessor, 0.7);
            if MotionModule::frame(fighter.module_accessor) >= 0.0 && MotionModule::frame(fighter.module_accessor) < 1.0 {  
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 6.0, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 2.55, /*B*/ 0.48);
            }
        };
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            /*if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_lw") || MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_lwr") || MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_lwl") {
                DEFENCE_BOOST[ENTRY_ID] = true;
            };*/
            DEFENCE_BOOST[ENTRY_ID] = true;
        }
        if status_kind == *FIGHTER_STATUS_KIND_DEAD {
            DEFENCE_BOOST[ENTRY_ID] = false;
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        };
        if sv_information::is_ready_go() == false {
            DEFENCE_BOOST[ENTRY_ID] = false;
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        };
        if status_kind == *FIGHTER_STATUS_KIND_MISS_FOOT {
            DEFENCE_BOOST[ENTRY_ID] = false;
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        palutena_frame
    );
=======
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::app::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Hash40;
use smash::app::{sv_information};

static mut HOLD_TIME : [f32; 8] = [0.0; 8];
static mut DEFENCE_BOOST : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_PALUTENA )]
fn palutena_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if ![*FIGHTER_STATUS_KIND_GLIDE_START, *FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP){
                HOLD_TIME[ENTRY_ID] +=1.0;
            }
            if HOLD_TIME[ENTRY_ID] == 18.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_START.into(), true.into());
            }
        }
        else {
            HOLD_TIME[ENTRY_ID] = 0.0;
        };
        if [*FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_JUMP].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_WAIT].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_GLIDE].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if status_kind == *FIGHTER_STATUS_KIND_LANDING || status_kind == *FIGHTER_STATUS_KIND_LANDING_LIGHT || status_kind == *FIGHTER_STATUS_KIND_GLIDE_LANDING || status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR || status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR ||
        status_kind == *FIGHTER_STATUS_KIND_DEAD || status_kind == *FIGHTER_STATUS_KIND_MISS_FOOT || status_kind == *FIGHTER_STATUS_KIND_DAMAGE || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL || 
        status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR || status_kind == *FIGHTER_STATUS_KIND_CLIFF_CATCH { 
            macros::STOP_SE(fighter, Hash40::new("se_palutena_glide_loop"));
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        };
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_START {
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        }
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_ATTACK {
            macros::STOP_SE(fighter, Hash40::new("se_palutena_glide_loop"));
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        }
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_END {
            macros::STOP_SE(fighter, Hash40::new("se_palutena_glide_loop"));
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        };
        if DEFENCE_BOOST[ENTRY_ID] == true {
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 0.7);
            DamageModule::set_reaction_mul(fighter.module_accessor, 0.7);
            if MotionModule::frame(fighter.module_accessor) >= 0.0 && MotionModule::frame(fighter.module_accessor) < 1.0 {  
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 6.0, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 2.55, /*B*/ 0.48);
            }
        };
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            /*if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_lw") || MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_lwr") || MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_lwl") {
                DEFENCE_BOOST[ENTRY_ID] = true;
            };*/
            DEFENCE_BOOST[ENTRY_ID] = true;
        }
        if status_kind == *FIGHTER_STATUS_KIND_DEAD {
            DEFENCE_BOOST[ENTRY_ID] = false;
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        };
        if sv_information::is_ready_go() == false {
            DEFENCE_BOOST[ENTRY_ID] = false;
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        };
        if status_kind == *FIGHTER_STATUS_KIND_MISS_FOOT {
            DEFENCE_BOOST[ENTRY_ID] = false;
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        palutena_frame
    );
>>>>>>> 70e591ed528511fa22d74147c05b77221fd7f3d5
}