use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;
use crate::utils::FIGHTER_CUTIN_MANAGER;

#[acmd_script(//Attack11
    agent = "littlemac", 
    script = "game_attack11", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_jab1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=25, FKB=0, BKB=25, Size=4.8, X=0.0, Y=10.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=25, FKB=0, BKB=25, Size=4.8, X=0.0, Y=10.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.5, Angle=180, KBG=20, FKB=0, BKB=25, Size=5.3, X=0.0, Y=10.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=20, FKB=0, BKB=25, Size=5.3, X=0.0, Y=10.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_MIDDLE), false)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        }
        frame(Frame=7)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART)
        }
    });        
}

#[acmd_script(//Attack12
    agent = "littlemac", 
    script = "game_attack12", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_jab2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=25, FKB=0, BKB=25, Size=5.5, X=0.0, Y=9.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=25, FKB=0, BKB=25, Size=5.5, X=0.0, Y=9.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=20, FKB=0, BKB=20, Size=6.3, X=0.0, Y=9.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=2, Frames=2.0, Unk=false)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
        }
        frame(Frame=9)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
        }
        FT_MOTION_RATE(FSM=0.6)
        frame(Frame=19)
        FT_MOTION_RATE(FSM=0.5)
    });        
}

#[acmd_script(//Attack13
    agent = "littlemac", 
    script = "game_attack13", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_jab3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        FT_MOTION_RATE(FSM=1.0)
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=60, KBG=100, FKB=0, BKB=50, Size=7.5, X=0.0, Y=8.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=9.0, Angle=60, KBG=100, FKB=0, BKB=50, Size=7.2, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });        
}

#[acmd_script(//Attack100
    agent = "littlemac", 
    script = "game_attack100", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_jab100(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        let lua_state = fighter.lua_state_agent;
        acmd!(lua_state, {
            frame(Frame=1)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=8, FKB=0, BKB=14, Size=11.0, X=0.0, Y=8.0, Z=10.5, X2=0.0, Y2=8.0, Z2=10.5, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            frame(Frame=3)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=8, FKB=0, BKB=14, Size=11.0, X=0.0, Y=8.0, Z=10.5, X2=0.0, Y2=8.0, Z2=10.5, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            frame(Frame=5)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=8, FKB=0, BKB=14, Size=11.0, X=0.0, Y=8.0, Z=10.5, X2=0.0, Y2=8.0, Z2=10.5, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            frame(Frame=7)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=8, FKB=0, BKB=14, Size=11.0, X=0.0, Y=8.0, Z=10.5, X2=0.0, Y2=8.0, Z2=10.5, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            frame(Frame=9)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=8, FKB=0, BKB=14, Size=11.0, X=0.0, Y=8.0, Z=10.5, X2=0.0, Y2=8.0, Z2=10.5, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            frame(Frame=11)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=8, FKB=0, BKB=14, Size=11.0, X=0.0, Y=8.0, Z=10.5, X2=0.0, Y2=8.0, Z2=10.5, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            frame(Frame=13)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=8, FKB=0, BKB=14, Size=11.0, X=0.0, Y=8.0, Z=10.5, X2=0.0, Y2=8.0, Z2=10.5, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            frame(Frame=15)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=8, FKB=0, BKB=14, Size=11.0, X=0.0, Y=8.0, Z=10.5, X2=0.0, Y2=8.0, Z2=10.5, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            frame(Frame=17)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=8, FKB=0, BKB=14, Size=11.0, X=0.0, Y=8.0, Z=10.5, X2=0.0, Y2=8.0, Z2=10.5, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            frame(Frame=19)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=8, FKB=0, BKB=14, Size=11.0, X=0.0, Y=8.0, Z=10.5, X2=0.0, Y2=8.0, Z2=10.5, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            wait_loop_clear(0)
        });  
    }      
}

#[acmd_script(//Attack100SubL
    agent = "littlemac", 
    script = "game_attack100subl", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_jab100subl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=8, FKB=0, BKB=14, Size=11.0, X=0.0, Y=8.0, Z=10.5, X2=0.0, Y2=8.0, Z2=10.5, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
    });        
}

#[acmd_script(//Attack100SubR
    agent = "littlemac", 
    script = "game_attack100subr", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_jab100subr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=8, FKB=0, BKB=14, Size=11.0, X=0.0, Y=8.0, Z=10.5, X2=0.0, Y2=8.0, Z2=10.5, Hitlag=0.5, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
    });        
}

#[acmd_script(//Attack100End
    agent = "littlemac", 
    script = "game_attack100end", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_jab100end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=75, KBG=131, FKB=0, BKB=90, Size=8.0, X=0.0, Y=14.5, Z=8.5, X2=0.0, Y2=4.0, Z2=8.5, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=75, KBG=131, FKB=0, BKB=90, Size=8.0, X=0.0, Y=14.5, Z=13.0, X2=0.0, Y2=4.0, Z2=17.0, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
    });        
}

#[acmd_script(//AttackDash
    agent = "littlemac", 
    script = "game_attackdash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_dashattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=100, FKB=20, BKB=0, Size=3.0, X=0.0, Y=6.0, Z=7.0, X2=0.0, Y2=6.0, Z2=3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_G_d, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
        }
        frame(Frame=5)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
            AttackModule::clear_all()
        }
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=17.0, Angle=60, KBG=75, FKB=0, BKB=90, Size=8.5, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=17.0, Angle=68, KBG=75, FKB=0, BKB=90, Size=7.5, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("shoulderl"), Damage=17.0, Angle=75, KBG=75, FKB=0, BKB=90, Size=7.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_HIGH), false)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_NORMAL, DamageThreshold=0)
        }
    });
}

#[acmd_script(//AttackS3
    agent = "littlemac", 
    script = "game_attacks3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_sidetilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=7.0, Angle=60, KBG=90, FKB=22, BKB=0, Size=7.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=7.0, Angle=40, KBG=90, FKB=37, BKB=0, Size=6.2, X=-1.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=7.0, Angle=36, KBG=90, FKB=42, BKB=0, Size=6.2, X=-5.8, Y=0.5, Z=0.0, X2=-3.8, Y2=0.0, Z2=0.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=3, Part=0, Bone=hash40("arml"), Damage=7.0, Angle=0, KBG=90, FKB=12, BKB=0, Size=7.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=4, Part=0, Bone=hash40("arml"), Damage=7.0, Angle=10, KBG=90, FKB=42, BKB=0, Size=6.2, X=-1.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=5, Part=0, Bone=hash40("arml"), Damage=7.0, Angle=10, KBG=90, FKB=60, BKB=0, Size=6.2, X=-5.8, Y=0.5, Z=0.0, X2=-3.8, Y2=0.0, Z2=0.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=4)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=14.0, Angle=361, KBG=130, FKB=0, BKB=40, Size=7.5, X=3.7, Y=-2.2, Z=-0.8, X2=3.7, Y2=2.2, Z2=0.8, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=14.0, Angle=361, KBG=130, FKB=0, BKB=40, Size=6.0, X=-3.8, Y=0.0, Z=0.0, X2=-1.0, Y2=0.0, Z2=0.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=4)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackHi3
    agent = "littlemac", 
    script = "game_attackhi3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_uptilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=14.7, Angle=96, KBG=127, FKB=0, BKB=35, Size=8.5, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=14.7, Angle=96, KBG=127, FKB=0, BKB=35, Size=7.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=14.7, Angle=96, KBG=127, FKB=0, BKB=35, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=2, Frames=2.0, Unk=false)
        }
        wait(Frames=7)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackLw3
    agent = "littlemac", 
    script = "game_attacklw3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_downtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK)
            JostleModule::set_status(false)
        }
        frame(Frame=3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=12.0, Angle=80, KBG=50, FKB=0, BKB=45, Size=8.5, X=4.4, Y=0.2, Z=0.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=12.0, Angle=80, KBG=50, FKB=0, BKB=45, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=12.0, Angle=80, KBG=50, FKB=0, BKB=45, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_LOW), false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=6)
        if(is_excute){
            JostleModule::set_status(true)
        }
    });
}

#[acmd_script(//AttackS4Hi
    agent = "littlemac", 
    script = "game_attacks4hi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_sidesmashup(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=14)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=25.0, Angle=78, KBG=97, FKB=0, BKB=29, Size=8.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=25.0, Angle=85, KBG=97, FKB=0, BKB=29, Size=7.0, X=-1.5, Y=0.0, Z=0.0, X2=-3.0, Y2=0.0, Z2=0.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackS4
    agent = "littlemac", 
    script = "game_attacks4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_sidesmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=14)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=24.0, Angle=361, KBG=96, FKB=0, BKB=29, Size=8.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=24.0, Angle=361, KBG=96, FKB=0, BKB=29, Size=7.0, X=-1.5, Y=0.0, Z=0.0, X2=-3.0, Y2=0.0, Z2=0.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackS4Lw
    agent = "littlemac", 
    script = "game_attacks4lw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_sidesmashdown(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=15)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=23.0, Angle=330, KBG=50, FKB=0, BKB=88, Size=8.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=7, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=23.0, Angle=330, KBG=50, FKB=0, BKB=88, Size=7.8, X=0.0, Y=7.5, Z=3.0, X2=0.0, Y2=8.5, Z2=0.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=7, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_LOW), false)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackHi4
    agent = "littlemac", 
    script = "game_attackhi4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_upsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=7)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=26.0, Angle=86, KBG=83, FKB=0, BKB=40, Size=7.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=25.0, Angle=85, KBG=96, FKB=0, BKB=25, Size=9.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=25.0, Angle=85, KBG=96, FKB=0, BKB=25, Size=7.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=25.0, Angle=85, KBG=96, FKB=0, BKB=25, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        frame(Frame=15)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackLw4
    agent = "littlemac", 
    script = "game_attacklw4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_downsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=7)
        if(is_excute){
            JostleModule::set_status(false)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=21.0, Angle=30, KBG=94, FKB=0, BKB=25, Size=8.5, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=21.0, Angle=30, KBG=94, FKB=0, BKB=25, Size=7.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=21.0, Angle=30, KBG=94, FKB=0, BKB=25, Size=7.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_LOW), false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=13)
        if(is_excute){
            JostleModule::set_status(true)
        }
        frame(Frame=15)
        if(is_excute){
            JostleModule::set_status(false)
        }
        frame(Frame=17)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=23.0, Angle=30, KBG=94, FKB=0, BKB=25, Size=8.5, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=23.0, Angle=30, KBG=94, FKB=0, BKB=25, Size=7.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=23.0, Angle=30, KBG=94, FKB=0, BKB=25, Size=7.5, X=1.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_LOW), false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=22)
        if(is_excute){
            JostleModule::set_status(true)
        }
    });
}

#[acmd_script(//AttackAirN
    agent = "littlemac", 
    script = "game_attackairn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            SET_SPEED_EX(0.0, 1.4, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=2)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=16.0, Angle=367, KBG=100, FKB=9, BKB=0, Size=8.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=16.0, Angle=88, KBG=80, FKB=0, BKB=4, Size=7.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=16.0, Angle=367, KBG=80, FKB=0, BKB=4, Size=7.0, X=1.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=16)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=39)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
    });
}

#[acmd_script(//AttackAirF 
    agent = "littlemac", 
    script = "game_attackairf", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=24.0, Angle=30, KBG=96, FKB=0, BKB=29, Size=4.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=24.0, Angle=30, KBG=96, FKB=0, BKB=29, Size=2.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=24.0, Angle=30, KBG=96, FKB=0, BKB=29, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=35)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(//AttackAirB
    agent = "littlemac", 
    script = "game_attackairb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=11)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=20.0, Angle=30, KBG=100, FKB=0, BKB=30, Size=8.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=20.0, Angle=30, KBG=100, FKB=0, BKB=30, Size=7.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=20.0, Angle=30, KBG=100, FKB=0, BKB=30, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=36)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}        

#[acmd_script(//AttackAirHi
    agent = "littlemac", 
    script = "game_attackairhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=18.0, Angle=80, KBG=100, FKB=0, BKB=25, Size=8.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=18.0, Angle=80, KBG=100, FKB=0, BKB=25, Size=8.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("shoulderl"), Damage=18.0, Angle=80, KBG=100, FKB=0, BKB=25, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=4)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=40)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=65)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
    });
}

#[acmd_script(//AttackAirLw
    agent = "littlemac", 
    script = "game_attackairlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=23.0, Angle=270, KBG=100, FKB=0, BKB=25, Size=7.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=23.0, Angle=270, KBG=100, FKB=0, BKB=25, Size=7.5, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=23.0, Angle=270, KBG=100, FKB=0, BKB=25, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=4)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=25)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(//Catch
    agent = "littlemac", 
    script = "game_catch", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_grab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=8)
        if(is_excute){
            GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=9)
        if(is_excute){
            CATCH(ID=0, Bone=hash40("top"), Size=5.9, X=0.0, Y=8.0, Z=7.0, X2=0.0, Y2=8.0, Z2=14.0, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
            CATCH(ID=1, Bone=hash40("top"), Size=4.45, X=0.0, Y=8.0, Z=5.55, X2=0.0, Y2=8.0, Z2=15.45, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
        }
        rust {
            macros::game_CaptureCutCommon(fighter);
        }
        wait(Frames=2)
        if(is_excute){
            sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
            GrabModule::set_rebound(CanCatchRebound=false)
        }
    });
}

#[acmd_script(//CatchDash
    agent = "littlemac", 
    script = "game_catchdash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_dashgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=12)
        if(is_excute){
            GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=13)
        if(is_excute){
            CATCH(ID=0, Bone=hash40("top"), Size=5.3, X=0.0, Y=8.0, Z=4.0, X2=0.0, Y2=8.0, Z2=15.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
            CATCH(ID=1, Bone=hash40("top"), Size=4.15, X=0.0, Y=8.0, Z=2.85, X2=0.0, Y2=8.0, Z2=16.85, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
        }
        rust {
            macros::game_CaptureCutCommon(fighter);
        }
        wait(Frames=2)
        if(is_excute){
            sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
            GrabModule::set_rebound(CanCatchRebound=false)
        }
    });
}

#[acmd_script(//CatchTurn
    agent = "littlemac", 
    script = "game_catchturn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_pivotgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=13)
        if(is_excute){
            GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=14)
        if(is_excute){
            CATCH(ID=0, Bone=hash40("top"), Size=5.9, X=0.0, Y=8.0, Z=-4.0, X2=0.0, Y2=8.0, Z2=-22.6, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
            CATCH(ID=1, Bone=hash40("top"), Size=4.45, X=0.0, Y=8.0, Z=-2.55, X2=0.0, Y2=8.0, Z2=-23.05, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
        }
        rust {
            macros::game_CaptureCutCommon(fighter);
        }
        wait(Frames=2)
        if(is_excute){
            sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
            GrabModule::set_rebound(CanCatchRebound=false)
        }
    });
}

#[acmd_script(//CatchAttack
    agent = "littlemac", 
    script = "game_catchattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_pummel(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=100, FKB=30, BKB=0, Size=6.2, X=0.0, Y=10.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_catch_only_all(true, false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//ThrowF
    agent = "littlemac", 
    script = "game_throwf", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_throwf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=8.0, Angle=45, KBG=115, FKB=0, BKB=52, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=15)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=80, KBG=100, FKB=0, BKB=60, Size=6.0, X=0.0, Y=10.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_catch_only_all(true, false)
            CHECK_FINISH_CAMERA(18, 3)
            rust{
                lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.4);
            }
        }
        frame(Frame=17)
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
    agent = "littlemac", 
    script = "game_throwb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=9.0, Angle=45, KBG=136, FKB=0, BKB=44, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=17)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=100, KBG=100, FKB=0, BKB=60, Size=4.0, X=0.0, Y=10.0, Z=-11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_catch_only_all(true, false)
            WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT)
            CHECK_FINISH_CAMERA(11, 8)
            rust{
                lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.4);
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
            REVERSE_LR()
            AttackModule::clear_all()
        }
    });
}  

#[acmd_script(//ThrowHi
    agent = "littlemac", 
    script = "game_throwhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_throwhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=10.0, Angle=85, KBG=118, FKB=0, BKB=75, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=80, KBG=100, FKB=0, BKB=60, Size=4.0, X=0.0, Y=14.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_catch_only_all(true, false)
            CHECK_FINISH_CAMERA(13, 11)
            rust{
                lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.2);
            }
        }
        frame(Frame=12)
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
    agent = "littlemac", 
    script = "game_throwlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_throwlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=14.0, Angle=87, KBG=78, FKB=0, BKB=50, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=14)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=80, KBG=100, FKB=0, BKB=60, Size=4.0, X=0.0, Y=7.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_catch_only_all(true, false)
            CHECK_FINISH_CAMERA(10, 1)
            rust{
                lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.2);
            }
        }
        frame(Frame=16)
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

#[acmd_script(//CliffAttack
    agent = "littlemac", 
    script = "game_cliffattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_cliffattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=23)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=45, KBG=50, FKB=0, BKB=90, Size=8.0, X=0.0, Y=4.0, Z=6.0, X2=0.0, Y2=4.0, Z2=-5.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SlipAttack
    agent = "littlemac", 
    script = "game_slipattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_slipattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=18)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=361, KBG=80, FKB=0, BKB=60, Size=7.5, X=0.0, Y=4.5, Z=10.0, X2=0.0, Y2=4.5, Z2=4.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=22)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=361, KBG=80, FKB=0, BKB=60, Size=7.5, X=0.0, Y=4.5, Z=-8.5, X2=0.0, Y2=4.5, Z2=-4.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//DownAttackD
    agent = "littlemac", 
    script = "game_downattackd", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_downattackd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=14)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=48, KBG=78, FKB=0, BKB=80, Size=9.0, X=0.0, Y=5.0, Z=15.0, X2=0.0, Y2=5.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=19)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=48, KBG=78, FKB=0, BKB=80, Size=9.0, X=0.0, Y=5.0, Z=-13.0, X2=0.0, Y2=5.0, Z2=-5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//DownAttackU
    agent = "littlemac", 
    script = "game_downattacku", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_downattacku(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=14)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=48, KBG=78, FKB=0, BKB=80, Size=9.0, X=0.0, Y=5.0, Z=15.0, X2=0.0, Y2=5.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=19)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=48, KBG=78, FKB=0, BKB=80, Size=9.0, X=0.0, Y=5.0, Z=-13.0, X2=0.0, Y2=5.0, Z2=-5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialNDash
    agent = "littlemac", 
    script = "game_specialndash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_neutralb1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=20.0, Angle=35, KBG=58, FKB=0, BKB=48, Size=7.5, X=4.0, Y=0.0, Z=0.0, X2=-11.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            WorkModule::set_float(20.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN)
            WorkModule::set_float(46.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=20.0, Angle=35, KBG=58, FKB=0, BKB=48, Size=7.5, X=4.0, Y=0.0, Z=0.0, X2=-11.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=20.0, Angle=35, KBG=58, FKB=0, BKB=48, Size=7.5, X=4.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            WorkModule::set_float(20.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN)
            WorkModule::set_float(46.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=20.0, Angle=35, KBG=70, FKB=0, BKB=48, Size=7.5, X=4.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=20.0, Angle=35, KBG=58, FKB=0, BKB=48, Size=7.5, X=3.3, Y=-0.3, Z=-1.4, X2=0.0, Y2=-0.6, Z2=-1.4, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            WorkModule::set_float(20.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN)
            WorkModule::set_float(46.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=20.0, Angle=35, KBG=58, FKB=0, BKB=48, Size=7.5, X=3.3, Y=-0.3, Z=-0.3, X2=0.0, Y2=-0.3, Z2=-0.3, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=18)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialNDashTurn
    agent = "littlemac", 
    script = "game_specialndashturn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_neutralb2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_TURN_DASH_START)
        }
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=18.0, Angle=35, KBG=56, FKB=0, BKB=48, Size=7.5, X=4.0, Y=0.0, Z=0.0, X2=-11.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            WorkModule::set_float(18.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN)
            WorkModule::set_float(45.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=18.0, Angle=35, KBG=56, FKB=0, BKB=48, Size=7.5, X=4.0, Y=0.0, Z=0.0, X2=-11.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=18.0, Angle=35, KBG=56, FKB=0, BKB=48, Size=7.5, X=4.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            WorkModule::set_float(18.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN)
            WorkModule::set_float(45.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=18.0, Angle=35, KBG=56, FKB=0, BKB=48, Size=7.5, X=4.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=14)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=18.0, Angle=35, KBG=70, FKB=0, BKB=50, Size=7.5, X=3.3, Y=-0.3, Z=-1.4, X2=0.0, Y2=-0.6, Z2=-1.4, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            WorkModule::set_float(18.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN)
            WorkModule::set_float(45.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=18.0, Angle=35, KBG=70, FKB=0, BKB=50, Size=7.5, X=3.3, Y=-0.3, Z=-0.3, X2=0.0, Y2=-0.3, Z2=-0.3, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=22)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialNMaxDash
    agent = "littlemac", 
    script = "game_specialnmaxdash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_neutralb3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=48.0, Angle=361, KBG=61, FKB=0, BKB=48, Size=7.5, X=3.6, Y=0.0, Z=0.0, X2=-6.0, Y2=0.0, Z2=0.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=48.0, Angle=361, KBG=61, FKB=0, BKB=48, Size=7.5, X=3.6, Y=0.0, Z=0.0, X2=-5.0, Y2=2.0, Z2=2.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=48.0, Angle=361, KBG=61, FKB=0, BKB=48, Size=7.5, X=3.6, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=48.0, Angle=361, KBG=61, FKB=0, BKB=48, Size=7.5, X=3.6, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=13)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=48.0, Angle=361, KBG=61, FKB=0, BKB=48, Size=7.0, X=3.6, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=48.0, Angle=361, KBG=61, FKB=0, BKB=48, Size=7.5, X=3.6, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        wait(Frames=26)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialNMaxDashTurn
    agent = "littlemac", 
    script = "game_specialnmaxdashturn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_neutralb4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_MAX_TURN_DASH_START)
        }
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=46.0, Angle=361, KBG=60, FKB=0, BKB=48, Size=7.5, X=3.6, Y=0.0, Z=0.0, X2=-6.0, Y2=0.0, Z2=0.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=46.0, Angle=361, KBG=60, FKB=0, BKB=48, Size=7.5, X=3.6, Y=0.0, Z=0.0, X2=-5.0, Y2=-1.0, Z2=-1.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=11)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=46.0, Angle=361, KBG=60, FKB=0, BKB=48, Size=7.5, X=3.6, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=46.0, Angle=361, KBG=60, FKB=0, BKB=48, Size=7.5, X=3.6, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=17)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=46.0, Angle=361, KBG=60, FKB=0, BKB=48, Size=7.0, X=3.6, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=46.0, Angle=361, KBG=60, FKB=0, BKB=48, Size=7.5, X=3.6, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        wait(Frames=26)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialN2
    agent = "littlemac", 
    script = "game_specialn2", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_neutralb5(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=9)
        if(is_excute){
            AttackModule::set_damage_shake_scale(0.67)
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=777.0, Angle=80, KBG=100, FKB=0, BKB=25, Size=8.5, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=777.0, Angle=80, KBG=100, FKB=0, BKB=25, Size=8.5, X=0.0, Y=9.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_invalid_xlu(0, true)
            AttackModule::set_invalid_xlu(1, true)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear(ID=1, false)
        }
        frame(Frame=10)
        if(is_excute){
            SA_SET(State=SITUATION_KIND_AIR)
        }
        frame(Frame=12)
        if(is_excute){
            AttackModule::clear_all()
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_NORMAL, DamageThreshold=0)
        }
        frame(Frame=30)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_RESET_KO_GAUGE)
        }
        frame(Frame=39)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_KO_GRAVITY)
        }
    });
}

#[acmd_script(//SpecialAirNDash
    agent = "littlemac", 
    script = "game_specialairndash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_neutralbair1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=16.0, Angle=35, KBG=75, FKB=0, BKB=25, Size=7.0, X=3.0, Y=0.0, Z=0.0, X2=-4.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            WorkModule::set_float(16.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN)
            WorkModule::set_float(42.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=16.0, Angle=35, KBG=75, FKB=0, BKB=25, Size=7.0, X=3.0, Y=0.0, Z=0.0, X2=-4.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=16.0, Angle=35, KBG=75, FKB=0, BKB=25, Size=7.0, X=3.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            WorkModule::set_float(16.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN)
            WorkModule::set_float(42.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=16.0, Angle=35, KBG=75, FKB=0, BKB=25, Size=7.0, X=3.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=11)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialAirNDashTurn
    agent = "littlemac", 
    script = "game_specialairndash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_neutralbair2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_TURN_DASH_START)
        }
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=16.0, Angle=35, KBG=71, FKB=0, BKB=25, Size=7.0, X=3.0, Y=0.0, Z=0.0, X2=-4.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            WorkModule::set_float(16.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN)
            WorkModule::set_float(41.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=16.0, Angle=35, KBG=71, FKB=0, BKB=25, Size=7.0, X=3.0, Y=0.0, Z=0.0, X2=-4.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=16.0, Angle=35, KBG=71, FKB=0, BKB=25, Size=7.0, X=3.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            WorkModule::set_float(16.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN)
            WorkModule::set_float(41.0, FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=16.0, Angle=35, KBG=71, FKB=0, BKB=25, Size=7.0, X=3.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=15)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialAirNMaxDash
    agent = "littlemac", 
    script = "game_specialairnmaxdash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_neutralbair3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=44.0, Angle=361, KBG=75, FKB=0, BKB=25, Size=8.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=44.0, Angle=361, KBG=75, FKB=0, BKB=25, Size=8.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        wait(Frames=11)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialAirNMaxDashTurn
    agent = "littlemac", 
    script = "game_specialairnmaxdashturn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_neutralbair4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_MAX_TURN_DASH_START)
        }
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=43.0, Angle=361, KBG=71, FKB=0, BKB=25, Size=8.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY) {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=43.0, Angle=361, KBG=71, FKB=0, BKB=25, Size=8.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        wait(Frames=11)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialAirN2
    agent = "littlemac", 
    script = "game_specialairn2", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_neutralbair5(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
        }
        frame(Frame=9)
        if(is_excute){
            AttackModule::set_damage_shake_scale(0.67)
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=777.0, Angle=80, KBG=100, FKB=0, BKB=25, Size=8.5, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=777.0, Angle=80, KBG=100, FKB=0, BKB=25, Size=8.5, X=0.0, Y=9.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_invalid_xlu(0, true)
            AttackModule::set_invalid_xlu(1, true)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=30)
       if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_RESET_KO_GAUGE)
        }
        frame(Frame=39)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_KO_GRAVITY)
        }
    });
}

#[acmd_script(//SpecialSJump
    agent = "littlemac", 
    script = "game_specialsjump", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=4)
        if(is_excute){
            HIT_NODE(hash40("hip"), HIT_STATUS_XLU)
            HIT_NODE(hash40("legl"), HIT_STATUS_XLU)
            HIT_NODE(hash40("legr"), HIT_STATUS_XLU)
            HIT_NODE(hash40("kneel"), HIT_STATUS_XLU)
            HIT_NODE(hash40("kneer"), HIT_STATUS_XLU)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=28.0, Angle=70, KBG=91, FKB=0, BKB=30, Size=8.0, X=0.0, Y=0.0, Z=10.0, X2=0.0, Y2=0.0, Z2=2.0, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_G_d, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
        }
        frame(Frame=18)
        if(is_excute){
            HitModule::set_status_all(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialAirSBlow
    agent = "littlemac", 
    script = "game_specialairsblow", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_sidebair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
        frame(Frame=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=30.0, Angle=265, KBG=80, FKB=0, BKB=75, Size=9.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("shoulderl"), Damage=30.0, Angle=275, KBG=80, FKB=0, BKB=75, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=30.0, Angle=270, KBG=80, FKB=0, BKB=75, Size=8.5, X=0.0, Y=4.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=30.0, Angle=260, KBG=80, FKB=0, BKB=75, Size=8.0, X=0.0, Y=17.0, Z=10.0, X2=0.0, Y2=10.0, Z2=11.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_HIGH), false)
        }
    });
}

#[acmd_script(//SpecialHi
    agent = "littlemac", 
    script = "game_specialhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_upb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            SA_SET(State=SITUATION_KIND_AIR)
        }
        frame(Frame=3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.8, Angle=367, KBG=50, FKB=0, BKB=37, Size=8.0, X=0.0, Y=26.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        frame(Frame=19)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=20)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
        frame(Frame=23)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=270, KBG=110, FKB=0, BKB=45, Size=12.0, X=0.0, Y=28.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=4.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        frame(Frame=26)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialLwHit
    agent = "littlemac", 
    script = "game_speciallwhit", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=15)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ATTACK_START)
        }
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=18.0, Angle=361, KBG=50, FKB=0, BKB=90, Size=8.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            rust {
                AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
            }
        }
        frame(Frame=19)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=18.0, Angle=361, KBG=50, FKB=0, BKB=90, Size=12.0, X=0.0, Y=9.0, Z=3.0, X2=0.0, Y2=9.0, Z2=9.0, Hitlag=2.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            rust {
                AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
                AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
            }
        }
        frame(Frame=22)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ATTACK_END)
        }
        frame(Frame=45)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
    });
}

#[acmd_script(//AppealHiR
    agent = "littlemac", 
    script = "game_appealhir", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_uptauntr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ItemModule::have_item(ItemKind(*ITEM_KIND_BUMPER), 0, 0, false, false)
        }
    });
}

#[acmd_script(//AppealHiL
    agent = "littlemac", 
    script = "game_appealhil", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_uptauntl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ItemModule::have_item(ItemKind(*ITEM_KIND_BUMPER), 0, 0, false, false)
        }
    });
}

#[acmd_script(//AppealSR
    agent = "littlemac", 
    script = "game_appealsr", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_sidetauntr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ItemModule::have_item(ItemKind(*ITEM_KIND_ROCKETBELT), 0, 0, false, false)
        }
    });
}

#[acmd_script(//AppealSL
    agent = "littlemac", 
    script = "game_appealsl", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_sidetauntl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ItemModule::have_item(ItemKind(*ITEM_KIND_ROCKETBELT), 0, 0, false, false)
        }
    });
}

#[acmd_script(//AppealLwR
    agent = "littlemac", 
    script = "game_appeallwr", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_downtauntr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ItemModule::have_item(ItemKind(*ITEM_KIND_KILLEREYE), 0, 0, false, false)
        }
    });
}

#[acmd_script(//AppealLwL
    agent = "littlemac", 
    script = "game_appeallwl", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_downtauntl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ItemModule::have_item(ItemKind(*ITEM_KIND_KILLEREYE), 0, 0, false, false)
        }
    });
}

#[acmd_script(//FinalAttack
    agent = "littlemac", 
    script = "game_finalattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_finalattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WHOLE_HIT(HIT_STATUS_XLU)
        }
        frame(Frame=1)
        if(is_excute){
            CAM_ZOOM_OUT()
        }
        WorkModule::is_flag(FIGHTER_LITTLEMAC_STATUS_FINAL_FLAG_HIT_ALL_OPPONENT)
        if(methodlib::L2CValue::operator==(lib::L2CValueconst&)const(false, true)){
            WorkModule::is_flag(FIGHTER_LITTLEMAC_STATUS_FINAL_FLAG_DEMON_DOJO_ZOOM)
            if(methodlib::L2CValue::operator==(lib::L2CValueconst&)const(false, true)){
                get_value_float(SO_VAR_FLOAT_LR)
                if(0x1462c0(0, 0)){
                    if(is_excute){
                        PostureModule::scale(300, 0)
                        0x1462c0(0, 2.5)
                        CAM_ZOOM_IN_FINAL_arg13(-15, 10, 0, -5, 70, true, FIGHTER_LITTLEMAC_STATUS_FINAL_WORK_INT_ATTACK_HIT_OBJECT_ID, 0, 20, 5)
                    }
                    else{
                        get_value_float(FIGHTER_LITTLEMAC_STATUS_FINAL_WORK_INT_ATTACK_HIT_OBJECT_ID)
                        if(0x1462c0(680, 0)){
                            if(is_excute){
                                PostureModule::scale(300, 0)
                                0x1462c0(0, 2.5)
                                CAM_ZOOM_IN_FINAL_arg13(-30, 10, -20, -5, 70, true, FIGHTER_LITTLEMAC_STATUS_FINAL_WORK_INT_ATTACK_HIT_OBJECT_ID, 0, 20, 5)
                            }
                            else{
                                if(is_excute){
                                    CAM_ZOOM_IN_FINAL_arg13(300, 0, 2.5, 15, 10, 0, -5, 70, true, FIGHTER_LITTLEMAC_STATUS_FINAL_WORK_INT_ATTACK_HIT_OBJECT_ID, 0, -15, 5)
                                }
                                else{
                                    if(is_excute){
                                        CAM_ZOOM_IN_FINAL_arg13(300, 0, 2.5, 30, 10, 20, -5, 70, true, FIGHTER_LITTLEMAC_STATUS_FINAL_WORK_INT_ATTACK_HIT_OBJECT_ID, 0, -15, 5)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        frame(Frame=2)
        for(47 Iterations){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=315, KBG=100, FKB=50, BKB=0, Size=10.0, X=0.0, Y=20.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=225, KBG=100, FKB=50, BKB=0, Size=10.0, X=0.0, Y=20.0, Z=30.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.0, Angle=45, KBG=100, FKB=50, BKB=0, Size=10.0, X=0.0, Y=0.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.0, Angle=135, KBG=100, FKB=50, BKB=0, Size=10.0, X=0.0, Y=0.0, Z=30.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
                AttackModule::set_no_dead_all(true, false)
                AttackModule::set_no_uniq_effect_all(true, false)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
            }
            wait(Frames=2)
        }
        frame(Frame=143)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=40, KBG=100, FKB=0, BKB=30, Size=20.0, X=0.0, Y=10.0, Z=20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
            AttackModule::set_no_dead_all(true, false)
            WorkModule::on_flag(Flag=FIGHTER_LITTLEMAC_STATUS_FINAL_FLAG_ATTACK_REQUEST_LOOP_DAMAGE_MOTION)
            WorkModule::set_int64(hash40("fall_damage"), FIGHTER_LITTLEMAC_STATUS_FINAL_WORK_INT_ATTACK_REQUEST_LOOP_DAMAGE_MOTION)
        }
        frame(Frame=144)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::off_flag(Flag=FIGHTER_LITTLEMAC_STATUS_FINAL_FLAG_ATTACK_REQUEST_LOOP_DAMAGE_MOTION)
        }
        frame(Frame=149)
        if(is_excute){
            CAM_ZOOM_OUT()
            WHOLE_HIT(HIT_STATUS_XLU)
            AttackModule::clear_all()
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=888.0, Angle=68, KBG=120, FKB=0, BKB=80, Size=40.0, X=0.0, Y=14.0, Z=18.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
            AttackModule::set_force_reaction(0, true, false)
            WorkModule::on_flag(Flag=FIGHTER_LITTLEMAC_STATUS_FINAL_FLAG_ATTACK_END_HIT)
            AttackModule::set_final_finish_cut_in(0, true, false, -1, false)
        }
        WorkModule::is_flag(FIGHTER_LITTLEMAC_STATUS_FINAL_FLAG_HIT_ALL_OPPONENT)
        if(methodlib::L2CValue::operator==(lib::L2CValueconst&)const(false, true)){
            if(is_excute){
                PostureModule::scale(2, 0)
                0x1462c0(-1186192792, 2.5)
                CAM_ZOOM_IN_arg5(0, 0)
            }
        }
        get_value_float(SO_VAR_FLOAT_LR)
        if(0x1462c0(1421420, 0)){
            if(is_excute){
                camera(MA_MSC_CMD_CAMERA_CAM_OFFSET, -10, 10)
            }
            else{
                if(is_excute){
                    camera(MA_MSC_CMD_CAMERA_CAM_OFFSET, 10, 10)
                }
            }
        }
        frame(Frame=151)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=176)
        if(is_excute){
            CAM_ZOOM_OUT()
        }
    });
} 

#[acmd_script(//FinalAirAttack
    agent = "littlemac", 
    script = "game_finalairattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn littlemac_finalairattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WHOLE_HIT(HIT_STATUS_XLU)
        }
        frame(Frame=1)
        if(is_excute){
            CAM_ZOOM_OUT()
        }
        WorkModule::is_flag(FIGHTER_LITTLEMAC_STATUS_FINAL_FLAG_HIT_ALL_OPPONENT)
        if(methodlib::L2CValue::operator==(lib::L2CValueconst&)const(false, true)){
            WorkModule::is_flag(FIGHTER_LITTLEMAC_STATUS_FINAL_FLAG_DEMON_DOJO_ZOOM)
            if(methodlib::L2CValue::operator==(lib::L2CValueconst&)const(false, true)){
                get_value_float(SO_VAR_FLOAT_LR)
                if(0x1462c0(0, 0)){
                    if(is_excute){
                        PostureModule::scale(300, 0)
                        0x1462c0(0, 2.5)
                        CAM_ZOOM_IN_FINAL_arg13(-15, 10, 0, -5, 70, true, FIGHTER_LITTLEMAC_STATUS_FINAL_WORK_INT_ATTACK_HIT_OBJECT_ID, 0, 20, 5)
                    }
                    else{
                        get_value_float(FIGHTER_LITTLEMAC_STATUS_FINAL_WORK_INT_ATTACK_HIT_OBJECT_ID)
                        if(0x1462c0(680, 0)){
                            if(is_excute){
                                PostureModule::scale(300, 0)
                                0x1462c0(0, 2.5)
                                CAM_ZOOM_IN_FINAL_arg13(-30, 10, -20, -5, 70, true, FIGHTER_LITTLEMAC_STATUS_FINAL_WORK_INT_ATTACK_HIT_OBJECT_ID, 0, 20, 5)
                            }
                            else{
                                if(is_excute){
                                    CAM_ZOOM_IN_FINAL_arg13(300, 0, 2.5, 15, 10, 0, -5, 70, true, FIGHTER_LITTLEMAC_STATUS_FINAL_WORK_INT_ATTACK_HIT_OBJECT_ID, 0, -15, 5)
                                }
                                else{
                                    if(is_excute){
                                        CAM_ZOOM_IN_FINAL_arg13(300, 0, 2.5, 30, 10, 20, -5, 70, true, FIGHTER_LITTLEMAC_STATUS_FINAL_WORK_INT_ATTACK_HIT_OBJECT_ID, 0, -15, 5)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        frame(Frame=2)
        for(47 Iterations){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=315, KBG=100, FKB=50, BKB=0, Size=10.0, X=0.0, Y=20.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=225, KBG=100, FKB=50, BKB=0, Size=10.0, X=0.0, Y=20.0, Z=30.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.0, Angle=45, KBG=100, FKB=50, BKB=0, Size=10.0, X=0.0, Y=0.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.0, Angle=135, KBG=100, FKB=50, BKB=0, Size=10.0, X=0.0, Y=0.0, Z=30.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
                AttackModule::set_no_dead_all(true, false)
                AttackModule::set_no_uniq_effect_all(true, false)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
            }
            wait(Frames=2)
        }
        frame(Frame=143)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=40, KBG=100, FKB=0, BKB=30, Size=20.0, X=0.0, Y=10.0, Z=20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
            AttackModule::set_no_dead_all(true, false)
            WorkModule::on_flag(Flag=FIGHTER_LITTLEMAC_STATUS_FINAL_FLAG_ATTACK_REQUEST_LOOP_DAMAGE_MOTION)
            WorkModule::set_int64(hash40("fall_damage"), FIGHTER_LITTLEMAC_STATUS_FINAL_WORK_INT_ATTACK_REQUEST_LOOP_DAMAGE_MOTION)
        }
        frame(Frame=144)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::off_flag(Flag=FIGHTER_LITTLEMAC_STATUS_FINAL_FLAG_ATTACK_REQUEST_LOOP_DAMAGE_MOTION)
        }
        frame(Frame=149)
        if(is_excute){
            CAM_ZOOM_OUT()
            WHOLE_HIT(HIT_STATUS_XLU)
            AttackModule::clear_all()
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=888.0, Angle=68, KBG=120, FKB=0, BKB=80, Size=40.0, X=0.0, Y=14.0, Z=18.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
            AttackModule::set_force_reaction(0, true, false)
            WorkModule::on_flag(Flag=FIGHTER_LITTLEMAC_STATUS_FINAL_FLAG_ATTACK_END_HIT)
            AttackModule::set_final_finish_cut_in(0, true, false, -1, false)
        }
        WorkModule::is_flag(FIGHTER_LITTLEMAC_STATUS_FINAL_FLAG_HIT_ALL_OPPONENT)
        if(methodlib::L2CValue::operator==(lib::L2CValueconst&)const(false, true)){
            if(is_excute){
                PostureModule::scale(2, 0)
                0x1462c0(-1186192792, 2.5)
                CAM_ZOOM_IN_arg5(0, 0)
            }
        }
        get_value_float(SO_VAR_FLOAT_LR)
        if(0x1462c0(1421420, 0)){
            if(is_excute){
                camera(MA_MSC_CMD_CAMERA_CAM_OFFSET, -10, 10)
            }
            else{
                if(is_excute){
                    camera(MA_MSC_CMD_CAMERA_CAM_OFFSET, 10, 10)
                }
            }
        }
        frame(Frame=151)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=176)
        if(is_excute){
            CAM_ZOOM_OUT()
        }
    });
} 

pub fn install() {
    smashline::install_acmd_scripts!(
        littlemac_jab1,
        littlemac_jab2,
        littlemac_jab3,
        littlemac_jab100,
        littlemac_jab100subl,
        littlemac_jab100subr,
        littlemac_jab100end,
        littlemac_dashattack,
        littlemac_sidetilt,
        littlemac_uptilt,
        littlemac_downtilt,
        littlemac_sidesmashup,
        littlemac_sidesmash,
        littlemac_sidesmashdown,
        littlemac_upsmash,
        littlemac_downsmash,
        littlemac_nair,
        littlemac_fair,
        littlemac_bair,
        littlemac_uair,
        littlemac_dair,
        littlemac_grab,
        littlemac_dashgrab,
        littlemac_pivotgrab,
        littlemac_pummel,
        littlemac_throwf,
        littlemac_throwb,
        littlemac_throwhi,
        littlemac_throwlw,
        littlemac_downattacku,
        littlemac_downattackd,
        littlemac_cliffattack,
        littlemac_slipattack,
        littlemac_neutralb1,
        littlemac_neutralb2,
        littlemac_neutralb3,
        littlemac_neutralb4,
        littlemac_neutralb5,
        littlemac_neutralbair1,
        littlemac_neutralbair2,
        littlemac_neutralbair3,
        littlemac_neutralbair4,
        littlemac_neutralbair5,
        littlemac_sideb,
        littlemac_sidebair,
        littlemac_upb,
        littlemac_downb,
        littlemac_sidetauntr,
        littlemac_sidetauntl,
        littlemac_uptauntr,
        littlemac_uptauntl,
        littlemac_downtauntr,
        littlemac_downtauntl,
        littlemac_finalattack,
        littlemac_finalairattack
    );
}
