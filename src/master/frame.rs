use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_MASTER )]
pub fn frame_master(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        if status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_CANCEL {
            if situation_kind == *SITUATION_KIND_AIR {
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
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
        if status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_HIT {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -35.0, 0);
            }
        };
    }
}

#[weapon_frame( agent = WEAPON_KIND_MASTER_AXE )]
pub fn frame_master_axe(weapon : &mut L2CFighterBase) {
    unsafe { 
        let status_kind = StatusModule::status_kind(weapon.module_accessor);
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        if status_kind == *WEAPON_MASTER_AXE_STATUS_KIND_SPECIAL_LW || status_kind == *WEAPON_MASTER_AXE_STATUS_KIND_SPECIAL_LW_HIT {
            if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(owner_module_accessor, -35.0, 0);
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_master,
        frame_master_axe
    );
}