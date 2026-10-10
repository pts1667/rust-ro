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

pub fn alcouskou(ctx: &Ctx) -> Script {
    if ctx.var("BaseClass").get()? == constants::JOB_THIEF {
        ctx.lines_as(
            "Alcouskou",
            args![
                "As you live life you will encounter",
                "many things. Sometimes you will",
                "not understand and wonder why some",
                "things are so important. You may consider it",
                "as useless knowledge, but it isn't",
                "Let me explain."
            ],
        )?;
        ctx.next()?;
        let subject1 = ctx.menu(&["Sand Attack", "Back Slide", "Find Stone", "Stone Fling", "I will be back later."])?;
        let mut matched1 = false;
        if !matched1 && subject1 == 0 {
            matched1 = true;
        }
        if matched1 {
            match ctx.var("skill_thief_1").get()?.number()? {
                0 => {
                    if (ctx.items().count(7041)? > 4
                        && (ctx.player().job_level()? > 24
                            || (ctx.var("BaseJob").get()? == constants::JOB_ASSASSIN || ctx.var("BaseJob").get()? == constants::JOB_ROGUE)))
                    {
                        ctx.lines_as(
                            "Alcouskou",
                            args![
                                "Luckily, you have brought some",
                                "sand with you. It is very important",
                                "to a thief to have a small quantity at",
                                "all times. Most people do not",
                                "realize the value of such a common",
                                "substance."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Alcouskou",
                            args![
                                "In case you meet a powerful",
                                "monster in a dungeon with no",
                                "sand, you could use this sand to",
                                "blind the monster and flee.",
                                "You should have a special sand pocket."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Alcouskou",
                            args![
                                "What? You don't expect me to",
                                "do that for you as well do you?",
                                "You must be very lazy! ! !",
                                "Very well, find the one named",
                                "RuRumuni. He will make you a",
                                "sturdy leather pocket."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Alcouskou",
                            args![
                                "I will be preparing for your",
                                "return. Find RuRumuni in",
                                "west Payon. That is where I",
                                "heard he is these days."
                            ],
                        )?;
                        ctx.items().take(7041, 5)?;
                        ctx.var("skill_thief_1").set(Val::from(1))?;
                        return ctx.close();
                    }
                    ctx.lines_as(
                        "Alcouskou",
                        args![
                            "The most important part",
                            "of being a good thief is stealth.",
                            "A thief should never be seen or",
                            "touched unless he wants to.",
                            "Some consider this cowardly",
                            "but I think differently."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Alcouskou",
                        args![
                            "The way I see it, we live in a",
                            "world where survival of the fittest",
                            "rules our lives. ",
                            "They may think less of me for use",
                            "this special skill. . . What is this skill?",
                            "This is the sand blinding skill."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Alcouskou",
                        args![
                            "If you can throw or kick sand",
                            "in the eyes of your opponent,",
                            "not only does their defense decrease,",
                            "but their ability to attack is impaired.",
                            "It is so effective, you may even stun them."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Alcouskou",
                        args![
                            "Well, we have to survive too.",
                            "Its either us or them. . .",
                            "I think it is important and vital",
                            "that we prepare a little sand.",
                            "What do you think? If you like it ",
                            "go and get five Fine Grit."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Alcouskou",
                        args![
                            "I am sure you eager to learn this",
                            "skill, but you must first gather five Fine Grit",
                            "Until you have gathered them,",
                            "I cannot teach you this skill.",
                            "Don't be disappointed, hurry and gather them."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Alcouskou",
                        args![
                            "Oh, I almost forgot! ! !",
                            "If you want to learn this ",
                            "special skill, be sure that",
                            "you are sufficiently experienced",
                            "to use this properly. This means",
                            "you should be at least job level 25."
                        ],
                    )?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Alcouskou",
                        args![
                            "Go to west payon and find",
                            "RuRumuni. He will make you",
                            "fine durable pouch for your",
                            "sand."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Alcouskou",
                        args!["I will take these five Fine Grit", "and prepare them for you while I wait."],
                    )?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Alcouskou",
                        args![
                            "Okay! Great !!",
                            "What a fine pouch indeed!",
                            "Small and easy to carry, with enough",
                            "capacity for enough sand. ",
                            "This is a perfect ^3355FFLeather Bag of Infinity^000000 !!",
                            "for you to use with this skill."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Alcouskou",
                        args!["Well let's see what your skill", "is like -", "Try it out ! !"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Alcouskou",
                        args!["AHhh, watch out for my eyes !!", "^5533FF- *throwing sand* -^000000"],
                    )?;
                    ctx.next()?;
                    ctx.mes("^5533FF- *tossing sand* -^000000")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Alcouskou",
                        args![
                            "Hoo Hoo Hoo... You are a",
                            "natural! ! ! Excellent !",
                            "I guess I have nothing more",
                            "that I can teach you.",
                            "I hope that this skill will",
                            "aid you in the future. -"
                        ],
                    )?;
                    ctx.items().take(7042, 1)?;
                    ctx.call(Function::Skill, args!["TF_SPRINKLESAND", 1, constants::SKILL_PERM])?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        if !matched1 && subject1 == 1 {
            matched1 = true;
        }
        if matched1 {
            if (ctx.items().count(940)? > 19
                && (ctx.player().job_level()? > 34
                    || (ctx.var("BaseJob").get()? == constants::JOB_ASSASSIN || ctx.var("BaseJob").get()? == constants::JOB_ROGUE)))
            {
                ctx.lines_as("Alcouskou", args!["Okay! Let's practice!"])?;
                ctx.next()?;
                ctx.lines_as("Alcouskou", args!["Suuu Suuu uk -"])?;
                ctx.next()?;
                ctx.lines_as("Alcouskou", args!["Suuuuk - -"])?;
                ctx.next()?;
                ctx.lines_as("Alcouskou", args!["Suk - Suuuk - - -"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Alcouskou",
                    args!["Great! -At this level,", "I am sure you can increase", "your skill on your own."],
                )?;
                ctx.items().take(940, 20)?;
                ctx.call(Function::Skill, args!["TF_BACKSLIDING", 1, constants::SKILL_PERM])?;
                return ctx.close();
            }
            ctx.lines_as(
                "Alcouskou",
                args![
                    "Usually we like to think about",
                    "attacking and damage, but",
                    "it is important to realize that",
                    "fleeing is just as important",
                    "as attacking!",
                    "We pride ourselves in our"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "speed and quick dodges,",
                    "but I am sure that at times you",
                    "have realized while fighting that",
                    "despite the fact that our dodging",
                    "is superior, if we are hit once we are",
                    "serious danger."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "It is true that at times we",
                    "can view others as humorous as",
                    "we easily dodge their attacks.",
                    "But if we are attacked by many at",
                    "once, you must remember that we",
                    "may not even have room to doge."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "You must make a quick decision",
                    "to flee. Most would flee immediately,",
                    "but we don't need to.",
                    "Even if we don't see an opening,",
                    "our skill can allow us to slip out",
                    "of a very serious predicament."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "This skill uses our energies in",
                    "allowing us to slip out unnoticed.",
                    "In a short amount of time we can use",
                    "this skill to put a large amount of ",
                    "distance between us and our opponent.",
                    "This skill requires endless hours of"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "practice for us to master.",
                    "If you wish to learn and practice,",
                    "you will need to prepare some items.",
                    "Prepare ^3355FF20 Grasshopper's Leg^",
                    "to begin your training."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "Oh, by the way. . .",
                    "You need to have some background",
                    "in the skills of a thief to properly master",
                    "this skill. This means you require at",
                    "least the experience of job level ^3355FF35^000000 .",
                    "If not, I cannot teach you."
                ],
            )?;
            return ctx.close();
        }
        if !matched1 && subject1 == 2 {
            matched1 = true;
        }
        if matched1 {
            if (((ctx.items().count(912)? > 0 && ctx.items().count(948)? > 0) && ctx.items().count(908)? > 4)
                && (ctx.player().job_level()? > 19
                    || (ctx.var("BaseJob").get()? == constants::JOB_ASSASSIN || ctx.var("BaseJob").get()? == constants::JOB_ROGUE)))
            {
                ctx.lines_as(
                    "Alcouskou",
                    args![
                        "Wow, you have already prepared?",
                        "Great, I see promise in you. -",
                        "Your zeal is truly sincere.",
                        "Okay, Shall we begin your training?",
                        "Stone throwing . . . . .",
                        "Find a smoot stone with a good weight."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Alcouskou",
                    args![
                        "Picking the right stone is ",
                        "very important in being successful.",
                        "Well I could tell you a million",
                        "times but it is better to see for yourself.",
                        "Okay why don't you try the skill out",
                        "right here where I can watch you."
                    ],
                )?;
                ctx.next()?;
                ctx.mes("^3355FF- Shweeput ! -^000000")?;
                ctx.next()?;
                ctx.mes("^3355FF- Cheeeguk! -^000000")?;
                ctx.next()?;
                ctx.mes("^3355FF- Shyaaaakkk ! -^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "Alcouskou",
                    args![
                        "Very nice. You seem to take up -",
                        "the skill easily.",
                        "You can improve your skill with",
                        "practice on your own time.",
                        "I hope it aids you in the future.",
                        ". . . . . Hope to see you soon"
                    ],
                )?;
                ctx.items().take(912, 1)?;
                ctx.items().take(948, 1)?;
                ctx.items().take(908, 5)?;
                ctx.call(Function::Skill, args!["TF_PICKSTONE", 1, constants::SKILL_PERM])?;
                return ctx.close();
            }
            ctx.lines_as(
                "Alcouskou",
                args![
                    "The skilled and experienced",
                    "members of our guild are usually very handy!",
                    "They can pick up a small stone",
                    "by the road and use it to hit an",
                    "opponent accurately and quickly from",
                    "a distance."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "They realized what a waste it was",
                    "to not teach this skill to others. -",
                    "They founded a group to train",
                    "others in this skill. . .",
                    "That is how the ^3355FF' Find Stone '^000000 and",
                    "^3355FF' Stone Fling '^000000 skills came to be."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "It is handy to be able to find a stone",
                    "in most any place and have the",
                    "ability to hurl it into a distanced enemy.",
                    "A very valuable skill indeed. -",
                    "Without costing you a zeny, you",
                    "can have this skill at your disposal."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "I really think of this as a great skill.",
                    "What do you think of it?",
                    ". . . . . Hah Hah Hah . . . . . ."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "^3355FF' Find Stone ! '^000000 skill",
                    "can be used in just about any location.",
                    "Picking stones off the ground that",
                    "are smooth and well weighted for",
                    "throwing. It does take some familiarity",
                    "and skill to learn this skill well."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "The small amount of training required",
                    "does require that you put in a full",
                    "effort . . .-",
                    ". . . . . *Ahem* . . . . .",
                    "Let first begin by practicing how to pick up",
                    "one ^3355FFBear's Footskin^000000 to familiarize the action."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "And in lieu of a stone, try picking up",
                    " a ^3355FFZargon^000000 !",
                    "Would that be too little ?",
                    "Lets add ^3355FF5 Spawn^000000 !!",
                    "Show me your skill !",
                    "Retrieve these items by any means you see fit."
                ],
            )?;
            return ctx.close();
        }
        if !matched1 && subject1 == 3 {
            matched1 = true;
        }
        if matched1 {
            if ((ctx.items().count(910)? > 1 && ctx.items().count(911)? > 1)
                && (ctx.player().job_level()? > 14
                    || (ctx.var("BaseJob").get()? == constants::JOB_ASSASSIN || ctx.var("BaseJob").get()? == constants::JOB_ROGUE)))
            {
                ctx.lines_as(
                    "Alcouskou",
                    args![
                        "Wow! Have you already gathered the items!",
                        "Very well, do you wish to begin?",
                        "Prepare the items you have gathered .."
                    ],
                )?;
                ctx.next()?;
                ctx.mes("^3355FF- Shyuuk ! -^000000")?;
                ctx.next()?;
                ctx.mes("^3355FF- Shyuuuk Tuk. . -^000000")?;
                ctx.next()?;
                ctx.mes("^3355FF- Shyupattt !! - Tauk !! -^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "Alcouskou",
                    args![
                        "Strike~~~!!",
                        "That was excellent !",
                        "You know have sufficient",
                        "power and skill."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Alcouskou",
                    args![
                        "Okay, that is all there is",
                        "to it. How you use it in",
                        "the future is up to you ..",
                        "I wish you luck!"
                    ],
                )?;
                ctx.items().take(910, 2)?;
                ctx.items().take(911, 2)?;
                ctx.call(Function::Skill, args!["TF_THROWSTONE", 1, constants::SKILL_PERM])?;
                return ctx.close();
            }
            ctx.lines_as(
                "Alcouskou",
                args![
                    "The skilled and experienced",
                    "members of our guild are usually very handy!",
                    "They can pick up a small stone",
                    "by the road and use it to hit an",
                    "opponent accurately and quickly from",
                    "a distance."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "They realized what a waste it was",
                    "to not teach this skill to others. -",
                    "They founded a group to train",
                    "others in this skill. . .",
                    "That is how the ^3355FF' Find Stone '^000000 and",
                    "^3355FF' Stone Fling '^000000 skills came to be."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "It is handy to be able to find a stone",
                    "in most any place and have the",
                    "ability to hurl it into a distanced enemy.",
                    "A very valuable skill indeed. -",
                    "Without costing you a zeny, you",
                    "can have this skill at your disposal."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "I really think of this as a great skill.",
                    "What do you think of it?",
                    ". . . . . Hah Hah Hah . . . . . ."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "^3355FFStone Fling !!^000000",
                    "This skill can be used so",
                    "readily and does not take any",
                    "extra money if you can pick up",
                    "stones well . . .",
                    "If you don't know how to choose"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "stones well, you can always buy",
                    "them from those who can.",
                    "And if you can choose stones well,",
                    "it could be very profitable for you.",
                    "What do you think? Do you like the idea?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "If you wish to master this skill,",
                    "you will have to train quite a bit.",
                    "The training also requires some items",
                    "that won't be easy to find. . .",
                    "Of course, I am sure it won't be",
                    "impossible with your skill . ."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alcouskou",
                args![
                    "You will need two ^3355FF' Garlet '^000000",
                    "and two ^3355FF' Scell '^000000 to start with.",
                    "Make sure they are similar to stones",
                    "and able to be thrown. When you",
                    "have gathered these items, I will be",
                    "happy to teach you."
                ],
            )?;
            return ctx.close();
        }
        if !matched1 && subject1 == 4 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Alcouskou",
                args![
                    "It seems you are not very experienced. . .",
                    "We may need some more time to consider you."
                ],
            )?;
            return ctx.close();
        }
    }
    ctx.lines_as(
        "Alcouskou",
        args![
            "Most thieves and assassins",
            "have the basic skills to do",
            "well at their job. However,",
            "the skills that I can teach them",
            "cannot be learned anywhere else.",
            "If you every decide to"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Alcouskou",
        args![
            "become a thief or assassin,",
            "or know someone who is,",
            "come to me or send them to me.",
            "These new skills should be",
            "taught to all who want to learn them."
        ],
    )?;
    return ctx.close();
}

pub fn bag_seller(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "RuRumuni",
        args![
            "I am a humble merchant here",
            "in Payon. I buy the leather",
            "hides of animals brought in by",
            "the hunters and make leather",
            "pouches to sell. I grew up",
            "around leather working and am quite good at it."
        ],
    )?;
    ctx.next()?;
    match ctx.var("skill_thief_1").get()?.number()? {
        0 => {
            ctx.lines_as(
                "RuRumuni",
                args![
                    "There is a thief guild in the",
                    "area of Morocc. I know one there",
                    "that sends me thieves in need",
                    "of items I make such as a",
                    "^3355FF' Leather Bag of Infinity '^000000.",
                    "They visit my store often."
                ],
            )?;
            return ctx.close();
        }
        1 => {
            if ((ctx.items().count(952)? > 0 && ctx.items().count(1055)? > 0) && ctx.items().count(1025)? > 0) {
                ctx.lines_as(
                    "RuRumuni",
                    args![
                        "Ahhh... You have come for a leather bag.",
                        "Very good, very good.",
                        "I will make you the leather bag",
                        "right away if you wait just a bit.",
                        "TuTak TuTak Shyuku Shyuku Shyuku",
                        "- - - - -"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "RuRumuni",
                    args![
                        "Okay, here it is all done.",
                        "Take this leather bag",
                        "to the Thief guild's Alcouskou",
                        "If you take him this, he will",
                        "teach you the skill that you ",
                        "wish to learn."
                    ],
                )?;
                ctx.items().take(952, 1)?;
                ctx.items().take(1055, 1)?;
                ctx.items().take(1025, 1)?;
                ctx.var("skill_thief_1").set(Val::from(2))?;
                ctx.items().give(7042, 1)?;
                return ctx.close();
            }
            ctx.lines_as(
                "RuRumuni",
                args![
                    "Find Alcouskou of the thief guild",
                    "to place and order for a leather bag of infinity.",
                    "You should know that the leather bag of infinity",
                    "takes much time and skill to make.",
                    "In order to make such an item, you",
                    "must provide me with the materials."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "The items needed as materials are",
                "these items. ..",
                "^3355FF' Earthworm Peeling '^000000",
                "^3355FF' Cobweb '^000000",
                "^3355FF' Cactus Needle '^000000",
                "I need one of each."
            ])?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "RuRumuni",
                args![
                    "Here is your leather bag of infinity.",
                    "Take this to the Thief guilds",
                    "Alcouskou and let him know ",
                    "that you are now ready to learn",
                    "the skill."
                ],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}
