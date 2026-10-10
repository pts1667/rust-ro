use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn pisruik_qsk_al_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) {
        if ctx.var("JobLevel").get()?.number()? < 40 {
            ctx.lines_as(
                "Pisruik",
                args![
                    "^333333*Cough cough*^000000",
                    "Damn, if only I had",
                    "a little more money",
                    "to buy some medicine.",
                    "I should have stayed",
                    "home today, but..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.var("alche_sk").get()? == 0 {
            ctx.lines_as(
                "Pisruik",
                args![
                    "^333333*Cough cough*^000000",
                    "Ugh, there's nothing",
                    "worse than working when",
                    "you're supposed to be resting.",
                    "H-hey! Um, what are you doing?"
                ],
            )?;
            ctx.var("alche_sk").set(Val::from(1))?;
            ctx.next()?;
            ctx.lines_as(
                "Pisruik",
                args![
                    "Q-quit looking at",
                    "my test results right",
                    "this inst--oh. Wait.",
                    "You're not one of the",
                    "researchers here. Huh."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Pisruik",
                args![
                    "Uh... Don't you have",
                    "anything better to do",
                    "than to breathe down my",
                    "back? I'm trying to finish",
                    "something here! Oh, never",
                    "mind, I'm just cranky..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("alche_sk").get()? == 1 {
                ctx.lines_as(
                    "Pisruik",
                    args![
                        "You again? You don't seem",
                        "to be doing any research here.",
                        "Is there something you need?",
                        "Though, I'm afraid I can't be",
                        "of very much help to you."
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("What are you working on?:I don't need anything, thanks.")],
                )?) == 1
                {
                    ctx.lines_as(
                        "Pisruik",
                        args![
                            "Well, I'm not sure if I can",
                            "give you all of the details.",
                            "You see, everyone here is",
                            "a researcher that can't afford",
                            "to rent a lab for himself. So we all ended up sharing this one."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pisruik",
                        args![
                            "Even though we all pitched",
                            "in to rent this lab, we're all",
                            "getting pretty desperate. In",
                            "fact, a few of us have even",
                            "stolen work from each other.",
                            "That's pretty pathetic, huh?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pisruik",
                        args![
                            "I guess that's what happens",
                            "when you're poor and don't",
                            "have a day job. Things are",
                            "so bad right now, I can't even",
                            "afford to get new materials!",
                            "What can I possibly do?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pisruik",
                        args![
                            "Ah, I've got it! You're",
                            "an adventurer, right?",
                            "If you're still curious about",
                            "my research, I'll tell you more",
                            "about it if you help me out by",
                            "gathering some supplies for me."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pisruik",
                        args![
                            "I guess it couldn't do",
                            "much harm if you knew what",
                            "I was working on, anyway.",
                            "I mean, we'd have to be working",
                            "on the same project for you to",
                            "benefit. So, what's your name?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![((Val::from("I am called ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))],
                    )?;
                    ctx.next()?;
                    ctx.var("alche_sk").set(Val::from(2))?;
                    ctx.lines_as(
                        "Pisruik",
                        args![
                            ((Val::from("Ah, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                            "Would you please bring",
                            "^6600005 Yellow Gemstones^000000,",
                            "^6600004 Empty Potion Bottles^000000,",
                            "^66000010 Hearts of Mermaids^000000,",
                            "and ^66000010 Moth Dust^000000?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Pisruik",
                    args![
                        "If you came here to buy",
                        "Potion Manuals or something",
                        "like that, you've come to the",
                        "wrong guy. Everything you see",
                        "here is for the completion",
                        "of a personal project."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("alche_sk").get()? == 2 {
                    ctx.lines_as(
                        "Pisruik",
                        args![
                            "Great you're back!",
                            "Let's see, you were",
                            "supposed to bring me",
                            "4 Empty Potion Bottles...",
                            "And... And... What else",
                            "did I ask you to get?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("5 Yellow Gemstones:5 Blue Gemstones:5 Red Gemstones")])? {
                        1 => {
                            ctx.lines_as(
                                "Pisruik",
                                args![
                                    "Right, right!",
                                    "5 Yellow Gemstones.",
                                    "That's what I needed.",
                                    "I'm sure there was more,",
                                    "but what I can't recall exactly..."
                                ],
                            )?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("10 Hearts of Mermaid:10 Large Jellopies")],
                            )?) == 1
                            {
                                ctx.lines_as(
                                    "Pisruik",
                                    args![
                                        "Of course!",
                                        "10 Hearts of Mermaid!",
                                        "How could I forget that?",
                                        "And then, the last thing",
                                        "I asked you for was, um..."
                                    ],
                                )?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(ctx, &[Val::from("10 Frill:10 Moth Dust")])?) == 1 {
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "No, that can't have",
                                            "been it. I already have",
                                            "plenty of Frills. Hmmm...",
                                            "What am I missing now?"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Pisruik",
                                    args![
                                        "Right. I was just",
                                        "about to say that.",
                                        "So did you remember",
                                        "to bring me everything?"
                                    ],
                                )?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(ctx, &[Val::from("No.:Yes!")])?) == 1 {
                                    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])? == 1 {
                                        ctx.lines_as(
                                            "Pisruik",
                                            args![
                                                "You didn't...?",
                                                "Oh, just admit it.",
                                                "You don't want to",
                                                "do this for me, right?",
                                                "Don't go wasting your",
                                                "time just for my sake."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "Not yet, huh?",
                                            "Though I hope you can",
                                            "bring me that stuff as",
                                            "soon as possible, you",
                                            "don't have to do it, you",
                                            "know. Yeah, no big deal."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Pisruik",
                                    args![
                                        "Really now?",
                                        "Well, let me check",
                                        "what you brought to",
                                        "make sure you didn't",
                                        "forget anything. Hm..."
                                    ],
                                )?;
                                ctx.next()?;
                                if (((ctx.call(Function::CountItem, vec![Val::from(715)])?.number()? > 4
                                    && ctx.call(Function::CountItem, vec![Val::from(1093)])?.number()? > 3)
                                    && ctx.call(Function::CountItem, vec![Val::from(950)])?.number()? > 9)
                                    && ctx.call(Function::CountItem, vec![Val::from(1057)])?.number()? > 9)
                                {
                                    ctx.call(Function::DelItem, vec![Val::from(715), Val::from(5)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(1093), Val::from(4)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(950), Val::from(10)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(1057), Val::from(10)])?;
                                    ctx.var("alche_sk").set(Val::from(3))?;
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "Great, it looks like",
                                            "everything is here.",
                                            "Alright, let me take",
                                            "those. Now, guess what",
                                            "I'll be making with the",
                                            "materials you've brought."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Medicine?:Bomb?")])? {
                                        1 => {
                                            ctx.var("alche_sk").set(Val::from(4))?;
                                            ctx.lines_as(
                                                "Pisruik",
                                                args!["Hahahah, that's right!", "I'm working on making", "a new form of medicine."],
                                            )?;
                                        }
                                        2 => {
                                            ctx.var("alche_sk").set(Val::from(4))?;
                                            ctx.lines_as(
                                                "Pisruik",
                                                args![
                                                    "A bomb? Do I look like",
                                                    "a nutcase to you? No, no...",
                                                    "I'm developing a new form of",
                                                    "medicine. Sure, bombs make",
                                                    "good money, but where would",
                                                    "I test them? Here? No way!"
                                                ],
                                            )?;
                                        }
                                        _ => {}
                                    }
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "Anyway, this medicine",
                                            "reacts with the human body's",
                                            "digestive enzymes to initiate",
                                            "temporary metabolic changes",
                                            "that artificially stop heat",
                                            "absorption into the body."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "The actual effect of this",
                                            "medicine is that it greatly",
                                            "increases the body's resistance",
                                            "to most forms of heat! However,",
                                            "it will also reduce resistance",
                                            "to cold as a side effect."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "I know my medicine sounds",
                                            "a little weird, but think of",
                                            "the applications! If used in",
                                            "the right situations, this",
                                            "medicine may be quite handy."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "Ah, seeing as you're still",
                                            "here, would you mind helping",
                                            "me again? I need about, hmm,",
                                            "20 Maneater Blossoms. If you",
                                            "could bring them to me, it'd",
                                            "really help me out a lot."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(
                                        ctx,
                                        &[Val::from("Nope, I'm too busy!:Sure, I'll help you.:What's in it for me?")],
                                    )? {
                                        1 => {
                                            ctx.var("alche_sk").set(Val::from(5))?;
                                            ctx.lines_as(
                                                "Pisruik",
                                                args![
                                                    "I guess I'll have to gather",
                                                    "those on my own. Alright, well,",
                                                    "seeing as we've both gotten",
                                                    "what we wanted, I'll suppose",
                                                    "your business with me is done."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.var("alche_sk").set(Val::from(6))?;
                                            ctx.lines_as(
                                                "Pisruik",
                                                args![
                                                    "Thanks, I really",
                                                    "appreciate it. While",
                                                    "I'm waiting for you,",
                                                    "I can start boiling",
                                                    "the Clover extract."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            let choice = runtime::select_values(ctx, &[Val::from("Clover extract? What's that for?")])?;
                                            ctx.var("@menu").set(choice)?;
                                            ctx.lines_as(
                                                "Pisruik",
                                                args![
                                                    "Well, I need the Clover",
                                                    "extract for a compound",
                                                    "that I'm going to make with",
                                                    "the Maneater Blossoms. I'm",
                                                    "kind of weak, so I try not to",
                                                    "travel too far when I can."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Pisruik",
                                                args![
                                                    "Yeah, ever since I was",
                                                    "little, I've had a weak",
                                                    "heart and bad eyesight.",
                                                    "The doctor tells me to avoid",
                                                    "stress and hard work, but",
                                                    "researching is my life."
                                                ],
                                            )?;
                                            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Pisruik",
                                                    args![
                                                        "I wonder...",
                                                        "If you understand",
                                                        "the way I feel, how",
                                                        "much I've had to sacrifice",
                                                        "for my dream. Heh, anyway..."
                                                    ],
                                                )?;
                                            }
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Pisruik",
                                                args![
                                                    "I hope you understand that",
                                                    "it's too dangerous for me to",
                                                    "gather Maneater Blossoms on",
                                                    "my own, so if I'm going to get",
                                                    "as much help as I can. Thanks",
                                                    "again for being cooperative."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        3 => {
                                            ctx.lines_as(
                                                "Pisruik",
                                                args![
                                                    "Ha ha ha ha!",
                                                    "That's real business",
                                                    "like of you! Alright,",
                                                    "I may be poor, but if",
                                                    "you help me, I'll give you",
                                                    "the results of my research."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            if Val::from(runtime::select_values(
                                                ctx,
                                                &[Val::from("Nah, I'm too busy.:Alright, I'll help you.")],
                                            )?) == 1
                                            {
                                                ctx.var("alche_sk").set(Val::from(5))?;
                                                ctx.lines_as(
                                                    "Pisruik",
                                                    args![
                                                        "I guess I'll have to gather",
                                                        "those on my own. Alright, well,",
                                                        "seeing as we've both gotten",
                                                        "what we wanted, I'll suppose",
                                                        "your business with me is done."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            ctx.var("alche_sk").set(Val::from(6))?;
                                            ctx.lines_as(
                                                "Pisruik",
                                                args![
                                                    "Thanks, I really",
                                                    "appreciate it. While",
                                                    "I'm waiting for you,",
                                                    "I can start boiling",
                                                    "the Clover extract."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            let choice = runtime::select_values(ctx, &[Val::from("Clover extract? What's that for?")])?;
                                            ctx.var("@menu").set(choice)?;
                                            ctx.lines_as(
                                                "Pisruik",
                                                args![
                                                    "Well, I need the Clover",
                                                    "extract for a compound",
                                                    "that I'm going to make with",
                                                    "the Maneater Blossoms. I'm",
                                                    "kind of weak, so I try not to",
                                                    "travel too far when I can."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Pisruik",
                                                args![
                                                    "Yeah, ever since I was",
                                                    "little, I've had a weak",
                                                    "heart and bad eyesight.",
                                                    "The doctor tells me to avoid",
                                                    "stress and hard work, but",
                                                    "researching is my life."
                                                ],
                                            )?;
                                            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Pisruik",
                                                    args![
                                                        "I wonder...",
                                                        "If you understand",
                                                        "the way I feel, how",
                                                        "much I've had to sacrifice",
                                                        "for my dream. Heh, anyway..."
                                                    ],
                                                )?;
                                            }
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Pisruik",
                                                args![
                                                    "I hope you understand that",
                                                    "it's too dangerous for me to",
                                                    "gather Maneater Blossoms on",
                                                    "my own, so if I'm going to get",
                                                    "as much help as I can. Thanks",
                                                    "again for being cooperative."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                } else if (((ctx.call(Function::CountItem, vec![Val::from(715)])? == 0
                                    && ctx.call(Function::CountItem, vec![Val::from(1093)])? == 0)
                                    && ctx.call(Function::CountItem, vec![Val::from(950)])? == 0)
                                    && ctx.call(Function::CountItem, vec![Val::from(1057)])? == 0)
                                {
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "So you knew what",
                                            "you had to bring, came",
                                            "to remind me what I had",
                                            "forgotten, but didn't bring",
                                            "anything? Weird. Ah well.",
                                            "Come with the stuff next time."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "Oh, this isn't good, some",
                                            "of the items I asked for are",
                                            "missing. I'm sorry, but Alchemy",
                                            "gets dangerously unpredictable",
                                            "when things aren't used in just",
                                            "the right amounts. Hmmm..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "Well, I can afford to",
                                            "push my deadlines back",
                                            "if you promise to return",
                                            "with the materials I need",
                                            "as soon as you possibly can."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            ctx.lines_as(
                                "Pisruik",
                                args![
                                    "Large Jellopy?",
                                    "Yes, Large--no.",
                                    "Wait, that doesn't",
                                    "sound right at all.",
                                    "No, it was something",
                                    "else I need you to get."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Pisruik",
                                args![
                                    "Blue Gemstones...?",
                                    "No, that was for the",
                                    "potion that increases",
                                    "tolerance to the Water",
                                    "property, I think. What's",
                                    "wrong with my memory?!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.lines_as(
                                "Pisruik",
                                args![
                                    "Red Gemstones...?",
                                    "No, that was for the",
                                    "potion that increases",
                                    "tolerance to the Earth",
                                    "property, I think. What's",
                                    "wrong with my memory?!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    if ctx.var("alche_sk").get()? == 3 {
                        ctx.lines_as(
                            "Pisruik",
                            args![
                                "Why did you just leave?",
                                "You didn't even let me",
                                "finish talking! Oh well,",
                                "maybe it's not your fault.",
                                "Anyway, just so you know,",
                                "I'm developing a new medicine."
                            ],
                        )?;
                        ctx.var("alche_sk").set(Val::from(4))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("alche_sk").get()? == 4 {
                            ctx.lines_as(
                                "Pisruik",
                                args![
                                    "Alright, did you",
                                    "want to learn more",
                                    "about the medicine that",
                                    "I'm developing? I mean,",
                                    "that's why you came, right?"
                                ],
                            )?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(ctx, &[Val::from("No, thanks.:Yes, please.")])?) == 1 {
                                ctx.lines_as(
                                    "Pisruik",
                                    args![
                                        "Alright then.",
                                        "Really? Well, I'm",
                                        "willing to spend the",
                                        "time to explain it to",
                                        "you. After all, you did",
                                        "help me out just then."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Pisruik",
                                args![
                                    "I'm working on a new",
                                    "form of medicine that,",
                                    "hopefully, will be used",
                                    "for the betterment and",
                                    "protection of mankind!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Pisruik",
                                args![
                                    "Anyway, this medicine",
                                    "reacts with the human body's",
                                    "digestive enzymes to initiate",
                                    "temporary metabolic changes",
                                    "that artificially stop heat",
                                    "absorption into the body."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Pisruik",
                                args![
                                    "The actual effect of this",
                                    "medicine is that it greatly",
                                    "increases the body's resistance",
                                    "to most forms of heat! However,",
                                    "it will also reduce resistance",
                                    "to cold as a side effect."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Pisruik",
                                args![
                                    "I know my medicine sounds",
                                    "a little weird, but think of",
                                    "the applications! If used in",
                                    "the right situations, this",
                                    "medicine may be quite handy."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Pisruik",
                                args![
                                    "Ah, seeing as you're still",
                                    "here, would you mind helping",
                                    "me again? I need about, hmm,",
                                    "20 Maneater Blossoms. If you",
                                    "could bring them to me, it'd",
                                    "really help me out a lot."
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(
                                ctx,
                                &[Val::from("Nope, I'm too busy!:Sure, I'll help you.:What's in it for me?")],
                            )? {
                                1 => {
                                    ctx.var("alche_sk").set(Val::from(5))?;
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "I guess I'll have to gather",
                                            "those on my own. Alright, well,",
                                            "seeing as we've both gotten",
                                            "what we wanted, I'll suppose",
                                            "your business with me is done."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.var("alche_sk").set(Val::from(6))?;
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "Thanks, I really",
                                            "appreciate it. While",
                                            "I'm waiting for you,",
                                            "I can start boiling",
                                            "the Clover extract."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    let choice = runtime::select_values(ctx, &[Val::from("Clover extract? What's that for?")])?;
                                    ctx.var("@menu").set(choice)?;
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "Well, I need the Clover",
                                            "extract for a compound",
                                            "that I'm going to make with",
                                            "the Maneater Blossoms. I'm",
                                            "kind of weak, so I try not to",
                                            "travel too far when I can."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "Yeah, ever since I was",
                                            "little, I've had a weak",
                                            "heart and bad eyesight.",
                                            "The doctor tells me to avoid",
                                            "stress and hard work, but",
                                            "researching is my life."
                                        ],
                                    )?;
                                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Pisruik",
                                            args![
                                                "I wonder...",
                                                "If you understand",
                                                "the way I feel, how",
                                                "much I've had to sacrifice",
                                                "for my dream. Heh, anyway..."
                                            ],
                                        )?;
                                    }
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "I hope you understand that",
                                            "it's too dangerous for me to",
                                            "gather Maneater Blossoms on",
                                            "my own, so if I'm going to get",
                                            "as much help as I can. Thanks",
                                            "again for being cooperative."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                3 => {
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "Ha ha ha ha!",
                                            "That's real business",
                                            "like of you! Alright,",
                                            "I may be poor, but if",
                                            "you help me, I'll give you",
                                            "the results of my research."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    if Val::from(runtime::select_values(
                                        ctx,
                                        &[Val::from("Nah, I'm too busy.:Alright, I'll help you.")],
                                    )?) == 1
                                    {
                                        ctx.var("alche_sk").set(Val::from(5))?;
                                        ctx.lines_as(
                                            "Pisruik",
                                            args![
                                                "I guess I'll have to gather",
                                                "those on my own. Alright, well,",
                                                "seeing as we've both gotten",
                                                "what we wanted, I'll suppose",
                                                "your business with me is done."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.var("alche_sk").set(Val::from(6))?;
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "Thanks, I really",
                                            "appreciate it. While",
                                            "I'm waiting for you,",
                                            "I can start boiling",
                                            "the Clover extract."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    let choice = runtime::select_values(ctx, &[Val::from("Clover extract? What's that for?")])?;
                                    ctx.var("@menu").set(choice)?;
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "Well, I need the Clover",
                                            "extract for a compound",
                                            "that I'm going to make with",
                                            "the Maneater Blossoms. I'm",
                                            "kind of weak, so I try not to",
                                            "travel too far when I can."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "Yeah, ever since I was",
                                            "little, I've had a weak",
                                            "heart and bad eyesight.",
                                            "The doctor tells me to avoid",
                                            "stress and hard work, but",
                                            "researching is my life."
                                        ],
                                    )?;
                                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Pisruik",
                                            args![
                                                "I wonder...",
                                                "If you understand",
                                                "the way I feel, how",
                                                "much I've had to sacrifice",
                                                "for my dream. Heh, anyway..."
                                            ],
                                        )?;
                                    }
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Pisruik",
                                        args![
                                            "I hope you understand that",
                                            "it's too dangerous for me to",
                                            "gather Maneater Blossoms on",
                                            "my own, so if I'm going to get",
                                            "as much help as I can. Thanks",
                                            "again for being cooperative."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        } else {
                            if ctx.var("alche_sk").get()? == 5 {
                                ctx.lines_as(
                                    "Pisruik",
                                    args![
                                        "I'm busy right now.",
                                        "You didn't forget",
                                        "anything did you?",
                                        "If not, you better",
                                        "get going and let",
                                        "me do my work."
                                    ],
                                )?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from("Alright, sorry to bother you.:Can I still help you?")],
                                )?) == 1
                                {
                                    ctx.lines_as(
                                        "Pisruik",
                                        args!["Yeah, whatever.", "Just hurry up and leave", "so that I can concentrate."],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Pisruik",
                                    args![
                                        "Huh? What made you",
                                        "change your mind? Well,",
                                        "I can't afford not to accept",
                                        "any help, so I guess that's",
                                        "a \"Yes.\" Yeah, you can help."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Pisruik",
                                    args![
                                        "Alright, go and get me",
                                        "20 Maneater Blossoms.",
                                        "If I weren't so sickly, I'd get",
                                        "them myself, but--*Cough* as",
                                        "you can see, I don't feel so well."
                                    ],
                                )?;
                                ctx.var("alche_sk").set(Val::from(6))?;
                                ctx.next()?;
                                ctx.mes("[Pisruik]")?;
                                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
                                    ctx.lines(args![
                                        "I... I really",
                                        "appreciate your",
                                        "willingness to help",
                                        "me in my research..."
                                    ])?;
                                } else {
                                    ctx.lines(args![
                                        "I hope you get those",
                                        "items to me as soon as",
                                        "you can. And don't flake",
                                        "out on me this time!"
                                    ])?;
                                }
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("alche_sk").get()? == 6 {
                                    if ctx.call(Function::CountItem, vec![Val::from(1032)])?.number()? > 19 {
                                        ctx.call(Function::DelItem, vec![Val::from(1032), Val::from(20)])?;
                                        ctx.var("alche_sk").set(Val::from(7))?;
                                        ctx.lines_as(
                                            "Pisruik",
                                            args!["Thanks so much for", "bringing me these", "Maneater Blossoms."],
                                        )?;
                                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
                                            ctx.lines(args!["You don't know how", "much this means to me~"])?;
                                        } else {
                                            ctx.lines(args![
                                                "Now all I have to do",
                                                "is mix these with the",
                                                "Clover extract I prepared."
                                            ])?;
                                        }
                                        ctx.next()?;
                                        ctx.var("alche_sk").set(Val::from(9))?;
                                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUI_EXPLOSION")?])?;
                                        ctx.lines_as("Pisruik", args!["Ah!", "M-my face!"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Pisruik",
                                            args!["Hey...", "Are you alright?", "That was a pretty", "big explosion..."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["Your glasses...", "They're broken..."],
                                        )?;
                                        ctx.next()?;
                                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
                                            ctx.lines(args![
                                                "^3355FFThe explosion destroyed",
                                                "Pisruik's glasses, revealing",
                                                "the beautiful face of a",
                                                "gorgeous, gorgeous man.^000000"
                                            ])?;
                                        } else {
                                            ctx.lines(args![
                                                "^3355FFThe explosion blew off",
                                                "Pisruik's glasses. Without",
                                                "them, he looks more like",
                                                "a male model than a dorky",
                                                "scientific researcher.^000000"
                                            ])?;
                                        }
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["Holy crap!", "You're one", "good looking guy!"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Pisruik",
                                            args![
                                                "I c-can't see too",
                                                "well without my glasses.",
                                                "Well, at least I can tell",
                                                "that you're not bleeding.",
                                                "But are you alright?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["Oh, I'm fine.", "But what are you", "going to do about", "your glasses?"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Pisruik",
                                            args![
                                                "Shoot, you're right.",
                                                "I don't happen to have",
                                                "an extra pair. Hey, can",
                                                "you get me a pair of glasses,",
                                                "the same kind I used to wear?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Pisruik",
                                            args![
                                                "I know it's too much",
                                                "to ask you for, but I'm",
                                                "almost blind without them.",
                                                "I can't do very much if I can't",
                                                "even see. I'm really sorry",
                                                ((Val::from("about this, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                    + Val::from("."))
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as(
                                            "Pisruik",
                                            args![
                                                "Would you come back with",
                                                "20 Maneater Blossoms",
                                                "so that I can finish this",
                                                "medicine I'm working on?",
                                                "Thanks, thanks, I've got",
                                                "to hustle with this project..."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                } else {
                                    if ctx.var("alche_sk").get()? == 7 {
                                        ctx.var("alche_sk").set(Val::from(8))?;
                                        ctx.lines_as(
                                            "Pisruik",
                                            args![
                                                "Hmmm...",
                                                "Actually, I miscalculated",
                                                "the number of Maneater",
                                                "Blossoms that I need. Would",
                                                "you bring me one more? Sorry, I know it's kind of troublesome..."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.var("alche_sk").get()? == 8 {
                                            if ctx.call(Function::CountItem, vec![Val::from(1032)])?.number()? > 0 {
                                                ctx.call(Function::DelItem, vec![Val::from(1032), Val::from(1)])?;
                                                ctx.var("alche_sk").set(Val::from(7))?;
                                                ctx.lines_as(
                                                    "Pisruik",
                                                    args![
                                                        "Thanks so much!",
                                                        "Now I finally have the",
                                                        "exact amount of Maneater",
                                                        "Blossoms that I'll need."
                                                    ],
                                                )?;
                                                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
                                                    ctx.lines(args!["I'm really sorry for putting", "your through all this trouble."])?;
                                                } else {
                                                    ctx.lines(args!["Finally, I begin the most", "exciting part of this project!"])?;
                                                }
                                                ctx.next()?;
                                                ctx.var("alche_sk").set(Val::from(9))?;
                                                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUI_EXPLOSION")?])?;
                                                ctx.lines_as("Pisruik", args!["Ah!", "M-my face!"])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Pisruik",
                                                    args!["Hey...", "Are you alright?", "That was a pretty", "big explosion..."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args!["Your glasses...", "They're broken..."],
                                                )?;
                                                ctx.next()?;
                                                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
                                                    ctx.lines(args![
                                                        "^3355FFThe explosion destroyed",
                                                        "Pisruik's glasses, revealing",
                                                        "the beautiful face of a",
                                                        "gorgeous, gorgeous man.^000000"
                                                    ])?;
                                                } else {
                                                    ctx.lines(args![
                                                        "^3355FFThe explosion blew off",
                                                        "Pisruik's glasses. Without",
                                                        "them, he looks more like",
                                                        "a male model than a dorky",
                                                        "scientific researcher.^000000"
                                                    ])?;
                                                }
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args!["Holy crap!", "You're one", "good looking guy!"],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Pisruik",
                                                    args![
                                                        "I c-can't see too",
                                                        "well without my glasses.",
                                                        "Well, at least I can tell",
                                                        "that you're not bleeding.",
                                                        "But are you alright?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args!["Oh, I'm fine.", "But what are you", "going to do about", "your glasses?"],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Pisruik",
                                                    args![
                                                        "Shoot, you're right.",
                                                        "I don't happen to have",
                                                        "an extra pair. Hey, can",
                                                        "you get me a pair of glasses,",
                                                        "the same kind I used to wear?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Pisruik",
                                                    args![
                                                        "I know it's too much",
                                                        "to ask you for, but I'm",
                                                        "almost blind without them.",
                                                        "I can't do very much if I can't",
                                                        "even see. I'm really sorry",
                                                        ((Val::from("about this, ")
                                                            + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                            + Val::from("."))
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                ctx.lines_as(
                                                    "Pisruik",
                                                    args![
                                                        "Hmmm...",
                                                        "Actually, I miscalculated",
                                                        "the number of Maneater",
                                                        "Blossoms that I need. Would",
                                                        "you bring me one more? Sorry, I know it's kind of troublesome..."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        } else {
                                            if ctx.var("alche_sk").get()? == 9 {
                                                ctx.lines(args![
                                                    "^3355FFPisruik is holding his",
                                                    "broken glasses, squinting",
                                                    "his eyes. It seems he like",
                                                    "he really does need them,",
                                                    "even if he looks much less",
                                                    "dorky without them.^000000"
                                                ])?;
                                                ctx.next()?;
                                                if Val::from(runtime::select_values(
                                                    ctx,
                                                    &[Val::from("Let him try a pair of your glasses:Don't give him anything")],
                                                )?) == 1
                                                {
                                                    if ctx.call(Function::CountItem, vec![Val::from(2243)])?.number()? > 0 {
                                                        ctx.call(Function::DelItem, vec![Val::from(2243), Val::from(1)])?;
                                                        ctx.var("alche_sk").set(Val::from(10))?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args![
                                                                "Here, why don't you",
                                                                "check I'm carrying and",
                                                                "see if there's a pair of",
                                                                "glasses that you can use?"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Pisruik",
                                                            args![
                                                                "Huh? Oh, is that you?",
                                                                "Ah, this pair of glasses",
                                                                "works! Thanks a lot, now",
                                                                "I can see again! Now, let",
                                                                "me check the results of the",
                                                                "experiment we conducted."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Pisruik",
                                                            args![
                                                                "Okay, the test tube wasn't",
                                                                "damaged. Yes, according to",
                                                                "these readings, this medicine",
                                                                "should be fully functional!",
                                                                "I think it was a success!",
                                                                "Well, theoretically anyway."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Pisruik",
                                                            args![
                                                                "Hmm, changing the attributes",
                                                                "of the human body for certain",
                                                                "effects may cause controversy",
                                                                "later, but hopefully this thing",
                                                                "I've invented will be used for",
                                                                "good. Ah, that's right!"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Pisruik",
                                                            args![
                                                                "Would you like me to",
                                                                "teach you everything I've",
                                                                "learned in my research? You",
                                                                "should be able to create a new",
                                                                "type of potion by making use of",
                                                                "the knowledge I can teach you."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        if Val::from(runtime::select_values(ctx, &[Val::from("Sure!:No, thanks.")])?) == 1 {
                                                            ctx.lines_as(
                                                                "Pisruik",
                                                                args![
                                                                    ((Val::from("Great, ")
                                                                        + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                        + Val::from("!")),
                                                                    "I know I can trust you",
                                                                    "to use this research for",
                                                                    "good and noble ends. Now,",
                                                                    "please read this thesis and",
                                                                    "all of my additional notes..."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines(args![
                                                                "^3355FFPisruik thoroughly",
                                                                "explains the properties",
                                                                "of his medicine, the reaction",
                                                                "of the human organs to it, as",
                                                                "well as a few warnings about",
                                                                "the medicine's side effects.^000000"
                                                            ])?;
                                                            ctx.next()?;
                                                            ctx.var("alche_sk").set(Val::from(11))?;
                                                            ctx.call(Function::GetItem, vec![Val::from(7434), Val::from(1)])?;
                                                            ctx.lines_as(
                                                                "Pisruik",
                                                                args![
                                                                    "Well, you should be",
                                                                    "ready to make your own",
                                                                    "potions that are a variation",
                                                                    "of my medicine. But you'll",
                                                                    "probably need to keep that",
                                                                    "thesis as a ready reference."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.mes("[Pisruik]")?;
                                                            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                                                ctx.lines(args![
                                                                    "Hopefully, we'll",
                                                                    "meet again sometime",
                                                                    "in the future. Good luck on",
                                                                    "your journeys, adventurer.",
                                                                    "*Cough cough* Now... What",
                                                                    "will be my next project?"
                                                                ])?;
                                                            } else {
                                                                ctx.lines(args![
                                                                    "Anyway, I need to be",
                                                                    "working on a new project",
                                                                    "soon, so I suppose this is",
                                                                    "where we part ways for now.",
                                                                    "But I must say, it was truly",
                                                                    "a great pleasure to meet you..."
                                                                ])?;
                                                            }
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                        ctx.lines_as(
                                                            "Pisruik",
                                                            args![
                                                                "R-Really...?",
                                                                "Well, if you ever change",
                                                                "your mind, feel free to come",
                                                                "back for me to teach you."
                                                            ],
                                                        )?;
                                                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
                                                            ctx.lines(args![
                                                                "And it's no trouble at all!",
                                                                "I really enjoy your company..."
                                                            ])?;
                                                        }
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else {
                                                        ctx.lines(args![
                                                            "^3355FFUnfortunately, there",
                                                            "is nothing in your inventory",
                                                            "that seems like a suitable",
                                                            "replacement for Pisruik's",
                                                            "broken glasses.^000000"
                                                        ])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                }
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args!["Listen, you look so", "much better when you're", "not wearing glasses."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Pisruik", args!["Excuse me,", "come again?"])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args!["Hahahahhaha~!", "No-nothing at all!"],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ctx.var("alche_sk").get()? == 10 {
                                                ctx.lines_as(
                                                    "Pisruik",
                                                    args![
                                                        ((Val::from("So, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                            + Val::from(",")),
                                                        "Would you like me to",
                                                        "teach you the results",
                                                        "of the research I've''",
                                                        "been conducting?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                if Val::from(runtime::select_values(ctx, &[Val::from("Yes!:No, thanks.")])?) == 1 {
                                                    ctx.lines_as(
                                                        "Pisruik",
                                                        args![
                                                            ((Val::from("Great, ")
                                                                + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from("!")),
                                                            "I know I can trust you",
                                                            "to use this research for",
                                                            "good and noble ends. Now,",
                                                            "please read this thesis and",
                                                            "all of my additional notes..."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^3355FFPisruik thoroughly",
                                                        "explains the properties",
                                                        "of his medicine, the reaction",
                                                        "of the human organs to it, as",
                                                        "well as a few warnings about",
                                                        "the medicine's side effects.^000000"
                                                    ])?;
                                                    ctx.next()?;
                                                    ctx.var("alche_sk").set(Val::from(11))?;
                                                    ctx.call(Function::GetItem, vec![Val::from(7434), Val::from(1)])?;
                                                    ctx.lines_as(
                                                        "Pisruik",
                                                        args![
                                                            "Well, you should be",
                                                            "ready to make your own",
                                                            "potions that are a variation",
                                                            "of my medicine. But you'll",
                                                            "probably need to keep that",
                                                            "thesis as a ready reference."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.mes("[Pisruik]")?;
                                                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                                        ctx.lines(args![
                                                            "Hopefully, we'll",
                                                            "meet again sometime",
                                                            "in the future. Good luck on",
                                                            "your journeys, adventurer.",
                                                            "*Cough cough* Now... What",
                                                            "will be my next project?"
                                                        ])?;
                                                    } else {
                                                        ctx.lines(args![
                                                            "Anyway, I need to be",
                                                            "working on a new project",
                                                            "soon, so I suppose this is",
                                                            "where we part ways for now.",
                                                            "But I must say, it was truly",
                                                            "a great pleasure to meet you..."
                                                        ])?;
                                                    }
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                ctx.lines_as(
                                                    "Pisruik",
                                                    args![
                                                        "R-Really...?",
                                                        "Well, if you ever change",
                                                        "your mind, feel free to come",
                                                        "back for me to teach you."
                                                    ],
                                                )?;
                                                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
                                                    ctx.lines(args!["And it's no trouble at all!", "I really enjoy your company..."])?;
                                                }
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ctx.var("alche_sk").get()? == 11 {
                                                if ctx.call(Function::CountItem, vec![Val::from(7434)])? == 0 {
                                                    ctx.lines_as(
                                                        "Pisruik",
                                                        args![
                                                            "Uh oh...",
                                                            "You lost the thesis",
                                                            "I wrote for you? I don't",
                                                            "have the time to write",
                                                            "another one for you now..."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if ctx.call(Function::CountItem, vec![Val::from(7434)])? == 1 {
                                                    ctx.lines_as(
                                                        "Pisruik",
                                                        args![
                                                            "So, how have you been",
                                                            "using the potions that",
                                                            "I've taught you to make?",
                                                            "Hopefully, they'll come",
                                                            "in handy in your adventures."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if ctx.call(Function::CountItem, vec![Val::from(7434)])?.number()? > 1 {
                                                    ctx.lines_as(
                                                        "Pisruik",
                                                        args![
                                                            "Huh, so copies of my",
                                                            "thesis are circulating",
                                                            "around in public? Well,",
                                                            "I'm sorry, but I don't have",
                                                            "time to autograph your copy..."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            } else {
                                                ctx.lines_as(
                                                    "Pisruik",
                                                    args![
                                                        "Mmm...?",
                                                        "Did you need anything",
                                                        "in particular? Though,",
                                                        "I'm afraid someone in",
                                                        "my position won't be",
                                                        "much help to you."
                                                    ],
                                                )?;
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
    } else {
        ctx.lines_as(
            "Pisruik",
            args![
                "Mmm...?",
                "Did you need anything",
                "in particular? Though,",
                "I'm afraid someone in",
                "my position won't be",
                "much help to you."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn pisruik_qsk_al(ctx: &Ctx) -> Script {
    pisruik_qsk_al_body(ctx, Vec::new()).map(|_| ())
}

fn irache_qsk_al_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Irache",
        args![
            "Heh heh heh...!",
            "It's done! With this",
            "formula, I can melt any",
            "substance in the world!",
            "Hahahaha! Nothing stands",
            "between me and world domi--"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Irache",
        args![
            "OWWWW!",
            "The secret formula!",
            "It's burning through",
            "the test tube! I've made",
            "it too powerful! Confound it!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn irache_qsk_al(ctx: &Ctx) -> Script {
    irache_qsk_al_body(ctx, Vec::new()).map(|_| ())
}

fn degas_qsk_al_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Degas",
        args![
            "It's such a pain working",
            "so close to these other",
            "scientists. The guy next",
            "time is always cackling",
            "about taking over the",
            "world and whatnot."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Degas",
        args![
            "And this other geek is",
            "always coughing. Between",
            "the two of them, it's far too",
            "noisy to focus on my research!",
            "If only I could work in my very own private, secret laboratory..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn degas_qsk_al(ctx: &Ctx) -> Script {
    degas_qsk_al_body(ctx, Vec::new()).map(|_| ())
}

fn pile_of_books_qsk_al_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFIt's simply a pile",
        "of scattered documents.",
        "Although it seems unorganized,",
        "it doesn't look like the people",
        "here have any trouble finding^FFFFFF ^3355FF what they need when they need it."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn pile_of_books_qsk_al(ctx: &Ctx) -> Script {
    pile_of_books_qsk_al_body(ctx, Vec::new()).map(|_| ())
}
