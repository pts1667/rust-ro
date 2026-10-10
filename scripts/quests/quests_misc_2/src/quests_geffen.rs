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

fn blacksmith_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Blacksmith", args!["You've never heard of me? Why, I'm the Veteran Blacksmith of this land. I've spent 30 years in this hot and bloody hellfire, bending steel to my iron will!"])?;
    ctx.next()?;
    ctx.lines_as("Blacksmith", args!["But...", "I am growing older and my prowess begins to fade with age. So I've invented something for young, hot, upcoming Blacksmiths, in hopes that they too will master my craft..."])?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("What is it?:...")])?);
        let mut matched1 = false;
        if !matched1 && subject1 == 1 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Blacksmith", args!["We Blacksmiths must always manipulate steel under extreme heat, and the hot air and melted metals are more than most people can stand."])?;
            ctx.next()?;
            ctx.lines_as(
                "Blacksmith",
                args!["For the sake of our craft, our beautiful, yet masculine faces are put at risk..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Blacksmith",
                args!["But with this wonder of technology, there's no need to worry! Behold, the ^3355FFWelding Mask^000000 !!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Blacksmith", args!["It works by covering your face with a fat metal plate. Truly, it's an item that everyone should want! If not yourself, it'd be a good idea to give this to a friend who is studying smithing."])?;
            ctx.next()?;
            ctx.lines_as("Blacksmith", args!["So how about it ? If you give me ^2244FF50 Steel^000000 and ^4422FFonly 2000 Zeny^000000, I will make it for you right away~!!"])?;
            ctx.next()?;
            match ctx.menu(&["Hm... Not bad. Alright.", "How does it work again?"])? {
                0 => {
                    if ctx.items().count(999)? > 49 && ctx.player().zeny()? > 1999 {
                        ctx.lines_as(
                            "Blacksmith",
                            args!["Ah, I see that you appreciate a Blacksmith's work! Alright, I'll do it right away !!"],
                        )?;
                        ctx.next()?;
                        ctx.items().take(999, 50)?;
                        ctx.player().set_zeny(ctx.player().zeny()? - 2000)?;
                        ctx.items().give(2292, 1)?;
                        ctx.lines_as("Blacksmith", args!["Hahaha~! Make good use of that!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Blacksmith", args!["Hmmm...", "You do not have the items I require."])?;
                        ctx.next()?;
                        ctx.lines_as("Blacksmith", args!["A common Blacksmith would interpret this as a sign of disrepect, but I will forgive you. Just don't forget what I need if you want me to make you this mask."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                1 => {
                    ctx.lines_as("Blacksmith", args!["The ^3355FFWelding Mask^000000 protects your face by covering it with a fat metal plate. This invention is the result of 30 years of endless toil at the hammer and anvil."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched1 && subject1 == 2 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Blacksmith", args!["Ah...!", "I guess this is the first time you've met a Blacksmith as great as me, and you're just speechless with awe. Please, take your time and speak up when you are ready."])?;
            ctx.next()?;
            ctx.lines(args!["^3355FFThis guy seems", "a little full", "of himself.^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn blacksmith(ctx: &Ctx) -> Script {
    blacksmith_body(ctx, Vec::new()).map(|_| ())
}

fn eric_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Eric", args!["Please listen", "to my story", "of my blessed", "grandfather."])?;
    ctx.next()?;
    match ctx.menu(&["Talk", "Ask about Items needed", "Manufacture", "Quit"])? {
        0 => {
            ctx.lines_as("Eric", args!["My Grandfather, who passed away a few years ago, was special to me. He always stayed with me, and took care of me. He was much better than my old man, who was always busy working."])?;
            ctx.next()?;
            ctx.lines_as(
                "Eric",
                args!["To me, Grandfather's library was always like a fantasy land where I could escape. I spent most of my time there."],
            )?;
            ctx.next()?;
            ctx.lines_as("Eric", args!["I learned a lot of things from his books in the Library. Grandfather was interested in Alchemy and Music, and his Library was filled with many mysterious old books."])?;
            ctx.next()?;
            ctx.lines_as(
                "Eric",
                args![
                    "One day...",
                    "As usual, I went to the Library and discovered an interesting schematic."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Eric", args!["It showed some instrument which looked like Ear Muffs that, when worn on the head, could transmit music or whispers to the ears through a thin wire."])?;
            ctx.next()?;
            ctx.lines_as("Eric", args!["I guess...", "It is my grand father's unfinished work. I met a few Alchemists and asked them to produce it, but they rejected my offer..."])?;
            ctx.next()?;
            ctx.lines_as("Eric", args!["In the end, I thought 'I've gotta make it myself.' So I researched for a few years, and now I fully understand how to make that thing. But, I could never gather the materials to make it!"])?;
            ctx.next()?;
            ctx.lines_as("Eric", args!["If I can get those articles, I would build it right away. If you are interested in my idea, would you bring them for me?"])?;
            ctx.next()?;
            ctx.lines_as("Eric", args!["All I wanna do is build this thing in honor of my grandfather and finish the work he started. If you ask me to keep it, I wouldn't mind."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        1 => {
            ctx.lines_as(
                "Eric",
                args![
                    "40 ^3355FFSteel^000000",
                    "1 ^3355FFOridecon^000000",
                    "1 ^3355FFAlcohol^000000",
                    "1 ^3355FFCoal^000000",
                    "These are the things I need for Grandfather's masterpiece."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            if ctx.items().count(999)? > 39 && ctx.items().count(984)? > 0 && ctx.items().count(970)? > 0 && ctx.items().count(1003)? > 0 {
                ctx.items().take(999, 40)?;
                ctx.items().take(984, 1)?;
                ctx.items().take(970, 1)?;
                ctx.items().take(1003, 1)?;
                ctx.items().give(5001, 1)?;
                ctx.mes("*Tap! Tap! Tap!*")?;
                ctx.next()?;
                ctx.lines_as(
                    "Eric",
                    args![
                        "Thank you.",
                        "Because of you, I could accomplish my grandfather's wish. He would be happy if he looked down on me from Heaven."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Eric",
                    args!["As I said, please take this. I am just proud that I can make this by myself."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as("Eric", args!["Thank you for trying to help me, but you didn't bring all the items I need. I think you will find them all soon though."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        3 => {
            ctx.lines_as(
                "Eric",
                args!["N-no?", "Well...", "You have your own", "worries, I suppose.", "Farewell."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn eric(ctx: &Ctx) -> Script {
    eric_body(ctx, Vec::new()).map(|_| ())
}

fn nia_yagu_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_ball = Val::from(0);
    let mut l_end_time = Val::from(0);
    let mut l_input = Val::from(0);
    let mut l_input1 = Val::from(0);
    let mut l_input10 = Val::from(0);
    let mut l_input100 = Val::from(0);
    let mut l_name_record_s = Val::from("");
    let mut l_practice = Val::from(0);
    let mut l_retry = Val::from(0);
    let mut l_score_min = Val::from(0);
    let mut l_score_record = Val::from(0);
    let mut l_score_sec = Val::from(0);
    let mut l_start_time = Val::from(0);
    let mut l_strike = Val::from(0);
    let mut l_total_min = Val::from(0);
    let mut l_total_sec = Val::from(0);
    let mut l_total_time = Val::from(0);
    let mut l_yagu = Val::from(0);
    let mut l_yagu1 = Val::from(0);
    let mut l_yagu10 = Val::from(0);
    let mut l_yagu100 = Val::from(0);
    ctx.lines_as(
        "Nia",
        args![
            "Hello, I'm Nia, the fifth",
            "disciple of Mawong, the",
            "gaming mastermind. Now,",
            "how would you like to play a",
            "Number Match game with me?"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&[
        "Number Match Game?",
        "Practice Number Match",
        "Play Number Match",
        "Record Holder",
        "Cancel",
    ])? {
        0 => {
            ctx.lines_as(
                "Nia",
                args![
                    "Ah, well, the objective of",
                    "the Number Match game is to",
                    "guess the number I'm thinking",
                    "of. Of course, there are a few",
                    "rules that limit the numbers",
                    "that I can choose from."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nia",
                args![
                    "All the numbers that I make",
                    "up must be 3 digits, I can't",
                    "use 0 for any of the digits,",
                    "and I can't use the same",
                    "number for any of the digits",
                    "more than once."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nia",
                args![
                    "So, for example, I can't",
                    "use the numbers 103, 112,",
                    "252, or 701. Those numbers",
                    "either have a 0, or they use",
                    "the same number in the digits",
                    "more than once. Got it now?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nia",
                args![
                    "Now, you only get 5 guesses,",
                    "and after each guess, I'll tell",
                    "you how many digits you guessed",
                    "correctly, followed by the number of correctly guessed digits in",
                    "the correct digit place."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nia",
                args![
                    "For example, let's say the",
                    "number I come up with is 168.",
                    "If your first guess is 678, then you guessed two of the digits,",
                    "6 and 8, correctly. However,",
                    "only 8 is in the correct place."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nia",
                args![
                    "You know, it'd be a good idea",
                    "if you try the Practice Mode",
                    "first. Then, when you're more",
                    "comfortable, play the actual",
                    "Number Match game. Don't forget that speed counts for your score!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nia",
                args![
                    "Ah, if you happen to",
                    "make a new record in the",
                    "Number Match game, you'll",
                    "have your name recorded,",
                    "so don't miss this chance for",
                    "fame and maybe even fortune~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        1 => {
            ctx.lines_as(
                "Nia",
                args![
                    "Great, let's do a practice",
                    "Number Match game. Please",
                    "try to guess my number, okay?",
                    "If you want to cancel, then please enter a number greater than 1,000."
                ],
            )?;
            l_practice = Val::from(1);
        }
        2 => {
            ctx.lines_as(
                "Nia",
                args![
                    "Great, let's play Number Match!",
                    "You will have 5 chances to guess the number that I'm thinking of.",
                    "If you want to cancel, then please enter a number greater than 1,000."
                ],
            )?;
            l_practice = Val::from(0);
        }
        3 => {
            l_name_record_s = ctx.var("$050908_minus1_yagu$").get()?;
            if l_name_record_s == "" {
                l_name_record_s = Val::from("(null)");
            }
            l_score_record = ctx.var("$050908_minus1_yagu").get()?;
            l_score_min = l_score_record.clone().try_div(Val::from(60))?;
            l_score_sec = l_score_record.clone().try_rem(Val::from(60))?;
            ctx.lines_as(
                "Nia",
                args![
                    (Val::from("^ff0000") + l_name_record_s.clone()) + Val::from("^000000 is the"),
                    "record holder for the Number",
                    "Match game with a time of",
                    ((((Val::from("^ff0000") + l_score_min.clone()) + Val::from(" minutes, ")) + l_score_sec.clone())
                        + Val::from(" seconds^000000."))
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        4 => {
            ctx.lines_as(
                "Nia",
                args![
                    "Take care of yourself,",
                    "adventurer. If you ever",
                    "feel like playing a Number",
                    "Match game, come and talk",
                    "to me, alright? See you~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    ctx.next()?;
    loop {
        l_yagu100 = Val::from(ctx.rand_range(1, 9)?);
        l_yagu10 = Val::from(ctx.rand_range(1, 9)?);
        l_yagu1 = Val::from(ctx.rand_range(1, 9)?);
        if !l_yagu100.loosely_equals(&l_yagu10) && !l_yagu100.loosely_equals(&l_yagu1) && !l_yagu10.loosely_equals(&l_yagu1) {
            l_yagu = Val::from(100 * l_yagu100.number()? + 10 * l_yagu10.number()? + l_yagu1.number()?);
            break;
        }
    }
    l_start_time = ctx.call(Function::GetTimeTick, args![2])?;
    loop {
        loop {
            let (input, status) = runtime::input_number(ctx, Some(100), Some(999))?;
            l_input = input;
            if status != 0 {
                ctx.lines_as("Nia", args!["You've canceled the", "Number Match game."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            l_input100 = l_input.clone().try_div(Val::from(100))?;
            l_input10 = l_input.clone().try_rem(Val::from(100))?.try_div(Val::from(10))?;
            l_input1 = l_input.clone().try_rem(Val::from(10))?;
            if l_input100.number()? > 0
                && l_input10.number()? > 0
                && l_input1.number()? > 0
                && !l_input100.loosely_equals(&l_input10)
                && !l_input100.loosely_equals(&l_input1)
                && !l_input10.loosely_equals(&l_input1)
            {
                break;
            }
            ctx.lines_as(
                "Nia",
                args![
                    "You entered one of the",
                    "digits as 0, or input the",
                    "same numerical value in",
                    "the digits more than once.",
                    "Please enter another guess."
                ],
            )?;
            ctx.next()?;
        }
        l_retry = l_retry + Val::from(1);
        ctx.lines_as(
            "Nia",
            args![
                (Val::from("^ff0000") + ctx.player().name()?) + Val::from("^000000,"),
                (Val::from("your guess is ^0000ff") + l_input.clone()) + Val::from("^000000."),
                "Give me a moment to",
                "come up with your results."
            ],
        )?;
        ctx.next()?;
        l_strike = Val::from(0);
        if l_yagu100.loosely_equals(&l_input100) {
            l_strike = l_strike + Val::from(1);
        }
        if l_yagu10.loosely_equals(&l_input10) {
            l_strike = l_strike + Val::from(1);
        }
        if l_yagu1.loosely_equals(&l_input1) {
            l_strike = l_strike + Val::from(1);
        }
        l_ball = Val::from(0);
        if l_yagu100.loosely_equals(&l_input10) || l_yagu100.loosely_equals(&l_input1) {
            l_ball = l_ball + Val::from(1);
        }
        if l_yagu10.loosely_equals(&l_input100) || l_yagu10.loosely_equals(&l_input1) {
            l_ball = l_ball + Val::from(1);
        }
        if l_yagu1.loosely_equals(&l_input100) || l_yagu1.loosely_equals(&l_input10) {
            l_ball = l_ball + Val::from(1);
        }
        if l_strike == 3 {
            if l_practice.is_true() {
                ctx.lines_as(
                    "Nia",
                    args![
                        "Yes, the number that",
                        (Val::from("I guessed was ^ff0000") + l_yagu.clone()) + Val::from("^000000!"),
                        "Congratulations, you",
                        "just won this practice",
                        "game! Now you should be",
                        "ready for the real thing~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                l_end_time = ctx.call(Function::GetTimeTick, args![2])?;
                if runtime::op(&l_end_time, "<", &l_start_time)?.is_true() {
                    l_end_time = l_end_time + Val::from(3600);
                }
                l_total_time = l_end_time.clone().try_sub(l_start_time.clone())?;
                l_total_min = l_total_time.clone().try_div(Val::from(60))?;
                l_total_sec = l_total_time.clone().try_rem(Val::from(60))?;
                l_name_record_s = ctx.var("$050908_minus1_yagu$").get()?;
                if l_name_record_s == "" {
                    l_name_record_s = Val::from("(null)");
                }
                l_score_record = ctx.var("$050908_minus1_yagu").get()?;
                l_score_min = l_score_record.clone().try_div(Val::from(60))?;
                l_score_sec = l_score_record.clone().try_rem(Val::from(60))?;
                if runtime::op(&l_score_record, ">=", &l_total_time)?.is_true() || !l_score_record.is_true() {
                    ctx.var("$050908_minus1_yagu$").set(ctx.player().name()?)?;
                    ctx.var("$050908_minus1_yagu").set(l_total_time.clone())?;
                    ctx.lines_as(
                        "Nia",
                        args![
                            "Great! You guessed",
                            "my number, which was",
                            (Val::from("^ff0000") + l_yagu.clone()) + Val::from("^000000. You finished in"),
                            Val::from("^ff0000")
                                + l_total_min.clone()
                                + Val::from(" minutes, ")
                                + l_total_sec.clone()
                                + Val::from(" seconds^000000,"),
                            "breaking the previous record",
                            Val::from("of ^ff0000")
                                + l_score_min.clone()
                                + Val::from(" minutes, ")
                                + l_score_sec.clone()
                                + Val::from(" seconds^000000.")
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Nia",
                        args![
                            (Val::from("^ff0000") + l_name_record_s.clone()) + Val::from("^000000"),
                            "set that old record,",
                            "but now you are the",
                            "new person to beat in the",
                            "Match Game. Congratulations",
                            "for setting a new record!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Nia",
                    args![
                        "Nice work, you guessed",
                        "my number! The answer, of",
                        (Val::from("course, was ^ff0000") + l_yagu.clone()) + Val::from("^000000. You managed"),
                        "to guess this number correctly",
                        Val::from("in ^ff0000")
                            + l_total_min.clone()
                            + Val::from(" minutes, ")
                            + l_total_sec.clone()
                            + Val::from(" seconds^000000.")
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Nia",
                    args![
                        (Val::from("^ff0000") + l_name_record_s.clone()) + Val::from("^000000 is"),
                        "the current Number Match",
                        "record holder with a time",
                        Val::from("of ^ff0000")
                            + l_score_min.clone()
                            + Val::from(" minutes, ")
                            + l_score_sec.clone()
                            + Val::from(" seconds^000000."),
                        "Good luck, and hopefully you",
                        "can break this record someday~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if l_practice.is_true() {
            ctx.lines_as(
                "Nia",
                args![
                    (Val::from("Your guess has ^ff0000") + l_ball.clone()) + Val::from("^000000 of the"),
                    (Val::from("correct numbers. ^ff0000") + l_strike.clone()) + Val::from("^000000 of the"),
                    "digits in your guess have",
                    "the correct number in the",
                    "correct digit placement.",
                    "Alright, try it again~"
                ],
            )?;
            ctx.next()?;
        } else {
            ctx.lines_as(
                "Nia",
                args![
                    (Val::from("Your last guess had ^ff0000") + l_ball.clone()) + Val::from("^000000"),
                    "of the correct numbers,",
                    (Val::from("and ^ff0000") + l_strike.clone()) + Val::from("^000000 digits in your guess"),
                    "had the correct number in",
                    "the correct digit placement."
                ],
            )?;
            match l_retry.number()? {
                1 => ctx.mes("You have four guesses left.")?,
                2 => ctx.mes("You have three guesses left.")?,
                3 => ctx.mes("You have two guesses left.")?,
                4 => ctx.mes("You only have one guess left...")?,
                5 => {
                    ctx.lines(args![
                        Val::from("The answer was ^ff0000") + l_yagu.clone() + Val::from("^000000.")
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
            ctx.next()?;
        }
    }
    Ok(Val::from(0))
}

pub fn nia_yagu(ctx: &Ctx) -> Script {
    nia_yagu_body(ctx, Vec::new()).map(|_| ())
}
