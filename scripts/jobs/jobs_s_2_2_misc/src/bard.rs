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
enum WanderingBardStep {
    Start,
    SChangeJob,
}

fn lyric_mismatch(ctx: &Ctx, expected: &str) -> Result<bool, Stop> {
    let (input, _) = runtime::input_text(ctx, None, None)?;
    Ok(input != expected)
}

fn wandering_bard_run(ctx: &Ctx, step: WanderingBardStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_bard_s = Val::from(0);
    let mut change_job_now = false;
    let mut l_w_point = 0;
    'machine: loop {
        match step {
            WanderingBardStep::Start => {
                if ctx.var("Upper").get()? == 1 {
                    ctx.lines_as(
                        "Lalo",
                        args![
                            "Chosen ones who are destined to become Gods",
                            "are so many in this era",
                            "but they never realise their fate while alive.",
                            "They end up to become ordinary men..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lalo",
                        args![
                            "Wind and Clouds, please send this message to them,",
                            "who pursue food, clothing, shelter and wealth.",
                            "Tell them they are wasting their time...",
                            "Tell them they forget the most important goal of the life..."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.fx().cutin("job_bard_aiolo01", 255)?;
                    return Err(Stop::End);
                } else if ctx.var("BaseJob").get()? != constants::JOB_ARCHER {
                    if ctx.var("BaseJob").get()? == constants::JOB_BARD {
                        ctx.lines_as(
                            "Lalo",
                            args!["Ooh hey! How's your singing these days?", "I wonder if your voice got any better."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Lalo",
                            args![
                                "You don't forget to spread good news in each town, right?",
                                "And don't forget to learn new songs, too..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Lalo",
                            args![
                                "Never forget to have a positive attitude and the meaning of joy.",
                                "Our songs are supposed to deliver happiness and joy to everyone."
                            ],
                        )?;
                    } else if ctx.var("Class").get()? == constants::JOB_NOVICE {
                        ctx.fx().cutin("job_bard_aiolo01", 2)?;
                        ctx.lines_as(
                            "Lalo",
                            args![
                                "The sadness that overcomes my heart.. ",
                                "It will not reside..",
                                "Is this the reason behind my troubles,",
                                "is this why I am weak,",
                                "This must be why I cannot seem to forget you..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Lalo",
                            args![
                                "Oh, sorry. I didn't see you because I was concentrating on writing some lyrics.",
                                "Do you want to listen to my songs? Shall I sing a song for you?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Lalo",
                            args!["Heh... try asking someone else.", "I'm trying to compose a new song."],
                        )?;
                    } else {
                        ctx.fx().cutin("job_bard_aiolo01", 2)?;
                        ctx.lines_as(
                            "Lalo",
                            args!["Lalala, lalala. Beautiful Comodo.", "Always full of happy moments~"],
                        )?;
                        ctx.next()?;
                        ctx.mes("[Lalo]")?;
                        if ctx.var("Sex").get()? == constants::SEX_MALE {
                            ctx.lines(args!["Forget about your worries~", "And enjoy everything~"])?;
                        } else {
                            ctx.mes("Cute lady, shall we dance~")?;
                        }
                        ctx.mes("Youth never repeats itself~")?;
                    }
                    ctx.close_window()?;
                    ctx.fx().cutin("job_bard_aiolo01", 255)?;
                    return Err(Stop::End);
                } else if ctx.var("bard_q").get()? == 0 {
                    ctx.fx().cutin("job_bard_aiolo01", 2)?;
                    ctx.mes("[Lalo]")?;
                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                        ctx.mes("Hi! Delightful Archer.")?;
                    } else {
                        ctx.mes("Hello! Beautiful Archer Lady.")?;
                    }
                    ctx.mes("How can a wanderer like me help you?")?;
                    ctx.next()?;
                    'b1: {
                        let subject1 = ctx.menu(&["You have a nice voice.", "Could you sing for me, please?", "Nothing."])?;
                        let mut matched1 = false;
                        if !matched1 && subject1 == 0 {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.lines_as(
                                "Lalo",
                                args![
                                    "Hahaha! Of course!",
                                    "if you sing with a happy heart, your voice always gets better."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Lalo",
                                args![
                                    "But, to Bards your voice is your life.",
                                    "Sometimes your voice will go, but you must be careful."
                                ],
                            )?;
                            break 'b1;
                        }
                        if !matched1 && subject1 == 1 {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.lines_as("Lalo", args!["A song... let's see.", "Ok, I got one..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Lalo",
                                args!["I'll sing.. Drums of War.", "*ehem...*cough...gag..mememememe...", "1, 2, 3, 4..."],
                            )?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^000088The sound of horses galloping over the horizon",
                                "The dust that covers the distant sun",
                                "When thousands of eyes open in the night sky",
                                "The castle's fire will burn with power.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^000088I can hear.. the beating of my heart.",
                                "I can feel.. the blood rushing through my veins.",
                                ".. and the weight of my armor.",
                                "I can see.. my enemies.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^000088Louder, louder louder..",
                                "Give strength to the warriors!",
                                "Higher, higher, higher..",
                                "This day will never come again!^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^000088Shake the sky and roar through the land.",
                                "Make my heart pound again!",
                                "Let the trumpets sound, and castle walls ring.",
                                "This moment will never come again!^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Lalo",
                                args![
                                    "Hmm... that's always a good song to sing.",
                                    "How was it? Don't you think it's a nice song?"
                                ],
                            )?;
                            ctx.next()?;
                            if ctx.menu(&["Yes, it was very nice.", "No, not really..."])? == 0 {
                                ctx.lines_as("Lalo", args!["Thanks! if you enjoyed my song, it makes me happy, too."])?;
                                ctx.next()?;
                                if ctx.var("Sex").get()? == constants::SEX_MALE && ctx.player().job_level()? > 39 {
                                    ctx.lines_as(
                                        "Lalo",
                                        args![
                                            "It would be nice if more people went around and sang...",
                                            "Well, it's quite ok as it is now... hmmhmm."
                                        ],
                                    )?;
                                    ctx.var("bard_q").set(Val::from(1))?;
                                    ctx.quests().start(3000)?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("job_bard_aiolo01", 255)?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as("Lalo", args!["if you ever want to hear my song again, just ask."])?;
                                ctx.close_window()?;
                                ctx.fx().cutin("job_bard_aiolo01", 255)?;
                                return Err(Stop::End);
                            }
                            ctx.fx().cutin("job_bard_aiolo02", 2)?;
                            ctx.lines_as(
                                "Lalo",
                                args![
                                    "Hmm... Did I lose my senses, I'll have to try harder.",
                                    "Anyways.. Thanks for listening."
                                ],
                            )?;
                            break 'b1;
                        }
                        if !matched1 && subject1 == 2 {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.fx().cutin("job_bard_aiolo02", 2)?;
                            ctx.lines_as(
                                "Lalo",
                                args![
                                    "Oy, not requesting a song when you run into a Bard isn't very polite.",
                                    "Well... can't help it since you look like you're in a hurry anyways."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Lalo",
                                args![
                                    "Hunting is good... but you can't forget to relax once in a while.",
                                    "Youth is short and won't come again once it passes by.."
                                ],
                            )?;
                            break 'b1;
                        }
                    }
                    ctx.close_window()?;
                    ctx.fx().cutin("job_bard_aiolo02", 255)?;
                    return Err(Stop::End);
                } else if ctx.var("bard_q").get()? == 1 {
                    ctx.fx().cutin("job_bard_aiolo01", 2)?;
                    ctx.lines_as(
                        "Lalo",
                        args!["Hey there Archer fellow.", "How can a wanderer like me help you?"],
                    )?;
                    ctx.next()?;
                    'b2: {
                        let subject2 = ctx.menu(&["You have a nice voice.", "Could you sing for me, please?", "Nothing."])?;
                        let mut matched2 = false;
                        if !matched2 && subject2 == 0 {
                            matched2 = true;
                        }
                        if matched2 {
                            ctx.lines_as(
                                "Lalo",
                                args!["Hoho, your voice is rather nice as well?", "Ever think about singing?"],
                            )?;
                            ctx.next()?;
                            if ctx.menu(&["Of course!", "I can't quite possibly..."])? == 0 {
                                ctx.lines_as(
                                    "Lalo",
                                    args![
                                        "Haha, nice attitude. You have to be like that to become a Bard.",
                                        "I'll help you become a Bard then."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lalo",
                                    args![
                                        "But before that... do you think you can bring me a Flower?",
                                        "I need to smell the scent of a Flower to feel like teaching."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lalo",
                                    args![
                                        "It doesn't really matter which Flower, but try to bring one that I like.",
                                        "And don't just buy any random Flower, ok?"
                                    ],
                                )?;
                                ctx.var("bard_q").set(Val::from(2))?;
                                ctx.quests().change(3000, 3001)?;
                                ctx.close_window()?;
                                ctx.fx().cutin("job_bard_aiolo01", 255)?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as("Lalo", args!["Haha, what a timid one.", "Don't think so little of yourself."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Lalo",
                                args!["You have plenty of talent.", "Come again if you change your mind."],
                            )?;
                            break 'b2;
                        }
                        if !matched2 && subject2 == 1 {
                            matched2 = true;
                        }
                        if matched2 {
                            ctx.lines_as(
                                "Lalo",
                                args![
                                    "Hmm... seems like you have some singing talents?",
                                    "Don't just request songs.. singing to others is quite fun, too."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Lalo",
                                args![
                                    "Try enjoying your life as a Bard.",
                                    "You go from town to town, singing to the people. Doesn't it sound great?"
                                ],
                            )?;
                            break 'b2;
                        }
                        if !matched2 && subject2 == 2 {
                            matched2 = true;
                        }
                        if matched2 {
                            ctx.lines_as(
                                "Lalo",
                                args!["Hmm... I'm not sure what's what, but enjoy life.", "You look too uptight."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Lalo", args!["Well then~ Have a great time~"])?;
                            break 'b2;
                        }
                    }
                    ctx.close_window()?;
                    ctx.fx().cutin("job_bard_aiolo01", 255)?;
                    return Err(Stop::End);
                } else if ctx.var("bard_q").get()? == 2 {
                    ctx.fx().cutin("job_bard_aiolo01", 2)?;
                    ctx.lines_as("Lalo", args!["Welcome! Archer friend.", "Did you bring a Flower? Let me see."])?;
                    ctx.next()?;
                    ctx.mes("[Lalo]")?;
                    if ctx.items().count(629)? > 0 {
                        ctx.lines(args!["Ooh! It's a Singing Flower!", "It's full of my memories..."])?;
                        ctx.next()?;
                        ctx.items().take(629, 1)?;
                        ctx.lines_as(
                            "Lalo",
                            args!["My friend Tchaikovsky used to like it.", "I wonder what he's doing now..."],
                        )?;
                    } else if ctx.items().count(703)? > 0 {
                        ctx.lines(args![
                            "Aah... the cute Hinelle...",
                            "It doesn't have a scent but it's a very moderate cute flower."
                        ])?;
                        ctx.next()?;
                        ctx.items().take(703, 1)?;
                        ctx.lines_as(
                            "Lalo",
                            args![
                                "The leaves gave me strength when I used to fall.",
                                "I really like this flower, thank you."
                            ],
                        )?;
                    } else if ctx.items().count(704)? > 0 {
                        ctx.lines(args!["Aloe... This is a rare flower.", "How'd you get it? Rather skilled, eh?"])?;
                        ctx.next()?;
                        ctx.items().take(704, 1)?;
                        ctx.lines_as(
                            "Lalo",
                            args![
                                "The leaves are good and Aloe Vera is delicious, too..",
                                "but it's defnitely the most beautiful when it's a flower."
                            ],
                        )?;
                    } else if ctx.items().count(708)? > 0 {
                        ctx.lines(args![
                            "Ment... You can forget about all your hardships with one of these.",
                            "Nice to see it in such a long time!"
                        ])?;
                        ctx.next()?;
                        ctx.items().take(708, 1)?;
                        ctx.lines_as(
                            "Lalo",
                            args![
                                "I heard you can make Anodyne with it...",
                                "But that would be a slight waste.. thanks!"
                            ],
                        )?;
                    } else if ctx.items().count(709)? > 0 {
                        ctx.lines(args!["Ooh, isn't this an Izidor?", "It's a dangerous yet beautiful flower..."])?;
                        ctx.next()?;
                        ctx.items().take(709, 1)?;
                        ctx.lines_as(
                            "Lalo",
                            args!["The deep purple charms a person.. ", "Thank you, I really like this flower."],
                        )?;
                    } else if ctx.items().count(748)? > 0 {
                        ctx.lines(args![
                            "Ooh, a Witherless Rose. The strong flower that doesn't wither.",
                            "Great to give to a girlfriend."
                        ])?;
                        ctx.next()?;
                        ctx.items().take(748, 1)?;
                        ctx.lines_as(
                            "Lalo",
                            args![
                                "I wonder if it would be ok for a wanderer like me to accept it.",
                                "Haha, it should be ok.. right?"
                            ],
                        )?;
                    } else if ctx.items().count(749)? > 0 {
                        ctx.lines(args![
                            "Frozen Rose... you can't really call this a flower,",
                            "But it is still beautiful... a clear Rose."
                        ])?;
                        ctx.next()?;
                        ctx.items().take(749, 1)?;
                        ctx.lines_as(
                            "Lalo",
                            args![
                                "You can call it a flower even though it doesn't have a scent anymore.",
                                "Then I'll greatly take this."
                            ],
                        )?;
                    } else if ctx.items().count(710)? > 0 {
                        ctx.lines(args![
                            "Oh, isn't this an Illusion Flower!?",
                            "Wow, how did you obtain such a rare flower!!"
                        ])?;
                        ctx.next()?;
                        ctx.items().take(710, 1)?;
                        ctx.lines_as(
                            "Lalo",
                            args![
                                "Than you very much, aah... I feel like heaven is in front of my eyes.",
                                "What a wonderful feeling! I'm really happy!"
                            ],
                        )?;
                    } else if ctx.items().count(712)? > 0 {
                        ctx.fx().cutin("job_bard_aiolo02", 2)?;
                        ctx.lines(args!["Eh? This is just a normal flower.", "I like it... but it's not enough."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Lalo",
                            args![
                                "You can get this flower from the girl in Prontera.",
                                "Please bring me a different flower."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.fx().cutin("job_bard_aiolo02", 255)?;
                        return Err(Stop::End);
                    } else if ctx.items().count(744)? > 0 {
                        ctx.lines(args![
                            "Oh no, you brought a Bouquet?",
                            "You can't bring me something like this."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Lalo",
                            args![
                                "Go give this to a graduating Sage or something.",
                                "Since it's great as that kind of gift... Bring a different flower."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.fx().cutin("job_bard_aiolo01", 255)?;
                        return Err(Stop::End);
                    } else if ctx.items().count(745)? > 0 {
                        ctx.fx().cutin("job_bard_aiolo02", 2)?;
                        ctx.lines(args![
                            "Oy oy... did you go to a wedding or something?",
                            "What do you expect a guy to do with a Wedding Bouquet?"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Lalo",
                            args![
                                "It's not me. Go give it to a lady or something.",
                                "This isn't the type of flower I wanted."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.fx().cutin("job_bard_aiolo02", 255)?;
                        return Err(Stop::End);
                    } else if ctx.items().count(2207)? > 0 {
                        ctx.lines(args!["Mmm... a Fancy Flower.", "It's nice... but this isn't good enough."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Lalo",
                            args![
                                "I like flowers that have a scent and are beautiful.",
                                "I don't like fake flowers that go on top of heads."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.fx().cutin("job_bard_aiolo01", 255)?;
                        return Err(Stop::End);
                    } else if ctx.items().count(1032)? > 0 {
                        ctx.fx().cutin("job_bard_aiolo02", 2)?;
                        ctx.lines(args![
                            "...Agh, why'd you bring such a hideous thing?",
                            "Are you thinking at all?"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Lalo",
                            args![
                                "if you were trying to be funny, it was a good attempt...",
                                "but bring a normal flower now."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.fx().cutin("job_bard_aiolo02", 255)?;
                        return Err(Stop::End);
                    } else {
                        ctx.fx().cutin("job_bard_aiolo02", 2)?;
                        ctx.lines(args![
                            "Hmm? What... you didnt' bring anything.",
                            "Didn't I ask you to bring a flower?"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Lalo",
                            args![
                                "Well... if you want to learn on your own, then so be it.",
                                "Anyone can just go out and sing."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.fx().cutin("job_bard_aiolo02", 255)?;
                        return Err(Stop::End);
                    }
                    ctx.next()?;
                    ctx.fx().cutin("job_bard_aiolo01", 2)?;
                    ctx.lines_as(
                        "Lalo",
                        args!["As I promised, I'll help you become a Bard.", "But it's not easy my friend. Haha!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lalo",
                        args![
                            "It is important to get to know a lot of people to learn how to sing.",
                            "You must also keep up with all the things going on in different villages..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lalo",
                        args![
                            "There's a talking snowman in a town called Lutie.",
                            "Go there and bring back a present."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("bard_q").set(Val::from(3))?;
                    ctx.quests().change(3001, 3002)?;
                    ctx.var("xmas_npc").set(Val::from(1))?;
                    ctx.lines_as(
                        "Lalo",
                        args![
                            "if you become friends with ^008800Jack Frost^000000, you will receive something.",
                            "And also talk to the townspeople while you're at it..."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.fx().cutin("job_bard_aiolo01", 255)?;
                    return Err(Stop::End);
                } else if ctx.var("bard_q").get()?.number()? >= 3 || ctx.var("bard_q").get()?.number()? <= 5 {
                    if ctx.var("bard_q").get()? == 3 {
                        if ctx.var("xmas_npc").get()?.number()? > 10 {
                            ctx.fx().cutin("job_bard_aiolo01", 2)?;
                            ctx.lines_as(
                                "Lalo",
                                args![
                                    "How was the trip? Did you meet a lot of people?",
                                    "You should have been able to learn something more important than a gift."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Lalo",
                                args![
                                    "Then, do you want to try singing...?",
                                    "I'll sing a short melody...",
                                    "and you try after."
                                ],
                            )?;
                            if ctx.call(Function::CheckQuest, args![3003])? == -1 {
                                ctx.quests().change(3002, 3003)?;
                            }
                            ctx.next()?;
                            ctx.lines_as("Lalo", args!["Here I go.", "Ehem *clears throat*", "1, 2, 3, 4"])?;
                            ctx.next()?;
                        } else {
                            ctx.fx().cutin("job_bard_aiolo01", 2)?;
                            ctx.var("xmas_npc").set(Val::from(1))?;
                            ctx.lines_as(
                                "Lalo",
                                args!["Eh, you still haven't become his friend?", "Talking will not be enough."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Lalo",
                                args![
                                    "if you become friends with ^008800Jack Frost^000000, you will receive something.",
                                    "And talk with the village people, too..."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.fx().cutin("job_bard_aiolo01", 255)?;
                            return Err(Stop::End);
                        }
                    } else if ctx.var("bard_q").get()? == 4 {
                        ctx.fx().cutin("job_bard_aiolo01", 2)?;
                        ctx.lines_as(
                            "Lalo",
                            args!["Hmm... this time you can do better, right?", "Let's try again, you can do it."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Lalo", args!["I'll sing one part...", "and you try it after."])?;
                        ctx.next()?;
                        ctx.lines_as("Lalo", args!["Here we go.", "*Ehem*", "1, 2, 3, 4"])?;
                        ctx.next()?;
                    }
                    if ctx.var("bard_q").get()? != 5 {
                        l_bard_s = ctx.call(Function::Rand, args![1, 5])?;
                        if l_bard_s == 1 {
                            ctx.lines(args![
                                "^3377FFThere was a man^000000",
                                "who was said to be immortal.",
                                "His name Jichfreid,",
                                "Son of the hero Jichmunt.",
                                "The evil giant Papner,",
                                "Turned into a dragon and ate him."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "There was a man")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFThere was a man",
                                "who was said to be immortal.^000000",
                                "His name Jichfreid,",
                                "Son of the hero Jichmunt.",
                                "The evil giant Papner,",
                                "Turned into a dragon and ate him."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "who was said to be immortal.")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFThere was a man",
                                "who was said to be immortal.",
                                "His name Jichfreid,^000000",
                                "Son of the hero Jichmunt.",
                                "The evil giant Papner,",
                                "Turned into a dragon and ate him."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "His name Jichfreid,")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFThere was a man",
                                "who was said to be immortal.",
                                "His name Jichfreid,",
                                "Son of the hero Jichmunt.^000000",
                                "The evil giant Papner,",
                                "Turned into a dragon and ate him."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "Son of the hero Jichmunt.")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFThere was a man",
                                "who was said to be immortal.",
                                "His name Jichfreid,",
                                "Son of the hero Jichmunt.",
                                "The evil giant Papner,^000000",
                                "Turned into a dragon and ate him."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "The evil giant Papner,")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFThere was a man",
                                "who was said to be immortal.",
                                "His name Jichfreid,",
                                "Son of the hero Jichmunt.",
                                "The evil giant Papner,",
                                "Turned into a dragon and ate him.^000000"
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "Turned into a dragon and ate him.")? {
                                l_w_point += 1;
                            }
                        } else if l_bard_s == 2 {
                            ctx.lines(args![
                                "^3377FFA Merchant without money or equipment,^000000",
                                "a Merchant that couldn't sell anything.",
                                "But he was too proud to beg.",
                                "So he gathered some money selling items.",
                                "At first he only sold Red Potions.",
                                "Some say he sold Sweet Potatoes, too."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "A Merchant without money or equipment,")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFA Merchant without money or equipment,",
                                "a Merchant that couldn't sell anything.^000000",
                                "But he was too proud to beg.",
                                "So he gathered some money selling items.",
                                "At first he only sold Red Potions.",
                                "Some say he sold Sweet Potatoes, too."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "a Merchant that couldn't sell anything.")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFA Merchant without money or equipment,",
                                "a Merchant that couldn't sell anything.",
                                "But he was too proud to beg.^000000",
                                "So he gathered some money selling items.",
                                "At first he only sold Red Potions.",
                                "Some say he sold Sweet Potatoes, too."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "But he was too proud to beg.")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFA Merchant without money or equipment,",
                                "a Merchant that couldn't sell anything.",
                                "But he was too proud to beg.",
                                "So he gathered some money selling items.^000000",
                                "At first he only sold Red Potions.",
                                "Some say he sold Sweet Potatoes, too."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "So he gathered some money selling items.")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFA Merchant without money or equipment,",
                                "a Merchant that couldn't sell anything.",
                                "But he was too proud to beg.",
                                "So he gathered some money selling items.",
                                "At first he only sold Red Potions.^000000",
                                "Some say he sold Sweet Potatoes, too."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "At first he only sold Red Potions.")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFA Merchant without money or equipment,",
                                "a Merchant that couldn't sell anything.",
                                "But he was too proud to beg.",
                                "So he gathered some money selling items.",
                                "At first he only sold Red Potions.",
                                "Some say he sold Sweet Potatoes, too.^000000"
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "Some say he sold Sweet Potatoes, too.")? {
                                l_w_point += 1;
                            }
                        } else if l_bard_s == 3 {
                            ctx.lines(args![
                                "^3377FFAll Gods never age.^000000",
                                "The ever so Beautiful Goddess Eden,",
                                "Beautiful and graceful Goddess Eden,",
                                "Odin's daughter-in-law and Bragi's wife.",
                                "Her sweet apples in her basket,",
                                "All thanks to her sweet apples."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "All Gods never age.")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFAll Gods never age.",
                                "The ever so Beautiful Goddess Eden,^000000",
                                "Beautiful and graceful Goddess Eden,",
                                "Odin's daughter-in-law and Bragi's wife.",
                                "Her sweet apples in her basket,",
                                "All thanks to her sweet apples."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "The ever so Beautiful Goddess Eden,")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFAll Gods never age.",
                                "The ever so Beautiful Goddess Eden,",
                                "Beautiful and graceful Goddess Eden,^000000",
                                "Odin's daughter-in-law and Bragi's wife.",
                                "Her sweet apples in her basket,",
                                "All thanks to her sweet apples."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "Beautiful and graceful Goddess Eden,")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFAll Gods never age.",
                                "The ever so Beautiful Goddess Eden,",
                                "Beautiful and graceful Goddess Eden,",
                                "Odin's daughter-in-law and Bragi's wife.^000000",
                                "Her sweet apples in her basket,",
                                "All thanks to her sweet apples."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "Odin's daughter-in-law and Bragi's wife.")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFAll Gods never age.",
                                "The ever so Beautiful Goddess Eden,",
                                "Beautiful and graceful Goddess Eden,",
                                "Odin's daughter-in-law and Bragi's wife.",
                                "Her sweet apples in her basket,^000000",
                                "All thanks to her sweet apples."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "Her sweet apples in her basket,")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFAll Gods never age.",
                                "The ever so Beautiful Goddess Eden,",
                                "Beautiful and graceful Goddess Eden,",
                                "Odin's daughter-in-law and Bragi's wife.",
                                "Her sweet apples in her basket,",
                                "All thanks to her sweet apples.^000000"
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "All thanks to her sweet apples.")? {
                                l_w_point += 1;
                            }
                        } else if l_bard_s == 4 {
                            ctx.lines(args![
                                "^3377FFBragi, Bragi,^000000",
                                "Forever call the poets name.",
                                "My songs are his breath,",
                                "My mind is his will,",
                                "All wandering poets are his people,",
                                "And all praise shall go to him."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "Bragi, Bragi,")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFBragi, Bragi,",
                                "Forever call the poets name.^000000",
                                "My songs are his breath,",
                                "My mind is his will,",
                                "All wandering poets are his people,",
                                "And all praise shall go to him."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "Forever call the poets name.")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFBragi, Bragi,",
                                "Forever call the poets name.",
                                "My songs are his breath,^000000",
                                "My mind is his will,",
                                "All wandering poets are his people,",
                                "And all praise shall go to him."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "My songs are his breath,")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFBragi, Bragi,",
                                "Forever call the poets name.",
                                "My songs are his breath,",
                                "My mind is his will,^000000",
                                "All wandering poets are his people,",
                                "And all praise shall go to him."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "My mind is his will,")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFBragi, Bragi,",
                                "Forever call the poets name.",
                                "My songs are his breath,",
                                "My mind is his will,",
                                "All wandering poets are his people,^000000",
                                "And all praise shall go to him."
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "All wandering poets are his people,")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFBragi, Bragi,",
                                "Forever call the poets name.",
                                "My songs are his breath,",
                                "My mind is his will,",
                                "All wandering poets are his people,",
                                "And all praise shall go to him.^000000"
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "And all praise shall go to him.")? {
                                l_w_point += 1;
                            }
                        } else {
                            ctx.lines(args![
                                "^3377FFLouder, louder, louder.^000000",
                                "Give strength to the warriors!",
                                "Shake the sky and roar through the land.",
                                "Make my heart pound again!",
                                "Let the castle walls ring.",
                                "This day will never come again!"
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "Louder, louder, louder.")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFLouder, louder, louder.",
                                "Give strength to the warriors!^000000",
                                "Shake the sky and roar through the land.",
                                "Make my heart pound again!",
                                "Let the castle walls ring.",
                                "This day will never come again!"
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "Give strength to the warriors!")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFLouder, louder, louder.",
                                "Give strength to the warriors!",
                                "Shake the sky and roar through the land.^000000",
                                "Make my heart pound again!",
                                "Let the castle walls ring.",
                                "This day will never come again!"
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "Shake the sky and roar through the land.")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFLouder, louder, louder.",
                                "Give strength to the warriors!",
                                "Shake the sky and roar through the land.",
                                "Make my heart pound again!^000000",
                                "Let the castle walls ring.",
                                "This day will never come again!"
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "Make my heart pound again!")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFLouder, louder, louder.",
                                "Give strength to the warriors!",
                                "Shake the sky and roar through the land.",
                                "Make my heart pound again!",
                                "Let the castle walls ring.^000000",
                                "This day will never come again!"
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "Let the castle walls ring.")? {
                                l_w_point += 1;
                            }
                            ctx.lines(args![
                                "^3377FFLouder, louder, louder.",
                                "Give strength to the warriors!",
                                "Shake the sky and roar through the land.",
                                "Make my heart pound again!",
                                "Let the castle walls ring.",
                                "This day will never come again!^000000"
                            ])?;
                            ctx.next()?;
                            if lyric_mismatch(ctx, "This day will never come again!")? {
                                l_w_point += 1;
                            }
                        }
                        if l_w_point != 0 {
                            ctx.fx().cutin("job_bard_aiolo02", 2)?;
                            ctx.lines_as("Lalo", args!["Oy, You got the lyrics wrong!", "Can't you even sing along..?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Lalo",
                                args!["Your pronunciation is very unclear.", "Do a better job next time."],
                            )?;
                            ctx.close_window()?;
                            ctx.fx().cutin("job_bard_aiolo02", 255)?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Lalo", args![".........."])?;
                        ctx.next()?;
                        ctx.var("bard_q").set(Val::from(5))?;
                        ctx.lines_as(
                            "Lalo",
                            args!["Wonderful! Finished it in one try!", "You can become a great Bard. "],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Lalo",
                            args!["Mmm... So you will not become a Bard.", "But I want to give you a souvenir..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Lalo",
                            args!["Do you want to just change jobs now?", "Or do you want a present."],
                        )?;
                        ctx.next()?;
                        change_job_now = ctx.menu(&["Just change my job please.", "I'd be thankful for a present."])? == 0;
                    }
                    if change_job_now || ctx.var("bard_q").get()? == 5 {
                        if ctx.var("SkillPoint").get()?.is_true() {
                            ctx.fx().cutin("job_bard_aiolo01", 2)?;
                            ctx.lines_as(
                                "Lalo",
                                args![
                                    "Ah... Everything is good, but you still have some skill points left.",
                                    "Go learn the rest of the skills and come back."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Lalo",
                                args![
                                    "And I am going to give you a small present...",
                                    "So bring some trunks.",
                                    "It doesn't matter what kind, as long as they are 60 of the same kind..."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.fx().cutin("job_bard_aiolo01", 255)?;
                            return Err(Stop::End);
                        } else {
                            'b3: {
                                let subject3 = ctx.var("bard_q").get()?;
                                let mut matched3 = false;
                                let no_case3 = subject3 != 5;
                                if !matched3 && subject3 == 5 {
                                    matched3 = true;
                                }
                                if matched3 {
                                    if ctx.items().count(1019)? > 59 {
                                        wandering_bard_run(ctx, WanderingBardStep::SChangeJob, args![1019, 1901])?;
                                    } else if ctx.items().count(1068)? > 59 {
                                        wandering_bard_run(ctx, WanderingBardStep::SChangeJob, args![1068, 1903])?;
                                    } else if ctx.items().count(1067)? > 59 {
                                        wandering_bard_run(ctx, WanderingBardStep::SChangeJob, args![1067, 1903])?;
                                    } else if ctx.items().count(1066)? > 59 {
                                        if ctx.player().job_level()? > 49 {
                                            wandering_bard_run(ctx, WanderingBardStep::SChangeJob, args![1066, 1910])?;
                                        } else {
                                            wandering_bard_run(ctx, WanderingBardStep::SChangeJob, args![1066, 1905])?;
                                        }
                                    }
                                    ctx.fx().cutin("job_bard_aiolo01", 2)?;
                                    ctx.lines_as(
                                        "Lalo",
                                        args![
                                            "Mmm? Seems like you haven't prepared all trunks the yet? ",
                                            "Do you want to just change jobs anyways?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    if ctx.menu(&["Yes, just change my job already.", "No, I'll go prepare them."])? == 1 {
                                        break 'b3;
                                    }
                                }
                                if !matched3 && no_case3 {
                                    matched3 = true;
                                }
                                if matched3 {
                                    ctx.quests().complete(3003)?;
                                    shared::other_global_functions::job_change(ctx, args![constants::JOB_BARD])?;
                                    shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
                                    ctx.lines_as(
                                        "Lalo",
                                        args![
                                            "Very well! Hope you sing happy enjoyable songs.",
                                            "Live like the wind and the clouds!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Lalo", args!["See you again next time!"])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("job_bard_aiolo01", 255)?;
                                    return Err(Stop::End);
                                }
                            }
                        }
                    }
                    ctx.quests().change(3003, 3004)?;
                    ctx.lines_as(
                        "Lalo",
                        args![
                            "Hmm... very well, bring some trunks.",
                            "It doesn't matter what kind, as long as they are 60 of the same kind..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lalo",
                        args!["I will give you a gift once you bring them.", "Have a safe trip."],
                    )?;
                    ctx.close_window()?;
                    ctx.fx().cutin("job_bard_aiolo01", 255)?;
                    return Err(Stop::End);
                }
                ctx.fx().cutin("job_bard_aiolo01", 2)?;
                ctx.lines_as("Lalo", args!["Whee~ whee~ whee~"])?;
                ctx.close_window()?;
                ctx.fx().cutin("job_bard_aiolo01", 255)?;
                return Err(Stop::End);
            }
            WanderingBardStep::SChangeJob => {
                ctx.quests().complete(3004)?;
                shared::other_global_functions::job_change(ctx, args![constants::JOB_BARD])?;
                shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
                ctx.lines_as(
                    "Lalo",
                    args!["Good job. I will make you a job change souvenir with this.", "Wait just a moment."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lalo",
                    args![
                        "^3355FFScrape Scrape Tang Tang^000000",
                        "^3355FFSqueak Squeak Scratch Scratch^000000"
                    ],
                )?;
                ctx.call(Function::DelItem, vec![runtime::arg(&args, 0, Val::from(0)), Val::from(60)])?;
                ctx.call(Function::GetItem, vec![runtime::arg(&args, 1, Val::from(0)), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Lalo",
                    args!["Here you go, a souvenir. It is useful when you sing.", "Hope you sing happy songs."],
                )?;
                ctx.next()?;
                ctx.lines_as("Lalo", args!["See you next time!"])?;
                ctx.close_window()?;
                ctx.fx().cutin("job_bard_aiolo01", 255)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn wandering_bard(ctx: &Ctx) -> Script {
    wandering_bard_run(ctx, WanderingBardStep::Start, Vec::new()).map(|_| ())
}
