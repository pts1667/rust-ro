#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum FatherChristmasStep {
    Start,
    Intro,
    Problem,
    Proof,
    NotEnough,
    Farewell,
    OnInit,
}

fn father_christmas_run(ctx: &Ctx, mut step: FatherChristmasStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            FatherChristmasStep::Start => {
                ctx.mes("[Santa Claus]")?;
                if ctx.var("xmas_npc").get()? == 0 {
                    ctx.var("xmas_npc").set(1)?;
                }
                if ctx.var("#event_xmas").get()?.number()? > 0 && ctx.var("#event_xmas").get()?.number()? < 30 {
                    step = FatherChristmasStep::Intro;
                    continue 'machine;
                }
                ctx.mes("Merry Christmas!")?;
                if ctx.player().class()? == 0 || ctx.var("#event_xmas").get()?.number()? >= 30 {
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.mes("I have a gift for you! Ho Ho Ho!")?;
                ctx.call(Function::GetItem, args![ctx.call(Function::Rand, args![664, 667])?, 1])?;
                ctx.var("#event_xmas").set(ctx.var("#event_xmas").get()?.number()? + 1)?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            FatherChristmasStep::Intro => {
                ctx.lines(args!["I'm having a bit of a problem...", "Do you care to listen?"])?;
                ctx.next()?;
                let choice = runtime::select(ctx, &["Listen to Santa Claus.", "Give Santa Claus proof.", "Cancel."])?;
                ctx.var("@menu").set(choice)?;
                step = match choice {
                    1 => FatherChristmasStep::Problem,
                    2 => FatherChristmasStep::Proof,
                    3 => FatherChristmasStep::Farewell,
                    _ => return Err(Stop::Error("Invalid menu selection".into())),
                };
            }
            FatherChristmasStep::Problem => {
                ctx.lines_as(
                    "Santa Claus",
                    args![
                        "My problem is this.",
                        "There seems to be a man out there",
                        "that is impersonating me and spreading",
                        "terror throughout the land."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Santa Claus",
                    args![
                        "Like the Grinch of legend, he's taking",
                        "all the childrens' toys and keeping them",
                        "for himself."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Santa Claus",
                    args![
                        "I'm too busy here creating my batch of ",
                        "toys for next year, so I can't go",
                        "out and find him myself.",
                        "So I would like you to go out and",
                        "Destroy this man for me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Santa Claus",
                    args![
                        "He has in his posession one of my",
                        "magic sacks, however, so he will",
                        "escape into it to another place each",
                        "time you attack him."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Santa Claus",
                    args![
                        "However,",
                        "In his haste, he tends to drop things.",
                        "If by chance he drops one of his Stockings",
                        "With Holes that he uses to steal the",
                        "poor childrens' toys, pick it up."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Santa Claus",
                    args![
                        "If you collect 3 of these, I will give",
                        "you a prototype mystery box that",
                        "I've been keeping around the",
                        "lab. It spits out random presents",
                        "and saves me a ton of work."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            FatherChristmasStep::Proof => {
                ctx.mes("[Santa Claus]")?;
                if ctx.items().count(7034)? < 3 {
                    step = FatherChristmasStep::NotEnough;
                    continue 'machine;
                }
                ctx.items().take(7034, 3)?;
                ctx.lines(args![
                    "Seems you've been doing a",
                    "good job of taking down those",
                    "fake Santas. Keep it up!"
                ])?;
                ctx.next()?;
                ctx.items().give(644, 1)?;
                ctx.var("#event_xmas").set(ctx.var("#event_xmas").get()?.number()? + 1)?;
                ctx.lines_as(
                    "Santa Claus",
                    args![
                        "There's your reward.",
                        "If you get 3 more, I'll give you another.",
                        "Hope you get a good item."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            FatherChristmasStep::NotEnough => {
                ctx.lines(args![
                    "You don't have enough socks as proof.",
                    "Go take down those evil Santas",
                    "and get more for me and I'll reward you."
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            FatherChristmasStep::Farewell => {
                ctx.lines_as(
                    "Santa Claus",
                    args!["I see. Well, at the very least", "we shall meet again on Christmas morning."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            FatherChristmasStep::OnInit => {
                ctx.set_npc_visible("Santa Claus", false)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn father_christmas(ctx: &Ctx) -> Script {
    father_christmas_run(ctx, FatherChristmasStep::Start, Vec::new()).map(|_| ())
}

pub fn father_christmas_oninit(ctx: &Ctx) -> Script {
    father_christmas_run(ctx, FatherChristmasStep::OnInit, Vec::new()).map(|_| ())
}
