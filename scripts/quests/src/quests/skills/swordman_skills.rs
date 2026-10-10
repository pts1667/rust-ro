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

pub fn knightdethomas(ctx: &Ctx) -> Script {
    if ctx.var("BaseClass").get()? == constants::JOB_SWORDMAN
        && (ctx.var("JobLevel").get()?.number()? >= 35
            || ctx.var("BaseJob").get()? == constants::JOB_KNIGHT
            || ctx.var("BaseJob").get()? == constants::JOB_CRUSADER)
    {
        if ctx.call(Function::GetSkillLv, args!["SM_MOVINGRECOVERY"])? == 1 {
            ctx.lines_as(
                "De Thomas",
                args![
                    "Oh, it's you?",
                    "Long time no see!",
                    "You seem healthier than before.",
                    "Hahahaha!",
                    "Take care! See you again!"
                ],
            )?;
            return ctx.close();
        }
        if ctx.items().count(713)? >= 200 && ctx.items().count(1058)? >= 1 {
            ctx.lines_as("De Thomas", args!["Welcome back...", "Are you ready to learn Body Movin'?"])?;
            ctx.next()?;
            match ctx.menu(&["Yes.", "No, I'm not ready yet."])? {
                0 => {
                    ctx.lines_as("De Thomas", args!["Let's see....."])?;
                    ctx.next()?;
                    ctx.lines_as("De Thomas", args!["Ok! I shall now teach you...", "...The Body Movin' skill!"])?;
                    ctx.next()?;
                    ctx.items().take(713, 200)?;
                    ctx.items().take(1058, 1)?;
                    ctx.call(Function::Skill, args!["SM_MOVINGRECOVERY", 1, constants::SKILL_PERM])?;
                    ctx.lines_as("De Thomas", args!["There you go!", "Try it yourself.", "But don't overdo it."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "De Thomas",
                        args!["Oh yeah, I won't be needing your", "armor so you can keep it.", "Good luck now!"],
                    )?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as("De Thomas", args!["Is that so?", "Then come when you are prepared."])?;
                    return ctx.close();
                }
                _ => {}
            }
        } else {
            ctx.lines_as(
                "De Thomas",
                args![
                    "Hmmm... He's a swordsman...",
                    "You know that I am De Thomas Carlos, right?",
                    "Knight of Prontera's 3rd Calvary.",
                    "De Thomas Carlos!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("De Thomas", args!["You do not have all the items I asked for."])?;
            ctx.next()?;
            ctx.lines_as("De Thomas", args!["Remember, I need ^008800200 empty bottles^000000, your armor, and a ^008800Moth Wing^000000. Come back when you have it all."])?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "De Thomas",
            args![
                "My name is De Thomas Carlos.",
                "Knight of Prontera's 3rd Calvary.",
                "I have a certain duty these days.",
                "Ehem! Need I say more."
            ],
        )?;
        return ctx.close();
    }
    ctx.end()
}

pub fn knightdethomas_ontouch(ctx: &Ctx) -> Script {
    if ctx.var("BaseClass").get()? == constants::JOB_SWORDMAN
        && (ctx.var("JobLevel").get()?.number()? >= 35
            || ctx.var("BaseJob").get()? == constants::JOB_KNIGHT
            || ctx.var("BaseJob").get()? == constants::JOB_CRUSADER)
    {
        ctx.lines_as(
            "De Thomas",
            args![
                "Oh, no! You must have been hurt! Are you ok?",
                "You must have fought hard to get such serious injuries..",
                "Being a swordsman must come with a lot of responsibility and sacrifice."
            ],
        )?;
        ctx.next()?;
        ctx.mes("[De Thomas]")?;
        if ctx.var("Sex").get()? == constants::SEX_MALE {
            ctx.mes("For these swordsmen and knights, there is a wonderful skill.")?;
        } else {
            ctx.mes("For these swordswomen and knights, there is a wonderful skill young lady.")?;
        }
        ctx.lines(args![
            "I present to you - HP Recovery While Moving!",
            "Body moving is a splendid skill",
            "that allows you to regain strength(HP)",
            "while you are moving!"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "De Thomas",
            args![
                "It is currently under development",
                "so it may not recover that much,",
                "but it will help a little.",
                "What do you think? Would you like to learn this skill?"
            ],
        )?;
        ctx.next()?;
        match ctx.menu(&["What a nice skill! I want to learn it!", "No, thank you."])? {
            0 => {
                ctx.lines_as(
                    "De Thomas",
                    args![
                        "Very well. I will tell you what you need to learn this skill.",
                        "First, your job level must be higher than ^00880035^000000.",
                        "You will also need ^008800200 empty bottles^000000.",
                        "Why? Because it is proof that you fought fiercely to have used that many potions."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "De Thomas",
                    args![
                        "Also, the armor you used in battle.",
                        "This is also proof of an experienced fighter.",
                        "For the armor... your armor is perfect!",
                        "Bring your armor!",
                        "Last but not least... bring me one ^008800Moth Wing^000000."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Eh? You need that, too?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "De Thomas",
                    args![
                        "Not really.. I don't really NEED it.",
                        "It's just that my niece has gotten a bug hunting as a holiday task during the summer vacation.",
                        "Of course! It would be much easier for me to get it myself.",
                        "but I must work here all the time so I don't exactly have the time to go out and get it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "De Thomas",
                    args![
                        "Don't you think it is pitiful that I have to stay in once place everyday, not being able to go outside?",
                        "Please, find me one...*sniffsniff*",
                        "If you don't..."
                    ],
                )?;
                return ctx.close();
            }
            1 => {
                ctx.lines_as("De Thomas", args!["..."])?;
                return ctx.close();
            }
            _ => {}
        }
    }
    ctx.end()
}

pub fn leon_von_frich(ctx: &Ctx) -> Script {
    if ctx.var("BaseClass").get()? == constants::JOB_SWORDMAN
        && (ctx.var("JobLevel").get()?.number()? >= 25
            || ctx.var("BaseJob").get()? == constants::JOB_KNIGHT
            || ctx.var("BaseJob").get()? == constants::JOB_CRUSADER)
    {
        if ctx.call(Function::GetSkillLv, args!["SM_FATALBLOW"])? == 1 {
            ctx.lines_as(
                "Leon",
                args![
                    "Eh?",
                    "I was wondering who that was!",
                    "Why it's you from before!",
                    "Nice to see you again! How are you?",
                    "Be careful! Hahaha!"
                ],
            )?;
            return ctx.close();
        } else if ctx.items().count(1752)? > 9
            && ctx.items().count(1751)? > 9
            && ctx.items().count(532)? > 0
            && ctx.items().count(962)? > 29
            && ctx.items().count(526)? > 4
        {
            ctx.lines_as(
                "Leon",
                args![
                    "Ooh! You are more than ready",
                    "to learn Fatal Blow!",
                    "So how about it? Would you like to learn?"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Yes!", "No, I don't want to.", "But, what is Fatal Blow?"])? {
                0 => {
                    ctx.lines_as("Leon", args!["OK, lets begin!"])?;
                    ctx.next()?;
                    ctx.items().take(1752, 10)?;
                    ctx.items().take(1751, 10)?;
                    ctx.items().take(532, 1)?;
                    ctx.items().take(962, 30)?;
                    ctx.items().take(526, 5)?;
                    ctx.call(Function::Skill, args!["SM_FATALBLOW", 1, constants::SKILL_PERM])?;
                    ctx.lines_as(
                        "Leon",
                        args!["Success!", "Go use your new skill to its full potential.", "Hahahahahahahaha!"],
                    )?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as("Leon", args!["I don't like you!!!"])?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Leon",
                        args![
                            "I developed this skill recently.",
                            "When you use bash, depending ",
                            "on your level, you can stun ",
                            "your opponent. .",
                            "You have learned bash, haven't you?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Leon",
                        args![
                            "What do you think. Stun is",
                            "a very useful technique. Don't you find this skill attractive?",
                            "When you think you do, just come right back to me!"
                        ],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        ctx.lines_as("Leon", args!["Ooh! A young and strong swordsman!"])?;
        ctx.next()?;
        ctx.lines_as("Leon", args!["Wow, seeing your arm, you must enjoy using bash?"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Eh, I... just...")])?;
        ctx.var("@menu").set(choice)?;
        ctx.mes("[Leon]")?;
        if ctx.var("Sex").get()? == constants::SEX_FEMALE {
            ctx.lines(args![
                "No need to be surprised.",
                "If you use a sword, of course you ought to have a good arm!"
            ])?;
        } else {
            ctx.lines(args![
                "Nothing to be embarrassed about.",
                "Even if you are a female you need a strong arm to use a sword!"
            ])?;
        }
        ctx.lines(args![
            "In times of only useless and lazy youngsters,",
            "I'm glad I met someone strong like you."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Leon",
            args!["Yes, I would like to give a present to an awesome swordsman like you."],
        )?;
        ctx.next()?;
        'b2: {
            let subject2 = Val::from(runtime::select_values(ctx, &[Val::from("What present?:It's ok.")])?);
            let mut matched2 = false;
            let no_case2 = !subject2.loosely_equals(&Val::from(1)) && !subject2.loosely_equals(&Val::from(2));
            if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                matched2 = true;
            }
            if matched2 {
                ctx.lines_as("Leon", args!["Haha nothing special, but a skill to attack the vital point!"])?;
                ctx.next()?;
            }
            if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                matched2 = true;
            }
            if matched2 {
                ctx.lines_as(
                    "Leon",
                    args!["...Haha nothing special, just a skill that aims at the vital spot!"],
                )?;
                ctx.next()?;
            }
        }
        ctx.lines_as(
            "Leon",
            args![
                "It's a skill I developed recently.",
                "When you use bash, depending on",
                "your level, your opponent can",
                "become stunned.",
                "You have learned bash, haven't you?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Leon",
            args![
                "When I was a swordsman like you,",
                "I used to enjoy using Bash. Every time, I thought",
                "- maybe the attack would be more powerful",
                "if I use stun at the same time."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Leon",
            args![
                "I drew back from the battlefield to do research",
                "and finally, I developed this wonderful new skill!",
                "Would you like to learn this skill?"
            ],
        )?;
        ctx.next()?;
        match ctx.menu(&["Yes.", "No.", "Do you have any advice on how to eat sushi?"])? {
            0 => {
                ctx.lines_as("Leon", args!["Ok. I'll tell you the requirements.", "First you need to have level 5 Bash.", "You will also need to prepare 10 Fire Arrows, 10 Silver Arrows, 1 bottle of Banana Juice, 30 Tentacles, and 5 bottles of Royal Jelly.", "They are.. somewhat like ingredients."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Leon",
                    args!["come to me again once you have all the materials.", "We shall talk then."],
                )?;
                return ctx.close();
            }
            1 => {
                ctx.lines_as(
                    "Leon",
                    args!["Hahahahahahahahahaha!", "... ", " ... ", " ...", "I'm at a loss of words!?"],
                )?;
                return ctx.close();
            }
            2 => {
                ctx.lines_as(
                    "Sushi King Leon",
                    args![
                        "The best way to eat sushi is",
                        "with your hands.",
                        "That is the basic.",
                        "And dip the fish, not the rice,",
                        "in the soy sauce."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sushi King Leon",
                    args![
                        "That way you get a richer flavor.",
                        "Also, always eat the kind that is in season.",
                        "Eating in the order of white fish then",
                        "blue fish will make it taste better!",
                        "Mmm! I like sushi~~!"
                    ],
                )?;
                return ctx.close();
            }
            _ => {}
        }
    }
    ctx.lines_as(
        "Leon",
        args![
            "Oh, no! I have nothing to offer you!",
            "I can't say nice and fun things to anyone",
            "other than swordsmen!",
            "See you in a better world!"
        ],
    )?;
    ctx.close()
}

pub fn leon_von_frich_ontouch(ctx: &Ctx) -> Script {
    ctx.lines_as("Leon", args!["Hahahahahahaha!", "Hahahahahahaha!"])?;
    ctx.close()
}

pub fn juan(ctx: &Ctx) -> Script {
    if ctx.var("BaseClass").get()? == constants::JOB_SWORDMAN {
        if ctx.call(Function::GetSkillLv, args!["SM_AUTOBERSERK"])? == 1 {
            ctx.lines_as(
                "Juan",
                args![
                    "Mmm? Long time no see!",
                    "How are you?",
                    "You got stronger than before.",
                    "Many expect great things from you.",
                    "You can do it."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("JobLevel").get()?.number()? < 10
            && ctx.var("BaseJob").get()? != constants::JOB_KNIGHT
            && ctx.var("BaseJob").get()? != constants::JOB_CRUSADER
        {
            ctx.lines_as(
                "?",
                args![
                    "What are you?",
                    "Eh, still a beginner.",
                    "I'm busy, so go train a little more",
                    "before coming back."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("JobLevel").get()?.number()? < 30
            && ctx.var("BaseJob").get()? != constants::JOB_KNIGHT
            && ctx.var("BaseJob").get()? != constants::JOB_CRUSADER
        {
            ctx.lines_as("Juan", args!["Oh, nice to meet you.", "You can be on your way. (smiley~)"])?;
            return ctx.close();
        } else if ctx.items().count(924)? > 34 && ctx.items().count(958)? > 9 && ctx.items().count(957)? > 9 && ctx.items().count(518)? > 9
        {
            ctx.lines_as(
                "Juan",
                args![
                    "Ooh. Young swordsman!",
                    "You are ready to learn the",
                    "newest skill, Auto Berserk?!"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Hoho, I would like to learn it now.", "What is that?"])? {
                0 => {
                    ctx.lines_as("Juan", args!["Ok. Then..."])?;
                    ctx.next()?;
                    ctx.items().take(924, 35)?;
                    ctx.items().take(958, 10)?;
                    ctx.items().take(957, 10)?;
                    ctx.items().take(518, 10)?;
                    ctx.call(Function::Skill, args!["SM_AUTOBERSERK", 1, constants::SKILL_PERM])?;
                    ctx.lines_as(
                        "Juan",
                        args![
                            "You have just become a swordsman",
                            "that can use Auto Berserk.",
                            "You can go about ",
                            "and achieve great things!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Juan", args!["Good luck!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Juan",
                        args![
                            "....................................oh yeah.",
                            "I forgot to say something.",
                            "There are some things you must keep in mind."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Juan",
                        args![
                            "Once you regain health,",
                            "this skill will subside.",
                            "Also, there isn't really a time limit",
                            "but it can still disappear when",
                            "it is attacked with a skill that can",
                            "nullify provoke."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Juan",
                        args![
                            "If you don't remember these characteristics,",
                            "you may run into some problems on the battlefield",
                            "when the skill disappears all of a sudden."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Juan", args!["Then... bye for real~"])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Juan",
                        args![
                            "Auto Berserk?",
                            "It's a skill crucial on the battlefield.",
                            "When your health is in red,",
                            "your hidden potential provokes yourself",
                            "to help you in battle."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Juan",
                        args![
                            "It is perfect for those that",
                            "fight on the battlefield like fire!",
                            "With your ability, you can learn",
                            "this skill right now.",
                            "Then, I shall tell you the necessary materials."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Juan",
                        args![
                            "You need 35 Powder of Butterfly.",
                            "The energy from the magnificent",
                            "wings of a butterfly will",
                            "help you gather your strength!",
                            "And 10 Horrendous Mouth.",
                            "10 Decayed Nail.",
                            "and last but not least...",
                            "10 Honey!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Juan",
                        args![
                            "Did you get all that down?",
                            "As always, please come back",
                            "when you are ready.",
                            "I look forward to seeing you again."
                        ],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        ctx.lines_as(
            "Juan",
            args![
                "Oh no, you have more injuries",
                "since the last time I saw you.",
                "You went into battle like this?",
                "Seems like you are straining yourself."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Juan",
            args![
                "Even though you may have a lot of strength",
                "you can't do much when you reach your limits so",
                "don't overestimate your powers.",
                "Of course you could always use the",
                "skill we developed to overcome these limits."
            ],
        )?;
        ctx.next()?;
        match ctx.menu(&[
            "Eh! What are you talking about?",
            "Haha, there can't be such a thing.",
            "Keuuuuuuuh!",
        ])? {
            0 => {
                ctx.lines_as(
                    "Juan",
                    args![
                        "The skill is called Berserk.",
                        "It is deemed the flower of a battlefield.",
                        "When your health is red,",
                        "your hidden potential is provokes yourself",
                        "to help you in battle."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Juan",
                    args![
                        "It is perfect for those that",
                        "fight on the battlefield like fire!",
                        "With your ability, you can learn",
                        "this skill right now.",
                        "Then, I shall tell you the necessary materials."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Juan",
                    args![
                        "You need 35 Powder of Butterfly.",
                        "The energy from the magnificent",
                        "wings of a butterfly will",
                        "help you gather your strength!",
                        "And 10 Horrendous Mouth.",
                        "10 Decayed Nail.",
                        "and last but not least...",
                        "10 Honeys!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Juan",
                    args![
                        "Did you get all that down?",
                        "As always, please come back",
                        "when you are ready.",
                        "I look forward to seeing you again."
                    ],
                )?;
                return ctx.close();
            }
            1 => {
                ctx.lines_as(
                    "Juan",
                    args!["Bleh, were you fooled all your life.", "I don't know. Don't talk to me."],
                )?;
                return ctx.close();
            }
            2 => {
                ctx.lines_as(
                    "Juan",
                    args!["Keuuuuuuuuuuuuuh!", "Ooowwwwwuuuuuuuuuuuuuhhh!", "Keuaaaaaaaaaaah!"],
                )?;
                return ctx.close();
            }
            _ => {}
        }
    }
    ctx.lines_as(
        "Juan",
        args![
            "Are you enjoying your trip?",
            "I hope you have nice days ahead of you.",
            "Ah, I am just a kind knight Juan.",
            "Don't worry about me too much. Hahaha..."
        ],
    )?;
    ctx.close()
}
