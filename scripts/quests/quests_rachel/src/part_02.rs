use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn katinshuell_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.var("rach_vice").get()?.number()? > 21 {
        ctx.lines_as(
            "Katinshuell",
            args![
                "If I had only turned",
                "myself in... Maybe if I had",
                "made an effort to pay for",
                "my crime, me and Bruspetti",
                "could have had a chance..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if (ctx.var("rach_vice").get()? == 21 && ctx.call(Function::CountItem, vec![Val::from(1201)])?.number()? > 0) {
            ctx.lines_as(
                "Katinshuell",
                args![
                    "Please... Please just",
                    "leave me alone. I've lost",
                    "the woman I love because",
                    "of something stupid I did",
                    "in the past. If you want to",
                    "turn me in, go ahead..."
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(1201), Val::from(1)])?;
            ctx.var("rach_vice").set(Val::from(22))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(8121), Val::from(8122)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("rach_vice").get()? == 21 {
                ctx.lines_as(
                    "Katinshuell",
                    args![
                        "I suppose it's my",
                        "fate to bear this guilt.",
                        "It's already destroyed my",
                        "best chance of ever being",
                        "truly happy. Bruspetti..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if (ctx.var("rach_vice").get()? == 20 && ctx.call(Function::CountItem, vec![Val::from(1201)])?.number()? > 0) {
                    ctx.lines_as(
                        "Katinshuell",
                        args![
                            "I suppose there's",
                            "no reason to hide",
                            "anything anymore...",
                            "You've probably figured",
                            "the important stuff by now..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Katinshuell",
                        args![
                            "I used to work as one",
                            "of the security guards in",
                            "Lighthalzen. We basically",
                            "watched the border between",
                            "the rich area and the slums."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Katinshuell",
                        args![
                            "I actually was ordered",
                            "to kill someone that was",
                            "repeatedly moving between",
                            "the slums and the rich area.",
                            "He was a wealthy kid...",
                            "Didn't really deserve it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["......", "........."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Katinshuell",
                        args![
                            "I was paid a lot of money",
                            "to do it. At first, I thought",
                            "I was just doing my job. You",
                            "know, I mean, the law was on",
                            "my side. But the boy's blood",
                            "wouldn't wash off my hands..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["......", "........."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Katinshuell",
                        args![
                            "I was scared to kill him,",
                            "but once I did it, the guilt",
                            "and the torment has been",
                            "unbearable! I haven't been",
                            "able to sleep... That's why",
                            "I had to leave Lighthalzen."
                        ],
                    )?;
                    ctx.var("rach_vice").set(Val::from(21))?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["......", "........."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Katinshuell",
                        args![
                            "Bruspetti shocked me when",
                            "she had learned about what",
                            "I did back then. My one true",
                            "love... She couldn't bear the",
                            "truth. When she heard it from",
                            "my own lips, she went mad..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["......", "........."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Katinshuell",
                        args![
                            "I couldn't stop her...",
                            "I struggled, and tried to",
                            "save her, but she managed",
                            "to drown herself in Freya's",
                            "Spring. She's gone from my",
                            "life. Just like that. Forever."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["......", "........."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Katinshuell",
                        args![
                            "I came here after",
                            "killing a man to start",
                            "a new life, maybe get",
                            "a clean slate, but I end up",
                            "indirectly killing the woman",
                            "I love. Why God?! Why?!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["......", "........."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Katinshuell",
                        args![
                            "Are you happy now?",
                            "Are you happy now that?",
                            "I've told you the truth?",
                            "Doesn't that make me and",
                            "you feel so much better?!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["......", "........."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Katinshuell", args!["...............................", "God! My life sucks!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("rach_vice").get()? == 21 {
                        ctx.lines_as(
                            "Katinshuell",
                            args![
                                "^333333*Pant pant*^000000",
                                "After all this",
                                "time... I thought",
                                "I could run away...",
                                "But the voices still",
                                "h-haunt me... Ha ha ha..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if (ctx.var("rach_vice").get()? == 20 && ctx.call(Function::CountItem, vec![Val::from(1201)])?.number()? > 0) {
                            ctx.lines_as(
                                "Katinshuell",
                                args![
                                    "I...",
                                    "I don't deserve this!",
                                    "Why do you keep hounding",
                                    "me with these questions?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Did... Did something",
                                    "happen when Bruspetti",
                                    "confronted you at Freya's",
                                    "Spring? What did exactly",
                                    "did she learn about",
                                    "you in Lighthalzen?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Katinshuell", args!["I... I don't have", "to tell you anything!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "How can you say that?!",
                                    "I just read in Bruspetti's",
                                    "diary that she learned",
                                    "something horrible about",
                                    "you, and it involved a",
                                    "Knife... just like this one."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Katinshuell",
                                args![
                                    ".........!",
                                    ".............",
                                    "NOoo! I'm sorry, it's",
                                    "my fault! I was desperate!",
                                    "You don't understand how",
                                    "I used to live, you d-don't--!"
                                ],
                            )?;
                            ctx.var("rach_vice").set(Val::from(21))?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Huh...?", "Mr. Katinshuell?"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if (ctx.var("rach_vice").get()? == 19 || ctx.var("rach_vice").get()? == 20) {
                                ctx.lines_as(
                                    "Katinshuell",
                                    args![
                                        "I...",
                                        "I don't deserve this!",
                                        "Why do you keep hounding",
                                        "me with these questions?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Did... Did something",
                                        "happen when Bruspetti",
                                        "confronted you at Freya's",
                                        "Spring? What did exactly",
                                        "did she learn about",
                                        "you in Lighthalzen?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Katinshuell", args!["I... I don't have", "to tell you anything!"])?;
                                ctx.var("rach_vice").set(Val::from(20))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(8120), Val::from(8121)])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "(^333333He's still resisting",
                                        "me... How can I get",
                                        "him to reveal the truth?^000000)"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("rach_vice").get()? == 18 {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "Let's see...",
                                            "Oh this entry looks",
                                            "interesting. It's all",
                                            "about you, Katinshuell."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "- Date: OOXX -",
                                        "I'm so in love with him,",
                                        "but he always changes the",
                                        "subject whenever I ask him",
                                        "personal questions about his",
                                        "past. Could it be that he's",
                                        "hiding something from me?"
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "- Date: OOXX -",
                                        "Is it another woman?",
                                        "I can't help but feel",
                                        "jealous! I need to know.",
                                        "That's why I've decided to",
                                        "go to Lighthalzen and see",
                                        "what I can find out."
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "- Date: OOXO -",
                                        "Dear Diary,",
                                        "Today I just learned",
                                        "the horrible truth... I need",
                                        "to make him confess it to",
                                        "me. I hope we can still be",
                                        "together after all of this..."
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "- Date: OOXO -",
                                        "But I don't know if",
                                        "I can live with this!",
                                        "The man I love... It's",
                                        "unthinkable that he'd",
                                        "use a Knife t-to... I just",
                                        "want to throw up."
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "- Date: OOXO -",
                                        "I'm planning to meet him",
                                        "tomorrow at Freya's Spring.",
                                        "I can't help but look at him",
                                        "differently now, but still...",
                                        "He's the man I truly love."
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "- Date: OOXO -",
                                        "I hope that I have",
                                        "good news the next",
                                        "time I write in this diary...",
                                        "Well, here's hoping."
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "Well, that was",
                                            "the very last page...",
                                            "What happened when",
                                            "Bruspetti confronted",
                                            "you at Freya's Spring?"
                                        ],
                                    )?;
                                    ctx.var("rach_vice").set(Val::from(19))?;
                                    ctx.next()?;
                                    ctx.lines_as("Katinshuell", args![".........", "I... I...", "........."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("rach_vice").get()? == 17 {
                                        ctx.lines_as(
                                            "Katinshuell",
                                            args![
                                                "Damn it! I don't want to",
                                                "think about her anymore!",
                                                "Get away from me, and take",
                                                "Bruspetti's diary with you!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args![
                                                "What?! No way, not after",
                                                "that little outburst about",
                                                "killing or not killing Bruspetti!",
                                                "Fine, if you're not going to",
                                                "talk, then I'm going to flip",
                                                "through this diary for answers."
                                            ],
                                        )?;
                                        ctx.var("rach_vice").set(Val::from(18))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.var("rach_vice").get()? == 16 {
                                            ctx.lines_as(
                                                "Katinshuell",
                                                args![
                                                    "Ha ha ha...",
                                                    "Is this some sort",
                                                    "of interrogation?",
                                                    "I-I've done nothing",
                                                    "wrong! Go ahead,",
                                                    "ask me anything!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["Alright then, why", "don't you explain this?"],
                                            )?;
                                            ctx.next()?;
                                            'b1: {
                                                let subject1 = Val::from(runtime::select_values(
                                                    ctx,
                                                    &[Val::from("Mr. Shendar's House:Lighthalzen:Freya's Spring:Bruspetti")],
                                                )?);
                                                let mut matched1 = false;
                                                let no_case1 = !subject1.loosely_equals(&Val::from(1))
                                                    && !subject1.loosely_equals(&Val::from(2))
                                                    && !subject1.loosely_equals(&Val::from(3))
                                                    && !subject1.loosely_equals(&Val::from(4));
                                                if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                                                    matched1 = true;
                                                }
                                                if matched1 {
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args![
                                                            "Mr. Shendar, Bruspetti's",
                                                            "father, says that someone",
                                                            "has been sneaking into his",
                                                            "house for some reason.",
                                                            "Now, why would you do that?"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args![
                                                            "You recognized",
                                                            "Bruspetti's diary",
                                                            "pretty quickly...",
                                                            "And you did mention",
                                                            "you were looking for it."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Katinshuell",
                                                        args![
                                                            "Hmpf! S-stop",
                                                            "talking crazy talk!",
                                                            "I don't know what",
                                                            "you're talking about!"
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                                                    matched1 = true;
                                                }
                                                if matched1 {
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args![
                                                            "You know, I found out",
                                                            "that Bruspetti was planning",
                                                            "on going to Lighthalzen.",
                                                            "It seems that she needed",
                                                            "to learn something really",
                                                            "important over there..."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Katinshuell", args!["No...!", "She couldn't have...!"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args![
                                                            "Alright. It's no use",
                                                            "pretending that you don't",
                                                            "know her. You were her",
                                                            "boyfriend, weren't you?",
                                                            "What would she be trying",
                                                            "to find in Lighthalzen?"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Katinshuell",
                                                        args![
                                                            "No! No no no!",
                                                            "Shut up! Shut up!",
                                                            "Please! Just leave",
                                                            "me alone! Get away",
                                                            "from me right now!"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    match runtime::select_values(
                                                        ctx,
                                                        &[Val::from("Mr. Shendar's House:Freya's Spring:Recent Break-up")],
                                                    )? {
                                                        1 => {
                                                            ctx.lines_as(
                                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                args![
                                                                    "So while she was away",
                                                                    "in Lighthalzen, you snuck",
                                                                    "in Mr. Shendar's house and--"
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Katinshuell",
                                                                args![
                                                                    "Y-yes! Yes, that's right!",
                                                                    "It's because I was so worried",
                                                                    "about her! I had no idea that",
                                                                    "she went all the way over to",
                                                                    "Lighthalzen! Ha ha! Ha ha!"
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                args![
                                                                    "...............................",
                                                                    "(^333333Nuts! I think that",
                                                                    "backfired, so I'm going",
                                                                    "to have to try this again.)^000000"
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                        2 => {
                                                            ctx.lines_as(
                                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                args![
                                                                    "So while she was away",
                                                                    "in Lighthalzen, you went",
                                                                    "to Freya's Spring, all",
                                                                    "by yourself, didn't you?!"
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Katinshuell", args!["......", ".........", "Um? ...Yes."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                args![
                                                                    "...............................",
                                                                    "(^333333Nuts! I think that",
                                                                    "backfired, so I'm going",
                                                                    "to have to try this again.)^000000"
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                        3 => {
                                                            ctx.lines_as(
                                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                args![
                                                                    "Wait a second! Your break-up",
                                                                    "and Bruspetti's sudden need",
                                                                    "to investigate something in",
                                                                    "Lighthalzen... They're related",
                                                                    "somehow, aren't they?"
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Katinshuell",
                                                                args!["......", ".........", "No! It's not true!"],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines(args![
                                                                ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from("]"))
                                                            ])?;
                                                            ctx.next()?;
                                                            match runtime::select_values(
                                                                ctx,
                                                                &[Val::from("Mr. Shendar's house:Freya's Spring")],
                                                            )? {
                                                                1 => {
                                                                    ctx.lines_as(
                                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                        args![
                                                                            "So while she was away",
                                                                            "in Lighthalzen, you snuck",
                                                                            "in Mr. Shendar's house and--"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Katinshuell",
                                                                        args![
                                                                            "Y-yes! Yes, that's right!",
                                                                            "It's because I was so worried",
                                                                            "about her! I had no idea that",
                                                                            "she went all the way over to",
                                                                            "Lighthalzen! Ha ha! Ha ha!"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                        args![
                                                                            "...............................",
                                                                            "(^333333Nuts! I think that",
                                                                            "backfired, so I'm going",
                                                                            "to have to try this again.)^000000"
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                                2 => {
                                                                    ctx.lines_as(
                                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                        args![
                                                                            "Hmm... Why are you",
                                                                            "always hanging out at",
                                                                            "Freya's Spring alone?",
                                                                            "And why has Bruspetti",
                                                                            "not returned home yet?"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Katinshuell",
                                                                        args![
                                                                            "...Will you stop talking nonsense?",
                                                                            "What is your evidence to convict me with the crime?",
                                                                            "I don't wish to hear you any longer."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                        args![
                                                                            "Even after you killed Bruspetti,",
                                                                            "you became worried about another possibility.",
                                                                            "So you were plotting to get rid of the possibility, too."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.var("rach_vice").set(Val::from(17))?;
                                                                    ctx.lines_as(
                                                                        "Katinshuell",
                                                                        args![
                                                                            "No! Stop it! Stop!",
                                                                            "Are you implying that",
                                                                            "I killed her?! I didn't!",
                                                                            "Quit this nonsense",
                                                                            "before I get really angry!"
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                                _ => {}
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                                                    matched1 = true;
                                                }
                                                if matched1 {
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args![
                                                            "Freya's Spring.",
                                                            "Why are you always",
                                                            "hanging around there",
                                                            "by yourself, eh?"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Katinshuell",
                                                        args![
                                                            "...............................",
                                                            "It's because I really",
                                                            "miss my ex-girlfriend."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args!["Oh. Right.", "That's a pretty", "good reason."],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                if !matched1 && subject1.loosely_equals(&Val::from(4)) {
                                                    matched1 = true;
                                                }
                                                if matched1 {
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args!["If Bruspetti is not", "your girlfriend, then...", "Who is?! Answer that!"],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Katinshuell",
                                                        args![
                                                            "...............................",
                                                            "Fine, fine. You win.",
                                                            "She was my ex-girlfriend.",
                                                            "I just don't like talking",
                                                            "about Bruspetti, that's all.",
                                                            "Now will you leave me alone?"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args![
                                                            "(^333333Nuts! He wasn't supposed",
                                                            "to cave in like that! I need",
                                                            "to question him again until",
                                                            "my gut feeling is satisfied!^000000)"
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                        } else {
                                            if (ctx.var("rach_vice").get()? == 15
                                                && ctx.call(Function::CountItem, vec![Val::from(7571)])?.is_true())
                                            {
                                                ctx.lines_as(
                                                    "Katinshuell",
                                                    args![
                                                        "Oh, it's you again.",
                                                        "What do you want now?",
                                                        "I just told you that I'm not",
                                                        "in the mood for talking with",
                                                        "anyone. I'm still coping with",
                                                        "breaking up with my girlfriend."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args![
                                                        "Katinshuell... I get the",
                                                        "feeling that you're hiding",
                                                        "something. By any chance,",
                                                        "was Bruspetti your girlfriend?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Katinshuell",
                                                    args![
                                                        "Huh?! Er, wh-wh-what?",
                                                        "What makes you think that?",
                                                        "That's none of your business!",
                                                        "Besides, don't you think that",
                                                        "a girl like her is too good",
                                                        "for some guy like me?!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args![
                                                        "I don't have any real",
                                                        "reason to suspect you of",
                                                        "anything, but my gut feeling",
                                                        "and the way you reacted isn't",
                                                        "very reassuring. Well, I guess",
                                                        "I can flip through this book..."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Katinshuell",
                                                    args![
                                                        "Hey! Hey that's...",
                                                        "That's Bruspetti's diary!",
                                                        "I-I've been looking for-- um..."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args![
                                                        "Hey! How would you",
                                                        "know about that?!",
                                                        "Tell me the truth!",
                                                        "Actually, you know what?",
                                                        "Why don't we look at the",
                                                        "truth together? How's that?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines(args![
                                                    "^3355FFYou nonchalantly",
                                                    "toss the diary at",
                                                    "Katinshuell's feet,",
                                                    "and it opens to one of",
                                                    "the pages in the middle.^000000"
                                                ])?;
                                                ctx.next()?;
                                                ctx.lines_as("Katinshuell", args!["......!"])?;
                                                ctx.call(Function::DelItem, vec![Val::from(7571), Val::from(1)])?;
                                                ctx.var("rach_vice").set(Val::from(16))?;
                                                ctx.call(Function::ChangeQuest, vec![Val::from(8119), Val::from(8120)])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                if (ctx.var("rach_vice").get()? == 10 || ctx.var("rach_vice").get()? == 11) {
                                                    ctx.lines_as(
                                                        "Katinshuell",
                                                        args![
                                                            "Argh! I'm so depressed.",
                                                            "Honestly, I just want to",
                                                            "be left by myself for a really",
                                                            "long time. What the heck do you",
                                                            "want to ask me about, anyway?"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    let (input, status) = runtime::input_text(ctx, None, None)?;
                                                    l_input_s = input;
                                                    if l_input_s.clone() == "Lighthalzen" {
                                                        ctx.lines_as(
                                                            "Katinshuell",
                                                            args![
                                                                "Lighthalzen...?",
                                                                "a city in the Schwarzwald",
                                                                "Republic. That, and it's",
                                                                "a tough place to live in.",
                                                                "That's all I really know..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                    } else if l_input_s.clone() == "Bruspetti" {
                                                        ctx.lines_as(
                                                            "Katinshuell",
                                                            args![
                                                                "...It's been a while",
                                                                "since I heard that name.",
                                                                "Bruspetti... She's pretty",
                                                                "popular around this town...",
                                                                "Everyone liked her. Is that",
                                                                "all you wanted to know?"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                    } else if l_input_s.clone() == "Freya's Spring" {
                                                        ctx.lines_as(
                                                            "Katinshuell",
                                                            args![
                                                                "Freya's Spring...",
                                                                "That's just a park",
                                                                "where old people and",
                                                                "couples sort of hang out.",
                                                                "I don't like going there,",
                                                                "though. You shouldn't, either."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                    } else {
                                                        ctx.lines_as(
                                                            "Katinshuell",
                                                            args![
                                                                ((Val::from("...") + l_input_s.clone()) + Val::from("?")),
                                                                "What? I don't understand you..."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    if ctx.var("rach_vice").get()? == 10 {
                                                        ctx.var("rach_vice").set(Val::from(11))?;
                                                        ctx.call(Function::ChangeQuest, vec![Val::from(8114), Val::from(8115)])?;
                                                    }
                                                    ctx.lines_as(
                                                        "Katinshuell",
                                                        args![
                                                            "You happy, now?",
                                                            "Quit trying to pry into",
                                                            "my personal business,",
                                                            "and just enjoy touring this",
                                                            "little town. That's what",
                                                            "you came to do, right?"
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    if ctx.var("rach_vice").get()? == 4 {
                                                        ctx.lines_as(
                                                            "Katinshuell",
                                                            args![
                                                                "Uh... ",
                                                                "I just told you that I broke",
                                                                "up with my girlfriend. You",
                                                                "know, I kinda sorta wanna",
                                                                "be left alone. Go away..."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else if ctx.var("rach_vice").get()? == 3 {
                                                        ctx.lines_as(
                                                            "Katinshuell",
                                                            args![
                                                                "Eh? Oh, it's one of you",
                                                                "guys, those do-gooder",
                                                                "adventurers. You must not",
                                                                "have anything else to do...",
                                                                "Otherwise, why talk to me?"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Katinshuell",
                                                            args![
                                                                "Geez... I know you guys go",
                                                                "around solving problems,",
                                                                "but this is one thing you",
                                                                "can't handle. Me and my",
                                                                "girlfriend are history now.",
                                                                "It's over between us."
                                                            ],
                                                        )?;
                                                        ctx.var("rach_vice").set(Val::from(4))?;
                                                        ctx.call(Function::ChangeQuest, vec![Val::from(8107), Val::from(8108)])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else if ctx.var("rach_vice").get()? == 2 {
                                                        ctx.lines_as(
                                                            "Katinshuell",
                                                            args![
                                                                "You're an adventurer,",
                                                                "so I don't think you'd ",
                                                                "understand how difficult",
                                                                "it is to live a quiet, peaceful",
                                                                "life. When you've lived through",
                                                                "certain things, it's tough."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Katinshuell",
                                                            args![
                                                                "When you're busy, you're",
                                                                "distracted. But even if I'm",
                                                                "left all alone, these thoughts",
                                                                "of mine never cease to haunt me..."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else if ctx.var("rach_vice").get()? == 1 {
                                                        ctx.lines_as(
                                                            "Katinshuell",
                                                            args![
                                                                "You're an adventurer,",
                                                                "so I don't think you'd ",
                                                                "understand how difficult",
                                                                "it is to live a quiet, peaceful",
                                                                "life. When you've lived through",
                                                                "certain things, it's tough."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Katinshuell",
                                                            args![
                                                                "When you're busy, you're",
                                                                "distracted. But even if I'm",
                                                                "left all alone, these thoughts of mine never cease to haunt me..."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else if ctx.var("rach_vice").get()? == 0 {
                                                        ctx.lines_as(
                                                            "Katinshuell",
                                                            args![
                                                                "You're an adventurer,",
                                                                "so I don't think you'd ",
                                                                "understand how difficult",
                                                                "it is to live a quiet, peaceful",
                                                                "life. When you've lived through",
                                                                "certain things, it's tough."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Katinshuell",
                                                            args![
                                                                "I don't need any more",
                                                                "excitement in my life.",
                                                                "I just want to live quietly,",
                                                                "away from other people,",
                                                                "and to be left all alone for",
                                                                "some semblance of peace."
                                                            ],
                                                        )?;
                                                        if ctx.var("friendship").get()?.number()? > 10 {
                                                            ctx.var("rach_vice").set(Val::from(1))?;
                                                        }
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ctx.lines_as(
        "Katinshuell",
        args![
            "Do you believe in karma?",
            "I do... No matter what you",
            "do, even if you don't get",
            "caught, somehow it just",
            "catches up to you."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Katinshuell",
        args![
            "Have you ever felt",
            "guilty about something,",
            "but hid from the truth?",
            "Let me tell you that it must",
            "be a painful experience. It's",
            "better to confess when you can."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn katinshuell(ctx: &Ctx) -> Script {
    katinshuell_body(ctx, Vec::new()).map(|_| ())
}

fn mr_shendar_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rach_vice").get()?.number()? > 21 {
        ctx.lines_as(
            "Mr. Shendar",
            args![
                "When will my precious",
                "daughter Bruspetti be",
                "coming home? I'm sure that",
                "she can take care of herself,",
                "but a father can't help but",
                "worry himself to death."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mr. Shendar",
            args![
                "Ah, if you happen to",
                "see my daughter in your",
                "travels, please tell her",
                "that her daddy is waiting",
                "for her to come home.",
                "Thanks, adventurer."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("rach_vice").get()? == 14 {
            ctx.lines_as(
                "Mr. Shendar",
                args!["So you have a pretty", "good idea of who was", "sneaking around my house?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Well, I suspect this guy",
                    "named Katinshuell because",
                    "he might have been Bruspetti's",
                    "boyfriend and he's been acting",
                    "funny. Still, I don't have any",
                    "real evidence of that."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mr. Shendar",
                args![
                    "Bruspetti's boyfriend?!",
                    "Hmpf! I don't like that",
                    "boy already. I haven't",
                    "met him yet, but its my",
                    "solemn duty as a father",
                    "to hate and distrust him!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("rach_vice").get()? == 13 {
                ctx.lines_as(
                    "Mr. Shendar",
                    args![
                        "Hey! Hey! Are you",
                        "the pervert that's been",
                        "sneaking into my house",
                        "to prey on my daughter?!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Wha--?", "No, I'm just a--"],
                )?;
                ctx.next()?;
                ctx.call(Function::PercentHeal, vec![Val::from(-50), Val::from(0)])?;
                ctx.mes("^3355FF*BAM!*^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Ow! What did", "you do that for?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Shendar",
                    args![
                        "Still alive, eh? Hmpf!",
                        "I'll finish you off! I'll...",
                        "Er, huh?! Wait, I think",
                        "I know you. You're that",
                        "adventurer that's been",
                        "here before, right?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Shendar",
                    args![
                        "Oh... I think I've made",
                        "a mistake. I thought you",
                        "were that person that's",
                        "been prowling around my",
                        "house. I don't know who he,",
                        "but I'm sure it's not you..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Wait, wait...",
                        "So someone has been",
                        "sneaking around here?",
                        "That's weird. Wait, that",
                        "guy, the one that might",
                        "be Bruspetti's boyfriend..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Right, his name was",
                        "Katinshuell. That guy's",
                        "been acting really funny.",
                        "Maybe he was the one sneaking",
                        "around here? I'd better go",
                        "and ask him about this..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Shendar",
                    args!["Wait, where are", "you going? I... I'm", "sorry for hitting you?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Shendar",
                    args![
                        "Curses! If that guy is my",
                        "daughter's boyfriend, and he's",
                        "trespassing into my home, then",
                        "he could be really dangerous.",
                        "It's too bad that I can't ask",
                        "Bruspetti anything about him..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Shendar",
                    args![
                        "Hm... Alright, adventurer.",
                        "If you can find it, I'll allow",
                        "you to take and read my",
                        "daughter's diary. We need",
                        "to learn more about this guy",
                        "Katinshuell. If he's dangerous..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Shendar",
                    args![
                        "I'm sorry... I'm asking you",
                        "for your help... But there's",
                        "nothing else I can do as her",
                        "father. Besides, adventurers",
                        "like you can offer her the",
                        "best protection..."
                    ],
                )?;
                ctx.var("rach_vice").set(Val::from(14))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8117), Val::from(8118)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("rach_vice").get()? == 9 {
                    ctx.lines_as(
                        "Mr. Shendar",
                        args![
                            "Curses! I should have never",
                            "mentioned my daughter's",
                            "diary to an adventurer like",
                            "you! I know that your kind",
                            "is naturally too curious...!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("rach_vice").get()? == 8 {
                    ctx.lines_as(
                        "Mr. Shendar",
                        args![
                            "What? My daughter went",
                            "to Lighthalzen? She doesn't",
                            "know anyone there. Hmm, maybe",
                            "that's the place where she was",
                            "planning to go to learn more",
                            "about what was bothering her."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Shendar",
                        args![
                            "Yes, she was very generic:",
                            "''Dad, I need to go somewhere.''",
                            "and ''There's something I need",
                            "to make sure of.'' Leaving out",
                            "all the details does not put",
                            "a doting father at ease!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Shendar",
                        args![
                            "Well, thank you for letting",
                            "me know where she might be.",
                            "Now, I wish I knew what she",
                            "was planning on doing. Maybe",
                            "she wrote something about it",
                            "in her diary? Yes, perhaps..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Shendar",
                        args![
                            "No! I can't do it!",
                            "As a loving father, I can't",
                            "allow myself to invade my",
                            "precious daughter's privacy.",
                            "Even if she did carelessly leave",
                            "her diary on top of her drawer."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Shendar",
                        args!["Oh, but how a father", "worries. Will my sweet,", "darling Bruspetti be alright?"],
                    )?;
                    ctx.var("rach_vice").set(Val::from(9))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(8112), Val::from(8113)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("rach_vice").get()? == 4 || ctx.var("rach_vice").get()? == 5) {
                    ctx.lines_as(
                        "Mr. Shendar",
                        args![
                            "I miss my daughter. Yes,",
                            "the house smells different",
                            "without her around. It's a little",
                            "weird to be talking about this,",
                            "but Bruspetti always did smell",
                            "nice, just like her mother."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Shendar",
                        args![
                            "Even if they didn't wear",
                            "perfume, they had this",
                            "distinctively pleasant",
                            "scent about them. I guess",
                            "it must be pheremonal?"
                        ],
                    )?;
                    if ctx.var("rach_vice").get()? == 4 {
                        ctx.var("rach_vice").set(Val::from(5))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(8108), Val::from(8109)])?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("rach_vice").get()? == 2 {
                    ctx.lines_as(
                        "Mr. Shendar",
                        args![
                            "Take my advice: never",
                            "have a daughter! You worry",
                            "too much about them, and they",
                            "neglect their parents once",
                            "they grow up! I bet she's",
                            "having a grand old time..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Shendar",
                        args![
                            "Confound it...!",
                            "Whoever that guy is,",
                            "he better not try any funny",
                            "business! As her father, I have",
                            "the legal right to wring his",
                            "little neck! At least, I should..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("rach_vice").get()? == 1 {
                    ctx.lines_as(
                        "Mr. Shendar",
                        args![
                            "I just found out that",
                            "my precious daughter has",
                            "been going out with someone.",
                            "She went to some other town",
                            "for this boy, and I haven't",
                            "heard a word from her since!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Shendar",
                        args![
                            "Take my advice: never",
                            "have a daughter! You worry",
                            "too much about them, and they",
                            "neglect their parents once",
                            "they grow up! I bet she's",
                            "having a grand old time..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Shendar",
                        args![
                            "Probably on some lovey",
                            "dovey trip with whoever that",
                            "boy is. She's been gone an",
                            "awful long time, but she's",
                            "also an adult now. I... I guess",
                            "she should be just fine."
                        ],
                    )?;
                    ctx.var("rach_vice").set(Val::from(2))?;
                    ctx.call(Function::SetQuest, vec![Val::from(8106)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
    ctx.lines_as(
        "Mr. Shendar",
        args![
            "I just found out that",
            "my precious daughter has",
            "been going out with someone.",
            "She went to some other town",
            "to see this boy, and I haven't",
            "heard a word from her since!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mr. Shendar",
        args![
            "Take my advice: never",
            "have a daughter! You worry",
            "too much about them, and they",
            "neglect their parents once",
            "they grow up! I bet she's",
            "having a grand old time..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mr_shendar(ctx: &Ctx) -> Script {
    mr_shendar_body(ctx, Vec::new()).map(|_| ())
}

fn lachellen_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.var("rach_vice").get()?.number()? > 21 {
        ctx.lines_as(
            "Lachellen",
            args![
                "If you happen to see",
                "Bruspetti, tell her to",
                "come talk to me. I need",
                "to know whether I figured",
                "out who her boyfriend is!",
                "I can't wait to see her~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("rach_vice").get()? == 11 || ctx.var("rach_vice").get()? == 12) {
        ctx.lines_as(
            "Lachellen",
            args![
                "Oh, hello again~",
                "Is there any way I can",
                "help you this time? I know",
                "you're looking for Bruspetti,",
                "so I'll help you if you have",
                "any questions for me~"
            ],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s.clone() == "Lighthalzen" {
            ctx.lines_as(
                "Lachellen",
                args![
                    "Lighthalzen? Oh, that's",
                    "right! Bruspetti did ask",
                    "me about Lighthalzen a while",
                    "ago. Yes, she seemed really",
                    "interested in learning more",
                    "about that place recently."
                ],
            )?;
            ctx.next()?;
        } else if l_input_s.clone() == "Freya's Spring" {
            ctx.lines_as(
                "Lachellen",
                args![
                    "Freya's Spring?",
                    "Oh, I like that place!",
                    "Speaking of which, I know",
                    "someone named Katinshuell",
                    "that went there pretty often.",
                    "Usually men don't go alone..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lachellen",
                args![
                    "Wait, that's the place",
                    "I told you about, where",
                    "Bruspetti and her boyfriend",
                    "usually met. Do you think",
                    "she and Katinshuell...?",
                    "Ooh, maybe I figured it out!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lachellen",
                args![
                    "But yeah, that place is",
                    "usually filled with couples,",
                    "so you look like a real loser",
                    "if you go there alone. That's",
                    "why I want a boyfriend now..."
                ],
            )?;
            ctx.var("rach_vice").set(Val::from(12))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(8115), Val::from(8116)])?;
            ctx.next()?;
        } else {
            ctx.lines_as(
                "Lachellen",
                args![
                    "Oh, I'm sorry...",
                    "I don't know anything",
                    ((Val::from("about ") + l_input_s.clone()) + Val::from("."))
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Lachellen",
            args![
                "Well, I don't know if",
                "you learned anything",
                "important from me,",
                "but I hope I helped",
                "you, even if it was",
                "just a little bit."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rach_vice").get()? == 7 {
        ctx.lines_as(
            "Lachellen",
            args![
                "Let's see... Bruspetti",
                "and her guy would usually",
                "meet at... Um, I know where",
                "it is by feel, but I can't really",
                "give you good directions",
                "to get there. Er, sorry!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lachellen",
            args![
                "Luckily, this town",
                "isn't that big, so I'm",
                "sure you'll find something",
                "if you just keep looking.",
                "Oh, and if you find her,",
                "tell her I said ''hi,'' okay?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rach_vice").get()? == 6 {
        ctx.lines_as(
            "Lachellen",
            args![
                "Oh, Bruspetti's dad",
                "was talking about how",
                "good she smells? Yeah,",
                "she's kinda famous for",
                "that around here, so he's",
                "not creepy or anything."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lachellen",
            args![
                "I see, so you think you",
                "caught wind of her in some",
                "other town. Yeah, I heard",
                "that she went traveling to see",
                "if she could learn something,",
                "and she hasn't returned yet."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lachellen",
            args![
                "Oh, you know what?",
                "If she's in Lighthalzen,",
                "she's probably gone there to",
                "learn more about her boyfriend.",
                "Bruspetti mentioned something",
                "about that when I last saw her."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lachellen",
            args![
                "I don't know anything",
                "else. Hmm, maybe if you",
                "check the place where she",
                "and her boyfriend usually",
                "went on dates, you might",
                "be able to find something."
            ],
        )?;
        ctx.var("rach_vice").set(Val::from(7))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8110), Val::from(8111)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("rach_vice").get()? == 2 || ctx.var("rach_vice").get()? == 3) {
        ctx.lines_as(
            "Lachellen",
            args![
                "Ooh, I'm so jealous",
                "of Bruspetti! She's been",
                "spending so much time with",
                "her new boyfriend recently...",
                "But she still refuses to tell",
                "me what his name is."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lachellen",
            args![
                "She told me that she's",
                "serious about him, but",
                "she's also admitted that",
                "she doesn't know much about",
                "him. Let's see... He grew up in Lighthalzen? That's all she knows."
            ],
        )?;
        ctx.var("rach_vice").set(Val::from(3))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8106), Val::from(8107)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Lachellen",
        args![
            "My friend Bruspetti",
            "is a really nice girl~",
            "Everyone loves her, and",
            "she's sooo beautiful. All",
            "the guys are jealous of",
            "her new boyfriend!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lachellen",
        args![
            "Still, she's really",
            "shy, and won't tell me",
            "who he is. There's a lot",
            "I don't know about him, but",
            "I'm sure they're happy together."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn lachellen(ctx: &Ctx) -> Script {
    lachellen_body(ctx, Vec::new()).map(|_| ())
}

fn kid_1rachel_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Kid",
        args![
            "Hey, have you seen",
            "Bruspetti? She's really",
            "nice and always buys me",
            "lots and lots of cookies!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kid",
        args![
            "Hmm, I haven't seen",
            "her in a while, though.",
            "I heard she went traveling",
            "somewhere. How long do",
            "you think she'll be gone, huh?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn kid_1rachel(ctx: &Ctx) -> Script {
    kid_1rachel_body(ctx, Vec::new()).map(|_| ())
}

fn kid_2rachel_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Kid",
        args![
            "Oh wow! I wanna be an",
            "adventurer like you when",
            "I grow up! Go to all sorts",
            "of places, visit different",
            "towns. It sounds so fun!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kid",
        args![
            "Maybe if I find a really",
            "nice place, maybe I'll just",
            "stay and then move there.",
            "There's somewhere here",
            "that did that, and moved",
            "from Lighthalzen to here~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kid",
        args![
            "Yeah, as soon as I get",
            "a chance, I'm gonna ditch",
            "this town and see as much",
            "of the world as I can! I need",
            "to grow up faster than this!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn kid_2rachel(ctx: &Ctx) -> Script {
    kid_2rachel_body(ctx, Vec::new()).map(|_| ())
}

fn grandma_rachel_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rach_vice").get()?.number()? > 21 {
        ctx.lines_as(
            "Grandma",
            args![
                "When you get to be my",
                "age, you'll cherish all",
                "of your memories, even if",
                "the experience was hurtful",
                "when it actually happened."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Grandma",
            args![
                "Your memories are part",
                "of who you are and what",
                "makes you unique. I can",
                "appreciate living the life",
                "that I have, even if it's",
                "not particularly special."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rach_vice").get()? == 13 {
        ctx.lines_as(
            "Grandma",
            args![
                "When you get to be my",
                "age, you appreciate life",
                "more and can release your",
                "regrets more easily. I wasted",
                "much of my youth in needless",
                "worry when I could've relaxed."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rach_vice").get()? == 12 {
        ctx.lines_as(
            "Grandma",
            args![
                "Oh, back again, eh?",
                "I guess you must really",
                "like coming to this place",
                "too. Hm, that reminds me..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Grandma",
            args![
                "See the edge of the spring?",
                "The same young man frequently",
                "comes to that spot, and stares",
                "into the water, just dripping with sadness. Someone so young",
                "shouldn't be feeling like that."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Grandma",
            args![
                "You're supposed to come",
                "here to relax and enjoy",
                "the surrounding beauty,",
                "not wallow in your sorrow.",
                "I guess that boy doesn't",
                "agree with me on that."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Grandma",
            args![
                "You should be able",
                "to let go of whatever's",
                "bothering you, and just",
                "enjoy life as it is now."
            ],
        )?;
        ctx.var("rach_vice").set(Val::from(13))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8116), Val::from(8117)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Grandma",
        args![
            "I love this place,",
            "its beautiful scenary",
            "and serene atmosphere.",
            "It's so heavenly peaceful."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Grandma",
        args![
            "It brings my heart joy to see",
            "young couples can coming",
            "here and relaxing together.",
            "Isn't love a grand thing?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn grandma_rachel(ctx: &Ctx) -> Script {
    grandma_rachel_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ratrace1Step {
    Start,
    OnTouch,
}

fn ratrace1_run(ctx: &Ctx, mut step: Ratrace1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ratrace1Step::Start => {
                step = Ratrace1Step::OnTouch;
                continue 'machine;
            }
            Ratrace1Step::OnTouch => {
                if ctx.var("rach_vice").get()? == 5 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "This...",
                            "This smell...",
                            "It smells so nice!",
                            "Like rose petals riding",
                            "on a gentle breeze..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Wait, could this be", "the scent that Bruspetti's", "father mentioned earlier?"],
                    )?;
                    ctx.var("rach_vice").set(Val::from(6))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(8109), Val::from(8110)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn ratrace1(ctx: &Ctx) -> Script {
    ratrace1_run(ctx, Ratrace1Step::Start, Vec::new()).map(|_| ())
}

pub fn ratrace1_ontouch(ctx: &Ctx) -> Script {
    ratrace1_run(ctx, Ratrace1Step::OnTouch, Vec::new()).map(|_| ())
}
