use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lua2cpp::L2CFighterCommon;

#[fighter_frame( agent = FIGHTER_KIND_GEKKOUGA )]
pub fn gekkouga_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if status_kind == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_ATTACK {
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -20.0, 0);
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        gekkouga_opff
    );
}