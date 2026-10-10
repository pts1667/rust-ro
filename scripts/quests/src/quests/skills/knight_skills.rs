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

pub fn knight_kabuto(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_KNIGHT {
        if ctx.var("kngt_sk").get()? == 10 {
            ctx.lines_as(
                "Essofeit",
                args![
                    "Ah, it must be grand to",
                    "be an adventuring Knight",
                    "in this world. You must have",
                    "encountered all sorts of",
                    "dangerous monsters and",
                    "fearsome enemies, right?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "I'm almost jealous of",
                    "all the great experiences",
                    "that you must be having.",
                    "Someday, you'll have to tell",
                    "me your own stories of bravery."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("kngt_sk").get()? == 9 && ctx.call(Function::GetSkillLv, args!["KN_CHARGEATK"])? == 0 {
            ctx.lines_as(
                "Essofeit",
                args![
                    "Ah, I see that you've",
                    "made tremendous progress",
                    "in your pursuit of strength.",
                    "Transcendance is no small",
                    "feat, and it is a great honor",
                    "to achieve Lord Knight rank."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "However, I'm aware of the",
                    "drawback of memory erasure.",
                    "Therefore, I assume you're",
                    "here to learn the Charge Attack",
                    "skill once more. It will be my",
                    "pleasure to instruct you again."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "Charge Attack is an active skill that consumes 40 SP to damage",
                    "one target. If you're further from the target, you'll increase the",
                    "skill's damage and the delay",
                    "before damage is inflicted."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "During this delay, you will",
                    "be immobilized. Therefore,",
                    "the cost of increased damage",
                    "is greater risk to yourself.",
                    "For now, it would be best to practice this skill on your own."
                ],
            )?;
            ctx.var("kngt_sk").set(Val::from(10))?;
            ctx.call(Function::Skill, args!["KN_CHARGEATK", 1, constants::SKILL_PERM])?;
            ctx.next()?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "Good luck on your",
                    "adventures, Lord Knight.",
                    "I'm sure you'll make good",
                    "use of the Charge Attack",
                    "and bring pride to the",
                    "Prontera Chivalry."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("kngt_sk").get()? == 9 {
            ctx.lines_as(
                "Essofeit",
                args![
                    "Ah, it must be grand to",
                    "be an adventuring Knight",
                    "in this world. You must have",
                    "encountered all sorts of",
                    "dangerous monsters and",
                    "fearsome enemies, right?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "I'm almost jealous of",
                    "all the great experiences",
                    "that you must be having.",
                    "Someday, you'll have to tell",
                    "me your own stories of bravery."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("kngt_sk").get()? == 8 {
            ctx.lines_as(
                "Essofeit",
                args![
                    "I've named this skill,",
                    "''Charge Attack.'' It's not",
                    "a fancy name, but it's simple",
                    "and direct enough for you to",
                    "understand how it works.",
                    "Now let me teach it to you..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "Charge Attack is an active skill that consumes 40 SP to damage",
                    "one target. If you're further from the target, you'll increase the",
                    "skill's damage and the delay",
                    "before damage is inflicted."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "During this delay, you will",
                    "be immobilized. Therefore,",
                    "the cost of increased damage",
                    "is greater risk to yourself.",
                    "For now, it would be best to practice this skill on your own."
                ],
            )?;
            ctx.var("kngt_sk").set(Val::from(9))?;
            ctx.call(Function::Skill, args!["KN_CHARGEATK", 1, constants::SKILL_PERM])?;
            ctx.next()?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "I hope you make good",
                    "use of the Charge Attack",
                    "skill. Just like you, I will be",
                    "doing my best to bring honor",
                    "to the Knighthood with my",
                    "strength and courage!"
                ],
            )?;
            return ctx.close();
        } else if ctx.var("kngt_sk").get()? == 7 && ctx.items().count(530)? > 4 && ctx.items().count(748)? > 2 {
            ctx.lines_as(
                "Essofeit",
                args![
                    "The concept of honor",
                    "seems to be lost on today's",
                    "Knights. No longer do they",
                    "appreciate the meaning of",
                    "the word ''chivalry'' or the",
                    "noble pursuit for strength..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "Back in my day, Knights",
                    "were strong enough to get",
                    "at least 5 Candy Canes and",
                    "3 Witherless Roses through",
                    "hunting alone! But Knights",
                    "these days have grown soft..."
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Give him Candy Canes and Witherless Roses", "Cancel"])? == 0 {
                ctx.lines_as(
                    ctx.player().name()?,
                    args![
                        "There are still Knights",
                        "out there who believe in",
                        "honorably risking our lives",
                        "to achieve true strength..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Essofeit",
                    args![
                        "You...",
                        "You really understand.",
                        "You truly know the value",
                        "of hardship. It makes me",
                        "glad to see that a true",
                        "Knight like you still exists."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Essofeit",
                    args![
                        "Hahaha, finally, I've",
                        "found someone I can call",
                        "a true comrade. We're the",
                        "last of a dying breed if you",
                        "hadn't noticed, my friend. But",
                        "there is hope for Knights..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Essofeit",
                    args![
                        "It's decided, then.",
                        "As long as we live,",
                        "chivalry will never die!",
                        "Thanks to you, my faith",
                        "in the Knighthood has",
                        "been rekindled."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Essofeit",
                    args![
                        "In my recognition of you",
                        "as a true Knight, I will teach",
                        "you a special skill that I've",
                        "been researching recently. Why",
                        "don't you come back after I've",
                        "completed the preparations?"
                    ],
                )?;
                ctx.items().take(530, 5)?;
                ctx.items().take(748, 3)?;
                ctx.var("kngt_sk").set(Val::from(8))?;
                return ctx.close();
            }
            ctx.lines_as(
                "Essofeit",
                args![
                    "It's shameful...",
                    "Most new recruits into",
                    "the Prontera Chivalry are",
                    "more concerned about their",
                    "pensions than their honor!"
                ],
            )?;
            return ctx.close();
        } else if ctx.var("kngt_sk").get()? == 7 {
            ctx.lines_as(
                "Essofeit",
                args![
                    "Maybe I'm romanticizing",
                    "the past, but I don't agree",
                    "with all the bureaucracy",
                    "that is present today in",
                    "the Prontera Chivalry."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "Although, I admit",
                    "that I was a lot happier",
                    "back when honor was all",
                    "you needed. Nowadays, it ",
                    "seems like anyone can be",
                    "a Knight. Er, no offense~"
                ],
            )?;
            return ctx.close();
        } else if ctx.var("kngt_sk").get()? == 6 {
            ctx.lines_as(
                "Essofeit",
                args![
                    "So you've seen the Knights",
                    "of the 7th Division for what",
                    "they really are, eh? As a fellow Knight, you may understand my",
                    "feelings of disappointment."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "It's so disheartening...",
                    "I don't why we're Knights",
                    "or what we're training for.",
                    "When I was your age, well,",
                    "I wanted to risk my life and",
                    "achieve true strength."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "But... I'm sure that you",
                    "don't want to hear an old",
                    "man's tall tales. All those",
                    "wonderful battles, that glorious camaraderie forged between",
                    "rivals... It's all in the past."
                ],
            )?;
            ctx.next()?;
            ctx.var("kngt_sk").set(Val::from(7))?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "Maybe it's better that",
                    "I forget all about my old",
                    "fashioned ideals. I guess",
                    "times have changed, and that",
                    "my idea of chivalry may be dead."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("kngt_sk").get()? == 1 {
            ctx.lines_as(
                "Essofeit",
                args![
                    "Hm, why don't take a",
                    "tour of the 7th Division",
                    "and see the attitude of my",
                    "comrades for yourself? You'll",
                    "see Grand Master Maroujje",
                    "training the recruits outside."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("kngt_sk").get()? == 0 {
            ctx.lines_as(
                "Essofeit",
                args![
                    "Greetings. I am Essofeit",
                    "Lageiya of the 7th Division",
                    "of the Prontera Chivalry.",
                    "As a proud Knight upholding",
                    "the principles of honor and",
                    "chivalry, I am at your service."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "May I ask which division",
                    "you are from? Ah, you're",
                    "a Knight that's been granted",
                    "royal permisson to journey",
                    "as you please. That must be",
                    "great, the freedom you have."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "Sadly, the Division of",
                    "which I am member is too",
                    "bureaucratic. I believe that my",
                    "comrades are more concerned",
                    "with their pensions than with chivalry, or defending the weak."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "Even if we wanted to be",
                    "more active, there are too",
                    "many regulations that hamper",
                    "the good we can do. Over time,",
                    "the situation has grown much worse. But don't take my word..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Essofeit",
                args![
                    "Perhaps it will be better",
                    "if you visited the 7th Division",
                    "for yourself and speak to the",
                    "other Knights that are training. Hopefully, you will see what",
                    "I have seen for a long time..."
                ],
            )?;
            ctx.var("kngt_sk").set(Val::from(1))?;
            return ctx.close();
        }
    }
    ctx.lines_as(
        "Essofeit",
        args![
            "Greetings. I am Essofeit",
            "Lageiya of the 7th Division",
            "of the Prontera Chivalry.",
            "As a proud Knight upholding",
            "the principles of honor and",
            "chivalry, I am at your service."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Essofeit",
        args![
            "I only wish the others in",
            "my division would appreciate",
            "my values, and the true value",
            "of spilled blood. They may be",
            "my comrades, but I find it",
            "difficult to trust them."
        ],
    )?;
    ctx.close()
}

#[derive(Clone, Copy, Debug)]
enum TourStep {
    Start,
    OnTouch,
}

fn tour_run(ctx: &Ctx, mut step: TourStep) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TourStep::Start => {
                step = TourStep::OnTouch;
                continue 'machine;
            }
            TourStep::OnTouch => {
                if ctx.var("kngt_sk").get()?.number()? > 2 {
                    return Err(Stop::End);
                }
                if ctx.var("kngt_sk").get()?.number()? <= 2 {
                    ctx.lines_as("?", args!["Kiiiiiiai~!", "Yaaaaaaaaap!", "Hoo! Haa! Haiyah!"])?;
                    ctx.next()?;
                    ctx.lines_as("?", args!["Si-Aiyah!", "Rowr rowr rowr", "GrrrrrrrraaAAHH!"])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFThese Knights appear to be",
                        "putting themselves through",
                        "some gruelingly difficult",
                        "training. But as you listen",
                        "a little more closely, their",
                        "screams seem a bit dramatized.^000000"
                    ])?;
                    if ctx.var("kngt_sk").get()? == 1 {
                        ctx.var("kngt_sk").set(Val::from(2))?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "^3355FFThis group of Knights",
                    "appear to be undergoing",
                    "so pretty grueling training,",
                    "judging from the wailing pitch",
                    "of their battle screams.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn tour(ctx: &Ctx) -> Script {
    tour_run(ctx, TourStep::Start).map(|_| ())
}

pub fn tour_ontouch(ctx: &Ctx) -> Script {
    tour_run(ctx, TourStep::OnTouch).map(|_| ())
}

pub fn grand_master(ctx: &Ctx) -> Script {
    ctx.mes("[Grand Master]")?;
    if ctx.var("kngt_sk").get()? == 2 || ctx.var("kngt_sk").get()? == 3 {
        ctx.lines(args![
            "Alright men, you don't",
            "need to put ^333333that^000000 much effort",
            "into your training. You have",
            "to do this everyday, so make",
            "sure that you don't exhaust",
            "yourselves unnecessarily."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Grand Master",
            args![
                "The most important thing",
                "is that you don't injure",
                "yourselves. Take it easy",
                "and make sure that you",
                "don't pull a muscle.",
                "Um, that's an order!"
            ],
        )?;
        if ctx.var("kngt_sk").get()? == 2 {
            ctx.var("kngt_sk").set(Val::from(3))?;
        }
        return ctx.close();
    }
    ctx.lines(args![
        "Put your backs into it,",
        "men! I know training can",
        "be tough, but it'll make",
        "you harder, better, faster and",
        "stronger! Prontera's safety is your responsibility, Knights!"
    ])?;
    ctx.next()?;
    ctx.lines_as(
        "Grand Master",
        args!["...", "Oh, I'm just kidding around.", "Let's go take a break, guys."],
    )?;
    ctx.close()
}

pub fn knight_zabii(ctx: &Ctx) -> Script {
    if ctx.var("kngt_sk").get()? == 3 || ctx.var("kngt_sk").get()? == 4 {
        ctx.lines_as(
            "Zabi",
            args![
                "I guess we're more like",
                "government employees",
                "than actual Knights. We're",
                "overpaid, and don't really",
                "have to do anything other",
                "than pretend to train."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zabi",
            args![
                "Me? I plan on milking",
                "the system for all it's",
                "worth. Work here a few",
                "years, then live the rest",
                "of my life on a fat pension.",
                "Yeah, that's gonna be great."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zabi",
            args![
                "Whoa-whoa-whoa,",
                "the Grand Master's",
                "looking this way! Quit",
                "talkin' to me and let me",
                "grunt like I've got a hernia.",
                "Hooooo-AH! BWAH! HAI-YAH!"
            ],
        )?;
        if ctx.var("kngt_sk").get()? == 3 {
            ctx.var("kngt_sk").set(Val::from(4))?;
        }
        return ctx.close();
    }
    ctx.lines_as(
        "A Knight",
        args![
            "Hooooo-AH!",
            "BWAH! HAI-YAH!",
            "Oh man, I can barely",
            "breathe from all of this",
            "arduous training. Ugh,",
            "so incredibly enervated..."
        ],
    )?;
    ctx.close()
}

pub fn knight_drake(ctx: &Ctx) -> Script {
    if ctx.var("kngt_sk").get()? == 4 || ctx.var("kngt_sk").get()? == 5 {
        ctx.lines_as(
            "Gon",
            args![
                "What'll I get with",
                "my next paycheck?",
                "Ah~ I should get a nice",
                "necklace for my wife.",
                "I just know she'll love it!"
            ],
        )?;
        ctx.next()?;
        if ctx.var("kngt_sk").get()? == 4 {
            ctx.var("kngt_sk").set(Val::from(5))?;
        }
        ctx.lines_as(
            "Gon",
            args![
                "Let's see, how many",
                "more days until payday?",
                "One, two... Hmm. For some",
                "reason it never seems to",
                "come soon enough, you know?"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "A Knight",
        args![
            "Maybe I'm not not so good",
            "at fighting or defending the",
            "weak, but this Knight position",
            "is a pretty good job. Working",
            "here really lets me save up",
            "cash to invest in my future."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "A Knight",
        args![
            "We may never get to see",
            "any action, but at least",
            "I can take some pride",
            "in being a Knight."
        ],
    )?;
    ctx.close()
}

pub fn knight_sasword(ctx: &Ctx) -> Script {
    ctx.mes("[Jiya]")?;
    if ctx.var("kngt_sk").get()? == 5 || ctx.var("kngt_sk").get()? == 6 {
        ctx.lines(args![
            "Man, this division of",
            "the chivalry doesn't seem",
            "to have too much potential.",
            "Most of us here are pretty",
            "second rate, except maybe",
            "for that one guy, Essofeit."
        ])?;
        ctx.next()?;
        ctx.var("kngt_sk").set(Val::from(6))?;
        ctx.lines_as(
            "Jiya",
            args![
                "As for me, I don't have too",
                "much in the way of ambition.",
                "As long as I do what I'm told,",
                "they'll pay me. And as long",
                "as I get paid, I'm happy."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines(args![
        "D-don't talk to me!",
        "Gotta... muster up the...",
        "Strength to... f-finish all",
        "these... training exercises!",
        "Alright man, c-concentrate..."
    ])?;
    ctx.close()
}

pub fn knight_gattack(ctx: &Ctx) -> Script {
    ctx.mes("[Gatack]")?;
    if ctx.var("kngt_sk").get()? == 7 {
        ctx.lines(args![
            "Wait, where's",
            "Essofeit? He can't",
            "just skip daily training,",
            "even if he has the distinction",
            "of killing countless ^FF0000Mystcases^000000",
            "and ^FF0000Obeaunes^000000 in his time."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Gatack",
            args![
                "Gosh, I'm sick and tired of",
                "him talking about Knighthood.",
                "Essofeit insists that all Knights, even complete rookies, should be",
                "able to obtain 5 Candy Canes and 3 Witherless Roses from hunting."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatack",
            args![
                "Hell, I just hate to hear",
                "his preaching about his great",
                "experiences fighting monsters.",
                "But now Essofeit just researches stuff inside the building and",
                "doesn't do much else..."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines(args![
        "Man, Knight training",
        "is pretty rough. Don't",
        "they know that I'm far",
        "too delicate for all this",
        "strenuous activity?"
    ])?;
    ctx.next()?;
    ctx.lines_as(
        "Gatack",
        args![
            "Ah well, if I ever go",
            "down in battle, I won't",
            "be too surprised, seeing",
            "as I'm one of the weakest",
            "guys here. But when it happens,",
            "I'll be too busy looking good."
        ],
    )?;
    ctx.close()
}
