//! NPCs and events that run on server-side systems: the warper, job masters, castles, weddings and battlegrounds.

mod battleground_arena;
mod battleground_kvm;
mod battleground_npcs;
mod battleground_tierra;
mod castle_npcs;
mod npcs;
mod wedding_npcs;

use script_sdk_2::{Ctx, Script, VariableScope};

/// The event table entry of one `kind` of a battleground arena's events.
macro_rules! event {
    ($name:ident => $path:path, $arena:literal, $kind:literal) => {
        fn $name(ctx: &Ctx) -> Script {
            $path(ctx, $arena, $kind)
        }
    };
}

fn shop(ctx: &Ctx) -> Script {
    ctx.npc().open_shop()
}

/// Counts the monsters a player summoned that died.
fn summoned_mob_death(ctx: &Ctx) -> Script {
    if ctx.arguments()?.len() < 2 {
        return Err("Monster death event needs its runtime and class IDs".into());
    }
    ctx.increment(&[(VariableScope::Character, "SummonedMobKills", 1)]).map(|_| ())
}

event!(arena_0_0 => battleground_arena::event, 0, 0);
event!(arena_0_1 => battleground_arena::event, 0, 1);
event!(arena_0_10 => battleground_arena::event, 0, 10);
event!(arena_0_11 => battleground_arena::event, 0, 11);
event!(arena_0_12 => battleground_arena::event, 0, 12);
event!(arena_0_13 => battleground_arena::event, 0, 13);
event!(arena_0_2 => battleground_arena::event, 0, 2);
event!(arena_0_20 => battleground_arena::event, 0, 20);
event!(arena_0_21 => battleground_arena::event, 0, 21);
event!(arena_0_22 => battleground_arena::event, 0, 22);
event!(arena_0_23 => battleground_arena::event, 0, 23);
event!(arena_0_3 => battleground_arena::event, 0, 3);
event!(arena_0_30 => battleground_arena::event, 0, 30);
event!(arena_0_31 => battleground_arena::event, 0, 31);
event!(arena_0_32 => battleground_arena::event, 0, 32);
event!(arena_0_33 => battleground_arena::event, 0, 33);
event!(arena_0_4 => battleground_arena::event, 0, 4);
event!(arena_0_40 => battleground_arena::event, 0, 40);
event!(arena_0_41 => battleground_arena::event, 0, 41);
event!(arena_0_42 => battleground_arena::event, 0, 42);
event!(arena_0_43 => battleground_arena::event, 0, 43);
event!(arena_0_44 => battleground_arena::event, 0, 44);
event!(arena_0_45 => battleground_arena::event, 0, 45);
event!(arena_0_46 => battleground_arena::event, 0, 46);
event!(arena_0_47 => battleground_arena::event, 0, 47);
event!(arena_0_48 => battleground_arena::event, 0, 48);
event!(arena_0_50 => battleground_arena::event, 0, 50);
event!(arena_0_51 => battleground_arena::event, 0, 51);
event!(kvm_0_0 => battleground_kvm::event, 0, 0);
event!(kvm_0_1 => battleground_kvm::event, 0, 1);
event!(kvm_0_10 => battleground_kvm::event, 0, 10);
event!(kvm_0_11 => battleground_kvm::event, 0, 11);
event!(kvm_0_12 => battleground_kvm::event, 0, 12);
event!(kvm_0_13 => battleground_kvm::event, 0, 13);
event!(kvm_0_14 => battleground_kvm::event, 0, 14);
event!(kvm_0_15 => battleground_kvm::event, 0, 15);
event!(kvm_0_16 => battleground_kvm::event, 0, 16);
event!(kvm_0_17 => battleground_kvm::event, 0, 17);
event!(kvm_0_18 => battleground_kvm::event, 0, 18);
event!(kvm_0_19 => battleground_kvm::event, 0, 19);
event!(kvm_0_2 => battleground_kvm::event, 0, 2);
event!(kvm_0_20 => battleground_kvm::event, 0, 20);
event!(kvm_0_21 => battleground_kvm::event, 0, 21);
event!(kvm_0_22 => battleground_kvm::event, 0, 22);
event!(kvm_0_23 => battleground_kvm::event, 0, 23);
event!(kvm_0_24 => battleground_kvm::event, 0, 24);
event!(kvm_0_3 => battleground_kvm::event, 0, 3);
event!(kvm_0_30 => battleground_kvm::event, 0, 30);
event!(kvm_0_31 => battleground_kvm::event, 0, 31);
event!(kvm_0_32 => battleground_kvm::event, 0, 32);
event!(kvm_0_33 => battleground_kvm::event, 0, 33);
event!(kvm_0_34 => battleground_kvm::event, 0, 34);
event!(kvm_0_35 => battleground_kvm::event, 0, 35);
event!(kvm_0_4 => battleground_kvm::event, 0, 4);
event!(kvm_0_5 => battleground_kvm::event, 0, 5);
event!(tierra_0_0 => battleground_tierra::event, 0, 0);
event!(tierra_0_1 => battleground_tierra::event, 0, 1);
event!(tierra_0_10 => battleground_tierra::event, 0, 10);
event!(tierra_0_11 => battleground_tierra::event, 0, 11);
event!(tierra_0_12 => battleground_tierra::event, 0, 12);
event!(tierra_0_14 => battleground_tierra::event, 0, 14);
event!(tierra_0_15 => battleground_tierra::event, 0, 15);
event!(tierra_0_2 => battleground_tierra::event, 0, 2);
event!(tierra_0_20 => battleground_tierra::event, 0, 20);
event!(tierra_0_21 => battleground_tierra::event, 0, 21);
event!(tierra_0_22 => battleground_tierra::event, 0, 22);
event!(tierra_0_23 => battleground_tierra::event, 0, 23);
event!(tierra_0_24 => battleground_tierra::event, 0, 24);
event!(tierra_0_25 => battleground_tierra::event, 0, 25);
event!(tierra_0_3 => battleground_tierra::event, 0, 3);
event!(tierra_0_30 => battleground_tierra::event, 0, 30);
event!(tierra_0_31 => battleground_tierra::event, 0, 31);
event!(tierra_0_32 => battleground_tierra::event, 0, 32);
event!(tierra_0_33 => battleground_tierra::event, 0, 33);
event!(tierra_0_40 => battleground_tierra::event, 0, 40);
event!(tierra_0_41 => battleground_tierra::event, 0, 41);
event!(tierra_0_42 => battleground_tierra::event, 0, 42);
event!(tierra_0_43 => battleground_tierra::event, 0, 43);
event!(tierra_0_44 => battleground_tierra::event, 0, 44);
event!(tierra_0_45 => battleground_tierra::event, 0, 45);
event!(tierra_0_46 => battleground_tierra::event, 0, 46);
event!(tierra_0_47 => battleground_tierra::event, 0, 47);
event!(tierra_0_5 => battleground_tierra::event, 0, 5);
event!(tierra_0_50 => battleground_tierra::event, 0, 50);
event!(tierra_0_60 => battleground_tierra::event, 0, 60);
event!(tierra_0_61 => battleground_tierra::event, 0, 61);
event!(tierra_0_62 => battleground_tierra::event, 0, 62);
event!(tierra_0_63 => battleground_tierra::event, 0, 63);

script_sdk_2::script_module! {
    npcs {
        "battleground_arena" => battleground_arena::npc,
        "battleground_kvm" => battleground_kvm::npc,
        "battleground_recruiter" => battleground_npcs::npc,
        "battleground_tierra" => battleground_tierra::npc,
        "breeder" => npcs::breeder,
        "castle_flag" => castle_npcs::flag,
        "castle_kafra" => castle_npcs::kafra,
        "castle_lever" => castle_npcs::lever,
        "castle_steward" => castle_npcs::steward,
        "counter" => npcs::counter,
        "job_master" => npcs::job_master,
        "mount_master" => npcs::mount_master,
        "shop" => shop,
        "stylist" => npcs::stylist,
        "variables" => npcs::variables,
        "warper" => npcs::warper,
        "wedding_bishop" => wedding_npcs::bishop,
        "wedding_divorce" => wedding_npcs::divorce,
        "wedding_staff" => wedding_npcs::staff,
    }
    events {
        "arena_0_0" => arena_0_0,
        "arena_0_1" => arena_0_1,
        "arena_0_10" => arena_0_10,
        "arena_0_11" => arena_0_11,
        "arena_0_12" => arena_0_12,
        "arena_0_13" => arena_0_13,
        "arena_0_2" => arena_0_2,
        "arena_0_20" => arena_0_20,
        "arena_0_21" => arena_0_21,
        "arena_0_22" => arena_0_22,
        "arena_0_23" => arena_0_23,
        "arena_0_3" => arena_0_3,
        "arena_0_30" => arena_0_30,
        "arena_0_31" => arena_0_31,
        "arena_0_32" => arena_0_32,
        "arena_0_33" => arena_0_33,
        "arena_0_4" => arena_0_4,
        "arena_0_40" => arena_0_40,
        "arena_0_41" => arena_0_41,
        "arena_0_42" => arena_0_42,
        "arena_0_43" => arena_0_43,
        "arena_0_44" => arena_0_44,
        "arena_0_45" => arena_0_45,
        "arena_0_46" => arena_0_46,
        "arena_0_47" => arena_0_47,
        "arena_0_48" => arena_0_48,
        "arena_0_50" => arena_0_50,
        "arena_0_51" => arena_0_51,
        "kvm_0_0" => kvm_0_0,
        "kvm_0_1" => kvm_0_1,
        "kvm_0_10" => kvm_0_10,
        "kvm_0_11" => kvm_0_11,
        "kvm_0_12" => kvm_0_12,
        "kvm_0_13" => kvm_0_13,
        "kvm_0_14" => kvm_0_14,
        "kvm_0_15" => kvm_0_15,
        "kvm_0_16" => kvm_0_16,
        "kvm_0_17" => kvm_0_17,
        "kvm_0_18" => kvm_0_18,
        "kvm_0_19" => kvm_0_19,
        "kvm_0_2" => kvm_0_2,
        "kvm_0_20" => kvm_0_20,
        "kvm_0_21" => kvm_0_21,
        "kvm_0_22" => kvm_0_22,
        "kvm_0_23" => kvm_0_23,
        "kvm_0_24" => kvm_0_24,
        "kvm_0_3" => kvm_0_3,
        "kvm_0_30" => kvm_0_30,
        "kvm_0_31" => kvm_0_31,
        "kvm_0_32" => kvm_0_32,
        "kvm_0_33" => kvm_0_33,
        "kvm_0_34" => kvm_0_34,
        "kvm_0_35" => kvm_0_35,
        "kvm_0_4" => kvm_0_4,
        "kvm_0_5" => kvm_0_5,
        "summoned_mob_death" => summoned_mob_death,
        "tierra_0_0" => tierra_0_0,
        "tierra_0_1" => tierra_0_1,
        "tierra_0_10" => tierra_0_10,
        "tierra_0_11" => tierra_0_11,
        "tierra_0_12" => tierra_0_12,
        "tierra_0_14" => tierra_0_14,
        "tierra_0_15" => tierra_0_15,
        "tierra_0_2" => tierra_0_2,
        "tierra_0_20" => tierra_0_20,
        "tierra_0_21" => tierra_0_21,
        "tierra_0_22" => tierra_0_22,
        "tierra_0_23" => tierra_0_23,
        "tierra_0_24" => tierra_0_24,
        "tierra_0_25" => tierra_0_25,
        "tierra_0_3" => tierra_0_3,
        "tierra_0_30" => tierra_0_30,
        "tierra_0_31" => tierra_0_31,
        "tierra_0_32" => tierra_0_32,
        "tierra_0_33" => tierra_0_33,
        "tierra_0_40" => tierra_0_40,
        "tierra_0_41" => tierra_0_41,
        "tierra_0_42" => tierra_0_42,
        "tierra_0_43" => tierra_0_43,
        "tierra_0_44" => tierra_0_44,
        "tierra_0_45" => tierra_0_45,
        "tierra_0_46" => tierra_0_46,
        "tierra_0_47" => tierra_0_47,
        "tierra_0_5" => tierra_0_5,
        "tierra_0_50" => tierra_0_50,
        "tierra_0_60" => tierra_0_60,
        "tierra_0_61" => tierra_0_61,
        "tierra_0_62" => tierra_0_62,
        "tierra_0_63" => tierra_0_63,
    }
}
