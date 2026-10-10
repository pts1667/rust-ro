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

pub fn thief_trainer_t(ctx: &Ctx) -> Script {
    ctx.mes("[Yierhan]")?;
    if ctx.var("Class").get()? == constants::JOB_NOVICE {
        ctx.lines(args![
            "Eh...?",
            "A Novice?",
            "Still thinking",
            "what job you're",
            "gonna choose...",
            "Am I right?"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Yierhan",
            args![
                "Listen, if you ever decide to become a Thief--a smart choice",
                "I might add--come and talk to me. I'll show you the ropes!"
            ],
        )?;
        return ctx.close();
    } else if ctx.var("BaseClass").get()? == constants::JOB_THIEF && ctx.var("Upper").get()? != 2 {
        if ctx.var("Class").get()? != constants::JOB_THIEF && ctx.var("tu_thief01").get()?.number()? < 8 {
            ctx.lines(args![
                "Whaaaat are you",
                "doin' here? There's",
                "nothing I can teach you!",
                "You're waaay beyond me!"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Yierhan",
                args![
                    "In fact, I think",
                    "you're qualified",
                    "to teach me some stuff!",
                    "Come on! I need new moves!"
                ],
            )?;
            return ctx.close();
        }
        if ctx.var("tu_thief01").get()? == 0 {
            ctx.lines(args![
                "Heya pal.",
                "I'm Yierhan.",
                "I happen to be",
                "the guy in charge",
                "of training new Thieves."
            ])?;
            ctx.next()?;
            match ctx.menu(&["Training?", "Training? Right now?"])? {
                0 => {
                    ctx.lines_as("Yierhan", args!["Yeah, training. I mean, this kind of stuff is second nature to some people, but other guys need a little more help. So this is one of those 'just in case' things."])?;
                    ctx.next()?;
                }
                1 => {
                    ctx.lines_as(
                        "Yierhan",
                        args![
                            "Right now?",
                            "Yeah, right now!",
                            "But if you're not ready for some reason, I guess I can wait."
                        ],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
            ctx.lines_as("Yierhan", args!["Alright, first of all, Thieves use melee attacks. Well, most of us do. There are a few who like using long range Bows. But all of us are good at bein' fast!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Yierhan",
                args![
                    "So for melee attacks, which stat increases your damage? Come on",
                    "now, you should know this if you didn't skip the Novice Training Grounds."
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["^6B8E23INT^000000", "^2F4F2FSTR^000000", "^23238EDEX^000000"])? {
                0 => {
                    ctx.lines_as("Yierhan", args!["Say whaaat? ^6B8E23INT^000000 affects magic damage, magic defense and some skills. Thieves don't even work with magic!"])?;
                    ctx.next()?;
                    ctx.lines_as("Yierhan", args!["You musta skipped the Novice Training Grounds altogether! Not that I blame you though. Anyway, it's ^2F4F2FSTR^000000 that increases your attack damage, got it?"])?;
                    ctx.next()?;
                    ctx.lines_as("Yierhan", args!["So if you're feeling like the damage you're making is pretty weak, you might want more stat points in ^2F4F2FSTR^000000. How high your raise your own STR is really up to you."])?;
                    ctx.call(Function::GetExperience, args![200, 100])?;
                }
                1 => {
                    ctx.lines_as(
                        "Yierhan",
                        args![
                            "Yeah, that's right! If you wanna increase your damage, you need",
                            "to put some stat points into ^2F4F2FSTR^000000. Increasing STR also increases",
                            "your Max Weight Limit too."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Yierhan", args!["So if you're feeling like the damage you're making is pretty weak, you might want more stat points in ^2F4F2FSTR^000000. How high your raise your own STR is really up to you."])?;
                    ctx.next()?;
                    ctx.lines_as("Yierhan", args!["Sure, ^23238EDEX^000000 and LUK can", "also increase your attack damage, but they're insignificant compared to STR. I repeat: ^660000insignificant^000000."])?;
                    ctx.call(Function::GetExperience, args![400, 200])?;
                }
                2 => {
                    ctx.lines_as(
                        "Yierhan",
                        args![
                            "Say whaaat? ^23238EDEX^000000 affects",
                            "your attack accuracy, not your damage! Well, unless you're using",
                            "a Bow. Otherwise, it increases your damage only by a tiny bit."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Yierhan", args!["You musta skipped the Novice Training Grounds altogether! Not that I blame you though. Anyway, it's ^2F4F2FSTR^000000 that increases your attack damage, got it?"])?;
                    ctx.next()?;
                    ctx.lines_as("Yierhan", args!["So if you're feeling like the damage you're making is pretty weak, you might want more stat points in ^2F4F2FSTR^000000. How high your raise your own STR is really up to you."])?;
                    ctx.call(Function::GetExperience, args![200, 100])?;
                }
                _ => {}
            }
            ctx.var("tu_thief01").set(1)?;
            ctx.call(Function::SpecialEffect, args![constants::EF_HIT5])?;
            return ctx.close();
        } else if ctx.var("tu_thief01").get()? == 1 {
            ctx.lines(args![
                "Alright, enough about stats.",
                "You know what? I think I'll just talk to you about the skills that we Thieves use."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Yierhan",
                args![
                    "So level up your skills, learn a few new ones if you want, and",
                    "then come back over here."
                ],
            )?;
            ctx.var("tu_thief01").set(2)?;
            return ctx.close();
        } else if ctx.var("tu_thief01").get()? == 2 {
            ctx.mes("Okay, let me see your skills. You know you gotta change your battle strategy depending on what skills you have, right? Skills are just as important as stats!")?;
            ctx.next()?;
            let double_lv = ctx.call(Function::GetSkillLv, args!["TF_DOUBLE"])?;
            let miss_lv = ctx.call(Function::GetSkillLv, args!["TF_MISS"])?;
            let steal_lv = ctx.call(Function::GetSkillLv, args!["TF_STEAL"])?;
            let hiding_lv = ctx.call(Function::GetSkillLv, args!["TF_HIDING"])?;
            let poison_lv = ctx.call(Function::GetSkillLv, args!["TF_POISON"])?;
            let detoxify_lv = ctx.call(Function::GetSkillLv, args!["TF_DETOXIFY"])?;
            if double_lv == 0 && miss_lv == 0 && steal_lv == 0 && hiding_lv == 0 && poison_lv == 0 && detoxify_lv == 0 {
                ctx.lines_as("Yierhan", args!["You haven't learned any skills yet? Come on, it's your skills that'll set you apart from Novices and everyone else!"])?;
                return ctx.close();
            }
            if double_lv.number()? > 0 {
                ctx.lines_as(
                    "Yierhan",
                    args![
                        "Ah, so you've learned",
                        Val::from("Level ") + double_lv.clone() + Val::from(" Double Attack."),
                        "Nice! This skill gives you the chance to attack twice in one",
                        "attack. Wicked!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Yierhan", args!["It's a Passive skill, so it's always in effect and won't have to use any SP to use it. The higher your Double Attack skill level, the more double attacks you'll do."])?;
                ctx.next()?;
            }
            if miss_lv.number()? > 0 {
                ctx.lines_as(
                    "Yierhan",
                    args![
                        "Let's see...",
                        Val::from("Level ") + miss_lv.clone() + Val::from(" Increase Dodge?"),
                        "That increases your Flee Rate, meaning you've got a better chance of dodging attacks from your enemies."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Yierhan",
                    args![
                        "Just like the Double Attack skill, Increase Dodge is a Passive skill. It won't use SP and it's always in",
                        "effect. If you don't like to bruise, this is your skill."
                    ],
                )?;
                ctx.next()?;
            }
            if steal_lv.number()? > 0 {
                ctx.lines_as(
                    "Yierhan",
                    args![
                        "Whoa, so you've",
                        Val::from("got Level ") + steal_lv.clone() + Val::from(" Steal~"),
                        "Now that's the skill which gives our job its name! You can't use",
                        "it against people, though..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Yierhan", args!["But you can use Steal to take items from monsters. If you're lucky, you can get some good items that way. Oh, and Steal doesn't affect monster drop rates."])?;
                ctx.next()?;
            }
            if hiding_lv.number()? > 0 {
                ctx.lines_as(
                    "Yierhan",
                    args![
                        "You've learned",
                        Val::from("Level ") + hiding_lv.clone() + Val::from(" Hiding?"),
                        "Let's see, you can only learn",
                        "that after learning the Steal skill up to a certain level."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Yierhan", args!["Of course, you use ", "the Hiding skill to hide underground in an emergency, like when you're surrounded by tough enemies. Be careful though..."])?;
                ctx.next()?;
                ctx.lines_as("Yierhan", args!["Certain monsters will still be able to find you, no matter how well you hide. There are even a few monsters that can flush you out of hiding!"])?;
                ctx.next()?;
            }
            if poison_lv.number()? > 0 {
                ctx.lines_as(
                    "Yierhan",
                    args![
                        "Alright, I see that you",
                        Val::from("know Level ") + poison_lv.clone() + Val::from(" Envenom."),
                        "You like being dangerous,",
                        "don't you?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Yierhan", args!["This attack skill has the chance", "to poison your enemy for a set amount of time. While poisoned, an enemy will constantly lose its HP and will have decreased defense."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Yierhan",
                    args![
                        "Eh, but remember.",
                        "If the monster's too strong for you, you might not be able to poison it. So don't go crazy."
                    ],
                )?;
                ctx.next()?;
            }
            if detoxify_lv.number()? > 0 {
                ctx.lines_as(
                    "Yierhan",
                    args![
                        Val::from("Level ") + detoxify_lv.clone() + Val::from(" Detoxify."),
                        "If you took the trouble to learn that, you must be the cautious",
                        "type or something."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Yierhan",
                    args![
                        "You can only learn Detoxify",
                        "after you learn the Envenom skill. Detoxify allows you to counteract the effects of poison on a target."
                    ],
                )?;
                ctx.next()?;
            }
            ctx.lines_as(
                "Yierhan",
                args![
                    "Alright, I guess",
                    "if you want to know",
                    "about any other skills,",
                    "I can explain real quick."
                ],
            )?;
            ctx.var("tu_thief01").set(3)?;
            ctx.call(
                Function::GetExperience,
                args![ctx.player().base_level()? * 30, ctx.player().base_level()? * 15],
            )?;
            ctx.call(Function::SpecialEffect, args![constants::EF_HIT5])?;
            return ctx.close();
        } else if ctx.var("tu_thief01").get()? == 3 {
            ctx.lines(args![
                "So...",
                "Are there any",
                "skills you want",
                "explained or is this",
                "pretty much stuff you",
                "already know?"
            ])?;
            let mut read_double = false;
            let mut read_dodge = false;
            let mut read_steal = false;
            let mut read_hiding = false;
            let mut read_envenom = false;
            let mut read_detoxify = false;
            loop {
                ctx.next()?;
                match ctx.menu(&[
                    "Double Attack",
                    "Increase Dodge",
                    "Steal",
                    "Hiding",
                    "Envenom",
                    "Detoxify",
                    "I know enough.",
                ])? {
                    0 => {
                        ctx.lines_as(
                            "Yierhan",
                            args![
                                "Just like its name,",
                                "Double Attack gives your attacks the chance to be a double attack, two strikes in one blow."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Yierhan", args!["It's a Passive skill, so it's always in effect and won't have to use any SP to use it. The higher your Double Attack skill level, the more double attacks you'll do."])?;
                        read_double = true;
                    }
                    1 => {
                        ctx.lines_as(
                            "Yierhan",
                            args![
                                "Increase Dodge",
                                "gives a nice boost",
                                "to your Flee Rate that",
                                "the other jobs don't offer.",
                                "Why take your lumps when you",
                                "can avoid them altogether?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Yierhan",
                            args![
                                "Just like the Double Attack skill, Increase Dodge is a Passive skill. It won't use SP and it's always in",
                                "effect. If you don't like to bruise, this is your skill."
                            ],
                        )?;
                        read_dodge = true;
                    }
                    2 => {
                        ctx.lines_as(
                            "Yierhan",
                            args![
                                "Steal is an Active Skill that has the chance of nabbing you some",
                                "free items! You can't use it against other people, though."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::SpecialEffect, args![constants::EF_STEAL])?;
                        ctx.lines_as("Yierhan", args!["But you can use Steal to take items from monsters! If you're lucky, you can get some good items that way. Oh, and Steal doesn't affect monster drop rates."])?;
                        read_steal = true;
                    }
                    3 => {
                        ctx.lines_as("Yierhan", args!["Hiding is an active skill where you submerge yourself underground. You can only stay hidden so long, and you can't move, but sometimes it's better than being found!"])?;
                        ctx.next()?;
                        ctx.lines_as("Yierhan", args!["Of course, you use ", "the Hiding skill to hide underground in an emergency, like when you're surrounded by tough enemies. Be careful though..."])?;
                        ctx.next()?;
                        ctx.lines_as("Yierhan", args!["Certain monsters will still be able to find you, no matter how well you hide. There are even a few monsters that can flush you out of hiding!"])?;
                        read_hiding = true;
                    }
                    4 => {
                        ctx.lines_as(
                            "Yierhan",
                            args!["Envenom is an offensive Active Skill that every Thief should know. But that's just what I think."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Yierhan", args!["This attack skill has the chance", "to poison your enemy for a set amount of time. While poisoned, an enemy will constantly lose its HP and will have decreased defense.", "Remember that."])?;
                        read_envenom = true;
                    }
                    5 => {
                        ctx.lines_as(
                            "Yierhan",
                            args![
                                "Since Thieves deal",
                                "quite a bit with poison,",
                                "we've got to have a way",
                                "to, well, have a taste",
                                "of our own medicine."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Yierhan",
                            args![
                                "You can learn Detoxify",
                                "after you learn the Envenom skill. Detoxify allows you to counteract the effects of poison on a target."
                            ],
                        )?;
                        read_detoxify = true;
                        ctx.next()?;
                        ctx.call(Function::SpecialEffect, args![constants::EF_DETOXICATION])?;
                    }
                    6 => {
                        ctx.lines_as(
                            "Yierhan",
                            args![
                                "Yeah...",
                                "I'm tired of explaining these skills anyway. Let's move on to",
                                "the next part of Thief training..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Yierhan",
                            args!["Now that you're such an expert on skills, I want you to level up your skills and come back, got it?"],
                        )?;
                        ctx.var("tu_thief01").set(4)?;
                        ctx.call(Function::SpecialEffect, args![constants::EF_HIT5])?;
                        let read_count = [read_double, read_dodge, read_steal, read_hiding, read_envenom, read_detoxify]
                            .iter()
                            .filter(|&&read| read)
                            .count() as i32;
                        if read_count > 0 {
                            ctx.call(Function::GetExperience, args![read_count * 300, read_count * 100])?;
                        }
                        return ctx.close();
                    }
                    _ => {}
                }
            }
        } else if ctx.var("tu_thief01").get()? == 4 {
            ctx.mes("Alright, we studied the skills and you've been practicing a little, right? You better have...")?;
            ctx.next()?;
            ctx.lines_as("Yierhan", args!["Alright, now I got some actual fight training I want you to do. Here's a chance for you to figure what skills are best for which situations."])?;
            ctx.next()?;
            ctx.lines_as("Yierhan", args!["Go and get me", "10 ^ff0000Feather of Birds^000000.", "You can go ahead and kill Pickies to get those. It really shouldn't be that hard. Oh, and use this Wing thingee to come back."])?;
            ctx.var("tu_thief01").set(5)?;
            ctx.call(Function::SavePoint, args!["moc_ruins", 80, 164, 1, 1])?;
            ctx.items().give(602, 1)?;
            ctx.call(Function::GetExperience, args![100, 50])?;
            ctx.call(Function::SpecialEffect, args![constants::EF_HIT5])?;
            ctx.close_window()?;
            ctx.warp("moc_fild12", 158, 373)?;
            return ctx.end();
        } else if ctx.var("tu_thief01").get()? == 5 || ctx.var("tu_thief01").get()? == 6 {
            if ctx.items().count(916)? < 10 {
                ctx.lines(args![
                    "'Ey, you don't have the 10 ^ff0000Feather of Birds^000000 I asked you for! You gotta apply what you know, you know.",
                    "Now hurry up and do it!"
                ])?;
                ctx.close_window()?;
                ctx.warp("moc_fild07", 203, 38)?;
                return ctx.end();
            }
            ctx.lines(args![
                "Alright...!",
                "Nice work, pal.",
                "Seeing as you got these feathers, you must be really gung-ho about becoming a good Thief."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Yierhan",
                args![
                    "I hope you keep",
                    "putting in the work",
                    "to get better and better.",
                    "Always do your best! Oh,",
                    "and do you have any questions?"
                ],
            )?;
            ctx.next()?;
            if ctx.var("tu_thief01").get()? == 6 {
                match ctx.menu(&["About those traces...", "Nope."])? {
                    0 => {
                        ctx.lines_as(
                            "Yierhan",
                            args![
                                "You found out, eh?",
                                "Well, I didn't really",
                                "wanna tell you this, on",
                                "account of you bein' a brand",
                                "brand new Thief and all, but..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Yierhan",
                            args!["There was this", "fight that was in", "the Southern part", "of this town."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Yierhan", args!["Since it happened late at night, only a few people actually know about it. As for me, I stayed late at a guild meeting, so it was", "dumb luck that I saw it."])?;
                        ctx.next()?;
                        ctx.lines_as("Yierhan", args!["I went back to where the fight happened and I found traces that showed that the guys who were fighting went south."])?;
                        ctx.next()?;
                        ctx.lines_as("Yierhan", args!["Since poison was used in", "the fight, I'm guessing an Assassin was involved, but I can't be too sure. If you wanna check it out, follow the traces of that fight."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Yierhan",
                            args![
                                "Eh, but be careful",
                                "not to get too close",
                                "to the poison! That stuff",
                                "is pretty strong!"
                            ],
                        )?;
                        ctx.var("tu_thief01").set(8)?;
                        ctx.items().give(1207, 1)?;
                        ctx.call(Function::GetExperience, args![1000, 500])?;
                        ctx.call(Function::SpecialEffect, args![constants::EF_HIT5])?;
                        return ctx.close();
                    }
                    1 => {
                        ctx.lines_as("Yierhan", args!["Good!", "Less work for me!", "Alright, you better get stronger the next time I see you. Oh, and you can have this stuff. You know, since you're so gangster and all."])?;
                        ctx.var("tu_thief01").set(7)?;
                        ctx.items().give(1207, 1)?;
                        ctx.call(Function::GetExperience, args![500, 200])?;
                        ctx.call(Function::SpecialEffect, args![constants::EF_HIT5])?;
                        return ctx.close();
                    }
                    _ => {}
                }
            } else {
                match ctx.menu(&["It was nice to meet you.", "Nope."])? {
                    0 => {
                        ctx.lines_as(
                            "Yierhan",
                            args![
                                "Yeah, it was pretty cool just hanging out. Keep fighting",
                                "monsters the way you do and",
                                "you'll be a great Thief in no time."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Yierhan",
                            args![
                                "And since I like",
                                "you so much, kid,",
                                "you can have this.",
                                "Take it, it's yours!"
                            ],
                        )?;
                        ctx.call(Function::GetExperience, args![500, 200])?;
                    }
                    1 => {
                        ctx.lines_as("Yierhan", args!["Good!", "Less work for me!", "Alright, you better get stronger the next time I see you. Oh, and you can have this stuff. You know, since you're so gangster and all."])?;
                        ctx.call(Function::GetExperience, args![50, 20])?;
                    }
                    _ => {}
                }
                ctx.var("tu_thief01").set(7)?;
                ctx.items().give(1207, 1)?;
                ctx.call(Function::SpecialEffect, args![constants::EF_HIT5])?;
                return ctx.close();
            }
        } else if ctx.var("tu_thief01").get()? == 7 {
            ctx.lines_as(
                "Yierhan",
                args![
                    "You know...",
                    "There was this",
                    "fight that was in",
                    "the Southern part",
                    "of this town."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Yierhan", args!["Since it happened late at night, only a few people actually know about it. As for me, I stayed late at a guild meeting, so it was", "dumb luck that I saw it."])?;
            ctx.next()?;
            ctx.lines_as(
                "Yierhan",
                args!["I went back to where the fight happened and I found traces that showed that the guys who were fighting went south."],
            )?;
            ctx.next()?;
            ctx.lines_as("Yierhan", args!["Since poison was used in", "the fight, I'm guessing an Assassin was involved, but I can't be too sure. If you wanna check it out, follow the traces of that fight."])?;
            ctx.next()?;
            ctx.lines_as(
                "Yierhan",
                args![
                    "Eh, but be careful",
                    "not to get too close",
                    "to the poison! That stuff",
                    "is pretty strong!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Yierhan",
                args![
                    "Hey, you might run into",
                    "poison, so remember that",
                    "Green Herbs and Green Potions",
                    "will counteract it. Oh, and keep in mind that Red Gemstones can",
                    "be used in poison attacks."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Yierhan",
                args!["Hey, if you do", "decide to check it", "out, be real careful", "other there, okay?"],
            )?;
            ctx.var("tu_thief01").set(8)?;
            ctx.call(Function::GetExperience, args![200, 100])?;
            ctx.call(Function::SpecialEffect, args![constants::EF_HIT5])?;
            return ctx.close();
        } else if ctx.var("tu_thief01").get()? == 8 {
            ctx.lines(args!["Heya pal.", "You doin' alright?"])?;
            ctx.next()?;
            ctx.lines_as("Yierhan", args!["Fighting against something", "you know nothing about is always risky. Since I've heard there are outsiders around flaunting their power, you better be careful."])?;
            ctx.next()?;
            ctx.lines_as(
                "Yierhan",
                args![
                    "It's a good policy to just be really careful on your adventures. Look out for monsters and look",
                    "out for people! Got it?"
                ],
            )?;
            return ctx.close();
        } else if ctx.var("tu_thief01").get()?.number()? < 26 {
            ctx.lines(args![
                "I heard there was",
                "this one Assassin",
                "that went on a mission",
                "and never returned."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Yierhan",
                args![
                    "What's so weird about it was that the Assassin Guild reported that the mission was completed!",
                    "I remember hearing that guy",
                    "was really good..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Yierhan", args!["Crazy, isn't it?", "Anyway, take", "care of yourself."])?;
            return ctx.close();
        } else if ctx.var("tu_thief01").get()? == 26 {
            ctx.lines(args![
                "'Ey, did you",
                "complete your mission?",
                "I know, I know, the thing you've gotta do is pretty rough."
            ])?;
            ctx.next()?;
            match ctx.menu(&["I'm still investigating.", "Not yet.", "Yes, I did."])? {
                0 => {
                    ctx.lines_as(
                        "Yierhan",
                        args!["Ah, gotcha. Well, that's understandable. I mean, these things take time, you know?"],
                    )?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Yierhan",
                        args![
                            "Yeah...?",
                            "That's alright.",
                            "I guess these kinds of things require patience. And thinking.",
                            "You know, things I'm horrible at."
                        ],
                    )?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Yierhan",
                        args!["Alright...!", "So what's up?", "Hit me up with", "what you know~"],
                    )?;
                    ctx.next()?;
                    ctx.mes("^3355FFYou explain the results of your investigation to Yierhan and tell him about the scrap of cloth you found on your mission.^000000")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Yierhan",
                        args!["Scrap of cloth?", "Huh, alright. Say,", "lemme have a looksee."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Yierhan",
                        args!["Whoa!", "You did great.", "This is some pretty", "important information!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Yierhan", args!["You see, the pattern on this cloth is sort of like one of the codes Assassins use. And this particular pattern looks like something from one of those higher Assassins."])?;
                    ctx.next()?;
                    ctx.lines_as("Yierhan", args!["I might be able to figure out what happened that night with the new information this cloth might lead me to. Thanks a lot!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Yierhan",
                        args![
                            "From here on,",
                            "the higher ups in the",
                            "guild will take over this investigation. You did your job perfectly, so it's time for you to hone your skills."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Yierhan",
                        args!["Hey, this stuff is yours. Think of it as a reward for helping us out. Take care of yourself now~"],
                    )?;
                    ctx.var("tu_thief01").set(27)?;
                    ctx.items().give(2307, 1)?;
                    ctx.player().set_zeny(ctx.player().zeny()? + 5000)?;
                    ctx.call(Function::GetExperience, args![8000, 3000])?;
                    return ctx.close();
                }
                _ => {}
            }
        } else {
            ctx.lines_as(
                "Yierhan",
                args![
                    "Hey...",
                    "You got dreams,",
                    "don't you? I know,",
                    "it's a bit of a deep",
                    "subject I pulled outta",
                    "nowhere, but..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Yierhan", args!["I just like telling people to follow their dreams. So do it. Life without anything to look forward to is pretty boring, doncha think?"])?;
            return ctx.close();
        }
    } else if ctx.var("BaseClass").get()? == constants::JOB_MAGE && ctx.var("Upper").get()? != 2 {
        if ctx.var("tu_magician01").get()?.number()? < 7 {
            ctx.lines_as(
                "Yierhan",
                args![
                    "Heya.",
                    "I'm the Thief trainer around here. Sure, it looks like we don't got much in common, but..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Yierhan",
                args![
                    "I actually got a few magic using friends here and there. That mystic stuff is waaay over my head, but",
                    "I got a lotta respect for it."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("tu_magician01").get()? == 7 {
            ctx.lines(args![
                "Hm...?",
                "That's weird, usually only",
                "Thieves hang around this joint. What's someone like you doing",
                "here? Unless..."
            ])?;
            ctx.next()?;
            ctx.var("@menu")
                .set(runtime::select_values(ctx, &[Val::from("I'm here on behalf of 'Mana.'")])?)?;
            ctx.lines_as("Yierhan", args!["Right, you must be the help that Mana sent! You came just at the right time. You see, we found something weird in South Morocc."])?;
            ctx.next()?;
            ctx.lines_as("Yierhan", args!["Basically, we found traces of poison that were used in a fight. We were going to investigate it, but we've been swamped with all this other work."])?;
            ctx.next()?;
            ctx.lines_as(
                "Yierhan",
                args![
                    "That's why we've been asking",
                    "for help from the Mage Guild. Fortunately, I'm pals with Mana, so..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Yierhan", args!["Anyway, head over to", "South Morocc since it seems to be a good place to start investigating. You'll see what we found right outside the South Morocc gate."])?;
            ctx.var("tu_magician01").set(8)?;
            return ctx.close();
        } else if ctx.var("tu_magician01").get()?.number()? < 26 {
            ctx.lines(args![
                "I heard there was",
                "this one Assassin",
                "that went on a mission",
                "and never returned."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Yierhan",
                args![
                    "What's so weird about it was that the Assassin Guild reported that the mission was completed!",
                    "I remember hearing that guy",
                    "was really good..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Yierhan",
                args![
                    "Be careful, pal.",
                    "It seems your",
                    "investigation",
                    "might be related to",
                    "that mysterious Assassin..."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("tu_magician01").get()? == 26 {
            ctx.lines(args![
                "So how's the",
                "investigation",
                "coming along?",
                "I've been so busy,",
                "I couldn't focus",
                "on it at all..."
            ])?;
            ctx.next()?;
            match ctx.menu(&["I'm still investigating.", "I'm not done yet...", "Oh, I finished~"])? {
                0 => {
                    ctx.lines_as(
                        "Yierhan",
                        args!["Ah, gotcha. Well, that's understandable. I mean, these things take time, you know?"],
                    )?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Yierhan",
                        args![
                            "Yeah...?",
                            "That's alright.",
                            "I guess these kinds of things require patience. And thinking.",
                            "You know, things I'm horrible at."
                        ],
                    )?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Yierhan",
                        args!["Alright...!", "So what's up?", "Hit me up with", "what you know~"],
                    )?;
                    ctx.next()?;
                    ctx.mes("^3355FFYou explain the results of your investigation to Yierhan and tell him about the scrap of cloth you found on your mission.^000000")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Yierhan",
                        args!["Scrap of cloth?", "Huh, alright. Say,", "lemme have a looksee."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Yierhan",
                        args!["Whoa!", "You did great.", "This is some pretty", "important information!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Yierhan", args!["You see, the pattern on this cloth is sort of like one of the codes Assassins use. And this particular pattern looks like something from one of those higher Assassins."])?;
                    ctx.next()?;
                    ctx.lines_as("Yierhan", args!["I might be able to figure out what happened that night with the new information this cloth might lead me to. Thanks a lot!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Yierhan",
                        args![
                            "From here on,",
                            "the higher ups in the",
                            "guild will take over this investigation. You did your job perfectly, so it's time for you to hone your skills."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Yierhan", args!["Hey, this stuff is yours. Think of it as a reward for helping us out. Right, and I'll let Mana know you did a great job. Take care of yourself now~"])?;
                    ctx.var("tu_magician01").set(27)?;
                    ctx.player().set_zeny(ctx.player().zeny()? + 5000)?;
                    ctx.call(Function::GetExperience, args![5000, 2000])?;
                    return ctx.close();
                }
                _ => {}
            }
        } else {
            ctx.lines(args![
                "Just as I thought,",
                "more than one group",
                "was involved in all this.",
                "Huh. Something real bad",
                "might happen soon..."
            ])?;
            return ctx.close();
        }
    }
    ctx.lines(args![
        "Some people think",
        "the desert is just a",
        "dangerous, uncomfortable",
        "place where no one wants",
        "to be. But there's all",
        "sorts of great stuff here."
    ])?;
    ctx.next()?;
    ctx.lines_as(
        "Yierhan",
        args![
            "We got the blazing heat that encourages skimpy outfits, we got... Cacti. We got more sand than the beach. Um... Sandstorms?"
        ],
    )?;
    ctx.close()
}
