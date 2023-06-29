use crate::imports::BuildImports::*;

static mut SWORD_MUL : [f32; 8] = [1.0; 8];

#[fighter_frame( agent = FIGHTER_KIND_MIISWORDSMAN )]
pub fn frame_miiswordsman(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let joint_scale = Vector3f { x: SWORD_MUL[ENTRY_ID], y: SWORD_MUL[ENTRY_ID], z: SWORD_MUL[ENTRY_ID]};
        
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &joint_scale);
        AttackModule::set_attack_scale(fighter.module_accessor, 1.0 * SWORD_MUL[ENTRY_ID], true);
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT){
            SWORD_MUL[ENTRY_ID] += 0.05;
        }
        if sv_information::is_ready_go() == false {
            SWORD_MUL[ENTRY_ID] = 1.0;
        }
        if [*FIGHTER_STATUS_KIND_MISS_FOOT, 
        *FIGHTER_STATUS_KIND_DEAD, 
        *FIGHTER_STATUS_KIND_WIN, 
        *FIGHTER_STATUS_KIND_LOSE].contains(&status_kind) {
            SWORD_MUL[ENTRY_ID] = 1.0;
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_miiswordsman
    );
}