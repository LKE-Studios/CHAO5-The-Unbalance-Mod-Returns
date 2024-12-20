use crate::imports::BuildImports::*;
use crate::kamek::kamek::frame::*;

pub static speed_x_air : f32 = 3.2;
pub static speed_x_ground : f32 = 1.5;
pub static gravity_speed : f32 = 0.0;

unsafe extern "C" fn status_kamek_SpecialS_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {	
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
        0.into()
    }
    else {
        original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
    }
}

unsafe extern "C" fn status_kamek_SpecialS_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x_air);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x_air);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity_speed);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity_speed);
        }
        else {
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x_ground);
        }
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_kamek_SpecialS_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        let lr = PostureModule::lr(fighter.module_accessor);
        PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        JostleModule::set_status(fighter.module_accessor, false);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(kamek_SpecialS_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
    }
}

unsafe extern "C" fn kamek_SpecialS_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
        else if fighter.sub_air_check_stop_ceil().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_s").into(), Hash40::new("special_air_s").into(), true.into());
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y * gravity_speed);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT);
        }
    }
    if GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
        if frame >= 22.0 && frame <= 90.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR.into(), true.into());
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn status_kamek_SpecialS_CheckAttack(fighter: &mut L2CFighterCommon, param2: &L2CValue, param3: &L2CValue) -> L2CValue {
    let table = param3.get_table() as *mut smash2::lib::L2CTable;
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let module_accessor = sv_battle_object::module_accessor(object_id);
            if FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_ID_EFFECT[ENTRY_ID] == 1 {
                let mut params = CreateItemParam {
                    founder_pos: smash::Vector4f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor), w: 0.0},
                    item_pos: smash::Vector4f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor) + 12.0, z: PostureModule::pos_z(module_accessor), w: 0.0},
                    item_kind: ItemKind(*ITEM_KIND_MUSHROOM),
                    another_battle_object_id: *BATTLE_OBJECT_ID_INVALID as u32,
                    variation_kind: *ITEM_VARIATION_NONE,
                    lr_dir: PostureModule::lr(module_accessor),
                    owner_id: (*(module_accessor)).battle_object_id,
                    unk_20: 20,
                    pokeball_or_assist_kind: *ITEM_KIND_NONE,
                    unk_0: 0,
                    weird_flag: 0x633F800000,
                    unk_1_weird: 1,
                    unk_approx_0: 0.0,
                    unk_02: 0.0
                };
                KineticModule::clear_speed_all(module_accessor);
                let battle_object = create_item(ITEM_MANAGER, &mut params, false, false, false);
                let hover_stone = (*battle_object).module_accessor;
                StatusModule::change_status_request(hover_stone, *ITEM_STATUS_KIND_THROW, false);
                StatusModule::change_status_request(hover_stone, *ITEM_STATUS_KIND_LANDING, false);
            }
            else if FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_ID_EFFECT[ENTRY_ID] == 2 {
                let mut params = CreateItemParam {
                    founder_pos: smash::Vector4f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor), w: 0.0},
                    item_pos: smash::Vector4f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor) + 12.0, z: PostureModule::pos_z(module_accessor), w: 0.0},
                    item_kind: ItemKind(*ITEM_KIND_MUSHD),
                    another_battle_object_id: *BATTLE_OBJECT_ID_INVALID as u32,
                    variation_kind: *ITEM_VARIATION_NONE,
                    lr_dir: PostureModule::lr(module_accessor),
                    owner_id: (*(module_accessor)).battle_object_id,
                    unk_20: 20,
                    pokeball_or_assist_kind: *ITEM_KIND_NONE,
                    unk_0: 0,
                    weird_flag: 0x633F800000,
                    unk_1_weird: 1,
                    unk_approx_0: 0.0,
                    unk_02: 0.0
                };
                KineticModule::clear_speed_all(module_accessor);
                let battle_object = create_item(ITEM_MANAGER, &mut params, false, false, false);
                let hover_stone = (*battle_object).module_accessor;
                StatusModule::change_status_request(hover_stone,*ITEM_STATUS_KIND_THROW,false);
                StatusModule::change_status_request(hover_stone,*ITEM_STATUS_KIND_LANDING,false);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn status_kamek_SpecialS_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);  
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;  
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_ID_EFFECT[ENTRY_ID] = 0;
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
    }
}

pub fn install() {
    Agent::new("ness")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, status_kamek_SpecialS_Pre)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, status_kamek_SpecialS_Init)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, status_kamek_SpecialS_Main)
    .status(CheckAttack, *FIGHTER_STATUS_KIND_SPECIAL_S, status_kamek_SpecialS_CheckAttack)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, status_kamek_SpecialS_End)
    .install();
}