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

pub fn grunburti_1(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Dwarf Grunburti",
        args!["A human?!", "This land is full", "of your kind. What", "brings you here?"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Dwarf Grunburti",
        args![
            "Living in Midgard and away from",
            "my home town is painful enough, let alone facing a whiny human. Leave me alone!"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&[
        "Ask him about Dwarves.",
        "Ask him what he is doing.",
        "Request weapon creation.",
        "Cancel.",
    ])? {
        0 => {
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "What's to know about Dwarves?",
                    "We're the toughest race. After all, we can live anywhere, no matter",
                    "how harsh the environment may be."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "I doubt your feeble mind can comprehend the how unendurable",
                    "it must be for you to live in my homeland, but you can try.",
                    "Try to imagine."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dwarf Grunburti",
                args!["Most of the knowledge and skills that you humans are so proud of were probably handed down from Dwarves."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "This is especially true in the",
                    "case of smithing. Even the tools and weapons of the gods were made by my ancestors. You know that?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "We have the greatest knowlege and skills, but have accepted our fate to live in the cold, barren lands of the Giants."
                ],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "Did you just ask what I am doing here? Mwahahaha! Waiting for any humans stupid enough to come",
                    "here so I can kick their ass!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Dwarf Grunburti", args!["Oh don't worry. I'd only humiliate myself by fighting with a weakling like you. Humans are so fragile, but they stubbornly cling to their arrogance and fight amongst", "each other."])?;
            ctx.next()?;
            ctx.lines_as("Dwarf Grunburti", args!["As a race, your people are just hopeless. I wouldn't even share the same continent with a human! But alas, I have no choice."])?;
            ctx.next()?;
            ctx.lines_as("Dwarf Grunburti", args!["The path to my hometown", "disappeared after the war 1,000 years ago. Somehow, I can no longer find the way through the Yggdrasil Tree that will take me back..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "I'm stuck here in Midgard.",
                    "At least I was fortunate enough to find this cave. We Dwarves are most cozy living underground, after all."
                ],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.call(Function::GetCharacterId, args![2])?;
            if runtime::op(&ctx.var("$god1").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                || runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                || runtime::op(&ctx.var("$god3").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                || runtime::op(&ctx.var("$god4").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
            {
                if runtime::op(&ctx.var("$god1").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true()
                    && runtime::op(&ctx.var("$god2").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true()
                    && runtime::op(&ctx.var("$god3").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true()
                    && runtime::op(&ctx.var("$god4").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true()
                    && ctx.call(Function::GetGuildInfo, args![ctx.call(Function::GetCharacterId, args![2])?, 2])? == 1
                {
                    ctx.lines_as(
                        "Dwarf Grunburti",
                        args![
                            "Hmm...",
                            "I'll need some things to make a weapon for you. What exactly were you interested in having?"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Brisingamen", "Megingjard", "Sleipnir", "Mjolnir"])? {
                        0 => {
                            ctx.lines_as(
                                "Dwarf Grunburti",
                                args![
                                    "I will need...",
                                    "^0000FF4 Freya's Jewel",
                                    "4 Silver Ornament",
                                    "3 Snow Crystal",
                                    "3 Ripple",
                                    "3 Drifting Air",
                                    "2 Sapphire",
                                    "3 Pearl",
                                    "10 Opal",
                                    "5 Cursed Ruby",
                                    "20 Gold^000000",
                                    "1 Necklace^000000..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Dwarf Grunburti", args!["However, the time for me to create this Brisingamen has not yet come. You'll have to wait until the seals are released. Mwahahaha!"])?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as(
                                "Dwarf Grunburti",
                                args![
                                    "I will need...",
                                    "^0000FF1 Gleipnir",
                                    "20 Gold",
                                    "10 Sapphire",
                                    "10 Oridecon",
                                    "1 Belt^000000"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Dwarf Grunburti", args!["However, the seals on Megingjard have not yet been released. Until then, you'll have to wait! Bwahahaha!"])?;
                            return ctx.close();
                        }
                        2 => {
                            ctx.lines_as(
                                "Dwarf Grunburti",
                                args![
                                    "I will need...",
                                    "^0000FF3 Wheel of the Unknown",
                                    "5 Feather of Angel Wing",
                                    "3 Sprit of Fish",
                                    "4 Amblem of the Sun God",
                                    "3 Breath of Spirit",
                                    "20 Gold",
                                    "10 Elunium",
                                    "1 Slotted Boots^000000"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Dwarf Grunburti", args!["But even so, I can't create Sleipnir until the seals have been broken. Otherwise, you're out of luck, human. Hahaha~!"])?;
                            return ctx.close();
                        }
                        3 => {
                            ctx.lines_as(
                                "Dwarf Grunburti",
                                args![
                                    "I will need...",
                                    "^0000FF2 Thor's Gauntlets",
                                    "4 Iron Maiden",
                                    "5 Wrath of Valkyrie",
                                    "5 Omen of Tempest",
                                    "5 Billow",
                                    "20 Oridecon",
                                    "5 Elunium",
                                    "40 Gold",
                                    "1 Stunner^000000"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Dwarf Grunburti", args!["But I can't even create a replica of Mjolnir if the seals are still in place. Until they're released, you'll just have to wait. Bwahaha!"])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                ctx.lines_as(
                    "Dwarf Grunburti",
                    args!["We Dwarves have too much pride to demonstrate our skills in front of a lowly human!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dwarf Grunburti",
                    args!["Stop bothering me and get out of here! Go play with some monsters, you simple minded fool!"],
                )?;
                return ctx.close();
            }
            if ctx.call(Function::GetGuildInfo, args![ctx.call(Function::GetCharacterId, args![2])?, 2])? == 0 {
                ctx.lines_as(
                    "Dwarf Grunburti",
                    args![
                        "I'll only present",
                        "my magnificent skills",
                        "to a human of incredibly",
                        "high standing!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dwarf Grunburti",
                    args![
                        "Although I hate",
                        "most humans, I have",
                        "no choice but to respect the ones chosen by destiny. Go and bring your ^0000FFguildmaster^000000!"
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Dwarf Grunburti",
                args!["Although it is very humiliating to present my valuable skills to a human being..."],
            )?;
            ctx.next()?;
            ctx.lines_as("Dwarf Grunburti", args!["All the seals have been released, and he who holds the Emperium is chosen by destiny. I have no choice but to respect you."])?;
            ctx.next()?;
            ctx.lines_as("Dwarf Grunburti", args!["First of all, I shall confirm whether or not you have brought all the necessary materials to forge a godly item! I hate it when humans come here without any purpose!"])?;
            ctx.next()?;
            if ctx.items().count(7073)? > 3
                && ctx.items().count(7077)? > 3
                && ctx.items().count(7088)? > 2
                && ctx.items().count(7090)? > 2
                && ctx.items().count(7092)? > 2
                && ctx.items().count(726)? > 1
                && ctx.items().count(722)? > 2
                && ctx.items().count(727)? > 9
                && ctx.items().count(724)? > 4
                && ctx.items().count(969)? > 19
                && ctx.items().count(2603)? > 0
            {
                ctx.lines_as(
                    "Dwarf Grunburti",
                    args!["Hmm...", "I guess you", "want to have", "^0000FFBrisingamen^000000!"],
                )?;
                ctx.next()?;
            } else if ctx.items().count(7058)? > 0
                && ctx.items().count(969)? > 19
                && ctx.items().count(726)? > 9
                && ctx.items().count(984)? > 9
                && ctx.items().count(2627)? > 0
            {
                ctx.lines_as(
                    "Dwarf Grunburti",
                    args!["Hmm...", "I guess you", "want to have", "^0000FFMegingjard^000000!"],
                )?;
                ctx.next()?;
            } else if ctx.items().count(7076)? > 2
                && ctx.items().count(7079)? > 4
                && ctx.items().count(7083)? > 2
                && ctx.items().count(7086)? > 3
                && ctx.items().count(7087)? > 2
                && ctx.items().count(969)? > 19
                && ctx.items().count(985)? > 9
                && ctx.items().count(2406)? > 0
            {
                ctx.lines_as(
                    "Dwarf Grunburti",
                    args!["Hmm...", "I guess you", "want to have", "^0000FFSleipnir^000000!"],
                )?;
                ctx.next()?;
            } else if ctx.items().count(7074)? > 1
                && ctx.items().count(7075)? > 3
                && ctx.items().count(7078)? > 4
                && ctx.items().count(7089)? > 4
                && ctx.items().count(7091)? > 4
                && ctx.items().count(984)? > 19
                && ctx.items().count(985)? > 4
                && ctx.items().count(969)? > 39
                && ctx.items().count(1522)? > 0
            {
                ctx.lines_as(
                    "Dwarf Grunburti",
                    args!["Hmm...", "I guess you", "want to have", "^0000FFMjolnir^000000!"],
                )?;
                ctx.next()?;
            } else {
                ctx.lines_as("Dwarf Grunburti", args!["...", "......"])?;
                ctx.next()?;
                ctx.lines_as("Dwarf Grunburti", args!["...", "......", "........."])?;
                ctx.next()?;
                ctx.lines_as("Dwarf Grunburti", args!["...", "......", ".........", "............."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Dwarf Grunburti",
                    args!["How dare you come here unprepared?! Did you forget what you needed to bring? Listen carefully this time!"],
                )?;
                ctx.next()?;
                match ctx.menu(&["Brisingamen", "Megingjard", "Sleipnir", "Mjolnir"])? {
                    0 => {
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "I will need...",
                                "^0000FF4 Freya's Jewel",
                                "4 Silver Ornament",
                                "3 Snow Crystal",
                                "3 Ripple",
                                "3 Drifting Air",
                                "2 Sapphire",
                                "3 Pearl",
                                "10 Opal",
                                "5 Cursed Ruby",
                                "20 Gold^000000",
                                "1 Necklace^000000..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "Hmmm, but now",
                                "that the seals have",
                                "been released, you must hurry before another human can claim",
                                "one of the godly items..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args!["Hurry...!", "Once I forge", "Brisingamen,", "the seals will", "activate again!"],
                        )?;
                        return ctx.close();
                    }
                    1 => {
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "I will need...",
                                "^0000FF1 Gleipnir",
                                "20 Gold",
                                "10 Sapphire",
                                "10 Oridecon",
                                "1 Belt^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "Hmmm, but now",
                                "that the seals have",
                                "been released, you must hurry before another human can claim",
                                "one of the godly items..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args!["Hurry...!", "Once I forge", "Megingjard,", "the seals will", "activate again!"],
                        )?;
                        return ctx.close();
                    }
                    2 => {
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "I will need...",
                                "^0000FF3 Wheel of the Unknown",
                                "5 Feather of Angel Wing",
                                "3 Sprit of Fish",
                                "4 Amblem of the Sun God",
                                "3 Breath of Spirit",
                                "20 Gold",
                                "10 Elunium",
                                "1 Slotted Boots^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "Hmmm, but now",
                                "that the seals have",
                                "been released, you must hurry before another human can claim",
                                "one of the godly items..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args!["Hurry...!", "Once I forge", "Sleipnir,", "the seals will", "activate again!"],
                        )?;
                        return ctx.close();
                    }
                    3 => {
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "I will need...",
                                "^0000FF2 Thor's Gauntlets",
                                "4 Iron Maiden",
                                "5 Wrath of Valkyrie",
                                "5 Omen of Tempest",
                                "5 Billow",
                                "20 Oridecon",
                                "5 Elunium",
                                "40 Gold"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "Hmmm, but now",
                                "that the seals have",
                                "been released, you must hurry before another human can claim",
                                "one of the godly items..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args!["Hurry...!", "Once I forge", "the Mjolnir,", "the seals will", "activate again!"],
                        )?;
                        return ctx.close();
                    }
                    _ => {}
                }
            }
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "Hmpf.",
                    "I can't get any work done in here. Come with me to the ^0000FFunderground laboratory^000000 as my guest, human."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Dwarf Grunburti", args!["Now hurry up! ^0000FFOnly one of each godly item^000000 will be given to the humans ^FF0000right after all the seals are released^000000!"])?;
            ctx.close_window()?;
            ctx.warp("que_god01", 214, 63)?;
            return ctx.end();
        }
        3 => {
            ctx.lines_as(
                "Dwarf Grunburti",
                args!["Muhahahahahaha!", "Out of my sight,", "you dirty, filthy", "human...!"],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn god_hopewarp1(ctx: &Ctx) -> Script {
    return ctx.end();
}

pub fn god_hopewarp1_oninit(ctx: &Ctx) -> Script {
    ctx.call(
        Function::WaitingRoom,
        args!["Laboratory Entrance", 2, "#god_hopewarp1::OnStartArena", 1],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, args![])?;
    return ctx.end();
}

pub fn god_hopewarp1_onstartarena(ctx: &Ctx) -> Script {
    ctx.call(Function::WarpWaitingPc, args!["que_god01", 155, 63])?;
    ctx.npc().do_event("Grunburti#god::OnEnable")?;
    ctx.call(Function::DisableWaitingRoomEvent, args![])?;
    return ctx.end();
}

pub fn god_hopewarp1_onreset(ctx: &Ctx) -> Script {
    ctx.call(Function::EnableWaitingRoomEvent, args![])?;
    return ctx.end();
}

pub fn que_godnpc1(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Entrance Notice",
        args!["Only the most", "worthy of humans", "will possess the", "power of the gods."],
    )?;
    return ctx.close();
}

pub fn grunburti_god(ctx: &Ctx) -> Script {
    if runtime::op(&ctx.var("$god1").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
        || runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
        || runtime::op(&ctx.var("$god3").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
        || runtime::op(&ctx.var("$god4").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
    {
        ctx.lines_as(
            "Dwarf Grunburti",
            args![
                "One of the godly",
                "items has been created,",
                "and the seals have been",
                "restored. You'll have to",
                "wait until they're all",
                "released again...!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dwarf Grunburti",
            args!["Bwahahahahahahaa!", "Even though you're", "just a human, I feel", "pity for you~"],
        )?;
        return ctx.close();
    }
    let l_gid = ctx.call(Function::GetCharacterId, args![2])?;
    if ctx.call(Function::GetGuildInfo, args![ctx.call(Function::GetCharacterId, args![2])?, 2])? == 0 {
        ctx.lines_as("Dwarf Grunburti", args!["How in the...", "Get out of here!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Dwarf Grunburti",
            args![
                "How did one of you",
                "stupid humans get in",
                "here?! Only those who",
                "hold the Emperium can",
                "even think of entering",
                "this place...!"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Dwarf Grunburti", args!["It's incredibly", "humiliating to do work for a human. But since destiny has chosen you as the bearer of an Emperium, I have no choice but to oblige your requests."])?;
    ctx.next()?;
    ctx.lines_as("Dwarf Grunburti", args!["We only have", "^FF000010 minutes^000000 to recreate one godly treasure. After that, the seals will restore themselves and I won't be able to create anything until they're released again..."])?;
    ctx.next()?;
    ctx.lines_as(
        "Dwarf Grunburti",
        args![
            "^FF0000Don't be too slow^000000,",
            "otherwise ^FF0000I will give",
            "the chance to another",
            "human^000000 right away!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Dwarf Grunburti",
        args!["Now...!", "Choose the item", "you wish for me", "to create!"],
    )?;
    ctx.next()?;
    match ctx.menu(&["Brisingamen", "Megingjard", "Sleipnir", "Mjolnir"])? {
        0 => {
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "B-Brisingamen?!",
                    "I've never expected",
                    "such insolence...!",
                    "This necklace will never have any meaning in my eyes unless it's worn by the goddess Freya."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "But who am I to judge your worthiness? These treasures",
                    "select their owners with their own will. Let's see if Brisingamen will find you worthy!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "Once again, I need following materials in order to reproduce",
                    "this godly treasure..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "^0000FF4 Freya's Jewel",
                    "4 Silver Ornament",
                    "3 Snow Crystal",
                    "3 Ripple",
                    "3 Drifting Air",
                    "2 Sapphire",
                    "3 Pearl",
                    "10 Opal",
                    "5 Cursed Ruby",
                    "20 Gold",
                    "1 Necklace^000000"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Make Brisingamen.", "Cancel."])? {
                0 => {
                    if ctx.items().count(7073)? > 3
                        && ctx.items().count(7077)? > 3
                        && ctx.items().count(7088)? > 2
                        && ctx.items().count(7090)? > 2
                        && ctx.items().count(7092)? > 2
                        && ctx.items().count(726)? > 1
                        && ctx.items().count(722)? > 2
                        && ctx.items().count(727)? > 9
                        && ctx.items().count(724)? > 4
                        && ctx.items().count(969)? > 19
                        && ctx.items().count(2603)? > 0
                    {
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "Never in my wildest",
                                "imaginings have I thought that I'd be crafting this masterpiece for a mere human. Give me a moment."
                            ],
                        )?;
                        ctx.next()?;
                        if runtime::op(&ctx.var("$god1").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                            || runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                            || runtime::op(&ctx.var("$god3").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                            || runtime::op(&ctx.var("$god4").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                        {
                            ctx.lines_as(
                                "Dwarf Grunburti",
                                args![
                                    "But...",
                                    "The seals",
                                    "have just been",
                                    "restored. You'll have to",
                                    "wait until they're all",
                                    "released again...!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Dwarf Grunburti",
                                args!["Bwahahahahahahaa!", "Even though you're", "just a human, I feel", "pity for you~"],
                            )?;
                            return ctx.close();
                        }
                        ctx.items().take(7073, 4)?;
                        ctx.items().take(7077, 4)?;
                        ctx.items().take(7088, 3)?;
                        ctx.items().take(7090, 3)?;
                        ctx.items().take(7092, 3)?;
                        ctx.items().take(726, 2)?;
                        ctx.items().take(722, 3)?;
                        ctx.items().take(727, 10)?;
                        ctx.items().take(724, 5)?;
                        ctx.items().take(969, 20)?;
                        ctx.items().take(2603, 1)?;
                        ctx.items().give(2630, 1)?;
                        ctx.var("$god1").set(Val::from(0))?;
                        ctx.var("$god2").set(Val::from(0))?;
                        ctx.var("$god3").set(Val::from(0))?;
                        ctx.var("$god4").set(Val::from(0))?;
                        ctx.call(
                            Function::Announce,
                            args![
                                Val::from("[Brisingamen] has come into the hands of [")
                                    + ctx.player().name()?
                                    + "], master of the ["
                                    + ctx.call(Function::GetGuildInfo, args![l_gid.clone(), 0])?
                                    + "] guild.",
                                constants::BC_ALL
                            ],
                        )?;
                        ctx.lines_as("Dwarf Grunburti", args!["Ah, just look at this dazzling beauty. No other piece of jewelry complemented Freya as well as Brisingamen."])?;
                        return ctx.close();
                    } else {
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "Idiot human!",
                                "You didn't bring",
                                "everything I need to",
                                "recreate Brisingamen!",
                                "Hurry...!"
                            ],
                        )?;
                        return ctx.close();
                    }
                }
                1 => {
                    ctx.lines_as(
                        "Dwarf Grunburti",
                        args!["Muhahahaha~", "Somehow, I figured", "you'd back out, human!"],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        1 => {
            ctx.lines_as("Dwarf Grunburti", args!["M- Megingjard?!", "The girdle of might?!"])?;
            ctx.next()?;
            ctx.lines_as("Dwarf Grunburti", args!["This belt was worn long ago by Thor, the mightest warrior ever. But since these treasures select their owners, we'll see whether or not it recognizes you as worthy!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "Once again, I need following materials in order to reproduce",
                    "this godly treasure..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dwarf Grunburti",
                args!["^0000FF1 Gleipnir", "20 Gold", "10 Sapphire", "10 Oridecon", "1 Belt^000000..."],
            )?;
            ctx.next()?;
            match ctx.menu(&["Make Megingjard.", "Cancel."])? {
                0 => {
                    if ctx.items().count(7058)? > 0
                        && ctx.items().count(969)? > 19
                        && ctx.items().count(726)? > 9
                        && ctx.items().count(984)? > 9
                        && ctx.items().count(2627)? > 0
                    {
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "Twenty years ago,",
                                "I'd never believe that",
                                "something so powerful and dangerous as Megingjard would end up in the hands of a human..."
                            ],
                        )?;
                        ctx.next()?;
                        if runtime::op(&ctx.var("$god1").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                            || runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                            || runtime::op(&ctx.var("$god3").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                            || runtime::op(&ctx.var("$god4").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                        {
                            ctx.lines_as(
                                "Dwarf Grunburti",
                                args![
                                    "But...",
                                    "The seals",
                                    "have just been",
                                    "restored. You'll have to",
                                    "wait until they're all",
                                    "released again...!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Dwarf Grunburti",
                                args!["Bwahahahahahahaa!", "Even though you're", "just a human, I feel", "pity for you~"],
                            )?;
                            return ctx.close();
                        }
                        ctx.items().take(7058, 1)?;
                        ctx.items().take(969, 20)?;
                        ctx.items().take(726, 10)?;
                        ctx.items().take(984, 10)?;
                        ctx.items().take(2627, 1)?;
                        ctx.items().give(2629, 1)?;
                        ctx.var("$god1").set(Val::from(0))?;
                        ctx.var("$god2").set(Val::from(0))?;
                        ctx.var("$god3").set(Val::from(0))?;
                        ctx.var("$god4").set(Val::from(0))?;
                        ctx.call(
                            Function::Announce,
                            args![
                                Val::from("[Megingjard] the godly item has been given to [")
                                    + ctx.player().name()?
                                    + "], the master of the guild ["
                                    + ctx.call(Function::GetGuildInfo, args![l_gid.clone(), 0])?
                                    + "].",
                                constants::BC_ALL
                            ],
                        )?;
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "Here...",
                                "Be careful with how",
                                "you use the strength",
                                "of a god. Just a fair",
                                "warning, human..."
                            ],
                        )?;
                        return ctx.close();
                    } else {
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "Idiot human!",
                                "You didn't bring",
                                "everything I need to",
                                "recreate Megingjard!",
                                "Hurry...!"
                            ],
                        )?;
                        return ctx.close();
                    }
                }
                1 => {
                    ctx.lines_as(
                        "Dwarf Grunburti",
                        args!["Muhahahaha~", "Somehow, I figured", "you'd back out, human!"],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        2 => {
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "Sleipnir...",
                    "Now, understand",
                    "that I can't create",
                    "the eight-legged stallion",
                    "of legend. However..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "I can create a pair",
                    "of shoes that will possess the power of Sleipnir. It's a strange, but feasible process..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "Once again, I need following materials in order to reproduce",
                    "this godly treasure..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "^0000FF3 Wheel of the Unknown",
                    "5 Feather of Angel Wing",
                    "3 Sprit of Fish",
                    "4 Amblem of the Sun God",
                    "3 Breath of Spirit",
                    "20 Gold",
                    "10 Elunium",
                    "1 Slotted Boots^000000..."
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Make Sleipnir.", "Cancel."])? {
                0 => {
                    if ctx.items().count(7076)? > 2
                        && ctx.items().count(7079)? > 4
                        && ctx.items().count(7083)? > 2
                        && ctx.items().count(7086)? > 3
                        && ctx.items().count(7087)? > 2
                        && ctx.items().count(969)? > 19
                        && ctx.items().count(985)? > 9
                        && ctx.items().count(2406)? > 0
                    {
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "I never believed",
                                "that the speed of",
                                "Sleipnir would be",
                                "used by a human.",
                                "Give me a moment..."
                            ],
                        )?;
                        ctx.next()?;
                        if runtime::op(&ctx.var("$god1").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                            || runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                            || runtime::op(&ctx.var("$god3").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                            || runtime::op(&ctx.var("$god4").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                        {
                            ctx.lines_as(
                                "Dwarf Grunburti",
                                args![
                                    "But...",
                                    "The seals",
                                    "have just been",
                                    "restored. You'll have to",
                                    "wait until they're all",
                                    "released again...!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Dwarf Grunburti",
                                args!["Bwahahahahahahaa!", "Even though you're", "just a human, I feel", "pity for you~"],
                            )?;
                            return ctx.close();
                        }
                        ctx.items().take(7076, 3)?;
                        ctx.items().take(7079, 5)?;
                        ctx.items().take(7083, 3)?;
                        ctx.items().take(7086, 4)?;
                        ctx.items().take(7087, 3)?;
                        ctx.items().take(969, 20)?;
                        ctx.items().take(985, 10)?;
                        ctx.items().take(2406, 1)?;
                        ctx.items().give(2410, 1)?;
                        ctx.var("$god1").set(Val::from(0))?;
                        ctx.var("$god2").set(Val::from(0))?;
                        ctx.var("$god3").set(Val::from(0))?;
                        ctx.var("$god4").set(Val::from(0))?;
                        ctx.call(
                            Function::Announce,
                            args![
                                Val::from("[Sleipnir] the godly item has been given to [")
                                    + ctx.player().name()?
                                    + "], the master of the guild ["
                                    + ctx.call(Function::GetGuildInfo, args![l_gid.clone(), 0])?
                                    + "].",
                                constants::BC_ALL
                            ],
                        )?;
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args!["There...", "Wear these, and", "move with the speed of", "the legendary Sleipnir..."],
                        )?;
                        return ctx.close();
                    } else {
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "Idiot human!",
                                "You didn't bring",
                                "everything I need to",
                                "recreate Sleipnir!",
                                "Hurry...!"
                            ],
                        )?;
                        return ctx.close();
                    }
                }
                1 => {
                    ctx.lines_as(
                        "Dwarf Grunburti",
                        args!["Muhahahaha~", "Somehow, I figured", "you'd back out, human!"],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        3 => {
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "M-M-Mjolnir!?",
                    "Even though the",
                    "moment is at hand,",
                    "I can scarcely believe..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "^333333*Sigh...*^000000",
                    "This is almost a disgrace to gods and the Dwarf race. But perhaps, wielding Mjolnir may be your destiny..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "Once again, I need following materials in order to reproduce",
                    "this godly treasure..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dwarf Grunburti",
                args![
                    "^0000FF2 Thor's Gauntlets",
                    "4 Iron Maiden",
                    "5 Wrath of Valkyrie",
                    "5 Omen of Tempest",
                    "5 Billow",
                    "20 Oridecon",
                    "5 Elunium",
                    "40 Gold",
                    "1 Stunner^000000..."
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Make Mjolnir.", "Cancel."])? {
                0 => {
                    if ctx.items().count(7074)? > 1
                        && ctx.items().count(7075)? > 3
                        && ctx.items().count(7078)? > 4
                        && ctx.items().count(7089)? > 4
                        && ctx.items().count(7091)? > 4
                        && ctx.items().count(984)? > 19
                        && ctx.items().count(985)? > 4
                        && ctx.items().count(969)? > 39
                        && ctx.items().count(1522)? > 0
                    {
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "Do not disgrace",
                                "Thor, lord of Thunder,",
                                "or you will regret it.",
                                "Mark my words..."
                            ],
                        )?;
                        ctx.next()?;
                        if runtime::op(&ctx.var("$god1").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                            || runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                            || runtime::op(&ctx.var("$god3").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                            || runtime::op(&ctx.var("$god4").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true()
                        {
                            ctx.lines_as(
                                "Dwarf Grunburti",
                                args![
                                    "But...",
                                    "The seals",
                                    "have just been",
                                    "restored. You'll have to",
                                    "wait until they're all",
                                    "released again...!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Dwarf Grunburti",
                                args!["Bwahahahahahahaa!", "Even though you're", "just a human, I feel", "pity for you~"],
                            )?;
                            return ctx.close();
                        }
                        ctx.items().take(7074, 2)?;
                        ctx.items().take(7075, 4)?;
                        ctx.items().take(7078, 5)?;
                        ctx.items().take(7089, 5)?;
                        ctx.items().take(7091, 5)?;
                        ctx.items().take(984, 20)?;
                        ctx.items().take(985, 5)?;
                        ctx.items().take(969, 40)?;
                        ctx.items().take(1522, 1)?;
                        ctx.items().give(1530, 1)?;
                        ctx.var("$god1").set(Val::from(0))?;
                        ctx.var("$god2").set(Val::from(0))?;
                        ctx.var("$god3").set(Val::from(0))?;
                        ctx.var("$god4").set(Val::from(0))?;
                        ctx.call(
                            Function::Announce,
                            args![
                                Val::from("[Mjolnir] has been bestowed to [")
                                    + ctx.player().name()?
                                    + "], the master of the ["
                                    + ctx.call(Function::GetGuildInfo, args![l_gid.clone(), 0])?
                                    + "] guild.",
                                constants::BC_ALL
                            ],
                        )?;
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "It's done.",
                                "Take it. How does",
                                "it feel to hold the world's most powerful weapon in your grasp?"
                            ],
                        )?;
                        return ctx.close();
                    } else {
                        ctx.lines_as(
                            "Dwarf Grunburti",
                            args![
                                "Idiot human!",
                                "You didn't bring",
                                "everything I need",
                                "to recreate Mjolnir!",
                                "Hurry...!"
                            ],
                        )?;
                        return ctx.close();
                    }
                }
                1 => {
                    ctx.lines_as(
                        "Dwarf Grunburti",
                        args!["Muhahahaha~", "Somehow, I figured", "you'd back out, human!"],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        _ => {}
    }
    return ctx.end();
}

pub fn grunburti_god_onenable(ctx: &Ctx) -> Script {
    ctx.call(Function::InitNpcTimer, args![])?;
    return ctx.end();
}

pub fn grunburti_god_ontimer10000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            "que_god01",
            "Remember that you only have 10 minutes for this! Hurry up!",
            constants::BC_MAP
        ],
    )?;
    return ctx.end();
}

pub fn grunburti_god_ontimer610000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            "que_god01",
            "You're too slow! I'm going to give another human a chance! Next!",
            constants::BC_MAP
        ],
    )?;
    return ctx.end();
}

pub fn grunburti_god_ontimer612000(ctx: &Ctx) -> Script {
    ctx.npc().do_event("god_wep_warpmaster::OnEnable")?;
    return ctx.end();
}

pub fn grunburti_god_ontimer615000(ctx: &Ctx) -> Script {
    ctx.npc().do_event("god_wep_warpmaster::OnDisable")?;
    ctx.npc().do_event("#god_hopewarp1::OnReset")?;
    ctx.call(Function::StopNpcTimer, args![])?;
    return ctx.end();
}

#[derive(Clone, Copy, Debug)]
enum GodWepWarpmasterStep {
    Start,
    OnEnable,
    OnDisable,
}

fn god_wep_warpmaster_run(ctx: &Ctx, mut step: GodWepWarpmasterStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GodWepWarpmasterStep::Start => {
                step = GodWepWarpmasterStep::OnEnable;
                continue 'machine;
            }
            GodWepWarpmasterStep::OnEnable => {
                for i in 1..=6_i32 {
                    ctx.call(Function::EnableNpc, args![Val::from("god_failwarp#") + i])?;
                }
                return Err(Stop::End);
            }
            GodWepWarpmasterStep::OnDisable => {
                for i in 1..=6_i32 {
                    ctx.call(Function::DisableNpc, args![Val::from("god_failwarp#") + i])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn god_wep_warpmaster(ctx: &Ctx) -> Script {
    god_wep_warpmaster_run(ctx, GodWepWarpmasterStep::Start, Vec::new()).map(|_| ())
}

pub fn god_wep_warpmaster_onenable(ctx: &Ctx) -> Script {
    god_wep_warpmaster_run(ctx, GodWepWarpmasterStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn god_wep_warpmaster_ondisable(ctx: &Ctx) -> Script {
    god_wep_warpmaster_run(ctx, GodWepWarpmasterStep::OnDisable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GodFailwarp1Step {
    Start,
    OnInit,
    OnTouch,
}

fn god_failwarp_1_run(ctx: &Ctx, mut step: GodFailwarp1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GodFailwarp1Step::Start => {
                step = GodFailwarp1Step::OnInit;
                continue 'machine;
            }
            GodFailwarp1Step::OnInit => {
                ctx.call(Function::DisableNpc, args![])?;
                return Err(Stop::End);
            }
            GodFailwarp1Step::OnTouch => {
                ctx.warp("prontera", 156, 324)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn god_failwarp_1(ctx: &Ctx) -> Script {
    god_failwarp_1_run(ctx, GodFailwarp1Step::Start, Vec::new()).map(|_| ())
}

pub fn god_failwarp_1_oninit(ctx: &Ctx) -> Script {
    god_failwarp_1_run(ctx, GodFailwarp1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn god_failwarp_1_ontouch(ctx: &Ctx) -> Script {
    god_failwarp_1_run(ctx, GodFailwarp1Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn godly_item_quests_god(ctx: &Ctx) -> Script {
    shared::other_gm_npcs::f_gm_npc(ctx, args![])?;
    ctx.lines_as(
        "Use in case of emergency",
        args!["Please enter password.", "If you wish to cancel, please enter 0."],
    )?;
    ctx.next()?;
    let l_i = shared::other_gm_npcs::f_gm_npc(ctx, args![1854, 0, 0, 4000])?;
    if l_i == -2 {
        ctx.lines_as("Use in case of emergency", args!["Password is incorrect."])?;
        return ctx.close();
    } else if l_i == -1 {
        ctx.lines_as("Use in case of emergency", args!["You have canceled your request."])?;
        return ctx.close();
    } else if l_i == 0 {
        return ctx.close();
    } else {
        ctx.lines_as("Use in case of emergency", args!["What services would you like to use?"])?;
        ctx.next()?;
        match ctx.menu(&["Turn off Warps.", "Reset Timer.", "Reset chat room."])? {
            0 => {
                ctx.lines_as("Use in case of emergency", args!["Press the 'Next' button to turn off warps."])?;
                ctx.next()?;
                ctx.npc().do_event("god_wep_warpmaster::OnDisable")?;
                ctx.lines_as("Use in case of emergency", args!["You have successfully turned off warps."])?;
                return ctx.close();
            }
            1 => {
                ctx.lines_as("Use in case of emergency", args!["Press the 'Next' button to reset timer."])?;
                ctx.next()?;
                ctx.npc().do_event("Grunburti#god::OnEnable")?;
                ctx.lines_as("Use in case of emergency", args!["You have successfully reset timer."])?;
                return ctx.close();
            }
            2 => {
                ctx.lines_as(
                    "Use in case of emergency",
                    args!["Please press the 'Next' button to reset the arena chat room in que_god01."],
                )?;
                ctx.next()?;
                ctx.npc().do_event("#god_hopewarp1::OnReset")?;
                ctx.lines_as(
                    "Use in case of emergency",
                    args!["You have successfully reset the arena chat room (Laboratory Entrance 1/2)."],
                )?;
                return ctx.close();
            }
            _ => {}
        }
    }
    Ok(())
}
