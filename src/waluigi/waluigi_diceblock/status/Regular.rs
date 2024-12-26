use crate::imports::BuildImports::*;

unsafe extern "C" fn status_waluigi_diceblock_Regular_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn status_waluigi_diceblock_Regular_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    waluigi_Regular_helper(weapon);
    weapon.fastshift(L2CValue::Ptr(waluigi_diceblock_Regular_Main_loop as *const () as _))
}

unsafe extern "C" fn waluigi_Regular_helper(weapon: &mut L2CWeaponCommon) {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let rand_num = WorkModule::get_int(owner_module_accessor, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
    WorkModule::add_int(owner_module_accessor, 1, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_FRAME);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("frame_dice"), true);
    if rand_num == 0  {
        WorkModule::set_int(owner_module_accessor, 0, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        ModelModule::set_scale(weapon.module_accessor, 1.0);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_1"), true);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if rand_num == 1 {
        WorkModule::set_int(owner_module_accessor, 1, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        ModelModule::set_scale(weapon.module_accessor, 1.0);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_2"), true);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if rand_num == 2  {
        WorkModule::set_int(owner_module_accessor, 2, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        ModelModule::set_scale(weapon.module_accessor, 1.0);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_3"), true);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if rand_num == 3 {
        WorkModule::set_int(owner_module_accessor, 3, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        ModelModule::set_scale(weapon.module_accessor, 1.0);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_4"), true);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if rand_num == 4  {
        WorkModule::set_int(owner_module_accessor, 4, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        ModelModule::set_scale(weapon.module_accessor, 1.0);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_5"), true);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if rand_num == 5  {
        WorkModule::set_int(owner_module_accessor, 5, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        ModelModule::set_scale(weapon.module_accessor, 1.0);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_6"), true);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if rand_num == 6 {
        WorkModule::set_int(owner_module_accessor, 6, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        ModelModule::set_scale(weapon.module_accessor, 1.1);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_7"), true);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if rand_num == 7  {
        WorkModule::set_int(owner_module_accessor, 7, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        ModelModule::set_scale(weapon.module_accessor, 1.2);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_8"), true);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if rand_num == 8  {
        WorkModule::set_int(owner_module_accessor, 8, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        ModelModule::set_scale(weapon.module_accessor, 1.5);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_9"), true);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_10"), false);
    }
    else if rand_num == 9  {
        WorkModule::set_int(owner_module_accessor, 9, FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
        ModelModule::set_scale(weapon.module_accessor, 1.75);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_10"), true);
    }
}

unsafe extern "C" fn waluigi_diceblock_Regular_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let remaining_life = life <= 0;
    if !remaining_life {
        if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
            return 0.into();
        }
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        weapon.pop_lua_stack(1);
        MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_EFFECT, Hash40::new("effect_bound"), -1);
        if remaining_life {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            weapon.pop_lua_stack(1);
            return 0.into();
        }
    }
    else {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    0.into()
}

unsafe extern "C" fn status_waluigi_diceblock_Regular_Exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
	WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
	let frame = MotionModule::frame(weapon.module_accessor);
	let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
	if frame <= 1.0 {
		WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
	}
	if life < 20 {
		StatusModule::change_status_force(weapon.module_accessor, WEAPON_WALUIGI_DICEBLOCK_STATUS_KIND_BREAK, false);
	}
	0.into()
}

pub fn install() {
    Agent::new("dolly_diceblock")
	.status(Pre, *WEAPON_MARIO_FIREBALL_STATUS_KIND_REGULAR, status_waluigi_diceblock_Regular_Pre)
	.status(Main, *WEAPON_MARIO_FIREBALL_STATUS_KIND_REGULAR, status_waluigi_diceblock_Regular_Main)
    .status(Exec, *WEAPON_MARIO_FIREBALL_STATUS_KIND_REGULAR, status_waluigi_diceblock_Regular_Exec)
    .install();
}