use crate::imports::BuildImports::*;

unsafe extern "C" fn status_shizue_Dead_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let dead_mode = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_DEAD_MODE);
    if dead_mode == *FIGHTER_DEAD_MODE_DEADUP_STAR {
        let se_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("vc_shizue_damage_twinkle"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, se_handle as i32, 1.0, 0);
    } 
    else if dead_mode != *FIGHTER_DEAD_MODE_DEADUP_FALL {
        let rand_val = sv_math::rand(hash40("fighter"), 2);
        let sound = match rand_val {
            0 => "vc_shizue_missfoot01",
            _ => "vc_shizue_missfoot02",
        };
        let se_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new(sound), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, se_handle as i32, 1.2, 0);
    }
    let dead_reason = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_DEAD_REASON);
    if dead_reason == *FIGHTER_DEAD_REASON_KNOCKOUT_DEATH {
        let se_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("vc_shizue_damage_twinkle"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, se_handle as i32, 1.0, 0);
    }
    original_status(Main, fighter, *FIGHTER_STATUS_KIND_DEAD)(fighter)
}

pub fn install() {
    Agent::new("shizue")  
    .status(Main, *FIGHTER_STATUS_KIND_DEAD, status_shizue_Dead_Main)
    .install();
}