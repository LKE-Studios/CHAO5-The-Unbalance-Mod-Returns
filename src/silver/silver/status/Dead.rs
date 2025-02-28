use crate::imports::BuildImports::*;

unsafe extern "C" fn status_silver_Dead_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SILVER = color >= 120 && color <= 127;
    if SILVER {
        let dead_mode = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_DEAD_MODE);
        fighter.status_DeadSub();
        if dead_mode == *FIGHTER_DEAD_MODE_NORMAL {
            let rand_val = sv_math::rand(hash40("fighter"), 2);
            let sound = match rand_val {
                0 => hash40("vc_silver_missfoot01"),
                _ => hash40("vc_silver_missfoot02"),
            };
            SoundModule::play_se(fighter.module_accessor, Hash40::new_raw(sound), true, false, false, false, enSEType(0));
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Dead_Main as *const () as _))
    }    
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_DEAD)(fighter)
    }
}

pub fn install() {
    Agent::new("mewtwo")  
    .status(Main, *FIGHTER_STATUS_KIND_DEAD, status_silver_Dead_Main)
    .install();
}