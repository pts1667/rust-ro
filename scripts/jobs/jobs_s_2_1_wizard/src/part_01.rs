use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn wizard_guildsman_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("Upper").get()? == 1 {
        ctx.lines_as(
            "Catherine",
            args![
                "? Excuse me, I am wondering if we have met before...?",
                "Hey~ you have changed a lot! So, what happened?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Catherine",
            args![
                "I feel you have become so much powerful, eh?",
                "Congratulations and good luck with your life!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_WIZARD")?) {
            ctx.lines_as(
                "Catherine",
                args![
                    "Since you're already a Wizard, you don't have any more business with me...?",
                    "Now, excuse me."
                ],
            )?;
            ctx.next()?;
            ctx.mes("[Catherine]")?;
            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                ctx.mes("Well, If you intended to ask me on a date... I appreciate it...hohoho.")?;
            } else {
                ctx.mes("Well, if you fix me up with a cute guy... I'd appreciate it...hohoho!.")?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.lines_as(
                "Wizard Guildsman",
                args![
                    "Oh my goodness, it's a novice~ ain't you the cutest little thing.",
                    "By the way honey, this place is for Wizards."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Wizard Guildsman",
                args!["If you are interested in magic,", "Go visit the ^0000FFMage^000000 guild."],
            )?;
        } else {
            ctx.lines_as(
                "Wizard Guildsman",
                args![
                    "Huh? what brings you to such a high place...?",
                    "If you don't have any specific business, please leave immediately.",
                    "This place is for Wizards, you know?"
                ],
            )?;
        }
        ctx.next()?;
        ctx.lines_as("Wizard Guildsman", args!["Ok, then. Farewell."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("wiz_q").get()? == 0 {
        ctx.lines_as(
            "Wizard Guildsman",
            args!["Huh? What are you doing way up here...?", "So what brings you here?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("I want to become a Wizard.:...nothing.")],
        )?) == 1
        {
            ctx.lines_as(
                "Wizard Guildsman",
                args![
                    "I see... Well, of course you want to become a Wizard, otherwise you wouldn't have walked up all those stairs right?",
                    "Anyways, I would like to welcome you. I will assist you in becoming a Wizard."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Wizard Guildsman",
                args![
                    "My name is Catherine Medici. Just between me and you, I just became a Wizard too!",
                    "You can call me Cathy if you want. Hehehehehe~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Catherine",
                args![
                    "Many people want to become Wizards, but only the patient and strong ones will eventually accomplish their goals.",
                    "In order to become a Wizard, one must undergo difficult quests."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Catherine",
                args![
                    "Also only mages who have reached a job level of 40 or higher are qualified for these quests.",
                    "Lower level mages aren't experienced enough with magic to become Wizards..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Catherine",
                args![
                    "Well, I will give you more information when you apply for the job.",
                    "So! Do you want to apply now?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Yes, I do.:On second thought, Let me think about it.")],
            )?) == 1
            {
                if ctx.var("JobLevel").get()?.number()? < 40 {
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "Hey hey, weren't you listening to me?",
                            "I told you that you need to be at least job level 40 to sign up for the quest..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "No need to hurry, go experience and ponder more about the world of magic.",
                            "When I deem you're qualified, I will accept your application."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("SkillPoint").get()?.is_true() {
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "Umm... You're well qualified, but you have some unused skill points left.",
                            "You'd better learn more skills before applying again."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Catherine",
                    args![
                        "Very well then, I accept your application.",
                        ((Val::from("Your name is...") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                            + Val::from(", isn't it? I am not sure if I pronounced it correctly?"))
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Catherine",
                    args![
                        "The Wizard job change quest is divided into 3 parts.",
                        "1st part, gathering magic items.",
                        "2nd part, a written test.",
                        "The last part is a practical magic test."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Catherine",
                    args![
                        "Among one of these tests, we exempt those who are at job level 50.",
                        "It is enough to prove to us the effort it will take to become a Wizard if a mage has learned that much already."
                    ],
                )?;
                ctx.next()?;
                if ctx.var("JobLevel").get()? == 50 {
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "Hmm, come to think of it, you're at job level 50!?",
                            "You must have worked very hard. Your going to become a very powerful Wizard, I can feel it in you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "Ok. You already passed the 1st test then.",
                            "Don't get too laid back though, there are still 2 more tests left."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "Go talk to the man in the corner and he will give you the remaining exams.",
                            "Be careful. We have lost many Mages due to the difficulty of the exams."
                        ],
                    )?;
                    ctx.var("wiz_q").set(Val::from(3))?;
                    ctx.call(Function::SetQuest, vec![Val::from(9015)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Catherine",
                    args![
                        "Then, since I have your application and everything else I need, I'll give you some info about the first test.",
                        "You can memorize this or write it down, doesn't matter. But remember it for sure."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Catherine",
                    args![
                        "The first test is collecting magic items.",
                        "The important part is that you must gather these items on your own."
                    ],
                )?;
                ctx.next()?;
                ctx.var("wizard_m1")
                    .set(ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?)?;
                ctx.lines_as("Catherine", args!["The items you must collect are..."])?;
                if ctx.call(Function::Rand, vec![Val::from(2)])?.is_true() {
                    ctx.var("wiz_q").set(Val::from(1))?;
                    ctx.call(Function::SetQuest, vec![Val::from(9013)])?;
                    ctx.lines(args![
                        "^3355FFRed Gemstone^000000 10 each,",
                        "^3355FFBlue Gemstone^000000 10 each,",
                        "^3355FFYellow Gemstone^000000 10 each,"
                    ])?;
                } else {
                    ctx.var("wiz_q").set(Val::from(2))?;
                    ctx.call(Function::SetQuest, vec![Val::from(9014)])?;
                    ctx.lines(args![
                        "^3355FFCrystal Blue^000000 5 each,",
                        "^3355FFGreen Live^000000 5 each,",
                        "^3355FFRed Blood^000000 5 each,",
                        "^3355FFWind of Verdure^000000 5 each,"
                    ])?;
                }
                ctx.mes("...is it too hard? No, any would be Wizard should be able to at least get these items.")?;
                ctx.next()?;
                ctx.lines_as("Catherine", args!["Well, good luck.", "I'll be happily waiting. ~Hehe."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Catherine",
                args![
                    "Oh, ok then, take your time.",
                    "Since I'll always be here, accepting applications...*sigh*...anyways! ~Hehehehe."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Wizard Guildsman",
            args![
                "Geez, what a lame person.",
                "You have no business here, hope you don't mind, now off you go!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("wiz_q").get()? == 1 {
            ctx.lines_as(
                "Catherine",
                args!["Let me see, did you get all the items?", "Then let's check..."],
            )?;
            ctx.next()?;
            if ((ctx.call(Function::CountItem, vec![Val::from(716)])?.number()? > 9
                && ctx.call(Function::CountItem, vec![Val::from(717)])?.number()? > 9)
                && ctx.call(Function::CountItem, vec![Val::from(715)])?.number()? > 9)
            {
                ctx.lines_as(
                    "Catherine",
                    args![
                        "Perfect! You got all the items. I like!~",
                        "These items will be put to great use in our guild. ~Hehehee."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(716), Val::from(10)])?;
                ctx.call(Function::DelItem, vec![Val::from(717), Val::from(10)])?;
                ctx.call(Function::DelItem, vec![Val::from(715), Val::from(10)])?;
                ctx.var("wiz_q").set(Val::from(3))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(9013), Val::from(9015)])?;
                ctx.lines_as(
                    "Catherine",
                    args![
                        "Good for you! You passed the first test.",
                        "But there are still two more left, so don't get too relaxed."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Catherine", args!["Go talk to the guy in the corner for the rest of the tests.", "It might be a bit hard, so be careful, I wouldn't want you ending up like most of the Mages that come for these exams...poor souls indeed, may they rest in peace. ~Hehehe."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Catherine",
                args!["Hey, what's this? Umm...I don't think you have everything yet!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Catherine", args!["It might've been tough coming all the way up here, what with this altitude and all, but go back and collect the items.", "Life's short enough as it is, so stop wasting your time and gather the items I told you before."])?;
            ctx.next()?;
            ctx.lines_as(
                "Catherine",
                args![
                    "^3355FFRed Gemstone^000000 10 each,",
                    "^3355FFBlue Gemstone^000000 10 each,",
                    "^3355FFYellow Gemstone^000000 10 each,",
                    "Don't forget this time, and bring all the items, ok?..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("wiz_q").get()? == 2 {
                ctx.lines_as(
                    "Catherine",
                    args!["Did you get all the items?", "Let's see... Do you have the right ones?..."],
                )?;
                ctx.next()?;
                if (((ctx.call(Function::CountItem, vec![Val::from(991)])?.number()? > 4
                    && ctx.call(Function::CountItem, vec![Val::from(993)])?.number()? > 4)
                    && ctx.call(Function::CountItem, vec![Val::from(990)])?.number()? > 4)
                    && ctx.call(Function::CountItem, vec![Val::from(992)])?.number()? > 4)
                {
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "Perfect! Good job...I'm satisfied! ~Hehe",
                            "Our guild greatly appreciates these items and will use them wisely."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(991), Val::from(5)])?;
                    ctx.call(Function::DelItem, vec![Val::from(993), Val::from(5)])?;
                    ctx.call(Function::DelItem, vec![Val::from(990), Val::from(5)])?;
                    ctx.call(Function::DelItem, vec![Val::from(992), Val::from(5)])?;
                    ctx.var("wiz_q").set(Val::from(3))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(9014), Val::from(9015)])?;
                    ctx.lines_as("Catherine", args!["Good. You've passed the first test now.", "But you still have two more tests to go, so don't get too laid back, because it only gets harder from here. ~Hehehe"])?;
                    ctx.next()?;
                    ctx.lines_as("Catherine", args!["Go talk to that guy in the corner to take the rest of the tests.", "It might be a bit hard, so be careful, I wouldn't want you ending up like most of the Mages that come for these exams...poor souls indeed, may they rest in peace. ~Hehehe."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Catherine", args!["Eh, what's this? I don't think you have everything?"])?;
                ctx.next()?;
                ctx.lines_as("Catherine", args!["I regret that you had to go through the trouble of coming all the way up here, but go get the right items again.", "Stop wasting your time and get the items I initially requested. For goodness sakes...This is the easiest part of the exam."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Catherine",
                    args![
                        "^3355FFCrystsal Blue^000000 5 each,",
                        "^3355FFGreen Live^000000 5 each,",
                        "^3355FFRed Blood^000000 5 each,",
                        "^3355FFWind of Verdure^000000 5 each,",
                        "Don't forget them this time and gather the correct ones, ok? See you soon..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("wiz_q").get()? == 3 {
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "What is it? You didn't go talk to the guy in the corner?",
                            "You can't become a Wizard by just brining the items I requested, no no, that just wont do..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "You can only prove yourself Wizard material after you take the two remaining tests.",
                            "I'll be waiting, so go now."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("wiz_q").get()? == 4 {
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "*sigh*...Poor thing, what a pity.",
                            "How can you think of trying to become a Wizard when you can't even answer those simple questions?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "Let's see... Should I give you some hints as your senior?",
                            "But I'm a bit thirsty, so give me 1 Apple Juice, and we got a deal. ~Hehehe"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Give me some hints, please.:I want to try again on my own!")])? {
                        1 => {
                            if ctx.call(Function::CountItem, vec![Val::from(531)])?.number()? > 0 {
                                ctx.call(Function::DelItem, vec![Val::from(531), Val::from(1)])?;
                                ctx.lines_as(
                                    "Catherine",
                                    args!["Yummers, Apple Juice is the best...", "Gulp gulp gulp... Haaaaaah... ~Hehe"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Catherine", args!["Well then, I'll give you a hint for the second test."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Catherine",
                                    args![
                                        "He asks three types of questions.",
                                        "Questions about Magic, Monsters...",
                                        "and Magicians...also known as Mages."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Catherine", args!["It's up to him to decide which questions to ask.", "*sigh*...He'd look much better if he cut his hair and shaved...", "He always looks so grungy because he doesn't take care of himself...*sigh*...pity, he could be a regular lady killer. ~tehehehe"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Catherine",
                                    args![
                                        "Anyways, as for the questions about magic...",
                                        "They're usually about the spells you've learned."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Catherine",
                                    args![
                                        "If it's something you haven't learned or haven't experienced, or anything like that...",
                                        "I heard there's a nice place you can refer to that's very well organized."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Catherine",
                                    args![
                                        "You just have to cast the spell, you know...the one that looks like a big bold *e* symbol.",
                                        "The magic words are... iro.ragnarokonline.com ~Or so they say!",
                                        "Strange spell, don't you think? I don't know how to make it work myself."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Catherine",
                                    args![
                                        "And about the monster questions.",
                                        "Fighting them yourself and learning is the best way to go about that."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Catherine",
                                    args![
                                        "But you know in Prontera, there is a big library.",
                                        "You can get information about most of the monsters in that library.",
                                        "Going there and studying a bit more would work too."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Catherine",
                                    args![
                                        "Last, but not least, questions about Mages.",
                                        "This is something that most others cannot teach you...",
                                        "Because you are a Mage."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Catherine",
                                    args![
                                        "It's hard to know about others when you don't even know about yourself, right?",
                                        "So if you get one of these questions, think about it carefully then answer."
                                    ],
                                )?;
                            } else {
                                ctx.lines_as("Catherine", args!["Like I said, I'll think about it if you give me 1 Apple Juice.", "If not... Well, you can think about it on your own. Hey I don't make the rules here i just follow em! Can't argue with this one though. ~tehehehe"])?;
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Catherine",
                                args![
                                    "Yeah, you get the most satisfaction when you solve things on your own.",
                                    "Go finish the rest of the tests with that spirit!"
                                ],
                            )?;
                        }
                        _ => {}
                    }
                    ctx.next()?;
                    ctx.lines_as("Catherine", args!["Well then, see you soon! *crosses fingers* dont end up like the others before you! May God rest thier souls...huh? Oh nothing! ~Hehehehe", "Hurry, he's waiting for you!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("wiz_q").get()? == 5 {
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "Tehehehe~ I was listening all along.",
                            "Well done in answering all those questions. I want to give something, like a present..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "You still have one more test to go, right?",
                            "Just a bit more and you will be qualified to be a Wizard, so I'll give you the present then. ~Hehe"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Catherine", args!["See ya, well...hopefully!", "He's waiting for you!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("wiz_q").get()? == 6 {
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "Eh, did you leave in the middle of the test?",
                            "You...*sigh*...I didn't think you would do such a dishonorable thing."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("[Catherine]")?;
                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                        ctx.lines(args![
                            "Granted you're a mage, but how could a guy be so weak?!",
                            "Well, at least your alive still. Go back and try harder this time. ~tehehehe"
                        ])?;
                    } else {
                        ctx.lines(args!["Don't pretend to be weak just because you're a girl. Look at me! I did it, and so can you. You can't ask for sympathy.", "You came all this way to become a Wizard! Now come on, you can do it!!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Catherine",
                            args![
                                "I agree that it is difficult, but not to the point of giving up.",
                                "You of all people should find the strength and patience to complete this test!"
                            ],
                        )?;
                    }
                    ctx.next()?;
                    ctx.lines_as("Catherine", args!["So, try harder this time...", "He's waiting!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("wiz_q").get()? == 7 {
                    if ctx.var("JobLevel").get()?.number()? < 40 {
                        ctx.var("wiz_q").set(Val::from(0))?;
                        ctx.lines_as(
                            "Catherine",
                            args![
                                "Hey, what don't you get it?",
                                "I said you must be at least job level 40 to change your job, got it?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Catherine",
                            args![
                                "There is nothing to be in hurry, so why don't you take your time for studying?",
                                "When the time comes, I will welcome you with open arms."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if ctx.var("SkillPoint").get()?.is_true() {
                        ctx.lines_as(
                            "Catherine",
                            args![
                                "Are you done with all the tests? Oh drats, it seems like you still have some skill points left.",
                                "Learn some other skills with your remaining points, and then come talk to me again."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "Wooooooooooooow...you finished all the tests?!",
                            "Congratulations, congrats, congrats! Wooopiiieeeee! ~Hehehehehehe"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Catherine",
                        args!["Well, no need to wait, I congratulate you. I hearby deem you Wizard."],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::CompleteQuest, vec![Val::from(9018)])?;
                    shared::other_global_functions::job_change(ctx, vec![ctx.constant("JOB_WIZARD")?])?;
                    shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "Since you're a Wizard now, act like a Wizard, got it?",
                            "Us wizards have to be careful since we possess the ultimate power of magic."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Catherine", args!["Don't go about casting spells in town for no reason, or bother others with spells that are in their own battle.", "A Wizard's magic is meant for their own battles with monsters."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "Go join parties with others and keep training.",
                            "And...oh, wait, I prepared a present for you, one sec."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Catherine", args!["rummage rummage...", "shuffle shuffle..."])?;
                    ctx.next()?;
                    ctx.call(Function::GetItem, vec![Val::from(505), Val::from(6)])?;
                    ctx.lines_as(
                        "Catherine",
                        args![
                            "Here you go. I hope you'll make good use of it when you need it. ~tehehe",
                            "I gave them to you as a present, so don't go out and sell it. Use it for yourself, ok?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Catherine",
                        args!["Well, then...*sight*...live a wonderful life as a Wizard, become the strongest out there!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn wizard_guildsman(ctx: &Ctx) -> Script {
    wizard_guildsman_body(ctx, Vec::new()).map(|_| ())
}

fn gloomy_wizard_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_wizard_t = Val::from(0);
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_WIZARD")?) {
            ctx.lines_as(
                "Raulel",
                args![
                    "*Cough* *cough* what do you want?",
                    "If you are a person that uses magic, then you need to make sure you are well informed about it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Raulel", args!["Don't live dishonestly, or impolitely, or else one day you'll be caught in a spell you can't control, and BOOM, your dead!"])?;
            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                ctx.mes("If you don't want that to happen, then learn how to use spells properly!")?;
                ctx.next()?;
                ctx.lines_as(
                    "Raulel",
                    args!["You may live life crippled if you get obsessed with the love of Greater Magic. ~haha"],
                )?;
            } else {
                ctx.mes("So learn how to use magic properly, or you would just be better off giving up on using magic.")?;
                ctx.next()?;
                ctx.lines_as(
                    "Raulel",
                    args![
                        "If you don't want that, go hit on a guy or something! ~Hahahaha",
                        "If you don't pay attention to yourself, you'll be engulfed by magic one day..."
                    ],
                )?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?) {
            ctx.lines_as(
                "Raulel",
                args![
                    "Go away, one who works for the Church!",
                    "Magic repels Holy power, jeez...your messing up my aura."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Raulel",
                args![
                    "And plus, *cough* *cough* my health isn't all that good right now either...",
                    "Don't come any closer, just leave!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.lines_as(
                "Raulel",
                args!["Why did a little one like you come here?!", "Get lost! ~Hahahahaha"],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("geffen"), Val::from(120), Val::from(110)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Raulel",
            args!["*sneeze* *cough* Oooowww...my entire body is in pain. I feel like I'm trapped in a tub of ice water!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Raulel", args!["What do you want? Jeez...just get lost, won't you?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("wiz_q").get()? == 0 {
        ctx.lines_as(
            "Raulel",
            args!["*cough* *cough* *sneeze* I don't know who you are and what you do, but I don't have any business with you."],
        )?;
        ctx.next()?;
        ctx.lines_as("Raulel", args!["Go away! Get lost!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if (ctx.var("wiz_q").get()? == 1 || ctx.var("wiz_q").get()? == 2) {
            ctx.lines_as("Raulel", args!["Hahahaha~ You're the one that wants to become a Wizard?!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Raulel",
                args!["*sneeze* If you just lived as you were, all you'd have to do was hunt a little and live the easy life..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Raulel",
                args!["*Cough* *cough* Let's see how well you live as a Wizard. ~Hahahahhaha"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("wiz_q").get()? == 3 || ctx.var("wiz_q").get()? == 4) {
            if ctx.var("wiz_q").get()? == 3 {
                ctx.lines_as(
                    "Raulel",
                    args![
                        "*Cough* *cough*...You must've passed the first test.",
                        "Ok, I'm the Wizard in charge of your testing from now on. My name is 'Raulel Asparagus'."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Raulel",
                    args!["*sneeze* It's not too late yet, wouldn't you rather just go back to town and enjoy the peaceful life?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Raulel",
                    args!["Hahahaha~ You don't know how dangerous it is...to deal with Greater Magic."],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "I want to live as a normal Mage.:I would like to continue with the tests.",
                    )],
                )?) == 1
                {
                    ctx.lines_as("Raulel", args!["Hahaha~ *sneeze* Good choice...*cough* *cough*~", "Best not to even dream about life as a Wizard. Graa...Greaa...*sneeze* Greater Magic wasn't meant for humans to use!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Raulel",
                        args![
                            "Leave the top of this tower quietly and don't ever look back.",
                            "Just live peacefully with the powers that you have right now."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Raulel",
                    args![
                        "*sneeze* Hahahaha~ Now there's a foolish one here!",
                        "Well then, let's see how good you are. *cough* I want to see this with my own two eyes!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Raulel",
                    args![
                        "*sneeze* Then let's begin the test!",
                        "If you don't answer them all correctly, you fail. Hahahahahahahahaha~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Raulel",
                    args![
                        "I'll give you 10 questions so give me the right answers.",
                        "If you get something wrong, I won't tell you what it is!"
                    ],
                )?;
                if ctx.call(Function::CheckQuest, vec![Val::from(9016)])? == -1 {
                    ctx.call(Function::ChangeQuest, vec![Val::from(9015), Val::from(9016)])?;
                }
                ctx.next()?;
                ctx.lines_as("Raulel", args!["*Cough* *cough* Then here go the questions!"])?;
            } else if ctx.var("wiz_q").get()? == 4 {
                ctx.lines_as(
                    "Raulel",
                    args!["Hahahaha~ Are you that desperate? *sneeze* What a pain in the arse..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Raulel",
                    args!["Since you don't want to settle for a stable and peaceful life, I'll give you another chance..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Raulel",
                    args![
                        "If you miss one single question, then just give up. You wouldn't have any talent in being a Wizard! ~Hahahahaha"
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Because of you, I want to live as a normal Mage now.:I would like to continue with the tests.",
                    )],
                )?) == 1
                {
                    ctx.lines_as("Raulel", args!["Hahahaha~ Surprising, comming from you, that's a very wise choice...*cough* *cough*", "If i were you, i would never, ever dream of becoming a Wizard again. Gre...Greaa...*sneeze* Greater Magic wasn't meant for humans to use."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Raulel",
                        args![
                            "Just leave the top of this tower quietly and never look back.",
                            "Live peacefully with the powers that you have right now."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Raulel",
                    args![
                        "Hahahahahahaha~ Now there's a foolish one right here!",
                        "Well then, let's see just how good you can be! *sneeze* I want to see this with my own two eyes."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Raulel", args!["Then let's begin the test!"])?;
            }
            ctx.next()?;
            let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
            if subject1 == 1 {
                ctx.lines_as(
                    "Raulel",
                    args!["1. Which of the following is not necessary to learn Fire Wall?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Fire Bolt Lv 4:Fire Ball Lv 5:Sight Lv 1:Napalm Beat Lv 4")],
                )?) == 4
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["2. Regaurdless of it's previous attribute, What does the monster's attribute change to when you cast Frost Diver on it?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Water:Earth:Fire:Wind")])?) == 1 {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Raulel",
                    args!["3. When you completely master Napalm Beat, what is the ratio of the increased MATK using that spell?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("1.6 times:1.7 times:2 times:20 times")],
                )?) == 2
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["4. What item do you need when casting Stone Curse?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Red Blood:Blue Gemstone:Yellow Gemstone:Red Gemstone")],
                )?) == 4
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Raulel",
                    args!["5. Which of the following is not required to master Safety Wall?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Napalm Beat Lv 4:Soul Strike Lv 5:Increase SP Recovery Lv 6:Safety Wall Lv 7",
                    )],
                )?) == 3
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["6. Without the INT bonus, what amount of SP is recovered every 10 seconds when you have learned Increase SP Recovery Lv 7?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("14:21:28:35")])?) == 2 {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["7. Using Energy Coat, when you have 50% of your SP remaining, how much SP is used when hit, and what percentage is damage reduced by?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Damage 18% SP1.5%:Damage 18% SP2%:Damage 24% SP1.5%:Damage 24% SP2%")],
                )?) == 2
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Raulel",
                    args!["8. How much SP is consumed and how many times can you avoid attacks when using Safety Wall Lv 6?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("SP 40, 6 times:SP 35, 6 times:SP 40, 7 times:SP 35, 7 times")],
                )?) == 3
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["9. How much SP is needed when using Lv 10 Thunderstorm?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("84:74:64:54")])?) == 2 {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Raulel",
                    args!["10. Which skill is most useful training in the Byalan Dungeon?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Lightning Bolt:Fire Bolt:Cold Bolt:Sight")],
                )?) == 1
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
            } else if subject1 == 2 {
                ctx.lines_as("Raulel", args!["1. Which monster can you obtain a slotted Guard from?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Thief Bug:PecoPeco:Pupa:Kobold (Hammer)")],
                )?) == 3
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Raulel",
                    args!["2. Which of the following is the easiest monster for a low level Mage to hunt?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Flora:Giearth:Golem:Myst")])?) == 1 {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["3. Which monster will not be affected by Stone Curse?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Elder Willow:Evil Druid:Magnolia:Marc")],
                )?) == 2
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Raulel",
                    args!["4. When attacking a Lv 3 water attribute monster with a wind attribute weapon, what is the damage percentage?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("125%:150%:175%:200%")])?) == 4 {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Raulel",
                    args!["5. If a Baby Desert Wolf and a Familiar fought, which one would win?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Baby Desert Wolf:Familiar:Neither:I don't know")],
                )?) == 1
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["6. Which of the following cannot be a Cute Pet?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Poporing:Roda Frog:Smokie:Poison Spore")],
                )?) == 2
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Raulel",
                    args!["7. Choose the monster that is weak against a fire attribute attack."],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Dagger Goblin:Mace Goblin:Morningstar Goblin:Hammer Goblin")],
                )?) == 4
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["8. Which of the following has the highest defense?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Horn:Chonchon:Andre:Caramel")])?) == 4 {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["9. Choose the monster that's of a different species."])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Poring:Mastering:Ghostring:Spore")])?) == 3 {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["10. Which of the following is not an Undead monster?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Drake:Megalodon:Deviace:Khalitzburg")],
                )?) == 3
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
            } else if subject1 == 3 {
                ctx.lines_as("Raulel", args!["1. Which stat is the most important for a Mage?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("INT:AGI:DEX:VIT")])?) == 1 {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["2. Which attribute does not have a 'Bolt' type attack?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Water:Earth:Fire:Wind")])?) == 2 {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["3. Choose the one that does not relate to a Mage."])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Weak physical strength.:Attacks at a distance.:Good at selling stuff.:Magic Defense is high.",
                    )],
                )?) == 3
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["4. Which town is the home of Mages?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Prontera:Morocc:Alberta:Geffen")])?) == 4 {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["5. Which of the following cards has nothing to do with INT?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Andre Egg Card:Soldier Andre Card:Baby Desert Wolf Card:Elder Willow Card",
                    )],
                )?) == 2
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["6. What is the Mage good at compared to other job classes?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Exceptional Vocal Ability:Exceptional Acting Ability:Exceptional Dance Skills:Exceptional Magic Skills",
                    )],
                )?) == 4
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["7. What is the INT bonus at Job Lv 40 for a Mage?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("8:7:6:5")])?) == 4 {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["8. Which item can a Mage not equip?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Knife:Boys Cap:Sandle:Eye of Dullahan")],
                )?) == 2
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Raulel",
                    args!["9. Which of the following is the catalyst when making the Mage test solution 3?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Blue Gemstone:Red Gemstone:Yellow Gemstone:Red Blood")],
                )?) == 1
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
                ctx.lines_as("Raulel", args!["10. Which card is irrelevant to magic?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Marduk Card:Magnolia Card:Willow Card:Maya Card")],
                )?) == 2
                {
                    l_wizard_t = (l_wizard_t.clone() + Val::from(10));
                }
            }
            ctx.mes("[Raulel]")?;
            if ctx.var("wiz_q").get()? == 4 {
                ctx.mes("Good job, you finished answered all the questions... Go buy yourself some potions or something if you have the Zeny. Haha...")?;
                ctx.next()?;
                ctx.lines_as(
                    "Raulel",
                    args![((Val::from("Your score is... ") + l_wizard_t.clone()) + Val::from("points....."))],
                )?;
                if l_wizard_t.clone() == 100 {
                    ctx.var("wiz_q").set(Val::from(5))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(9016), Val::from(9017)])?;
                    ctx.lines(args![
                        "Hahahahahahah~ Well done, you passed the second test.",
                        "It wasn't done in one try like mine was, but I'll let you slide..."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Raulel",
                        args![
                            "*sneeze* Don't relax just yet, there's still the matter of the third and final test.",
                            "I advise you to rest a bit while the final test is prepared. Your gonna need it. Hahahahaha~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if l_wizard_t.clone() == 90 {
                    ctx.var("wiz_q").set(Val::from(5))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(9016), Val::from(9017)])?;
                    ctx.lines(args![
                        "Hahaha~ Since you only missed one problem, you passed the second test.",
                        "It wasn't done in one try like mine was, but I'll let you slide..."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Raulel",
                        args![
                            "*sneeze* Don't relax just yet, there's still the matter of the third and final test.",
                            "I advise you to rest a bit while the final test is prepared. Your gonna need it. Hahahahaha~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if l_wizard_t.clone() == 80 {
                    ctx.var("wiz_q").set(Val::from(5))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(9016), Val::from(9017)])?;
                    ctx.lines(args![
                        "Sheez... You didn't do very well, but you passed the second test.",
                        "It wasn't done in one try like mine was, but I'll let you slide..."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Raulel",
                        args![
                            "*sneeze* Don't relax just yet, there's still the matter of the third and final test.",
                            "I advise you to rest a bit while the final test is prepared. Your gonna need it. Hahahahaha~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.mes("You failed. Go study some more!")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Raulel",
                        args![
                            "*cough* *cough* Did you really think you could become a Wizard with such a mediocre level like yours?",
                            "Get lost! If you were a Wizard right now, the monsters that I fight, would eat you up in no time!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            ctx.mes("Hmmm...Good job, you finished answering all the questions, go buy yourself some potions or something, thats IF you have the Zeny. Hahahahahahahah~")?;
            ctx.next()?;
            ctx.lines_as(
                "Raulel",
                args![((Val::from("Your score is... ") + l_wizard_t.clone()) + Val::from(" points!"))],
            )?;
            if l_wizard_t.clone() == 100 {
                ctx.var("wiz_q").set(Val::from(5))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(9016), Val::from(9017)])?;
                ctx.mes("*cough* *Cough* Well done, you passed the second test.")?;
                ctx.next()?;
                ctx.lines_as(
                    "Raulel",
                    args![
                        "Hahahaha~ Don't relax just yet, there's still the third test.",
                        "*sneeze* I advise you to rest a bit while the final test is prepared...Hahahahah~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if l_wizard_t.clone() == 90 {
                ctx.var("wiz_q").set(Val::from(5))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(9016), Val::from(9017)])?;
                ctx.mes("Hahahaha~ I'll let you slide by since you only missed one problem. You passed the second test.")?;
                ctx.next()?;
                ctx.lines_as(
                    "Raulel",
                    args![
                        "Hahahaha~ Don't relax just yet, there's still the third test.",
                        "*sneeze* I advise you to rest a bit while the final test is prepared...Hahahahah~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.var("wiz_q").set(Val::from(4))?;
                ctx.mes("You failed. I will let you come back again...after you've learned more relating to the type of questions I've asked you.")?;
                ctx.next()?;
                ctx.lines_as("Raulel", args!["Tisk...not enough, not enough! Did you really think you could become a Wizard with the little bit of knowledge you have?", "Get lost! If you were a Wizard right now, the monsters I deal with would eat you up in no time!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if ctx.var("wiz_q").get()? == 5 {
            ctx.lines_as(
                "Raulel",
                args!["Ok, hope you got plenty of rest. Hahahahahah~", "Then let's begin the last test."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Raulel",
                args!["Should I explain a little about this final test? It is difficult, I will not hide that from you..."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("No, it's ok, I'm ready.:I would like to listen.")],
            )?) == 1
            {
                ctx.lines_as("Raulel", args!["What a rash person. Your the type that rushes into battle without thinking, what in the world are you doing here instead of with the Prontera Chivalry? Heck, go for it! *cough* Not my fault if you end up dying.", "Just consider yourself a glass cannon...because the monsters are going to break you into pieces. Hahahahahahahahaha~"])?;
                ctx.next()?;
                ctx.var("wiz_q").set(Val::from(6))?;
                ctx.call(
                    Function::SavePoint,
                    vec![Val::from("geffen"), Val::from(120), Val::from(107), Val::from(1), Val::from(1)],
                )?;
                ctx.lines_as(
                    "Raulel",
                    args![
                        "Then, as you wish. I'll send you there right now.",
                        "Oh, if you see a white light at the end of a tunnel, that means your pathetic cause you failed! Hahahahahah~"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("job_wiz"), Val::from(57), Val::from(154)])?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Raulel",
                args![
                    "What a devoted person. Very well, I'll explain.",
                    "No matter how hard this last test may seem, if you do as I say, you can finish it quickly and easily."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Raulel",
                args![
                    "The final test has a total of 3 parts.",
                    "The order is Water Room, Earth Room, Fire Room. In each room, there are monsters of that particular attribute."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Raulel",
                args![
                    "You'll find out what monsters will be there once you go in. If you use attacks with the *sneeze*",
                    "right attribute, it shouldn't be too hard. Hahaha~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Raulel",
                args![
                    "Once you defeat all the monsters within the given time in any one room...",
                    "you'll be moved to the next room."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Raulel", args!["After these three rooms are clear, the testing is over.", "You will become a Wizard which is controlled by Greater Magic Powers! Know this...There is no returning to an easy life."])?;
            ctx.next()?;
            ctx.lines_as(
                "Raulel",
                args![
                    "Hahaha~ You look frightened. You know, it's not too late to turn back and live an easy life.",
                    "If you want, I can send you back to town right now... What do you want to do?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from(
                    "Continue testing.:I want to go back because I have butterflies in my stomach.",
                )],
            )?) == 1
            {
                ctx.var("wiz_q").set(Val::from(6))?;
                ctx.call(
                    Function::SavePoint,
                    vec![Val::from("geffen"), Val::from(120), Val::from(107), Val::from(1), Val::from(1)],
                )?;
                ctx.lines_as(
                    "Raulel",
                    args![
                        "You are indeed, very determined. Ok! Hahahahahaha~",
                        "*Cough* *cough* As you wish, we shall begin the final test!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("job_wiz"), Val::from(57), Val::from(154)])?;
                return Err(Stop::End);
            }
            ctx.var("wiz_q").set(Val::from(6))?;
            ctx.lines_as(
                "Raulel",
                args![
                    "Good thinking. This is a better choice for you. Hahahahah~",
                    "Go back and live a easy life, Greater Magic is a force that should not be wield by types like yourself."
                ],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("geffen"), Val::from(120), Val::from(110)])?;
            return Err(Stop::End);
        } else if ctx.var("wiz_q").get()? == 6 {
            if ctx.var("wiz_q2").get()? == 6 {
                ctx.lines_as(
                    "Raulel",
                    args![
                        "Hahahahahaha~ I've never seen anyone so...sooo...*sneeze* tenacious as you.",
                        "So you want to try again eh? Even though I've ridiculed you for your failures before??"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Raulel", args!["Ok then, here's a proposition. Since you're probably worn out as it is, and I can clearly see the lust for Greater Magic burning in your eyes...", "Hahahahaha~ yeah! Go bring me a ^3355FFWorn Out Scroll^000000."])?;
                ctx.next()?;
                ctx.var("wiz_q2").set((ctx.var("wiz_q2").get()? + Val::from(1)))?;
                ctx.lines_as(
                    "Raulel",
                    args![
                        "If not, you can take the test again...",
                        "Well, I'll send you to take the test for now. Hahahaha~"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("job_wiz"), Val::from(57), Val::from(154)])?;
                return Err(Stop::End);
            } else if ctx.var("wiz_q2").get()?.number()? > 6 {
                ctx.lines_as(
                    "Raulel",
                    args![
                        "Oh! So you're back? Hahahahaha~",
                        "*Cough* Cough* Do you want to take the test again? Or did you bring the ^3355FFWorn Out Scroll^000000?"
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Continue the test.:Worn Out Scroll...")],
                )?) == 1
                {
                    ctx.call(
                        Function::SavePoint,
                        vec![Val::from("geffen"), Val::from(120), Val::from(107), Val::from(1), Val::from(1)],
                    )?;
                    ctx.lines_as(
                        "Raulel",
                        args![
                            "Hahaha~ Ok, at least you have some spirit.",
                            "I'll send you in again, try dying once more will yah? Hahahahahahahahaha~"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("job_wiz"), Val::from(57), Val::from(154)])?;
                    return Err(Stop::End);
                }
                if ctx.call(Function::CountItem, vec![Val::from(618)])?.number()? > 0 {
                    ctx.call(Function::DelItem, vec![Val::from(618), Val::from(1)])?;
                    ctx.lines_as(
                        "Raulel",
                        args![
                            "Hahahahahahahaha~ *Cough* *cough* So you ended up bringing one of these eh? Good job...",
                            "I think I can continue my research with this..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("wiz_q2").set(Val::from(0))?;
                    ctx.var("wiz_q").set(Val::from(7))?;
                    ctx.lines_as("Raulel", args!["Even though your not Grade A Wizard material, I can tell your serious about wanting the Greater Magic. I'll tell Catherine that you passed. Hahahahahahahahah~", "You went through a lot of trouble here, and that is the true purpose for us selecting Wizards. Only those who will devote themselves to the art will ever become Wizards. Good luck to you. Become much Stronger. Hahahahahaha~"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            ctx.lines_as(
                "Raulel",
                args![
                    "*sneeze* What? You want to take the test again?",
                    "Geez...you already failed the battle test! Hahahahahahahaha~ So you like magic that much, eh?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Raulel",
                args![
                    "Since your so weak that you can't finish this final test on your own...you need a separate test to help you out.",
                    "*Cough* If you can't pass the battle test, then do a good job with this one. Hahahahahahah~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Raulel",
                args!["Well, you better answer these problems if you plan on becoming a Wizard. Hahahahahahaha~"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Raulel",
                args!["1. Choose the monster with a different attribute than the others."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Mantis:Cornutus:Giearth:Caramel")])?) == 2 {
                l_wizard_t = (l_wizard_t.clone() + Val::from(20));
            }
            ctx.lines_as("Raulel", args!["2. Choose the monster that is not a looting one."])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Yoyo:Magnolia:Metaller:Zerom")])?) == 4 {
                l_wizard_t = (l_wizard_t.clone() + Val::from(20));
            }
            ctx.lines_as("Raulel", args!["3. Which of these monsters does not recognize casting?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Marina:Vitata:Scorpion:Giearth")])?) == 1 {
                l_wizard_t = (l_wizard_t.clone() + Val::from(20));
            }
            ctx.lines_as(
                "Raulel",
                args!["4. Choose the spell that would be efficient against a Marine Sphere."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Cold Bolt:Fire Bolt:Lightning Bolt:Stone Curse")],
            )?) == 3
            {
                l_wizard_t = (l_wizard_t.clone() + Val::from(20));
            }
            ctx.lines_as("Raulel", args!["5. Choose the monster that can move."])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Hydra:Madragora:Greatest General:Frilldora")],
            )?) == 4
            {
                l_wizard_t = (l_wizard_t.clone() + Val::from(20));
            }
            ctx.lines_as("Raulel", args!["*pfft* Do it right, so I don't have to ask again."])?;
            ctx.next()?;
            ctx.lines_as(
                "Raulel",
                args![((Val::from("You got ") + l_wizard_t.clone()) + Val::from(" points."))],
            )?;
            if l_wizard_t.clone() == 100 {
                ctx.mes("Hahahahahaha~ *Cough* *cough* If you can answer all these questions correctly, how is it you can't do well in battles??")?;
                ctx.next()?;
            } else if l_wizard_t.clone() == 80 {
                ctx.lines(args!["Eh, soso...", "I'll let you retake the test."])?;
                ctx.next()?;
            } else {
                ctx.mes("You failed! Go study some more!")?;
                ctx.next()?;
                ctx.lines_as(
                    "Raulel",
                    args!["You lack something...*sneez*...like intelligence. That's why you keep on failing. Hahahahahahahaha~"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Begin the test please.:Can I get another explanation?")],
            )?) == 1
            {
                ctx.lines_as(
                    "Raulel",
                    args![
                        "Nobody is going to help you become a Wizard. Hahahahahahahaha~",
                        "*Cough* *cough* No point in crying after dying..."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(100)])?;
                ctx.lines_as("Raulel", args!["Then, as you wish. I'll send you to fight.", "Oh! If you see some tall pearly gates and hear a booming deep voice from behind it, that means that your a failure when it comes to Magic. Hahahahahahahahaha~"])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("job_wiz"), Val::from(57), Val::from(154)])?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Raulel",
                args![
                    "*Cough* *cough* Then I shall explain.",
                    "The test may be hard, but just do as I tell you and it shouldn't be a problem."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Raulel", args!["There are 3 parts to this final test.", "The order is...*sneez*...the Water Room, Earth Room, and then the Fire Room. Each room has monsters of that attribute in it."])?;
            ctx.next()?;
            ctx.lines_as(
                "Raulel",
                args![
                    "You'll see what monsters they are when you enter.",
                    "If you use the appropriate spells against them, it shouldn't be that difficult. Hahahahahahaha~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Raulel",
                args![
                    "Within the given time, if you defeat all the monsters...",
                    "you will be sent to the next room."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Raulel", args!["After that, the test is over.", "You will then become a Wizard controlled by Greater Magic powers! There is no coming back to the easy life you have known thus far."])?;
            ctx.next()?;
            ctx.lines_as("Raulel", args!["Hahahahaha~ You look frightened. It's not too late you know.", "*Cough* *cough* You can give up and go back to town! Just forget about the Greater Magic and live a normal life. What do yah say?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Continue with the test.:I'm too scared, I would like to quit.")],
            )?) == 1
            {
                ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(100)])?;
                ctx.lines_as(
                    "Raulel",
                    args![
                        "This time when you die, don't come back crying. Hahahahahahahahah~ *Cough *cough*",
                        "As you wish, let's begin the final test!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("job_wiz"), Val::from(57), Val::from(154)])?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Raulel",
                args![
                    "Comming from you, thats some darn good thinking. That's more a fit for you anyways. Hahahahahahahahaha~",
                    "Go back and live a quiet and peaceful life!"
                ],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("geffen"), Val::from(120), Val::from(110)])?;
            return Err(Stop::End);
        } else if ctx.var("wiz_q").get()? == 7 {
            ctx.lines_as(
                "Raulel",
                args![
                    "You shouldn't have any more business with me as far as I'm concerned.",
                    "But, since your so darned persistent, I'll let you take the test again. Hahahahaha~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Raulel",
                args![
                    "Go! Go and become the Wizard you really want to be.",
                    "And be careful! Greater Magic will always be after you..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn gloomy_wizard(ctx: &Ctx) -> Script {
    gloomy_wizard_body(ctx, Vec::new()).map(|_| ())
}

fn arena_assistant_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Arena Assistant",
        args![
            "Welcome to the Wizard Job Change Arena.",
            "If you would like to take the final test, then please enter the waiting room."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Arena Assistant",
        args![
            "If someone is already taking the test, please wait.",
            "All testing status will be broadcasted, and will begin as soon as the previous tester has gone through."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Arena Assistant",
        args![
            "Each person may take approximately 5 to 10 minutes.",
            "If you would like to leave the arena, please log off anytime."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn arena_assistant(ctx: &Ctx) -> Script {
    arena_assistant_body(ctx, Vec::new()).map(|_| ())
}

fn arena_assistant_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WaitingRoom,
        vec![
            Val::from("Waiting Room"),
            Val::from(20),
            Val::from("Arena Assistant::OnStartArena"),
            Val::from(1),
        ],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn arena_assistant_oninit(ctx: &Ctx) -> Script {
    arena_assistant_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn arena_assistant_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::KillMonster, vec![Val::from("job_wiz"), Val::from("All")])?;
    ctx.call(
        Function::WarpWaitingPc,
        vec![Val::from("job_wiz"), Val::from(114), Val::from(169)],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Water::OnEnable")])?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn arena_assistant_onstartarena(ctx: &Ctx) -> Script {
    arena_assistant_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn arena_assistant_onstart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn arena_assistant_onstart(ctx: &Ctx) -> Script {
    arena_assistant_onstart_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RoomOfWaterStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnMyMobDead,
    OnTimer1000,
    OnTimer2000,
    OnTimer3000,
    OnTimer33000,
    OnTimer63000,
    OnTimer93000,
    OnTimer123000,
    OnTimer153000,
    OnTimer173000,
    OnTimer183000,
    OnTimer184000,
    OnTimer185000,
    OnTimer186000,
}

fn room_of_water_run(ctx: &Ctx, mut step: RoomOfWaterStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RoomOfWaterStep::Start => {
                step = RoomOfWaterStep::OnInit;
                continue 'machine;
            }
            RoomOfWaterStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Water")])?;
                return Err(Stop::End);
            }
            RoomOfWaterStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Room of Water")])?;
                {
                    ctx.var(".mymobs").set(Val::from(8))?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_wiz"),
                            Val::from(129),
                            Val::from(170),
                            Val::from("Obeaune"),
                            Val::from(1044),
                            Val::from(1),
                            Val::from("Room of Water::OnMyMobDead"),
                        ],
                    )?;
                }
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(109),
                        Val::from(174),
                        Val::from("Phen"),
                        Val::from(1158),
                        Val::from(1),
                        Val::from("Room of Water::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(118),
                        Val::from(174),
                        Val::from("Shellfish"),
                        Val::from(1074),
                        Val::from(1),
                        Val::from("Room of Water::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(109),
                        Val::from(165),
                        Val::from("Vadon"),
                        Val::from(1066),
                        Val::from(1),
                        Val::from("Room of Water::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(118),
                        Val::from(165),
                        Val::from("Cornutus"),
                        Val::from(1067),
                        Val::from(1),
                        Val::from("Room of Water::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(101),
                        Val::from(157),
                        Val::from("Marina"),
                        Val::from(1141),
                        Val::from(1),
                        Val::from("Room of Water::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(126),
                        Val::from(157),
                        Val::from("Marin"),
                        Val::from(1242),
                        Val::from(1),
                        Val::from("Room of Water::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(98),
                        Val::from(170),
                        Val::from("Magnolia"),
                        Val::from(1138),
                        Val::from(1),
                        Val::from("Room of Water::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RoomOfWaterStep::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("job_wiz"), Val::from("All")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Water")])?;
                return Err(Stop::End);
            }
            RoomOfWaterStep::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.var("wiz_q2").set((ctx.var("wiz_q2").get()? + Val::from(1)))?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_wiz"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(" has succeeded in eliminating the monsters.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Water#Door::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            RoomOfWaterStep::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("Water Room; The job change test will now proceed."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfWaterStep::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("Time limit is 3 minutes. We will now start the test."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfWaterStep::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("Please eliminate all monsters within the time limit."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfWaterStep::OnTimer33000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("2 minutes and 30 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfWaterStep::OnTimer63000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("2 minutes remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfWaterStep::OnTimer93000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("1 minute and 30 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfWaterStep::OnTimer123000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("1 minute remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfWaterStep::OnTimer153000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("30 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfWaterStep::OnTimer173000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("10 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfWaterStep::OnTimer183000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("Time is up."), ctx.constant("BC_MAP")?],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Water::OnDisable")])?;
                return Err(Stop::End);
            }
            RoomOfWaterStep::OnTimer184000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Room of Water#Failed")])?;
                return Err(Stop::End);
            }
            RoomOfWaterStep::OnTimer185000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("Next candidate, please enter."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfWaterStep::OnTimer186000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Water#Failed")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Water::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Arena Assistant::OnStart")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn room_of_water(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::Start, Vec::new()).map(|_| ())
}

pub fn room_of_water_oninit(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::OnInit, Vec::new()).map(|_| ())
}

pub fn room_of_water_onenable(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn room_of_water_ondisable(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn room_of_water_onmymobdead(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn room_of_water_ontimer1000(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn room_of_water_ontimer2000(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn room_of_water_ontimer3000(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn room_of_water_ontimer33000(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::OnTimer33000, Vec::new()).map(|_| ())
}

pub fn room_of_water_ontimer63000(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::OnTimer63000, Vec::new()).map(|_| ())
}

pub fn room_of_water_ontimer93000(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::OnTimer93000, Vec::new()).map(|_| ())
}

pub fn room_of_water_ontimer123000(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::OnTimer123000, Vec::new()).map(|_| ())
}

pub fn room_of_water_ontimer153000(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::OnTimer153000, Vec::new()).map(|_| ())
}

pub fn room_of_water_ontimer173000(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::OnTimer173000, Vec::new()).map(|_| ())
}

pub fn room_of_water_ontimer183000(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::OnTimer183000, Vec::new()).map(|_| ())
}

pub fn room_of_water_ontimer184000(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::OnTimer184000, Vec::new()).map(|_| ())
}

pub fn room_of_water_ontimer185000(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::OnTimer185000, Vec::new()).map(|_| ())
}

pub fn room_of_water_ontimer186000(ctx: &Ctx) -> Script {
    room_of_water_run(ctx, RoomOfWaterStep::OnTimer186000, Vec::new()).map(|_| ())
}
