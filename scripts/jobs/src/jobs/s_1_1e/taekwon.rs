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

pub fn phoenix(ctx: &Ctx) -> Script {
    if ctx.var("Upper").get()? == 2 {
        ctx.lines_as("Phoenix", args!["Hello, child."])?;
        return ctx.close();
    } else if ctx.var("Class").get()? == constants::JOB_TAEKWON {
        ctx.lines_as(
            "Phoenix",
            args![
                "How is your training",
                "coming along? As your",
                "techniques become more",
                "refined or spectacular,",
                "never forget that you can",
                "always rely on the basics."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("Class").get()?.number()? > constants::JOB_NOVICE
        || (ctx.var("Class").get()? == constants::JOB_NOVICE && ctx.var("tk_q").get()? == 0)
    {
        ctx.lines_as(
            "Phoenix",
            args![
                "This land. Our once",
                "beautiful world has been",
                "stained by evil: there are",
                "too many men corrupted by",
                "darkness, too many monsters",
                "threatening the innocent..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Phoenix",
            args![
                "The havoc that reigns in this",
                "world is too much for normal",
                "humans, which cannot stand",
                "up for themselves against such",
                "overwhelming odds. Still, one",
                "must aspire to fight the odds."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Phoenix",
            args![
                "And so, I've dedicated",
                "myself to becoming stronger.",
                "I have been training to achieve",
                "enlightenment, developing an art to hone the mind and body that",
                "I wish to share with the world."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Phoenix",
            args![
                "I may not be able to change",
                "the world on my own, but I'll",
                "never stop training myself",
                "spiritually and physically.",
                "I know that the answer",
                "will come in time..."
            ],
        )?;
        ctx.next()?;
        if ctx.var("Class").get()? != constants::JOB_NOVICE {
            ctx.lines_as(
                "Phoenix",
                args![
                    "Noble adventurer:",
                    "if you know anyone who",
                    "has not chosen his path",
                    "in life, please recommend",
                    "him to me. If interested,",
                    "I may teach him my art..."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Phoenix",
            args![
                "If you have not decided on",
                "the path you wish to take in",
                "life, I'd like you to consider",
                "becoming a practitioner of my",
                "art. It won't be easy, but it will lead you to great strength..."
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Okay, I will join you.", "No, thank you."])? == 0 {
            if !shared::other_global_functions::f_canchangejob(ctx, vec![])?.is_true() {
                ctx.lines_as(
                    "Phoenix",
                    args![
                        "How unfortunate!",
                        "You're not yet ready to",
                        "begin training under my",
                        "tutelage with your current",
                        "Job Level. Please return when you reach Job Level 9 or higher."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Phoenix",
                args![
                    "Very well... I accept you",
                    "as my student. In beginning",
                    "training, your physical body",
                    "must first be conditioned in",
                    "order to perform the skills",
                    "that you will be learning."
                ],
            )?;
            ctx.next()?;
            if ctx.player().base_level()? > 19 {
                ctx.lines_as(
                    "Phoenix",
                    args![
                        "Hm. I see that you have",
                        "undergone sufficient physical",
                        "training as a Novice. Very good. Then let us prepare for your",
                        "spiritual training. Take a deep",
                        "breath, speak to me when ready."
                    ],
                )?;
                ctx.var("tk_q").set(Val::from(2))?;
                ctx.quests().start(6001)?;
                return ctx.close();
            }
            ctx.lines_as(
                "Phoenix",
                args![
                    "The mind is not necessarily",
                    "bound to the limits of the body, but you will never fulfill your",
                    "true potential without integrating mind and body. Go, gain ^FF00001 more",
                    "Base Level^000000, and then return."
                ],
            )?;
            ctx.next()?;
            ctx.var("taek_q").set(ctx.var("BaseLevel").get()?)?;
            ctx.var("tk_q").set(Val::from(1))?;
            ctx.quests().start(6000)?;
            ctx.lines_as(
                "Phoenix",
                args![
                    "I understand this is not an",
                    "easy task for Novices, but you",
                    "must ready yourself for the",
                    "hardship for this job. I shall",
                    "expect you to be stronger",
                    "the next time we meet."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Phoenix",
            args![
                "I understand. One's life can",
                "take many paths, but you can",
                "only choose to travel on one",
                "at a time. I hope that you work",
                "towards enlightenment in your",
                "very own way, adventurer."
            ],
        )?;
        return ctx.close();
    }
    match ctx.var("tk_q").get()?.number()? {
        1 => {
            if ctx.player().base_level()? > ctx.var("taek_q").get()?.number()? {
                ctx.var("tk_q").set(Val::from(2))?;
                ctx.quests().change(6000, 6001)?;
                ctx.lines_as(
                    "Phoenix",
                    args![
                        "Good. I sense that you",
                        "are more in tune with your",
                        "inner strength. That is the",
                        "natural result of leveling up.",
                        "We're ready to proceed with",
                        "the next portion of training."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Phoenix",
                args![
                    "You must gain ^FF00001 more",
                    "Base Level^000000 to prove that",
                    "you can endure the hardship",
                    "that entails this job. Never",
                    "neglect your training."
                ],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Phoenix",
                args![
                    "For your spiritual training,",
                    "I will ask you a series of",
                    "questions to test your spirit.",
                    "Relax. Answer as honestly",
                    "as you can. Your will and",
                    "convictions will be tested."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Phoenix",
                args![
                    "As a practitioner of my",
                    "art, the ability to quickly",
                    "make the best decision will",
                    "be necessary in battle. Now,",
                    "we will begin the questioning."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Phoenix",
                args!["When you encounter", "great difficulty, how do", "you generally respond?"],
            )?;
            ctx.next()?;
            match ctx.menu(&["I face it head on.", "Avoid it somehow.", "Regroup and analyze the problem."])? {
                0 => {
                    ctx.lines_as(
                        "Phoenix",
                        args![
                            "Yes, that is the answer",
                            "I wanted. Even if you cannot",
                            "handle a problem at first, we",
                            "can only benefit from such",
                            "strong determination. Don't",
                            "let any obstacle stop you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Phoenix",
                        args![
                            "Even if you fail, you",
                            "can only learn from the",
                            "experience when you give",
                            "your all. Half-hearted",
                            "attempts rarely yield",
                            "fruitful results."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Phoenix",
                        args![
                            "Seeing as you already",
                            "understand the importance",
                            "of one's will, we'll proceed",
                            "to the next question."
                        ],
                    )?;
                    ctx.next()?;
                }
                1 => {
                    ctx.lines_as(
                        "Phoenix",
                        args![
                            "Fool! How do you expect",
                            "to mature if you run away",
                            "from challenges? Fear can",
                            "be a healthy reaction that",
                            "can save your life, but true",
                            "cowardice is despicable."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Phoenix",
                        args![
                            "It disappoints me to",
                            "hear you say that. Never",
                            "say such a thing to me again.",
                            "Hm. Contemplate the meanings",
                            "of courage and cowardice, and",
                            "then speak to me once again."
                        ],
                    )?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Phoenix",
                        args![
                            "Regroup? It is good to do that",
                            "after you have been defeated.",
                            "But it is best to face problems",
                            "once you encounter them.",
                            "You will not always have",
                            "the luxury of regrouping."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Phoenix",
                        args![
                            "Problems can be predicted",
                            "and analyzed, but I think",
                            "immediate retreat is unwise.",
                            "Contemplate on your fears,",
                            "as well as what you define as",
                            "failure. Then, return to me."
                        ],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
            ctx.lines_as(
                "Phoenix",
                args![
                    "On your travels, you will",
                    "encounter many people with",
                    "differing backgrounds and",
                    "viewpoints. Inevitably, you",
                    "will encounter someone whose",
                    "way of life you cannot fathom."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Phoenix",
                args![
                    "Likewise, this person will",
                    "not understand your way of",
                    "life. When your two viewpoints",
                    "clash, causing heated conflict,",
                    "how would you respond?"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&[
                "Insist that I'm right, regardless",
                "Disregard conflicting viewpoint",
                "Accept differences and learn from them",
            ])? {
                0 => {
                    ctx.lines_as(
                        "Phoenix",
                        args![
                            "It's important to have your",
                            "own opinion. However, you",
                            "must recognize that you may",
                            "be wrong, and an opposing",
                            "view may have some merit."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Phoenix",
                        args![
                            "There is no one right",
                            "answer and the light of",
                            "truth can take many shades.",
                            "Such is the way of nature.",
                            "To force ideas on others is",
                            "an oppressive practice."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Phoenix",
                        args![
                            "Do not limit yourself",
                            "to a single view, and do",
                            "not stifle your growth by",
                            "adhering to a single truth.",
                            "Contemplate on this, and",
                            "then speak with me again."
                        ],
                    )?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Phoenix",
                        args![
                            "It is important to get",
                            "along with others, but",
                            "you will bring no value",
                            "to this world without your",
                            "own unique contributions,",
                            "thoughts and opinions."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Phoenix",
                        args![
                            "A conflict of ideals, when",
                            "conducted with respect for",
                            "yourself and others, is a",
                            "great opportunity to broaden",
                            "your understanding of the",
                            "world as it is to others."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Phoenix",
                        args![
                            "Reflect on this idea of",
                            "establishing harmony with",
                            "the self, and harmony with",
                            "others. Then, return to me."
                        ],
                    )?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Phoenix",
                        args![
                            "Good. You must see",
                            "differences for what they",
                            "truly are. You must also",
                            "take criticism to your own",
                            "views with grace and ",
                            "sincere consideration."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Phoenix",
                        args![
                            "It is impossible to know",
                            "everything in this world.",
                            "It is impossible to understand",
                            "every view. But that does not",
                            "mean that views you do not",
                            "understand are meritless."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Phoenix",
                        args![
                            "The one with whom you",
                            "disagree may have the",
                            "answer you do not know.",
                            "In your time of weakness,",
                            "this person may be your",
                            "greatest help. Remember that."
                        ],
                    )?;
                    ctx.next()?;
                }
                _ => {}
            }
            ctx.lines_as(
                "Phoenix",
                args![
                    "I am satisfied by the",
                    "answers you have given",
                    "me. Please reflect on what",
                    "we have discussed for a little",
                    "while. When your mind is calm,",
                    "come and speak to me."
                ],
            )?;
            ctx.var("tk_q").set(Val::from(3))?;
            ctx.quests().change(6001, 6002)?;
            return ctx.close();
        }
        3 => {
            ctx.lines_as(
                "Phoenix",
                args![
                    "Are you feeling calm",
                    "and at peace? I will ask",
                    "you a very important question.",
                    "Give me your honest answer."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Phoenix",
                args![
                    "^FF0000Are you ready to dedicate",
                    "yourself to the special art",
                    "I will teach you, and uphold",
                    "the dignity of its philosophy?"
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Yes.", "No."])? == 0 {
                ctx.lines_as(
                    "Phoenix",
                    args![
                        "Very well. You are no",
                        "longer just a student.",
                        "You are now entrusted with",
                        "the powers and responsibilities",
                        "of a disciple of ^FF0000Taekwon Do^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Phoenix",
                    args![
                        "''Taekwon'' has the meaning",
                        "of ''punching and kicking,''",
                        "and ''Do'' has the meaning",
                        "of ''art.'' This martial art is",
                        "focused on skills using",
                        "the fists and the feet."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Phoenix",
                    args![
                        "Please use this discipline",
                        "to hone your body and mind",
                        "and learn the skills that are",
                        "best suited to you. Never",
                        "shirk your training, or bring",
                        "shame to Taekwon Do."
                    ],
                )?;
                ctx.next()?;
                ctx.quests().complete(6002)?;
                shared::other_global_functions::job_change(ctx, args![constants::JOB_TAEKWON])?;
                shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
                ctx.items().give(2101, 1)?;
                ctx.lines_as(
                    "Phoenix",
                    args![
                        "You are still young, so",
                        "I assume you'll want a job",
                        "title. Mm. In that case, you"
                    ],
                )?;
                if ctx.var("Sex").get()? == constants::SEX_FEMALE {
                    ctx.mes("are now a ^FF0000Taekwon Girl^000000.")?;
                } else {
                    ctx.mes("are now a ^FF0000Taekwon Boy^000000.")?;
                }
                ctx.mes("Yes, that sounds good. ")?;
                ctx.next()?;
                ctx.lines_as(
                    "Phoenix",
                    args![
                        "Please, take this training",
                        "uniform and guard set: make",
                        "good use of these gifts. As",
                        "you travel and train, enlighten",
                        "others about our art and learn what you can from them in return."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Phoenix",
                    args![
                        "It is now time for you to",
                        "embark on your own journey",
                        "to find new challenges to",
                        "develop your strength.",
                        "Carry yourself with pride",
                        "as a Taekwon Do practitioner..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Phoenix",
                    args![
                        "Very well. I wish you luck.",
                        "I hope to see you again",
                        Val::from("sometime, ") + ctx.player().name()? + Val::from(".")
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Phoenix",
                args![
                    "Hm, perhaps you are not",
                    "quite ready to progress from",
                    "your status as a student to",
                    "a full fledged disciple.",
                    "When you feel prepared,",
                    "come and speak to me."
                ],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}
