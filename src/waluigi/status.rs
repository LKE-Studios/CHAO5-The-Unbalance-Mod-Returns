use crate::imports::BuildImports::*;

#[status_script( agent = "dolly", status = FIGHTER_STATUS_KIND_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn status_waluigi_wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
	let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {	
        fighter.status_pre_Wait()
    }
    else {
        original!(fighter)
    }	
}

#[status_script( agent = "dolly", status = FIGHTER_STATUS_KIND_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn status_waluigi_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {
        fighter.status_Wait()
    }
    else {
        original!(fighter)
    }	
}

#[status_script( agent = "dolly", status = FIGHTER_STATUS_KIND_SQUAT_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn status_waluigi_squatwait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {	
        fighter.status_SquatWait()
    }
    else {
        original!(fighter)
    }	
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_WALK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn status_waluigi_walk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {		
        fighter.status_Walk()
    }
    else {
        original!(fighter)
    }	
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_TURN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn status_waluigi_turn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {			
        fighter.status_pre_Turn()
    }
    else {
        original!(fighter)
    }	
}

#[status_script( agent = "dolly", status = FIGHTER_STATUS_KIND_TURN_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn status_waluigi_turndash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {		
        fighter.status_pre_TurnDash()
    }
    else {
        original!(fighter)
    }	
}

#[status_script( agent = "dolly", status = FIGHTER_STATUS_KIND_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn status_waluigi_landing_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {	
        fighter.sub_landing_uniq_process_init()
    }
    else {
        original!(fighter)
    }	
}

#[status_script( agent = "dolly", status = FIGHTER_STATUS_KIND_LANDING_LIGHT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn status_waluigi_landinglight_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {	
        fighter.sub_landing_uniq_process_init()
    }
    else {
        original!(fighter)
    }	
}

#[status_script( agent = "dolly", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn status_waluigi_special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {	
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, true, *FIGHTER_TREADED_KIND_ENABLE, true, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
        0.into()
    }
    else {
        original!(fighter)
    }
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn status_waluigi_special_sf_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {   
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, true, *FIGHTER_TREADED_KIND_ENABLE, true, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
        0.into()
    }
    else {
        original!(fighter)
    }
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn status_waluigi_special_s_command_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {  
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, true, *FIGHTER_TREADED_KIND_ENABLE, true, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
        0.into()
    }
    else {
        original!(fighter)
    }
}

#[fighter_init]
fn waluigi_agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR); 
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);	
        let WALUIGI = color >= 120 && color <= 130;
	    if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {  
            fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Bool(false));
            fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(waluigi_check_special_command as *const () as _));
        }
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
    install_status_scripts!(
        status_waluigi_wait_pre,
        status_waluigi_wait_main,
        status_waluigi_squatwait_main,
        status_waluigi_walk_main,
        status_waluigi_turn_pre,
        status_waluigi_turndash_pre,
        status_waluigi_landing_init,
        status_waluigi_landinglight_init,
        status_waluigi_special_s_pre,
        status_waluigi_special_sf_pre,
        status_waluigi_special_s_command_pre
    );
    install_agent_init_callbacks!(
        waluigi_agent_init
    );
}

