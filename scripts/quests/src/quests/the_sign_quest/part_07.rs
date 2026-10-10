use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn valkyrie_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Valkyrie Sandra]")?;
    if ctx.var("sign_q").get()?.number()? < 81 {
        ctx.lines(args![
            "Only the chosen",
            "can enter this place.",
            "I will not ask how you've",
            "gained entrance, but I will",
            "require you to leave."
        ])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("gef_fild07"), Val::from(180), Val::from(242)])?;
        return Err(Stop::End);
    } else {
        if ctx.var("sign_q").get()? == 81 {
            ctx.lines(args![((Val::from("Welcome, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")), "to this realm of holiness.", "You have endured great difficulty and tested your courage to obtain the Sobbing Starlight, which will be the certificate for your test."])?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Test...?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Valkyrie Sandra",
                args![
                    "The gods have decided that",
                    "you are worthy of undergoing",
                    "a special test that will judge",
                    "your merits. If you are a true",
                    "hero who is pure of heart,",
                    "you will certainly succeed..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Valkyrie Sandra",
                args![
                    "Now, there is a critical",
                    "situation in the ^FF0000realm where",
                    "the fallen warriors dwell^000000. The",
                    "gods wish for you to restore the balance by quelling a specific",
                    "evil influence in that place."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Valkyrie Sandra",
                args![
                    "There will be many obstacles.",
                    "First, the fallen, who will target their anger and hatred towards",
                    "you. Where they have failed to",
                    "enter Valhalla, you have a rare",
                    "and wondrous opportunity..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Valkyrie Sandra",
                args![
                    "Secondly, and more",
                    "importantly, you must find",
                    "this evil influence on your",
                    "own. This will judge your",
                    "ability to discern good from evil. Do not trust appearances..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Valkyrie Sandra",
                args![
                    "This is your task.",
                    "I cannot tell you more.",
                    "It is now your duty to",
                    "travel there and ferret",
                    "out true darkness from",
                    "one of the hearts there..."
                ],
            )?;
            ctx.var("sign_q").set(Val::from(82))?;
            ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(100)])?;
            {
                if ctx.var("BaseLevel").get()?.number()? < 56 {
                    ctx.call(Function::GetExperience, vec![Val::from(9000), Val::from(0)])?;
                } else {
                    if (ctx.var("BaseLevel").get()?.number()? > 55 && ctx.var("BaseLevel").get()?.number()? < 61) {
                        ctx.call(Function::GetExperience, vec![Val::from(12000), Val::from(0)])?;
                    } else {
                        if (ctx.var("BaseLevel").get()?.number()? > 60 && ctx.var("BaseLevel").get()?.number()? < 66) {
                            ctx.call(Function::GetExperience, vec![Val::from(20000), Val::from(0)])?;
                        } else {
                            if (ctx.var("BaseLevel").get()?.number()? > 65 && ctx.var("BaseLevel").get()?.number()? < 71) {
                                ctx.call(Function::GetExperience, vec![Val::from(35000), Val::from(0)])?;
                            } else if (ctx.var("BaseLevel").get()?.number()? > 70 && ctx.var("BaseLevel").get()?.number()? < 76) {
                                ctx.call(Function::GetExperience, vec![Val::from(70000), Val::from(0)])?;
                            } else if (ctx.var("BaseLevel").get()?.number()? > 75 && ctx.var("BaseLevel").get()?.number()? < 81) {
                                ctx.call(Function::GetExperience, vec![Val::from(120000), Val::from(0)])?;
                            } else if (ctx.var("BaseLevel").get()?.number()? > 80 && ctx.var("BaseLevel").get()?.number()? < 86) {
                                ctx.call(Function::GetExperience, vec![Val::from(160000), Val::from(0)])?;
                            } else if (ctx.var("BaseLevel").get()?.number()? > 85 && ctx.var("BaseLevel").get()?.number()? < 91) {
                                ctx.call(Function::GetExperience, vec![Val::from(210000), Val::from(0)])?;
                            } else {
                                ctx.call(Function::GetExperience, vec![Val::from(350000), Val::from(0)])?;
                            }
                        }
                    }
                }
            }
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("geffen"), Val::from(120), Val::from(100)])?;
            return Err(Stop::End);
        } else {
            if ctx.var("sign_q").get()?.number()? < 95 {
                ctx.lines(args![
                    "I commend your ability",
                    "to survive in the realm",
                    "of the dead. I imagine it",
                    "must be very difficult for",
                    "a mortal to endure its cruelty."
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("sign_q").get()? == 95 {
                    ctx.lines(args![
                        "Regrettably, I can't really",
                        "provide any answers for you.",
                        "You must overcome the ordeals",
                        "of the gods on your own. All I can do is offer my guidance and prayer. ^FFFFFFcobo^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Valkyrie Sandra",
                        args![
                            "Remember that the",
                            "demonstration of your belief,",
                            "rather than the decision to",
                            "blindly adhere to faith, is",
                            "what the gods want to see."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Valkyrie Sandra",
                        args!["Always step forward", "with wisdom and courage.", "That is all I can tell you."],
                    )?;
                    ctx.var("sign_q").set(Val::from(96))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if (ctx.var("sign_q").get()? == 129 || ctx.var("sign_q").get()? == 130) {
                        ctx.lines(args!["Welcome back.", "Not only have you passed the trials that the gods have set for you, you have tested your courage in the realm of the dead and protected Midgard from attack."])?;
                        ctx.next()?;
                        ctx.lines_as("Valkyrie Sandra", args!["I'm pleased to announce that the gods have been watching you and decided to invite you to Valhalla. However, Midgard still has great need of you."])?;
                        ctx.next()?;
                        ctx.lines_as("Valkyrie Sandra", args!["As a symbol of this promise to invite you to Valhalla, you shall be rewarded with 'The Sign' which will show all others that you are a great warrior whose courage was tested by the gods themselves."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Valkyrie Sandra",
                            args![
                                "Congratulations,",
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(". Verily,")),
                                "you are an honorable",
                                "hero worthy of praise!"
                            ],
                        )?;
                        ctx.var("sign_q").set(Val::from(137))?;
                        ctx.call(Function::GetItem, vec![Val::from(7314), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("sign_q").get()?.number()? < 150 {
                        ctx.lines(args![
                            "The gods are watching",
                            "you. Prove your courage,",
                            "and at that moment, you",
                            "will earn the honor of being",
                            "selected by the gods."
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("sign_q").get()?.number()? > 199 && ctx.var("sign_q").get()?.number()? < 202) {
                        ctx.lines(args![
                            "You have yet to",
                            "fully prove your courage.",
                            "Once you do, you will be",
                            "summoned by the gods."
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("sign_q").get()? == 202 {
                        ctx.lines(args![
                            "The gods are disappointed",
                            "in what you have decided to",
                            "do. Unfortunately, you have",
                            "been banished from Valhalla",
                            "and this place as well."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Valkyrie Sandra",
                            args![
                                "Although you have failed,",
                                "I hope that you find your",
                                "another way to win back",
                                "the favor of the gods..."
                            ],
                        )?;
                        ctx.var("sign_q").set(Val::from(203))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.call(Function::CountItem, vec![Val::from(7314)])?.number()? < 1 {
                        ctx.lines(args![
                            "Only the chosen",
                            "can enter this place.",
                            "I will not ask how you've",
                            "gained entrance, but I will",
                            "require you to leave."
                        ])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("gef_fild07"), Val::from(180), Val::from(242)])?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines(args![
                            "Great warrior,",
                            "your time has not yet",
                            "come. Please focus on",
                            "your training until you",
                            "are summoned by the gods..."
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
}

pub fn valkyrie_sign(ctx: &Ctx) -> Script {
    valkyrie_sign_body(ctx, Vec::new()).map(|_| ())
}

pub fn serin_dummy(ctx: &Ctx) -> Script {
    serin_dummy_run(ctx, SerinDummyStep::Start, Vec::new()).map(|_| ())
}

pub fn serin_dummy_ondisable(ctx: &Ctx) -> Script {
    serin_dummy_run(ctx, SerinDummyStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn serin_dummy_oninit(ctx: &Ctx) -> Script {
    serin_dummy_run(ctx, SerinDummyStep::OnInit, Vec::new()).map(|_| ())
}

pub fn serin_dummy_onenable(ctx: &Ctx) -> Script {
    serin_dummy_run(ctx, SerinDummyStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn serin_dummy_onstart(ctx: &Ctx) -> Script {
    serin_dummy_run(ctx, SerinDummyStep::OnStart, Vec::new()).map(|_| ())
}

pub fn serin_dummy_ontimer3000(ctx: &Ctx) -> Script {
    serin_dummy_run(ctx, SerinDummyStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn serin_dummy_ontimer6000(ctx: &Ctx) -> Script {
    serin_dummy_run(ctx, SerinDummyStep::OnTimer6000, Vec::new()).map(|_| ())
}

pub fn serin_dummy_ontimer9000(ctx: &Ctx) -> Script {
    serin_dummy_run(ctx, SerinDummyStep::OnTimer9000, Vec::new()).map(|_| ())
}

pub fn serin_dummy_ontimer13000(ctx: &Ctx) -> Script {
    serin_dummy_run(ctx, SerinDummyStep::OnTimer13000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DarkLordSerinStep {
    Start,
    OnDisable,
    OnInit,
    OnEnable,
}

fn dark_lord_serin_run(ctx: &Ctx, mut step: DarkLordSerinStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DarkLordSerinStep::Start => {
                return Err(Stop::End);
            }
            DarkLordSerinStep::OnDisable => {
                step = DarkLordSerinStep::OnInit;
                continue 'machine;
            }
            DarkLordSerinStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Dark Lord#serin")])?;
                return Err(Stop::End);
            }
            DarkLordSerinStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Dark Lord#serin")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dark_lord_serin(ctx: &Ctx) -> Script {
    dark_lord_serin_run(ctx, DarkLordSerinStep::Start, Vec::new()).map(|_| ())
}

pub fn dark_lord_serin_ondisable(ctx: &Ctx) -> Script {
    dark_lord_serin_run(ctx, DarkLordSerinStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn dark_lord_serin_oninit(ctx: &Ctx) -> Script {
    dark_lord_serin_run(ctx, DarkLordSerinStep::OnInit, Vec::new()).map(|_| ())
}

pub fn dark_lord_serin_onenable(ctx: &Ctx) -> Script {
    dark_lord_serin_run(ctx, DarkLordSerinStep::OnEnable, Vec::new()).map(|_| ())
}

fn serin_serin_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_fail_s1 = Val::from(0);
    let mut l_fail_s2 = Val::from(0);
    let mut l_fail_s3 = Val::from(0);
    let mut l_fail_s4 = Val::from(0);
    let mut l_fail_s5 = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    if ctx.var("sign_q").get()?.number()? < 132 {
        ctx.lines_as("Serin", args!["....?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("sign_q").get()? == 132 {
            ctx.lines_as("Serin", args!["As its magic power grows,", "the Magic Circle is nearing completion. The symbol which I've asked you for is the last material needed for summoning Dark Lord."])?;
            ctx.next()?;
            ctx.lines_as(
                "Serin",
                args![
                    "However, the power of that",
                    "symbol can be used to complete",
                    "the Magic Circle or destroy it.",
                    "Would you hand it over to me?",
                    "I choose to destroy this..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Serin", args!["Please...", "Let me have the symbol."])?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Give Serin the symbol.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as("Serin", args!["This could be", "pretty dangerous", "so please step back..."])?;
            ctx.call(Function::DelItem, vec![Val::from(7305), Val::from(1)])?;
            ctx.var("sign_q").set(Val::from(133))?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#dummy::OnEnable")])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#dummy::OnStart")])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#serin::OnDisable")])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("sign_q").get()? == 133 {
                ctx.lines_as("Serin", args!["I'm sorry.", "But I have to do this", "to summon Dark Lord."])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("But why...?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "It was never the witch",
                        "that wanted Dark Lord summoned,",
                        "but me. I've always wanted to return to my human form. I'm sick and tired of being bound here in Niflheim!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "Unfortunately, the symbol",
                        "which Lady Hell gave you is",
                        "limited to a one time use. Its",
                        "power wasn't enough to bring me",
                        "back, so summoning Dark Lord is",
                        "my last chance."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("But if Dark Lord comes to Midgard...")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "Oh, I'm pretty sure of",
                        "what will happen if Dark Lord",
                        "enters the realm of the living. He'll destroy Midgard and bring death to thousands and thousands of people."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "So if I came back to life",
                        "and everyone else were dead,",
                        "However, by summoning Dark Lord,",
                        "you might be thinking it'd be pretty pointless for me to come back. But you know what? I don't care."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "The living don't appreciate",
                        "what they have, so they ought",
                        "to be punished. They can all go to Niflheim while I enjoy the warmth of the sun and the fresh outside air."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "Maybe you might not",
                        "pity my situation since you've",
                        "never been bound to Niflheim,",
                        "but I'm begging you. Don't get",
                        "in my way."
                    ],
                )?;
                ctx.next()?;
                'b1: {
                    let subject1 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Alright. I'll let you go.:No, you have to be stopped.")],
                    )?);
                    let mut matched1 = false;
                    let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                    if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Serin",
                            args![
                                "Are you really willing",
                                "to throw so much away for",
                                "my sake? You do understand",
                                "that you'll be failing the ordeals set before you by the gods..."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("I changed my mind for Midgard.:I do, and it's alright...")])? {
                            1 => {
                                ctx.lines_as(
                                    "Serin",
                                    args![
                                        "...",
                                        "I really don't want to fight you,",
                                        "but I've come too far to give up now. If you insist on interfering, then you leave me no choice..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("Dark Lord#serin::OnEnable")])?;
                                ctx.lines_as(
                                    "Dark Lord",
                                    args![
                                        "^330033Insolent mortal!",
                                        "Do you really think",
                                        "you have a chance of",
                                        "stopping me? Hmpf.",
                                        "Every fool must learn.",
                                        "Prepare to die!^000000"
                                    ],
                                )?;
                                ctx.var("sign_q").set(Val::from(134))?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#serin::OnDisable")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("Dark Lord#serin::OnDisable")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("CallMonster#serin::Oncall")])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Serin",
                                    args![
                                        "The least I can",
                                        "do is send you back",
                                        "to Midgard and",
                                        "ask the Dark Lord to spare",
                                        "you. Thank you so much..."
                                    ],
                                )?;
                                ctx.var("sign_q").set(Val::from(200))?;
                                ctx.call(Function::GetExperience, vec![Val::from(200000), Val::from(0)])?;
                                ctx.var("$@sign_w2").set(Val::from(0))?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("Dark Lord#serin::OnDisable")])?;
                                ctx.close_window()?;
                                ctx.call(Function::Warp, vec![Val::from("umbala"), Val::from(132), Val::from(203)])?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Serin",
                            args![
                                "...",
                                "I really don't want to fight you,",
                                "but I've come too far to give up now. If you insist on interfering, then you leave me no choice..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(
                            Function::Emotion,
                            vec![
                                ctx.constant("ET_KIK")?,
                                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Dark Lord#serin")])?,
                            ],
                        )?;
                        ctx.lines_as(
                            "Dark Lord",
                            args![
                                "^330033Insolent mortal!",
                                "Do you really think",
                                "you have a chance of",
                                "stopping me? Hmpf.",
                                "Every fool must learn.",
                                "Prepare to die!^000000"
                            ],
                        )?;
                        ctx.var("sign_q").set(Val::from(134))?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#serin::OnDisable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Dark Lord#serin::OnDisable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("CallMonster#serin::Oncall")])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            } else if ctx.var("sign_q").get()? == 134 {
                ctx.lines_as(
                    "Serin",
                    args![
                        "I see now.",
                        "Soon, I'll lose my memories",
                        "and remain dead in Niflheim.",
                        "Just like all the others. Still,",
                        "may I ask you one question?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "With that symbol, you",
                        "could order the dead to do",
                        "whatever you want. Why didn't",
                        "you use it to command me to quit?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Serin", args!["...", "......", ".........."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "I see it now.",
                        "The kindness in your eyes",
                        "tells me everything. You wanted to give me another chance. But in the end, I managed to destroy the",
                        "chance you had given me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "Even though I'm nothing but",
                        "a spirit now, it was an honor",
                        "for me to meet somebody like you. Although I'll lose my memories, I'll try not to forget you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "My memories of your",
                        "courage and kindness",
                        "are more precious to",
                        "me than life itself."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args!["Farewell, now.", "And good luck on", "your travels, my friend..."],
                )?;
                ctx.var("sign_q").set(Val::from(135))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("sign_q").get()? == 135 {
                ctx.lines(args![
                    "^3355FFYou helped the",
                    "unconscious Serin",
                    "and returned to Niflheim.^000000"
                ])?;
                ctx.close_window()?;
                ctx.var("sign_q").set(Val::from(136))?;
                ctx.var("$@sign_w2").set(Val::from(0))?;
                ctx.call(Function::Warp, vec![Val::from("que_sign01"), Val::from(117), Val::from(137)])?;
                return Err(Stop::End);
            } else if ctx.var("sign_q").get()? == 199 {
                l_fail_s1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(800)])?;
                l_fail_s2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(700)])?;
                l_fail_s3 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(600)])?;
                l_fail_s4 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(500)])?;
                l_fail_s5 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(400)])?;
                if ctx.call(Function::CountItem, vec![Val::from(2643)])? == 1 {
                    if (((ctx.var("sign_sq").get()? == 0 && l_fail_s3.clone() == 356)
                        || (ctx.var("sign_sq").get()? == 1 && l_fail_s2.clone() == 356))
                        || l_fail_s1.clone() == 356)
                    {
                        ctx.var("sign_q").set(Val::from(200))?;
                        ctx.lines_as(
                            "Serin",
                            args![
                                "Even though you tried",
                                "to stop me, I still don't wish",
                                "to take your life. I understand",
                                "that you've merely trying to",
                                "protect your Midgard."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Serin",
                            args![
                                "However...",
                                "I do not think you",
                                "have the courage to",
                                "overcome the ordeals ",
                                "of the gods. Please don't",
                                "come here anymore..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Serin", args!["Farewell..."])?;
                        ctx.close_window()?;
                        ctx.var("$@sign_w2").set(Val::from(0))?;
                        ctx.call(Function::Warp, vec![Val::from("niflheim"), Val::from(30), Val::from(156)])?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Serin",
                            args!["Haven't you", "given up yet?", "Please don't", "try to stop me!"],
                        )?;
                        ctx.close_window()?;
                        ctx.var("sign_q").set(Val::from(134))?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#serin::OnDisable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Dark Lord#serin::OnDisable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("CallMonster#serin::Oncall")])?;
                        return Err(Stop::End);
                    }
                } else {
                    if (((ctx.var("sign_sq").get()? == 0 && l_fail_s5.clone() == 356)
                        || (ctx.var("sign_sq").get()? == 1 && l_fail_s4.clone() == 356))
                        || l_fail_s3.clone() == 356)
                    {
                        ctx.var("sign_q").set(Val::from(200))?;
                        ctx.lines_as(
                            "Serin",
                            args![
                                "Even though you tried",
                                "to stop me, I still don't wish",
                                "to take your life. I understand",
                                "that you've merely trying to",
                                "protect your Midgard."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Serin",
                            args![
                                "However...",
                                "I do not think you",
                                "have the courage to",
                                "overcome the ordeals ",
                                "of the gods. Please don't",
                                "come here anymore..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Serin", args!["Farewell..."])?;
                        ctx.close_window()?;
                        ctx.var("$@sign_w2").set(Val::from(0))?;
                        ctx.call(Function::Warp, vec![Val::from("niflheim"), Val::from(30), Val::from(156)])?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Serin",
                            args!["Haven't you", "given up yet?", "Please don't", "try to stop me!"],
                        )?;
                        ctx.close_window()?;
                        ctx.var("sign_q").set(Val::from(134))?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#serin::OnDisable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Dark Lord#serin::OnDisable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("CallMonster#serin::Oncall")])?;
                        return Err(Stop::End);
                    }
                }
            } else if ctx.var("sign_q").get()? == 200 {
                ctx.lines_as("Serin", args!["Let me guide", "you to where", "you belong."])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("umbala"), Val::from(132), Val::from(203)])?;
                return Err(Stop::End);
            } else {
                ctx.mes("..........")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    return Err(Stop::End);
}

pub fn serin_serin(ctx: &Ctx) -> Script {
    serin_serin_body(ctx, Vec::new()).map(|_| ())
}

fn serin_serin_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Serin#serin")])?;
    return Err(Stop::End);
}

pub fn serin_serin_ondisable(ctx: &Ctx) -> Script {
    serin_serin_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn serin_serin_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Serin#serin")])?;
    return Err(Stop::End);
}

pub fn serin_serin_onenable(ctx: &Ctx) -> Script {
    serin_serin_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn serin_witch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_fail_s1 = Val::from(0);
    let mut l_fail_s2 = Val::from(0);
    let mut l_fail_s3 = Val::from(0);
    let mut l_fail_s4 = Val::from(0);
    let mut l_fail_s5 = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    if ctx.var("sign_q").get()?.number()? < 124 {
        ctx.lines_as("Serin", args!["...", "......"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("sign_q").get()? == 124 {
            ctx.lines_as(
                "Serin",
                args![
                    "You're finally here.",
                    "That witch has been watching",
                    "my every move, so she probably",
                    "knows what I'm up to by now..."
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Why are you doing this...")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Serin",
                args![
                    "You don't understand",
                    "the horrific experience",
                    "of being bound to Niflheim.",
                    "I'm sick of breathing death",
                    "and feeding on despair.",
                    "I want to live again!"
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Is that why you want Dark Lord...?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Serin",
                args![
                    "Unfortunately, the symbol",
                    "which Lady Hell gave you is",
                    "limited to a one time use. Its",
                    "power wasn't enough to bring me",
                    "back, so summoning Dark Lord is",
                    "my last chance."
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("But if Dark Lord comes to Midgard...")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Serin",
                args![
                    "Oh, I'm pretty sure of",
                    "what will happen if Dark Lord",
                    "enters the realm of the living. He'll destroy Midgard and bring death to thousands and thousands of people."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Serin",
                args![
                    "So if I came back to life",
                    "and everyone else were dead,",
                    "However, by summoning Dark",
                    "Lord, you might be thinking it'd be pretty pointless for me to come back. But I don't care."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Serin", args!["The living don't appreciate", "what they have, so they ought", "to be punished. They can all go to Niflheim while I enjoy the warmth of the sun and the fresh outside air. Everyone should die..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Serin",
                args![
                    "Maybe you might not",
                    "pity my situation since you've",
                    "never been bound to Niflheim,",
                    "but I'm begging you. Don't get",
                    ((Val::from("in my way, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("I can't let you do this!:Okay, have it your way.")])? {
                1 => {
                    ctx.var("sign_q").set(Val::from(125))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Serin",
                        args![
                            "Are you really willing",
                            "to throw so much away for",
                            "my sake? You do understand",
                            "that you'll be failing the ordeals set before you by the gods..."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("I changed my mind for Midgard.:I do, and it's alright...")])? {
                        1 => {
                            ctx.var("sign_q").set(Val::from(125))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Serin",
                                args![
                                    "The least I can",
                                    "do is send you back",
                                    "to Midgard and",
                                    "ask the Dark Lord to spare",
                                    "you. Thank you so much..."
                                ],
                            )?;
                            ctx.var("sign_q").set(Val::from(200))?;
                            ctx.call(Function::GetExperience, vec![Val::from(200000), Val::from(0)])?;
                            ctx.var("$@sign_w1").set(Val::from(0))?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("umbala"), Val::from(132), Val::from(203)])?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        } else if ctx.var("sign_q").get()? == 125 {
            if ctx.call(Function::CountItem, vec![Val::from(2643)])? == 1 {
                ctx.lines_as(
                    "Serin",
                    args![
                        "If you think that",
                        "I can't summon Dark Lord",
                        "without the symbol, then you're",
                        "mistaken. I've been collecting",
                        "the power of despair!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "Do you remember what",
                        "happened to the dead that you",
                        "helped on my behalf? Even if they seemed to have found shelter at first, they cannot escape my",
                        "deep hollow of despair..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "And that poor little girl,",
                        "Alakina Anne? She still",
                        "hasn't realized that she's dead!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "You gave her hope,",
                        "but ultimately, you can't",
                        "keep your promise to bring her home. Oh, her disappointment",
                        "must be so crushing..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "Yes...",
                        "By trying to help,",
                        "you gave them nothing",
                        "but pain, just as I planned.",
                        "Now I have enough despair",
                        "to summon Dark Lord!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "I don't want us to fight,",
                        "but I can't let you stop me.",
                        "I've already sold my soul to",
                        "Dark Lord to become a living",
                        "human again so I can't give up",
                        "now! We'll have to battle!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.var("sign_q").set(Val::from(126))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("CallMonster#witch::Oncall")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#witch::OnDisable")])?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Serin",
                    args![
                        "If you think that",
                        "I can't summon Dark Lord",
                        "without the symbol, then you're",
                        "mistaken. I've been collecting",
                        "the power of despair!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "What most adventurers",
                        "don't know is that many of the",
                        "souls in Niflheim are simply",
                        "doomed to suffer. That's the",
                        "nature of this realm and one",
                        "of the rules of balance..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "It was child's play to ask",
                        "some adventurers to help",
                        "these souls. But once they learn that these souls can't be helped,",
                        "it's too late! In failing them, the adventurers add to their despair!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "And by increasing",
                        "the despair here, it",
                        "also adds to the power I'll",
                        "use to summon Dark Lord!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "I don't want us to fight,",
                        "but I can't let you stop me.",
                        "I've already sold my soul to",
                        "Dark Lord to become a living",
                        "human again so I can't give up",
                        "now! We'll have to battle!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.var("sign_q").set(Val::from(126))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("CallMonster#witch::Oncall")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#witch::OnDisable")])?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("sign_q").get()? == 126 {
                ctx.lines_as(
                    "Serin",
                    args![
                        "I see now.",
                        "Soon, I'll lose my memories",
                        "and remain dead in Niflheim.",
                        "Just like all the others. Still,",
                        "may I ask you one question?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "With that symbol, you",
                        "could order the dead to do",
                        "whatever you want. Why didn't",
                        "you use it to command me to quit?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Serin", args!["...", "......", ".........."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "I see it now.",
                        "The kindness in your eyes",
                        "tells me everything. You wanted to give me another chance. But in the end, I managed to destroy the",
                        "chance you had given me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "Even though I'm nothing but",
                        "a spirit now, it was an honor",
                        "for me to meet somebody like you. Although I'll lose my memories, I'll try not to forget you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args![
                        "My memories of your",
                        "courage and kindness",
                        "are more precious to",
                        "me than life itself."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Serin",
                    args!["Farewell, now.", "And good luck on", "your travels, my friend..."],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7308), Val::from(1)])?;
                ctx.var("sign_q").set(Val::from(127))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("sign_q").get()? == 127 {
                ctx.lines(args![
                    "^3355FFYou helped the",
                    "unconscious Serin",
                    "and returned to Niflheim.^000000"
                ])?;
                ctx.close_window()?;
                ctx.var("sign_q").set(Val::from(128))?;
                ctx.var("$@sign_w1").set(Val::from(0))?;
                ctx.call(Function::Warp, vec![Val::from("que_sign01"), Val::from(117), Val::from(137)])?;
                return Err(Stop::End);
            } else if ctx.var("sign_q").get()? == 198 {
                l_fail_s1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(1000)])?;
                l_fail_s2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(900)])?;
                l_fail_s3 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(800)])?;
                l_fail_s4 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(700)])?;
                l_fail_s5 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(600)])?;
                if ctx.call(Function::CountItem, vec![Val::from(2643)])? == 1 {
                    if (((ctx.var("sign_sq").get()? == 0 && l_fail_s3.clone() == 356)
                        || (ctx.var("sign_sq").get()? == 1 && l_fail_s2.clone() == 356))
                        || l_fail_s1.clone() == 356)
                    {
                        ctx.call(Function::DelItem, vec![Val::from(7308), Val::from(1)])?;
                        ctx.var("sign_q").set(Val::from(200))?;
                        ctx.lines_as(
                            "Serin",
                            args![
                                "Even though you tried",
                                "to stop me, I still don't wish",
                                "to take your life. I understand",
                                "that you've merely trying to",
                                "protect your Midgard."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Serin",
                            args![
                                "However...",
                                "I do not think you",
                                "have the courage to",
                                "overcome the ordeals ",
                                "of the gods. Please don't",
                                "come here anymore..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args!["^3355FFSerin took", "1 Witch's Tonic", "from you.^000000"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Serin",
                            args![
                                "I'm sorry...",
                                "But I'm taking",
                                "your Witch's Tonic",
                                "to help me summon",
                                "Dark Lord. Farewell..."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.var("$@sign_w1").set(Val::from(0))?;
                        ctx.call(Function::Warp, vec![Val::from("niflheim"), Val::from(30), Val::from(156)])?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Serin",
                            args!["Haven't you", "given up yet?", "Please don't", "try to stop me!"],
                        )?;
                        ctx.close_window()?;
                        ctx.var("sign_q").set(Val::from(126))?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#witch::OnDisable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("CallMonster#witch::Oncall")])?;
                        return Err(Stop::End);
                    }
                } else {
                    if (((ctx.var("sign_sq").get()? == 0 && l_fail_s5.clone() == 356)
                        || (ctx.var("sign_sq").get()? == 1 && l_fail_s4.clone() == 356))
                        || l_fail_s3.clone() == 356)
                    {
                        ctx.call(Function::DelItem, vec![Val::from(7308), Val::from(1)])?;
                        ctx.var("sign_q").set(Val::from(200))?;
                        ctx.lines_as(
                            "Serin",
                            args![
                                "Even though you tried",
                                "to stop me, I still don't wish",
                                "to take your life. I understand",
                                "that you've merely trying to",
                                "protect your Midgard."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Serin",
                            args![
                                "However...",
                                "I do not think you",
                                "have the courage to",
                                "overcome the ordeals ",
                                "of the gods. Please don't",
                                "come here anymore..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args!["^3355FFSerin took", "1 Witch's Tonic", "from you.^000000"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Serin",
                            args![
                                "I'm sorry...",
                                "But I'm taking",
                                "your Witch's Tonic",
                                "to help me summon",
                                "Dark Lord. Farewell..."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.var("$@sign_w1").set(Val::from(0))?;
                        ctx.call(Function::Warp, vec![Val::from("niflheim"), Val::from(30), Val::from(156)])?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Serin",
                            args!["Haven't you", "given up yet?", "Please don't", "try to stop me!"],
                        )?;
                        ctx.close_window()?;
                        ctx.var("sign_q").set(Val::from(126))?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#witch::OnDisable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("CallMonster#witch::Oncall")])?;
                        return Err(Stop::End);
                    }
                }
            } else if ctx.var("sign_q").get()? == 200 {
                ctx.lines_as("Serin", args!["Let me guide", "you to where", "you belong.", "......."])?;
                ctx.close_window()?;
                ctx.var("$@sign_w1").set(Val::from(0))?;
                ctx.call(Function::Warp, vec![Val::from("umbala"), Val::from(132), Val::from(203)])?;
                return Err(Stop::End);
            } else {
                ctx.lines_as("Serin", args!["........."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    return Err(Stop::End);
}

pub fn serin_witch(ctx: &Ctx) -> Script {
    serin_witch_body(ctx, Vec::new()).map(|_| ())
}

fn serin_witch_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Serin#witch")])?;
    return Err(Stop::End);
}

pub fn serin_witch_ondisable(ctx: &Ctx) -> Script {
    serin_witch_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn serin_witch_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Serin#witch")])?;
    return Err(Stop::End);
}

pub fn serin_witch_onenable(ctx: &Ctx) -> Script {
    serin_witch_onenable_body(ctx, Vec::new()).map(|_| ())
}
