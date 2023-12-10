use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
    },
    smash_script::*,
    smashline::*
};

pub unsafe fn REG_LANDING_SE(fighter: &mut L2CAgentBase, se: Hash40){
    fighter.clear_lua_stack();
    lua_args!(fighter, se);
    sv_animcmd::REG_LANDING_SE(fighter.lua_state_agent);
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

pub unsafe fn FT_IS_SAME_FIGHTER_CATEGORY(fighter: &mut L2CAgentBase, unk: i32) -> bool
{
    fighter.clear_lua_stack();
    lua_args!(fighter, unk);
    sv_animcmd::FT_IS_SAME_FIGHTER_CATEGORY(fighter.lua_state_agent);
    let ret = fighter.pop_lua_stack(1).get_bool();
    ret
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