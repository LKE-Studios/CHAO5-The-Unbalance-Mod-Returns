#![allow(dead_code)]

use smash::{
    *,
    phx::Hash40,
    lib::lua_const::*,
    app::{lua_bind::*, *}
};
use smash::lib::L2CAgent;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Vector2f;
use bitflags::bitflags;
use skyline::{
    c_str,
    from_c_str,
    hooks::{
        getRegionAddress,
        InlineCtx,
        Region
    },
    nn::ro::LookupSymbol,
};

// Transition Hook static muts:
// 0 - Don't change 
// 1 - Force off
// 2 - Force on (doesnt really work)
pub static mut CAN_UPB: [i32; 8] = [0; 8];
pub static mut CAN_SIDEB: [i32; 8] = [0; 8];
pub static mut CAN_DOWNB: [i32; 8] = [0; 8];
pub static mut CAN_NEUTRALB: [i32; 8] = [0; 8];
pub static mut CAN_JUMP_SQUAT: [i32; 8] = [0; 8];
pub static mut CAN_DOUBLE_JUMP: [i32; 8] = [0; 8];
pub static mut CAN_CLIFF: [i32; 8] = [0; 8];
pub static mut CAN_ATTACK_AIR: [i32; 8] = [0; 8];
pub static mut CAN_AIRDODGE: [i32; 8] = [0; 8];
pub static mut CAN_RAPID_JAB: [i32; 8] = [0; 8];
pub static mut CAN_JAB: [i32; 8] = [0; 8];

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn is_enable_transition_term_hook(module_accessor: &mut smash::app::BattleObjectModuleAccessor, flag: i32) -> bool {
    let ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if CAN_UPB[ENTRY_ID] != 0 && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
        if CAN_UPB[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else if CAN_SIDEB[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S {
        if CAN_SIDEB[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else if CAN_DOWNB[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
        if CAN_DOWNB[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else if CAN_CLIFF[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH{
        if CAN_CLIFF[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else if CAN_AIRDODGE[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR {
        if CAN_AIRDODGE[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else if CAN_NEUTRALB[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N {
        if CAN_NEUTRALB[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else if CAN_JUMP_SQUAT[ENTRY_ID] != 0  && (flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT || flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON) {
        if CAN_JUMP_SQUAT[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else if CAN_DOUBLE_JUMP[ENTRY_ID] != 0  && (flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL || flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON) {
        if CAN_DOUBLE_JUMP[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else if CAN_ATTACK_AIR[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR  {
        if CAN_ATTACK_AIR[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else {
        original!()(module_accessor, flag)
    }
}

pub fn compare_mask(mask1: i32, mask2: i32) -> bool {
    return (mask1 & mask2) != 0;
}

#[skyline::from_offset(crate::offsets::get_battle_object_from_id())]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

pub fn get_battle_object_from_accessor(boma: *mut BattleObjectModuleAccessor) -> *mut BattleObject {
    unsafe {
        get_battle_object_from_id((*boma).battle_object_id)
    }
}

pub fn get_fighter_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CFighterCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
    }
}

pub fn get_fighter_common_from_entry_id(entry_id: u32) -> Option<&'static mut L2CFighterCommon> {
    if let Some(object) = get_battle_object_from_entry_id(entry_id) {
        unsafe {
            Some(get_fighter_common_from_accessor(std::mem::transmute((*object).module_accessor)))
        }
    } else {
        None
    }
}

pub fn get_lua_state_from_entry_id(entry_id: u32) -> Option<u64> {
    get_fighter_common_from_entry_id(entry_id).map(|x| x.lua_state_agent)
}

pub fn get_active_battle_object_id_from_entry_id(entry_id: u32) -> Option<u32> {
    use smash::lib::lua_const::*;
    use smash::app::lua_bind::*;
    let object = get_battle_object_from_entry_id(entry_id)?;
    if object.is_null() { return None; }
    let object = unsafe { &mut *object };
    let kind = object.kind as i32;
    let status = unsafe {
        StatusModule::status_kind(object.module_accessor)
    };
    if status != *FIGHTER_STATUS_KIND_NONE && status != *FIGHTER_STATUS_KIND_STANDBY {
        return Some(object.battle_object_id);
    }
    if kind == *FIGHTER_KIND_ELIGHT || kind == *FIGHTER_KIND_EFLAME {
        Some(object.battle_object_id + 0x10000)
    } 
    else if kind == *FIGHTER_KIND_PZENIGAME || kind == *FIGHTER_KIND_PFUSHIGISOU || kind == *FIGHTER_KIND_PLIZARDON {
        let next_id = object.battle_object_id + 0x10000;
        let next_object = unsafe { &mut *get_battle_object_from_id(next_id) };
        let next_status = unsafe {
            StatusModule::status_kind(next_object.module_accessor)
        };
        if next_status != *FIGHTER_STATUS_KIND_NONE && next_status != *FIGHTER_STATUS_KIND_STANDBY {
            Some(next_id)
        } 
        else {
            Some(next_id + 0x10000)
        }
    } 
    else {
        Some(object.battle_object_id)
    }
}

///This gets ALL active battle object IDs, including both Ice Climbers, and only the ACTIVE character of Pokemon Trainer and Aegis.
pub unsafe fn get_all_active_battle_object_ids() -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    for entry_id in 0..8 {
        //Get the active battle object id and add it to the list
        let id = get_active_battle_object_id_from_entry_id(entry_id).unwrap_or(*BATTLE_OBJECT_ID_INVALID as u32);
        vec.push(id);
        //From here on out, we are doing this to account for both ice climbers
        //Get the object back from the id
        let object = get_battle_object_from_id(id);
        if object.is_null() { 
            continue; 
        }
        let object = unsafe { 
            &mut *object 
        };
        //Get the fighter kind - check if it is popo
        let kind = object.kind as i32;
        if kind != *FIGHTER_KIND_POPO { 
            continue; 
        }
        //If it is popo, get nana and add her to the list too
        let boma = &mut *(*object).module_accessor;
        let nana_id = WorkModule::get_int(boma, *FIGHTER_POPO_INSTANCE_WORK_ID_INT_PARTNER_OBJECT_ID) as u32;
        let nana_object = get_battle_object_from_id(nana_id);
        if nana_object.is_null() { 
            continue; 
        }
        let nana_object = unsafe { 
            &mut *nana_object 
        };
        vec.push(nana_object.battle_object_id);
    }
    return vec;
}

pub fn byte_search<T: Eq>(needle: &[T]) -> Option<usize> {   
    let text = unsafe {
        let start = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const T;
        let end = skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as *const T;
        let length = end.offset_from(start) as usize;
        std::slice::from_raw_parts(start, length)
    };

    text.windows(needle.len()).position(|window| window == needle)
}

pub fn byte_search_rodata<T: Eq>(needle: &[T]) -> Option<usize> {
    const RODATA_LEN: usize = 0xCC8C9B;
    let (rodata, text_len) = unsafe {
        let start = skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as *const T;
        let end = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as usize + RODATA_LEN) as *const T;
        let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const T;
        let length = end.offset_from(start) as usize;
        (std::slice::from_raw_parts(start, length), start.offset_from(text) as usize)
    };
    rodata.windows(needle.len()).position(|window| window == needle).map(|x| x + text_len)
}

pub fn offset_to_addr<T>(offset: usize) -> *const T {
    unsafe {
        (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8).add(offset) as _
    }
}

extern "C" {
    #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__get_fighter_entry_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
    fn get_fighter_entry(manager: *mut smash::app::FighterManager, entry_id: u32) -> *mut u8;
}

pub fn get_battle_object_from_entry_id(entry_id: u32) -> Option<*mut BattleObject> {
    unsafe {
        let entry = get_fighter_entry(singletons::FighterManager(), entry_id);
        if entry.is_null() {
            None
        } else {
            Some(*(entry.add(0x4160) as *mut *mut BattleObject))
        }
    }
}

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E"]
    pub static FIGHTER_MANAGER: *mut smash::app::FighterManager;
}
extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E"]
    pub static FIGHTER_CUTIN_MANAGER: *mut smash::app::FighterCutInManager;
}

pub fn get_module_accessor_by_entry_id(entry_id: i32) -> *mut smash::app::BattleObjectModuleAccessor {
    unsafe {
        &mut *smash::app::sv_battle_object::module_accessor(smash::app::Fighter::get_id_from_entry_id(entry_id))
    }
}

pub fn is_grounded(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    let situation_kind = unsafe { StatusModule::situation_kind(module_accessor) as i32 };
    situation_kind == SITUATION_KIND_GROUND
}

pub unsafe fn set_position_lock(entry_id: i32){
    lua_bind::FighterManager::set_position_lock(FIGHTER_MANAGER, FighterEntryID(entry_id), true);
}

pub unsafe fn unset_position_lock(entry_id: i32){
    lua_bind::FighterManager::set_position_lock(FIGHTER_MANAGER, FighterEntryID(entry_id), false);
}


pub unsafe fn change_motion(module_accessor: *mut BattleObjectModuleAccessor, anim: &str){
    MotionModule::change_motion(module_accessor, Hash40::new(anim), 0.0, 1.0, false, 0.0, false, false);
}
pub unsafe fn get_entry_id(module_accessor: *mut BattleObjectModuleAccessor) -> usize{
    WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize
}

pub unsafe fn disable_gravity(module_accessor: *mut BattleObjectModuleAccessor){
    KineticModule::unable_energy(module_accessor,  *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
}

pub unsafe fn enable_gravity(module_accessor: *mut BattleObjectModuleAccessor){
    KineticModule::enable_energy(module_accessor,  *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
}

//BomaExt, helps with various things
pub trait BomaExt {
    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool;
    unsafe fn is_fighter(&mut self) -> bool;
    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool;
    unsafe fn is_weapon(&mut self) -> bool;
    unsafe fn kind(&mut self) -> i32;
    unsafe fn down_input(&mut self) -> bool;
    unsafe fn change_status_req(&mut self, kind: i32, repeat: bool) -> i32;
    unsafe fn is_in_hitlag(&mut self) -> bool;
}

impl BomaExt for BattleObjectModuleAccessor {
    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        let cat = fighter_pad_cmd_flag.into();
        match cat {
            CommandCat::Cat1(cat) => Cat1::new(self).intersects(cat),
            CommandCat::Cat4(cat) => Cat4::new(self).intersects(cat)
        }
    }
    unsafe fn is_fighter(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_FIGHTER;
    }
    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool {
        let kind = StatusModule::status_kind(self);
        return kinds.contains(&kind);
    }
    unsafe fn is_weapon(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_WEAPON;
    }
    unsafe fn kind(&mut self) -> i32 {
        return smash::app::utility::get_kind(self);
    }
    unsafe fn change_status_req(&mut self, kind: i32, repeat: bool) -> i32 {
        return StatusModule::change_status_request_from_script(self, kind, repeat) as i32;
    }
    unsafe fn down_input(&mut self) -> bool {
        let stick_y = ControlModule::get_stick_y(self);
        //Checks if you're holding down the control stick less than the shield drop threshold
        if stick_y <= -0.6875 {
            return true;
        }
        //Checks if you flick the stick down more than 3 times but less than 20 times, or your stick is less than or equal to -1.0
        if ControlModule::get_flick_y(self) >= 3 && ControlModule::get_flick_y(self) < 20 || stick_y <= -1.0 {
            return true;
        };
        return false;
    }
    unsafe fn is_in_hitlag(&mut self) -> bool {
        let hitlag_frame = WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME);
        if hitlag_frame > 0 {
            return true;
        }
        return false;
    }
}

//Frame Info, helps with a few things like Momentum Transfer
pub struct FrameInfo {
    pub lua_state: u64,
    pub agent: *mut L2CAgent,
    pub boma: *mut smash::app::BattleObjectModuleAccessor,
    pub fighter_kind: i32,
    pub status_kind: i32,
    pub situation_kind: i32,
    pub motion_kind: smash::phx::Hash40,
    pub cur_frame: f32,
    pub frame: f32,
    pub cat: [i32; 4],
    pub facing: f32,
    pub stick_x: f32,
    pub stick_y: f32,
    pub id: usize,
    pub costume_slot_number: i32
}

impl FrameInfo {
    pub unsafe fn update_and_get(fighter: &mut L2CFighterCommon) -> Option<Self> {
        let lua_state = fighter.lua_state_agent;
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if !(0..8).contains(&id) {
            return None;
        }
        let cat1 = ControlModule::get_command_flag_cat(boma, 0);
        let cat2 = ControlModule::get_command_flag_cat(boma, 1);
        let cat3 = ControlModule::get_command_flag_cat(boma, 2);
        let cat4 = ControlModule::get_command_flag_cat(boma, 3);
        let cur_frame = MotionModule::frame(boma);
        Some(Self {
            lua_state: lua_state,
            agent: fighter as *mut L2CFighterCommon as *mut L2CAgent,
            boma: boma as *mut smash::app::BattleObjectModuleAccessor,
            fighter_kind: smash::app::utility::get_kind(boma),
            status_kind: StatusModule::status_kind(boma),
            situation_kind: StatusModule::situation_kind(boma),
            motion_kind: Hash40::new_raw(MotionModule::motion_kind(boma)),
            cur_frame: MotionModule::frame(boma),
            frame: cur_frame + 1.0,
            cat: [cat1, cat2, cat3, cat4],
            facing: PostureModule::lr(boma),
            stick_x: ControlModule::get_stick_x(boma),
            stick_y: ControlModule::get_stick_y(boma),
            id: id,
            costume_slot_number: WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)
        })
    }
}

pub(crate) fn is_jc(module_accessor: &mut smash::app::BattleObjectModuleAccessor, fighter_kind : i32, status_kind : i32, frame : f32) -> bool {
	unsafe {
        //[fighter_kind, status_kind, hit_condition, jc_start, jc_end]
        let jump_cancel = [
            [*FIGHTER_KIND_MEWTWO, *FIGHTER_STATUS_KIND_SPECIAL_LW, 0, -1, -1]
        ];
        for i in &jump_cancel {
            if fighter_kind == i[0] && status_kind == i[1] {
                println!("jc status");
                if i[3] != -1 && i[4] != -1 {
                    if (frame as i32) < i[3] || (frame as i32) >= i[4] {
                        continue;
                    };
                };
                if i[2] != 0 {
                    if AttackModule::is_infliction_status(module_accessor, i[2]) {
                        return true;
                    };
                } else {
                    return true;
                };
            };
        };
        return false;
	}
}

pub(crate) fn check_jump(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	unsafe {
        if ControlModule::check_button_on_trriger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            return true;
        };
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
            if ControlModule::get_flick_y(module_accessor) >= 3 && ControlModule::get_stick_y(module_accessor) >= 0.7 {
                return true;
            };
        };
        if ControlModule::check_button_on_trriger(module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) {
            return true;
        };
        return false;
	}
}

pub(crate) unsafe fn ray_check_pos(module_accessor: &mut smash::app::BattleObjectModuleAccessor, x_distance : f32, y_distance: f32, ignore_plat: bool) -> u64 {
	GroundModule::ray_check(module_accessor, &Vector2f{ x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor)} as *const Vector2f, &Vector2f{ x: x_distance, y: y_distance} as *const Vector2f, ignore_plat)
}

#[repr(C)]
pub struct CreateItemParam {
    pub founder_pos: Vector4f,
    pub item_pos: Vector4f,
    pub item_kind: smash::app::ItemKind,
    pub another_battle_object_id: u32,
    pub variation_kind: i32,
    pub lr_dir: f32,
    pub owner_id: u32,
    pub unk_20: u32,
    pub pokeball_or_assist_kind: i32,
    pub unk_0: u64,
    pub weird_flag: u64,
    pub unk_1_weird: u64,
    pub unk_approx_0: f32,
    pub unk_02: f32
}

pub struct FuseKind(i32);

impl FuseKind {
    pub const FUSE: i32 = 0;
    pub const REFUSE: i32 = 1;
}

pub struct FuseType(i32);

impl FuseType {
    pub const NORMAL: i32 = 0;
    pub const POWER: i32 = 1;
    pub const ELEMENTAL: i32 = 2;
}

bitflags! {
    pub struct Cat1: i32 {
        const AttackN       = 0x1;
        const AttackS3      = 0x2;
        const AttackHi3     = 0x4;
        const AttackLw3     = 0x8;
        const AttackS4      = 0x10;
        const AttackHi4     = 0x20;
        const AttackLw4     = 0x40;
        const AttackAirN    = 0x80;
        const AttackAirF    = 0x100;
        const AttackAirB    = 0x200;
        const AttackAirHi   = 0x400;
        const AttackAirLw   = 0x800;
        const SpecialN      = 0x1000;
        const SpecialS      = 0x2000;
        const SpecialHi     = 0x4000;
        const SpecialLw     = 0x8000;
        const SpecialAny    = 0xF000;
        const Walk          = 0x10000;
        const Dash          = 0x20000;
        const Turn          = 0x40000;
        const TurnDash      = 0x80000;
        const Jump          = 0x100000;
        const JumpButton    = 0x200000;
        const AirEscape     = 0x400000;
        const Squat         = 0x800000;
        const Escape        = 0x1000000;
        const EscapeF       = 0x2000000;
        const EscapeB       = 0x4000000;
        const WallJumpLeft  = 0x8000000;
        const WallJumpRight = 0x10000000;
        const Catch         = 0x20000000;
        const NoCmd         = 0x40000000;
    }
    pub struct Cat2: i32 {
        const AppealSL            = 0x1;
        const AppealSR            = 0x2;
        const AppealHi            = 0x4;
        const AppealLw            = 0x8;
        const AppealSmash         = 0x10;
        const AppealAll           = 0x1F;
        const AttackDashAttackHi4 = 0x20;
        const FallJump            = 0x40;
        const DashAttackS4        = 0x80;
        const DamageFallToFall    = 0x100;
        const DownToDownStandFB   = 0x200;
        const DownToDownStand     = 0x400;
        const GuardToPass         = 0x800;
        const SquatToSquatF       = 0x1000;
        const SquatToSquatB       = 0x2000;
        const TurnToEscapeF       = 0x4000;
        const TurnToEscapeB       = 0x8000;
        const StickEscapeF        = 0x10000;
        const StickEscapeB        = 0x20000;
        const StickEscape         = 0x40000;
        const SpecialNReverseLR   = 0x80000;
        const ThrowF              = 0x100000;
        const ThrowB              = 0x200000;
        const ThrowHi             = 0x400000;
        const ThrowLw             = 0x800000;
        const CommonGuard         = 0x1000000;
        const AirLasso            = 0x2000000;
        const AttackN2            = 0x4000000;
        const FinalReverseLR      = 0x8000000;
    }
    pub struct Cat3: i32 {
        const ItemLightThrowFB4    = 0x1;
        const ItemLightThrowHi4    = 0x2;
        const ItemLightThrowLw4    = 0x4;
        const ItemLightThrowHi     = 0x8;
        const ItemLightThrowLw     = 0x10;
        const ItemLightDrop        = 0x20;
        const ItemLightThrowFB     = 0x40;
        const ItemLightThrowAirFB  = 0x80;
        const ItemLightThrowAirFB4 = 0x100;
        const ItemLightThrowAirHi  = 0x200;
        const ItemLightThrowAirHi4 = 0x400;
        const ItemLightThrowAirLw  = 0x800;
        const ItemLightThrowAirLw4 = 0x1000;
        const ItemLightDropAir     = 0x2000;
        const ItemHeavyThrowFB     = 0x4000;
        const ItemGetAir           = 0x8000;
        const SpecialSSmash        = 0x10000;
        const SpecialSSmashDash    = 0x20000;

        const ItemLightThrow       = 0x58;
        const ItemLightThrowAir    = 0xA80;
        const ItemLightThrow4      = 0x7;
        const ItemLightThrow4Air   = 0x1500;
        const ItemLightThrowAll    = 0x5F;
        const ItemLightThrowAirAll = 0x1F80;
    }
    pub struct Cat4: i32 {
        const SpecialNCommand       = 0x1;
        const SpecialN2Command      = 0x2;
        const SpecialSCommand       = 0x4;
        const SpecialHiCommand      = 0x8;
        const Command6N6            = 0x10;
        const Command4N4            = 0x20;
        const AttackCommand1        = 0x40;
        const SpecialHi2Command     = 0x80;
        const SuperSpecialCommand   = 0x100;
        const SuperSpecialRCommand  = 0x200;
        const SuperSpecial2Command  = 0x400;
        const SuperSpecial2RCommand = 0x800;
        const Command623NB          = 0x1000;
        const Command623Strict      = 0x2000;
        const Command623ALong       = 0x4000;
        const Command623BLong       = 0x8000;
        const Command623A           = 0x10000;
        const Command2              = 0x20000;
        const Command3              = 0x40000;
        const Command1              = 0x80000;
        const Command6              = 0x100000;
        const Command4              = 0x200000;
        const Command8              = 0x400000;
        const Command9              = 0x800000;
        const Command7              = 0x1000000;
        const Command6N6AB          = 0x2000000;
        const Command323Catch       = 0x4000000;
    }
    pub struct Buttons: i32 {
        const Attack      = 0x1;
        const Special     = 0x2;
        const Jump        = 0x4;
        const Guard       = 0x8;
        const Catch       = 0x10;
        const Smash       = 0x20;
        const JumpMini    = 0x40;
        const CStickOn    = 0x80;
        const StockShare  = 0x100;
        const AttackRaw   = 0x200;
        const AppealHi    = 0x400;
        const SpecialRaw  = 0x800;
        const AppealLw    = 0x1000;
        const AppealSL    = 0x2000;
        const AppealSR    = 0x4000;
        const FlickJump   = 0x8000;
        const GuardHold   = 0x10000;
        const SpecialRaw2 = 0x20000;
        // We leave a blank at 0x4000 because the internal control mapping will map 1 << InputKind to the button bitfield, and so our shorthop button
        // would get mapped to FullHop (issue #776)
        const FullHop  = 0x80000;
        const CStickOverride = 0x100000;
        const SpecialAll  = 0x20802;
        const AttackAll   = 0x201;
        const AppealAll   = 0x7400;
    }
}

#[derive(Copy, Clone)]
pub enum CommandCat {
    Cat1(Cat1),
    Cat4(Cat4)
}

impl Into<CommandCat> for Cat1 {
    fn into(self) -> CommandCat {
        CommandCat::Cat1(self)
    }
}

impl Into<CommandCat> for Cat4 {
    fn into(self) -> CommandCat {
        CommandCat::Cat4(self)
    }
}

impl Cat1 {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe {
            Cat1::from_bits_unchecked(ControlModule::get_command_flag_cat(boma, 0))
        }
    }
}

impl Cat4 {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe { 
            Cat4::from_bits_unchecked(ControlModule::get_command_flag_cat(boma, 3)) 
        }
    }
}

pub fn install() {
	skyline::install_hook!(
        is_enable_transition_term_hook
    );
}

/*
todo
pub unsafe fn get_nearest_opponent(module_accessor: *mut BattleObjectModuleAccessor) -> i32{

    let entry_id = get_entry_id(module_accessor);
    let mut lowestavg = 0.0;
    let entry_count = FighterManager::entry_count(FIGHTER_MANAGER);
    for i in 0..entry_count{
        let curr_module_accessor = get_module_accessor_by_entry_id(i);
        let avg = (PostureModule::pos_x(curr_module_accessor) + PostureModule::pos_y(curr_module_accessor)) / 2;
    }
}
 */
