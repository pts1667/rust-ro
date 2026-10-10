use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn laura_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    if !(ctx.var("hg_odin").get()?.is_true()) {
        ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Laura",
            args!["Hm? What are you doing", "here? Ashe, Ashe! Get", "this person out of here!"],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        ctx.call(Function::Warp, vec![Val::from("hu_in01"), Val::from(15), Val::from(76)])?;
        return Err(Stop::End);
    } else {
        if ctx.var("hg_odin").get()? == 1 {
            ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(2)])?;
            ctx.lines_as(
                "Laura",
                args!["Hm? Oh, you must be", "here in response to our", "recruitment notice, yes?"],
            )?;
            if ctx.var("BaseLevel").get()?.number()? < 60 {
                ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(2)])?;
                ctx.lines(args![
                    "Let me take a look at you...",
                    "Oh, you're all skin and bone!",
                    "I can't use someone like you."
                ])?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
            ctx.lines(args![
                "I'm Laura Laurence, the",
                "Rune-Midgarts Kingdom rep",
                "for the Odin Shrine Expedition."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Laura",
                args!["My assistant, Ashe", "Milton, is right over", "there. Say, 'Hello,' Ashe."],
            )?;
            ctx.next()?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_SMILE")?,
                    ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Ashe")])?,
                ],
            )?;
            ctx.lines_as("Ashe", args!["Hello!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Laura",
                args![
                    "As you may already know,",
                    "we're here to uncover relics",
                    "from the Odin Shrine. However,",
                    "the shrine is infested with",
                    "powerful creatures that a weak",
                    "woman like me can never defeat."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(2)])?;
            ctx.lines_as(
                "Laura",
                args![
                    "Would you let a poor",
                    "fragile woman like me go",
                    "in there alone? Someone like",
                    "you must go in there for me.",
                    "Hence, the recruitment notice."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Laura",
                args![
                    "Now, we can't actually pay",
                    "you for your work. In fact,",
                    "we have absolutely nothing to",
                    "give you. But you'd be doing",
                    "a great service your country,",
                    "the Rune-Midgarts' Kingdom."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(2)])?;
            ctx.lines_as(
                "Laura",
                args![
                    "Think about it!",
                    "You'd be making a great",
                    "contribution, helping us",
                    "learn more about the age",
                    "when the gods showed",
                    "themselves to humans!"
                ],
            )?;
            ctx.next()?;
            'b1: {
                let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Nah, forget it.:Yes, I'll do it!")])?);
                let mut matched1 = false;
                let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Laura",
                        args![
                            "W-Wait! Think about",
                            "the importance of learning",
                            "about the age of gods! If you",
                            "don't help us, then who will?",
                            "We need to work together to",
                            "build a better future, right?"
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("But you can't pay me.:Alright, I'll do it.")],
                    )?) == 1
                    {
                        ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(2)])?;
                        ctx.lines_as(
                            "Laura",
                            args![
                                "You're right.",
                                "It's like I said",
                                "before. I don't have",
                                "anything to pay you.",
                                "..............................."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(2)])?;
                        ctx.lines_as(
                            "Laura",
                            args![
                                "Curses! If this project had",
                                "more funding, we'd have more",
                                "than a cheesy recruitment ad,",
                                "and some professional staff",
                                "like those rich Schwarzwald",
                                "Republic boys in the next room!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(2)])?;
                        ctx.lines_as(
                            "Laura",
                            args![
                                "Fine, it's your choice",
                                "whether you want to work",
                                "for us. I'm just disappointed",
                                "that someone like you won't",
                                "help us. I mean, don't you",
                                "have any patriotism at all?"
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(255)])?;
                        return Err(Stop::End);
                    }
                }
                if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Laura",
                        args![
                            "Perfect! Now, you first",
                            "you need to go to the shrine",
                            "and survey the area for your",
                            "own benefit. Talk to the Boatman, and he'll take you there, okay?",
                            "Hurry up and come back soon!"
                        ],
                    )?;
                    ctx.var("hg_odin").set(Val::from(12))?;
                    ctx.call(Function::SetQuest, vec![Val::from(11003)])?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(255)])?;
                    return Err(Stop::End);
                }
            }
        } else {
            if (ctx.var("hg_odin").get()?.number()? > 1 && ctx.var("hg_odin").get()?.number()? < 6) {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Excuse me, but", "what are you guys", "doing over here?"],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(2)])?;
                ctx.lines_as(
                    "Laura",
                    args![
                        "What are we doing...?",
                        "This is the shrine expedition",
                        "office. You know, we're doing",
                        "research on the artifacts found",
                        "in the Odin Shrine on behalf",
                        "of the Rune-Midgarts Kingdom."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("About Alex and Julian:Alright, now I understand.")])? {
                    1 => {
                        ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(2)])?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "These people in the other",
                                "room, Alex and Julian, are",
                                "they also working in this same",
                                "expedition in the Odin Shrine?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(2)])?;
                        ctx.lines_as(
                            "Laura",
                            args![
                                "...Oh. You must be working",
                                "for those rich Schwaltvalt",
                                "Republic kids. I can't work",
                                "with that impudent girl, at",
                                "all, so we're actually working",
                                "in separate offices."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(2)])?;
                        ctx.lines_as(
                            "Laura",
                            args![
                                "Sorry, let me introduce",
                                "myself. I'm Laura Laurence,",
                                "the leader of the Odin Shrine",
                                "expedition for the Rune-Midgarts Kingdom. Ashe Milton is my",
                                "assistant for this excavation."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(
                            Function::Emotion,
                            vec![
                                ctx.constant("ET_SMILE")?,
                                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Ashe")])?,
                            ],
                        )?;
                        ctx.lines_as("Ashe", args!["Hello!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Laura",
                            args![
                                "If you'd rather do",
                                "a service for your",
                                "country and quit working",
                                "for those wealthy snobs, then",
                                "come back here and let me know."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(255)])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(2)])?;
                        ctx.lines_as("Laura", args!["......", ".........", "............"])?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(255)])?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                if ctx.var("hg_odin").get()? == 6 {
                    ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Laura",
                        args![
                            "Hey, I know you...",
                            "You're a volunteer for the",
                            "other office's expedition,",
                            "right? Your timing couldn't",
                            "be more perfect. Why don't",
                            "you help us for a while?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Laura",
                        args![
                            "Basically, I want you",
                            "to spy on that snobby girl,",
                            "Alex of the Schwarzwald",
                            "Republic office, and report",
                            "what she's actually doing to me."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Laura",
                        args![
                            "I have to know...",
                            "I need to know whether",
                            "they discovered ''that thing''",
                            "already. They can't have found",
                            "it before us. It's impossible!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Laura",
                        args![
                            "Anyway, just eavesdrop",
                            "on the Schwaltvalt Office",
                            "by putting your ear really",
                            "close to the door, and listen",
                            "to them talk. Will you do that?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Laura",
                        args!["...............................", "Of course, you're", "going to do it. Right?"],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(2)])?;
                    ctx.lines_as("Laura", args!["RIGHT?!"])?;
                    ctx.next()?;
                    let choice = runtime::select_values(
                        ctx,
                        &[Val::from("Yeah, yeah, whatever.:Sure thing.:I s-s-s-suppose...!:Yes, Ma'am!")],
                    )?;
                    ctx.var("@menu").set(choice)?;
                    ctx.var("hg_odin").set(Val::from(17))?;
                    ctx.call(Function::CompleteQuest, vec![Val::from(11002)])?;
                    ctx.call(Function::SetQuest, vec![Val::from(11006)])?;
                    ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(2)])?;
                    ctx.lines_as("Laura", args!["Good.", "Now do it!"])?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(255)])?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("hg_odin").get()? == 12 {
                        ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(2)])?;
                        ctx.lines_as(
                            "Laura",
                            args![
                                "Ask the Boatman to take",
                                "you to the Odin Shrine so",
                                "that you can actually see it",
                                "for yourself. If you don't think you can handle the expedition,",
                                "then you can just let me know."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(255)])?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("hg_odin").get()? == 13 {
                            ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(2)])?;
                            ctx.lines_as(
                                "Laura",
                                args![
                                    "Have you been to the",
                                    "Odin Shrine already?",
                                    "I know it's scary, but we",
                                    "need to finish our research",
                                    "for the good of mankind."
                                ],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("Yeah, yeah, whatever.:I see.:You're right, Ma'am.")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                "Laura",
                                args!["Well, then.", "Did you happen to", "find anything while", "you were there?"],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Jellopies.:No.")])? {
                                1 => {
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Laura",
                                        args![
                                            "You can freakin' find",
                                            "Jellopy anywhere! I meant,",
                                            "did you find anything special",
                                            "while you were at the shrine!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                }
                                2 => {
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Laura",
                                        args![
                                            "Well, that's fine.",
                                            "I did tell you to just",
                                            "take a look around, so",
                                            "I didn't expect you to",
                                            "bring me anything."
                                        ],
                                    )?;
                                    ctx.next()?;
                                }
                                _ => {}
                            }
                            ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(2)])?;
                            ctx.lines_as(
                                "Laura",
                                args![
                                    "Anyway, are you still willing",
                                    "to help us? I know that I said",
                                    "that we can't pay you, but to",
                                    "be fair, I can offer you something from my special collection.",
                                    "How does that sound?"
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Who are the guys in the other room?:Well...:Yes!")])? {
                                1 => {
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Laura",
                                        args![
                                            "Oh, those are the",
                                            "Schwarzwald Republic",
                                            "representatives for this",
                                            "Odin Shrine expedition.",
                                            "Personally, I think they're",
                                            "just a bunch of rich phonies."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Laura",
                                        args![
                                            "And that Alex girl...",
                                            "She may seem polite,",
                                            "but she's always giving",
                                            "these dirty looks, and",
                                            "greets me with a fake",
                                            "smile. I hate her!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(
                                        Function::Emotion,
                                        vec![
                                            ctx.constant("ET_PROFUSELY_SWEAT")?,
                                            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Ashe")])?,
                                        ],
                                    )?;
                                    ctx.lines_as(
                                        "Ashe",
                                        args![
                                            "Ms. Laurence...!",
                                            "Stop it, please!",
                                            "It's embarassing to",
                                            "hear you talk about",
                                            "a colleague like that."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Laura",
                                        args![
                                            "C-colleague?! No way!",
                                            "I despise that spoiled brat,",
                                            "and she doesn't deserve the",
                                            "position that her rich daddy",
                                            "probably bought for her! Grrr!",
                                            "Come back later, I'm too mad!"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(255)])?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Laura",
                                        args![
                                            "If you're having second",
                                            "thoughts, it might not be",
                                            "the best idea to work for me.",
                                            "I mean, I believe in following",
                                            "your gut instincts, you know?"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(255)])?;
                                    return Err(Stop::End);
                                }
                                3 => {
                                    ctx.var("hg_odin").set(Val::from(14))?;
                                    ctx.call(Function::ChangeQuest, vec![Val::from(11003), Val::from(11004)])?;
                                    ctx.lines_as(
                                        "Laura",
                                        args![
                                            "That's what I've been",
                                            "waiting to hear! Alright,",
                                            "please fetch me ^3355FF5 Runes of",
                                            "the Darkness^000000 from the Odin",
                                            "Shrine. Go now, adventurer~"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(255)])?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        } else {
                            if ctx.var("hg_odin").get()? == 14 {
                                ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Laura",
                                    args!["Hm? You've got a job", "to do, don't you? What", "are you still doing here?"],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("What was I supposed to gather?:Don't worry, I'm leaving.")])?
                                {
                                    1 => {
                                        ctx.lines_as(
                                            "Laura",
                                            args![
                                                "You're supposed",
                                                "to bring me ^3355FF5 Runes",
                                                "of the Darkness^000000. Now,",
                                                "don't forget this time!"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(255)])?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.lines_as(
                                            "Laura",
                                            args![
                                                "Well, try to hurry up",
                                                "if you can. I'm already",
                                                "behind schedule because",
                                                "we have to work with those",
                                                "snobby rich kids in the",
                                                "other office. ^333333*Sigh...*^000000"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(255)])?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            } else {
                                if ctx.var("hg_odin").get()? == 15 {
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Laura",
                                        args!["Oh, you've come back.", "So where are the Runes", "of the Darkness you brought?"],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Er, I don't have them yet...:Right here!")])? {
                                        1 => {
                                            ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Laura",
                                                args![
                                                    "This isn't really one",
                                                    "of those jobs where you",
                                                    "can take your time. Sure, I didn't give you a deadline, but",
                                                    "we can't make progress on our research until you make progress."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(255)])?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            if ctx.call(Function::CountItem, vec![Val::from(7511)])?.number()? > 4 {
                                                ctx.call(Function::DelItem, vec![Val::from(7511), Val::from(5)])?;
                                                ctx.var("hg_odin").set(Val::from(16))?;
                                                ctx.call(Function::ChangeQuest, vec![Val::from(11004), Val::from(11005)])?;
                                                ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
                                                ctx.lines_as(
                                                    "Laura",
                                                    args![
                                                        "Oooh, nice job. It's good",
                                                        "that we have more of these",
                                                        "to study. Though, it seems",
                                                        "that the more we research,",
                                                        "the quicker it seems that",
                                                        "we're not getting anywhere..."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Laura",
                                                    args![
                                                        "Hey...",
                                                        "Ashe. Why aren't we",
                                                        "making any progress?",
                                                        "What exactly have you",
                                                        "been doing? None of those",
                                                        "runes have any writing?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Ashe",
                                                    args![
                                                        "Well, I think there's",
                                                        "supposed to be writing,",
                                                        "but these runes are pretty",
                                                        "old. I've got to polish this",
                                                        "gunk off without damaging",
                                                        "any writing that's there..."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Laura",
                                                    args![
                                                        "By the way, what have the other guys been doing?",
                                                        "What, they don't do anything but wasting their time?",
                                                        "Ah~ I see. They are thinking that this is the perfect excuse",
                                                        "for them to relax and rest.",
                                                        "Well, somehow they are the citizens of this country, you know."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Laura",
                                                    args![
                                                        "What do you think those",
                                                        "Schwarzwald snobs are doing?",
                                                        "They must think this project",
                                                        "is an excuse for them to relax",
                                                        "and slack off! They can afford to, obviously. Hmpf! Rich people!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Laura",
                                                    args![
                                                        "They've got things so easy,",
                                                        "while we're barely covering",
                                                        "our research expenses!",
                                                        "In fact, we've had to cut",
                                                        "so many corners, skip",
                                                        "meals and amenities..."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Ashe",
                                                    args![
                                                        "I know! How can we",
                                                        "carry on professional",
                                                        "research in a shack?",
                                                        "We're literally living at",
                                                        "poverty standards here...",
                                                        "Are we still researchers?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Laura",
                                                    args!["I know that!", "I know that already!", "S-stop bringing it up!"],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Laura",
                                                    args![
                                                        "This is pathetic.",
                                                        "I can't even afford the",
                                                        "energy to continue this",
                                                        "conversation. You too,",
                                                        "why don't you take a rest?"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(255)])?;
                                                return Err(Stop::End);
                                            } else if !(ctx.call(Function::CountItem, vec![Val::from(7511)])?.is_true()) {
                                                ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Laura",
                                                    args![
                                                        "..................",
                                                        "What is this?!",
                                                        "You're supposed to",
                                                        "bring me 5 of them!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Laura",
                                                    args![
                                                        "Is this your way of",
                                                        "saying that you don't",
                                                        "want to work for us?",
                                                        "Why don't you say it",
                                                        "to my face, eh?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Ashe",
                                                    args![
                                                        "Ms. Laurence, please,",
                                                        "calm down! We're in no",
                                                        "position to yell at anyone,",
                                                        "especially volunteers! You",
                                                        "of all people should know",
                                                        "that they're doing us a favor!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Laura",
                                                    args![
                                                        "Okay, Ashe, I get it!",
                                                        "And as for you, I'll give",
                                                        "you another chance. Next",
                                                        "time you see me, bring",
                                                        "5 Runes of the Darkness!"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(255)])?;
                                                return Err(Stop::End);
                                            } else if ctx.call(Function::CountItem, vec![Val::from(7511)])? == 1 {
                                                ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Laura",
                                                    args![
                                                        "...Only one?",
                                                        "Look at me. I asked",
                                                        "you for five. I know that",
                                                        "one is better than nothing...",
                                                        "But it's pretty darn close. Go",
                                                        "get me 4 more this instant!"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(255)])?;
                                                return Err(Stop::End);
                                            } else if ctx.call(Function::CountItem, vec![Val::from(7511)])? == 2 {
                                                ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Laura",
                                                    args![
                                                        "One, two...",
                                                        "Huh? Th-that's all?!",
                                                        "When I ask for 5 Runes",
                                                        "of the Darkness, I mean",
                                                        "5, not 2. Now hurry up",
                                                        "and just do what I asked!"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(255)])?;
                                                return Err(Stop::End);
                                            } else {
                                                ctx.lines_as(
                                                    "Laura",
                                                    args![
                                                        "One, two, three...",
                                                        "That's it? Don't you",
                                                        "know how to count to 5?",
                                                        "Go and get me more of those",
                                                        "Runes of the Darkness now!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(2)])?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(255)])?;
                                                return Err(Stop::End);
                                            }
                                        }
                                        _ => {}
                                    }
                                } else if ctx.var("hg_odin").get()? == 16 {
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Laura",
                                        args![
                                            "I wonder how much progress",
                                            "those pampered Schwarzwald",
                                            "snobs actually made. They've",
                                            "got the funds, so they could",
                                            "actually be further ahead",
                                            "than we are right now..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Laura",
                                        args![
                                            "I have to know...",
                                            "I need to know whether",
                                            "they discovered ''that thing''",
                                            "already. They can't have found",
                                            "it before us. It's impossible!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(2)])?;
                                    ctx.lines_as("Laura", args!["...Hey, you."])?;
                                    ctx.next()?;
                                    if Val::from(runtime::select_values(ctx, &[Val::from("...:...Me?")])?) == 1 {
                                        ctx.lines_as("Laura", args!["Hey, hey!", "Hey hey hey!"])?;
                                        ctx.next()?;
                                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...Me?"])?;
                                        ctx.next()?;
                                    }
                                    ctx.lines_as(
                                        "Laura",
                                        args![
                                            "Basically, I want you",
                                            "to spy on that snobby girl,",
                                            "Alex of the Schwarzwald",
                                            "Republic office, and report",
                                            "what she's actually doing to me."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Laura",
                                        args![
                                            "Anyway, just eavesdrop",
                                            "on the Schwaltvalt Office",
                                            "by putting your ear really",
                                            "close to the door, and listen",
                                            "to them talk. Will you do that?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Laura",
                                        args!["...............................", "Of course, you're", "going to do it. Right?"],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(2)])?;
                                    ctx.lines_as("Laura", args!["RIGHT?!"])?;
                                    ctx.next()?;
                                    let choice = runtime::select_values(
                                        ctx,
                                        &[Val::from("Yeah, yeah, whatever.:Sure thing.:I s-s-s-suppose...!:Yes, Ma'am!")],
                                    )?;
                                    ctx.var("@menu").set(choice)?;
                                    ctx.var("hg_odin").set(Val::from(17))?;
                                    l_i = Val::from(11002);
                                    'l7: loop {
                                        if !(l_i.clone().number()? <= 11005) {
                                            break 'l7;
                                        }
                                        'b7: {
                                            if (ctx.call(Function::CheckQuest, vec![l_i.clone()])?.number()? > -1
                                                && ctx.call(Function::CheckQuest, vec![l_i.clone()])?.number()? < 2)
                                            {
                                                ctx.call(Function::CompleteQuest, vec![l_i.clone()])?;
                                            }
                                        }
                                        l_i = (l_i.clone() + Val::from(1));
                                    }
                                    ctx.call(Function::SetQuest, vec![Val::from(11006)])?;
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(2)])?;
                                    ctx.lines_as("Laura", args!["Good.", "Now do it!"])?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(255)])?;
                                    return Err(Stop::End);
                                } else if ctx.var("hg_odin").get()? == 17 {
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Laura",
                                        args![
                                            "What are you standing",
                                            "around here for? Go and",
                                            "eavesdrop on those snobs",
                                            "next door! I must know",
                                            "what they're really up to!"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(255)])?;
                                    return Err(Stop::End);
                                } else if (ctx.var("hg_odin").get()? == 18 || ctx.var("hg_odin").get()? == 19) {
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Laura",
                                        args![
                                            "Frankly, I know I'm paying",
                                            "you nothing to be spy, but",
                                            "please tell me what you",
                                            "learned about the Schwarzwald",
                                            "Republic Research Team."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    'b8: {
                                        let subject8 =
                                            Val::from(runtime::select_values(ctx, &[Val::from("Giantes:Ymir's Heart:Nothing")])?);
                                        let mut matched8 = false;
                                        let no_case8 = !subject8.loosely_equals(&Val::from(1))
                                            && !subject8.loosely_equals(&Val::from(2))
                                            && !subject8.loosely_equals(&Val::from(3));
                                        if !matched8 && subject8.loosely_equals(&Val::from(1)) {
                                            matched8 = true;
                                        }
                                        if matched8 {
                                            ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(2)])?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "I'm guessing that you're",
                                                    "already familiar with the",
                                                    "creature known as Giantes",
                                                    "since you're an archaeologist."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Laura",
                                                args![
                                                    "Of course! It's an",
                                                    "ancient beast on which",
                                                    "the Guardians were inspired.",
                                                    "N-no way...! Are they here to",
                                                    "finish their guardian research?",
                                                    "What else did you learn?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                        }
                                        if !matched8 && subject8.loosely_equals(&Val::from(2)) {
                                            matched8 = true;
                                        }
                                        if matched8 {
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["Well, have you", "ever heard of Ymir's..."],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(
                                                Function::Emotion,
                                                vec![
                                                    ctx.constant("ET_SURPRISE")?,
                                                    ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Ashe")])?,
                                                ],
                                            )?;
                                            ctx.lines_as("Ashe", args!["...Heart?", "Ymir's Heart, right?"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Laura", args!["Ashe, you know about", "this? What exactly", "is Ymir's Heart?"])?;
                                            ctx.next()?;
                                            match runtime::select_values(
                                                ctx,
                                                &[Val::from("It's candy.:It's a book.:It's a place.:Ymir was a giant...")],
                                            )? {
                                                1 => {
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args!["It's this...", "Um, really..."],
                                                    )?;
                                                    ctx.next()?;
                                                }
                                                2 => {
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args!["Er, you see...", "A lot of people know", "about Ymir's Heart..."],
                                                    )?;
                                                    ctx.next()?;
                                                }
                                                3 => {
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args![
                                                            "You know the expression,",
                                                            "''Home is where the heart",
                                                            "is?'' Er, well, Ymir's Heart..."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                }
                                                4 => {
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args!["The truth is,", "Ymir was this giant..."],
                                                    )?;
                                                    ctx.next()?;
                                                }
                                                _ => {}
                                            }
                                            ctx.call(
                                                Function::Emotion,
                                                vec![
                                                    ctx.constant("ET_OK")?,
                                                    ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Ashe")])?,
                                                ],
                                            )?;
                                            ctx.lines_as(
                                                "Ashe",
                                                args![
                                                    "Ymir's Heart?",
                                                    "That's a clothing",
                                                    "brand, isn't it? Yeah,",
                                                    "it's popular amongst",
                                                    "the trendy crowd in the",
                                                    "Schwarzwald Republic."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Laura",
                                                args![
                                                    "Clothing brand?",
                                                    "You mean those snobs",
                                                    "are just spending their",
                                                    "time talking about fashion",
                                                    "in there? Huh. Don't they",
                                                    "have better things to do?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Laura",
                                                args![
                                                    "In the meantime, would",
                                                    "you do one more favor for",
                                                    "me? I'd like you to go to",
                                                    "the shrine one last time.",
                                                    "I hear that it's built on",
                                                    "two connected islands..."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Laura",
                                                args![
                                                    "I want you to check the",
                                                    "inner island, and see",
                                                    "what you can find while",
                                                    "I investigate the Schwarzwald",
                                                    "Research Team. I'm sorry, but",
                                                    "you're the only one I can ask."
                                                ],
                                            )?;
                                            ctx.var("hg_odin").set(Val::from(20))?;
                                            ctx.call(Function::ChangeQuest, vec![Val::from(11006), Val::from(11007)])?;
                                            ctx.close_window()?;
                                            ctx.call(Function::Cutin, vec![Val::from("hu_laura04.bmp"), Val::from(255)])?;
                                            return Err(Stop::End);
                                        }
                                        if !matched8 && subject8.loosely_equals(&Val::from(3)) {
                                            matched8 = true;
                                        }
                                        if matched8 {
                                            ctx.lines_as("Laura", args!["Nothing...?", "Are you sure that's", "all you learned?"])?;
                                            ctx.close_window()?;
                                            ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(255)])?;
                                            return Err(Stop::End);
                                        }
                                    }
                                } else if ctx.var("hg_odin").get()? == 20 {
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Laura",
                                        args![
                                            "I wonder...",
                                            "What could be going on?",
                                            "Hopefully I'll be able",
                                            "to find the answers..."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(255)])?;
                                    return Err(Stop::End);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn laura(ctx: &Ctx) -> Script {
    laura_body(ctx, Vec::new()).map(|_| ())
}

fn julian_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !(ctx.var("hg_odin").get()?.is_true()) {
        ctx.lines_as("Julian", args!["^333333*Sigh...*^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("hg_odin").get()? == 1 {
            ctx.lines_as(
                "Julian",
                args![
                    "Say, are you here",
                    "for a job? We're looking",
                    "for someone that's passionate",
                    "about ancient history. If you",
                    "want to make money, then this",
                    "this the wrong place for you~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Julian",
                args![
                    "This is supposed to be",
                    "a collaborative excavation,",
                    "but the team leaders are too",
                    "stubborn and refuse to make",
                    "any compromises. It's been",
                    "incredibly frustrating..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Julian",
                args![
                    "Basically, each of our teams",
                    "is conducting its own research",
                    "for now. Now, we need your help",
                    "to find any relics from the Odin Shrine. We'd appreciate it if you",
                    "brought as many as you can!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("hg_odin").get()? == 2 {
            ctx.lines_as(
                "Julian",
                args![
                    "You sure you want",
                    "to work for someone",
                    "like my boss? Can't",
                    "you tell that she's",
                    "evil? Escape from this",
                    "place while you still can!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Alex", args!["Julian...!"])?;
            ctx.next()?;
            ctx.lines_as("Julian", args!["See what I mean?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("hg_odin").get()? == 3 {
            ctx.lines_as(
                "Julian",
                args![
                    "Wow, you must really",
                    "like this kind of work,",
                    "huh? I didn't really expect",
                    "that you'd go to the shrine..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("hg_odin").get()? == 4 {
            ctx.lines_as(
                "Julian",
                args![
                    "I'm amazed to see you",
                    "working so hard. You must",
                    "like pleasing other people,",
                    "I guess, but try not to be too",
                    "kind. Wouldn't want that..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("hg_odin").get()? == 5 {
            ctx.lines_as(
                "Julian",
                args![
                    "Geez...",
                    "You're a really nice",
                    "person, you know that?",
                    "Just try not to be taken",
                    "for a sucker, okay?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Julian",
                args![
                    "Why are both of our",
                    "team leaders so bull",
                    "headed? Maybe it has",
                    "something to do with",
                    "them being old and single.",
                    "Don't tell them I said that!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn julian(ctx: &Ctx) -> Script {
    julian_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ashe5Step {
    Start,
    OnTouch,
}

fn ashe_5_run(ctx: &Ctx, mut step: Ashe5Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ashe5Step::Start => {
                step = Ashe5Step::OnTouch;
                continue 'machine;
            }
            Ashe5Step::OnTouch => {
                if ctx.var("hg_odin").get()? == 18 {
                    ctx.lines_as("Ashe", args!["......?"])?;
                    ctx.next()?;
                    ctx.var("hg_odin").set(Val::from(19))?;
                    ctx.lines_as("Ashe", args!["......", "........."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn ashe_5(ctx: &Ctx) -> Script {
    ashe_5_run(ctx, Ashe5Step::Start, Vec::new()).map(|_| ())
}

pub fn ashe_5_ontouch(ctx: &Ctx) -> Script {
    ashe_5_run(ctx, Ashe5Step::OnTouch, Vec::new()).map(|_| ())
}

fn ashe_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = ctx.var("hg_odin").get()?;
    if subject1 == 0 {
        ctx.lines_as(
            "Ashe",
            args![
                "Hello, may I help you?",
                "Please understand that",
                "unauthorized personnel",
                "are prohibited from entering",
                "this area. Have you come to",
                "assist in the expedition?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 1 {
        ctx.lines_as(
            "Ashe",
            args![
                "Oh, you've here in response",
                "to the recruitment notice?",
                "Well then, let me give you",
                "a brief idea about the work.",
                "First off, we can't pay you any",
                "money, so you'd be a volunteer."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ashe",
            args![
                "Instead, you're allowed to",
                "keep whatever you excavate",
                "in the Odin Shrine, so long",
                "as we don't need it for our",
                "research. This opportunity",
                "might prove worthwhile to you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ashe",
            args![
                "Now, we should be working",
                "together with the Schwaltvalt",
                "Republic's Research Team, but",
                "our leaders aren't getting along at all. There's just a huge",
                "clash of personalities..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ashe",
            args![
                "In any case, if you'd",
                "like to join us in our",
                "efforts to uncover the",
                "secrets of the past, then",
                "please speak to Ms. Laurence."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 2 || subject1 == 12 {
        ctx.lines_as(
            "Ashe",
            args!["So you're off to the", "Odin Shrine? Good luck,", "and be careful, alright?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 3 || subject1 == 13 {
        ctx.lines_as(
            "Ashe",
            args![
                "Oh, you're back",
                "earlier than I thought!",
                "I think someone with",
                "your talent can be",
                "a really big help to us!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 4 {
        ctx.lines_as(
            "Ashe",
            args![
                "Hm? You've decided to",
                "help the Schwarzwald Team?",
                "That's fine, but I'm afraid",
                "Ms. Laurence may be a little",
                "upset by that. Well, it's fine",
                "with me. I don't really care."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 5 {
        ctx.lines_as(
            "Ashe",
            args![
                "How's the work coming",
                "along? I know the team",
                "leaders hate each other,",
                "but they should recognize",
                "that one person's victory",
                "in research is victory for all."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 6 {
        ctx.lines_as(
            "Ashe",
            args![
                "How's the work coming",
                "along? I know the team",
                "leaders hate each other,",
                "but they should recognize",
                "that one person's victory",
                "in research is victory for all."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 14 {
        ctx.lines_as("Ashe", args!["Please keep up", "the good work~"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 15 {
        if ctx.call(Function::CountItem, vec![Val::from(7511)])?.number()? > 4 {
            ctx.lines_as(
                "Ashe",
                args![
                    "Well, it looks like",
                    "everything turned out",
                    "alright in the end, huh?",
                    "Good work getting those runes."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Ashe", args!["Oh, dear, you don't", "look very well. Did", "something happen?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 16 {
        ctx.lines_as(
            "Ashe",
            args![
                "I know that Ms. Laurence",
                "can be... vocally aggressive,",
                "but I'm sure that she really",
                "appreciates all of your hard",
                "work. Thank you so much for",
                "all of your efforts~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 17 {
        ctx.lines_as(
            "Ashe",
            args![
                "Why does Ms. Laurence",
                "have to be this way?",
                "I honestly think this",
                "rivalry is awfully petty..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 19 {
        ctx.lines_as("Ashe", args!["Ehm...", "Don't worry", "about it~"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 20 {
        ctx.lines_as(
            "Ashe",
            args!["Please don't ask", "me about anything...", "Besides, I mean", "you no harm~"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn ashe(ctx: &Ctx) -> Script {
    ashe_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum EntranceStep {
    Start,
    OnTouch,
}

fn entrance_run(ctx: &Ctx, mut step: EntranceStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            EntranceStep::Start => {
                step = EntranceStep::OnTouch;
                continue 'machine;
            }
            EntranceStep::OnTouch => {
                if (ctx.var("hg_odin").get()? == 22 || ctx.var("hg_odin").get()? == 23) {
                    ctx.lines_as("Laura", args!["So it means that everyone fooled me!"])?;
                    ctx.next()?;
                    ctx.mes("== Laura is violently wielding a thick file in the air. ==")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn entrance(ctx: &Ctx) -> Script {
    entrance_run(ctx, EntranceStep::Start, Vec::new()).map(|_| ())
}

pub fn entrance_ontouch(ctx: &Ctx) -> Script {
    entrance_run(ctx, EntranceStep::OnTouch, Vec::new()).map(|_| ())
}

fn laura_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("hg_odin").get()? == 22 || ctx.var("hg_odin").get()? == 23) {
        ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(2)])?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_FRET")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Laura#2")])?,
            ],
        )?;
        ctx.lines_as(
            "Laura",
            args![
                "Wh-what...?!",
                "Does this mean",
                "that everyone has",
                "had me fooled this",
                "whole time?! This is",
                "complete lunacy!"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("hu_alex04.bmp"), Val::from(0)])?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_FRET")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Alex#2")])?,
            ],
        )?;
        ctx.lines_as(
            "Alex",
            args![
                "What are you talking",
                "about, you crazy wench?",
                "You saw the shrine on the",
                "other side! That's no secret!"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Laura",
            args![
                "I'm not talking about that!",
                "I saw your brother digging",
                "everywhere in town in the",
                "middle of the night!"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("hu_alex04.bmp"), Val::from(0)])?;
        ctx.lines_as(
            "Alex",
            args![
                "Lies...!",
                "Where the hell did",
                "you get that idea?",
                "What makes you think",
                "there are artifacts buried",
                "around here inside town?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Alex",
            args![
                "Besides, what makes you",
                "think that my brother would",
                "waste so much time doing",
                "something so ridiculous?"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Laura",
            args![
                "Hahahaha! Yeah,",
                "defend your brother",
                "as much as you like,",
                "you hypocrite! What do",
                "you care about him, I know",
                "you two really hate each other!"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("hu_alex04.bmp"), Val::from(0)])?;
        ctx.lines_as(
            "Alex",
            args![
                "What was that...?",
                "I dare you to say",
                "that again! I'll make",
                "sure that you regret it!"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("hu_laura01.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Laura",
            args![
                "I know that you were helping",
                "him in his excavations. There's",
                "no way so much progress can be",
                "done by one man alone! In fact,",
                "I have evidence that proves it!"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFLaura violently",
            "waves a thick file",
            "clasped in her hands.^000000"
        ])?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HUK")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Julian#2")])?,
            ],
        )?;
        ctx.lines_as("Julian", args!["What...?", "Argh, that's--!"])?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_ANGER")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Laura#2")])?,
            ],
        )?;
        ctx.lines_as(
            "Laura",
            args!["That's right.", "Now tell me, what the", "hell is ''^FF0000Ymir's Heart?^000000''"],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("hu_alex02.bmp"), Val::from(0)])?;
        ctx.lines_as("Alex", args!["......", "Julian!"])?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_PROFUSELY_SWEAT")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Julian#2")])?,
            ],
        )?;
        ctx.lines_as(
            "Julian",
            args![
                "Uh, this isn't my fault!",
                "Sh-she doesn't have any",
                "right to come into our office!"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("hu_laura02.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Laura",
            args![
                "Hah! Aren't we supposed to",
                "be equal partners in the same",
                "expedition? That's what I heard",
                "at the beginning! But you guys",
                "have something different in mind!"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Laura",
            args![
                "It's awfully suspicious: you",
                "guys are only going out at",
                "night, and you're making so",
                "little progress with the Odin",
                "Shrine excavation. Tell me,",
                "what are your true goals?!"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("hu_alex03.bmp"), Val::from(0)])?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_PROFUSELY_SWEAT")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Alex#2")])?,
            ],
        )?;
        ctx.lines_as(
            "Alex",
            args![
                "I-I don't know what you're",
                "talking about! J-Julian must",
                "have gotten some mission, and",
                "has been carrying it out without my consent! Don't go twisting",
                "the facts, you... you thief!"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(2)])?;
        ctx.lines_as("Laura", args!["Thief...?!", "Look who's", "talking!"])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFLaura tossed the file",
            "away, and threateningly",
            "rolled up her sleeves.^000000"
        ])?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_CRY")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Ashe#2")])?,
            ],
        )?;
        ctx.lines_as("Ashe", args!["S-stop!", "Both of you!"])?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_PROFUSELY_SWEAT")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Laura#2")])?,
            ],
        )?;
        ctx.lines_as("Laura", args!["Stay out of", "this, Ashe!"])?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("hu_alex04.bmp"), Val::from(0)])?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_FRET")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Alex#2")])?,
            ],
        )?;
        ctx.lines_as("Alex", args!["Yeah, this is none", "of your business!"])?;
        ctx.next()?;
        ctx.var("hg_odin").set(Val::from(25))?;
        ctx.lines_as("Ashe", args!["Oh... Oh, dear!"])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("hu_alex04.bmp"), Val::from(255)])?;
        return Err(Stop::End);
    } else if ctx.var("hg_odin").get()?.number()? > 23 {
        ctx.call(Function::Cutin, vec![Val::from("hu_laura03.bmp"), Val::from(2)])?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_FRET")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Laura#2")])?,
            ],
        )?;
        ctx.lines_as("Laura", args!["Hah...!", "Did you...", "Just hit me?"])?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("hu_alex04.bmp"), Val::from(0)])?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_FRET")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Alex#2")])?,
            ],
        )?;
        ctx.lines_as("Alex", args!["You crazy...!"])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("hu_alex04.bmp"), Val::from(255)])?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn laura_2(ctx: &Ctx) -> Script {
    laura_2_body(ctx, Vec::new()).map(|_| ())
}

fn ashe_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("hg_odin").get()? == 22 || ctx.var("hg_odin").get()? == 23) {
        ctx.lines_as("Ashe", args!["This...", "This doesn't", "look good at all!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("hg_odin").get()? == 25 {
        ctx.lines(args!["^3355FFAshe picked up", "the file that Laura", "tossed away.^000000"])?;
        ctx.next()?;
        ctx.lines_as(
            "Ashe",
            args![
                "Um, I've got a favor",
                "of my own to ask you.",
                "Would you please bring",
                "this file to Morocc? Just",
                "bring it to my comrade",
                "over at the South Gate."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ashe",
            args![
                "I'm sorry, but we don't",
                "have much time left. I guess",
                "this is the last time we will",
                "ever see each other, so thank",
                "you for all your help, friend."
            ],
        )?;
        ctx.next()?;
        ctx.var("hg_odin").set(Val::from(59))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(11007), Val::from(11008)])?;
        ctx.lines_as(
            "Ashe",
            args![
                "Don't worry, my comrade",
                "at Morocc's South Gate will",
                "explain everything once you",
                "find him. Please hurry!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("hg_odin").get()? == 59 {
        ctx.lines_as(
            "Ashe",
            args![
                "Don't worry, my comrade",
                "at Morocc's South Gate will",
                "explain everything once you",
                "find him. Please hurry, and",
                "find him as soon as you can!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn ashe_2(ctx: &Ctx) -> Script {
    ashe_2_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ashe3Step {
    Start,
    OnTouch,
}

fn ashe_3_run(ctx: &Ctx, mut step: Ashe3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ashe3Step::Start => {
                step = Ashe3Step::OnTouch;
                continue 'machine;
            }
            Ashe3Step::OnTouch => {
                if ctx.var("hg_odin").get()? == 25 {
                    ctx.lines(args!["^3355FFAshe picked up", "the file that Laura", "tossed away.^000000"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ashe",
                        args![
                            "Um, I've got a favor",
                            "of my own to ask you.",
                            "Would you please bring",
                            "this file to Morocc? Just",
                            "bring it to my comrade",
                            "over at the South Gate."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ashe",
                        args![
                            "I'm sorry, but we don't",
                            "have much time left. I guess",
                            "this is the last time we will",
                            "ever see each other, so thank",
                            "you for all your help, friend."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("hg_odin").set(Val::from(59))?;
                    ctx.lines_as(
                        "Ashe",
                        args![
                            "Don't worry, my comrade",
                            "at Morocc's South Gate will",
                            "explain everything once you",
                            "find him. Please hurry!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn ashe_3(ctx: &Ctx) -> Script {
    ashe_3_run(ctx, Ashe3Step::Start, Vec::new()).map(|_| ())
}

pub fn ashe_3_ontouch(ctx: &Ctx) -> Script {
    ashe_3_run(ctx, Ashe3Step::OnTouch, Vec::new()).map(|_| ())
}

fn julian_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("hg_odin").get()? == 22 || ctx.var("hg_odin").get()? == 23) {
        ctx.lines_as("Julian", args!["I...", "I don't know", "what I should do!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Julian",
        args![
            "I've stopped caring",
            "whether they fight or",
            "not... I'm so not get",
            "involved. If you want,",
            "you can try to stop them..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn julian_2(ctx: &Ctx) -> Script {
    julian_2_body(ctx, Vec::new()).map(|_| ())
}

fn hit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn hit(ctx: &Ctx) -> Script {
    hit_body(ctx, Vec::new()).map(|_| ())
}
