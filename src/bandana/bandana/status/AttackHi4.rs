use crate::imports::BuildImports::*;

unsafe extern "C" fn status_bandana_AttackHi4_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        let effect_handle = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BANDANA_INSTANCE_WORK_ID_INT_ATTACK_HI4_EFFECT_HANDLE);
        EffectModule::set_scale(fighter.module_accessor, effect_handle as u32, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("kirby_vacuum"), false, false);
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_HI4)(fighter)
    }
}

pub fn install() {
    Agent::new("edge")
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_HI4, status_bandana_AttackHi4_End)
    .install();
}