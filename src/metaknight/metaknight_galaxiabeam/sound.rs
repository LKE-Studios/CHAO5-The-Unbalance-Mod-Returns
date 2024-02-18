use crate::imports::BuildImports::*;

//Shoot
unsafe extern "C" fn sound_metaknight_galaxiabeam_Shoot(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        SET_TAKEOUT_SE_STATUS(fighter, Hash40::new("se_metaknight_attackair_f03"));
    }
}

pub fn install() {
    Agent::new("metaknight_galaxiabeam")
    .sound_acmd("sound_shoot", sound_metaknight_galaxiabeam_Shoot)
    .install();
}