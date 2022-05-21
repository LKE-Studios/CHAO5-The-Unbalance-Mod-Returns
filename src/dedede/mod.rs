use smash::app::sv_animcmd::*;
use smash::hash40;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;
use crate::utils::FIGHTER_CUTIN_MANAGER;

#[acmd_script(//SquatWait
    agent = "dedede", 
    script = "game_squatwait", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_crouch(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 74, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10.0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sleep"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_SLAP, /*Type*/ *ATTACK_REGION_BODY);
    }
}

#[acmd_script(//Attack11 
    agent = "dedede", 
    script = "game_attack11", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_jab1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=10)
        if(is_excute){
            FT_MOTION_RATE(FSM=1.0)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.5, Angle=361, KBG=15, FKB=0, BKB=35, Size=5.8, X=0.0, Y=6.5, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.5, Angle=361, KBG=15, FKB=0, BKB=35, Size=5.8, X=0.0, Y=6.5, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.5, Angle=361, KBG=15, FKB=0, BKB=25, Size=5.8, X=0.0, Y=6.5, Z=14.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=8.5, Angle=180, KBG=15, FKB=0, BKB=25, Size=5.8, X=0.0, Y=6.5, Z=19.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=8.5, Angle=361, KBG=15, FKB=0, BKB=25, Size=5.8, X=0.0, Y=6.5, Z=19.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            AttackModule::set_add_reaction_frame(ID=0, Frames=5.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=1, Frames=5.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=2, Frames=5.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=3, Frames=5.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=4, Frames=5.0, Unk=false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=12)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        }
        frame(Frame=22)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
        }
    });        
}

#[acmd_script(//Attack12 
    agent = "dedede", 
    script = "game_attack12", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_jab2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        FT_MOTION_RATE(FSM=0.467)
        frame(Frame=11)
        if(is_excute){
            FT_MOTION_RATE(FSM=1.0)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.2, Angle=361, KBG=15, FKB=0, BKB=35, Size=6.5, X=0.0, Y=6.5, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.2, Angle=361, KBG=15, FKB=0, BKB=30, Size=6.5, X=0.0, Y=6.5, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=7.2, Angle=361, KBG=12, FKB=0, BKB=25, Size=6.8, X=0.0, Y=6.5, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=7.2, Angle=180, KBG=12, FKB=0, BKB=25, Size=6.8, X=0.0, Y=6.5, Z=22.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=7.2, Angle=361, KBG=12, FKB=0, BKB=25, Size=6.8, X=0.0, Y=6.5, Z=22.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            AttackModule::set_add_reaction_frame(ID=0, Frames=5.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=1, Frames=5.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=2, Frames=5.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=3, Frames=5.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=4, Frames=5.0, Unk=false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=18)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
        }
        frame(Frame=22)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        }
    });
}

#[acmd_script(//Attack100 
    agent = "dedede", 
    script = "game_attack100", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_jab100(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        let lua_state = fighter.lua_state_agent;
        acmd!(lua_state, {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.2, Angle=361, KBG=12, FKB=0, BKB=10, Size=7.5, X=0.0, Y=7.5, Z=10.0, X2=0.0, Y2=7.5, Z2=15.0, Hitlag=0.5, SDI=0.6, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.2, Angle=361, KBG=12, FKB=0, BKB=8, Size=11.0, X=0.0, Y=7.5, Z=22.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.4, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
                AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=5.0)
                AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=5.0)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            frame(Frame=2)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.2, Angle=361, KBG=12, FKB=0, BKB=10, Size=7.5, X=0.0, Y=7.5, Z=10.0, X2=0.0, Y2=7.5, Z2=15.0, Hitlag=0.5, SDI=0.6, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.2, Angle=361, KBG=12, FKB=0, BKB=8, Size=11.0, X=0.0, Y=7.5, Z=22.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.4, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
                AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=5.0)
                AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=5.0)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            frame(Frame=5)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.2, Angle=361, KBG=12, FKB=0, BKB=10, Size=7.5, X=0.0, Y=7.5, Z=10.0, X2=0.0, Y2=7.5, Z2=15.0, Hitlag=0.5, SDI=0.6, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.2, Angle=361, KBG=12, FKB=0, BKB=8, Size=11.0, X=0.0, Y=7.5, Z=22.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.4, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
                AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=5.0)
                AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=5.0)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            frame(Frame=8)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.2, Angle=361, KBG=12, FKB=0, BKB=10, Size=7.5, X=0.0, Y=7.5, Z=10.0, X2=0.0, Y2=7.5, Z2=15.0, Hitlag=0.5, SDI=0.6, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.2, Angle=361, KBG=12, FKB=0, BKB=8, Size=11.0, X=0.0, Y=7.5, Z=22.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.4, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
                AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=5.0)
                AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=5.0)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            frame(Frame=11)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.2, Angle=361, KBG=12, FKB=0, BKB=10, Size=7.5, X=0.0, Y=7.5, Z=10.0, X2=0.0, Y2=7.5, Z2=15.0, Hitlag=0.5, SDI=0.6, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.2, Angle=361, KBG=12, FKB=0, BKB=8, Size=11.0, X=0.0, Y=7.5, Z=22.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.4, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
                AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=5.0)
                AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=5.0)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            frame(Frame=14)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.2, Angle=361, KBG=12, FKB=0, BKB=10, Size=7.5, X=0.0, Y=7.5, Z=10.0, X2=0.0, Y2=7.5, Z2=15.0, Hitlag=0.5, SDI=0.6, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.2, Angle=361, KBG=12, FKB=0, BKB=8, Size=11.0, X=0.0, Y=7.5, Z=22.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.4, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
                AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=5.0)
                AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=5.0)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            frame(Frame=17)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.2, Angle=361, KBG=12, FKB=0, BKB=10, Size=7.5, X=0.0, Y=7.5, Z=10.0, X2=0.0, Y2=7.5, Z2=15.0, Hitlag=0.5, SDI=0.6, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.2, Angle=361, KBG=12, FKB=0, BKB=8, Size=11.0, X=0.0, Y=7.5, Z=22.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.4, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
                AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=5.0)
                AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=5.0)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            frame(Frame=20)
            wait_loop_clear(0)
        });
    }
}

#[acmd_script(//Attack100Sub
    agent = "dedede", 
    script = "game_attack100sub", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_jab100sub(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.2, Angle=361, KBG=12, FKB=0, BKB=10, Size=7.5, X=0.0, Y=7.5, Z=10.0, X2=0.0, Y2=7.5, Z2=15.0, Hitlag=0.5, SDI=0.6, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.2, Angle=361, KBG=12, FKB=0, BKB=8, Size=11.0, X=0.0, Y=7.5, Z=22.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.4, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=5.0)
            AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=5.0)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
    });
}

#[acmd_script(//Attack100End 
    agent = "dedede", 
    script = "game_attack100end", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_jab100end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.7, Angle=49, KBG=127, FKB=0, BKB=80, Size=11.0, X=0.0, Y=13.0, Z=17.0, X2=0.0, Y2=7.5, Z2=17.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.7, Angle=49, KBG=127, FKB=0, BKB=80, Size=11.0, X=0.0, Y=13.0, Z=24.0, X2=0.0, Y2=7.5, Z2=27.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("top"), 11, 0, 13, 16, 0, 7.5, 27, 2.0, 1.25, 80, false, 2, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackDash
    agent = "dedede", 
    script = "game_attackdash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_dashattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=26)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=24.0, Angle=310, KBG=92, FKB=0, BKB=65, Size=10.0, X=0.0, Y=5.0, Z=11.8, X2=0.0, Y2=5.0, Z2=5.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
        }
        frame(Frame=28)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=23.0, Angle=310, KBG=92, FKB=0, BKB=55, Size=12.0, X=0.0, Y=4.0, Z=11.8, X2=0.0, Y2=4.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
        }
        frame(Frame=36)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=0, KBG=92, FKB=0, BKB=55, Size=9.5, X=0.0, Y=3.5, Z=10.5, X2=0.0, Y2=3.5, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
        }
        frame(Frame=39)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=0, KBG=92, FKB=0, BKB=55, Size=8.0, X=0.0, Y=3.0, Z=9.5, X2=0.0, Y2=3.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
        }
        frame(Frame=42)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackS3
    agent = "dedede", 
    script = "game_attacks3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_sidetilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        FT_MOTION_RATE(FSM=0.9)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=4.0, Angle=22, KBG=100, FKB=60, BKB=0, Size=8.0, X=-8.0, Y=0.0, Z=0.0, X2=-2.0, Y2=0.0, Z2=0.0, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("hammer1"), Damage=4.0, Angle=22, KBG=100, FKB=50, BKB=0, Size=8.5, X=1.0, Y=0.0, Z=0.0, X2=8.0, Y2=0.0, Z2=0.0, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=100, KBG=100, FKB=30, BKB=0, Size=8.0, X=0.0, Y=4.0, Z=19.2, X2=0.0, Y2=4.0, Z2=25.8, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=4.0, Angle=250, KBG=100, FKB=26, BKB=0, Size=9.0, X=0.0, Y=9.5, Z=20.0, X2=0.0, Y2=9.5, Z2=25.0, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        frame(Frame=22)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=23)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=11.0, Angle=50, KBG=161, FKB=0, BKB=30, Size=8.5, X=-8.0, Y=0.0, Z=0.0, X2=-2.0, Y2=0.0, Z2=0.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("hammer1"), Damage=11.0, Angle=50, KBG=161, FKB=0, BKB=30, Size=8.5, X=1.0, Y=0.0, Z=0.0, X2=8.0, Y2=0.0, Z2=0.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=2, Part=0, Bone=hash40("hammer2"), Damage=11.0, Angle=50, KBG=161, FKB=0, BKB=30, Size=13.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        frame(Frame=24)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackHi3
    agent = "dedede", 
    script = "game_attackhi3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_uptilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=6)
        if(is_excute){
            FighterAreaModuleImpl::enable_fix_jostle_area(7.0, 6.0)
        }
        frame(Frame=7)
        if(is_excute){
            HIT_NODE(hash40("head"), HIT_STATUS_XLU)
            HIT_NODE(hash40("shoulderl"), HIT_STATUS_XLU)
            HIT_NODE(hash40("arml"), HIT_STATUS_XLU)
            ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=17.5, Angle=96, KBG=97, FKB=0, BKB=65, Size=12.0, X=-1.5, Y=1.0, Z=1.1, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
            ATTACK(ID=1, Part=0, Bone=hash40("head"), Damage=17.5, Angle=96, KBG=97, FKB=0, BKB=65, Size=13.5, X=4.8, Y=0.9, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
            FighterAreaModuleImpl::enable_fix_jostle_area(2.0, 6.0)
            AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_HIGH), false)
        }
        frame(Frame=16)
        if(is_excute){
            HIT_NODE(hash40("head"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("shoulderl"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("arml"), HIT_STATUS_NORMAL)
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackLw3
    agent = "dedede", 
    script = "game_attacklw3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_downtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        FT_MOTION_RATE(FSM=0.3)
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=35, KBG=75, FKB=0, BKB=60, Size=12.0, X=0.0, Y=6.5, Z=12.0, X2=0.0, Y2=6.5, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=2)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.5, Angle=35, KBG=75, FKB=0, BKB=60, Size=12.0, X=0.0, Y=5.0, Z=12.0, X2=0.0, Y2=5.0, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=10)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackS4
    agent = "dedede", 
    script = "game_attacks4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_sidesmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=32)
        if(is_excute){
            FighterAreaModuleImpl::enable_fix_jostle_area(6.0, 7.0)
        }
        frame(Frame=33)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=40)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hammer2"), Damage=0.0, Angle=0, KBG=0, FKB=0, BKB=0, Size=8.5, X=-6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("hammer2"), Damage=0.0, Angle=0, KBG=0, FKB=0, BKB=0, Size=8.5, X=-12.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=2, Part=0, Bone=hash40("hammer2"), Damage=33.0, Angle=45, KBG=55, FKB=0, BKB=85, Size=10.5, X=1.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            methodlib::L2CValue::as_hash()const(0, hash40("top"), 16, 2, 2)
            AttackModule::set_vec_target_pos()
            methodlib::L2CValue::as_hash()const(1, hash40("top"), 16, 2, 2)
            AttackModule::set_vec_target_pos()
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear(ID=0, false)
            ATTACK(ID=1, Part=0, Bone=hash40("hammer1"), Damage=35.0, Angle=45, KBG=85, FKB=0, BKB=50, Size=8.5, X=6.0, Y=-3.0, Z=0.0, X2=3.0, Y2=-5.0, Z2=0.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=2, Part=0, Bone=hash40("hammer2"), Damage=35.0, Angle=361, KBG=74, FKB=0, BKB=70, Size=11.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        frame(Frame=44)
        if(is_excute){
            AttackModule::clear(ID=1, false)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=30.0, Angle=60, KBG=30, FKB=0, BKB=100, Size=14.0, X=0.0, Y=5.0, Z=5.0, X2=0.0, Y2=5.0, Z2=27.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        wait(Frames=5)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackHi4
    agent = "dedede", 
    script = "game_attackhi4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_upsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        FT_MOTION_RATE(FSM=0.7)
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=17)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hammer2"), Damage=25.0, Angle=80, KBG=95, FKB=0, BKB=40, Size=12.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("hammer2"), Damage=25.0, Angle=80, KBG=96, FKB=0, BKB=40, Size=10.0, X=-6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=2, Part=0, Bone=hash40("hammer2"), Damage=25.0, Angle=80, KBG=96, FKB=0, BKB=40, Size=10.0, X=-12.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=23.0, Angle=80, KBG=96, FKB=0, BKB=40, Size=8.0, X=0.0, Y=6.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        frame(Frame=29)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackLw4
    agent = "dedede", 
    script = "game_attacklw4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_downsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=14)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hammer2"), Damage=22.0, Angle=20, KBG=80, FKB=0, BKB=60, Size=12.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("hammer2"), Damage=22.0, Angle=20, KBG=80, FKB=0, BKB=60, Size=10.0, X=-7.0, Y=-3.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        frame(Frame=23)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackAirN
    agent = "dedede", 
    script = "game_attackairn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        FT_MOTION_RATE(FSM=0.6)
        frame(Frame=3)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=320, KBG=101, FKB=0, BKB=40, Size=14.0, X=0.0, Y=12.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        }
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=18.0, Angle=325, KBG=101, FKB=0, BKB=40, Size=11.0, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        }
        frame(Frame=30)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=35)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(//AttackAirF 
    agent = "dedede", 
    script = "game_attackairf", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        FT_MOTION_RATE(FSM=0.7)
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=13)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hammer2"), Damage=21.0, Angle=275, KBG=98, FKB=0, BKB=25, Size=12.0, X=-2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("hammer2"), Damage=21.0, Angle=280, KBG=98, FKB=0, BKB=25, Size=9.0, X=-12.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        frame(Frame=16)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=40)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(//AttackAirB
    agent = "dedede", 
    script = "game_attackairb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=17)
        if(is_excute){
            FT_MOTION_RATE(FSM=1.0)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=88, FKB=0, BKB=30, Size=10.5, X=0.0, Y=10.0, Z=-18.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=88, FKB=0, BKB=30, Size=10.5, X=0.0, Y=10.0, Z=-12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=88, FKB=0, BKB=30, Size=10.5, X=0.0, Y=10.0, Z=-6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        frame(Frame=20)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=33)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(//AttackAirHi
    agent = "dedede", 
    script = "game_attackairhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=10)
        for(7 Iterations){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=2.5, Angle=367, KBG=100, FKB=20, BKB=0, Size=12.0, X=17.0, Y=0.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
                ATTACK(ID=1, Part=0, Bone=hash40("hammer1"), Damage=2.5, Angle=367, KBG=100, FKB=20, BKB=0, Size=12.0, X=17.0, Y=0.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
                ATTACK(ID=2, Part=0, Bone=hash40("hammer1"), Damage=2.5, Angle=95, KBG=100, FKB=40, BKB=0, Size=10.0, X=9.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
                ATTACK(ID=3, Part=0, Bone=hash40("hammer1"), Damage=2.5, Angle=95, KBG=100, FKB=40, BKB=0, Size=10.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
            }
            wait(Frames=1)
        }
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=14.0, Angle=60, KBG=141, FKB=0, BKB=55, Size=12.5, X=17.0, Y=0.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("hammer1"), Damage=14.0, Angle=60, KBG=141, FKB=0, BKB=55, Size=12.5, X=17.0, Y=0.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=2, Part=0, Bone=hash40("hammer1"), Damage=14.0, Angle=60, KBG=141, FKB=0, BKB=55, Size=11.0, X=9.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=3, Part=0, Bone=hash40("hammer1"), Damage=14.0, Angle=60, KBG=141, FKB=0, BKB=55, Size=11.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        wait(Frames=4)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=42)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(//AttackAirLw
    agent = "dedede", 
    script = "game_attackairlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=7)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=22)
        if(is_excute){
            FT_MOTION_RATE(FSM=1.0)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=25.0, Angle=270, KBG=90, FKB=0, BKB=20, Size=10.5, X=0.0, Y=-6.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=25.0, Angle=270, KBG=100, FKB=0, BKB=20, Size=12.0, X=0.0, Y=-6.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=22.0, Angle=270, KBG=100, FKB=0, BKB=20, Size=14.5, X=0.0, Y=-5.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        frame(Frame=25)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=44)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(//Catch
    agent = "dedede", 
    script = "game_catch", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_grab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=8)
        if(is_excute){
            CATCH(ID=0, Bone=hash40("top"), Size=6.5, X=0.0, Y=5.0, Z=4.0, X2=0.0, Y2=5.0, Z2=15.5, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
            CATCH(ID=1, Bone=hash40("top"), Size=4.25, X=0.0, Y=5.0, Z=1.75, X2=0.0, Y2=5.0, Z2=17.75, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
        }
        rust {
            macros::game_CaptureCutCommon(fighter);
        }
        wait(Frames=3)
        if(is_excute){
            sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
            GrabModule::set_rebound(CanCatchRebound=false)
        }
    });
}

#[acmd_script(//CatchDash
    agent = "dedede", 
    script = "game_catchdash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_dashgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=10)
        if(is_excute){
            GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=11)
        if(is_excute){
            CATCH(ID=0, Bone=hash40("top"), Size=5.6, X=0.0, Y=7.0, Z=4.0, X2=0.0, Y2=7.0, Z2=18.4, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
            CATCH(ID=1, Bone=hash40("top"), Size=3.8, X=0.0, Y=7.0, Z=2.2, X2=0.0, Y2=7.0, Z2=20.2, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
        }
        rust {
            macros::game_CaptureCutCommon(fighter);
        }
        wait(Frames=3)
        if(is_excute){
            sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
            GrabModule::set_rebound(CanCatchRebound=false)
        }
    });
}

#[acmd_script(//CatchTurn
    agent = "dedede", 
    script = "game_catchturn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_pivotgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=11)
        if(is_excute){
            GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=12)
        if(is_excute){
            CATCH(ID=0, Bone=hash40("top"), Size=6.5, X=0.0, Y=6.0, Z=-4.0, X2=0.0, Y2=6.0, Z2=-24.3, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
            CATCH(ID=1, Bone=hash40("top"), Size=4.25, X=0.0, Y=6.0, Z=-1.75, X2=0.0, Y2=6.0, Z2=-26.55, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
        }
        rust {
            macros::game_CaptureCutCommon(fighter);
        }
        wait(Frames=3)
        if(is_excute){
            sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
            GrabModule::set_rebound(CanCatchRebound=false)
        }
    });
}

#[acmd_script(//CatchAttack
    agent = "dedede", 
    script = "game_catchattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_pummel(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        FT_MOTION_RATE(FSM=0.574)
        frame(Frame=2)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=100, FKB=30, BKB=0, Size=5.3, X=0.0, Y=10.0, Z=12.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.66, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
            AttackModule::set_catch_only_all(true, false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//ThrowF
    agent = "dedede", 
    script = "game_throwf", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_throwf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=12.0, Angle=45, KBG=70, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=12.0, Angle=361, KBG=150, FKB=0, BKB=30, Size=6.0, X=17.0, Y=0.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("hammer1"), Damage=12.0, Angle=361, KBG=150, FKB=0, BKB=30, Size=6.0, X=17.0, Y=0.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=2, Part=0, Bone=hash40("hammer1"), Damage=12.0, Angle=361, KBG=150, FKB=0, BKB=30, Size=4.0, X=9.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=3, Part=0, Bone=hash40("hammer1"), Damage=12.0, Angle=361, KBG=150, FKB=0, BKB=30, Size=4.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            AttackModule::set_catch_only_all(true, false)
        }
        frame(Frame=13)
        if(is_excute){
            CHECK_FINISH_CAMERA(20, 0)
            rust {
                lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.8);
                lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, Vector3f{x: 5.0, y: 0.0, z: 0.0});
            }
        }
        frame(Frame=14)
        if(is_excute){
            rust {
                let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
                let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
                let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
                macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
            }
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//ThrowB
    agent = "dedede", 
    script = "game_throwb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=13.0, Angle=60, KBG=65, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=6)
        if(is_excute){
            REVERSE_LR()
        }
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=12.0, Angle=361, KBG=150, FKB=0, BKB=30, Size=6.0, X=17.0, Y=0.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("hammer1"), Damage=12.0, Angle=361, KBG=150, FKB=0, BKB=30, Size=6.0, X=17.0, Y=0.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=2, Part=0, Bone=hash40("hammer1"), Damage=12.0, Angle=361, KBG=150, FKB=0, BKB=30, Size=4.0, X=9.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=3, Part=0, Bone=hash40("hammer1"), Damage=12.0, Angle=361, KBG=150, FKB=0, BKB=30, Size=4.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            AttackModule::set_catch_only_all(true, false)
        }
        frame(Frame=18)
        if(is_excute){
            CHECK_FINISH_CAMERA(28, 8)
            FighterCutInManager::set_throw_finish_zoom_rate(1.2)
            rust {
                lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.8);
                lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, Vector3f{x: 8.0, y: 0.0, z: 0.0});
            }
        }
        frame(Frame=19)
        if(is_excute){
            rust {
                let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
                let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
                let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
                macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
            }
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//ThrowHi
    agent = "dedede", 
    script = "game_throwhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_throwup(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=11.0, Angle=90, KBG=57, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=80, FKB=0, BKB=30, Size=7.0, X=0.0, Y=29.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_catch_only_all(true, false)
        }
        frame(Frame=18)
        if(is_excute){
            CHECK_FINISH_CAMERA(2, 32)
            rust {
                lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.7);
                lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, Vector3f{x: 0.0, y: 12.0, z: 0.0});
            }
        }
        frame(Frame=19)
        if(is_excute){
            rust {
                let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
                let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
                let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
                macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
            }
            AttackModule::clear_all()
        }
    });
}      

#[acmd_script(//ThrowLw
    agent = "dedede", 
    script = "game_throwlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_throwdown(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=18.0, Angle=80, KBG=82, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=24)
        if(is_excute){
            FT_CATCH_STOP(8, 1)
        }
        frame(Frame=25)
        if(is_excute){
            CHECK_FINISH_CAMERA(8, 0)
            rust {
                lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.3);
                lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, Vector3f{x: 0.0, y: 2.0, z: 0.0});
            }
        }
        frame(Frame=26)
        if(is_excute){
            rust {
                let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
                let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
                let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
                macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
            }
        }
    });
}

#[acmd_script(//CliffAttack
    agent = "dedede", 
    script = "game_cliffattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_cliffattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=24)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=18.0, Angle=45, KBG=50, FKB=0, BKB=90, Size=10.76, X=0.0, Y=5.5, Z=18.0, X2=0.0, Y2=5.5, Z2=3.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SlipAttack
    agent = "dedede", 
    script = "game_slipattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_slipattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=19)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=50, FKB=0, BKB=60, Size=11.0, X=0.0, Y=6.0, Z=19.5, X2=0.0, Y2=6.0, Z2=3.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=31)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=50, FKB=0, BKB=60, Size=11.0, X=0.0, Y=6.0, Z=-21.0, X2=0.0, Y2=6.0, Z2=-5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//DownAttackD
    agent = "dedede", 
    script = "game_downattackd", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_downattackd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=18)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=48, KBG=48, FKB=0, BKB=80, Size=12.0, X=0.0, Y=6.0, Z=17.0, X2=0.0, Y2=6.0, Z2=3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=23)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=48, KBG=48, FKB=0, BKB=80, Size=12.0, X=0.0, Y=6.0, Z=-17.0, X2=0.0, Y2=6.0, Z2=-3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//DownAttackU
    agent = "dedede", 
    script = "game_downattacku", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_downattacku(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=18)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=48, KBG=48, FKB=0, BKB=80, Size=12.0, X=0.0, Y=6.0, Z=17.0, X2=0.0, Y2=6.0, Z2=3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=23)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=48, KBG=48, FKB=0, BKB=80, Size=12.0, X=0.0, Y=6.0, Z=-17.0, X2=0.0, Y2=6.0, Z2=-3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialNStart
    agent = "dedede", 
    script = "game_specialnstart", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_neutralb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.8)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=17)
        FT_MOTION_RATE(FSM=1.0)
        if(is_excute){
            CATCH(ID=0, Bone=hash40("top"), Size=22.0, X=0.0, Y=9.0, Z=4.0, X2=0.0, Y2=9.0, Z2=7.0, Status=FIGHTER_STATUS_KIND_SWALLOWED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=5.0, Angle=361, KBG=100, FKB=0, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            SEARCH(0, 0, hash40("top"), 4.0, 0.0, 13.0, 5.5, 0.0, 3.5, 5.5, COLLISION_KIND_MASK_AH, HIT_STATUS_MASK_ALL, 1, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_ALL, true)
            search(MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, COLLISION_TARGET_PROPERTY, COLLISION_PROPERTY_MASK_REFLECT)
        }
        if(0x157710(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                CATCH(ID=0, Bone=hash40("top"), Size=35.5, X=0.0, Y=7.0, Z=4.0, X2=0.0, Y2=7.0, Z2=7.0, Status=FIGHTER_STATUS_KIND_SWALLOWED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=5.0, Angle=361, KBG=100, FKB=0, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
                SEARCH(0, 0, hash40("top"), 4.0, 0.0, 9.5, 5.5, 0.0, 2.5, 5.5, COLLISION_KIND_MASK_AH, HIT_STATUS_MASK_ALL, 1, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_ALL, true)
                search(MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, COLLISION_TARGET_PROPERTY, COLLISION_PROPERTY_MASK_REFLECT)
            }
        }
    });
}

#[acmd_script(//SpecialNLoop
    agent = "dedede", 
    script = "game_specialnloop", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_neutralbloop(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_INHALE)
            CATCH(ID=0, Bone=hash40("top"), Size=15.0, X=0.0, Y=9.0, Z=5.0, X2=0.0, Y2=9.0, Z2=40.0, Status=FIGHTER_STATUS_KIND_SWALLOWED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.0, Angle=170, KBG=100, FKB=30, BKB=0, Size=20.0, X=0.0, Y=9.0, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA_d, Hitbits=COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.0, Angle=170, KBG=100, FKB=10, BKB=0, Size=20.0, X=0.0, Y=6.0, Z=23.0, X2=0.0, Y2=12.0, Z2=23.0, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA_d, Hitbits=COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.0, Angle=0, KBG=0, FKB=0, BKB=0, Size=9.0, X=0.0, Y=8.5, Z=10.0, X2=0.0, Y2=8.5, Z2=21.0, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA_d, Hitbits=COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=30.0, Angle=361, KBG=100, FKB=0, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            SEARCH(0, 0, hash40("top"), 4.0, 0.0, 13.0, 5.5, 0.0, 3.5, 5.5, COLLISION_KIND_MASK_AH, HIT_STATUS_MASK_ALL, 1, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_ALL, true)
            search(MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, COLLISION_TARGET_PROPERTY, COLLISION_PROPERTY_MASK_REFLECT)
            SEARCH(1, 0, hash40("top"), 9.5, 0.0, 8.5, 11.0, 0.0, 8.5, 20.5, COLLISION_KIND_MASK_AH, HIT_STATUS_MASK_ALL, 1, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_ALL, true)
            search(MA_MSC_CMD_SEARCH_SET_OPPONENT, 1, 0, COLLISION_TARGET_PROPERTY, COLLISION_PROPERTY_MASK_REFLECT)
        }
        if(0x157710(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
                WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_INHALE)
                CATCH(ID=0, Bone=hash40("top"), Size=15.0, X=0.0, Y=5.5, Z=5.0, X2=0.0, Y2=5.5, Z2=40.0, Status=FIGHTER_STATUS_KIND_SWALLOWED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.0, Angle=170, KBG=100, FKB=30, BKB=0, Size=20.0, X=0.0, Y=6.5, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA_d, Hitbits=COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.0, Angle=170, KBG=100, FKB=10, BKB=0, Size=20.0, X=0.0, Y=3.5, Z=23.0, X2=0.0, Y2=9.5, Z2=23.0, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA_d, Hitbits=COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.0, Angle=0, KBG=0, FKB=0, BKB=0, Size=9.0, X=0.0, Y=6.0, Z=10.0, X2=0.0, Y2=6.0, Z2=21.0, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA_d, Hitbits=COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=30.0, Angle=361, KBG=100, FKB=0, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
                SEARCH(0, 0, hash40("top"), 4.0, 0.0, 9.5, 5.5, 0.0, 2.5, 5.5, COLLISION_KIND_MASK_AH, HIT_STATUS_MASK_ALL, 1, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_ALL, true)
                search(MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, COLLISION_TARGET_PROPERTY, COLLISION_PROPERTY_MASK_REFLECT)
                SEARCH(1, 0, hash40("top"), 9.5, 0.0, 6.0, 11.0, 0.0, 6.0, 20.5, COLLISION_KIND_MASK_AH, HIT_STATUS_MASK_ALL, 1, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_ALL, true)
                search(MA_MSC_CMD_SEARCH_SET_OPPONENT, 1, 0, COLLISION_TARGET_PROPERTY, COLLISION_PROPERTY_MASK_REFLECT)
            }
        }
    });
}

#[acmd_script(//SpecialNSpit
    agent = "dedede", 
    script = "game_specialnspit", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_neutralbspit(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=40.0, Angle=361, KBG=0, FKB=0, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=5.0, Angle=361, KBG=100, FKB=0, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            FighterAreaModuleImpl::enable_fix_jostle_area(14.7, 4)
        }
        if(0x157710(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                FighterAreaModuleImpl::enable_fix_jostle_area(4, 4)
            }
        }
        if(is_excute){
            KineticModule::suspend_energy(FIGHTER_KINETIC_ENERGY_ID_CONTROL)
        }
        frame(Frame=5)
        if(is_excute){
            FighterAreaModuleImpl::enable_fix_jostle_area(6.5, 5)
        }
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_SPIT)
            ArticleModule::generate_article(FIGHTER_DEDEDE_GENERATE_ARTICLE_STAR_MISSILE)
            ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), FIGHTER_DEDEDE_STATUS_SPECIAL_N_WORK_INT_TARGET_TASK, FIGHTER_DEDEDE_STATUS_SPECIAL_N_WORK_INT_TARGET_HIT_GROUP, FIGHTER_DEDEDE_STATUS_SPECIAL_N_WORK_INT_TARGET_HIT_NO)
        }
    });
}

#[acmd_script(//SpecialNSwallow
    agent = "dedede", 
    script = "game_specialnswallow", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_neutralbswallow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=999.0, Angle=361, KBG=100, FKB=0, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
        }
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_SWALLOW_COMP)
        }
        frame(Frame=6)
        if(is_excute){
            FighterAreaModuleImpl::enable_fix_jostle_area(14.7, 4)
        }
        if(0x157710(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                FighterAreaModuleImpl::enable_fix_jostle_area(4, 4)
            }
        }
    });
}

#[acmd_script(//SpecialAirNStart
    agent = "dedede", 
    script = "game_specialairnstart", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_neutralbair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.8)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=17)
        FT_MOTION_RATE(FSM=1.0)
        if(is_excute){
            CATCH(ID=0, Bone=hash40("top"), Size=22.0, X=0.0, Y=9.0, Z=4.0, X2=0.0, Y2=9.0, Z2=7.0, Status=FIGHTER_STATUS_KIND_SWALLOWED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=5.0, Angle=361, KBG=100, FKB=0, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            SEARCH(0, 0, hash40("top"), 4.0, 0.0, 13.0, 5.5, 0.0, 3.5, 5.5, COLLISION_KIND_MASK_AH, HIT_STATUS_MASK_ALL, 1, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_ALL, true)
            search(MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, COLLISION_TARGET_PROPERTY, COLLISION_PROPERTY_MASK_REFLECT)
        }
        if(0x157710(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                CATCH(ID=0, Bone=hash40("top"), Size=35.5, X=0.0, Y=7.0, Z=4.0, X2=0.0, Y2=7.0, Z2=7.0, Status=FIGHTER_STATUS_KIND_SWALLOWED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=5.0, Angle=361, KBG=100, FKB=0, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
                SEARCH(0, 0, hash40("top"), 4.0, 0.0, 9.5, 5.5, 0.0, 2.5, 5.5, COLLISION_KIND_MASK_AH, HIT_STATUS_MASK_ALL, 1, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_ALL, true)
                search(MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, COLLISION_TARGET_PROPERTY, COLLISION_PROPERTY_MASK_REFLECT)
            }
        }
    });
}

#[acmd_script(//SpecialAirNLoop
    agent = "dedede", 
    script = "game_specialairnloop", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_neutralbairloop(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_INHALE)
            CATCH(ID=0, Bone=hash40("top"), Size=15.0, X=0.0, Y=9.0, Z=5.0, X2=0.0, Y2=9.0, Z2=40.0, Status=FIGHTER_STATUS_KIND_SWALLOWED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.0, Angle=170, KBG=100, FKB=30, BKB=0, Size=20.0, X=0.0, Y=9.0, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA_d, Hitbits=COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.0, Angle=170, KBG=100, FKB=10, BKB=0, Size=20.0, X=0.0, Y=6.0, Z=23.0, X2=0.0, Y2=12.0, Z2=23.0, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA_d, Hitbits=COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.0, Angle=0, KBG=0, FKB=0, BKB=0, Size=9.0, X=0.0, Y=8.5, Z=10.0, X2=0.0, Y2=8.5, Z2=21.0, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA_d, Hitbits=COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=30.0, Angle=361, KBG=100, FKB=0, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            SEARCH(0, 0, hash40("top"), 4.0, 0.0, 13.0, 5.5, 0.0, 3.5, 5.5, COLLISION_KIND_MASK_AH, HIT_STATUS_MASK_ALL, 1, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_ALL, true)
            search(MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, COLLISION_TARGET_PROPERTY, COLLISION_PROPERTY_MASK_REFLECT)
            SEARCH(1, 0, hash40("top"), 9.5, 0.0, 8.5, 11.0, 0.0, 8.5, 20.5, COLLISION_KIND_MASK_AH, HIT_STATUS_MASK_ALL, 1, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_ALL, true)
            search(MA_MSC_CMD_SEARCH_SET_OPPONENT, 1, 0, COLLISION_TARGET_PROPERTY, COLLISION_PROPERTY_MASK_REFLECT)
        }
        if(0x157710(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
                WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_INHALE)
                CATCH(ID=0, Bone=hash40("top"), Size=15.0, X=0.0, Y=5.5, Z=5.0, X2=0.0, Y2=5.5, Z2=40.0, Status=FIGHTER_STATUS_KIND_SWALLOWED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.0, Angle=170, KBG=100, FKB=30, BKB=0, Size=20.0, X=0.0, Y=6.5, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA_d, Hitbits=COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.0, Angle=170, KBG=100, FKB=10, BKB=0, Size=20.0, X=0.0, Y=3.5, Z=23.0, X2=0.0, Y2=9.5, Z2=23.0, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA_d, Hitbits=COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.0, Angle=0, KBG=0, FKB=0, BKB=0, Size=9.0, X=0.0, Y=6.0, Z=10.0, X2=0.0, Y2=6.0, Z2=21.0, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA_d, Hitbits=COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=30.0, Angle=361, KBG=100, FKB=0, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
                SEARCH(0, 0, hash40("top"), 4.0, 0.0, 9.5, 5.5, 0.0, 2.5, 5.5, COLLISION_KIND_MASK_AH, HIT_STATUS_MASK_ALL, 1, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_ALL, true)
                search(MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, COLLISION_TARGET_PROPERTY, COLLISION_PROPERTY_MASK_REFLECT)
                SEARCH(1, 0, hash40("top"), 9.5, 0.0, 6.0, 11.0, 0.0, 6.0, 20.5, COLLISION_KIND_MASK_AH, HIT_STATUS_MASK_ALL, 1, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_ALL, true)
                search(MA_MSC_CMD_SEARCH_SET_OPPONENT, 1, 0, COLLISION_TARGET_PROPERTY, COLLISION_PROPERTY_MASK_REFLECT)
            }
        }
    });
}

#[acmd_script(//SpecialAirNSpit
    agent = "dedede", 
    script = "game_specialairnspit", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_neutralbairspit(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=40.0, Angle=361, KBG=0, FKB=0, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=5.0, Angle=361, KBG=100, FKB=0, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            FighterAreaModuleImpl::enable_fix_jostle_area(14.7, 4)
        }
        if(0x157710(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                FighterAreaModuleImpl::enable_fix_jostle_area(4, 4)
            }
        }
        if(is_excute){
            KineticModule::suspend_energy(FIGHTER_KINETIC_ENERGY_ID_CONTROL)
        }
        frame(Frame=5)
        if(is_excute){
            FighterAreaModuleImpl::enable_fix_jostle_area(6.5, 5)
        }
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_SPIT)
            ArticleModule::generate_article(FIGHTER_DEDEDE_GENERATE_ARTICLE_STAR_MISSILE)
            ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), FIGHTER_DEDEDE_STATUS_SPECIAL_N_WORK_INT_TARGET_TASK, FIGHTER_DEDEDE_STATUS_SPECIAL_N_WORK_INT_TARGET_HIT_GROUP, FIGHTER_DEDEDE_STATUS_SPECIAL_N_WORK_INT_TARGET_HIT_NO)
        }
    });
}

#[acmd_script(//SpecialAirNSwallow
    agent = "dedede", 
    script = "game_specialairnswallow", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_neutralbairswallow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=999.0, Angle=361, KBG=100, FKB=0, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
        }
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_SWALLOW_COMP)
        }
        frame(Frame=6)
        if(is_excute){
            FighterAreaModuleImpl::enable_fix_jostle_area(14.7, 4)
        }
        if(0x157710(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                FighterAreaModuleImpl::enable_fix_jostle_area(4, 4)
            }
        }
    });
}

#[acmd_script(//SpecialSStart
    agent = "dedede", 
    script = "game_specialsstart", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_sideb1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_GENERATE)
        }
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_PUTOUT)
        }
        frame(Frame=29)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_THROW)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=30.0, Angle=361, KBG=40, FKB=0, BKB=100, Size=12.5, X=0.0, Y=8.0, Z=14.0, X2=0.0, Y2=8.0, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialSStartSub
    agent = "dedede", 
    script = "game_specialsstartsub", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_sideb2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=30.0, Angle=361, KBG=40, FKB=0, BKB=100, Size=12.5, X=0.0, Y=8.0, Z=14.0, X2=0.0, Y2=8.0, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialSMiss
    agent = "dedede", 
    script = "game_specialsmiss", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_sideb3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_MISS_SEARCH)
        }
        frame(Frame=6)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_MISS_SEARCH)
        }
        frame(Frame=29)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=30.0, Angle=361, KBG=40, FKB=0, BKB=100, Size=12.5, X=0.0, Y=8.0, Z=14.0, X2=0.0, Y2=8.0, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialAirSStart
    agent = "dedede", 
    script = "game_specialairsstart", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_sidebair1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_GENERATE)
        }
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_PUTOUT)
        }
        frame(Frame=29)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_THROW)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=30.0, Angle=361, KBG=40, FKB=0, BKB=100, Size=12.5, X=0.0, Y=8.0, Z=14.0, X2=0.0, Y2=8.0, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialAirSStartSub
    agent = "dedede", 
    script = "game_specialairsstartsub", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_sidebair2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=30.0, Angle=361, KBG=40, FKB=0, BKB=100, Size=12.5, X=0.0, Y=8.0, Z=14.0, X2=0.0, Y2=8.0, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialAirSMiss
    agent = "dedede", 
    script = "game_specialairsmiss", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_sidebair3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_MISS_SEARCH)
        }
        frame(Frame=6)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_MISS_SEARCH)
        }
        frame(Frame=29)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=30.0, Angle=361, KBG=40, FKB=0, BKB=100, Size=12.5, X=0.0, Y=8.0, Z=14.0, X2=0.0, Y2=8.0, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialSAttack
    agent = "dedede_gordo", 
    script = "game_specialsattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_gordo1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=25.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=5.9, X=3.8, Y=3.8, Z=0.0, X2=-3.8, Y2=-3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=25.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=5.9, X=3.8, Y=-3.8, Z=0.0, X2=-3.8, Y2=3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=25.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
        }
        frame(Frame=30)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=30.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=5.9, X=3.8, Y=3.8, Z=0.0, X2=-3.8, Y2=-3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-6.2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=30.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=5.9, X=3.8, Y=-3.8, Z=0.0, X2=-3.8, Y2=3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-6.2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=30.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-6.2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
        }
        frame(Frame=60)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=40.0, Angle=60, KBG=54, FKB=0, BKB=30, Size=5.9, X=3.8, Y=3.8, Z=0.0, X2=-3.8, Y2=-3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-10.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=40.0, Angle=60, KBG=54, FKB=0, BKB=30, Size=5.9, X=3.8, Y=-3.8, Z=0.0, X2=-3.8, Y2=3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-10.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=40.0, Angle=60, KBG=54, FKB=0, BKB=30, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-10.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
        }
        frame(Frame=90)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=50.0, Angle=60, KBG=48, FKB=0, BKB=21, Size=0.9, X=3.8, Y=3.8, Z=0.0, X2=-3.8, Y2=-3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-24.7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=50.0, Angle=60, KBG=48, FKB=0, BKB=21, Size=0.9, X=3.8, Y=-3.8, Z=0.0, X2=-3.8, Y2=3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-24.7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=50.0, Angle=60, KBG=48, FKB=0, BKB=21, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-24.7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            AttackModule::set_poison_param(ID=0, Frames=480, Rehit=40, Damage=1.0, Unk=false)
            AttackModule::set_poison_param(ID=1, Frames=480, Rehit=40, Damage=1.0, Unk=false)
            AttackModule::set_poison_param(ID=2, Frames=480, Rehit=40, Damage=1.0, Unk=false)
        }
    });
}

#[acmd_script(//SpecialSShot
    agent = "dedede_gordo", 
    script = "game_specialsshot", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_gordo2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=25.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=5.9, X=3.8, Y=3.8, Z=0.0, X2=-3.8, Y2=-3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=25.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=5.9, X=3.8, Y=-3.8, Z=0.0, X2=-3.8, Y2=3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=25.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
        }
        frame(Frame=15)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=30.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=5.9, X=3.8, Y=3.8, Z=0.0, X2=-3.8, Y2=-3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-6.2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=30.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=5.9, X=3.8, Y=-3.8, Z=0.0, X2=-3.8, Y2=3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-6.2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=30.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-6.2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
        }
        frame(Frame=30)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=40.0, Angle=60, KBG=54, FKB=0, BKB=30, Size=5.9, X=3.8, Y=3.8, Z=0.0, X2=-3.8, Y2=-3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-10.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=40.0, Angle=60, KBG=54, FKB=0, BKB=30, Size=5.9, X=3.8, Y=-3.8, Z=0.0, X2=-3.8, Y2=3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-10.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=40.0, Angle=60, KBG=54, FKB=0, BKB=30, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-10.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
        }
        frame(Frame=45)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=50.0, Angle=60, KBG=48, FKB=0, BKB=21, Size=0.9, X=3.8, Y=3.8, Z=0.0, X2=-3.8, Y2=-3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-24.7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=50.0, Angle=60, KBG=48, FKB=0, BKB=21, Size=0.9, X=3.8, Y=-3.8, Z=0.0, X2=-3.8, Y2=3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-24.7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=50.0, Angle=60, KBG=48, FKB=0, BKB=21, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-24.7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            AttackModule::set_poison_param(ID=0, Frames=480, Rehit=40, Damage=1.0, Unk=false)
            AttackModule::set_poison_param(ID=1, Frames=480, Rehit=40, Damage=1.0, Unk=false)
            AttackModule::set_poison_param(ID=2, Frames=480, Rehit=40, Damage=1.0, Unk=false)
        }
    });
}

#[acmd_script(//SpecialSThrow
    agent = "dedede_gordo", 
    script = "game_specialsthrow", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_gordo3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=25.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=5.9, X=3.8, Y=3.8, Z=0.0, X2=-3.8, Y2=-3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=25.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=5.9, X=3.8, Y=-3.8, Z=0.0, X2=-3.8, Y2=3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=25.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
        }
        frame(Frame=30)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=30.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=5.9, X=3.8, Y=3.8, Z=0.0, X2=-3.8, Y2=-3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-6.2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=30.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=5.9, X=3.8, Y=-3.8, Z=0.0, X2=-3.8, Y2=3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-6.2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=30.0, Angle=60, KBG=0, FKB=0, BKB=0, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-6.2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
        }
        frame(Frame=60)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=40.0, Angle=60, KBG=54, FKB=0, BKB=30, Size=5.9, X=3.8, Y=3.8, Z=0.0, X2=-3.8, Y2=-3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-10.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=40.0, Angle=60, KBG=54, FKB=0, BKB=30, Size=5.9, X=3.8, Y=-3.8, Z=0.0, X2=-3.8, Y2=3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-10.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=40.0, Angle=60, KBG=54, FKB=0, BKB=30, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-10.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
        }
        frame(Frame=90)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=50.0, Angle=60, KBG=48, FKB=0, BKB=21, Size=0.9, X=3.8, Y=3.8, Z=0.0, X2=-3.8, Y2=-3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-24.7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=50.0, Angle=60, KBG=48, FKB=0, BKB=21, Size=0.9, X=3.8, Y=-3.8, Z=0.0, X2=-3.8, Y2=3.8, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-24.7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=50.0, Angle=60, KBG=48, FKB=0, BKB=21, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-24.7, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            AttackModule::set_poison_param(ID=0, Frames=480, Rehit=40, Damage=1.0, Unk=false)
            AttackModule::set_poison_param(ID=1, Frames=480, Rehit=40, Damage=1.0, Unk=false)
            AttackModule::set_poison_param(ID=2, Frames=480, Rehit=40, Damage=1.0, Unk=false)
        }
    });
}

#[acmd_script(//SpecialSWallStop
    agent = "dedede_gordo", 
    script = "game_specialswallstop", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_gordo4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=60.0, Angle=275, KBG=50, FKB=0, BKB=50, Size=6.9, X=5.3, Y=5.3, Z=0.0, X2=-5.3, Y2=-5.3, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-5, Trip=0.0, Rehit=60, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=60.0, Angle=275, KBG=50, FKB=0, BKB=50, Size=6.9, X=5.3, Y=-5.3, Z=0.0, X2=-5.3, Y2=5.3, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-5, Trip=0.0, Rehit=60, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=60.0, Angle=275, KBG=50, FKB=0, BKB=50, Size=8.5, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-5, Trip=0.0, Rehit=60, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            AttackModule::set_poison_param(ID=0, Frames=480, Rehit=40, Damage=1.0, Unk=false)
            AttackModule::set_poison_param(ID=1, Frames=480, Rehit=40, Damage=1.0, Unk=false)
            AttackModule::set_poison_param(ID=2, Frames=480, Rehit=40, Damage=1.0, Unk=false)
        }
    });
}

#[acmd_script(//SpecialHiJump
    agent = "dedede", 
    script = "game_specialhijump", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_upb1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS)
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=5)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
        frame(Frame=11)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_SUPER_JUMP_WORK_FLAG_TURN)
        }
        frame(Frame=52)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_NONE)
            rust {
                KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: -4.0, z: 0.0});
            }
            HIT_NODE(hash40("footr"), HIT_STATUS_XLU)
            HIT_NODE(hash40("footl"), HIT_STATUS_XLU)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=40.0, Angle=270, KBG=80, FKB=0, BKB=52, Size=14.8, X=0.0, Y=7.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_bury"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=40.0, Angle=270, KBG=80, FKB=0, BKB=52, Size=12.5, X=0.0, Y=7.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
        }
    });
}

#[acmd_script(//SpecialHiLandingL
    agent = "dedede", 
    script = "game_specialhilandingl", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_upb2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            HitModule::set_status_all(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            HIT_NODE(hash40("virtualwaist"), HIT_STATUS_OFF)
        }
        frame(Frame=3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=43.0, Angle=270, KBG=70, FKB=0, BKB=85, Size=16.0, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=43.0, Angle=60, KBG=71, FKB=0, BKB=88, Size=17.0, X=0.0, Y=8.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=43.0, Angle=60, KBG=71, FKB=0, BKB=88, Size=17.0, X=0.0, Y=8.0, Z=-10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=2)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_SUPER_JUMP_WORK_FLAG_STAR_RIGHT)
            ArticleModule::generate_article(FIGHTER_DEDEDE_GENERATE_ARTICLE_STAR)
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_SUPER_JUMP_WORK_FLAG_STAR_LEFT)
            ArticleModule::generate_article(FIGHTER_DEDEDE_GENERATE_ARTICLE_STAR)
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialHiLandingR
    agent = "dedede", 
    script = "game_specialhilandingr", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_upb3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            HitModule::set_status_all(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            HIT_NODE(hash40("virtualwaist"), HIT_STATUS_OFF)
        }
        frame(Frame=3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=43.0, Angle=270, KBG=70, FKB=0, BKB=85, Size=16.0, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=43.0, Angle=60, KBG=71, FKB=0, BKB=88, Size=17.0, X=0.0, Y=8.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=43.0, Angle=60, KBG=71, FKB=0, BKB=88, Size=17.0, X=0.0, Y=8.0, Z=-10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=2)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_SUPER_JUMP_WORK_FLAG_STAR_RIGHT)
            ArticleModule::generate_article(FIGHTER_DEDEDE_GENERATE_ARTICLE_STAR)
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_SUPER_JUMP_WORK_FLAG_STAR_LEFT)
            ArticleModule::generate_article(FIGHTER_DEDEDE_GENERATE_ARTICLE_STAR)
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//Fly
    agent = "dedede_star", 
    script = "game_fly", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_star(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=66, KBG=84, FKB=0, BKB=30, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_NONE)
        }
    });
}


#[acmd_script(//SpecialAirHiTurnL
    agent = "dedede", 
    script = "game_specialairhiturnl", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_upbairl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            SET_SPEED_EX(0, 0.2, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
        }
        if(is_excute){
            HitModule::set_status_all(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            HIT_NODE(hash40("virtualwaist"), HIT_STATUS_OFF)
        }
        frame(Frame=20)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
    });
}

#[acmd_script(//SpecialAirHiTurnR
    agent = "dedede", 
    script = "game_specialairhiturnr", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_upbairr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            SET_SPEED_EX(0, 0.2, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
        }
        if(is_excute){
            HitModule::set_status_all(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            HIT_NODE(hash40("virtualwaist"), HIT_STATUS_OFF)
        }
        frame(Frame=20)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
    });
}

#[acmd_script(//SpecialLw
    agent = "dedede", 
    script = "game_speciallw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=20.0, Angle=361, KBG=93, FKB=0, BKB=30, Size=13.0, X=16.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=93, FKB=0, BKB=30, Size=9.0, X=0.0, Y=7.0, Z=2.0, X2=0.0, Y2=7.0, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=15)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_NORMAL, DamageThreshold=0)
        }
    });
}

#[acmd_script(//SpecialLwMax
    agent = "dedede", 
    script = "game_speciallwmax", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_downbmax(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=888.0, Angle=361, KBG=46, FKB=0, BKB=60, Size=12.0, X=16.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=888.0, Angle=361, KBG=46, FKB=0, BKB=60, Size=8.0, X=0.0, Y=7.0, Z=2.0, X2=0.0, Y2=7.0, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=15)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_NORMAL, DamageThreshold=0)
        }
    });
}

#[acmd_script(//SpecialAirLw
    agent = "dedede", 
    script = "game_specialairlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_downbair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=20.0, Angle=361, KBG=93, FKB=0, BKB=30, Size=13.0, X=16.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=93, FKB=0, BKB=30, Size=9.0, X=0.0, Y=7.0, Z=2.0, X2=0.0, Y2=7.0, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=15)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_NORMAL, DamageThreshold=0)
        }
    });
}

#[acmd_script(//SpecialAirLwMax
    agent = "dedede", 
    script = "game_specialairlwmax", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_downbairmax(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=888.0, Angle=361, KBG=46, FKB=0, BKB=60, Size=12.0, X=16.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=888.0, Angle=361, KBG=46, FKB=0, BKB=60, Size=8.0, X=0.0, Y=7.0, Z=2.0, X2=0.0, Y2=7.0, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=15)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_NORMAL, DamageThreshold=0)
        }
    });
}

#[acmd_script(//AppealHiR
    agent = "dedede", 
    script = "game_appealhir", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_uptauntr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ItemModule::have_item(ItemKind(*ITEM_KIND_MAXIMTOMATO), 0, 0, false, false)
        }
    });
}

#[acmd_script(//AppealHiL
    agent = "dedede", 
    script = "game_appealhil", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_uptauntl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ItemModule::have_item(ItemKind(*ITEM_KIND_MAXIMTOMATO), 0, 0, false, false)
        }
    });
}

#[acmd_script(//AppealSR
    agent = "dedede", 
    script = "game_appealsr", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_sidetauntr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=24.0, Angle=70, KBG=100, FKB=0, BKB=35, Size=11.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=4.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
        }
    });
}

#[acmd_script(//AppealSL
    agent = "dedede", 
    script = "game_appealsl", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_sidetauntl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=24.0, Angle=70, KBG=100, FKB=0, BKB=35, Size=11.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=4.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
        }
    });
}

#[acmd_script(//AppealLwR
    agent = "dedede", 
    script = "game_appeallwr", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_downtauntr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ItemModule::have_item(ItemKind(*ITEM_KIND_UNIRA), 0, 0, false, false)
        }
    });
}

#[acmd_script(//AppealLwL
    agent = "dedede", 
    script = "game_appeallwl", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_downtauntl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ItemModule::have_item(ItemKind(*ITEM_KIND_UNIRA), 0, 0, false, false)
        }
    });
}

#[acmd_script(//FinalEnd
    agent = "dedede", 
    script = "game_finalend", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_final(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            CAM_ZOOM_OUT()
            camera(MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_DEDEDE_FINAL, ID=0, Damage=900.0, Angle=65, KBG=135, FKB=0, BKB=80, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_ABS_SET)
        }
        frame(Frame=30)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_END_EXIT)
        }
    });
}

#[acmd_script(//FinalAirEnd
    agent = "dedede", 
    script = "game_finalair", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn dedede_finalairend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            CAM_ZOOM_OUT()
            camera(MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_DEDEDE_FINAL, ID=0, Damage=900.0, Angle=65, KBG=135, FKB=0, BKB=80, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_ABS_SET)
        }
        frame(Frame=30)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_END_EXIT)
        }
    });
}

pub fn install() {
    smashline::install_acmd_scripts!(
        dedede_jab1,
        dedede_jab2,
        dedede_jab100,
        dedede_jab100sub,
        dedede_jab100end,
        dedede_dashattack,
        dedede_sidetilt,
        dedede_uptilt,
        dedede_downtilt,
        dedede_sidesmash,
        dedede_upsmash,
        dedede_downsmash,
        dedede_nair,
        dedede_fair,
        dedede_bair,
        dedede_uair,
        dedede_dair,
        dedede_grab,
        dedede_pivotgrab,
        dedede_dashgrab,
        dedede_pummel,
        dedede_throwf,
        dedede_throwb,
        dedede_throwup,
        dedede_throwdown,
        dedede_cliffattack,
        dedede_slipattack,
        dedede_downattackd,
        dedede_downattacku,
        dedede_neutralb,
        dedede_neutralbloop,
        dedede_neutralbspit,
        dedede_neutralbswallow,
        dedede_neutralbair,
        dedede_neutralbairloop,
        dedede_neutralbairspit,
        dedede_neutralbairswallow,
        dedede_sideb1,
        dedede_sideb2,
        dedede_sideb3,
        dedede_sidebair1,
        dedede_sidebair2,
        dedede_sidebair3,
        dedede_gordo1,
        dedede_gordo2,
        dedede_gordo3,
        dedede_gordo4,
        dedede_upb1,
        dedede_upb2,
        dedede_upb3,
        dedede_upbairl,
        dedede_upbairr,
        dedede_star,
        dedede_downb,
        dedede_downbmax,
        dedede_downbair,
        dedede_downbairmax,
        dedede_uptauntr,
        dedede_uptauntl,
        dedede_sidetauntr,
        dedede_sidetauntl,
        dedede_downtauntr,
        dedede_downtauntl,
        dedede_final,
        dedede_finalair
    );
}
