use crate::imports::BuildImports::*;

#[weapon_frame( agent = WEAPON_KIND_POPO_BLIZZARD )]
pub fn frame_popo_blizzard(weapon : &mut L2CFighterBase) {
    unsafe {
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(owner_module_accessor, -3.0, 0);
        }
    };
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_popo_blizzard
    );
}