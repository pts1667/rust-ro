use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum TicketHelper2Step {
    Start,
    OnInit,
}

fn ticket_helper_2_run(ctx: &Ctx, mut step: TicketHelper2Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_m = Val::from(0);
    let mut l_m1 = Val::from(0);
    let mut l_menu_s = Val::from("");
    let mut l_string_s: Vec<Val> = Vec::new();
    'machine: loop {
        match step {
            TicketHelper2Step::Start => {
                if !(ctx.call(Function::CheckWeight, vec![Val::from("Spawn"), Val::from(200)])?.is_true()) {
                    ctx.lines_as(
                        "Ticket Helper",
                        args![
                            "Welcome to the",
                            "Monster Race Arena.",
                            "If you'd like to participate",
                            "in the ^3131FFDouble Monster Race^000000,",
                            "then please select 1 out of",
                            "the 6 monsters from the list."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ticket Helper",
                        args![
                            "Wait, wait...",
                            "I can't give you",
                            "anything right now.",
                            "You're carrying way",
                            "too many things..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if (!(ctx.var("monster_race_2_1").get()?.is_true()) && !(ctx.var("monster_race_2_2").get()?.is_true())) {
                    ctx.lines_as(
                        "Ticket Helper",
                        args![
                            "Hello there!",
                            "Interested in wagering on",
                            "the Dual Monster Race?",
                            "I'm here to help you if you've",
                            "got any questions, or if you",
                            "want to place your wager."
                        ],
                    )?;
                    ctx.next()?;
                    'b1: {
                        let subject1 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Check Monster Status:Wager on Race:Monster Race?:Cancel")],
                        )?);
                        let mut matched1 = false;
                        let no_case1 = !subject1.loosely_equals(&Val::from(1))
                            && !subject1.loosely_equals(&Val::from(2))
                            && !subject1.loosely_equals(&Val::from(3))
                            && !subject1.loosely_equals(&Val::from(4));
                        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                            matched1 = true;
                        }
                        if matched1 {
                            l_i = Val::from(1);
                            'l2: loop {
                                if !(l_i.clone().number()? <= 6) {
                                    break 'l2;
                                }
                                'b2: {
                                    ctx.lines(args![
                                        ((((((Val::from("Monster ") + l_i.clone()) + Val::from(" [^CC6600Luck^000000: "))
                                            + runtime::getd(
                                                ctx,
                                                &(Val::from("$@mon_r02_Luk") + l_i.clone()),
                                                &[
                                                    (".@i", runtime::Local::Scalar(&l_i)),
                                                    (".@m", runtime::Local::Scalar(&l_m)),
                                                    (".@m1", runtime::Local::Scalar(&l_m1)),
                                                    (".@menu$", runtime::Local::Scalar(&l_menu_s)),
                                                    (".@string$", runtime::Local::Array(&l_string_s))
                                                ]
                                            )?)
                                            + Val::from("] [^EE0000HP^000000: "))
                                            + runtime::getd(
                                                ctx,
                                                &(Val::from("$@mon_r02_tire") + l_i.clone()),
                                                &[
                                                    (".@i", runtime::Local::Scalar(&l_i)),
                                                    (".@m", runtime::Local::Scalar(&l_m)),
                                                    (".@m1", runtime::Local::Scalar(&l_m1)),
                                                    (".@menu$", runtime::Local::Scalar(&l_menu_s)),
                                                    (".@string$", runtime::Local::Array(&l_string_s))
                                                ]
                                            )?)
                                            + Val::from("]"))
                                    ])?;
                                }
                                l_i = (l_i.clone() + Val::from(1));
                            }
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.lines_as(
                                "Ticket Helper",
                                args![
                                    "Alright, please choose which",
                                    "two monsters that you think",
                                    "will win 1st and 2nd place.",
                                    "If both your monsters come",
                                    "in 1st and 2nd, in any order,",
                                    "you'll win the wager."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ticket Helper",
                                args![
                                    "Now, please tell me",
                                    "your first choice for one",
                                    "of the monsters that",
                                    "will win this race."
                                ],
                            )?;
                            ctx.next()?;
                            l_i = Val::from(1);
                            'l3: loop {
                                if !(l_i.clone().number()? <= 6) {
                                    break 'l3;
                                }
                                'b3: {
                                    l_menu_s = (((l_menu_s.clone() + Val::from("Monster ")) + l_i.clone()) + Val::from(":"));
                                }
                                l_i = (l_i.clone() + Val::from(1));
                            }
                            'l4: loop {
                                if !(true) {
                                    break 'l4;
                                }
                                'b4: {
                                    if l_m1.clone().number()? > 0 {
                                        l_menu_s = runtime::replacestr(
                                            &l_menu_s.clone(),
                                            &(Val::from("Monster ") + l_m1.clone()),
                                            &Val::from(""),
                                            &[],
                                        )?;
                                    }
                                    l_m = Val::from(runtime::select_values(ctx, &[l_menu_s.clone()])?);
                                    let subject5 = l_m.clone();
                                    if subject5 == 1 {
                                        let base = Val::from(0).number()?;
                                        runtime::local_set(&mut l_string_s, &Val::from(base + 0), Val::from("a friendly"), true);
                                        runtime::local_set(&mut l_string_s, &Val::from(base + 1), Val::from("Poring type monster"), true);
                                    } else if subject5 == 2 {
                                        let base = Val::from(0).number()?;
                                        runtime::local_set(&mut l_string_s, &Val::from(base + 0), Val::from("an adorable"), true);
                                        runtime::local_set(&mut l_string_s, &Val::from(base + 1), Val::from("Lunatic type monster"), true);
                                    } else if subject5 == 3 {
                                        let base = Val::from(0).number()?;
                                        runtime::local_set(&mut l_string_s, &Val::from(base + 0), Val::from("a darling"), true);
                                        runtime::local_set(&mut l_string_s, &Val::from(base + 1), Val::from("Savage Babe monster"), true);
                                    } else if subject5 == 4 {
                                        let base = Val::from(0).number()?;
                                        runtime::local_set(&mut l_string_s, &Val::from(base + 0), Val::from("a gentle baby"), true);
                                        runtime::local_set(&mut l_string_s, &Val::from(base + 1), Val::from("Desert Wolf monster"), true);
                                    } else if subject5 == 5 {
                                        let base = Val::from(0).number()?;
                                        runtime::local_set(&mut l_string_s, &Val::from(base + 0), Val::from("a small, yet"), true);
                                        runtime::local_set(&mut l_string_s, &Val::from(base + 1), Val::from("demonic, Deviruchi"), true);
                                    } else if subject5 == 6 {
                                        let base = Val::from(0).number()?;
                                        runtime::local_set(&mut l_string_s, &Val::from(base + 0), Val::from("a naughty"), true);
                                        runtime::local_set(&mut l_string_s, &Val::from(base + 1), Val::from("Baphomet Jr. monster"), true);
                                    }
                                    ctx.lines_as(
                                        "Ticket Helper",
                                        args![
                                            "You've chosen",
                                            (((Val::from("^0000FFMonster ") + l_m.clone()) + Val::from("^000000, "))
                                                + runtime::local_get(&l_string_s, &Val::from(0), true)),
                                            (runtime::local_get(&l_string_s, &Val::from(1), true) + Val::from(".")),
                                            "Are you sure you want",
                                            "to choose this monster?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 2 {
                                        ctx.lines_as(
                                            "Ticket Helper",
                                            args![
                                                "You have canceled",
                                                "your wager. Okay,",
                                                "I understand. Perhaps",
                                                "you'd feel more comfortable",
                                                "checking the monsters first?"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    if l_m1.clone() == 0 {
                                        ctx.lines_as(
                                            "Ticket Helper",
                                            args![
                                                "Now, please make",
                                                "your second choice",
                                                "for the monster that",
                                                "you think will place",
                                                "1st or 2nd in this race."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        l_m1 = l_m.clone();
                                    } else {
                                        if !(ctx.var("$@mon_time_2_2").get()?.is_true()) {
                                            ctx.lines_as(
                                                "Ticket Helper",
                                                args![
                                                    "You've wagered on",
                                                    ((((Val::from("^0000FFMonster ") + l_m1.clone())
                                                        + Val::from("^000000 and ^0000FFMonster "))
                                                        + l_m.clone())
                                                        + Val::from("^000000")),
                                                    "to win this race. Good luck!",
                                                    "I really hope that the odds",
                                                    "work out in your favor~"
                                                ],
                                            )?;
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_CHUP")?])?;
                                            ctx.var("monster_race_2_1").set(l_m1.clone())?;
                                            ctx.var("monster_race_2_2").set(l_m.clone())?;
                                            ctx.call(Function::GetItem, vec![Val::from(7514), Val::from(1)])?;
                                        } else {
                                            ctx.lines_as(
                                                "Ticket Helper",
                                                args![
                                                    "I'm very sorry, but a",
                                                    "monster race is underway.",
                                                    "Please wait, and then place",
                                                    "your wager for the next race."
                                                ],
                                            )?;
                                        }
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.lines_as(
                                "Ticket Helper",
                                args![
                                    "Monster Races originated from",
                                    "simple children's games in which",
                                    "Cute Pets would race against each other. This grew into an adult",
                                    "pastime that is so popular, we've built a racing arena in Hugel."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ticket Helper",
                                args![
                                    "Our Monster Race Arena hosts",
                                    "two types of monster races. First, we have the Single Monster Race,",
                                    "in which those that wagered on the 1st place monster are rewarded."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Eclar Ellbird",
                                args![
                                    "Then, we have the Dual Monster",
                                    "Race in which those that wagered on the 1st and 2nd place monsters",
                                    "are equally rewarded. The house",
                                    "odds and wager rewards are greater in Dual Races than Single Races."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ticket Helper",
                                args![
                                    "Although a small entrance",
                                    "fee is required, we only use",
                                    "the money to give rewards to",
                                    "participants and maintain this",
                                    "arena. Therefore, we're not",
                                    "profiting from this enterprise."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ticket Helper",
                                args![
                                    "Also, we prohibit others",
                                    "from making personal bets",
                                    "and wagers, using items and",
                                    "zeny, based on the outcomes",
                                    "of these races. That kind of",
                                    "gambling is illegal here..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ticket Helper",
                                args![
                                    "Once you enter the Race Arena, you will receive a Racing Ticket.",
                                    "Keep in mind that winning Racing Tickets can only be exchanged for",
                                    "Prize Medals during a 5 minute window after the end of the race."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ticket Helper",
                                args![
                                    "You're already here",
                                    "inside the Monster Race",
                                    "Arena, so you may as well",
                                    "try placing a wager. It's",
                                    "more fun than you'd think~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(4)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.lines_as(
                                "Ticket Helper",
                                args![
                                    "You have canceled",
                                    "your wager. Okay,",
                                    "I understand. Perhaps",
                                    "you'd feel more comfortable",
                                    "checking the monsters first?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                } else {
                    if ctx.call(Function::CountItem, vec![Val::from(7514)])?.is_true() {
                        ctx.lines_as(
                            "Ticket Helper",
                            args![
                                "You've wagered on",
                                ((((Val::from("^0000FFMonster ") + ctx.var("monster_race_2_1").get()?)
                                    + Val::from("^000000 and ^0000FFMonster "))
                                    + ctx.var("monster_race_2_2").get()?)
                                    + Val::from("^000000")),
                                "for this Dual Monster Race."
                            ],
                        )?;
                        ctx.next()?;
                    }
                    ctx.lines_as(
                        "Ticket Helper",
                        args![
                            "The start of the race will be",
                            "announced through a broadcast.",
                            "You can refer to your Mini-Map",
                            "to track the monsters' race",
                            "positions. Thank you, and",
                            "have a good time!"
                        ],
                    )?;
                    ctx.call(
                        Function::ViewPoint,
                        vec![Val::from(1), Val::from(43), Val::from(35), Val::from(0), Val::from(16711680)],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = TicketHelper2Step::OnInit;
                continue 'machine;
            }
            TicketHelper2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Ticket Helper#2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ticket_helper_2(ctx: &Ctx) -> Script {
    ticket_helper_2_run(ctx, TicketHelper2Step::Start, Vec::new()).map(|_| ())
}

pub fn ticket_helper_2_oninit(ctx: &Ctx) -> Script {
    ticket_helper_2_run(ctx, TicketHelper2Step::OnInit, Vec::new()).map(|_| ())
}

fn game_guide_double_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Game Guide",
        args!["Welcome to the", "Monster Race Arena.", "How can I help you?"],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Monster Race Info:Wager Info:Ticket Redemption Info")])? {
        1 => {
            ctx.lines_as(
                "Game Guide",
                args![
                    "Monster Races originated from",
                    "simple children's games in which",
                    "Cute Pets would race against each other. This grew into an adult",
                    "pastime that is so popular, we've built a racing arena in Hugel."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Game Guide",
                args![
                    "Our Monster Race Arena hosts",
                    "two types of monster races. First, we have the Single Monster Race,",
                    "in which those that wagered on the 1st place monster are rewarded."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Game Guide",
                args![
                    "Then, we have the Dual Monster",
                    "Race in which the house odds and rewards are greater than in Single",
                    "Monster Races: you must wager on 2 monsters, and they must place in",
                    "1st and 2nd for you to win."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Game Guide",
                args![
                    "Although a small entrance",
                    "fee is required, we only use",
                    "the money to give rewards to",
                    "participants and maintain this",
                    "arena. Therefore, we're not",
                    "profiting from this enterprise."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Game Guide",
                args![
                    "Also, we prohibit others",
                    "from making personal bets",
                    "and wagers, using items and",
                    "zeny, based on the outcomes",
                    "of these races. That kind of",
                    "gambling is illegal here..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Game Guide",
                args![
                    "Once you enter the Race Arena, you will receive a Racing Ticket.",
                    "Keep in mind that winning Racing Tickets can only be exchanged for",
                    "Prize Medals during a 5 minute window after the end of the race."
                ],
            )?;
            ctx.next()?;
        }
        2 => {
            ctx.lines_as(
                "Game Guide",
                args![
                    "Before placing a wager, you",
                    "must get a free Racing Ticket",
                    "from the Ticket Helper. There,",
                    "I've marked the Ticket Helper",
                    "on your Mini-Map, so you can",
                    "find him pretty easily."
                ],
            )?;
            ctx.call(
                Function::ViewPoint,
                vec![Val::from(1), Val::from(73), Val::from(22), Val::from(1), Val::from(16724821)],
            )?;
            ctx.next()?;
        }
        3 => {
            ctx.lines_as(
                "Game Guide",
                args![
                    "If you wagered on the winner",
                    "of a Single Monster Race, or",
                    "on the 1st or 2nd place winners",
                    "in a Dual Monster Race, then",
                    "you can exchange your Racing",
                    "Ticket for Prize Medals."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Game Guide",
                args![
                    "However, you must exchange",
                    "your Racing Ticket with the",
                    "Medal Distributor within the",
                    "5 minute window after the end",
                    "of the race. ^FF0000Your ticket becomes^FFFFFF ^FF0000 void after these 5 minutes.^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Game Guide",
                args![
                    "When this 5 minute window",
                    "elapses, you will be teleported outside, and we will immediately",
                    "begin preparing for the next race. Make sure that you remember this",
                    "information when you wager."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Game Guide",
                args![
                    "If you haven't received",
                    "your free Racing Ticket,",
                    "then please visit the Ticket",
                    "Helper. There, I've just marked",
                    "his location on your Mini-Map."
                ],
            )?;
            ctx.call(
                Function::ViewPoint,
                vec![Val::from(1), Val::from(67), Val::from(45), Val::from(2), Val::from(13525760)],
            )?;
            ctx.next()?;
        }
        _ => {}
    }
    ctx.lines_as(
        "Game Guide",
        args![
            "Thank you, and",
            "I hope you enjoy",
            "your time here in the",
            "Monster Racing Arena."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn game_guide_double(ctx: &Ctx) -> Script {
    game_guide_double_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MedalDistributorMedalStep {
    Start,
    OnInit,
}

fn medal_distributor_medal_run(ctx: &Ctx, mut step: MedalDistributorMedalStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MedalDistributorMedalStep::Start => {
                if !(ctx
                    .call(Function::CheckWeight, vec![Val::from("Jellopy"), Val::from(20)])?
                    .is_true())
                {
                    ctx.lines_as(
                        "Medal Distributor",
                        args![
                            "I'm sorry, but I can't",
                            "reward you with any medals",
                            "until you make more space",
                            "available in your Inventory."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Medal Distributor",
                    args![
                        "Hello there~",
                        "If you've wagered on the",
                        "winning monster in a recent",
                        "race, then you can exchange",
                        "your game ticket here for",
                        "some Prize Medals."
                    ],
                )?;
                ctx.next()?;
                if !(ctx.call(Function::CountItem, vec![Val::from(7514)])?.is_true()) {
                    ctx.lines_as(
                        "Medal Distributor",
                        args![
                            "Please remember that you can",
                            "only exchange winning Game",
                            "Tickets for Prize Medals right",
                            "after the race finishes. Prize",
                            "Medals may be given to Wayne",
                            "in Hugel in exchange for items."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Medal Distributor",
                    args![
                        "Please remember that you can",
                        "only exchange winning Game",
                        "Tickets for Prize Medals right",
                        "after the race finishes. Prize",
                        "Medals may be given to Wayne",
                        "in Hugel in exchange for items."
                    ],
                )?;
                ctx.next()?;
                if ((ctx.var("monster_race_2_1").get()?.loosely_equals(&ctx.var("$@mon_race_2_1").get()?)
                    && ctx.var("monster_race_2_2").get()?.loosely_equals(&ctx.var("$@mon_race_2_2").get()?))
                    || (ctx.var("monster_race_2_1").get()?.loosely_equals(&ctx.var("$@mon_race_2_2").get()?)
                        && ctx.var("monster_race_2_2").get()?.loosely_equals(&ctx.var("$@mon_race_2_1").get()?)))
                {
                    ctx.lines_as(
                        "Medal Distributor",
                        args![
                            "Congratulations! It's really",
                            "difficult to guess the winners",
                            "of a Dual Monster Race, so you",
                            "must be really lucky! Would you",
                            "like to exchange your winning",
                            "Racing Ticket for Prize Medals?"
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Yes, please.:No, thanks.")])?) == 1 {
                        ctx.lines_as(
                            "Medal Distributor",
                            args![
                                "Okay, everything looks good,",
                                "so here's your Prize Medals~",
                                "If you want to trade these",
                                "medals for items, please",
                                "visit Wayne in Hugel. We hope",
                                "you enjoyed the Monster Race~"
                            ],
                        )?;
                        ctx.var("monster_race_2_2").set(Val::from(7))?;
                        ctx.var("monster_race_2_1").set(ctx.var("monster_race_2_2").get()?)?;
                        ctx.call(Function::DelItem, vec![Val::from(7514), Val::from(1)])?;
                        ctx.call(Function::GetItem, vec![Val::from(7515), Val::from(15)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Medal Distributor",
                        args![
                            "Um, are you sure? You ",
                            "can only exchange a winning",
                            "Racing Ticket for Prize Medals",
                            "for a short time after the race. If you made a mistake, you",
                            "should ask me again quickly."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (!(ctx.var("monster_race_2_1").get()?.is_true()) && !(ctx.var("monster_race_2_2").get()?.is_true())) {
                    ctx.lines_as(
                        "Medal Distributor",
                        args![
                            "Well, better luck next time...",
                            "Although you can't always",
                            "be lucky, it's always fun to",
                            "wager on the monster races!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("monster_race_2_1").get()? == 7 && ctx.var("monster_race_2_2").get()? == 7) {
                    ctx.lines_as(
                        "Medal Distributor",
                        args![
                            "Thanks for visiting the",
                            "Monster Race Arena, and",
                            "I hope you enjoy your time",
                            "here. I'll see you next time~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (!ctx.var("monster_race_2_1").get()?.loosely_equals(&ctx.var("$@mon_race_2_1").get()?)
                    || !ctx.var("monster_race_2_2").get()?.loosely_equals(&ctx.var("$@mon_race_2_2").get()?))
                {
                    ctx.lines_as(
                        "Medal Distributor",
                        args![
                            "You may not have wagered",
                            "on the winning monster in",
                            "the last race, but I hope that",
                            "you get lucky next time~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = MedalDistributorMedalStep::OnInit;
                continue 'machine;
            }
            MedalDistributorMedalStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Medal Distributor#medal")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn medal_distributor_medal(ctx: &Ctx) -> Script {
    medal_distributor_medal_run(ctx, MedalDistributorMedalStep::Start, Vec::new()).map(|_| ())
}

pub fn medal_distributor_medal_oninit(ctx: &Ctx) -> Script {
    medal_distributor_medal_run(ctx, MedalDistributorMedalStep::OnInit, Vec::new()).map(|_| ())
}

fn exit_guide_double_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Exit Guide",
        args![
            "If you have a winning Racing",
            "Ticket, please make sure that",
            "you redeem it for Prize Medals",
            "now. All Racing Tickets become",
            "void once the next race begins."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Exit Guide",
        args![
            "If you wish to leave",
            "the arena, then I can guide",
            "you outside. Would you like",
            "to leave the arena right now?"
        ],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 2 {
        ctx.lines_as(
            "Exit Guide",
            args!["Alright, then.", "Just let me know", "whenever you're", "ready to leave."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !(ctx.call(Function::CountItem, vec![Val::from(7514)])?.is_true()) {
        ctx.lines_as(
            "Exit Guide",
            args![
                "Thank you for",
                "your patronage, and",
                "I hope that you come",
                "visit us again soon~"
            ],
        )?;
    } else {
        ctx.lines_as(
            "Exit Guide",
            args![
                "In accordance with our",
                "policies, I must take your",
                "Racing Ticket before you leave.",
                "Thank you for your patronage,",
                "and I hope you enjoy your time",
                "here in the Monster Race Arena."
            ],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(7514), Val::from(1)])?;
    }
    ctx.var("monster_race_2_2").set(Val::from(0))?;
    ctx.var("monster_race_2_1").set(ctx.var("monster_race_2_2").get()?)?;
    ctx.close_window()?;
    ctx.call(Function::Warp, vec![Val::from("hugel"), Val::from(63), Val::from(73)])?;
    return Err(Stop::End);
}

pub fn exit_guide_double(ctx: &Ctx) -> Script {
    exit_guide_double_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Mob1MainStep {
    Start,
    MN,
    AfterMN,
    OnEnable,
    OnTouchNPC,
    OnMyMobDead,
    OnDisable,
    OnInit,
}

fn mob1_main_run(ctx: &Ctx, mut step: Mob1MainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_m_s = Val::from("");
    let mut l_n: Vec<Val> = Vec::new();
    let mut l_n_s: Vec<Val> = Vec::new();
    let mut l_x = Val::from(0);
    let mut l_y = Val::from(0);
    'machine: loop {
        match step {
            Mob1MainStep::Start => {
                step = Mob1MainStep::AfterMN;
                continue 'machine;
            }
            Mob1MainStep::MN => {
                let base = Val::from(1).number()?;
                runtime::local_set(&mut l_n_s, &Val::from(base + 0), Val::from("poring"), true);
                runtime::local_set(&mut l_n_s, &Val::from(base + 1), Val::from("lunatic"), true);
                runtime::local_set(&mut l_n_s, &Val::from(base + 2), Val::from("savagebebe"), true);
                runtime::local_set(&mut l_n_s, &Val::from(base + 3), Val::from("desertwolf"), true);
                runtime::local_set(&mut l_n_s, &Val::from(base + 4), Val::from("deviruchi"), true);
                runtime::local_set(&mut l_n_s, &Val::from(base + 5), Val::from("baphomet"), true);
                l_i = Val::from(1);
                'l1: loop {
                    if !(l_i.clone().number()? <= 6) {
                        break 'l1;
                    }
                    'b1: {
                        if runtime::compare(
                            &ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?,
                            &runtime::local_get(&l_n_s, &l_i.clone(), true),
                        )
                        .is_true()
                        {
                            break 'l1;
                        }
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                return Ok(l_i.clone());
                return Ok(Val::from(0));
            }
            Mob1MainStep::AfterMN => {
                step = Mob1MainStep::OnEnable;
                continue 'machine;
            }
            Mob1MainStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![])?;
                let base = Val::from(1).number()?;
                runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(1725), false);
                runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(1726), false);
                runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(1727), false);
                runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(1728), false);
                runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(1730), false);
                runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(1729), false);
                let position = ctx
                    .call(Function::GetMapXy, vec![ctx.constant("BL_NPC")?])?
                    .into_array()
                    .ok_or_else(|| Stop::Error("Invalid position".into()))?;
                l_m_s = position[0].clone();
                l_x = position[1].clone();
                l_y = position[2].clone();
                l_i = ctx.var("mn").get()?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("p_track02"),
                        Val::from(58),
                        l_y.clone(),
                        (Val::from("Monster ") + l_i.clone()),
                        runtime::local_get(&l_n, &l_i.clone(), false),
                        Val::from(1),
                        (ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                return Err(Stop::End);
            }
            Mob1MainStep::OnTouchNPC => {
                l_i = ctx.var("mn").get()?;
                if !(ctx.var("$@mon_race_2_1").get()?.is_true()) {
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_MVP")?])?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("p_track02"),
                            ((Val::from("Monster ") + l_i.clone()) + Val::from(" has reached the Finish Line!")),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x66FFCC"),
                        ],
                    )?;
                    ctx.var("$@mon_race_2_1").set(l_i.clone())?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("::OnDisable"))],
                    )?;
                } else {
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_MVP")?])?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("p_track02"),
                            ((Val::from("The race is over! Monster ") + l_i.clone()) + Val::from(" has reached the Finish Line!")),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x66FFCC"),
                        ],
                    )?;
                    ctx.var("$@mon_race_2_2").set(l_i.clone())?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#poring1::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#lunatic1::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#savagebebe1::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#desertwolf1::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#deviruchi1::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#baphomet1::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("TrapGlobal#race02::OnDisable")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Medal Distributor#medal")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#race_timer2-3::OnEnable")])?;
                }
                step = Mob1MainStep::OnMyMobDead;
                continue 'machine;
            }
            Mob1MainStep::OnMyMobDead => {
                return Err(Stop::End);
            }
            Mob1MainStep::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![
                        Val::from("p_track02"),
                        (ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                step = Mob1MainStep::OnInit;
                continue 'machine;
            }
            Mob1MainStep::OnInit => {
                if ctx.call(Function::StrNpcInfo, vec![Val::from(2)])? != "main" {
                    ctx.call(Function::DisableNpc, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob1_main(ctx: &Ctx) -> Script {
    mob1_main_run(ctx, Mob1MainStep::Start, Vec::new()).map(|_| ())
}

pub fn mob1_main_onenable(ctx: &Ctx) -> Script {
    mob1_main_run(ctx, Mob1MainStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn mob1_main_ontouchnpc(ctx: &Ctx) -> Script {
    mob1_main_run(ctx, Mob1MainStep::OnTouchNPC, Vec::new()).map(|_| ())
}

pub fn mob1_main_onmymobdead(ctx: &Ctx) -> Script {
    mob1_main_run(ctx, Mob1MainStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn mob1_main_ondisable(ctx: &Ctx) -> Script {
    mob1_main_run(ctx, Mob1MainStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn mob1_main_oninit(ctx: &Ctx) -> Script {
    mob1_main_run(ctx, Mob1MainStep::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TrapglobalRace02Step {
    Start,
    OnEnable,
    OnDisable,
    OnInit,
}

fn trapglobal_race02_run(ctx: &Ctx, mut step: TrapglobalRace02Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_c = Val::from(0);
    let mut l_i = Val::from(0);
    let mut l_line = Val::from(0);
    let mut l_tired = Val::from(0);
    'machine: loop {
        match step {
            TrapglobalRace02Step::Start => {
                step = TrapglobalRace02Step::OnEnable;
                continue 'machine;
            }
            TrapglobalRace02Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("TrapGlobal#race02")])?;
                l_c = Val::from(1);
                'l1: loop {
                    if !(l_c.clone().number()? <= 6) {
                        break 'l1;
                    }
                    'b1: {
                        l_line = ctx.call(Function::Rand, vec![Val::from(1), Val::from(70)])?;
                        ctx.call(Function::EnableNpc, vec![(Val::from("starting#race02_") + l_c.clone())])?;
                        ctx.call(
                            Function::EnableNpc,
                            vec![((Val::from("Tire#race02_") + l_c.clone()) + Val::from("_1"))],
                        )?;
                        if l_line.clone().number()? <= 10 {
                            l_tired = ctx.call(Function::Rand, vec![Val::from(50), Val::from(60)])?;
                            ctx.call(
                                Function::EnableNpc,
                                vec![((Val::from("Luk#race02_") + l_c.clone()) + Val::from("_5"))],
                            )?;
                            ctx.call(
                                Function::EnableNpc,
                                vec![((Val::from("Luk#race02_") + l_c.clone()) + Val::from("_6"))],
                            )?;
                        } else {
                            if l_line.clone().number()? <= 30 {
                                l_tired = ctx.call(Function::Rand, vec![Val::from(40), Val::from(60)])?;
                                ctx.call(
                                    Function::EnableNpc,
                                    vec![((Val::from("Luk#race02_") + l_c.clone()) + Val::from("_5"))],
                                )?;
                                if l_tired.clone().number()? < 50 {
                                    ctx.call(
                                        Function::EnableNpc,
                                        vec![((Val::from("Tire#race02_") + l_c.clone()) + Val::from("_2"))],
                                    )?;
                                }
                            } else if l_line.clone().number()? <= 40 {
                                l_tired = ctx.call(Function::Rand, vec![Val::from(30), Val::from(50)])?;
                                ctx.call(
                                    Function::EnableNpc,
                                    vec![((Val::from("Luk#race02_") + l_c.clone()) + Val::from("_1"))],
                                )?;
                                ctx.call(
                                    Function::EnableNpc,
                                    vec![((Val::from("Tire#race02_") + l_c.clone()) + Val::from("_2"))],
                                )?;
                                if l_tired.clone().number()? < 40 {
                                    ctx.call(
                                        Function::EnableNpc,
                                        vec![((Val::from("Tire#race02_") + l_c.clone()) + Val::from("_3"))],
                                    )?;
                                }
                            } else if l_line.clone().number()? <= 50 {
                                l_tired = ctx.call(Function::Rand, vec![Val::from(20), Val::from(40)])?;
                                ctx.call(
                                    Function::EnableNpc,
                                    vec![((Val::from("Luk#race02_") + l_c.clone()) + Val::from("_1"))],
                                )?;
                                ctx.call(
                                    Function::EnableNpc,
                                    vec![((Val::from("Luk#race02_") + l_c.clone()) + Val::from("_2"))],
                                )?;
                                ctx.call(
                                    Function::EnableNpc,
                                    vec![((Val::from("Tire#race02_") + l_c.clone()) + Val::from("_2"))],
                                )?;
                                ctx.call(
                                    Function::EnableNpc,
                                    vec![((Val::from("Tire#race02_") + l_c.clone()) + Val::from("_3"))],
                                )?;
                                if l_tired.clone().number()? < 30 {
                                    ctx.call(
                                        Function::EnableNpc,
                                        vec![((Val::from("Tire#race02_") + l_c.clone()) + Val::from("_4"))],
                                    )?;
                                }
                            } else if l_line.clone().number()? <= 60 {
                                l_tired = ctx.call(Function::Rand, vec![Val::from(10), Val::from(30)])?;
                                l_i = Val::from(1);
                                'l2: loop {
                                    if !(l_i.clone().number()? <= 3) {
                                        break 'l2;
                                    }
                                    'b2: {
                                        ctx.call(
                                            Function::EnableNpc,
                                            vec![(((Val::from("Luk#race02_") + l_c.clone()) + Val::from("_")) + l_i.clone())],
                                        )?;
                                    }
                                    l_i = (l_i.clone() + Val::from(1));
                                }
                                l_i = Val::from(2);
                                'l3: loop {
                                    if !(l_i.clone().number()? <= 4) {
                                        break 'l3;
                                    }
                                    'b3: {
                                        ctx.call(
                                            Function::EnableNpc,
                                            vec![(((Val::from("Tire#race02_") + l_c.clone()) + Val::from("_")) + l_i.clone())],
                                        )?;
                                    }
                                    l_i = (l_i.clone() + Val::from(1));
                                }
                                if l_tired.clone().number()? < 20 {
                                    ctx.call(
                                        Function::EnableNpc,
                                        vec![((Val::from("Tire#race02_") + l_c.clone()) + Val::from("_5"))],
                                    )?;
                                }
                            } else if l_line.clone().number()? <= 70 {
                                l_tired = ctx.call(Function::Rand, vec![Val::from(0), Val::from(20)])?;
                                l_i = Val::from(1);
                                'l4: loop {
                                    if !(l_i.clone().number()? <= 4) {
                                        break 'l4;
                                    }
                                    'b4: {
                                        ctx.call(
                                            Function::EnableNpc,
                                            vec![(((Val::from("Luk#race02_") + l_c.clone()) + Val::from("_")) + l_i.clone())],
                                        )?;
                                    }
                                    l_i = (l_i.clone() + Val::from(1));
                                }
                                l_i = Val::from(2);
                                'l5: loop {
                                    if !(l_i.clone().number()? <= 5) {
                                        break 'l5;
                                    }
                                    'b5: {
                                        ctx.call(
                                            Function::EnableNpc,
                                            vec![(((Val::from("Tire#race02_") + l_c.clone()) + Val::from("_")) + l_i.clone())],
                                        )?;
                                    }
                                    l_i = (l_i.clone() + Val::from(1));
                                }
                                if l_tired.clone().number()? < 10 {
                                    ctx.call(
                                        Function::EnableNpc,
                                        vec![((Val::from("Tire#race02_") + l_c.clone()) + Val::from("_6"))],
                                    )?;
                                }
                            }
                        }
                        runtime::setd(
                            ctx,
                            &(Val::from("$@mon_r02_luk") + l_c.clone()),
                            l_line.clone(),
                            &mut [
                                (".@c", runtime::LocalMut::Scalar(&mut l_c)),
                                (".@i", runtime::LocalMut::Scalar(&mut l_i)),
                                (".@line", runtime::LocalMut::Scalar(&mut l_line)),
                                (".@tired", runtime::LocalMut::Scalar(&mut l_tired)),
                            ],
                        )?;
                        runtime::setd(
                            ctx,
                            &(Val::from("$@mon_r02_tire") + l_c.clone()),
                            l_tired.clone(),
                            &mut [
                                (".@c", runtime::LocalMut::Scalar(&mut l_c)),
                                (".@i", runtime::LocalMut::Scalar(&mut l_i)),
                                (".@line", runtime::LocalMut::Scalar(&mut l_line)),
                                (".@tired", runtime::LocalMut::Scalar(&mut l_tired)),
                            ],
                        )?;
                    }
                    l_c = (l_c.clone() + Val::from(1));
                }
                return Err(Stop::End);
            }
            TrapglobalRace02Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("TrapGlobal#race02")])?;
                l_i = Val::from(1);
                'l6: loop {
                    if !(l_i.clone().number()? <= 6) {
                        break 'l6;
                    }
                    'b6: {
                        ctx.call(
                            Function::DoNpcEvent,
                            vec![((Val::from("starting#race02_") + l_i.clone()) + Val::from("::OnDisable"))],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                return Err(Stop::End);
            }
            TrapglobalRace02Step::OnInit => {
                l_i = Val::from(1);
                'l7: loop {
                    if !(l_i.clone().number()? <= 6) {
                        break 'l7;
                    }
                    'b7: {
                        runtime::setd(
                            ctx,
                            &(Val::from("$@mon_r02_luk") + l_i.clone()),
                            Val::from(0),
                            &mut [
                                (".@c", runtime::LocalMut::Scalar(&mut l_c)),
                                (".@i", runtime::LocalMut::Scalar(&mut l_i)),
                                (".@line", runtime::LocalMut::Scalar(&mut l_line)),
                                (".@tired", runtime::LocalMut::Scalar(&mut l_tired)),
                            ],
                        )?;
                        runtime::setd(
                            ctx,
                            &(Val::from("$@mon_r02_tire") + l_i.clone()),
                            Val::from(0),
                            &mut [
                                (".@c", runtime::LocalMut::Scalar(&mut l_c)),
                                (".@i", runtime::LocalMut::Scalar(&mut l_i)),
                                (".@line", runtime::LocalMut::Scalar(&mut l_line)),
                                (".@tired", runtime::LocalMut::Scalar(&mut l_tired)),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn trapglobal_race02(ctx: &Ctx) -> Script {
    trapglobal_race02_run(ctx, TrapglobalRace02Step::Start, Vec::new()).map(|_| ())
}

pub fn trapglobal_race02_onenable(ctx: &Ctx) -> Script {
    trapglobal_race02_run(ctx, TrapglobalRace02Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn trapglobal_race02_ondisable(ctx: &Ctx) -> Script {
    trapglobal_race02_run(ctx, TrapglobalRace02Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn trapglobal_race02_oninit(ctx: &Ctx) -> Script {
    trapglobal_race02_run(ctx, TrapglobalRace02Step::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Starting2Step {
    Start,
    OnTouchNPC,
    OnDisable,
    OnInit,
}

fn starting_2_run(ctx: &Ctx, mut step: Starting2Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_speed = Val::from(0);
    let mut l_start = Val::from(0);
    'machine: loop {
        match step {
            Starting2Step::Start => {
                step = Starting2Step::OnTouchNPC;
                continue 'machine;
            }
            Starting2Step::OnTouchNPC => {
                l_start = ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?;
                if l_start.clone().number()? < 11 {
                    l_speed = Val::from(60);
                } else {
                    if l_start.clone().number()? < 21 {
                        l_speed = Val::from(70);
                    } else {
                        if l_start.clone().number()? < 31 {
                            l_speed = Val::from(80);
                        } else {
                            if l_start.clone().number()? < 41 {
                                l_speed = Val::from(90);
                            } else {
                                if l_start.clone().number()? < 51 {
                                    l_speed = Val::from(100);
                                } else if l_start.clone().number()? < 61 {
                                    l_speed = Val::from(110);
                                } else if l_start.clone().number()? < 71 {
                                    l_speed = Val::from(120);
                                } else if l_start.clone().number()? < 81 {
                                    l_speed = Val::from(130);
                                } else if l_start.clone().number()? < 91 {
                                    l_speed = Val::from(140);
                                } else {
                                    l_speed = Val::from(150);
                                }
                            }
                        }
                    }
                }
                ctx.call(
                    Function::StartStatus,
                    vec![ctx.constant("SC_WALKSPEED")?, Val::from(5000), l_speed.clone()],
                )?;
                return Err(Stop::End);
            }
            Starting2Step::OnDisable => {
                l_i = Val::from(1);
                'l1: loop {
                    if !(l_i.clone().number()? < 7) {
                        break 'l1;
                    }
                    'b1: {
                        ctx.call(
                            Function::DisableNpc,
                            vec![
                                (((Val::from("Luk#") + ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?) + Val::from("_"))
                                    + l_i.clone()),
                            ],
                        )?;
                        ctx.call(
                            Function::DisableNpc,
                            vec![
                                (((Val::from("Tire#") + ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?) + Val::from("_"))
                                    + l_i.clone()),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                step = Starting2Step::OnInit;
                continue 'machine;
            }
            Starting2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn starting_2(ctx: &Ctx) -> Script {
    starting_2_run(ctx, Starting2Step::Start, Vec::new()).map(|_| ())
}

pub fn starting_2_ontouchnpc(ctx: &Ctx) -> Script {
    starting_2_run(ctx, Starting2Step::OnTouchNPC, Vec::new()).map(|_| ())
}

pub fn starting_2_ondisable(ctx: &Ctx) -> Script {
    starting_2_run(ctx, Starting2Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn starting_2_oninit(ctx: &Ctx) -> Script {
    starting_2_run(ctx, Starting2Step::OnInit, Vec::new()).map(|_| ())
}
