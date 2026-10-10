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

pub fn applegamble(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_amount = Val::from(0);
    let mut l_giveapple = Val::from(0);
    let mut l_player1 = Val::from(0);
    let mut l_player2 = Val::from(0);
    let mut l_player3 = Val::from(0);
    let mut l_playersub = Val::from(0);
    let mut l_playertotal = Val::from(0);
    let mut l_table1 = Val::from(0);
    let mut l_table2 = Val::from(0);
    let mut l_table3 = Val::from(0);
    let mut l_tablesub = Val::from(0);
    let mut l_tabletotal = Val::from(0);
    let l_npc_name_s = runtime::arg(&args, 0, Val::from(0));
    match ctx.menu(&["Play Dice Game", "Learn Dice Game Rules", "Cancel"])? {
        2 => {
            ctx.lines_as(
                l_npc_name_s.clone(),
                args![
                    "I'm up for a game of",
                    "dice whenever you feel",
                    "like it. Just talk to me if",
                    "you ever get hit with the",
                    "sudden urge to gamble, kay?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        1 => {
            ctx.lines_as(
                l_npc_name_s.clone(),
                args![
                    "The rules for the Dice game",
                    "are pretty simple. First, you",
                    "place a bet by wagering Apples.",
                    "You can bet a maximum of 50",
                    "Apples at a time. To keep things",
                    "legal, I can only accept Apples."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_npc_name_s.clone(),
                args![
                    "But hey, if all that zeny",
                    "is burning a hole in your",
                    "pocket, head over to Fruitz",
                    "and you can buy as many",
                    "Apples as you want, playah~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_npc_name_s.clone(),
                args![
                    "Now, we begin with me",
                    "rolling two 6-sided dice.",
                    "When it's your turn, you'll",
                    "roll two 6-sided dice. After",
                    "that, both of us will have the",
                    "option of rolling a third die."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_npc_name_s.clone(),
                args![
                    "Now here's the important",
                    "thing. If your total is higher",
                    "than 12, you'll bust, meaning",
                    "that you lose. Otherwise, the",
                    "person with the higher total",
                    "is the winner. Got it?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_npc_name_s.clone(),
                args![
                    "Now, you'll be the first",
                    "to decide whether or not",
                    "you'll roll the third die. Then,",
                    "depending on your result, I'll",
                    "roll my third die... Or maybe not."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_npc_name_s.clone(),
                args![
                    "When you win, you'll",
                    "receive twice as many",
                    "Apples as you wagered.",
                    "But if we happen to tie, you",
                    "get the Apples that you bet",
                    "returned to you. Fair, right?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    ctx.lines_as(
        l_npc_name_s.clone(),
        args![
            "Ooh, so you'll play with",
            "me? Great! How many",
            "Apples would you like to bet?",
            "Remember, you can wager",
            "up to 50 Apples. If you'd like",
            "to cancel, please enter '0'."
        ],
    )?;
    ctx.next()?;
    loop {
        let (input, _) = runtime::input_number(ctx, None, None)?;
        l_amount = input;
        if l_amount == 0 {
            ctx.lines_as(
                l_npc_name_s.clone(),
                args!["Changed your mind?", "I understand. Well then,", "I hope we can play sometime."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if !(1..=50).contains(&l_amount.number()?) {
            ctx.lines_as(
                l_npc_name_s.clone(),
                args![
                    "You can't bet more than",
                    "50 Apples. Remember, we",
                    "need to keep these stakes",
                    "reasonable. Please enter",
                    "a value no greater than 50."
                ],
            )?;
            ctx.next()?;
            continue;
        }
        ctx.lines_as(
            l_npc_name_s.clone(),
            args![
                "So you'll be",
                Val::from("betting ^FF0000") + l_amount.clone() + Val::from("^000000 Apples."),
                "Is that right?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Yes", "No"])? == 1 {
            ctx.lines_as(
                l_npc_name_s.clone(),
                args![
                    "Mm, made a mistake?",
                    "Alright, please enter the",
                    "number of Apples you",
                    "wish to place in this bet"
                ],
            )?;
            ctx.next()?;
            continue;
        }
        if ctx.call(Function::CountItem, args![512])?.number()? < l_amount.number()? {
            ctx.lines(args![
                "I'm sorry, but you",
                "don't seem to have",
                "enough Apples for this",
                "bet... You can't gamble",
                "if you can't play, you know."
            ])?;
            ctx.next()?;
            continue;
        }
        ctx.call(Function::DelItem, args![512, l_amount.clone()])?;
        ctx.lines_as(
            l_npc_name_s.clone(),
            args!["Good!", "Now we can start", "this game! I'll roll first~"],
        )?;
        break;
    }
    ctx.mes("^3355FF*Rolling and rumbling*^000000")?;
    ctx.next()?;
    l_giveapple = Val::from(l_amount.number()? * 2);
    l_table1 = ctx.call(Function::Rand, args![1, 6])?;
    l_table2 = ctx.call(Function::Rand, args![1, 6])?;
    l_tablesub = l_table1.clone() + l_table2.clone();
    l_tabletotal = l_tablesub.clone();
    ctx.lines_as(
        l_npc_name_s.clone(),
        args![
            ((((Val::from("I got a ^0000FF") + l_table1.clone()) + Val::from("^000000 and a ^0000FF")) + l_table2.clone())
                + Val::from("^000000.")),
            ((Val::from("That's a total of ^0000FF") + l_tablesub.clone()) + Val::from("^000000.")),
            ((Val::from("^FF0000") + ctx.player().name()?) + Val::from("^000000, now it's your turn."))
        ],
    )?;
    ctx.next()?;
    let choice = runtime::select_values(ctx, &[Val::from("Cast Dice.")])?;
    ctx.var("@menu").set(choice)?;
    ctx.mes("^3355FF*Rolling and rumbling*^000000")?;
    l_player1 = ctx.call(Function::Rand, args![1, 6])?;
    l_player2 = ctx.call(Function::Rand, args![1, 6])?;
    l_playersub = l_player1.clone() + l_player2.clone();
    if l_playersub.number()? > 9 && l_amount.number()? > 39 {
        l_player1 = ctx.call(Function::Rand, args![1, 6])?;
        l_player2 = ctx.call(Function::Rand, args![1, 6])?;
        l_playersub = l_player1.clone() + l_player2.clone();
    }
    l_playertotal = l_playersub.clone();
    ctx.next()?;
    ctx.lines_as(
        l_npc_name_s.clone(),
        args![
            ((((((((Val::from("^FF0000") + ctx.player().name()?) + Val::from("^000000, you have ^FF0000")) + l_player1.clone())
                + Val::from("^000000 and ^FF0000"))
                + l_player2.clone())
                + Val::from("^000000. The total is ^FF0000"))
                + l_playersub.clone())
                + Val::from("^000000 ."))
        ],
    )?;
    ctx.next()?;
    ctx.lines(args![Val::from("[") + l_npc_name_s.clone() + Val::from("]")])?;
    if l_playersub.loosely_equals(&l_tablesub) {
        ctx.lines(args![
            ((((((Val::from("Currently my total is ^0000FF") + l_tablesub.clone()) + Val::from("^000000 and ^FF0000"))
                + ctx.player().name()?)
                + Val::from("^000000, your total is ^FF0000"))
                + l_playersub.clone())
                + Val::from("^000000. We are making an even game. Would you like to cast dice again?"))
        ])?;
    } else if l_playersub.number()? > l_tablesub.number()? {
        ctx.lines(args![
            ((((((((Val::from("Currently my total is ^0000FF") + l_tablesub.clone()) + Val::from("^000000 and ^FF0000"))
                + ctx.player().name()?)
                + Val::from("^000000, your total is ^FF0000"))
                + l_playersub.clone())
                + Val::from("^000000. ^FF0000"))
                + ctx.player().name()?)
                + Val::from("^000000, you are currently winning this game. Would you like to cast dice again?"))
        ])?;
    } else if l_tablesub.number()? > l_playersub.number()? {
        ctx.lines(args![
            ((((((Val::from("Currently my total is ^0000FF") + l_tablesub.clone()) + Val::from("^000000 and ^FF0000"))
                + ctx.player().name()?)
                + Val::from("^000000, your total is ^FF0000"))
                + l_playersub.clone())
                + Val::from("^000000. I am winning this game. Would you like to cast dice again?"))
        ])?;
    }
    ctx.next()?;
    match ctx.menu(&["Cast dice.", "Cancel."])? {
        0 => {
            ctx.mes("^3355FF*Rolling and rumbling*^000000")?;
            l_player3 = ctx.call(Function::Rand, args![1, 6])?;
            l_playertotal = l_playertotal + l_player3.clone();
            ctx.next()?;
            ctx.lines(args![Val::from("[") + l_npc_name_s.clone() + Val::from("]")])?;
            if l_playertotal.number()? > 12 {
                ctx.lines(args![
                    ((((((Val::from("^FF0000") + ctx.player().name()?) + Val::from("^000000, you got ^FF0000")) + l_player3.clone())
                        + Val::from("^000000 and the total is now ^FF0000"))
                        + l_playertotal.clone())
                        + Val::from("^000000. You lost this game. I am sorry but please try again."))
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if l_playertotal.number()? < l_tablesub.number()? {
                ctx.lines(args![((((((Val::from("^FF0000") + ctx.player().name()?) + Val::from("^000000, you got ^FF0000")) + l_player3.clone()) + Val::from("^000000 and the total is now ^FF0000")) + l_playertotal.clone()) + Val::from("^000000. Even though you casted dice again, still your total is smaller than mine. You lost the game. I am sorry and please try again."))])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if l_playertotal.loosely_equals(&l_tablesub) {
                if l_tablesub.number()? > 8 {
                    ctx.lines(args![
                        ((((((Val::from("^FF0000") + ctx.player().name()?) + Val::from("^000000, you got ^FF0000")) + l_player3.clone())
                            + Val::from("^000000 and the total is now ^FF0000"))
                            + l_playertotal.clone())
                            + Val::from(
                                "^000000. I don't want to take any risk, let's end this game in a draw. Let's play again some other time~"
                            ))
                    ])?;
                    ctx.close_window()?;
                    ctx.call(Function::GetItem, args![512, l_amount.clone()])?;
                    return Err(Stop::End);
                }
                ctx.lines(args!["Alright.", "Let me cast the dice again."])?;
            } else {
                ctx.lines(args![
                    ((((((Val::from("^FF0000") + ctx.player().name()?) + Val::from("^000000, you got ^FF0000")) + l_player3.clone())
                        + Val::from("^000000 and the total is now ^FF0000"))
                        + l_playertotal.clone())
                        + Val::from("^000000. Now it is my turn."))
                ])?;
            }
        }
        1 => {
            ctx.lines(args![Val::from("[") + l_npc_name_s.clone() + Val::from("]")])?;
            if l_playersub.number()? > l_tablesub.number()? {
                ctx.mes("I see, you don't want to take risk of losing the game. Okay, let me cast dice again.")?;
            } else if l_playersub.loosely_equals(&l_tablesub) {
                if l_tablesub.number()? > 8 {
                    ctx.mes("I see, you don't want to take risk of losing this game. Neither do I, let's end this game in a draw. Let's play again some other time~")?;
                    ctx.close_window()?;
                    ctx.call(Function::GetItem, args![512, l_amount.clone()])?;
                    return Err(Stop::End);
                }
                ctx.lines(args!["Alright.", "Let me cast the dice again."])?;
            } else {
                ctx.lines(args![
                    "It couldn't hurt to try.",
                    "Well, I win this time.",
                    "I'm sorry, let's try play",
                    "again sometime."
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        _ => {}
    }
    ctx.next()?;
    ctx.mes("^3355FF*Rolling and rumbling*^000000")?;
    l_table3 = ctx.call(Function::Rand, args![1, 6])?;
    l_tabletotal = l_tabletotal + l_table3.clone();
    ctx.next()?;
    ctx.lines(args![Val::from("[") + l_npc_name_s.clone() + Val::from("]")])?;
    if l_tabletotal.number()? > 12 {
        ctx.lines(args![((((Val::from("I got ^0000FF") + l_table3.clone()) + Val::from("^000000 and the total is now ^0000FF")) + l_tabletotal.clone()) + Val::from("^000000. I lost this game since my total exceeded 12. Let me give you my apples. Congratulations, that was a great game."))])?;
        ctx.close_window()?;
        ctx.call(Function::GetItem, args![512, l_giveapple.clone()])?;
        return Err(Stop::End);
    } else if l_playertotal.number()? > l_tabletotal.number()? {
        ctx.lines(args![
            ((((((((Val::from("I got ^0000FF") + l_table3.clone()) + Val::from("^000000 and the total is now ^0000FF"))
                + l_tabletotal.clone())
                + Val::from("^000000. With total ^FF0000"))
                + l_playertotal.clone())
                + Val::from("^000000 you won this game, ^FF0000"))
                + ctx.player().name()?)
                + Val::from("^000000. Let me give you my apples. It was a great game and I hope we will play again some other time."))
        ])?;
        ctx.close_window()?;
        ctx.call(Function::GetItem, args![512, l_giveapple.clone()])?;
        return Err(Stop::End);
    } else if l_playertotal.loosely_equals(&l_tabletotal) {
        ctx.lines(args![
            ((((((((Val::from("I got ^0000FF") + l_table3.clone()) + Val::from("^000000 and the total is now ^0000FF"))
                + l_tabletotal.clone())
                + Val::from("^000000. With total ^FF0000"))
                + l_playertotal.clone())
                + Val::from("^000000 this game came out even, ^FF0000"))
                + ctx.player().name()?)
                + Val::from(
                    "^000000. Let me give you your apple back. It was a great game and I hope we will play again some other time."
                ))
        ])?;
        ctx.close_window()?;
        ctx.call(Function::GetItem, args![512, l_amount.clone()])?;
        return Err(Stop::End);
    } else if l_playertotal.number()? < l_tabletotal.number()? {
        ctx.lines(args![
            ((((((((Val::from("I got ^0000FF") + l_table3.clone()) + Val::from("^000000 and the total is now ^0000FF"))
                + l_tabletotal.clone())
                + Val::from("^000000. With total ^FF0000"))
                + l_playertotal.clone())
                + Val::from("^000000 you lost this game, ^FF0000"))
                + ctx.player().name()?)
                + Val::from("^000000. I am sorry but please try again."))
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}
