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
enum SugarStep {
    Start,
    CandyCane,
    Cookie,
    AskAmount,
    Teacher,
    TooMany,
    NoMoney,
    Declined,
}

fn sugar_run(ctx: &Ctx, mut step: SugarStep) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SugarStep::Start => {
                ctx.var("@maplenum").set(0)?;
                ctx.var("@mapleitemid").set(0)?;
                ctx.var("@mapleprice").set(0)?;
                ctx.var("@maplepricet").set(0)?;
                ctx.lines_as(
                    "Sugar",
                    args![
                        "Welcome!",
                        "How delicious are sweets?",
                        "My teacher.........",
                        "The sweets craftsman of ARUBERUTA",
                        "There are sweets that is built hard."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sugar",
                    args![
                        "It was given by the darling person.",
                        "In return of the present ....",
                        "heartfelt like",
                        "the sweetness of the present some how."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select(ctx, &["Please give me!", "I don't need it.", "The teacher."])?;
                ctx.var("@menu").set(choice)?;
                match choice {
                    1 => {}
                    2 => {
                        step = SugarStep::Declined;
                        continue 'machine;
                    }
                    3 => {
                        step = SugarStep::Teacher;
                        continue 'machine;
                    }
                    _ => return Err(Stop::Error("Invalid menu selection".into())),
                }
                ctx.lines_as(
                    "Sugar",
                    args![
                        "Yes!",
                        "Select from menu here.",
                        "Since there is a limitation in numbers",
                        "Not more than ^ff0000 5 pieces^000000.",
                        "are allowed to carry out?"
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select(ctx, &["Candy", "Candy Cane", "Well baked cookie"])?;
                ctx.var("@menu").set(choice)?;
                match choice {
                    1 => {}
                    2 => {
                        step = SugarStep::CandyCane;
                        continue 'machine;
                    }
                    3 => {
                        step = SugarStep::Cookie;
                        continue 'machine;
                    }
                    _ => return Err(Stop::Error("Invalid menu selection".into())),
                }
                ctx.var("@mapleprice").set(3000)?;
                ctx.var("@mapleitemid").set(529)?;
                ctx.lines_as(
                    "Sugar",
                    args![
                        "It is a candy, and the price is",
                        "3000 Zeny each.",
                        "How many do you like to purchase?"
                    ],
                )?;
                ctx.next()?;
                step = SugarStep::AskAmount;
                continue 'machine;
            }
            SugarStep::CandyCane => {
                ctx.var("@mapleprice").set(4000)?;
                ctx.var("@mapleitemid").set(530)?;
                ctx.lines_as(
                    "Sugar",
                    args![
                        "It is a candy cane, and the price is",
                        "4000 Zeny each.",
                        "How many do you like to purchase?"
                    ],
                )?;
                ctx.next()?;
                step = SugarStep::AskAmount;
                continue 'machine;
            }
            SugarStep::Cookie => {
                ctx.var("@mapleprice").set(2000)?;
                ctx.var("@mapleitemid").set(538)?;
                ctx.lines_as(
                    "Sugar",
                    args![
                        "It is a well baked cookie, and the price is",
                        "2000 Zeny each.",
                        "How many do you like to purchase?"
                    ],
                )?;
                ctx.next()?;
                step = SugarStep::AskAmount;
                continue 'machine;
            }
            SugarStep::AskAmount => {
                let (input, _status) = runtime::input_number(ctx, None, None)?;
                ctx.var("@maplenum").set(input)?;
                if ctx.var("@maplenum").get()?.number()? > 5 {
                    step = SugarStep::TooMany;
                    continue 'machine;
                }
                if ctx.var("@maplenum").get()?.number()? == 0 {
                    step = SugarStep::Declined;
                    continue 'machine;
                }
                ctx.var("@maplepricet")
                    .set(ctx.var("@mapleprice").get()?.number()? * ctx.var("@maplenum").get()?.number()?)?;
                if ctx.player().zeny()? < ctx.var("@maplepricet").get()?.number()? {
                    step = SugarStep::NoMoney;
                    continue 'machine;
                }
                ctx.player()
                    .set_zeny(ctx.player().zeny()? - ctx.var("@maplepricet").get()?.number()?)?;
                ctx.call(
                    Function::GetItem,
                    args![ctx.var("@mapleitemid").get()?, ctx.var("@maplenum").get()?],
                )?;
                ctx.lines_as(
                    "Sugar",
                    args![
                        "Thank you!!!",
                        "These sweets are really delicious.",
                        "Since my teacher of sweet is the No.1 teacher's in world!",
                        "Although you may eat by yourself",
                        "don't eat so much or you'll grow fat.",
                        "Please take care!!!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            SugarStep::Teacher => {
                ctx.lines_as(
                    "Sugar",
                    args![
                        "Yes",
                        "The teacher of mine",
                        "is Mr. Kuberu, a sweets craftsman.",
                        "Making sweets under two persons.",
                        "which is allowed to self-train."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sugar",
                    args![
                        "Although selling is seemingly to carried out ....",
                        "Where he is now?",
                        "Which I don't know."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            SugarStep::TooMany => {
                ctx.lines_as(
                    "Sugar",
                    args![
                        "???",
                        "You seem to have a failure on hearing.",
                        "I will tell you once again?",
                        "You can only purchase",
                        "^ff0000 5 pieces^000000 at once."
                    ],
                )?;
                ctx.next()?;
                step = SugarStep::AskAmount;
                continue 'machine;
            }
            SugarStep::NoMoney => {
                ctx.lines_as(
                    "Sugar",
                    args![
                        "???",
                        "Hmmm it seems you don't have enough money",
                        "to make that purchase.",
                        "I will ask you to check your money first."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            SugarStep::Declined => {
                ctx.lines_as(
                    "Sugar",
                    args![
                        "Really .... You might regret it..",
                        "If you change your mind.",
                        "I am just here ok.",
                        "Have a nice day!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn sugar(ctx: &Ctx) -> Script {
    sugar_run(ctx, SugarStep::Start).map(|_| ())
}
