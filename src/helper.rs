use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
    },
    smash_script::*,
    smashline::*
};

pub trait ToF32 {
    fn to_f32(self) -> f32;
}

impl ToF32 for i32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for i64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for f32 {
    fn to_f32(self) -> f32 { self }
}

impl ToF32 for f64 {
    fn to_f32(self) -> f32 { self as f32 }
}

pub unsafe fn REG_LANDING_SE(fighter: &mut L2CAgentBase, se: Hash40){
    fighter.clear_lua_stack();
    lua_args!(fighter, se);
    sv_animcmd::REG_LANDING_SE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

pub unsafe fn ADD_SPEED_NO_LIMIT(fighter: &mut L2CAgentBase, x_speed: f32, y_speed: f32){
    fighter.clear_lua_stack();
    lua_args!(fighter, x_speed, y_speed);
    sv_animcmd::ADD_SPEED_NO_LIMIT(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

pub unsafe fn LAST_EFFECT_SET_WORK_INT(fighter: &mut L2CAgentBase, unk: i32){
    fighter.clear_lua_stack();
    lua_args!(fighter, unk);
    sv_animcmd::LAST_EFFECT_SET_WORK_INT(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

pub unsafe fn EFFECT_OFF_HANDLE(fighter: &mut L2CAgentBase, unk: i32){
    fighter.clear_lua_stack();
    lua_args!(fighter, unk);
    sv_animcmd::EFFECT_OFF_HANDLE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

pub unsafe fn FT_CATCH_STOP(fighter: &mut L2CAgentBase, unk1: f32, unk2: f32){
    fighter.clear_lua_stack();
    lua_args!(fighter, unk1, unk2);
    sv_animcmd::FT_CATCH_STOP(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

pub unsafe fn FT_REMOVE_FINAL_AURA(fighter: &mut L2CAgentBase, unk: bool){
    fighter.clear_lua_stack();
    lua_args!(fighter, unk);
    sv_animcmd::FT_REMOVE_FINAL_AURA(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

pub unsafe fn FT_IS_SAME_FIGHTER_CATEGORY(fighter: &mut L2CAgentBase, unk: i32) -> bool
{
    fighter.clear_lua_stack();
    lua_args!(fighter, unk);
    sv_animcmd::FT_IS_SAME_FIGHTER_CATEGORY(fighter.lua_state_agent);
    let ret = fighter.pop_lua_stack(1).get_bool();
    ret
}

pub unsafe fn EFFECT_FOLLOW_RND(
    fighter: &mut L2CAgentBase, 
    effect_type: Hash40, 
    bone: Hash40,
    x_pos: f32,
    y_pos: f32,
    z_pos: f32,
    x_rot: f32,
    y_rot: f32,
    z_rot: f32,
    unk1: f32,
    unk2: f32,
    unk3: f32,
    unk4: f32,
    unk5: f32,
    unk6: f32,
    unk7: f32,
    unk8: bool
) {
    fighter.clear_lua_stack();
    lua_args!(fighter, effect_type, bone, x_pos, y_pos, z_pos, x_rot, y_rot, z_rot, unk1, unk2, unk3, unk4, unk5, unk6, unk7, unk8);
    sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
    fighter.pop_lua_stack(1).get_bool();
}

pub unsafe fn EFFECT_BRANCH_SITUATION(
    fighter: &mut L2CAgentBase, 
    effect_type: Hash40, 
    effect_type2: Hash40, 
    bone: Hash40,
    x_pos: f32,
    y_pos: f32,
    z_pos: f32,
    x_rot: f32,
    y_rot: f32,
    z_rot: f32,
    size: f32,
    unk1: f32,
    unk2: f32,
    unk3: f32,
    unk4: f32,
    unk5: f32,
    unk6: f32,
    unk7: bool
) {
    fighter.clear_lua_stack();
    lua_args!(fighter, effect_type, effect_type2, bone, x_pos, y_pos, z_pos, x_rot, y_rot, z_rot, size, unk1, unk2, unk3, unk4, unk5, unk6, unk7);
    sv_animcmd::EFFECT_BRANCH_SITUATION(fighter.lua_state_agent);
    fighter.pop_lua_stack(1).get_bool();
}

pub unsafe fn EFFECT_FOLLOW_COLOR(
    fighter: &mut L2CAgentBase, 
    effect_type: Hash40, 
    bone: Hash40,
    x_pos: f32,
    y_pos: f32,
    z_pos: f32,
    x_rot: f32,
    y_rot: f32,
    z_rot: f32,
    unk1: f32,
    unk2: bool,
    unk3: f32,
    unk4: f32,
    unk5: f32
) {
    fighter.clear_lua_stack();
    lua_args!(fighter, effect_type, bone, x_pos, y_pos, z_pos, x_rot, y_rot, z_rot, unk1, unk2, unk3, unk4, unk5);
    sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
    fighter.pop_lua_stack(1).get_bool();
}


pub unsafe fn EFFECT_FOLLOW_FLIP_arg13(
    fighter: &mut L2CAgentBase,
    unk1: Hash40,
    unk2: Hash40,
    unk3: Hash40,
    unk4: f32,
    unk5: f32,
    unk6: f32,
    unk7: f32,
    unk8: f32,
    unk9: f32,
    unk10: f32,
    unk11: bool,
    unk12: i32,
    unk13: i32,
){
    fighter.clear_lua_stack();
    lua_args!(fighter, unk1, unk2, unk3, unk4, unk5, unk6, unk7, unk8, unk9, unk10, unk11, unk12, unk13);
    sv_animcmd::EFFECT_FOLLOW_FLIP_arg13(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

pub unsafe fn EFFECT_GLOBAL(
    fighter: &mut L2CAgentBase,
    unk1: Hash40, 
    unk2: f32, 
    unk3: f32,
    unk4: f32,
    unk5: f32,
    unk6: f32,
    unk7: f32,
    unk8: f32,
    unk9: bool
)
{
    fighter.clear_lua_stack();
    lua_args!(fighter, unk1, unk2, unk3, unk4, unk5, unk6, unk7, unk8, unk9);
    sv_animcmd::EFFECT_GLOBAL(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

pub unsafe fn DOWN_EFFECT(
    fighter: &mut L2CAgentBase,
    h1: Hash40,
    h2: Hash40,
    i1: i32,
    i2: i32,
    i3: i32,
    i4: i32,
    i5: i32,
    i6: i32,
    i7: i32,
    i8: i32,
    i9: i32,
    i10: i32,
    i11: i32,
    i12: i32,
    i13: i32,
    b1: bool,
)
{
    fighter.clear_lua_stack();
    lua_args!(fighter, h1, h2, i1, i2, i3, i4, i5, i6, i7, i8, i9, i10, i11, i12, i13, b1);
    sv_animcmd::DOWN_EFFECT(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

pub unsafe fn FT_ATTACK_ABS_CAMERA_QUAKE(
    fighter: &mut L2CAgentBase,
    unk1: i32,
    unk2: i32,
){
    fighter.clear_lua_stack();
    lua_args!(fighter, unk1, unk2);
    sv_animcmd::FT_ATTACK_ABS_CAMERA_QUAKE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

pub unsafe fn IS_EXIST_ARTICLE(fighter: &mut L2CAgentBase, article: i32){
    fighter.clear_lua_stack();
    lua_args!(fighter, article);
    sv_animcmd::IS_EXIST_ARTICLE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

pub unsafe fn SET_RATE_ARTICLE(fighter: &mut L2CAgentBase, article: i32, rate: f32){
    fighter.clear_lua_stack();
    lua_args!(fighter, article, rate);
    sv_animcmd::SET_RATE_ARTICLE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

pub unsafe fn CLR_SPEED(fighter: &mut L2CAgentBase, _type: i32){
    fighter.clear_lua_stack();
    lua_args!(fighter, _type);
    sv_animcmd::CLR_SPEED(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

pub unsafe fn REQ_MOTION_CAMERA(fighter: &mut L2CAgentBase, camera: Hash40, unk: bool) {
    fighter.clear_lua_stack();
    lua_args!(fighter, camera, unk);
    sv_animcmd::REQ_MOTION_CAMERA(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

pub unsafe fn CHECK_VALID_START_CAMERA<A: ToF32, B: ToF32, C: ToF32, D: ToF32, E: ToF32, F: ToF32>(fighter: &mut L2CAgentBase, unk1: A, unk2: B, unk3: C, unk4: D, unk5: E, unk6: F, unk7: bool) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk1.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32(), unk5.to_f32(), unk6.to_f32(), unk7);
    sv_animcmd::CHECK_VALID_START_CAMERA(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

pub unsafe fn SET_TAKEOUT_SE_STATUS(agent: &mut L2CAgentBase, se: Hash40) {
    agent.clear_lua_stack();
    lua_args!(agent, se);
    sv_animcmd::SET_TAKEOUT_SE_STATUS(agent.lua_state_agent);
    agent.clear_lua_stack();
}

pub unsafe fn FOOT_EFFECT_FLIP(
    fighter: &mut L2CAgentBase,
    unk1: Hash40,
    unk2: Hash40,
    unk3: Hash40,
    unk4: f32,
    unk5: f32,
    unk6: f32,
    unk7: f32,
    unk8: f32,
    unk9: f32,
    size: f32,
    unk11: f32,
    unk12: f32,
    unk13: f32,
    unk14: f32,
    unk15: f32,
    unk16: f32,
    unk17: bool,
    unk18: i32,
){
    fighter.clear_lua_stack();
    lua_args!(fighter, unk1, unk2, unk3, unk4, unk5, unk6, unk7, unk8, unk9, size, unk11, unk12, unk13, unk14, unk15, unk16, unk17, unk18);
    sv_animcmd::FOOT_EFFECT_FLIP(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

pub unsafe fn IS_RANDOM(
    fighter: &mut L2CAgentBase,
    unk1: i32,
){
    fighter.clear_lua_stack();
    lua_args!(fighter, unk1);
    sv_animcmd::IS_RANDOM(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}