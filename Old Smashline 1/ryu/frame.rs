use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_RYU )]
pub fn frame_ryu(fighter : &mut L2CFighterCommon) {
    unsafe { 
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            fighter.sub_wait_ground_check_common(false.into());
            fighter.sub_air_check_fall_common();
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
        };
        if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP {
            fighter.sub_wait_ground_check_common(false.into());
            fighter.sub_air_check_fall_common();
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
        };
        if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND {
            fighter.sub_wait_ground_check_common(false.into());
            fighter.sub_air_check_fall_common();
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_ryu
    );
}