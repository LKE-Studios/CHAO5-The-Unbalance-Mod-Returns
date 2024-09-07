use crate::imports::BuildImports::*;

static mut INKLING_COLORS : [Vector3f; 256] = [Vector3f {x: 0.0, y: 0.0, z: 0.0,}; 256]; //Used to tint the hitbox effects
static mut INK_COLOR_OFFSET : usize = 0x0767510;

pub unsafe extern "C" fn frame_inkling_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);   
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_100 {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(fighter.module_accessor, -1.0, 0);
        }
    };
    if motion_kind == hash40("attack_air_lw") && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        let pos = Vector3f{x: 0.0, y: -2.0, z: 0.0};
        let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
        let handle2 = EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("inkling_blaster_muzzle"), Hash40::new("top"), &pos, &rot, 2.2,&Vector3f{x: 0.0, y: 0.0, z: 0.0},&Vector3f{x: 0.0, y: 0.0, z: 0.0}, false, 0, 0, 0) as u32;
        let costumenum = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as usize;
        EffectModule::set_rgb(fighter.module_accessor, handle2, INKLING_COLORS[costumenum].x, INKLING_COLORS[costumenum].y, INKLING_COLORS[costumenum].z);
        EffectModule::set_rate_last(fighter.module_accessor, 0.5);
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_N_SHOOT].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_GROUND {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_INKLING_STATUS_KIND_CHARGE_INK_START, false);
            }
        }
    }
    if status_kind == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN {
        if MotionModule::frame(fighter.module_accessor) > 25.0 {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.sub_wait_ground_check_common(false.into());
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD, false);
                }
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP, false);
                }
            }
            if situation_kind == *SITUATION_KIND_AIR {
                fighter.sub_air_check_fall_common();
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                }
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                }
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                }
            }
        }
    };
    if status_kind == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_END {
        if MotionModule::frame(fighter.module_accessor) > 10.0 {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.sub_wait_ground_check_common(false.into());
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD, false);
                }
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP, false);
                }
            }
            if situation_kind == *SITUATION_KIND_AIR {
                fighter.sub_air_check_fall_common();
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                }
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                }
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                }
            }
        }
    };
}

#[skyline::hook(offset = INK_COLOR_OFFSET, inline)]
pub fn get_ink_colors(ctx: &mut InlineCtx) {
    //Assigns RGB values for the relevant slot in the effect.prc to the declared Vector
    unsafe {
        let color_address = *(ctx.registers[12].x.as_ref());
        let red = *((color_address) as *const f32);
        let green = *((color_address + 4) as *const f32);
        let blue = *((color_address + 8) as *const f32);
        let index = (*(ctx.registers[8].x.as_ref()) -1) as usize;
        INKLING_COLORS[index].x = red;
        INKLING_COLORS[index].y = green;
        INKLING_COLORS[index].z = blue;
    }
}

pub fn install() {
    Agent::new("inkling")
    .on_line(Main, frame_inkling_Main)
    .install();
    skyline::install_hooks!(
        get_ink_colors
    );
}