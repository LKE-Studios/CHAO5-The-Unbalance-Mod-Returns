use crate::imports::BuildImports::*;

unsafe extern "C" fn status_link_AscendEnd_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn status_link_AscendEnd_Init(fighter: &mut L2CFighterCommon) -> L2CValue { 
    GroundModule::set_collidable(fighter.module_accessor, true);
    GroundModule::set_gr_collision_mode(fighter.module_accessor, true);
    JostleModule::set_status(fighter.module_accessor, true);
    KineticModule::clear_speed_all(fighter.module_accessor);
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    0.into()
}

unsafe extern "C" fn status_link_AscendEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        PostureModule::add_pos(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    }
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("ascend_end"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_AscendEnd_Main_loop as *const () as _))
}

unsafe extern "C" fn link_AscendEnd_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {    
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::update_trans_move_speed(fighter.module_accessor);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn status_link_AscendEnd_End(fighter: &mut L2CFighterCommon) -> L2CValue { 
    WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
    WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_START_Y);
    WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ASCEND_FRAME);
    0.into()
}

pub fn install() {
    Agent::new("link")
    .status(Pre, FIGHTER_LINK_STATUS_KIND_ASCEND_END, status_link_AscendEnd_Pre)
    .status(Init, FIGHTER_LINK_STATUS_KIND_ASCEND_END, status_link_AscendEnd_Init)
    .status(Main, FIGHTER_LINK_STATUS_KIND_ASCEND_END, status_link_AscendEnd_Main)
    .status(Exit, FIGHTER_LINK_STATUS_KIND_ASCEND_END, status_link_AscendEnd_End)
    .install();
}