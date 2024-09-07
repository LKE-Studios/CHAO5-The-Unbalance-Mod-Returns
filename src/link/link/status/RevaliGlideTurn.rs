use crate::imports::BuildImports::*;

unsafe extern "C" fn status_link_RevaliGlideTurn_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn status_link_RevaliGlideTurn_Init(fighter: &mut L2CFighterCommon) -> L2CValue { 
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let glide_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_ascend_revali"), hash40("glide_speed_x"));
    let gravity_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_ascend_revali"), hash40("gravity_speed_mul"));
    let glide_turn_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_ascend_revali"), hash40("glide_turn_mul"));
    let lr = PostureModule::lr(fighter.module_accessor);
    let limit_y_mul = gravity_speed_mul;
    let new_gravity_speed = air_speed_y_stable * limit_y_mul;
    PostureModule::reverse_lr(fighter.module_accessor);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, new_gravity_speed);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, new_gravity_speed);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, glide_speed_x * glide_turn_mul, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, (glide_speed_x * lr) * glide_turn_mul, 0.0);
    0.into()
}

unsafe extern "C" fn status_link_RevaliGlideTurn_Main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("revali_glide_turn"), 0.0, 1.0, false, 0.0, false, false);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
    }
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("glide_turn"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_RevaliGlideTurn_Main_loop as *const () as _))
}

unsafe extern "C" fn link_RevaliGlideTurn_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {    
    let get_stick_prev_y = ControlModule::get_stick_prev_y(fighter.module_accessor);
    let glide_drop_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_ascend_revali"), hash40("glide_drop_stick_y"));
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if get_stick_prev_y < glide_drop_stick_y {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_REVALI_GLIDE_DROP.into(), false.into());
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_REVALI_GLIDE_LANDING.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_REVALI_GLIDE.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn status_link_RevaliGlideTurn_End(fighter: &mut L2CFighterCommon) -> L2CValue { 
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

pub fn install() {
    Agent::new("link")
    .status(Pre, FIGHTER_LINK_STATUS_KIND_REVALI_GLIDE_TURN, status_link_RevaliGlideTurn_Pre)
    .status(Init, FIGHTER_LINK_STATUS_KIND_REVALI_GLIDE_TURN, status_link_RevaliGlideTurn_Init)
    .status(Main, FIGHTER_LINK_STATUS_KIND_REVALI_GLIDE_TURN, status_link_RevaliGlideTurn_Main)
    .status(End, FIGHTER_LINK_STATUS_KIND_REVALI_GLIDE_TURN, status_link_RevaliGlideTurn_End)
    .install();
}