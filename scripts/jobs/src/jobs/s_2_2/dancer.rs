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

pub fn sonotora_1(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Athena Sonotora",
        args!["They say the", "famous dance school", "here in Comodo is going", "to open soon."],
    )?;
    ctx.next()?;
    ctx.lines_as("Athena Sonotora", args!["Aah...", "To be a prima donna", "in the spotlight!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Athena Sonotora",
        args![
            "I want to sign up too,",
            "but the requirements are",
            "so specific. I wonder if",
            "I should just try anyways..."
        ],
    )?;
    ctx.close()
}

pub fn bor_robin_1(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Bor Robin",
        args![
            "Aah....",
            "A prima donna",
            "in the spotlight!",
            "I'll be able to watch them become Dancers right before my eyes...!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Bor Robin",
        args!["It's great to be", "a man in this day and age! Hurray for the Comodo Theater!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Bor Robin",
        args![
            "Mm?",
            "You want",
            "to go, too?",
            "It's a good opportunity to watch the Dancer job change test."
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Go to the Job Change Area", "Cancel"])? == 0 {
        ctx.lines_as("Bor Robin", args!["Yaay~~", "Let's go!"])?;
        ctx.close_window()?;
        ctx.warp("job_duncer", 70, 49)?;
        return ctx.end();
    }
    ctx.lines_as(
        "Bor Robin",
        args!["Huh...", "Well, I can't", "help it if you don't", "want to accompany me."],
    )?;
    ctx.close()
}

fn aile_da_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_count: Vec<Val> = Vec::new();
    let mut l_i = Val::from(0);
    let mut l_item: Vec<Val> = Vec::new();
    let mut l_item_nd = Val::from(0);
    let mut l_size = Val::from(0);
    if ctx.var("Upper").get()? == 1 {
        ctx.lines_as(
            "Aile",
            args![
                "One two three four,",
                "Two two three four,",
                "three four, three four,",
                "one two three four.",
                "Um?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Aile",
            args!["I'm sorry, but you're interrupting my practice by looking at me funny."],
        )?;
        ctx.next()?;
        ctx.lines_as("Aile", args![".......", ".....Hey, haven't I seen you before?"])?;
        ctx.next()?;
        ctx.lines_as("Aile", args!["Err...", "That's weird, I can't remember where I've seen you."])?;
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return Err(Stop::End);
    }
    if ctx.var("BaseJob").get()? != constants::JOB_ARCHER {
        if ctx.var("BaseJob").get()? == constants::JOB_BARD {
            ctx.fx().cutin("job_dancer_eir01", 2)?;
            ctx.lines_as(
                "Aile",
                args![
                    "Welcome~!",
                    "Let me know",
                    "if you have any new songs. We can always use some new music to complement our performances."
                ],
            )?;
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
            return Err(Stop::End);
        } else if ctx.var("BaseJob").get()? == constants::JOB_DANCER {
            ctx.fx().cutin("", 2)?;
            ctx.lines_as(
                "Aile",
                args![
                    "Welcome~!",
                    "How are you",
                    "these days?",
                    "Do many people enjoy",
                    "your performances?"
                ],
            )?;
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
            return Err(Stop::End);
        }
        ctx.fx().cutin("job_dancer_eir03", 2)?;
        ctx.lines_as(
            "Aile",
            args![
                "Welco--Mmm?",
                "Hey, only authorized personnel can come here. Not just anyone can enter the Dance School."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Aile",
            args!["If you want to watch, why don't you go to the Dance Stage in town?"],
        )?;
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return Err(Stop::End);
    }
    if ctx.var("danc_q").get()? == 0 && ctx.var("Sex").get()? == constants::SEX_FEMALE && ctx.var("BaseJob").get()? == constants::JOB_ARCHER
    {
        ctx.fx().cutin("job_dancer_eir01", 2)?;
        ctx.lines_as(
            "Aile",
            args![
                "Welcome~!",
                "This is the",
                "'Comodo Dance School,'",
                "where we teach various dances from different countries. We provide entertainement for travelers from all over the world."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Aile", args!["We also provide the opportunity for aspiring Dancers to become famous throughout the Rune-Midgarts Kingdom! Doesn't dancing in the spotlight sound spectacular?"])?;
        ctx.next()?;
        ctx.lines_as("Aile", args!["I think it's fair to let you know that our school is selective. So we don't accept students who don't seem to have the talent to become Dancers."])?;
        ctx.next()?;
        ctx.fx().cutin("job_dancer_eir02", 2)?;
        ctx.lines_as("Aile", args!["What do you think?", "Do you want to sign up? You only have to write a couple of things on the application, and you can just come to the lessons once or twice and try it out."])?;
        ctx.next()?;
        ctx.fx().cutin("job_dancer_eir01", 2)?;
        ctx.lines_as("Aile", args!["So what do", "you want to do~?"])?;
        ctx.next()?;
        if ctx.menu(&["Fill out the application.", "I'll pass."])? == 0 {
            if ctx.player().job_level()? > 39 {
                ctx.fx().cutin("job_dancer_eir02", 2)?;
                ctx.lines_as("Aile", args!["Good choice!!", "Just fill out the application right there."])?;
                ctx.next()?;
                ctx.mes("...")?;
                ctx.next()?;
                ctx.lines(args!["...", "......"])?;
                ctx.next()?;
                ctx.mes("^3355FF*Shuffle Shuffle*^000000")?;
                ctx.next()?;
                ctx.fx().cutin("job_dancer_eir01", 2)?;
                ctx.lines_as(
                    "Aile",
                    args![
                        "Your name is",
                        ((Val::from("") + Val::from(ctx.player().name()?)) + Val::from("?")),
                        "Wow! What a pretty name! Just a moment, I have to show this to the director, so come back in a little bit, okay?"
                    ],
                )?;
                ctx.close_window()?;
                ctx.fx().cutin("", 255)?;
                ctx.var("danc_q").set(Val::from(1))?;
                ctx.quests().start(7000)?;
                return Err(Stop::End);
            } else {
                ctx.fx().cutin("job_dancer_eir01", 2)?;
                ctx.lines_as(
                    "Aile",
                    args![
                        "Mmm...",
                        "It seems that",
                        "you aren't quite qualified to enroll in our school yet. You need to be at least Job Level 40."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aile",
                    args!["Well, I hope", "that you apply", "again when you meet", "the requirements."],
                )?;
                ctx.close_window()?;
                ctx.fx().cutin("", 255)?;
                return Err(Stop::End);
            }
        }
        ctx.fx().cutin("job_dancer_eir01", 2)?;
        ctx.lines_as(
            "Aile",
            args![
                "Aww~",
                "Just think about it.",
                "Don't forget to come back",
                "if you change your mind."
            ],
        )?;
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return Err(Stop::End);
    } else if ctx.var("Sex").get()? == constants::SEX_MALE {
        ctx.fx().cutin("job_dancer_eir03", 2)?;
        ctx.lines_as(
            "Aile",
            args![
                "Welco--Mmm?",
                "Hey, this place is only for authorized personnel. If you want to sing, you should go look into being a Bard."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Aile",
            args![
                "Not all Archers",
                "can become Dancers.",
                "At least, not without some sort of sex change~"
            ],
        )?;
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return Err(Stop::End);
    } else if ctx.var("danc_q").get()? == 1 {
        ctx.fx().cutin("job_dancer_eir01", 2)?;
        ctx.lines_as(
            "Aile",
            args![
                "Good.",
                "Since you signed up earlier, I'll let you know some things you'll need to bring for your lessons."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Aile", args!["We're short on some supplies, but you'll be using them for yourself anyway. Just think of it as part of the tuition, so don't worry too much."])?;
        ctx.next()?;
        l_item_nd = ctx.call(Function::Rand, args![1, 10])?;
        if l_item_nd.number()? > 0 && l_item_nd.number()? < 3 {
            ctx.var("danc_q").set(Val::from(2))?;
            ctx.quests().change(7000, 7001)?;
            ctx.lines_as("Aile", args!["First, there's the tuition fee of ^CD688910,000 Zeny^000000. Then, you'll about ^CD688920 Sticky Mucus^000000 for shoe polish."])?;
            ctx.next()?;
            ctx.lines_as("Aile", args!["Then, bring ^CD68893 Jellopy^000000 and ^CD68895 Red Potions^000000 to use as ointment. And of course, you'll need a pair of ^CD6889Shoes^000000."])?;
            ctx.next()?;
            ctx.lines_as(
                "Aile",
                args![
                    "Once again, that's",
                    "^CD688910,000 Zeny^000000,",
                    "^CD688920 Sticky Mucus^000000,",
                    "^CD68893 Jellopy^000000,",
                    "^CD68895 Red Potions^000000 and",
                    "^CD68891 Shoes^000000."
                ],
            )?;
        } else if l_item_nd == 4 {
            ctx.var("danc_q").set(Val::from(3))?;
            ctx.quests().change(7000, 7002)?;
            ctx.lines_as("Aile", args!["First, there's the tuition fee of ^CD688910,000 Zeny^000000. Then, bring ^CD68895 Earthworm Peelings^000000 for polishing the floor and, of course, a pair of ^CD6889Boots^000000."])?;
            ctx.next()?;
            ctx.lines_as(
                "Aile",
                args![
                    "Once again that's",
                    "^CD688910,000 Zeny^000000,",
                    "^CD68895 Earthworm Peelings^000000 and ",
                    "^CD68891 Boots^000000."
                ],
            )?;
        } else {
            ctx.var("danc_q").set(Val::from(4))?;
            ctx.quests().change(7000, 7003)?;
            ctx.lines_as("Aile", args!["First, there's the tuition fee of ^CD688910,000 Zeny^000000. Then, bring ^CD68892 Clam Shells^000000 for your costume, ^CD68895 Yellow Potions^000000 and ^CD688920 Jellopy^000000 to treat foot injuries."])?;
            ctx.next()?;
            ctx.lines_as("Aile", args!["You'll also need to bring ^CD688910 Black Hairs^000000 to make wigs for the performances and, of course, a pair of ^CD6889Sandals^000000. Once again, that's..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Aile",
                args![
                    "^CD688910,000 Zeny^000000,",
                    "^CD68892 Clam Shells^000000,",
                    "^CD68895 Yellow Potions^000000,",
                    "^CD688920 Jellopy^000000,",
                    "^CD688910 Black Hairs^000000 and",
                    "^CD6889Sandals^000000."
                ],
            )?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Aile",
            args!["Once you've gathered everything that you need, come back so that we can begin the lessons, okay?"],
        )?;
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return Err(Stop::End);
    } else if ctx.var("danc_q").get()?.number()? >= 2 && ctx.var("danc_q").get()?.number()? <= 4 {
        let subject1 = ctx.var("danc_q").get()?;
        if subject1 == 2 {
            runtime::local_set(&mut l_item, &Val::from(0), Val::from(938), false);
            runtime::local_set(&mut l_item, &Val::from(1), Val::from(909), false);
            runtime::local_set(&mut l_item, &Val::from(2), Val::from(501), false);
            runtime::local_set(&mut l_item, &Val::from(3), Val::from(2403), false);
            runtime::local_set(&mut l_count, &Val::from(0), Val::from(20), false);
            runtime::local_set(&mut l_count, &Val::from(1), Val::from(3), false);
            runtime::local_set(&mut l_count, &Val::from(2), Val::from(5), false);
            runtime::local_set(&mut l_count, &Val::from(3), Val::from(1), false);
        } else if subject1 == 3 {
            runtime::local_set(&mut l_item, &Val::from(0), Val::from(1055), false);
            runtime::local_set(&mut l_item, &Val::from(1), Val::from(2405), false);
            runtime::local_set(&mut l_count, &Val::from(0), Val::from(5), false);
            runtime::local_set(&mut l_count, &Val::from(1), Val::from(1), false);
        } else if subject1 == 4 {
            runtime::local_set(&mut l_item, &Val::from(0), Val::from(965), false);
            runtime::local_set(&mut l_item, &Val::from(1), Val::from(503), false);
            runtime::local_set(&mut l_item, &Val::from(2), Val::from(909), false);
            runtime::local_set(&mut l_item, &Val::from(3), Val::from(1020), false);
            runtime::local_set(&mut l_item, &Val::from(4), Val::from(2401), false);
            runtime::local_set(&mut l_count, &Val::from(0), Val::from(2), false);
            runtime::local_set(&mut l_count, &Val::from(1), Val::from(5), false);
            runtime::local_set(&mut l_count, &Val::from(2), Val::from(20), false);
            runtime::local_set(&mut l_count, &Val::from(3), Val::from(10), false);
            runtime::local_set(&mut l_count, &Val::from(4), Val::from(1), false);
        }
        l_size = Val::from(l_item.len() as i32);
        l_i = Val::from(0);
        'l2: loop {
            if l_i.number()? >= l_size.number()? {
                break 'l2;
            }
            'b2: {
                if runtime::op(
                    &ctx.call(Function::CountItem, vec![runtime::local_get(&l_item, &l_i, false)])?,
                    "<",
                    &runtime::local_get(&l_count, &l_i, false),
                )?
                .is_true()
                {
                    break 'l2;
                }
            }
            l_i = l_i.clone() + Val::from(1);
        }
        if l_i.loosely_equals(&l_size) && ctx.player().zeny()? > 9999 {
            ctx.fx().cutin("job_dancer_eir02", 2)?;
            ctx.lines_as(
                "Aile",
                args![
                    "Oh...!",
                    "You brought",
                    "everything!",
                    "Alright then,",
                    "let me take your",
                    "tuition fee."
                ],
            )?;
            ctx.next()?;
            ctx.fx().cutin("job_dancer_eir01", 2)?;
            ctx.player().set_zeny(ctx.player().zeny()? - 10000)?;
            ctx.lines_as("Aile", args!["Next, go to ^CD6889Bijou^000000, who is in charge of the interviewing process. She will have a couple of things she'll need to ask you."])?;
            ctx.var("danc_q").set(Val::from(5))?;
            if ctx.call(Function::CheckQuest, args![7001])? != -1 {
                ctx.quests().change(7001, 7004)?;
            } else if ctx.call(Function::CheckQuest, args![7002])? != -1 {
                ctx.quests().change(7002, 7004)?;
            } else {
                ctx.quests().change(7003, 7004)?;
            }
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
            return Err(Stop::End);
        } else {
            ctx.fx().cutin("job_dancer_eir01", 2)?;
            ctx.lines_as(
                "Aile",
                args![
                    "Mmm...?",
                    "You don't have",
                    "everything yet?",
                    "Let me remind you",
                    "so you can bring",
                    "what you need next time."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Aile", args!["Bring...", "^CD688910,000 Zeny^000000,"])?;
            if ctx.var("danc_q").get()? == 2 {
                ctx.lines(args![
                    ((((Val::from("^CD6889") + runtime::local_get(&l_count, &Val::from(0), false)) + Val::from(" "))
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item, &Val::from(0), false)])?)
                        + Val::from("^000000,")),
                    ((((Val::from("^CD6889") + runtime::local_get(&l_count, &Val::from(1), false)) + Val::from(" "))
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item, &Val::from(1), false)])?)
                        + Val::from("^000000,")),
                    ((((Val::from("^CD6889") + runtime::local_get(&l_count, &Val::from(2), false)) + Val::from(" "))
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item, &Val::from(2), false)])?)
                        + Val::from("^000000 and")),
                    ((((Val::from("^CD6889") + runtime::local_get(&l_count, &Val::from(3), false)) + Val::from(" "))
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item, &Val::from(3), false)])?)
                        + Val::from("^000000."))
                ])?;
            } else if ctx.var("danc_q").get()? == 3 {
                ctx.lines(args![
                    ((((Val::from("^CD6889") + runtime::local_get(&l_count, &Val::from(0), false)) + Val::from(" "))
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item, &Val::from(0), false)])?)
                        + Val::from("^000000 and")),
                    ((((Val::from("^CD6889") + runtime::local_get(&l_count, &Val::from(1), false)) + Val::from(" "))
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item, &Val::from(1), false)])?)
                        + Val::from("^000000."))
                ])?;
            } else {
                ctx.lines(args![
                    ((((Val::from("^CD6889") + runtime::local_get(&l_count, &Val::from(0), false)) + Val::from(" "))
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item, &Val::from(0), false)])?)
                        + Val::from("^000000,")),
                    ((((Val::from("^CD6889") + runtime::local_get(&l_count, &Val::from(1), false)) + Val::from(" "))
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item, &Val::from(1), false)])?)
                        + Val::from("^000000,")),
                    ((((Val::from("^CD6889") + runtime::local_get(&l_count, &Val::from(2), false)) + Val::from(" "))
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item, &Val::from(2), false)])?)
                        + Val::from("^000000,")),
                    ((((Val::from("^CD6889") + runtime::local_get(&l_count, &Val::from(3), false)) + Val::from(" "))
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item, &Val::from(3), false)])?)
                        + Val::from("^000000 and")),
                    ((((Val::from("^CD6889") + runtime::local_get(&l_count, &Val::from(4), false)) + Val::from(" "))
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item, &Val::from(4), false)])?)
                        + Val::from("^000000."))
                ])?;
            }
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
            return Err(Stop::End);
        }
    } else if ctx.var("danc_q").get()? == 5 {
        ctx.fx().cutin("job_dancer_eir01", 2)?;
        ctx.lines_as(
            "Aile",
            args!["Hmm...?", "Are you having", "trouble finding", "^CD6889Bijou^000000?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Aile", args!["You need to talk to her because she's in charge of the interviewing process. Don't worry, she should be somewhere here in the Dance School."])?;
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return Err(Stop::End);
    } else if ctx.var("danc_q").get()?.number()? > 5 {
        ctx.fx().cutin("job_dancer_eir01", 2)?;
        ctx.lines_as("Aile", args!["I'll be looking", "forward to a great", "performance~"])?;
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return Err(Stop::End);
    } else {
        ctx.fx().cutin("job_dancer_eir03", 2)?;
        ctx.lines_as(
            "Aile",
            args!["Welcom--Hm?", "Hey, only authorized", "personnel are allowed", "in here."],
        )?;
        ctx.next()?;
        ctx.lines_as("Aile", args!["If you want to watch, be quiet and don't disturb the performers. Everyone here is busy practicing so that they can become fine Dancers."])?;
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return Err(Stop::End);
    }
}

pub fn aile_da(ctx: &Ctx) -> Script {
    aile_da_body(ctx, Vec::new()).map(|_| ())
}

pub fn bijou_da(ctx: &Ctx) -> Script {
    let mut l_da_score = Val::from(0);
    let mut l_jlevel = Val::from(0);
    if ctx.var("SkillPoint").get()?.is_true() {
        ctx.lines_as(
            "Bijou",
            args![
                "You can't change jobs",
                "if you still have skill",
                "points left. Use the rest",
                "and come back later."
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("BaseJob").get()? != constants::JOB_ARCHER {
        if ctx.var("BaseJob").get()? == constants::JOB_BARD {
            ctx.lines_as(
                "Bijou",
                args![
                    "Welcome~",
                    "Ooh, a Bard! Do you have any new songs to show us? We can always use some musical accompaniment for our dances."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("BaseJob").get()? == constants::JOB_DANCER {
            ctx.lines_as("Bijou", args!["Oh my...!", "Welcome back~"])?;
            ctx.next()?;
            ctx.lines_as(
                "Bijou",
                args![
                    "How are you",
                    "these days?",
                    "A lot of people",
                    "must love watching",
                    "you dance. Are you",
                    "enjoying the spotlight?"
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Bijou",
            args!["Oh dear~", "You seem to have traveled quite a distance to watch me perform."],
        )?;
        ctx.next()?;
        ctx.lines_as("Bijou", args!["I'm sorry, but I've retired. Now I'm focusing on training new Dancers. If you go to the Center Stage, you can watch my students~"])?;
        return ctx.close();
    } else if ctx.var("danc_q").get()?.number()? < 5 {
        ctx.lines_as("Bijou", args!["Oh my~", "You want to", "become a Dancer,", "don't you?"])?;
        ctx.next()?;
        ctx.lines_as("Bijou", args!["I know you're excited, but the first step is the application. Go over to the left side of the stage where Aile can help you with that."])?;
        return ctx.close();
    } else if ctx.var("danc_q").get()?.number()? > 4 && ctx.var("danc_q").get()?.number()? < 7 {
        if ctx.var("danc_q").get()? == 5 {
            ctx.lines_as("Bijou", args!["Oh my~", "You want to", "become a Dancer,", "don't you?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Bijou",
                args![
                    "G-goodness!",
                    "Look at that stomach fat!",
                    "Well, it's not much, so you'll lose it in no time. Especially since I'll be handling your training~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Bijou",
                args![
                    "Still...",
                    "The idea of the",
                    "perfect body sure",
                    "has changed since",
                    "I was young. Anyway..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Bijou",
                args![
                    "Let's start",
                    "with the interview.",
                    "I'm only going to ask",
                    "a couple of simple things",
                    "so don't worry~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Bijou", args!["Okay...", "Let's begin."])?;
            ctx.next()?;
        } else {
            ctx.lines_as(
                "Bijou",
                args!["Oh, you're back~", "Have you studied", "some more? Try to", "pass this time, okay?"],
            )?;
            ctx.next()?;
        }
        let subject1 = ctx.rand_range(1, 3)?;
        if subject1 == 1 {
            ctx.lines_as(
                "Bijou",
                args![
                    "1. The Dancer's dance, ^CD6889Lady Luck^000000,",
                    "increases which of the following?"
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Intelligence (INT)", "Dexterity (DEX)", "Vitality (VIT)", "Critical Attack Rate"])? == 3 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as(
                "Bijou",
                args!["2. Of the following,", "which can you not consider", "to be a dance?"],
            )?;
            ctx.next()?;
            'b2: {
                let subject2 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Tango:Tap Dance:HIP-HOP:Hip Shaker:Lightning Bolt")],
                )?);
                let mut matched2 = false;
                let no_case2 = !subject2.loosely_equals(&Val::from(5));
                if !matched2 && no_case2 {
                    matched2 = true;
                }
                if matched2 {
                    l_da_score = l_da_score.clone().try_sub(Val::from(10))?;
                    break 'b2;
                }
                if !matched2 && subject2.loosely_equals(&Val::from(5)) {
                    matched2 = true;
                }
                if matched2 {
                    l_da_score = l_da_score.clone() + Val::from(10);
                    break 'b2;
                }
            }
            ctx.lines_as("Bijou", args!["3. Which of the following", "best describes a Dancer?"])?;
            ctx.next()?;
            if ctx.menu(&["Person who yells.", "A loud person.", "A person who dances.", "A person who sings."])? == 2 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as("Bijou", args!["4. Which of the following", "cannot be associated with Comodo?"])?;
            ctx.next()?;
            if ctx.menu(&[
                "Beach city.",
                "Dancer Job Change.",
                "Always dark like the night.",
                "Dungeons in 3 directions.",
                "A lot of Thieves.",
            ])? == 4
            {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as(
                "Bijou",
                args!["5. Before Comodo, what is the region name of the region NorthEast of Pharoah's Lighthouse Island?"],
            )?;
            ctx.next()?;
            if ctx.menu(&["Elmeth Plateau", "Comuko Beach", "Comodo Beach", "Ginai Swamp"])? == 2 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as("Bijou", args!["6. Who is the most", "beautiful dancer?"])?;
            ctx.next()?;
            let subject3 = Val::from(runtime::select_values(
                ctx,
                &[((Val::from("") + Val::from(ctx.player().name()?)) + Val::from(":Bijou:Aile:Bonjour"))],
            )?);
            if subject3 == 1 {
                ctx.lines_as(
                    "Bijou",
                    args![
                        "...",
                        "That's...",
                        "^660000completely wrong^000000.",
                        "Didn't you see the",
                        "other choices?!",
                        "Minus points...!"
                    ],
                )?;
                l_da_score = l_da_score.clone().try_sub(Val::from(10))?;
                ctx.next()?;
            } else if subject3 == 2 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as(
                "Bijou",
                args!["7. Of the following,", "who can perform together", "with a Dancer?"],
            )?;
            ctx.next()?;
            if ctx.menu(&["Assassin", "Bard", "Alchemist", "Sage"])? == 1 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as("Bijou", args!["8. Which of the following", "is not a specialty of Comodo?"])?;
            ctx.next()?;
            if ctx.menu(&["Berserk Potion", "Clam Shell", "Crab Shell", "Shining Stone"])? == 3 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as("Bijou", args!["9. Who is the manager", "of the Comodo Casino?"])?;
            ctx.next()?;
            if ctx.menu(&["Yoo", "Moo", "Hoon", "Roul"])? == 1 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as("Bijou", args!["10. Who accepts the", "Dancer job change", "applications?"])?;
            ctx.next()?;
            if ctx.menu(&["Bijou", "Aile", "Athena", "Sonotora"])? == 1 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
        } else if subject1 == 2 {
            ctx.lines_as(
                "Bijou",
                args!["1. What is the effect", "of the combined skill,", "^CD6889Mental Sensing^000000?"],
            )?;
            ctx.next()?;
            if ctx.menu(&[
                "Instant monster death.",
                "Doubles damage.",
                "Increases experience.",
                "Increases attack speed.",
            ])? == 2
            {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as(
                "Bijou",
                args!["2. Which is considered", "bad etiquette on the dance", "floor after a dance?"],
            )?;
            ctx.next()?;
            if ctx.menu(&[
                "Thank your partner.",
                "Praise your partner's dance.",
                "Ask to dance a different dance.",
                "Criticize your partner.",
            ])? == 3
            {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as(
                "Bijou",
                args![
                    "3. Which is not an",
                    "appropriate response",
                    "when someone makes",
                    "a mistake while you",
                    "are dancing together?"
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&[
                "Smile at each other and continue dancing.",
                "Point out the mistake.",
                "Ignore it if the dancer does not realize it.",
                "Give them a smile.",
            ])? == 1
            {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as("Bijou", args!["4. In which town", "can you change jobs", "to a Dancer?"])?;
            ctx.next()?;
            if ctx.menu(&["Cocomo", "Sandarman", "Comudo", "Comodo"])? == 3 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as("Bijou", args!["5. How many dungeons", "are directly connected", "to Comodo?"])?;
            ctx.next()?;
            if ctx.menu(&["1", "2", "3", "4"])? == 2 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as("Bijou", args!["6. Which of the following", "is not a Cute Pet monster?"])?;
            ctx.next()?;
            if ctx.menu(&["Isis", "Argiope", "Dokebi", "Deviruchi"])? == 1 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as("Bijou", args!["7. Who is the most", "graceful dancer?"])?;
            ctx.next()?;
            let subject4 = Val::from(runtime::select_values(
                ctx,
                &[((Val::from("") + Val::from(ctx.player().name()?)) + Val::from(":Bijou:Isis:Mercy Bokou"))],
            )?);
            if subject4 == 1 {
                ctx.lines_as(
                    "Bijou",
                    args![
                        "...",
                        "That's...",
                        "^660000completely wrong^000000.",
                        "Didn't you see the",
                        "other choices?!",
                        "Minus points...!"
                    ],
                )?;
                l_da_score = l_da_score.clone().try_sub(Val::from(10))?;
                ctx.next()?;
            } else if subject4 == 2 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as("Bijou", args!["8. What is the", "exact name of the", "Kafra in Comodo?"])?;
            ctx.next()?;
            if ctx.menu(&[
                "Kafra Headquarters",
                "Kafra West Headquarters",
                "Kafra Service",
                "Kafra Headquarters",
                " Western Branch",
            ])? == 3
            {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as("......", args!["9. What is my name?"])?;
            ctx.next()?;
            if ctx.menu(&["Borjuis", "Bourgeois", "Bijou", "Beruberu"])? == 2 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as("Bijou", args!["10. What is the", "effect of ^CD6889Lullaby^000000?"])?;
            ctx.next()?;
            if ctx.menu(&[
                "Casts the Blind effect in the area.",
                "Casts the Sleep effect on the area.",
                "Puts a night effect on the area.",
                "Freezes the area.",
            ])? == 1
            {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
        } else if subject1 == 3 {
            ctx.lines_as(
                "Bijou",
                args!["1. What is the effect", "of the skill ^CD6889Dance Lessons^000000?"],
            )?;
            ctx.next()?;
            'b5: {
                let subject5 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Increases INT:Increases the effect of dancing skills:Increase damage of Whip weapons.:Inflict Stun on a certain area around the caster.",
                    )],
                )?);
                let mut matched5 = false;
                let no_case5 = !subject5.loosely_equals(&Val::from(2)) && !subject5.loosely_equals(&Val::from(3));
                if !matched5 && no_case5 {
                    matched5 = true;
                }
                if matched5 {
                    break 'b5;
                }
                if !matched5 && subject5.loosely_equals(&Val::from(2)) {
                    matched5 = true;
                }
                if !matched5 && subject5.loosely_equals(&Val::from(3)) {
                    matched5 = true;
                }
                if matched5 {
                    l_da_score = l_da_score.clone() + Val::from(10);
                    break 'b5;
                }
            }
            ctx.lines_as(
                "Bijou",
                args![
                    "2. What dance uses shoes",
                    "that are designed to make",
                    "sound as the dancer rolls",
                    "their feet and taps the",
                    "ground to create a rhythm?"
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Tap Dance", "Improve Concentration", "Tango", "Double Strafing"])? == 0 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as(
                "Bijou",
                args!["3. Which of the following", "is not a characteristic of a Dancer?"],
            )?;
            ctx.next()?;
            if ctx.menu(&[
                "Uses Dance skills. ",
                "Attacks from a distance.",
                "Uses Whips.",
                "Uses Two-handed swords.",
            ])? == 3
            {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as("Bijou", args!["4. Which town has", "the most Dancers?"])?;
            ctx.next()?;
            if ctx.menu(&["Al De Baran", "Juno", "Morocc", "Comodo"])? == 3 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as("Bijou", args!["5. Of the following,", "who dances most beautifully?"])?;
            ctx.next()?;
            let subject6 = Val::from(runtime::select_values(
                ctx,
                &[((Val::from("") + Val::from(ctx.player().name()?)) + Val::from(":Bijou:Isis:Guton Tak"))],
            )?);
            if subject6 == 1 {
                ctx.lines_as(
                    "Bijou",
                    args![
                        "...",
                        "That's...",
                        "^660000completely wrong^000000.",
                        "Didn't you see the",
                        "other choices?!",
                        "Minus points...!"
                    ],
                )?;
                l_da_score = l_da_score.clone().try_sub(Val::from(10))?;
                ctx.next()?;
            } else if subject6 == 2 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as(
                "Bijou",
                args!["6. What is the Dancer", "better at than the other", "job classes?"],
            )?;
            ctx.next()?;
            if ctx.menu(&["Health", "Acting ", "Dancing ", "Magic "])? == 2 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as("Bijou", args!["7. Who is the manager", "of the Comodo Casino?"])?;
            ctx.next()?;
            if ctx.menu(&["Ryu", "Moo", "Roul", "Hoon"])? == 1 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as("Bijou", args!["8. What item cannot", "be equipped by a Dancer?"])?;
            ctx.next()?;
            if ctx.menu(&["Kitty Band ", "Two-handed Sword", "Sandals", "Earring"])? == 1 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
            ctx.lines_as(
                "Bijou",
                args!["9. Do you think you", "can say this quiz is", "frustrating and annoying?"],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Yes:No")])?;
            ctx.var("@menu").set(choice)?;
            l_da_score = l_da_score.clone() + Val::from(10);
            ctx.lines_as("Bijou", args!["10. Which of the following", "is not a Jazz musician?"])?;
            ctx.next()?;
            if ctx.menu(&["Art Blakey", "Billie Holiday ", "Louis Armstrong ", "Bud Powell ", "Elder Willow "])? == 4 {
                l_da_score = l_da_score.clone() + Val::from(10);
            }
        }
        ctx.lines_as(
            "Bijou",
            args!["Good job~", "It seems like you", "answered all the", "questions~"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bijou",
            args![
                "Let's see...",
                "Your score is",
                ((Val::from("") + l_da_score.clone()) + Val::from(" points..."))
            ],
        )?;
        if l_da_score == 100 {
            ctx.var("danc_q").set(Val::from(7))?;
            ctx.lines(args!["Very well done!", "A perfect score!"])?;
            ctx.next()?;
            ctx.lines_as("Bijou", args!["There aren't too many people who apply for the Dancer job with this kind of knowledge. I'm sorry for judging you by your looks~"])?;
            ctx.next()?;
            ctx.lines_as(
                "Bijou",
                args![
                    "Whew~",
                    "Now you only have the Dance Test. While we prepare the test, why don't you rest a bit? Ho ho ho~"
                ],
            )?;
            return ctx.close();
        } else if l_da_score.number()? > 70 {
            ctx.var("danc_q").set(Val::from(7))?;
            ctx.mes("It wasn't perfect, but I'll let you pass.")?;
            return ctx.close();
        } else {
            ctx.var("danc_q").set(Val::from(6))?;
            ctx.mes("You.. You failed!")?;
            ctx.next()?;
            ctx.lines_as(
                "Bijou",
                args![
                    "Was it too hard?",
                    "When I was young, everyone knew at least enough to pass this test. Go and study some more before coming back, okay?"
                ],
            )?;
            return ctx.close();
        }
    } else if ctx.var("danc_q").get()? == 7 {
        ctx.lines_as(
            "Bijou",
            args![
                "Okay...",
                "Are you ready",
                "for the Dance Test?",
                "If you like, I can",
                "explain the instructions."
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Listen to instructions.", "Go to the testing area."])? == 0 {
            ctx.lines_as("Bijou", args!["First of all, each person gets ^CD68891 minute^000000 for the test, and everyone dances ^CD6889one at a time^000000. Don't worry if you've never danced before~"])?;
            ctx.next()?;
            ctx.lines_as("Bijou", args!["Once you enter the testing area, you will see the stage. First, ^CD6889change your camera angle so that it faces forward^000000. It will probably work if you ^CD6889double-click on the right mouse button^000000."])?;
            ctx.next()?;
            ctx.lines_as(
                "Bijou",
                args!["If you don't reset your camera angle, you may get the ^CD6889Up, Down, Left, Right^000000 commands confused."],
            )?;
            ctx.next()?;
            ctx.lines_as("Bijou", args!["Wait for your turn in the ^CD6889waiting room^000000. If the person in front of you fails, or if it's your turn in line, your test will begin."])?;
            ctx.next()?;
            ctx.lines_as("Bijou", args!["If there are a lot of people, not everyone might fit in the waiting room. If that's the case, just create an orderly line~"])?;
            ctx.next()?;
            ctx.lines_as("Bijou", args!["When the test begins, the music will be broadcast, as well as the direction in which you should move. Just follow the instructions and move your legs."])?;
            ctx.next()?;
            ctx.lines_as("Bijou", args!["Remember, ^CD6889you will be disqualified if you don't perform the steps with the right timing^000000. Be careful, the test is very strict~"])?;
            return ctx.close();
        }
        ctx.lines_as("Bijou", args!["Well then~", "Good luck...!!"])?;
        ctx.quests().change(7004, 7005)?;
        ctx.var("danc_q").set(Val::from(8))?;
        ctx.close_window()?;
        ctx.warp("job_duncer", 105, 109)?;
        return ctx.end();
    } else if ctx.var("danc_q").get()? == 8 {
        ctx.lines_as(
            "Bijou",
            args![
                "Oh my...",
                "Did you",
                "fail last time?",
                "Don't worry, just",
                "try to feel the rhythm~"
            ],
        )?;
        ctx.close_window()?;
        ctx.warp("job_duncer", 105, 109)?;
        return ctx.end();
    } else if ctx.var("danc_q").get()? == 9 {
        if ctx.var("SkillPoint").get()?.is_true() {
            ctx.lines_as(
                "Bijou",
                args![
                    "You can't change jobs",
                    "if you still have skill",
                    "points left. Use the rest",
                    "and come back later."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as("Bijou", args!["Oh my...", "I saw your", "dance earlier.", "You were great!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Bijou",
            args!["Your performance shows that you are than qualified to become a Dancer. Well then, let me change your job."],
        )?;
        ctx.next()?;
        ctx.lines_as("Bijou", args!["With the blessing of our goddess, you shall be reborn as a Dancer. From now on, no one will leave your presense without a smile~"])?;
        ctx.next()?;
        l_jlevel = ctx.var("JobLevel").get()?;
        ctx.mes("[Bijou]")?;
        ctx.quests().complete(7006)?;
        shared::other_global_functions::job_change(ctx, args![constants::JOB_DANCER])?;
        shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
        ctx.lines(args!["Ooh...!", "You look great", "as a Dancer~", "Congratulations!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Bijou",
            args![
                "Here's a small",
                "gift from me.",
                "Please take it.",
                "May your performances always bring joy to your audience~"
            ],
        )?;
        if l_jlevel == 50 {
            ctx.items().give(1953, 1)?;
        } else {
            ctx.items().give(1950, 1)?;
        }
        return ctx.close();
    }
    Ok(())
}

pub fn waiting_room_dance(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn waiting_room_dance_oninit(ctx: &Ctx) -> Script {
    ctx.call(
        Function::WaitingRoom,
        args!["Waiting Room", 20, "Waiting Room#dance::OnStartArena", 1],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    ctx.end()
}

pub fn waiting_room_dance_onstartarena(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("dance#up", false)?;
    ctx.set_npc_visible("dance#down", false)?;
    ctx.set_npc_visible("dance#left", false)?;
    ctx.set_npc_visible("dance#right", false)?;
    ctx.set_npc_visible("dance#cen", false)?;
    ctx.npc().do_event("dance#return::OnDisable")?;
    ctx.call(Function::WarpWaitingPc, args!["job_duncer", 69, 110, 1])?;
    ctx.npc().do_event("Bijou#dance_timer::OnEnable")?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    ctx.end()
}

pub fn waiting_room_dance_onenable(ctx: &Ctx) -> Script {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    ctx.end()
}

pub fn waiting_room_click(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Pyorgin",
        args!["Please wait in", "the waiting room.", "Click the Chatroom", "box to enter."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Pyorgin",
        args!["Also, those who", "are curious about", "the test can watch", "backstage."],
    )?;
    ctx.close()
}

#[derive(Clone, Copy, Debug)]
enum BijouDanceTimerStep {
    Start,
    OnEnable,
    OnDisable,
    OnTimer2000,
    OnTimer5000,
    OnTimer7000,
    OnTimer8000,
    OnTimer11000,
    OnTimer12000,
    OnTimer15000,
    OnTimer16000,
    OnTimer19000,
    OnTimer20000,
    OnTimer23000,
    OnTimer23500,
    OnTimer27000,
    OnTimer28500,
    OnTimer30000,
    OnTimer34000,
    OnTimer35000,
    OnTimer38500,
    OnTimer40000,
    OnTimer43000,
    OnTimer49000,
    OnTimer50000,
    OnTimer53000,
    OnTimer54000,
    OnTimer60000,
    OnTimer61000,
    OnTimer66000,
    OnTimer67000,
    OnTimer69000,
    OnTimer70000,
    OnTimer74000,
    OnTimer75000,
    OnTimer80000,
    OnTimer81000,
    OnTimer82000,
    OnTimer89000,
}

fn bijou_dance_timer_run(ctx: &Ctx, mut step: BijouDanceTimerStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BijouDanceTimerStep::Start => {
                step = BijouDanceTimerStep::OnEnable;
                continue 'machine;
            }
            BijouDanceTimerStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "job_duncer",
                        "Okay, let's begin. Now relax, the test is 1 minute~",
                        constants::BC_MAP
                    ],
                )?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer5000 => {
                ctx.call(Function::MapAnnounce, args!["job_duncer", " Up!", constants::BC_MAP])?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer7000 => {
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", true)?;
                ctx.set_npc_visible("dance#left", true)?;
                ctx.set_npc_visible("dance#right", true)?;
                ctx.set_npc_visible("dance#cen", true)?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer8000 => {
                ctx.npc().do_event("Backdancer#1::OnSmile")?;
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", false)?;
                ctx.set_npc_visible("dance#left", false)?;
                ctx.set_npc_visible("dance#right", false)?;
                ctx.set_npc_visible("dance#cen", false)?;
                ctx.call(Function::MapAnnounce, args!["job_duncer", " Down!", constants::BC_MAP])?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer11000 => {
                ctx.set_npc_visible("dance#up", true)?;
                ctx.set_npc_visible("dance#down", false)?;
                ctx.set_npc_visible("dance#left", true)?;
                ctx.set_npc_visible("dance#right", true)?;
                ctx.set_npc_visible("dance#cen", true)?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer12000 => {
                ctx.npc().do_event("Backdancer#1::OnSmile")?;
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", false)?;
                ctx.set_npc_visible("dance#left", false)?;
                ctx.set_npc_visible("dance#right", false)?;
                ctx.set_npc_visible("dance#cen", false)?;
                ctx.call(Function::MapAnnounce, args!["job_duncer", " Left~!", constants::BC_MAP])?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer15000 => {
                ctx.set_npc_visible("dance#up", true)?;
                ctx.set_npc_visible("dance#down", true)?;
                ctx.set_npc_visible("dance#left", false)?;
                ctx.set_npc_visible("dance#right", true)?;
                ctx.set_npc_visible("dance#cen", true)?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer16000 => {
                ctx.npc().do_event("Backdancer#1::OnSmile")?;
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", false)?;
                ctx.set_npc_visible("dance#left", false)?;
                ctx.set_npc_visible("dance#right", false)?;
                ctx.set_npc_visible("dance#cen", false)?;
                ctx.call(
                    Function::MapAnnounce,
                    args!["job_duncer", " Left, then Right~!", constants::BC_MAP],
                )?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer19000 => {
                ctx.set_npc_visible("dance#up", true)?;
                ctx.set_npc_visible("dance#down", true)?;
                ctx.set_npc_visible("dance#left", true)?;
                ctx.set_npc_visible("dance#right", false)?;
                ctx.set_npc_visible("dance#cen", true)?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer20000 => {
                ctx.npc().do_event("Backdancer#1::OnSmile")?;
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", false)?;
                ctx.set_npc_visible("dance#left", false)?;
                ctx.set_npc_visible("dance#right", false)?;
                ctx.set_npc_visible("dance#cen", false)?;
                ctx.call(
                    Function::MapAnnounce,
                    args!["job_duncer", " Back to the Center~ !", constants::BC_MAP],
                )?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer23000 => {
                ctx.set_npc_visible("dance#up", true)?;
                ctx.set_npc_visible("dance#down", true)?;
                ctx.set_npc_visible("dance#left", true)?;
                ctx.set_npc_visible("dance#right", true)?;
                ctx.set_npc_visible("dance#cen", false)?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer23500 => {
                ctx.npc().do_event("Backdancer#1::OnSmile")?;
                ctx.call(
                    Function::MapAnnounce,
                    args!["job_duncer", " Hold in place... ", constants::BC_MAP],
                )?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer27000 => {
                ctx.npc().do_event("Backdancer#1::OnSmile")?;
                ctx.call(
                    Function::MapAnnounce,
                    args!["job_duncer", " Hold then 'Improve Concentration!'", constants::BC_MAP],
                )?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer28500 => {
                ctx.call(
                    Function::MapAnnounce,
                    args!["job_duncer", " Pay attention! ", constants::BC_MAP],
                )?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer30000 => {
                ctx.npc().do_event("Backdancer#1::OnSmile")?;
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", false)?;
                ctx.set_npc_visible("dance#left", false)?;
                ctx.set_npc_visible("dance#right", false)?;
                ctx.set_npc_visible("dance#cen", false)?;
                ctx.call(Function::MapAnnounce, args!["job_duncer", " Left!", constants::BC_MAP])?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer34000 => {
                ctx.set_npc_visible("dance#up", true)?;
                ctx.set_npc_visible("dance#down", true)?;
                ctx.set_npc_visible("dance#left", false)?;
                ctx.set_npc_visible("dance#right", true)?;
                ctx.set_npc_visible("dance#cen", true)?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer35000 => {
                ctx.npc().do_event("Backdancer#1::OnSmile")?;
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", false)?;
                ctx.set_npc_visible("dance#left", false)?;
                ctx.set_npc_visible("dance#right", false)?;
                ctx.set_npc_visible("dance#cen", false)?;
                ctx.call(Function::MapAnnounce, args!["job_duncer", " Down!", constants::BC_MAP])?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer38500 => {
                ctx.call(
                    Function::MapAnnounce,
                    args!["job_duncer", " Down, then Right~ ", constants::BC_MAP],
                )?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer40000 => {
                ctx.set_npc_visible("dance#up", true)?;
                ctx.set_npc_visible("dance#down", true)?;
                ctx.set_npc_visible("dance#left", true)?;
                ctx.set_npc_visible("dance#right", false)?;
                ctx.set_npc_visible("dance#cen", true)?;
                ctx.call(Function::MapAnnounce, args!["job_duncer", " Hold it~", constants::BC_MAP])?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer43000 => {
                ctx.npc().do_event("Backdancer#1::OnSmile")?;
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", false)?;
                ctx.set_npc_visible("dance#left", false)?;
                ctx.set_npc_visible("dance#right", false)?;
                ctx.set_npc_visible("dance#cen", false)?;
                ctx.call(
                    Function::MapAnnounce,
                    args!["job_duncer", " Left, Center, Right, Up!", constants::BC_MAP],
                )?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer49000 => {
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", true)?;
                ctx.set_npc_visible("dance#left", true)?;
                ctx.set_npc_visible("dance#right", true)?;
                ctx.set_npc_visible("dance#cen", true)?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer50000 => {
                ctx.npc().do_event("Backdancer#1::OnSmile")?;
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", false)?;
                ctx.set_npc_visible("dance#left", false)?;
                ctx.set_npc_visible("dance#right", false)?;
                ctx.set_npc_visible("dance#cen", false)?;
                ctx.call(Function::MapAnnounce, args!["job_duncer", " Right!", constants::BC_MAP])?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer53000 => {
                ctx.set_npc_visible("dance#up", true)?;
                ctx.set_npc_visible("dance#down", true)?;
                ctx.set_npc_visible("dance#left", true)?;
                ctx.set_npc_visible("dance#right", false)?;
                ctx.set_npc_visible("dance#cen", true)?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer54000 => {
                ctx.npc().do_event("Backdancer#1::OnSmile")?;
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", false)?;
                ctx.set_npc_visible("dance#left", false)?;
                ctx.set_npc_visible("dance#right", false)?;
                ctx.set_npc_visible("dance#cen", false)?;
                ctx.call(
                    Function::MapAnnounce,
                    args!["job_duncer", " Left, Center, Down, Up~! ", constants::BC_MAP],
                )?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer60000 => {
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", true)?;
                ctx.set_npc_visible("dance#left", true)?;
                ctx.set_npc_visible("dance#right", true)?;
                ctx.set_npc_visible("dance#cen", true)?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer61000 => {
                ctx.npc().do_event("Backdancer#1::OnSmile")?;
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", false)?;
                ctx.set_npc_visible("dance#left", false)?;
                ctx.set_npc_visible("dance#right", false)?;
                ctx.set_npc_visible("dance#cen", false)?;
                ctx.call(
                    Function::MapAnnounce,
                    args!["job_duncer", " Once again~ Left, Center, Down, Up~ ! ", constants::BC_MAP],
                )?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer66000 => {
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", true)?;
                ctx.set_npc_visible("dance#left", true)?;
                ctx.set_npc_visible("dance#right", true)?;
                ctx.set_npc_visible("dance#cen", true)?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer67000 => {
                ctx.npc().do_event("Backdancer#1::OnSmile")?;
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", false)?;
                ctx.set_npc_visible("dance#left", false)?;
                ctx.set_npc_visible("dance#right", false)?;
                ctx.set_npc_visible("dance#cen", false)?;
                ctx.call(Function::MapAnnounce, args!["job_duncer", " Down~!", constants::BC_MAP])?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer69000 => {
                ctx.set_npc_visible("dance#up", true)?;
                ctx.set_npc_visible("dance#down", false)?;
                ctx.set_npc_visible("dance#left", true)?;
                ctx.set_npc_visible("dance#right", true)?;
                ctx.set_npc_visible("dance#cen", true)?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer70000 => {
                ctx.npc().do_event("Backdancer#1::OnSmile")?;
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", false)?;
                ctx.set_npc_visible("dance#left", false)?;
                ctx.set_npc_visible("dance#right", false)?;
                ctx.set_npc_visible("dance#cen", false)?;
                ctx.call(Function::MapAnnounce, args!["job_duncer", " Left!", constants::BC_MAP])?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer74000 => {
                ctx.set_npc_visible("dance#up", true)?;
                ctx.set_npc_visible("dance#down", true)?;
                ctx.set_npc_visible("dance#left", false)?;
                ctx.set_npc_visible("dance#right", true)?;
                ctx.set_npc_visible("dance#cen", true)?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer75000 => {
                ctx.npc().do_event("Backdancer#1::OnSmile")?;
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", false)?;
                ctx.set_npc_visible("dance#left", false)?;
                ctx.set_npc_visible("dance#right", false)?;
                ctx.set_npc_visible("dance#cen", false)?;
                ctx.call(Function::MapAnnounce, args!["job_duncer", " Center!", constants::BC_MAP])?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer80000 => {
                ctx.set_npc_visible("dance#up", true)?;
                ctx.set_npc_visible("dance#down", true)?;
                ctx.set_npc_visible("dance#left", true)?;
                ctx.set_npc_visible("dance#right", true)?;
                ctx.set_npc_visible("dance#cen", false)?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer81000 => {
                ctx.npc().do_event("Backdancer#1::OnSmile")?;
                ctx.set_npc_visible("dance#up", false)?;
                ctx.set_npc_visible("dance#down", false)?;
                ctx.set_npc_visible("dance#left", false)?;
                ctx.set_npc_visible("dance#right", false)?;
                ctx.set_npc_visible("dance#cen", false)?;
                ctx.call(
                    Function::MapAnnounce,
                    args!["job_duncer", " Okay, Finish~ 'Arrow Shower!'", constants::BC_MAP],
                )?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer82000 => {
                ctx.npc().do_event("dance#poring::OnEnable")?;
                return Err(Stop::End);
            }
            BijouDanceTimerStep::OnTimer89000 => {
                ctx.npc().do_event("dance#poring::OnDisable")?;
                ctx.npc().do_event("dance#return::OnEnable")?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn bijou_dance_timer(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::Start, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_onenable(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ondisable(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer2000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer5000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer7000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer8000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer11000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer11000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer12000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer12000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer15000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer15000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer16000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer16000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer19000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer19000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer20000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer20000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer23000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer23000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer23500(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer23500, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer27000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer27000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer28500(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer28500, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer30000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer34000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer34000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer35000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer35000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer38500(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer38500, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer40000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer40000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer43000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer43000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer49000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer49000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer50000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer50000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer53000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer53000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer54000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer54000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer60000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer61000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer61000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer66000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer66000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer67000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer67000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer69000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer69000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer70000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer70000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer74000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer74000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer75000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer75000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer80000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer80000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer81000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer81000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer82000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer82000, Vec::new()).map(|_| ())
}

pub fn bijou_dance_timer_ontimer89000(ctx: &Ctx) -> Script {
    bijou_dance_timer_run(ctx, BijouDanceTimerStep::OnTimer89000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DanceReturnStep {
    Start,
    OnTouch,
    OnDisable,
    OnEnable,
}

fn dance_return_run(ctx: &Ctx, mut step: DanceReturnStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DanceReturnStep::Start => {
                step = DanceReturnStep::OnTouch;
                continue 'machine;
            }
            DanceReturnStep::OnTouch => {
                ctx.call(
                    Function::MapAnnounce,
                    args!["job_duncer", "Good! Well done! Go back to Bijou!", constants::BC_MAP],
                )?;
                ctx.var("danc_q").set(Val::from(9))?;
                ctx.quests().change(7005, 7006)?;
                ctx.warp("comodo", 188, 162)?;
                return Err(Stop::End);
            }
            DanceReturnStep::OnDisable => {
                ctx.set_npc_visible("dance#return", false)?;
                ctx.npc().do_event("dance#return#2::OnDisable")?;
                ctx.npc().do_event("dance#return#3::OnDisable")?;
                return Err(Stop::End);
            }
            DanceReturnStep::OnEnable => {
                ctx.set_npc_visible("dance#return", true)?;
                ctx.npc().do_event("dance#return#2::OnEnable")?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dance_return(ctx: &Ctx) -> Script {
    dance_return_run(ctx, DanceReturnStep::Start, Vec::new()).map(|_| ())
}

pub fn dance_return_ontouch(ctx: &Ctx) -> Script {
    dance_return_run(ctx, DanceReturnStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn dance_return_ondisable(ctx: &Ctx) -> Script {
    dance_return_run(ctx, DanceReturnStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn dance_return_onenable(ctx: &Ctx) -> Script {
    dance_return_run(ctx, DanceReturnStep::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DanceReturn2Step {
    Start,
    OnTouch,
    OnDisable,
    OnEnable,
}

fn dance_return_2_run(ctx: &Ctx, mut step: DanceReturn2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DanceReturn2Step::Start => {
                step = DanceReturn2Step::OnTouch;
                continue 'machine;
            }
            DanceReturn2Step::OnTouch => {
                ctx.var("danc_q").set(Val::from(9))?;
                ctx.warp("comodo", 188, 162)?;
                return Err(Stop::End);
            }
            DanceReturn2Step::OnDisable => {
                ctx.set_npc_visible("dance#return#2", false)?;
                return Err(Stop::End);
            }
            DanceReturn2Step::OnEnable => {
                ctx.set_npc_visible("dance#return#2", true)?;
                ctx.npc().do_event("dance#return#3::OnEnable")?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dance_return_2(ctx: &Ctx) -> Script {
    dance_return_2_run(ctx, DanceReturn2Step::Start, Vec::new()).map(|_| ())
}

pub fn dance_return_2_ontouch(ctx: &Ctx) -> Script {
    dance_return_2_run(ctx, DanceReturn2Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn dance_return_2_ondisable(ctx: &Ctx) -> Script {
    dance_return_2_run(ctx, DanceReturn2Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn dance_return_2_onenable(ctx: &Ctx) -> Script {
    dance_return_2_run(ctx, DanceReturn2Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DanceReturn3Step {
    Start,
    OnTouch,
    OnDisable,
    OnEnable,
}

fn dance_return_3_run(ctx: &Ctx, mut step: DanceReturn3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DanceReturn3Step::Start => {
                step = DanceReturn3Step::OnTouch;
                continue 'machine;
            }
            DanceReturn3Step::OnTouch => {
                ctx.var("danc_q").set(Val::from(9))?;
                ctx.warp("comodo", 188, 162)?;
                return Err(Stop::End);
            }
            DanceReturn3Step::OnDisable => {
                ctx.set_npc_visible("dance#return#3", false)?;
                return Err(Stop::End);
            }
            DanceReturn3Step::OnEnable => {
                ctx.set_npc_visible("dance#return#3", true)?;
                ctx.npc().do_event("Bijou#dance_timer::OnDisable")?;
                ctx.npc().do_event("Waiting Room#dance::OnEnable")?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dance_return_3(ctx: &Ctx) -> Script {
    dance_return_3_run(ctx, DanceReturn3Step::Start, Vec::new()).map(|_| ())
}

pub fn dance_return_3_ontouch(ctx: &Ctx) -> Script {
    dance_return_3_run(ctx, DanceReturn3Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn dance_return_3_ondisable(ctx: &Ctx) -> Script {
    dance_return_3_run(ctx, DanceReturn3Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn dance_return_3_onenable(ctx: &Ctx) -> Script {
    dance_return_3_run(ctx, DanceReturn3Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DancestepStep {
    Start,
    OnTouch,
}

fn dancestep_run(ctx: &Ctx, mut step: DancestepStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DancestepStep::Start => {
                step = DancestepStep::OnTouch;
                continue 'machine;
            }
            DancestepStep::OnTouch => {
                ctx.npc().do_event("Backdancer#1::OnOmg")?;
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "job_duncer",
                        (Val::from(" ") + Val::from(ctx.player().name()?)) + Val::from(", you lack rhythm... Your timing was too late!"),
                        constants::BC_MAP
                    ],
                )?;
                ctx.var("danc_q").set(Val::from(8))?;
                ctx.npc().do_event("Bijou#dance_timer::OnDisable")?;
                ctx.npc().do_event("Waiting Room#dance::OnEnable")?;
                ctx.warp("comodo", 188, 162)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dancestep(ctx: &Ctx) -> Script {
    dancestep_run(ctx, DancestepStep::Start, Vec::new()).map(|_| ())
}

pub fn dancestep_ontouch(ctx: &Ctx) -> Script {
    dancestep_run(ctx, DancestepStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DancePoringStep {
    Start,
    OnEnable,
    OnMyMobDead,
    OnDisable,
}

fn dance_poring_run(ctx: &Ctx, mut step: DancePoringStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DancePoringStep::Start => {
                step = DancePoringStep::OnEnable;
                continue 'machine;
            }
            DancePoringStep::OnEnable => {
                ctx.call(
                    Function::AreaMonster,
                    args!["job_duncer", 68, 105, 70, 107, "Poring", 1002, 1, "dance#poring::OnMyMobDead"],
                )?;
                return Err(Stop::End);
            }
            DancePoringStep::OnMyMobDead => {
                ctx.call(
                    Function::MapAnnounce,
                    args!["job_duncer", " Good! Well done! ", constants::BC_MAP],
                )?;
                return Err(Stop::End);
            }
            DancePoringStep::OnDisable => {
                ctx.call(Function::KillMonster, args!["job_duncer", "All"])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dance_poring(ctx: &Ctx) -> Script {
    dance_poring_run(ctx, DancePoringStep::Start, Vec::new()).map(|_| ())
}

pub fn dance_poring_onenable(ctx: &Ctx) -> Script {
    dance_poring_run(ctx, DancePoringStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn dance_poring_onmymobdead(ctx: &Ctx) -> Script {
    dance_poring_run(ctx, DancePoringStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn dance_poring_ondisable(ctx: &Ctx) -> Script {
    dance_poring_run(ctx, DancePoringStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn backdancer_1(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn backdancer_1_onsmile(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_BEST)?;
    ctx.npc().do_event("Backdancer#2::OnSmile")?;
    ctx.npc().do_event("Backdancer#3::OnSmile")?;
    ctx.npc().do_event("Backdancer#4::OnSmile")?;
    ctx.end()
}

pub fn backdancer_1_onomg(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_HUK)?;
    ctx.npc().do_event("Backdancer#2::OnOmg")?;
    ctx.npc().do_event("Backdancer#3::OnOmg")?;
    ctx.npc().do_event("Backdancer#4::OnOmg")?;
    ctx.end()
}

pub fn backdancer_2(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn backdancer_2_onsmile(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_BEST)?;
    ctx.end()
}

pub fn backdancer_2_onomg(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_HUK)?;
    ctx.end()
}

pub fn backdancer_3(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn backdancer_3_onsmile(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_BEST)?;
    ctx.end()
}

pub fn backdancer_3_onomg(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_HUK)?;
    ctx.end()
}

pub fn backdancer_4(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn backdancer_4_onsmile(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_BEST)?;
    ctx.end()
}

pub fn backdancer_4_onomg(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_HUK)?;
    ctx.end()
}
