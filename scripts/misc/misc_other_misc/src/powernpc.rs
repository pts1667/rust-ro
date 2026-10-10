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

#[derive(Clone, Copy, Debug)]
enum IronManGnpStep {
    Start,
    LLifting,
}

fn iron_man_gnp_run(ctx: &Ctx, mut step: IronManGnpStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            IronManGnpStep::Start => {
                if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
                    ctx.lines(args![
                        "- Wait a minute !! -",
                        "- Currently you're carrying -",
                        "- too many items with you. -",
                        "- Please try again -",
                        "- after you lose some weight. -"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("BaseJob").get()? == constants::JOB_NOVICE {
                    ctx.lines_as("Songmoodoo", args!["Children are not allowed in here."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("in_battle").get()? == 3 {
                    ctx.var("in_battle").set(Val::from(1))?;
                }
                if ctx.var("in_battle").get()?.number()? < 1 {
                    ctx.lines_as(
                        "Songmoodoo",
                        args![
                            "Hey hey~ This is not something",
                            "you can see everyday.",
                            "Oh~ you look strong!",
                            "Wanna try?"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Why not?", "?????", "No thanks."])? {
                        0 => {
                            ctx.lines_as(
                                "Songmoodoo",
                                args![
                                    "Haha~ I knew you would try.",
                                    "If you could lift this up,",
                                    "I'll tell you something interesting.",
                                    "Ready?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines(args!["^0000FFSongmoodoo points to a rock", "and asks you to lift it."])?;
                            if ctx.var("BaseJob").get()?.number()? < constants::JOB_KNIGHT {
                                iron_man_gnp_run(ctx, IronManGnpStep::LLifting, args![690])?;
                            } else if ctx.var("BaseClass").get()? == constants::JOB_SWORDMAN
                                || ctx.var("BaseClass").get()? == constants::JOB_THIEF
                                || ctx.var("BaseClass").get()? == constants::JOB_MERCHANT
                                || ctx.var("BaseJob").get()? == constants::JOB_MONK
                            {
                                iron_man_gnp_run(ctx, IronManGnpStep::LLifting, args![1100])?;
                            } else {
                                iron_man_gnp_run(ctx, IronManGnpStep::LLifting, args![730])?;
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Songmoodoo",
                                args![
                                    "That's not a big deal.",
                                    "If you believe you're strong,",
                                    "this might be a good chance to experience",
                                    "something new. Wanna try??"
                                ],
                            )?;
                        }
                        2 => {
                            ctx.lines_as("Songmoodoo", args!["I guess not..."])?;
                            ctx.call(Function::Emotion, args![constants::ET_HNG])?;
                        }
                        _ => {}
                    }
                } else if ctx.var("in_battle").get()? == 1 {
                    ctx.lines_as("Songmoodoo", args!["Good to see you again!", "Wanna go??"])?;
                    ctx.next()?;
                    if ctx.menu(&["Sure", "Maybe next time"])? == 0 {
                        ctx.lines_as(
                            "Songmoodoo",
                            args!["Good! Haha.", "I like your confidence.", "Good luck to you~"],
                        )?;
                        ctx.close_window()?;
                        ctx.warp("gon_test", 53, 6)?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as("Songmoodoo", args!["Well... alright.", "I'll see you next time then."])?;
                } else {
                    ctx.lines_as(
                        "Songmoodoo",
                        args![
                            "You ran away from there?",
                            "Guess you're not strong enough!",
                            "I'll give you another chance.",
                            "See you again."
                        ],
                    )?;
                    ctx.call(Function::Emotion, args![constants::ET_HNG])?;
                    ctx.var("in_battle").set(Val::from(1))?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            IronManGnpStep::LLifting => {
                let weight = runtime::arg(&args, 0, Val::from(0));
                if ctx.call(Function::CheckWeight, args![7049, weight.clone()])?.is_true() {
                    ctx.mes("You lifted the Stone lightly.^000000")?;
                    ctx.var("in_battle").set(Val::from(1))?;
                    ctx.call(Function::GetItem, args![7049, weight])?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, args![constants::ET_BEST])?;
                    ctx.lines_as(
                        "Songmoodoo",
                        args![
                            "Wow~ Excellent~",
                            "I'll take you to someplace nice",
                            "next time I see you.",
                            "See ya~"
                        ],
                    )?;
                } else {
                    ctx.mes("Looks too heavy for you.^000000")?;
                    ctx.next()?;
                    ctx.lines_as("Songmoodoo", args!["You lack training.", "Come back after more practice."])?;
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn iron_man_gnp(ctx: &Ctx) -> Script {
    iron_man_gnp_run(ctx, IronManGnpStep::Start, Vec::new()).map(|_| ())
}

pub fn administrator_gnp(ctx: &Ctx) -> Script {
    ctx.mes("[Administrator]")?;
    if ctx.var("BaseJob").get()? == constants::JOB_NOVICE {
        ctx.mes("Children are not allowed in here.")?;
        return ctx.close();
    }
    if ctx.var("$@in_battle").get()?.is_true() {
        ctx.lines(args!["Sorry, the field of fight", "is occupied right now.", "Try again later."])?;
        return ctx.close();
    }
    ctx.lines(args!["Are you ready?", " ", "Remember, you have to pay", "500z to fight."])?;
    ctx.next()?;
    if ctx.menu(&["Yes, let me fight!", "One moment, please."])? == 1 {
        ctx.lines_as("Administrator", args!["Ok, see you later."])?;
        return ctx.close();
    }
    if ctx.player().zeny()? < 500 {
        ctx.lines_as("Administrator", args!["I'm sorry but you don't have enough zeny."])?;
        return ctx.close();
    }
    ctx.player().set_zeny(ctx.player().zeny()? - 500)?;
    ctx.var("in_battle").set(Val::from(1))?;
    ctx.warp("gon_test", 42, 86)?;
    ctx.var("$@in_battle").set(Val::from(1))?;
    ctx.call(Function::SetNpcTimer, args![0, "Summoner#gnp"])?;
    ctx.call(Function::StartNpcTimer, args!["Summoner#gnp"])?;
    ctx.end()
}

#[derive(Clone, Copy, Debug)]
enum SummonerGnpStep {
    Start,
    OnInit,
    OnTimer120000,
    OnTimer180000,
    OnTimer182000,
    OnGnpMobDead,
    OnTimer184000,
}

// Index is group - 1. Every group has six monsters except the last, which has five.
const GNP_MOBS: [(&[&str], &[i32]); 11] = [
    (
        &[
            "Leather ribbon",
            "Sitotoxism",
            "Certificate of blood donation",
            "Tarantulla",
            "DangRangKwon",
            "Molar of Desert",
        ],
        &[1419, 1428, 1434, 1430, 1457, 1432],
    ),
    (
        &[
            "Hog Skeleton",
            "Cannibal Bear",
            "Miner",
            "Fighting Dog",
            "Mermaid Princess",
            "Only Son",
        ],
        &[1462, 1442, 1469, 1460, 1425, 1472],
    ),
    (
        &["SamYeupchoom", "Hunting Dog", "Nutcracker", "Sea Horse", "JAKK", "Corpse"],
        &[1454, 1455, 1443, 1426, 1436, 1423],
    ),
    (
        &["Marduk", "Onion Stem", "Worm", "Autodoll", "Girl with Matches", "Red Evil"],
        &[1458, 1440, 1429, 1459, 1444, 1422],
    ),
    (
        &["Naga", "Mold", "Tracing Missiles", "Aryong", "Abiryong", "Bacterium"],
        &[1421, 1481, 1424, 1465, 1466, 1433],
    ),
    (
        &[
            "Winning System",
            "Fat Archer",
            "Little black goat",
            "Perverted",
            "Treasure Box",
            "Greenhorn",
        ],
        &[1427, 1473, 1431, 1446, 1474, 1471],
    ),
    (
        &[
            "Hurricane",
            "External Hog",
            "Landlord of Maze",
            "Knight of grudge",
            "Archer of grudge",
            "Papillon",
        ],
        &[1450, 1439, 1461, 1467, 1453, 1479],
    ),
    (
        &["Lip", "Wendigo", "E Card", "Tentacle Monster", "Muscular Alarm", "Devil Cross"],
        &[1451, 1475, 1437, 1441, 1476, 1435],
    ),
    (
        &[
            "Maggot",
            "Large Frame",
            "Season of reading",
            "Shining Fingers",
            "Handbag",
            "Major knight of grudge",
        ],
        &[1477, 1448, 1478, 1489, 1488, 1438],
    ),
    (
        &[
            "Queen",
            "Man of Fire",
            "Sword of Executor",
            "Mutant Dragon",
            "Mixed Soup",
            "Great Sword",
        ],
        &[1482, 1464, 1487, 1449, 1456, 1486],
    ),
    (
        &["Monster Bird", "Torturer", "Warrior", "Vice-Torturer", "Huge Sword"],
        &[1447, 1483, 1490, 1484, 1485],
    ),
];

fn area_announce(ctx: &Ctx, text: &str) -> Result<(), Stop> {
    ctx.call(Function::AreaAnnounce, args!["gon_test", 41, 81, 74, 92, text, 0])?;
    Ok(())
}

fn summoner_gnp_run(ctx: &Ctx, mut step: SummonerGnpStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SummonerGnpStep::Start => {
                ctx.mes("[SongYeunWoo]")?;
                if ctx.var("in_battle").get()? == 3 {
                    ctx.mes("Please come back after registration.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("in_battle").get()? == 2 {
                    ctx.lines(args![
                        "You had a single match already.",
                        "You can have a match once at a time.",
                        "Please re-enter if you want a match",
                        "with other monsters."
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args!["Welcome.", "Which monster will you recall?"])?;
                ctx.next()?;
                let group = runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Group 1:Group 2:Group 3:Group 4:Group 5:Group 6:Group 7:Group 8:Group 9:Group 10:Group 11",
                    )],
                )?;
                let base = (group - 1) * 6;
                let mut mob_names = ctx.var(".gnpmobsname$").get_at(runtime::index(&Val::from(base))?)?;
                for i in 1..6 {
                    mob_names = mob_names + Val::from(":") + ctx.var(".gnpmobsname$").get_at(runtime::index(&Val::from(base + i))?)?;
                }
                let mob = runtime::select_values(ctx, &[mob_names])?;
                ctx.lines_as("SongYeunWoo", args!["Let the fight begin!"])?;
                ctx.close_window()?;
                ctx.var("in_battle").set(Val::from(2))?;
                if ctx.call(Function::GetNpcTimer, args![0])?.number()? < 180000 {
                    let slot = runtime::index(&Val::from((group - 1) * 6 + mob - 1))?;
                    ctx.call(
                        Function::Monster,
                        args![
                            "gon_test",
                            56,
                            86,
                            ctx.var(".gnpmobsname$").get_at(slot)?,
                            ctx.var(".gnpmobsid").get_at(slot)?,
                            1,
                            "Summoner#gnp::OnGnpMobDead"
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            SummonerGnpStep::OnInit => {
                ctx.call(Function::InitNpcTimer, args![])?;
                ctx.call(Function::StopNpcTimer, args![])?;
                for (group, (names, ids)) in GNP_MOBS.iter().enumerate() {
                    let base = group as i32 * 6;
                    for (i, &name) in names.iter().enumerate() {
                        ctx.var(".gnpmobsname$")
                            .set_at(runtime::index(&Val::from(base + i as i32))?, Val::from(name))?;
                    }
                    for (i, &id) in ids.iter().enumerate() {
                        ctx.var(".gnpmobsid")
                            .set_at(runtime::index(&Val::from(base + i as i32))?, Val::from(id))?;
                    }
                }
                return Err(Stop::End);
            }
            SummonerGnpStep::OnTimer120000 => {
                area_announce(ctx, "1 min. left")?;
                return Err(Stop::End);
            }
            SummonerGnpStep::OnTimer180000 => {
                ctx.call(Function::KillMonster, args!["gon_test", "Summoner#gnp::OnGnpMobDead"])?;
                return Err(Stop::End);
            }
            SummonerGnpStep::OnTimer182000 => {
                area_announce(ctx, "Time Over.")?;
                return Err(Stop::End);
            }
            SummonerGnpStep::OnGnpMobDead => {
                ctx.call(Function::SetNpcTimer, args![0])?;
                ctx.var("in_battle").set(Val::from(1))?;
                area_announce(ctx, "Thank you. Please come again.")?;
                ctx.call(Function::Sleep, args![4000])?;
                step = SummonerGnpStep::OnTimer184000;
                continue 'machine;
            }
            SummonerGnpStep::OnTimer184000 => {
                ctx.call(Function::StopNpcTimer, args![])?;
                ctx.call(Function::AreaWarp, args!["gon_test", 41, 81, 74, 92, "gon_test", 44, 4])?;
                ctx.var("$@in_battle").set(Val::from(0))?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn summoner_gnp(ctx: &Ctx) -> Script {
    summoner_gnp_run(ctx, SummonerGnpStep::Start, Vec::new()).map(|_| ())
}

pub fn summoner_gnp_oninit(ctx: &Ctx) -> Script {
    summoner_gnp_run(ctx, SummonerGnpStep::OnInit, Vec::new()).map(|_| ())
}

pub fn summoner_gnp_ontimer120000(ctx: &Ctx) -> Script {
    summoner_gnp_run(ctx, SummonerGnpStep::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn summoner_gnp_ontimer180000(ctx: &Ctx) -> Script {
    summoner_gnp_run(ctx, SummonerGnpStep::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn summoner_gnp_ontimer182000(ctx: &Ctx) -> Script {
    summoner_gnp_run(ctx, SummonerGnpStep::OnTimer182000, Vec::new()).map(|_| ())
}

pub fn summoner_gnp_ongnpmobdead(ctx: &Ctx) -> Script {
    summoner_gnp_run(ctx, SummonerGnpStep::OnGnpMobDead, Vec::new()).map(|_| ())
}

pub fn summoner_gnp_ontimer184000(ctx: &Ctx) -> Script {
    summoner_gnp_run(ctx, SummonerGnpStep::OnTimer184000, Vec::new()).map(|_| ())
}

pub fn guide_of_field_of_fight(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "SongHeeYeon",
        args![".....", "Hi, there~", "This is a field of fight.", "Got any questions?"],
    )?;
    ctx.next()?;
    match ctx.menu(&["A field of fight?", "You got a minute lady?", "Get in.", "Out.", "Nope."])? {
        0 => {
            ctx.lines_as(
                "SongHeeYeon",
                args![
                    "Just like the name of this place,",
                    "it is a field for matches.",
                    "We have various monsters",
                    "in different levels."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "SongHeeYeon",
                args![
                    "1st Class Boss Monsters are in middle of preparation yet.",
                    "We charge you a small fee to enter here."
                ],
            )?;
        }
        1 => {
            ctx.lines_as(
                "SongHeeYeon",
                args!["Eh.... excuse me?", "Ah, I'm afraid I have to work right now...", "Sorry..."],
            )?;
        }
        2 => {
            ctx.lines_as("SongHeeYeon", args!["Yes, thank you.", "Have a good time."])?;
            ctx.close_window()?;
            ctx.var("in_battle").set(Val::from(3))?;
            ctx.warp("gon_test", 25, 98)?;
            return ctx.end();
        }
        3 => {
            ctx.lines_as("SongHeeYeon", args!["Thank you.", "Please come again."])?;
            ctx.close_window()?;
            ctx.warp("gonryun", 177, 112)?;
            return ctx.end();
        }
        _ => {
            ctx.lines_as("SongHeeYeon", args!["........", "Goodbye..."])?;
        }
    }
    ctx.close()
}

pub fn chowanan_gnp(ctx: &Ctx) -> Script {
    ctx.lines_as("ChowAnAn", args!["Want to go back?"])?;
    ctx.next()?;
    if ctx.menu(&["Yes.", "No."])? == 0 {
        ctx.lines_as("ChowAnAn", args!["Thank you.", "Please come again."])?;
        ctx.close_window()?;
        ctx.var("in_battle").set(Val::from(1))?;
        ctx.warp("gon_test", 44, 4)?;
        return ctx.end();
    }
    ctx.lines_as("ChowAnAn", args!["Thank you."])?;
    ctx.close()
}
