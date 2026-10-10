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
enum HalloweenMagicianIro08Step {
    Start,
    Rules,
    Participate,
    TicketExchange,
    NextTime,
    MainMenu,
    MainMenu2,
    NotEnough,
    Enough,
}

fn halloween_magician_iro08_run(ctx: &Ctx, mut step: HalloweenMagicianIro08Step) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HalloweenMagicianIro08Step::Start => {
                if ctx.var("hallow08").get()?.number()? < 1 {
                    ctx.lines_as(
                        "Halloween Magician",
                        args![
                            "Kkkkkkkkk!",
                            "I have a special event this Halloween that tests your luck and agility.",
                            "Are you interested?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Halloween Magician",
                        args![
                            "Come on! Don't be a wuss!",
                            "If you collect enough tickets you can get good prizes!",
                            "So what do you say?"
                        ],
                    )?;
                    ctx.next()?;
                    step = HalloweenMagicianIro08Step::MainMenu;
                    continue 'machine;
                }
                if ctx.var("hallow08kill").get()? == 1 {
                    ctx.lines_as(
                        "Halloween Magician",
                        args![
                            "You a 'fraidy cat or something?!",
                            "You know you want to try again...",
                            "Do you know the rules?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("hallow08kill").set(0)?;
                    step = if ctx.menu(&["Yes, I know.", "No, I don't know."])? == 1 {
                        HalloweenMagicianIro08Step::Rules
                    } else {
                        HalloweenMagicianIro08Step::Participate
                    };
                    continue 'machine;
                }
                if ctx.var("hallow08kill").get()? == 2 {
                    ctx.lines_as(
                        "Halloween Magician",
                        args![
                            "Oh, well done! You are alright!",
                            "Isn't it fun with zombies??",
                            "You know, zombies were people too!",
                            "Ha!",
                            "Kkkkkkk."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.items().give(7941, 1)?;
                    ctx.var("hallow08kill").set(0)?;
                    ctx.lines_as(
                        "Halloween Magician",
                        args!["As I promised", "You can get Halloween tickets for cool items."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("hallow08").get()?.number()? > 0 {
                    ctx.lines_as(
                        "Halloween Magician",
                        args!["Well, do you want to hear the rules again or, just get back to it..."],
                    )?;
                    ctx.next()?;
                    step = HalloweenMagicianIro08Step::MainMenu2;
                    continue 'machine;
                }
                step = HalloweenMagicianIro08Step::Rules;
                continue 'machine;
            }
            HalloweenMagicianIro08Step::Rules => {
                ctx.lines_as(
                    "Halloween Magician",
                    args![
                        "This village is like a virtual Payon.",
                        "There are zombies and ghouls roaming around and three southern exits, but only one works.",
                        "That's up to you to find out."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Halloween Magician",
                    args![
                        "You can't use any skills to kill the ghouls or zombies.",
                        "And one more thing...",
                        "you shouldn't forget..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Halloween Magician",
                    args![
                        "All participants should be wearing nothing.",
                        "Put all belongings in your storage and come back here when your weight is '0'."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Halloween Magician",
                    args![
                        "Oh and one more thing!",
                        "You can't be riding a PecoPeco or have a Cart.",
                        "If you are, then I will remove them before you enter.",
                        "Got it?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Halloween Magician", args!["Remember, there are three exits but only one works randomly, the zombies and ghouls roaming around there can't be killed and you can't be wearing anything."])?;
                ctx.next()?;
                if ctx.var("hallow08").get()?.number()? > 0 {
                    ctx.lines_as("Halloween Magician", args!["Hey...", "Come back once you're ready."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = HalloweenMagicianIro08Step::MainMenu;
            }
            HalloweenMagicianIro08Step::Participate => {
                ctx.lines_as("Halloween Magician", args!["Ok, you are ready.", "Let me check your weight."])?;
                ctx.next()?;
                if ctx.var("Weight").get()?.number()? > 0 {
                    ctx.lines_as("Halloween Magician", args!["Gosh!", "There's always a black sheep anywhere."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Halloween Magician",
                        args!["You think I wouldn't notice that your weight is above '0'?", "You're overweight..."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Halloween Magician",
                    args!["You seem good to go, and your weight is just right."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Halloween Magician",
                    args!["I was quite swamped with my work, so I'm exhausted."],
                )?;
                ctx.next()?;
                ctx.lines_as("Halloween Magician", args!["I sometimes forget to send you there..."])?;
                ctx.next()?;
                ctx.lines_as("Halloween Magician", args!["I hope you come back well."])?;
                ctx.close_window()?;
                if ctx.var("hallow08").get()?.number()? < 1 {
                    ctx.var("hallow08").set(1)?;
                }
                ctx.var("hallow08kill").set(1)?;
                ctx.var("@hallow08warp").set(ctx.call(Function::Rand, args![1, 3])?)?;
                ctx.call(Function::PercentHeal, args![-98, 0])?;
                ctx.call(Function::SetRiding, args![0])?;
                ctx.call(Function::SetCart, args![0])?;
                ctx.warp("evt_zombie", 155, 246)?;
                return Err(Stop::End);
            }
            HalloweenMagicianIro08Step::TicketExchange => {
                ctx.lines_as(
                    "Halloween Magician",
                    args!["You want to exchange tickets for prizes?", "Good job! Kkkkkk!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Halloween Magician", args!["Lemme tell you what items you can exchange for."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Halloween Magician",
                    args![
                        "5 tickets for Pumpkin Pie.",
                        "20 tickets for Pumpkin-Head.",
                        "50 tickets for Old Blue Box.",
                        "70 tickets for Old Purple Box.",
                        "200 tickets for Old Card Album."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Halloween Magician", args!["What would you like to exchange for?"])?;
                ctx.next()?;
                let (cost, prize) = match ctx.menu(&["Pumpkin Pie", "Pumpkin-Head", "Old Blue Box", "Old Purple Box", "Old Card Album"])? {
                    0 => (5, 12192),
                    1 => (20, 5134),
                    2 => (50, 603),
                    3 => (70, 617),
                    _ => (200, 616),
                };
                if ctx.items().count(7941)? < cost {
                    step = HalloweenMagicianIro08Step::NotEnough;
                    continue 'machine;
                }
                ctx.call(Function::DelItem, args![7941, cost])?;
                ctx.call(Function::GetItem, args![prize, 1])?;
                step = HalloweenMagicianIro08Step::Enough;
                continue 'machine;
            }
            HalloweenMagicianIro08Step::NextTime => {
                ctx.lines_as("Halloween Magician", args!["Ok, see you then.", "Kkkkkkkk."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            HalloweenMagicianIro08Step::MainMenu => {
                let choice = runtime::select(
                    ctx,
                    &[
                        "Explain it to me.",
                        "I want to participate.",
                        "I want to exchange tickets for prizes.",
                        "I'll come back next time.",
                    ],
                )?;
                ctx.var("@menu").set(choice)?;
                step = match choice {
                    1 => HalloweenMagicianIro08Step::Rules,
                    2 => HalloweenMagicianIro08Step::Participate,
                    3 => HalloweenMagicianIro08Step::TicketExchange,
                    4 => HalloweenMagicianIro08Step::NextTime,
                    _ => return Err(Stop::Error("Invalid menu selection".into())),
                };
            }
            HalloweenMagicianIro08Step::MainMenu2 => {
                let choice = runtime::select(
                    ctx,
                    &[
                        "Get me back there now!",
                        "Please, tell me the rules",
                        "I want to exchange tickets for prizes.",
                        "I'll come back next time.",
                    ],
                )?;
                ctx.var("@menu").set(choice)?;
                step = match choice {
                    1 => HalloweenMagicianIro08Step::Participate,
                    2 => HalloweenMagicianIro08Step::Rules,
                    3 => HalloweenMagicianIro08Step::TicketExchange,
                    4 => HalloweenMagicianIro08Step::NextTime,
                    _ => return Err(Stop::Error("Invalid menu selection".into())),
                };
            }
            HalloweenMagicianIro08Step::NotEnough => {
                ctx.lines_as(
                    "Halloween Magician",
                    args![
                        "You don't have enough tickets!",
                        "Can't you even count?",
                        "Please come here with the right number of tickets."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            HalloweenMagicianIro08Step::Enough => {
                ctx.lines_as("Halloween Magician", args!["Here it is.", "Do you need...", "anything else?"])?;
                ctx.next()?;
                step = if ctx.var("hallow08").get()? == 1 {
                    HalloweenMagicianIro08Step::MainMenu2
                } else {
                    HalloweenMagicianIro08Step::MainMenu
                };
            }
        }
    }
}

pub fn halloween_magician_iro08(ctx: &Ctx) -> Script {
    halloween_magician_iro08_run(ctx, HalloweenMagicianIro08Step::Start).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Zombiewarp001Step {
    Start,
    OnTouch,
}

fn zombiewarp001_run(ctx: &Ctx, mut step: Zombiewarp001Step) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Zombiewarp001Step::Start => {
                step = Zombiewarp001Step::OnTouch;
                continue 'machine;
            }
            Zombiewarp001Step::OnTouch => {
                if ctx.var("@hallow08warp").get()? == 1 {
                    ctx.var("hallow08kill").set(2)?;
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_BASH])?;
                    ctx.warp("payon", 28, 142)?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn zombiewarp001(ctx: &Ctx) -> Script {
    zombiewarp001_run(ctx, Zombiewarp001Step::Start).map(|_| ())
}

pub fn zombiewarp001_ontouch(ctx: &Ctx) -> Script {
    zombiewarp001_run(ctx, Zombiewarp001Step::OnTouch).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Zombiewarp002Step {
    Start,
    OnTouch,
}

fn zombiewarp002_run(ctx: &Ctx, mut step: Zombiewarp002Step) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Zombiewarp002Step::Start => {
                step = Zombiewarp002Step::OnTouch;
                continue 'machine;
            }
            Zombiewarp002Step::OnTouch => {
                if ctx.var("@hallow08warp").get()? == 2 {
                    ctx.var("hallow08kill").set(2)?;
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_BASH])?;
                    ctx.warp("payon", 121, 40)?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn zombiewarp002(ctx: &Ctx) -> Script {
    zombiewarp002_run(ctx, Zombiewarp002Step::Start).map(|_| ())
}

pub fn zombiewarp002_ontouch(ctx: &Ctx) -> Script {
    zombiewarp002_run(ctx, Zombiewarp002Step::OnTouch).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Zombiewarp003Step {
    Start,
    OnTouch,
}

fn zombiewarp003_run(ctx: &Ctx, mut step: Zombiewarp003Step) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Zombiewarp003Step::Start => {
                step = Zombiewarp003Step::OnTouch;
                continue 'machine;
            }
            Zombiewarp003Step::OnTouch => {
                if ctx.var("@hallow08warp").get()? == 3 {
                    ctx.var("hallow08kill").set(2)?;
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_BASH])?;
                    ctx.warp("payon", 253, 95)?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn zombiewarp003(ctx: &Ctx) -> Script {
    zombiewarp003_run(ctx, Zombiewarp003Step::Start).map(|_| ())
}

pub fn zombiewarp003_ontouch(ctx: &Ctx) -> Script {
    zombiewarp003_run(ctx, Zombiewarp003Step::OnTouch).map(|_| ())
}
