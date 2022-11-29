use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    }
};

#[skyline::hook(replace = L2CFighterCommon_sub_glide_stick_check_uniq)]
unsafe fn sub_glide_stick_check_uniq(fighter: &mut L2CFighterCommon) {
    let stick_x = fighter.global_table[0x1A].get_f32(); // 0x1A
    if stick_x.abs() < 0.5 {
        return;
    }
    let flick_x = fighter.global_table[0x1C].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_INPUT_BACK) {
        if flick_x < 3 && stick_x * lr < 0.0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_INPUT_BACK);
        }
    }
    else {
        if flick_x < 3 && stick_x * lr > 0.0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_INPUT);
        }
    }
}

#[skyline::hook(replace = L2CFighterCommon_sub_glide_check)]
unsafe fn sub_glide_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_GLIDE) {
        return false.into();
    }
    let jump_button_on_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_JUMP_WORK_INT_BUTTON_ON_FRAME);
    if jump_button_on_frame <= jump_hold_frame(fighter.global_table[0x2].get_i32()) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_INPUT) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_ENABLE) {
                if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
                    fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_START.into(), true.into());
                    return true.into();
                }
            }
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_ENABLE) {
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_START.into(), true.into());
                return true.into();
            }
        }
    }
    false.into()
}

unsafe fn jump_hold_frame(kind: i32) -> i32 {
    if kind == *FIGHTER_KIND_BUDDY {
        return 22;
    }
    if kind == *FIGHTER_KIND_METAKNIGHT {
        return 16;
    }
    if kind == *FIGHTER_KIND_PALUTENA {
        return 18;
    }
    if kind == *FIGHTER_KIND_PLIZARDON {
        return 28;
    }
    if kind == *FIGHTER_KIND_RIDLEY {
        return 25;
    }
    if kind == *FIGHTER_KIND_TRAIL {
        return 24;
    }
    else {
        return 20;
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_glide_stick_check_uniq,
            sub_glide_check
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}