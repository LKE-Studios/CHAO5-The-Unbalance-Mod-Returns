use crate::imports::BuildImports::*;

static mut DEFENCE_BOOST : [bool; 8] = [false; 8];
static mut GFX_COUNTER : [i32; 8] = [0; 8];

#[fighter_frame( agent = FIGHTER_KIND_MIIFIGHTER )]
fn frame_miifighter(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if DEFENCE_BOOST[ENTRY_ID] == true {
            GFX_COUNTER[ENTRY_ID] += 1;
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 0.75);
            DamageModule::set_reaction_mul(fighter.module_accessor, 0.75);
            if GFX_COUNTER[ENTRY_ID] >= 20 {
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_aura_light"), Hash40::new("waist"), &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 5.0, true, 0, 0, 0, 0, 0, true, true);
                LAST_EFFECT_SET_COLOR(fighter, /*R*/ 5.3, /*G*/ 0.13, /*B*/ 0.1);
                GFX_COUNTER[ENTRY_ID] = 0;
            };
        };
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            DEFENCE_BOOST[ENTRY_ID] = true;
        }
        if status_kind == *FIGHTER_STATUS_KIND_DEAD || status_kind == *FIGHTER_STATUS_KIND_MISS_FOOT || 
        sv_information::is_ready_go() == false {
            DEFENCE_BOOST[ENTRY_ID] = false;
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_miifighter
    );
}