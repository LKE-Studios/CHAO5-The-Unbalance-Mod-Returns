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

#[acmd_script(//Final
    agent = "ptrainer_pfushigisou", 
    script = "game_final", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn ptrainerfushigisou_final(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=30)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.2, Angle=367, KBG=100, FKB=45, BKB=5, Size=6.5, X=0.0, Y=10.0, Z=200.0, X2=0.0, Y2=10.0, Z2=200.0, Hitlag=0.9, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.2, Angle=10, KBG=100, FKB=45, BKB=5, Size=6.5, X=0.0, Y=10.0, Z=15.0, X2=0.0, Y2=10.0, Z2=200.0, Hitlag=0.9, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            AttackModule::set_final_finish_cut_in(0, true, true, -1, false)
            AttackModule::set_final_finish_cut_in(1, true, true, -1, false)
        }
        frame(Frame=200)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=201)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=361, KBG=128, FKB=0, BKB=80, Size=9.0, X=0.0, Y=10.0, Z=15.0, X2=0.0, Y2=10.0, Z2=200.0, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            AttackModule::set_force_reaction(0, true, false)
            AttackModule::set_final_finish_cut_in(0, true, true, -1, false)
        }
        frame(Frame=210)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//FinalAir
    agent = "ptrainer_pfushigisou", 
    script = "game_finalair", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn ptrainerfushigisou_finalair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=30)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.2, Angle=367, KBG=100, FKB=45, BKB=5, Size=6.5, X=0.0, Y=10.0, Z=200.0, X2=0.0, Y2=10.0, Z2=200.0, Hitlag=0.9, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.2, Angle=10, KBG=100, FKB=45, BKB=5, Size=6.5, X=0.0, Y=10.0, Z=15.0, X2=0.0, Y2=10.0, Z2=200.0, Hitlag=0.9, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            AttackModule::set_final_finish_cut_in(0, true, true, -1, false)
            AttackModule::set_final_finish_cut_in(1, true, true, -1, false)
        }
        frame(Frame=200)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=201)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=361, KBG=128, FKB=0, BKB=80, Size=9.0, X=0.0, Y=10.0, Z=15.0, X2=0.0, Y2=10.0, Z2=200.0, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            AttackModule::set_force_reaction(0, true, false)
            AttackModule::set_final_finish_cut_in(0, true, true, -1, false)
        }
        frame(Frame=210)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//FinalEnd
    agent = "ptrainer_pzenigame", 
    script = "game_final", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn ptrainerzenigame_final(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=30)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.5, Angle=30, KBG=100, FKB=20, BKB=0, Size=5.0, X=0.0, Y=0.0, Z=25.0, X2=0.0, Y2=-8.0, Z2=40.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.5, Angle=330, KBG=100, FKB=20, BKB=0, Size=5.0, X=0.0, Y=10.0, Z=25.0, X2=0.0, Y2=17.0, Z2=40.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_NONE)
            AttackModule::set_no_uniq_effect_all(true, false)
            AttackModule::set_final_finish_cut_in(0, true, true, -1, false)
            AttackModule::set_final_finish_cut_in(2, true, true, -1, false)
        }
        frame(Frame=36)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.5, Angle=30, KBG=100, FKB=20, BKB=0, Size=5.0, X=0.0, Y=0.0, Z=25.0, X2=0.0, Y2=-18.0, Z2=60.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.5, Angle=330, KBG=100, FKB=20, BKB=0, Size=5.0, X=0.0, Y=10.0, Z=25.0, X2=0.0, Y2=26.0, Z2=60.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_NONE)
            AttackModule::set_no_uniq_effect_all(true, false)
            AttackModule::set_final_finish_cut_in(0, true, true, -1, false)
            AttackModule::set_final_finish_cut_in(2, true, true, -1, false)
        }
        frame(Frame=40)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=110, KBG=100, FKB=80, BKB=0, Size=10.0, X=0.0, Y=-19.0, Z=63.0, X2=0.0, Y2=-22.0, Z2=70.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=20, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_NONE)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=10.0, Angle=250, KBG=100, FKB=80, BKB=0, Size=10.0, X=0.0, Y=28.0, Z=63.0, X2=0.0, Y2=31.0, Z2=70.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=20, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_NONE)
            AttackModule::set_no_uniq_effect_all(true, false)
            AttackModule::set_final_finish_cut_in(1, true, true, -1, false)
            AttackModule::set_final_finish_cut_in(3, true, true, -1, false)
        }
        frame(Frame=46)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=110, KBG=100, FKB=80, BKB=0, Size=10.0, X=0.0, Y=-19.0, Z=63.0, X2=0.0, Y2=-30.0, Z2=90.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=20, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_NONE)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=10.0, Angle=250, KBG=100, FKB=80, BKB=0, Size=10.0, X=0.0, Y=28.0, Z=63.0, X2=0.0, Y2=40.0, Z2=90.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=20, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_NONE)
            AttackModule::set_no_uniq_effect_all(true, false)
            AttackModule::set_final_finish_cut_in(1, true, true, -1, false)
            AttackModule::set_final_finish_cut_in(3, true, true, -1, false)
        }
        frame(Frame=210)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(//FinalAir
    agent = "pzenigame_ptrainer", 
    script = "game_finalair", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn ptrainerzenigame_finalair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=30)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.5, Angle=30, KBG=100, FKB=20, BKB=0, Size=5.0, X=0.0, Y=0.0, Z=25.0, X2=0.0, Y2=-8.0, Z2=40.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.5, Angle=330, KBG=100, FKB=20, BKB=0, Size=5.0, X=0.0, Y=10.0, Z=25.0, X2=0.0, Y2=17.0, Z2=40.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_NONE)
            AttackModule::set_no_uniq_effect_all(true, false)
            AttackModule::set_final_finish_cut_in(0, true, true, -1, false)
            AttackModule::set_final_finish_cut_in(2, true, true, -1, false)
        }
        frame(Frame=36)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.5, Angle=30, KBG=100, FKB=20, BKB=0, Size=5.0, X=0.0, Y=0.0, Z=25.0, X2=0.0, Y2=-18.0, Z2=60.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.5, Angle=330, KBG=100, FKB=20, BKB=0, Size=5.0, X=0.0, Y=10.0, Z=25.0, X2=0.0, Y2=26.0, Z2=60.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_NONE)
            AttackModule::set_no_uniq_effect_all(true, false)
            AttackModule::set_final_finish_cut_in(0, true, true, -1, false)
            AttackModule::set_final_finish_cut_in(2, true, true, -1, false)
        }
        frame(Frame=40)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=110, KBG=100, FKB=80, BKB=0, Size=10.0, X=0.0, Y=-19.0, Z=63.0, X2=0.0, Y2=-22.0, Z2=70.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=20, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_NONE)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=10.0, Angle=250, KBG=100, FKB=80, BKB=0, Size=10.0, X=0.0, Y=28.0, Z=63.0, X2=0.0, Y2=31.0, Z2=70.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=20, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_NONE)
            AttackModule::set_no_uniq_effect_all(true, false)
            AttackModule::set_final_finish_cut_in(1, true, true, -1, false)
            AttackModule::set_final_finish_cut_in(3, true, true, -1, false)
        }
        frame(Frame=46)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=110, KBG=100, FKB=80, BKB=0, Size=10.0, X=0.0, Y=-19.0, Z=63.0, X2=0.0, Y2=-30.0, Z2=90.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=20, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_NONE)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=10.0, Angle=250, KBG=100, FKB=80, BKB=0, Size=10.0, X=0.0, Y=28.0, Z=63.0, X2=0.0, Y2=40.0, Z2=90.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=20, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_NONE)
            AttackModule::set_no_uniq_effect_all(true, false)
            AttackModule::set_final_finish_cut_in(1, true, true, -1, false)
            AttackModule::set_final_finish_cut_in(3, true, true, -1, false)
        }
        frame(Frame=210)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

pub fn install() {
    smashline::install_acmd_scripts!(
        ptrainerfushigisou_final,
        ptrainerfushigisou_finalair,
        ptrainerzenigame_final,
        ptrainerzenigame_finalair
    );
}
