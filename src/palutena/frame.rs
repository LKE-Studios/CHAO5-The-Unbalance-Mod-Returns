use crate::imports::BuildImports::*;

static mut GODDESS_POWER_UP : [bool; 8] = [false; 8];
static mut GFX_COUNTER : [i32; 8] = [0; 8];
static POWER_MUL : f32 = 1.1;

#[fighter_frame( agent = FIGHTER_KIND_PALUTENA )]
fn frame_palutena(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if [
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_GLIDE_LANDING,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_ESCAPE_AIR,
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_MISS_FOOT,
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_CLIFF_CATCH
        ].contains(&status_kind) { 
            STOP_SE(fighter, Hash40::new("se_palutena_glide_loop"));
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        };
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_START {
            KineticModule::clear_speed_all(fighter.module_accessor);
            SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            if MotionModule::frame(fighter.module_accessor) >= 0.0 && MotionModule::frame(fighter.module_accessor) < 1.0 {  
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_start"), false, -1.0);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_ATTACK {
            if MotionModule::frame(fighter.module_accessor) >= 0.0 && MotionModule::frame(fighter.module_accessor) < 1.0 {  
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_attack"), false, -1.0);
            }
            STOP_SE(fighter, Hash40::new("se_palutena_glide_loop"));
            if MotionModule::frame(fighter.module_accessor) >= 26.0 && MotionModule::frame(fighter.module_accessor) < 27.0 {  
                ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_END {
            if MotionModule::frame(fighter.module_accessor) >= 0.0 && MotionModule::frame(fighter.module_accessor) < 1.0 {  
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("glide_end"), false, -1.0);
            }
            STOP_SE(fighter, Hash40::new("se_palutena_glide_loop"));
        }
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE_LANDING {
            if MotionModule::frame(fighter.module_accessor) >= 25.0 && MotionModule::frame(fighter.module_accessor) < 26.0 {  
                ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }
        if GODDESS_POWER_UP[ENTRY_ID] == true {
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 0.7);
            DamageModule::set_reaction_mul(fighter.module_accessor, 0.7);
            AttackModule::set_power_up(fighter.module_accessor, POWER_MUL);
            if GFX_COUNTER[ENTRY_ID] >= 6 {
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_aura_light"), Hash40::new("waist"), &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 0.15, true, 0, 0, 0, 0, 0, true, true);
                LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 2.55, /*B*/ 0.48);
                GFX_COUNTER[ENTRY_ID] = 0;
            };
        };
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            GODDESS_POWER_UP[ENTRY_ID] = true;
        }
        if status_kind == *FIGHTER_STATUS_KIND_DEAD || status_kind == *FIGHTER_STATUS_KIND_MISS_FOOT || 
        sv_information::is_ready_go() == false {
            GODDESS_POWER_UP[ENTRY_ID] = false;
            AttackModule::set_power_up(fighter.module_accessor, 1.0);
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_palutena
    );
}