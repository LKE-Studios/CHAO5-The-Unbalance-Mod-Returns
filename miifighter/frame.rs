use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Hash40;
use smash::app::{sv_information};

static mut DEFENCE_BOOST : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_MIIFIGHTER )]
fn miifighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if DEFENCE_BOOST[ENTRY_ID] == true {
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 0.75);
            DamageModule::set_reaction_mul(fighter.module_accessor, 0.75);
            if MotionModule::frame(fighter.module_accessor) >= 0.0 && MotionModule::frame(fighter.module_accessor) < 1.0 {  
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 6.0, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 5.3, /*G*/ 0.13, /*B*/ 0.1);
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
        miifighter_frame
    );
}