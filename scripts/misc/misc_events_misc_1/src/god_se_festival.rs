#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants, runtime};

pub fn festival_manager_gq_fes0(ctx: &Ctx) -> Script {
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.lines_as(
        "Festival Manager",
        args!["I am the festival NPC manager for the God SE quest.", "Please enter your password."],
    )?;
    ctx.next()?;
    if shared::other_gm_npcs::f_gm_npc(ctx, args!["07godsefes", 1])? == 0 {
        ctx.lines_as("Festival Manager", args!["Password is incorrect."])?;
        return ctx.close();
    }
    ctx.lines_as("Festival Manager", args!["What would you like to do?"])?;
    ctx.next()?;
    if ctx.menu(&["Disable festival NPCs", "Enable festival NPCs"])? == 0 {
        ctx.lines_as("Festival Manager", args!["Ending festivals and disabling NPCs."])?;
        ctx.npc().do_event("Rmimi Ravies#gq_fes01::Onover")?;
        ctx.npc().do_event("Rmimi Ravies#gq_fes01::Onover")?;
        return ctx.close();
    }
    ctx.lines_as("Festival Manager", args!["Resetting festivals and enabling NPCs."])?;
    ctx.npc().do_event("Rmimi Ravies#gq_fes01::Onover")?;
    ctx.npc().do_event("Rmimi Ravies#gq_fes01::OnEnable")?;
    ctx.close()
}

pub fn rmimi_ravies_gq_fes01(ctx: &Ctx) -> Script {
    if ctx.items().count(7840)? <= 0 {
        ctx.lines_as("Rmimi Ravies", args!["Don't you have any voucher?"])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Rmimi Ravies",
        args![
            "This is the flower.",
            "This is just a formality, but let me check your qualifications."
        ],
    )?;
    ctx.next()?;
    if ctx.call(Function::GetGuildInfo, args![ctx.call(Function::GetCharacterId, args![2])?, 2])? != 1 {
        ctx.lines_as("Rmimi Ravies", args!["It seems you are not worthy."])?;
        ctx.next()?;
        ctx.lines_as(
            "Rmimi Ravies",
            args!["I don't know how you obtained that flower, but if you're not capable of leading others.. then you cannot continue."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rmimi Ravies",
            args!["Too tough?", "Well, what can you do, that's part of the job."],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Rmimi Ravies", args!["Your qualification is verified as a Guild master."])?;
    ctx.next()?;
    ctx.lines_as(
        "Rmimi Ravies",
        args!["I wonder if you know what this flower's use is... Let me explain it to you."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rmimi Ravies",
        args!["This is the holy flower given by the Wish Maiden in Valkyrie, it can summon certain monsters by certain summoners."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rmimi Ravies",
        args!["There are only two summoners in this world...", "Me and my sister Rhehe..."],
    )?;
    ctx.next()?;
    ctx.lines_as("Rmimi Ravies", args!["We are totllly different, but we can..."])?;
    ctx.next()?;
    ctx.lines_as("Rmimi Ravies", args!["..................................."])?;
    ctx.next()?;
    ctx.lines_as("Rmimi Ravies", args!["Anyway,", "Do you want to summon monsters?"])?;
    ctx.next()?;
    if ctx.menu(&["Do not summon", "Summon, please"])? == 0 {
        ctx.lines_as("Rmimi Ravies", args!["I don't like to be interrupted by others.."])?;
        ctx.next()?;
        ctx.lines_as("Rmimi Ravies", args!["But it's just business."])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Rmimi Ravies",
        args![
            "I will explain to you how to summon monsters.",
            "This flower can summon the monsters 'Valkyrie's Blessing' and 'Valkyrie's Present' for one hour here in Juno."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rmimi Ravies",
        args![
            "An announcement will be made stating which monster is summoned.",
            "This festival is given by the Wish maiden for all adventurers to enjoy all over the world."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Rmimi Ravies", args!["Now, are you ready to summon monsters?"])?;
    ctx.next()?;
    if ctx.menu(&["No, I'm not ready now.", "Yes! I'm ready for that."])? == 0 {
        ctx.lines_as("Rmimi Ravies", args!["If you are not ready, why did you come to me?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Rmimi Ravies",
            args!["I'll be waiting until you are ready.", "It is just business afterall."],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Rmimi Ravies", args!["Now we are ready."])?;
    ctx.next()?;
    if ctx.var("$@gqse_festival").get()? != 0 {
        ctx.lines_as(
            "Rmimi Ravies",
            args!["It seems that a Valkyrie's Blessing summoning ritual is already in progress elsewhere."],
        )?;
        ctx.next()?;
        ctx.lines_as("Rmimi Ravies", args!["Please try again later."])?;
        return ctx.close();
    }
    ctx.lines_as("Rmimi Ravies", args!["Ok, we would ge started to summon monsters."])?;
    ctx.items().take(7840, 1)?;
    ctx.next()?;
    ctx.lines_as("Rmimi Ravies", args!["Have a good time."])?;
    ctx.npc().do_event("Rmimi Ravies#gq_fes01::OnStart")?;
    ctx.call(
        Function::Announce,
        args![
            Val::from("[")
                + ctx.player().name()?
                + Val::from("] member of [")
                + ctx.call(Function::GetGuildInfo, args![ctx.call(Function::GetCharacterId, args![2])?, 0])?
                + Val::from("] is summoning a 'Valkyrie's Present' in 'Juno'."),
            constants::BC_ALL,
            "0x70dbdb"
        ],
    )?;
    ctx.close()
}

pub fn rmimi_ravies_gq_fes01_oninit(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Rmimi Ravies#gq_fes01", true)?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_onenable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Rmimi Ravies#gq_fes01", true)?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ondisable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Rmimi Ravies#gq_fes01", false)?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_onstart(ctx: &Ctx) -> Script {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.var("$@gqse_festival").set(Val::from(1))?;
    ctx.end()
}

fn announce_juno(ctx: &Ctx, message: &str) -> Result<(), Stop> {
    ctx.call(Function::Announce, args![message, constants::BC_ALL, "0x70dbdb"])?;
    Ok(())
}

fn summon_valkyries(ctx: &Ctx, map: &str, on_dead: &str, monsters: &[(&str, i32, i32)]) -> Result<(), Stop> {
    for &(name, id, count) in monsters {
        ctx.call(Function::Monster, args![map, 0, 0, name, id, count, on_dead])?;
    }
    Ok(())
}

pub fn rmimi_ravies_gq_fes01_oncall(ctx: &Ctx) -> Script {
    if ctx
        .call(Function::MobCount, args!["yuno", "Rmimi Ravies#gq_fes01::OnMyMobDead"])?
        .number()?
        < 31
    {
        summon_valkyries(
            ctx,
            "yuno",
            "Rmimi Ravies#gq_fes01::OnMyMobDead",
            &[
                ("Valkyrie's Blessing", 1083, 100),
                ("Valkyrie's Gift", 1951, 25),
                ("Valkyrie's Gift", 1952, 25),
                ("Valkyrie's Gift", 1953, 25),
                ("Valkyrie's Gift", 1954, 25),
                ("Valkyrie's Prank", 1002, 10),
            ],
        )?;
    } else if ctx
        .call(Function::MobCount, args!["yuno", "Rmimi Ravies#gq_fes01::OnMyMobDead"])?
        .number()?
        > 149
    {
        summon_valkyries(
            ctx,
            "yuno",
            "Rmimi Ravies#gq_fes01::OnMyMobDead",
            &[
                ("Valkyrie's Blessing", 1083, 1),
                ("Valkyrie's Gift", 1951, 1),
                ("Valkyrie's Gift", 1952, 1),
                ("Valkyrie's Gift", 1953, 1),
                ("Valkyrie's Gift", 1954, 1),
            ],
        )?;
    } else {
        summon_valkyries(
            ctx,
            "yuno",
            "Rmimi Ravies#gq_fes01::OnMyMobDead",
            &[
                ("Valkyrie's Blessing", 1083, 50),
                ("Valkyrie's Gift", 1951, 10),
                ("Valkyrie's Gift", 1952, 10),
                ("Valkyrie's Gift", 1953, 10),
                ("Valkyrie's Gift", 1954, 10),
                ("Valkyrie's Prank", 1002, 5),
            ],
        )?;
    }
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_onover(ctx: &Ctx) -> Script {
    ctx.call(Function::KillMonster, args!["yuno", "Rmimi Ravies#gq_fes01::OnMyMobDead"])?;
    ctx.var("$@gqse_festival").set(Val::from(0))?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_onmymobdead(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer10000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "The summoning ceremony will start in 5 min in Juno.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer13000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Juno to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer120000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "The summoning ceremony will start in 3 min in Juno.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer123000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Juno to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer240000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "The summoning ceremony will start in 1 min in Juno.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer243000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Juno to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer300000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The first 'Valkyrie's Present' has been summoned here in Juno by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rmimi Ravies#gq_fes01::Oncall")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer303000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer308000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Juno to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer600000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The second 'Valkyrie's Present' has been summoned here in Juno by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rmimi Ravies#gq_fes01::oncall")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer603000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer608000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Juno to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer900000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The third 'Valkyrie's Present' has been summoned here in Juno by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rmimi Ravies#gq_fes01::Oncall")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer903000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer908000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Juno to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer1200000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The fourth 'Valkyrie's Present' has been summoned here in Juno by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rmimi Ravies#gq_fes01::Oncall")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer1203000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer1208000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Juno to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer1500000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The fifth 'Valkyrie's Present' has been summoned here in Juno by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rmimi Ravies#gq_fes01::Oncall")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer1503000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer1508000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Juno to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer1800000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The sixth 'Valkyrie's Present' has been summoned here in Juno by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rmimi Ravies#gq_fes01::Oncall")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer1803000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer1808000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Juno to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer2100000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The seventh 'Valkyrie's Present' has been summoned here in Juno by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rmimi Ravies#gq_fes01::Oncall")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer2103000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer2108000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Juno to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer2400000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The eighth 'Valkyrie's Present' has been summoned here in Juno by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rmimi Ravies#gq_fes01::Oncall")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer2403000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer2408000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Juno to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer2700000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The ninth 'Valkyrie's Present' has been summoned here in Juno by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rmimi Ravies#gq_fes01::Oncall")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer2703000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer2708000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Juno to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer3000000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The first0 'Valkyrie's Present' has been summoned here in Juno by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rmimi Ravies#gq_fes01::Oncall")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer3003000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer3008000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Juno to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer3300000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The eleventh 'Valkyrie's Present' has been summoned here in Juno by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rmimi Ravies#gq_fes01::Oncall")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer3303000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer3308000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Juno to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer3600000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The twelfth 'Valkyrie's Present' has been summoned here in Juno by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rmimi Ravies#gq_fes01::Oncall")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer3603000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "The final ceremony will be performed for 5 minutes.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer3608000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Juno to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rmimi_ravies_gq_fes01_ontimer3900000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "The entire ceremony is over now, I am sure all of you had fun.")?;
    ctx.npc().do_event("Rmimi Ravies#gq_fes01::Onover")?;
    ctx.end()
}

pub fn festival_manager_gq_fes2(ctx: &Ctx) -> Script {
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.lines_as(
        "Festival Manager",
        args!["I am the festival NPC manager for the God SE quest.", "Please enter your password."],
    )?;
    ctx.next()?;
    if shared::other_gm_npcs::f_gm_npc(ctx, args!["07godsefes", 1])? == 0 {
        ctx.lines_as("Festival Manager", args!["Password is incorrect."])?;
        return ctx.close();
    }
    ctx.lines_as("Festival Manager", args!["What would you like to do?"])?;
    ctx.next()?;
    if ctx.menu(&["Disable festival NPCs", "Enable festival NPCs"])? == 0 {
        ctx.lines_as("Festival Manager", args!["Ending festivals and disabling NPCs."])?;
        ctx.npc().do_event("Rhehe Ravies#gq_fes03::Onover")?;
        ctx.npc().do_event("Rhehe Ravies#gq_fes03::Ondisable")?;
        return ctx.close();
    }
    ctx.lines_as("Festival Manager", args!["Resetting festivals and enabling NPCs."])?;
    ctx.npc().do_event("Rhehe Ravies#gq_fes03::Onover")?;
    ctx.npc().do_event("Rhehe Ravies#gq_fes03::OnEnable")?;
    ctx.close()
}

pub fn rhehe_ravies_gq_fes03(ctx: &Ctx) -> Script {
    if ctx.items().count(7840)? <= 0 {
        ctx.lines_as("Rhehe Ravies", args!["Don't you have anything to give me to prove yourself?"])?;
        return ctx.close();
    }
    ctx.lines_as("Rhehe Ravies", args!["A flower!!", "Shall we proceed?"])?;
    ctx.next()?;
    if ctx.call(Function::GetGuildInfo, args![ctx.call(Function::GetCharacterId, args![2])?, 2])? != 1 {
        ctx.lines_as(
            "Rhehe Ravies",
            args!["...Seems like you're not in a worthy position for me to talk to?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rhehe Ravies",
            args!["I don't know how you obtained that flower, but if you're not capable of leading others.. then you cannot continue."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rhehe Ravies",
            args!["Too tough?", "Well, what can you do, that's part of the job."],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Rhehe Ravies",
        args!["Eeeeh, so young and yet you're a guild master? Your guildsmen must be jealous."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Reumimi Ravies",
        args!["I'm not sure if you understand the uses for that flower, so I'll briefly explain."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Reumimi Ravies",
        args![
            "That flower has been passed down as a gift from the Valkyrie Wish Maiden herself.",
            "It may be used to summon special monsters through summoners in special areas."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rhehe Ravies",
        args![
            "There are only two summoners in existence who are capable of using that flower.",
            "One is me, and the other is my twin sister Reumimi."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rhehe Ravies",
        args!["Twin sisters, but we do not have much alike, little fingers in front of you ..."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rhehe Ravies",
        args![
            "..................................",
            ".....Oops! I wasn't supposed to tell anyone..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rhehe Ravies",
        args!["... Hehehe, anyways let's move on.", "So do you wish to summon or not!?"],
    )?;
    ctx.next()?;
    if ctx.menu(&["Do not summon", "Summon"])? == 0 {
        ctx.lines_as("Rhehe Ravies", args!["...*Cries*."])?;
        ctx.next()?;
        ctx.lines_as(
            "Rhehe Ravies",
            args![
                "If you've got no business with me, please don't start a conversation.",
                "If it were my sister, she would've humiliated you for it."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Rhehe Ravies",
        args![
            "Okay then, I'll explain a little about the summoning ritual.",
            "Here in Juno, the summoning ritual through the use of that flower will summon Valkyrie's Blessing and Valkyrie's Gift.",
            "The effects of the summoning ritual will last approximately one hour."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rhehe Ravies",
        args![
            "During the summoning ritual, continuous broadcasts will be made.",
            "It is a courtesy of the Valkyrie Wish Maiden, in order to unite adventurers from all over to participate in the festival."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Rhehe Ravies", args!["Are you ready to begin the summoning ritual~?"])?;
    ctx.next()?;
    if ctx.menu(&["No, not yet.", "Yes! I'm ready!"])? == 0 {
        ctx.lines_as("Rhehe Ravies", args!["You're not even ready, why bother talking to me?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Rhehe Ravies",
            args![
                "Oh well, if you've got other things to do then I'll wait.",
                "After all, that's also part of my job."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Rhehe Ravies",
        args!["Very well, then I will check for a reasonable time to begin the summoning ritual."],
    )?;
    ctx.next()?;
    if ctx.var("$@gqse_festival").get()? != 0 {
        ctx.lines_as("Rhehe Ravies", args!["*Cries*", "It doesn't seem like now is a good time."])?;
        ctx.next()?;
        ctx.lines_as("Rhehe Ravies", args!["Please try again later."])?;
        return ctx.close();
    }
    ctx.lines_as("Rhehe Ravies", args!["Good! Now seems like a good time, so let's begin!"])?;
    ctx.items().take(7840, 1)?;
    ctx.next()?;
    ctx.lines_as("Rhehe Ravies", args!["I hope you enjoy yourself!"])?;
    ctx.npc().do_event("Rhehe Ravies#gq_fes03::OnStart")?;
    ctx.call(
        Function::Announce,
        args![
            Val::from("[")
                + ctx.player().name()?
                + Val::from("] member of [")
                + ctx.call(Function::GetGuildInfo, args![ctx.call(Function::GetCharacterId, args![2])?, 0])?
                + Val::from("] is summoning a 'Valkyrie's Present' in 'Rachel'."),
            constants::BC_ALL,
            "0x70dbdb"
        ],
    )?;
    ctx.close()
}

pub fn rhehe_ravies_gq_fes03_oninit(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Rhehe Ravies#gq_fes03", true)?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_onenable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Rhehe Ravies#gq_fes03", true)?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ondisable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Rhehe Ravies#gq_fes03", false)?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_onstart(ctx: &Ctx) -> Script {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.var("$@gqse_festival").set(Val::from(1))?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_oncall(ctx: &Ctx) -> Script {
    if ctx
        .call(Function::MobCount, args!["rachel", "Rhehe Ravies#gq_fes03::OnMyMobDead"])?
        .number()?
        < 31
    {
        summon_valkyries(
            ctx,
            "rachel",
            "Rhehe Ravies#gq_fes03::OnMyMobDead",
            &[
                ("Valkyrie's Blessing", 1083, 100),
                ("Valkyrie's Gift", 1951, 25),
                ("Valkyrie's Gift", 1952, 25),
                ("Valkyrie's Gift", 1953, 25),
                ("Valkyrie's Gift", 1954, 25),
                ("Valkyrie's Prank", 1002, 10),
            ],
        )?;
    } else if ctx
        .call(Function::MobCount, args!["rachel", "Rhehe Ravies#gq_fes03::OnMyMobDead"])?
        .number()?
        > 149
    {
        summon_valkyries(
            ctx,
            "rachel",
            "Rhehe Ravies#gq_fes03::OnMyMobDead",
            &[
                ("Valkyrie's Blessing", 1083, 1),
                ("Valkyrie's Gift", 1951, 1),
                ("Valkyrie's Gift", 1952, 1),
                ("Valkyrie's Gift", 1953, 1),
                ("Valkyrie's Gift", 1954, 1),
            ],
        )?;
    } else {
        summon_valkyries(
            ctx,
            "rachel",
            "Rhehe Ravies#gq_fes03::OnMyMobDead",
            &[
                ("Valkyrie's Blessing", 1083, 50),
                ("Valkyrie's Gift", 1951, 10),
                ("Valkyrie's Gift", 1952, 10),
                ("Valkyrie's Gift", 1953, 10),
                ("Valkyrie's Gift", 1954, 10),
                ("Valkyrie's Prank", 1002, 5),
            ],
        )?;
    }
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_onover(ctx: &Ctx) -> Script {
    ctx.call(Function::KillMonster, args!["rachel", "Rhehe Ravies#gq_fes03::OnMyMobDead"])?;
    ctx.var("$@gqse_festival").set(Val::from(0))?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_onmymobdead(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer10000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "The summoning ceremony will start in 5 min in Rachel.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer13000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Rachel to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer120000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning of Valkyrie's Gift will begin in approximately 3 min in Rachel.",
    )?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer123000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Rachel to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer240000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "The summoning ceremony will start in 1 min in Rachel.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer243000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Rachel to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer300000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The first 'Valkyrie's Present' has been summoned here in Rachel by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rhehe Ravies#gq_fes03::Oncall")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer303000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer308000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Rachel to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer600000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The second 'Valkyrie's Present' has been summoned here in Rachel by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rhehe Ravies#gq_fes03::Oncall")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer603000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer608000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Rachel to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer900000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The third 'Valkyrie's Present' has been summoned here in Rachel by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rhehe Ravies#gq_fes03::Oncall")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer903000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer908000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Rachel to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer1200000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The fourth 'Valkyrie's Present' has been summoned here in Rachel by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rhehe Ravies#gq_fes03::Oncall")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer1203000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer1208000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Rachel to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer1500000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The fifth 'Valkyrie's Present' has been summoned here in Rachel by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rhehe Ravies#gq_fes03::Oncall")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer1503000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer1508000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Rachel to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer1800000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The sixth 'Valkyrie's Present' has been summoned here in Rachel by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rhehe Ravies#gq_fes03::Oncall")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer1803000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer1808000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Rachel to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer2100000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The seventh 'Valkyrie's Present' has been summoned here in Rachel by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rhehe Ravies#gq_fes03::Oncall")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer2103000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer2108000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Rachel to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer2400000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The eighth 'Valkyrie's Present' has been summoned here in Rachel by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rhehe Ravies#gq_fes03::Oncall")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer2403000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer2408000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Rachel to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer2700000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The ninth 'Valkyrie's Present' has been summoned here in Rachel by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rhehe Ravies#gq_fes03::Oncall")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer2703000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer2708000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Rachel to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer3000000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The tenth 'Valkyrie's Present' has been summoned here in Rachel by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rhehe Ravies#gq_fes03::Oncall")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer3003000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer3008000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Rachel to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer3300000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The eleventh 'Valkyrie's Present' has been summoned here in Rachel by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rhehe Ravies#gq_fes03::Oncall")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer3303000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The summoning ceremony will be performed 12 times at five-minute intervals for about one hour.",
    )?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer3308000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Rachel to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer3600000(ctx: &Ctx) -> Script {
    announce_juno(
        ctx,
        "The twelfth 'Valkyrie's Present' has been summoned here in Rachel by the Wish maiden.",
    )?;
    ctx.npc().do_event("Rhehe Ravies#gq_fes03::Oncall")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer3603000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "The final ceremony will be performed for 5 minutes.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer3608000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "Please come to Rachel to encounter the summoning ceremony.")?;
    ctx.end()
}

pub fn rhehe_ravies_gq_fes03_ontimer3900000(ctx: &Ctx) -> Script {
    announce_juno(ctx, "The entire ceremony is over now, I am sure all of you had fun.")?;
    ctx.npc().do_event("Rhehe Ravies#gq_fes03::Onover")?;
    ctx.end()
}
