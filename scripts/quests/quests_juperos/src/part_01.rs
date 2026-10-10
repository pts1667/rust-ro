use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum ScholarStep {
    Start,
    FuncJupHist,
    AfterFuncJupHist,
}

fn scholar_run(ctx: &Ctx, mut step: ScholarStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_arg: Vec<Val> = Vec::new();
    'machine: loop {
        match step {
            ScholarStep::Start => {
                'b1: {
                    let subject1 = ctx.var("yuno_hist").get()?;
                    let mut matched1 = false;
                    let no_case1 = !subject1.loosely_equals(&Val::from(0))
                        && !subject1.loosely_equals(&Val::from(1))
                        && !subject1.loosely_equals(&Val::from(2))
                        && !subject1.loosely_equals(&Val::from(3))
                        && !subject1.loosely_equals(&Val::from(4))
                        && !subject1.loosely_equals(&Val::from(5))
                        && !subject1.loosely_equals(&Val::from(6))
                        && !subject1.loosely_equals(&Val::from(7))
                        && !subject1.loosely_equals(&Val::from(8))
                        && !subject1.loosely_equals(&Val::from(9))
                        && !subject1.loosely_equals(&Val::from(10));
                    if !matched1 && subject1.loosely_equals(&Val::from(0)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as("Scholar", args!["...Mm? ", "...Yes?"])?;
                        ctx.next()?;
                        ctx.lines_as("Scholar", args!["...", "......", "May I help you?"])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Oh! N-Nothing!:Excuse me...")])? {
                            1 => {
                                ctx.lines_as("Scholar", args!["...", "......", "Hmm?", "...........", "Hmpf."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as("Scholar", args!["...", "......", "Hmm?", "...........", "Hmmm..."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Scholar",
                                    args![
                                        "You must be lost.",
                                        "This is the scholarly",
                                        "research section, you know,",
                                        "content you couldn't possibly",
                                        "fathom. The popular novels and picture books are someplace else."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...", "......"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Scholar",
                                    args![
                                        "Why don't you rummage",
                                        "through the bookshelves?",
                                        "I'm sure you can find some",
                                        "book there that can hold your",
                                        "interest. Well, depending on",
                                        "your actual attention span..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "(What's her damage?!",
                                        "Does she have an attitude problem or is she just stuck-up?)"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as("Scholar", args!["...Mm? ", "...Yes?"])?;
                        ctx.next()?;
                        ctx.lines_as("Scholar", args!["...", "......", "May I help you?"])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Oh! N-Nothing!:By any chance...")])? {
                            1 => {
                                ctx.lines_as("Scholar", args!["...", "......", "Hmm?", "...........", "Hmpf."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["By any chance...", "Are you conducting", "research about Juperos?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Scholar",
                                    args![
                                        "Why yes, that is",
                                        "correct. But how did",
                                        "you come to learn about",
                                        "my current research project?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Oh, I managed to read",
                                        "a thesis paper entitled,",
                                        "''The Fall of Juperos,'' and",
                                        "I just thought that the writing",
                                        "style and your personality",
                                        "seem to match for some reason."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Scholar",
                                    args!["Oh...! You read my", "thesis? So what did", "you think about it?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "So far, it's alright, but",
                                        "quite frankly it's incomplete.",
                                        "I mean, you don't have much in",
                                        "in the way of conjecture, much",
                                        "less any evidence to back up",
                                        "any of your statements."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Scholar",
                                    args![
                                        "....",
                                        "Let me apologize for",
                                        "being rude to you earlier.",
                                        "As you know, my name is",
                                        "Fayruz Khrhiyha. May I ask",
                                        "what your name might be?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        ((Val::from("I'm ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                                        "a brave adventurer in the",
                                        "service of his royal majesty,",
                                        "the wise and benevolent",
                                        "King Tristram III."
                                    ],
                                )?;
                                ctx.var("yuno_hist").set(Val::from(2))?;
                                ctx.call(Function::SetQuest, vec![Val::from(11017)])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        ((Val::from("Well, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                                        "I understand that my thesis",
                                        "still requires more evidence.",
                                        "But I'd need some ancient",
                                        "documents from Juperos",
                                        "to complete my research..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "If you happen to travel",
                                        "through Juperos and find",
                                        "any ancient documents, would",
                                        "you bring them to me? Having",
                                        "those would help my research",
                                        "efforts immensely. Thank you..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                        matched1 = true;
                    }
                    if matched1 {
                        if (((ctx.call(Function::CountItem, vec![Val::from(7352)])?.is_true()
                            || ctx.call(Function::CountItem, vec![Val::from(7353)])?.is_true())
                            || ctx.call(Function::CountItem, vec![Val::from(7354)])?.is_true())
                            || ctx.call(Function::CountItem, vec![Val::from(7355)])?.is_true())
                        {
                            ctx.lines_as(
                                "Fayruz",
                                args![
                                    "Ah, it's you! Listen,",
                                    "I just found a record of",
                                    "an adventurer who explored",
                                    "Juperos. There's mention",
                                    "of a stone statue here that",
                                    "just might be noteworthy..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Fayruz",
                                args![
                                    "If you happen to find",
                                    "yourself in Juperos,",
                                    "would you find the stone",
                                    "statue at the entrance of",
                                    "its dungeon and read the",
                                    "engraved message for me?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Fayruz",
                                args![
                                    "According to my notes,",
                                    "there's a spell that will",
                                    "make its reader memorize",
                                    "its message, even if they don't",
                                    "know the language. So come",
                                    "to me if you manage to read it."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "If you ever chance to",
                                "travel through Juperos,",
                                "would you let me know if you",
                                "find anything that might help",
                                "my research there? I'd be",
                                "very grateful for your help."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Well, you look",
                                "quite pleased.",
                                "May I ask what",
                                "happened to put that",
                                "expression on your face?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("I found something in Juperos.:Nothing much.")])? {
                            1 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "I went to Juperos like",
                                        "you asked and found that",
                                        "stone statue you were talking",
                                        "about. Just like you said, there was an engraved message on it."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "Fascinating!",
                                        "So is it really enchanted",
                                        "so anyone can memorize it?",
                                        "Wh-what does the message say?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFYou recite the message",
                                    "engraved on the stone",
                                    "statue, unable to interpret",
                                    "the sounds you're uttering,",
                                    "but weirdly enough, you can",
                                    "easily recall them from memory.^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Fayruz",
                                    args!["Ah, I see! Wait,", "give me a moment to", "properly translate this..."],
                                )?;
                                ctx.next()?;
                                ctx.lines(args!["...", "......", "........."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "It means, ''Do you wish to",
                                        "see the end of the madness?",
                                        "He is waiting where the three",
                                        "columns were destroyed, where",
                                        "two hundred illusions wander.''"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "''You will see him, the one",
                                        "who was vain and extravagant,",
                                        "with your own eyes at the place where the light passes through."
                                    ],
                                )?;
                                ctx.var("yuno_hist").set(Val::from(4))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(11018), Val::from(11019)])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "Ah, usually, descriptions",
                                        "of the ''vain and extravagant",
                                        "one'' refer to the mad scientist rumored to have lived in that",
                                        "ancient era. But if this is true, I may have to rework my thesis..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "I have another favor to",
                                        "ask of you. If you find any",
                                        "object of historical significance in Juperos, would you bring it to",
                                        "me? I'll reward you, of course."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "It would be most helpful",
                                        "if you could manage to find",
                                        "documents that existed from that era. Fortunately, back then,",
                                        "they made all their records on material more durable than paper."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "Nothing, huh?",
                                        "My life is also fairly",
                                        "uneventful, but somehow,",
                                        "I'm don't think I'm content."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(4)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                ((Val::from("Oh hello, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                                "So what brings you to",
                                "the Juno Library today?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Nice weather today, isn't it?:I found something in Juperos.:Nothing much.",
                            )],
                        )? {
                            1 => {
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "Well, I wouldn't know.",
                                        "It's late whenever I go",
                                        "out, so I always happen to",
                                        "miss the sunlight. I guess",
                                        "I really miss nice weather",
                                        "sometimes, you know?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                if (((ctx.call(Function::CountItem, vec![Val::from(7352)])?.is_true()
                                    || ctx.call(Function::CountItem, vec![Val::from(7353)])?.is_true())
                                    || ctx.call(Function::CountItem, vec![Val::from(7354)])?.is_true())
                                    || ctx.call(Function::CountItem, vec![Val::from(7355)])?.is_true())
                                {
                                    ctx.lines_as("Fayruz", args!["Oh, really?!", "That's great news!", "W-what did you find?"])?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^3355FFIn her excitement,",
                                        "Fayruz begins to",
                                        "rummage through your",
                                        "things before you get",
                                        "the chance to answer her.^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Fayruz",
                                        args![
                                            "Oh, this must be it!",
                                            "Would you mind if I keep",
                                            "this Transparent Plate for",
                                            "my research? In return, I'll",
                                            "tell you some tales about",
                                            "Juperos that I've learned."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    'b6: {
                                        let subject6 = Val::from(runtime::select_values(
                                            ctx,
                                            &[Val::from("Please, be my guest.:No way, you can't have it.")],
                                        )?);
                                        let mut matched6 = false;
                                        let no_case6 = !subject6.loosely_equals(&Val::from(1)) && !subject6.loosely_equals(&Val::from(2));
                                        if !matched6 && subject6.loosely_equals(&Val::from(1)) {
                                            matched6 = true;
                                        }
                                        if matched6 {
                                            if ctx.call(Function::CountItem, vec![Val::from(7352)])?.is_true() {
                                                scholar_run(ctx, ScholarStep::FuncJupHist, vec![Val::from(7352), Val::from(1)])?;
                                            } else if ctx.call(Function::CountItem, vec![Val::from(7353)])?.is_true() {
                                                scholar_run(ctx, ScholarStep::FuncJupHist, vec![Val::from(7353), Val::from(2)])?;
                                            } else if ctx.call(Function::CountItem, vec![Val::from(7354)])?.is_true() {
                                                scholar_run(ctx, ScholarStep::FuncJupHist, vec![Val::from(7354), Val::from(4)])?;
                                            } else if ctx.call(Function::CountItem, vec![Val::from(7355)])?.is_true() {
                                                scholar_run(ctx, ScholarStep::FuncJupHist, vec![Val::from(7352), Val::from(8)])?;
                                            }
                                        }
                                        if !matched6 && subject6.loosely_equals(&Val::from(2)) {
                                            matched6 = true;
                                        }
                                        if matched6 {
                                            ctx.lines_as(
                                                "Fayruz",
                                                args![
                                                    "Mm? Are you serious?",
                                                    "This object is very valuable",
                                                    "to a researcher like me, but",
                                                    "I have no idea what use it",
                                                    "would be for an adventurer.",
                                                    "Well, you have your reasons..."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                    }
                                }
                                ctx.lines_as("Fayruz", args!["Oh, really?!", "That's great news!", "W-what did you find?"])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFIn her excitement,",
                                    "Fayruz begins to",
                                    "rummage through your",
                                    "things before you get",
                                    "the chance to answer her.^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "Oh. There isn't anything",
                                        "here that would help in my",
                                        "research, but thank you anyway.",
                                        "If you find anything else while",
                                        "you're in Juperos, please come back and show it to me, alright?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            3 => {
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "Ah, I see. Well, while",
                                        "you're here, why don't you",
                                        "read something? There are",
                                        "many books that cover some",
                                        "interesting topics, like the",
                                        "Schwarzwald economy..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "Oh, in any case, please",
                                        "don't forget the favor I asked",
                                        "of you. If you find anything",
                                        "in Juperos that's historically",
                                        "significant, I'd appreciate it",
                                        "if you bring it right away."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(5)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                ((Val::from("Oh, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                                "Have you come back with",
                                "something from Juperos?",
                                "I've been hoping you'd come",
                                "back with something that'd",
                                "help me in my research!"
                            ],
                        )?;
                        ctx.next()?;
                        'b7: {
                            let subject7 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Take a look at this.:Oh, I'm sorry...")],
                            )?);
                            let mut matched7 = false;
                            let no_case7 = !subject7.loosely_equals(&Val::from(1)) && !subject7.loosely_equals(&Val::from(2));
                            if !matched7 && subject7.loosely_equals(&Val::from(1)) {
                                matched7 = true;
                            }
                            if matched7 {
                                'b8: {
                                    let subject8 = ctx.var("jupe_hist").get()?;
                                    let mut matched8 = false;
                                    let no_case8 = !subject8.loosely_equals(&Val::from(1))
                                        && !subject8.loosely_equals(&Val::from(2))
                                        && !subject8.loosely_equals(&Val::from(4))
                                        && !subject8.loosely_equals(&Val::from(8));
                                    if !matched8 && subject8.loosely_equals(&Val::from(1)) {
                                        matched8 = true;
                                    }
                                    if matched8 {
                                        scholar_run(
                                            ctx,
                                            ScholarStep::FuncJupHist,
                                            vec![Val::from(7353), Val::from(7354), Val::from(7355), Val::from(7352)],
                                        )?;
                                    }
                                    if !matched8 && subject8.loosely_equals(&Val::from(2)) {
                                        matched8 = true;
                                    }
                                    if matched8 {
                                        scholar_run(
                                            ctx,
                                            ScholarStep::FuncJupHist,
                                            vec![Val::from(7352), Val::from(7354), Val::from(7355), Val::from(7353)],
                                        )?;
                                    }
                                    if !matched8 && subject8.loosely_equals(&Val::from(4)) {
                                        matched8 = true;
                                    }
                                    if matched8 {
                                        scholar_run(
                                            ctx,
                                            ScholarStep::FuncJupHist,
                                            vec![Val::from(7352), Val::from(7353), Val::from(7355), Val::from(7354)],
                                        )?;
                                    }
                                    if !matched8 && subject8.loosely_equals(&Val::from(8)) {
                                        matched8 = true;
                                    }
                                    if matched8 {
                                        scholar_run(
                                            ctx,
                                            ScholarStep::FuncJupHist,
                                            vec![Val::from(7352), Val::from(7353), Val::from(7354), Val::from(7355)],
                                        )?;
                                    }
                                }
                            }
                            if !matched7 && subject7.loosely_equals(&Val::from(2)) {
                                matched7 = true;
                            }
                            if matched7 {
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "Ah, I see. Well, while",
                                        "you're here, why don't you",
                                        "read something? There are",
                                        "many books that cover some",
                                        "interesting topics, like...",
                                        "like... Self-Honesty (?)."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "Oh, in any case, please",
                                        "don't forget the favor I asked",
                                        "of you. If you find anything",
                                        "in Juperos that's historically",
                                        "significant, I'd appreciate it",
                                        "if you bring it right away."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(6)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                ((Val::from("Oh, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                                "The Transparent Plate",
                                "that you brought for me",
                                "last time is really helping me",
                                "in my research. If you get the",
                                "chance, please bring me more!"
                            ],
                        )?;
                        ctx.var("yuno_hist").set(Val::from(7))?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "This new data is adding",
                                "a lot more credibility to my",
                                "thesis. Oh, I'll be with you",
                                "in a moment, let me finish",
                                "translating this one last",
                                "passage really quickly..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(7)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                ((Val::from("Hello, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("~")),
                                "Oh, were you able to look",
                                "in Juperos for anything that",
                                "might help me in my research?"
                            ],
                        )?;
                        ctx.next()?;
                        'b9: {
                            let subject9 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Yeah, take a look at this.:No, I'm sorry...")],
                            )?);
                            let mut matched9 = false;
                            let no_case9 = !subject9.loosely_equals(&Val::from(1)) && !subject9.loosely_equals(&Val::from(2));
                            if !matched9 && subject9.loosely_equals(&Val::from(1)) {
                                matched9 = true;
                            }
                            if matched9 {
                                'b10: {
                                    let subject10 = ctx.var("jupe_hist").get()?;
                                    let mut matched10 = false;
                                    let no_case10 = !subject10.loosely_equals(&Val::from(3))
                                        && !subject10.loosely_equals(&Val::from(5))
                                        && !subject10.loosely_equals(&Val::from(6))
                                        && !subject10.loosely_equals(&Val::from(9))
                                        && !subject10.loosely_equals(&Val::from(10))
                                        && !subject10.loosely_equals(&Val::from(12));
                                    if !matched10 && subject10.loosely_equals(&Val::from(3)) {
                                        matched10 = true;
                                    }
                                    if matched10 {
                                        scholar_run(
                                            ctx,
                                            ScholarStep::FuncJupHist,
                                            vec![Val::from(7354), Val::from(7355), Val::from(7352), Val::from(7353)],
                                        )?;
                                    }
                                    if !matched10 && subject10.loosely_equals(&Val::from(5)) {
                                        matched10 = true;
                                    }
                                    if matched10 {
                                        scholar_run(
                                            ctx,
                                            ScholarStep::FuncJupHist,
                                            vec![Val::from(7353), Val::from(7355), Val::from(7352), Val::from(7354)],
                                        )?;
                                    }
                                    if !matched10 && subject10.loosely_equals(&Val::from(6)) {
                                        matched10 = true;
                                    }
                                    if matched10 {
                                        scholar_run(
                                            ctx,
                                            ScholarStep::FuncJupHist,
                                            vec![Val::from(7352), Val::from(7355), Val::from(7353), Val::from(7354)],
                                        )?;
                                    }
                                    if !matched10 && subject10.loosely_equals(&Val::from(9)) {
                                        matched10 = true;
                                    }
                                    if matched10 {
                                        scholar_run(
                                            ctx,
                                            ScholarStep::FuncJupHist,
                                            vec![Val::from(7353), Val::from(7354), Val::from(7352), Val::from(7355)],
                                        )?;
                                    }
                                    if !matched10 && subject10.loosely_equals(&Val::from(10)) {
                                        matched10 = true;
                                    }
                                    if matched10 {
                                        scholar_run(
                                            ctx,
                                            ScholarStep::FuncJupHist,
                                            vec![Val::from(7352), Val::from(7354), Val::from(7353), Val::from(7355)],
                                        )?;
                                    }
                                    if !matched10 && subject10.loosely_equals(&Val::from(12)) {
                                        matched10 = true;
                                    }
                                    if matched10 {
                                        scholar_run(
                                            ctx,
                                            ScholarStep::FuncJupHist,
                                            vec![Val::from(7352), Val::from(7353), Val::from(7354), Val::from(7355)],
                                        )?;
                                    }
                                }
                            }
                            if !matched9 && subject9.loosely_equals(&Val::from(2)) {
                                matched9 = true;
                            }
                            if matched9 {
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "Oh, that's fine.",
                                        "Besides, I don't really",
                                        "have a deadline to complete",
                                        "this research project. Still,",
                                        "I just want you to know that",
                                        "I really appreciate your help."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(8)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                                "I'm having great difficulty in",
                                "translating that Transparent",
                                "Plate you brought for me that",
                                "last time. I'm so frustrated..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Wait a minute...",
                                "This here means...",
                                "Alright. Okay. Yes.",
                                "Yes! Of course, how",
                                "could I not see it before!"
                            ],
                        )?;
                        ctx.var("yuno_hist").set(Val::from(9))?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args!["I'll be with you", "in just a second!", "I think I just made", "a real through...!"],
                        )?;
                        ctx.next()?;
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(9)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Ah, I've been",
                                ((Val::from("expecting you, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                                "So did you have been to Juperos again? I'm really hoping that you",
                                "were able to find something new that would help in my research..."
                            ],
                        )?;
                        ctx.next()?;
                        'b11: {
                            let subject11 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Actually, I did find this...:I'm sorry, I haven't...")],
                            )?);
                            let mut matched11 = false;
                            let no_case11 = !subject11.loosely_equals(&Val::from(1)) && !subject11.loosely_equals(&Val::from(2));
                            if !matched11 && subject11.loosely_equals(&Val::from(1)) {
                                matched11 = true;
                            }
                            if matched11 {
                                'b12: {
                                    let subject12 = ctx.var("jupe_hist").get()?;
                                    let mut matched12 = false;
                                    let no_case12 = !subject12.loosely_equals(&Val::from(7))
                                        && !subject12.loosely_equals(&Val::from(11))
                                        && !subject12.loosely_equals(&Val::from(13))
                                        && !subject12.loosely_equals(&Val::from(14));
                                    if !matched12 && subject12.loosely_equals(&Val::from(7)) {
                                        matched12 = true;
                                    }
                                    if matched12 {
                                        scholar_run(
                                            ctx,
                                            ScholarStep::FuncJupHist,
                                            vec![Val::from(7355), Val::from(7352), Val::from(7353), Val::from(7354)],
                                        )?;
                                    }
                                    if !matched12 && subject12.loosely_equals(&Val::from(11)) {
                                        matched12 = true;
                                    }
                                    if matched12 {
                                        scholar_run(
                                            ctx,
                                            ScholarStep::FuncJupHist,
                                            vec![Val::from(7354), Val::from(7352), Val::from(7353), Val::from(7355)],
                                        )?;
                                    }
                                    if !matched12 && subject12.loosely_equals(&Val::from(13)) {
                                        matched12 = true;
                                    }
                                    if matched12 {
                                        scholar_run(
                                            ctx,
                                            ScholarStep::FuncJupHist,
                                            vec![Val::from(7353), Val::from(7352), Val::from(7354), Val::from(7355)],
                                        )?;
                                    }
                                    if !matched12 && subject12.loosely_equals(&Val::from(14)) {
                                        matched12 = true;
                                    }
                                    if matched12 {
                                        scholar_run(
                                            ctx,
                                            ScholarStep::FuncJupHist,
                                            vec![Val::from(7352), Val::from(7353), Val::from(7354), Val::from(7355)],
                                        )?;
                                    }
                                }
                            }
                            if !matched11 && subject11.loosely_equals(&Val::from(2)) {
                                matched11 = true;
                            }
                            if matched11 {
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "Ah, I see. Well, while",
                                        "you're here, why don't you",
                                        "read something? There are",
                                        "many books that cover some",
                                        "interesting topics, like",
                                        "modern adventure history."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "Oh, in any case, please",
                                        "don't forget the favor I asked",
                                        "of you. If you find anything",
                                        "in Juperos that's historically",
                                        "significant, I'd appreciate it",
                                        "if you bring it right away."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(10)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                ((Val::from("Oh hello, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                                "So what exactly brings you",
                                "to the Juno Library this time?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("I found another Transparent Plate.:Just visiting, really.")])? {
                            1 => {
                                if (((ctx.call(Function::CountItem, vec![Val::from(7352)])?.is_true()
                                    || ctx.call(Function::CountItem, vec![Val::from(7353)])?.is_true())
                                    || ctx.call(Function::CountItem, vec![Val::from(7354)])?.is_true())
                                    || ctx.call(Function::CountItem, vec![Val::from(7355)])?.is_true())
                                {
                                    ctx.lines_as(
                                        "Fayruz",
                                        args![
                                            "Hmm, well, we've made as",
                                            "much headway as we can",
                                            "with the Transparent Plates",
                                            "you've already given me, but",
                                            "it can't hurt to have too much",
                                            "evidence to back my theories."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Fayruz",
                                        args![
                                            "I really appreciate",
                                            "your continuing efforts",
                                            "to help me. Please, would",
                                            "you take this as my way",
                                            "saying ''Thanks?'' You've been",
                                            ((Val::from("great, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                + Val::from("..."))
                                        ],
                                    )?;
                                    if ctx.call(Function::CountItem, vec![Val::from(7352)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(7352), Val::from(1)])?;
                                    } else if ctx.call(Function::CountItem, vec![Val::from(7353)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(7353), Val::from(1)])?;
                                    } else if ctx.call(Function::CountItem, vec![Val::from(7354)])?.is_true() {
                                        ctx.call(Function::DelItem, vec![Val::from(7354), Val::from(1)])?;
                                    } else {
                                        ctx.call(Function::DelItem, vec![Val::from(7355), Val::from(1)])?;
                                    }
                                    ctx.call(Function::GetItem, vec![Val::from(644), Val::from(1)])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "Mmm...?",
                                        "It doesn't look like",
                                        "you brought another",
                                        "Transparent Plate.",
                                        "Are you sure that you",
                                        "didn't misplace it?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Fayruz",
                                    args![
                                        "Ah, I see. Well,",
                                        "thanks to your help,",
                                        "I've made a great deal",
                                        "of progress on my thesis.",
                                        "I really appreciate what you",
                                        "have done for me, adventurer."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                }
                step = ScholarStep::AfterFuncJupHist;
                continue 'machine;
            }
            ScholarStep::FuncJupHist => {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_arg, &Val::from(base + 0), runtime::arg(&args, 0, Val::from(0)), false);
                runtime::local_set(&mut l_arg, &Val::from(base + 1), runtime::arg(&args, 1, Val::from(0)), false);
                runtime::local_set(&mut l_arg, &Val::from(base + 2), runtime::arg(&args, 2, Val::from(0)), false);
                runtime::local_set(&mut l_arg, &Val::from(base + 3), runtime::arg(&args, 3, Val::from(0)), false);
                let subject14 = ctx.var("yuno_hist").get()?;
                if subject14 == 4 {
                    ctx.lines_as(
                        "Fayruz",
                        args![
                            "Thank you so much,",
                            "you don't know what",
                            "this means to me! Okay,",
                            "please relax and take a",
                            "seat. Close your eyes while",
                            "I tell you this ancient story."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFFayruz begins to",
                        "relate an ancient tale",
                        "about Juperos that seems",
                        "typical for a classic story, but her way of storytelling subtly",
                        "draws you into a vicarious, yet extremely vivid experience."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou feel the protagonist's",
                        "glories and tragedies as if",
                        "you were actually there with",
                        "the hero on his journeys. The",
                        "tale eventually comes to an end",
                        "and you awaken from the trance,",
                        "gently brought back to reality.^000000"
                    ])?;
                    ctx.call(
                        Function::DelItem,
                        vec![runtime::local_get(&l_arg, &Val::from(0), false), Val::from(1)],
                    )?;
                    ctx.var("yuno_hist").set(Val::from(5))?;
                    ctx.var("jupe_hist").set(runtime::local_get(&l_arg, &Val::from(1), false))?;
                    ctx.call(Function::GetExperience, vec![Val::from(100000), Val::from(0)])?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(11019), Val::from(11020)])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Fayruz",
                        args![
                            "Everyone can relate",
                            "to these old, classic",
                            "stories. I hope this tale had",
                            "as meaning for you as it did",
                            "for me when I first heard it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Fayruz",
                        args![
                            "If you can find me",
                            "another artifact from",
                            "Juperos, I'll share another",
                            "tale like that with you. Now",
                            "how does that sound? Okay",
                            "then, I'll see you, adventurer~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject14 == 5 {
                    if ((ctx
                        .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(0), false)])?
                        .is_true()
                        || ctx
                            .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(1), false)])?
                            .is_true())
                        || ctx
                            .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(2), false)])?
                            .is_true())
                    {
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Oh, that's unexpected.",
                                "This Transparent Plate",
                                "seems to have been made",
                                "in a different era than the",
                                "one you gave me earlier.",
                                "How intriguing..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "*Sigh* I really wish",
                                "that I could explore",
                                "Juperos on my own, but",
                                "I'm just not strong enough.",
                                "In a way, I'm quite jealous of you. But it can't be helped..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "You know, that reminds",
                                "me of this great story of",
                                "a tragic hero that I'd like to",
                                "share with you. Let your",
                                "mind wander as I relate this ageless, yet bittersweet tale..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFFayruz tells you a story",
                            "with a bright beginning, full",
                            "of hope that fills you with the",
                            "bliss of the heavens, but then",
                            "suddenly plummets you into all the despair and torment of hell.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFThe story finally",
                            "reaches its ending",
                            "and you're surprised",
                            "to find yourself sitting",
                            "in the Juno Library.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "I know it's a very",
                                "depressing story, but",
                                "I hope you enjoyed it.",
                                "I think you'd agree that",
                                "it contains a truth about",
                                "mankind that can't be ignored."
                            ],
                        )?;
                        if ctx
                            .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(0), false)])?
                            .is_true()
                        {
                            ctx.call(
                                Function::DelItem,
                                vec![runtime::local_get(&l_arg, &Val::from(0), false), Val::from(1)],
                            )?;
                            if runtime::local_get(&l_arg, &Val::from(0), false) == 7352 {
                                ctx.var("jupe_hist").set((ctx.var("jupe_hist").get()? + Val::from(1)))?;
                            } else {
                                ctx.var("jupe_hist").set((ctx.var("jupe_hist").get()? + Val::from(2)))?;
                            }
                        } else if ctx
                            .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(1), false)])?
                            .is_true()
                        {
                            ctx.call(
                                Function::DelItem,
                                vec![runtime::local_get(&l_arg, &Val::from(1), false), Val::from(1)],
                            )?;
                            if runtime::local_get(&l_arg, &Val::from(1), false) == 7353 {
                                ctx.var("jupe_hist").set((ctx.var("jupe_hist").get()? + Val::from(2)))?;
                            } else {
                                ctx.var("jupe_hist").set((ctx.var("jupe_hist").get()? + Val::from(4)))?;
                            }
                        } else {
                            ctx.call(
                                Function::DelItem,
                                vec![runtime::local_get(&l_arg, &Val::from(2), false), Val::from(1)],
                            )?;
                            if runtime::local_get(&l_arg, &Val::from(2), false) == 7354 {
                                ctx.var("jupe_hist").set((ctx.var("jupe_hist").get()? + Val::from(4)))?;
                            } else {
                                ctx.var("jupe_hist").set((ctx.var("jupe_hist").get()? + Val::from(8)))?;
                            }
                        }
                        ctx.var("yuno_hist").set(Val::from(6))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(11020), Val::from(11021)])?;
                        ctx.call(Function::GetExperience, vec![Val::from(100000), Val::from(0)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "By now I'm sure you've",
                                "figured that these classic",
                                "tales are like condensed",
                                "experiences, refined and",
                                "immutable truths that we",
                                "can see in our own reality."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "If you find more of",
                                "these Transparent",
                                "Plates in Juperos, I'd be",
                                "very happy to share another",
                                ((Val::from("story with you, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx
                        .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(3), false)])?
                        .is_true()
                    {
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Oh, this one seems",
                                "to have been created",
                                "in a similar era as the",
                                "one you gave me earlier.",
                                "I'm not sure how much new",
                                "information this may provide..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Still, I'm sure this will",
                                "helpful in my research. I just",
                                "won't be as making progress",
                                "as quickly as I had projected.",
                                "Please, I'd like you to take this as a token of my gratitude."
                            ],
                        )?;
                        ctx.call(
                            Function::DelItem,
                            vec![runtime::local_get(&l_arg, &Val::from(3), false), Val::from(1)],
                        )?;
                        ctx.call(Function::GetItem, vec![Val::from(644), Val::from(1)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Now if you'll excuse",
                                "me, I need to go back",
                                "to compiling my research...",
                                "Thank you so much for",
                                ((Val::from("your help, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Oh. There isn't anything",
                                "here that would help in my",
                                "research, but thank you anyway.",
                                "If you find anything else while",
                                "you're in Juperos, please come back and show it to me, alright?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else if subject14 == 7 {
                    if (ctx
                        .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(0), false)])?
                        .is_true()
                        || ctx
                            .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(1), false)])?
                            .is_true())
                    {
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Is this another",
                                "Transparent Plate?",
                                "Yes, it's quite different",
                                "than the last one you",
                                "brought over to me...",
                                "This is so exciting!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Oh. You must be thinking",
                                "that I'm a complete academia",
                                "addict. Well, my life might be",
                                "a little uneventful, but there",
                                "are other things I think about!",
                                "Like, well... It's weird but..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "You see, there's this",
                                "guy that I like. I'm not sure",
                                "where he might be now, but",
                                "his name is Nadim Amal. He's",
                                "my friend's brother who I first",
                                "met 10 years ago. ^333333*Sigh...*^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Just recently, I saw",
                                "him with his sister, my",
                                "friend from Morocc. It's",
                                "weird to think that I'd have",
                                "these feelings for him after",
                                "all this time, isn't it? Oh...!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "I really should repay",
                                "you for this Transparent",
                                "Plate. Why don't I tell you",
                                "the scariest story that I know?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFFayruz tells you a",
                            "creepy horror story that",
                            "makes you shiver with fear.",
                            "You've heard other ghost",
                            "stories, but you've never been",
                            "so deeply immersed in one before.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "It is only when the",
                            "story ends and you return",
                            "to your senses that you notice that you're soaked in cold sweat.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "It may be a natural",
                                "response, but all people",
                                "fear the unknown in one way",
                                "or another. Scary stories are",
                                "appealing because we actually",
                                "like the strange and grotesque."
                            ],
                        )?;
                        if ctx
                            .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(0), false)])?
                            .is_true()
                        {
                            ctx.call(
                                Function::DelItem,
                                vec![runtime::local_get(&l_arg, &Val::from(0), false), Val::from(1)],
                            )?;
                            if runtime::local_get(&l_arg, &Val::from(0), false) == 7352 {
                                ctx.var("jupe_hist").set((ctx.var("jupe_hist").get()? + Val::from(1)))?;
                            } else if runtime::local_get(&l_arg, &Val::from(0), false) == 7353 {
                                ctx.var("jupe_hist").set((ctx.var("jupe_hist").get()? + Val::from(2)))?;
                            } else {
                                ctx.var("jupe_hist").set((ctx.var("jupe_hist").get()? + Val::from(4)))?;
                            }
                        } else if ctx
                            .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(1), false)])?
                            .is_true()
                        {
                            ctx.call(
                                Function::DelItem,
                                vec![runtime::local_get(&l_arg, &Val::from(1), false), Val::from(1)],
                            )?;
                            if runtime::local_get(&l_arg, &Val::from(1), false) == 7353 {
                                ctx.var("jupe_hist").set((ctx.var("jupe_hist").get()? + Val::from(2)))?;
                            } else if runtime::local_get(&l_arg, &Val::from(1), false) == 7354 {
                                ctx.var("jupe_hist").set((ctx.var("jupe_hist").get()? + Val::from(4)))?;
                            } else {
                                ctx.var("jupe_hist").set((ctx.var("jupe_hist").get()? + Val::from(8)))?;
                            }
                        }
                        ctx.var("yuno_hist").set(Val::from(8))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(11021), Val::from(11022)])?;
                        ctx.call(Function::GetExperience, vec![Val::from(100000), Val::from(0)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Well... That's just my",
                                "opinion. Anyway, if you",
                                "find anything else in Juperos",
                                "that may help in my research,",
                                "please come back and show it",
                                "to me, alright? See you later~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx
                        .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(2), false)])?
                        .is_true()
                        || ctx
                            .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(3), false)])?
                            .is_true())
                    {
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Oh, this one seems",
                                "to have been created",
                                "in a similar era as the",
                                "one you gave me earlier.",
                                "I'm not sure how much new",
                                "information this may provide..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Still, I'm sure this will",
                                "helpful in my research. I just",
                                "won't be as making progress",
                                "as quickly as I had projected.",
                                "Please, I'd like you to take this as a token of my gratitude."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Now if you'll excuse",
                                "me, I need to go back",
                                "to compiling my research...",
                                "Thank you so much for",
                                ((Val::from("your help, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
                            ],
                        )?;
                        if ctx
                            .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(2), false)])?
                            .is_true()
                        {
                            ctx.call(
                                Function::DelItem,
                                vec![runtime::local_get(&l_arg, &Val::from(2), false), Val::from(1)],
                            )?;
                        } else if ctx
                            .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(3), false)])?
                            .is_true()
                        {
                            ctx.call(
                                Function::DelItem,
                                vec![runtime::local_get(&l_arg, &Val::from(3), false), Val::from(1)],
                            )?;
                        }
                        ctx.call(Function::GetItem, vec![Val::from(644), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Oh. There isn't anything",
                                "here that would help in my",
                                "research, but thank you anyway.",
                                "If you find anything else while",
                                "you're in Juperos, please come back and show it to me, alright?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else if subject14 == 9 {
                    if ctx
                        .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(0), false)])?
                        .is_true()
                    {
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Oooh...! This one is",
                                "much different than the",
                                "other ones you gave me",
                                "before. This should provide",
                                "a wealth of brand new insights",
                                "into the Juperos civilization!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "All the Transparent",
                                "Plates you've given me",
                                "should contain more than",
                                "enough data for me to fully",
                                "complete my research thesis.",
                                "Once again, thank you so much~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Still, that doesn't mean that",
                                "I will stop collecting data for",
                                "my research. Anyway, I have one last story to tell you, about",
                                "a man of pure heart chosen by the gods to serve and protect mankind."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "For this purpose he was given",
                                "gaudy armor which contained",
                                "amazing powers, as well as a",
                                "book detailing the instructions",
                                "for its use. However, he promptly",
                                "lost these instructions..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFThe story about the",
                            "greatest Juperosian hero",
                            "that Fayruz tells you is very",
                            "humorous at first, but then it",
                            "covers the entire spectrum",
                            "of emotion and humanity..."
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFThe story ends and you",
                            "are left with a swelling",
                            "feeling of indefatigable",
                            "hope and inspiration...",
                            "You can make it if you try!^000000"
                        ])?;
                        if runtime::local_get(&l_arg, &Val::from(0), false) == 7352 {
                            ctx.var("jupe_hist").set((ctx.var("jupe_hist").get()? + Val::from(1)))?;
                        } else if runtime::local_get(&l_arg, &Val::from(0), false) == 7353 {
                            ctx.var("jupe_hist").set((ctx.var("jupe_hist").get()? + Val::from(2)))?;
                        } else if runtime::local_get(&l_arg, &Val::from(0), false) == 7354 {
                            ctx.var("jupe_hist").set((ctx.var("jupe_hist").get()? + Val::from(4)))?;
                        } else {
                            ctx.var("jupe_hist").set((ctx.var("jupe_hist").get()? + Val::from(8)))?;
                        }
                        ctx.var("yuno_hist").set(Val::from(10))?;
                        ctx.call(Function::CompleteQuest, vec![Val::from(11022)])?;
                        ctx.call(
                            Function::DelItem,
                            vec![runtime::local_get(&l_arg, &Val::from(0), false), Val::from(1)],
                        )?;
                        ctx.call(Function::GetExperience, vec![Val::from(100000), Val::from(0)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Well, that is the",
                                "last and the best story",
                                "that I have to share",
                                "with you. Perhaps next",
                                "time, I'll fill you in on my",
                                "research progress~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ((ctx
                        .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(1), false)])?
                        .is_true()
                        || ctx
                            .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(2), false)])?
                            .is_true())
                        || ctx
                            .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(3), false)])?
                            .is_true())
                    {
                        ctx.lines_as(
                            "Fayruz",
                            args!["Hmm...", "This one seems to be created in a similar time", "as the previous one."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Don't you worry.",
                                "This will help my research of course,",
                                "although I do not think this will",
                                "help me in advancing my research",
                                "with a great speed unlike this other one."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Fayruz", args!["Please take this as a token of my gratitude."])?;
                        ctx.next()?;
                        ctx.lines_as("Fayruz", args!["Now, excuse me. I need to go back to my research."])?;
                        if ctx
                            .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(1), false)])?
                            .is_true()
                        {
                            ctx.call(
                                Function::DelItem,
                                vec![runtime::local_get(&l_arg, &Val::from(1), false), Val::from(1)],
                            )?;
                        } else if ctx
                            .call(Function::CountItem, vec![runtime::local_get(&l_arg, &Val::from(2), false)])?
                            .is_true()
                        {
                            ctx.call(
                                Function::DelItem,
                                vec![runtime::local_get(&l_arg, &Val::from(2), false), Val::from(1)],
                            )?;
                        } else {
                            ctx.call(
                                Function::DelItem,
                                vec![runtime::local_get(&l_arg, &Val::from(3), false), Val::from(1)],
                            )?;
                        }
                        ctx.call(Function::GetItem, vec![Val::from(644), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Fayruz",
                            args![
                                "Oh. There isn't anything",
                                "here that would help in my",
                                "research, but thank you anyway.",
                                "If you find anything else while",
                                "you're in Juperos, please come back and show it to me, alright?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                return Ok(Val::from(0));
            }
            ScholarStep::AfterFuncJupHist => {
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn scholar(ctx: &Ctx) -> Script {
    scholar_run(ctx, ScholarStep::Start, Vec::new()).map(|_| ())
}
