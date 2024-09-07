use crate::imports::BuildImports::*;

pub unsafe extern "C" fn start_waluigi_Init(fighter : &mut L2CFighterCommon) {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR); 
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);	
    let WALUIGI = color >= 120 && color <= 130;
    if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {  
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Bool(false));
        fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(waluigi_check_special_command as *const () as _));
    }
}

pub unsafe extern "C" fn waluigi_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR); 
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);	
    let WALUIGI = color >= 120 && color <= 130;
    if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {  
        return false.into();    
    }
    if waluigi_check_super_special_command(fighter).get_bool() {
        return true.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND)
    && waluigi_check_special_hi_command(fighter).get_bool() {
        return true.into();
    }
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_LW_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND.into(), true.into());
        return true.into();
    }
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_S_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND.into(), true.into());
        return true.into();
    }
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_S_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND.into(), true.into());
        return true.into();
    }
    false.into()
}

unsafe extern "C" fn waluigi_check_super_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR); 
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);	
    let WALUIGI = color >= 120 && color <= 130;
    if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {
        let cat4 = fighter.global_table[CMD_CAT4].get_i32();
        WorkModule::set_int(fighter.module_accessor, cat4, *FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_CAT4_SPECIAL_COMMAND);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
            if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND != 0
            && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2) {
                fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2.into(), true.into());
                return true.into();
            }
            if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND != 0
            && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL) {
                fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL.into(), true.into());
                return true.into();
            }
            if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_R_COMMAND != 0
            && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2) {
                let opplr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
                if opplr != 0.0 {
                    PostureModule::reverse_lr(fighter.module_accessor);
                }
                fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2.into(), true.into());
                return true.into();
            }
            if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND != 0
            && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL) {
                let opplr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
                if opplr != 0.0 {
                    PostureModule::reverse_lr(fighter.module_accessor);
                }
                fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL.into(), true.into());
                return true.into();
            }
        }
    }	
    false.into()
}

unsafe extern "C" fn waluigi_check_special_hi_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR); 
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);	
    let WALUIGI = color >= 120 && color <= 130;
    if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {
        let cat4 = fighter.global_table[CMD_CAT4].get_i32();
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI2_COMMAND != 0
        && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_HI_UNIQ].clone()).get_bool() {
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND.into(), true.into());
            return true.into();
        }
    }	
    false.into()
}

pub fn install() {
    Agent::new("dolly")
    .on_start(start_waluigi_Init)
    .install();
}