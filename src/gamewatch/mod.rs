use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script(//Attack100 
    agent = "gamewatch", 
    script = "game_attack100", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_jab100(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        let lua_state = fighter.lua_state_agent;
        acmd!(lua_state, {
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.8, Angle=361, KBG=20, FKB=0, BKB=12, Size=5.5, X=0.0, Y=5.0, Z=14.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.8, Angle=361, KBG=20, FKB=0, BKB=12, Size=4.0, X=0.0, Y=5.0, Z=8.0, X2=0.0, Y2=5.0, Z2=5.0, Hitlag=0.7, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
                AttackModule::set_add_reaction_frame(ID=0, Frames=4.0, Unk=false)
                AttackModule::set_add_reaction_frame(ID=1, Frames=4.0, Unk=false)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=10)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=10)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            if(is_excute){
                ArticleModule::change_motion(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_100"), false, -1.0)

            }
            frame(Frame=5)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.4, Angle=361, KBG=20, FKB=0, BKB=12, Size=9.5, X=0.0, Y=5.0, Z=14.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.4, Angle=361, KBG=20, FKB=0, BKB=12, Size=9.0, X=0.0, Y=5.0, Z=8.0, X2=0.0, Y2=5.0, Z2=5.0, Hitlag=0.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
                AttackModule::set_add_reaction_frame(ID=0, Frames=4.0, Unk=false)
                AttackModule::set_add_reaction_frame(ID=1, Frames=4.0, Unk=false)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=10)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=10)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            frame(Frame=13)
            wait_loop_clear(0)
        });
    }
}

#[acmd_script(//Attack100Sub
    agent = "gamewatch", 
    script = "game_attack100sub", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_jab100sub(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.4, Angle=361, KBG=20, FKB=0, BKB=12, Size=9.5, X=0.0, Y=5.0, Z=14.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.4, Angle=361, KBG=20, FKB=0, BKB=12, Size=9.0, X=0.0, Y=5.0, Z=8.0, X2=0.0, Y2=5.0, Z2=5.0, Hitlag=0.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
            AttackModule::set_add_reaction_frame(ID=0, Frames=4.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=1, Frames=4.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=10)
            ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=10)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
    });
}

#[acmd_script(//Attack100End
    agent = "gamewatch", 
    script = "game_attack100end", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_jab100end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ArticleModule::change_motion(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_100_end"), false, -1.0)
        }
        frame(Frame=2)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=50, KBG=160, FKB=0, BKB=66, Size=10.0, X=0.0, Y=9.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_poison"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=50, KBG=160, FKB=0, BKB=66, Size=9.0, X=0.0, Y=1.0, Z=0.0, X2=0.0, Y2=-2.0, Z2=0.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_poison"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackDash
    agent = "gamewatch", 
    script = "game_attackdash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_dashattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        FT_MOTION_RATE(FSM=0.45)
        if(is_excute){
            FighterAreaModuleImpl::enable_fix_jostle_area(9.0, 4.0)
        }
        frame(Frame=6)
        if(is_excute){
            FT_MOTION_RATE(FSM=1.0)
            JostleModule::set_status(false)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=25, KBG=94, FKB=0, BKB=70, Size=10.0, X=0.0, Y=3.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.9, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.5, Angle=1, KBG=89, FKB=0, BKB=70, Size=8.5, X=0.0, Y=3.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.9, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
        }
        frame(Frame=20)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=39)
        if(is_excute){
            JostleModule::set_status(true)
        }
    });
}

#[acmd_script(//AttackS3
    agent = "gamewatch", 
    script = "game_attacks3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_sidetilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.75)
        frame(Frame=8)
        if(is_excute){
            WorkModule::set_int(WEAPON_GAMEWATCH_NORMAL_WEAPON_KIND_CHAIR, FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND)
            ArticleModule::generate_article(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, -1)
            ArticleModule::change_motion(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_s3"), false, -1.0)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=30, KBG=145, FKB=0, BKB=35, Size=8.0, X=0.0, Y=4.5, Z=15.0, X2=0.0, Y2=6.5, Z2=17.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.0, Angle=30, KBG=145, FKB=0, BKB=35, Size=7.5, X=0.0, Y=4.0, Z=7.0, X2=0.0, Y2=4.0, Z2=12.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=10)
        if(is_excute){
            FT_MOTION_RATE(FSM=1.0)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=30, KBG=135, FKB=0, BKB=35, Size=8.3, X=0.0, Y=4.5, Z=15.0, X2=0.0, Y2=6.5, Z2=15.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=30, KBG=135, FKB=0, BKB=35, Size=7.8, X=0.0, Y=4.0, Z=7.0, X2=0.0, Y2=4.0, Z2=12.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=12.0, Angle=30, KBG=120, FKB=0, BKB=35, Size=7.3, X=0.0, Y=4.5, Z=15.0, X2=0.0, Y2=6.5, Z2=18.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=12.0, Angle=30, KBG=120, FKB=0, BKB=35, Size=6.8, X=0.0, Y=4.0, Z=7.0, X2=0.0, Y2=4.0, Z2=12.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=21)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=25)
        if(is_excute){
            ArticleModule::remove(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
    });
}

#[acmd_script(//AttackHi3
    agent = "gamewatch", 
    script = "game_attackhi3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_uptilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            HIT_NODE(hash40("handr"), HIT_STATUS_XLU)
            HIT_NODE(hash40("handl"), HIT_STATUS_XLU)
        }
        frame(Frame=10)
        if(is_excute){
            HIT_NODE(hash40("arml"), HIT_STATUS_XLU)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.0, Angle=170, KBG=100, FKB=60, BKB=0, Size=9.5, X=0.0, Y=14.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_flower"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=170, KBG=100, FKB=60, BKB=0, Size=8.0, X=0.0, Y=10.6, Z=5.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_flower"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=13.0, Angle=145, KBG=100, FKB=80, BKB=0, Size=9.5, X=0.0, Y=18.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_flower"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=13.0, Angle=145, KBG=100, FKB=80, BKB=0, Size=8.0, X=0.0, Y=10.6, Z=5.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_flower"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=13)
        if(is_excute){
            HitModule::set_status_all(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            AttackModule::clear_all()
        }
        frame(Frame=20)
        if(is_excute){
            HIT_NODE(hash40("armr"), HIT_STATUS_XLU)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=14.0, Angle=75, KBG=120, FKB=0, BKB=50, Size=9.5, X=0.0, Y=18.0, Z=-8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_flower"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=75, KBG=120, FKB=0, BKB=50, Size=8.0, X=0.0, Y=10.6, Z=-5.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_flower"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=23)
        if(is_excute){
            HitModule::set_status_all(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            AttackModule::clear_all()
        }
        frame(Frame=24)
        if(is_excute){
            VisibilityModule::set_default_int64(hash40("lhand") as i64)
        }
    });
}

#[acmd_script(//AttackLw3
    agent = "gamewatch", 
    script = "game_attacklw3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_downtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WorkModule::set_int(WEAPON_GAMEWATCH_NORMAL_WEAPON_KIND_MANHOLE, FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND)
            ArticleModule::generate_article(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, -1)
            ArticleModule::change_motion(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_lw3"), false, -1.0)
        }
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=88, KBG=130, FKB=0, BKB=65, Size=8.2, X=0.0, Y=2.1, Z=6.7, X2=0.0, Y2=2.1, Z2=17.8, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.0, Angle=89, KBG=130, FKB=0, BKB=65, Size=8.2, X=0.0, Y=9.3, Z=6.7, X2=0.0, Y2=2.1, Z2=17.8, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=11.5, Angle=90, KBG=128, FKB=0, BKB=65, Size=17.4, X=0.0, Y=15.8, Z=0.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_none"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_LOW), false)
        }
        frame(Frame=9)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=25)
        if(is_excute){
            ArticleModule::remove(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
    });
}

#[acmd_script(//AttackS4
    agent = "gamewatch", 
    script = "game_attacks4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_sidesmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=8)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=17)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.5, Angle=48, KBG=100, FKB=0, BKB=45, Size=6.6, X=0.0, Y=5.3, Z=7.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=21.5, Angle=48, KBG=97, FKB=0, BKB=46, Size=6.9, X=0.0, Y=6.8, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=23.0, Angle=48, KBG=97, FKB=0, BKB=46, Size=9.3, X=0.0, Y=7.8, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackHi4
    agent = "gamewatch", 
    script = "game_attackhi4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_upsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=21)
        if(is_excute){
            WHOLE_HIT(HIT_STATUS_INVINCIBLE)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=21.0, Angle=83, KBG=98, FKB=0, BKB=40, Size=10.5, X=0.0, Y=9.5, Z=-6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_HEAD)
        }
        frame(Frame=22)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=21.0, Angle=83, KBG=98, FKB=0, BKB=40, Size=10.5, X=0.0, Y=9.5, Z=-6.0, X2=0.0, Y2=8.0, Z2=5.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_HEAD)
        }
        wait(Frames=8)
        if(is_excute){
            HitModule::set_status_all(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackHi4R
    agent = "gamewatch", 
    script = "game_attackhi4r", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_upsmashr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=21)
        if(is_excute){
            WHOLE_HIT(HIT_STATUS_INVINCIBLE)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=21.0, Angle=83, KBG=98, FKB=0, BKB=40, Size=10.5, X=0.0, Y=9.5, Z=-6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_HEAD)
        }
        frame(Frame=22)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=21.0, Angle=83, KBG=98, FKB=0, BKB=40, Size=10.5, X=0.0, Y=9.5, Z=-6.0, X2=0.0, Y2=8.0, Z2=5.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_HEAD)
        }
        wait(Frames=8)
        if(is_excute){
            HitModule::set_status_all(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackLw4
    agent = "gamewatch", 
    script = "game_attacklw4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_downsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.785)
        frame(Frame=8)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=15)
        FT_MOTION_RATE(FSM=1.0)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=20, KBG=80, FKB=0, BKB=30, Size=6.5, X=0.0, Y=3.6, Z=-5.0, X2=0.0, Y2=3.6, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=18.0, Angle=270, KBG=88, FKB=0, BKB=35, Size=10.5, X=0.0, Y=3.6, Z=-14.2, X2=0.0, Y2=3.6, Z2=-12.2, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G_d, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_bury"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=18.0, Angle=280, KBG=88, FKB=0, BKB=35, Size=10.5, X=0.0, Y=3.6, Z=14.2, X2=0.0, Y2=3.6, Z2=12.2, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G_d, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_bury"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=18.0, Angle=270, KBG=85, FKB=0, BKB=60, Size=10.5, X=0.0, Y=3.6, Z=-14.2, X2=0.0, Y2=3.6, Z2=-12.2, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_noamal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=18.0, Angle=280, KBG=85, FKB=0, BKB=60, Size=10.5, X=0.0, Y=3.6, Z=14.2, X2=0.0, Y2=3.6, Z2=12.2, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_noamal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=5)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AttackAirN
    agent = "gamewatch", 
    script = "game_attackairn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        FT_MOTION_RATE(FSM=0.5)
        if(is_excute){
            WorkModule::set_int(WEAPON_GAMEWATCH_NORMAL_WEAPON_KIND_TROPICALFISH, FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND)
            ArticleModule::generate_article(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, -1)
            ArticleModule::change_motion(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_air_n"), false, -1.0)
        }
        frame(Frame=3)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=10.0, Angle=165, KBG=100, FKB=30, BKB=0, Size=9.0, X=0.0, Y=19.4, Z=3.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=10.0, Angle=165, KBG=100, FKB=30, BKB=0, Size=9.0, X=0.0, Y=18.6, Z=-2.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=367, KBG=100, FKB=30, BKB=0, Size=8.5, X=0.0, Y=13.0, Z=-1.0, X2=0.0, Y2=13.0, Z2=6.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=105, KBG=100, FKB=45, BKB=0, Size=8.5, X=0.0, Y=10.0, Z=-1.0, X2=0.0, Y2=10.0, Z2=6.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=4)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=11.0, Angle=160, KBG=100, FKB=30, BKB=0, Size=9.0, X=0.0, Y=20.8, Z=10.8, X2=0.0, Y2=21.0, Z2=9.6, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=11.0, Angle=150, KBG=100, FKB=30, BKB=0, Size=9.0, X=0.0, Y=17.9, Z=-8.8, X2=0.0, Y2=18.3, Z2=-7.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=367, KBG=100, FKB=20, BKB=0, Size=9.0, X=0.0, Y=13.0, Z=-1.0, X2=0.0, Y2=13.0, Z2=6.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=11.0, Angle=105, KBG=100, FKB=20, BKB=0, Size=9.0, X=0.0, Y=10.0, Z=-1.0, X2=0.0, Y2=10.0, Z2=6.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=4)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=17)
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=8.0, Angle=145, KBG=100, FKB=30, BKB=0, Size=9.5, X=0.0, Y=12.4, Z=13.3, X2=0.0, Y2=13.6, Z2=13.1, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=140, KBG=100, FKB=30, BKB=0, Size=9.5, X=0.0, Y=11.0, Z=-9.9, X2=0.0, Y2=12.0, Z2=-9.7, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=120, KBG=100, FKB=15, BKB=0, Size=9.0, X=0.0, Y=13.0, Z=-1.0, X2=0.0, Y2=13.0, Z2=6.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=120, KBG=100, FKB=15, BKB=0, Size=9.0, X=0.0, Y=10.0, Z=-1.0, X2=0.0, Y2=10.0, Z2=6.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=4)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=22)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=70, KBG=150, FKB=0, BKB=40, Size=9.0, X=0.0, Y=2.9, Z=13.5, X2=0.0, Y2=1.5, Z2=13.5, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=16.0, Angle=70, KBG=150, FKB=0, BKB=40, Size=9.0, X=0.0, Y=2.4, Z=-10.2, X2=0.0, Y2=1.0, Z2=-10.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=16.0, Angle=70, KBG=150, FKB=0, BKB=40, Size=12.5, X=0.0, Y=12.0, Z=1.8, X2=0.0, Y2=12.2, Z2=3.4, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=6)
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
    agent = "gamewatch", 
    script = "game_attackairf", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            FT_MOTION_RATE(FSM=0.5)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            ArticleModule::generate_article(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BOMB, false, -1)
        }
        frame(Frame=10)
        if(is_excute){
            FT_MOTION_RATE(FSM=1.0)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=50, FKB=30, BKB=0, Size=6.0, X=0.0, Y=9.0, Z=11.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
            AttackModule::set_add_reaction_frame(ID=0, Frames=5.0, Unk=false)
        }
        frame(Frame=14)
        if(is_excute){
            AttackModule::clear_all()
            ArticleModule::shoot(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BOMB, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false)
        }
        frame(Frame=19)
        if(is_excute){
            FT_MOTION_RATE(FSM=0.75)
            CancelModule::enable_cancel()
        }
        frame(Frame=42)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(//Bomb
    agent = "gamewatch_bomb", 
    script = "game_burst", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_bomb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            VisibilityModule::set_whole(false)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=25.0, Angle=45, KBG=119, FKB=0, BKB=50, Size=22.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BOMB, Type=ATTACK_REGION_OBJECT)
            ControlModule::set_rumble(Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
            AREA_WIND_2ND_RAD(0, 1, 0.02, 1000, 1, 0, 0, 25)
        }
        wait(Frames=2)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x199c462b5d)
        }
    });
}

#[acmd_script(//AttackAirB
    agent = "gamewatch", 
    script = "game_attackairb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WorkModule::set_int(WEAPON_GAMEWATCH_NORMAL_WEAPON_KIND_TURTLE, FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND)
            ArticleModule::generate_article(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, -1)
            ArticleModule::change_motion(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_air_b"), false, -1.0)
        }
        frame(Frame=10)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        for(3 Iterations){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=367, KBG=100, FKB=0, BKB=0, Size=8.2, X=0.0, Y=3.5, Z=-14.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=367, KBG=100, FKB=0, BKB=0, Size=7.3, X=0.0, Y=1.8, Z=-8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            }
            wait(Frames=4)
            if(is_excute){
                AttackModule::clear_all()
            }
        }
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=185, FKB=0, BKB=35, Size=10.0, X=0.0, Y=3.5, Z=-14.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_poison"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=185, FKB=0, BKB=35, Size=9.5, X=0.0, Y=1.8, Z=-8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_poison"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=38)
        if(is_excute){
            ArticleModule::remove_exist(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
        frame(Frame=38)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(//AttackAirHi
    agent = "gamewatch_breath", 
    script = "game_attackairhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.8, Angle=97, KBG=87, FKB=63, BKB=0, Size=10.8, X=0.0, Y=2.4, Z=0.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=6, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
        }
        wait(Frames=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.8, Angle=97, KBG=87, FKB=55, BKB=0, Size=10.8, X=0.0, Y=2.4, Z=0.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=6, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
        }
        wait(Frames=14)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.8, Angle=97, KBG=87, FKB=40, BKB=0, Size=10.8, X=0.0, Y=2.4, Z=0.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=6, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
        }
        wait(Frames=7)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=80, KBG=100, FKB=0, BKB=65, Size=16.3, X=0.0, Y=1.8, Z=0.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=85, KBG=100, FKB=0, BKB=65, Size=20.0, X=0.0, Y=15.8, Z=0.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_none"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
        }
    });
}

#[acmd_script(//AttackAirLw
    agent = "gamewatch", 
    script = "game_attackairlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
            SET_SPEED_EX(0, 1, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
            WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
        }
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=12)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
            SET_SPEED_EX(0, -3.75, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
            WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=22.0, Angle=270, KBG=106, FKB=0, BKB=20, Size=15.0, X=0.0, Y=-6.5, Z=0.8, X2=0.0, Y2=-6.5, Z2=0.8, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=2)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=275, KBG=117, FKB=0, BKB=22, Size=15.0, X=0.0, Y=-8.0, Z=-0.6, X2=0.0, Y2=-8.0, Z2=2.2, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=48)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=50)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(//LandingAirB
    agent = "gamewatch", 
    script = "game_landingairb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_bairland(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        rust{
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON);
        }
        if(sv_animcmd::IS_EXIST_ARTICLE(fighter.lua_state_agent)){
            if(is_excute){
                ArticleModule::change_motion(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("landing_air_b"), false, -1.0)
            }
        }
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=58, KBG=77, FKB=0, BKB=95, Size=10.0, X=0.0, Y=4.0, Z=-16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=58, KBG=77, FKB=0, BKB=95, Size=10.0, X=0.0, Y=4.0, Z=-8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//LandingAirLw
    agent = "gamewatch", 
    script = "game_landingairlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_dairland(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            SET_SPEED_EX(0, 0, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.5, Angle=65, KBG=100, FKB=0, BKB=60, Size=15.5, X=0.0, Y=2.0, Z=5.0, X2=0.0, Y2=2.0, Z2=-2.2, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=4)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//CatchAttack
    agent = "gamewatch", 
    script = "game_catchattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_pummel(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        FT_MOTION_RATE(FSM=0.6)
        frame(Frame=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=100, FKB=30, BKB=0, Size=8.0, X=0.0, Y=6.6, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_flower"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
            AttackModule::set_catch_only_all(true, false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//Catch
    agent = "gamewatch", 
    script = "game_catch", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_grab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=6)
        if(is_excute){
            CATCH(ID=0, Bone=hash40("top"), Size=6.2, X=0.0, Y=5.0, Z=4.0, X2=0.0, Y2=5.0, Z2=12.3, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
            CATCH(ID=1, Bone=hash40("top"), Size=3.6, X=0.0, Y=5.0, Z=2.4, X2=0.0, Y2=5.0, Z2=15.9, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
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
    agent = "gamewatch", 
    script = "game_catchdash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_dashgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=8)
        if(is_excute){
            GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=9)
        if(is_excute){
            CATCH(ID=0, Bone=hash40("top"), Size=6.6, X=0.0, Y=5.0, Z=4.0, X2=0.0, Y2=5.0, Z2=17.5, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
            CATCH(ID=1, Bone=hash40("top"), Size=3.3, X=0.0, Y=5.0, Z=2.7, X2=0.0, Y2=5.0, Z2=20.8, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
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
    agent = "gamewatch", 
    script = "game_catchturn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_pivotgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=9)
        if(is_excute){
            GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=10)
        if(is_excute){
            CATCH(ID=0, Bone=hash40("top"), Size=6.2, X=0.0, Y=5.0, Z=-4.0, X2=0.0, Y2=5.0, Z2=-20.6, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
            CATCH(ID=1, Bone=hash40("top"), Size=3.6, X=0.0, Y=5.0, Z=-2.4, X2=0.0, Y2=5.0, Z2=-22.2, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
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

#[acmd_script(//ThrowF
    agent = "gamewatch", 
    script = "game_throwf", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_throwf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=12.0, Angle=52, KBG=65, FKB=0, BKB=100, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_flower"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_THROW_FLAG_SET_STOCK_ICON)
            WorkModule::set_float(50.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=5)
        if(is_excute){
            WorkModule::set_float(-110.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=9)
        if(is_excute){
            WorkModule::set_float(120.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=13)
        if(is_excute){
            WorkModule::set_float(-30.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=17)
        if(is_excute){
            WorkModule::set_float(135.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=22)
        if(is_excute){
            WorkModule::set_float(-70.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=25)
        if(is_excute){
            CHECK_FINISH_CAMERA(14, 6)
        }
        frame(Frame=26)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_THROW_FLAG_SET_STOCK_ICON)
            rust {
                let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
                let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
                let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
                macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
            }
        }
    });
}

#[acmd_script(//ThrowB
    agent = "gamewatch", 
    script = "game_throwb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=14.0, Angle=270, KBG=82, FKB=0, BKB=100, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_THROW_FLAG_SET_STOCK_ICON)
            WorkModule::set_float(-50.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=5)
        if(is_excute){
            WorkModule::set_float(110.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=9)
        if(is_excute){
            WorkModule::set_float(-120.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=13)
        if(is_excute){
            WorkModule::set_float(30.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=17)
        if(is_excute){
            REVERSE_LR()
            WorkModule::set_float(135.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=21)
        if(is_excute){
            WorkModule::set_float(-70.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=25)
        if(is_excute){
            CHECK_FINISH_CAMERA(12, 3)
        }
        frame(Frame=26)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_THROW_FLAG_SET_STOCK_ICON)
            rust {
                let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
                let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
                let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
                macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
            }
        }
    });
}

#[acmd_script(//ThrowHi
    agent = "gamewatch", 
    script = "game_throwhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_throwup(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=12.0, Angle=90, KBG=29, FKB=0, BKB=100, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_THROW_FLAG_SET_STOCK_ICON)
            WorkModule::set_float(-50.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=5)
        if(is_excute){
            WorkModule::set_float(110.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=9)
        if(is_excute){
            WorkModule::set_float(-120.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=13)
        if(is_excute){
            WorkModule::set_float(30.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=17)
        if(is_excute){
            WorkModule::set_float(-135.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=21)
        if(is_excute){
            WorkModule::set_float(70.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=25)
        if(is_excute){
            CHECK_FINISH_CAMERA(1, 15)
        }
        frame(Frame=26)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_THROW_FLAG_SET_STOCK_ICON)
            rust {
                let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
                let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
                let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
                macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
            }
        }
    });
}      

#[acmd_script(//ThrowLw
    agent = "gamewatch", 
    script = "game_throwlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_throwdown(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            FT_LEAVE_NEAR_OTTOTTO(-2.5, 2.5)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=11.0, Angle=80, KBG=138, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_THROW_FLAG_SET_STOCK_ICON)
            WorkModule::set_float(-50.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=5)
        if(is_excute){
            WorkModule::set_float(110.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=9)
        if(is_excute){
            WorkModule::set_float(-120.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=13)
        if(is_excute){
            WorkModule::set_float(30.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=17)
        if(is_excute){
            WorkModule::set_float(135.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=21)
        if(is_excute){
            WorkModule::set_float(70.0, FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE)
        }
        frame(Frame=34)
        if(is_excute){
            CHECK_FINISH_CAMERA(-9, 0)
        }
        frame(Frame=35)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_THROW_FLAG_SET_STOCK_ICON)
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
    agent = "gamewatch", 
    script = "game_cliffattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_cliffattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=24)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=32.0, Angle=45, KBG=35, FKB=0, BKB=150, Size=9.0, X=0.0, Y=5.0, Z=13.0, X2=0.0, Y2=5.0, Z2=3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SlipAttack
    agent = "gamewatch", 
    script = "game_slipattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_slipattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=19)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=361, KBG=50, FKB=0, BKB=60, Size=8.5, X=0.0, Y=4.5, Z=10.5, X2=0.0, Y2=4.5, Z2=4.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=29)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=361, KBG=50, FKB=0, BKB=60, Size=8.5, X=0.0, Y=4.5, Z=-10.5, X2=0.0, Y2=4.5, Z2=-4.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//DownAttackD
    agent = "gamewatch", 
    script = "game_downattackd", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_downattackd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=48, KBG=48, FKB=0, BKB=80, Size=9.0, X=0.0, Y=6.0, Z=12.5, X2=0.0, Y2=6.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=21)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=48, KBG=48, FKB=0, BKB=80, Size=9.0, X=0.0, Y=6.0, Z=-12.5, X2=0.0, Y2=6.0, Z2=-5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//DownAttackU
    agent = "gamewatch", 
    script = "game_downattacku", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_downattacku(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=48, KBG=48, FKB=0, BKB=80, Size=9.0, X=0.0, Y=6.0, Z=12.5, X2=0.0, Y2=6.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=21)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=48, KBG=48, FKB=0, BKB=80, Size=9.0, X=0.0, Y=6.0, Z=-12.5, X2=0.0, Y2=6.0, Z2=-5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}


#[acmd_script(//SpecialN
    agent = "gamewatch", 
    script = "game_specialn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_neutralb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        FT_MOTION_RATE(FSM=0.75)
        frame(Frame=18)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_SHOOT)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=65, KBG=99, FKB=0, BKB=45, Size=8.2, X=0.0, Y=8.5, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=16.0, Angle=65, KBG=99, FKB=0, BKB=45, Size=7.2, X=0.0, Y=6.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=21)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_RAPID_CHECK)
        }
        frame(Frame=24)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_COUNT_ENABLE)
        }
        frame(Frame=36)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_RAPID_CHECK)
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_LOOP_CHECK)
        }
        frame(Frame=48)
        FT_MOTION_RATE(FSM=1.0)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_COUNT_CHECK)
        }
    });
}

#[acmd_script(//Fish
    agent = "gamewatch_food", 
    script = "game_fish", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_fish(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=270, KBG=100, FKB=0, BKB=30, Size=5.4, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-1, Trip=0.1, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
        }
    });
}

#[acmd_script(//FriedPrawns
    agent = "gamewatch_food", 
    script = "game_friedprawns", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_prawns(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=280, KBG=100, FKB=0, BKB=30, Size=5.4, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-1, Trip=0.1, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
        }
    });
}

#[acmd_script(//Sausage
    agent = "gamewatch_food", 
    script = "game_sausage", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_sausage(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=275, KBG=100, FKB=0, BKB=30, Size=7.4, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-1, Trip=0.1, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
        }
    });
}

#[acmd_script(//Steak
    agent = "gamewatch_food", 
    script = "game_steak", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_steak(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=270, KBG=100, FKB=0, BKB=30, Size=10.4, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-1, Trip=0.1, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
        }
    });
}

#[acmd_script(//SpecialAirN
    agent = "gamewatch", 
    script = "game_specialairn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_neutralbair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        FT_MOTION_RATE(FSM=0.75)
        frame(Frame=18)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_SHOOT)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=65, KBG=99, FKB=0, BKB=45, Size=8.2, X=0.0, Y=8.5, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=16.0, Angle=65, KBG=99, FKB=0, BKB=45, Size=7.2, X=0.0, Y=6.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=21)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_RAPID_CHECK)
        }
        frame(Frame=24)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_COUNT_ENABLE)
        }
        frame(Frame=36)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_RAPID_CHECK)
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_LOOP_CHECK)
        }
        frame(Frame=48)
        FT_MOTION_RATE(FSM=1.0)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_N_FLAG_COUNT_CHECK)
        }
    });
}

#[acmd_script(//SpecialS2_Sub
    agent = "gamewatch", 
    script = "game_specials2_sub", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_sideb2sub(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=999.0, Angle=361, KBG=80, FKB=0, BKB=100, Size=8.0, X=0.0, Y=10.6, Z=8.9, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=999.0, Angle=361, KBG=80, FKB=0, BKB=100, Size=7.0, X=0.0, Y=7.0, Z=5.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
        }
    });
}

#[acmd_script(//SpecialS2
    agent = "gamewatch", 
    script = "game_specials2", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_sideb2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL)
	        VisibilityModule::set_default_int64(hash40("panel") as i64)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=999.0, Angle=361, KBG=80, FKB=0, BKB=100, Size=8.0, X=0.0, Y=10.6, Z=8.9, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=999.0, Angle=361, KBG=80, FKB=0, BKB=100, Size=7.0, X=0.0, Y=7.0, Z=5.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=24)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialAirS2
    agent = "gamewatch", 
    script = "game_specialairs2", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_sidebair2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL)
	        VisibilityModule::set_default_int64(hash40("panel") as i64)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=999.0, Angle=361, KBG=80, FKB=0, BKB=100, Size=8.0, X=0.0, Y=10.6, Z=8.9, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=999.0, Angle=361, KBG=80, FKB=0, BKB=100, Size=7.0, X=0.0, Y=7.0, Z=5.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=24)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialS2SFX
    agent = "gamewatch", 
    script = "sound_specials2", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn gamewatch_sideb2sfx(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            PLAY_SE(hash40("se_gamewatch_special_s01"))
        }
    });
}

#[acmd_script(//SpecialAirS2SFX
    agent = "gamewatch", 
    script = "sound_specialairs2", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn gamewatch_sidebair2sfx(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            PLAY_SE(hash40("se_gamewatch_special_s01"))
        }
    });
}

#[acmd_script(//SpecialS2GFX
    agent = "gamewatch", 
    script = "effect_specials2", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn gamewatch_sideb2gfx(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_action_smoke_h"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
        }
        for(12 Iterations){
            wait(Frames=2)
            if(is_excute){
                EFFECT(hash40("sys_damage_elec"), hash40("rot"), 0, 2, 0, 0, 0, 0, 1.2, 16, 20, 16, 0, 0, 0, false)
            }
        }
    });
}

#[acmd_script(//SpecialAirS2GFX
    agent = "gamewatch", 
    script = "effect_specialairs2", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn gamewatch_sidebair2gfx(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        for(12 Iterations){
            wait(Frames=2)
            if(is_excute){
                EFFECT(hash40("sys_damage_elec"), hash40("rot"), 0, 2, 0, 0, 0, 0, 1.2, 16, 20, 16, 0, 0, 0, false)
            }
        }
    });
}

#[acmd_script(//SpecialS7SFX
    agent = "gamewatch", 
    script = "sound_specials7", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn gamewatch_sideb7sfx(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            PLAY_SE(hash40("se_gamewatch_special_s01"))
        }
    });
}

#[acmd_script(//SpecialAirS7
    agent = "gamewatch", 
    script = "sound_specialairs7", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn gamewatch_sidebair7sfx(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            PLAY_SE(hash40("se_gamewatch_special_s01"))
        }
    });
}

#[acmd_script(//SpecialS7GFX
    agent = "gamewatch", 
    script = "effect_specials7", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn gamewatch_sideb7gfx(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_action_smoke_h"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
        }
        for(12 Iterations){
            wait(Frames=2)
            if(is_excute){
                EFFECT(hash40("sys_damage_elec"), hash40("rot"), 0, 2, 0, 0, 0, 0, 1.2, 16, 20, 16, 0, 0, 0, false)
            }
        }
    });
}

#[acmd_script(//SpecialAirS7GFX
    agent = "gamewatch", 
    script = "effect_specialairs7", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn gamewatch_sidebair7gfx(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        for(12 Iterations){
            wait(Frames=2)
            if(is_excute){
                EFFECT(hash40("sys_damage_elec"), hash40("rot"), 0, 2, 0, 0, 0, 0, 1.2, 16, 20, 16, 0, 0, 0, false)
            }
        }
    });
}

#[acmd_script(//SpecialS7_Sub
    agent = "gamewatch", 
    script = "game_specials7_sub", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_sideb7sub(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=999.0, Angle=45, KBG=80, FKB=0, BKB=120, Size=9.0, X=0.0, Y=10.6, Z=8.9, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=999.0, Angle=45, KBG=80, FKB=0, BKB=120, Size=9.0, X=0.0, Y=7.0, Z=5.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
        }
    });
}

#[acmd_script(//SpecialS7
    agent = "gamewatch", 
    script = "game_specials7", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_sideb7(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL)
	        VisibilityModule::set_default_int64(hash40("panel") as i64)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=999.0, Angle=361, KBG=80, FKB=0, BKB=100, Size=8.0, X=0.0, Y=10.6, Z=8.9, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=999.0, Angle=361, KBG=80, FKB=0, BKB=100, Size=7.0, X=0.0, Y=7.0, Z=5.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=24)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialAirS7
    agent = "gamewatch", 
    script = "game_specialairs7", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_sidebair7(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL)
	        VisibilityModule::set_default_int64(hash40("panel") as i64)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=999.0, Angle=361, KBG=80, FKB=0, BKB=100, Size=8.0, X=0.0, Y=10.6, Z=8.9, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=999.0, Angle=361, KBG=80, FKB=0, BKB=100, Size=7.0, X=0.0, Y=7.0, Z=5.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=24)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialS9_Sub
    agent = "gamewatch", 
    script = "game_specials9_sub", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_sideb9sub(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=999.0, Angle=361, KBG=80, FKB=0, BKB=100, Size=8.0, X=0.0, Y=10.6, Z=8.9, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=999.0, Angle=361, KBG=80, FKB=0, BKB=100, Size=7.0, X=0.0, Y=7.0, Z=5.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
        }
    });
}

#[acmd_script(//SpecialS9
    agent = "gamewatch", 
    script = "game_specials9", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_sideb9(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL)
	        VisibilityModule::set_default_int64(hash40("panel") as i64)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=999.0, Angle=361, KBG=80, FKB=0, BKB=100, Size=8.0, X=0.0, Y=10.6, Z=8.9, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=999.0, Angle=361, KBG=80, FKB=0, BKB=100, Size=7.0, X=0.0, Y=7.0, Z=5.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
            DamageModule::heal(-999.0, 0);
        }
        frame(Frame=24)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialAirS9
    agent = "gamewatch", 
    script = "game_specialairs9", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_sidebair9(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL)
	        VisibilityModule::set_default_int64(hash40("panel") as i64)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=999.0, Angle=361, KBG=80, FKB=0, BKB=100, Size=8.0, X=0.0, Y=10.6, Z=8.9, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=999.0, Angle=361, KBG=80, FKB=0, BKB=100, Size=7.0, X=0.0, Y=7.0, Z=5.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_OBJECT)
            DamageModule::heal(-999.0, 0);
        }
        frame(Frame=24)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialHi
    agent = "gamewatch", 
    script = "game_specialhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_upb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=275, KBG=68, FKB=0, BKB=102, Size=8.0, X=0.0, Y=6.0, Z=7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.0, Angle=275, KBG=68, FKB=0, BKB=102, Size=8.0, X=0.0, Y=6.0, Z=-7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
            AttackModule::set_add_reaction_frame(ID=0, Frames=10.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=1, Frames=10.0, Unk=false)
        }
        frame(Frame=5)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("waist"), Damage=16.0, Angle=270, KBG=80, FKB=0, BKB=60, Size=10.5, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_DECIDE_DIRECTION)
        }
        frame(Frame=10)
        FT_MOTION_RATE(FSM=0.8)
        frame(Frame=11)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE)
        }
        frame(Frame=14)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS)
        }
        frame(Frame=30)
        FT_MOTION_RATE(FSM=1.0)
        frame(Frame=33)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_PARACHUTE_OPEN)
        }
    });
}

#[acmd_script(//SpecialAirHi
    agent = "gamewatch", 
    script = "game_specialairhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_upbair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=275, KBG=68, FKB=0, BKB=102, Size=8.0, X=0.0, Y=6.0, Z=7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.0, Angle=275, KBG=68, FKB=0, BKB=102, Size=8.0, X=0.0, Y=6.0, Z=-7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
            AttackModule::set_add_reaction_frame(ID=0, Frames=10.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=1, Frames=10.0, Unk=false)
        }
        frame(Frame=5)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("waist"), Damage=16.0, Angle=270, KBG=80, FKB=0, BKB=60, Size=10.5, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_DECIDE_DIRECTION)
        }
        frame(Frame=10)
        FT_MOTION_RATE(FSM=0.8)
        frame(Frame=11)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE)
        }
        frame(Frame=14)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS)
        }
        frame(Frame=30)
        FT_MOTION_RATE(FSM=1.0)
        frame(Frame=33)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_PARACHUTE_OPEN)
        }
    });
}

#[acmd_script(//SpecialLw
    agent = "gamewatch", 
    script = "game_speciallw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_ABSORB_ENABLE)
            rust {
                let energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut smash::app::KineticEnergy;
                smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            }
        }
    });
}

#[acmd_script(//SpecialLwEnd
    agent = "gamewatch", 
    script = "game_speciallwend", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_downbend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            rust {
                let energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut smash::app::KineticEnergy;
                smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            }
        }
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_LOOP)
        }
    });
}

#[acmd_script(//SpecialLwShoot
    agent = "gamewatch", 
    script = "game_speciallwshoot", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_downbshoot(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            rust {
                let energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut smash::app::KineticEnergy;
                smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            }
            ArticleModule::set_visibility_whole(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_OIL, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
        frame(Frame=2)
        if(is_excute){
            ArticleModule::set_visibility_whole(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_OIL, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=80, FKB=0, BKB=25, Size=6.4, X=0.0, Y=7.7, Z=11.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            AttackModule::set_special_paint(0, SpecialPaintKind{_address: *SPECIAL_PAINT_OIL as u8})
        }
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=78, FKB=0, BKB=25, Size=9.0, X=0.0, Y=9.5, Z=20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            AttackModule::set_special_paint(0, SpecialPaintKind{_address: *SPECIAL_PAINT_OIL as u8})
        }
        frame(Frame=15)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=76, FKB=0, BKB=25, Size=16.0, X=0.0, Y=7.0, Z=32.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            AttackModule::set_special_paint(0, SpecialPaintKind{_address: *SPECIAL_PAINT_OIL as u8})
        }
        frame(Frame=27)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//SpecialAirLw
    agent = "gamewatch", 
    script = "game_specialairlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_downbair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_ABSORB_ENABLE)
            rust {
                let energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut smash::app::KineticEnergy;
                smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            }
        }
    });
}

#[acmd_script(//SpecialAirLwEnd
    agent = "gamewatch", 
    script = "game_specialairlwend", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_downbairend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            rust {
                let energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut smash::app::KineticEnergy;
                smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            }
        }
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GAMEWATCH_STATUS_SPECIAL_LW_FLAG_LOOP)
        }
    });
}

#[acmd_script(//SpecialAirLwShoot
    agent = "gamewatch", 
    script = "game_speciallwshoot", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_downbairshoot(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            rust {
                let energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut smash::app::KineticEnergy;
                smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            }
            ArticleModule::set_visibility_whole(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_OIL, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
        frame(Frame=2)
        if(is_excute){
            ArticleModule::set_visibility_whole(FIGHTER_GAMEWATCH_GENERATE_ARTICLE_OIL, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=80, FKB=0, BKB=25, Size=6.4, X=0.0, Y=7.7, Z=11.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            AttackModule::set_special_paint(0, SpecialPaintKind{_address: *SPECIAL_PAINT_OIL as u8})
        }
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=78, FKB=0, BKB=25, Size=9.0, X=0.0, Y=9.5, Z=20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            AttackModule::set_special_paint(0, SpecialPaintKind{_address: *SPECIAL_PAINT_OIL as u8})
        }
        frame(Frame=15)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=76, FKB=0, BKB=25, Size=16.0, X=0.0, Y=7.0, Z=32.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            AttackModule::set_special_paint(0, SpecialPaintKind{_address: *SPECIAL_PAINT_OIL as u8})
        }
        frame(Frame=27)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//AppealHiR
    agent = "gamewatch", 
    script = "game_appealhir", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_uptauntr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ItemModule::have_item(ItemKind(*ITEM_KIND_BOOMERANG), 0, 0, false, false)
        }
    });
}

#[acmd_script(//AppealHiL
    agent = "gamewatch", 
    script = "game_appealhil", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_uptauntl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ItemModule::have_item(ItemKind(*ITEM_KIND_BUMPER), 0, 0, false, false)
        }
    });
}

#[acmd_script(//AppealSR
    agent = "gamewatch", 
    script = "game_appealsr", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_sidetauntr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ItemModule::have_item(ItemKind(*ITEM_KIND_TABEMONO), 0, 0, false, false)
        }
    });
}

#[acmd_script(//AppealSL
    agent = "gamewatch", 
    script = "game_appealsl", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_sidetauntl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ItemModule::have_item(ItemKind(*ITEM_KIND_POWBLOCK), 0, 0, false, false)
        }
    });
}

#[acmd_script(//AppealLwR
    agent = "gamewatch", 
    script = "game_appeallwr", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_downtauntr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ItemModule::have_item(ItemKind(*ITEM_KIND_BOMBHEI), 0, 0, false, false)
        }
    });
}

#[acmd_script(//AppealLwL
    agent = "gamewatch", 
    script = "game_appeallwl", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn gamewatch_downtauntl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ItemModule::have_item(ItemKind(*ITEM_KIND_SANDBAG), 0, 0, false, false)
        }
    });
}

pub fn install() {
    smashline::install_acmd_scripts!(
        gamewatch_jab100,
        gamewatch_jab100sub,
        gamewatch_jab100end,
        gamewatch_dashattack,
        gamewatch_sidetilt,
        gamewatch_uptilt,
        gamewatch_downtilt,
        gamewatch_sidesmash,
        gamewatch_upsmash,
        gamewatch_upsmashr,
        gamewatch_downsmash,
        gamewatch_nair,
        gamewatch_fair,
        gamewatch_bomb,
        gamewatch_bair,
        gamewatch_uair,
        gamewatch_dair,
        gamewatch_bairland,
        gamewatch_dairland,
        gamewatch_pummel,
        gamewatch_grab,
        gamewatch_dashgrab,
        gamewatch_pivotgrab,
        gamewatch_throwf,
        gamewatch_throwb,
        gamewatch_throwup,
        gamewatch_throwdown,
        gamewatch_cliffattack,
        gamewatch_slipattack,
        gamewatch_downattackd,
        gamewatch_downattacku,
        gamewatch_neutralb,
        gamewatch_neutralbair,
        gamewatch_fish,
        gamewatch_prawns,
        gamewatch_sausage,
        gamewatch_steak,
        gamewatch_sideb2sub,
        gamewatch_sideb2,
        gamewatch_sidebair2,
        gamewatch_sideb2sfx,
        gamewatch_sidebair2sfx,
        gamewatch_sideb2gfx,
        gamewatch_sidebair2gfx,
        gamewatch_sideb7sub,
        gamewatch_sideb7,
        gamewatch_sidebair7,
        gamewatch_sideb7sfx,
        gamewatch_sidebair7sfx,
        gamewatch_sideb7gfx,
        gamewatch_sidebair7gfx,
        gamewatch_sideb9sub,
        gamewatch_sideb9,
        gamewatch_sidebair9,
        gamewatch_upb,
        gamewatch_upbair,
        gamewatch_downb,
        gamewatch_downbend,
        gamewatch_downbshoot,
        gamewatch_downbair,
        gamewatch_downbairend,
        gamewatch_downbairshoot,
        gamewatch_uptauntr,
        gamewatch_uptauntl,
        gamewatch_sidetauntr,
        gamewatch_sidetauntl,
        gamewatch_downtauntr,
        gamewatch_downtauntl
    );
}