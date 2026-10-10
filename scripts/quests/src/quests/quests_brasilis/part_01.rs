use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn angelo_br_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("BaseLevel").get()?.number()? < 40 {
        ctx.lines_as("Angelo", args!["Pets went out the village~!!", "Gosh... what can I do... ?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CheckQuest, vec![Val::from(9032), ctx.constant("PLAYTIME")?])? == 2 {
        ctx.call(Function::EraseQuest, vec![Val::from(9032)])?;
    }
    if ctx.call(Function::CheckQuest, vec![Val::from(9032), ctx.constant("PLAYTIME")?])? == 0 {
        ctx.lines_as(
            "Angelo",
            args!["The day is not finished yet.", "You can only help once a day. Hehe."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CheckQuest, vec![Val::from(9030)])? == 1 {
        ctx.lines_as(
            "Angelo",
            args![
                "My pets are in the field outside of the village.",
                "Why did they leave? Please find them."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CheckQuest, vec![Val::from(9031)])? == 1 {
        ctx.lines_as(
            "Angelo",
            args![
                "Oh, thank you. You found all of 3 puppies.",
                "Thanks a lot.",
                "I hope this is useful to you. hoho."
            ],
        )?;
        if (ctx.constant("VIP_SCRIPT")?.is_true() && ctx.call(Function::VipStatus, vec![ctx.constant("VIP_STATUS_ACTIVE")?])?.is_true()) {
            ctx.call(Function::GetExperience, vec![Val::from(75000), Val::from(0)])?;
        } else {
            ctx.call(Function::GetExperience, vec![Val::from(50000), Val::from(0)])?;
        }
        ctx.call(Function::EraseQuest, vec![Val::from(9031)])?;
        ctx.call(Function::SetQuest, vec![Val::from(9032)])?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_ASSUMPTIO")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Angelo",
        args![
            "Are you an adventurer? You came here right on time.",
            "Puppies have been disappearing.",
            "And someone said that they saw them out on the field just outside the village...."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Angelo",
        args![
            "It's pretty difficult and dangerous to find 'em.",
            "You have to find ^0000FF3 puppies^000000."
        ],
    )?;
    ctx.call(Function::SetQuest, vec![Val::from(9030)])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn angelo_br(ctx: &Ctx) -> Script {
    angelo_br_body(ctx, Vec::new()).map(|_| ())
}

fn angelo_br_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn angelo_br_oninit(ctx: &Ctx) -> Script {
    angelo_br_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn angelo_br_ontimer10000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Angelo#br::OnGo")])?;
    return Err(Stop::End);
}

pub fn angelo_br_ontimer10000(ctx: &Ctx) -> Script {
    angelo_br_ontimer10000_body(ctx, Vec::new()).map(|_| ())
}

fn angelo_br_ongo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn angelo_br_ongo(ctx: &Ctx) -> Script {
    angelo_br_ongo_body(ctx, Vec::new()).map(|_| ())
}

fn puppy_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i: Vec<Val> = Vec::new();
    ctx.lines_as("Puppy", args!["bow! wow wow!"])?;
    if ctx.call(Function::CheckQuest, vec![Val::from(9030)])? == 1 && ctx.var("brazil_kid").get()?.number()? < 3 {
        ctx.next()?;
        ctx.var("brazil_kid").set((ctx.var("brazil_kid").get()? + Val::from(1)))?;
        ctx.lines(args![
            ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
        ])?;
        if ctx.var("brazil_kid").get()? == 3 {
            ctx.lines(args!["Good. I found all 3 puppies.", "Now I need to go tell Angelo."])?;
            ctx.var("brazil_kid").set(Val::from(0))?;
            ctx.call(Function::EraseQuest, vec![Val::from(9030)])?;
            ctx.call(Function::SetQuest, vec![Val::from(9031)])?;
        } else {
            ctx.lines(args!["Ah... who's a good puppy?", "Ok, where are the others?"])?;
        }
        if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?, &Val::from("1")).is_true() {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_i, &Val::from(base + 0), Val::from(2), false);
            runtime::local_set(&mut l_i, &Val::from(base + 1), Val::from(3), false);
        } else if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?, &Val::from("2")).is_true() {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_i, &Val::from(base + 0), Val::from(1), false);
            runtime::local_set(&mut l_i, &Val::from(base + 1), Val::from(3), false);
        } else {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_i, &Val::from(base + 0), Val::from(1), false);
            runtime::local_set(&mut l_i, &Val::from(base + 1), Val::from(2), false);
        }
        ctx.call(
            Function::DoNpcEvent,
            vec![
                (((Val::from("Puppy#") + runtime::charat(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?, &Val::from(0))?)
                    + runtime::local_get(&l_i, &ctx.call(Function::Rand, vec![Val::from(2)])?, false))
                    + Val::from("::OnEnable")),
            ],
        )?;
        ctx.call(Function::DisableNpc, vec![ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn puppy_bra(ctx: &Ctx) -> Script {
    puppy_bra_body(ctx, Vec::new()).map(|_| ())
}

fn puppy_bra_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !(runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?, &Val::from("1")).is_true()) {
        ctx.call(Function::DisableNpc, vec![ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?])?;
    }
    return Err(Stop::End);
}

pub fn puppy_bra_oninit(ctx: &Ctx) -> Script {
    puppy_bra_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn puppy_bra_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?])?;
    return Err(Stop::End);
}

pub fn puppy_bra_onenable(ctx: &Ctx) -> Script {
    puppy_bra_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn puppy_bra_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?])?;
    return Err(Stop::End);
}

pub fn puppy_bra_ondisable(ctx: &Ctx) -> Script {
    puppy_bra_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn lucia_brasilis_run(ctx: &Ctx, mut step: LuciaBrasilisStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_nchecktime = Val::from(0);
    let mut l_nqstate1 = Val::from(0);
    let mut l_nqstate2 = Val::from(0);
    'machine: loop {
        match step {
            LuciaBrasilisStep::Start => {
                if ctx.var("BaseLevel").get()?.number()? < 40 {
                    ctx.lines_as(
                        "Lucia",
                        args![
                            "Hello.",
                            "I'm worried about ^FF0000Strange Hydra^000000's on",
                            "the south beach.",
                            "I hope some experienced adventurers",
                            "will come to help."
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_OHNO")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                l_nqstate1 = ctx.call(Function::CheckQuest, vec![Val::from(9028)])?;
                l_nqstate2 = ctx.call(Function::CheckQuest, vec![Val::from(9029)])?;
                if l_nqstate1.clone() == -1 {
                    ctx.lines_as(
                        "Lucia",
                        args!["Hello.", "Have you come here to hunt ^FF0000Strange Hydra^000000s?"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Yes.:No.:^006400What is happening here?^000000")])? {
                        1 => {
                            ctx.call(Function::SetQuest, vec![Val::from(9028)])?;
                            ctx.call(Function::GetItem, vec![Val::from(12408), Val::from(1)])?;
                            ctx.lines_as(
                                "Lucia",
                                args![
                                    "Here, take this ^006400Hydra Ball^000000.",
                                    "Use it to capture a ^FF0000Strange Hydra^8B4513.^000000",
                                    "I hope you can do it~!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as("Lucia", args!["Ah, I misunderstood.", "See you then."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            step = LuciaBrasilisStep::LWhatHappen;
                            continue 'machine;
                        }
                        _ => {}
                    }
                    step = LuciaBrasilisStep::HoistEnd1;
                    continue 'machine;
                    step = LuciaBrasilisStep::LWhatHappen;
                    continue 'machine;
                } else if (l_nqstate1.clone() == 0 || l_nqstate1.clone() == 1) {
                    if ctx.call(Function::CountItem, vec![Val::from(6221)])?.number()? > 0 {
                        ctx.lines_as("Lucia", args!["Hello, you really did it!"])?;
                        if ctx.call(Function::CheckWeight, vec![Val::from(11502), Val::from(3)])?.is_true() {
                            ctx.lines(args!["I hope you will come", "again to help me.", "Have a nice day~!"])?;
                            ctx.call(Function::DelItem, vec![Val::from(6221), Val::from(1)])?;
                            ctx.call(Function::CompleteQuest, vec![Val::from(9028)])?;
                            if l_nqstate2.clone().number()? > -1 {
                                ctx.call(Function::EraseQuest, vec![Val::from(9029)])?;
                            }
                            ctx.call(Function::SetQuest, vec![Val::from(9029)])?;
                            ctx.call(Function::ConsumeItem, vec![Val::from(12070)])?;
                            ctx.call(Function::ConsumeItem, vec![Val::from(12055)])?;
                            ctx.call(Function::ConsumeItem, vec![Val::from(12065)])?;
                            ctx.call(Function::GetItem, vec![Val::from(505), Val::from(5)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines(args![
                                " ",
                                "I'd like to reward you,",
                                "however your bags are full.",
                                "Please make room and come back!"
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        ctx.mes("[Lucia]")?;
                        if ctx.call(Function::CountItem, vec![Val::from(12408)])?.number()? < 1 {
                            ctx.lines(args![
                                "Did you need another ^006400Hydra Ball^000000?",
                                "I will give you one more."
                            ])?;
                            ctx.call(Function::GetItem, vec![Val::from(12408), Val::from(1)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.mes("Any problems?")?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("No.:^006400Tell me again what happened^000000")])? {
                                1 => {
                                    ctx.lines_as("Lucia", args!["Ok, please do me a favor."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    step = LuciaBrasilisStep::LWhatHappen;
                                    continue 'machine;
                                }
                                _ => {}
                            }
                        }
                    }
                } else {
                    ctx.lines_as(
                        "Lucia",
                        args![
                            ((Val::from("Oh, ^0000FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from("^000000 you're back."))
                        ],
                    )?;
                    l_nchecktime = ctx.call(Function::CheckQuest, vec![Val::from(9029), ctx.constant("PLAYTIME")?])?;
                    if (l_nchecktime.clone() == 0 || l_nchecktime.clone() == 1) {
                        ctx.lines(args![
                            "I'm so grateful for your help.",
                            "Each ^006400Hydra Ball^000000 is provided ^006400every 24 hours^000000",
                            "Please come at the appropriate time."
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if l_nqstate1.clone().number()? > -1 {
                            ctx.call(Function::EraseQuest, vec![Val::from(9028)])?;
                        }
                        ctx.call(Function::CompleteQuest, vec![Val::from(9029)])?;
                        ctx.mes("Did you come here to hunt ^FF0000Strange Hydra^000000s?")?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Yes.:No.:^006400What is happening here?^000000")])? {
                            1 => {
                                ctx.call(Function::SetQuest, vec![Val::from(9028)])?;
                                ctx.call(Function::GetItem, vec![Val::from(12408), Val::from(1)])?;
                                ctx.lines_as(
                                    "Lucia",
                                    args![
                                        "Here, take this ^006400Hydra Ball^000000.",
                                        "Use it to capture a ^FF0000Strange Hydra^8B4513.^000000",
                                        "I hope you can do it~!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as("Lucia", args!["Ah, I misunderstood.", "See you then."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            3 => {
                                step = LuciaBrasilisStep::LWhatHappen;
                                continue 'machine;
                            }
                            _ => {}
                        }
                    }
                }
                step = LuciaBrasilisStep::HoistEnd2;
                continue 'machine;
            }
            LuciaBrasilisStep::LWhatHappen => {
                ctx.lines_as(
                    "Lucia",
                    args![
                        "One day ^FF0000Strange Hydra^000000s",
                        "came here and surrounded the town.",
                        "We're not sure what attracted them but some say that it's because of you adventurers."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lucia",
                    args![
                        "In any case, to contain the ^FF0000Strange Hydra^000000s,",
                        "you have to use this specially designed tool a.k.a. a ^8B4513Hydra Ball^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lucia",
                    args![
                        "If you still have the ^006400Hydra Ball^000000,",
                        "please use it on the ^FF0000Strange Hydra^000000s that",
                        "you can find at the beach.",
                        "If you are lucky, the tool will work perfectly."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lucia",
                    args!["I hope many adventurers", "volunteer for this job.", " ", "I really hate Hydra!"],
                )?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            LuciaBrasilisStep::HoistEnd1 => {
                step = LuciaBrasilisStep::HoistEnd2;
                continue 'machine;
            }
            LuciaBrasilisStep::HoistEnd2 => {
                return Err(Stop::End);
            }
            LuciaBrasilisStep::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            LuciaBrasilisStep::OnTimer7000 => {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn lucia_brasilis(ctx: &Ctx) -> Script {
    lucia_brasilis_run(ctx, LuciaBrasilisStep::Start, Vec::new()).map(|_| ())
}

pub fn lucia_brasilis_oninit(ctx: &Ctx) -> Script {
    lucia_brasilis_run(ctx, LuciaBrasilisStep::OnInit, Vec::new()).map(|_| ())
}

pub fn lucia_brasilis_ontimer7000(ctx: &Ctx) -> Script {
    lucia_brasilis_run(ctx, LuciaBrasilisStep::OnTimer7000, Vec::new()).map(|_| ())
}

fn candy_maker_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !(ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])?.is_true()) {
        ctx.mes("- You can't start the quest. Please reduce the weight in your inventory. -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("brazil_gua").get()? == 0 {
        ctx.lines_as("Candy Maker", args!["Yo, do you know a berry called ^FF0000Guarana^000000?"])?;
        ctx.next()?;
        ctx.lines_as("Candy Maker", args!["Guarana is a really special berry raised in a specific area, it relieves physical fatigue, and gives power to the body. It even detoxes waste from the body."])?;
        ctx.next()?;
        ctx.lines_as(
            "Candy Maker",
            args![
                "I used to sell the candy made of it back in the day.",
                "I got a prize sometimes every year in the <annual best product contest>. Those were the good 'ol days."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Candy Maker", args!["Since then, the output of that fruit has reduced and the price has gone up so now candy ingredients were changed to coconuts or other tropical fruits instead. I miss the guarana candy."])?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("How can I taste this guarana candy?:End conversation.")],
            )?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as("Candy Maker", args!["Hmm? I already sold out of all my old supply."])?;
                ctx.next()?;
                ctx.lines_as("Candy Maker", args!["But if you can find some guarana, I can make it for you."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("How do I find guarana?:End conversation.")])? {
                    1 => {
                        ctx.lines_as("Candy Maker", args!["Will you find the guarana?? Hoooooh~"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Candy Maker",
                            args![
                                "Can you find it?",
                                "It's probably very expensive.",
                                "Trading isn't my thing. Let me think."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Candy Maker",
                            args![
                                "Let me introduce you to someone with whom I used to do guarana business with.",
                                "He might still be dealing it."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Candy Maker",
                            args![
                                "His name is Cherto.",
                                "If you can't find him in the city, go to museum.",
                                "He's a vain person so he likes to act big.",
                                "He's probably wandering in the museum trying to show off to someone for sure."
                            ],
                        )?;
                        ctx.var("brazil_gua").set(Val::from(1))?;
                        ctx.call(Function::SetQuest, vec![Val::from(2192)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Candy Maker", args!["Don't you want to try the guarana candy?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as("Candy Maker", args!["Those were the good 'ole days..."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if ctx.var("brazil_gua").get()? == 1 {
        ctx.lines_as(
            "Candy Maker",
            args!["If you want to get the guarana, find Cherto.", "Maybe he will be in the museum."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("brazil_gua").get()? == 10 {
        if !(ctx.call(Function::CountItem, vec![Val::from(6237)])?.is_true()) {
            ctx.mes("- The guarana that I had has disappeared. -")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.call(Function::DelItem, vec![Val::from(6237), Val::from(1)])?;
        ctx.lines_as("Candy Maker", args!["Did you get the guarana?"])?;
        ctx.next()?;
        ctx.mes("- You give the guarana to him. -")?;
        ctx.next()?;
        ctx.lines_as(
            "Candy Maker",
            args![
                "Wow! You have special talent.",
                "It's the best thing I have ever seen so far. Cool~!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Candy Maker",
            args!["Good, let's make the candy~!", "Long time no see my wonderful guarana candy..."],
        )?;
        ctx.next()?;
        ctx.lines(args!["- hash hash hash hash hash hash -", "- hash hash hash hash hash hash -"])?;
        ctx.next()?;
        ctx.lines_as(
            "Candy Maker",
            args![
                "Look! It's the popular guarana candy.",
                "Try to savor its amazing taste hey~ take it easy. hahaha!!"
            ],
        )?;
        ctx.var("brazil_gua").set(Val::from(11))?;
        ctx.call(Function::CompleteQuest, vec![Val::from(2200)])?;
        ctx.call(Function::GetItem, vec![Val::from(12414), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("brazil_gua").get()? == 11 {
        ctx.lines_as(
            "Candy Maker",
            args!["Guarana candy. That was the most unique masterpiece in my life for sure!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Candy Maker",
            args![
                "Since you helped me, guarana supply has been steadily rising.",
                "So, naturally I'm back to making guarana candy."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Candy Maker", args!["What about it? Wanna buy some?", "It's 4000 zeny each."])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Buy a Guarana Candy.:Cancel.")])? {
            1 => {
                if ctx.var("Zeny").get()?.number()? > 3999 {
                    ctx.lines_as("Candy Maker", args!["Here is a delicious guarana candy."])?;
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(4000))?))?;
                    ctx.call(Function::GetItem, vec![Val::from(12414), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Candy Maker",
                        args![
                            "What? You should say before if you don't have money!",
                            "Even if you are poor, I can't give this away for free."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines_as(
                    "Candy Maker",
                    args!["Sometimes some people don't like it due to it's arousal effect."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Candy Maker",
            args!["Guarana candy. That was the most unique masterpiece in my life for sure!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn candy_maker(ctx: &Ctx) -> Script {
    candy_maker_body(ctx, Vec::new()).map(|_| ())
}

fn cherto_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("brazil_gua").get()?.number()? <= 1 {
        ctx.lines_as("Cherto", args!["Hmm... hey man, you are from outside, aren't you?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Cherto",
            args![
                "Cherto can figure it out even if it's the first time. You can't trick Cherto.",
                "Cherto has sharp eyes like an eagle! Hahaha!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Cherto", args!["Ok, ok. Yes, yes. I see!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Cherto",
            args![
                "Anyway, you arrived in Brasilis but don't know what to do?",
                "Am I right?",
                "You don't know how fortunate you are to have found a really proper helper as myself."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cherto",
            args![
                "Cherto takes it by your expression that you want to say, ''You are a master!'' Right?",
                "Cherto, I can read and figure out all at once! That is written in your face!"
            ],
        )?;
        ctx.next()?;
        if ctx.var("brazil_gua").get()? == 0 {
            ctx.lines_as(
                "Cherto",
                args!["Cherto would love to stay here and explain everything to you but he is a busy man."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Cherto",
                args!["If you have a curious thing to ask to Cherto. Cherto will be kind enough to answer."],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Guarana?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as("Cherto", args!["What? Do you want to find a guarana?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Cherto",
                args![
                    "Guarana is only raised in this area, it has a soft inside and is coverd with a light fur.",
                    "It seems a little bit weird but the flower is really big and smells beautiful."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Cherto", args!["A long time ago, guarana was used to relieve desease and thirst. But recently it's getting popular to revitalize body power and increase blood circulation."])?;
            ctx.next()?;
            ctx.lines_as(
                "Cherto",
                args!["Although it has such great effects, Cherto is sorry to inform you that we can't get it anymore."],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Whaaaat??")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as("Cherto", args!["For a while now, guarana berries haven't been growing here."])?;
            ctx.next()?;
            ctx.lines_as("Cherto", args!["Even if Cherto managed to find one, it will rot quickly."])?;
            ctx.next()?;
            ctx.lines_as("Cherto", args!["If only it didn't happen!"])?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("What are you talking about?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Cherto",
                args![
                    "Quiet!!!!!!!!!!!!!!!!",
                    "This story has been forbidden! Someone might be listening to our conversation..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cherto",
                args![
                    "If Cherto tells you, you might get us into trouble. But you look like you really wanna know so let me give you a tip.",
                    "Come closer. Cherto will whisper so nobody can listen in."
                ],
            )?;
            ctx.var("brazil_gua").set(Val::from(2))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("brazil_gua").get()? == 2 {
        ctx.lines_as("Cherto", args!["A Guarana boy was born."])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Guarana kid?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Cherto",
            args![
                "There was woman who was an expert botanist.",
                "The woman was really popular to all living creatures."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Cherto", args!["At around the time her baby was born, she started a guarana farm. For some reason, her brothers were jealous so they destroyed the farm and disappeared.", "That kind of story..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Cherto",
            args!["We can't be sure that baby was born in the world but since that time, all guarana in Brasilis disappeared."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cherto",
            args![
                "Who is the guarana kid?",
                "Pedro who is famous as a greedy man?",
                "Meto who can't endure about all the fruits?",
                "Hovenue who is gloomy?",
                "They might know~!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cherto",
            args!["What about you?", "Who is the guarana kid?", "Will you figure out it? hohohhhhh~"],
        )?;
        ctx.var("brazil_gua").set(Val::from(3))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2192), Val::from(2193)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("brazil_gua").get()? == 3 {
        ctx.lines_as("Cherto", args!["Can you find the guarana kid?", "Maybe yes? Maybe no?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("brazil_gua").get()? == 4 {
        ctx.lines_as("Cherto", args!["Did you find guarana kid?"])?;
        ctx.next()?;
        ctx.mes("- I tell Cherto about the kid making animal-like sounds. -")?;
        ctx.next()?;
        ctx.lines_as(
            "Cherto",
            args![
                "Hoooh. That's unbelievable.",
                "That kid might be a guarana kid. Sure...",
                "According to the story the kid can have conversations with animals."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cherto",
            args!["If he can make crying sounds of animals, they might be able to converse!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Now, what can I do?",
                "If he is the kid from the legend, is there any way to raise the guarana again?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cherto",
            args![
                "Haha!! What'd Cherto say?",
                "Cherto knows all~!!",
                "Cherto's already thought",
                "of the next step."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cherto",
            args![
                "In Brasilis there is an expert Mage.",
                "His name is Paje.",
                "Take this note over to him.",
                "He will show the solution for you and the kid."
            ],
        )?;
        ctx.var("brazil_gua").set(Val::from(5))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2194), Val::from(2195)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Cherto", args!["hoho tickle~tickle~~~~!!!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn cherto(ctx: &Ctx) -> Script {
    cherto_body(ctx, Vec::new()).map(|_| ())
}

fn strange_kid_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("brazil_gua").get()?.number()? < 3 {
        ctx.lines_as("Strange Kid", args!["................"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("brazil_gua").get()? == 3 {
            ctx.lines_as("Strange Kid", args!["................"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Try to talk.:Pretend to pass by.")])?) == 2 {
                ctx.lines_as("Strange Kid", args!["................"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.mes("What can I say to him?")?;
            ctx.next()?;
            'l1: loop {
                if !(true) {
                    break 'l1;
                }
                'b1: {
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "What's your name?:How old are you?:What are you doing?:End conversation.",
                        )],
                    )? {
                        1 => {
                            ctx.lines_as("Strange Kid", args!["Kaaaaaaao~", "Grrrrrrrrr - kaaan-"])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines_as("Strange Kid", args!["Booooowoooooo-", "Booooowoooooo- -"])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines_as(
                                "Strange Kid",
                                args!["chamber pot braeee chamber pot brae chamber pot brae -", "Bbeeeebbeee -"],
                            )?;
                            ctx.next()?;
                        }
                        4 => {
                            ctx.lines_as("Strange Kid", args!["Kaaaaaaao~", "Grrrrrrrrr - kaaan-"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["He makes strange sound like an animals.", "Should I ask advice from Cherto?"],
                            )?;
                            ctx.var("brazil_gua").set(Val::from(4))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(2193), Val::from(2194)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
            }
        } else if ctx.var("brazil_gua").get()? == 4 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["He makes strange sounds like an animal.", "Should I ask advice from Cherto?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("brazil_gua").get()?.number()? > 4 && ctx.var("brazil_gua").get()?.number()? < 9) {
            ctx.lines_as("Strange Kid", args!["Ah...? ah.....?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("brazil_gua").get()? == 9 {
            ctx.lines_as("Strange Kid", args!["ah... ahah....."])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["I don't have a story but there are lots of friends waiting outside."],
            )?;
            ctx.next()?;
            ctx.mes("- You give the feather, fresh meat and branch of grapes to the kid -")?;
            ctx.next()?;
            ctx.lines_as("Strange Kid", args!["Ah............."])?;
            ctx.next()?;
            ctx.lines_as("Strange Kid", args!["Un, uhh....", "mooo... mommy....."])?;
            ctx.next()?;
            ctx.lines_as(
                "Strange Kid",
                args!["Ah..........", "bird....", "mon, mon, mon...key......", "boo, booow..........."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Dog?!",
                    "kkk yes. Lots of friends want to meet you.",
                    "Don't be lonely anymore and be happy with your friends."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Strange Kid", args!["ah....he...hehe...."])?;
            ctx.next()?;
            ctx.mes("- He starts to smile lightly and laughs. -")?;
            ctx.next()?;
            ctx.lines_as(
                "Strange Kid",
                args!["Ye......yes.......", "tha... than......thank......yo.........you."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Alright such a cute smile!", "Be a happy kid as always."],
            )?;
            ctx.next()?;
            ctx.lines_as("Strange Kid", args!["Uh......"])?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["what?"])?;
            ctx.next()?;
            ctx.lines_as("Strange Kid", args!["hey.........."])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Do you want to say anything?"],
            )?;
            ctx.next()?;
            ctx.mes("- You get closer and pretend to take caution. -")?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CHUP")?])?;
            ctx.mes("(kiss~)")?;
            ctx.next()?;
            ctx.mes("- The kid laughs again lightly then puts something in your hand. -")?;
            ctx.next()?;
            ctx.mes("- It's a fresh berry that's colored red and hard. -")?;
            ctx.next()?;
            ctx.lines_as("Strange Kid", args!["ga...ra..........na..."])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Oops, guarana berry?", "Ah! Thank you very much!"],
            )?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_CHUPCHUP")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["If I have this, I can make a guarana candy.", "I better find that Candy Maker!"],
            )?;
            ctx.var("brazil_gua").set(Val::from(10))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2199), Val::from(2200)])?;
            ctx.call(Function::GetItem, vec![Val::from(6237), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("brazil_gua").get()? == 10 {
            if !(ctx.call(Function::CountItem, vec![Val::from(6237)])?.is_true()) {
                ctx.lines_as("Strange Kid", args!["He........."])?;
                ctx.call(Function::GetItem, vec![Val::from(6237), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.mes("- The kid is smiling. -")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn strange_kid_bra(ctx: &Ctx) -> Script {
    strange_kid_bra_body(ctx, Vec::new()).map(|_| ())
}

fn mage_paje_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_display: Vec<Val> = Vec::new();
    ctx.lines_as("Mage Paje", args!["Abracadabra~"])?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_POISONHIT")?, ctx.constant("AREA")?, Val::from("Poring#bra")],
    )?;
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_display, &Val::from(base + 0), Val::from(800), false);
    runtime::local_set(&mut l_display, &Val::from(base + 1), Val::from(876), false);
    runtime::local_set(&mut l_display, &Val::from(base + 2), Val::from(909), false);
    ctx.call(
        Function::SetNpcDisplay,
        vec![
            Val::from("Poring#bra"),
            runtime::local_get(&l_display, &ctx.call(Function::Rand, vec![Val::from(3)])?, false),
        ],
    )?;
    if ctx.var("brazil_gua").get()? != 5 {
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.next()?;
    ctx.lines_as(
        "Mage Paje",
        args!["Ohoooh~!", "I have a guest.", "Good to see you.", "I am the Mage Paje."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["Hello. Mr. Cherto told me to find you."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mage Paje",
        args![
            "Um.. Mr. Cherto? What's happened?",
            "Have you come here to ask about lots of weird rumors?"
        ],
    )?;
    ctx.next()?;
    ctx.mes("- You give the note to Paje-")?;
    ctx.next()?;
    ctx.lines_as("Mage Paje", args!["Ohoooh~", "Hmm gosh.. that's what happened."])?;
    ctx.next()?;
    ctx.lines_as(
        "Mage Paje",
        args![
            "I can't help you directly.",
            "But I will give you simple magic so you can figure it out by yourself."
        ],
    )?;
    ctx.next()?;
    let choice = runtime::select_values(ctx, &[Val::from("What kind of magic?")])?;
    ctx.var("@menu").set(choice)?;
    ctx.lines_as(
        "Mage Paje",
        args!["It's a magic that will make you appear as an animal to other animals. Pretty cool huh?"],
    )?;
    ctx.next()?;
    ctx.lines_as("Mage Paje", args!["Ok~ I will give you the magic.", "Most animals are really sensitive so they might be aware of it. Find a Toucan in the field that's oblivious to the spell. You'll know when you talk to it."])?;
    ctx.next()?;
    ctx.lines_as("Mage Paje", args!["Good luck~!"])?;
    ctx.var("brazil_gua").set(Val::from(6))?;
    ctx.call(Function::ChangeQuest, vec![Val::from(2195), Val::from(2196)])?;
    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_ASSUMPTIO")?])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mage_paje_bra(ctx: &Ctx) -> Script {
    mage_paje_bra_body(ctx, Vec::new()).map(|_| ())
}

fn poring_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    Ok(Val::from(0))
}

pub fn poring_bra(ctx: &Ctx) -> Script {
    poring_bra_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ToucanBraStep {
    Start,
    OnTouch,
}

fn toucan_bra_run(ctx: &Ctx, mut step: ToucanBraStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ToucanBraStep::Start => {
                step = ToucanBraStep::OnTouch;
                continue 'machine;
            }
            ToucanBraStep::OnTouch => {
                if ctx.var("brazil_gua").get()? == 6 {
                    ctx.lines_as("Toucan", args!["Baaeecc!", "I've never seen you before.", "Baaeec!"])?;
                    ctx.next()?;
                    ctx.lines_as("Toucan", args!["It's the middle of the new and old continent... I know I've never seen you before but you seem familiar, like a woman dancing a samba. "])?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("What are you talking about?")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as("Toucan", args!["I can feel some similar power like guarana kid. bbaaaeeeccc!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Toucan",
                        args!["That kid has had a really lonely time, baaecc! Perhaps you are a friend of him? Baaeec!!"],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Not yet... but I want to be a friend.")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Toucan",
                        args!["The kid who received care from guarana woman is also a friend of animals. Bbaaeecc!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Toucan",
                        args!["I'd like to give the symbol of a toucan representative for the kid. Bbaaeecc!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Toucan", args!["If you want to relieve his loneliness, can you help me?"])?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Absolutely!")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Toucan",
                        args![
                            "It's my feather.",
                            "Send it to the kid.",
                            "We will keep our promise of friendship between guarana kid and Toucan forever. Bbaaeecc!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("- You take a feather from Toucan. - ")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Toucan",
                        args![
                            "There have to be others around here like me.",
                            "Why don't you find a jaguar Bbaaeecc!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Toucan", args!["I will give a blessing from Toucan to you."])?;
                    ctx.next()?;
                    ctx.lines_as("Toucan", args!["Fly fly far away. bbaaaeeeccckkk--!"])?;
                    ctx.var("brazil_gua").set(Val::from(7))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2196), Val::from(2197)])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SEISMICWEAPON")?])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("bra_fild01"), Val::from(68), Val::from(146)])?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Toucan", args!["Bbbaaeec~! Baaeec~!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn toucan_bra(ctx: &Ctx) -> Script {
    toucan_bra_run(ctx, ToucanBraStep::Start, Vec::new()).map(|_| ())
}

pub fn toucan_bra_ontouch(ctx: &Ctx) -> Script {
    toucan_bra_run(ctx, ToucanBraStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum JaguarBraStep {
    Start,
    OnTouch,
}

fn jaguar_bra_run(ctx: &Ctx, mut step: JaguarBraStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            JaguarBraStep::Start => {
                step = JaguarBraStep::OnTouch;
                continue 'machine;
            }
            JaguarBraStep::OnTouch => {
                if ctx.var("brazil_gua").get()? == 7 {
                    ctx.lines_as("Jaguar", args!["Hhooww..hhooww....."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jaguar",
                        args![
                            "Smelling! This smell is from a human!",
                            "Somewhere, a human!",
                            "I got it. You are!!!"
                        ],
                    )?;
                    ctx.call(
                        Function::NpcSpecialEffect,
                        vec![ctx.constant("EF_HIT1")?, ctx.constant("AREA")?, Val::from("Jaguar#bra")],
                    )?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_HUK")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jaguar",
                        args![
                            "Don't be afraid human.",
                            "I don't have enough power to hunt humans, just waiting time to end my lifetime in this jungle."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Jaguar", args!["Anyway you can talk with me, are you a guarana kid?"])?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Yes? N...o......actually....")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Jaguar",
                        args![
                            "The son of guarana woman became our friend also.",
                            "They treated all life preciously.",
                            "I hope you are same the as her."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jaguar",
                        args![
                            "Bird's chirpings informed me.",
                            "The son of guarana woman has a diseased heart.",
                            "Her brothers made him lonely, don't you think?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jaguar",
                        args![
                            "Here is fresh meat that I hunted just a few days ago.",
                            "Take it and give it to the poor kid."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jaguar",
                        args![
                            "I can give this tiny thing to you so, don't forget it.",
                            "The jungle will welcome you whenever!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("- You get fresh meat from Jaguar. -")?;
                    ctx.next()?;
                    ctx.lines_as("Jaguar", args!["Monkey, who's always meddling with others, wants to meet you."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jaguar",
                        args![
                            "I will give you a Jaguar's high blessing.",
                            "Go to monkey by flowing through the wind like a bee.",
                            "Let's meet again my friend!"
                        ],
                    )?;
                    ctx.var("brazil_gua").set(Val::from(8))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2197), Val::from(2198)])?;
                    ctx.close_window()?;
                    ctx.call(Function::ConsumeItem, vec![Val::from(12016)])?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Jaguar", args!["krrrrrr...."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn jaguar_bra(ctx: &Ctx) -> Script {
    jaguar_bra_run(ctx, JaguarBraStep::Start, Vec::new()).map(|_| ())
}

pub fn jaguar_bra_ontouch(ctx: &Ctx) -> Script {
    jaguar_bra_run(ctx, JaguarBraStep::OnTouch, Vec::new()).map(|_| ())
}

fn monkeybra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    Ok(Val::from(0))
}

pub fn monkeybra(ctx: &Ctx) -> Script {
    monkeybra_body(ctx, Vec::new()).map(|_| ())
}

fn monkey_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("brazil_gua").get()? == 8 {
        ctx.lines_as("Monkey", args!["What is it??!!", "We don't tolerate humans? Get out~!!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Monkey",
            args![
                "Nono... wait.... that scent!!",
                "I can smell Jaguar from you, who are you?",
                "Gosh, maybe there's no jaguar without fur and weird shape!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["........................", "Are you saying that I look like an animal?!?!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Monkey", args!["Uh? Aren't you a jaguar?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Monkey",
            args![
                "Ahhha. Jaguar send you to me, right?? kkkikkki",
                "But you don't look like guarana kid."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("I've come here to help him.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Monkey",
            args![
                "I heard guarana kid became lonely, is he?",
                "We are experts in acrobatic acts, does kid like it?? kkkickkksk!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Monkey",
            args![
                "Give this branch of grapes to guarana kid.",
                "We will make him have fun during the whole night whenever he comes to us!! kkkickkksk!"
            ],
        )?;
        ctx.next()?;
        ctx.mes("- You get a bunch of grapes from Monkey. -")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Good~ Now it's time to go back to the kid~!!"],
        )?;
        ctx.var("brazil_gua").set(Val::from(9))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2198), Val::from(2199)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Monkey", args!["kkkickkksk!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn monkey_bra(ctx: &Ctx) -> Script {
    monkey_bra_body(ctx, Vec::new()).map(|_| ())
}

fn botanist_karmen_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("brazil_regia").get()? == 0 {
        ctx.lines_as(
            "Karmen",
            args![
                "Brasilis' climate is special.",
                "This climate offers special cases in botany classes different from any other regions of the world."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Karmen",
            args![
                "The plants here have robust frames and are clear and colorful.",
                "Here the plants are really huge and we can feel their presence."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Karmen",
            args![
                "One of them, a Water Lily, is a really gorgeous and unique plant.",
                "This flower is quite sensitive so it doesn't bloom everywhere."
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Interesting.:End conversation.")])?) == 2 {
            ctx.lines_as("Karmen", args!["I guess you aren't interested in botany."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Karmen", args!["It doesn't appear easily and it is a mysterious flower even to the natives, so the Brasilis people believe that a person will get great luck if someone finds it."])?;
        ctx.next()?;
        ctx.lines_as(
            "Karmen",
            args!["As a botanist, I have been hanging around here to find the lucky flower but as I expected, it hasn't shown itself yet."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Karmen",
            args!["I believe that with enough perseverence, this flower will show me it's beautiful brilliance."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Karmen",
            args![
                "Ah, if you are interested more in the Water Lily story, find someone named Marta.",
                "She is wise and knows lots of stories here in Brasilis."
            ],
        )?;
        ctx.var("brazil_regia").set(Val::from(1))?;
        ctx.call(Function::SetQuest, vec![Val::from(2201)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("brazil_regia").get()? == 1 {
        ctx.lines_as(
            "Karmen",
            args![
                "Ah, if you are interested more in the Water Lily story, find someone named Marta.",
                "She is wise and knows lots of stories here in Brasilis."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("brazil_regia").get()? == 9 {
        ctx.mes("- You show a lotus flower to Karmen and talk about the story so far. -")?;
        ctx.next()?;
        ctx.lines_as(
            "Karmen",
            args![
                "Wow!! You had a really good experience.",
                "So~~~ the water lily lives in the depths of brasilis, right?",
                "I wil try to find it again by myself, I won't give up!!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Karmen",
            args![
                "I am so grateful that I met you.",
                "The water lily must truly be a lucky flower. hahaha"
            ],
        )?;
        ctx.var("brazil_regia").set(Val::from(10))?;
        ctx.call(Function::CompleteQuest, vec![Val::from(2207)])?;
        if (ctx.constant("VIP_SCRIPT")?.is_true() && ctx.call(Function::VipStatus, vec![ctx.constant("VIP_STATUS_ACTIVE")?])?.is_true()) {
            ctx.call(Function::GetExperience, vec![Val::from(75000), Val::from(0)])?;
        } else {
            ctx.call(Function::GetExperience, vec![Val::from(50000), Val::from(0)])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Karmen",
            args!["This climate offers special cases in botany classes different from any other regions of the world."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Karmen",
            args![
                "The plants here have robust frames and are clear and colorful.",
                "Here the plants are really huge and we can feel their presence."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Karmen", args!["It's a botanist's dream."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn botanist_karmen_bra(ctx: &Ctx) -> Script {
    botanist_karmen_bra_body(ctx, Vec::new()).map(|_| ())
}

fn marta_bra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("brazil_regia").get()? == 1 {
        ctx.lines_as("Brasilis Boy", args!["Grandma! That person has a weird smell."])?;
        ctx.next()?;
        ctx.lines_as("Marta", args!["This person isn't from here.", "Say hello to our guest."])?;
        ctx.next()?;
        ctx.lines_as("Brasilis Boy", args!["heee~ hi!!", "I am Kaka!!", "Whats your name?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![((Val::from("I am ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Brasilis Boy",
            args![
                "The outsider has a weird name!",
                "Thas ok! If we keep talking we'll be friends! Cheer up!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Marta", args!["Hehe...", "So, why have you come here stranger~?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I heard you knows lots of stories, is that true?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kaka",
            args![
                "Wooo! how you know my grandma knows lots of stories, amazing~?",
                "Grandma is really wise and kind so, I heard lotsa things."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marta",
            args![
                "Hehe. Kaka always listens to many stories every night, he really likes my stories.",
                "Kaka always makes me happy because he asks so many curious things. That is pure happiness."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marta",
            args!["Ok, Kaka why don't you invite our guest today to our small meeting?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Kaka", args!["Ok grandma~!!"])?;
        ctx.next()?;
        ctx.lines_as("Marta", args!["Hey~ do you have special story that you want to listen to?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["About the mysterious water lily?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Marta", args!["Water lily....", "It's from a long long time ago."])?;
        ctx.next()?;
        ctx.lines_as(
            "Marta",
            args![
                "Before Brasilis was established.",
                "A tribe that lived with the giant waterfall and jungle as friends spent their whole time with nature."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Marta", args!["One of tribe chiefs had a pretty daughter called 'Naia'."])?;
        ctx.next()?;
        ctx.lines_as(
            "Marta",
            args!["Naia liked listening to stories like Kaka so, her mom told her stories every night about nature and gods."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kaka",
            args!["Woooa, she's just like me!", "Maybe she would be pretty... hehe."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marta",
            args![
                "According to her mother...",
                "If the moon in the sky loves some woman in the earth, he turns her into a star so that they can stay together forever."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marta",
            args!["After Naia heard this story, she went to her dad to ask if it was true or not."],
        )?;
        ctx.next()?;
        ctx.lines_as("Kaka", args!["So, what did he say?"])?;
        ctx.next()?;
        ctx.lines_as("Marta", args!["^3131FF'My dear, Naia the moon is one of the bravest men. But he can no longer have a bride. So you can't become a star... Sorry~.^000000"])?;
        ctx.next()?;
        ctx.lines_as("Kaka", args!["Did Naia wants to be the bride of the man?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Marta",
            args!["Yes Kaka, imagine the moon how beautiful and mysterious, that's just ideal for girls."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kaka",
            args!["But the moon doesn't meet a human as his wife anymore? What was going on with Naia?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Marta", args!["Naia was really a nice girl."])?;
        ctx.next()?;
        ctx.lines_as(
            "Marta",
            args![
                "Although her parents tried to prevent her, she still went to the forest to meet the moon every night.",
                "Sadly, even with all of her effort, the moon didn't show any reaction to her."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Marta", args!["One day she also went to the top of the mountain to be closer to him. She decided to take a rest for a while around the lake."])?;
        ctx.next()?;
        ctx.lines_as(
            "Marta",
            args![
                "That's when.. Naia saw it.",
                "It was the moon he was shining beautifully over the waving lake lightly."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Kaka", args!["I know, it's just the moon reflecting on the water. Right?!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Marta",
            args!["Yes, but to her, the image made her fall into the lake without hesitating and drowned."],
        )?;
        ctx.next()?;
        ctx.lines_as("Kaka", args!["Oh no."])?;
        ctx.next()?;
        ctx.lines_as(
            "Marta",
            args![
                "The moon was also watching her from the sky.",
                "He felt sad and pitied her. So he decided to turn her into a beautiful flower to thank her for her love."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Marta", args!["That is the story of the mysterious flower people called the Brasilis Water Flower.", "This Naia flower appears as light white during daytime but in the night turns into red due to it's love connection to the moon."])?;
        ctx.next()?;
        ctx.lines_as("Kaka", args!["How sad but beautiful!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Marta",
            args![
                "How about you stranger?",
                "Did you enjoy this story?",
                "If you want to listen to another story, just come to me.",
                "If you don't mind playing with my grandson a ~ little. hoohoo."
            ],
        )?;
        ctx.var("brazil_regia").set(Val::from(2))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("brazil_regia").get()?.number()? > 1 {
        ctx.lines_as(
            "Kaka",
            args![
                "My grandma is really a bit tired doing some tribe stuff!",
                "Could you come another day?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Marta",
            args!["You are not from around here.", "I can sense a strange earth smell."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marta",
            args![
                "But your eyes shine with strength.",
                "Indeed you are spreading out spirit and will from your whole body."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Marta", args!["If you work at it you will be a great person someday."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn marta_bra(ctx: &Ctx) -> Script {
    marta_bra_body(ctx, Vec::new()).map(|_| ())
}
