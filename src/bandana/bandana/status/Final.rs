use crate::imports::BuildImports::*;

pub static final_dive_speed_y : f32 = 3.5;
pub static final_hold_speed_y : f32 = 6.0;

unsafe extern "C" fn status_bandana_Final_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, (*FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | *FIGHTER_STATUS_ATTR_FINAL) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_FINAL as u32, 0);
        0.into()
    }
    else {
        original_status(Pre, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter)
    }
}

unsafe extern "C" fn status_bandana_Final_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.set_situation(SITUATION_KIND_AIR.into());
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("final_air"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("final"), 0.0, 1.0, false, 0.0, false, false);
        }
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x201bc9217c));
        fighter.sub_shift_status_main(L2CValue::Ptr(bandana_Final_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter)
    }
}

unsafe extern "C" fn bandana_Final_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if [hash40("final"), hash40("final_air")].contains(&motion_kind) {
        if frame >= 25.0 && frame < 54.0 {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, final_dive_speed_y);
        }
        else {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        if MotionModule::is_end(fighter.module_accessor) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("final_hold"), 0.0, 1.0, false, 0.0, false, false);
            return 0.into();
        }
    }
    if motion_kind == hash40("final_hold") {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -final_hold_speed_y);
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        if stick_y <= -0.5 {
            GroundModule::pass_floor(fighter.module_accessor);
            if ray_check_pos(module_accessor, 0.0, -3.0, false) == 1 {
                fighter.change_status(FIGHTER_EDGE_STATUS_KIND_FINAL_END.into(), true.into());
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            }
        }
        else {
            GroundModule::pass_floor(fighter.module_accessor);
            if ray_check_pos(module_accessor, 0.0, -3.0, false) == 1 {
                fighter.change_status(FIGHTER_EDGE_STATUS_KIND_FINAL_END.into(), true.into());
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn status_bandana_Final_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_edge_final01_01"), 0);
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_edge_final01_02"), 0);
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_STATUS_KIND_FINAL)(fighter)
    }
}

pub fn install() {
    Agent::new("edge")
    .status(Pre, *FIGHTER_STATUS_KIND_FINAL, status_bandana_Final_Pre)
    .status(Main, *FIGHTER_STATUS_KIND_FINAL, status_bandana_Final_Main)
    .status(End, *FIGHTER_STATUS_KIND_FINAL, status_bandana_Final_End)
    .install();
}
