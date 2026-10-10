use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn rebarev_doug_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1301), Val::from(3)])? == 0 {
        ctx.mes("- You are carrying too many items! -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(Function::Cutin, vec![Val::from("god_rebeireb"), Val::from(2)])?;
    if ((((ctx.call(Function::CountItem, vec![Val::from(7080)])?.number()? > 3
        && ctx.call(Function::CountItem, vec![Val::from(7081)])?.number()? > 4)
        && ctx.call(Function::CountItem, vec![Val::from(7082)])?.number()? > 3)
        && ctx.call(Function::CountItem, vec![Val::from(7084)])?.number()? > 2)
        && ctx.call(Function::CountItem, vec![Val::from(7085)])?.number()? > 2)
    {
        ctx.lines_as(
            "Rebarev Doug",
            args!["What's this?", "You...! You have", "everything I need to", "create Gleipnir!"],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Make Gleipnir.:Cancel.")])? {
            1 => {
                ctx.lines_as(
                    "Rebarev Doug",
                    args![
                        "I'm the only human on earth",
                        "blessed with the ability to create Gleipnir. Aside from the Dwarves, I'm the only person that can make this item!"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(7080), Val::from(4)])?;
                ctx.call(Function::DelItem, vec![Val::from(7081), Val::from(5)])?;
                ctx.call(Function::DelItem, vec![Val::from(7082), Val::from(4)])?;
                ctx.call(Function::DelItem, vec![Val::from(7084), Val::from(3)])?;
                ctx.call(Function::DelItem, vec![Val::from(7085), Val::from(3)])?;
                ctx.call(Function::GetItem, vec![Val::from(7058), Val::from(1)])?;
                ctx.lines_as("Rebarev Doug", args!["There you go!"])?;
                ctx.next()?;
                ctx.lines_as("Rebarev Doug", args!["Gleipnir is said to be so strong that not even the Fenrir Wolf could break it! I take great pride in being able to create this!"])?;
            }
            2 => {}
            _ => {}
        }
    }
    if runtime::op(&ctx.var("$god1").get()?, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
        ctx.lines_as(
            "Rebarev Doug",
            args![
                "We are Crusaders that have",
                "been training in preparation",
                "for the Holy War that is to come."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rebarev Doug",
            args![
                "As our forefathers have",
                "done a thousand years ago, we shall vanquish the hordes of Demons when the day comes."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rebarev Doug",
            args![
                "If you have any questions about Crusaders, feel free to ask me.",
                "I may be too old and weak for fighting, but I will spread our message as much as I can."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from(
                "What is Holy Cross?:What is Grand Cross?:What is Sacrifice?:What is Gleipnir?",
            )],
        )? {
            1 => {
                ctx.lines_as("Rebarev Doug", args!["Holy Cross is the first manifestation of a Crusader's faith. By making the sign of the cross, Crusader's can inflict a holy attack on their enemies."])?;
                ctx.next()?;
                ctx.lines_as("Rebarev Doug", args!["Monsters that are weak against holiness, particularly the Undead, will be blinded by the light of the Holy Cross. All they can do is wait for holy judgment."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Rebarev Doug",
                    args![
                        "Based on our latest research,",
                        "the Holy Cross can inflict 4.5 times more damage than a normal attack once it is mastered."
                    ],
                )?;
            }
            2 => {
                ctx.lines_as(
                    "Rebarev Doug",
                    args![
                        "Grand Cross...!",
                        "The righteous fury of God is",
                        "given form in this destructive skill."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rebarev Doug",
                    args![
                        "God grants us his power so that",
                        "we may punish the forces of evil, and one's rage is embodied in a giant crucifix of power."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Rebarev Doug", args!["The power of the crucifix of light can damage the enemy three times. Each strike will have five times the strength of your normal attacks."])?;
                ctx.next()?;
                ctx.lines_as("Rebarev Doug", args!["However, when Crusaders use Grand Cross, they must exercise extreme caution. The human body is too weak to fully embrace the will of God."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Rebarev Doug",
                    args![
                        "Therefore, using this skill, will cause the Crusader to suffer from damage. However, this damage can",
                        "be reduced to half with Faith."
                    ],
                )?;
            }
            3 => {
                ctx.lines_as(
                    "Rebarev Doug",
                    args![
                        "In practicing what we preach, Crusaders choose to protect",
                        "others by bearing the suffering of their fellow man."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Rebarev Doug", args!["The Sacrifice skill allows Crusaders to keep their comrades safe from harm by receiving the damage that was intended for them."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Rebarev Doug",
                    args![
                        "It is the greatest demonstration of faith,",
                        "as well as of love for your comrades. Such an attitude is essential for a servant of God!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Rebarev Doug", args!["However, if the people you want to protect stray away from you, you cannot use the Sacrifice skill for them unless they are close to you again."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Rebarev Doug",
                    args!["As you visualize the suffering of those you want to protect, you'll never hesitate to sacrifice yourself."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rebarev Doug",
                    args!["Thank you, God, for granting me with the power to protect my people."],
                )?;
            }
            4 => {
                ctx.lines_as(
                    "Rebarev Doug",
                    args!["Gleipnir is the essence of Megingjard which I have been researching."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rebarev Doug",
                    args!["It's the only binding strong enough that will allow a human to wear Megingjard around his waist."],
                )?;
                ctx.next()?;
                ctx.lines_as("Rebarev Doug", args!["Although I can no longer fight, I've been studying how to create Gleipnir for years. I will be more than willing to create it for you if you can bring me..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Rebarev Doug",
                    args![
                        "^0000FF4 Cat Tread^000000,",
                        "^0000FF5 Woman's Moustache^000000,",
                        "^0000FF4 Root of Stone^000000,",
                        "^0000FF3 Sputum of Bird^000000 and",
                        "^0000FF3 Sinew of Bear^000000."
                    ],
                )?;
            }
            _ => {}
        }
    } else {
        if (runtime::op(&ctx.var("$god1").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true()
            && runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true())
        {
            if ctx.var("BaseLevel").get()?.number()? > 59 {
                if ctx.var("god_eremes").get()? == 0 {
                    ctx.lines_as("Rebarev Doug", args!["..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rebarev Doug",
                        args![
                            "Why are you wandering",
                            "around this sacred place?",
                            "Do you not understand that",
                            "we must prepare for",
                            "the Holy War?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Rebarev Doug", args!["No matter how much I explain, the ignorant never fully understand the importance of our task. The balance of power will shift when we least expect it!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rebarev Doug",
                        args![
                            "I wish I could go back",
                            "to those times when I would",
                            "train Crusaders, rather than",
                            "preach to the ignorant."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rebarev Doug",
                        args![
                            "I miss my Crusaders who",
                            "were enthusiastic to listen to what I had to say, and carried out their orders with loyalty."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Rebarev Doug", args!["I am wonder", "where they have", "all gone now..."])?;
                    ctx.next()?;
                    ctx.lines_as("Rebarev Doug", args!["...", "......", "........."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rebarev Doug",
                        args!["Adventurer, if you have the time to listen to me speak to myself, why don't you perform a task for me?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rebarev Doug",
                        args![
                            "It may be an enriching",
                            "experience for you. It's possible that you'll even have a greater appreciation for Crusaders."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("What is it?:I'm sick and tired of doin' favors.")])? {
                        1 => {
                            ctx.lines_as("Rebarev Doug", args!["I want you to find the members of the 1st Squad in the 3rd Platoon of the 3rd Company. If you happen to encounter them in your travels, please ask them how they are doing."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rebarev Doug",
                                args![
                                    "It shouldn't be that difficult to do and won't be a waste of your time if you are already planning",
                                    "to explore the world."
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Sure, why not.:I'm sorry, but no.")])? {
                                1 => {
                                    ctx.lines_as(
                                        "Rebarev Doug",
                                        args![
                                            "Excellent!",
                                            "All I want you to do is find my old comrades, and inform me of how",
                                            "they are doing."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Rebarev Doug",
                                        args!["However...", "There is one problem.", "I don't where they are."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Rebarev Doug", args!["You could try to find a record of residency changes by citizens of the Rune-Midgarts Kingdom in", "the Prontera Library..."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Rebarev Doug", args!["However, the records of", "Crusader personnel is considered confidential, so I am not sure of whether or not you can view them."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Rebarev Doug", args!["You must understand, my position requires me to remain in this area 24 hours, 7 days a week. I cannot leave without the consent of the upper hierarchy."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Rebarev Doug", args!["However, I know you have the freedom and the time to go wherever you please. I'd appreciate if you would do this for me."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Rebarev Doug",
                                        args!["My old comrades in arms.", "You have no idea how much", "I miss them..."],
                                    )?;
                                    ctx.var("god_eremes").set(Val::from(1))?;
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Rebarev Doug",
                                        args![
                                            "^333333*Sigh...*^000000",
                                            "Please understand that",
                                            "I would not be asking this",
                                            "of you if it were not for",
                                            "the restraints of",
                                            "my position."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Rebarev Doug",
                                        args![
                                            "I have an obligation to this Kingdom to keep my post and",
                                            "inform adventurers about the",
                                            "Crusader class. Because of this",
                                            "awesome responsibility, I must",
                                            "remain here, ever vigilant."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Rebarev Doug",
                                        args!["If you can't understand, then you must be taking your freedom for granted..."],
                                    )?;
                                }
                                _ => {}
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Rebarev Doug",
                                args!["As far as I remember, this is the first time I have asked you to do me a favor."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rebarev Doug",
                                args![
                                    "Unlike you, I know the meaning",
                                    "of responsibility. I remain here, helping inquisitive adventurers for the sake of King and country."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rebarev Doug",
                                args![
                                    "Still...",
                                    "Know this.",
                                    "Although the results may not be immediate, you will always be rewarded when you help others."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Rebarev Doug", args!["Please reconsider", "when you get the chance."])?;
                        }
                        _ => {}
                    }
                } else if (ctx.var("god_eremes").get()?.number()? > 0 && ctx.var("god_eremes").get()?.number()? < 4) {
                    if (ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? > 6 && ctx.var("god_eremes").get()? == 2) {
                        ctx.lines_as(
                            "Rebarev Doug",
                            args![
                                "I wonder how my old",
                                "comrades are doing now.",
                                "I can't even remember",
                                "the last time I saw them..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Rebarev Doug", args!["Hmm...?", "Haven't you left", "to search for them yet?"])?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Where should I go?:I am about to leave.:What do you mean by final mission?",
                            )],
                        )? {
                            1 => {
                                ctx.lines_as(
                                    "Rebarev Doug",
                                    args![
                                        "Well...",
                                        "You should be able",
                                        "find some clue as to",
                                        "where to find them through",
                                        "^0000FFThe Platoon Records^000000 in",
                                        "the Prontera Library."
                                    ],
                                )?;
                            }
                            2 => {
                                ctx.lines_as(
                                    "Rebarev Doug",
                                    args![
                                        "Oh, you are...",
                                        "I wonder what they",
                                        "have been doing since",
                                        "our ^0000FFfinal mission^000000..."
                                    ],
                                )?;
                            }
                            3 => {
                                ctx.lines_as(
                                    "Rebarev Doug",
                                    args![
                                        "Final mission...?",
                                        "Well, I can't recall",
                                        "everything at the moment,",
                                        "but it was the reason why",
                                        "our squad broke up."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rebarev Doug",
                                    args![
                                        "We were a great team.",
                                        "But in the military system,",
                                        "you need to follow the orders",
                                        "of your superiors..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Rebarev Doug", args!["If only...", "^660000He^000000 hadn't interfered..."])?;
                                ctx.next()?;
                                ctx.lines_as("Rebarev Doug", args!["..."])?;
                                ctx.next()?;
                                ctx.lines_as("Rebarev Doug", args!["...", "......"])?;
                                ctx.next()?;
                                ctx.lines_as("Rebarev Doug", args!["Let's not talk about that. What's important that our squad was disbanded because of we failed our final mission. Now, will you please go find ^0000FFThe 3rd Platoon Records^000000 in the Prontera Library for me?"])?;
                                ctx.var("god_eremes").set(Val::from(3))?;
                                ctx.next()?;
                                ctx.lines_as("Rebarev Doug", args!["Remember, it might be helpful to ask the librarian for the file with a record on ^660000the 1st squad's final mission^000000."])?;
                            }
                            _ => {}
                        }
                    } else {
                        ctx.lines_as(
                            "Rebarev Doug",
                            args![
                                "I wonder how my old",
                                "comrades are doing now.",
                                "I can't even remember",
                                "the last time I saw them..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Rebarev Doug", args!["Hmm...?", "Haven't you left", "to search for them yet?"])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Where should I go?:I am about to leave.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Rebarev Doug",
                                    args![
                                        "Well...",
                                        "You should be able",
                                        "find some clue as to",
                                        "where to find them through",
                                        "^0000FFThe 3rd Platoon Records^000000 in",
                                        "the Prontera Library."
                                    ],
                                )?;
                            }
                            2 => {
                                ctx.lines_as(
                                    "Rebarev Doug",
                                    args![
                                        "Oh, you are...",
                                        "I wonder what they",
                                        "have been doing since",
                                        "our ^0000FFfinal mission^000000..."
                                    ],
                                )?;
                                ctx.var("god_eremes").set(Val::from(2))?;
                            }
                            _ => {}
                        }
                    }
                } else {
                    if (ctx.var("god_eremes").get()?.number()? > 3 && ctx.var("god_eremes").get()?.number()? < 18) {
                        ctx.lines_as(
                            "Rebarev Doug",
                            args!["Huh...?", "The librarian", "didn't let you", "read the records?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Rebarev Doug", args!["He may have responsibility,", "but at the end of the day, he's just like you and me. You can find some way to convince him to help you, I'm sure."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rebarev Doug",
                            args![
                                "Please, I beg you,",
                                "would you find out",
                                "what happened to the",
                                "rest of the 1st Squad?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Rebarev Doug", args!["Oh...", "It might be helpful to know that we were the ^660000Crusader Third Company, Third Platoon, First Squad^000000."])?;
                    } else if (ctx.var("god_eremes").get()?.number()? > 17 && ctx.var("god_eremes").get()?.number()? < 20) {
                        if (((((ctx.var("god_megin_1").get()?.number()? > 0 || ctx.var("god_megin_2").get()?.number()? > 0)
                            || ctx.var("god_megin_3").get()?.number()? > 0)
                            || ctx.var("god_megin_4").get()?.number()? > 0)
                            || ctx.var("god_megin_5").get()?.number()? > 0)
                            || ctx.var("god_megin_6").get()?.number()? > 0)
                        {
                            ctx.lines_as("Rebarev Doug", args!["Oh...", "So did you meet them?", "Are they all okay?"])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^3355FFYou tell Rebarev Doug",
                                "that although they are",
                                "suffering from migraines",
                                "and memory loss, the rest",
                                "of the 1st Squad is fine."
                            ])?;
                        } else {
                            ctx.lines(args![
                                "^3355FFYou tell Rebarev Doug",
                                "that you have read the",
                                "3rd Platoon records and",
                                "explain what you have",
                                "managed to learn.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rebarev Doug",
                                args![
                                    "Oh, I'm glad to hear that",
                                    "you were able to read the",
                                    "records! But I don't trust",
                                    "what's written on paper.",
                                    "After all, it may be",
                                    "out of date..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Rebarev Doug", args!["If it's possible, I hope you can meet the rest of the 1st Squad face to face, so I can know for sure that they're all alright."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rebarev Doug",
                                args![
                                    "A man's life is too",
                                    "short and his friends",
                                    "are too few. Please find",
                                    "out how my old comrades",
                                    "are doing for me."
                                ],
                            )?;
                        }
                    } else {
                        if (ctx.var("god_eremes").get()?.number()? > 19 && ctx.var("god_eremes").get()?.number()? < 23) {
                            ctx.lines_as(
                                "Rebarev Doug",
                                args![
                                    "Welcome back~",
                                    "It's been a while since I've last seen you. Have you met the rest",
                                    "of the 1st Squad?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Rebarev Doug", args!["Hm...?", "What's that", "strange look for?"])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Confront Rebarev Doug.:Act unsuspicious.")])? {
                                1 => {
                                    ctx.mes("^3355FFYou confront Rebarev Doug about the memory problems and migraines of the 1st Squad...^000000")?;
                                    ctx.next()?;
                                    ctx.mes("^3355FFThinking about Royal Myst's story and the mentions of Egnigem begin to jumble your thoughts...^000000")?;
                                    ctx.next()?;
                                    ctx.lines_as("Rebarev Doug", args!["What are you talking about? I don't understand, you're talking about lots of different things at once!"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Rebarev Doug",
                                        args![
                                            "Don't look at me like that. If you're done speaking, please",
                                            "excuse me and tell me your news some other time."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Rebarev Doug",
                                        args![
                                            "I appreciate that you've delivered news of my old comrades to me.",
                                            "Now, I need to continue my research under the command of his Majesty."
                                        ],
                                    )?;
                                    ctx.var("god_eremes").set(Val::from(21))?;
                                }
                                2 => {
                                    ctx.lines(args![
                                        "^3355FFYou keep your suspicions to yourself and tell Rebarev Doug about the members of",
                                        "the 1st Squad.^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.mes("^3355FFOf course, you withhold some details and only tell him what he seems to want to hear.^000000.")?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Rebarev Doug",
                                        args!["Great!", "I'm glad to hear", "they're doing so well!", "Ha ha! Hahahahahahaahah!"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Rebarev Doug", args!["I appreciate that you've delivered news of my old comrades to me.", "Now, I need to continue my research under the command of his Majesty. Once again, thank you for your help."])?;
                                    ctx.var("god_eremes").set(Val::from(22))?;
                                }
                                _ => {}
                            }
                        } else if (ctx.var("god_eremes").get()?.number()? > 22 && ctx.var("god_eremes").get()?.number()? < 25) {
                            ctx.lines(args![
                                "^3355FFYou confront",
                                "Rebarev Doug with the",
                                "information you learned",
                                "from Egnigem.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^3355FFHowever, he didn't",
                                "seem the least bit agitated. In fact, he exuded a calmness that makes you feel nervous.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as("Rebarev Doug", args!["It's far too late to talk about that, though I feel sorry for you for knowing too much. At this point, there's nothing you can prove."])?;
                            ctx.next()?;
                            ctx.lines_as("Rebarev Doug", args!["Don't you get it?", "The elite and powerful rule this world. I am untouchable! And the weak and the lowly can rot in hell! Heh heh!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rebarev Doug",
                                args!["Do you hate me?", "Do you really hate me?", "Take a look at this face."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rebarev Doug",
                                args![
                                    "This is the",
                                    "face of authority!",
                                    "Go ahead and report me to the judge in charge of all military crimes! He's right behind me!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rebarev Doug",
                                args![
                                    "If at least 100 people report the same crime, then maybe he'll hold",
                                    "a trial. But do you really want to reveal this to the public?"
                                ],
                            )?;
                        } else if (ctx.var("god_eremes").get()?.number()? > 23 && ctx.var("god_eremes").get()?.number()? < 26) {
                            ctx.lines_as(
                                "Rebarev Doug",
                                args!["I didn't think", "you'd actually do it.", "But you'll be sorry later..."],
                            )?;
                        } else {
                            ctx.lines_as("Rebarev Doug", args!["Don't you get it?", "The elite and powerful rule this world. I am untouchable! And the weak and the lowly can rot in hell! Heh heh!"])?;
                        }
                    }
                }
            } else {
                ctx.lines_as(
                    "Rebarev Doug",
                    args!["You're not fit", "to even speak to me.", "Even if you are an adventurer..."],
                )?;
            }
        } else {
            ctx.lines_as(
                "Rebarev Doug",
                args![
                    "We are Crusaders that have",
                    "been training in preparation",
                    "for the Holy War that is to come."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rebarev Doug",
                args![
                    "As our forefathers have",
                    "done a thousand years ago, we shall vanquish the hordes of Demons when the day comes."
                ],
            )?;
        }
    }
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from("god_rebeireb"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn rebarev_doug(ctx: &Ctx) -> Script {
    rebarev_doug_body(ctx, Vec::new()).map(|_| ())
}

fn crusader_god_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (runtime::op(&ctx.var("$god1").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true()
        && runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true())
    {
        if (ctx.var("god_eremes").get()?.number()? > 22 && ctx.var("god_eremes").get()?.number()? < 25) {
            ctx.lines_as(
                "Max Von Shedough",
                args![
                    "Welcome, friend!",
                    "Here in the Prontera Castle, we Crusaders are busily preparing for the Holy War that is to come."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Max Von Shedough",
                args![
                    "Let me introduce myself.",
                    "My name is Max Von Shedough, the military judge! Do you have any business with me?"
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFYou handed",
                "Max Von Shedough",
                "your petition against",
                "Rebarev Doug based on",
                "your investigation",
                "with Egnigem.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Max Von Shedough",
                args![
                    "Hm? This is serious.",
                    "Alright, I will investigate your claim to the best of my ability."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Max Von Shedough", args!["However, as of now, the petition you've just given me is considered classified information. Please keep this a military secret."])?;
            ctx.var("$god2").set((ctx.var("$god2").get()? + Val::from(1)))?;
            if ctx.var("$god2").get()?.loosely_equals(&ctx.var("$@god_check1").get()?) {
                ctx.call(
                    Function::Announce,
                    vec![Val::from("The 2nd seal of [Megingjard] has appeared."), ctx.constant("BC_ALL")?],
                )?;
            } else if ctx.var("$god2").get()?.loosely_equals(&ctx.var("$@god_check2").get()?) {
                if (((ctx.var("$god1").get()?.loosely_equals(&ctx.var("$@god_check2").get()?)
                    && ctx.var("$god2").get()?.loosely_equals(&ctx.var("$@god_check2").get()?))
                    && ctx.var("$god3").get()?.loosely_equals(&ctx.var("$@god_check2").get()?))
                    && ctx.var("$god4").get()?.loosely_equals(&ctx.var("$@god_check2").get()?))
                {
                    ctx.call(
                        Function::Announce,
                        vec![
                            Val::from("Four seals have been released at the same time with the seal of [Megingjard]."),
                            ctx.constant("BC_ALL")?,
                        ],
                    )?;
                } else {
                    ctx.call(
                        Function::Announce,
                        vec![
                            Val::from("The 2nd seal of [Megingjard] has been released."),
                            ctx.constant("BC_ALL")?,
                        ],
                    )?;
                }
            }
            if ctx.var("god_eremes").get()? == 23 {
                ctx.var("god_eremes").set(Val::from(25))?;
            } else if ctx.var("god_eremes").get()? == 24 {
                ctx.var("god_eremes").set(Val::from(26))?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("god_eremes").get()?.number()? > 26 {
            ctx.lines_as(
                "Max Von Shedough",
                args!["Unfortunately, I'm not sure if it's possible to hold a trial against Rebarev Doug."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Max Von Shedough",
                args![
                    "Regrettably, it seems that he still has too much influence. Still, let me assure you that I'll do everything I can..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Max Von Shedough",
                args!["But count on me,", "I'll be doing my", "best to get him indicted."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Max Von Shedough",
                args![
                    "Welcome, friend!",
                    "Here in the Prontera Castle, we Crusaders are busily preparing for the Holy War that is to come."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Max Von Shedough",
                args![
                    "Let me introduce myself.",
                    "My name is Max Von Shedough, the military judge! Sadly, even we Crusaders are not immune to corruption..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Max Von Shedough",
            args![
                "Welcome, friend!",
                "Here in the Prontera Castle, we Crusaders are busily preparing for the Holy War that is to come."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Max Von Shedough",
            args![
                "Let me introduce myself.",
                "My name is Max Von Shedough,",
                "the military judge! Sadly, even we Crusaders are not immune to corruption..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn crusader_god(ctx: &Ctx) -> Script {
    crusader_god_body(ctx, Vec::new()).map(|_| ())
}

fn a_file_megin1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("god_eremes").get()? == 12 {
        ctx.lines(args!["^3355FFYou have found", "^660000The 3rd Platoon Records^3355FF!^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("god_eremes").get()?.number()? > 6 && ctx.var("god_eremes").get()?.number()? < 12) {
        if (ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? > 6 && ctx.var("god_eremes").get()?.number()? > 6) {
            ctx.mes("^3355FFYou see a shelf filled with many files. You begin searching through them, one by one.^000000")?;
            ctx.var("god_eremes").set((ctx.var("god_eremes").get()? + Val::from(1)))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.mes("^3355FFYou see a shelf filled with many files. You begin searching through them, one by one.^000000")?;
            ctx.close_window()?;
        }
    } else {
        if ctx.var("god_eremes").get()?.number()? < 7 {
            ctx.lines_as(
                "Librarian Jekan",
                args![
                    ((Val::from("I'm sorry ")
                        + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            Val::from("sir")
                        } else {
                            Val::from("ma'am")
                        }))
                        + Val::from(",")),
                    "but special authorization is required to browse that section. Otherwise, it's off limits."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Librarian Jekan", args!["W-wait...!", "That section", "is off limits!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn a_file_megin1(ctx: &Ctx) -> Script {
    a_file_megin1_body(ctx, Vec::new()).map(|_| ())
}

fn a_file_megin2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("god_eremes").get()? == 12 {
        ctx.mes("You have found ^0000FFThe 3rd Platoon Records^000000!")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("god_eremes").get()?.number()? > 6 && ctx.var("god_eremes").get()?.number()? < 12) {
        if (ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? > 6 && ctx.var("god_eremes").get()?.number()? > 6) {
            ctx.mes("^3355FFYou see a shelf filled with many files. You begin searching through them, one by one.^000000")?;
            ctx.var("god_eremes").set((ctx.var("god_eremes").get()? + Val::from(1)))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.mes("^3355FFYou see a shelf filled with many files. You begin searching through them, one by one.^000000")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("god_eremes").get()?.number()? < 7 {
            ctx.lines_as("Librarian Jekan", args!["W-wait...!", "That section", "is off limits!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Librarian Jekan", args!["W-wait...!", "That section", "is off limits!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn a_file_megin2(ctx: &Ctx) -> Script {
    a_file_megin2_body(ctx, Vec::new()).map(|_| ())
}

fn a_file_megin3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("god_eremes").get()? == 12 {
        ctx.lines(args!["You have found", "^0000FFThe 3rd Platoon Records^000000!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("god_eremes").get()?.number()? > 6 && ctx.var("god_eremes").get()?.number()? < 12) {
        if (ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? > 6 && ctx.var("god_eremes").get()?.number()? > 6) {
            ctx.mes("^3355FFYou see a shelf filled with many files. You begin searching through them, one by one.^000000")?;
            ctx.var("god_eremes").set((ctx.var("god_eremes").get()? + Val::from(1)))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.mes("^3355FFYou see a shelf filled with many files. You begin searching through them, one by one.^000000")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("god_eremes").get()?.number()? < 7 {
            ctx.lines_as("Librarian Jekan", args!["W-wait...!", "That section", "is off limits!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Librarian Jekan", args!["W-wait...!", "That section", "is off limits!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn a_file_megin3(ctx: &Ctx) -> Script {
    a_file_megin3_body(ctx, Vec::new()).map(|_| ())
}

fn a_file_megin4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("god_eremes").get()? == 12 {
        ctx.mes("You have found ^0000FFThe 3rd Platoon Records^000000!")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("god_eremes").get()?.number()? > 6 && ctx.var("god_eremes").get()?.number()? < 12) {
        if (ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? > 6 && ctx.var("god_eremes").get()?.number()? > 6) {
            ctx.mes("^3355FFYou see a shelf filled with many files. You begin searching through them, one by one.^000000")?;
            ctx.var("god_eremes").set((ctx.var("god_eremes").get()? + Val::from(1)))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.mes("^3355FFYou see a shelf filled with many files. You begin searching through them, one by one.^000000")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("god_eremes").get()?.number()? < 7 {
            ctx.lines_as("Librarian Jekan", args!["W-wait...!", "That section", "is off limits!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Librarian Jekan", args!["W-wait...!", "That section", "is off limits!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn a_file_megin4(ctx: &Ctx) -> Script {
    a_file_megin4_body(ctx, Vec::new()).map(|_| ())
}

fn a_file_megin5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("god_eremes").get()? == 12 {
        ctx.lines(args!["You have found", "^0000FFThe 3rd Platoon Records^000000!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("god_eremes").get()?.number()? > 6 && ctx.var("god_eremes").get()?.number()? < 12) {
        if (ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? > 6 && ctx.var("god_eremes").get()?.number()? > 6) {
            ctx.mes("^3355FFYou see a shelf filled with many files. You begin searching through them, one by one.^000000")?;
            ctx.var("god_eremes").set((ctx.var("god_eremes").get()? + Val::from(1)))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.mes("^3355FFYou see a shelf filled with many files. You begin searching through them, one by one.^000000")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("god_eremes").get()?.number()? < 7 {
            ctx.lines_as("Librarian Jekan", args!["W-wait...!", "That section", "is off limits!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Librarian Jekan", args!["W-wait...!", "That section", "is off limits!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn a_file_megin5(ctx: &Ctx) -> Script {
    a_file_megin5_body(ctx, Vec::new()).map(|_| ())
}

fn librarian_megin_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if (runtime::op(&ctx.var("$god1").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true()
        && runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true())
    {
        if (ctx.var("god_eremes").get()?.number()? > 2 && ctx.var("god_eremes").get()?.number()? < 7) {
            ctx.lines_as(
                "Librarian Jekan",
                args!["Ah, please do", "not touch the files", "in this section."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Librarian Jekan",
                args![
                    "Some of the information",
                    "here is classified, and it is therefore prohibited by the Prontera Military to browse",
                    "through them."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Librarian Jekan", args!["I'm currently in the process of searching for the classified files in that section, so that I can separate it from the information that is public domain."])?;
            ctx.next()?;
            ctx.lines_as(
                "Librarian Jekan",
                args![
                    "I'd highly",
                    "appreciate it",
                    "if you ^660000didn't^000000",
                    "interfere with my work."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Librarian Jekan",
                args![
                    "Darn it...!",
                    "I can't read anything when the light is this dim! It's bad enough my eyes have gone bad..."
                ],
            )?;
            'l1: loop {
                if !(true) {
                    break 'l1;
                }
                'b1: {
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "You have bad eyes?:I want to read some documents.:Let me help you find those files...:What kind of files are you looking for?",
                        )],
                    )? {
                        1 => {
                            ctx.lines_as(
                                "Librarian Jekan",
                                args![
                                    "I've worked",
                                    "under dim light",
                                    "for so long, my eyes have",
                                    "gone bad. They should get me ^0000FFa new light^000000, otherwise I'll go blind sooner or later..."
                                ],
                            )?;
                            if (ctx.var("god_eremes").get()? == 5
                                && (ctx.call(Function::CountItem, vec![Val::from(2203)])?.number()? > 0
                                    || ctx.call(Function::CountItem, vec![Val::from(1041)])?.number()? > 0))
                            {
                                ctx.next()?;
                                ctx.lines_as("Librarian Jekan", args!["Hey, that's some pretty useful stuff that you've got with you. Do you mind letting me borrow it for a while? It'll help me in finding those files..."])?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("Yes, I mind!:No, I don't mind.")])? {
                                    1 => {
                                        ctx.lines_as("Librarian Jekan", args!["^333333*Sniff*^000000", "N-never mind..."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.lines_as(
                                            "Librarian Jekan",
                                            args!["Oh, you are so kind!", "Y-you really do want", "to help me, don't you?"],
                                        )?;
                                        if (ctx.call(Function::CountItem, vec![Val::from(2203)])?.is_true()
                                            && ctx.call(Function::CountItem, vec![Val::from(1041)])?.is_true())
                                        {
                                            ctx.call(
                                                Function::DelItem,
                                                vec![Val::from(1041), ctx.call(Function::CountItem, vec![Val::from(1041)])?],
                                            )?;
                                            ctx.call(Function::DelItem, vec![Val::from(2203), Val::from(1)])?;
                                        } else if ctx.call(Function::CountItem, vec![Val::from(2203)])?.is_true() {
                                            ctx.call(Function::DelItem, vec![Val::from(2203), Val::from(1)])?;
                                        } else if ctx.call(Function::CountItem, vec![Val::from(1041)])?.is_true() {
                                            ctx.call(
                                                Function::DelItem,
                                                vec![Val::from(1041), ctx.call(Function::CountItem, vec![Val::from(1041)])?],
                                            )?;
                                        }
                                        ctx.var("god_eremes").set(Val::from(6))?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Librarian Jekan",
                                            args!["Thank you...", "Now I won't have to", "worry so much about", "finding that file."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Librarian Jekan",
                                            args!["If it's alright,", "would you help me", "look for it...?"],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            } else if ctx.var("god_eremes").get()?.number()? > 5 {
                                ctx.next()?;
                                ctx.lines_as(
                                    "Librarian Jekan",
                                    args![
                                        "Thank you...",
                                        "The stuff you let",
                                        "me borrow will really",
                                        "help me in looking for",
                                        "all of those files..."
                                    ],
                                )?;
                            }
                        }
                        2 => {
                            if ctx.var("god_eremes").get()? == 6 {
                                ctx.lines_as(
                                    "Librarian Jekan",
                                    args![
                                        "Read some documents?",
                                        "Well, you've come here",
                                        "to the Library. That's",
                                        "a good start."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Librarian Jekan",
                                    args![
                                        "Well, as long as",
                                        "you know what kind",
                                        "of document you're",
                                        "looking for, you might",
                                        "be able to find it."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Librarian Jekin",
                                    args!["I've got to find that", "classified file as soon", "as I can!"],
                                )?;
                                ctx.close_window()?;
                                ctx.var("god_eremes").set(Val::from(7))?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as("Librarian Jekan", args!["Hmm...?", "Hey, haven't", "I told you already?"])?;
                                ctx.next()?;
                                ctx.lines_as("Librarian Jekan", args!["The public is prohibited from browsing the files in this section! How many times do I need to repeat myself?"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        3 => {
                            if ctx.var("god_eremes").get()? == 4 {
                                ctx.lines_as("Librarian Jekan", args!["I'd gladly accept your", "help if it weren't for the fact that I cannot allow classified information to be released to the public."])?;
                                ctx.next()?;
                                ctx.lines_as("Librarian Jekan", args!["I have no idea how something so important was shuffled into the files here in Prontera Library, but we can't allow the public to access it."])?;
                                ctx.next()?;
                                ctx.lines_as("Librarian Jekan", args!["Still, that file doesn't seem to be that important, but I still have to waste my time on finding it..."])?;
                                ctx.next()?;
                                ctx.lines_as("Librarian Jekan", args!["Damn, my eyes are sore.", "Working as a government official is easy except for the times when the beaucrats make you do stuff like this."])?;
                                ctx.var("god_eremes").set(Val::from(5))?;
                            } else if ctx.var("god_eremes").get()?.number()? > 4 {
                                ctx.lines_as(
                                    "Librarian Jekan",
                                    args!["I think I'm going to go", "insane looking for this file...!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Librarian Jekan",
                                    args!["Huh...?", "Did you just say", "that you wanted to", "help me? I-I'd appreciate it."],
                                )?;
                            } else {
                                ctx.lines_as(
                                    "Librarian Jekan",
                                    args![
                                        "Um, you're not even",
                                        "supposed to know what",
                                        "kind of file I'm looking for. Even if you did, I'm not allowed to show you what's inside!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        4 => {
                            ctx.lines_as(
                                "Librarian Jekan",
                                args![
                                    "Hmm. I'm not really allowed to disclose any information related to that file. I'm sorry about that."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["But isn't", "the file about..."],
                            )?;
                            let (input, status) = runtime::input_text(ctx, None, None)?;
                            ctx.var("@str$").set(input)?;
                            ctx.lines(args![((Val::from("^0000FF") + ctx.var("@str$").get()?) + Val::from("?"))])?;
                            ctx.next()?;
                            if ctx.var("@str$").get()? == "the 1st squad's final mission" {
                                ctx.lines_as("Librarian Jekan", args!["Hmm...?", "How did you", "know about that?"])?;
                                ctx.var("god_eremes").set(Val::from(4))?;
                            } else {
                                ctx.lines_as("Librarian Jekan", args!["Errmmm...", "I don't think so....?"])?;
                            }
                        }
                        _ => {}
                    }
                }
            }
        } else {
            if ctx.var("god_eremes").get()? == 2 {
                ctx.lines_as("Librarian Jekan", args!["Ah, please do not touch the files in this section. Usually, it's public domain but it seems that a classified file managed to get misplaced there."])?;
                ctx.next()?;
                ctx.lines_as("Librarian Jekan", args!["For now, I cannot let anyone without special authorization to look through that section. I must find that file and send it to the Prontera military as soon as I can."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Librarian Jekan",
                    args!["Ergh! This light isn't bright enough for my eyes. They're starting to get sore again..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Librarian Jekan",
                    args!["So what brings you here? If you're looking for something in particular, you should try another section."],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "I want to read some documents.:Do you have bad eyes?:Let me help you to find the files.:What kind of files are you looking for?",
                    )],
                )? {
                    1 => {
                        ctx.lines_as("Librarian Jekan", args!["I told you already! The files in this section aren't open to the public right now! Don't make me repeat myself!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Librarian Jekan",
                            args![
                                "Since I have worked under a dim light for a long time, my eyes have gone bad.",
                                "They should get me a new light, otherwise I will be gone blind sooner or later..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.lines_as(
                            "Librarian Jekan",
                            args![
                                "Um? How do you know what file I am looking for?",
                                "You don't even know what is written inside?",
                                "Stop interrupting me, leave!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    4 => {
                        ctx.lines_as(
                            "Librarian Jekan",
                            args!["Hmm... I am not supposed to reveal anything related to the file."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Isn't the file..."])?;
                        let (input, status) = runtime::input_text(ctx, None, None)?;
                        ctx.var("@str$").set(input)?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![((Val::from("^0000FF") + ctx.var("@str$").get()?) + Val::from("?"))],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Librarian Jekan", args!["Maybe...", "Maybe not~"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                if (ctx.var("god_eremes").get()?.number()? > 6 && ctx.var("god_eremes").get()?.number()? < 12) {
                    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? > 4 {
                        ctx.lines_as("Librarian Jekan", args!["Hmm? So did you find it? I've been searching for that file for a long time, but I haven't been able to find it."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Librarian Jekan",
                            args!["I'll go ahead and search the right side, so do you think you could search the left?"],
                        )?;
                    } else {
                        ctx.lines_as(
                            "Librarian Jekan",
                            args!["Did you find it yet? All this time, and I still haven't been able to dig up that file."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Librarian Jekan",
                            args!["I'll go ahead and check the left side, so would you look for it in the middle or the right side?"],
                        )?;
                    }
                } else {
                    if ctx.var("god_eremes").get()? == 12 {
                        ctx.lines_as(
                            "Librarian Jekan",
                            args![
                                "Ah! There it is!",
                                "Thank you so much!",
                                "I'm glad to see that there are still kind and thoughtful people like you in the world."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Librarian Jekan", args!["I really want to give you something to show you my gratitude, but I've got to send this back to the Prontera Military right away."])?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("I want to read the document.")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Librarian Jekan",
                            args![
                                "Oh, right, you mentioned that, didn't you? It doesn't seem seem",
                                "so important, but I'm not really quite sure..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Librarian Jekan",
                            args!["Well, it couldn't hurt if I made a copy of this first for you to read, but I'll need some materials."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Librarian Jekan", args!["I'll even cast the", "Search Magic on it, so that it'll be just like the original. All right, I need the following things to make a copy..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Librarian Jekan",
                            args![
                                "2 Slick Paper,",
                                "1 Oil Paper,",
                                "3 Squid Ink,",
                                "3 Feather of Birds and",
                                "20 Blue Gemstones."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Librarian Jekan",
                            args!["Please get those as soon as possible, and I'll be here making the other preparations to make a copy."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Librarian Jekan", args!["Also, I'll keep this copy in the Library for you to read, and to prevent it from being leaked to the public. So you'll have to come visit each time you wish to read it, okay?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Librarian Jekan",
                            args!["Okay, now...", "Come back here", "as soon as you can."],
                        )?;
                        ctx.var("god_eremes").set(Val::from(13))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("god_eremes").get()? == 13 {
                            if ((((ctx.call(Function::CountItem, vec![Val::from(7111)])?.number()? > 1
                                && ctx.call(Function::CountItem, vec![Val::from(7151)])?.is_true())
                                && ctx.call(Function::CountItem, vec![Val::from(1024)])?.number()? > 2)
                                && ctx.call(Function::CountItem, vec![Val::from(916)])?.number()? > 2)
                                && ctx.call(Function::CountItem, vec![Val::from(717)])?.number()? > 19)
                            {
                                ctx.lines_as("Librarian Jekan", args!["Oh, you came back.", "I didn't expect you to return here so quickly. Whatever's inside must be really important for you to know."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Librarian Jekan",
                                    args!["Anyway, please", "give me a moment", "while I make that", "copy for you."],
                                )?;
                                ctx.call(Function::DelItem, vec![Val::from(7111), Val::from(2)])?;
                                ctx.call(Function::DelItem, vec![Val::from(7151), Val::from(1)])?;
                                ctx.call(Function::DelItem, vec![Val::from(1024), Val::from(3)])?;
                                ctx.call(Function::DelItem, vec![Val::from(916), Val::from(3)])?;
                                ctx.call(Function::DelItem, vec![Val::from(717), Val::from(20)])?;
                                ctx.var("god_eremes").set(Val::from(14))?;
                                ctx.next()?;
                                ctx.lines_as("Librarian Jekan", args!["Alright then...", "I'll see you in a bit."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Librarian Jekan",
                                    args![
                                        "Have you forgotten",
                                        "what you need to make",
                                        "a copy? Alright, let me",
                                        "tell you again..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Librarian Jekan",
                                    args![
                                        "2 Slick Paper,",
                                        "1 Oil Paper,",
                                        "3 Squid Ink,",
                                        "3 Feather of Birds and",
                                        "20 Blue Gemstones."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Librarian Jekan",
                                    args!["Please come", "back with those items", "as soon as you can."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else {
                            if (ctx.var("god_eremes").get()?.number()? > 13 && ctx.var("god_eremes").get()?.number()? < 16) {
                                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? > 4 {
                                    ctx.lines_as(
                                        "Librarian Jekan",
                                        args!["Let me go over and review the document before I make a copy..."],
                                    )?;
                                } else {
                                    ctx.lines_as(
                                        "Librarian Jekan",
                                        args!["Let me go over and review the document before I make a copy..."],
                                    )?;
                                    ctx.var("god_eremes").set((ctx.var("god_eremes").get()? + Val::from(1)))?;
                                }
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("god_eremes").get()? == 16 {
                                    ctx.lines_as(
                                        "Librarian Jekan",
                                        args![
                                            "There you go. As I thought, the document didn't seem to contain any crucially important data."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Librarian Jekan", args!["Just remember, this copy needs to stay in the Prontera Library. That means you have to come back if you need to read the document again."])?;
                                    ctx.var("god_eremes").set(Val::from(17))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("god_eremes").get()?.number()? > 16 {
                                        ctx.lines_as("Librarian Jekan", args!["Welcome, my friend!", "So, what do you need today?"])?;
                                        ctx.next()?;
                                        'l5: loop {
                                            if !(true) {
                                                break 'l5;
                                            }
                                            'b5: {
                                                match runtime::select_values(
                                                    ctx,
                                                    &[Val::from("Record Search.:Quit searching.:Search Help:Converse.")],
                                                )? {
                                                    1 => {
                                                        ctx.lines(args![
                                                            "^663300- Search Magic Initiated -",
                                                            "- Please enter a keyword. -",
                                                            " ",
                                                            "*Search Magic is",
                                                            " case sensitive.",
                                                            "Please do not use",
                                                            "capital letters.^000000"
                                                        ])?;
                                                        ctx.next()?;
                                                        let (input, status) = runtime::input_text(ctx, None, None)?;
                                                        l_input_s = input;
                                                        if runtime::compare(&l_input_s.clone(), &Val::from("crusader")) == 1 {
                                                            ctx.lines(args!["^663300[Keyword: ^996633Crusader^663300]", "Crusaders are warriors preparing for the upcoming Holy War against Evil. Experienced swordsmen, usually with remarkable spiritual prowess.^000000"])?;
                                                            ctx.close_window()?;
                                                            if runtime::compare(&l_input_s.clone(), &Val::from("3rd_company")) == 1 {
                                                                ctx.lines(args!["^663300[Keyword: ^9966333rd Company^663300]", "Only the best Crusaders are selected to form the ranks of all squads in the 3rd Company. Most transfer records for 3rd Company Squads are unavailable.^000000"])?;
                                                                ctx.next()?;
                                                                ctx.lines(args!["^663300[Keyword: ^9966333rd Company^663300]", "The 1st Squad of the 3rd Platoon is an exception, as all members retired or were transferred to other forces. More specific information can be found by searching ^9966333rd Platoon^663300."])?;
                                                                ctx.close_window()?;
                                                                if runtime::compare(&l_input_s.clone(), &Val::from("3rd_platoon")) == 1 {
                                                                    ctx.lines(args!["^663300[Keyword: ^9966333rd Platoon^663300]", "The 3rd Platoon is considered the elite force in the 3rd Company. Only records for the 1st Squad in the 3rd Platoon currently exist due to special circumstances.^000000"])?;
                                                                    ctx.next()?;
                                                                    ctx.lines(args!["^663300[Keyword: ^9966333rd Platoon^663300]", "More specific information can be found by searching ^9966331st Squad^663300."])?;
                                                                    ctx.close_window()?;
                                                                    if runtime::compare(&l_input_s.clone(), &Val::from("1st_squad")) == 1 {
                                                                        ctx.lines(args![
                                                                            "^663300[Keyword: ^9966331st Squad^663300]",
                                                                            "1st Squad.",
                                                                            "Complete",
                                                                            "Member Roster",
                                                                            "...^000000"
                                                                        ])?;
                                                                        ctx.next()?;
                                                                        ctx.lines(args![
                                                                            "^663300[Keyword: ^9966331st Squad^663300]",
                                                                            "^0000FF : Rebarev Doug",
                                                                            " : Jack O",
                                                                            " : Zan.Huadoku",
                                                                            " : Cuaque Donon",
                                                                            " : Emma Searth",
                                                                            " : Royal Myst",
                                                                            " : The Nineball^000000"
                                                                        ])?;
                                                                        if ctx.var("god_eremes").get()? == 17 {
                                                                            ctx.var("god_eremes").set(Val::from(18))?;
                                                                        }
                                                                        ctx.close_window()?;
                                                                        if runtime::compare(&l_input_s.clone(), &Val::from("record")) == 1 {
                                                                            ctx.lines(args!["^663300[Keyword: ^9966331st Squad Record^663300]", "The 1st Squad was famous as the most skilled search party within the Crusaders. However, it was disbanded as a result of its final mission.^000000"])?;
                                                                            ctx.next()?;
                                                                            ctx.lines(args!["^663300[Keyword: ^9966331st Squad Record^663300]", "The 1st Squad was secretly", "assigned by the royal court to find and acquire ^996633godly artifacts^663300.", "However, the mission failed due to a factional dispute caused by one member's act of insubordination.^000000"])?;
                                                                            ctx.next()?;
                                                                            ctx.lines(args!["^663300[Keyword: ^9966331st Squad Record^663300]", "The member guilty of insubordination was dropped from the squad during this mission. However, his actions were enough to result in the failure of the 1st Squad's final mission.^000000"])?;
                                                                            ctx.next()?;
                                                                            ctx.lines(args!["^663300[Keyword: ^9966331st Squad Record^663300]", "Due to this disgraceful incident, the 1st Squad was disbanded and", "its members have been sent to disciplinary retraining for 3 months."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines(args!["^663300[Keyword: ^9966331st Squad Record^663300]", "Most of the members of the 1st Squad were transferred to other squads or retired. Their former leader, Rebarev Doug, is currently in charge of researching godly artifacts under royal edict.^000000"])?;
                                                                            ctx.close_window()?;
                                                                            if ctx.var("god_eremes").get()? == 18 {
                                                                                ctx.var("god_eremes").set(Val::from(19))?;
                                                                            }
                                                                        }
                                                                    } else {
                                                                        ctx.lines(args![
                                                                            ((Val::from("^663300[Keyword: ^996633") + l_input_s.clone())
                                                                                + Val::from("^663300]")),
                                                                            "- No result has been found.-"
                                                                        ])?;
                                                                        ctx.close_window()?;
                                                                    }
                                                                }
                                                            } else if runtime::compare(&l_input_s.clone(), &Val::from("3rd_platoon")) == 1 {
                                                                ctx.lines(args![
                                                                    ((Val::from("^663300[Keyword: ^996633") + l_input_s.clone())
                                                                        + Val::from("^663300]")),
                                                                    "Each Company consists",
                                                                    "of 4 Platoons. Please",
                                                                    "specify Company.^000000"
                                                                ])?;
                                                                ctx.next()?;
                                                                ctx.close_window()?;
                                                            } else if runtime::compare(&l_input_s.clone(), &Val::from("1st_squad")) == 1 {
                                                                ctx.lines(args![((Val::from("^663300[Keyword: ^996633") + l_input_s.clone()) + Val::from("^663300]")), "The 1st Squad : Crusaders.", "Each platoon consists of 4 squads. Please specify Company and Platoon for information on a specific squad.^000000"])?;
                                                                ctx.close_window()?;
                                                            } else if runtime::compare(&l_input_s.clone(), &Val::from("record")) == 1 {
                                                                ctx.lines(args![
                                                                    ((Val::from("^663300[Keyword: ^996633") + l_input_s.clone())
                                                                        + Val::from("^663300]")),
                                                                    "- No result has been found.-"
                                                                ])?;
                                                                ctx.close_window()?;
                                                            } else {
                                                                ctx.lines(args![
                                                                    ((Val::from("^663300[Keyword: ^996633") + l_input_s.clone())
                                                                        + Val::from("^663300]")),
                                                                    ((Val::from("^663300[Keyword: ^996633") + l_input_s.clone())
                                                                        + Val::from("^663300]")),
                                                                    ((Val::from("-search with a keyword : ") + l_input_s.clone())
                                                                        + Val::from(" -")),
                                                                    "- No result has been found.-"
                                                                ])?;
                                                                ctx.close_window()?;
                                                            }
                                                        } else {
                                                            if runtime::compare(&l_input_s.clone(), &Val::from("3rd_company")) == 1 {
                                                                ctx.lines(args![((Val::from("^663300[Keyword: ^996633") + l_input_s.clone()) + Val::from("^663300]")), "^663300[Keyword: ^9966333rd Company^663300]", "Only the best Crusaders are selected to form the ranks of all squads in the 3rd Company. Most transfer records for 3rd Company Squads are unavailable.^000000"])?;
                                                                ctx.next()?;
                                                                ctx.lines(args!["^663300[Keyword: ^9966333rd Company^663300]", "The 1st Squad of the 3rd Platoon is an exception, as all members retired or were transferred to other forces. More specific information can be found by searching ^9966333rd Platoon^663300."])?;
                                                                ctx.close_window()?;
                                                                if runtime::compare(&l_input_s.clone(), &Val::from("3rd_platoon")) == 1 {
                                                                    ctx.lines(args!["^663300[Keyword: ^9966333rd Platoon^663300]", "The 3rd Platoon is considered the elite force in the 3rd Company. Only records for the 1st Squad in the 3rd Platoon currently exist due to special circumstances.^000000"])?;
                                                                    ctx.next()?;
                                                                    ctx.lines(args!["^663300[Keyword: ^9966333rd Platoon^663300]", "More specific information can be found by searching ^9966331st Squad^663300."])?;
                                                                    ctx.close_window()?;
                                                                    if runtime::compare(&l_input_s.clone(), &Val::from("1st_squad")) == 1 {
                                                                        ctx.lines(args![
                                                                            "^663300[Keyword: ^9966331st Squad^663300]",
                                                                            "1st Squad.",
                                                                            "Complete",
                                                                            "Member Roster",
                                                                            "...^000000"
                                                                        ])?;
                                                                        ctx.next()?;
                                                                        ctx.lines(args![
                                                                            "^663300[Keyword: ^9966331st Squad^663300]",
                                                                            "^0000FF : Rebarev Doug",
                                                                            " : Jack O",
                                                                            " : Zan.Huadoku",
                                                                            " : Cuaque Donon",
                                                                            " : Emma Searth",
                                                                            " : Royal Myst",
                                                                            " : The Nineball^000000"
                                                                        ])?;
                                                                        ctx.close_window()?;
                                                                        if ctx.var("god_eremes").get()? == 17 {
                                                                            ctx.var("god_eremes").set(Val::from(18))?;
                                                                        }
                                                                        if runtime::compare(&l_input_s.clone(), &Val::from("record")) == 1 {
                                                                            ctx.lines(args!["^663300[Keyword: ^9966331st Squad Record^663300]", "The 1st Squad was famous as the most skilled search party within the Crusaders. However, it was disbanded as a result of its final mission.^000000"])?;
                                                                            ctx.next()?;
                                                                            ctx.lines(args!["^663300[Keyword: ^9966331st Squad Record^663300]", "The 1st Squad was secretly", "assigned by the royal court to find and acquire ^996633godly artifacts^663300.", "However, the mission failed due to a factional dispute caused by one member's act of insubordination.^000000"])?;
                                                                            ctx.next()?;
                                                                            ctx.lines(args!["^663300[Keyword: ^9966331st Squad Record^663300]", "The member guilty of insubordination was dropped from the squad during this mission. However, his actions were enough to result in the failure of the 1st Squad's final mission.^000000"])?;
                                                                            ctx.next()?;
                                                                            ctx.lines(args!["^663300[Keyword: ^9966331st Squad Record^663300]", "Due to this disgraceful incident, the 1st Squad was disbanded and", "its members have been sent to disciplinary retraining for 3 months."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines(args!["^663300[Keyword: ^9966331st Squad Record^663300]", "Most of the members of the 1st Squad were transferred to other squads or retired. Their former leader, Rebarev Doug, is currently in charge of researching godly artifacts under royal edict.^000000"])?;
                                                                            ctx.close_window()?;
                                                                            if ctx.var("god_eremes").get()? == 18 {
                                                                                ctx.var("god_eremes").set(Val::from(19))?;
                                                                            }
                                                                        }
                                                                    } else {
                                                                        ctx.lines(args![
                                                                            ((Val::from("^663300[Keyword: ^996633") + l_input_s.clone())
                                                                                + Val::from("^663300]")),
                                                                            "- No result has been found.-"
                                                                        ])?;
                                                                        ctx.close_window()?;
                                                                    }
                                                                }
                                                            } else {
                                                                if runtime::compare(&l_input_s.clone(), &Val::from("3rd_platoon")) == 1 {
                                                                    ctx.lines(args![
                                                                        ((Val::from("^663300[Keyword: ^996633") + l_input_s.clone())
                                                                            + Val::from("^663300]")),
                                                                        "The 3rd Platoon : ",
                                                                        "- No result has been found.-",
                                                                        "- Suggested to enter a more specific keyword.-"
                                                                    ])?;
                                                                    ctx.close_window()?;
                                                                } else {
                                                                    if runtime::compare(&l_input_s.clone(), &Val::from("1st_squad")) == 1 {
                                                                        ctx.lines(args![
                                                                            ((Val::from("^663300[Keyword: ^996633") + l_input_s.clone())
                                                                                + Val::from("^663300]")),
                                                                            "The 3rd Company : ",
                                                                            "- No result has been found.-",
                                                                            "- Suggested to enter a more specific keyword.-"
                                                                        ])?;
                                                                        ctx.close_window()?;
                                                                    } else {
                                                                        if (runtime::compare(&l_input_s.clone(), &Val::from("record")) == 1
                                                                            && ctx.var("god_eremes").get()?.number()? > 17)
                                                                        {
                                                                            ctx.lines(args![((Val::from("^663300[Keyword: ^996633") + l_input_s.clone()) + Val::from("^663300]")), "- No result has been found.-", "- Suggested to enter a specific name of the force for a better research.-"])?;
                                                                            ctx.next()?;
                                                                            ctx.close_window()?;
                                                                        } else {
                                                                            if runtime::compare(
                                                                                &l_input_s.clone(),
                                                                                &Val::from("rebarev_doug"),
                                                                            ) == 1
                                                                            {
                                                                                ctx.lines(args![((Val::from("^663300[Keyword: ^996633") + l_input_s.clone()) + Val::from("^663300]")), "Former leader of", "3rd Company, 3rd Platoon,", "1st Squad. Serves as instructor of Crusader Boot Camp and is currently", "conducting research by royal edict.^000000"])?;
                                                                                ctx.next()?;
                                                                                ctx.lines(args![
                                                                                    ((Val::from("^663300[Keyword: ^996633")
                                                                                        + l_input_s.clone())
                                                                                        + Val::from("^663300]")),
                                                                                    "Current location:",
                                                                                    "^996633Prontera Castle, Prontera^663300.^000000"
                                                                                ])?;
                                                                                ctx.close_window()?;
                                                                            } else {
                                                                                if runtime::compare(
                                                                                    &l_input_s.clone(),
                                                                                    &Val::from("egnigem"),
                                                                                ) == 1
                                                                                {
                                                                                    ctx.lines(args![
                                                                                        ((Val::from("^663300[Keyword: ^996633")
                                                                                            + l_input_s.clone())
                                                                                            + Val::from("^663300]")),
                                                                                        "^FF0000Prohibited Search Term!^000000"
                                                                                    ])?;
                                                                                    ctx.close_window()?;
                                                                                } else {
                                                                                    if runtime::compare(
                                                                                        &l_input_s.clone(),
                                                                                        &Val::from("zan.huadoku"),
                                                                                    ) == 1
                                                                                    {
                                                                                        ctx.lines(args![((Val::from("^663300[Keyword: ^996633") + l_input_s.clone()) + Val::from("^663300]")), "Former member of", "3rd Company, 3rd Platoon", "1st Squad. Serving as weapon quartermaster since disbanding", "of 1st Squad."])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines(args![
                                                                                            ((Val::from("^663300[Keyword: ^996633")
                                                                                                + l_input_s.clone())
                                                                                                + Val::from("^663300]")),
                                                                                            "Current location:",
                                                                                            "^996633Blacksmith Guild, Geffen^663300.^000000"
                                                                                        ])?;
                                                                                        ctx.close_window()?;
                                                                                    } else {
                                                                                        if runtime::compare(
                                                                                            &l_input_s.clone(),
                                                                                            &Val::from("cuaque_donon"),
                                                                                        ) == 1
                                                                                        {
                                                                                            ctx.lines(args![
                                                                                                ((Val::from("^663300[Keyword: ^996633")
                                                                                                    + l_input_s.clone())
                                                                                                    + Val::from("^663300]")),
                                                                                                "Former member of",
                                                                                                "3rd Company, 3rd Platoon",
                                                                                                "1st Squad. Retired from service",
                                                                                                "and now works at an Inn."
                                                                                            ])?;
                                                                                            ctx.next()?;
                                                                                            ctx.lines(args![
                                                                                                ((Val::from("^663300[Keyword: ^996633")
                                                                                                    + l_input_s.clone())
                                                                                                    + Val::from("^663300]")),
                                                                                                "Current location:",
                                                                                                "^996633Inn, Morocc^663300.^000000"
                                                                                            ])?;
                                                                                            ctx.close_window()?;
                                                                                        } else if runtime::compare(
                                                                                            &l_input_s.clone(),
                                                                                            &Val::from("jack_o"),
                                                                                        ) == 1
                                                                                        {
                                                                                            ctx.lines(args![((Val::from("^663300[Keyword: ^996633") + l_input_s.clone()) + Val::from("^663300]")), "Former member of", "3rd Company, 3rd Platoon", "1st Squad. Serving as recruiting officer since disbanding of 1st Squad."])?;
                                                                                            ctx.next()?;
                                                                                            ctx.lines(args![
                                                                                                ((Val::from("^663300[Keyword: ^996633")
                                                                                                    + l_input_s.clone())
                                                                                                    + Val::from("^663300]")),
                                                                                                "Current location:",
                                                                                                "^996633Alberta Port^663300.^000000"
                                                                                            ])?;
                                                                                            ctx.close_window()?;
                                                                                        } else if runtime::compare(
                                                                                            &l_input_s.clone(),
                                                                                            &Val::from("emma_searth"),
                                                                                        ) == 1
                                                                                        {
                                                                                            ctx.lines(args![
                                                                                                ((Val::from("^663300[Keyword: ^996633")
                                                                                                    + l_input_s.clone())
                                                                                                    + Val::from("^663300]")),
                                                                                                "Former member of",
                                                                                                "3rd Company, 3rd Platoon",
                                                                                                "1st Squad. Retired since",
                                                                                                "disbanding of 1st Squad."
                                                                                            ])?;
                                                                                            ctx.next()?;
                                                                                            ctx.lines(args![
                                                                                                ((Val::from("^663300[Keyword: ^996633")
                                                                                                    + l_input_s.clone())
                                                                                                    + Val::from("^663300]")),
                                                                                                "Current location:",
                                                                                                "^996633Al De Baran^663300.^000000"
                                                                                            ])?;
                                                                                            ctx.close_window()?;
                                                                                        } else if runtime::compare(
                                                                                            &l_input_s.clone(),
                                                                                            &Val::from("royal_myst"),
                                                                                        ) == 1
                                                                                        {
                                                                                            ctx.lines(args![
                                                                                                ((Val::from("^663300[Keyword: ^996633")
                                                                                                    + l_input_s.clone())
                                                                                                    + Val::from("^663300]")),
                                                                                                "Former member of",
                                                                                                "3rd Company, 3rd Platoon",
                                                                                                "1st Squad. Current duty is unknown."
                                                                                            ])?;
                                                                                            ctx.next()?;
                                                                                            ctx.lines(args![
                                                                                                ((Val::from("^663300[Keyword: ^996633")
                                                                                                    + l_input_s.clone())
                                                                                                    + Val::from("^663300]")),
                                                                                                "Current location:",
                                                                                                "^996633Casino, Comodo^663300.^000000"
                                                                                            ])?;
                                                                                            ctx.close_window()?;
                                                                                        } else if runtime::compare(
                                                                                            &l_input_s.clone(),
                                                                                            &Val::from("the_nineball"),
                                                                                        ) == 1
                                                                                        {
                                                                                            ctx.lines(args![((Val::from("^663300[Keyword: ^996633") + l_input_s.clone()) + Val::from("^663300]")), "Former member of", "3rd Company, 3rd Platoon", "1st Squad. Serves as security officer since disbanding of 1st Squad."])?;
                                                                                            ctx.next()?;
                                                                                            ctx.lines(args![
                                                                                                ((Val::from("^663300[Keyword: ^996633")
                                                                                                    + l_input_s.clone())
                                                                                                    + Val::from("^663300]")),
                                                                                                "Current location:",
                                                                                                "^996633Tavern, Jawaii^663300.^000000"
                                                                                            ])?;
                                                                                            ctx.close_window()?;
                                                                                        } else {
                                                                                            ctx.lines(args![
                                                                                                ((Val::from("^663300[Keyword: ^996633")
                                                                                                    + l_input_s.clone())
                                                                                                    + Val::from("^663300]")),
                                                                                                ((Val::from("^663300[Keyword: ^996633")
                                                                                                    + l_input_s.clone())
                                                                                                    + Val::from("^663300]")),
                                                                                                ((Val::from("^663300[Keyword: ^996633")
                                                                                                    + l_input_s.clone())
                                                                                                    + Val::from("^663300]")),
                                                                                                "No result has been found."
                                                                                            ])?;
                                                                                            ctx.close_window()?;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    2 => {
                                                        ctx.lines(args!["^663300- Search Magic -", "- is being shut down. -^000000"])?;
                                                        ctx.next()?;
                                                        ctx.mes("^663300- ... -^000000")?;
                                                        ctx.next()?;
                                                        ctx.lines(args!["^663300- ... -^000000", "^663300- ... -^000000"])?;
                                                        ctx.next()?;
                                                        ctx.lines(args!["^663300- Search Magic -", "- Deactivated. -^000000"])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    3 => {
                                                        ctx.lines_as(
                                                            "Librarian Jekan",
                                                            args![
                                                                "Well...",
                                                                "This is a pretty",
                                                                "obsolete version",
                                                                "of Search Magic,",
                                                                "so I understand if",
                                                                "it's giving you trouble."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Librarian Jekan", args!["Now...", "First, if you're going to enter a single search term, like someone's name, you would enter '^660000lee_hester^000000' if you were looking up 'Lee Hester.'"])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Librarian Jekan", args!["Don't use any", "capital letters!", "And use underscores to", "indicate spaces between first and last names and within military unit names."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Librarian Jekan", args!["For example, the Fourth Company would be entered as ^6600004th_company^000000. Now... bear with me."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Librarian Jekan", args!["If you're looking for information on a specific squad, you need to know the Platoon and Company it falls under."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Librarian Jekan", args!["For example, if you wanted to look up the Crusader 4th Squad of the 4th Platoon of the 4th Company, you would input..."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Librarian Jekan", args!["'^660000crusader 4th_company 4th_platoon 4th_squad record^000000' all in one line exactly like that. I hope this helps you in your search..."])?;
                                                        ctx.next()?;
                                                    }
                                                    4 => {
                                                        ctx.lines_as(
                                                            "Librarian Jekan",
                                                            args![
                                                                "You can tell our",
                                                                "government doesn't",
                                                                "really know how to",
                                                                "allocate its funds",
                                                                "and resources."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Librarian Jekan", args!["First, there's Jawaii.", "Do newlyweds really need", "an entire island for themselves? Then again, I am single so I'm probably biased."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Librarian Jekan", args!["And look at me.", "Geniuses aren't supposed to waste their precious time doing useless things for libraries!"])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    _ => {}
                                                }
                                            }
                                        }
                                    } else {
                                        ctx.lines_as("Librarian Jekan", args!["Ah, please do not touch any books in that section. ...I'm sure there's nothing there that could be useful to you."])?;
                                        ctx.next()?;
                                        match runtime::select_values(ctx, &[Val::from("Ignore him and check books.:Step back.")])? {
                                            1 => {
                                                ctx.lines_as(
                                                    "Librarian Jekan",
                                                    args!["Hey...!", "I thought I said", "not to touch those!"],
                                                )?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Warp, vec![Val::from("prontera"), Val::from(120), Val::from(264)])?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                ctx.lines_as(
                                                    "Librarian Jekan",
                                                    args!["Thank you. Feel", "free to browse through", "the other sections."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Librarian Jekan", args!["It's really nice to have visitors in the Library, especially when so many people don't read books nowadays."])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        ctx.lines_as(
            "Librarian Jekan",
            args![
                "Ah, please do not",
                "touch any books in",
                "that section. There's",
                "nothing useful over in",
                "that section anyway."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Ignore him and check books.:Step back.")])? {
            1 => {
                ctx.lines_as("Librarian Jekan", args!["I told you...!", "Don't touch", "the books here!"])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("prontera"), Val::from(120), Val::from(264)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Librarian Jekan",
                    args!["Thank you. Feel", "free to browse through", "the other sections."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Librarian Jekan",
                    args!["It's really nice to have visitors in the Library, especially when so many people don't read books nowadays."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn librarian_megin(ctx: &Ctx) -> Script {
    librarian_megin_body(ctx, Vec::new()).map(|_| ())
}
