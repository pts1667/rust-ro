use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn school_of_fish_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_fcast = Val::from(0);
    let mut l_rhea_ran = Val::from(0);
    if (ctx.call(Function::CheckQuest, vec![Val::from(12060), ctx.constant("PLAYTIME")?])? == -1
        && ctx.call(Function::CountItem, vec![Val::from(6039)])?.number()? < 20)
    {
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BUBBLE")?])?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_INVENOM")?])?;
        l_fcast = Val::from(15);
        if ctx.call(Function::IsEquipped, vec![Val::from(2550)])?.is_true() {
            l_fcast = (l_fcast.clone().try_sub(Val::from(2))?);
        }
        if ctx.call(Function::IsEquipped, vec![Val::from(2443)])?.is_true() {
            l_fcast = (l_fcast.clone().try_sub(Val::from(2))?);
        }
        if ctx.call(Function::IsEquipped, vec![Val::from(2764)])?.is_true() {
            l_fcast = (l_fcast.clone().try_sub(Val::from(3))?);
        }
        if ctx.call(Function::IsEquipped, vec![Val::from(2775)])?.is_true() {
            l_fcast = (l_fcast.clone().try_sub(Val::from(1))?);
        }
        if ctx.call(Function::IsEquipped, vec![Val::from(1599)])?.is_true() {
            l_fcast = (l_fcast.clone().try_sub(Val::from(3))?);
        }
        if ctx.call(Function::IsEquipped, vec![Val::from(2199)])?.is_true() {
            l_fcast = (l_fcast.clone().try_sub(Val::from(4))?);
        }
        ctx.call(Function::ProgressBar, vec![Val::from("ffff00"), l_fcast.clone()])?;
        if ctx.var("ep13_1_rhea").get()? == 13 && ctx.call(Function::Rand, vec![Val::from(1), Val::from(20)])? == 2 {
            ctx.call(Function::GetItem, vec![Val::from(6037), Val::from(1)])?;
            ctx.var("ep13_1_rhea").set(Val::from(14))?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BUBBLE")?])?;
            ctx.call(
                Function::MapAnnounce,
                vec![
                    ctx.call(Function::StrCharInfo, vec![Val::from(3)])?,
                    (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" has caught a Messy File!!")),
                    ctx.constant("BC_MAP")?,
                    Val::from("0xff77ff"),
                ],
            )?;
        }
        l_rhea_ran = ctx.call(Function::Rand, vec![Val::from(1), Val::from(70)])?;
        if l_rhea_ran.clone().number()? < 20 {
            ctx.call(Function::GetItem, vec![Val::from(6039), Val::from(1)])?;
        } else {
            if l_rhea_ran.clone() == 20 {
                ctx.call(Function::GetItem, vec![Val::from(908), Val::from(1)])?;
            } else {
                if l_rhea_ran.clone() == 21 {
                    ctx.call(Function::GetItem, vec![Val::from(909), Val::from(1)])?;
                } else {
                    if l_rhea_ran.clone() == 22 {
                        ctx.call(Function::GetItem, vec![Val::from(963), Val::from(1)])?;
                    } else {
                        if l_rhea_ran.clone() == 23 {
                            ctx.call(Function::GetItem, vec![Val::from(956), Val::from(1)])?;
                        } else {
                            if l_rhea_ran.clone() == 24 {
                                ctx.call(Function::GetItem, vec![Val::from(6049), Val::from(1)])?;
                            } else {
                                if l_rhea_ran.clone() == 25 {
                                    ctx.call(Function::GetItem, vec![Val::from(918), Val::from(1)])?;
                                } else if l_rhea_ran.clone() == 26 {
                                    ctx.call(Function::GetItem, vec![Val::from(960), Val::from(1)])?;
                                } else if l_rhea_ran.clone() == 27 {
                                    ctx.call(Function::GetItem, vec![Val::from(910), Val::from(1)])?;
                                } else if l_rhea_ran.clone() == 28 {
                                    ctx.call(Function::GetItem, vec![Val::from(938), Val::from(1)])?;
                                } else if (l_rhea_ran.clone().number()? > 28 && l_rhea_ran.clone().number()? < 40) {
                                    ctx.call(Function::GetItem, vec![Val::from(7049), Val::from(1)])?;
                                } else {
                                    ctx.mes("Nothing was caught.")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                        }
                    }
                }
            }
        }
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(200)])? == 3 {
            ctx.call(Function::GetItem, vec![Val::from(644), Val::from(1)])?;
            ctx.call(
                Function::MapAnnounce,
                vec![
                    ctx.call(Function::StrCharInfo, vec![Val::from(3)])?,
                    (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" has caught a Gift Box!!")),
                    ctx.constant("BC_MAP")?,
                    Val::from("0x00ffff"),
                ],
            )?;
        }
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(500)])? == 3 {
            ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
            ctx.call(
                Function::MapAnnounce,
                vec![
                    ctx.call(Function::StrCharInfo, vec![Val::from(3)])?,
                    (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" has caught an Old Blue Box!!")),
                    ctx.constant("BC_MAP")?,
                    Val::from("0x00ffff"),
                ],
            )?;
        }
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3000)])? == 3 {
            ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
            ctx.call(
                Function::MapAnnounce,
                vec![
                    ctx.call(Function::StrCharInfo, vec![Val::from(3)])?,
                    (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" has caught an Old Purple Box!!")),
                    ctx.constant("BC_MAP")?,
                    Val::from("0x44ff44"),
                ],
            )?;
        }
        return Err(Stop::End);
    } else {
        ctx.mes("Fish are swimming in the water.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn school_of_fish_1(ctx: &Ctx) -> Script {
    school_of_fish_1_body(ctx, Vec::new()).map(|_| ())
}

fn henry_clifford_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CountItem, vec![Val::from(6049)])?.number()? > 0 {
        ctx.lines_as(
            "Henry Clifford",
            args![
                "Congratulations, you've caught a precious Marlin.",
                "I'll give you 2 Cat Trading Points in exchange for your Marlin."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Henry Clifford",
            args![
                "I got them from a black marketer,",
                "but I guarantee the authenticity of these points.",
                "Are you interested?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes.:No, thanks.")])? {
            1 => {
                ctx.call(Function::DelItem, vec![Val::from(6049), Val::from(1)])?;
                ctx.var("ep13_yong1").set((ctx.var("ep13_yong1").get()? + Val::from(2)))?;
                ctx.lines_as(
                    "Henry Clifford",
                    args![
                        "Thank you for the Marlin.",
                        "I've given you the points in return.",
                        "You may go check your points through the Cat Paw Agent."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Henry Clifford",
                    args!["Alright, if you say so.", "But feel free to come back if you change your mind."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines_as(
            "Henry Clifford",
            args![
                "You may have hear",
                "about this already, but",
                "this river is full of rare fish.",
                "I've followed these cats",
                "to see if the rumor is true."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Henry Clifford",
            args![
                "Marlin, the legendary silver",
                "fish is the best prey of all.",
                "You can also catch precious",
                "treasure floating in the river."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Henry Clifford",
            args![
                "If you happen to catch a Marlin,",
                "please trade it with me.",
                "I'll give you something",
                "nice in return."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn henry_clifford(ctx: &Ctx) -> Script {
    henry_clifford_body(ctx, Vec::new()).map(|_| ())
}

fn cat_paw_mining_agent_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckQuest, vec![Val::from(12062), ctx.constant("PLAYTIME")?])? == -1 {
        if (ctx.call(Function::CountItem, vec![Val::from(6048)])?.number()? > 2 && ctx.var("ep13_yong1").get()?.number()? > 9) {
            ctx.lines_as(
                "Cat Paw Mining Agent",
                args!["Oh, wow~", "Thank you for collecting minerals for me."],
            )?;
            ctx.next()?;
            ctx.call(Function::DelItem, vec![Val::from(6048), Val::from(3)])?;
            ctx.call(Function::GetExperience, vec![Val::from(30000), Val::from(0)])?;
            ctx.call(Function::SetQuest, vec![Val::from(12062)])?;
            ctx.var("ep13_yong1").set((ctx.var("ep13_yong1").get()? + Val::from(1)))?;
            let choice = runtime::select_values(ctx, &[Val::from("I'm freezing! Take them quickly.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Cat Paw Mining Agent",
                args![
                    "Yes, yes~ I've received the mineral samples.",
                    "I'll go ahead and add points to your credit."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Cat Paw Mining Agent",
                args![
                    "To collect minerals,",
                    "we have to go to the",
                    "frozen land to the east.",
                    "But for short-haired cats",
                    "like me, the weather",
                    "is unbearable."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cat Paw Mining Agent",
                args![
                    "If the weather",
                    "is good tomorrow,",
                    "I might try to go,",
                    "but not today.",
                    "It looks freezing to",
                    "you too, doesn't it?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if (ctx.call(Function::CheckQuest, vec![Val::from(12062), ctx.constant("PLAYTIME")?])? == 0
            || ctx.call(Function::CheckQuest, vec![Val::from(12062), ctx.constant("PLAYTIME")?])? == 1)
        {
            ctx.lines_as(
                "Cat Paw Mining Agent",
                args![
                    "I hope you'll bring me minerals again tomorrow...",
                    "If you can. It's much better to wait for you to do it than go there on my own."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.call(Function::EraseQuest, vec![Val::from(12062)])?;
            ctx.lines_as(
                "Cat Paw Mining Agent",
                args![
                    "Umm... It's time to collect minerals.",
                    "How'd you like to collect minerals for me?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn cat_paw_mining_agent(ctx: &Ctx) -> Script {
    cat_paw_mining_agent_body(ctx, Vec::new()).map(|_| ())
}

fn mysterious_rock_0_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_rhea_ran = Val::from(0);
    if (ctx.call(Function::CountItem, vec![Val::from(6048)])?.number()? < 3
        && ctx.call(Function::CheckQuest, vec![Val::from(12062), ctx.constant("PLAYTIME")?])? == -1)
    {
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_REPAIRWEAPON")?])?;
        ctx.call(Function::ProgressBar, vec![Val::from("ffff00"), Val::from(10)])?;
        l_rhea_ran = ctx.call(Function::Rand, vec![Val::from(1), Val::from(20)])?;
        if l_rhea_ran.clone().number()? < 13 {
            ctx.call(Function::GetItem, vec![Val::from(7049), Val::from(1)])?;
        } else if l_rhea_ran.clone() == 13 {
            ctx.call(Function::GetItem, vec![Val::from(990), Val::from(1)])?;
        } else if l_rhea_ran.clone() == 14 {
            ctx.call(Function::GetItem, vec![Val::from(991), Val::from(1)])?;
        } else if l_rhea_ran.clone() == 15 {
            ctx.call(Function::GetItem, vec![Val::from(992), Val::from(1)])?;
        } else if l_rhea_ran.clone() == 16 {
            ctx.call(Function::GetItem, vec![Val::from(993), Val::from(1)])?;
        } else {
            ctx.call(Function::GetItem, vec![Val::from(6048), Val::from(1)])?;
        }
        ctx.call(Function::InitNpcTimer, vec![])?;
        ctx.call(Function::DisableNpc, vec![])?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "This rock contains unidentified minerals.",
            "It's not possible to mine more than the limit."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn mysterious_rock_0(ctx: &Ctx) -> Script {
    mysterious_rock_0_body(ctx, Vec::new()).map(|_| ())
}

fn mysterious_rock_0_ontimer120000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn mysterious_rock_0_ontimer120000(ctx: &Ctx) -> Script {
    mysterious_rock_0_ontimer120000_body(ctx, Vec::new()).map(|_| ())
}

fn piece_of_crack_sec_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input = Val::from(0);
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.mes("1~3000")?;
    let (input, status) = runtime::input_number(ctx, None, None)?;
    l_input = input;
    ctx.next()?;
    if (l_input.clone().number()? < 1 || l_input.clone().number()? > 3000) {
        ctx.lines(args!["Cat trading Point adjust", "You can enter the number between 1~3000."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.var("ep13_yong1").set(l_input.clone())?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn piece_of_crack_sec(ctx: &Ctx) -> Script {
    piece_of_crack_sec_body(ctx, Vec::new()).map(|_| ())
}

fn hibba_agip_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_start = Val::from(0);
    ctx.call(Function::Cutin, vec![Val::from("ep13_captin_edq"), Val::from(2)])?;
    l_start = (((((ctx.var("ep13_newbs").get()? + ctx.var("ep13_ryu").get()?) + ctx.var("mao_morocc2").get()?)
        + ctx.var("ep13_1_rhea").get()?)
        + ctx.var("ep13_animal").get()?)
        + ctx.var("ep13_start").get()?);
    if (ctx.var("ep13_1_edq").get()? == 0 && l_start.clone().number()? > 115) {
        ctx.lines_as(
            "Hibba Agip",
            args![
                "*Sigh* Look at me, I used to command the desert of Morocc, but I've been deployed to a world that only God knows where..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Staff Officer Abidal", args!["Come on'that's not ture. In fact, you used to be an official of a small town, and have been promoted to the commander of the expedition.", "Your success is almost too good to be true."])?;
        ctx.next()?;
        ctx.lines_as("Instructor Igrid", args!["Hey, boss. Can we just go through the motions until we get out of here? This so-called Ash-Vacuum is boring. It's heaven for the kingdom's scholars, but it's not for me."])?;
        ctx.next()?;
        ctx.lines_as(
            "Hibba Agip",
            args![
                "Geez, do you have to keep calling me boss? It's Commander, alright?",
                "Hmpf! So disrespectful."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Staff Officer Abidal", args!["Can you guys stop complaining?", "God..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Igrid",
            args!["Who are you? What business do you have with Commander Agip?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Hibba Agip", args!["What is it?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Staff Officer Abidal",
            args![
                ((Val::from("Err? Hey, aren't you that famous adventurer,") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from("? I've heard many good things about you."))
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Igrid",
            args!["I don't know what you're talking about. Boss, do you know this adventurer?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hibba Agip",
            args!["Stop calling me boss! Call me Commander, alright? Hey Abidal, is this adventurer really famous?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Staff Officer Abidal",
            args![
                ((Val::from("Commander, haven't you read the report? The adventurer who helped the expedition solve the XX case was")
                    + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from("."))
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hibba Agip",
            args!["*Ahem Ahem* I see. I've been too busy to read all the reports. So what brings you here? A reward?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Staff Officer Abidal",
            args!["This is perfect. I was going to send a messenger to you for an important discussion."],
        )?;
        ctx.next()?;
        ctx.lines_as("Hibba Agip", args!["Hey Abidal, he's here to see me, not you. Remember?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Staff Officer Abidal",
            args![
                ((Val::from("Commander, please let me talk. ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from("I'd like to talk to you privately. Can you come back later?"))
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hibba Agip",
            args![
                "Why do you want to talk privately?",
                "Is it because of me? Grrr... I'll be watching you, Abidal."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Igrid",
            args!["What are you still doing here? Go, go speak to Abidal."],
        )?;
        ctx.var("ep13_1_edq").set(Val::from(1))?;
        ctx.call(Function::SetQuest, vec![Val::from(3085)])?;
        ctx.call(Function::Cutin, vec![Val::from("ep13_captin_edq"), Val::from(255)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("ep13_1_edq").get()? == 1 {
            ctx.lines_as(
                "Hibba Agip",
                args!["I'm not the one who has business with you. You should go speak to Staff Officer Abidal."],
            )?;
            ctx.call(Function::Cutin, vec![Val::from("ep13_captin_edq"), Val::from(255)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("ep13_1_edq").get()? == 2 {
                ctx.lines_as(
                    "Hibba Agip",
                    args![
                        ((Val::from("Oh yes, right... You're") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                            + Val::from(", right? Abidal has told me good things about you."))
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hibba Agip",
                    args!["We're lucky to have you, my friend... I hope you're as good as the stories play you up to be."],
                )?;
                ctx.next()?;
                ctx.lines_as("Staff Officer Abidal", args!["Commander, please!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Hibba Agip",
                    args![
                        "Alright, alright. gosh will you stop yelling at me?",
                        "You're not going to live long if you get angry at all the little things."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hibba Agip",
                    args![
                        "Anyways, I took some time to check your background and reports of your accomplishments.",
                        "I must say that I'm quite impressed."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Hibba Agip", args!["As you know by now, I have to send out an expedition report to the national leaders of the Midgard continent, and I need someone that I can trust."])?;
                ctx.next()?;
                ctx.lines_as("Hibba Agip", args!["I'd go if I could, but my duty is to command, and my instructor over there is too dense. He'll embarrass me by acting stupid in front of the national leaders."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Instructor Igrid",
                    args!["Come on, boss... Er, Commander!", "Don't you think you're exaggerating?"],
                )?;
                ctx.next()?;
                ctx.lines_as("Hibba Agip", args!["Hmm... Anyways, this is not a difficult mission. All you have to do is deliver this expedition report to officials of three countries of Midgard. Simple right?"])?;
                ctx.next()?;
                ctx.call(Function::GetItem, vec![Val::from(11012), Val::from(1)])?;
                ctx.var("ep13_1_edq").set(Val::from(3))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(3086), Val::from(3087)])?;
                ctx.lines_as("Hibba Agip", args!["The officials are ^0000FFMinister Laur^000000 of the Rune-Midgarts' Kingdom, ^0000FFPriest Nuria^000000 of Arunafeltz, and ^0000FFGerhart^000000 of the Schwatzvalt Republic."])?;
                ctx.next()?;
                ctx.lines_as("Hibba Agip", args!["A messenger, who will inform the official of your arrival, is waiting for you at the time-space gap to Midgard. You'd better find him before you leave."])?;
                ctx.next()?;
                ctx.lines_as("Hibba Agip", args!["Since the officials are busy attending to national matters, they won't meet you unless they're informed of your arrival beforehand. I doubt those pretentious guys would even want to see me."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Hibba Agip",
                    args!["Travel safely, adventurer, and come back in one piece, alright?"],
                )?;
                ctx.call(Function::Cutin, vec![Val::from("ep13_captin_edq"), Val::from(255)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("ep13_1_edq").get()? == 3 {
                    ctx.lines_as(
                        "Hibba Agip",
                        args!["Haven't you left yet? It's hard to make ends meet, isn't it?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Hibba Agip", args!["I mean, if you want to be paid, you'll have to work. Do you still not understand? I'm saying that you better proceed with your mission."])?;
                    ctx.call(Function::Cutin, vec![Val::from("ep13_captin_edq"), Val::from(255)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("ep13_1_edq").get()? == 4 {
                        ctx.lines_as("Hibba Agip", args!["Hmm..."])?;
                        ctx.next()?;
                        ctx.lines_as("Staff Officer Abidal", args!["......"])?;
                        ctx.next()?;
                        ctx.lines_as("Hibba Agip", args!["Alright, alright. I know why you've come to see me."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Staff Officer Abidal",
                            args!["They haven't gotten the report yet. We should send out soldiers..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Hibba Agip", args!["That's not why I'm concerned. The problem is that the assignee failed to complete the mission and lost the report."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hibba Agip",
                            args!["Anyways... It's alright. You don't have to feel bad. I understand you didn't mean it this way."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Instructor Igrid",
                            args!["Commander, who might have done this? I will destroy him myself."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Hibba Agip", args!["Don't impose. Our troops aren't just from one nation, they are from thre different countries, and are here to pursue their country's own interest."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Staff Officer Abidal",
                            args!["Hard to say... who is behind this. If I had to guess, either side could be behind this..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hibba Agip",
                            args!["*Spit* This is why I don't trust politicians. Abidal, resolving this incident is your top priority."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hibba Agip",
                            args!["Igrid, evaluate the security of this camp and discipline among the soldiers."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Hibba Agip", args!["Now it's your turn... Abidal disposed reports from various departments after he used them to compile the expedition report."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hibba Agip",
                            args![
                                "We take the security of our documents seriously. So... There are no extra copies of the expedition report."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Staff Officer Abidal",
                            args!["I still remember at least 30% of the document. I might be able to rewrite another..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Hibba Agip", args!["I need it to be perfect. What can we do with only 30%?"])?;
                        ctx.next()?;
                        ctx.lines_as("Hibba Agip", args!["Hey, you must find the report. If you bring as may pages of the report as you can, then Abidal will use them to fill missing parts."])?;
                        ctx.next()?;
                        ctx.lines_as("Hibba Agip", args!["Some of them might have be consumed by monsters, or have found their way to the sea, but I don't care. You'll find them at all costs, and that's my order."])?;
                        ctx.next()?;
                        ctx.lines_as("Hibba Agip", args!["I hope you'll do your best. You need to finish what you've started. We'll do our best to help you. Am I right, guys? Abidal? Igrid?"])?;
                        ctx.next()?;
                        ctx.lines_as("Staff Officer Abidal", args!["Of Course, I'm happy to..."])?;
                        ctx.next()?;
                        ctx.lines_as("Instructor Igrid", args!["......"])?;
                        ctx.next()?;
                        ctx.lines_as("Hibba Agip", args!["Now, search everywhere for the pages of that report. Staff Officer Abidal will bind them into the full report for you."])?;
                        ctx.next()?;
                        ctx.lines_as("Hibba Agip", args!["Ah, and... I've received a report that you tried to hand out the report to our enemy when the situation happened."])?;
                        ctx.next()?;
                        ctx.lines_as("Instructor Igrid", args!["Argh..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hibba Agip",
                            args!["Hahaha! You're smart, my friend. Of course, I don't like smart soldiers."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Hibba Agip", args!["Let me tell you this: I can care less about justice or righteousness, but loyalty and faithfulness are very important. Keep that in mind for your own good, alright?"])?;
                        ctx.var("ep13_1_edq").set(Val::from(61))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(3088), Val::from(3089)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hibba Agip",
                            args![
                                "I wish you good luck in finding the pages. Try to clean any smudges off them when you find them, alright?"
                            ],
                        )?;
                        ctx.call(Function::Cutin, vec![Val::from("ep13_captin_edq"), Val::from(255)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("ep13_1_edq").get()? == 5 {
                            ctx.lines_as("Hibba Agip", args!["Hmm..."])?;
                            ctx.next()?;
                            ctx.lines_as("Staff Officer Abidal", args!["......"])?;
                            ctx.next()?;
                            ctx.lines_as("Hibba Agip", args!["Alright, alright. I know why you've come to see me."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Staff Officer Abidal",
                                args!["They haven't gotten the report yet. We should send out soldiers..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Hibba Agip", args!["That's not why I'm concerned. The problem is that the assignee failed to complete the mission and lost the report."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hibba Agip",
                                args!["Anyways... It's alright. You don't have to feel bad. I understand you didn't mean it this way."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Instructor Igrid", args!["Commander, I'll go find those evil scoundrels..."])?;
                            ctx.next()?;
                            ctx.lines_as("Hibba Agip", args!["Don't impose. Our troops aren't just from one nation, they are from thre different countries, and are here to pursue their country's own interest."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Staff Officer Abidal",
                                args!["I wonder who's behind this. Frankly, I must say that every country is a suspect."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hibba Agip",
                                args![
                                    "*Spit* This is why I don't trust politicians. Abidal, resolving this incident is your top priority."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hibba Agip",
                                args!["Igrid, evaluate the security of this camp and discipline among the soldiers."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Hibba Agip", args!["Now it's your turn... Abidal disposed reports from various departments after he used them to compile the expedition report."])?;
                            ctx.next()?;
                            ctx.lines_as("Hibba Agip", args!["We take the security of our documents seriously. So... There are no extra copies of the expedition report."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Staff Officer Abidal",
                                args!["I still remember at least 30% of the document. I might be able to rewrite another..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Hibba Agip", args!["I need it to be perfect. What can we do with only 30%?"])?;
                            ctx.next()?;
                            ctx.lines_as("Hibba Agip", args!["Hey, you must find the report. If you bring as may pages of the report as you can, then Abidal will use them to fill missing parts."])?;
                            ctx.next()?;
                            ctx.lines_as("Hibba Agip", args!["Some of them might have be consumed by monsters, or have found their way to the sea, but I don't care. You'll find them at all costs, and that's my order."])?;
                            ctx.next()?;
                            ctx.lines_as("Hibba Agip", args!["I hope you'll do your best. You need to finish what you've started. We'll do our best to help you. Am I right, guys? Abidal? Igrid?"])?;
                            ctx.next()?;
                            ctx.lines_as("Staff Officer Abidal", args!["Of Course, I'm happy to..."])?;
                            ctx.next()?;
                            ctx.lines_as("Instructor Igrid", args!["......"])?;
                            ctx.next()?;
                            ctx.lines_as("Hibba Agip", args!["Now, search everywhere for the pages of that report. Staff Officer Abidal will bind them into the full report for you."])?;
                            ctx.next()?;
                            ctx.lines_as("Hibba Agip", args!["Ah, and... I've received a report that you fought hard against the enemy to protect the report from them."])?;
                            ctx.next()?;
                            ctx.lines_as("Instructor Igrid", args!["Umm..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hibba Agip",
                                args!["Hahaha, you're fearless and I can make good use of a soldier like you at any time of the day."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Hibba Agip", args!["Let me tell you this, loyalty and faithfulness are very, very important. They may cause you hardship sometimes, but those qualities will guarantee you the last laugh."])?;
                            ctx.var("ep13_1_edq").set(Val::from(62))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(3088), Val::from(3089)])?;
                            ctx.next()?;
                            ctx.lines_as("Hibba Agip", args!["I wish you good luck in finding the pages. Try to clean any smudges off them when you find them, alright?"])?;
                            ctx.call(Function::Cutin, vec![Val::from("ep13_captin_edq"), Val::from(255)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if (ctx.var("ep13_1_edq").get()? == 61 || ctx.var("ep13_1_edq").get()? == 62) {
                                ctx.lines_as("Hibba Agip", args!["Talk to my staff officer until you're finisehd finding the pages of the lost report. Right now, I'm swarmed with work."])?;
                                ctx.next()?;
                                ctx.lines_as("Hibba Agip", args!["I'm sure you'll easily find those if you keep your eyes open. I must warn you about the monsters around this area though. Who knows? They might love the taste of paper."])?;
                                ctx.call(Function::Cutin, vec![Val::from("ep13_captin_edq"), Val::from(255)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if (ctx.var("ep13_1_edq").get()? == 71 || ctx.var("ep13_1_edq").get()? == 72) {
                                    ctx.lines_as("Hibba Agip", args!["How have you been doing? I've heard Igrid has personally send out his soldiers to support you guys.", "Haven't you met them?"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hibba Agip",
                                        args![
                                            "While searching for the report, look around and meet our soldiers.",
                                            "They've been sent to help you and Abidal. Ask their help if you think you need it."
                                        ],
                                    )?;
                                    ctx.call(Function::Cutin, vec![Val::from("ep13_captin_edq"), Val::from(255)])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("ep13_1_edq").get()? == 8 {
                                        ctx.lines_as("Hibba Agip", args!["So, the report has been restored successfully?"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hibba Agip",
                                            args!["This was finished more quickly than I thought. Let me see..."],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::DelItem, vec![Val::from(11012), Val::from(1)])?;
                                        ctx.var("ep13_1_edq").set(Val::from(9))?;
                                        ctx.call(Function::ChangeQuest, vec![Val::from(3092), Val::from(3093)])?;
                                        ctx.lines_as(
                                            "Hibba Agip",
                                            args!["It looks alright. I don't have to read this thoroughly because I trust Abidal."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Hibba Agip", args!["Now, let's go back to your original mission. I'd like you to deliver this to the national leaders of the three countries of Midgard."])?;
                                        ctx.next()?;
                                        ctx.call(Function::GetItem, vec![Val::from(11012), Val::from(3)])?;
                                        ctx.lines_as(
                                            "Hibba Agip",
                                            args![
                                                "Dont worry too much, my friend.",
                                                "I've arranged troops in the time-space gap to protect the report."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Hibba Agip", args!["Of course, you can't see them because they're undercover soldiers under my direct command. Hahaha!"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hibba Agip", args!["Let me remind you again. The officials are these three people: ^0000FFMinister Laur^000000 of the Rune-Midgarts Kingdom, ^0000FFPriest Nuria^000000 of Arunafeltz and ^0000FFGerhart^000000 of the Schwarzwald Republic."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hibba Agip",
                                            args!["Make sure to deliver the report to them without fail. Come back in one piece, alright?"],
                                        )?;
                                        ctx.call(Function::Cutin, vec![Val::from("ep13_captin_edq"), Val::from(255)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if (ctx.var("ep13_1_edq").get()? == 9
                                            && ctx.call(Function::CountItem, vec![Val::from(11012)])? == 3)
                                        {
                                            ctx.lines_as("Hibba Agip", args!["Let me remind you again. The officials are these three people: ^0000FFMinister Laur^000000 of the Rune-Midgarts Kingdom, ^0000FFPriest Nuria^000000 of Arunafeltz and ^0000FFGerhart^000000 of the Schwarzwald Republic."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Hibba Agip",
                                                args![
                                                    "Make sure to deliver the report to them without fail. Come back in one piece, alright?"
                                                ],
                                            )?;
                                            ctx.call(Function::Cutin, vec![Val::from("ep13_captin_edq"), Val::from(255)])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.var("ep13_1_edq").get()? == 9 {
                                                ctx.lines_as(
                                                    "Hibba Agip",
                                                    args!["Is something wrong? Alright, we'll write another report for you."],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(
                                                    Function::DelItem,
                                                    vec![Val::from(11012), ctx.call(Function::CountItem, vec![Val::from(11012)])?],
                                                )?;
                                                ctx.call(Function::GetItem, vec![Val::from(11012), Val::from(3)])?;
                                                ctx.lines_as("Hibba Agip", args!["Let me remind you again. The officials are these three people: ^0000FFMinister Laur^000000 of the Rune-Midgarts Kingdom, ^0000FFPriest Nuria^000000 of Arunafeltz and ^0000FFGerhart^000000 of the Schwarzwald Republic."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hibba Agip", args!["Make sure to deliver the report to them without fail. Come back in one piece, alright?"])?;
                                                ctx.call(Function::Cutin, vec![Val::from("ep13_captin_edq"), Val::from(255)])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                if ctx.var("ep13_1_edq").get()?.number()? > 100 {
                                                    ctx.lines_as("Hibba Agip", args!["Let me remind you again. The officials are these three people: ^0000FFMinister Laur^000000 of the Rune-Midgarts Kingdom, ^0000FFPriest Nuria^000000 of Arunafeltz and ^0000FFGerhart^000000 of the Schwarzwald Republic."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Hibba Agip", args!["Make sure to deliver the report to them without fail. Come back in one piece, alright?"])?;
                                                    ctx.call(Function::Cutin, vec![Val::from("ep13_captin_edq"), Val::from(255)])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    if ctx.var("ep13_1_edq").get()? == 13 {
                                                        ctx.lines_as("Hibba Agip", args!["Oh, how was your trip? Hey, you look better than you did when you left. Did you drop by your hometown and enjoy great food or something?"])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Hibba Agip", args!["So tell me, did you bring the report to all three national leaders? Good job."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Hibba Agip", args!["I understand there were some unforeseen obstacles we didn't anticipate since we thought this mission was pretty trivial, but you still did your best."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Hibba Agip", args!["I must warn you, however, this is not the end of these strange events. You're involved in this situation so deeply that you can't get out of it. Hahaha!"])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Hibba Agip",
                                                            args!["Don't worry. I'm not here to fool around. Isn't that right?"],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Staff Officer Abidal",
                                                            args!["Yes, sir. We can't let the same thing happen again."],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Instructor Igrid",
                                                            args!["I shall find out who's behind this, and destroy him."],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Hibba Agip", args!["So... I was hoping that you'll come back and help us once we figure out who's behind this. Will you?"])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Hibba Agip",
                                                            args!["And please take this small reward.", "Thank you for your help so far."],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.call(Function::GetExperience, vec![Val::from(250000), Val::from(0)])?;
                                                        ctx.call(Function::GetItem, vec![Val::from(12110), Val::from(3)])?;
                                                        ctx.var("ep13_1_edq").set(Val::from(14))?;
                                                        ctx.call(Function::CompleteQuest, vec![Val::from(3094)])?;
                                                        ctx.lines_as(
                                                            "Hibba Agip",
                                                            args![
                                                                "Then I'll see you next time. Keep yourself in one piece, alright?",
                                                                "Hahaha!"
                                                            ],
                                                        )?;
                                                        ctx.call(Function::Cutin, vec![Val::from("ep13_captin_edq"), Val::from(255)])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else {
                                                        if ctx.var("ep13_1_edq").get()? == 14 {
                                                            ctx.lines_as("Hibba Agip", args!["Hey, how have you been? It's too early to say that we've figured out who's the mastermind."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Hibba Agip",
                                                                args!["Please be patient, I'll contact you when the time is right."],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Hibba Agip", args!["Wait, have you explored this area yet? I mean the wide tundra and fields around the camp."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Hibba Agip", args!["There are many fun things outside. If you are an adventurer, go explore for yourself. Come back after you've explored the areas around the camp."])?;
                                                            ctx.call(Function::Cutin, vec![Val::from("ep13_captin_edq"), Val::from(255)])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else {
                                                            if (ctx.var("ep13_1_edq").get()? == 15 && ctx.var("ins_nyd").get()? == 1) {
                                                                ctx.lines_as(
                                                                    "Hibba Agip",
                                                                    args!["Ah, it's you again. So, are you accustomed to this area now?"],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Hibba Agip", args!["What brings you here again? You seem to have something to say to me."])?;
                                                                ctx.next()?;
                                                                match runtime::select_values(
                                                                    ctx,
                                                                    &[Val::from(
                                                                        "I just want to say hello.:I found a weird portal below the mine cave.",
                                                                    )],
                                                                )? {
                                                                    1 => {
                                                                        ctx.lines_as("Hibba Agip", args!["Well hello then..."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Hibba Agip", args!["Please be patient. I'll contact you when the time is right."])?;
                                                                        ctx.call(
                                                                            Function::Cutin,
                                                                            vec![Val::from("ep13_captin_edq"), Val::from(255)],
                                                                        )?;
                                                                    }
                                                                    2 => {
                                                                        ctx.lines_as(
                                                                            "Hibba Agip",
                                                                            args!["What portal? I've never heard about it."],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Hibba Agip", args!["That place must be where both races fight against each other. It might be dangerous there."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Hibba Agip", args!["I have no idea of what it is like. It is not in my field of expertise."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Hibba Agip", args!["There is someone here who can help you. He is a historian from Rune-Midgarts. He may be aware of some relics or something that could help."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Hibba Agip", args!["Find the historian and see if he has any useful information for you."])?;
                                                                        ctx.var("ins_nyd").set(Val::from(2))?;
                                                                        ctx.call(
                                                                            Function::Cutin,
                                                                            vec![Val::from("ep13_captin_edq"), Val::from(255)],
                                                                        )?;
                                                                    }
                                                                    _ => {}
                                                                }
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else {
                                                                if (ctx.var("ins_nyd").get()?.number()? > 1
                                                                    && ctx.var("ins_nyd").get()?.number()? < 7)
                                                                {
                                                                    ctx.lines_as(
                                                                        "Hibba Agip",
                                                                        args!["He's at his post, you can't find him?"],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Hibba Agip", args!["His name is Magniffer... look for him.. he's somewhere around here..."])?;
                                                                    ctx.call(
                                                                        Function::Cutin,
                                                                        vec![Val::from("ep13_captin_edq"), Val::from(255)],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else {
                                                                    if ctx.var("ins_nyd").get()? == 7 {
                                                                        ctx.lines_as("Hibba Agip", args!["Did you talk with the scholar? Thesedays, I haven't seen him for a while."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Staff Officer Abidal",
                                                                            args!["That's because you don't like talking to him."],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Hibba Agip", args!["Hey, don't judge me like that...the problem is only that I can't see him often."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Hibba Agip", args!["Whatever. Did something happen because he showed up to me? Tell me more."])?;
                                                                        ctx.next()?;
                                                                        match runtime::select_values(ctx, &[Val::from("Tell the story.")])?
                                                                        {
                                                                            1 => {
                                                                                ctx.lines_as("Hibba Agip", args!["Hmmm... umm... so..."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Hibba Agip", args!["So, that's why that happened like that... It's so complicated..."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Hibba Agip", args!["And you guys...you just take a task that you start up at first."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as(
                                                                                    "Staff Officer Abidal",
                                                                                    args!["You sent him to Doctor Magnifer first...."],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as(
                                                                                    "Hibba Agip",
                                                                                    args![
                                                                                        "I just thought that he needed someone to talk to."
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as(
                                                                                    "Staff Officer Abidal",
                                                                                    args!["Ah yeah... you're right."],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Hibba Agip", args!["Hmm... for example, you want to investigate a cave that you found, but you got on the Laphine and Sapha's nerves. right?"])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Hibba Agip", args!["And that two guys pay attention and stay quiet, but they are doing it for themselves.."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Hibba Agip", args!["The most important thing is that this place is the ground that the Yggdrasilberry takes root acording to Magnifer."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as(
                                                                                    "Hibba Agip",
                                                                                    args![
                                                                                        "Abidal, do you have anything to say about this?"
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Staff Officer Abidal", args!["Our object is to make contact with one tribe and investgate this world and how adaptable it can be."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Staff Officer Abidal", args!["Actually, we don't have to make this situation bigger than it already is."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Hibba Agip", args!["Yeah right. I don't want to extend this thing anymore..."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Staff Officer Abidal", args!["But the two tribes have remained neutral towards us up until now."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Staff Officer Abidal", args!["We don't really know what attitude they have towards us."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Hibba Agip", args!["Yeah.. that makes sense... If they made a general attack, we would probably fail."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Staff Officer Abidal", args!["We should map out a strategy. There is a method that uses Laphine and Sapha's together."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Staff Officer Abidal", args!["We send this guy to one of the tribes and ask for cooperation in our investigation."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Hibba Agip", args!["Will they help us? They have already said threatening words..."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Staff Officer Abidal", args!["Of course they will help us. if they reject our demand, we just say that we'll ask for help from the other tribe."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Hibba Agip", args!["Right, if we do that, they will have no other choice."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Staff Officer Abidal", args!["In this situation, they want to avoid showing that place to the other tribe."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as(
                                                                                    "Staff Officer Abidal",
                                                                                    args!["So, they have to cooperate with us."],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as(
                                                                                    "Hibba Agip",
                                                                                    args!["Oh, you have a malicious idea."],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Staff Officer Abidal", args!["That's called that Kuhotanrangjigea from east. I didn't make that by myself."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as(
                                                                                    "Hibba Agip",
                                                                                    args![
                                                                                        ((Val::from(
                                                                                            "Whatever.. Hey, you there... your name is "
                                                                                        ) + ctx.call(
                                                                                            Function::StrCharInfo,
                                                                                            vec![Val::from(0)]
                                                                                        )?) + Val::from(
                                                                                            " right? Do you know what to do?"
                                                                                        ))
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Hibba Agip", args!["Go to one of the tribes and ask for help officially. I'll send someone to the other tribe."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Hibba Agip", args!["I'm going to give you a choice. Which tribe do you want to go to?"])?;
                                                                                ctx.next()?;
                                                                                match runtime::select_values(
                                                                                    ctx,
                                                                                    &[Val::from("Laphine:Sapha")],
                                                                                )? {
                                                                                    1 => {
                                                                                        ctx.lines_as("Hibba Agip", args!["Okay, then I place responsibility on you. from now on you focus on acquiring a good reputation with the Laphine."])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as("Hibba Agip", args!["It won't be easy to get a good impression from the Sapha again."])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as("Hibba Agip", args!["So you must earn us a good reputation from the Laphine."])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "Hibba Agip",
                                                                                            args!["Will you choose the Laphine?"],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        match runtime::select_values(
                                                                                            ctx,
                                                                                            &[Val::from("Choose Laphine.:Think again.")],
                                                                                        )? {
                                                                                            1 => {
                                                                                                ctx.lines_as("Hibba Agip", args!["Good. You are now responsible with building our relationship with the Laphine tribe."])?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines_as("Hibba Agip", args!["Go to the chief of the Laphine and if he doesn't accept our request, just say that you are going to ask for help from the Sapha."])?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines_as("Hibba Agip", args!["You have your orders! Now I'm going to look for an adventurer who is willing to be sent to the Sapha."])?;
                                                                                                ctx.var("ins_nyd").set(Val::from(81))?;
                                                                                                ctx.call(
                                                                                                    Function::Cutin,
                                                                                                    vec![
                                                                                                        Val::from("ep13_captin_edq"),
                                                                                                        Val::from(255),
                                                                                                    ],
                                                                                                )?;
                                                                                            }
                                                                                            2 => {
                                                                                                ctx.lines_as(
                                                                                                    "Hibba Agip",
                                                                                                    args![
                                                                                                        "Yeah? Then think again and decide."
                                                                                                    ],
                                                                                                )?;
                                                                                                ctx.call(
                                                                                                    Function::Cutin,
                                                                                                    vec![
                                                                                                        Val::from("ep13_captin_edq"),
                                                                                                        Val::from(255),
                                                                                                    ],
                                                                                                )?;
                                                                                            }
                                                                                            _ => {}
                                                                                        }
                                                                                    }
                                                                                    2 => {
                                                                                        ctx.lines_as("Hibba Agip", args!["Okay, then I place responsibility on you. from now on you focus on acquiring a good reputation with the Sapha."])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as("Hibba Agip", args!["It won't be easy to get a good impression from the Laphine again."])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as("Hibba Agip", args!["So you must earn us a good reputation from the Sapha."])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "Hibba Agip",
                                                                                            args!["Will you choose the Sapha?"],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        match runtime::select_values(
                                                                                            ctx,
                                                                                            &[Val::from("Choose Sapha.:Think again.")],
                                                                                        )? {
                                                                                            1 => {
                                                                                                ctx.lines_as("Hibba Agip", args!["Good. You are now responsible with building our relationship with the Sapha tribe."])?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines_as("Hibba Agip", args!["Go to the chief of the Sapha and if he doesn't accept our request, just say that you are going to ask for help from the Laphine."])?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines_as("Hibba Agip", args!["You have your orders! Now I'm going to look for an adventurer who is willing to be sent to the Laphine."])?;
                                                                                                ctx.var("ins_nyd").set(Val::from(82))?;
                                                                                                ctx.call(
                                                                                                    Function::Cutin,
                                                                                                    vec![
                                                                                                        Val::from("ep13_captin_edq"),
                                                                                                        Val::from(255),
                                                                                                    ],
                                                                                                )?;
                                                                                            }
                                                                                            2 => {
                                                                                                ctx.lines_as(
                                                                                                    "Hibba Agip",
                                                                                                    args![
                                                                                                        "Yeah? then think again and decide."
                                                                                                    ],
                                                                                                )?;
                                                                                                ctx.call(
                                                                                                    Function::Cutin,
                                                                                                    vec![
                                                                                                        Val::from("ep13_captin_edq"),
                                                                                                        Val::from(255),
                                                                                                    ],
                                                                                                )?;
                                                                                            }
                                                                                            _ => {}
                                                                                        }
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                            }
                                                                            _ => {}
                                                                        }
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    } else {
                                                                        if ctx.var("ins_nyd").get()? == 81 {
                                                                            ctx.lines_as("Hibba Agip", args!["Good. You are now responsible with building our relationship with the Laphine tribe."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Hibba Agip", args!["Go to the chief of the Laphine and if he doesn't accept our request, just say that you are going to ask for help from the Sapha."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Hibba Agip", args!["I'm going to look for an adventurer who is willing to be sent to the Sapha."])?;
                                                                            ctx.call(
                                                                                Function::Cutin,
                                                                                vec![Val::from("ep13_captin_edq"), Val::from(255)],
                                                                            )?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        } else if ctx.var("ins_nyd").get()? == 82 {
                                                                            ctx.lines_as("Hibba Agip", args!["Good. You are now responsible with building our relationship with the Sapha tribe."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Hibba Agip", args!["Go to the chief of the Sapha and if he doesn't accept our request, just say that you are going to ask for help from the Laphine."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Hibba Agip", args!["You have your orders! Now I'm going to look for an adventurer who is willing to be sent to the Laphine."])?;
                                                                            ctx.call(
                                                                                Function::Cutin,
                                                                                vec![Val::from("ep13_captin_edq"), Val::from(255)],
                                                                            )?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        } else if ((ctx.var("ins_nyd").get()? == 91
                                                                            || ctx.var("ins_nyd").get()? == 101)
                                                                            || ctx.var("ins_nyd").get()? == 111)
                                                                        {
                                                                            ctx.lines_as("Hibba Agip", args!["How's the activity in Laphine? They have a stern character."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as(
                                                                                "Hibba Agip",
                                                                                args!["I'm not cut out for matching with them...ew..."],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Hibba Agip", args!["I'm going to send a few able men to the Sapha, you don't have to worry about it."])?;
                                                                            ctx.call(
                                                                                Function::Cutin,
                                                                                vec![Val::from("ep13_captin_edq"), Val::from(255)],
                                                                            )?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        } else if ((ctx.var("ins_nyd").get()? == 92
                                                                            || ctx.var("ins_nyd").get()? == 102)
                                                                            || ctx.var("ins_nyd").get()? == 112)
                                                                        {
                                                                            ctx.lines_as("Hibba Agip", args!["How's the activity in Sapha? They have a stern character."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as(
                                                                                "Hibba Agip",
                                                                                args!["I'm not cut out for matching with them...ew..."],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Hibba Agip", args!["I'm going to send a few able men to the Laphine, you don't have to worry about it."])?;
                                                                            ctx.call(
                                                                                Function::Cutin,
                                                                                vec![Val::from("ep13_captin_edq"), Val::from(255)],
                                                                            )?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        } else if ((ctx.var("ins_nyd").get()? == 121
                                                                            || ctx.var("ins_nyd").get()? == 131)
                                                                            || ctx.var("ins_nyd").get()? == 132)
                                                                        {
                                                                            ctx.lines_as("Hibba Agip", args!["You've come back alive? Let us celebrate your immunity!"])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Hibba Agip", args!["Who would have thought that that outsiders would be treated so vulgar by the leaders of the other tribes?"])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Hibba Agip", args!["I have said before this thing is not the last, and certainly there is something waiting for us."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as(
                                                                                "Hibba Agip",
                                                                                args![
                                                                                    "It is time for us to head out, is it not, instructor?"
                                                                                ],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as(
                                                                                "Staff Officer Abidal",
                                                                                args!["Right... It's time..."],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Instructor Igrid", args!["I'm ready."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as(
                                                                                "Hibba Agip",
                                                                                args![
                                                                                    "I am looking forward to what will happen... Whew..."
                                                                                ],
                                                                            )?;
                                                                            ctx.call(
                                                                                Function::Cutin,
                                                                                vec![Val::from("ep13_captin_edq"), Val::from(255)],
                                                                            )?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        } else {
                                                                            ctx.lines_as("Hibba Agip", args!["*Sigh* Look at me, I used to command the desert of Morocc, but I've been deployed to a world that only God knows where..."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Staff Officer Abidal", args!["Come on, that's not true. In fact, you used to be an official of a small town, and have been promoted to the commander of the expedition. Your success is almost too good to be true."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Instructor Igrid", args!["Hey, boss. Can we just go through the motions until we get out of here? This so-called Ash-Vacuum is boring. It's heaven for the kingdom scholars, but it's not for me."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Hibba Agip", args!["Geez, do you have to keep calling me boss? I'm the Commander, alright? So disrespectful."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as(
                                                                                "Staff Officer Abidal",
                                                                                args!["Can you guys stop complaining? God..."],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Instructor Igrid", args!["Who are you? What business do you have with Commander Agip?"])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Hibba Agip", args!["What is it? If you have too much time on your hands, you'd better go outside and find something productive to do."])?;
                                                                            ctx.call(
                                                                                Function::Cutin,
                                                                                vec![Val::from("ep13_captin_edq"), Val::from(255)],
                                                                            )?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn hibba_agip(ctx: &Ctx) -> Script {
    hibba_agip_body(ctx, Vec::new()).map(|_| ())
}
