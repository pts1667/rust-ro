use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn law_enforcement_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn law_enforcement(ctx: &Ctx) -> Script {
    law_enforcement_body(ctx, Vec::new()).map(|_| ())
}

fn law_enforcement_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    ctx.call(Function::MapAnnounce, vec![Val::from("lighthalzen"), Val::from("Attention, citizens. Our security has been breached and the city is in Gangster Alert status. Please find shelter immediately!"), ctx.constant("BC_MAP")?])?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    l_i = Val::from(0);
    'l1: loop {
        if !(l_i.clone().number()? < 30) {
            break 'l1;
        }
        'b1: {
            ctx.call(
                Function::Monster,
                vec![
                    Val::from("lighthalzen"),
                    Val::from(0),
                    Val::from(0),
                    Val::from("Gangster"),
                    Val::from(1592),
                    Val::from(1),
                    Val::from("Law Enforcement::OnMyMobDead"),
                ],
            )?;
        }
        l_i = (l_i.clone() + Val::from(1));
    }
    return Err(Stop::End);
}

pub fn law_enforcement_onenable(ctx: &Ctx) -> Script {
    law_enforcement_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn law_enforcement_ontimer220000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::KillMonster,
        vec![Val::from("lighthalzen"), Val::from("Law Enforcement::OnMyMobDead")],
    )?;
    ctx.call(Function::MapAnnounce, vec![Val::from("lighthalzen"), Val::from("Attention, citizens. Our law enforcement department has successfully contained the situation. Alert status has been canceled."), ctx.constant("BC_MAP")?])?;
    ctx.var("$@lhz_gangster_alert").set(Val::from(0))?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn law_enforcement_ontimer220000(ctx: &Ctx) -> Script {
    law_enforcement_ontimer220000_body(ctx, Vec::new()).map(|_| ())
}

fn law_enforcement_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn law_enforcement_onmymobdead(ctx: &Ctx) -> Script {
    law_enforcement_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn guard_lhz01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CountItem, vec![Val::from(7350)])?.number()? > 0 {
        ctx.lines_as(
            "Guard",
            args![
                "Hold it right th--!",
                "Oh. I'm sorry. I didn't",
                "realize you were carrying",
                "a pass and had authorization."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("lighthalzen"), Val::from(303), Val::from(229)])?;
        return Err(Stop::End);
    }
    if ctx.var("$@lhz_gangster_alert").get()?.number()? >= 100 {
        ctx.lines_as(
            "Guard",
            args![
                "Recently too many people",
                "have been traveling between",
                "Uptown and the ghetto, so",
                "we've heightened security",
                "around here. But how can",
                "so many sneak through us?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("$@lhz_gangster_alert").get()?.number()? > 14 {
        ctx.lines_as(
            "Guard",
            args![
                "Recently too many people",
                "have been traveling between",
                "Uptown and the ghetto, so",
                "we've heightened security",
                "around here. But how can",
                "so many sneak through us?"
            ],
        )?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Law Enforcement::OnEnable")])?;
        ctx.var("$@lhz_gangster_alert").set(Val::from(100))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 22
        || ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 2)
    {
        ctx.lines_as("Guard", args!["Zzzz... Zzz...", "ZZZzzzzzzzzzz..."])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThis guard is",
            "dozing off, so this is",
            "the perfect opportunity",
            "to sneak past him.^000000"
        ])?;
        ctx.close_window()?;
        ctx.var("$@lhz_gangster_alert")
            .set((ctx.var("$@lhz_gangster_alert").get()? + Val::from(1)))?;
        ctx.call(Function::Warp, vec![Val::from("lighthalzen"), Val::from(303), Val::from(229)])?;
        return Err(Stop::End);
    }
    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])? == 3 {
        ctx.lines(args![
            "^3355FFThe guard seems distracted",
            "and is looking elsewhere. Now's",
            "your chance to sneak past him!^000000"
        ])?;
        ctx.close_window()?;
        ctx.var("$@lhz_gangster_alert")
            .set((ctx.var("$@lhz_gangster_alert").get()? + Val::from(1)))?;
        ctx.call(Function::Warp, vec![Val::from("lighthalzen"), Val::from(303), Val::from(229)])?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Guard",
            args![
                "Hold it right there!",
                "I can't permit anyone",
                "to enter the slums.",
                "Go back to where you",
                "came from, adventurer!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn guard_lhz01(ctx: &Ctx) -> Script {
    guard_lhz01_body(ctx, Vec::new()).map(|_| ())
}

fn guard_lhz02_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CountItem, vec![Val::from(7350)])?.number()? > 0 {
        ctx.lines_as(
            "Guard",
            args![
                "Hold it right th--!",
                "Oh. I'm sorry. I didn't",
                "realize you were carrying",
                "a pass and had authorization."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("lighthalzen"), Val::from(260), Val::from(199)])?;
        return Err(Stop::End);
    }
    if ctx.var("$@lhz_gangster_alert").get()?.number()? >= 100 {
        ctx.lines_as(
            "Guard",
            args![
                "Recently too many people",
                "have been traveling between",
                "Uptown and the ghetto, so",
                "we've heightened security",
                "around here. But how can",
                "so many sneak through us?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("$@lhz_gangster_alert").get()?.number()? > 14 {
        ctx.lines_as(
            "Guard",
            args![
                "Recently too many people",
                "have been traveling between",
                "Uptown and the ghetto, so",
                "we've heightened security",
                "around here. But how can",
                "so many sneak through us?"
            ],
        )?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Law Enforcement::OnEnable")])?;
        ctx.var("$@lhz_gangster_alert").set(Val::from(100))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 22
        || ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 2)
    {
        ctx.lines_as("Guard", args!["Zzzz... Zzz...", "ZZZzzzzzzzzzz..."])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThis guard is",
            "dozing off, so this is",
            "the perfect opportunity",
            "to sneak past him.^000000"
        ])?;
        ctx.close_window()?;
        ctx.var("$@lhz_gangster_alert")
            .set((ctx.var("$@lhz_gangster_alert").get()? + Val::from(1)))?;
        ctx.call(Function::Warp, vec![Val::from("lighthalzen"), Val::from(260), Val::from(199)])?;
        return Err(Stop::End);
    }
    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])? == 3 {
        ctx.lines(args![
            "^3355FFThe guard seems distracted",
            "and is looking elsewhere. Now's",
            "your chance to sneak past him!^000000"
        ])?;
        ctx.close_window()?;
        ctx.var("$@lhz_gangster_alert")
            .set((ctx.var("$@lhz_gangster_alert").get()? + Val::from(1)))?;
        ctx.call(Function::Warp, vec![Val::from("lighthalzen"), Val::from(260), Val::from(199)])?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Guard",
            args![
                "Hold it right there!",
                "I can't permit anyone to",
                "enter Uptown Lighthalzen!",
                "If you don't have a pass,",
                "then move on out of here!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn guard_lhz02(ctx: &Ctx) -> Script {
    guard_lhz02_body(ctx, Vec::new()).map(|_| ())
}

fn fishbone_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_player_name_s = Val::from("");
    if runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(512))?.is_true() {
        ctx.lines_as(
            "Fishbone",
            args![
                "Oh hey, it's you!",
                "Now, listen. If you wanna",
                "get into Regenschirm again,",
                "you gotta enter that Sewer Pipe",
                "that's like, northeast from me.",
                "It's gross, but you gotta..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Fishbone",
            args![
                "Anyway, good luck in",
                "whatever it is you're doing",
                "here in Lighthalzen. Oh, and",
                "be real careful! Something",
                "shadier than me is going",
                "on in this big city~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("lhz_boss").get()? == 6 || ctx.var("BaseLevel").get()?.number()? >= 60) {
        if ctx.var("lhz_sincube").get()? == 0 {
            ctx.lines_as(
                "Fishbone",
                args![
                    "What...?",
                    "Is there something",
                    "on my face? Quit looking",
                    "at me and let me do my work."
                ],
            )?;
            ctx.var("lhz_sincube").set(Val::from(1))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.var("lhz_sincube").get()? == 1 {
            ctx.lines_as(
                "Fishbone",
                args![
                    "Hey. Hey you.",
                    "You're still looking",
                    "at me. Quit it. I'm busy",
                    "here and you're bothering me."
                ],
            )?;
            ctx.var("lhz_sincube").set(Val::from(2))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.var("lhz_sincube").get()? == 2 {
            ctx.lines_as(
                "Fishbone",
                args![
                    "Seriously, you are",
                    "starting to really get",
                    "on my nerves! What could",
                    "you possibly want from me?!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_player_name_s.clone(),
                args![
                    "Nothing really.",
                    "I just feel like",
                    "watching you work.",
                    "What exactly are you",
                    "doing anyway, huh?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fishbone",
                args![
                    "Hey! If I could just show",
                    "you what I'm doing, I wouldn't",
                    "be so uptight about you looking",
                    "at what I'm doing, now would I?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_player_name_s.clone(),
                args!["Nah...", "I just think", "you're uptight", "in general."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fishbone",
                args![
                    "Muthafruit!",
                    "So you're just",
                    "gonna sit and watch",
                    "me all day?! Fine! Then",
                    "I'll just ignore you, jerk!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fishbone",
                args![
                    "...",
                    "......",
                    "Damn it!",
                    "This is really",
                    "ticking me off! Why",
                    "don't you go away?!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_player_name_s.clone(),
                args![
                    "Well, I was also",
                    "wondering why you're",
                    "working in a place that's",
                    "so, um... depressing."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fishbone",
                args![
                    "Alright, fine!",
                    "But once I tell you",
                    "what I do here, you're",
                    "outta here! Okay? Now,",
                    "your lips are frickin' sealed.",
                    "See, what I do is kinda illegal."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fishbone",
                args![
                    "I provide routes that let",
                    "people enter Lighthalzen,",
                    "or even go anywhere in this",
                    "city, without authorization.",
                    "Hell, I could even get you",
                    "into that Laboratory..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_player_name_s.clone(),
                args!["W-wait!", "Laboratory?!", "Are you serious?", "Can you get me", "into that place?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fishbone",
                args![
                    "Sonuva--You're not",
                    "gonna leave me alone,",
                    "are you? Look, you seem",
                    "okay, even if you are kinda",
                    "stubborn, kid. But I don't trust",
                    "or serve strangers, period."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_player_name_s.clone(),
                args![
                    "But isn't there",
                    "anything I can do",
                    "so you can help me",
                    "get into the Laboratory?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fishbone",
                args![
                    "No way, no how.",
                    "No matter how much you",
                    "beg, I can't take a chance",
                    "and just trust anyone who",
                    "wants to know a secret",
                    "route. Now get outta here!"
                ],
            )?;
            ctx.var("lhz_sincube").set(Val::from(3))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.var("lhz_sincube").get()? == 3 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Please...",
                    "Please tell me",
                    "how I can get inside",
                    "the Laboratory. I'm...",
                    "I'm begging you!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fishbone",
                args![
                    "No! Now shaddup",
                    "and stop begging, okay?",
                    "You're only embarassing",
                    "yourself! Good grief..."
                ],
            )?;
            ctx.var("lhz_sincube").set(Val::from(4))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.var("lhz_sincube").get()? == 4 {
            l_player_name_s = ctx.call(Function::StrCharInfo, vec![Val::from(0)])?;
            ctx.lines_as(
                l_player_name_s.clone(),
                args![
                    "Why won't you",
                    "heeeeelp meee?",
                    "Pleeeeeeeease~",
                    "You're the only one",
                    "who knoooooows~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fishbone",
                args![
                    "Yeesh, you're a nutty",
                    "kid. Look, ''no'' means",
                    "''no.'' That's it. That's",
                    "final. How many times do",
                    "you gotta make me spell it out?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_player_name_s.clone(),
                args![
                    "But there's gotta",
                    "be something I can",
                    "do so you'll help me?",
                    "Th-that's the way i-it",
                    "always w-works. You",
                    "can't just-- You c-can't..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fishbone",
                args![
                    "Alright, you got me.",
                    "But if I'm gonna risk",
                    "my life to help you get",
                    "into that Laboratory I want",
                    "something in return. Okay?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fishbone",
                args![
                    "All you need to do is",
                    "come back here with",
                    "^FF000071,381,305,294,921,000 zeny^000000.",
                    "Then I'll give you all the help",
                    "you need. Alright, good luck",
                    "to you, brave adventurer."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_player_name_s.clone(),
                args![
                    "You're joking right?!",
                    "There's no way I can ever",
                    "get that much zeny! I'd...",
                    "I'd have to at least take",
                    "over the world or, or..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fishbone",
                args![
                    "Good point.",
                    "Fine, maybe that",
                    "price is a little high.",
                    "I'll just cut it in half,",
                    "then. Still want my",
                    "help? Then bring me..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Fishbone", args![" ", " ", "...^FF000020 Jellopy^000000."])?;
            ctx.next()?;
            ctx.lines_as("Fishbone", args!["Gosh.", "It's like I'm", "doing this for free..."])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Okay."), Val::from("No! It's impossible!")],
            )?) == 1
            {
                ctx.lines_as(
                    "Fishbone",
                    args![
                        "Alright, then",
                        "bring me back a",
                        "total of 20 Jellopy.",
                        "But you gotta have",
                        "exactly 20 Jellopy on",
                        "you, okay? Seeya pal~"
                    ],
                )?;
                ctx.var("lhz_sincube").set(Val::from(5))?;
                ctx.call(Function::SetQuest, vec![Val::from(12014)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Fishbone",
                args![
                    "What are you--",
                    "Okay, now you're the",
                    "one who's joking around.",
                    "Look, it's either bring me",
                    "20 Jellopy or 71,381,3--",
                    "whatever number I said-- zeny!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.var("lhz_sincube").get()? == 5 {
            ctx.lines_as("Fishbone", args!["So...", "Did you bring", "the stuff?"])?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[
                    Val::from("I'm still working on it."),
                    Val::from("Here you go!"),
                    Val::from("Um... Stuff?"),
                ],
            )? {
                1 => {
                    ctx.lines_as(
                        "Fishbone",
                        args![
                            "Still working on it?",
                            "Okay, I know I didn't",
                            "ask you for very much,",
                            "but gimme some respect",
                            "and get serious about it!",
                            "It's 20 friggin' Jellopies!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    if ctx.call(Function::CountItem, vec![Val::from(909)])?.number()? < 20 {
                        ctx.lines_as(
                            "Fishbone",
                            args![
                                "Uh...",
                                "I said 20 Jellopies,",
                                "didn't I? This ain't",
                                "enough pal, so go out",
                                "and get some more!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Fishbone",
                        args![
                            "Ooh. Hey, good work.",
                            "Okay, I can see you're",
                            "the reliable type. A deal",
                            "is a deal, so I'll tell you",
                            "how you can get inside",
                            "that Laboratory."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Fishbone",
                        args![
                            "Now, if you're wondering",
                            "why I don't got much qualm",
                            "against helping you, it's",
                            "because I used to work in",
                            "the Laboratory... But then",
                            "they laid me off unfairly."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Fishbone",
                        args![
                            "Anyway, anything I can",
                            "do to cause them trouble",
                            "is good in my book. Let me",
                            "get some stuff ready and",
                            "then I'll tell you what you",
                            "really need to know."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(909), Val::from(20)])?;
                    ctx.var("lhz_sincube").set(Val::from(6))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as(
                        "Fishbone",
                        args![
                            "What the...?",
                            "How could you forget",
                            "something like that?",
                            "Anyway, I asked you to",
                            "bring 20 Jellopies."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if ctx.var("lhz_sincube").get()? == 6 {
            ctx.mes("[Fishbone]")?;
            if ctx.var("lhz_boss").get()? == 6 {
                ctx.lines(args![
                    "way to get into Regenschrim",
                    "Lab. However, I do know of",
                    "this secret maze that should",
                    "get you there. Still, if you're",
                    "willing and ready to go..."
                ])?;
            } else {
                ctx.lines(args![
                    "Okay...",
                    "Are you ready now?",
                    "I'm gonna send you",
                    "someplace where you",
                    "can find the Lab entrance."
                ])?;
            }
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Not yet."), Val::from("Yes.")])?) == 1 {
                ctx.lines_as(
                    "Fishbone",
                    args!["Eh...?", "Alright, it's", "not a problem.", "Just take your time."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Fishbone", args!["Great!", "Okay then,", "here we go!"])?;
            ctx.close_window()?;
            ctx.var("lhz_sincube").set(Val::from(7))?;
            let subject2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
            if subject2 == 1 {
                ctx.call(Function::Warp, vec![Val::from("lhz_cube"), Val::from(67), Val::from(193)])?;
            } else if subject2 == 2 {
                ctx.call(Function::Warp, vec![Val::from("lhz_cube"), Val::from(66), Val::from(136)])?;
            } else if subject2 == 3 {
                ctx.call(Function::Warp, vec![Val::from("lhz_cube"), Val::from(66), Val::from(74)])?;
            }
            return Err(Stop::End);
        }
        if ctx.var("lhz_sincube").get()?.number()? < 10 {
            ctx.lines_as("Fishbone", args!["Hm, you must not have", "accomplished whatever"])?;
            if ctx.var("lhz_boss").get()? == 6 {
                ctx.lines(args!["it is you need to do in the", "Regenschirm Laboratory yet."])?;
            } else {
                ctx.lines(args!["it was you were doing", "in the Laboratory. Do"])?;
            }
            ctx.lines(args!["you want me to send", "you there again?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("No."), Val::from("Yes.")])?) == 1 {
                ctx.lines_as(
                    "Fishbone",
                    args![
                        "Geez, you must have",
                        "really had a hard time",
                        "there. Okay, well, when",
                        "you're ready to go there,",
                        "just let me know, got it?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Fishbone",
                args![
                    "Got a lot of",
                    "spirit in you,",
                    "don't you? Heh!",
                    "I like you style~",
                    "Get ready, 'cuz",
                    "here we go...!"
                ],
            )?;
            ctx.close_window()?;
            ctx.var("lhz_sincube").set(Val::from(7))?;
            let subject3 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
            if subject3 == 1 {
                ctx.call(Function::Warp, vec![Val::from("lhz_cube"), Val::from(67), Val::from(193)])?;
            } else if subject3 == 2 {
                ctx.call(Function::Warp, vec![Val::from("lhz_cube"), Val::from(66), Val::from(136)])?;
            } else if subject3 == 3 {
                ctx.call(Function::Warp, vec![Val::from("lhz_cube"), Val::from(66), Val::from(74)])?;
            }
            return Err(Stop::End);
        }
        if ctx.var("lhz_sincube").get()? == 10 {
            ctx.lines_as(
                "Fishbone",
                args![
                    "Hey, you came back!",
                    "Good, I was starting",
                    "to get a little worried",
                    "about what happened to",
                    "you. So did you get what",
                    "you wanted over there?"
                ],
            )?;
            if ctx.call(Function::IsBeginQuest, vec![Val::from(12014)])? == 1 {
                ctx.call(Function::CompleteQuest, vec![Val::from(12014)])?;
            }
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Yeah...", "I hope so,", "anyway."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fishbone",
                args![
                    "Good! Whatever you did,",
                    "I hope it messes them up big",
                    "time! I usedta be a respected",
                    "scientist for Regenschirm till",
                    "they laid me off! Serves those",
                    "ungrateful jerkoffs right!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fishbone",
                args![
                    "Then again, my current",
                    "line of work seems to suit",
                    "me much better. And I don't",
                    "gotta worry about formulas",
                    "and algorithms anymore."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fishbone",
                args![
                    "Oh yeah, the place you",
                    "just went to? There's a",
                    "secret path the Laboratory",
                    "there. If you want, I can",
                    "send you back there. So",
                    "what do you say?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Sure"), Val::from("No thanks~")])?) == 1 {
                ctx.lines_as("Fishbone", args!["Okay then, here", "we go! Good luck", "to you, buddy."])?;
                ctx.close_window()?;
                let subject4 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                if subject4 == 1 {
                    ctx.call(Function::Warp, vec![Val::from("lhz_cube"), Val::from(67), Val::from(193)])?;
                } else if subject4 == 2 {
                    ctx.call(Function::Warp, vec![Val::from("lhz_cube"), Val::from(66), Val::from(136)])?;
                } else if subject4 == 3 {
                    ctx.call(Function::Warp, vec![Val::from("lhz_cube"), Val::from(66), Val::from(74)])?;
                }
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Fishbone",
                args![
                    "Alright, it's your",
                    "choice. Oh, and that",
                    "pass you have should",
                    "let you into the Laboratory",
                    "anyway. Good luck to you, pal."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Fishbone",
            args!["Heya pal!", "It's been a while.", "You doin' good? I'm", "just peachy, thanks~"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("BaseLevel").get()?.number()? < 60 {
        ctx.lines(args![
            "Hey kid, get",
            "outta here! Can't",
            "you tell I'm dealin'",
            "in something shady",
            "here? This is no place",
            "for baby faced guys like you!"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_boss").get()?.number()? > 6 {
        ctx.lines_as(
            "Fishbone",
            args!["Heya pal!", "It's been a while.", "You doin' good? I'm", "just peachy, thanks~"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn fishbone(ctx: &Ctx) -> Script {
    fishbone_body(ctx, Vec::new()).map(|_| ())
}

fn bundle_of_files_cube_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_sincube").get()?.number()? < 7 {
        ctx.lines(args!["^3355FFThere are a bunch", "of files scattered", "on the ground.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_sincube").get()?.number()? < 10 {
        if ctx.var("lhz_secret01").get()?.number()? < 1 {
            ctx.lines(args![
                "^3355FFThere are a bunch",
                "of files scattered",
                "on the ground. They",
                "seem to contain all",
                "sorts of information,",
                "but they're all mixed up.^000000"
            ])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Rummage through the files."), Val::from("Cancel")],
            )?) == 1
            {
                ctx.lines(args![
                    "^3355FFWhile you are",
                    "rummaging through",
                    "the files, a Red Key",
                    "drops to the ground",
                    "with a clink. You decide",
                    "to keep this ^000000Red Key^3355FF.^000000"
                ])?;
                ctx.var("lhz_secret01").set(Val::from(1))?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FFThis is the place",
            "where you found the",
            "^000000Red Key^3355FF while you were",
            "looking through the files",
            "scattered on the ground.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFThis is the place",
        "where you found the",
        "^000000Red Key^3355FF while you were",
        "looking through the files",
        "scattered on the ground.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bundle_of_files_cube(ctx: &Ctx) -> Script {
    bundle_of_files_cube_body(ctx, Vec::new()).map(|_| ())
}

fn picture_cube_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    let mut l_number_rand = Val::from(0);
    if ctx.var("lhz_secret01").get()?.number()? < 2 {
        ctx.lines(args![
            "^3355FFThis picture hanging",
            "on the wall catches",
            "your attention for some",
            "inexplicably strange reason.^000000"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Look behind picture."), Val::from("Cancel")],
        )?) == 1
        {
            ctx.lines(args![
                "^3355FFYou push and pull",
                "with all your strength,",
                "but the picture won't",
                "budge. If it's too hard",
                "to move, there must be",
                "something hidden behind it.^000000"
            ])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_secret01").get()? == 2 {
        ctx.lines(args![
            "^3355FFThis picture hanging",
            "on the wall catches",
            "your attention for some",
            "inexplicably strange reason.^000000"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Look behind picture."), Val::from("Cancel")],
        )?) == 1
        {
            ctx.lines(args![
                "^3355FFYou push and pull",
                "with all your strength,",
                "but this picture is too",
                "hard to move with just",
                "brute strength alone.^000000"
            ])?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_input_s = input;
            if l_input_s.clone() == "Jackknife" {
                ctx.lines(args![
                    "^3355FFYou take the",
                    "Jackknife, thrust",
                    "it under the picture",
                    "and twist it in order to",
                    "pry the picture off the wall.^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFBehind the picture, you",
                    "find that the following",
                    "numbers are written:^000000",
                    " "
                ])?;
                l_number_rand = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                if l_number_rand.clone() == 1 {
                    ctx.mes("4 3 2 9 1 6 8 2 7")?;
                    ctx.var("lhz_secret01").set(Val::from(3))?;
                } else if l_number_rand.clone() == 2 {
                    ctx.mes("3 6 4 1 2 8 7 1 5")?;
                    ctx.var("lhz_secret01").set(Val::from(4))?;
                } else {
                    ctx.mes("4 9 3 7 6 2 8 6 6")?;
                    ctx.var("lhz_secret01").set(Val::from(5))?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines(args![
                "^3355FFUnfortunately,",
                "doing that apparently",
                "wasn't enough to move",
                "the picture. Perhaps you",
                "should try something else.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_secret01").get()?.number()? < 6 {
        ctx.lines(args![
            "^3355FFThe following numbers",
            "were written behind the",
            "picture. If they were hidden,",
            "these numbers must have",
            "some kind of importance.^000000",
            " "
        ])?;
        if ctx.var("lhz_secret01").get()? == 3 {
            ctx.mes("4 3 2 9 1 6 8 2 7")?;
        } else if ctx.var("lhz_secret01").get()? == 4 {
            ctx.mes("3 6 4 1 2 8 7 1 5")?;
        } else if ctx.var("lhz_secret01").get()? == 5 {
            ctx.mes("4 9 3 7 6 2 8 6 6")?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args!["^3355FFThere is a picture", "hanging on the wall.^000000"])?;
    ctx.next()?;
    if Val::from(runtime::select_values(
        ctx,
        &[Val::from("Look beneath picture."), Val::from("Cancel")],
    )?) == 1
    {
        ctx.lines(args![
            "^3355FFThere are some",
            "numbers behind the",
            "picture, but now there is",
            "no need to memorize them.^000000"
        ])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn picture_cube(ctx: &Ctx) -> Script {
    picture_cube_body(ctx, Vec::new()).map(|_| ())
}

fn drawer_cube_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.var("lhz_secret01").get()?.number()? < 1 {
        ctx.lines(args![
            "^3355FFThe drawer here",
            "looks interesting,",
            "but it's locked and",
            "you can't open it.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_secret01").get()? == 1 {
        ctx.lines(args![
            "^3355FF The drawer here",
            "looks interesting, but",
            "it's locked. Hopefully, you",
            "can figure how to open it.^000000"
        ])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s.clone() == "Red Key" {
            ctx.lines(args![
                "^3355FFYou insert the Red Key",
                "into the lock and open the",
                "drawer. Inside, you find a",
                "^000000Jackknife^3355FF that you decide",
                "to take. After all, it might",
                "be handy sometime.^000000"
            ])?;
            ctx.var("lhz_secret01").set(Val::from(2))?;
        } else {
            ctx.lines(args![
                "^3355FFUnfortunately, you",
                "can't open or break",
                "the lock on the drawer",
                "by doing that. You need",
                "to try something else.^000000"
            ])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFThis is the drawer",
        "in which you found the",
        "^000000Jackknife^3355FF. It is now empty",
        "and devoid of purpose.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn drawer_cube(ctx: &Ctx) -> Script {
    drawer_cube_body(ctx, Vec::new()).map(|_| ())
}

fn chest_cube_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_sincube").get()?.number()? < 7 {
        ctx.lines(args![
            "^3355FFYou've found a chest",
            "with an axe laid on",
            "top. The axe's purpose",
            "is completely utilitarian",
            "and isn't suited for battle.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_sincube").get()? == 7 {
        ctx.lines(args![
            "^3355FFYou've found a chest,",
            "but more importantly,",
            "there is a utility Axe",
            "laid on top of it.^000000"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Take the utility Axe."), Val::from("Cancel")],
        )?) == 1
        {
            ctx.lines(args![
                "^3355FFWithout shame or",
                "an ounce of guilt, you",
                "pick up the utility ^000000Axe^3355FF",
                "and claim it as your own.^000000"
            ])?;
            ctx.var("lhz_sincube").set(Val::from(8))?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFThis is just a normal",
        "chest. There used to be",
        "a utility ^000000Axe^3355FF on top of",
        "it until you picked it up.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn chest_cube(ctx: &Ctx) -> Script {
    chest_cube_body(ctx, Vec::new()).map(|_| ())
}

fn barrel_cube_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_input = Val::from(0);
    let mut l_input_s = Val::from("");
    let mut l_number_line = Val::from(0);
    let mut l_numbers: Vec<Val> = Vec::new();
    if ctx.var("lhz_sincube").get()?.number()? < 8 {
        ctx.lines(args!["^3355FFYou have", "found a shabbily", "constructed barrel.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_sincube").get()? == 8 {
        ctx.lines(args![
            "^3355FFYou have",
            "found a shabbily",
            "constructed barrel.",
            "You sense that there's",
            "something inside, but",
            "you need something to",
            "smash the barrel open.^000000"
        ])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s.clone() == "Axe" {
            ctx.lines(args![
                "^3355FFAxe in hand, you lift",
                "it above your head and",
                "swing it downwards,",
                "smashing off the top of",
                "the barrel. Inside, you find a",
                "box topped with a metal plate.^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFThe numbers one",
                "through nine are",
                "etched on the surface",
                "of the metal plate on",
                "top of the box you found,",
                "like some kind of keypad...^000000"
            ])?;
            ctx.var("lhz_sincube").set(Val::from(9))?;
        } else {
            ctx.lines(args![
                "^3355FFDoing that probably won't",
                "break open this keg. You'll",
                "need to try something else.^000000"
            ])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_sincube").get()? == 9 {
        if ctx.var("lhz_secret01").get()?.number()? < 3 {
            ctx.lines(args![
                "^3355FFYou decide to enter",
                "some numbers into the",
                "metallic keypad. Remember,",
                "you can only enter single",
                "digit numbers at one time...^000000"
            ])?;
            ctx.next()?;
            l_i = Val::from(0);
            'l1: loop {
                if !(l_i.clone().number()? < 9) {
                    break 'l1;
                }
                'b1: {
                    let (input, status) = runtime::input_number(ctx, None, None)?;
                    l_input = input;
                }
                l_i = (l_i.clone() + Val::from(1));
            }
            ctx.mes("^3355FFNothing happened...^000000")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.var("lhz_secret01").get()?.number()? <= 5 {
            let subject2 = ctx.var("lhz_secret01").get()?;
            if subject2 == 3 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_numbers, &Val::from(base + 0), Val::from(4), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 1), Val::from(3), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 2), Val::from(2), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 3), Val::from(9), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 4), Val::from(1), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 5), Val::from(6), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 6), Val::from(8), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 7), Val::from(2), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 8), Val::from(7), false);
            } else if subject2 == 4 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_numbers, &Val::from(base + 0), Val::from(3), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 1), Val::from(6), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 2), Val::from(4), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 3), Val::from(1), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 4), Val::from(2), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 5), Val::from(8), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 6), Val::from(7), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 7), Val::from(1), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 8), Val::from(5), false);
            } else if subject2 == 5 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_numbers, &Val::from(base + 0), Val::from(4), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 1), Val::from(9), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 2), Val::from(3), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 3), Val::from(7), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 4), Val::from(6), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 5), Val::from(2), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 6), Val::from(8), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 7), Val::from(6), false);
                runtime::local_set(&mut l_numbers, &Val::from(base + 8), Val::from(6), false);
            }
            ctx.lines(args![
                "^3355FFYou decide to enter",
                "some numbers into the",
                "numberpad etched on the",
                "metal plate on top of the box",
                "you found inside the keg.^000000"
            ])?;
            ctx.next()?;
            l_i = Val::from(0);
            'l3: loop {
                if !(l_i.clone().number()? < 9) {
                    break 'l3;
                }
                'b3: {
                    let (input, status) = runtime::input_number(ctx, None, None)?;
                    l_input = input;
                    if l_input.clone().loosely_equals(&runtime::local_get(&l_numbers, &l_i.clone(), false)) {
                        l_number_line = (l_number_line.clone() + Val::from(1));
                    }
                }
                l_i = (l_i.clone() + Val::from(1));
            }
            if l_number_line.clone() == 9 {
                ctx.lines(args![
                    "^3355FFThe metal plate slides",
                    "open and you find a key",
                    "Key inside the box. You",
                    "to keep this ^000000Yellow Key^3355FF.^000000"
                ])?;
                ctx.var("lhz_secret01").set(Val::from(6))?;
            } else {
                ctx.lines(args![
                    "^3355FFNothing happened.",
                    "It's likely that you did not",
                    "enter the correct numbers.^000000"
                ])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FFThis is the box with",
            "the keypad in which you",
            "found the ^000000Yellow Key^3355FF.",
            "The box is now empty.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFThis is a box",
        "topped with a metal",
        "plate that looks like",
        "a crudely made keypad.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn barrel_cube(ctx: &Ctx) -> Script {
    barrel_cube_body(ctx, Vec::new()).map(|_| ())
}

fn power_generator_cube_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.var("lhz_secret01").get()?.number()? < 6 {
        ctx.lines(args![
            "^3355FFThis is a noisily",
            "operating huge machine",
            "with a front panel that has",
            "a strange mark. There is a",
            "keyhole on the machine",
            "right next to this panel.^000000"
        ])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        ctx.lines(args![
            "^3355FFNothing happened.",
            "You probably need",
            "to find the right key to",
            "insert into the keyhole.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_secret01").get()? == 6 {
        ctx.lines(args![
            "^3355FFThis is a noisily",
            "operating huge machine",
            "with a front panel that has",
            "a strange mark. There is a",
            "keyhole on the machine",
            "right next to this panel.^000000"
        ])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s.clone() == "Yellow Key" {
            ctx.lines(args![
                "^3355FFYou insert the",
                "Yellow Key into the",
                "keyhole and turn it,",
                "causing the machine",
                "to sputter and deactivate.^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFYou notice that the",
                "^000000Status Light ^3355FFnext to the",
                "bed has now turned off.^000000"
            ])?;
            ctx.var("lhz_secret01").set(Val::from(7))?;
        } else {
            ctx.lines(args![
                "^3355FFNothing happened.",
                "You probably need",
                "to find the right key to",
                "insert into the keyhole.^000000"
            ])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("^3355FFIt's a giant machine.^000000")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn power_generator_cube(ctx: &Ctx) -> Script {
    power_generator_cube_body(ctx, Vec::new()).map(|_| ())
}

fn status_light_cube_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_secret01").get()?.number()? < 7 {
        ctx.lines(args![
            "^3355FFThe Status Light is",
            "on. It looks like there's",
            "something inside the",
            "bulb, but you can't get",
            "near it since it generates",
            "incredibly scorching heat.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFNow, if you could",
            "somehow shut down the",
            "Power Generator in this",
            "room, the bulb would be",
            "cool enough for you to touch...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_secret01").get()? == 7 {
        ctx.lines(args!["The Status Light", "is now off and the", "bulb has cooled down."])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Smash the light bulb."), Val::from("Cancel")],
        )?) == 1
        {
            ctx.lines(args![
                "^3355FFYou smash the",
                "Status Light's bulb",
                "and discover another key.",
                "You obtained a ^000000Black Key^3355FF.^000000"
            ])?;
            ctx.var("lhz_secret01").set(Val::from(8))?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args!["^3355FFYou find the remains", "of a broken light bulb.^000000"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn status_light_cube(ctx: &Ctx) -> Script {
    status_light_cube_body(ctx, Vec::new()).map(|_| ())
}

fn desk_cube_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_secret02").get()?.number()? < 2 {
        ctx.lines(args!["^3355FFYou've found", "a completely", "cluttered desk.^000000"])?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("On the desk"), Val::from("Under the desk"), Val::from("Desk drawer")],
        )? {
            1 => {
                ctx.lines(args![
                    "^3355FFVarious documents,",
                    "books and lab equipment",
                    "are scattered on the desk.",
                    "But none of them seem",
                    "all that useful right now.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                if ctx.var("lhz_secret02").get()?.number()? < 1 {
                    ctx.lines(args![
                        "^3355FFUnder this desk, of",
                        "all conceivable places,",
                        "you find a ^000000Short Stick^3355FF",
                        "that you decide to keep.",
                        "You never know when",
                        "you'll need one of those.^000000"
                    ])?;
                    ctx.var("lhz_secret02").set(Val::from(1))?;
                } else {
                    ctx.lines(args![
                        "^3355FFThis is where you",
                        "found your ^000000Short Stick^3355FF.",
                        "Sadly, there are no more",
                        "hidden treasures for you to",
                        "discover beneath this desk.^000000"
                    ])?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines(args![
                    "^3355FFThe desk drawer is",
                    "locked and probably",
                    "with good reason. After all,",
                    "you just tried to invade this",
                    "private and intimate space.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("lhz_secret02").get()? == 2 {
        ctx.lines(args![
            "^3355FFYou're back at the",
            "messy desk which",
            "is probably used by",
            "a high ranking executive",
            "who has someone else",
            "do his desk tidying for him.^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("On the desk"), Val::from("Under the desk"), Val::from("Desk drawer")],
        )? {
            1 => {
                ctx.lines(args![
                    "^3355FFAlas, no matter how much",
                    "you rummage through it,",
                    "the clutter on the desk",
                    "proves to be useless to you.^000000"
                ])?;
                ctx.next()?;
                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                    ctx.lines(args![
                        "^3355FFAn issue of the",
                        "Dancer magazine,",
                        "''Harmonic Lick'' catches",
                        "your eye, but you really",
                        "shouldn't be interested",
                        "in that publication just",
                        "because of the pictures.^000000"
                    ])?;
                } else {
                    ctx.lines(args![
                        "^3355FFAn issue of the female",
                        "entertainment magazine,",
                        "''Magnum Break'' catches",
                        "your eye, but adventurers",
                        "have no time to look at",
                        "pictures of beautiful men.^000000"
                    ])?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args![
                    "^3355FFUnder the desk, you",
                    "discover another Short",
                    "Stick which happens to",
                    "fit perfectly into the other",
                    "Short Stick you found earlier."
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFYou combine the two",
                    "lesser sticks to create",
                    "a stick that is superior",
                    "to the sum of its parts.",
                    "You are now the proud",
                    "bearer of a ^000000Long Stick^3355FF.^000000"
                ])?;
                ctx.var("lhz_secret02").set(Val::from(3))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines(args![
                    "^3355FFThis desk drawer is",
                    "locked shut and probably",
                    "always will be. It's likely",
                    "that nothing really valuable",
                    "is inside, aside from perhaps",
                    "a hip flask or a candy stash."
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    ctx.lines(args![
        "^3355FFThis desk is cluttered",
        "with all sorts of random",
        "objects, but such is its",
        "destiny as office equipment.^000000"
    ])?;
    ctx.next()?;
    match runtime::select_values(
        ctx,
        &[Val::from("On the desk"), Val::from("Under the desk"), Val::from("Desk drawer")],
    )? {
        1 => {
            ctx.lines(args![
                "^3355FFAlas, no matter how much",
                "you rummage through it,",
                "the clutter on the desk",
                "proves to be useless to you.^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFThere's part of a newspaper",
                "here, but it's only the Comics",
                "section which, of course, isn't",
                "informative enough for world",
                "savvy adventurers like you.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines(args![
                "^3355FFThis is where you found",
                "one of two Short Sticks",
                "to make your ^000000Long Stick^3355FF.",
                "There isn't anything else",
                "under here, so it's impossible",
                "to make your stick any longer.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            ctx.lines(args![
                "^3355FFThis desk drawer is",
                "locked very securely.",
                "The owner of this desk",
                "was wise to provide a",
                "measure of drawer security.",
                "But why put sticks under the",
                "desk? This is most curious...^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn desk_cube(ctx: &Ctx) -> Script {
    desk_cube_body(ctx, Vec::new()).map(|_| ())
}

fn bed_cube_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.var("lhz_secret02").get()? == 0 {
        ctx.lines(args![
            "^3355FFYou've found a bed",
            "in which the sheets",
            "are slovenly arranged.^000000"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("On the bed"), Val::from("Under the bed")],
        )?) == 1
        {
            ctx.lines(args![
                "^3355FFNo one's in the bed",
                "and if you climbed in,",
                "you'd just be alone. For",
                "some people, this may",
                "be an immutable truth.^000000"
            ])?;
        } else {
            ctx.lines(args![
                "^3355FFUnder the bed, you",
                "discover a ^000000Short Stick^3355FF",
                "which you decide to",
                "keep. You never know",
                "when certain, seemingly",
                "useless objects will save you.^000000"
            ])?;
            ctx.var("lhz_secret02").set(Val::from(2))?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_secret02").get()? == 1 {
        ctx.lines(args![
            "^3355FFYou've found a bed",
            "in which the sheets",
            "are slovenly arranged.^000000"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("On the bed"), Val::from("Under the bed")],
        )?) == 1
        {
            ctx.lines(args![
                "^3355FFNo one's in the bed",
                "and if you climbed in,",
                "you'd just be alone. For",
                "some people, this may",
                "be an immutable truth.^000000"
            ])?;
        } else {
            ctx.lines(args![
                "^3355FFUnder the bed, you",
                "discover another Short",
                "Stick which happens to",
                "fit perfectly into the other",
                "Short Stick you found earlier."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFYou combine the two",
                "lesser sticks to create",
                "a stick that is superior",
                "to the sum of its parts.",
                "You are now the proud",
                "bearer of a ^000000Long Stick^3355FF.^000000"
            ])?;
            ctx.var("lhz_secret02").set(Val::from(3))?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_secret02").get()? == 2 {
        ctx.lines(args![
            "^3355FFYou've found a bed",
            "in which the sheets",
            "are slovenly arranged.^000000"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("On the bed"), Val::from("Under the bed")],
        )?) == 1
        {
            ctx.lines(args![
                "^3355FFNo one's in the bed",
                "and if you climbed in,",
                "you'd just be alone. For",
                "some people, this may",
                "be an immutable truth.^000000"
            ])?;
        } else {
            ctx.lines(args![
                "^3355FFIn a distant and",
                "dusty corner beneath",
                "the bed, you manage to",
                "spot an object. If only you",
                "could reach it somehow...^000000"
            ])?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_input_s = input;
            if l_input_s.clone() == "Short Stick" {
                ctx.lines(args![
                    "^3355FFYou try to reach",
                    "the object by using",
                    "your Short Stick. After",
                    "a few attempts, you had no",
                    "choice but to admit that your",
                    "stick just wasn't long enough.^000000"
                ])?;
            } else {
                ctx.lines(args![
                    "^3355FFUnfortunately,",
                    "whatever you used to",
                    "try to reach the object",
                    "didn't work. You'll have",
                    "to think of something else.^000000"
                ])?;
            }
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_secret02").get()? == 3 {
        ctx.lines(args![
            "^3355FFYou find a messy",
            "bed that may be more",
            "than meets the eye.^000000"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("On the bed"), Val::from("Under the bed")],
        )?) == 1
        {
            ctx.lines(args![
                "^3355FFNo one's in the bed",
                "and if you climbed in,",
                "you'd just be alone. For",
                "some people, this may",
                "be an immutable truth.^000000"
            ])?;
        } else {
            ctx.lines(args![
                "^3355FFIn a distant and",
                "dusty corner beneath",
                "the bed, you manage to",
                "spot an object. If only you",
                "could reach it somehow...^000000"
            ])?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_input_s = input;
            if l_input_s.clone() == "Long Stick" {
                ctx.lines(args![
                    "^3355FFWith Long Stick in",
                    "hand, you manage to",
                    "reach the object and drag",
                    "it under the bed towards you.",
                    "You now possess the ^000000Cube^3355FF",
                    "that was under the bed.^000000"
                ])?;
                ctx.var("lhz_secret02").set(Val::from(4))?;
            } else {
                ctx.lines(args![
                    "^3355FFUnfortunately,",
                    "whatever you used to",
                    "try to reach the object",
                    "didn't work. You'll have",
                    "to think of something else.^000000"
                ])?;
            }
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFYou've found a bed",
        "in which the sheets",
        "are slovenly arranged.^000000"
    ])?;
    ctx.next()?;
    if Val::from(runtime::select_values(
        ctx,
        &[Val::from("On the bed"), Val::from("Under the bed")],
    )?) == 1
    {
        ctx.lines(args![
            "^3355FFNo one's in the bed",
            "and if you climbed in,",
            "you'd just be alone. For",
            "some people, this may",
            "be an immutable truth.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFThis is where you",
        "managed to find some",
        "sort of strange ^000000Cube^3355FF.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bed_cube(ctx: &Ctx) -> Script {
    bed_cube_body(ctx, Vec::new()).map(|_| ())
}

fn goblet_cube_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_sincube").get()?.number()? < 10 {
        if ctx.var("lhz_secret03").get()? == 0 {
            ctx.lines(args![
                "^3355FFYou see an empty",
                "bottle and a goblet.",
                "It looks like you",
                "missed the party.^000000"
            ])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Examine the goblet."), Val::from("Cancel")],
            )?) == 1
            {
                ctx.lines(args![
                    "^3355FFInside the goblet,",
                    "you find a ^000000Rusty Key^3355FF",
                    "which you decide to keep.",
                    "However, you'll need to get",
                    "rid of that rust somehow.",
                    "Perhaps you can dip the key",
                    "in some corrosive chemical?^000000"
                ])?;
                ctx.var("lhz_secret03").set(Val::from(1))?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FFThis is the goblet where",
            "you found that ^000000Rusty Key^3355FF.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFThis is goblet cup where",
        "you found that ^000000Rusty Key^3355FF.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn goblet_cube(ctx: &Ctx) -> Script {
    goblet_cube_body(ctx, Vec::new()).map(|_| ())
}
