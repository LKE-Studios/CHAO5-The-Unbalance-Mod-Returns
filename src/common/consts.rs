use crate::imports::BuildImports::*;

//COMMON
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START: i32 = 495;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER: i32 = 496;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK: i32 = 497; //Tracks if a fighter used a certain special move in the air
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD: i32 = 498;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED: i32 = 499;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT: i32 = 500;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT: i32 = 501;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE: i32 = 502;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK: i32 = 503;
pub const FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT: i32 = 504; //Tracks if a player got hit during One-Hit mode
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW: i32 = 505;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH: i32 = 506;
pub const FIGHTER_INSTANCE_WORK_ID_INT_MASHING: i32 = 507;
pub const FIGHTER_INSTANCE_WORK_ID_INT_PARRIED: i32 = 508;
pub const FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER: i32 = 509;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO: i32 = 510; //Returns false for exactly one frame after is_ready_go becomes true, used to initiate certain events exactly once at the start of a match
pub const FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER: i32 = 511;
pub const FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_GFX_TIMER: i32 = 512;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL: i32 = 513;
pub const FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX: i32 = 514;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS: i32 = 515; //Flags when you just used a Final Smash in Special Smash
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH_DONE: i32 = 516;
pub const FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_XLU_FRAME: i32 = 517;

//DIDDY
pub const FIGHTER_DIDDY_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LAUGH_TRIGGER: i32 = 0x200000E4;
pub const FIGHTER_DIDDY_INSTANCE_WORK_ID_INT_BANANA_ID: i32 = 0x200000E5;

//METAKNIGHT
pub const FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_SPECIAL_N: i32 = 0x200000E6;

pub unsafe fn metaknight_special_n_disable (fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind == *FIGHTER_KIND_METAKNIGHT {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_SPECIAL_N) {
            METAKNIGHT_DISABLE_SPECIAL_N[ENTRY_ID] = true;
        }
    }
}