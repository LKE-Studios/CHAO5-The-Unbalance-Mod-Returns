use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::phx::Vector3f;
use smash::phx::Hash40;
//use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;
use smash::lua2cpp::L2CFighterBase;

#[fighter_frame( agent = FIGHTER_KIND_MASTER )]
pub fn master_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        if status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_CANCEL {
            if situation_kind == *SITUATION_KIND_AIR {
                if WorkModule::get_int(boma, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                    WorkModule::set_int(boma, *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                }
            }
        }
        if status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT || status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT_DASH {
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{x:2.0, y:2.0, z:2.0});
            AttackModule::set_attack_scale(fighter.module_accessor, 1.9, true);
            if MotionModule::frame(fighter.module_accessor) > 52.0 {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{x:1.0, y:1.0, z:1.0});
                AttackModule::set_attack_scale(fighter.module_accessor, 1.0, true);
            }
        };
        if status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_HIT || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -999.0, 0);
            }
        };
    }
}

#[weapon_frame( agent = WEAPON_KIND_MASTER_AXE )]
pub fn master_axe_opwf(fighter : &mut L2CFighterBase) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        if status_kind == *WEAPON_MASTER_AXE_STATUS_KIND_SPECIAL_LW || status_kind == *WEAPON_MASTER_AXE_STATUS_KIND_SPECIAL_LW_HIT {
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -999.0, 0);
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        master_opff,
        master_axe_opwf
    );
}