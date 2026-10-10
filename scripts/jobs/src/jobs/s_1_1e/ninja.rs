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

pub fn akagi(ctx: &Ctx) -> Script {
    if ctx.var("Class").get()? == constants::JOB_NOVICE {
        if ctx.var("JobLevel").get()? == 10 {
            ctx.lines_as(
                "Akagi",
                args![
                    "Hmmm...",
                    "You must have come,",
                    "sensing that someone",
                    "is waiting for you here.",
                    "Tell me, do you seek",
                    "the path of patience?"
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["No", "Yes"])? == 0 {
                ctx.lines_as(
                    "Akagi",
                    args!["I see.", "To each his own,", "I suppose. Take", "care of yourself."],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Akagi",
                args!["Very well.", "Then, let me", "set you on that", "path right away..."],
            )?;
            ctx.close_window()?;
            let spot = ctx.call(Function::Rand, args![3])?;
            if spot == 1 {
                ctx.warp("amatsu", 170, 229)?;
            } else if spot == 2 {
                ctx.warp("amatsu", 216, 188)?;
            } else {
                ctx.warp("amatsu", 178, 176)?;
            }
            return ctx.end();
        } else {
            ctx.lines_as(
                "Akagi",
                args!["Hm? I cannot be", "of any service to", "you until you grow", "a little stronger..."],
            )?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "Akagi",
            args![
                "Hmm...",
                "You and I...",
                "We are fairly equal in",
                "terms of combat ability.",
                "Perhaps we can spar",
                "together sometime."
            ],
        )?;
        return ctx.close();
    }
}

pub fn kuuga_gai_nq(ctx: &Ctx) -> Script {
    if ctx.var("Upper").get()? == 2 {
        ctx.lines_as(
            "Kuuga Gai",
            args![
                "I... I've never",
                "seen a baby as",
                "powerful as you!",
                "G-get away, you",
                "freak of nature!"
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("Class").get()? == constants::JOB_NOVICE {
        if ctx.var("JobLevel").get()?.number()? < 10 {
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "Hm? Have you come to",
                    "learn how to be a Ninja?",
                    "You're not quite experienced",
                    "enough yet, so come back",
                    "after you're more familiar",
                    "with fighting monsters."
                ],
            )?;
            return ctx.close();
        }
        if ctx.var("ninj_q").get()? == 0 {
            ctx.lines_as(ctx.player().name()?, args!["Excuse me.", "H-hello?"])?;
            ctx.next()?;
            ctx.lines_as("Kuuga Gai", args!["...............................", "How did you do that?"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args!["Do what? I didn't", "do anything, I don't think..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "H-How are you able to",
                    "see me? I'm supposed to",
                    "be invisible to the naked eye.",
                    "Ah, now I get it. Wildcat Joe",
                    "must have sent you to kill me! I won't fall for your tricks! Die!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "W-wait! I-I don't even",
                    "know who Wildcat Joe is!",
                    "Calm down, there's no",
                    "need to get violent!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "What...?",
                    "How did you dodge",
                    "all of my attacks?",
                    "You've got some talent,",
                    "I'll give you that."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "...............................",
                    "I came here hoping",
                    "to change my job",
                    "to a Ninja."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "...Oh. Is that all?",
                    "Hmm, you've got great",
                    "potential, but I can't help",
                    "you now. I've got too many",
                    "enemies, and I can't let my",
                    "guard down for even a second."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "That Wildcat Joe is",
                    "completely ruthless...!",
                    "He could strike at any time!",
                    "He'll do anything to achieve",
                    "victory over his enemies!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "Wait, wait, I just",
                    "thought of something.",
                    "Maybe you can help me out.",
                    "Do what I ask, and I'll teach",
                    "you a few of my skills if you",
                    "really want to be a Ninja."
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Sure.", "No, thanks."])? == 1 {
                ctx.lines_as(
                    "Kuuga Gai",
                    args![
                        "Hm? Well, alright.",
                        "Still, I don't see",
                        "why we can't help",
                        "each other in this",
                        "little predicament..."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "Great! Now, I wanted to",
                    "ask Wildcat Joe if he'd",
                    "agree to a temporary truce.",
                    "I'm aware that both of us",
                    "are out of weapons, so we",
                    "should get well equipped first."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "Please take this letter,",
                    "and deliver it to Wildcat",
                    "Joe in Einbroch. He's a master",
                    "of disguise, so keep a careful",
                    "eye out for him. Ah, and look",
                    "for him in a high place."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "Yeah, Wildcat Joe",
                    "always did have a thing",
                    "for hiding in high places.",
                    "Anyway, after you give him",
                    "the letter, come back and",
                    "let me know his answer."
                ],
            )?;
            ctx.var("ninj_q").set(Val::from(1))?;
            ctx.quests().start(6015)?;
            return ctx.close();
        } else if ctx.var("ninj_q").get()? == 1 {
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "Even if this task",
                    "isn't that urgent,",
                    "please hurry over to",
                    "Einbroch and deliver",
                    "my letter to Wildcat Joe."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("ninj_q").get()? == 2 {
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "Did you deliver that",
                    "letter to Wildcat Joe?",
                    "I still need to know his",
                    "response to my proposal",
                    "for a truce. Anyway, see",
                    "if you can needle him for it."
                ],
            )?;
            return ctx.close();
        } else if ctx.var("ninj_q").get()? == 3 {
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "Ah, you've returned.",
                    "So did Wildcat Joe send",
                    "you back here with his",
                    "response? Great, great,",
                    "please let me read it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "What...?! How could he",
                    "reject my proposal for",
                    "a truce?! This can only",
                    "mean that he's made another",
                    "Kunai. Nuts! I have to catch",
                    "up to him, or I'm a goner!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "Listen, you've got to help",
                    "me out again! I need you to",
                    "gather some materials so that",
                    "I can craft my own Kunai to fight Wildcat Joe. Then, I'll go ahead",
                    "and change your job to a Ninja."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "All you need",
                    "to bring me is",
                    "^3355FF5 Cyfars^000000 and",
                    "^3355FF1 Phracon^000000.",
                    "Please get those",
                    "as quickly as you can!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "Huh? That's funny,",
                    "Wildcat Joe actually",
                    "asked me to gather",
                    "those same materials."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "Curses! Then that means...",
                    "You actually helped Joe",
                    "in crafting his Kunai! No!",
                    "I should have thought about",
                    "that earlier! Well, it's too",
                    "late now. Just h-hurry it up!"
                ],
            )?;
            ctx.var("ninj_q").set(Val::from(4))?;
            ctx.quests().change(6017, 6018)?;
            return ctx.close();
        } else if ctx.var("ninj_q").get()? == 4 {
            if ctx.items().count(7053)? < 5 || ctx.items().count(1010)? < 1 {
                ctx.lines_as(
                    "Kuuga Gai",
                    args![
                        "Hurry and bring",
                        "^3355FF5 Cyfars^000000 and",
                        "^3355FF1 Phracon^000000 to me,",
                        "so that I can craft",
                        "my own Kunai to use",
                        "against Wildcat Joe!"
                    ],
                )?;
                return ctx.close();
            }
            if ctx.var("SkillPoint").get()? != 0 {
                ctx.lines_as(
                    "Kuuga Gai",
                    args![
                        "Whoa, whoa...",
                        "You still have some",
                        "leftover Skill Points.",
                        "You'd better spend all",
                        "of them before you",
                        "change jobs, right?"
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "Ah, you're back with",
                    "everything that I need.",
                    "You've come earlier than",
                    "I expected, eh? Great,",
                    "as promised, I'll turn",
                    "you into a Ninja."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "Let me formally introduce",
                    "myself. I am High Ninja Kuuga Gai",
                    "in the Touga Ninja Corps, and",
                    "I'm in charge of the search",
                    "party to find Sir Kazma."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "Sir Kazma is the chief",
                    "of my village, but he's",
                    "run away. This has resulted",
                    "in an internal conflict within",
                    "the Ninja Corps. Things are",
                    "pretty unstable right now..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "I initially didn't want to",
                    "accept you as a Ninja because",
                    "of this complicated situation.",
                    "However, you've proven that",
                    "you're truly worthy of joining",
                    "the Ninja ranks."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "According to his letter, even",
                    "Joe thinks highly of you. Just",
                    "remember that, as a Ninja, your",
                    "mission is your highest priority. But don't let mission objectives",
                    "supersede your conscience."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "''Secrecy above all else.''",
                    "To keep our secrets in the",
                    "shadows, you can only buy",
                    "or sell Ninja weapons with",
                    "authorized dealers. Please",
                    "keep that in mind."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "As of today, you are",
                    "now a proud member of the",
                    "Touga Ninja Corps. Be as",
                    "agile as the wind, and as",
                    "quiet as the falling shadows."
                ],
            )?;
            shared::other_global_functions::job_change(ctx, args![constants::JOB_NINJA])?;
            ctx.var("ninj_q").set(Val::from(5))?;
            ctx.items().take(7053, 5)?;
            ctx.items().take(1010, 1)?;
            ctx.items().give(13010, 1)?;
            ctx.quests().complete(6018)?;
            return ctx.close();
        } else {
            ctx.lines_as(
                "Kuuga Gai",
                args![
                    "How have you been?",
                    "Train hard: you want",
                    "to be able to vanish",
                    "without a trace. If you",
                    "can do that, you'll get",
                    "the respect of any Ninja~"
                ],
            )?;
            return ctx.close();
        }
    } else if ctx.var("BaseClass").get()? == constants::JOB_NINJA {
        ctx.lines_as(
            "Kuuga Gai",
            args![
                "How have you been?",
                "Train hard: you want",
                "to be able to vanish",
                "without a trace. If you",
                "can do that, you'll get",
                "the respect of any Ninja~"
            ],
        )?;
        return ctx.close();
    } else {
        ctx.lines_as(
            "Kuuga Gai",
            args![
                "What...?",
                "How were you able",
                "to find me hidden",
                "in the shadows?!",
                "You must be more than",
                "a common adventurer, eh?"
            ],
        )?;
        return ctx.close();
    }
}

pub fn suspicious_man_nq(ctx: &Ctx) -> Script {
    if ctx.var("ninj_q").get()? == 1 {
        ctx.lines_as(
            "Suspicious Man",
            args![
                "I've traveled to many",
                "countries, but I've never",
                "been on a building as high",
                "as Einbroch Tower. All the",
                "buildings in my hometown",
                "are tiny in comparison..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args![
                "Oh, are you from",
                "Amatsu? I'm looking",
                "for someone named",
                "Wildcat Joe from there."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Suspicious Man",
            args![
                "...No. No, I'm actually",
                "from Izlude, and I'm only",
                "here in Einbroch for some",
                "minerals. Tell me, why are",
                "you looking for this Wildcat Joe?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args![
                "Well, I need to deliver",
                "this letter to him and",
                "get his response so that",
                "I can become a Ninja."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Suspicious Man",
            args![
                "Really? Now that I think",
                "about it, I do think that I've",
                "run once or twice into him",
                "in this town. Though, he prefers to be called ''Red Leopard Joe,''",
                "instead of ''Wildcat Joe.''"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args![
                "I really want to help you",
                "find him, but first I need",
                "to find the minerals that",
                "I'm looking for. If you don't",
                "mind, would you help me?",
                "Then I can help you find Joe."
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Don't worry, I'll find him alone.", "Sure, I'll help you."])? == 0 {
            ctx.lines_as(
                "Suspicious Man",
                args![
                    "You sure about that...?",
                    "Red Leopard Joe is a true",
                    "master of disguise. You'll",
                    "need all the help you can",
                    "get to find him..."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Suspicious Man",
            args![
                "Great, I'm glad to",
                "hear that. Please",
                "help me find",
                "^3355FF5 Cyfars^000000 and",
                "^3355FF1 Phracon^000000."
            ],
        )?;
        ctx.var("ninj_q").set(Val::from(2))?;
        ctx.quests().change(6015, 6016)?;
        return ctx.close();
    } else if ctx.var("ninj_q").get()? == 2 {
        if ctx.items().count(7053)? < 5 || ctx.items().count(1010)? < 1 {
            ctx.lines_as(
                "Suspicious Man",
                args![
                    "Please bring",
                    "^3355FF5 Cyfars^000000 and",
                    "^3355FF1 Phracon^000000 to me as",
                    "soon as you can. Then,",
                    "I can help you find",
                    "Red Leopard Joe."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Suspicious Man",
            args![
                "Good, good. You've",
                "brought the minerals...",
                "Now, it's my turn to",
                "help you now. Here,",
                "let me see that letter."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["?????!!"])?;
        ctx.next()?;
        ctx.lines_as("Suspicious Man", args!["Why? Didn't you bring Kuuga Gai's letter for me?"])?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["Are you...", "Are you Wildcat Joe?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Suspicious Man",
            args![
                "...Yes, but I prefer to",
                "be called Red Leopard Joe.",
                "Kuuga Gai sent you to me, right?",
                "He's the only one who calls",
                "me that. So you want to be",
                "a Ninja, eh? Hmm, alright."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Red Leopard Joe",
            args![
                "If you want to be a Ninja,",
                "you should always be careful",
                "of what you see and what you trust. Don't forget that if your",
                "secrets are ever discovered, then you're finished as a Ninja."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Red Leopard Joe",
            args![
                "Remember to move",
                "quickly, and to always",
                "vanish without a trace.",
                "To remain hidden in the",
                "shadows is really our",
                "ultimate power."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["I see...", "..........."])?;
        ctx.next()?;
        ctx.lines_as(
            "Red Leopard Joe",
            args![
                "For now, let me read",
                "this letter. Let's see...",
                "Hm. I thought that Kuuga Gai",
                "would want to challenge me",
                "again, but he actually wants",
                "a temporary truce? Hah!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Red Leopard Joe",
            args![
                "Thanks to your help,",
                "I now have the minerals",
                "I need to construct a Kunai!",
                "Hahaha! I won't agree to a truce when I have the advantage!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Red Leopard Joe",
            args![
                "Anyway, let me write my",
                "response to him. I'll also",
                "give you my recommendation...",
                "I think you'll make a very fine",
                "Ninja, even if I did trick you",
                "just earlier. Heh heh heh!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["......", ".........", "............"])?;
        ctx.next()?;
        ctx.lines_as(
            "Red Leopard Joe",
            args![
                "Here you go.",
                "Please bring this",
                "letter to Kuuga Gai.",
                "It'll take a while to",
                "return to Amatsu, so let",
                "me send you there directly..."
            ],
        )?;
        ctx.items().take(1010, 1)?;
        ctx.items().take(7053, 5)?;
        ctx.var("ninj_q").set(Val::from(3))?;
        ctx.quests().change(6016, 6017)?;
        ctx.close_window()?;
        ctx.warp("amatsu", 113, 127)?;
        return ctx.end();
    } else if ctx.var("ninj_q").get()? == 3 {
        ctx.lines_as(
            "Red Leopard Joe",
            args![
                "Eh? I'm not sure what",
                "happened, but it seems",
                "that you haven't delivered",
                "my response to Kuuga Gai yet.",
                "Shall I directly send you",
                "to Amatsu right now?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["No, thanks.", "Yes, please."])? == 0 {
            ctx.lines_as(
                "Red Leopard Joe",
                args!["Alright. Well, I was", "just trying to save", "you some time."],
            )?;
            return ctx.close();
        }
        ctx.lines_as("Red Leopard Joe", args!["Okay, then.", "Goodbye for now."])?;
        ctx.close_window()?;
        ctx.warp("amatsu", 113, 127)?;
        return ctx.end();
    } else if ctx.var("ninj_q").get()? == 4 {
        ctx.lines_as(
            "Red Leopard Joe",
            args![
                "Kuuga Gai asked you to",
                "gather some materials",
                "too? Oh well, I suppose",
                "that I can't blame him.",
                "Besides, I should be able",
                "to beat him in a fair fight~"
            ],
        )?;
        return ctx.close();
    } else if ctx.var("ninj_q").get()? == 5 && ctx.var("BaseClass").get()? == constants::JOB_NINJA {
        ctx.lines_as(
            "Red Leopard Joe",
            args![
                "Oh, you're a Ninja~",
                "I hope you continue to",
                "train yourself and master",
                "all the Ninja skills that",
                "you can. Always remember",
                "to blend into the shadows."
            ],
        )?;
        return ctx.close();
    } else {
        ctx.lines_as(
            "Tourist",
            args![
                "I've traveled to many",
                "countries, but I've never",
                "been on a building as high",
                "as Einbroch Tower. All the",
                "buildings in my hometown",
                "are tiny in comparison..."
            ],
        )?;
        return ctx.close();
    }
}
