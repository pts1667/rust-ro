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
enum ArenaPointManagerStep {
    Start,
    ExchangePoints,
}

fn show_point_totals(ctx: &Ctx) -> Result<(), Stop> {
    ctx.lines_as(
        "Arena Point Manager",
        args![
            ctx.player().name()? + ",",
            "you currently have",
            format!("{} Arena Points", ctx.var("arena_point").get()?.text()),
            format!("and {} Turbo Track Points.", ctx.var("tt_point").get()?.text()),
        ],
    )
}

fn show_updated_totals(ctx: &Ctx) -> Result<(), Stop> {
    ctx.lines_as(
        "Arena Point Manager",
        args![
            ctx.player().name()? + ",",
            "you now have",
            format!("^4682B4{}^000000 Arena Points", ctx.var("arena_point").get()?.text()),
            format!("and ^00688B{}^000000 Turbo Track Points.", ctx.var("tt_point").get()?.text()),
            "Thank you for your patronage.",
        ],
    )
}

fn show_max_arena_warning(ctx: &Ctx) -> Result<(), Stop> {
    ctx.lines_as(
        "Arena Point Manager",
        args![
            "You will exceed the",
            "maximum amount of",
            "Arena Points if we proceed",
            "with this conversion of your",
            "Turbo Track Points. You cannot",
            "have more than 29,000 Arena Points."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Arena Point Manager",
        args![
            "Please spend some of",
            "your Arena Points before",
            "using this service again.",
            "Thank you for your patronage."
        ],
    )
}

fn show_canceled(ctx: &Ctx) -> Result<(), Stop> {
    ctx.lines_as("Arena Point Manager", args!["You have", "canceled", "this service."])
}

fn arena_point_manager_run(ctx: &Ctx, step: ArenaPointManagerStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArenaPointManagerStep::Start => {
                ctx.lines_as(
                    "Arena Point Manager",
                    args![
                        "I hope you're having a good",
                        "time in the Arena. If you've",
                        "earned Turbo Track Points at",
                        "Al De Baran's Turbo Track, I can,",
                        "convert them into Arena Points."
                    ],
                )?;
                ctx.next()?;
                match ctx.menu(&["Point Check", "Convert Points", "^660000Conversion Info^000000"])? {
                    0 => show_point_totals(ctx)?,
                    1 => {
                        show_point_totals(ctx)?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Arena Point Manager",
                            args![
                                "Please choose from among",
                                "the Track Point to Arena Point",
                                "conversions. Keep in mind that",
                                "when you convert more than 10",
                                "Track Points at one time, you can only convert in ^4D4DFFmultiples of 10^000000."
                            ],
                        )?;
                        ctx.next()?;
                        match ctx.menu(&[
                            "2 TP -> 1 AP",
                            "4 TP -> 2 AP",
                            "6 TP -> 3 AP",
                            "8 TP -> 4 AP",
                            "10 TP and more",
                            "Cancel",
                        ])? {
                            0 => return arena_point_manager_run(ctx, ArenaPointManagerStep::ExchangePoints, args![28999, 2, 1]),
                            1 => return arena_point_manager_run(ctx, ArenaPointManagerStep::ExchangePoints, args![28998, 4, 2]),
                            2 => return arena_point_manager_run(ctx, ArenaPointManagerStep::ExchangePoints, args![28997, 6, 2]),
                            3 => return arena_point_manager_run(ctx, ArenaPointManagerStep::ExchangePoints, args![28996, 8, 4]),
                            4 => {
                                ctx.lines_as(
                                    "Arena Point Manager",
                                    args![
                                        "Please enter the number",
                                        "of times you wish to convert",
                                        "10 Turbo Track Points into",
                                        "Arena Points. The largest",
                                        "value you may enter is 20.",
                                        "To cancel, enter ''^3355FF0^000000.''"
                                    ],
                                )?;
                                ctx.next()?;
                                let (input, _) = runtime::input_number(ctx, None, None)?;
                                let times = input.number()?;
                                if times <= 0 {
                                    show_canceled(ctx)?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                if times > 20 {
                                    ctx.lines_as(
                                        "Arena Point Manager",
                                        args![
                                            "Your request exceeds",
                                            "the maximum limit. Please",
                                            "enter a value no greater than 20."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                let want_point1 = 10 * times;
                                let want_point = 5 * times;
                                let my_arena_all = ctx.var("arena_point").get()?.number()? + want_point;
                                let my_turbo_all = ctx.var("tt_point").get()?.number()? - want_point1;
                                if my_arena_all > 28999 {
                                    show_max_arena_warning(ctx)?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                if my_turbo_all < 0 {
                                    ctx.lines_as(
                                        "Arena Point Manager",
                                        args![
                                            "I'm sorry, but",
                                            "you don't have enough",
                                            "Turbo Track Points to",
                                            "perform this Arena",
                                            "Point conversion."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Arena Point Manager",
                                    args![
                                        "You have converted",
                                        "10 Turbo Track Points",
                                        format!("into 5 Arena Points {times} times."),
                                        format!("A total of {want_point1} Turbo Track Points were converted into"),
                                        format!("{want_point} Arena Points."),
                                    ],
                                )?;
                                ctx.var("tt_point").set(my_turbo_all)?;
                                ctx.var("arena_point").set(my_arena_all)?;
                                ctx.next()?;
                                show_updated_totals(ctx)?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            5 => show_canceled(ctx)?,
                            _ => {}
                        }
                    }
                    _ => {}
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ArenaPointManagerStep::ExchangePoints => {
                let limit = runtime::arg(&args, 0, Val::from(0)).number()?;
                if ctx.var("arena_point").get()?.number()? > limit {
                    show_max_arena_warning(ctx)?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                let track_points = runtime::arg(&args, 1, Val::from(0)).number()?;
                if ctx.var("tt_point").get()?.number()? >= track_points {
                    let arena_points = runtime::arg(&args, 2, Val::from(0)).number()?;
                    ctx.lines_as(
                        "Arena Point Manager",
                        args![
                            ctx.player().name()? + ",",
                            format!("you've converted {track_points} Track"),
                            format!("Points into {arena_points} Arena Point."),
                        ],
                    )?;
                    ctx.var("tt_point").set(ctx.var("tt_point").get()?.number()? - track_points)?;
                    ctx.var("arena_point").set(ctx.var("arena_point").get()?.number()? + arena_points)?;
                    ctx.next()?;
                    show_updated_totals(ctx)?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Arena Point Manager",
                    args![
                        "I'm sorry, but you don't have",
                        "enough Turbo Track Points.",
                        "You need at least 2 Turbo Track Points for this conversion service."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arena_point_manager(ctx: &Ctx) -> Script {
    arena_point_manager_run(ctx, ArenaPointManagerStep::Start, Vec::new()).map(|_| ())
}
