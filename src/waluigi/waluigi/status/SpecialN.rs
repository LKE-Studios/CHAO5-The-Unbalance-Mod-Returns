use crate::imports::BuildImports::*;

pub static num_9_recover_amount : f32 = 180.0;

unsafe extern "C" fn status_waluigi_SpecialN_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_n").into());
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
    }
    waluigi_SpecialN_diceblock_helper(fighter);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE_DONE);
    fighter.sub_shift_status_main(L2CValue::Ptr(waluigi_SpecialN_Main_loop as *const () as _))
}

unsafe extern "C" fn waluigi_SpecialN_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let hop_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("hop_speed_y"));
    let gravity_accel = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("gravity_accel"));
    let gravity_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("gravity_max"));
    if fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE) {
            ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_DOLLY_GENERATE_ARTICLE_WAVE, false, -1);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE) {
                if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, hop_speed_y);
                    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -gravity_accel);
                    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity_max);
                }
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE_DONE);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE);
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE_DONE) {
            if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
                sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_STOP_RESET_TYPE_AIR, 0.0, speed_y, 0.0, 0.0, 0.0);
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -gravity_accel);
                sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity_max);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            else {
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
        else {
            fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_n").into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

pub unsafe extern "C" fn waluigi_SpecialN_diceblock_helper(fighter: &mut L2CFighterCommon) {
    let frame = MotionModule::frame(fighter.module_accessor);
    let mut rand_num = WorkModule::get_int(fighter.module_accessor, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
    rand_num = sv_math::rand(hash40("dolly"), 10);
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_FRAME);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_WALUIGI_INSTANCE_WORK_ID_FLAG_DICEBLOCK_INVISIBLE) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("dolly_Kart_Glider_VIS_O_OBJShape"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10_trans"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"), false);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_WALUIGI_INSTANCE_WORK_ID_FLAG_DICEBLOCK_SELECT_NUM) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"), true);
        if rand_num == 0 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), true); 
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_WALUIGI_INSTANCE_WORK_ID_FLAG_DICEBLOCK_OUT);
            WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        }
        else if rand_num == 1 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), true);
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_WALUIGI_INSTANCE_WORK_ID_FLAG_DICEBLOCK_OUT);
            WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        }
        else if rand_num == 2  {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), true);
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_WALUIGI_INSTANCE_WORK_ID_FLAG_DICEBLOCK_OUT);
            WorkModule::set_int(fighter.module_accessor, 2, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        }
        else if rand_num == 3  {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), true);
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_WALUIGI_INSTANCE_WORK_ID_FLAG_DICEBLOCK_OUT);
            WorkModule::set_int(fighter.module_accessor, 3, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        }
        else if rand_num == 4  {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), true);
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_WALUIGI_INSTANCE_WORK_ID_FLAG_DICEBLOCK_OUT);
            WorkModule::set_int(fighter.module_accessor, 4, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        }
        else if rand_num == 5  {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), true);
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_WALUIGI_INSTANCE_WORK_ID_FLAG_DICEBLOCK_OUT);
            WorkModule::set_int(fighter.module_accessor, 5, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        }
        else if rand_num == 6 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), true);
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_WALUIGI_INSTANCE_WORK_ID_FLAG_DICEBLOCK_OUT);
            WorkModule::set_int(fighter.module_accessor, 6, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        }
        else if rand_num == 7  {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), true);
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_WALUIGI_INSTANCE_WORK_ID_FLAG_DICEBLOCK_OUT);
            WorkModule::set_int(fighter.module_accessor, 7, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        }
        else if rand_num == 8  {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), true);
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_WALUIGI_INSTANCE_WORK_ID_FLAG_DICEBLOCK_OUT);
            DamageModule::heal(fighter.module_accessor, -num_9_recover_amount, 0);
            WorkModule::set_int(fighter.module_accessor, 8, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        }
        else if rand_num == 9  {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), true);
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_WALUIGI_INSTANCE_WORK_ID_FLAG_DICEBLOCK_OUT);
            WorkModule::set_int(fighter.module_accessor, 9, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        }
    }
}

pub fn install() {
    Agent::new("dolly")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, status_waluigi_SpecialN_Main)
    .install();
}