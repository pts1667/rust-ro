use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn physics_professor_sa_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Aebecee George]")?;
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SAGE")?) {
            ctx.lines(args![
                "Tee hee, hello there! What brings you here again, sweetie?",
                "Oh, I see... you're just excited because you finally became a Sage? Tee hee~"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Aebecee George",
                args![
                    "Oh well, although you have become a Sage, you're just doing the same thing, tee hee...we're mages down to the core!",
                    "We're way better than any other class because we're pretty good with our heads, you know?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aebecee George",
                args![
                    "Okay, is there anything you want to talk about?",
                    "Oh dearie, don't be nervous...where's your sense of adventure?"
                ],
            )?;
        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.mes("Well, aren't you the cutest little Novice~")?;
            ctx.next()?;
            ctx.lines_as(
                "Aebecee George",
                args![
                    "What are you doing here, sweetcakes? Do you just come over to see what's in here?",
                    "Do you want some candy? Candy, my precious? Tee hee."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aebecee George",
                args![
                    "Oh gosh...Sorry, I thought I had some...",
                    "But I've got other treats, you catch my drift?"
                ],
            )?;
        } else {
            ctx.mes("Hey there, tee hee, how's it going sailor?")?;
            ctx.next()?;
            ctx.lines_as(
                "Aebecee George",
                args![
                    "Did you come here to look around? Huh...?",
                    "Well, if you're not here for business, you must be here..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Aebecee George", args!["...for PLEASURE.", "Hey wait! Come back~!"])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("sage_q").get()? == 13 {
        if ctx.var("sage_q2").get()? == 0 {
            ctx.lines(args![
                "Hello~? Nice to meet you, tee hee.",
                "Did you come to see me? Oh, you're a student!"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Aebecee George",
                args![
                    "Tee hee, I am the Professor in charge of you, Abecee George.",
                    "So...aren't you happy to be with me, dearie? Aren't you gay? Tee hee~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aebecee George",
                args![
                    "However, before we get start class, I need you to do me...a favor.",
                    "Don't be scared, its just an eensy weensy favor. Tee hee~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aebecee George",
                args!["Please bring me ^3355FF30 Stone^000000, that's all.", "It's not so hard, isn't it?"],
            )?;
            ctx.next()?;
            ctx.var("sage_q2").set(Val::from(1))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2049), Val::from(2060)])?;
            ctx.lines_as(
                "Aebecee George",
                args![
                    "Why don't you ask a thief pal for help?",
                    "We'll start the class when you bring me those stones~ Tee hee~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("sage_q2").get()? == 1 {
            if ctx.call(Function::CountItem, vec![Val::from(7049)])?.number()? > 29 {
                ctx.lines(args![
                    "Oh~ how sweet! You brought them all~ thank you~",
                    "Oh, can you wait a little bit? I need to do something before we start. Tee hee~"
                ])?;
                ctx.next()?;
                ctx.lines_as("Aebecee George", args!["Hocus-focus!!"])?;
                ctx.next()?;
                ctx.lines_as("Aebecee George", args!["Hocus~focus!!"])?;
                ctx.next()?;
                ctx.lines_as("Aebecee George", args!["Ho~cus~fo~cus!!"])?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(7049), Val::from(30)])?;
                ctx.lines_as(
                    "Aebecee George",
                    args!["Tee hee, you naughty stone~", "Only 3 of them worked for me. Tee hee~"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aebecee George",
                    args![
                        "Here's my other favor to ask of you. Oh, are those tears of...joy?",
                        "Oh, I'm sooo excited too! Tee hee~"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::GetItem, vec![Val::from(991), Val::from(1)])?;
                ctx.call(Function::GetItem, vec![Val::from(993), Val::from(1)])?;
                ctx.call(Function::GetItem, vec![Val::from(992), Val::from(1)])?;
                ctx.lines_as(
                    "Aebecee George",
                    args![
                        "I will give you these elemental ores...but...they are not for free.",
                        "Whoa there cowboy! Don't pull out so soon~ Listen, tee hee~"
                    ],
                )?;
                ctx.var("sage_q2").set(Val::from(2))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(2060), Val::from(2061)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Aebecee George",
                    args![
                        "Please make arrows using these items and bring them to me, pretty please~",
                        "50 ^3355FFCrystal Arrow^000000,",
                        "50 ^3355FFStone Arrow^000000,",
                        "50 ^3355FFWind Arrow^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aebecee George",
                    args![
                        "Ask any of your Archer friends if you know any.",
                        "I am looking forward to seeing you again, tee hee~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines(args![
                    "You don't have all the items? And here I was, getting all hot and bothered.",
                    "Go on, sweetheart, and hurry baaack~"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Aebecee George",
                    args![
                        "I asked you to bring ^3355FF30 Stone^000000.",
                        "There are many out there, so you won't have too hard a time getting them~ tee hee."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("sage_q2").get()? == 2 {
                if ((ctx.call(Function::CountItem, vec![Val::from(1754)])?.number()? > 49
                    && ctx.call(Function::CountItem, vec![Val::from(1756)])?.number()? > 49)
                    && ctx.call(Function::CountItem, vec![Val::from(1755)])?.number()? > 49)
                {
                    ctx.lines(args![
                        "Oh~ how sweet! You brought them all~ Oh thank you~",
                        "Well now, let's get down to business. Tee hee~"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aebecee George",
                        args![
                            "I'll say something, and you just write everything down.",
                            "Don't forget to underline the important sentences~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aebecee George",
                        args![
                            "Water property magic is sooo strong against the fire property.",
                            "Just remember those burly, sexay fire fighters, shall we say, putting out that fire with water. Ooh, wet!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aebecee George",
                        args![
                            "Wind property magic totally dominates the water property! Oh, yes~",
                            "Think of a hot flash, and by that I mean ligntening, striking a lake. Oh, I'm bad!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Aebecee George", args!["In terms of strength, magic with earth property goes on top of wind property magic! Ho~~", "Just think of how the wind passionately beats against the cliffside near the Lookout Point or Lover's Lane. Tee hee~"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aebecee George",
                        args![
                            "Magic with fire property is strong against earth property! Tee hee~",
                            "Just think of burning wood, sprung from the earth, in a cozy fireplace. On the bearskin, in the dark~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aebecee George",
                        args!["...Oh my gosh! Time flies sooo fast!", "Let's call it a day, dear, tee hee~"],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(1754), Val::from(50)])?;
                    ctx.call(Function::DelItem, vec![Val::from(1756), Val::from(50)])?;
                    ctx.call(Function::DelItem, vec![Val::from(1755), Val::from(50)])?;
                    ctx.lines_as(
                        "Aebecee George",
                        args![
                            "When you come to the next class, bring ^3355FF1 Holy Water^000000~",
                            "I hope you have at least one Priest friend. Oh, you don't? Well, it couldn't hurt to get friendly with one."
                        ],
                    )?;
                    ctx.var("sage_q2").set(Val::from(3))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2061), Val::from(2062)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines(args!["What are you doing here, precious...are you trying to fool me? Tee hee~", "I hope you didn't sell the elemental ores I gave you? Naughty naughty~ Don't you know diamonds are a girl's best friend? Tee hee~"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aebecee George",
                        args![
                            "50 ^3355FFCrystal Arrow^000000,",
                            "50 ^3355FFStone Arrow^000000,",
                            "50 ^3355FFWind Arrow^000000.",
                            "Ask your Archer friend if you have one~ tee hee."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("sage_q2").get()? == 3 {
                    if ctx.call(Function::CountItem, vec![Val::from(523)])?.number()? > 0 {
                        ctx.lines(args![
                            "Oh~ how sweet! You brought some Holy Water~ thanks honey~",
                            "Oh righty, we should start class. Tee hee~"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aebecee George",
                            args![
                                "Just Like last time, I will speak and you just write down everything I say.",
                                "Even if you don't understand, just pretend and go with the flow, alright? Tee hee~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Aebecee George", args!["Water Magic is weak against wind property! Yes, that's right~", "Just remember why you don't keep the electric fan around when you're taking a hot bubble bath. Ooh! So electric!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aebecee George",
                            args![
                                "Magic with the wind property is weak against earth property!",
                                "Think of how the wind can't, shall we say, penetrate a mud facial mask~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Aebecee George", args!["Magic with earth property is weak against fire property! Tee hee~", "Just think of burning wood, sprung from the earth, in a cozy fireplace. On the be--oh? I said that already? Oh poopy."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aebecee George",
                            args![
                                "Magic with fire property is weak against water property! Yes~",
                                "When things get flaming hot, it's best to splash things in cold water. Don't you agree~?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aebecee George",
                            args![
                                "Oh, it's the end of the class~ tee hee.",
                                "So? Do you think you've learned a lot? You can thank me, honey!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(523), Val::from(1)])?;
                        ctx.lines_as(
                            "Aebecee George",
                            args![
                                "Well, that's it. You can write your thesis with what I've taught you!",
                                "Tee hee~ yes! Yes! I am the best teacher in the world! Oh, I am on fire! Tee hee~"
                            ],
                        )?;
                        ctx.var("sage_q2").set(Val::from(0))?;
                        ctx.var("sage_q").set(Val::from(14))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(2062), Val::from(2051)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aebecee George",
                            args![
                                "Okay, please gather these items so you can write the thesis, dear~ Tee hee~",
                                "^3355FF1 Feather of Birds^000000 which will be used as a pen,",
                                "^3355FF1 Animal Skin^000000 which will be used as paper,",
                                "^3355FF1 Trunk^000000 which will be used to bind a book,",
                                "^3355FF1 Squid Ink^000000 which will be used as ink,",
                                "^3355FF1 Empty Bottle^000000 to keep that Squid Ink from splashing."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Aebecee George", args!["So get on out there cowboy and hurry back~ Tee hee~"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines(args![
                            "What are you doing here, precious...are you playing games? Tee hee.",
                            "Oh, poor dear. Did you already forget what I told you? Do you want me to remind you? Tee hee~"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aebecee George",
                            args![
                                "^3355FF1 Holy Water^000000,",
                                "It couldn't hurt to get friendly with an Aco or Priest...tee hee."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.mes("So...do you have the time? I'm kidding, girlfriend!")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    } else {
        if ctx.var("sage_q").get()? == 14 {
            if ((((ctx.call(Function::CountItem, vec![Val::from(916)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(919)])?.number()? > 0)
                && ctx.call(Function::CountItem, vec![Val::from(1019)])?.number()? > 0)
                && ctx.call(Function::CountItem, vec![Val::from(1024)])?.number()? > 0)
                && ctx.call(Function::CountItem, vec![Val::from(713)])?.number()? > 0)
            {
                ctx.call(Function::DelItem, vec![Val::from(916), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(919), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(1019), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(1024), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(713), Val::from(1)])?;
                ctx.lines(args![
                    "Okay~ so its now or never~ tee hee~",
                    "I taught you everything I know. Don't be nervous, you'll be okay~"
                ])?;
                ctx.next()?;
                ctx.mes("..........")?;
                ctx.next()?;
                ctx.mes("....................")?;
                ctx.next()?;
                ctx.mes(".................................")?;
                ctx.next()?;
                ctx.mes(".....Magic spells are varied into 4 elements such as")?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Water, Earth, Fire and Wind.:Earth, Water, Fire and Wind.:Water, Wind, Earth and Fire.",
                    )],
                )? {
                    1 => {
                        ctx.mes("Water, Earth, Fire and Wind.")?;
                    }
                    2 => {
                        ctx.mes("Earth, Water, Fire and Wind.")?;
                    }
                    3 => {
                        ctx.mes("Water, Wind, Earth and Fire.")?;
                    }
                    _ => {}
                }
                ctx.mes("Each property has an opposing property,")?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Magic with wind property is strong against water:Magic with water property is strong against fire:Magic with fire property is strong against earth",
                    )],
                )? {
                    1 => {
                        ctx.mes("Magic with wind property is strong against water")?;
                    }
                    2 => {
                        ctx.mes("Magic with water property is strong against fire")?;
                    }
                    3 => {
                        ctx.mes("Magic with fire property is strong against earth")?;
                    }
                    _ => {}
                }
                ctx.mes("Magic with earth property is strong against wind.")?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "However, that does not work on the opposite case :This theory works the same for earth property weapons:Elemental properties are varied by monster types",
                    )],
                )? {
                    1 => {
                        ctx.mes("However, that does not work on the opposite case ")?;
                    }
                    2 => {
                        ctx.mes("This theory works the same for earth property weapons")?;
                    }
                    3 => {
                        ctx.mes("Elemental properties are varied by monster types")?;
                    }
                    _ => {}
                }
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "You must be aware of the limit of your ability.:You must apply different types of property by the situation or place.:Red Potion is rumored to taste like strawberries.",
                    )],
                )? {
                    1 => {
                        ctx.mes("You must be aware of the limit of your ability.")?;
                    }
                    2 => {
                        ctx.mes("You must apply different types of property by the situation or place.")?;
                    }
                    3 => {
                        ctx.mes("Red Potion is rumored to taste like strawberries.")?;
                    }
                    _ => {}
                }
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "The most adorable NPC is YuPi in Prontera:Red Potion is rumored to be made out of Porings:You never know the limits of magic",
                    )],
                )? {
                    1 => {
                        ctx.mes("The most adorable NPC is YuPi in Prontera")?;
                    }
                    2 => {
                        ctx.mes("Red Potion is rumored to be made out of Porings")?;
                    }
                    3 => {
                        ctx.mes("You never know the limits of magic")?;
                    }
                    _ => {}
                }
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Merchant Lady in Morocc is also as cute as YuPi.:Nobody knows why Red Potion tastes like strawberries.:It is not suggested to be too addicted to magic spells.",
                    )],
                )? {
                    1 => {
                        ctx.mes("Merchant Lady in Morocc is also as cute as YuPi.")?;
                    }
                    2 => {
                        ctx.mes("Nobody knows why Red Potion tastes like strawberries.")?;
                    }
                    3 => {
                        ctx.mes("It is not suggested to be too addicted to magic spells.")?;
                    }
                    _ => {}
                }
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "If I had a Bunny Band,:If so, what about the taste of White Potion?:Only pertinent uses of magic, as well as rest",
                    )],
                )? {
                    1 => {
                        ctx.mes("If I had a Bunny Band,")?;
                    }
                    2 => {
                        ctx.mes("If so, what about the taste of White Potion?")?;
                    }
                    3 => {
                        ctx.mes("Only pertinent uses of magic, as well as rest")?;
                    }
                    _ => {}
                }
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "I would want to give it to her as a present.:I can't even imagine the taste.:will guarantee you a safe battle.",
                    )],
                )? {
                    1 => {
                        ctx.mes("I would want to give it to her as a present.")?;
                    }
                    2 => {
                        ctx.mes("I can't even imagine the taste.")?;
                    }
                    3 => {
                        ctx.mes("will guarantee you a safe battle.")?;
                    }
                    _ => {}
                }
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Although the Bunny Band best fits the Acolyte class:I love this thrilling feeling:Forming a party with people of different classes",
                    )],
                )? {
                    1 => {
                        ctx.mes("Although the Bunny Band best fits the Acolyte class")?;
                    }
                    2 => {
                        ctx.mes("I love this thrilling feeling")?;
                    }
                    3 => {
                        ctx.mes("Forming a party with people of different classes")?;
                    }
                    _ => {}
                }
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "I still wonder if the Bunny Band would be perfect for GOD-POING.:I won't be able to drink it even if my HP is less than 1.:is considered the best way to ready for battle.",
                    )],
                )? {
                    1 => {
                        ctx.mes("I still wonder if the Bunny Band would be perfect for GOD-POING.")?;
                    }
                    2 => {
                        ctx.mes("I won't be able to drink it even if my HP is less than 1.")?;
                    }
                    3 => {
                        ctx.mes("is considered the best way to ready for battle.")?;
                    }
                    _ => {}
                }
                ctx.next()?;
                ctx.mes("..........")?;
                ctx.next()?;
                ctx.mes("....................")?;
                ctx.next()?;
                ctx.mes(".................................")?;
                ctx.next()?;
                ctx.var("sage_q").set(Val::from(15))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(2051), Val::from(2052)])?;
                ctx.lines_as(
                    "Aebecee George",
                    args![
                        "Tee hee, so... are you done with your thesis? What do you think of your work? I think you did fine~",
                        "Don't forget, this is the first and the last time you'll write a thesis, okay? Tee hee~"
                    ],
                )?;
                ctx.call(Function::GetItem, vec![Val::from(1550), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Aebecee George",
                    args!["Okay, now you can show the dean your thesis~", "You're almost there~ tee hee."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines(args![
                    "What are you still doing here, sweet cheeks? Tee hee~",
                    "Oh, poor dear. Did you already forget what I told you? Do you want me to remind you? Not a problem, tee hee."
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Aebecee George",
                    args![
                        "^3355FF1 Feather of Birds^000000 which will be used as a pen,",
                        "^3355FF1 Animal Skin^000000 which will be used as paper,",
                        "^3355FF1 Trunk^000000 which will be used to bind a book,",
                        "^3355FF1 Squid Ink^000000 which will be used as ink,",
                        "^3355FF1 Empty Bottle^000000 which will be used for holding squid ink."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aebecee George",
                    args![
                        "You are supposed to prepare all of those items, aren't you?",
                        "Hurry baaack~! Tee hee~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("sage_q").get()? == 15 {
                ctx.lines(args![
                    "What are you doing here, dearie? Don't you want to meet the dean and show him your stuff?",
                    "Take it from me, boys like him won't wait for you forever, tee hee~"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Aebecee George",
                    args![
                        "I know, I know...you wanna play with little ol' me. But you should become a Sage first.",
                        "Please hurry up, tee hee."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.mes("Well well...what do we have here?")?;
                ctx.next()?;
                ctx.lines_as(
                    "Aebecee George",
                    args![
                        "You're not a Sage. Pity, sweet thang like yourself...",
                        "But seeing as you're not here for business, you must be here for..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Aebecee George", args!["...PLEASURE. Wait, where are you going, hon?"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn physics_professor_sa(ctx: &Ctx) -> Script {
    physics_professor_sa_body(ctx, Vec::new()).map(|_| ())
}

fn test_helper_talk_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Test Helper",
        args![
            "Welcome to the Sage practical examination hall.",
            "If you wish to take the test right now, please enter the waiting room."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Test Helper", args!["If someone is currently taking the test, please wait until that person finishes.", "The progression of the test is announced on the entire map. An announcement will be made when the next person is ready to leave the waiting room."])?;
    ctx.next()?;
    ctx.lines_as(
        "Test Helper",
        args![
            "The Test takes 5 ~ 10 minutes per each person.",
            "If you wish to leave the arena, please log out from the game."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn test_helper_talk(ctx: &Ctx) -> Script {
    test_helper_talk_body(ctx, Vec::new()).map(|_| ())
}

fn waiting_room_sg_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn waiting_room_sg(ctx: &Ctx) -> Script {
    waiting_room_sg_body(ctx, Vec::new()).map(|_| ())
}

fn waiting_room_sg_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Waiting Room#sg")])?;
    ctx.call(
        Function::WaitingRoom,
        vec![
            Val::from("Waiting Room"),
            Val::from(20),
            Val::from("Waiting Room#sg::OnStartArena"),
            Val::from(1),
        ],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn waiting_room_sg_oninit(ctx: &Ctx) -> Script {
    waiting_room_sg_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn waiting_room_sg_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WarpWaitingPc,
        vec![Val::from("job_sage"), Val::from(116), Val::from(97)],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Arena#1::OnEnable")])?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn waiting_room_sg_onstartarena(ctx: &Ctx) -> Script {
    waiting_room_sg_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn waiting_room_sg_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn waiting_room_sg_onenable(ctx: &Ctx) -> Script {
    waiting_room_sg_onenable_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Arena1Step {
    Start,
    OnInit,
    OnEnable,
    OnReset,
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

fn arena_1_run(ctx: &Ctx, mut step: Arena1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Arena1Step::Start => {
                step = Arena1Step::OnInit;
                continue 'machine;
            }
            Arena1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Arena#1")])?;
                return Err(Stop::End);
            }
            Arena1Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Arena#1")])?;
                ctx.var(".mymobs").set(Val::from(16))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(111),
                        Val::from(102),
                        Val::from("Grade F"),
                        Val::from(1183),
                        Val::from(1),
                        Val::from("Arena#1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(120),
                        Val::from(102),
                        Val::from("Grade F"),
                        Val::from(1183),
                        Val::from(1),
                        Val::from("Arena#1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(111),
                        Val::from(93),
                        Val::from("Grade F"),
                        Val::from(1183),
                        Val::from(1),
                        Val::from("Arena#1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(120),
                        Val::from(93),
                        Val::from("Grade F"),
                        Val::from(1183),
                        Val::from(1),
                        Val::from("Arena#1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(107),
                        Val::from(98),
                        Val::from("Grade F"),
                        Val::from(1183),
                        Val::from(1),
                        Val::from("Arena#1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(124),
                        Val::from(98),
                        Val::from("Grade F"),
                        Val::from(1183),
                        Val::from(1),
                        Val::from("Arena#1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(115),
                        Val::from(106),
                        Val::from("Grade F"),
                        Val::from(1183),
                        Val::from(1),
                        Val::from("Arena#1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(115),
                        Val::from(90),
                        Val::from("Grade F"),
                        Val::from(1183),
                        Val::from(1),
                        Val::from("Arena#1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(103),
                        Val::from(110),
                        Val::from("Grade D"),
                        Val::from(1184),
                        Val::from(1),
                        Val::from("Arena#1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(128),
                        Val::from(110),
                        Val::from("Grade D"),
                        Val::from(1184),
                        Val::from(1),
                        Val::from("Arena#1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(103),
                        Val::from(85),
                        Val::from("Grade D"),
                        Val::from(1184),
                        Val::from(1),
                        Val::from("Arena#1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(128),
                        Val::from(85),
                        Val::from("Grade D"),
                        Val::from(1184),
                        Val::from(1),
                        Val::from("Arena#1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(107),
                        Val::from(106),
                        Val::from("Grade D"),
                        Val::from(1184),
                        Val::from(1),
                        Val::from("Arena#1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(124),
                        Val::from(106),
                        Val::from("Grade D"),
                        Val::from(1184),
                        Val::from(1),
                        Val::from("Arena#1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(107),
                        Val::from(89),
                        Val::from("Grade D"),
                        Val::from(1184),
                        Val::from(1),
                        Val::from("Arena#1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(124),
                        Val::from(89),
                        Val::from("Grade D"),
                        Val::from(1184),
                        Val::from(1),
                        Val::from("Arena#1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Arena1Step::OnReset => {
                ctx.call(Function::KillMonster, vec![Val::from("job_sage"), Val::from("All")])?;
                return Err(Stop::End);
            }
            Arena1Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_sage"),
                            ((Val::from(" ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(", successfully defeat all the monsters.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Arena#2::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            Arena1Step::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_sage"),
                        Val::from("The practical examination has started."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            Arena1Step::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_sage"),
                        Val::from("1st test - time limit 3 minutes."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            Arena1Step::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_sage"),
                        Val::from("Please defeat all the monsters within the time limit."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            Arena1Step::OnTimer33000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_sage"),
                        Val::from("2 minutes 30 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            Arena1Step::OnTimer63000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_sage"), Val::from("2 minutes remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            Arena1Step::OnTimer93000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_sage"),
                        Val::from("1 minute 30 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            Arena1Step::OnTimer123000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_sage"), Val::from("1 min remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            Arena1Step::OnTimer153000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_sage"), Val::from("30 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            Arena1Step::OnTimer173000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_sage"), Val::from("10 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            Arena1Step::OnTimer183000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_sage"), Val::from("Time over."), ctx.constant("BC_MAP")?],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Arena#1::OnReset")])?;
                return Err(Stop::End);
            }
            Arena1Step::OnTimer184000 => {
                ctx.call(
                    Function::AreaWarp,
                    vec![
                        Val::from("job_sage"),
                        Val::from(100),
                        Val::from(82),
                        Val::from(131),
                        Val::from(113),
                        Val::from("yuno"),
                        Val::from(324),
                        Val::from(258),
                    ],
                )?;
                return Err(Stop::End);
            }
            Arena1Step::OnTimer185000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_sage"), Val::from("Next candidate, enter."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            Arena1Step::OnTimer186000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Arena#1")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Waiting Room#sg::OnEnable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arena_1(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::Start, Vec::new()).map(|_| ())
}

pub fn arena_1_oninit(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn arena_1_onenable(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn arena_1_onreset(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::OnReset, Vec::new()).map(|_| ())
}

pub fn arena_1_onmymobdead(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn arena_1_ontimer1000(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn arena_1_ontimer2000(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn arena_1_ontimer3000(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn arena_1_ontimer33000(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::OnTimer33000, Vec::new()).map(|_| ())
}

pub fn arena_1_ontimer63000(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::OnTimer63000, Vec::new()).map(|_| ())
}

pub fn arena_1_ontimer93000(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::OnTimer93000, Vec::new()).map(|_| ())
}

pub fn arena_1_ontimer123000(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::OnTimer123000, Vec::new()).map(|_| ())
}

pub fn arena_1_ontimer153000(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::OnTimer153000, Vec::new()).map(|_| ())
}

pub fn arena_1_ontimer173000(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::OnTimer173000, Vec::new()).map(|_| ())
}

pub fn arena_1_ontimer183000(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::OnTimer183000, Vec::new()).map(|_| ())
}

pub fn arena_1_ontimer184000(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::OnTimer184000, Vec::new()).map(|_| ())
}

pub fn arena_1_ontimer185000(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::OnTimer185000, Vec::new()).map(|_| ())
}

pub fn arena_1_ontimer186000(ctx: &Ctx) -> Script {
    arena_1_run(ctx, Arena1Step::OnTimer186000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Arena2Step {
    Start,
    OnInit,
    OnEnable,
    OnReset,
    OnMyMobDead,
    OnTimer1000,
    OnTimer2000,
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

fn arena_2_run(ctx: &Ctx, mut step: Arena2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Arena2Step::Start => {
                step = Arena2Step::OnInit;
                continue 'machine;
            }
            Arena2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Arena#2")])?;
                return Err(Stop::End);
            }
            Arena2Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Arena#2")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Arena#1::OnReset")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Arena#1")])?;
                ctx.var(".mymobs").set(Val::from(24))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(111),
                        Val::from(102),
                        Val::from("Numerical Value"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(111),
                        Val::from(102),
                        Val::from("Physics"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(111),
                        Val::from(102),
                        Val::from("History"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(120),
                        Val::from(102),
                        Val::from("Geography"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(120),
                        Val::from(102),
                        Val::from("Astronomy"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(120),
                        Val::from(102),
                        Val::from("Meteorology"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(111),
                        Val::from(93),
                        Val::from("Architecture"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(111),
                        Val::from(93),
                        Val::from("Control"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(111),
                        Val::from(93),
                        Val::from("Instrumentology"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(120),
                        Val::from(102),
                        Val::from("Statistics;"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(120),
                        Val::from(102),
                        Val::from("Graphic Method"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(120),
                        Val::from(102),
                        Val::from("Language"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(111),
                        Val::from(93),
                        Val::from("Sitology"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(111),
                        Val::from(93),
                        Val::from("Dietetics"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(111),
                        Val::from(93),
                        Val::from("Landscape Architecture"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(120),
                        Val::from(93),
                        Val::from("Anthropology"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(120),
                        Val::from(93),
                        Val::from("Biology"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(120),
                        Val::from(93),
                        Val::from("Ethics"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(107),
                        Val::from(98),
                        Val::from("Economy"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(107),
                        Val::from(98),
                        Val::from("Politics"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(107),
                        Val::from(98),
                        Val::from("Photography"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(124),
                        Val::from(98),
                        Val::from("Dendrology"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(124),
                        Val::from(98),
                        Val::from("Hygiene"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(124),
                        Val::from(98),
                        Val::from("Sociology"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Arena#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Arena2Step::OnReset => {
                ctx.call(Function::KillMonster, vec![Val::from("job_sage"), Val::from("All")])?;
                return Err(Stop::End);
            }
            Arena2Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_sage"),
                            ((Val::from(" ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(", successfully defeat all the monsters.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Arena#Doorkeeper::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Arena#3::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            Arena2Step::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_sage"),
                        Val::from("Second test - time limit 3 minutes."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            Arena2Step::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_sage"),
                        Val::from("Please defeat all the monsters within the time limit."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            Arena2Step::OnTimer33000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_sage"),
                        Val::from("2 minutes 30 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            Arena2Step::OnTimer63000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_sage"), Val::from("2 minutes remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            Arena2Step::OnTimer93000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_sage"),
                        Val::from("1 minute 30 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            Arena2Step::OnTimer123000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_sage"), Val::from("1 min remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            Arena2Step::OnTimer153000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_sage"), Val::from("30 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            Arena2Step::OnTimer173000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_sage"), Val::from("10 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            Arena2Step::OnTimer183000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_sage"), Val::from("Time over."), ctx.constant("BC_MAP")?],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Arena#2::OnReset")])?;
                return Err(Stop::End);
            }
            Arena2Step::OnTimer184000 => {
                ctx.call(
                    Function::AreaWarp,
                    vec![
                        Val::from("job_sage"),
                        Val::from(100),
                        Val::from(82),
                        Val::from(131),
                        Val::from(113),
                        Val::from("yuno"),
                        Val::from(324),
                        Val::from(258),
                    ],
                )?;
                return Err(Stop::End);
            }
            Arena2Step::OnTimer185000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_sage"),
                        Val::from("Next candidate, please enter."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            Arena2Step::OnTimer186000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Arena#2")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Waiting Room#sg::OnEnable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arena_2(ctx: &Ctx) -> Script {
    arena_2_run(ctx, Arena2Step::Start, Vec::new()).map(|_| ())
}

pub fn arena_2_oninit(ctx: &Ctx) -> Script {
    arena_2_run(ctx, Arena2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn arena_2_onenable(ctx: &Ctx) -> Script {
    arena_2_run(ctx, Arena2Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn arena_2_onreset(ctx: &Ctx) -> Script {
    arena_2_run(ctx, Arena2Step::OnReset, Vec::new()).map(|_| ())
}

pub fn arena_2_onmymobdead(ctx: &Ctx) -> Script {
    arena_2_run(ctx, Arena2Step::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn arena_2_ontimer1000(ctx: &Ctx) -> Script {
    arena_2_run(ctx, Arena2Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn arena_2_ontimer2000(ctx: &Ctx) -> Script {
    arena_2_run(ctx, Arena2Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn arena_2_ontimer33000(ctx: &Ctx) -> Script {
    arena_2_run(ctx, Arena2Step::OnTimer33000, Vec::new()).map(|_| ())
}

pub fn arena_2_ontimer63000(ctx: &Ctx) -> Script {
    arena_2_run(ctx, Arena2Step::OnTimer63000, Vec::new()).map(|_| ())
}

pub fn arena_2_ontimer93000(ctx: &Ctx) -> Script {
    arena_2_run(ctx, Arena2Step::OnTimer93000, Vec::new()).map(|_| ())
}

pub fn arena_2_ontimer123000(ctx: &Ctx) -> Script {
    arena_2_run(ctx, Arena2Step::OnTimer123000, Vec::new()).map(|_| ())
}

pub fn arena_2_ontimer153000(ctx: &Ctx) -> Script {
    arena_2_run(ctx, Arena2Step::OnTimer153000, Vec::new()).map(|_| ())
}

pub fn arena_2_ontimer173000(ctx: &Ctx) -> Script {
    arena_2_run(ctx, Arena2Step::OnTimer173000, Vec::new()).map(|_| ())
}

pub fn arena_2_ontimer183000(ctx: &Ctx) -> Script {
    arena_2_run(ctx, Arena2Step::OnTimer183000, Vec::new()).map(|_| ())
}

pub fn arena_2_ontimer184000(ctx: &Ctx) -> Script {
    arena_2_run(ctx, Arena2Step::OnTimer184000, Vec::new()).map(|_| ())
}

pub fn arena_2_ontimer185000(ctx: &Ctx) -> Script {
    arena_2_run(ctx, Arena2Step::OnTimer185000, Vec::new()).map(|_| ())
}

pub fn arena_2_ontimer186000(ctx: &Ctx) -> Script {
    arena_2_run(ctx, Arena2Step::OnTimer186000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArenaDoorkeeperStep {
    Start,
    OnInit,
    OnEnable,
    OnReset,
    OnDisable,
    OnMyMobDead,
    OnTimer1000,
    OnTimer30000,
    OnTimer50000,
    OnTimer60000,
    OnTimer61000,
    OnTimer62000,
    OnTimer63000,
}

fn arena_doorkeeper_run(ctx: &Ctx, mut step: ArenaDoorkeeperStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArenaDoorkeeperStep::Start => {
                step = ArenaDoorkeeperStep::OnInit;
                continue 'machine;
            }
            ArenaDoorkeeperStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Arena#Doorkeeper")])?;
                return Err(Stop::End);
            }
            ArenaDoorkeeperStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Arena#Doorkeeper")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Arena#2::OnReset")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Arena#2")])?;
                ctx.var(".mymobs").set(Val::from(1))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(116),
                        Val::from(97),
                        Val::from("Academic Probation"),
                        Val::from(1179),
                        Val::from(1),
                        Val::from("Arena#Doorkeeper::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ArenaDoorkeeperStep::OnReset => {
                ctx.call(Function::KillMonster, vec![Val::from("job_sage"), Val::from("All")])?;
                return Err(Stop::End);
            }
            ArenaDoorkeeperStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Arena#3")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Arena#Doorkeeper")])?;
                return Err(Stop::End);
            }
            ArenaDoorkeeperStep::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_sage"),
                            ((Val::from("Congratulations, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(". You passed the test.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                    ctx.var("sage_q").set(Val::from(8))?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Test Helper#sg::OnEnable")])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Arena#Doorkeeper")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            ArenaDoorkeeperStep::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_sage"),
                        Val::from("3rd test - Time limit 1 minute."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ArenaDoorkeeperStep::OnTimer30000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_sage"), Val::from("30 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            ArenaDoorkeeperStep::OnTimer50000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_sage"), Val::from("10 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            ArenaDoorkeeperStep::OnTimer60000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_sage"), Val::from("Time over."), ctx.constant("BC_MAP")?],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Arena#Doorkeeper::OnReset")])?;
                return Err(Stop::End);
            }
            ArenaDoorkeeperStep::OnTimer61000 => {
                ctx.call(
                    Function::AreaWarp,
                    vec![
                        Val::from("job_sage"),
                        Val::from(100),
                        Val::from(82),
                        Val::from(131),
                        Val::from(113),
                        Val::from("yuno"),
                        Val::from(324),
                        Val::from(258),
                    ],
                )?;
                return Err(Stop::End);
            }
            ArenaDoorkeeperStep::OnTimer62000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_sage"),
                        Val::from("Next candidate, please enter."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ArenaDoorkeeperStep::OnTimer63000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Arena#Doorkeeper")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Waiting Room#sg::OnEnable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arena_doorkeeper(ctx: &Ctx) -> Script {
    arena_doorkeeper_run(ctx, ArenaDoorkeeperStep::Start, Vec::new()).map(|_| ())
}

pub fn arena_doorkeeper_oninit(ctx: &Ctx) -> Script {
    arena_doorkeeper_run(ctx, ArenaDoorkeeperStep::OnInit, Vec::new()).map(|_| ())
}

pub fn arena_doorkeeper_onenable(ctx: &Ctx) -> Script {
    arena_doorkeeper_run(ctx, ArenaDoorkeeperStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn arena_doorkeeper_onreset(ctx: &Ctx) -> Script {
    arena_doorkeeper_run(ctx, ArenaDoorkeeperStep::OnReset, Vec::new()).map(|_| ())
}

pub fn arena_doorkeeper_ondisable(ctx: &Ctx) -> Script {
    arena_doorkeeper_run(ctx, ArenaDoorkeeperStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn arena_doorkeeper_onmymobdead(ctx: &Ctx) -> Script {
    arena_doorkeeper_run(ctx, ArenaDoorkeeperStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn arena_doorkeeper_ontimer1000(ctx: &Ctx) -> Script {
    arena_doorkeeper_run(ctx, ArenaDoorkeeperStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn arena_doorkeeper_ontimer30000(ctx: &Ctx) -> Script {
    arena_doorkeeper_run(ctx, ArenaDoorkeeperStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn arena_doorkeeper_ontimer50000(ctx: &Ctx) -> Script {
    arena_doorkeeper_run(ctx, ArenaDoorkeeperStep::OnTimer50000, Vec::new()).map(|_| ())
}

pub fn arena_doorkeeper_ontimer60000(ctx: &Ctx) -> Script {
    arena_doorkeeper_run(ctx, ArenaDoorkeeperStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn arena_doorkeeper_ontimer61000(ctx: &Ctx) -> Script {
    arena_doorkeeper_run(ctx, ArenaDoorkeeperStep::OnTimer61000, Vec::new()).map(|_| ())
}

pub fn arena_doorkeeper_ontimer62000(ctx: &Ctx) -> Script {
    arena_doorkeeper_run(ctx, ArenaDoorkeeperStep::OnTimer62000, Vec::new()).map(|_| ())
}

pub fn arena_doorkeeper_ontimer63000(ctx: &Ctx) -> Script {
    arena_doorkeeper_run(ctx, ArenaDoorkeeperStep::OnTimer63000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Arena3Step {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
}

fn arena_3_run(ctx: &Ctx, mut step: Arena3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Arena3Step::Start => {
                step = Arena3Step::OnInit;
                continue 'machine;
            }
            Arena3Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Arena#3")])?;
                return Err(Stop::End);
            }
            Arena3Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(107),
                        Val::from(106),
                        Val::from("Absent 3 times"),
                        Val::from(1185),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(124),
                        Val::from(106),
                        Val::from("Being Late 5 times"),
                        Val::from(1185),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(107),
                        Val::from(89),
                        Val::from("Cheating 2 times"),
                        Val::from(1185),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_sage"),
                        Val::from(124),
                        Val::from(89),
                        Val::from("Cheating 4 times"),
                        Val::from(1185),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            Arena3Step::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("job_sage"), Val::from("All")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arena_3(ctx: &Ctx) -> Script {
    arena_3_run(ctx, Arena3Step::Start, Vec::new()).map(|_| ())
}

pub fn arena_3_oninit(ctx: &Ctx) -> Script {
    arena_3_run(ctx, Arena3Step::OnInit, Vec::new()).map(|_| ())
}

pub fn arena_3_onenable(ctx: &Ctx) -> Script {
    arena_3_run(ctx, Arena3Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn arena_3_ondisable(ctx: &Ctx) -> Script {
    arena_3_run(ctx, Arena3Step::OnDisable, Vec::new()).map(|_| ())
}
