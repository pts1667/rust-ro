use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn kidd_hall_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mao_request").get()? == 124 {
        ctx.lines_as(
            "Kidd",
            args![
                "Something major is going",
                "to happen... You've got to",
                "call the other members of",
                "Dandelion, and I've got to",
                "contact everyone in my guild!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dandelion Member",
            args![
                "Let me go, I can feel",
                "something happening in",
                "Morocc Castle! We have",
                "to go over there right now!"
            ],
        )?;
        ctx.close_window()?;
    } else {
        if (ctx.var("mao_request").get()?.number()? < 3
            || (ctx.var("mao_request").get()?.number()? > 102 && ctx.var("mao_request").get()?.number()? < 124))
        {
            ctx.lines_as("Kidd", args!["......", "........."])?;
            ctx.close_window()?;
        } else {
            if ctx.var("mao_request").get()? == 3 {
                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                ctx.lines_as(
                    "Kidd",
                    args![
                        "Oh good, you're here",
                        "just in time. This member",
                        "from Dandelion is going to",
                        "explain our mission to us.",
                        "Now, if you'll give him",
                        "your full attention..."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                ctx.lines_as(
                    "Dandelion Member",
                    args![
                        "Ah, so you're the one",
                        "working with Kidd on this",
                        "mission? I'm your client,",
                        "the representative of the",
                        "Dandelion Organization."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dandelion Member",
                    args![
                        "If you don't already know,",
                        "we're a public service group",
                        "that does great volunteer work.",
                        "However, one of our projects,",
                        "a child daycare center, was",
                        "ruined by a Mr. Raiyan Moore."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dandelion Member",
                    args![
                        "It's horrible what he did:",
                        "Raiyan Moore kidnapped all",
                        "of the children at our daycare",
                        "center! We can't help but feel",
                        "responsible, and we're doing",
                        "all that we can to find them..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dandelion Member",
                    args![
                        "However, the people of",
                        "our organization don't have",
                        "the skills to find him, much",
                        "less deal with this dangerous",
                        "man. That's why we're asking",
                        "for your help in this matter."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                ctx.lines_as(
                    "Kidd",
                    args![
                        "That's how the children",
                        "have been disappearing?",
                        "Hmm. According to Raiyan",
                        "Moore's profile, he's more",
                        "of an academic or a scholar",
                        "than a dangerous kidnapper..."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                ctx.lines_as(
                    "Dandelion Member",
                    args![
                        "Yes, he would appear",
                        "innocuous according to",
                        "the file we've given you,",
                        "but I do not think that",
                        "Raiyan Moore is a person",
                        "that we should underestimate."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFThe representative from",
                    "the Dandelion Organization",
                    "pulled out a necklace pendant",
                    "from under his shirt, pressed",
                    "it to his forehead, and then",
                    "quickly mumbled some words.^000000"
                ])?;
                ctx.var("mao_request").set(Val::from(4))?;
                ctx.close_window()?;
            } else {
                if ctx.var("mao_request").get()? == 4 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["(^333333Kidd? What...", "What is he doing", "with that pendant?^000000)"],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Kidd",
                        args![
                            "I've seen him do that",
                            "a couple of times. I dunno,",
                            "I guess he's just praying.",
                            "Though, I doubt it's to any",
                            "deity or god or whatever",
                            "that I'm familiar with..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kidd",
                        args![
                            "Anyway, back to the task at",
                            "hand. All we know is that our",
                            "target was last seen checking",
                            "out the Morocc ruins. Now, I think that we should begin our search ",
                            "by visiting the Juno Library."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Why should we go there?"), Val::from("Right, I understand!")])? {
                        1 => {
                            ctx.lines_as(
                                "Kidd",
                                args![
                                    "Our target, Raiyan Moore",
                                    "is a scholar, an academic.",
                                    "If he was researching the",
                                    "Morocc Ruins, there's a good",
                                    "chance that he went to the",
                                    "Juno Library beforehand."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kidd",
                                args![
                                    "If we're lucky, we might",
                                    "find some clue as to what",
                                    "Raiyan Moore is trying to do,",
                                    "and where he might be now.",
                                    "Pretty good idea, huh?"
                                ],
                            )?;
                        }
                        2 => {
                            ctx.lines_as(
                                "Kidd",
                                args![
                                    "I know that Juno is pretty",
                                    "far, but I think it's the",
                                    "best lead to follow for now.",
                                    "It's not an easy journey to",
                                    "make, so I wanna thank you",
                                    "for being so understanding."
                                ],
                            )?;
                        }
                        _ => {}
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        "Kidd",
                        args![
                            "Anyway, if we're lucky,",
                            "maybe someone there might",
                            "be familiar with Raiyan's work",
                            "and could point us in the right",
                            "direction. Alright then, I'll meet up with you in the Juno Library."
                        ],
                    )?;
                    ctx.var("mao_request").set(Val::from(5))?;
                    ctx.close_window()?;
                } else {
                    if (ctx.var("mao_request").get()?.number()? > 4 && ctx.var("mao_request").get()?.number()? < 8) {
                        ctx.lines_as(
                            "Kidd",
                            args![
                                "Hmm...? What are you still",
                                "doing here? There's no need",
                                "to wait up for me, I'll just catch up with you at the Juno Library.",
                                "Besides, I've got a few other",
                                "things to take care of first..."
                            ],
                        )?;
                        ctx.close_window()?;
                    } else {
                        if ctx.var("mao_request").get()? == 8 {
                            ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kidd",
                                args![
                                    "Hey you, Dandelion guy,",
                                    "you know why these four",
                                    "directions in Morocc are",
                                    "so important to Moore?",
                                    "He wrote something about",
                                    "it in his research journal."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                            ctx.lines_as(
                                "Dandelion Member",
                                args![
                                    "Hm? No, not really.",
                                    "In fact, it's even more",
                                    "baffling. What could be",
                                    "here in Morocc, and how",
                                    "can it possibly relate",
                                    "to the missing children?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kidd",
                                args![
                                    "Yeah, well, it's all we",
                                    "have to go on for now.",
                                    "Although he's probably",
                                    "gone by now, we need to",
                                    "follow through on this lead... If we're lucky, we'll catch Moore."
                                ],
                            )?;
                            ctx.var("mao_request").set(Val::from(9))?;
                            ctx.close_window()?;
                        } else {
                            if ctx.var("mao_request").get()? == 9 {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["So...", "How should we go", "about investigating the", "four directions in Morocc?"],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Kidd",
                                    args![
                                        "Hmm... Why don't we try",
                                        "this? I'll check the south",
                                        "part of Morocc for anything",
                                        "out of the ordinary, while you",
                                        "check the west part of Morocc."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Kidd",
                                    args![
                                        "Since Moore was performing",
                                        "historical research, it would",
                                        "probably be best for you to",
                                        "investigate the pyramids west",
                                        "of Morocc. If you happen to find anything, then meet me back here."
                                    ],
                                )?;
                                ctx.var("mao_request").set(Val::from(10))?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                ctx.lines_as(
                                    "Dandelion Member",
                                    args![
                                        "Good luck to the",
                                        "two of you. Although",
                                        "I have no idea what you",
                                        "might find, I hope that it",
                                        "will lead you to Raiya Moore..."
                                    ],
                                )?;
                                ctx.close_window()?;
                            } else {
                                if ctx.var("mao_request").get()? == 10 {
                                    ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Kidd",
                                        args![
                                            "For now, investigate",
                                            "the west part of Morocc",
                                            "where the pyramids are",
                                            "located. If you find anything",
                                            "that may interest Moore, come",
                                            "back and report it to me here."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                } else {
                                    if ctx.var("mao_request").get()? == 11 {
                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "Kidd",
                                            args![
                                                "Ah, you've returned.",
                                                "I found some sort of",
                                                "elemental crest over in",
                                                "the south part of Morocc.",
                                                "It was full of Earth magic..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args![
                                                "Really? I actually found the",
                                                "Wind elemental crest west",
                                                "of Morocc. Do you think this",
                                                "means that there's elemental",
                                                "crests in the fields north and",
                                                "east of Morocc as well?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "Kidd",
                                            args![
                                                "I'm not sure, but I can tell",
                                                "that there's some serious power",
                                                "in those elemental crests. Now,",
                                                "from what we read in Moore's",
                                                "research, the four directions",
                                                "in Morocc have to be balanced."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                        ctx.lines_as(
                                            "Dandelion Member",
                                            args![
                                                "Hmm. Although your mission",
                                                "is to find Raiyan Moore and",
                                                "rescue the missing children,",
                                                "we cannot allow Moore to cause",
                                                "more harm. We can't ignore the crests, and should check them..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "Kidd",
                                            args![
                                                "I agree... In fact, the",
                                                "Earth crest seemed pretty",
                                                "unstable when I found it.",
                                                "For now, go and stabilize",
                                                "all four elemental crests",
                                                "located just outside of Morocc."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Kidd",
                                            args![
                                                "While you're doing that,",
                                                "I'm going to visit Morocc's",
                                                "local historian and see what",
                                                "I can learn about those crests",
                                                "and Raiyan Moore's intentions."
                                            ],
                                        )?;
                                        ctx.var("mao_request").set(Val::from(12))?;
                                        ctx.close_window()?;
                                    } else {
                                        if ctx.var("mao_request").get()? == 12 {
                                            ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Kidd",
                                                args![
                                                    "Hmm... If you want to",
                                                    "stabilize the crests, you'll",
                                                    "probably need to counter the",
                                                    "the element of a crest with the",
                                                    "opposing, superior element,",
                                                    "to balance them out, I guess."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Kidd",
                                                args![
                                                    "You'll need all four",
                                                    "types of those elemental",
                                                    "stones. Anyway, you already",
                                                    "know where the west crest is.",
                                                    "The south crest is in the lowest part of the field south of Morocc."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Kidd",
                                                args![
                                                    "As for the north and east",
                                                    "crest locations, you're on",
                                                    "your own. Anyway, don't forget",
                                                    "to bring a Flame Heart, Mystic",
                                                    "Frozen, Great Nature and",
                                                    "Rough Wind with you, okay?"
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                        } else {
                                            if (ctx.var("mao_request").get()?.number()? > 12
                                                && ctx.var("mao_request").get()?.number()? < 16)
                                            {
                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Kidd",
                                                    args![
                                                        "Have you been able to",
                                                        "find the other elemental",
                                                        "crests? Don't forget to",
                                                        "stabilize them by using",
                                                        "elemental stones of a",
                                                        "superior, opposing element."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                            } else {
                                                if ctx.var("mao_request").get()? == 16 {
                                                    ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                                    ctx.lines_as(
                                                        "Dandelion Member",
                                                        args![
                                                            "Is that what happened?",
                                                            "I can't believe the historian",
                                                            "would do that, Kidd. Did you",
                                                            "explain why you needed to",
                                                            "speak to him, and let him",
                                                            "know that it was urgent?"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                                                    ctx.lines_as("Kidd", args!["Hell, yeah...!", "Well, you know.", "In my usual way..."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args![
                                                            "I just came back",
                                                            "from stabilizing those",
                                                            "elemental crests. Wait,",
                                                            "did something happen?"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                                    ctx.lines_as(
                                                        "Dandelion Member",
                                                        args![
                                                            "I'm not sure...",
                                                            "Kidd seems very upset",
                                                            "for some reason. I'm",
                                                            "guessing something must",
                                                            "have happened when he",
                                                            "went to visit that historian."
                                                        ],
                                                    )?;
                                                    ctx.var("mao_request").set(Val::from(17))?;
                                                    ctx.close_window()?;
                                                } else {
                                                    if ctx.var("mao_request").get()? == 17 {
                                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                                                        ctx.lines_as(
                                                            "Kidd",
                                                            args![
                                                                "Alright...",
                                                                "So you know how",
                                                                "I was supposed to",
                                                                "visit our historian?",
                                                                "He, um, refused to see me..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args!["What...?!", "Why would he do", "something like that?"],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Kidd",
                                                            args![
                                                                "I have no idea!",
                                                                "But I personally think",
                                                                "that he's got something",
                                                                "against Assassins. So",
                                                                "I want you to go and",
                                                                "try talking to him..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                                                            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                                                        {
                                                            ctx.lines_as(
                                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                args!["Wait a second.", "I'm an Assassin too!"],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Kidd",
                                                                args![
                                                                    "Yeah, but he already",
                                                                    "recognizes me. At least",
                                                                    "you can go in disguise...",
                                                                    "You know, wear one of",
                                                                    "those cute hats or whatever."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                args!["Alright, alright.", "I'll try talking", "to him for you..."],
                                                            )?;
                                                            ctx.var("mao_request").set(Val::from(18))?;
                                                            ctx.close_window()?;
                                                        } else {
                                                            ctx.lines_as(
                                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                args!["Alright, alright.", "I'll try talking", "to him for you..."],
                                                            )?;
                                                            ctx.var("mao_request").set(Val::from(18))?;
                                                            ctx.close_window()?;
                                                        }
                                                    } else {
                                                        if ctx.var("mao_request").get()? == 18 {
                                                            ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                                                            ctx.lines_as(
                                                                "Kidd",
                                                                args![
                                                                    "You should be able to",
                                                                    "find our local historian in",
                                                                    "the Morocc Inn. If you can, ask",
                                                                    "him about the crests around",
                                                                    "Morocc and their importance."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Kidd",
                                                                args![
                                                                    "We need to gather as",
                                                                    "many clues as we can",
                                                                    "about Raiyan Moore's",
                                                                    "research so that we can",
                                                                    "better understand his motives."
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                        } else {
                                                            if ctx.var("mao_request").get()? == 19 {
                                                                ctx.call(
                                                                    Function::Cutin,
                                                                    vec![Val::from("mocseal_kid01.bmp"), Val::from(2)],
                                                                )?;
                                                                ctx.lines_as(
                                                                    "Kidd",
                                                                    args![
                                                                        "Hey, you're back.",
                                                                        "So were you able to",
                                                                        "talk to the historian?"
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                    args!["Say, what's that", "you're drinking?"],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.call(
                                                                    Function::Cutin,
                                                                    vec![Val::from("mocseal_dan01.bmp"), Val::from(0)],
                                                                )?;
                                                                ctx.lines_as(
                                                                    "Dandelion Member",
                                                                    args![
                                                                        "Oh, this?",
                                                                        "It's a Tropical Sograt.",
                                                                        "Trust me, it's delicious.",
                                                                        "Here, let me buy you a glass.",
                                                                        "Master! Let me have another",
                                                                        "glass of Tropical Sograt!"
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Master",
                                                                    args![
                                                                        "Alright, alright~",
                                                                        "Coming right up.",
                                                                        "Here you are, enjoy.",
                                                                        "Take this."
                                                                    ],
                                                                )?;
                                                                ctx.var("mao_request").set(Val::from(20))?;
                                                                ctx.call(Function::GetItem, vec![Val::from(12112), Val::from(1)])?;
                                                                ctx.next()?;
                                                                ctx.call(
                                                                    Function::Cutin,
                                                                    vec![Val::from("mocseal_kid01.bmp"), Val::from(2)],
                                                                )?;
                                                                ctx.lines_as(
                                                                    "Kidd",
                                                                    args![
                                                                        "Hey, drink that later.",
                                                                        "Did you figure out why",
                                                                        "the historian refused to",
                                                                        "see me? Gimme your report~"
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                            } else {
                                                                if ctx.var("mao_request").get()? == 20 {
                                                                    ctx.call(
                                                                        Function::Cutin,
                                                                        vec![Val::from("mocseal_kid01.bmp"), Val::from(2)],
                                                                    )?;
                                                                    ctx.lines_as(
                                                                        "Kidd",
                                                                        args![
                                                                            "Alright, now tell me.",
                                                                            "Were you able to talk",
                                                                            "to the historian? Also,",
                                                                            "did you figure out why",
                                                                            "he didn't wanna talk to me?"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                        args![
                                                                            "Oh, that? I never got to",
                                                                            "speak to the historian, but",
                                                                            "I did talk to his assistant.",
                                                                            "It turns out that you scared",
                                                                            "him off by brandishing your",
                                                                            "blades in front of him..."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Kidd",
                                                                        args![
                                                                            "You gotta be kidding me!",
                                                                            "I'm an Assassin! What am",
                                                                            "I supposed to be doing if",
                                                                            "not looking cool and totally",
                                                                            "hardcore, huh? Damn babies...",
                                                                            "So what else did you learn?"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Well, those elemental crests", "are supposed to protect some", "seal beneath Morocc Castle", "that keeps this monster, Satan", "Morocc, from invading our world. It might be just a legend, but..."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                        args![
                                                                            "The assistant told me that",
                                                                            "Thanatos Tower was actually",
                                                                            "a place originally used by",
                                                                            "Satan Morocc to summon his",
                                                                            "minions. Now, demons or angels",
                                                                            "or whatever inhabit that tower."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Kidd",
                                                                        args![
                                                                            "Alright...",
                                                                            "But what does all",
                                                                            "this have to do with",
                                                                            "the missing children?"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                        args![
                                                                            "Well... Let's say that",
                                                                            "Raiyan Moore's goal is to",
                                                                            "revive Satan Morocc, if it",
                                                                            "truly exists. It might be",
                                                                            "possible to do that by",
                                                                            "offering a blood sacrifice."
                                                                        ],
                                                                    )?;
                                                                    ctx.var("mao_request").set(Val::from(21))?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Kidd",
                                                                        args![
                                                                            "Then that would mean",
                                                                            "that the children were",
                                                                            "kidnapped to-- No way!",
                                                                            "That's... That's sick!"
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                } else {
                                                                    if ctx.var("mao_request").get()? == 21 {
                                                                        ctx.call(
                                                                            Function::Cutin,
                                                                            vec![Val::from("mocseal_kid01.bmp"), Val::from(2)],
                                                                        )?;
                                                                        ctx.lines_as(
                                                                            "Kidd",
                                                                            args![
                                                                                "You know... I've been",
                                                                                "thinking about what we",
                                                                                "should do next. If Satan",
                                                                                "Morocc really exists, then",
                                                                                "that would be a worldwide",
                                                                                "catastrophe, right?"
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Kidd",
                                                                            args![
                                                                                "However, we don't really",
                                                                                "know if such a powerful",
                                                                                "monster really does exist.",
                                                                                "I don't want to believe it,",
                                                                                "but we gotta investigate the",
                                                                                "possibility, find some proof."
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.call(
                                                                            Function::Cutin,
                                                                            vec![Val::from("mocseal_dan01.bmp"), Val::from(0)],
                                                                        )?;
                                                                        ctx.lines_as("Dandelion Member", args!["There are those seals", "around Morocc, but we're", "not sure if they help keep", "Satan Morocc sealed, or if", "their magic makes it easier for humans to live in that desert."])?;
                                                                        ctx.next()?;
                                                                        ctx.call(
                                                                            Function::Cutin,
                                                                            vec![Val::from("mocseal_kid01.bmp"), Val::from(2)],
                                                                        )?;
                                                                        ctx.lines_as(
                                                                            "Kidd",
                                                                            args![
                                                                                "Yeah, yeah, I know. Okay,",
                                                                                "this is what we'll do. You go",
                                                                                "to Thanatos Tower and see if",
                                                                                "you can find those kids, Raiyan",
                                                                                "Moore, or concrete proof of",
                                                                                "Satan Morocc's existence."
                                                                            ],
                                                                        )?;
                                                                        ctx.var("mao_request").set(Val::from(22))?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Kidd", args!["If you're lucky enough to", "encounter Raiyan Moore", "while you're there, bring him", "back, dead or alive, it doesn't", "matter. In the meantime, I'll talk to the historian's assistant..."])?;
                                                                        ctx.close_window()?;
                                                                    } else {
                                                                        if ctx.var("mao_request").get()? == 22 {
                                                                            ctx.call(
                                                                                Function::Cutin,
                                                                                vec![Val::from("mocseal_kid01.bmp"), Val::from(2)],
                                                                            )?;
                                                                            ctx.lines_as(
                                                                                "Kidd",
                                                                                args![
                                                                                    "So how's the investigation of",
                                                                                    "Thanatos Tower coming along?",
                                                                                    "Did you learn anything about",
                                                                                    "Raiyan Moore or Satan Morocc?"
                                                                                ],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            match runtime::select_values(
                                                                                ctx,
                                                                                &[Val::from("Yes, I did."), Val::from("No, not yet...")],
                                                                            )? {
                                                                                1 => {
                                                                                    if ctx.var("thana_quest").get()?.number()? > 1 {
                                                                                        ctx.lines_as(
                                                                                            ctx.call(
                                                                                                Function::StrCharInfo,
                                                                                                vec![Val::from(0)],
                                                                                            )?,
                                                                                            args![
                                                                                                "Well, I found some old log",
                                                                                                "entries about Satan Morocc.",
                                                                                                "They pretty much confirm that",
                                                                                                "Satan Morocc is real, and that",
                                                                                                "it's sealed under Morocc Castle. "
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            ctx.call(
                                                                                                Function::StrCharInfo,
                                                                                                vec![Val::from(0)],
                                                                                            )?,
                                                                                            args![
                                                                                                "The Rekenber Corporation",
                                                                                                "is also responsible for the",
                                                                                                "tower's reconstruction, and",
                                                                                                "they ultimately plan to repair",
                                                                                                "all twelve of its levels."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.call(
                                                                                            Function::Cutin,
                                                                                            vec![
                                                                                                Val::from("mocseal_kid01.bmp"),
                                                                                                Val::from(2),
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.lines_as(
                                                                                            "Kidd",
                                                                                            args![
                                                                                                "So it's true...",
                                                                                                "Satan Morocc does exist.",
                                                                                                "If Raiyan Moore kidnapped",
                                                                                                "those children, then he must",
                                                                                                "be planning to sacrifice them",
                                                                                                "to revive that monster."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.call(
                                                                                            Function::Cutin,
                                                                                            vec![
                                                                                                Val::from("mocseal_dan01.bmp"),
                                                                                                Val::from(0),
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.lines_as(
                                                                                            "Dandelion Member",
                                                                                            args![
                                                                                                "I wish we could say that we",
                                                                                                "were jumping to conclusions,",
                                                                                                "but Moore's research results",
                                                                                                "and his behavior... He can't be",
                                                                                                "planning anything else. This",
                                                                                                "is the worst case scenario..."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.call(
                                                                                            Function::Cutin,
                                                                                            vec![
                                                                                                Val::from("mocseal_kid01.bmp"),
                                                                                                Val::from(2),
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.lines_as(
                                                                                            "Kidd",
                                                                                            args![
                                                                                                "Damn it!",
                                                                                                "We have to find",
                                                                                                "Raiyan Moore now!",
                                                                                                "We're running out of",
                                                                                                "time! What'll we do?!"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.var("mao_request").set(Val::from(23))?;
                                                                                        ctx.close_window()?;
                                                                                    } else {
                                                                                        ctx.call(
                                                                                            Function::Cutin,
                                                                                            vec![
                                                                                                Val::from("mocseal_kid01.bmp"),
                                                                                                Val::from(2),
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.lines_as("Kidd", args!["Hmm... Your investigation", "wasn't thorough enough. You", "need to find concrete proof about Satan Morocc's existence or Raiyan", "Moore's intentions. Hurry back to Thanatos Tower and find it!"])?;
                                                                                        ctx.close_window()?;
                                                                                    }
                                                                                }
                                                                                2 => {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("mocseal_kid01.bmp"), Val::from(2)],
                                                                                    )?;
                                                                                    ctx.lines_as(
                                                                                        "Kidd",
                                                                                        args![
                                                                                            "Alright. Come back",
                                                                                            "as soon as you finish",
                                                                                            "your investigation. I'm",
                                                                                            "getting the feeling that we",
                                                                                            "no longer have the luxury",
                                                                                            "of time, so please hurry!"
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.close_window()?;
                                                                                }
                                                                                _ => {}
                                                                            }
                                                                        } else {
                                                                            if ctx.var("mao_request").get()? == 23 {
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("mocseal_kid01.bmp"), Val::from(2)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Kidd",
                                                                                    args![
                                                                                        "We need to calm down...",
                                                                                        "We're no good to those",
                                                                                        "kids if we let ourselves",
                                                                                        "get frustrated. Okay, did",
                                                                                        "you find any trace of Raiyan",
                                                                                        "Moore over at Thanatos Tower?"
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as(
                                                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                                    args![
                                                                                        "Not at all. I've been",
                                                                                        "asking around, but haven't",
                                                                                        "been able to get any clue as",
                                                                                        "to where he can be right now.",
                                                                                        "How can we possibly find him?"
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("mocseal_dan01.bmp"), Val::from(0)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Dandelion Member",
                                                                                    args![
                                                                                        "This is bad... Not only",
                                                                                        "are the missing children in",
                                                                                        "danger, but if Raiyan may even",
                                                                                        "revive Satan Morocc. We cannot",
                                                                                        "allow that to happen at any cost."
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("mocseal_kid01.bmp"), Val::from(2)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Kidd",
                                                                                    args![
                                                                                        "I know, I've been",
                                                                                        "searching places where",
                                                                                        "the children might have",
                                                                                        "been hidden, but there's",
                                                                                        "just so many of them. We",
                                                                                        "really got to think of ou--"
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("mocseal_kid01.bmp"), Val::from(255)],
                                                                                )?;
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("mocseal_dan01.bmp"), Val::from(255)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Litheron",
                                                                                    args![
                                                                                        "Um? I'm really sorry to",
                                                                                        "interrupt. I know I'm not",
                                                                                        "supposed to bother you guys",
                                                                                        "on assignment, but I overheard",
                                                                                        "you and wanted to ask about",
                                                                                        "the guy you're looking for."
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as(
                                                                                    "Dandelion Member",
                                                                                    args!["You mean...", "Raiyan Moore?"],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as(
                                                                                    "Litheron",
                                                                                    args![
                                                                                        "Yeah. Does he wear",
                                                                                        "the same uniform as you?",
                                                                                        "If he does, I think I saw him",
                                                                                        "here earlier going by the name",
                                                                                        "of ''Mr. R.'' Also, I think he",
                                                                                        "might also be Lin's client..."
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("mocseal_kid01.bmp"), Val::from(2)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Kidd",
                                                                                    args![
                                                                                        "What, Lin's client?!",
                                                                                        ((Val::from("Wait, ")
                                                                                            + ctx.call(
                                                                                                Function::StrCharInfo,
                                                                                                vec![Val::from(0)]
                                                                                            )?)
                                                                                            + Val::from(", do you recall")),
                                                                                        "what Lin's assignment was?"
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                match runtime::select_values(
                                                                                    ctx,
                                                                                    &[
                                                                                        Val::from("Well, um..."),
                                                                                        Val::from("Bodyguard, right?"),
                                                                                    ],
                                                                                )? {
                                                                                    1 => {
                                                                                        ctx.lines_as(
                                                                                            "Kidd",
                                                                                            args![
                                                                                                "Damn it!",
                                                                                                "I don't remember",
                                                                                                "it either! But I've got",
                                                                                                "a bad feeling about this!",
                                                                                                "We gotta go talk to our",
                                                                                                "commanding officer!"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.var("mao_request").set(Val::from(24))?;
                                                                                    }
                                                                                    2 => {
                                                                                        ctx.lines_as(
                                                                                            "Kidd",
                                                                                            args![
                                                                                                "That's right! She's",
                                                                                                "supposed to be a bodyguard",
                                                                                                "for someone called ''Mr. R!''",
                                                                                                "How could we overlook this?!",
                                                                                                "Damn it, we need to talk to our",
                                                                                                "commanding officer right now!"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.var("mao_request").set(Val::from(24))?;
                                                                                    }
                                                                                    _ => {}
                                                                                }
                                                                                ctx.next()?;
                                                                                ctx.lines_as(
                                                                                    "Litheron",
                                                                                    args![
                                                                                        "Hey, hey...",
                                                                                        "What's going on?",
                                                                                        "Was I not supposed to",
                                                                                        "tell that to you guys?"
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as(
                                                                                    "Kidd",
                                                                                    args![
                                                                                        "No, no...",
                                                                                        "You were actually",
                                                                                        "a really great help!",
                                                                                        "Thanks, Litheron, ",
                                                                                        "I'll buy you a drink later~"
                                                                                    ],
                                                                                )?;
                                                                                ctx.close_window()?;
                                                                            } else {
                                                                                if ctx.var("mao_request").get()? == 24 {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("mocseal_kid01.bmp"), Val::from(2)],
                                                                                    )?;
                                                                                    ctx.lines_as("Kidd", args!["Now I remember...!", "Lin is supposed to work", "as a bodyguard for someone", "named, ''Mr. R!'' We're looking", "for Raiyan Moore--if they're the same person-- then that means--"])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as(
                                                                                        "Kidd",
                                                                                        args![
                                                                                            "Hurry, we need to tell",
                                                                                            "our commanding officer,",
                                                                                            "Valdes! I'll meet you there",
                                                                                            "in a flash, so just get over",
                                                                                            "there as quickly as you can!"
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.close_window()?;
                                                                                } else if ctx.var("mao_request").get()? == 25 {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("mocseal_kid01.bmp"), Val::from(2)],
                                                                                    )?;
                                                                                    ctx.lines_as("Kidd", args!["Raiyan Moore's supposed", "to be in the room on the other", "side of the commanding officer's room. Check it, quickly! Once", "I get my hands on that guy..."])?;
                                                                                    ctx.close_window()?;
                                                                                } else if (ctx.var("mao_request").get()? == 26
                                                                                    || ctx.var("mao_request").get()? == 27)
                                                                                {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("mocseal_kid01.bmp"), Val::from(2)],
                                                                                    )?;
                                                                                    ctx.lines_as("Kidd", args!["Something huge is going", "on at Morocc Castle. If that's", "where Satan Morocc is sealed,", "then the demon is beginning", "to revive. We gotta stop it before Satan Morocc can enter our world!"])?;
                                                                                    ctx.close_window()?;
                                                                                } else if (((ctx.var("mao_request").get()? == 28
                                                                                    || ctx.var("mao_request").get()? == 29)
                                                                                    || ctx.var("mao_request").get()? == 126)
                                                                                    || ctx.var("mao_request").get()? == 127)
                                                                                {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("mocseal_kid01.bmp"), Val::from(2)],
                                                                                    )?;
                                                                                    ctx.lines_as(
                                                                                        "Kidd",
                                                                                        args![
                                                                                            "Man, that was close!",
                                                                                            "We got really lucky.",
                                                                                            "Yeah... Let's talk about this",
                                                                                            "some more with Valdes in",
                                                                                            "the commanding officer's room."
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.next()?;
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("mocseal_dan01.bmp"), Val::from(0)],
                                                                                    )?;
                                                                                    ctx.lines_as(
                                                                                        "Dandelion Member",
                                                                                        args![
                                                                                            "I still can't believe it...",
                                                                                            "Those poor, poor children..."
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.call(
                                                                                        Function::Emotion,
                                                                                        vec![
                                                                                            ctx.constant("ET_SCRATCH")?,
                                                                                            Val::from(
                                                                                                ctx.call(
                                                                                                    Function::GetCharacterId,
                                                                                                    vec![Val::from(0)],
                                                                                                )?
                                                                                                .is_true(),
                                                                                            ),
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.close_window()?;
                                                                                } else if (ctx.var("mao_request").get()?.number()? > 102
                                                                                    && ctx.var("mao_request").get()?.number()? < 126)
                                                                                {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("mocseal_kid01.bmp"), Val::from(2)],
                                                                                    )?;
                                                                                    ctx.lines_as(
                                                                                        "Kidd",
                                                                                        args![
                                                                                            "Hey, you're...",
                                                                                            "You're Lin's partner,",
                                                                                            "aren't you? Good luck",
                                                                                            "working with her-- she",
                                                                                            "can be pretty bossy."
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.close_window()?;
                                                                                } else {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("mocseal_kid01.bmp"), Val::from(1)],
                                                                                    )?;
                                                                                    ctx.lines_as(
                                                                                        "Kidd",
                                                                                        args![
                                                                                            "Damn it, I feel like",
                                                                                            "we all failed. I can",
                                                                                            "understand how Lin feels.",
                                                                                            "Still, I hate standing by",
                                                                                            "and waiting around..."
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.close_window()?;
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
                            }
                        }
                    }
                }
            }
        }
    }
    ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(255)])?;
    ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn kidd_hall(ctx: &Ctx) -> Script {
    kidd_hall_body(ctx, Vec::new()).map(|_| ())
}
