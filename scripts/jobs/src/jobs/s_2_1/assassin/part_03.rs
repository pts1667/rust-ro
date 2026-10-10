use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn guildmaster_asn2_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_assassin_sangdam = Val::from(0);
    ctx.call(
        Function::SavePoint,
        vec![Val::from("moc_ruins"), Val::from(79), Val::from(99), Val::from(1), Val::from(1)],
    )?;
    if (ctx.var("assin_q").get()? == 7 && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?)) {
        ctx.var("assin_q").set(Val::from(8))?;
        ctx.lines_as(
            "Guildmaster",
            args!["Welcome.", "I apologize for", "making you go", "through the maze."],
        )?;
        ctx.next()?;
        ctx.lines_as("Guildmaster", args!["I saw your resume just now. You're well known as a Thief with guts. Rarely do we receive potential Assassins of your stature."])?;
        ctx.next()?;
        ctx.lines_as(
            "Guildmaster",
            args![
                "May I ask you some questions if you don't mind? You don't have to be nervous. Just remember: if you lie, I will kill you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guildmaster",
            args!["First off, what do you think is the priority of an Assassin?"],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("More power.:An Assassin's pride.:Endless practice.")])? {
            1 => {
                ctx.lines_as(
                    "Guildmaster",
                    args![
                        "More power...",
                        "Yes, you may think",
                        "of Assassins as much",
                        "stronger than Thieves."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Guildmaster", args!["However, for what reason do you wish for more power? To show off? Personal revenge? For what purpose will you use this power?"])?;
                ctx.next()?;
                ctx.lines_as("Guildmaster", args!["Why do you want", "to be stronger", "than you are now?"])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Revenge...!:Money~:I want to travel.")])? {
                    1 => {
                        ctx.lines_as(
                            "Guildmaster",
                            args![
                                "Revenge...?",
                                "Yes, I understand. All of us hold grudges against someone else eventually."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Guildmaster", args!["However, keep in mind that we are not allowed to be emotionally attached. Carry out your duties without question. That is our way."])?;
                        ctx.next()?;
                        ctx.lines_as("Guildmaster", args!["Being an Assassin means", "to abandon the ego."])?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.var("assin_q").set(Val::from(9))?;
                        ctx.lines_as("Guildmaster", args!["Financial reasons...? I won't deny that we all need money to live. But being Assassin means living for a higher purpose."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildmaster",
                            args!["Being an Assassin means", "to abandon such worldly", "attachments..."],
                        )?;
                        ctx.next()?;
                    }
                    3 => {
                        ctx.var("assin_q").set(Val::from(10))?;
                        ctx.lines_as(
                            "Guildmaster",
                            args!["Good idea. Traveling around the world will allow you to broaden your experiences."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildmaster",
                            args!["Most of us tend to avoid gathering into groups though, but I'm sure you already caught that, yes?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildmaster",
                            args!["In a way, being an Assassin is to live life in loneliness..."],
                        )?;
                        ctx.next()?;
                    }
                    _ => {}
                }
            }
            2 => {
                ctx.lines_as(
                    "Guildmaster",
                    args!["An Assassin's pride...", "Did other Assassins tell you that...?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Guildmaster",
                    args!["Pride is certainly important, but pride is worth nothing if you do not have any ability."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Guildmaster",
                    args!["Most of the Assassins you've met before me are brethren that have shared many difficult times together."],
                )?;
                ctx.next()?;
                ctx.lines_as("Guildmaster", args!["I can understand why their pride and dignity would be so important to them. Now, for what reason do you wish to become an Assassin?"])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "I like the solitude.:Making money being an Assassin.:They just look interesting.",
                    )],
                )? {
                    1 => {
                        ctx.var("assin_q").set(Val::from(11))?;
                        ctx.lines_as(
                            "Guildmaster",
                            args![
                                "You got the point...",
                                "We are lonely. We will always be alone, even amongst each other..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildmaster",
                            args!["In a way, being", "an Assassin equals", "nothing, I would say."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Guildmaster", args!["But, as I told you before, we have comrades. I recommend having at least one comrade to back up you when you're on a mission."])?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.var("assin_q").set(Val::from(12))?;
                        ctx.lines_as("Guildmaster", args!["Well, I can't deny it, we do need money to make a living. But don't you think we should pursue something even more important than money?"])?;
                        ctx.next()?;
                    }
                    3 => {
                        ctx.var("assin_q").set(Val::from(13))?;
                        ctx.lines_as(
                            "Guildmaster",
                            args!["Style and appearance is only superficial. It is sad that many people think this way..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildmaster",
                            args!["Such disgraceful Assassins that have lost their true focus are dealt with in our own manner..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildmaster",
                            args![
                                "Don't forget...",
                                "Assassins don't toy around. We are not into a style or trend, and we never will be."
                            ],
                        )?;
                        ctx.next()?;
                    }
                    _ => {}
                }
            }
            3 => {
                ctx.lines_as(
                    "Guildmaster",
                    args![
                        "Endless Practice...",
                        "I think you have what it takes. Is there a reason you want to be an Assassin?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Guildmaster",
                    args![
                        "Unlike the Thief class, the Assassin job doesn't allow self-indulgence. Tell me the",
                        "reason you train endlessly."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("To broaden my skills.:It's a goal of mine.:For spiritual improvement.")],
                )? {
                    1 => {
                        ctx.var("assin_q").set(Val::from(14))?;
                        ctx.lines_as("Guildmaster", args!["Learning skills comes naturally with the Assassin job. But don't think of skills as the best value of your training."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildmaster",
                            args!["You won't be satisfied in becoming an Assassin if you think this..."],
                        )?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.var("assin_q").set(Val::from(15))?;
                        ctx.lines_as(
                            "Guildmaster",
                            args!["It's a goal of yours, eh? Well, I guess you've got to have goals..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Guildmaster", args!["I once knew a person who had goals. Long ago, I met someone on assignment who wanted to keep from getting killed before becoming a level 54 knight."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildmaster",
                            args!["It's to bad I had to kill him before he was level 52... ^666666*Sigh*^000000 Oh well."],
                        )?;
                        ctx.next()?;
                        ctx.mes(
                            "I'm a bit worried about you. I hope you realize that once you become an Assassin, there's no turning back...",
                        )?;
                        ctx.next()?;
                    }
                    3 => {
                        ctx.var("assin_q").set(Val::from(16))?;
                        ctx.lines_as("Guildmaster", args!["Good idea...", "That is a good way to improve yourself. I've seen many people who know how to be strong physically but not in their mental state."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildmaster",
                            args!["I hope you're not a hypocrite. Spiritual discipline is the best way for you to survive."],
                        )?;
                        ctx.next()?;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        ctx.lines_as(
            "Guildmaster",
            args!["Sadly, there are some nit-wits who are eager to be Assassins even though they don't know anything..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guildmaster",
            args!["They cause problems and bring us disgrace. Their activities often result in horrible situations."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guildmaster",
            args![
                "Be careful lest you become one of them once you become an Assassin. The responsibility rests solely on your shoulders..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guildmaster",
            args!["So if you could become an Assassin right now, what is the first thing you would do?"],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from(
                "I would go hunt right away.:There are people waiting for me.:Check how I can help as an Assassin.",
            )],
        )? {
            1 => {
                ctx.lines_as("Guildmaster", args!["Hunt...", "Is that all...?"])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "I would level up fast.:I want to explore my Assassin skills.:I will go where I couldn't go as a Thief.",
                    )],
                )? {
                    1 => {
                        l_assassin_sangdam = (l_assassin_sangdam.clone() + Val::from(10));
                        ctx.lines_as(
                            "Guildmaster",
                            args![
                                "Don't act recklessly...",
                                "Being an Assassin never makes you a different person. And don't rely on chance."
                            ],
                        )?;
                        ctx.next()?;
                    }
                    2 => {
                        l_assassin_sangdam = (l_assassin_sangdam.clone() + Val::from(5));
                        ctx.lines_as("Guildmaster", args!["It is good for one to examine oneself. I can understand that you will be excited by the great change in your ability."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildmaster",
                            args!["In the meantime, I hope you won't forget the Assassin mentality."],
                        )?;
                        ctx.next()?;
                    }
                    3 => {
                        ctx.lines_as("Guildmaster", args!["Very well...", "Exploring places you've never seen before. But know that being an Assassin never makes you a different person."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildmaster",
                            args!["Don't force yourself too much.", "Take your time and travel wisely."],
                        )?;
                        ctx.next()?;
                    }
                    _ => {}
                }
            }
            2 => {
                ctx.lines_as("Guildmaster", args!["Who is waiting", "for you, might I ask?"])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("My friends.:My Guildsmen.:My lover.")])? {
                    1 => {
                        l_assassin_sangdam = (l_assassin_sangdam.clone() + Val::from(5));
                        ctx.lines_as(
                            "Guildmaster",
                            args!["I see...", "Appreciate them for caring about you, even when you're alone."],
                        )?;
                        ctx.next()?;
                    }
                    2 => {
                        l_assassin_sangdam = (l_assassin_sangdam.clone() + Val::from(5));
                        ctx.lines_as("Guildmaster", args!["Great...", "Comrades for whom you would die for..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildmaster",
                            args!["As an Assassin, find a job that you can do for them without them knowing.."],
                        )?;
                        ctx.next()?;
                    }
                    3 => {
                        ctx.lines_as(
                            "Guildmaster",
                            args![
                                "Haha, the needs of the body are sometimes hard to ignore. It's best to accept that part of human nature."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.mes("[Guildmaster]")?;
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            ctx.mes("But you must never reveal to her the Assassin side of your life. No matter what it takes.")?;
                        } else {
                            ctx.mes("But you must never reveal to him the Assassin side of your life, no matter what it takes.")?;
                        }
                        ctx.next()?;
                        ctx.lines_as("Guildmaster", args!["Love your beloved forever, even if you can't openly express it. Sometimes, life doesn't allow you to find true love more than once."])?;
                        ctx.next()?;
                    }
                    _ => {}
                }
            }
            3 => {
                ctx.lines_as(
                    "Guildmaster",
                    args!["That's most admirable. Is there anything that you would like to ask me about?"],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Places where Assassins can level up...:Main goals as an Assassin.:Financial consulting.",
                    )],
                )? {
                    1 => {
                        l_assassin_sangdam = (l_assassin_sangdam.clone() + Val::from(5));
                        ctx.lines_as(
                            "Guildmaster",
                            args!["It all depends on your mind. Any place could be the best to level up according to your mind state."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Guildmaster", args!["You must know how to", "survive in any situation."])?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as(
                            "Guildmaster",
                            args!["There are many Assassins out there. Look to them as your trainers, and ask for their opinions."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Guildmaster", args!["I hope you will become an excellent Assassin. And when you reach a certain level, you must guide newbies as your trainers have."])?;
                        ctx.next()?;
                    }
                    3 => {
                        l_assassin_sangdam = (l_assassin_sangdam.clone() + Val::from(10));
                        ctx.lines_as(
                            "Guildmaster",
                            args!["Oh my lord...", "Are you planning to become an Assassin in order to make money?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildmaster",
                            args!["People of that nature are unwelcome. If such is your goal, you may wish to reconsider your job..."],
                        )?;
                        ctx.next()?;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        ctx.lines_as(
            "Guildmaster",
            args!["It was nice to meet you. You reminded me of the good ol' days."],
        )?;
        ctx.next()?;
        ctx.lines_as("Guildmaster", args!["Please give me", "one second..."])?;
        ctx.next()?;
        ctx.call(
            Function::MapAnnounce,
            vec![
                Val::from("in_moc_16"),
                ((Val::from("Those involved with the testing of ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from(", please gather before me.")),
                ctx.constant("BC_MAP")?,
            ],
        )?;
        ctx.lines_as(
            "Guildmaster",
            args![
                ((Val::from("Those involved with the testing of ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from(", please gather before me."))
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Guildmaster", args!["They will", "be here soon."])?;
        ctx.next()?;
        ctx.call(Function::EnableNpc, vec![Val::from("[Huey]")])?;
        ctx.call(Function::EnableNpc, vec![Val::from("[Khai]")])?;
        ctx.call(Function::EnableNpc, vec![Val::from("[The Anonymous One]")])?;
        ctx.call(Function::EnableNpc, vec![Val::from("[Barcardi]")])?;
        ctx.call(Function::EnableNpc, vec![Val::from("[Beholder]")])?;
        ctx.call(Function::EnableNpc, vec![Val::from("[Thomas]")])?;
        ctx.call(Function::EnableNpc, vec![Val::from("[Gayle Maroubitz]")])?;
        ctx.lines_as("The Anonymous One", args!["I am here."])?;
        ctx.next()?;
        ctx.lines_as(
            "Guildmaster",
            args![
                ((Val::from("I would like to listen to your opinion of ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from(" for the job change test."))
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "The Anonymous One",
            args![
                "Ah yeah...",
                "I think",
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("")),
                "is decent."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guildmaster",
            args!["Well...", "The Anonymous One", "supports you. How", "about you, Huey?"],
        )?;
        ctx.next()?;
        if ctx.var("assin_q3").get()? == 1 {
            ctx.lines_as("Huey", args!["A rarity.", "You can tell", "by the job level."])?;
            ctx.next()?;
            ctx.lines_as("Huey", args!["I agree with", "the Anonymous One."])?;
        } else {
            ctx.lines_as(
                "Huey",
                args![
                    ((((Val::from("Although ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                        + Val::from(" looks too mellow and gentle, kind of like a pussycat, "))
                        + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                        + Val::from(" has the stuff."))
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Huey", args!["If it's alright with you, I'd like to get back to my job."])?;
            ctx.next()?;
            ctx.lines_as("Guildmaster", args!["Yes...", "That is all, Huey."])?;
        }
        ctx.next()?;
        ctx.lines_as("Guildmaster", args!["So...", "'Beholder,' what", "is your opinion?"])?;
        ctx.next()?;
        ctx.call(Function::DisableNpc, vec![Val::from("[Huey]")])?;
        ctx.lines_as(
            "Beholder",
            args!["Well, I don't like the course score. But, somehow the whole test was passed. I'm okay with this person."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guildmaster",
            args![
                "Hmm...",
                ((Val::from("It seems we are all in agreement. Good. I don't have any problem with ")
                    + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from(" as well..."))
            ],
        )?;
        ctx.next()?;
        let subject9 = ctx.var("assin_q").get()?;
        if subject9 == 8 {
            ctx.lines_as(
                "Guildmaster",
                args!["Even though you're driven by personal revenge, I hope it will go away as you train..."],
            )?;
            ctx.next()?;
        } else if subject9 == 9 {
            ctx.lines_as(
                "Guildmaster",
                args!["Even though your main concern for now is being a rich, I'm sure you'll pursue something even greater..."],
            )?;
            ctx.next()?;
        } else if subject9 == 10 {
            ctx.lines_as(
                "Guildmaster",
                args!["Eager to travel all around the world, I hope your real identity is found in your journeys..."],
            )?;
            ctx.next()?;
        } else if subject9 == 11 {
            ctx.lines_as(
                "Guildmaster",
                args!["You seem to know a lot about Assassins. I don't think frustration from being alone will be difficult for you..."],
            )?;
            ctx.next()?;
        } else if subject9 == 12 {
            ctx.lines_as(
                "Guildmaster",
                args!["You have an idiocy about money, but I believe that you should be able to overcome it."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Guildmaster",
                args!["Although I do not trust you for now, I will give you a chance..."],
            )?;
            ctx.next()?;
        } else if subject9 == 13 {
            ctx.lines_as("Guildmaster", args!["Even though you're enamored with Assassins superficially, I believe you will realize the real value of the Assassin job sooner or later."])?;
            ctx.next()?;
        } else if subject9 == 14 {
            ctx.lines_as("Guildmaster", args!["One of the rare people who seeks better skills, I hope you will realize the importance of spiritual discipline sooner or later."])?;
            ctx.next()?;
        } else if subject9 == 15 {
            ctx.lines_as(
                "Guildmaster",
                args!["Sooner or later, you will find a new goal to which you can devote yourself..."],
            )?;
            ctx.next()?;
        } else if subject9 == 16 {
            ctx.lines_as(
                "Guildmaster",
                args![
                    "I know some people care only about their physical training, but",
                    "I believe you stand out amongst them..."
                ],
            )?;
            ctx.next()?;
        }
        let subject10 = ctx.var("assin_q3").get()?;
        if subject10 == 1 {
            ctx.lines_as(
                "Guildmaster",
                args!["Well, I've said too much. Please choose a weapon as a present."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Guildmaster",
                args!["You can choose a Jur, Katar, Main Gauche, or a Gladius. As a master, I love them all."],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Jur:Katar:Main Gauche:Gladius")])? {
                1 => {
                    ctx.lines_as(
                        "Guildmaster",
                        args!["A Jur...", "Good choice. There you are. I hope it will serve you well."],
                    )?;
                    ctx.call(Function::GetItem, vec![Val::from(1251), Val::from(1)])?;
                }
                2 => {
                    ctx.lines_as(
                        "Guildmaster",
                        args![
                            "A Katar...",
                            "Here you are.",
                            "Although it's used,",
                            "I know it will",
                            "serve you well."
                        ],
                    )?;
                    ctx.call(Function::GetItem, vec![Val::from(1253), Val::from(1)])?;
                }
                3 => {
                    ctx.lines_as(
                        "Guildmaster",
                        args!["I see. You want to use both hands. Here, take your Main Gauche."],
                    )?;
                    ctx.call(Function::GetItem, vec![Val::from(1208), Val::from(1)])?;
                }
                4 => {
                    ctx.lines_as(
                        "Guildmaster",
                        args![
                            "A Gladius...",
                            "It used to rule over the Assassin weapon market. Please take care of my gladius."
                        ],
                    )?;
                    ctx.call(Function::GetItem, vec![Val::from(1220), Val::from(1)])?;
                }
                _ => {}
            }
            ctx.var("assin_q3").set(Val::from(3))?;
            ctx.next()?;
        } else if subject10 == 2 {
            ctx.lines_as("Guildmaster", args!["Well, I talked too much.", "Please take this first."])?;
            ctx.next()?;
            let subject12 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?;
            if subject12 == 1 {
                ctx.call(Function::GetItem, vec![Val::from(1207), Val::from(1)])?;
            } else if subject12 == 2 {
                ctx.call(Function::GetItem, vec![Val::from(1250), Val::from(1)])?;
            } else if subject12 == 3 {
                ctx.call(Function::GetItem, vec![Val::from(1216), Val::from(1)])?;
            } else if subject12 == 4 {
                ctx.call(Function::GetItem, vec![Val::from(1201), Val::from(1)])?;
            } else if subject12 == 5 {
                ctx.call(Function::GetItem, vec![Val::from(1252), Val::from(1)])?;
            }
            ctx.var("assin_q3").set(Val::from(3))?;
        }
        ctx.lines_as(
            "Guildmaster",
            args!["Well, I am giving you a token. Please return to the Assassin expert, the Ferocious-Looking Huey, at the entrance."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guildmaster",
            args!["Upon receiving this token, Huey will promote you to an Assassin."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guildmaster",
            args![
                ((Val::from("You, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from(", have chosen to live as an Assassin. May you learn our ways and be an honorable example to others."))
            ],
        )?;
        ctx.call(
            Function::SavePoint,
            vec![Val::from("moc_ruins"), Val::from(79), Val::from(99), Val::from(1), Val::from(1)],
        )?;
        ctx.call(Function::GetItem, vec![Val::from(1008), Val::from(1)])?;
        ctx.var("assin_q").set(Val::from(17))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8006), Val::from(8007)])?;
        ctx.next()?;
        ctx.lines_as(
            "Guildmaster",
            args!["Okay, all of you may go back to your positions. I will send you to the entrance as well. Let's move..."],
        )?;
        ctx.call(Function::DisableNpc, vec![Val::from("[Huey]")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("[Khai]")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("[The Anonymous One]")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("[Barcardi]")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("[Beholder]")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("[Thomas]")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("[Gayle Maroubitz]")])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(17), Val::from(19)])?;
        return Err(Stop::End);
    } else if ctx.var("assin_q").get()? == 17 {
        ctx.lines_as("Guildmaster", args!["Umm...?", "How come you're in here...?"])?;
        ctx.next()?;
        ctx.lines_as("Guildmaster", args!["You already finished your test. Why don't you go try to get the ^006699Necklace of Oblivion^000000 so you can change your job?"])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(17), Val::from(19)])?;
        return Err(Stop::End);
    } else if (ctx.var("assin_q").get()?.number()? > 7 && ctx.var("assin_q").get()?.number()? < 17) {
        ctx.var("assin_q").set(Val::from(7))?;
        ctx.lines_as(
            "Guildmaster",
            args!["What the hell? You pressed 'Cancel' during the process. Do you want to change your job or what?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guildmaster",
            args![
                "^666666*Sigh...*^000000",
                "Ok, let's start again. If you don't listen this time, you won't leave this room alive. You got me?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Guildmaster", args!["Umm? How come your in here?"])?;
        ctx.next()?;
        ctx.lines_as("Guildmaster", args!["You already finished your test, go give your ^006699Necklace of Oblivion^000000 to the Ferocious-looking guy so you can change your job!"])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(17), Val::from(19)])?;
        return Err(Stop::End);
    }
}

pub fn guildmaster_asn2_ontouch(ctx: &Ctx) -> Script {
    guildmaster_asn2_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn master_assist_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Assistent Gayle Maroubitz",
        args!["Sorry, but I'm not in charge of job changes. Go to the Guildmaster, as he has told you."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn master_assist(ctx: &Ctx) -> Script {
    master_assist_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Info1Step {
    Start,
    OnTouch,
}

fn info_1_run(ctx: &Ctx, mut step: Info1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Info1Step::Start => {
                step = Info1Step::OnTouch;
                continue 'machine;
            }
            Info1Step::OnTouch => {
                ctx.lines_as(
                    "Guildmaster",
                    args!["Huh.", "Now, that place is blocked. You might want to check the other side."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn info_1(ctx: &Ctx) -> Script {
    info_1_run(ctx, Info1Step::Start, Vec::new()).map(|_| ())
}

pub fn info_1_ontouch(ctx: &Ctx) -> Script {
    info_1_run(ctx, Info1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Info2Step {
    Start,
    OnTouch,
}

fn info_2_run(ctx: &Ctx, mut step: Info2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Info2Step::Start => {
                step = Info2Step::OnTouch;
                continue 'machine;
            }
            Info2Step::OnTouch => {
                ctx.lines_as(
                    "Guildmaster",
                    args!["You're getting warmer. You're almost there. Just, turn around a little bit."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn info_2(ctx: &Ctx) -> Script {
    info_2_run(ctx, Info2Step::Start, Vec::new()).map(|_| ())
}

pub fn info_2_ontouch(ctx: &Ctx) -> Script {
    info_2_run(ctx, Info2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Info3Step {
    Start,
    OnTouch,
}

fn info_3_run(ctx: &Ctx, mut step: Info3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Info3Step::Start => {
                step = Info3Step::OnTouch;
                continue 'machine;
            }
            Info3Step::OnTouch => {
                ctx.lines_as("Guildmaster", args!["Hmm.", "Now, that place", "is blocked."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn info_3(ctx: &Ctx) -> Script {
    info_3_run(ctx, Info3Step::Start, Vec::new()).map(|_| ())
}

pub fn info_3_ontouch(ctx: &Ctx) -> Script {
    info_3_run(ctx, Info3Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Info4Step {
    Start,
    OnTouch,
}

fn info_4_run(ctx: &Ctx, mut step: Info4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Info4Step::Start => {
                step = Info4Step::OnTouch;
                continue 'machine;
            }
            Info4Step::OnTouch => {
                ctx.lines_as(
                    "Guildmaster",
                    args!["Umm...", "You're heading for my assistant. Do you still need to talk to him?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn info_4(ctx: &Ctx) -> Script {
    info_4_run(ctx, Info4Step::Start, Vec::new()).map(|_| ())
}

pub fn info_4_ontouch(ctx: &Ctx) -> Script {
    info_4_run(ctx, Info4Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Info5Step {
    Start,
    OnTouch,
}

fn info_5_run(ctx: &Ctx, mut step: Info5Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Info5Step::Start => {
                step = Info5Step::OnTouch;
                continue 'machine;
            }
            Info5Step::OnTouch => {
                ctx.lines_as("Guildmaster", args!["Well done...", "I can feel your steps near me."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn info_5(ctx: &Ctx) -> Script {
    info_5_run(ctx, Info5Step::Start, Vec::new()).map(|_| ())
}

pub fn info_5_ontouch(ctx: &Ctx) -> Script {
    info_5_run(ctx, Info5Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Info6Step {
    Start,
    OnTouch,
}

fn info_6_run(ctx: &Ctx, mut step: Info6Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Info6Step::Start => {
                step = Info6Step::OnTouch;
                continue 'machine;
            }
            Info6Step::OnTouch => {
                ctx.lines_as("Guildmaster", args!["Hm? Not bad. You're almost here."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn info_6(ctx: &Ctx) -> Script {
    info_6_run(ctx, Info6Step::Start, Vec::new()).map(|_| ())
}

pub fn info_6_ontouch(ctx: &Ctx) -> Script {
    info_6_run(ctx, Info6Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Info7Step {
    Start,
    OnTouch,
}

fn info_7_run(ctx: &Ctx, mut step: Info7Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Info7Step::Start => {
                step = Info7Step::OnTouch;
                continue 'machine;
            }
            Info7Step::OnTouch => {
                ctx.lines_as("Guildmaster", args!["I don't think you're going the right way."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn info_7(ctx: &Ctx) -> Script {
    info_7_run(ctx, Info7Step::Start, Vec::new()).map(|_| ())
}

pub fn info_7_ontouch(ctx: &Ctx) -> Script {
    info_7_run(ctx, Info7Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Info8Step {
    Start,
    OnTouch,
}

fn info_8_run(ctx: &Ctx, mut step: Info8Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Info8Step::Start => {
                step = Info8Step::OnTouch;
                continue 'machine;
            }
            Info8Step::OnTouch => {
                ctx.lines_as("Guildmaster", args!["No sense of direction, eh?"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn info_8(ctx: &Ctx) -> Script {
    info_8_run(ctx, Info8Step::Start, Vec::new()).map(|_| ())
}

pub fn info_8_ontouch(ctx: &Ctx) -> Script {
    info_8_run(ctx, Info8Step::OnTouch, Vec::new()).map(|_| ())
}

fn moc_assin_dup_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn moc_assin_dup(ctx: &Ctx) -> Script {
    moc_assin_dup_body(ctx, Vec::new()).map(|_| ())
}

fn moc_assin_dup_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![])?;
    return Err(Stop::End);
}

pub fn moc_assin_dup_oninit(ctx: &Ctx) -> Script {
    moc_assin_dup_oninit_body(ctx, Vec::new()).map(|_| ())
}
