use crate::imports::BuildImports::*;
use crate::silver::silver::status::SpecialHi::*;

unsafe extern "C" fn status_silver_SpecialHi2_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SILVER = color >= 120 && color <= 127;
    if SILVER {
        silver_SpecialHi_function(fighter);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        if !StopModule::is_stop(fighter.module_accessor) {
            silver_SpecialHi2_Sub_Status(fighter, false.into());
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(silver_SpecialHi2_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2)(fighter)
    } 
}

unsafe extern "C" fn silver_SpecialHi2_Sub_Status(fighter: &mut L2CFighterCommon, param_3: L2CValue) -> L2CValue {
    if !param_3.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        let int_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_FRAME); 
        if int_frame >= 2 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_FLAG_CHECK_GROUND); 
        }
    }
    else {
        let int_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_FRAME); 
        let move_xlu = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("move_xlu"));
        let move_cliff_check = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("move_cliff_check"));
        if int_frame == move_xlu {
            GroundModule::set_passable_check(fighter.module_accessor, false);
            if move_cliff_check == 0 {
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
            }
        }
    }
    0.into()
}

unsafe extern "C" fn silver_SpecialHi2_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3.into(), true.into());
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    let int_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_FRAME);
    let move_time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("move_time"));
    0.into()
}

pub fn install() {
    Agent::new("mewtwo")
    .status(Main, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, status_silver_SpecialHi2_Main)
    .install();
}
