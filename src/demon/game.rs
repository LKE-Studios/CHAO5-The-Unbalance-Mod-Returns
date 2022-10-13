use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;
use crate::utils::FIGHTER_CUTIN_MANAGER;
use smash::hash40;
use smash2;

#[acmd_script(//Attack11
    agent = "demon", 
    script = "game_attack11", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_jab1(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.444)
    if(is_excute){
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(Frame=6)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=10, FKB=0, BKB=30, Size=4.0, X=0.0, Y=14.5, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=10, FKB=0, BKB=30, Size=5.2, X=0.0, Y=13.5, Z=8.75, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=10, FKB=0, BKB=30, Size=5.2, X=0.0, Y=13.5, Z=3.25, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=10, FKB=0, BKB=30, Size=5.2, X=0.0, Y=11.0, Z=8.75, X2=0.0, Y2=13.5, Z2=8.75, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=10, FKB=0, BKB=30, Size=5.2, X=0.0, Y=11.0, Z=3.25, X2=0.0, Y2=13.5, Z2=3.25, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=7.0, Angle=33, KBG=15, FKB=0, BKB=35, Size=5.6, X=0.0, Y=13.2, Z=3.25, X2=0.0, Y2=13.2, Z2=9.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_add_reaction_frame_revised(0, 15.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 15.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 15.0, false)
        AttackModule::set_add_reaction_frame_revised(3, 15.0, false)
        AttackModule::set_add_reaction_frame_revised(4, 15.0, false)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.3)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.3)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.3)
    }
    wait(Frames=3)
    if(is_excute){
        AttackModule::clear_all()
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
    }
}

#[acmd_script(//Attack12
    agent = "demon", 
    script = "game_attack12", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_jab2(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.5)
    if(is_excute){
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(Frame=1)
    if(is_excute){
        JostleModule::set_push_speed_x_overlap_rate_status(20.0)
    }
    frame(Frame=7)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_ENABLE_COMBO)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.5, Angle=361, KBG=10, FKB=0, BKB=30, Size=5.0, X=0.0, Y=14.25, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.5, Angle=361, KBG=10, FKB=0, BKB=30, Size=6.4, X=0.0, Y=13.0, Z=8.75, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=7.5, Angle=361, KBG=10, FKB=0, BKB=30, Size=6.4, X=0.0, Y=13.0, Z=3.25, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=7.5, Angle=361, KBG=10, FKB=0, BKB=30, Size=6.4, X=0.0, Y=10.25, Z=8.75, X2=0.0, Y2=13.0, Z2=8.75, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=7.5, Angle=361, KBG=10, FKB=0, BKB=30, Size=6.4, X=0.0, Y=10.25, Z=3.25, X2=0.0, Y2=13.0, Z2=3.25, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=7.5, Angle=30, KBG=15, FKB=0, BKB=30, Size=6.6, X=0.0, Y=12.9, Z=3.25, X2=0.0, Y2=12.9, Z2=9.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_add_reaction_frame(ID=0, Frames=16.0, Unk=false)
    }
    wait(Frames=2)
    if(is_excute){
        JostleModule::set_push_speed_x_overlap_rate_status(0.0)
        AttackModule::clear_all()
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS)
    }
    frame(Frame=10)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH)
    }
    frame(Frame=27)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH)
    }
}

#[acmd_script(//Attack13
    agent = "demon", 
    script = "game_attack13", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_jab3(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.66)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(Frame=6)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        ATTACK(ID=0, Part=1, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=10, FKB=0, BKB=35, Size=5.0, X=0.0, Y=14.5, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=5.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=10, FKB=0, BKB=35, Size=6.5, X=0.0, Y=13.5, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=5.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=10, FKB=0, BKB=35, Size=6.5, X=0.0, Y=13.5, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=5.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=10, FKB=0, BKB=35, Size=6.5, X=0.0, Y=8.75, Z=9.0, X2=0.0, Y2=13.5, Z2=9.0, Hitlag=0.5, SDI=5.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=10, FKB=0, BKB=35, Size=6.5, X=0.0, Y=9.0, Z=3.0, X2=0.0, Y2=13.5, Z2=3.0, Hitlag=0.5, SDI=5.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=5, Part=1, Bone=hash40("top"), Damage=8.0, Angle=33, KBG=15, FKB=0, BKB=35, Size=6.6, X=0.0, Y=15.0, Z=3.0, X2=0.0, Y2=15.0, Z2=9.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS)
    }
}

#[acmd_script(//FlashPunch
    agent = "demon", 
    script = "game_flashpunch", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_flashpunch(fighter: &mut L2CAgentBase) {
    if(is_excute){
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(Frame=2)
    if(is_excute){
        ATTACK(ID=0, Part=1, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=100, FKB=35, BKB=0, Size=4.0, X=0.0, Y=5.0, Z=4.0, X2=0.0, Y2=5.0, Z2=5.0, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA_d, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
    }
    frame(Frame=7)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=43, KBG=112, FKB=0, BKB=65, Size=5.0, X=0.0, Y=9.5, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=16.0, Angle=43, KBG=112, FKB=0, BKB=65, Size=7.0, X=0.0, Y=8.0, Z=4.0, X2=0.0, Y2=8.0, Z2=8.5, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=16.0, Angle=43, KBG=112, FKB=0, BKB=65, Size=7.0, X=0.0, Y=10.5, Z=4.0, X2=0.0, Y2=10.5, Z2=8.5, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=4)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//Attack14
    agent = "demon", 
    script = "game_attack14", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_jab4(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.5)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(Frame=10)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.5, Angle=361, KBG=10, FKB=0, BKB=35, Size=5.0, X=0.0, Y=14.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=4.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.5, Angle=361, KBG=10, FKB=0, BKB=35, Size=6.6, X=0.0, Y=13.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=4.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.5, Angle=361, KBG=10, FKB=0, BKB=35, Size=6.6, X=0.0, Y=13.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=4.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=6.5, Angle=361, KBG=10, FKB=0, BKB=35, Size=6.6, X=0.0, Y=9.5, Z=9.0, X2=0.0, Y2=13.0, Z2=9.0, Hitlag=0.5, SDI=4.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=6.5, Angle=361, KBG=10, FKB=0, BKB=35, Size=6.6, X=0.0, Y=9.5, Z=4.0, X2=0.0, Y2=13.0, Z2=4.0, Hitlag=0.5, SDI=4.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=6.5, Angle=36, KBG=15, FKB=0, BKB=30, Size=6.6, X=0.0, Y=14.0, Z=4.0, X2=0.0, Y2=14.0, Z2=9.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS)
    }
}

#[acmd_script(//Attack15
    agent = "demon", 
    script = "game_attack15", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_jab5(fighter: &mut L2CAgentBase) {
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(Frame=11)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.5, Angle=42, KBG=20, FKB=0, BKB=40, Size=7.0, X=0.0, Y=13.0, Z=4.0, X2=0.0, Y2=15.0, Z2=11.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.5, Angle=0, KBG=5, FKB=0, BKB=40, Size=6.5, X=0.0, Y=10.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=4.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.5, Angle=0, KBG=5, FKB=0, BKB=40, Size=6.5, X=0.0, Y=6.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=4.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=5.5, Angle=0, KBG=5, FKB=0, BKB=40, Size=6.6, X=0.0, Y=9.75, Z=7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=4.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=5.5, Angle=0, KBG=5, FKB=0, BKB=40, Size=6.6, X=0.0, Y=13.5, Z=10.75, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=4.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=5.5, Angle=0, KBG=5, FKB=0, BKB=40, Size=5.5, X=0.0, Y=14.5, Z=10.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=4.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame_revised(1, 15.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 15.0, false)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS)
    }
}

#[acmd_script(//Attack16
    agent = "demon", 
    script = "game_attack16", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_jab6(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.6)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(Frame=23)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=0, KBG=5, FKB=0, BKB=30, Size=6.9, X=0.0, Y=12.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.0, Angle=0, KBG=5, FKB=0, BKB=30, Size=6.9, X=0.0, Y=10.0, Z=7.25, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=7.0, Angle=0, KBG=5, FKB=0, BKB=30, Size=6.9, X=0.0, Y=8.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=7.0, Angle=0, KBG=5, FKB=0, BKB=30, Size=5.5, X=0.0, Y=15.0, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=7.0, Angle=0, KBG=5, FKB=0, BKB=30, Size=7.0, X=0.0, Y=12.0, Z=10.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame_revised(0, 9.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 9.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 9.0, false)
    }
    wait(Frames=1)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=0, KBG=5, FKB=0, BKB=30, Size=6.9, X=0.0, Y=10.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.0, Angle=0, KBG=5, FKB=0, BKB=30, Size=6.9, X=0.0, Y=8.0, Z=7.25, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=7.0, Angle=0, KBG=5, FKB=0, BKB=30, Size=6.9, X=0.0, Y=8.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=7.0, Angle=0, KBG=5, FKB=0, BKB=30, Size=5.5, X=0.0, Y=10.0, Z=10.5, X2=0.0, Y2=10.1, Z2=10.5, Hitlag=0.4, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=7.0, Angle=0, KBG=5, FKB=0, BKB=30, Size=7.0, X=0.0, Y=8.0, Z=10.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame_revised(0, 9.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 9.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 9.0, false)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS)
    }
}

#[acmd_script(//Attack17
    agent = "demon", 
    script = "game_attack17", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_jab7(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.66)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(Frame=13)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.5, Angle=0, KBG=10, FKB=0, BKB=30, Size=6.3, X=0.0, Y=6.25, Z=6.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.5, Angle=0, KBG=10, FKB=0, BKB=30, Size=6.3, X=0.0, Y=7.5, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.5, Angle=0, KBG=10, FKB=0, BKB=30, Size=5.0, X=0.0, Y=6.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=5.5, Angle=0, KBG=10, FKB=0, BKB=30, Size=6.5, X=0.0, Y=5.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame_revised(0, 15.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 15.0, false)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS)
    }
}

#[acmd_script(//Attack18
    agent = "demon", 
    script = "game_attack18", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_jab8(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.5)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(Frame=18)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.5, Angle=0, KBG=10, FKB=0, BKB=30, Size=6.3, X=0.0, Y=6.25, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.5, Angle=0, KBG=10, FKB=0, BKB=30, Size=6.3, X=0.0, Y=6.75, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.5, Angle=0, KBG=10, FKB=0, BKB=30, Size=6.3, X=0.0, Y=7.25, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=8.5, Angle=0, KBG=10, FKB=0, BKB=30, Size=5.0, X=0.0, Y=6.25, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=8.5, Angle=0, KBG=10, FKB=0, BKB=30, Size=6.5, X=0.0, Y=5.75, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=3.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x193bdcb0cc, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame_revised(0, 12.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 12.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 12.0, false)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS)
    }
}

#[acmd_script(//Attack19
    agent = "demon", 
    script = "game_attack19", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_jab9(fighter: &mut L2CAgentBase) {
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(Frame=15)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=50, FKB=0, BKB=60, Size=6.0, X=0.0, Y=11.5, Z=7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x1985267897), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=50, FKB=0, BKB=60, Size=7.0, X=0.0, Y=10.0, Z=3.0, X2=0.0, Y2=10.0, Z2=7.0, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x1985267897), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_attack_level(0, FIGHTER_RYU_SAVING_LV_2 as u8)
        AttackModule::set_no_finish_camera(0, true, false)
        AttackModule::set_no_finish_camera(1, true, false)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS)
    }
}

#[acmd_script(//Attack110
    agent = "demon", 
    script = "game_attack110", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_jab10(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.7)
    if(is_excute){
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(Frame=28)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=18.0, Angle=53, KBG=90, FKB=0, BKB=50, Size=5.0, X=0.0, Y=16.0, Z=7.0, X2=0.0, Y2=14.0, Z2=6.5, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=18.0, Angle=53, KBG=90, FKB=0, BKB=50, Size=7.0, X=0.0, Y=10.0, Z=5.5, X2=0.0, Y2=16.0, Z2=7.0, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=18.0, Angle=53, KBG=90, FKB=0, BKB=50, Size=7.0, X=0.0, Y=4.0, Z=4.0, X2=0.0, Y2=10.0, Z2=5.5, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//Attack100 
    agent = "demon", 
    script = "game_attack100", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_jab100(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.5)
    if(is_excute){
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    for _ in 0..i32::MAX {
        frame(Frame=2)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=85, KBG=50, FKB=0, BKB=15, Size=8.2, X=0.0, Y=10.0, Z=13.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=2.5, Angle=75, KBG=50, FKB=0, BKB=15, Size=8.0, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=2.5, Angle=90, KBG=30, FKB=0, BKB=8, Size=8.0, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=85, KBG=50, FKB=0, BKB=15, Size=8.2, X=0.0, Y=10.0, Z=13.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=2.5, Angle=75, KBG=50, FKB=0, BKB=15, Size=8.0, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=2.5, Angle=90, KBG=30, FKB=0, BKB=8, Size=8.0, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
        frame(Frame=15)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=85, KBG=50, FKB=0, BKB=15, Size=8.2, X=0.0, Y=10.0, Z=13.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=2.5, Angle=75, KBG=50, FKB=0, BKB=15, Size=8.0, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=2.5, Angle=90, KBG=30, FKB=0, BKB=8, Size=8.0, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
        frame(Frame=21)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=85, KBG=50, FKB=0, BKB=15, Size=8.2, X=0.0, Y=10.0, Z=13.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=2.5, Angle=75, KBG=50, FKB=0, BKB=15, Size=8.0, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=2.5, Angle=90, KBG=30, FKB=0, BKB=8, Size=8.0, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
        frame(Frame=27)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=85, KBG=50, FKB=0, BKB=15, Size=8.2, X=0.0, Y=10.0, Z=13.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=2.5, Angle=75, KBG=50, FKB=0, BKB=15, Size=8.0, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=2.5, Angle=90, KBG=30, FKB=0, BKB=8, Size=8.0, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, 0);
        wait_loop_clear(fighter.lua_state_agent);
    }
}

#[acmd_script(//Attack100Sub
    agent = "demon", 
    script = "game_attack100sub", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_jab100sub(fighter: &mut L2CAgentBase) {
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=85, KBG=50, FKB=0, BKB=15, Size=8.2, X=0.0, Y=10.0, Z=13.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=2.5, Angle=75, KBG=50, FKB=0, BKB=15, Size=8.0, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=2.5, Angle=90, KBG=30, FKB=0, BKB=8, Size=8.0, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
    }
}

#[acmd_script(//Attack100End
    agent = "demon", 
    script = "game_attack100end", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_jab100end(fighter: &mut L2CAgentBase) {
    if(is_excute){
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    wait(Frames=6)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=45, KBG=116, FKB=0, BKB=50, Size=12.0, X=0.0, Y=10.0, Z=10.0, X2=0.0, Y2=10.0, Z2=15.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new_raw(0x193bdcb0cc), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//AttackDash
    agent = "demon", 
    script = "game_attackdash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_dashattack(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.75)
    frame(Frame=4)
    if(is_excute){
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("footl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("footr"), HitStatus(*HIT_STATUS_XLU), 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 17.0, 8.0, 0, 4.0, 8.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(Frame=13)
    if(is_excute){
        JostleModule::set_push_speed_x_overlap_rate_status(0.05)
    }
    frame(Frame=15)
    if(is_excute){    
        FT_MOTION_RATE(FSM=1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=19.0, Angle=44, KBG=93, FKB=0, BKB=65, Size=6.5, X=0.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=19.0, Angle=44, KBG=93, FKB=0, BKB=65, Size=5.5, X=0.0, Y=12.0, Z=6.0, X2=0.0, Y2=2.5, Z2=1.5, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("footl"), Damage=22.0, Angle=44, KBG=93, FKB=0, BKB=65, Size=5.5, X=0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("footl"), Damage=22.0, Angle=44, KBG=93, FKB=0, BKB=65, Size=7.5, X=0.5, Y=-1.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=19.0, Angle=44, KBG=93, FKB=0, BKB=65, Size=5.6, X=1.0, Y=2.6, Z=1.8, X2=-1.0, Y2=2.6, Z2=1.8, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_down_only(4, true)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear(ID=1, false)
        AttackModule::clear(ID=4, false)
        ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=19.0, Angle=44, KBG=93, FKB=0, BKB=63, Size=6.5, X=0.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("footl"), Damage=22.0, Angle=44, KBG=93, FKB=0, BKB=63, Size=5.5, X=0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("footl"), Damage=22.0, Angle=44, KBG=93, FKB=0, BKB=63, Size=7.5, X=0.5, Y=-1.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=4)
    if(is_excute){
        JostleModule::set_push_speed_x_overlap_rate_status(0.0)
        AttackModule::clear_all()
    }
}

#[acmd_script(//AttackStand1
    agent = "demon", 
    script = "game_attackstand1", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_dashattack2(fighter: &mut L2CAgentBase) {
    frame(Frame=6)
    if(is_excute){
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, FIGHTER_DEMON_REFLECTOR_KIND_ATTACK_STAND1, FIGHTER_DEMON_REFLECTOR_GROUP_ATTACK_STAND1);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legl"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(Frame=12)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=24.0, Angle=84, KBG=70, FKB=0, BKB=80, Size=6.8, X=0.0, Y=10.5, Z=10.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=30, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=24.0, Angle=84, KBG=70, FKB=0, BKB=80, Size=6.8, X=0.0, Y=11.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=30, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=24.0, Angle=84, KBG=70, FKB=0, BKB=80, Size=7.2, X=0.0, Y=10.5, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=30, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame_revised(0, 20.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 20.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 20.0, false)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.1)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.1)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.1)
    }
    frame(Frame=13)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=24.0, Angle=84, KBG=60, FKB=0, BKB=80, Size=6.8, X=0.0, Y=7.25, Z=7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=30, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=24.0, Angle=84, KBG=60, FKB=0, BKB=80, Size=6.8, X=0.0, Y=11.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=30, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=24.0, Angle=84, KBG=60, FKB=0, BKB=80, Size=7.2, X=0.0, Y=10.5, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=30, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame_revised(0, 20.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 20.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 20.0, false)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.1)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.1)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.1)
    }
    frame(Frame=17)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//AttackS3Hi
    agent = "demon", 
    script = "game_attacks3hi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_sidetiltup(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.8)
    frame(Frame=12)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 17.0, 8.0, 0, 4.0, 8.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("footl"), HitStatus(*HIT_STATUS_XLU), 0);
        ATTACK(ID=0, Part=0, Bone=hash40("footl"), Damage=18.5, Angle=20, KBG=94, FKB=0, BKB=37, Size=5.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("footl"), Damage=18.5, Angle=20, KBG=94, FKB=0, BKB=37, Size=7.25, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("kneel"), Damage=18.5, Angle=20, KBG=94, FKB=0, BKB=37, Size=6.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("legl"), Damage=18.5, Angle=20, KBG=94, FKB=0, BKB=37, Size=6.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=3, ShieldstunMul=1.5)
    }
    frame(Frame=17)
    if(is_excute){
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all()
    }
}

#[acmd_script(//AttackS3
    agent = "demon", 
    script = "game_attacks3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_sidetilt(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.8)
    frame(Frame=12)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 17.0, 8.0, 0, 4.0, 8.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("footl"), HitStatus(*HIT_STATUS_XLU), 0);
        ATTACK(ID=0, Part=0, Bone=hash40("footl"), Damage=18.5, Angle=20, KBG=94, FKB=0, BKB=37, Size=5.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("footl"), Damage=18.5, Angle=20, KBG=94, FKB=0, BKB=37, Size=7.25, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("kneel"), Damage=18.5, Angle=20, KBG=94, FKB=0, BKB=37, Size=6.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("legl"), Damage=18.5, Angle=20, KBG=94, FKB=0, BKB=37, Size=6.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=3, ShieldstunMul=1.5)
    }
    frame(Frame=17)
    if(is_excute){
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all()
    }
}

#[acmd_script(//AttackS3Lw
    agent = "demon", 
    script = "game_attacks3lw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_sidetiltdown(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.8)
    frame(Frame=12)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 17.0, 8.0, 0, 4.0, 8.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("footl"), HitStatus(*HIT_STATUS_XLU), 0);
        ATTACK(ID=0, Part=0, Bone=hash40("footl"), Damage=18.5, Angle=20, KBG=94, FKB=0, BKB=37, Size=5.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("footl"), Damage=18.5, Angle=20, KBG=94, FKB=0, BKB=37, Size=7.25, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("kneel"), Damage=18.5, Angle=20, KBG=94, FKB=0, BKB=37, Size=6.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("legl"), Damage=18.5, Angle=20, KBG=94, FKB=0, BKB=37, Size=6.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=3, ShieldstunMul=1.5)
    }
    frame(Frame=17)
    if(is_excute){
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all()
    }
}

#[acmd_script(//AttackStand21
    agent = "demon", 
    script = "game_attackstand21", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_sidetilt2(fighter: &mut L2CAgentBase) {
    frame(Frame=1)
    FT_MOTION_RATE(FSM=0.5)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
    frame(Frame=5)
    FT_MOTION_RATE(FSM=1.0)
    if(is_excute){
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legr"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(Frame=15)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=0, KBG=50, FKB=60, BKB=0, Size=11.0, X=0.0, Y=11.0, Z=10.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=0, KBG=50, FKB=60, BKB=0, Size=11.0, X=0.0, Y=12.0, Z=2.0, X2=0.0, Y2=11.5, Z2=7.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=10, KBG=40, FKB=100, BKB=0, Size=11.0, X=0.0, Y=14.0, Z=3.0, X2=0.0, Y2=13.0, Z2=9.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame_revised(0, 10.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 10.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 10.0, false)
    }
    frame(Frame=18)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=21)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        AttackModule::clear_all()
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_STAND_2_FLAG_CHECK_STEP)
    }
}

#[acmd_script(//AttackStand22
    agent = "demon", 
    script = "game_attackstand22", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_sidetilt3(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.6)
    frame(Frame=1)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
    frame(Frame=10)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.5, Angle=65, KBG=100, FKB=40, BKB=0, Size=6.0, X=0.0, Y=4.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.5, Angle=65, KBG=100, FKB=40, BKB=0, Size=9.0, X=0.0, Y=5.25, Z=3.0, X2=0.0, Y2=4.0, Z2=7.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.5, Angle=15, KBG=100, FKB=30, BKB=0, Size=9.0, X=0.0, Y=8.5, Z=3.0, X2=0.0, Y2=7.5, Z2=6.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame(ID=0, Frames=10.0, Unk=false)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=2.0)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=2.0)
    }
    frame(Frame=11)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.5, Angle=65, KBG=100, FKB=40, BKB=0, Size=6.0, X=0.0, Y=4.5, Z=12.0, X2=0.0, Y2=4.4, Z2=12.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.5, Angle=65, KBG=100, FKB=40, BKB=0, Size=9.0, X=0.0, Y=5.75, Z=3.0, X2=0.0, Y2=4.2, Z2=11.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.5, Angle=15, KBG=100, FKB=30, BKB=0, Size=9.0, X=0.0, Y=8.0, Z=3.0, X2=0.0, Y2=7.0, Z2=9.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame(ID=0, Frames=10.0, Unk=false)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=2.0)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=2.0)
    }
    frame(Frame=12)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=15)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_STAND_2_FLAG_CHECK_STEP)
    }
}

#[acmd_script(//AttackStand23
    agent = "demon", 
    script = "game_attackstand23", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_sidetilt4(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.6)
    frame(Frame=1)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
    frame(Frame=15)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=75, KBG=100, FKB=40, BKB=0, Size=7.0, X=0.0, Y=4.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.0, Angle=75, KBG=100, FKB=40, BKB=0, Size=9.0, X=0.0, Y=5.75, Z=3.0, X2=0.0, Y2=4.2, Z2=11.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=7.0, Angle=70, KBG=100, FKB=40, BKB=0, Size=9.0, X=0.0, Y=7.75, Z=3.0, X2=0.0, Y2=6.2, Z2=11.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame(ID=0, Frames=10.0, Unk=false)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=2.0)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=2.0)
    }
    frame(Frame=17)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=20)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_STAND_2_FLAG_CHECK_STEP)
    }
}

#[acmd_script(//AttackStand24
    agent = "demon", 
    script = "game_attackstand24", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_sidetilt5(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.7)
    if(is_excute){
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 17.0, 8.0, 0, 4.0, 8.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(Frame=13)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.6, Angle=53, KBG=74, FKB=0, BKB=80, Size=6.0, X=0.0, Y=13.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.6, Angle=53, KBG=74, FKB=0, BKB=80, Size=8.0, X=0.0, Y=11.0, Z=4.0, X2=0.0, Y2=12.0, Z2=12.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=15.6, Angle=53, KBG=74, FKB=0, BKB=80, Size=8.0, X=0.0, Y=12.5, Z=4.0, X2=0.0, Y2=13.5, Z2=12.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=15.6, Angle=53, KBG=74, FKB=0, BKB=80, Size=7.0, X=0.0, Y=7.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=16)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//AttackStand31
    agent = "demon", 
    script = "game_attackstand31", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_sidetilt6(fighter: &mut L2CAgentBase) {
    frame(Frame=1)
    if(is_excute){
        JostleModule::set_push_speed_x_overlap_rate_status(0.1)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legr"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(Frame=9)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=90, KBG=120, FKB=61, BKB=0, Size=6.6, X=0.0, Y=17.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.0, Angle=90, KBG=120, FKB=61, BKB=0, Size=7.2, X=0.0, Y=17.0, Z=11.0, X2=0.0, Y2=4.2, Z2=3.5, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=7.0, Angle=90, KBG=120, FKB=61, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=7.0, Angle=70, KBG=120, FKB=21, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=7.0, Angle=90, KBG=120, FKB=21, BKB=0, Size=6.6, X=0.0, Y=17.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=7.0, Angle=90, KBG=120, FKB=21, BKB=0, Size=7.2, X=0.0, Y=17.0, Z=11.0, X2=0.0, Y2=4.2, Z2=3.5, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame_revised(0, 12.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 12.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 12.0, false)
        AttackModule::set_add_reaction_frame_revised(3, 12.0, false)
        AttackModule::set_add_reaction_frame_revised(4, 12.0, false)
        AttackModule::set_add_reaction_frame_revised(5, 12.0, false)
    }
    frame(Frame=11)
    if(is_excute){
        JostleModule::set_push_speed_x_overlap_rate_status(0.0)
        AttackModule::clear_all()
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_STAND_3_FLAG_CHECK_STEP)
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(Frame=23)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
}

#[acmd_script(//AttackStand32
    agent = "demon", 
    script = "game_attackstand32", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_sidetilt7(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.78)
    if(is_excute){
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 17.0, 8.0, 0, 4.0, 8.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(Frame=13)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legr"), HitStatus(*HIT_STATUS_XLU), 0);
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=18.0, Angle=50, KBG=72, FKB=0, BKB=70, Size=7.2, X=0.0, Y=20.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=40, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_ignore_ground_shield(0, true)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=18.0, Angle=50, KBG=72, FKB=0, BKB=70, Size=6.4, X=0.0, Y=17.0, Z=3.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=40, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_ignore_ground_shield(1, true)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=18.0, Angle=50, KBG=72, FKB=0, BKB=70, Size=6.4, X=0.0, Y=12.0, Z=1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=40, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_ignore_ground_shield(2, true)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=18.0, Angle=50, KBG=72, FKB=0, BKB=70, Size=7.2, X=0.0, Y=20.5, Z=5.0, X2=0.0, Y2=21.5, Z2=3.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=40, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_ignore_ground_shield(3, true)
    }
    frame(Frame=14)
    if(is_excute){
        AttackModule::clear(ID=3, false)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=18.0, Angle=50, KBG=72, FKB=0, BKB=70, Size=7.2, X=0.0, Y=17.0, Z=10.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=40, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=18.0, Angle=50, KBG=72, FKB=0, BKB=70, Size=6.4, X=0.0, Y=15.0, Z=6.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=40, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=18.0, Angle=50, KBG=72, FKB=0, BKB=70, Size=6.4, X=0.0, Y=13.0, Z=3.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=40, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.5)
    }
    frame(Frame=15)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=18.0, Angle=50, KBG=72, FKB=0, BKB=70, Size=7.2, X=0.0, Y=10.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=40, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=18.0, Angle=50, KBG=72, FKB=0, BKB=70, Size=6.4, X=0.0, Y=10.5, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=40, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=18.0, Angle=50, KBG=72, FKB=0, BKB=70, Size=6.4, X=0.0, Y=10.5, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=40, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.5)
    }
    frame(Frame=18)
    if(is_excute){
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all()
    }
}

#[acmd_script(//AttackStand4
    agent = "demon", 
    script = "game_attackstand4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_backtiltd(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.76)
    frame(Frame=12)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legr"), HitStatus(*HIT_STATUS_XLU), 0);
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.5, Angle=0, KBG=2, FKB=0, BKB=35, Size=5.0, X=0.0, Y=8.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_bind"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=14.5, Angle=0, KBG=2, FKB=0, BKB=35, Size=6.6, X=0.0, Y=7.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_bind"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=14.5, Angle=0, KBG=2, FKB=0, BKB=35, Size=6.2, X=0.0, Y=9.5, Z=1.5, X2=0.0, Y2=7.0, Z2=9.0, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_bind"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.7)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.7)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.7)
    }
    frame(Frame=14)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.5, Angle=0, KBG=2, FKB=0, BKB=35, Size=5.0, X=0.0, Y=6.5, Z=5.5, X2=0.0, Y2=6.4, Z2=5.5, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_bind"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=14.5, Angle=0, KBG=2, FKB=0, BKB=35, Size=6.6, X=0.0, Y=4.5, Z=6.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_bind"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=14.5, Angle=0, KBG=2, FKB=0, BKB=35, Size=6.2, X=0.0, Y=9.5, Z=1.5, X2=0.0, Y2=4.5, Z2=6.5, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_bind"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.7)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.7)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.7)
    }
    frame(Frame=16)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//AttackStand5
    agent = "demon", 
    script = "game_attackstand5", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_backtilt(fighter: &mut L2CAgentBase) {
    frame(Frame=2)
    if(is_excute){
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, -15.0, 0, 4.0, 8.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("bust"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(Frame=10)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=23.2, Angle=46, KBG=76, FKB=0, BKB=45, Size=6.0, X=0.0, Y=15.0, Z=10.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=23.2, Angle=46, KBG=76, FKB=0, BKB=45, Size=10.2, X=0.0, Y=10.5, Z=5.5, X2=0.0, Y2=13.72, Z2=10.5, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=13)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//AttackStand6
    agent = "demon", 
    script = "game_attackstand6", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_backtiltu(fighter: &mut L2CAgentBase) {
    if(is_excute){
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, -15.0, 0, 4.0, 8.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(Frame=6)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_STAND_6_FLAG_KEEP_SITUATION_AIR)
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_STAND_6_FLAG_IGNORE_CHANGE_FALL)
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legl"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(Frame=13)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=270, KBG=80, FKB=0, BKB=60, Size=5.0, X=0.0, Y=15.0, Z=11.25, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=20.0, Angle=270, KBG=80, FKB=0, BKB=60, Size=7.0, X=0.0, Y=14.0, Z=10.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=20.0, Angle=270, KBG=80, FKB=0, BKB=60, Size=6.5, X=0.0, Y=13.25, Z=6.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=20.0, Angle=270, KBG=80, FKB=0, BKB=60, Size=6.5, X=0.0, Y=12.5, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=20.0, Angle=270, KBG=80, FKB=0, BKB=60, Size=6.0, X=0.0, Y=11.5, Z=8.0, X2=0.0, Y2=3.0, Z2=3.0, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=17)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=20)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_STAND_6_FLAG_KEEP_SITUATION_AIR)
    }
    frame(Frame=21)
    if(is_excute){
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::off_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_STAND_6_FLAG_IGNORE_CHANGE_FALL)
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

#[acmd_script(//AttackHi3
    agent = "demon", 
    script = "game_attackhi3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_uptilt(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.5)
    frame(Frame=1)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
    frame(Frame=4)
    if(is_excute){
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("bust"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(Frame=9)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=88, KBG=5, FKB=0, BKB=71, Size=9.0, X=0.0, Y=13.0, Z=7.5, X2=0.0, Y2=0.0, Z2=24.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=88, KBG=5, FKB=0, BKB=71, Size=7.0, X=0.0, Y=15.5, Z=10.0, X2=0.0, Y2=0.0, Z2=24.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=10.0, Angle=88, KBG=5, FKB=0, BKB=71, Size=8.0, X=0.0, Y=13.0, Z=4.0, X2=0.0, Y2=0.0, Z2=24.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=10.0, Angle=70, KBG=5, FKB=0, BKB=46, Size=9.0, X=0.0, Y=13.0, Z=7.5, X2=0.0, Y2=0.0, Z2=24.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=10.0, Angle=70, KBG=5, FKB=0, BKB=46, Size=7.0, X=0.0, Y=15.5, Z=10.0, X2=0.0, Y2=0.0, Z2=24.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=10.0, Angle=70, KBG=5, FKB=0, BKB=46, Size=8.0, X=0.0, Y=13.0, Z=4.0, X2=0.0, Y2=0.0, Z2=24.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_add_reaction_frame_revised(0, 6.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 6.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 6.0, false)
        AttackModule::set_add_reaction_frame_revised(3, 6.0, false)
        AttackModule::set_add_reaction_frame_revised(4, 6.0, false)
        AttackModule::set_add_reaction_frame_revised(5, 6.0, false)
    }
    frame(Frame=10)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=88, KBG=5, FKB=0, BKB=68, Size=9.0, X=0.0, Y=15.0, Z=5.5, X2=0.0, Y2=0.0, Z2=24.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=88, KBG=5, FKB=0, BKB=68, Size=7.0, X=0.0, Y=17.5, Z=5.0, X2=0.0, Y2=0.0, Z2=24.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=10.0, Angle=88, KBG=5, FKB=0, BKB=68, Size=8.0, X=0.0, Y=13.0, Z=4.0, X2=0.0, Y2=0.0, Z2=24.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=10.0, Angle=70, KBG=5, FKB=0, BKB=46, Size=9.0, X=0.0, Y=15.0, Z=5.5, X2=0.0, Y2=0.0, Z2=24.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=10.0, Angle=70, KBG=5, FKB=0, BKB=46, Size=7.0, X=0.0, Y=17.5, Z=5.0, X2=0.0, Y2=0.0, Z2=24.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=10.0, Angle=70, KBG=5, FKB=0, BKB=46, Size=8.0, X=0.0, Y=13.0, Z=4.0, X2=0.0, Y2=0.0, Z2=24.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_add_reaction_frame_revised(0, 8.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 8.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 8.0, false)
        AttackModule::set_add_reaction_frame_revised(3, 8.0, false)
        AttackModule::set_add_reaction_frame_revised(4, 8.0, false)
        AttackModule::set_add_reaction_frame_revised(5, 8.0, false)
    }
    frame(Frame=12)
    if(is_excute){
        AttackModule::clear_all()
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_HI_3_FLAG_CHECK_STEP)
    }
    frame(Frame=21)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script(//AttackHi32
    agent = "demon", 
    script = "game_attackhi32", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_uptilt2(fighter: &mut L2CAgentBase) {
    frame(Frame=1)
    if(is_excute){
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("bust"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(Frame=14)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=81, KBG=75, FKB=0, BKB=70, Size=5.0, X=0.0, Y=14.5, Z=9.75, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.45, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=25, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=17.0, Angle=81, KBG=75, FKB=0, BKB=70, Size=7.3, X=0.0, Y=16.25, Z=10.75, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.45, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=25, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=17.0, Angle=81, KBG=75, FKB=0, BKB=70, Size=8.0, X=0.0, Y=13.5, Z=7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.45, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=25, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=17.0, Angle=81, KBG=75, FKB=0, BKB=70, Size=5.0, X=0.0, Y=14.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.45, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=25, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
    }
    frame(Frame=15)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=81, KBG=75, FKB=0, BKB=70, Size=5.0, X=0.0, Y=19.6, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.45, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=25, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=17.0, Angle=81, KBG=75, FKB=0, BKB=70, Size=7.3, X=0.0, Y=21.3, Z=7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.45, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=25, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=17.0, Angle=81, KBG=75, FKB=0, BKB=70, Size=8.0, X=0.0, Y=17.2, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.45, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=25, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=17.0, Angle=81, KBG=75, FKB=0, BKB=70, Size=6.0, X=0.0, Y=15.5, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.45, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=25, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
    }
    frame(Frame=16)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=81, KBG=75, FKB=0, BKB=70, Size=5.0, X=0.0, Y=20.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.45, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=25, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=17.0, Angle=81, KBG=75, FKB=0, BKB=70, Size=7.3, X=0.0, Y=22.1, Z=5.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.45, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=25, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=17.0, Angle=81, KBG=75, FKB=0, BKB=70, Size=8.0, X=0.0, Y=17.6, Z=5.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.45, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=25, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=17.0, Angle=81, KBG=75, FKB=0, BKB=70, Size=6.0, X=0.0, Y=16.0, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.45, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=25, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
    }
    frame(Frame=18)
    if(is_excute){
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all()
    }
}

#[acmd_script(//AttackLw3
    agent = "demon", 
    script = "game_attacklw3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_downtilt(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.55)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
    frame(Frame=12)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
    frame(Frame=13)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_ATTACK_LW_3_FLAG_CHECK_STEP)
    }
    frame(Frame=16)
    if(is_excute){
        FT_MOTION_RATE(FSM=0.9)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=22.0, Angle=85, KBG=38, FKB=0, BKB=65, Size=6.0, X=0.0, Y=6.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_ATTACKLW3, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=22.0, Angle=85, KBG=38, FKB=0, BKB=65, Size=9.0, X=0.0, Y=5.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_ATTACKLW3, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=22.0, Angle=85, KBG=38, FKB=0, BKB=65, Size=6.0, X=0.0, Y=7.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_ATTACKLW3, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=22.0, Angle=85, KBG=38, FKB=0, BKB=65, Size=5.8, X=0.0, Y=9.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_ATTACKLW3, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_add_reaction_frame(ID=0, Frames=25.0, Unk=false)
    }
    wait(Frames=3)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//AttackSquat1
    agent = "demon", 
    script = "game_attacksquat1", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_crouchattack1(fighter: &mut L2CAgentBase) {
    if(is_excute){
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 15.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(Frame=13)
    if(is_excute){
        HIT_NODE(hash40("shoulderr"), HIT_STATUS_XLU)
        HIT_NODE(hash40("armr"), HIT_STATUS_XLU)
        HIT_NODE(hash40("legr"), HIT_STATUS_XLU)
        HIT_NODE(hash40("kneer"), HIT_STATUS_XLU)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=24.0, Angle=0, KBG=84, FKB=0, BKB=50, Size=6.5, X=0.0, Y=10.5, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=24.0, Angle=0, KBG=84, FKB=0, BKB=50, Size=8.0, X=0.0, Y=10.5, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=24.0, Angle=0, KBG=84, FKB=0, BKB=50, Size=6.5, X=0.0, Y=3.5, Z=0.0, X2=0.0, Y2=10.5, Z2=5.5, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame(ID=0, Frames=30.0, Unk=false)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.1)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.1)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.1)
    }
    frame(Frame=14)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=24.0, Angle=0, KBG=84, FKB=0, BKB=50, Size=6.5, X=0.0, Y=7.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=24.0, Angle=0, KBG=84, FKB=0, BKB=50, Size=8.0, X=0.0, Y=7.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=24.0, Angle=0, KBG=84, FKB=0, BKB=50, Size=6.5, X=0.0, Y=3.5, Z=0.0, X2=0.0, Y2=7.0, Z2=10.0, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame(ID=0, Frames=30.0, Unk=false)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.1)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.1)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.1)
    }
    frame(Frame=16)
    if(is_excute){
        AttackModule::clear_all()
        HIT_NODE(hash40("legr"), HIT_STATUS_NORMAL)
        HIT_NODE(hash40("kneer"), HIT_STATUS_NORMAL)
    }
    frame(Frame=19)
    if(is_excute){
        HitModule::set_status_all(HIT_STATUS_NORMAL)
    }
}

#[acmd_script(//AttackSquat2
    agent = "demon", 
    script = "game_attacksquat2", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_crouchattack2(fighter: &mut L2CAgentBase) {
    if(is_excute){
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 15.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(Frame=5)
    if(is_excute){
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(Frame=6)
    if(is_excute){
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.0, Angle=77, KBG=50, FKB=0, BKB=25, Size=7.0, X=0.0, Y=9.0, Z=13.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=13.0, Angle=77, KBG=50, FKB=0, BKB=25, Size=6.4, X=0.0, Y=9.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=13.0, Angle=77, KBG=50, FKB=0, BKB=25, Size=6.0, X=0.0, Y=9.0, Z=5.0, X2=0.0, Y2=9.0, Z2=12.0, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=270, KBG=10, FKB=60, BKB=0, Size=6.0, X=0.0, Y=3.0, Z=5.0, X2=0.0, Y2=3.0, Z2=12.0, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_down_only(0, true)
        AttackModule::set_add_reaction_frame_revised(1, 50.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 50.0, false)
        AttackModule::set_add_reaction_frame_revised(3, 50.0, false)
        AttackModule::set_add_reaction_frame_revised(0, 50.0, false)
    }
    frame(Frame=8)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=15)
    if(is_excute){
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_INPUT)
    }
}

#[acmd_script(//AttackSquat3
    agent = "demon", 
    script = "game_attacksquat3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_crouchattack3(fighter: &mut L2CAgentBase) {
    if(is_excute){
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 15.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(Frame=10)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=19.0, Angle=55, KBG=100, FKB=0, BKB=78, Size=5.5, X=0.0, Y=6.0, Z=12.25, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_saving"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=19.0, Angle=55, KBG=100, FKB=0, BKB=78, Size=7.0, X=0.0, Y=5.0, Z=13.25, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_saving"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=19.0, Angle=55, KBG=100, FKB=0, BKB=78, Size=6.6, X=0.0, Y=5.0, Z=4.0, X2=0.0, Y2=5.0, Z2=11.5, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_saving"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_2 as u8);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_2 as u8);
        AttackModule::set_attack_level(fighter.module_accessor, 2, *FIGHTER_RYU_SAVING_LV_2 as u8);
        AttackModule::set_add_reaction_frame(ID=0, Frames=25.0, Unk=false)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.5)
    }
    frame(Frame=16)
    if(is_excute){
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all()
    }
}

#[acmd_script(//AttackSquat4
    agent = "demon", 
    script = "game_attacksquat4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_crouchattack4(fighter: &mut L2CAgentBase) {
    frame(Frame=1)
    if(is_excute){
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 15.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("bust"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(Frame=12)
    if(is_excute){
        HIT_NODE(hash40("shoulderr"), HIT_STATUS_XLU)
        HIT_NODE(hash40("armr"), HIT_STATUS_XLU)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=21.0, Angle=361, KBG=93, FKB=0, BKB=40, Size=5.0, X=0.0, Y=11.25, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x19f2214801, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_ATTACKSQUAT4, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=21.0, Angle=361, KBG=93, FKB=0, BKB=40, Size=7.0, X=0.0, Y=10.0, Z=4.0, X2=0.0, Y2=10.0, Z2=9.0, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x19f2214801, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_ATTACKSQUAT4, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=21.0, Angle=361, KBG=93, FKB=0, BKB=40, Size=7.0, X=0.0, Y=10.0, Z=4.0, X2=0.0, Y2=10.0, Z2=9.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=0x19f2214801, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_ATTACKSQUAT4, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_no_finish_camera(0, true, false)
        AttackModule::set_no_finish_camera(1, true, false)
        AttackModule::set_no_finish_camera(2, true, false)
    }
    frame(Frame=14)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//AttackS4Transform
    agent = "demon", 
    script = "game_attacks4transform", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_sidesmashtrans(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(Frame=1)
    if(is_excute){
        WorkModule::set_int(-1, FIGHTER_DEMON_STATUS_ATTACK_S4_WORK_INT_CRITICAL_HIT_NO)
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    FT_MOTION_RATE(FSM=1)
    frame(Frame=6)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 15.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(fighter.lua_state_agent, 10.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 11.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 13.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 14.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 21.0);
    FT_MOTION_RATE(FSM=1.0)
    frame(Frame=25)
    if(is_excute){
        WorkModule::set_int(1, FIGHTER_DEMON_STATUS_ATTACK_S4_WORK_INT_CRITICAL_HIT_NO)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=38.0, Angle=361, KBG=66, FKB=0, BKB=20, Size=6.0, X=0.0, Y=8.5, Z=4.0, X2=0.0, Y2=13.5, Z2=4.0, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=30, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("handl"), Damage=40.0, Angle=361, KBG=66, FKB=0, BKB=20, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=30, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=0.1)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.75)
    }
    wait(Frames=2)
    FT_MOTION_RATE(FSM=1.0)
    if(is_excute){
        AttackModule::clear_all()
    }
    if WorkModule::is_flag(FIGHTER_DEMON_STATUS_ATTACK_S4_FLAG_HIT) == false {
        if(is_excute){
            QUAKE(CAMERA_QUAKE_KIND_S)
        }
    }
    frame(fighter.lua_state_agent, 51.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 57.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 58.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 59.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 61.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

#[acmd_script(//AttackS4
    agent = "demon", 
    script = "game_attacks4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_sidesmash(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(Frame=1)
    if(is_excute){
        WorkModule::set_int(-1, FIGHTER_DEMON_STATUS_ATTACK_S4_WORK_INT_CRITICAL_HIT_NO)
    }
    FT_MOTION_RATE(FSM=1.0)
    frame(Frame=6)
    if(is_excute){
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 15.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    FT_MOTION_RATE(FSM=1.0)
    frame(Frame=25)
    if(is_excute){
        WorkModule::set_int(1, FIGHTER_DEMON_STATUS_ATTACK_S4_WORK_INT_CRITICAL_HIT_NO)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=38.0, Angle=361, KBG=66, FKB=0, BKB=20, Size=6.0, X=0.0, Y=8.5, Z=4.0, X2=0.0, Y2=13.5, Z2=4.0, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("handl"), Damage=40.0, Angle=361, KBG=66, FKB=0, BKB=20, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=0.1)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.75)
    }
    wait(Frames=2)
    FT_MOTION_RATE(FSM=1.0)
    if(is_excute){
        AttackModule::clear_all()
    }
    if WorkModule::is_flag(FIGHTER_DEMON_STATUS_ATTACK_S4_FLAG_HIT) == false { 
        if(is_excute){
            QUAKE(CAMERA_QUAKE_KIND_S)
        }
    }
    frame(fighter.lua_state_agent, 51.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 57.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 58.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 59.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 61.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

#[acmd_script(//AttackHi4Transform
    agent = "demon", 
    script = "game_attackhi4transform", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_upsmashtrans(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(Frame=1)
    FT_MOTION_RATE(FSM=0.7)
    if(is_excute){
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(Frame=7)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=9)
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(Frame=11)
    FT_MOTION_RATE(FSM=0.6)
    frame(Frame=12)
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(Frame=16)
    FT_MOTION_RATE(FSM=1.0)
    frame(Frame=17)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=28.0, Angle=90, KBG=77, FKB=0, BKB=65, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=28.0, Angle=90, KBG=77, FKB=0, BKB=65, Size=12.5, X=0.0, Y=9.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=28.0, Angle=90, KBG=77, FKB=0, BKB=65, Size=8.5, X=0.0, Y=11.0, Z=3.5, X2=0.0, Y2=8.0, Z2=2.0, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_M, false)
    }
    frame(Frame=18)
    if(is_excute){
        AttackModule::clear(ID=2, false)
        ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=28.0, Angle=90, KBG=77, FKB=0, BKB=65, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("handr"), Damage=28.0, Angle=90, KBG=77, FKB=0, BKB=65, Size=12.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_M, false)
    }
    wait(Frames=2)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=28.0, Angle=90, KBG=77, FKB=0, BKB=65, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("handr"), Damage=28.0, Angle=90, KBG=77, FKB=0, BKB=65, Size=12.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_S, false)
    }
    wait(Frames=3)
    if(is_excute){
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    FT_MOTION_RATE(FSM=0.8)
    if(is_excute){
        AttackModule::clear_all()
    }
    sv_animcmd::frame(fighter.lua_state_agent, 51.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    sv_animcmd::frame(fighter.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    sv_animcmd::frame(fighter.lua_state_agent, 54.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 9.0);
    sv_animcmd::frame(fighter.lua_state_agent, 56.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

#[acmd_script(//AttackHi4
    agent = "demon", 
    script = "game_attackhi4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_upsmash(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(Frame=1)
    FT_MOTION_RATE(FSM=0.7)
    if(is_excute){
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(Frame=7)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=9)
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(Frame=11)
    FT_MOTION_RATE(FSM=0.6)
    frame(Frame=12)
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(Frame=16)
    FT_MOTION_RATE(FSM=1.0)
    frame(Frame=17)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=28.0, Angle=90, KBG=77, FKB=0, BKB=65, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=28.0, Angle=90, KBG=77, FKB=0, BKB=65, Size=12.5, X=0.0, Y=9.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=28.0, Angle=90, KBG=77, FKB=0, BKB=65, Size=8.5, X=0.0, Y=11.0, Z=3.5, X2=0.0, Y2=8.0, Z2=2.0, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_M, false)
    }
    frame(Frame=18)
    if(is_excute){
        AttackModule::clear(ID=2, false)
        ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=28.0, Angle=90, KBG=77, FKB=0, BKB=65, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("handr"), Damage=28.0, Angle=90, KBG=77, FKB=0, BKB=65, Size=12.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_M, false)
    }
    wait(Frames=2)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=28.0, Angle=90, KBG=77, FKB=0, BKB=65, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("handr"), Damage=28.0, Angle=90, KBG=77, FKB=0, BKB=65, Size=12.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_S, false)
    }
    wait(Frames=3)
    if(is_excute){
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    FT_MOTION_RATE(FSM=0.8)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(fighter.lua_state_agent, 51.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 54.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 9.0);
    frame(fighter.lua_state_agent, 56.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

#[acmd_script(//AttackLw4Transform
    agent = "demon", 
    script = "game_attacklw4transform", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_downsmashtrans(fighter: &mut L2CAgentBase) {
    let rand_effect = [Hash40::new("collision_attr_normal"), Hash40::new("collision_attr_fire"), Hash40::new("collision_attr_aura"), Hash40::new("collision_attr_purple"), Hash40::new("collision_attr_bind"), Hash40::new("collision_attr_bury"), Hash40::new("collision_attr_elec"), Hash40::new("collision_attr_coin"), Hash40::new("collision_attr_ice"), Hash40::new("collision_attr_cutup"), Hash40::new("collision_attr_pierce"), Hash40::new("collision_attr_flower"), Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_magic"), Hash40::new("collision_attr_sleep"), Hash40::new("collision_attr_curse_poison"), Hash40::new("collision_attr_saving"), Hash40::new("collision_attr_death")];
    let rng = smash::app::sv_math::rand(hash40("demon"), rand_effect.len() as i32);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(Frame=1)
    if(is_excute){
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    FT_MOTION_RATE(FSM=0.8)
    frame(Frame=3)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(fighter.lua_state_agent, 12.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 13.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 14.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 15.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 16.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    FT_MOTION_RATE(FSM=1.0)
    frame(Frame=20)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=20, FKB=0, BKB=100, Size=8.0, X=0.0, Y=8.2, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=20, FKB=0, BKB=100, Size=7.0, X=0.0, Y=8.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=100, FKB=0, BKB=20, Size=8.0, X=0.0, Y=8.2, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=100, FKB=0, BKB=20, Size=7.0, X=0.0, Y=8.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_add_reaction_frame_revised(0, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(3, 25.0, false)
        ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1, ShieldstunMul=1.2)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 2, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 3, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
    }
    frame(Frame=21)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=10, FKB=0, BKB=100, Size=8.0, X=0.0, Y=6.3, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=10, FKB=0, BKB=100, Size=7.0, X=0.0, Y=6.3, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=100, FKB=0, BKB=20, Size=8.0, X=0.0, Y=6.3, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=100, FKB=0, BKB=20, Size=7.0, X=0.0, Y=6.3, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_add_reaction_frame_revised(0, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(3, 25.0, false)
        ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1, ShieldstunMul=1.2)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 2, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 3, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
    }
    frame(Frame=22)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=20, FKB=0, BKB=100, Size=8.0, X=0.0, Y=2.5, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=20, FKB=0, BKB=100, Size=7.0, X=0.0, Y=3.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=100, FKB=0, BKB=20, Size=8.0, X=-5.0, Y=2.5, Z=11.5, X2=5.0, Y2=2.5, Z2=11.5, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=100, FKB=0, BKB=20, Size=7.0, X=0.0, Y=3.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_add_reaction_frame_revised(0, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(3, 25.0, false)
        ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1, ShieldstunMul=1.2)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 2, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 3, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(fighter.lua_state_agent, 46.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 47.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 48.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 50.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 52.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 9.0);
    frame(fighter.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

#[acmd_script(//AttackLw4
    agent = "demon", 
    script = "game_attacklw4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_downsmash(fighter: &mut L2CAgentBase) {
    let rand_effect = [Hash40::new("collision_attr_normal"), Hash40::new("collision_attr_fire"), Hash40::new("collision_attr_aura"), Hash40::new("collision_attr_purple"), Hash40::new("collision_attr_bind"), Hash40::new("collision_attr_bury"), Hash40::new("collision_attr_elec"), Hash40::new("collision_attr_coin"), Hash40::new("collision_attr_ice"), Hash40::new("collision_attr_cutup"), Hash40::new("collision_attr_pierce"), Hash40::new("collision_attr_flower"), Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_magic"), Hash40::new("collision_attr_sleep"), Hash40::new("collision_attr_curse_poison"), Hash40::new("collision_attr_saving"), Hash40::new("collision_attr_death")];
    let rng = smash::app::sv_math::rand(hash40("demon"), rand_effect.len() as i32);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(Frame=1)
    FT_MOTION_RATE(FSM=0.8)
    if(is_excute){
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(Frame=3)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(fighter.lua_state_agent, 12.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 13.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 14.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 15.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 16.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 18.0);
    FT_MOTION_RATE(FSM=1.0)
    frame(Frame=20)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=20, FKB=0, BKB=100, Size=8.0, X=0.0, Y=8.2, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=20, FKB=0, BKB=100, Size=7.0, X=0.0, Y=8.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=100, FKB=0, BKB=20, Size=8.0, X=0.0, Y=8.2, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=100, FKB=0, BKB=20, Size=7.0, X=0.0, Y=8.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_add_reaction_frame_revised(0, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(3, 25.0, false)
        ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1, ShieldstunMul=1.2)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 2, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 3, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
    }
    frame(Frame=21)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=10, FKB=0, BKB=100, Size=8.0, X=0.0, Y=6.3, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=10, FKB=0, BKB=100, Size=7.0, X=0.0, Y=6.3, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=100, FKB=0, BKB=20, Size=8.0, X=0.0, Y=6.3, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=100, FKB=0, BKB=20, Size=7.0, X=0.0, Y=6.3, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_add_reaction_frame_revised(0, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(3, 25.0, false)
        ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1, ShieldstunMul=1.2)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 2, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 3, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
    }
    frame(Frame=22)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=20, FKB=0, BKB=100, Size=8.0, X=0.0, Y=2.5, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=20, FKB=0, BKB=100, Size=7.0, X=0.0, Y=3.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=100, FKB=0, BKB=20, Size=8.0, X=-5.0, Y=2.5, Z=11.5, X2=5.0, Y2=2.5, Z2=11.5, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=26.0, Angle=277, KBG=100, FKB=0, BKB=20, Size=7.0, X=0.0, Y=3.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=15, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=rand_effect[rng as usize], SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_add_reaction_frame_revised(0, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 25.0, false)
        AttackModule::set_add_reaction_frame_revised(3, 25.0, false)
        ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1, ShieldstunMul=1.2)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 2, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 3, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(fighter.lua_state_agent, 46.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 47.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 48.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 50.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 52.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 9.0);
    frame(fighter.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

#[acmd_script(//AttackStep2s
    agent = "demon", 
    script = "game_attackstep2s", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_spinningdemon(fighter: &mut L2CAgentBase) {
    if(is_excute){
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 16.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(Frame=9)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=65, KBG=100, FKB=55, BKB=0, Size=5.0, X=0.0, Y=8.25, Z=0.5, X2=0.0, Y2=2.25, Z2=0.5, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G_d, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=80, KBG=100, FKB=55, BKB=0, Size=9.0, X=0.0, Y=5.25, Z=3.0, X2=0.0, Y2=5.25, Z2=8.25, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=10.0, Angle=65, KBG=100, FKB=55, BKB=0, Size=9.0, X=0.0, Y=7.5, Z=3.0, X2=0.0, Y2=7.5, Z2=7.25, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame(ID=0, Frames=16.0, Unk=false)
    }
    frame(Frame=10)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=65, KBG=100, FKB=55, BKB=0, Size=5.0, X=0.0, Y=8.25, Z=0.5, X2=0.0, Y2=2.25, Z2=0.5, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G_d, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=80, KBG=100, FKB=55, BKB=0, Size=9.0, X=0.0, Y=5.25, Z=3.0, X2=0.0, Y2=5.25, Z2=12.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame(ID=0, Frames=16.0, Unk=false)
    }
    frame(Frame=11)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=65, KBG=100, FKB=55, BKB=0, Size=5.0, X=0.0, Y=8.25, Z=0.5, X2=0.0, Y2=2.25, Z2=0.5, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G_d, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=80, KBG=100, FKB=55, BKB=0, Size=9.0, X=0.0, Y=5.25, Z=3.0, X2=0.0, Y2=5.25, Z2=11.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_add_reaction_frame(ID=0, Frames=16.0, Unk=false)
    }
    frame(Frame=12)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=33)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=25.0, Angle=44, KBG=71, FKB=0, BKB=65, Size=2.5, X=0.0, Y=15.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=25.0, Angle=44, KBG=71, FKB=0, BKB=65, Size=4.0, X=0.0, Y=14.0, Z=8.0, X2=0.0, Y2=11.5, Z2=8.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=25.0, Angle=44, KBG=71, FKB=0, BKB=65, Size=3.5, X=0.0, Y=13.0, Z=3.5, X2=0.0, Y2=8.5, Z2=3.5, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_L, false)
    }
    frame(Frame=36)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//AttackStep2s
    agent = "demon", 
    script = "game_attackstep2s", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_windgodfist(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=1.7)
    if(is_excute){
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 16.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("bust"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(Frame=10)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=150.0, Angle=88, KBG=9, FKB=0, BKB=20, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=150.0, Angle=88, KBG=9, FKB=0, BKB=20, Size=9.0, X=0.0, Y=13.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=150.0, Angle=88, KBG=9, FKB=0, BKB=20, Size=8.0, X=-1.0, Y=9.0, Z=3.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("handr"), Damage=150.0, Angle=73, KBG=9, FKB=0, BKB=20, Size=7.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=150.0, Angle=73, KBG=9, FKB=0, BKB=20, Size=10.0, X=0.0, Y=13.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=150.0, Angle=73, KBG=9, FKB=0, BKB=20, Size=8.0, X=-1.0, Y=9.0, Z=3.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.2)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.2)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.2)
        AttackModule::set_add_reaction_frame_revised(0, 100.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 100.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 100.0, false)
        AttackModule::set_add_reaction_frame_revised(3, 100.0, false)
        AttackModule::set_add_reaction_frame_revised(4, 100.0, false)
        AttackModule::set_add_reaction_frame_revised(5, 100.0, false)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(4, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(5, CAMERA_QUAKE_KIND_M, false)
    }
    frame(Frame=11)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=150.0, Angle=88, KBG=9, FKB=0, BKB=20, Size=6.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=150.0, Angle=88, KBG=9, FKB=0, BKB=20, Size=9.0, X=0.0, Y=18.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=150.0, Angle=88, KBG=9, FKB=0, BKB=20, Size=8.0, X=-1.0, Y=13.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("handr"), Damage=150.0, Angle=73, KBG=9, FKB=0, BKB=20, Size=7.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=150.0, Angle=73, KBG=9, FKB=0, BKB=20, Size=10.0, X=0.0, Y=18.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=150.0, Angle=73, KBG=9, FKB=0, BKB=20, Size=8.0, X=-1.0, Y=13.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, =hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.2)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.2)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.2)
        AttackModule::set_add_reaction_frame_revised(0, 100.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 100.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 100.0, false)
        AttackModule::set_add_reaction_frame_revised(3, 100.0, false)
        AttackModule::set_add_reaction_frame_revised(4, 100.0, false)
        AttackModule::set_add_reaction_frame_revised(5, 100.0, false)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(4, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(5, CAMERA_QUAKE_KIND_M, false)
    }
    frame(Frame=12)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=150.0, Angle=88, KBG=9, FKB=0, BKB=20, Size=6.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=150.0, Angle=88, KBG=9, FKB=0, BKB=20,  Size=9.0, X=0.0, Y=19.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=150.0, Angle=88, KBG=9, FKB=0, BKB=20,  Size=8.0, X=0.0, Y=15.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("handr"), Damage=150.0, Angle=73, KBG=9, FKB=0, BKB=20, Size=7.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=150.0, Angle=73, KBG=9, FKB=0, BKB=20, Size=10.0, X=0.0, Y=19.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=150.0, Angle=73, KBG=9, FKB=0, BKB=20, Size=8.0, X=0.0, Y=15.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.2)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.2)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.2)
        AttackModule::set_add_reaction_frame_revised(0, 100.0, false)
        AttackModule::set_add_reaction_frame_revised(1, 100.0, false)
        AttackModule::set_add_reaction_frame_revised(2, 100.0, false)
        AttackModule::set_add_reaction_frame_revised(3, 100.0, false)
        AttackModule::set_add_reaction_frame_revised(4, 100.0, false)
        AttackModule::set_add_reaction_frame_revised(5, 100.0, false)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(4, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(5, CAMERA_QUAKE_KIND_M, false)
    }
    frame(Frame=14)
    FT_MOTION_RATE(FSM=0.9)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//AttackStep2f
    agent = "demon", 
    script = "game_attackstep2f", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_elecwindgodfist(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=2.0)
    if(is_excute){
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 16.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("bust"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(Frame=10)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=5000000.0, Angle=88, KBG=5, FKB=0, BKB=102, Size=8.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5000000.0, Angle=88, KBG=5, FKB=0, BKB=102, Size=8.0, X=0.0, Y=13.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5000000.0, Angle=88, KBG=5, FKB=0, BKB=102, Size=8.0, X=-1.0, Y=9.0, Z=3.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("handr"), Damage=5000000.0, Angle=81, KBG=5, FKB=0, BKB=92, Size=8.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=5000000.0, Angle=81, KBG=5, FKB=0, BKB=92, Size=8.0, X=0.0, Y=13.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=50.0, Angle=81, KBG=61, FKB=0, BKB=20, Size=7.0, X=-1.0, Y=9.0, Z=3.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_invalid_invincible(fighter.module_accessor, 5, true);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1.2, ShieldstunMul=5.0)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.2)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.2)
        AttackModule::set_add_reaction_frame_revised(0, 19, false)
        AttackModule::set_add_reaction_frame_revised(1, 19, false)
        AttackModule::set_add_reaction_frame_revised(2, 19, false)
        AttackModule::set_add_reaction_frame_revised(3, 14, false)
        AttackModule::set_add_reaction_frame_revised(4, 14, false)
        AttackModule::set_add_reaction_frame_revised(5, 14, false)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(4, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(5, CAMERA_QUAKE_KIND_L, false)
    }
    frame(Frame=11)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=5000000.0, Angle=88, KBG=5, FKB=0, BKB=102, Size=8.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5000000.0, Angle=88, KBG=5, FKB=0, BKB=102, Size=8.0, X=0.0, Y=18.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5000000.0, Angle=88, KBG=5, FKB=0, BKB=102, Size=8.0, X=-1.0, Y=13.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("handr"), Damage=5000000.0, Angle=81, KBG=5, FKB=0, BKB=92, Size=8.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=5000000.0, Angle=81, KBG=5, FKB=0, BKB=92, Size=8.0, X=0.0, Y=18.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=50.0, Angle=81, KBG=61, FKB=0, BKB=20, Size=7.0, X=-1.0, Y=13.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=f32::NAN, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_invalid_invincible(fighter.module_accessor, 5, true);
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.2)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.2)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.2)
        AttackModule::set_add_reaction_frame_revised(0, 19, false)
        AttackModule::set_add_reaction_frame_revised(1, 19, false)
        AttackModule::set_add_reaction_frame_revised(2, 19, false)
        AttackModule::set_add_reaction_frame_revised(3, 14, false)
        AttackModule::set_add_reaction_frame_revised(4, 14, false)
        AttackModule::set_add_reaction_frame_revised(5, 14, false)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(4, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(5, CAMERA_QUAKE_KIND_L, false)
    }
    frame(Frame=12)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=5000000.0, Angle=88, KBG=5, FKB=0, BKB=102, Size=8.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5000000.0, Angle=88, KBG=5, FKB=0, BKB=102, Size=8.0, X=0.0, Y=19.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5000000.0, Angle=88, KBG=5, FKB=0, BKB=102, Size=8.0, X=0.0, Y=15.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("handr"), Damage=5000000.0, Angle=81, KBG=5, FKB=0, BKB=92, Size=8.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=5000000.0, Angle=81, KBG=5, FKB=0, BKB=92, Size=8.0, X=0.0, Y=19.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=50.0, Angle=81, KBG=61, FKB=0, BKB=20, Size=7.0, X=0.0, Y=15.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_invalid_invincible(fighter.module_accessor, 5, true);
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.2)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.2)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.2)
        AttackModule::set_add_reaction_frame_revised(0, 19, false)
        AttackModule::set_add_reaction_frame_revised(1, 19, false)
        AttackModule::set_add_reaction_frame_revised(2, 19, false)
        AttackModule::set_add_reaction_frame_revised(3, 14, false)
        AttackModule::set_add_reaction_frame_revised(4, 14, false)
        AttackModule::set_add_reaction_frame_revised(5, 14, false)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(4, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(5, CAMERA_QUAKE_KIND_L, false)
    }
    frame(Frame=15)
    if(is_excute){
        AttackModule::clear_all(fighter.module_accessor);
    }
    FT_MOTION_RATE(FSM=0.9)
}

#[acmd_script(//AttackStep2l
    agent = "demon", 
    script = "game_attackstep2l", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_dragonupper(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.9)
    frame(Frame=2)
    if(is_excute){
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 13.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("bust"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(Frame=19)
    FT_MOTION_RATE(FSM=1.0)
    frame(Frame=22)
    if(is_excute){
        AttackModule::set_damage_shake_scale(1.5)
        ATTACK(ID=0, Part=0, Bone=hash40("handl"), Damage=999.0, Angle=60, KBG=65, FKB=0, BKB=80, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=999.0, Angle=60, KBG=65, FKB=0, BKB=80, Size=8.25, X=0.0, Y=10.5, Z=4.75, X2=0.0, Y2=16.0, Z2=7.0, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=999.0, Angle=60, KBG=65, FKB=0, BKB=80, Size=8.25, X=0.0, Y=5.0, Z=2.5, X2=0.0, Y2=10.5, Z2=4.75, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=0.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=0.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=0.5)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_L, false)
    }
    frame(Frame=23)
    if(is_excute){
        AttackModule::clear(ID=2, false)
        AttackModule::set_damage_shake_scale(1.6)
        ATTACK(ID=0, Part=0, Bone=hash40("handl"), Damage=999.0, Angle=70, KBG=60, FKB=0, BKB=80, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.1, Hitlag=0.17, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=999.0, Angle=70, KBG=60, FKB=0, BKB=80, Size=8.0, X=0.0, Y=13.0, Z=3.0, X2=0.0, Y2=23.0, Z2=3.0, Hitlag=0.17, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=0.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=0.5)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
    }
    frame(Frame=24)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("handl"), Damage=60.0, Angle=70, KBG=60, FKB=0, BKB=18, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.17, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=60.0, Angle=70, KBG=60, FKB=0, BKB=18, Size=8.0, X=0.0, Y=14.0, Z=3.0, X2=0.0, Y2=24.0, Z2=3.0, Hitlag=0.17, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=0.5)
        ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=0.5)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
    }
    frame(Frame=25)
    if(is_excute){
        HitModule::set_status_all(HIT_STATUS_NORMAL)
        ATTACK(ID=0, Part=0, Bone=hash40("handl"), Damage=55.0, Angle=70, KBG=61, FKB=0, BKB=20, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=55.0, Angle=70, KBG=61, FKB=0, BKB=20, Size=8.0, X=0.0, Y=21.0, Z=3.0, X2=0.0, Y2=24.0, Z2=3.0, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_M, false)
    }
    frame(Frame=26)
    if(is_excute){
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=50.0, Angle=70, KBG=62, FKB=0, BKB=20, Size=8.0, X=0.0, Y=21.5, Z=3.0, X2=0.0, Y2=24.5, Z2=3.0, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_M, false)
    }
    frame(Frame=27)
    if(is_excute){
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=45.0, Angle=70, KBG=66, FKB=0, BKB=20, Size=8.0, X=0.0, Y=22.0, Z=3.0, X2=0.0, Y2=25.0, Z2=3.0, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_M, false)
        AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_M, false)
    }
    frame(Frame=28)
    FT_MOTION_RATE(FSM=0.9)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//AttackAirN
    agent = "demon", 
    script = "game_attackairn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_nair(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.5)
    frame(Frame=3)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=8)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=22.0, Angle=285, KBG=70, FKB=0, BKB=70, Size=5.8, X=0.0, Y=6.0, Z=9.25, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=22.0, Angle=300, KBG=80, FKB=0, BKB=60, Size=5.8, X=0.0, Y=6.0, Z=9.25, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=20.0, Angle=280, KBG=85, FKB=0, BKB=40, Size=9.5, X=0.0, Y=6.0, Z=9.25, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=20.0, Angle=270, KBG=85, FKB=0, BKB=40, Size=6.0, X=0.0, Y=9.0, Z=6.25, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear(ID=0, false)
        AttackModule::clear(ID=1, false)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=19.0, Angle=290, KBG=85, FKB=0, BKB=30, Size=9.5, X=0.0, Y=6.0, Z=9.25, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=19.0, Angle=290, KBG=85, FKB=0, BKB=30, Size=9.0, X=0.0, Y=9.0, Z=6.25, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
    }
    frame(Frame=17)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=25)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
}

#[acmd_script(//AttackAirF 
    agent = "demon", 
    script = "game_attackairf", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_fair(fighter: &mut L2CAgentBase) {
    frame(Frame=3)
    if(is_excute){
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.0, 0, 10.0, 0.0, 0, 4.0, 13.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=8)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("toel"), Damage=21.0, Angle=20, KBG=95, FKB=0, BKB=35, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=21.0, Angle=20, KBG=95, FKB=0, BKB=35, Size=9.5, X=0.0, Y=6.5, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=21.0, Angle=20, KBG=95, FKB=0, BKB=35, Size=8.0, X=0.0, Y=7.5, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=21.0, Angle=20, KBG=95, FKB=0, BKB=35, Size=9.5, X=0.0, Y=8.0, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=2)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("toel"), Damage=17.0, Angle=362, KBG=99, FKB=0, BKB=40, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=17.0, Angle=362, KBG=99, FKB=0, BKB=40, Size=9.5, X=0.0, Y=6.5, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=17.0, Angle=362, KBG=99, FKB=0, BKB=40, Size=9.0, X=0.0, Y=7.5, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=17.0, Angle=362, KBG=99, FKB=0, BKB=40, Size=9.0, X=0.0, Y=8.0, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=25)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
}

#[acmd_script(//AttackAirB
    agent = "demon", 
    script = "game_attackairb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_bair(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.7)
    frame(Frame=5)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=11)
    if(is_excute){
        FT_MOTION_RATE(FSM=1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("toer"), Damage=26.0, Angle=40, KBG=85, FKB=0, BKB=40, Size=7.0, X=-0.3, Y=-1.0, Z=1.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=26.0, Angle=40, KBG=85, FKB=0, BKB=40, Size=9.0, X=0.0, Y=11.0, Z=-9.0, X2=0.0, Y2=11.0, Z2=-3.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=26.0, Angle=40, KBG=85, FKB=0, BKB=40, Size=7.0, X=0.0, Y=11.0, Z=-11.0, X2=0.0, Y2=11.0, Z2=-3.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=3)
    if(is_excute){
        AttackModule::clear(ID=2, false)
        ATTACK(ID=0, Part=0, Bone=hash40("toer"), Damage=20.0, Angle=40, KBG=92, FKB=0, BKB=40, Size=7.0, X=-0.3, Y=-1.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=20.0, Angle=40, KBG=92, FKB=0, BKB=40, Size=9.0, X=0.0, Y=11.0, Z=-11.0, X2=0.0, Y2=11.0, Z2=-3.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=5)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=35)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
}        

#[acmd_script(//AttackAirHi
    agent = "demon", 
    script = "game_attackairhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_uair(fighter: &mut L2CAgentBase) {
    frame(Frame=2)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=4)
    if(is_excute){
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
        ATTACK(ID=0, Part=0, Bone=hash40("toer"), Damage=20.0, Angle=77, KBG=89, FKB=0, BKB=40, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=20.0, Angle=77, KBG=89, FKB=0, BKB=40, Size=8.5, X=0.0, Y=5.5, Z=4.0, X2=0.0, Y2=10.0, Z2=9.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=20.0, Angle=77, KBG=89, FKB=0, BKB=40, Size=8.5, X=0.0, Y=13.5, Z=3.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=20.0, Angle=77, KBG=89, FKB=0, BKB=40, Size=8.5, X=0.0, Y=5.5, Z=4.0, X2=0.0, Y2=10.0, Z2=9.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=20.0, Angle=77, KBG=89, FKB=0, BKB=40, Size=8.5, X=0.0, Y=13.5, Z=3.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=5)
    if(is_excute){
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=20.0, Angle=77, KBG=91, FKB=0, BKB=40, Size=8.5, X=0.0, Y=10.0, Z=9.0, X2=0.0, Y2=18.0, Z2=9.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=20.0, Angle=77, KBG=91, FKB=0, BKB=40, Size=8.5, X=0.0, Y=13.5, Z=3.5, X2=0.0, Y2=15.5, Z2=4.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=20.0, Angle=77, KBG=90, FKB=0, BKB=44, Size=8.5, X=0.0, Y=10.0, Z=9.0, X2=0.0, Y2=18.0, Z2=9.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=20.0, Angle=77, KBG=90, FKB=0, BKB=44, Size=8.5, X=0.0, Y=13.5, Z=3.5, X2=0.0, Y2=15.5, Z2=4.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=6)
    if(is_excute){
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=19.0, Angle=70, KBG=90, FKB=0, BKB=40, Size=10.0, X=0.0, Y=18.0, Z=9.0, X2=0.0, Y2=21.0, Z2=2.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=19.0, Angle=70, KBG=90, FKB=0, BKB=40, Size=9.5, X=0.0, Y=13.5, Z=2.0, X2=0.0, Y2=15.5, Z2=2.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=19.0, Angle=70, KBG=90, FKB=0, BKB=45, Size=10.0, X=0.0, Y=18.0, Z=9.0, X2=0.0, Y2=21.0, Z2=2.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=19.0, Angle=70, KBG=90, FKB=0, BKB=45, Size=9.5, X=0.0, Y=13.5, Z=2.0, X2=0.0, Y2=15.5, Z2=2.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=7)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("toer"), Damage=18.0, Angle=77, KBG=96, FKB=0, BKB=40, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=18.0, Angle=70, KBG=96, FKB=0, BKB=40, Size=10.0, X=0.0, Y=21.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=18.0, Angle=70, KBG=96, FKB=0, BKB=40, Size=8.5, X=0.0, Y=13.5, Z=2.0, X2=0.0, Y2=15.5, Z2=2.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=18.0, Angle=70, KBG=96, FKB=0, BKB=40, Size=10.0, X=0.0, Y=21.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=18.0, Angle=70, KBG=96, FKB=0, BKB=40, Size=8.5, X=0.0, Y=13.5, Z=2.0, X2=0.0, Y2=15.5, Z2=2.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=10)
    if(is_excute){
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all()
    }
    frame(Frame=30)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
}

#[acmd_script(//AttackAirLw
    agent = "demon", 
    script = "game_attackairlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_dair(fighter: &mut L2CAgentBase) {
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_KEEP_AIR)
    }
    frame(Frame=1)
    if(is_excute){
        MotionModule::set_rate(1.5)
        WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
        SET_SPEED_EX(0.0, 2.5, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
        WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
        KineticModule::suspend_energy(FIGHTER_KINETIC_ENERGY_ID_CONTROL)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE)
    }
    frame(Frame=4)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
        SET_SPEED_EX(0.0, 2.0, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
        WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
    }
    frame(Frame=7)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
        SET_SPEED_EX(0.0, 1.5, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
        WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
    }
    frame(Frame=10)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
        SET_SPEED_EX(0.0, 1.0, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
        WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
    }
    frame(Frame=13)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
        SET_SPEED_EX(0.0, 0.5, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
        WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
    }
    frame(Frame=16)
    if(is_excute){
        MotionModule::set_rate(1.0)
    }
    frame(Frame=17)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
        SET_SPEED_EX(0.0, -1.0, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
        WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
    }
    frame(Frame=20)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
        SET_SPEED_EX(0.0, -2.0, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
        WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
    }
    frame(Frame=22)
    if(is_excute){
        let rng_hitbox = smash::app::sv_math::rand(hash40("demon"), 8);
        if rng_hitbox == 0 {
            ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=999.0, Angle=270, KBG=100, FKB=0, BKB=20, Size=10.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=999.0, Angle=270, KBG=100, FKB=0, BKB=20, Size=10.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("footl"), Damage=999.0, Angle=270, KBG=100, FKB=0, BKB=20, Size=10.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=3, Part=0, Bone=hash40("legl"), Damage=999.0, Angle=270, KBG=100, FKB=0, BKB=20, Size=10.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=4, Part=0, Bone=hash40("kneel"), Damage=999.0, Angle=270, KBG=100, FKB=0, BKB=20, Size=10.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=5, Part=0, Bone=hash40("footl"), Damage=999.0, Angle=270, KBG=100, FKB=0, BKB=20, Size=10.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        }
        else {
            ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=10.0, Angle=300, KBG=100, FKB=80, BKB=0, Size=10.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=10.0, Angle=300, KBG=100, FKB=80, BKB=0, Size=10.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("footl"), Damage=10.0, Angle=300, KBG=100, FKB=80, BKB=0, Size=10.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=3, Part=0, Bone=hash40("legl"), Damage=10.0, Angle=70, KBG=100, FKB=55, BKB=0, Size=10.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=4, Part=0, Bone=hash40("kneel"), Damage=10.0, Angle=70, KBG=100, FKB=55, BKB=0, Size=10.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=5, Part=0, Bone=hash40("footl"), Damage=10.0, Angle=70, KBG=100, FKB=55, BKB=0, Size=10.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        }
    }
    frame(Frame=24)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
        SET_SPEED_EX(0.0, -3.5, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
        WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
        AttackModule::clear_all()
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_KEEP_AIR)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        ATTACK(ID=0, Part=1, Bone=hash40("legl"), Damage=35.0, Angle=270, KBG=70, FKB=0, BKB=30, Size=8.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=1, Bone=hash40("kneel"), Damage=35.0, Angle=270, KBG=70, FKB=0, BKB=30, Size=9.0, X=-0.5, Y=-2.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=1, Bone=hash40("footl"), Damage=35.0, Angle=270, KBG=70, FKB=0, BKB=30, Size=10.5, X=-1.0, Y=-0.5, Z=3.5, X2=-6.5, Y2=-8.0, Z2=0.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=25)
    if(is_excute){
        ATTACK(ID=0, Part=1, Bone=hash40("legl"), Damage=35.0, Angle=270, KBG=70, FKB=0, BKB=30, Size=8.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=1, Bone=hash40("kneel"), Damage=35.0, Angle=270, KBG=70, FKB=0, BKB=30, Size=9.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=1, Bone=hash40("footl"), Damage=35.0, Angle=270, KBG=70, FKB=0, BKB=30, Size=10.5, X=-0.2, Y=-0.5, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=45)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=55)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
}

#[acmd_script(//LandingAirLw
    agent = "demon", 
    script = "game_landingairlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_dairland(fighter: &mut L2CAgentBase) {
    if(is_excute){
        SET_SPEED_EX(0.0, 0.0, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
    }
    frame(Frame=1)
    if(is_excute){
        AttackModule::clear(ID=1, false)
        AttackModule::clear(ID=2, false)
        ATTACK(ID=0, Part=1, Bone=hash40("top"), Damage=30.0, Angle=50, KBG=75, FKB=0, BKB=30, Size=22.0, X=0.0, Y=3.0, Z=10.0, X2=0.0, Y2=3.0, Z2=-10.0, Hitlag=0.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_bind"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=3)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//Catch
    agent = "demon", 
    script = "game_catch", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_grab(fighter: &mut L2CAgentBase) {
    frame(Frame=6)
    if(is_excute){
        GrabModule::set_rebound(CanCatchRebound=true)
    }
    frame(Frame=7)
    if(is_excute){
        CATCH(ID=0, Bone=hash40("top"), Size=6.0, X=0.0, Y=9.0, Z=7.0, X2=0.0, Y2=9.0, Z2=14.2, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
        CATCH(ID=1, Bone=hash40("top"), Size=5.0, X=0.0, Y=9.0, Z=5.0, X2=0.0, Y2=9.0, Z2=15.2, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
        macros::game_CaptureCutCommon(fighter);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

#[acmd_script(//CatchDash
    agent = "demon", 
    script = "game_catchdash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_dashgrab(fighter: &mut L2CAgentBase) {
    frame(Frame=8)
    if(is_excute){
        GrabModule::set_rebound(CanCatchRebound=true)
    }
    frame(Frame=9)
    if(is_excute){
        CATCH(ID=0, Bone=hash40("top"), Size=6.6, X=0.0, Y=6.6, Z=5.0, X2=0.0, Y2=6.6, Z2=15.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
        CATCH(ID=1, Bone=hash40("top"), Size=5.8, X=0.0, Y=6.6, Z=3.2, X2=0.0, Y2=6.6, Z2=16.5, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
        macros::game_CaptureCutCommon(fighter);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

#[acmd_script(//CatchTurn
    agent = "demon", 
    script = "game_catchturn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_pivotgrab(fighter: &mut L2CAgentBase) {
    frame(Frame=9)
    if(is_excute){
        GrabModule::set_rebound(CanCatchRebound=true)
    }
    frame(Frame=10)
    if(is_excute){
        CATCH(ID=0, Bone=hash40("top"), Size=6.3, X=0.0, Y=6.6, Z=-5.0, X2=0.0, Y2=6.6, Z2=-19.5, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
        CATCH(ID=1, Bone=hash40("top"), Size=5.65, X=0.0, Y=6.6, Z=-3.35, X2=0.0, Y2=6.6, Z2=-20.1, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
        macros::game_CaptureCutCommon(fighter);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

#[acmd_script(//CatchCommand
    agent = "demon", 
    script = "game_catchcommand", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_grabcommand(fighter: &mut L2CAgentBase) {
    frame(Frame=1)
    FT_MOTION_RATE(FSM=1.6)
    frame(Frame=3)
    if(is_excute){
        GrabModule::set_rebound(CanCatchRebound=true)
    }
    FT_START_ADJUST_MOTION_FRAME_arg1(1.0)
    frame(Frame=4)
    if(is_excute){
        CATCH(ID=0, Bone=hash40("top"), Size=4.0, X=0.0, Y=6.6, Z=5.0, X2=0.0, Y2=6.6, Z2=21.2, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
        CATCH(ID=1, Bone=hash40("top"), Size=2.0, X=0.0, Y=6.6, Z=3.0, X2=0.0, Y2=6.6, Z2=22.2, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
        macros::game_CaptureCutCommon(fighter);
    }
    wait(Frames=1)
    if(is_excute){
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(Frame=4)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_DEMON_STATUS_CATCH_COMMAND_FLAG_CHANGE_THROW)
    }
}

#[acmd_script(//CatchAttack
    agent = "demon", 
    script = "game_catchattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_pummel(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(FSM=0.4)
    frame(Frame=5)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=100, FKB=30, BKB=0, Size=9.0, X=0.0, Y=10.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_CATCHATTACK, Type=ATTACK_REGION_PUNCH)
        DamageModule::heal(fighter.module_accessor, -5.25, 0);
        AttackModule::set_catch_only_all(true, false)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//ThrowF
    agent = "demon", 
    script = "game_throwf", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_throwf(fighter: &mut L2CAgentBase) {
    if !smash2::app::FighterCutInManager::is_vr_mode() {
        if smash2::app::FighterCutInManager::is_one_on_one_including_thrown(&*(fighter.module_accessor as *const smash2::app::BattleObjectModuleAccessor)) {
            if (is_excute) {
                FighterSpecializer_Demon::check_disabled_motion_camera_of_scale(fighter.module_accessor);
                FighterSpecializer_Demon::check_disabled_motion_camera_of_stage(fighter.module_accessor);
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_THROW_MOTION_CAMERA) {
                if(is_excute){
                    CHECK_VALID_START_CAMERA(0, 0, 0, 0, 0, 0, false);
                }
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_THROW_MOTION_CAMERA) {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
                        if(is_excute){
                            REQ_MOTION_CAMERA(Hash40::new("e01throwf.nuanmb"), false);
                        }
                    }
                }
                if(is_excute){
                    let scale = PostureModule::scale(fighter.module_accessor);
                    CAM_ZOOM_IN_arg5(fighter, 7.0, 0.0, scale * 1.5, 0.0, 0.0);
                }
            }
        }
    }
    if(is_excute){
        ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=10.0, Angle=40, KBG=125, FKB=0, BKB=50, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
    }
    frame(Frame=11)
    if PostureModule::scale(fighter.module_accessor) <= 1.4 {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=58, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("footr"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=58, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        }
        else {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=58, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("footr"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=58, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
            }
        }
    }
    if(is_excute){
        AttackModule::set_catch_only_all(true, false)
    }
    frame(Frame=13)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=39)
    if PostureModule::scale(fighter.module_accessor) <= 1.4 {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=58, Size=5.0, X=1.0, Y=14.0, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=58, Size=5.0, X=1.5, Y=15.5, Z=10.5, X2=5.0, Y2=15.5, Z2=10.5, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        }
        else {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=58, Size=4.0, X=1.0, Y=14.0, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=58, Size=4.0, X=1.5, Y=16.0, Z=9.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
            }
        }
    }
    if(is_excute){
        AttackModule::set_catch_only_all(true, false)
    }
    frame(Frame=40)
    if PostureModule::scale(fighter.module_accessor) <= 1.4 {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=58, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("footr"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=58, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
        }
        else {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=58, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("footr"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=58, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
            }
        }
    }
    if(is_excute){
        AttackModule::set_catch_only_all(true, false)
        CHECK_FINISH_CAMERA(9, 1)
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.3);
        lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, Vector3f{x:5.0,y:2.0,z:0.0});
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all()
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    	CAM_ZOOM_OUT()
    }
}  

#[acmd_script(//ThrowB
    agent = "demon", 
    script = "game_throwb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_throwb(fighter: &mut L2CAgentBase) {
    if !smash2::app::FighterCutInManager::is_vr_mode() {
        if smash2::app::FighterCutInManager::is_one_on_one_including_thrown(&*(fighter.module_accessor as *const smash2::app::BattleObjectModuleAccessor)) {
            if (is_excute) {
                FighterSpecializer_Demon::check_disabled_motion_camera_of_scale(fighter.module_accessor);
                FighterSpecializer_Demon::check_disabled_motion_camera_of_stage(fighter.module_accessor);
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_THROW_MOTION_CAMERA) {
                if(is_excute){
                    CHECK_VALID_START_CAMERA(0, 7, 0, 35, 0, 0, false)
                }
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_THROW_MOTION_CAMERA) {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
                        if(is_excute){
                            REQ_MOTION_CAMERA(Hash40::new("e01throwb.nuanmb"), false);
                        }
                    }
                }
                if(is_excute){
                    let scale = PostureModule::scale(fighter.module_accessor);
                    CAM_ZOOM_IN_arg5(fighter, 7.0, 0.0, scale * 1.5, 0.0, 0.0);
                } 
            }
        }
    }
    frame(Frame=27)
    if(is_excute){
        PostureModule::reverse_lr(fighter.module_accessor);
        ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=1.0, Angle=58, KBG=300, FKB=0, BKB=30, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
    }
    frame(Frame=46)
    if(is_excute){
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=30.0, Angle=58, KBG=55, FKB=0, BKB=60, Size=4.0, X=0.0, Y=2.0, Z=-12.0, X2=0.0, Y2=2.0, Z2=-22.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_BODY)
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 600, /*Rehit*/ 30, /* Damage*/ 1.7, /*Unk*/ false);
        AttackModule::set_catch_only_all(true, false)
        CHECK_FINISH_CAMERA(17, 0)
    }
    frame(fighter.lua_state_agent, 47.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    	CAM_ZOOM_OUT()
        AttackModule::clear_all(fighter.module_accessor);
    }
}  

#[acmd_script(//ThrowHi
    agent = "demon", 
    script = "game_throwhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_throwhi(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    if(is_excute){
        ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=2.0, Angle=76, KBG=45, FKB=10, BKB=65, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(fighter.lua_state_agent, 38.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, false, 0);
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 43.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 48.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
    frame(fighter.lua_state_agent, 56.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 63.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 70.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 72.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 73.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 74.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 75.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}   

#[acmd_script(//ThrowLw
    agent = "demon", 
    script = "game_throwlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_throwlw(fighter: &mut L2CAgentBase) {
    if !smash2::app::FighterCutInManager::is_vr_mode() {
        if smash2::app::FighterCutInManager::is_one_on_one_including_thrown(&*(fighter.module_accessor as *const smash2::app::BattleObjectModuleAccessor)) {
            if (is_excute) {
                FighterSpecializer_Demon::check_disabled_motion_camera_of_scale(fighter.module_accessor);
                FighterSpecializer_Demon::check_disabled_motion_camera_of_stage(fighter.module_accessor);
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_THROW_MOTION_CAMERA) {
                if(is_excute){
                    CHECK_VALID_START_CAMERA(0, 0, 0, 0, 0, 0, false)
                }
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_THROW_MOTION_CAMERA) {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
                        if(is_excute){
                            REQ_MOTION_CAMERA(Hash40::new("e01throwlw.nuanmb"), false);
                        }
                    }
                }
                if(is_excute){
                    let scale = PostureModule::scale(fighter.module_accessor);
                    CAM_ZOOM_IN_arg5(fighter, 7.0, 0.0, scale * 1.5, 0.0, 0.0);
                } 
            }
        }
    }
    if(is_excute){
        ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=8.0, Angle=72, KBG=110, FKB=0, BKB=30, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
    }
    frame(Frame=34)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=50, KBG=65, FKB=0, BKB=50, Size=10.0, X=0.0, Y=12.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH02, Type=ATTACK_REGION_HEAD)
        AttackModule::set_catch_only_all(true, false)
        CHECK_FINISH_CAMERA(9, 4)
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(fighter.module_accessor);
        CAM_ZOOM_OUT()
    }
}  

#[acmd_script(//ThrowCommand
    agent = "demon", 
    script = "game_throwcommand", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_throwcommand(fighter: &mut L2CAgentBase) {
    if !smash2::app::FighterCutInManager::is_vr_mode() {
        if smash2::app::FighterCutInManager::is_one_on_one_including_thrown(&*(fighter.module_accessor as *const smash2::app::BattleObjectModuleAccessor)) {
            if (is_excute) {
                FighterSpecializer_Demon::check_disabled_motion_camera_of_scale(fighter.module_accessor);
                FighterSpecializer_Demon::check_disabled_motion_camera_of_stage(fighter.module_accessor);
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_THROW_MOTION_CAMERA) {
                if(is_excute){
                    CHECK_VALID_START_CAMERA(0, 7, 0, 50, 30, 0, false)
                }
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_THROW_MOTION_CAMERA) {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
                        if(is_excute){
                            REQ_MOTION_CAMERA(Hash40::new("e01throwb.nuanmb"), false);
                        }
                    }
                }
                if(is_excute){
                    let scale = PostureModule::scale(fighter.module_accessor);
                    CAM_ZOOM_IN_arg5(fighter, 7.0, 0.0, scale * 1.5, 0.0, 0.0);
                } 
            }
        }
    }
    frame(Frame=10)
    if(is_excute){
        PostureModule::reverse_lr(fighter.module_accessor);
        ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=5.0, Angle=165, KBG=137, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_B, Unk=0.0, Unk=true, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_B, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
    }
    frame(Frame=32)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=60, KBG=80, FKB=0, BKB=50, Size=5.0, X=0.0, Y=8.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_THROWCOMMAND, Type=ATTACK_REGION_KICK)
        AttackModule::set_catch_only_all(true, false)
    }
    frame(Frame=34)
    if(is_excute){
        AttackModule::clear_all()
    }
    if WorkModule::is_flag(FIGHTER_DEMON_STATUS_THROW_COMMAND_FLAG_USE_OTHER_PARAM) == false {
        if(is_excute){
            CHECK_FINISH_CAMERA(18, 2)
        }
        else {
            if(is_excute){
                CHECK_FINISH_CAMERA(18, 15)
            }
        }
    }
    if(is_excute){
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.3);
        lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, Vector3f{x:9.0,y:2.0,z:0.0});
    }
    frame(Frame=80)
    let rng_hitbox = smash::app::sv_math::rand(hash40("demon"), 4);
    if rng_hitbox == 0 {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=999.0, Angle=19, KBG=50, FKB=0, BKB=45, Size=6.0, X=0.0, Y=7.5, Z=-14.0, X2=0.0, Y2=6.0, Z2=-14.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_THROWCOMMAND, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=999.0, Angle=19, KBG=50, FKB=0, BKB=45, Size=4.0, X=0.0, Y=8.0, Z=-6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_THROWCOMMAND, Type=ATTACK_REGION_KICK)
            AttackModule::set_catch_only_all(true, false)
        }
        else {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=30.0, Angle=19, KBG=50, FKB=0, BKB=45, Size=6.0, X=0.0, Y=7.5, Z=-14.0, X2=0.0, Y2=6.0, Z2=-14.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_THROWCOMMAND, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=30.0, Angle=19, KBG=50, FKB=0, BKB=45, Size=4.0, X=0.0, Y=8.0, Z=-6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_THROWCOMMAND, Type=ATTACK_REGION_KICK)
            AttackModule::set_catch_only_all(true, false)
        }
    }
    frame(fighter.lua_state_agent, 81.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    	CAM_ZOOM_OUT()
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//FlyThrow
    agent = "demon_blaster", 
    script = "game_flythrow", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_blaster(fighter: &mut L2CAgentBase) {
    if PostureModule::scale(fighter.module_accessor) <= 1.4 { 
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=25.0, Angle=55, KBG=80, FKB=0, BKB=30, Size=8.0, X=0.0, Y=0.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=25.0, Angle=55, KBG=80, FKB=0, BKB=30, Size=8.0, X=0.0, Y=0.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 660, /*Rehit*/ 40, /* Damage*/ 1.9, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 660, /*Rehit*/ 40, /* Damage*/ 1.9, /*Unk*/ false);
            AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 0, /*Ink*/ 50.0);
            AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 1, /*Ink*/ 50.0);
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.1)
            ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.1)
        }
        else {
            if PostureModule::scale(fighter.module_accessor) <= 0.5 { 
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=25.0, Angle=55, KBG=80, FKB=0, BKB=30, Size=11.0, X=0.0, Y=0.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
                    ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=25.0, Angle=55, KBG=80, FKB=0, BKB=30, Size=11.0, X=0.0, Y=0.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
                    AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 660, /*Rehit*/ 40, /* Damage*/ 1.9, /*Unk*/ false);
                    AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 660, /*Rehit*/ 40, /* Damage*/ 1.9, /*Unk*/ false);
                    AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 0, /*Ink*/ 50.0);
                    AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 1, /*Ink*/ 50.0);
                    ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.1)
                    ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.1)
                }
            }
            else {
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=25.0, Angle=55, KBG=80, FKB=0, BKB=30, Size=9.0, X=0.0, Y=0.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
                    ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=25.0, Angle=55, KBG=80, FKB=0, BKB=30, Size=9.0, X=0.0, Y=0.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
                    AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 660, /*Rehit*/ 40, /* Damage*/ 1.9, /*Unk*/ false);
                    AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 660, /*Rehit*/ 40, /* Damage*/ 1.9, /*Unk*/ false);
                    AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 0, /*Ink*/ 50.0);
                    AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 1, /*Ink*/ 50.0);
                    ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.1)
                    ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.1)
                }
            }
        }
    }
    frame(Frame=1)
    if(is_excute){
        AttackModule::clear(ID=1, false)
    }
}

#[acmd_script(//CliffAttack
    agent = "demon", 
    script = "game_cliffattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_cliffattack(fighter: &mut L2CAgentBase) {
    frame(Frame=24)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=18.0, Angle=45, KBG=60, FKB=0, BKB=90, Size=9.0, X=0.0, Y=5.0, Z=10.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=3)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//SlipAttack
    agent = "demon", 
    script = "game_slipattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_slipattack(fighter: &mut L2CAgentBase) {
    frame(Frame=19)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=361, KBG=90, FKB=0, BKB=60, Size=9.5, X=0.0, Y=4.5, Z=-10.0, X2=0.0, Y2=4.5, Z2=-3.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=24)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=361, KBG=90, FKB=0, BKB=60, Size=9.5, X=0.0, Y=4.5, Z=9.5, X2=0.0, Y2=4.5, Z2=3.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//DownAttackD
    agent = "demon", 
    script = "game_downattackd", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_downattackd(fighter: &mut L2CAgentBase) {
    frame(Frame=19)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=48, KBG=88, FKB=0, BKB=80, Size=9.0, X=0.0, Y=5.0, Z=-12.0, X2=0.0, Y2=5.0, Z2=-5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=24)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=48, KBG=88, FKB=0, BKB=80, Size=9.0, X=0.0, Y=5.0, Z=12.0, X2=0.0, Y2=5.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//DownAttackU
    agent = "demon", 
    script = "game_downattacku", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_downattacku(fighter: &mut L2CAgentBase) {
    frame(Frame=19)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=48, KBG=88, FKB=0, BKB=80, Size=9.0, X=0.0, Y=5.0, Z=-12.0, X2=0.0, Y2=5.0, Z2=-5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=24)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=48, KBG=88, FKB=0, BKB=80, Size=9.0, X=0.0, Y=5.0, Z=12.0, X2=0.0, Y2=5.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
}

#[acmd_script(//SpecialNStart
    agent = "demon",
    script = "game_specialnstart",
    category = ACMD_GAME,
    low_priority )]
unsafe fn demon_neutralbstart(fighter: &mut L2CAgentBase) {
    

#[acmd_script(//SpecialAirN
    agent = "demon",
    script = "game_specialairn",
    category = ACMD_GAME,
    low_priority )]
unsafe fn demon_neutralbair(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
        macros::SET_SPEED_EX(fighter, 0.8, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_demon_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
}

#[acmd_script(//MoveM
    agent = "demon_hadoken", 
    script = "game_movem", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_hadoken1(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.8, /*Angle*/ 0, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 0.5, /*Z*/ -0.5, /*X2*/ Some(0.0), /*Y2*/ Some(-5.2), /*Z2*/ Some(-0.5), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.8, /*Angle*/ 0, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 6.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(-2.5), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.8, /*Angle*/ 60, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 67, /*Size*/ 6.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(-2.5), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ShieldstunMul*/ 1.13);
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.8, /*Angle*/ 0, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(-2.5), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.8, /*Angle*/ 0, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 1.3, /*Z*/ -1.25, /*X2*/ Some(0.0), /*Y2*/ Some(-1.3), /*Z2*/ Some(-1.25), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.8, /*Angle*/ 60, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 1.3, /*Z*/ -1.25, /*X2*/ Some(0.0), /*Y2*/ Some(-1.3), /*Z2*/ Some(-1.25), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ShieldstunMul*/ 1.13);
    }
}

#[acmd_script(//MoveS
    agent = "demon_hadoken", 
    script = "game_moves", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_hadoken2(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 0, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 0.5, /*Z*/ -0.5, /*X2*/ Some(0.0), /*Y2*/ Some(-5.2), /*Z2*/ Some(-0.5), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 0, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(-3.0), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 60, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 67, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(-3.0), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ShieldstunMul*/ 1.06);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 0, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(-2.5), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 0, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 1.3, /*Z*/ -1.25, /*X2*/ Some(0.0), /*Y2*/ Some(-1.3), /*Z2*/ Some(-1.25), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 60, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 1.3, /*Z*/ -1.25, /*X2*/ Some(0.0), /*Y2*/ Some(-1.3), /*Z2*/ Some(-1.25), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ShieldstunMul*/ 1.06);
    }
}

#[acmd_script(//MoveSpM
    agent = "demon_hadoken", 
    script = "game_movespm", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_hadoken3(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.8, /*Angle*/ 80, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ -5.2, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-5.2), /*Z2*/ Some(0.0), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.8, /*Angle*/ 366, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 38, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-5.2), /*Z2*/ Some(0.0), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 10.0, /*Unk*/ false);
    }
    wait(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.3, /*Angle*/ 80, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.15, /*X*/ 0.0, /*Y*/ -0.9, /*Z*/ 0.95, /*X2*/ Some(0.0), /*Y2*/ Some(-0.9), /*Z2*/ Some(-5.15), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.3, /*Angle*/ 366, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 38, /*Size*/ 5.3, /*X*/ 0.0, /*Y*/ 0.25, /*Z*/ 0.3, /*X2*/ Some(0.0), /*Y2*/ Some(0.25), /*Z2*/ Some(-4.5), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.3, /*Angle*/ 80, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.4, /*X*/ 0.0, /*Y*/ -2.4, /*Z*/ -1.1, /*X2*/ Some(0.0), /*Y2*/ Some(-2.4), /*Z2*/ Some(-2.9), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.3, /*Angle*/ 366, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 38, /*Size*/ 6.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -2.0, /*X2*/ Some(0.0), /*Y2*/ Some(-1.0), /*Z2*/ Some(-2.0), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 4.0, /*Unk*/ false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 4.0, /*Unk*/ false);
    }
}

#[acmd_script(//MoveSpM_Last
    agent = "demon_hadoken", 
    script = "game_movespm_last", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_hadoken4(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 55, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
    }
}

#[acmd_script(//MoveSpS
    agent = "demon_hadoken", 
    script = "game_movesps", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_hadoken5(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.8, /*Angle*/ 80, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ -5.2, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-5.2), /*Z2*/ Some(0.0), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.8, /*Angle*/ 366, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 38, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-5.2), /*Z2*/ Some(0.0), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 10.0, /*Unk*/ false);
    }
    wait(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.3, /*Angle*/ 80, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.15, /*X*/ 0.0, /*Y*/ -0.9, /*Z*/ 0.95, /*X2*/ Some(0.0), /*Y2*/ Some(-0.9), /*Z2*/ Some(-5.15), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.3, /*Angle*/ 366, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 38, /*Size*/ 5.3, /*X*/ 0.0, /*Y*/ 0.25, /*Z*/ 0.3, /*X2*/ Some(0.0), /*Y2*/ Some(0.25), /*Z2*/ Some(-4.5), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.3, /*Angle*/ 80, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.4, /*X*/ 0.0, /*Y*/ -2.4, /*Z*/ -1.1, /*X2*/ Some(0.0), /*Y2*/ Some(-2.4), /*Z2*/ Some(-2.9), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.3, /*Angle*/ 366, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 38, /*Size*/ 6.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -2.0, /*X2*/ Some(0.0), /*Y2*/ Some(-1.0), /*Z2*/ Some(-2.0), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 4.0, /*Unk*/ false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 4.0, /*Unk*/ false);
    }
}

#[acmd_script(//MoveSpS_Last
    agent = "demon_hadoken", 
    script = "game_movesps_last", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_hadoken6(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 55, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
    }
}

#[acmd_script(//MoveSpW
    agent = "demon_hadoken", 
    script = "game_movespw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_hadoken7(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.8, /*Angle*/ 80, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ -5.2, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-5.2), /*Z2*/ Some(0.0), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.8, /*Angle*/ 366, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 38, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-5.2), /*Z2*/ Some(0.0), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 10.0, /*Unk*/ false);
    }
    wait(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.3, /*Angle*/ 80, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.15, /*X*/ 0.0, /*Y*/ -0.9, /*Z*/ 0.95, /*X2*/ Some(0.0), /*Y2*/ Some(-0.9), /*Z2*/ Some(-5.15), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.3, /*Angle*/ 366, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 38, /*Size*/ 5.3, /*X*/ 0.0, /*Y*/ 0.25, /*Z*/ 0.3, /*X2*/ Some(0.0), /*Y2*/ Some(0.25), /*Z2*/ Some(-4.5), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.3, /*Angle*/ 80, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.4, /*X*/ 0.0, /*Y*/ -2.4, /*Z*/ -1.1, /*X2*/ Some(0.0), /*Y2*/ Some(-2.4), /*Z2*/ Some(-2.9), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.3, /*Angle*/ 366, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 38, /*Size*/ 6.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -2.0, /*X2*/ Some(0.0), /*Y2*/ Some(-1.0), /*Z2*/ Some(-2.0), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 4.0, /*Unk*/ false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 4.0, /*Unk*/ false);
    }
}

#[acmd_script(//MoveSpW_Last
    agent = "demon_hadoken", 
    script = "game_movespw_last", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_hadoken8(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 55, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
    }
}

#[acmd_script(//MoveW
    agent = "demon_hadoken", 
    script = "game_movew", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_hadoken9(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 0, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 0.5, /*Z*/ -0.5, /*X2*/ Some(0.0), /*Y2*/ Some(-5.2), /*Z2*/ Some(-0.5), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 0, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(-3.0), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 60, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 67, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(-3.0), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ShieldstunMul*/ 1.06);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 0, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(-2.5), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 0, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 1.3, /*Z*/ -1.25, /*X2*/ Some(0.0), /*Y2*/ Some(-1.3), /*Z2*/ Some(-1.25), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 60, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 1.3, /*Z*/ -1.25, /*X2*/ Some(0.0), /*Y2*/ Some(-1.3), /*Z2*/ Some(-1.25), /*Hitlag*/ 1.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ShieldstunMul*/ 1.06);
    }
}

#[acmd_script(//SpecialSStart
    agent = "demon", 
    script = "game_specialsstart", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_sideb1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 1.0, 3.5, 8.5, 8.5);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_demon_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 275, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 4.5, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(4.5), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(fighter.module_accessor, /*ID*/ 0, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(fighter.module_accessor, /*ID*/ 0, /*Size*/ 0.5);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_demon_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

#[acmd_script(//SpecialS
    agent = "demon", 
    script = "game_specials", 
    category = ACMD_GAME,
    low_priority )]
unsafe fn demon_sideb2(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 5.5, 3.0, 9.0, 3.0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legr"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 340, 65, 0, 60, 7.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_KICK);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_M as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 19.0, 270, 75, 0, 60, 7.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_KICK);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 21.0, 280, 80, 0, 60, 7.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 8.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_KICK);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(fighter.module_accessor, 0, 0.1);
    }
    frame(fighter.lua_state_agent, 9.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 270, 90, 0, 60, 7.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_KICK);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_M as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 21.5, 275, 95, 0, 60, 7.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_KICK);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 23.0, 280, 96, 0, 60, 7.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_KICK);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(fighter.module_accessor, 0, 0.1);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_demon_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

#[acmd_script(//SpecialAirSStart
    agent = "demon", 
    script = "game_specialairsstart", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_sidebair1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 1.0, 3.5, 8.5, 8.5);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_demon_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 275, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 4.5, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(4.5), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(fighter.module_accessor, /*ID*/ 0, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(fighter.module_accessor, /*ID*/ 0, /*Size*/ 0.5);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_demon_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

#[acmd_script(//SpecialAirS
    agent = "demon", 
    script = "game_specialairs", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_sidebair2(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legr"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 18.5, 270, 90, 0, 60, 8.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_KICK);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_M as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 19.5, 275, 95, 0, 60, 8.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_KICK);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 21.0, 280, 96, 0, 60, 8.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 8.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_KICK);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(fighter.module_accessor, 0, 0.1);
    }
    frame(fighter.lua_state_agent, 9.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 275, 90, 0, 60, 8.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_KICK);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_M as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 22.0, 280, 95, 0, 60, 8.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_KICK);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 23.6, 285, 96, 0, 60, 8.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_KICK);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(fighter.module_accessor, 0, 0.1);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_demon_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

#[acmd_script(//SpecialHi
    agent = "demon", 
    script = "game_specialhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_upb(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 30.0, 80, 55, 0, 80, 7.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_M as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 32.0, 80, 56, 0, 80, 7.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 33.5, 80, 57, 0, 80, 7.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 30.0, 80, 64, 0, 80, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_M as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 30.0, 80, 64, 0, 80, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 30.0, 80, 64, 0, 80, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 40.0, 85, 76, 0, 21, 6.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_M as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 40.0, 85, 76, 0, 21, 6.5, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 40.0, 85, 76, 0, 21, 6.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script(//SpecialAirHi
    agent = "demon", 
    script = "game_specialairhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_upbair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 30.0, 80, 55, 0, 80, 7.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_M as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 31.0, 80, 56, 0, 80, 7.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 32.5, 80, 58, 0, 80, 7.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 29.0, 80, 60, 0, 75, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_M as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 29.0, 80, 60, 0, 75, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 29.0, 80, 60, 0, 75, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 40.0, 85, 75, 0, 21, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_M as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 40.0, 85, 75, 0, 21, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 40.0, 85, 75, 0, 21, 6.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}


#[acmd_script(//SpecialHiCommand
    agent = "demon", 
    script = "game_specialhicommand", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_upb2(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 32.5, 80, 48, 0, 80, 7.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_M as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 35.0, 80, 50, 0, 80, 7.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 500.0, 80, 69, 0, 80, 7.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 32.0, 80, 52, 0, 80, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_M as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 36.0, 80, 52, 0, 80, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 500.0, 80, 64, 0, 80, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 40.0, 88, 65, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_M as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 40.0, 88, 65, 0, 60, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 42.0, 88, 65, 0, 60, 6.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script(//SpecialAirHiCommand
    agent = "demon", 
    script = "game_specialairhicommand", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_upb2air(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 32.5, 80, 48, 0, 80, 7.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_M as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 35.0, 80, 50, 0, 80, 7.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 500.0, 80, 69, 0, 80, 7.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 32.0, 80, 52, 0, 80, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_M as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 36.0, 80, 52, 0, 80, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 500.0, 80, 64, 0, 80, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 40.0, 88, 65, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_demon_STRENGTH_M as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 40.0, 88, 65, 0, 60, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 42.0, 88, 65, 0, 60, 6.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script(//SpecialLw
    agent = "demon", 
    script = "game_speciallw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_downb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_LW_FLAG_DISABLE_SUPER_ARMOR);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_demon_SAVING_LV_1 as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 50.0, 60, 10, 0, 100, 5.5, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_demon_SAVING_LV_1 as u8);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_demon_SAVING_LV_2 as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 100.0, 60, 100, 0, 0, 6.3, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 100.0, 60, 100, 0, 0, 6.3, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_demon_SAVING_LV_2 as u8);
            AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_demon_SAVING_LV_2 as u8);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 999.0, 60, 100, 0, 0, 6.4, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 999.0, 60, 100, 0, 0, 6.4, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_demon_SAVING_LV_3 as u8);
            AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_demon_SAVING_LV_3 as u8);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
        }
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_LW_FLAG_REVERSE_MATERIAL_ANIM);
    }
}

#[acmd_script(//SpecialLwTurn
    agent = "demon", 
    script = "game_speciallwturn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_downbturn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        PostureModule::reverse_lr(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_LW_FLAG_DISABLE_SUPER_ARMOR);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_demon_SAVING_LV_1 as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 50.0, 60, 10, 0, 100, 6.5, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_demon_SAVING_LV_1 as u8);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_demon_SAVING_LV_2 as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 100.0, 60, 100, 0, 0, 6.3, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 100.0, 60, 100, 0, 0, 6.3, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_demon_SAVING_LV_2 as u8);
            AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_demon_SAVING_LV_2 as u8);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 999.0, 60, 100, 0, 0, 6.4, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 999.0, 60, 100, 0, 0, 6.4, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_demon_SAVING_LV_3 as u8);
            AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_demon_SAVING_LV_3 as u8);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
        }
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_LW_FLAG_REVERSE_MATERIAL_ANIM);
    }
}

#[acmd_script(//SpecialAirLw
    agent = "demon", 
    script = "game_specialairlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_downbair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_LW_FLAG_DISABLE_SUPER_ARMOR);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_demon_SAVING_LV_1 as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 50.0, 60, 10, 0, 100, 5.5, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_demon_SAVING_LV_1 as u8);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_demon_SAVING_LV_2 as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 100.0, 60, 100, 0, 0, 6.3, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 100.0, 60, 100, 0, 0, 6.3, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_demon_SAVING_LV_2 as u8);
            AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_demon_SAVING_LV_2 as u8);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 999.0, 60, 100, 0, 0, 6.4, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 999.0, 60, 100, 0, 0, 6.4, 0.0, 10.5, 11.5, Some(0.0), Some(10.5), Some(2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_demon_SAVING_LV_3 as u8);
            AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_demon_SAVING_LV_3 as u8);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
        }
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_LW_FLAG_REVERSE_MATERIAL_ANIM);
    }
}

#[acmd_script(//SpecialAirLwTurn
    agent = "demon", 
    script = "game_specialairlwturn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_downbairturn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        PostureModule::reverse_lr(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_LW_FLAG_DISABLE_SUPER_ARMOR);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_demon_SAVING_LV_1 as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 50.0, 60, 10, 0, 100, 6.5, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_demon_SAVING_LV_1 as u8);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_LW_INT_SAVING_LV) == *FIGHTER_demon_SAVING_LV_2 as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 100.0, 60, 100, 0, 0, 6.3, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 100.0, 60, 100, 0, 0, 6.3, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 22, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_demon_SAVING_LV_2 as u8);
            AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_demon_SAVING_LV_2 as u8);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 999.0, 60, 100, 0, 0, 6.4, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 999.0, 60, 100, 0, 0, 6.4, 0.0, 10.5, -13.5, Some(0.0), Some(10.5), Some(-2.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_demon_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_demon_SAVING_LV_3 as u8);
            AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_demon_SAVING_LV_3 as u8);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
        }
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_demon_STATUS_WORK_ID_SPECIAL_LW_FLAG_REVERSE_MATERIAL_ANIM);
    }
}

#[acmd_script(//AppealHiR
    agent = "demon", 
    script = "game_appealhir", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_uptauntr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        DamageModule::heal(fighter.module_accessor, -15.0, 0);
    }
}

#[acmd_script(//AppealHiL
    agent = "demon", 
    script = "game_appealhil", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_uptauntl(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        DamageModule::heal(fighter.module_accessor, -15.0, 0);
    }
}

#[acmd_script(//AppealSR
    agent = "demon", 
    script = "game_appealsr", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_sidetauntr(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_HOMERUNBAT), 0, 0, false, false);
    }
}

#[acmd_script(//AppealSL
    agent = "demon", 
    script = "game_appealsl", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_sidetauntl(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_HOMERUNBAT), 0, 0, false, false);
    }
}

#[acmd_script(//AppealLwR
    agent = "demon", 
    script = "game_appeallwr", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_downtauntr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 40.0, /*Angle*/ 270, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 20.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 4.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bury"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
    }
}

#[acmd_script(//AppealLwL
    agent = "demon", 
    script = "game_appeallwl", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_downtauntl(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 40.0, /*Angle*/ 270, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 20.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 4.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bury"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
    }
}


#[acmd_script(//FinalHit
    agent = "demon", 
    script = "game_finalhit", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_finalhit(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        SlowModule::set_whole(fighter.module_accessor, 2, 0);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 80, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_FINAL02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.18);
        macros::CAM_ZOOM_OUT(fighter);
        macros::CAM_ZOOM_IN_arg5(fighter, 3.0, 0.0, 1.9, 0.0, 0.0);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_demon_FINAL_CAMERA_OFFSET_2, *FIGHTER_demon_STATUS_WORK_ID_FINAL_INT_CAMERA_OFFSET_TYPE);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_demon_STATUS_WORK_ID_FINAL_FLAG_CAMERA_OFFSET);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_demon_STATUS_WORK_ID_FINAL_FLAG_REMOVE_FINAL_AURA);
        macros::CAM_ZOOM_OUT(fighter);
        macros::CAM_ZOOM_IN_arg5(fighter, 5.0, 0.0, 2.1, 0.0, 0.0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_demon_STATUS_WORK_ID_FINAL_FLAG_ATTACK_END_SET_PARAM);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_demon_STATUS_WORK_ID_FINAL_FLAG_ATTACK_END);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 300.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 50.0, /*X*/ 0.0, /*Y*/ 20.0, /*Z*/ 8.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_FINAL03, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.18);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_demon_FINAL_CAMERA_OFFSET_3, *FIGHTER_demon_STATUS_WORK_ID_FINAL_INT_CAMERA_OFFSET_TYPE);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_demon_STATUS_WORK_ID_FINAL_FLAG_CAMERA_OFFSET);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        SlowModule::clear_whole(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::CAM_ZOOM_OUT(fighter);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_demon_FINAL_CAMERA_OFFSET_RETURN, *FIGHTER_demon_STATUS_WORK_ID_FINAL_INT_CAMERA_OFFSET_TYPE);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_demon_STATUS_WORK_ID_FINAL_FLAG_CAMERA_OFFSET);
    }
} 

#[acmd_script(//FinalAirHit
    agent = "demon", 
    script = "game_finalairhit", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_finalhitair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        SlowModule::set_whole(fighter.module_accessor, 2, 0);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 80, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_FINAL02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.18);
        macros::CAM_ZOOM_OUT(fighter);
        macros::CAM_ZOOM_IN_arg5(fighter, 3.0, 0.0, 1.9, 0.0, 0.0);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_demon_FINAL_CAMERA_OFFSET_2, *FIGHTER_demon_STATUS_WORK_ID_FINAL_INT_CAMERA_OFFSET_TYPE);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_demon_STATUS_WORK_ID_FINAL_FLAG_CAMERA_OFFSET);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_demon_STATUS_WORK_ID_FINAL_FLAG_REMOVE_FINAL_AURA);
        macros::CAM_ZOOM_OUT(fighter, );
        macros::CAM_ZOOM_IN_arg5(fighter, 5.0, 0.0, 2.1, 0.0, 0.0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_demon_STATUS_WORK_ID_FINAL_FLAG_ATTACK_END_SET_PARAM);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_demon_STATUS_WORK_ID_FINAL_FLAG_ATTACK_END);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 300.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 50.0, /*X*/ 0.0, /*Y*/ 20.0, /*Z*/ 8.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_demon_FINAL03, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.18);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_demon_FINAL_CAMERA_OFFSET_3, *FIGHTER_demon_STATUS_WORK_ID_FINAL_INT_CAMERA_OFFSET_TYPE);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_demon_STATUS_WORK_ID_FINAL_FLAG_CAMERA_OFFSET);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        SlowModule::clear_whole(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::CAM_ZOOM_OUT(fighter);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_demon_FINAL_CAMERA_OFFSET_RETURN, *FIGHTER_demon_STATUS_WORK_ID_FINAL_INT_CAMERA_OFFSET_TYPE);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_demon_STATUS_WORK_ID_FINAL_FLAG_CAMERA_OFFSET);
    }
} 

#[acmd_script(//Move
    agent = "demon_shinkuhadoken", 
    script = "game_move", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn demon_final2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.5, /*Angle*/ 32, /*KBG*/ 100, /*FKB*/ 90, /*BKB*/ 0, /*Size*/ 23.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 75, /*BKB*/ 0, /*Size*/ 48.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.0, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 55.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 90.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 200.0, /*Angle*/ 50, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 30.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        demon_jab1w,
        demon_jab1s,
        demon_jab1ns,
        demon_jab2,
        demon_jab3,
        demon_dashattack,
        demon_sidetilts,
        demon_sidetiltw,
        demon_sidetiltn,
        demon_uptiltw,
        demon_uptilts,
        demon_downtilts,
        demon_downtiltw,
        demon_sidesmash,
        demon_upsmash,
        demon_downsmash,
        demon_nair,
        demon_fair,
        demon_bair,
        demon_uair,
        demon_dair,
        demon_grab,
        demon_dashgrab,
        demon_pivotgrab,
        demon_pummel,
        demon_throwf,
        demon_throwb,
        demon_throwhi,
        demon_throwlw,
        demon_downattacku,
        demon_downattackd,
        demon_cliffattack,
        demon_slipattack,
        demon_neutralbair,
        demon_kamehameha_fire,
        demon_kamehameha_start,
        demon_hadoken1,
        demon_hadoken2,
        demon_hadoken3,
        demon_hadoken4,
        demon_hadoken5,
        demon_hadoken6,
        demon_hadoken7,
        demon_hadoken8,
        demon_hadoken9,
        demon_sideb1,
        demon_sideb2,
        demon_sidebair1,
        demon_sidebair2,
        demon_upb,
        demon_upbair,
        demon_upb2,
        demon_upb2air,
        demon_downb,
        demon_downbturn,
        demon_downbair,
        demon_downbairturn,
        demon_sidetauntr,
        demon_sidetauntl,
        demon_uptauntr,
        demon_uptauntl,
        demon_downtauntr,
        demon_downtauntl,
        demon_finalhit,
        demon_finalhitair,
        demon_final2
    );
}
