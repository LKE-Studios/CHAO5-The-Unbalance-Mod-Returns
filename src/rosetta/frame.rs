use crate::imports::BuildImports::*;

#[weapon_frame( agent = WEAPON_KIND_ROSETTA_TICO )]
fn frame_tico(weapon: &mut L2CFighterBase) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent);
        let owner_module_accessor = &mut *smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let status_kind = StatusModule::status_kind(weapon.module_accessor);
        let weapon_kind = smash::app::utility::get_kind(module_accessor) as i32;
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let owner_motion_kind = MotionModule::motion_kind(owner_module_accessor);
        let owner_status_kind = StatusModule::status_kind(owner_module_accessor);

        if weapon_kind == WEAPON_KIND_ROSETTA_TICO {
            if [*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE].contains(&owner_status_kind)
            || [*WEAPON_ROSETTA_TICO_STATUS_KIND_FREE_GUARD, *WEAPON_ROSETTA_TICO_STATUS_KIND_FOLLOW_GUARD].contains(&status_kind)
            || motion_kind == hash40("follow_guard") {
                HitModule::set_whole(module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
            }
            if owner_status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF && owner_motion_kind != hash40("just_shield_off") || owner_status_kind == *FIGHTER_STATUS_KIND_FURAFURA {
                HitModule::set_whole(module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);	
            }
            if owner_status_kind == *WEAPON_ROSETTA_TICO_STATUS_KIND_SPECIAL_N_SHOOT {
                HitModule::set_whole(module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
            }
            if owner_status_kind == *WEAPON_ROSETTA_TICO_STATUS_KIND_SPECIAL_N_END {
                HitModule::set_whole(module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_tico
    );
}