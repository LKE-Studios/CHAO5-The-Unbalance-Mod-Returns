use smash::app::*;

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app11BossManagerEE9instance_E"]
    static BOSS_MANAGER: *mut smash::app::BossManager;
}

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app11ItemManagerEE9instance_E"]
    static ITEM_MANAGER: *mut smash::app::ItemManager;
}

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app12StageManagerEE9instance_E"]
    static STAGE_MANAGER: *mut smash::app::StageManager;
}

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E"]
    static FIGHTER_MANAGER: *mut smash::app::FighterManager;
}

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app16BattleObjectSlowEE9instance_E"]
    static BATTLE_OBJECT_SLOW: *mut smash::app::BattleObjectSlow;
}

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app17BattleObjectWorldEE9instance_E"]
    static BATTLE_OBJECT_WORLD: *mut smash::app::BattleObjectWorld;
}

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app17ItemParamAccessorEE9instance_E"]
    static ITEM_PARAM_ACCESSOR: *mut smash::app::ItemParamAccessor;
}

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app19BattleObjectManagerEE9instance_E"]
    static BATTLE_OBJECT_MANAGER: *mut smash::app::BattleObjectManager;
}

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E"]
    static FIGHTER_CUT_IN_MANAGER: *mut smash::app::FighterCutInManager;
}

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app21FighterParamAccessor2EE9instance_E"]
    static FIGHTER_PARAM_ACCESSOR2: *mut smash::app::FighterParamAccessor2;
}

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app21GimmickEventPresenterEE9instance_E"]
    static GIMMICK_EVENT_PRESENTER: *mut smash::app::GimmickEventPresenter;
}

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app22FighterPitBFinalModuleEE9instance_E"]
    static FIGHTER_PIT_B_FINAL_MODULE: *mut smash::app::FighterPitBFinalModule;
}

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app27FighterBayonettaFinalModuleEE9instance_E"]
    static FIGHTER_BAYONETTA_FINAL_MODULE: *mut smash::app::FighterBayonettaFinalModule;
}

// extern "C"{
//     #[link_name = "_ZN3lib9SingletonINS_10LuaManagerEE9instance_E"]
//     static LUA_MANAGER: *mut u8;
// }

// extern "C"{
//     #[link_name = "_ZN3lib9SingletonINS_13EffectManagerEE9instance_E"]
//     static EFFECT_MANAGER: *mut u8;
// }

macro_rules! expose_singleton {
    ($($public:ident, $private:ident)*) => {
        $(
            #[inline(always)]
            #[allow(non_snake_case)]
            pub fn $public() -> *mut $public {
                unsafe {
                    $private
                }
            }
        )*
    }
}

expose_singleton!(
    BossManager,                 BOSS_MANAGER
    ItemManager,                 ITEM_MANAGER
    StageManager,                STAGE_MANAGER
    FighterManager,              FIGHTER_MANAGER
    BattleObjectSlow,            BATTLE_OBJECT_SLOW
    BattleObjectWorld,           BATTLE_OBJECT_WORLD
    ItemParamAccessor,           ITEM_PARAM_ACCESSOR
    BattleObjectManager,         BATTLE_OBJECT_MANAGER
    FighterCutInManager,         FIGHTER_CUT_IN_MANAGER
    FighterParamAccessor2,       FIGHTER_PARAM_ACCESSOR2
    GimmickEventPresenter,       GIMMICK_EVENT_PRESENTER
    FighterPitBFinalModule,      FIGHTER_PIT_B_FINAL_MODULE
    FighterBayonettaFinalModule, FIGHTER_BAYONETTA_FINAL_MODULE
    // LuaManager,                  LUA_MANAGER
    // EffectManager,               EFFECT_MANAGER
);