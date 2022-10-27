//use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::phx::Hash40;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;

static mut HOLD_TIME : [f32; 8] = [0.0; 8];

#[fighter_frame( agent = FIGHTER_KIND_BUDDY )]
fn buddy_opff(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut smash::app::KineticEnergy;
        let anti_wind = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND) as *mut smash::app::KineticEnergy;
        let no_jostle = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE) as *mut smash::app::KineticEnergy;
        if ![*FIGHTER_STATUS_KIND_GLIDE_START, *FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR{
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP){
                HOLD_TIME[ENTRY_ID] +=1.0;
            }
            if HOLD_TIME[ENTRY_ID] == 22.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_START.into(), true.into());
            }
        }
        else {
            HOLD_TIME[ENTRY_ID] = 0.0;
        };
        if [*FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0 {
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
        if status_kind == *FIGHTER_STATUS_KIND_LANDING || status_kind == *FIGHTER_STATUS_KIND_LANDING_LIGHT || status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR || status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR || status_kind == *FIGHTER_STATUS_KIND_DEAD ||
        status_kind == *FIGHTER_STATUS_KIND_MISS_FOOT || status_kind == *FIGHTER_STATUS_KIND_DAMAGE || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR ||
        status_kind == *FIGHTER_STATUS_KIND_CLIFF_CATCH || status_kind == *FIGHTER_STATUS_KIND_GLIDE_LANDING { 
            macros::STOP_SE(fighter, Hash40::new("se_buddy_glide_loop"));
        };
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_ATTACK {
            macros::STOP_SE(fighter, Hash40::new("se_buddy_glide_loop"));
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        };
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_END {
            macros::STOP_SE(fighter, Hash40::new("se_buddy_glide_loop"));
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        };
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_START {
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        }
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_ATTACK {
            macros::STOP_SE(fighter, Hash40::new("se_buddy_glide_loop"));
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        }
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_END {
            macros::STOP_SE(fighter, Hash40::new("se_buddy_glide_loop"));
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            WorkModule::unable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        };
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE {
            if MotionModule::frame_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING) >= 4.0 && MotionModule::frame_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING) < 5.0 {
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_buddy_wing"));
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            smash::app::lua_bind::KineticEnergy::clear_speed(anti_wind);
            smash::app::lua_bind::KineticEnergy::clear_speed(no_jostle);
        };
        if status_kind == *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH {
            smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            smash::app::lua_bind::KineticEnergy::clear_speed(anti_wind);
            smash::app::lua_bind::KineticEnergy::clear_speed(no_jostle);
            fighter.sub_air_check_fall_common();
            fighter.sub_wait_ground_check_common(false.into());
            WorkModule::set_int(fighter.module_accessor, 5, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -20.0, 0);
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        buddy_opff
    );
}
