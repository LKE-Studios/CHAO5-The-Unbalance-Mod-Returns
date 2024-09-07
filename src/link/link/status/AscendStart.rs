use crate::imports::BuildImports::*;

unsafe extern "C" fn status_link_AscendStart_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_CLIFF, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn status_link_AscendStart_Init(fighter: &mut L2CFighterCommon) -> L2CValue { 
    GroundModule::set_ignore_boss(fighter.module_accessor, true);
    GroundModule::set_passable_check(fighter.module_accessor, false);
    GroundModule::set_collidable(fighter.module_accessor, false);
    JostleModule::set_status(fighter.module_accessor, false);
    KineticModule::clear_speed_all(fighter.module_accessor);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    0.into()
}

unsafe extern "C" fn status_link_AscendStart_Main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    let cbm_t_vec1 = Vector4f{ /* Red */ x: 1.0, /* Green */ y: 1.0, /* Blue */ z: 1.0, /* Alpha */ w: 1.0};
    let cbm_t_vec2 = Vector4f{ /* Red */ x: 0.25, /* Green */ y: 1.0, /* Blue */ z: 0.75, /* Alpha */ w: 0.5};
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("ascend_start"), 0.0, 1.0, false, 0.0, false, false);
    ColorBlendModule::set_main_color(fighter.module_accessor, /* Brightness */ &cbm_t_vec1, /* Diffuse */ &cbm_t_vec2, 0.21, 2.2, 3, /* Display Color */ true);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_AscendStart_Main_loop as *const () as _))
}

unsafe extern "C" fn link_AscendStart_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {    
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_ASCEND.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn status_link_AscendStart_End(fighter: &mut L2CFighterCommon) -> L2CValue { 
    0.into()
}

pub fn install() {
    Agent::new("link")
    .status(Pre, FIGHTER_LINK_STATUS_KIND_ASCEND_START, status_link_AscendStart_Pre)
    .status(Init, FIGHTER_LINK_STATUS_KIND_ASCEND_START, status_link_AscendStart_Init)
    .status(Main, FIGHTER_LINK_STATUS_KIND_ASCEND_START, status_link_AscendStart_Main)
    .status(End, FIGHTER_LINK_STATUS_KIND_ASCEND_START, status_link_AscendStart_End)
    .install();
}