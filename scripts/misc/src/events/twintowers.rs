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
enum TwinTowersTt1Step {
    Start,
    Declined,
    ChooseDestination,
    HiddenTemple,
    OrcDungeon,
    AntHell,
    MjolnirWastePit,
    Sphinx,
    GlastHeim,
    Comodo,
}

fn twin_towers_tt1_run(ctx: &Ctx, mut step: TwinTowersTt1Step) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TwinTowersTt1Step::Start => {
                ctx.lines_as(
                    "Twin Towers",
                    args![
                        "How are you? We are the Twin Towers.",
                        "It is such a pleasure to be able to meet you here.",
                        "I suppose you know that this is Ragnarok Online, a land of dreams and fantasies.",
                        "Are you having a joyous adventure and exciting experience?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Twin Towers",
                    args![
                        "Although we can't move around and can't live the way you do,",
                        "we love the world as much as you do!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Twin Towers",
                    args![
                        "May you experience the sensation of this lovely world!",
                        "For this reason, we are here at your service with our special magic.",
                        "Kindly let us know."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select(ctx, &["I shall accept your offer.", "I'll ask for your service next time."])?;
                ctx.var("@menu").set(choice)?;
                step = match choice {
                    1 => TwinTowersTt1Step::ChooseDestination,
                    2 => TwinTowersTt1Step::Declined,
                    _ => return Err(Stop::Error("Invalid menu selection".into())),
                };
                continue 'machine;
            }
            TwinTowersTt1Step::Declined => {
                ctx.lines_as(
                    "Twin Towers",
                    args![
                        "Er, what a pity. Traveling by yourself is still the best evidence of adventure.",
                        "Isn't this proving that you are still young?",
                        "We respect brave hearts like this"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Twin Towers",
                    args![
                        "There are good and bad times in life, moreover, adventure isn't an easy task in the first place.",
                        "Isn't this true?",
                        "Feel free to come to us when you have time, we will always be there to serve you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Twin Towers",
                    args![
                        "Forget all your troubles, and create a splendid legend in this wonderful world.",
                        "This is such a wonderful world, and you'll always be a great adventurer!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            TwinTowersTt1Step::ChooseDestination => {
                ctx.lines_as(
                    "Twin Towers",
                    args![
                        "The flaming passion of an adventurer,",
                        "The desire to explore the unknown realms,",
                        "The dedication and commitment to achieve the aspiration...",
                        "You are simply a true adventurer with what compassion."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Twin Towers", args!["We wish to help passionate adventurers.", "Although we are not able to move, luckily we have the special ability that can warp you to places of danger and excitement."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Twin Towers",
                    args![
                        "Come on! Where do you wish to go?",
                        "Just let us know you desired destination and we will send your there!"
                    ],
                )?;
                let choice = runtime::select(
                    ctx,
                    &[
                        "Hidden Temple",
                        "Orc Dungeon",
                        "Ant Hell",
                        "Mjolnir Waste Pit",
                        "Sphinx",
                        "Glast Heim",
                        "Comodo",
                    ],
                )?;
                ctx.var("@menu").set(choice)?;
                step = match choice {
                    1 => TwinTowersTt1Step::HiddenTemple,
                    2 => TwinTowersTt1Step::OrcDungeon,
                    3 => TwinTowersTt1Step::AntHell,
                    4 => TwinTowersTt1Step::MjolnirWastePit,
                    5 => TwinTowersTt1Step::Sphinx,
                    6 => TwinTowersTt1Step::GlastHeim,
                    7 => TwinTowersTt1Step::Comodo,
                    _ => return Err(Stop::Error("Invalid menu selection".into())),
                };
                continue 'machine;
            }
            TwinTowersTt1Step::HiddenTemple => {
                ctx.warp("prt_fild01", 136, 368)?;
                return Err(Stop::End);
            }
            TwinTowersTt1Step::OrcDungeon => {
                ctx.warp("gef_fild10", 67, 334)?;
                return Err(Stop::End);
            }
            TwinTowersTt1Step::AntHell => {
                ctx.warp("moc_fild04", 210, 329)?;
                return Err(Stop::End);
            }
            TwinTowersTt1Step::MjolnirWastePit => {
                ctx.warp("mjolnir_02", 79, 361)?;
                return Err(Stop::End);
            }
            TwinTowersTt1Step::Sphinx => {
                ctx.warp("moc_fild19", 105, 99)?;
                return Err(Stop::End);
            }
            TwinTowersTt1Step::GlastHeim => {
                ctx.warp("gef_fild06", 45, 304)?;
                return Err(Stop::End);
            }
            TwinTowersTt1Step::Comodo => {
                ctx.warp("cmd_fild01", 30, 317)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn twin_towers_tt1(ctx: &Ctx) -> Script {
    twin_towers_tt1_run(ctx, TwinTowersTt1Step::Start).map(|_| ())
}
