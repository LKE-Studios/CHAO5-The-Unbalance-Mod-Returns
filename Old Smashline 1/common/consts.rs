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
pub const FIGHTER_INSTANCE_WORK_ID_INT_LANDING_FRAME: i32 = 518;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_IS_OPPONENT_INVINCIBLE: i32 = 519;
pub const FIGHTER_STATUS_ATTACK_WORK_FLAG_CRITICAL: i32 = 0x100000B;
pub const FIGHTER_STATUS_GLIDE_WORK_FLAG_STOP_SE: i32 = 0x11000007;

//LINK
pub const FIGHTER_LINK_STATUS_KIND_SPECIAL_ASCENSION: i32 = 0x1D3;

//METAKNIGHT
pub const FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_N: i32 = 0x200000E6;
pub const FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_S: i32 = 0x200000E7;
pub const FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_HI: i32 = 0x200000E8;
pub const FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_LW: i32 = 0x200000E9;
pub const FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_AIR_DIVE: i32 = 0x1E9;
pub const FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_AIR_DIVE_END: i32 = 0x1EA;
pub const FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_AIR_DIVE_LANDING: i32 = 0x1EB;
pub const FIGHTER_METAKNIGHT_STATUS_SPECIAL_AIR_DIVE_WORK_FLOAT_ANGLE: i32 = 0x1100000C;

//PIT
pub const FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY: i32 = 0x1D3;
pub const FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN: i32 = 0x1ED;
pub const FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_END: i32 = 0x1EE;
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLOAT_GRAVITY: i32 = 0x100000E;
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLOAT_SPEED_X: i32 = 0x100000F;
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLOAT_SPEED_Y: i32 = 0x1000010;
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_TIME: i32 = 0x1000011;

//PLIZARDON
pub const FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_HI2: i32 = 0x1DB;
pub const FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_HI2_LANDING: i32 = 0x1D3;

//PZENIGAME
pub const FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_BLOW: i32 = 0x1D3;

//PFUSHIGISOU
pub const FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N2_CHARGE: i32 = 0x1D3;
pub const FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N2_SHOOT: i32 = 0x1DA;
pub const FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N2_END: i32 = 0x1DB;

//DIDDY
pub const FIGHTER_DIDDY_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LAUGH_TRIGGER: i32 = 0x200000E4;
pub const FIGHTER_DIDDY_INSTANCE_WORK_ID_INT_BANANA_ID: i32 = 0x200000E5;

//LUCARIO
pub const FIGHTER_LUCARIO_INSTANCE_WORK_ID_ENABLE_SPECIAL_HI: i32 = 0x200000E3;
pub const FIGHTER_LUCARIO_INSTANCE_WORK_ID_DISABLE_SPECIAL_HI: i32 = 0x200000E4;
pub const FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_AURA_POWER_INVINCIBLE: i32 = 0x200000E5;

//SILVER
pub const FIGHTER_SILVER_STATUS_SPECIAL_N_WORK_FLOAT_ANGLE: i32 = 0x100000C;

//WALUIGI
pub const FIGHTER_WALUIGI_STATUS_SPECIAL_DICE_BLOCK_WORK_ID_FLAG_VISIBLE: i32 = 0x200000E1;
pub const FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_S: i32 = 0x1D3;
pub const FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_S_DIVE: i32 = 0x1D2;
pub const FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_S_LANDING: i32 = 0x1D1;
pub const FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_LW_SHIELD: i32 = 0x1D0;
pub const FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_LW_ATTACK: i32 = 0x1D5;
pub const FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_LW_JUMP: i32 = 0x1CF;
pub const FIGHTER_WALUIGI_STATUS_KIND_SPECIAL_LW_SPECIAL: i32 = 0x1C0;
pub const FIGHTER_WALUIGI_STATUS_SPECIAL_N_DICE_BLOCK_WORK_ID_FLAG_VISIBLE: i32 = 0x200000EC;

//NINTEN
pub const FIGHTER_NINTEN_STATUS_KIND_SPECIAL_HI_ATTACK: i32 = 0x1D3;
pub const FIGHTER_NINTEN_STATUS_KIND_SPECIAL_HI_END: i32 = 0x1D2;