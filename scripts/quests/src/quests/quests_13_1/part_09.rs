use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Ep13ManFild01MonEdqStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnTimer600000,
    OnMyMobDead,
}

fn ep13_man_fild01_mon_edq_run(ctx: &Ctx, mut step: Ep13ManFild01MonEdqStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13ManFild01MonEdqStep::Start => {
                step = Ep13ManFild01MonEdqStep::OnInit;
                continue 'machine;
            }
            Ep13ManFild01MonEdqStep::OnInit => {
                ctx.call(Function::EnableNpc, vec![Val::from("ep13_man_fild01_mon_edq")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_man_fild01_mon_edq::OnEnable")])?;
                return Err(Stop::End);
            }
            Ep13ManFild01MonEdqStep::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("man_fild01"),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Nepenthes"),
                        Val::from(1988),
                        Val::from(7),
                        Val::from("ep13_man_fild01_mon_edq::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("man_fild01"),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Hillslion"),
                        Val::from(1989),
                        Val::from(7),
                        Val::from("ep13_man_fild01_mon_edq::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Ep13ManFild01MonEdqStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("man_fild01"), Val::from("ep13_man_fild01_mon_edq::OnMyMobDead")],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("ep13_man_fild01_mon_edq")])?;
                return Err(Stop::End);
            }
            Ep13ManFild01MonEdqStep::OnTimer600000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("man_fild01"), Val::from("ep13_man_fild01_mon_edq::OnMyMobDead")],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_man_fild01_mon_edq::OnEnable")])?;
                return Err(Stop::End);
            }
            Ep13ManFild01MonEdqStep::OnMyMobDead => {
                if (ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("man_fild01"), Val::from("ep13_man_fild01_mon_edq::OnMyMobDead")],
                    )?
                    .number()?
                    < 14
                    && (ctx.var("ep13_1_edq").get()? == 71 || ctx.var("ep13_1_edq").get()? == 72))
                {
                    ctx.call(Function::GetItem, vec![Val::from(6040), Val::from(1)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn ep13_man_fild01_mon_edq(ctx: &Ctx) -> Script {
    ep13_man_fild01_mon_edq_run(ctx, Ep13ManFild01MonEdqStep::Start, Vec::new()).map(|_| ())
}

pub fn ep13_man_fild01_mon_edq_oninit(ctx: &Ctx) -> Script {
    ep13_man_fild01_mon_edq_run(ctx, Ep13ManFild01MonEdqStep::OnInit, Vec::new()).map(|_| ())
}

pub fn ep13_man_fild01_mon_edq_onenable(ctx: &Ctx) -> Script {
    ep13_man_fild01_mon_edq_run(ctx, Ep13ManFild01MonEdqStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn ep13_man_fild01_mon_edq_ondisable(ctx: &Ctx) -> Script {
    ep13_man_fild01_mon_edq_run(ctx, Ep13ManFild01MonEdqStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn ep13_man_fild01_mon_edq_ontimer600000(ctx: &Ctx) -> Script {
    ep13_man_fild01_mon_edq_run(ctx, Ep13ManFild01MonEdqStep::OnTimer600000, Vec::new()).map(|_| ())
}

pub fn ep13_man_fild01_mon_edq_onmymobdead(ctx: &Ctx) -> Script {
    ep13_man_fild01_mon_edq_run(ctx, Ep13ManFild01MonEdqStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ep13ManFild03MonEdqStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnTimer600000,
    OnMyMobDead,
}

fn ep13_man_fild03_mon_edq_run(ctx: &Ctx, mut step: Ep13ManFild03MonEdqStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13ManFild03MonEdqStep::Start => {
                step = Ep13ManFild03MonEdqStep::OnInit;
                continue 'machine;
            }
            Ep13ManFild03MonEdqStep::OnInit => {
                ctx.call(Function::EnableNpc, vec![Val::from("ep13_man_fild03_mon_edq")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_man_fild03_mon_edq::OnEnable")])?;
                return Err(Stop::End);
            }
            Ep13ManFild03MonEdqStep::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("man_fild03"),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Centipede"),
                        Val::from(1987),
                        Val::from(7),
                        Val::from("ep13_man_fild03_mon_edq::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("man_fild03"),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Tatacho"),
                        Val::from(1986),
                        Val::from(7),
                        Val::from("ep13_man_fild03_mon_edq::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Ep13ManFild03MonEdqStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("man_fild03"), Val::from("ep13_man_fild03_mon_edq::OnMyMobDead")],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("ep13_man_fild03_mon_edq")])?;
                return Err(Stop::End);
            }
            Ep13ManFild03MonEdqStep::OnTimer600000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("man_fild03"), Val::from("ep13_man_fild03_mon_edq::OnMyMobDead")],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_man_fild03_mon_edq::OnEnable")])?;
                return Err(Stop::End);
            }
            Ep13ManFild03MonEdqStep::OnMyMobDead => {
                if (ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("man_fild03"), Val::from("ep13_man_fild03_mon_edq::OnMyMobDead")],
                    )?
                    .number()?
                    < 14
                    && (ctx.var("ep13_1_edq").get()? == 71 || ctx.var("ep13_1_edq").get()? == 72))
                {
                    ctx.call(Function::GetItem, vec![Val::from(6040), Val::from(1)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn ep13_man_fild03_mon_edq(ctx: &Ctx) -> Script {
    ep13_man_fild03_mon_edq_run(ctx, Ep13ManFild03MonEdqStep::Start, Vec::new()).map(|_| ())
}

pub fn ep13_man_fild03_mon_edq_oninit(ctx: &Ctx) -> Script {
    ep13_man_fild03_mon_edq_run(ctx, Ep13ManFild03MonEdqStep::OnInit, Vec::new()).map(|_| ())
}

pub fn ep13_man_fild03_mon_edq_onenable(ctx: &Ctx) -> Script {
    ep13_man_fild03_mon_edq_run(ctx, Ep13ManFild03MonEdqStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn ep13_man_fild03_mon_edq_ondisable(ctx: &Ctx) -> Script {
    ep13_man_fild03_mon_edq_run(ctx, Ep13ManFild03MonEdqStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn ep13_man_fild03_mon_edq_ontimer600000(ctx: &Ctx) -> Script {
    ep13_man_fild03_mon_edq_run(ctx, Ep13ManFild03MonEdqStep::OnTimer600000, Vec::new()).map(|_| ())
}

pub fn ep13_man_fild03_mon_edq_onmymobdead(ctx: &Ctx) -> Script {
    ep13_man_fild03_mon_edq_run(ctx, Ep13ManFild03MonEdqStep::OnMyMobDead, Vec::new()).map(|_| ())
}

fn pursuit_party_leader_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    let mut l_pattern_s: Vec<Val> = Vec::new();
    let mut l_quest = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(714), Val::from(3)])? == 0 {
        ctx.mes("- You cannot proceed with the quest when you're carrying too many items with you. -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("mao_morocc2").get()? == 0 {
        if (ctx.var("ep13_ryu").get()? == 100 || ctx.var("ep13_start").get()? == 100) {
            ctx.lines_as(
                "Echinacea",
                args![
                    "Oh adventurer, you've come at the perfect time.",
                    "Since you and I both know that we can't waste time on idle chitchat,",
                    "I'll cut to the chase."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Echinacea",
                args![
                    "Most expeditions come to this area to explore the Ash Vacuum,",
                    "but we've come here for a different reason."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Echinacea",
                args!["You know about this space gap that was caused by Satan Morocc, don't you?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Echinacea",
                args![
                    "Our job is to pursue Satan Morocc and figure out how this all happened.",
                    "Your job is to assist us, and prove that you can be a good member of our expedition."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Echinacea",
                args![
                    "... Umm... So, you'll have to.. Let me think which job would be perfect for you.",
                    "Umm.. I remember someone said that he needed assistance..."
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Hey, excuse me.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Echinacea",
                args![
                    "Ah, I know what you can do!",
                    "There's a guy conducting an investigation near the space gap. You can go help him.",
                    "Just do what he asks you to do, alright?",
                    "Now go!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Echinacea", args!["......", "By the way, adventurer,", "What's your name?"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "......",
                    "......",
                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                    "......"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Echinacea",
                args![
                    "Ah, you are",
                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                    ((Val::from("Well, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                        + Val::from(", I've just assigned you as Mr. Kidd's assistant.")),
                    "Good, everything's done on this end. I just needed to get your name on record."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Echinacea",
                args![
                    "What are you still doing here?",
                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                        + Val::from(", your job is to assist Mr. Kidd.")),
                    "Please go help him, will you?"
                ],
            )?;
            ctx.var("mao_morocc2").set(Val::from(1))?;
            ctx.call(Function::SetQuest, vec![Val::from(7012)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Echinacea",
                args![
                    "What are you doing here?",
                    "Only authorized personnel can enter this area.",
                    "Go back to the mainland as quickly as possible."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("mao_morocc2").get()? == 1 {
            ctx.lines_as(
                "Echinacea",
                args![
                    "Your job is to assist Mr. Kidd who's standing near the space gap, otherwise known as the gate.",
                    "Please go help him, okay?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("mao_morocc2").get()? == 2 {
                ctx.lines_as(
                    "Echinacea",
                    args![
                        "Why are you back?",
                        "You're irresponsible. How could you abandon such an important duty?",
                        "If you don't want to work, why did you even come here to the Ash Vacuum in the first place?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Echinacea",
                    args![
                        "Alas! There's no one I can trust!",
                        "You know, when I was working at the palace, I was.. *Blah Blah*..."
                    ],
                )?;
                ctx.next()?;
                ctx.mes("- Her rant doesn't seem to be ending any time soon. You'd better go back and help Mr. Kidd, even if your heart's not in it. -")?;
                ctx.next()?;
                ctx.var("mao_morocc2").set(Val::from(3))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(7013), Val::from(7014)])?;
                ctx.next()?;
                ctx.lines_as("Echinacea", args!["Hm? Where are you going? I'm not finished!", "Hey, hey!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if (ctx.var("mao_morocc2").get()?.number()? > 2 && ctx.var("mao_morocc2").get()?.number()? < 13) {
                    ctx.lines_as(
                        "Echinacea",
                        args![
                            "Mr. Kidd is such a great young man.",
                            "You should be thankful that I've assigned you to help him. Don't be lazy by neglecting your duties."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("mao_morocc2").get()? == 13 {
                        ctx.lines_as(
                            "Echinacea",
                            args![
                                "Oh, welcome back!",
                                "I knew you would successfully complete your duties.",
                                "So, tell me; how can I help you?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Echinacea",
                            args![
                                "A scholar who is familiar with magic?",
                                "What about that old man over there? If you couldn't tell, he comes from a distant country.",
                                "Why are you looking for a scholar, anyway?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Echinacea",
                            args![
                                "Did Mr. Kidd ask you to find one?",
                                "I see. He's probably looking for a solution for the spell scroll.",
                                "Then yes, that old man would be the perfect person to ask."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Echinacea",
                            args![
                                "He's from Arunafeltz, and has extensive knowledge in magic as well as sacred power.",
                                "I'm sure he'll be able to help you."
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Who?")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Echinacea",
                            args![
                                "Do you see an old man over there? He's trying to make a fire with wet firewood to roast a sweet potato.",
                                "His name is Mr. ^4d4dffDefaria^000000."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Echinacea",
                            args!["Why don't you go show it to him? He might be able to help Mr. Kidd with what he is trying to do."],
                        )?;
                        ctx.var("mao_morocc2").set(Val::from(14))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(7022), Val::from(7023)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("mao_morocc2").get()? == 14 {
                            ctx.lines_as(
                                "Echinacea",
                                args![
                                    "If you're seeking advice about the spell scroll, you should go talk to Mr. Defaria.",
                                    "I don't practice magic, so I wouldn't know anything about it."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if (ctx.var("mao_morocc2").get()?.number()? > 22 && ctx.var("mao_morocc2").get()?.number()? < 29) {
                                if ctx.var("mao_morocc2").get()?.number()? < 26 {
                                    ctx.lines_as(
                                        "Echinacea",
                                        args!["Oh, welcome back.", "How can I help you?", "Oh... You've brought the journal.."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Echinacea", args![".........", "......", "..."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Echinacea",
                                        args![
                                            "I see...",
                                            "...Basically it all comes down to Rayan.",
                                            "He's been abducted by Satan Morocc.",
                                            "If we find Rayan using this location-tracing spell."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    let choice = runtime::select_values(ctx, &[Val::from("We'll find Satan Morocc too!")])?;
                                    ctx.var("@menu").set(choice)?;
                                    ctx.lines_as(
                                        "Echinacea",
                                        args!["That's right.", "But it'll work only if Satan Morocc actually abducted Rayan."],
                                    )?;
                                    ctx.next()?;
                                }
                                ctx.lines_as("Echinacea", args!["Say, what's the tracing pattern number?"])?;
                                ctx.next()?;
                                let (input, status) = runtime::input_text(ctx, None, None)?;
                                l_input_s = input;
                                let base = Val::from(0).number()?;
                                runtime::local_set(&mut l_pattern_s, &Val::from(base + 0), Val::from("SDHF92F-SDF"), true);
                                runtime::local_set(&mut l_pattern_s, &Val::from(base + 1), Val::from("VWNM94GVWN90"), true);
                                runtime::local_set(&mut l_pattern_s, &Val::from(base + 2), Val::from("CM3-TRDFGHE0"), true);
                                l_quest = ctx.var("mao_morocc2").get()?;
                                if l_quest.clone().number()? > 25 {
                                    l_quest = (l_quest.clone().try_sub(Val::from(3))?);
                                }
                                ctx.lines_as("Echinacea", args![((Val::from("[") + l_input_s.clone()) + Val::from("]"))])?;
                                if l_input_s.clone().loosely_equals(&runtime::local_get(
                                    &l_pattern_s,
                                    &(l_quest.clone().try_sub(Val::from(23))?),
                                    true,
                                )) {
                                    ctx.mes("...Ah, there it is.")?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Echinacea",
                                        args![
                                            "This location-tracing spell uses a pair of magic stones of the same wavelength...",
                                            "We can use the spell with one stone to find the other stone with the matching wavelength."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Echinacea",
                                        args![
                                            "We'll analyze this pattern number and detect the wavelength.",
                                            "It'll take some time before we get the results."
                                        ],
                                    )?;
                                    if ctx.call(Function::CountItem, vec![Val::from(6029)])?.number()? > 0 {
                                        ctx.call(
                                            Function::DelItem,
                                            vec![Val::from(6029), ctx.call(Function::CountItem, vec![Val::from(6029)])?],
                                        )?;
                                    }
                                    ctx.var("mao_morocc2").set(Val::from(29))?;
                                    ctx.call(
                                        Function::ChangeQuest,
                                        vec![(Val::from(7031) + (l_quest.clone().try_sub(Val::from(23))?)), Val::from(7034)],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines(args![
                                        "It's not working for some reason.",
                                        "I can't find a magic stone that has the same wavelength as yours."
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Echinacea",
                                        args![
                                            "This location-tracing spell uses a pair of magic stones of the same wavelength..",
                                            "I can't find a magic stone that has the same wavelength as your pattern number."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Echinacea", args!["Go check the number again."])?;
                                    if ctx.var("mao_morocc2").get()?.number()? < 26 {
                                        ctx.var("mao_morocc2").set((l_quest.clone() + Val::from(3)))?;
                                    }
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            } else {
                                if ctx.var("mao_morocc2").get()? == 29 {
                                    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 2 {
                                        ctx.lines_as(
                                            "Echinacea",
                                            args![
                                                "Welcome, adventurer.",
                                                "I hope they've been able to trace the wavelength by now.",
                                                "How'd you like to go check the results?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Echinacea",
                                            args![
                                                "To the south of this camp, you'll see a building with a mana detector attached to it.",
                                                "Go ask the building manager about the trace results."
                                            ],
                                        )?;
                                        ctx.var("mao_morocc2").set(Val::from(30))?;
                                        ctx.call(Function::ChangeQuest, vec![Val::from(7034), Val::from(7035)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as(
                                            "Echinacea",
                                            args![
                                                "Tracing magic wavelength is not easy as you think.",
                                                "Please be patient: the results will arrive soon enough."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                } else {
                                    if ctx.var("mao_morocc2").get()? == 30 {
                                        ctx.lines_as(
                                            "Echinacea",
                                            args![
                                                "I'm in charge of tracing the wavelength from the Midgard Continent,",
                                                "and the south camp is tracing it in this area."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Echinacea", args!["I asked them to assign top priority to this task, so with any luck, they've finished it by now.", "..What are you doing? I said, go ask the building manager about the trace results."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("mao_morocc2").get()? == 31 {
                                        ctx.lines_as("Echinacea", args!["How are the results?"])?;
                                        ctx.next()?;
                                        ctx.mes("- You told her exactly what you were told by the soldier managing the building. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Echinacea",
                                            args![
                                                "Oh, is that the case? I see.",
                                                "That means.. The man called Rayan isn't on the Midgard Continent or in this Ash Vacuum."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Echinacea",
                                            args!["He must be somewhere else.", "As I suspected, he must be with Satan Morocc."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Echinacea",
                                            args![
                                                "Pursuing Satan Morocc was supposed to be our last resort..",
                                                "...I guess we have no choice now.."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Echinacea",
                                            args![
                                                "...I should log a report entry into this journal.",
                                                "Our pursuit of Satan Morocc is at an end for now."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Echinacea",
                                            args![
                                                "You did a great job helping us, and went above and beyond my expectations.",
                                                "Here, this is a reward for your help."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Echinacea",
                                            args!["Keep up the good work assisting the explorers in this camp."],
                                        )?;
                                        ctx.var("mao_morocc2").set(Val::from(100))?;
                                        ctx.call(Function::GetExperience, vec![Val::from(1200000), Val::from(200000)])?;
                                        ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
                                        ctx.call(Function::CompleteQuest, vec![Val::from(7036)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("mao_morocc2").get()? == 100 {
                                        ctx.lines_as("Echinacea", args!["...."])?;
                                        ctx.next()?;
                                        let choice = runtime::select_values(ctx, &[Val::from(".....")])?;
                                        ctx.var("@menu").set(choice)?;
                                        ctx.lines_as("Echinacea", args!["Hmm.."])?;
                                        ctx.next()?;
                                        let choice = runtime::select_values(ctx, &[Val::from("... . ..")])?;
                                        ctx.var("@menu").set(choice)?;
                                        ctx.lines_as(
                                            "Echinacea",
                                            args![
                                                "...Grr...!",
                                                "What?! What do you want?",
                                                "Why do you keep giving me that dirty look?!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        match runtime::select_values(ctx, &[Val::from("Nothing.:Something is bothering me...")])? {
                                            1 => {
                                                ctx.lines_as(
                                                    "Echinacea",
                                                    args![
                                                        "Argh...!",
                                                        "Are you annoying me on purpose?",
                                                        "Go and find something else to do!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Echinacea", args!["I'm stressed out and I've got a headache writing this report. I don't have the time to deal with you right now.", "Do you understand?"])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                ctx.lines_as(
                                                    "Echinacea",
                                                    args!["What is it that worries you?", "Please, I've got enough worries already."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Echinacea",
                                                    args![
                                                        "So, tell me, what is it?",
                                                        (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                                            + Val::from(", you've done a great job helping us...")),
                                                        "Oh, you must have questions about that last case, is that right?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Echinacea",
                                                    args![
                                                        "...Wait, are you worried because of him?",
                                                        "...I see, you're worried about him being supported by the country."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Echinacea", args!["This isn't a good time to disclose the truth. It'll cause more confusion.", "I believe the diplomats and presidents of both countries are responsible for those national matters."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Echinacea",
                                                    args!["This is out of your hands, adventurer.", "So stop worrying about it. Alright?"],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Echinacea",
                                                    args![
                                                        "No country wants to start a war, especially when we have a greater common enemy.",
                                                        "Just focus on what you can do to help us."
                                                    ],
                                                )?;
                                                ctx.var("mao_morocc2").set(Val::from(101))?;
                                                ctx.call(Function::SetQuest, vec![Val::from(7037)])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            _ => {}
                                        }
                                    } else if (ctx.var("mao_morocc2").get()?.number()? > 100
                                        && ctx.var("mao_morocc2").get()?.number()? < 106)
                                    {
                                        ctx.lines_as(
                                            "Echinacea",
                                            args![
                                                "We've stopped pursuing Satan Morocc, so you may go help the other explorers now.",
                                                "I've already rewarded you for your services."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Echinacea", args!["Remember; don't open your mouth about this to anyone."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("mao_morocc2").get()? == 106 {
                                        if ctx.call(Function::CountItem, vec![Val::from(549)])?.number()? > 1 {
                                            ctx.lines_as(
                                                "Echinacea",
                                                args![
                                                    "What is it? I thought you've been discharged from my service.",
                                                    "..What are these sweet potatoes?",
                                                    "Did that old man send these?",
                                                    "No way..! Are you..?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            let choice = runtime::select_values(ctx, &[Val::from("That's right.")])?;
                                            ctx.var("@menu").set(choice)?;
                                            ctx.lines_as(
                                                "Echinacea",
                                                args![
                                                    "What do you not understand?",
                                                    "Nothing good can come out of this if it's released to the public.",
                                                    "Can't you see?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Echinacea",
                                                args![
                                                    "Arunafeltz has replaced their leaders with the moderates just recently.",
                                                    "Shoving the remnants of the radicals in front of them is extremely dangerous."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Echinacea", args!["We've finally made a truce with them.", "Their sins will not be forgiven, but we won't make a stupid attempt to get revenge and end up shedding more blood!"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Echinacea", args!["Let the politicians do their job.", "I assure you they won't lead us to the worst case scenario. That's what you're worried about, right?"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Echinacea",
                                                args!["No, wait.. Do you..?", "Do you want us to wage war against Arunafeltz over this?!"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Echinacea",
                                                args![
                                                    "Let me ask you to forget the past and focus on the present.",
                                                    "For everyone's sake, please just look toward the future and move forward."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Echinacea",
                                                args![
                                                    "We won't do anything that you'd morally oppose.",
                                                    "You believe in peace, don't you?",
                                                    "Tell him that I'm grateful for these sweet potatoes, anyway.."
                                                ],
                                            )?;
                                            ctx.call(Function::DelItem, vec![Val::from(549), Val::from(2)])?;
                                            ctx.var("mao_morocc2").set(Val::from(107))?;
                                            ctx.call(Function::ChangeQuest, vec![Val::from(7040), Val::from(7041)])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["Err? I don't have two Nice Sweet Potatoes..", "Where can I get those?"],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                    } else {
                                        if ctx.var("mao_morocc2").get()?.number()? > 106 {
                                            ctx.lines_as("Echinacea", args!["Let us take care of the rest.", "I'm not suggesting that we should pretend as if the past never happened. All I'm saying is that we should work towards achieving a greater cause."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            ctx.lines_as(
                                                "Echinacea",
                                                args![
                                                    "You're doing a great job.",
                                                    "I suppose I have an eye for people after all.",
                                                    "Yes, that must be right.",
                                                    "Keep up the good work, will you?"
                                                ],
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
    Ok(Val::from(0))
}

pub fn pursuit_party_leader_1(ctx: &Ctx) -> Script {
    pursuit_party_leader_1_body(ctx, Vec::new()).map(|_| ())
}

fn manager_moc2_finder_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Manager", args!["Everything's okay so far!", "How may I help you?"])?;
    ctx.next()?;
    ctx.mes("[Manager]")?;
    if ctx.var("mao_morocc2").get()? == 30 {
        ctx.lines(args!["Everything's okay so far!", "How may I help you?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Manager",
            args!["Did Captain Echinacea send you?", "I have something to report to you."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manager",
            args![
                "The magic wavelength of the requested pattern number has not been detected in this area.",
                "It's not possible to trace the number around this area, but.."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("But?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Manager",
            args![
                "I can tell this magic pattern is supposed to be used to trace a human being.",
                "You know tracing magic stones always come in pairs, don't you?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manager",
            args!["Now, magic stones specifically used to trace humans have a special feature."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manager",
            args![
                "When the target of the trace dies, the pair of stones are automatically destroyed.",
                "Since this patterned magic stone is still intact.."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manager",
            args![
                "Well, this magic stone serves as proof that the target of the trace is still alive somewhere in the world.",
                "I'll keep watching.",
                "Hopefully, we'll detect the same wavelength."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Manager", args!["That's all for today."])?;
        ctx.var("mao_morocc2").set(Val::from(31))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(7035), Val::from(7036)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mao_morocc2").get()? == 31 {
        ctx.lines(args!["Everything's okay so far!", "How may I help you?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Manager",
            args![
                "Please report this to Captain Echinacea;",
                "We've failed to trace the location of the target,",
                "but he's still alive somewhere."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "This device detects mana and magic wavelengths around this area. identifying what enemies we're dealing with.",
            "It can be used to trace the wavelengths of location-tracing spells."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Manager",
            args![
                "It's a fantastic product made from a combination of magic and science.",
                "The Ash Vacuum hasn't been fully explored, and we don't know what kind of dangers to expect in this area."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manager",
            args![
                "By using this device, we can check the change and flow of mana around this area,",
                "and prepare for any possible threats."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn manager_moc2_finder(ctx: &Ctx) -> Script {
    manager_moc2_finder_body(ctx, Vec::new()).map(|_| ())
}

fn mr_kidd_ep13_dan01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(714), Val::from(1)])? == 0 {
        ctx.mes("- You cannot proceed with the quest when you're carrying too many items with you. -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("mao_morocc2").get()? == 1 {
        ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
        ctx.lines_as(
            "Mr. Kidd",
            args![
                "Hey, you.",
                "Don't get too close to anything except the gate.",
                "It's too dangerous."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Mr. Kidd", args!["......Do you have any business here?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Echinacea...:No.")])? {
            1 => {
                ctx.lines_as(
                    "Mr. Kidd",
                    args![
                        "An assistant? I don't think I need one, but...",
                        "I guess I should give you something to do.",
                        "Do you mind if I entrust you with a chore?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Yes.:No, I don't mind at all.")])? {
                    1 => {
                        ctx.lines_as(
                            "Mr. Kidd",
                            args![
                                "Then why did you come all this way?",
                                "I didn't really need any help to begin with. If you don't want to work, then stop bothering me."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Kidd",
                            args!["If you don't want to help us out then go back to Echinacea and let her know yourself!"],
                        )?;
                        ctx.var("mao_morocc2").set(Val::from(2))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(7012), Val::from(7013)])?;
                        ctx.close_window()?;
                    }
                    2 => {
                        ctx.lines_as(
                            "Mr. Kidd",
                            args![
                                "......",
                                "I don't know if Echinacea told you this already, but we are here",
                                "to follow the traces of Satan Morocc.",
                                "Some of us are here in the Ash Vacuum, and the rest are on the Midgard Continent."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Mr. Kidd", args!["We exchange information with our colleagues in Morocc on a regular basis, but we haven't heard from them for a while.", "How'd you like to go gather some information for me?", "It should be simple, so you I hope you'll be able to handle this."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Kidd",
                            args![
                                "At Southwest Morocc, you'll find an underground tavern.",
                                "Go find ^4d4dffRin^000000.",
                                "Tell her I've sent you, and she'll take care of you."
                            ],
                        )?;
                        ctx.var("mao_morocc2").set(Val::from(4))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(7012), Val::from(7015)])?;
                        ctx.close_window()?;
                    }
                    _ => {}
                }
            }
            2 => {
                ctx.lines_as(
                    "Mr. Kidd",
                    args![
                        "If you don't have any business here, then step aside. It's too dangerous.",
                        "You might not be able to return to this world if you enter a route that hasn't been properly identified."
                    ],
                )?;
                ctx.close_window()?;
            }
            _ => {}
        }
    } else {
        if ctx.var("mao_morocc2").get()? == 2 {
            ctx.call(Function::Cutin, vec![Val::from("moc2_kid02"), Val::from(2)])?;
            ctx.lines_as(
                "Mr. Kidd",
                args![
                    "I guess she forced you to do this, huh? Don't worry; you're not the first victim of her bossiness.",
                    "You should tell her if you really don't want to work for me."
                ],
            )?;
            ctx.close_window()?;
        } else {
            if ctx.var("mao_morocc2").get()? == 3 {
                ctx.call(Function::Cutin, vec![Val::from("moc2_kid05"), Val::from(2)])?;
                ctx.lines_as(
                    "Mr. Kidd",
                    args![
                        "I can tell you aren't so happy with this, huh?",
                        "While you're here, why don't you help us out and see where it goes?",
                        "Don't worry, I won't give you a hard time..."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                ctx.lines_as(
                    "Mr. Kidd",
                    args![
                        "......",
                        "I don't know if Echinacea told you this already, but we are here to follow the traces of Satan Morocc."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Kidd",
                    args!["Some of us are here in the Ash Vacuum, and the rest are on the Midgard Continent."],
                )?;
                ctx.next()?;
                ctx.lines_as("Mr. Kidd", args!["We exchange information with our colleagues in Morocc on a regular basis, but we haven't heard from them for a while."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Kidd",
                    args![
                        "How'd you like to go gather some information for me?",
                        "It should be simple, so you I hope you'll be able to handle this."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Kidd",
                    args![
                        "At Southwest Morocc, you'll find an underground tavern.",
                        "Go find ^4d4dffRin^000000.",
                        "Tell her I've sent you, and she'll take care of you."
                    ],
                )?;
                ctx.var("mao_morocc2").set(Val::from(4))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(7014), Val::from(7015)])?;
                ctx.close_window()?;
            } else {
                if (ctx.var("mao_morocc2").get()? == 4 || ctx.var("mao_morocc2").get()? == 5) {
                    ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                    ctx.lines_as(
                        "Mr. Kidd",
                        args![
                            "Let me explain one more time.",
                            "Go speak to Rin in an underground tavern at Southwest Morocc.",
                            "You won't miss the tavern if you look for a boy named Jack standing out the front."
                        ],
                    )?;
                    ctx.close_window()?;
                } else {
                    if (ctx.var("mao_morocc2").get()?.number()? > 5 && ctx.var("mao_morocc2").get()?.number()? < 9) {
                        ctx.call(Function::Cutin, vec![Val::from("moc2_kid03"), Val::from(2)])?;
                        ctx.lines_as("Mr. Kidd", args!["Are you helping Rin?", "..I see."])?;
                        ctx.close_window()?;
                    } else {
                        if ctx.var("mao_morocc2").get()? == 9 {
                            if (ctx.call(Function::CountItem, vec![Val::from(6029)])? == 1
                                && ctx.call(Function::CountItem, vec![Val::from(6027)])? == 1)
                            {
                                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                                ctx.lines_as("Mr. Kidd", args!["I heard Rin is wounded and bedridden...", "What happened?"])?;
                                ctx.next()?;
                                ctx.mes("- Instead of an answer, you gave him Rin's journal and the bloody crystal.-")?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Mr. Kidd",
                                    args![
                                        "...A journal? I see. Hopefully this'll help me understand the situation..",
                                        "Hmm..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.mes("......")?;
                                ctx.next()?;
                                ctx.lines(args!["......", "............"])?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("moc2_kid03"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Mr. Kidd",
                                    args![
                                        "*Sigh*..",
                                        "So Rayan has finally been located, huh?",
                                        "There was a skirmish at the mountain, and.. a raid...",
                                        "..So, how is she?"
                                    ],
                                )?;
                                ctx.next()?;
                                let choice = runtime::select_values(ctx, &[Val::from("Huh?")])?;
                                ctx.var("@menu").set(choice)?;
                                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                                ctx.lines_as("Mr. Kidd", args!["I mean Rin. How is she?", "Is she seriously wounded?"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "She needs to spend some time in bed,",
                                        "but she's conscious and still in one piece."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["She seems to have no problem taking care of herself,", "but I'm worried because sometimes she looks extremely exhausted. She mentioned that her vision blurs when she's very tired.."])?;
                                ctx.next()?;
                                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Don't worry; I don't think it's serious because she still had the strength to yell while writing that journal."])?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("moc2_kid03"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Mr. Kidd",
                                    args![
                                        "..Wh.. Who said I'm worried?",
                                        "I asked about her because I don't want her injury to interfere with our schedule."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Mr. Kidd",
                                    args![
                                        "As for you, I want you to come back later.",
                                        "You're my assistant, remember?",
                                        "I need you to substitute for Rin until she recovers..",
                                        "Is that understood?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("moc2_kid05"), Val::from(2)])?;
                                ctx.lines_as("Mr. Kidd", args!["Hm, thanks for all your help.", "Now, would you leave me alone for a while? I need to write a report based on the findings in this journal."])?;
                                ctx.call(Function::DelItem, vec![Val::from(6029), Val::from(1)])?;
                                ctx.call(Function::DelItem, vec![Val::from(6027), Val::from(1)])?;
                                ctx.var("mao_morocc2").set(Val::from(10))?;
                                ctx.call(Function::GetExperience, vec![Val::from(200000), Val::from(10000)])?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(7018), Val::from(7019)])?;
                                ctx.close_window()?;
                            } else {
                                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["(I was supposed to give him the 'Morocc Pursuit Journal' and a 'Bloody Crystal of the Darkness'...", "Err? Where have they gone? I need to find them.)"])?;
                                ctx.close_window()?;
                            }
                        } else {
                            if ctx.var("mao_morocc2").get()? == 10 {
                                ctx.lines(args![
                                    "Kidd is standing at the side of the space gap, leafing through the journal's pages and taking notes.",
                                    "The 'Bloody Crystal of Darkness' is sitting aside, and remains untouched."
                                ])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "I feel the crystal resonating with the space gap.",
                                    "I suddenly have the urge to touch the crystal..."
                                ])?;
                                ctx.next()?;
                                ctx.lines_as("Mr. Kidd", args!["Err? What are you trying to do?"])?;
                                ctx.next()?;
                                if ctx.var("$@moc_mao_gate1").get()? == 0 {
                                    ctx.call(Function::Warp, vec![Val::from("que_dan01"), Val::from(32), Val::from(27)])?;
                                    ctx.var("$@moc_mao_gate1").set(Val::from(1))?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["No, nothing.", "I'm sorry if I bothered you while you were writing your report."],
                                    )?;
                                    ctx.next()?;
                                    ctx.mes("- Kidd stopped you from touching the crystal, but somehow, it still captivates you. -")?;
                                    ctx.close_window()?;
                                }
                            } else {
                                if (ctx.var("mao_morocc2").get()? == 11 || ctx.var("mao_morocc2").get()? == 12) {
                                    ctx.call(Function::Cutin, vec![Val::from("moc2_kid02"), Val::from(2)])?;
                                    ctx.lines_as("Mr. Kidd", args!["..Are.. Are you alright?", "What just happened?"])?;
                                    ctx.next()?;
                                    let choice = runtime::select_values(ctx, &[Val::from("Err.. Huh?!")])?;
                                    ctx.var("@menu").set(choice)?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "..Err?",
                                            "..Kidd? Someone was calling my name...",
                                            "Rin! Where's Rin? What about Rayan?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Mr. Kidd",
                                        args!["Rin? Rayan?", "What are you talking about?", "Where the hell have you been?"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "What do you mean..",
                                            "Where I've been?",
                                            "..Oh yeah, I saw Rin fighting Rayan.",
                                            "She was defeated by magic, and then disappeared..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "Ah?! Ah! Right!",
                                            "I heard your voice once I found a strange scroll.",
                                            "The scroll!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.mes("- Ignoring the confused look on Mr. Kidd's face, you searched your pockets, and found the mysterious scroll. -")?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "- It wasn't a dream after all... What you saw actually happened. -",
                                        "- You show Kidd the scroll and tell him what happened. -"
                                    ])?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Mr. Kidd",
                                        args!["... ..", ".. Basically,", "you found this from a member of Rayan's gang?"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Mr. Kidd", args!["I know what this is:", "it's a scroll containing a magic spell that can be used by anyone, even if they aren't trained as mages."])?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("moc2_kid04"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Mr. Kidd",
                                        args![
                                            "Hmm, this scroll is sealed and needs some kind of pass code to use it.",
                                            "I'll consult with the camp scholars about this scroll later."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                                    ctx.lines_as("Mr. Kidd", args!["By the way, I saw you touch this 'Bloody Crystal of Darkness.'", "There was a bright flash of light around you, and then you were suddenly drawn into the space gap."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Mr. Kidd",
                                        args![
                                            "I thought I was just seeing things, but it's obvious now that it really happened.",
                                            "You were actually in the past for a little while there."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Mr. Kidd",
                                        args![
                                            "I'm afraid this space gap hides bigger secrets than we thought.",
                                            "..It not only warps space, but also time ..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    let choice = runtime::select_values(ctx, &[Val::from("It warps space and time?")])?;
                                    ctx.var("@menu").set(choice)?;
                                    ctx.call(Function::Cutin, vec![Val::from("moc2_kid03"), Val::from(2)])?;
                                    ctx.lines_as("Mr. Kidd", args!["Forget it. I was just thinking out loud.", "From now on, we should be extra careful about letting anything into this space gap. I need to report this incident to upper management."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Mr. Kidd", args!["I'm sorry, but can you ask a scholar about this spell scroll?", "If you don't know which scholar you should consult, then you'd better ^4d4dffask Echinacea^000000"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Mr. Kidd", args!["Thanks in advance."])?;
                                    ctx.var("mao_morocc2").set(Val::from(13))?;
                                    ctx.call(Function::ChangeQuest, vec![Val::from(7021), Val::from(7022)])?;
                                    ctx.close_window()?;
                                } else {
                                    if (ctx.var("mao_morocc2").get()? == 13 || ctx.var("mao_morocc2").get()? == 14) {
                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "Mr. Kidd",
                                            args![
                                                "Did you find a scholar to ask about the scroll?",
                                                "Huh? Not yet?",
                                                "If you don't know which scholar to consult, then you'd better ask Echinacea."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                    } else {
                                        if ctx.var("mao_morocc2").get()? == 15 {
                                            ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Mr. Kidd",
                                                args![
                                                    "So this scroll is from Arunafeltz, huh?",
                                                    "Is that why you're gathering items to unseal the scroll?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::Cutin, vec![Val::from("moc2_kid03"), Val::from(2)])?;
                                            ctx.lines_as("Mr. Kidd", args!["I see..."])?;
                                            ctx.next()?;
                                            ctx.lines(args![
                                                "- Kidd is lost in thought. -",
                                                "- Let's bring Defaria the items to unseal the spell scroll. -"
                                            ])?;
                                            ctx.close_window()?;
                                        } else {
                                            if ctx.var("mao_morocc2").get()? == 16 {
                                                ctx.call(Function::Cutin, vec![Val::from("moc2_kid05"), Val::from(2)])?;
                                                ctx.lines_as("Mr. Kidd", args!["Keep up the good work, adventurer."])?;
                                                ctx.close_window()?;
                                            } else {
                                                if ctx.var("mao_morocc2").get()? == 17 {
                                                    if ctx.call(Function::CountItem, vec![Val::from(14595)])?.number()? > 0 {
                                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                                                        ctx.lines_as(
                                                            "Mr. Kidd",
                                                            args![
                                                                "This must be the spell scroll, huh?",
                                                                "It contains a teleportation spell",
                                                                "that will send the user to a specific location.. Hmm..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.call(Function::Cutin, vec![Val::from("moc2_kid03"), Val::from(2)])?;
                                                        ctx.lines_as(
                                                            "Mr. Kidd",
                                                            args![
                                                                "You said you've heard Rayan ordering others to come to a certain place.",
                                                                "If the spell of this scroll leads to that place..",
                                                                ".... . No, it's too dangerous."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                                                        ctx.lines_as(
                                                            "Mr. Kidd",
                                                            args![
                                                                "Then again.. Somebody will have to go there eventually..",
                                                                "... .. .. ."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        let choice =
                                                            runtime::select_values(ctx, &[Val::from("Why are you looking at me?!")])?;
                                                        ctx.var("@menu").set(choice)?;
                                                        ctx.call(Function::Cutin, vec![Val::from("moc2_kid05"), Val::from(2)])?;
                                                        ctx.lines_as("Mr. Kidd", args!["You're my assistant, aren't you?"])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args![".... .....", "......"],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.call(Function::Cutin, vec![Val::from("moc2_kid04"), Val::from(2)])?;
                                                        ctx.lines_as("Mr. Kidd", args![".Alright, then. Would you rather capture reincarnations of Morocc and collect their remnants?", "Scholars suspect the recent space-time phenomenon has been caused by the 'Bloody Crystals of the Darkness,'", "so they want to study the crystals, which they believe are remnants of the of Morocc's reincarnations."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Mr. Kidd", args!["That sounds way more dangerous, doesn't it? Go check that place out instead.", "Both those missions are dangerous, but I think you'll be risking your life less by checking out that place."])?;
                                                        ctx.next()?;
                                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                                                        ctx.lines_as("Mr. Kidd", args!["Now, listen. Use the spell scroll to teleport there, and then investigate the area. Find some clues.", "Don't fight them.", "Don't put yourself in danger, and retreat when things get too dangerous."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Mr. Kidd", args!["By the time you come back,", "we should have figured out the cause of the strange phenomenon near this space gap.", "We'll think about everything else after you come back, alright?"])?;
                                                        ctx.close_window()?;
                                                    } else {
                                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                                                        ctx.lines_as("Mr. Kidd", args!["Where's the scroll? Did you use it?"])?;
                                                        ctx.next()?;
                                                        match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
                                                            1 => {
                                                                ctx.lines_as("Mr. Kidd", args!["That scroll contains a teleportation spell.", "Using it will lead you to a place that has something to do with them.", "You're supposed to gather some clues once you're there."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Mr. Kidd",
                                                                    args![
                                                                        "..I guess you haven't found any clues.",
                                                                        "Do you remember the location of that place?",
                                                                        "Why don't you go it check out again?"
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Mr. Kidd",
                                                                    args![
                                                                        "I'm sure you'll find something.",
                                                                        "Yes, there must be some sort of clue there.",
                                                                        "Do you understand?"
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                            }
                                                            2 => {
                                                                ctx.lines_as("Mr. Kidd", args!["That scroll contains a teleportation spell.", "Using it will lead you to a place that has something to do with them.", "You're supposed to gather some clues once you're there."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Mr. Kidd", args!["If you lost the scroll, I expect you to find that place at any cost.", "Is that clear?"])?;
                                                                ctx.close_window()?;
                                                            }
                                                            _ => {}
                                                        }
                                                    }
                                                } else {
                                                    if ctx.var("mao_morocc2").get()? == 18 {
                                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                                                        ctx.lines_as(
                                                            "Mr. Kidd",
                                                            args!["Did you find any clues?", "What did you find over there?"],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.mes("- You interrupted Mr. Kidd, and told him what you heard from the basement in the empty house near the national border. -")?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Mr. Kidd",
                                                            args!["..Are you sure?", "And the location is.. Okay, I got it.", "Thanks."],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.call(Function::Cutin, vec![Val::from("moc2_kid03"), Val::from(2)])?;
                                                        ctx.lines_as(
                                                            "Mr. Kidd",
                                                            args![
                                                                "Can you go back and spy on them a bit more?",
                                                                "I'll follow you as soon as I report these findings to the higher-ups."
                                                            ],
                                                        )?;
                                                        ctx.var("mao_morocc2").set(Val::from(19))?;
                                                        ctx.call(Function::ChangeQuest, vec![Val::from(7027), Val::from(7028)])?;
                                                        ctx.close_window()?;
                                                    } else {
                                                        if ctx.var("mao_morocc2").get()? == 19 {
                                                            ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                                                            ctx.lines_as(
                                                                "Mr. Kidd",
                                                                args![
                                                                    "They must be having a serious internal dispute,",
                                                                    "meaning it's the perfect time for us to raid them.",
                                                                    "Keep monitoring them for me, alright?"
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                        } else {
                                                            if ctx.var("mao_morocc2").get()? == 20 {
                                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                                                                ctx.lines_as(
                                                                    "Mr. Kidd",
                                                                    args!["..Phew, now I've seen everything.", "Oh, hey!"],
                                                                )?;
                                                                ctx.next()?;
                                                                let choice = runtime::select_values(ctx, &[Val::from("Where's Rayan?")])?;
                                                                ctx.var("@menu").set(choice)?;
                                                                ctx.call(Function::Cutin, vec![Val::from("moc2_kid03"), Val::from(2)])?;
                                                                ctx.lines_as("Mr. Kidd", args!["He's locked up in the basement, and Rin is interrogating him.", "You know, she's the best interrogator we've got. There'll be no escape for that guy .."])?;
                                                                ctx.next()?;
                                                                ctx.call(Function::Cutin, vec![Val::from("moc2_kid05"), Val::from(2)])?;
                                                                ctx.lines_as(
                                                                    "Mr. Kidd",
                                                                    args![
                                                                        "..Wait, what if the same thing happens..?",
                                                                        "No way, that's not possible.."
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Mr. Kidd",
                                                                    args![
                                                                        "The crystal no longer resonates with the space gap.",
                                                                        "Now it's exactly the same as a regular Crystal of Darkness."
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Mr. Kidd", args!["That means, the phenomenon that you've experienced with the 'Bloody Crystal of Darkness' can't be..", "........."])?;
                                                                ctx.next()?;
                                                                let choice =
                                                                    runtime::select_values(ctx, &[Val::from("Can't be explained?")])?;
                                                                ctx.var("@menu").set(choice)?;
                                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                                                                ctx.lines_as("Mr. Kidd", args!["..I guess not. It might have resonated to your mana or wavelength.", "Why don't you carry it with you? Let me know if you experience any strange phenomenon again."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Mr. Kidd",
                                                                    args![
                                                                        "Oh, and please bring ^4d4dffthis journal to Rin^000000.",
                                                                        "This will be your last assignment from me."
                                                                    ],
                                                                )?;
                                                                ctx.var("mao_morocc2").set(Val::from(21))?;
                                                                ctx.call(Function::GetItem, vec![Val::from(6027), Val::from(1)])?;
                                                                ctx.call(Function::GetItem, vec![Val::from(6029), Val::from(1)])?;
                                                                ctx.call(Function::ChangeQuest, vec![Val::from(7029), Val::from(7030)])?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Mr. Kidd", args!["..Do you have any questions?"])?;
                                                                ctx.next()?;
                                                                match runtime::select_values(
                                                                    ctx,
                                                                    &[Val::from(
                                                                        "Is this an exchange journal?:Why should I carry the crystal?:What is this last assignment?",
                                                                    )],
                                                                )? {
                                                                    1 => {
                                                                        ctx.call(
                                                                            Function::Cutin,
                                                                            vec![Val::from("moc2_kid04"), Val::from(2)],
                                                                        )?;
                                                                        ctx.lines_as(
                                                                            "Mr. Kidd",
                                                                            args![
                                                                                "..No, not at all.",
                                                                                "We're just using one journal to record each other's work."
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Why can't you use separate journals and then combine them later?"])?;
                                                                        ctx.next()?;
                                                                        ctx.call(
                                                                            Function::Cutin,
                                                                            vec![Val::from("mocseal_kid01"), Val::from(2)],
                                                                        )?;
                                                                        ctx.lines_as("Mr. Kidd", args!["......", "Ask Echinacea. I'm just doing what the boss tells me to do.", "Using one set journal is a good way to keep in contact and accumulate a history of work in chronological order."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Mr. Kidd",
                                                                            args![
                                                                                "We came up with the idea to share our experiences",
                                                                                "and understand any relation between incidents",
                                                                                "as quickly and easily as possible.."
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Mr. Kidd", args!["I know it's a little bit counterproductive, but I'm satisfied with the quality of information shared between us.", "The notes that Rin has absent-mindedly written down can turn out to be great clues for me.."])?;
                                                                        ctx.next()?;
                                                                        ctx.call(
                                                                            Function::Cutin,
                                                                            vec![Val::from("moc2_kid05"), Val::from(2)],
                                                                        )?;
                                                                        ctx.lines_as(
                                                                            "Mr. Kidd",
                                                                            args![
                                                                                "Yeah, I agree that the journal looks pretty disorganized."
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                    }
                                                                    2 => {
                                                                        ctx.call(
                                                                            Function::Cutin,
                                                                            vec![Val::from("moc2_kid04"), Val::from(2)],
                                                                        )?;
                                                                        ctx.lines_as(
                                                                            "Mr. Kidd",
                                                                            args!["... Intuition... I guess?", "... These kind of things."],
                                                                        )?;
                                                                        ctx.next()?;
                                                                    }
                                                                    3 => {
                                                                        ctx.call(
                                                                            Function::Cutin,
                                                                            vec![Val::from("mocseal_kid01"), Val::from(2)],
                                                                        )?;
                                                                        ctx.lines_as("Mr. Kidd", args!["Rayan has already been caught, so the task to locate him and Rin is already done.", "You were just an assistant to start with."])?;
                                                                        ctx.next()?;
                                                                    }
                                                                    _ => {}
                                                                }
                                                                ctx.call(Function::Cutin, vec![Val::from("moc2_kid03"), Val::from(2)])?;
                                                                ctx.lines_as("Mr. Kidd", args!["Anyways, please deliver the journal to her safely.", "I'm still not done yet; Echinacea already has another task for me..", "..*Sigh*.."])?;
                                                                ctx.close_window()?;
                                                            } else if ctx.var("mao_morocc2").get()? == 21 {
                                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                                                                ctx.lines_as(
                                                                    "Mr. Kidd",
                                                                    args![
                                                                        "I asked you to deliver the journal to Rin.",
                                                                        "You didn't forget it, did you?",
                                                                        "..."
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Mr. Kidd", args!["By the way, I'm still not convinced that we've captured the real Rayan.", "Will you tell Rin to write down the results of her interrogation in the journal?"])?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Mr. Kidd",
                                                                    args![
                                                                        "Of course, I'm pretty sure she'll do it without me reminding her."
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                            } else if (ctx.var("mao_morocc2").get()?.number()? > 21
                                                                && ctx.var("mao_morocc2").get()?.number()? < 31)
                                                            {
                                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(2)])?;
                                                                ctx.lines_as(
                                                                    "Mr. Kidd",
                                                                    args![
                                                                        "You don't have to report everything to me.",
                                                                        "I can check the journal later.",
                                                                        "Why don't you go assist Rin for now?"
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                            } else if (ctx.var("mao_morocc2").get()?.number()? > 30
                                                                && ctx.var("mao_morocc2").get()?.number()? < 104)
                                                            {
                                                                ctx.call(Function::Cutin, vec![Val::from("moc2_kid05"), Val::from(2)])?;
                                                                ctx.lines_as("Mr. Kidd", args!["..Things didn't turn out exactly as we hoped, but there's nothing we can do for now.", "Thank you for your service;", "you've been of great help to us."])?;
                                                                ctx.close_window()?;
                                                            } else if ctx.var("mao_morocc2").get()? == 104 {
                                                                if ctx.call(Function::CountItem, vec![Val::from(549)])?.number()? > 1 {
                                                                    ctx.call(
                                                                        Function::Cutin,
                                                                        vec![Val::from("mocseal_kid01"), Val::from(2)],
                                                                    )?;
                                                                    ctx.lines_as("Mr. Kidd", args!["What's up?", "I told you that I won't have any more work available for you. Well, for a good while, anyway.."])?;
                                                                    ctx.next()?;
                                                                    let choice = runtime::select_values(
                                                                        ctx,
                                                                        &[Val::from("Want a snack?:Mr. Defaria has sent me.")],
                                                                    )?;
                                                                    ctx.var("@menu").set(choice)?;
                                                                    ctx.call(Function::Cutin, vec![Val::from("moc2_kid05"), Val::from(2)])?;
                                                                    ctx.lines_as(
                                                                        "Mr. Kidd",
                                                                        args![
                                                                            "..I guess he's finally made a fire, huh?",
                                                                            "Tell him thanks for these sweet potatoes."
                                                                        ],
                                                                    )?;
                                                                    ctx.call(Function::DelItem, vec![Val::from(549), Val::from(2)])?;
                                                                    ctx.var("mao_morocc2").set(Val::from(105))?;
                                                                    ctx.next()?;
                                                                    match runtime::select_values(
                                                                        ctx,
                                                                        &[Val::from("Ask about Dandelion.:Ask about Arunafeltz.")],
                                                                    )? {
                                                                        1 => {
                                                                            ctx.call(
                                                                                Function::Cutin,
                                                                                vec![Val::from("moc2_kid03"), Val::from(2)],
                                                                            )?;
                                                                            ctx.lines_as("Mr. Kidd", args!["I'm indebted to them, but they all died before I had a chance to pay them back."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Mr. Kidd", args!["If you're asking about my political opinion, I'm not going to answer.", "This isn't something with which I can interfere."])?;
                                                                            ctx.next()?;
                                                                            ctx.call(
                                                                                Function::Cutin,
                                                                                vec![Val::from("moc2_kid04"), Val::from(2)],
                                                                            )?;
                                                                            ctx.lines_as("Mr. Kidd", args!["My personal opinion?", "Let me tell you that, from their point of view; their one last chance was destroyed.", "They're probably too ashamed to even talk about it."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Mr. Kidd", args!["Let our diplomats use that fact to their advantage, alright?"])?;
                                                                            ctx.close_window()?;
                                                                        }
                                                                        2 => {
                                                                            ctx.call(
                                                                                Function::Cutin,
                                                                                vec![Val::from("mocseal_kid01"), Val::from(2)],
                                                                            )?;
                                                                            ctx.lines_as(
                                                                                "Mr. Kidd",
                                                                                args![
                                                                                    "Nothing.",
                                                                                    "I have no comments.",
                                                                                    "No conflicts have arisen recently, either."
                                                                                ],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            ctx.call(
                                                                                Function::Cutin,
                                                                                vec![Val::from("moc2_kid04"), Val::from(2)],
                                                                            )?;
                                                                            ctx.lines_as("Mr. Kidd", args!["Besides, nationality isn't important to us.", "What's important is whether or not they're our enemies, and whether or not they're the target of our missions."])?;
                                                                            ctx.close_window()?;
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                } else {
                                                                    ctx.lines_as(
                                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                        args![
                                                                            "Err? I don't have two Nice Sweet Potatoes..",
                                                                            "Where can I get those?"
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                }
                                                            } else {
                                                                if ctx.var("mao_morocc2").get()?.number()? > 104 {
                                                                    ctx.call(
                                                                        Function::Cutin,
                                                                        vec![Val::from("mocseal_kid01"), Val::from(2)],
                                                                    )?;
                                                                    ctx.lines_as(
                                                                        "Mr. Kidd",
                                                                        args![
                                                                            "Don't try to understand everything.",
                                                                            "A mission is just a mission.",
                                                                            "Don't get too emotional or curious about it."
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                } else {
                                                                    ctx.call(
                                                                        Function::Cutin,
                                                                        vec![Val::from("mocseal_kid01"), Val::from(2)],
                                                                    )?;
                                                                    ctx.lines_as("Mr. Kidd", args!["...You'd better not approach the space gap off the official route.", "If you're not authorized, then turn away. This thing is too dangerous for capricious exploration."])?;
                                                                    ctx.close_window()?;
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
    ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn mr_kidd_ep13_dan01(ctx: &Ctx) -> Script {
    mr_kidd_ep13_dan01_body(ctx, Vec::new()).map(|_| ())
}

fn mr_kidd_ep13_dan01_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$@moc_mao_gate1").set(Val::from(0))?;
    return Err(Stop::End);
}

pub fn mr_kidd_ep13_dan01_oninit(ctx: &Ctx) -> Script {
    mr_kidd_ep13_dan01_oninit_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Moc2Talkevent01Step {
    Start,
    OnTouch,
}

fn moc2_talkevent01_run(ctx: &Ctx, mut step: Moc2Talkevent01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Moc2Talkevent01Step::Start => {
                step = Moc2Talkevent01Step::OnTouch;
                continue 'machine;
            }
            Moc2Talkevent01Step::OnTouch => {
                if ctx.var("mao_morocc2").get()? == 11 {
                    ctx.lines_as(
                        "Mr. Kidd",
                        args![
                            ((((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("? "))
                                + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from("?!")),
                            "Wake up!"
                        ],
                    )?;
                    ctx.var("mao_morocc2").set(Val::from(12))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn moc2_talkevent01(ctx: &Ctx) -> Script {
    moc2_talkevent01_run(ctx, Moc2Talkevent01Step::Start, Vec::new()).map(|_| ())
}

pub fn moc2_talkevent01_ontouch(ctx: &Ctx) -> Script {
    moc2_talkevent01_run(ctx, Moc2Talkevent01Step::OnTouch, Vec::new()).map(|_| ())
}
