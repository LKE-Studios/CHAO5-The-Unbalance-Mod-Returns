use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::phx::Vector3f;
use smash::app::{self, sv_information};
use smashline::*;
use smash::lua2cpp::L2CFighterCommon;
static mut SWORD_MUL : [f32; 8] = [1.0; 8];

#[fighter_frame( agent = FIGHTER_KIND_MIISWORDSMAN )]
pub fn miiswordsman_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let fighter_kind = smash::app::utility::get_kind(boma);
        let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let joint_scale = smash::phx::Vector3f { x: SWORD_MUL[ENTRY_ID], y: SWORD_MUL[ENTRY_ID], z: SWORD_MUL[ENTRY_ID]};
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("haver"), &joint_scale);
        AttackModule::set_attack_scale(fighter.module_accessor, 1.0 * SWORD_MUL[ENTRY_ID], true);
        if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT){
            SWORD_MUL[ENTRY_ID] += 0.05;
        }
        if sv_information::is_ready_go() == false {
            SWORD_MUL[ENTRY_ID] = 1.0;
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        miiswordsman_opff
    );
}