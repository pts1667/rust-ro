#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val};

pub fn summoner_cr5(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn summoner_cr5_ontouch(ctx: &Ctx) -> Script {
    ctx.var("crus_q").set(6)?;
    ctx.quests().change(3010, 3011)?;
    ctx.warp("prt_castle", 164, 28)?;
    ctx.end()
}

pub fn monster_summon_cr5(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn monster_summon_cr5_oninit(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Monster Summon#cr5", false)?;
    ctx.end()
}

pub fn monster_summon_cr5_ontouch(ctx: &Ctx) -> Script {
    ctx.warp("prt_castle", 35, 147)?;
    ctx.npc().do_event("Monster Summon#cr0::OnReset")?;
    ctx.npc().do_event("Monster Summon#cr4::OnReset")?;
    ctx.npc().do_event("Monster Summon#cr0::OnEnd")?;
    ctx.npc().do_event("Monster Summon#cr4::OnEnd")?;
    ctx.npc().do_event("Monster Summon#cr5::OnEnd")?;
    ctx.npc().do_event("Monster Summon#cr6::OnStop")?;
    ctx.npc().do_event("Monster Summon#cr6::OnEnd")?;
    ctx.npc().do_event("Waiting Room#cr1::OnStart")?;
    ctx.end()
}

pub fn monster_summon_cr5_onstart(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Monster Summon#cr5", true)?;
    ctx.end()
}

pub fn monster_summon_cr5_onend(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Monster Summon#cr5", false)?;
    ctx.end()
}
