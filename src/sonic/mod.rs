use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*
};

mod game;
mod status;
mod frame;
//mod expression;
//mod effect;
//mod sound;

pub const THROW_HI_STATUS_KIND: i32 = 0x47;

/*pub unsafe fn is_knuckles(boma: *mut BattleObjectModuleAccessor) -> bool {
    WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8
}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_SONIC {
            return;
        }
        if !is_knuckles(fighter.module_accessor) {
            return;
        }
        //Force throw status change
        fighter.global_table[THROW_HI_STATUS_KIND].assign(&FIGHTER_STATUS_KIND_THROW_KIRBY.into());
    }
}*/

pub fn install() {
    game::install();    
    frame::install();
    status::install();
    /*expression::install();
    effect::install();
    sound::install();
    install_agent_init_callbacks!(
        agent_init
    );*/
}
