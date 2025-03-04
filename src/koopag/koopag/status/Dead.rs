use crate::imports::BuildImports::*;

unsafe extern "C" fn status_koopag_Dead_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let dead_mode = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_DEAD_MODE);
    fighter.status_DeadSub();
    if dead_mode == *FIGHTER_DEAD_MODE_DEADUP_STAR {
        let se_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("vc_koopag_damage_twinkle"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, se_handle as i32, 1.0, 0);
    } 
    else if dead_mode != *FIGHTER_DEAD_MODE_DEADUP_FALL {
        let rand_val = sv_math::rand(hash40("fighter"), 2);
        let sound = match rand_val {
            0 => "vc_koopag_missfoot01",
            _ => "vc_koopag_missfoot02",
        };
        let se_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new(sound), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, se_handle as i32, 1.0, 0);
    }
    if FighterUtil::is_hp_mode(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("death_air"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("death"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(koopag_Dead_Main_loop as *const () as _))
    }
    else {
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Dead_Main as *const () as _))
    }
}

unsafe extern "C" fn koopag_Dead_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("death"), -1.0, 1.0, 0.0);
    }
    if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("death_air"), -1.0, 1.0, 0.0);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ALLOT_STATUSES);
        fighter.change_status(FIGHTER_STATUS_KIND_STANDBY.into(), false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("koopag")  
    .status(Main, *FIGHTER_STATUS_KIND_DEAD, status_koopag_Dead_Main)
    .install();
}