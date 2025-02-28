use crate::imports::BuildImports::*;

unsafe extern "C" fn status_sans_Final_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SANS = color >= 120 && color <= 127;
    if SANS {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x201bc9217c));
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("final_air"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_FINAL_FLAG_IS_AIR_SITUATION);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("final"), 0.0, 1.0, false, 0.0, false, false);
        }
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode {_address: *DAMAGE_NO_REACTION_MODE_ALWAYS as u8}, -1.0, -1.0, -1);
        AreaModule::set_whole(fighter.module_accessor, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
        fighter.sub_shift_status_main(L2CValue::Ptr(sans_Final_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter)
    } 
}

unsafe extern "C" fn sans_Final_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    CameraModule::req_quake(fighter.module_accessor, *CAMERA_QUAKE_KIND_MIDDLE);
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn status_sans_Final_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SANS = color >= 120 && color <= 127;
    if SANS {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e0aba2d68));
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_black"), false, false);
        if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_PALUTENA_STATUS_KIND_FINAL_WAIT {
            SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_palutena_final02"), 0);
        }
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter)
    } 
}

pub fn install() {
    Agent::new("palutena")
    .status(Main, *FIGHTER_STATUS_KIND_FINAL, status_sans_Final_Main)
    .status(End, *FIGHTER_STATUS_KIND_FINAL, status_sans_Final_End)
    .install();
}