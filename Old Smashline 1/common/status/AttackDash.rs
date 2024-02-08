use crate::imports::BuildImports::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_AttackDash)]
unsafe fn status_attackdash(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_dash"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
    let catch_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_dash_frame"));
    WorkModule::set_int(fighter.module_accessor, catch_dash_frame, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_CATCH_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_FLAG_ATTACK_HI4_DISABLE);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
        let jump_mini_attack_enable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("jump_mini_attack_enable_frame"));
        WorkModule::set_int(fighter.module_accessor, jump_mini_attack_enable_frame + 1, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
    let log = fighter.status_attack()["log_infos"]["attack_dash"].get_int();
    WorkModule::set_int64(fighter.module_accessor, log as i64, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let mini_jump_attack = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    if mini_jump_attack != 0 {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK)
        && WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) > 0 {
            let log = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_attack_dash_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(smash::lua2cpp::L2CFighterCommon_sub_attack_dash_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(smash::lua2cpp::L2CFighterCommon_bind_address_call_status_AttackDash_Main as *const () as _))
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_AttackDash_Main)]
unsafe fn status_attackdash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_kind = smash::app::utility::get_kind(boma);
    let frame = MotionModule::frame(boma);
    let const_stick_x = fighter.global_table[STICK_X].get_f32(); 
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = const_stick_x * lr;
    let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"));
    let f5 = [*FIGHTER_KIND_FOX, *FIGHTER_KIND_SONIC];
    let f6 = [*FIGHTER_KIND_PURIN, *FIGHTER_KIND_SHEIK, *FIGHTER_KIND_WARIO];
    let f7 = [*FIGHTER_KIND_DAISY, *FIGHTER_KIND_MARIOD, *FIGHTER_KIND_JACK, *FIGHTER_KIND_MARIO, *FIGHTER_KIND_MIIFIGHTER, *FIGHTER_KIND_GAMEWATCH, *FIGHTER_KIND_PALUTENA, *FIGHTER_KIND_PEACH, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_PICHU, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_ROSETTA, *FIGHTER_KIND_SNAKE, *FIGHTER_KIND_WIIFIT, *FIGHTER_KIND_ZELDA];
    let f8 = [*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_PITB, *FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_KEN, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_LITTLEMAC, *FIGHTER_KIND_LUCARIO, *FIGHTER_KIND_ROCKMAN, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_PIT, *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_RYU, *FIGHTER_KIND_TRAIL, *FIGHTER_KIND_TOONLINK, *FIGHTER_KIND_SZEROSUIT];
    let f9 = [*FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_DIDDY, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_INKLING, *FIGHTER_KIND_NESS, *FIGHTER_KIND_PIKMIN, *FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_REFLET, *FIGHTER_KIND_PICKEL, *FIGHTER_KIND_YOUNGLINK];
    let f10 = [*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_MASTER, *FIGHTER_KIND_CLOUD, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_POPO, *FIGHTER_KIND_NANA, *FIGHTER_KIND_KIRBY, *FIGHTER_KIND_MIISWORDSMAN, *FIGHTER_KIND_ELIGHT, *FIGHTER_KIND_MURABITO, *FIGHTER_KIND_LUIGI];
    let f11 = [*FIGHTER_KIND_DUCKHUNT, *FIGHTER_KIND_GANON, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_MIIGUNNER, *FIGHTER_KIND_PACMAN, *FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_RICHTER, *FIGHTER_KIND_SIMON, *FIGHTER_KIND_DOLLY, *FIGHTER_KIND_YOSHI];
    let f12 = [*FIGHTER_KIND_KOOPA, *FIGHTER_KIND_WOLF, *FIGHTER_KIND_SAMUS];
    let f13 = [*FIGHTER_KIND_KAMUI, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_SHULK];
    let f14 = [*FIGHTER_KIND_CHROM, *FIGHTER_KIND_LUCAS, *FIGHTER_KIND_LUCINA, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_ROY];
    let f15 = [*FIGHTER_KIND_EDGE];
    let f16 = [*FIGHTER_KIND_BAYONETTA, *FIGHTER_KIND_IKE, *FIGHTER_KIND_DEMON, *FIGHTER_KIND_METAKNIGHT];
    let f18 = [*FIGHTER_KIND_EFLAME];
    let f21 = [*FIGHTER_KIND_LINK];
    let f22 = [*FIGHTER_KIND_BRAVE];
    let f23 = [*FIGHTER_KIND_DEDEDE];
    if CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
        return 0.into();
    }
    let mini_jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    if 0 < mini_jump_attack_frame {
        if !StopModule::is_stop(fighter.module_accessor)
        && fighter.sub_check_button_jump().get_bool() {
            let log = fighter.status_attack();
            let info = log[0x10f40d7b92u64].get_i64();
            let mot = MotionModule::motion_kind(fighter.module_accessor);
            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(mot), -1);
            WorkModule::set_int64(fighter.module_accessor, info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            fighter.change_status_jump_mini_attack(true.into());
            return 1.into();
        }
    }
    if 1 == mini_jump_attack_frame {
        if !fighter.global_table[IS_STOP].get_bool()
        && WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) > 0 {
            let log = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    let turn_run_check = {stick_x * lr <= turn_run_stick_x};
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN)
    && turn_run_check
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && !ItemModule::is_have_item(fighter.module_accessor, 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && !ItemModule::is_have_item(fighter.module_accessor, 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
        return 0.into();
    }
    /* START OF NEW ADDITIONS */
    //DACUS and DACDS
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) != true
    && ((f5.contains(&fighter_kind) && frame <= 5.0) 
    || (f6.contains(&fighter_kind) && frame <= 6.0)
    || (f7.contains(&fighter_kind) && frame <= 7.0) 
    || (f8.contains(&fighter_kind) && frame <= 8.0) 
    || (f9.contains(&fighter_kind) && frame <= 9.0) 
    || (f10.contains(&fighter_kind) && frame <= 10.0)
    || (f11.contains(&fighter_kind) && frame <= 11.0)
    || (f12.contains(&fighter_kind) && frame <= 12.0)
    || (f13.contains(&fighter_kind) && frame <= 13.0)
    || (f14.contains(&fighter_kind) && frame <= 14.0)
    || (f15.contains(&fighter_kind) && frame <= 15.0)
    || (f16.contains(&fighter_kind) && frame <= 16.0)
    || (f18.contains(&fighter_kind) && frame <= 18.0)
    || (f21.contains(&fighter_kind) && frame <= 21.0)
    || (f22.contains(&fighter_kind) && frame <= 22.0)
    || (f23.contains(&fighter_kind) && frame <= 23.0)) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START) || WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START) {
            if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0 || (fighter.global_table[STICK_Y].get_f32() > 0.7 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4_START.into(), true.into());
                return 1.into();
            }
            if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 != 0 || (fighter.down_input() && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW4_START.into(), true.into());
                return 1.into();
            }
        }
    }
    //Samus Dash Attack Canceled Up Tilt + DACDS
    if fighter_kind == *FIGHTER_KIND_SAMUS {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) != true
        && frame <= 9.0 {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3) {
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0 || (fighter.global_table[STICK_Y].get_f32() > 0.7 && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)) {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI3.into(), true.into());
                    return 1.into();
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START) {
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 != 0 || (fighter.down_input() && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)) {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW4_START.into(), true.into());
                    return 1.into();
                }
            }
        }
    }
    /* END OF NEW ADDITIONS */
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_FALL
        }
        else if WorkModule::get_param_int(fighter.module_accessor, 0x17e10662a4, 0) == *FIGHTER_ATTACK_DASH_TYPE_SQUAT_WAIT {
            FIGHTER_STATUS_KIND_SQUAT_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_WAIT
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_attackdash,
            status_attackdash_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}