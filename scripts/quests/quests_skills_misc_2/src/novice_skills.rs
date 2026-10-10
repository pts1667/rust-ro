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

pub fn nami(ctx: &Ctx) -> Script {
    if (ctx.player().class()? == constants::JOB_NOVICE || ctx.player().class()? == constants::JOB_BABY)
        && (ctx.player().job_level()? > 3 || ctx.player().base_level()? > 11)
        && ctx.var("skill_nov").get()?.number()? < 3
    {
        ctx.lines_as(
            "Nami",
            args![
                "Hello!",
                "I want to be a nurse so bad!",
                "I always go and try to learn more.",
                "Actually, I'm really good.",
                "Do you want me to try on you? ? ? . ."
            ],
        )?;
        ctx.next()?;
        match ctx.menu(&["Continue conversation", "Slowly slink away. . . ."])? {
            0 => {
                if ctx.var("skill_nov").get()?.number()? >= 0 && ctx.var("skill_nov").get()?.number()? <= 2 {
                    match ctx.var("skill_nov").get()?.number()? {
                        0 => {
                            ctx.lines_as(
                                "Nami",
                                args![
                                    "Thank you for giving me this chance!",
                                    "I will do the best I can.",
                                    "When I am nursing people, I never",
                                    "give half effort. -"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nami",
                                args![
                                    "Let me explain to you about",
                                    "this skill I am using, ^3355FF' First Aid '^000000 ",
                                    "It doesn't take any special",
                                    "equipment or items . .",
                                    "Using only old cloth, left over potions,",
                                    "and some other unsubstantial materials"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nami",
                                args![
                                    "you can perform first aid. -",
                                    "It is a simple skill that you can use",
                                    "to regain a small amount of HP.",
                                    "I wouldn't mind teaching you. . .",
                                    "If you want to learn this skill,",
                                    "it does not take much to learn. . ."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nami",
                                args![
                                    "First aid does require",
                                    "some small preparations. . .",
                                    "Some simple items you should have on hand are",
                                    "^3355FF' 3 Red Herb '^000000 ",
                                    "^3355FF' 3 Clover '^000000 ",
                                    "^3355FF' 1 Sterilized Bandages '^000000 "
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nami",
                                args![
                                    "You can find these items being",
                                    "carried by monsters close by or",
                                    "even buy them from a merchant.",
                                    "It shouldn't be difficult to prepare",
                                    "these items for your first aid skill."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nami",
                                args![
                                    "The only item you must take",
                                    "a special trip for is the bandage.",
                                    "On the eastern side second floor ",
                                    "of the prontera castle you can find",
                                    "a nurse who will supply you with this",
                                    "item readily. You should see her for this item"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nami",
                                args![
                                    "If you go and find these items, ",
                                    "I will be happy to teach you this skill.",
                                    "Well, I will be awaiting your return."
                                ],
                            )?;
                            ctx.var("skill_nov").set(Val::from(1))?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as(
                                "Nami",
                                args![
                                    "First aid does require",
                                    "some small preparations. . .",
                                    "Some simple items you should have on hand are",
                                    "^3355FF' 3 Red Herb '^000000 ",
                                    "^3355FF' 3 Clover '^000000 ",
                                    "^3355FF' 1 Sterilized Bandages '^000000 "
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nami",
                                args![
                                    "You can find these items being",
                                    "carried by monsters close by or",
                                    "even buy them from a merchant.",
                                    "It shouldn't be difficult to prepare",
                                    "these items for your first aid skill."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nami",
                                args![
                                    "The only item you must take",
                                    "a special trip for is the bandage.",
                                    "Two maps east of here you can find",
                                    "a nurse who will supply you with this",
                                    "item readily. You should see her for",
                                    "this item. . . "
                                ],
                            )?;
                            return ctx.close();
                        }
                        2 => {
                            if ctx.items().count(507)? > 2 && ctx.items().count(705)? > 2 {
                                ctx.items().take(507, 3)?;
                                ctx.items().take(705, 3)?;
                                ctx.lines_as(
                                    "Nami",
                                    args![
                                        "Hello, welcome back!",
                                        "You have done well at finding",
                                        "the necessary items.",
                                        "I know that the Nurse is a little",
                                        "strange, I am sure it was a little perplexing. . .",
                                        "hee hee hee . . . . ."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Nami",
                                    args![
                                        "Well, let us begin our training.",
                                        "When using the first aid skill",
                                        "you will use about 3 SP and convert",
                                        "this energy into about 5 HP.",
                                        "This is done with your first aid skill",
                                        "and supplies."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Nami",
                                    args![
                                        "You should take this and place it here. . .",
                                        "Then you can stop the bleeding. . .",
                                        "After that you should apply this. . .",
                                        "There!!! Isn't it easy? ! ? !"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Nami",
                                    args![
                                        ". . Basically.",
                                        "You can take a little of left",
                                        "herbs and common items",
                                        "and combine them together and. . .",
                                        "Presto !!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Skill, args!["NV_FIRSTAID", 1, constants::SKILL_PERM])?;
                                ctx.var("skill_nov").set(Val::from(3))?;
                                ctx.lines_as(
                                    "Nami",
                                    args![
                                        "Yes yes, that's right!",
                                        "Now that you have this",
                                        "skill, I hope that it helps you",
                                        "in the future.",
                                        "Thank you and have a great day !~~~"
                                    ],
                                )?;
                                return ctx.close();
                            }
                            ctx.lines_as(
                                "Nami",
                                args![
                                    "First aid does require",
                                    "some small preparations. . .",
                                    "Some simple items you should have on hand are",
                                    "^3355FF' 3 Red_Herb '^000000 ",
                                    "^3355FF' 3 Clover '^000000 ",
                                    "^3355FF' 1 Sterilized Bandages '^000000 "
                                ],
                            )?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                ctx.lines_as(
                    "Nami",
                    args![
                        ". . . . . Z z z",
                        "I am so sorry!!!...I know I did it wrong...!!",
                        "Wahhhh ..",
                        "!! *sigh* !!",
                        "- *rolls her eyes* -"
                    ],
                )?;
                return ctx.close();
            }
            1 => {
                ctx.lines_as(
                    "Nami",
                    args![
                        "Hey! .. Where are you going? !",
                        "Come over here and have a shot !",
                        "Prick and its over !!",
                        "Grab that patient! ! !"
                    ],
                )?;
                return ctx.close();
            }
            _ => {}
        }
    }
    ctx.lines_as(
        "Nami",
        args![
            "I am working hard to receive",
            "my nursing license . . .",
            "I don't think that I lack anything",
            "to become a nurse . .",
            "It must be because of my lack of my experience ?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Nami",
        args![
            "If only you had at least",
            "a first job . .",
            "or maybe be over ^3355FF novice job level 4^000000",
            "I could have talked to",
            "you a little longer. . . . . . ."
        ],
    )?;
    ctx.close()
}

pub fn chivalry_member(ctx: &Ctx) -> Script {
    if (ctx.player().class()? == constants::JOB_NOVICE || ctx.player().class()? == constants::JOB_BABY)
        && ctx.player().job_level()? > 6
        && (ctx.var("skill_nov").get()?.number()? >= 3 && ctx.var("skill_nov").get()?.number()? <= 5)
    {
        ctx.lines_as(
            "Bulma",
            args![
                "Yeah. . . I look great. . .",
                "I am a knight in the knight's",
                "guild of Prontera! Kuhahhahhahah !!",
                "It hasn't been long since I became a",
                " knight, but I still look great huh?",
                "What do you think? ? ?"
            ],
        )?;
        ctx.next()?;
        match ctx.var("skill_nov").get()?.number()? {
            3 => {
                ctx.lines_as(
                    "Bulma",
                    args![
                        "Hello my young friend -",
                        "You remind me of myself as young",
                        "sword man. . . ",
                        "Kekekkek, Oh I miss those days . .",
                        "Look at me acting like an old man.",
                        "Heh heh Sorry..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bulma",
                    args![
                        "I'm still young!!! Aren't I???",
                        "If you just work hard and be patient,",
                        "you will soon receive the job you desire as well.",
                        "It takes patience, but this is",
                        "good life kekkeke",
                        "Hmm, I would like to help you out . ."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bulma",
                    args![
                        ". . . . . I'm not sure why you are looking",
                        "at me like that. I assure you that this",
                        "might seem a little weird at first, but",
                        "what I tell you will most likely be a great aid to you.."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bulma",
                    args![
                        "Ha ha... It looks like you are getting",
                        "a bit interested in what I have to say.",
                        "I can teach you a very useful skill!",
                        "This skill is acting like you are dead! '",
                        "No No, it is more than acting, you ",
                        "actually look dead!!! .."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bulma",
                    args![
                        "The name of the skill is ^3355FF' Play Dead '^000000",
                        "It is a skill I used as a novice.",
                        "But don't think little of it because",
                        "it is a novice skill. In fact, it takes",
                        "extreme concentration and skill",
                        "to even make this skill pass as believable."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bulma",
                    args![
                        "For example,",
                        "What if you are attacked by a strong",
                        "monster and can't survive.",
                        "You must play dead!",
                        "But if the monster was to tickle you,",
                        "could you control yourself?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bulma",
                    args![
                        "The skill will help you deal with",
                        "many situations such as this.",
                        "It is truly a skill for the strong minded.",
                        "The goal of the skill is to look",
                        "perfectly dead. . ."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bulma",
                    args![
                        "I think that's enough explanation.",
                        "I can tell by the look in your eyes",
                        "that you are ready for your training.",
                        "Lets not delay!",
                        "Ok, take this pill first. . .",
                        "Let's see how this goes. . ."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bulma",
                    args![
                        "Within in 10 minutes, you must",
                        "go to the 2nd floor of the Prontera Castle's",
                        "East wing. ^3355FF' Newbie Tag '^000000 is",
                        "the item you are seeking. ! . ."
                    ],
                )?;
                ctx.next()?;
                ctx.mes("^3355FF- *Gulp* (You have swallowed the pill) -^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "Bulma",
                    args![
                        "The pill that you have just taken",
                        "will make it difficult for you to breathe. . .",
                        "Kekekk . .AH HA - Just joking !",
                        "It is actually a pill to gives you",
                        "a mental calm so you can be patient.",
                        "I think that nothing is better"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bulma",
                    args![
                        "training than running.",
                        "Okay? GO! ! ! ! ~~~~~",
                        "If you are late, you have to do it again! !",
                        "Now GO ! GO ! GO !",
                        "Run ~~~~~!!!!"
                    ],
                )?;
                ctx.var("skill_nov").set(Val::from(4))?;
                return ctx.close();
            }
            4 => {
                ctx.lines_as(
                    "Bulma",
                    args![
                        "HEY! what are you doing here ? !",
                        "You must be very irresponsible to be",
                        "here when your time is running out.",
                        "Run Run Run! - - - - -"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bulma",
                    args![
                        "On the 2nd floor of the east wing!",
                        "It's in the Prontera castle!",
                        "Okayyyyy~~~!! *waves goodbye*"
                    ],
                )?;
                return ctx.close();
            }
            5 => {
                if ctx.items().count(7039)? > 0 {
                    ctx.lines_as(
                        "Bulma",
                        args![
                            "Hey... I see that you have -",
                            "finished your quest! ! ! . .",
                            "If you can endure all this,",
                            "it shouldn't be a problem to use",
                            "this skill. You are a natural!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bulma",
                        args![
                            "Now, if you ever feel threatened,",
                            "use this skill as you see fit.",
                            " ' Play Dead ' ",
                            "Okay okay, See you around ! ! !"
                        ],
                    )?;
                    ctx.var("skill_nov").set(Val::from(6))?;
                    ctx.items().take(7039, 1)?;
                    ctx.call(Function::Skill, args!["NV_TRICKDEAD", 1, constants::SKILL_PERM])?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Bulma",
                    args![
                        "What? -",
                        "Why haven't you finished your quest?",
                        " *Tsk* *Tsk* You must have lost the pass . . .",
                        "Such irresponsibility is not acceptable.",
                        "I can't accept you into training until",
                        "I know you are capable. Go and try again."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bulma",
                    args![
                        "Don't take me lightly. . .",
                        "If I tell you to come in 10 minutes,",
                        "I expect that of you.",
                        "In order for you to have another chance,",
                        "you must start from the beginning. . .",
                        "Do it right this time ! ! ! !"
                    ],
                )?;
                ctx.var("skill_nov").set(Val::from(4))?;
                return ctx.close();
            }
            _ => {}
        }
    }
    ctx.lines_as(
        "Bulma",
        args![
            "Yeah... I remember back to long ago !",
            "Especially those embarrassing Novice years.",
            "Wow... It is funny to think about those years now.",
            "Those years were difficult. . .",
            "Thankfully you can use the First Aid",
            "skill when you reach novice job level 7."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Bulma",
        args![
            "That saved me many times in the past. . .",
            "I am sure it will help you much as well.",
            "Hopefully, it will be something you use well . ."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Bulma",
        args![
            "If you have any friends who are novices,",
            "tell them about me.",
            "If I can, I will help them out",
            "as best as I can . . . ."
        ],
    )?;
    ctx.close()
}

pub fn nursing_instructor(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Dread Lord",
        args![
            ". . . . . . . . . .",
            ". . . . . . . . . .",
            "Stop pestering me! ! !",
            "I am very busy ! ! !",
            "Would you just bug off ? ! ? !"
        ],
    )?;
    ctx.close_window()?;
    if ctx.var("skill_nov").get()?.number()? >= 0 && ctx.var("skill_nov").get()?.number()? <= 5 {
        match ctx.var("skill_nov").get()?.number()? {
            0 => {
                ctx.lines_as(
                    "Dread Lord",
                    args![
                        "The people who work here at",
                        "Prontera clinic are battling life",
                        "and death everyday.",
                        "It takes a lot of patience and ",
                        "puts a lot of tension on us.",
                        "Sorry if we seem a bit uptight. . ."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dread Lord",
                    args![
                        "I am the Nursing director.",
                        "There are many things I must do.",
                        "If your business is complete, please leave."
                    ],
                )?;
                return ctx.close();
            }
            1 => {
                ctx.lines_as(
                    "Dread Lord",
                    args![
                        "Ohhh . .",
                        "You have come here for bandages ?",
                        "Do you even know how we get these",
                        "precious bandages ?",
                        "They come from a powerful monster",
                        "that is found in the pyramids of Morocc."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dread Lord",
                    args![
                        "The monster is some sort of mummy.",
                        "We take the rotten bandages from it's",
                        "diseased body and sanitize them. . .",
                        "Do you believe me ? . .",
                        "Heh heh... There is even poison",
                        "in the bandages."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dread Lord",
                    args![
                        "Fortunately, the poison kills other poisons",
                        "and does not hurt the patient . .",
                        "You must destroy poison with poison. . .",
                        "I think I heard something similar that. . .",
                        ". . . . . Anyway, I would usually give you a",
                        "hard time for taking these bandages lightly,"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dread Lord",
                    args![
                        "but I am much too tired today to",
                        "give you any trouble. . .",
                        "If you ever get skilled in medicine and first aid,",
                        "please consider joining our clinic.",
                        "Now don't take these bandages and then",
                        "go and get killed, be careful. -"
                    ],
                )?;
                ctx.next()?;
                ctx.mes("^3355FF- Got 1 Sterilized Bandages -^000000")?;
                ctx.var("skill_nov").set(Val::from(2))?;
                return ctx.close();
            }
            2 => {
                ctx.lines_as(
                    "Dread Lord",
                    args![
                        "Look, if your business is done",
                        "get out of my site! ! !",
                        "I have no time for this nonsense ..",
                        ". . . . .",
                        "- Click Click *Walks away* . . . . . -"
                    ],
                )?;
                return ctx.close();
            }
            4 => {
                if ctx.call(Function::CountItem, args![7039])? == 0 {
                    ctx.lines_as(
                        "Dread Lord",
                        args![
                            "Look at this guy!",
                            "Wake up and watch where you are going.",
                            "What are you thinking running around our clinic!",
                            "Running ?!?!",
                            "*Sigh* . . ",
                            "What do you want? ! ? !"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dread Lord",
                        args![
                            "I see. . . .",
                            ". . . . . what ?",
                            "You want to have a bandage to learn first aid?",
                            " *Arhg* Here take it !",
                            "-woosh - *storms away*"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("^3355FF- Got 1 Newbie Tag -^000000")?;
                    ctx.var("skill_nov").set(Val::from(5))?;
                    ctx.items().give(7039, 1)?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Dread Lord",
                    args![
                        "What ! !",
                        "I'm only going to give you one!",
                        "You don't need any more for the test !",
                        "Do you want to stay a novice forever???",
                        "I would be happy to arrange that !"
                    ],
                )?;
                return ctx.close();
            }
            5 => {
                ctx.lines_as("Dread Lord", args!["What more do you want !", "Get out of here !", "OUT !"])?;
                return ctx.close();
            }
            _ => {}
        }
    }
    ctx.lines_as(
        "Dread Lord",
        args![
            "Argh, Get out of here !!",
            "I don't like shouting but ..",
            "GET OUT OF HERE! ! ! ! ! !"
        ],
    )?;
    ctx.close()
}
