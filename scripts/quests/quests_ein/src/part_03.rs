use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

pub fn einbroch_smog_alert(ctx: &Ctx) -> Script {
    einbroch_smog_alert_run(ctx, EinbrochSmogAlertStep::Start, Vec::new()).map(|_| ())
}

pub fn einbroch_smog_alert_onenable(ctx: &Ctx) -> Script {
    einbroch_smog_alert_run(ctx, EinbrochSmogAlertStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn einbroch_smog_alert_onmymobdead(ctx: &Ctx) -> Script {
    einbroch_smog_alert_run(ctx, EinbrochSmogAlertStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn einbroch_smog_alert_ontimer600000(ctx: &Ctx) -> Script {
    einbroch_smog_alert_run(ctx, EinbrochSmogAlertStep::OnTimer600000, Vec::new()).map(|_| ())
}

fn liotzburg_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("einfactory").get()? == 13 || ctx.var("einfactory").get()? == 14) {
        if ctx.call(Function::IsBeginQuest, vec![Val::from(8029)])? == 1 {
            ctx.call(Function::ChangeQuest, vec![Val::from(8029), Val::from(8030)])?;
        }
        ctx.var("einfactory").set(Val::from(14))?;
        ctx.lines_as("Liotzburg", args!["What...?", "Factory Repair", "budget? No way!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Liotzburg",
            args![
                "Why waste money?",
                "We haven't had any",
                "problems so far! Look,",
                "everything's fine! Why",
                "are you exaggerating",
                "such small details?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Liotzburg",
            args![
                "The field overseer,",
                "Zelmeto, just came by to",
                "ask for a budget increase.",
                "Well, I think he's lying!",
                "Everything's perfect!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Liotzburg",
        args![
            "I'm the plant",
            "superintendant of this",
            "factory. Most of my employees",
            "are diligent workers. I can't say that of everyone, but overall we're doing an excellent job. Ha ha ha~!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Liotzburg",
        args![
            "So long as this factory",
            "is well maintained, we won't",
            "have to worry about this city's",
            "safety. The field overseer,",
            "Zelmeto, is also very reliable."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Liotzburg",
        args![
            "I can trust Zelmeto",
            "to look after things,",
            "so there's no need for",
            "me to go inside the factory.",
            "Delegating work is great!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Liotzburg",
        args![
            "Our factory will",
            "continue to develop",
            "and everyone will be",
            "proud of the progress",
            "we're making. Yes, I can",
            "assure you of that!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn liotzburg_ein(ctx: &Ctx) -> Script {
    liotzburg_ein_body(ctx, Vec::new()).map(|_| ())
}

fn liotzburg_ein_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("$einpolution").get()?.number()? > 9 && !(ctx.var("$@alrdeinpoll").get()?.is_true())) {
        ctx.lines_as(
            "Liotzburg",
            args![
                "What's going on?!",
                "Who's responsible?!",
                "God, I can't believe",
                "this is happening!",
                "^333333*Cough Cough!*^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Liotzburg",
            args![
                "I need to get out of here!",
                "You! D-do something and",
                "fix this! I gotta hide and find",
                "someplace safe!"
            ],
        )?;
        ctx.close_window()?;
        if (ctx.var("$einpolution").get()?.number()? > 9 && !(ctx.var("$@alrdeinpoll").get()?.is_true())) {
            ctx.call(Function::DoNpcEvent, vec![Val::from("Einbroch Smog Alert::OnEnable")])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Liotzburg#ein")])?;
        }
        return Err(Stop::End);
    } else {
        return Err(Stop::End);
    }
}

pub fn liotzburg_ein_ontouch(ctx: &Ctx) -> Script {
    liotzburg_ein_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn zelmeto_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("$einpolution").get()?.number()? > 9 {
        ctx.lines_as(
            "Zelmeto",
            args![
                "We've got a big problem",
                "here! I appreciate that you've",
                "been gathering the materials,",
                "but the machines have been",
                "broken for too long!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "Right when I tried to",
                "fix it, a huge shortout",
                "occurred. Our town is",
                "probably filled with",
                "toxic fog right now!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "I'll try my best to fix",
                "this, but we really should",
                "have allocated some funds",
                "to fix this machine earlier!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "The most important",
                "thing is that you get",
                "out of here and find",
                "shelter! Right now!"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("einbroch"), Val::from(131), Val::from(83)])?;
        return Err(Stop::End);
    }
    if ctx.var("einfactory").get()? == 16 {
        ctx.lines_as(
            "Zelmeto",
            args![
                "We'll be putting good",
                "use to the materials you",
                "gave me. With your help,",
                "our factory will operate",
                "safely. At least, for just",
                "a little while longer."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.var("einfactory").get()? == 15 && ctx.call(Function::CountItem, vec![Val::from(7325)])?.number()? > 19)
        && ctx.call(Function::CountItem, vec![Val::from(7317)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(7319)])?.number()? > 9)
    {
        ctx.lines_as(
            "Zelmeto",
            args![
                "Ah, it's you again.",
                "It's shameful letting",
                "other people know about",
                "our miserable situation..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "There's nothing",
                "worth seeing here,",
                "so there really isn't",
                "a point in you coming to",
                "visit this place anymore."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Give him materials.:Huh.")])? {
            1 => {
                ctx.lines_as(
                    "Zelmeto",
                    args![
                        "...Hm?",
                        "Aren't these the",
                        "materials we need",
                        "to make repairs in",
                        "the factory? How did",
                        "you find all of these?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zelmeto",
                    args![
                        "I don't know how",
                        "I can possibly pay you",
                        "back for this great favor.",
                        "I appreciate that you've",
                        "stepped forward to help us."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zelmeto",
                    args![
                        "Oh...!",
                        "In my years of managing,",
                        "I've learned the ultimate",
                        "motivation techniques. Let",
                        "me enhance your motivation",
                        "to show you my gratitude."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zelmeto",
                    args![
                        "Now...",
                        "Just open your mind",
                        "and listen to my words",
                        "of encouragement",
                        "and inspiration..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zelmeto",
                    args![
                        "^236B8EWhen the going",
                        "gets rough, you've",
                        "gotta get rougher!",
                        "You gotta climb that",
                        "mountain 'cause no one's",
                        "gonna climb it for you!^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zelmeto",
                    args![
                        "^236B8EDon't give it up!",
                        "Go for broke!",
                        "Losers are quitters",
                        "and quitters are losers!"
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7325), Val::from(20)])?;
                ctx.call(Function::DelItem, vec![Val::from(7317), Val::from(10)])?;
                ctx.call(Function::DelItem, vec![Val::from(7319), Val::from(10)])?;
                ctx.var("$einpolution").set((ctx.var("$einpolution").get()? + Val::from(1)))?;
                ctx.call(Function::CompleteQuest, vec![Val::from(8031)])?;
                ctx.var("einfactory").set(Val::from(16))?;
                {
                    if ctx.var("BaseLevel").get()?.number()? < 41 {
                        ctx.call(Function::GetExperience, vec![Val::from(615), Val::from(0)])?;
                    } else {
                        if ctx.var("BaseLevel").get()?.number()? < 51 {
                            ctx.call(Function::GetExperience, vec![Val::from(3075), Val::from(0)])?;
                        } else if ctx.var("BaseLevel").get()?.number()? < 61 {
                            ctx.call(Function::GetExperience, vec![Val::from(6604), Val::from(0)])?;
                        } else if ctx.var("BaseLevel").get()?.number()? < 71 {
                            ctx.call(Function::GetExperience, vec![Val::from(18508), Val::from(0)])?;
                        } else if ctx.var("BaseLevel").get()?.number()? < 81 {
                            ctx.call(Function::GetExperience, vec![Val::from(32066), Val::from(0)])?;
                        } else if ctx.var("BaseLevel").get()?.number()? < 91 {
                            ctx.call(Function::GetExperience, vec![Val::from(76026), Val::from(0)])?;
                        } else {
                            ctx.call(Function::GetExperience, vec![Val::from(290675), Val::from(0)])?;
                        }
                    }
                }
                ctx.next()?;
                ctx.lines_as(
                    "Zelmeto",
                    args![
                        "^333333*Whew*^000000",
                        "I haven't given that much",
                        "inspriration in a while, but",
                        "your help was well worth it.",
                        "I'm going to start the repairs, but once again, I'd like to thank you."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Zelmeto",
                    args![
                        "^333333*Sigh...*^000000",
                        "I'm really worried",
                        "about this factory's",
                        "future. What is our",
                        "superintendant thinking...?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("einfactory").get()? == 15 {
        ctx.lines_as(
            "Zelmeto",
            args![
                "We need",
                "at least",
                "20 ^FF0000Flexible Tubes^000000,",
                "10 ^FF0000Rusty Screw^000000 and",
                "10 ^FF0000Used Iron Plate^000000",
                "to repair this factory."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "^333333*Sigh...*^000000",
                "But there's no way",
                "we can get all of those",
                "things. Our budget isn't",
                "big enough to cover it..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("einfactory").get()? == 14 {
        ctx.lines_as("Zelmeto", args!["...", "......"])?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "^333333*Sigh*^000000",
                "My proposal was rejected",
                "by our superintendant. But",
                "maintainance and repairs",
                "are crucial for peak operating",
                "efficiency and worker safety!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "I'm frustrated and worried.",
                "Maybe nothing will happen",
                "for now, but we've got to",
                "safeguard our future by",
                "regularly maintaining",
                "all of these machines."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "Even possible threats",
                "to the safety of our workers",
                "can't be ignored. Isn't there",
                "something I can do? ^333333*Sigh*^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "If we can",
                "just get",
                "20 ^FF0000Flexible Tube^000000,",
                "10 ^FF0000Rusty Screw^000000 and",
                "10 ^FF0000Used Iron Plate^000000,",
                "we could make those repairs."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8030), Val::from(8031)])?;
        ctx.var("einfactory").set(Val::from(15))?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "But without funds, there's",
                "no way we can purchase",
                "those items. If something",
                "happens, who's going to",
                "be responsible?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("einfactory").get()? == 13 {
        ctx.lines_as(
            "Zelmeto",
            args!["I've got to report this", "to our superintendant", "as soon as possible."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "With any luck, he'll approve",
                "a budget increase so that we",
                "can get all of the materials",
                "needed for the repairs."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("einfactory").get()? == 12 {
        ctx.lines_as(
            "Zelmeto",
            args![
                "Well, I figured that both",
                "conveyors would have",
                "similar problems. We",
                "can fix them at the",
                "same time, but it'll",
                "be a hassle."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "Thank you so much for",
                "your help. Without you,",
                "I'm pretty sure we wouldn't",
                "know about these problems",
                "until it was too late."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "Now, I've got to make sure",
                "we have enough materials",
                "to make the repairs so that",
                "the machines will be safely",
                "functioning again."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8028), Val::from(8029)])?;
        ctx.var("einfactory").set(Val::from(13))?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "First, I better",
                "hurry and request",
                "an increase for the",
                "Factory Repair budget",
                "from our superintendant."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("einfactory").get()? == 11 {
        ctx.lines_as(
            "Zelmeto",
            args![
                "The machine which",
                "you are supposed to",
                "inspect right now",
                "is a large conveyor."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "Remember that we",
                "also have a small sized",
                "conveyor, so make sure",
                "that you examine the",
                "larger one, alright?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("einfactory").get()? == 10 {
        ctx.lines_as("Zelmeto", args!["This is", "worse than", "I imagined..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "We've got to start",
                "repairs as soon as we",
                "can! Hopefully, we can",
                "resolve this before any",
                "serious problems happen..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "Alright, the last",
                "thing that you need to",
                "inspect is a ^FF0000large converyor^000000.",
                "It's similar to the one you",
                "inspected before, but it's",
                "bigger and more powerful."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "We have only one of these",
                "machines and it's usually",
                "moved around a lot since",
                "a lot of people in the factory",
                "use it. I really don't know",
                "where it could be now."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8026), Val::from(8027)])?;
        ctx.var("einfactory").set(Val::from(11))?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "Still, I'm sure that",
                "it's inside the building,",
                "so you should be able to",
                "find it. I hope you can inspect",
                "that conveyor for me soon."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("einfactory").get()? == 9 {
        ctx.lines_as(
            "Zelmeto",
            args![
                "This time, you need",
                "to inspect an outdoor",
                "pipe that is located far",
                "outside of the factory."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "Since there aren't any",
                "other machines in that",
                "area, that pipe shouldn't",
                "be too hard to find."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("einfactory").get()? == 8 {
        ctx.lines_as(
            "Zelmeto",
            args![
                "Huh?",
                "I'm suprised to hear",
                "that. ^333333*Sigh*^000000 There's just",
                "too many things that need",
                "fixing. This is terrible..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "Well, let me worry",
                "about that for now. Please",
                "focus on continuing to inspect",
                "some of the other machines."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "Now, there's a pipe inside",
                "this factory that I want you",
                "to look at. Many of our pipes",
                "aren't in the best condition,",
                "but this particular one might",
                "be severely damaged."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "Now, the pipe I want",
                "you to inspect is located",
                "near those large caultrons",
                "of molten metal. You should",
                "be able to find it pretty easily."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8024), Val::from(8025)])?;
        ctx.var("einfactory").set(Val::from(9))?;
        ctx.lines_as("Zelmeto", args!["Thanks again", "for your help,", "adventurer."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("einfactory").get()? == 7 {
        ctx.lines_as(
            "Zelmeto",
            args![
                "The machine which",
                "I want you to inspect",
                "this time is a small",
                "sized conveyor."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "Be sure that you",
                "inspect the small",
                "one, since we also",
                "have a large conveyor",
                "in the factory as well."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("einfactory").get()? == 6 {
        ctx.lines_as(
            "Zelmeto",
            args![
                "I see...",
                "It's most likely that",
                "there was a short",
                "circuit and most",
                "of the internal devices",
                "were burnt out..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "Thanks for checking",
                "that out for me. Now,",
                "the next machine I need",
                "you to inspect is different",
                "than the others I've had",
                "you examine."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "It's a mechanical",
                "hand that transports",
                "small objects. We didn't",
                "really give it a name, but",
                "you should be able to find it."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "Recently, it seems",
                "that there have been",
                "problems in operating",
                "that machine. If something's",
                "broken, we need to know",
                "and fix it right away."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8022), Val::from(8023)])?;
        ctx.var("einfactory").set(Val::from(7))?;
        ctx.lines_as("Zelmeto", args!["Thanks again", "in advance."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("einfactory").get()? == 5 {
        ctx.lines_as(
            "Zelmeto",
            args![
                "I'd like you to inspect",
                "the control panel. It's",
                "fairly large and can be",
                "found in the middle of the",
                "factory. You shouldn't have",
                "too much trouble finding it."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("einfactory").get()? == 4 {
        ctx.lines_as(
            "Zelmeto",
            args![
                "What...?",
                "This is worse",
                "than I expected. But",
                "it's good that we know",
                "about these problems",
                "as soon as possible."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "Don't you worry,",
                "we'll take care of",
                "this. In the meantime,",
                "I'd like you to inspect",
                "the next machine for me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "I want you to check",
                "a ^FF0000control panel^000000. It's the",
                "same kind as the one",
                "you just inspected, but",
                "bigger in size."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "It's located in the",
                "middle of the factory,",
                "so you should be able",
                "to find it. It may be in bad",
                "condition, even though it's",
                "operating fine for now..."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8020), Val::from(8021)])?;
        ctx.var("einfactory").set(Val::from(5))?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "We need to ensure that",
                "it's stable, reliable and",
                "doesn't pose a threat to",
                "our workforce. Thanks",
                "again, adventurer."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("einfactory").get()? == 3 {
        ctx.lines_as(
            "Zelmeto",
            args![
                "You need to inspect",
                "an automatic pressure",
                "governor. It looks fine,",
                "but sometimes it makes",
                "strange noises."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "It probably will",
                "be a good idea to",
                "check that machine",
                "more carefully this",
                "time, just in case."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Zelmeto", args!["Thank you", "for helping us,", "adventurer."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("einfactory").get()? == 2 {
        ctx.lines_as(
            "Zelmeto",
            args![
                "Huh, I see.",
                "We must do something",
                "about that as soon as",
                "we can. Now, let me tell",
                "you what to check next."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "There are 3 automatic",
                "pressure governors which",
                "hammer the bent iron plates",
                "from above to flatten them. It",
                "seems that one of them may",
                "have some kind of problem."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zelmeto",
            args![
                "Please inspect the ^FF0000automatic pressure governors^000000. Even if the",
                "problem seems small, please",
                "report it to me. I know it might seem fine now, but I want to prevent an accident if I can."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8018), Val::from(8019)])?;
        ctx.var("einfactory").set(Val::from(3))?;
        ctx.lines_as("Zelmeto", args!["Thank you", "in advance,", "adventurer."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("einfactory").get()? == 1 {
        ctx.lines_as(
            "Zelmeto",
            args![
                "If you would,",
                "please inspect the",
                "2nd control panel that",
                "seems to have been",
                "broken for a while..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Zelmeto",
        args![
            "Ah, you must be a visitor.",
            "I'm Zelmeto Abellov, the",
            "field overseer. Have you",
            "been in this facility before?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Zelmeto",
        args![
            "This factory plays an",
            "important role in our city",
            "and generates a lot of income.",
            "However, our employees suffer",
            "from a poor work environment."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Zelmeto",
        args![
            "Our superintendant makes a lot",
            "of money and seems content with",
            "the current situation. However, the rest of the workforce doesn't enjoy all of the benefits he receives..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Zelmeto",
        args![
            "Many people have already",
            "quit and there are only a few",
            "people who continue to work",
            "here. So now we're understaffed",
            "and I'm in quite a bind..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Zelmeto",
        args![
            "There are some urgent",
            "tasks I need done, but",
            "there's no way for me",
            "to recruit new workers.",
            "Ah, I'm sorry, I've spoken too",
            "freely about my own problems..."
        ],
    )?;
    ctx.next()?;
    'b2: {
        let subject2 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("You're understaffed?:No, it's okay.")],
        )?);
        let mut matched2 = false;
        let no_case2 = !subject2.loosely_equals(&Val::from(1)) && !subject2.loosely_equals(&Val::from(2));
        if !matched2 && subject2.loosely_equals(&Val::from(1)) {
            matched2 = true;
        }
        if matched2 {
            ctx.lines_as(
                "Zelmeto",
                args![
                    "Yes, we are!",
                    "I don't have enough",
                    "people to inspect the",
                    "factory machines and",
                    "determine what kinds",
                    "of problems we have."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Zelmeto",
                args![
                    "It's a time consuming",
                    "task I'd rather do on my",
                    "own. However, between that",
                    "and managing the workforce,",
                    "I don't have enough time..."
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("I can help you.:Keep up the good job.")])? {
                1 => {
                    ctx.lines_as(
                        "Zelmeto",
                        args![
                            "You can help me?",
                            "I know something like",
                            "this is too much to ask,",
                            "but I'll accept any help",
                            "anyone offers me. I'm",
                            "that desperate."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Zelmeto",
                        args![
                            "Alright, I'll have you",
                            "inspect the machines",
                            "in the factory one by one.",
                            "It's imperative that we know",
                            "what needs to be repaired",
                            "and what's working fine."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Zelmeto",
                        args![
                            "First, find the ^FF00002nd control",
                            "panel^000000 and determine its",
                            "status. I'm fairly certain that",
                            "it broke a long time ago, but",
                            "it wouldn't hurt to make sure.",
                            "You should find it easily."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::SetQuest, vec![Val::from(8017)])?;
                    ctx.var("einfactory").set(Val::from(1))?;
                    ctx.lines_as(
                        "Zelmeto",
                        args![
                            "When you finish your",
                            "inspection, report back",
                            "to me so I can tell you",
                            "which machine to check",
                            "next. Thanks again for",
                            "offering to help."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Zelmeto",
                        args![
                            "Well, it's a living.",
                            "^333333*Sigh*^000000 I can put up with",
                            "this, but I hope the higher",
                            "ups will consider improving",
                            "the work environment here..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched2 && subject2.loosely_equals(&Val::from(2)) {
            matched2 = true;
        }
        if matched2 {
            ctx.lines_as(
                "Zelmeto",
                args![
                    "Thank you for",
                    "your kindness.",
                    "And please don't",
                    "let anyone know about",
                    "anything I just told you."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn zelmeto(ctx: &Ctx) -> Script {
    zelmeto_body(ctx, Vec::new()).map(|_| ())
}

fn s_2nd_control_panel_ins_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("einfactory").get()? == 1 || ctx.var("einfactory").get()? == 2) {
        if ctx.call(Function::IsBeginQuest, vec![Val::from(8017)])? == 1 {
            ctx.call(Function::ChangeQuest, vec![Val::from(8017), Val::from(8018)])?;
        }
        ctx.var("einfactory").set(Val::from(2))?;
        ctx.lines(args![
            "^3355FFIt's the 2nd control panel",
            "Zelmeto asked you to inspect.",
            "It looks totally broken: screws",
            "are missing, and the iron cover",
            "has been bent open, revealing",
            "a tangled mess of wires inside.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn s_2nd_control_panel_ins(ctx: &Ctx) -> Script {
    s_2nd_control_panel_ins_body(ctx, Vec::new()).map(|_| ())
}

fn s_3rd_pressure_governor_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("einfactory").get()? == 3 || ctx.var("einfactory").get()? == 4) {
        if ctx.call(Function::IsBeginQuest, vec![Val::from(8019)])? == 1 {
            ctx.call(Function::ChangeQuest, vec![Val::from(8019), Val::from(8020)])?;
        }
        ctx.var("einfactory").set(Val::from(4))?;
        ctx.lines(args![
            "^3355FFAt first glance, this",
            "pressure governor looks",
            "perfectly fine. But after you",
            "check it more carefully, you",
            "find that it's making strange",
            "grinding noises and a few of",
            "the surface screws are loose."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn s_3rd_pressure_governor_1(ctx: &Ctx) -> Script {
    s_3rd_pressure_governor_1_body(ctx, Vec::new()).map(|_| ())
}

fn main_control_panel_ins_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("einfactory").get()? == 5 || ctx.var("einfactory").get()? == 6) {
        if ctx.call(Function::IsBeginQuest, vec![Val::from(8021)])? == 1 {
            ctx.call(Function::ChangeQuest, vec![Val::from(8021), Val::from(8022)])?;
        }
        ctx.var("einfactory").set(Val::from(6))?;
        ctx.lines(args![
            "^3355FFThe main control panel",
            "doesn't look like it has",
            "any problems. But after",
            "tapping on its surface,",
            "you hear a disheartening",
            "hollow sound. It looks like",
            "it's missing some parts...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn main_control_panel_ins(ctx: &Ctx) -> Script {
    main_control_panel_ins_body(ctx, Vec::new()).map(|_| ())
}

fn conveyor_ins_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("einfactory").get()? == 7 || ctx.var("einfactory").get()? == 8) {
        if ctx.call(Function::IsBeginQuest, vec![Val::from(8023)])? == 1 {
            ctx.call(Function::ChangeQuest, vec![Val::from(8023), Val::from(8024)])?;
        }
        ctx.var("einfactory").set(Val::from(8))?;
        ctx.lines(args![
            "^3355FFThe conveyor's movements",
            "look jittery and clumsy. The",
            "mechanical arm also doesn't",
            "look powerful enough to bear",
            "the loads that it's carrying. The screws in the conveyor look",
            "loose and rusted over.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn conveyor_ins(ctx: &Ctx) -> Script {
    conveyor_ins_body(ctx, Vec::new()).map(|_| ())
}

fn pipe_ins_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("einfactory").get()? == 9 || ctx.var("einfactory").get()? == 10) {
        if ctx.call(Function::IsBeginQuest, vec![Val::from(8025)])? == 1 {
            ctx.call(Function::ChangeQuest, vec![Val::from(8025), Val::from(8026)])?;
        }
        ctx.var("einfactory").set(Val::from(10))?;
        ctx.lines(args![
            "^3355FFThe inspection of this",
            "pipe didn't take very long.",
            "It's bloated and worn out",
            "from long durations of ",
            "being overloaded with",
            "pressure. It's a wonder",
            "it hasn't exploded yet."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn pipe_ins(ctx: &Ctx) -> Script {
    pipe_ins_body(ctx, Vec::new()).map(|_| ())
}

fn conveyor_ins2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("einfactory").get()? == 11 || ctx.var("einfactory").get()? == 12) {
        if ctx.call(Function::IsBeginQuest, vec![Val::from(8027)])? == 1 {
            ctx.call(Function::ChangeQuest, vec![Val::from(8027), Val::from(8028)])?;
        }
        ctx.var("einfactory").set(Val::from(12))?;
        ctx.lines(args![
            "^3355FFThis conveyor seems",
            "to have similar problems",
            "as its smaller version. Its",
            "movements are awkward,",
            "erratic and weak, and almost",
            "all of its screws are rusted.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn conveyor_ins2(ctx: &Ctx) -> Script {
    conveyor_ins2_body(ctx, Vec::new()).map(|_| ())
}

fn factory_quest_test_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    l_i = shared::other_gm_npcs::f_gm_npc(ctx, vec![Val::from(8028), Val::from(0), Val::from(0), Val::from(9000)])?;
    if l_i.clone() == -2 {
        ctx.lines_as("Test1", args!["Boo~ya."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if l_i.clone() == -1 {
        ctx.lines_as("Test1", args!["Do you want", "to cancel~?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if l_i.clone() == 0 {
        ctx.lines_as("Test1", args!["Whoa...", "That is", "sooo wrong!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Test1",
            args![
                "The current value",
                "of the global variable",
                "for the Factory Quest",
                ((Val::from("is... ^3355FF") + ctx.var("z").get()?) + Val::from("^000000.")),
                "You wanna change?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("0:10:Invade")])? {
            1 => {
                ctx.lines_as("Test1", args!["Okay...!", "It's been", "changed to ''0.''"])?;
                ctx.var("$einpolution").set(Val::from(0))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Test2", args!["Okay...!", "It's been", "changed to ''10.''"])?;
                ctx.var("$einpolution").set(Val::from(10))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Einbroch Smog Alert::OnEnable")])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn factory_quest_test(ctx: &Ctx) -> Script {
    factory_quest_test_body(ctx, Vec::new()).map(|_| ())
}

fn buender_hikeman_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("ein_hicman"), Val::from(2)])?;
    if ctx.var("shinokas_quest").get()? == 0 {
        ctx.lines_as("Buender Hikeman", args!["...", "......"])?;
        ctx.next()?;
        ctx.lines_as("Buender Hikeman", args!["...", "......", "......You..."])?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_QUESTION")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Buender Hikeman", args!["...", "......", "......You...", "......Stop it..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Buender Hikeman",
            args!["...", "......", "......You...", "......Stop it...", "...You ^FF0000bastard^000000!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Buender Hikeman", args!["RaaaaAAAARGHHH!!"])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThe old man seems",
            "slightly irked at seeing",
            "you. Unfortunately, his",
            "screaming and rambling",
            "is totally incoherent.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Buender Hikeman",
            args![
                "It ^FF0000IS^000000 you!",
                "You're responsible!",
                "You've taken everything",
                "away from me!!"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("'What are you talking about?':Ignore Him.")])? {
            1 => {
                ctx.lines_as(
                    "Buender Hikeman",
                    args![
                        "How dare you...",
                        "How dare you treat ",
                        "after destroying all the",
                        "happiness in my life!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Buender Hikeman",
                    args![
                        "Damn you...",
                        "How can you have",
                        "the audacity to pretend",
                        "as if nothing happened?!",
                        "^333333*C-cough Cough...*^000000"
                    ],
                )?;
                ctx.next()?;
            }
            2 => {
                ctx.lines_as("Buender Hikeman", args!["W...wait!", "I said wait!", "^333333*Cough!*^000000"])?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from("ein_hicman"), Val::from(255)])?;
                return Err(Stop::End);
            }
            _ => {}
        }
        ctx.lines_as(
            "Buender Hikeman",
            args![
                "Are you so evil to",
                "just shallowly forget",
                "what you've done to our",
                "lives? Did you already",
                "forget what you did",
                "here in Einbech?!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Buender Hikeman",
            args![
                "It was such a long",
                "time ago, but I'll never",
                "forget. This town was",
                "small, but full of folk",
                "with warm hearts..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Buender Hikeman",
            args![
                "Me, Khartophe, Anuto,",
                "Maskharundt... All of",
                "us were friends hired",
                "by that big businessman",
                "to dig up ores in the mine."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Buender Hikeman",
            args![
                "And then there",
                "was you! All of us",
                "put together made the",
                "greatest mining team!",
                "That was, until, we",
                "discovered ^FF0000it^000000."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Buender Hikeman",
            args!["Yes...", "The mysterious ore", "that dazzled with a", "magnificent light."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Buender Hikeman",
            args![
                "But we should have known",
                "that the ^3131FFUngoliant^000000 would",
                "be around that ore. We",
                "should have realized",
                "the danger..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Buender Hikeman",
            args![
                "We reported our findings",
                "to our employer, and then",
                "the ore just disappeared. He",
                "must have sent it somewhere,",
                "it was none of our business."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Buender Hikeman",
            args![
                "Then life was back",
                "to normal for a while.",
                "But one day you yelled",
                "to us that you had found",
                "another special, mysterious",
                "ore in the mines."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Buender Hikeman",
            args![
                "But when we came",
                "over to check the hole",
                "you dug up, you know",
                "what we found...?!"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Ungoliant?:A mysterious ore, right?:Nothing...?")])? {
            1 => {
                ctx.lines_as(
                    "Buender Hikeman",
                    args!["Don't you remember", "what happened? What", "you did to us at that time?!"],
                )?;
                ctx.next()?;
            }
            2 => {
                ctx.lines_as(
                    "Buender Hikeman",
                    args!["Don't you remember", "what happened? What", "you did to us at that time?!"],
                )?;
                ctx.next()?;
            }
            3 => {
                ctx.lines_as("Buender Hikeman", args!["Yes...", "Nothing."])?;
                ctx.next()?;
            }
            _ => {}
        }
        ctx.lines_as("Buender Hikeman", args!["There was nothing", "inside the hole!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Buender Hikeman",
            args![
                "Then you pointed to",
                "the wall behind us and",
                "screamed that Ungoliant",
                "was coming! In our panic",
                "we started to dig our way out!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Buender Hikeman", args!["I remember that the expression on your face seemed so strange. I had thought you looked sad, but now I'm sure you were consumed by greed! We trusted you and you betrayed us! ^FFFFFFspace^000000"])?;
        ctx.next()?;
        ctx.lines_as(
            "Buender Hikeman",
            args![
                "When we finally smashed",
                "down that last wall, everything",
                "starting to fall around us. We",
                "were the only two to survive",
                "that tunnel collapse."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Buender Hikeman", args!["Then I learned...", "You planned it all along."])?;
        ctx.call(Function::SetQuest, vec![Val::from(2071)])?;
        ctx.var("shinokas_quest").set(Val::from(1))?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("ein_hicman"), Val::from(255)])?;
        return Err(Stop::End);
    } else {
        if ctx.var("shinokas_quest").get()? == 1 {
            ctx.lines_as("Buender Hikeman", args!["Bastard!", "I'm sick of", "your lies!"])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("I'm not who you think!:How did you survive the accident?")])? {
                1 => {
                    ctx.lines_as(
                        "Buender Hikeman",
                        args![
                            "Ha...!",
                            "Do you think",
                            "I'd so easily",
                            "forget the face",
                            "of the person who",
                            "shattered my life?!"
                        ],
                    )?;
                    ctx.next()?;
                }
                2 => {}
                _ => {}
            }
            ctx.next()?;
            ctx.lines_as(
                "Buender Hikeman",
                args![
                    "When I came to,",
                    "I was lying on my",
                    "stomach in the ruins",
                    "of that dark tunnel."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Buender Hikeman", args!["And I found...", "You know what I found."])?;
            ctx.next()?;
            ctx.lines_as(
                "Buender Hikeman",
                args![
                    "^333333*Cough Cough*^000000",
                    "The corpses of my friends!",
                    "Khartophe, Anuto, Maskharundt!",
                    "Great men and my best friends.",
                    "But where were you?!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Buender Hikeman",
                args![
                    "Your body was nowhere",
                    "to be found. I searched",
                    "the tunnel and finally",
                    "climbed outside where",
                    "I was found unconscious."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Buender Hikeman",
                args![
                    "I was so stupid.",
                    "It was because of",
                    "that ore! You killed our",
                    "friends and destroyed",
                    "my life for that thing!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Buender Hikeman",
                args![
                    "And now...",
                    "You come out of",
                    "hiding and show up.",
                    "What do you want of",
                    "me? What more can",
                    "you possibly take away?!"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("I'm not who you think I am!:I'd like to apologize.")])? {
                1 => {
                    ctx.lines_as(
                        "Buender Hikeman",
                        args![
                            "Quit lying!",
                            "You've stirred up",
                            "my hatred by showing",
                            "up again! I've never",
                            "forgotten that day!"
                        ],
                    )?;
                    ctx.next()?;
                }
                2 => {
                    ctx.lines_as(
                        "Buender Hikeman",
                        args!["Ha ha...", "Apologize?", "The harm is", "already done..."],
                    )?;
                    ctx.next()?;
                }
                _ => {}
            }
            ctx.lines_as(
                "Buender Hikeman",
                args![
                    "It's too late",
                    "for you now. For",
                    "the sake of my friends,",
                    "I'll have my vengeance!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Buender Hikeman", args!["Prepare to die!", "^3131FFShinokas^000000!!!!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Buender Hikeman",
                args![
                    "...!",
                    "^333333*Cough! Cough!*^000000",
                    "Noooo! N-not now...",
                    "^333333*Cough! Cough!*^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["...", "I better", "get away", "from him!"],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFYou ran away from Hikeman",
                "as he collapsed on the ground.",
                "It wouldn't be a good idea to",
                "provoke the old man anymore,",
                "intentionally or not.^000000"
            ])?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2071), Val::from(2072)])?;
            ctx.var("shinokas_quest").set(Val::from(2))?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from("ein_hicman"), Val::from(255)])?;
            return Err(Stop::End);
        } else if ctx.var("shinokas_quest").get()? == 2 {
            ctx.lines(args![
                "^3355FFIt'd be best",
                "to avoid aggravating",
                "the old man for now.^000000"
            ])?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from("ein_hicman"), Val::from(255)])?;
            return Err(Stop::End);
        } else if ctx.var("shinokas_quest").get()? == 10 {
            ctx.lines(args![
                "^3355FFHikeman is dozing",
                "off in his chair. Judging",
                "from the look of discomfort",
                "on his face, he seems to be",
                "having a nightmare."
            ])?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Excuse me..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Buender Hikeman",
                args![
                    "Huh...?",
                    ".........!!",
                    "Hahahahaha!",
                    "Come back for",
                    "your beating,",
                    "eh, Shinokas?!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Look...!",
                    "I'm not Shinokas,",
                    "okay? How can you",
                    "forget what he looks",
                    "like or how old he is?",
                    "I'm way younger!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Buender Hikeman", args!["What...?"])?;
            ctx.next()?;
            ctx.lines_as("Buender Hikeman", args!["................"])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
            ctx.next()?;
            ctx.lines_as("Buender Hikeman", args!["Uhhhh......"])?;
            ctx.next()?;
            ctx.lines_as("Buender Hikeman", args!["Huh."])?;
            ctx.next()?;
            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["And...", "I'm a cute girl!", "Shinokas is male", "and kind of..."],
                )?;
                ctx.next()?;
                ctx.lines_as("Buender Hikeman", args!["What...?", "IMPOSSIBLE!"])?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...", "......"])?;
                ctx.next()?;
                ctx.lines_as("Buender Hikeman", args!["...", "......"])?;
                ctx.next()?;
            }
            ctx.lines_as("Buender Hikeman", args!["It seems...", "I've made a", "huge mistake."])?;
            ctx.next()?;
            ctx.lines_as(
                "Buender Hikeman",
                args!["Ever since the accident, people have said that I haven't been the same. Maybe they're right."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Buender Hikeman",
                args![
                    "I keep making the same",
                    "mistakes, so maybe they're",
                    "right about me getting senile.",
                    "Did you come just to clear",
                    "up this misunderstanding?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Um...", "Actually..."],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Tell Hikeman about Shinokas's death.:Don't notify Hikeman.")])? {
                1 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Shinokas died", "a while ago in", "Einbroch."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Buender Hikeman", args!["Wh-what...?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Shinokas was killed by",
                            "some men. He thinks they",
                            "may have been the ones",
                            "who hired you guys. In the",
                            "end, he was betrayed too..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Buender Hikeman", args!["I...", "I see..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Shinokas may have",
                            "gotten some money,",
                            "but he spent the rest",
                            "of his life in hiding,",
                            "being hunted down."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Buender Hikeman",
                        args![
                            "Nothing's left.",
                            "I've got nothing",
                            "to look foward to.",
                            "I was living only to",
                            "avenge my friends..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Buender Hikeman",
                        args!["Please...", "Just go back to", "wherever you came", "from. Leave me alone..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "^333333Did Hikeman really want",
                            "revenge on Shinokas or did",
                            "he want to hear him out since",
                            "they used to be close friends?^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "^333333If I mention that",
                            "what they found was",
                            "the Ymir Heart Piece,",
                            "Hikeman might end up",
                            "getting hunted down, so",
                            "I better not say anything.^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "^333333Why are those men",
                            "so obsessed with that",
                            "Ymir Heart Piece? Is it",
                            "really worth this kind of",
                            "cruelty? Whoever they are,",
                            "their intentions can't be good."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from("ein_hicman"), Val::from(255)])?;
                    ctx.call(Function::CompleteQuest, vec![Val::from(2078)])?;
                    ctx.var("shinokas_quest").set(Val::from(11))?;
                    {
                        if ctx.var("BaseLevel").get()?.number()? < 70 {
                            ctx.call(Function::GetExperience, vec![Val::from(100000), Val::from(80000)])?;
                        } else if (ctx.var("BaseLevel").get()?.number()? > 69 && ctx.var("BaseLevel").get()?.number()? < 80) {
                            ctx.call(Function::GetExperience, vec![Val::from(300000), Val::from(100000)])?;
                        } else if (ctx.var("BaseLevel").get()?.number()? > 79 && ctx.var("BaseLevel").get()?.number()? < 90) {
                            ctx.call(Function::GetExperience, vec![Val::from(500000), Val::from(300000)])?;
                        } else {
                            ctx.call(Function::GetExperience, vec![Val::from(700000), Val::from(500000)])?;
                        }
                    }
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Yeah...",
                            "That' right.",
                            "I wanted to clear up",
                            "this misunderstanding",
                            "so you could calm down,",
                            "even if it's just a little bit."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Buender Hikeman",
                        args![
                            "Well, you don't have",
                            "to worry so much about",
                            "my stress. I find that the",
                            "winds that pass through",
                            "this town to be very relaxing."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Buender Hikeman",
                        args![
                            "Each time the wind",
                            "blows by, my vision blurs,",
                            "my memories haze and all",
                            "of my hatred just drifts away."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Buender Hikeman",
                        args![
                            "Without the wind, I'd only",
                            "have my hatred towards ^FF0000him^000000.",
                            "Maybe it's my only reason for",
                            "living and maybe I'm lonely,",
                            "but it's too late to feel",
                            "sorry for myself now."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from("ein_hicman"), Val::from(255)])?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.lines(args![
                "^3355FFHikeman is dozing",
                "off in his chair. Judging",
                "from the look of discomfort",
                "on his face, he seems to be",
                "having a nightmare."
            ])?;
            ctx.call(Function::Cutin, vec![Val::from("ein_hicman"), Val::from(255)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn buender_hikeman_ein(ctx: &Ctx) -> Script {
    buender_hikeman_ein_body(ctx, Vec::new()).map(|_| ())
}
