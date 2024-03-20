use crate::imports::BuildImports::*;

unsafe extern "C" fn status_murabito_SpecialLwPlant_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn status_murabito_SpecialLwPlant_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_plant"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(murabito_SpecialLwPlant_Main_Loop as *const () as _))
}

unsafe extern "C" fn murabito_SpecialLwPlant_function(fighter: &mut L2CFighterCommon) -> bool {
    let check_offset_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("check_offset_x"));
    let check_offset_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("check_offset_y"));
    let check_length = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("check_length"));
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let check_special_lw_plant = FighterSpecializer_Murabito::check_special_lw_plant(module_accessor, Vector2f{x: check_offset_y, y: -check_length}, check_offset_x);
    check_special_lw_plant.into()
}

unsafe extern "C" fn murabito_SpecialLwPlant_Main_Loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_DEFOREST.into(), false.into());
    }
    else {
        if !MotionModule::is_end(fighter.module_accessor) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_LW_PLANT_FLAG_CHECK_PLANT) {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_LW_PLANT_FLAG_CHECK_PLANT);
                if murabito_SpecialLwPlant_function(fighter) {
                    let lr = PostureModule::lr(fighter.module_accessor);
                    let back_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("back_speed"));
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, back_speed * (lr * -1.0), 0.0);
                    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                }
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_LW_PLANT_FLAG_PLANT) {
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                if !murabito_SpecialLwPlant_function(fighter) {
                    fighter.change_status(FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_DEFOREST.into(), false.into());
                }
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_SPROUT, false, -1);
                ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_SEED, ArticleOperationTarget(-1));
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_LW_PLANT_FLAG_SUCCESS);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_LW_PLANT_FLAG_PLANT);
            }
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    1.into()
}

pub fn install() {
    Agent::new("murabito")
    .status(Pre, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT, status_murabito_SpecialLwPlant_Pre)
    .status(Main, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT, status_murabito_SpecialLwPlant_Main)
    .install();
}