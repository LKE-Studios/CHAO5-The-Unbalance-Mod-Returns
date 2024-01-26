use crate::imports::BuildImports::*; 

pub static mut FIGHTER_BOOL_1: [bool; 9] = [false; 9];
pub static mut FIGHTER_BOOL_2: [bool; 9] = [false; 9];
pub static mut FIGHTER_BOOL_3: [bool; 9] = [false; 9];

#[skyline::hook(replace=smash::app::FighterUtil::is_valid_just_shield_reflector)]
unsafe fn is_valid_just_shield_reflector(_module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    return true;
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Landing_MainSub)]
pub unsafe fn status_landing_main_sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_ESCAPE_AIR || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        ControlModule::clear_command_one(module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
        ControlModule::clear_command_one(module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
        ControlModule::clear_command_one(module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);
    }
    original!()(fighter)
}

// Use this for general per-frame weapon-level hooks
// #[weapon_frame_callback]
// pub fn global_weapon_frame(fighter_base : &mut L2CFighterBase) {
//     unsafe {
//         let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
//         let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;

//         if frame % 10 == 0 {
//             println!("[Weapon Hook] Frame : {}", frame);
//         }
//     }
// }

pub unsafe fn get_player_number(module_accessor:  &mut smash::app::BattleObjectModuleAccessor) -> usize {
	if smash::app::utility::get_kind(module_accessor) == *WEAPON_KIND_PTRAINER_PTRAINER {
		WorkModule::get_int(module_accessor, *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_INT_FIGHTER_ENTRY_ID) as usize
	}
	else if smash::app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize
	}
	else {
		let mut owner_module_accessor = &mut *smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		while smash::app::utility::get_category(owner_module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER { //Keep checking the owner of the boma we're working with until we've hit a boma that belongs to a fighter
			owner_module_accessor = &mut *smash::app::sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		}
		WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize
	}
}

pub mod status;
pub mod frame;
mod param;
pub mod consts;
pub mod function;

pub fn is_glider(kind: i32) -> bool {
    [
        *FIGHTER_KIND_METAKNIGHT,
        *FIGHTER_KIND_PIT,
        *FIGHTER_KIND_PITB,
        *FIGHTER_KIND_PLIZARDON,
        *FIGHTER_KIND_RIDLEY,
        *FIGHTER_KIND_BUDDY,
        *FIGHTER_KIND_TRAIL,
        *FIGHTER_KIND_PALUTENA
    ].contains(&kind)
}

/*static mut SFX_COUNTER : [i32; 8] = [0; 8];

#[fighter_frame_callback(main)]
pub fn loupe_camera(fighter : &mut L2CFighterCommon) {
    unsafe {
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_LOUPE) {
            SFX_COUNTER[ENTRY_ID] += 1;
            if SFX_COUNTER[ENTRY_ID] == 1 {
                macros::PLAY_SE(fighter, Hash40::new("se_common_warning_out"));
            };
        }
        else {
            macros::STOP_SE(fighter, Hash40::new("se_common_warning_out"));
            SFX_COUNTER[ENTRY_ID] = 0;
        }
    }
}

#[fighter_frame_callback(main)]
pub fn loupe_off(fighter : &mut L2CFighterCommon) {
    unsafe {
        if sv_information::is_ready_go() == false {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_LOUPE);
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_LOUPE) {
            macros::STOP_SE(fighter, Hash40::new("se_common_warning_out"));
        }
    }
}

#[fighter_reset]
pub fn loupe_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        SFX_COUNTER[ENTRY_ID] = 0;
    }
}*/

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_landing_main_sub
        );
    }
}

pub fn install() {
    skyline::install_hook!(
        is_valid_just_shield_reflector
    );
    skyline::nro::add_hook(nro_hook);
    status::install();
    frame::install();
    param::install();
    function::install();
}