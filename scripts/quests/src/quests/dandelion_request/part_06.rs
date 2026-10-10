use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn dandelion_member_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(12112), Val::from(1)])? != 1 {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("mao_request").get()? == 124 {
        ctx.lines_as(
            "Dandelion Member",
            args![
                "H-hey...!",
                "Did you feel that?",
                "That eerie vibration",
                "of incredibly dark power..."
            ],
        )?;
        ctx.next()?;
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
            ctx.lines_as(
                "Dandelion Member",
                args!["Have you ever heard of", "the Dandelion organization?"],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Yes"), Val::from("No")])? {
                1 => {
                    ctx.lines_as(
                        "Dandelion Member",
                        args![
                            "Ah, so you already know",
                            "about us? I feel so ashamed...",
                            "We let that Raiyan Moore kidnap",
                            "the children that were entrusted to our care. I don't know if we",
                            "can ever forgive ourselves."
                        ],
                    )?;
                    ctx.close_window()?;
                }
                2 => {
                    ctx.lines_as(
                        "Dandelion Member",
                        args![
                            "You really haven't?",
                            "Well, I suppose we're",
                            "not a very popular or",
                            "glamorous group. Basically,",
                            "we're a non-profit organization working to improve public welfare."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dandelion Member",
                        args![
                            "We volunteer to clean the",
                            "streets, repair homes for",
                            "lower income households, and",
                            "even operated a day care center.",
                            "However, Raiyan Moore kidnapped all the children under our care..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dandelion Member",
                        args![
                            "I can only imagine the",
                            "suffering those poor kids",
                            "must be going through. I hope",
                            "that we can save them soon!"
                        ],
                    )?;
                    ctx.close_window()?;
                }
                _ => {}
            }
        } else {
            if ctx.var("mao_request").get()? == 3 {
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
                    ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                    ctx.lines_as(
                        "Dandelion Member",
                        args![
                            "All we know is that",
                            "Raiyan Moore has last",
                            "been spotted investigating",
                            "the ruins around Morocc.",
                            "For now, I suggest that you plan your next action with Kidd."
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
                    ctx.close_window()?;
                } else {
                    if (ctx.var("mao_request").get()?.number()? > 4 && ctx.var("mao_request").get()?.number()? < 8) {
                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                        ctx.lines_as(
                            "Dandelion Member",
                            args![
                                "Are you leaving now?",
                                "I hope that you catch",
                                "Raiyan Moore and that",
                                "you can rescue those",
                                "poor kids. Only a monster",
                                "would stoop to kidnapping..."
                            ],
                        )?;
                        ctx.close_window()?;
                    } else {
                        if ctx.var("mao_request").get()? == 8 {
                            ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                            ctx.lines_as(
                                "Dandelion Member",
                                args![
                                    "So you found some",
                                    "sort of clue as to where",
                                    "Raiyan would be? I agree",
                                    "that it sounds like a long",
                                    "shot, but we've got to try",
                                    "everything that we can!"
                                ],
                            )?;
                            ctx.close_window()?;
                        } else {
                            if ctx.var("mao_request").get()? == 9 {
                                ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                                ctx.lines_as(
                                    "Dandelion Member",
                                    args![
                                        "Raiyan seems to have",
                                        "been conducting some very",
                                        "peculiar research. What could",
                                        "the meaning of ''Morocc's four",
                                        "directions'' be? How can it",
                                        "possibly be of significance?"
                                    ],
                                )?;
                                ctx.close_window()?;
                            } else {
                                if ctx.var("mao_request").get()? == 10 {
                                    ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                                    ctx.lines_as(
                                        "Dandelion Member",
                                        args![
                                            "So you're really going to",
                                            "check the four directions",
                                            "in Morocc... What could",
                                            "possibly interest Raiyan",
                                            "Moore in Morocc? Perhaps",
                                            "he is not just a kidnapper..."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                } else {
                                    if ctx.var("mao_request").get()? == 11 {
                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                                        ctx.lines_as(
                                            "Dandelion Member",
                                            args![
                                                "You found a crest shaped",
                                                "like the Wind? Ah, Kidd",
                                                "mentioned that he found an",
                                                "elemental crest imbued with",
                                                "the power of Earth. You should",
                                                "share your findings with him..."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                    } else {
                                        if (ctx.var("mao_request").get()?.number()? > 11 && ctx.var("mao_request").get()?.number()? < 16) {
                                            ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                                            ctx.lines_as(
                                                "Dandelion Member",
                                                args![
                                                    "I don't understand what",
                                                    "could be so important about",
                                                    "these elemental crest devices",
                                                    "hidden in Morocc. What could",
                                                    "Raiyan Moore be planning...?"
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
                                                    ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                                    ctx.lines_as(
                                                        "Dandelion Member",
                                                        args![
                                                            "Now why would Morocc's",
                                                            "local historian refuse to see",
                                                            "Kidd? Maybe he didn't fully",
                                                            "understand the importance",
                                                            "of this mission? How strange..."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                } else {
                                                    if ctx.var("mao_request").get()? == 18 {
                                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                                        ctx.lines_as(
                                                            "Dandelion Member",
                                                            args![
                                                                "We have so very few leads",
                                                                "to finding Raiyan Moore, it's",
                                                                "completely frustrating. I know",
                                                                "that we have no choice, but",
                                                                "I'm worried to death about",
                                                                "those kidnapped children..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                                                        ctx.lines_as(
                                                            "Kidd",
                                                            args![
                                                                "Try not to worry",
                                                                "so much. We'll just",
                                                                "do what we always do:",
                                                                "everything we can. I only",
                                                                "hope it'll be enough this time."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                    } else {
                                                        if ctx.var("mao_request").get()? == 19 {
                                                            ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                                                            ctx.lines_as(
                                                                "Dandelion Member",
                                                                args![
                                                                    "Oh, you've returned?",
                                                                    "Were you able to talk to",
                                                                    "the historian? Heh, while you",
                                                                    "were gone, I've been helping",
                                                                    "myself to some of these drinks~"
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                args!["Say, what's that", "you're drinking?"],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
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
                                                            ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
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
                                                                    vec![Val::from("mocseal_dan01.bmp"), Val::from(0)],
                                                                )?;
                                                                ctx.lines_as(
                                                                    "Dandelion Member",
                                                                    args![
                                                                        "Isn't Tropical Sograt",
                                                                        "so good? I can't believe",
                                                                        "Assassins can enjoy this",
                                                                        "drink whenever they want~"
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
                                                                        "Don't forget that",
                                                                        "we've got a mission",
                                                                        "to finish! Come on,",
                                                                        "gimme your report now.",
                                                                        "What'd you learn exactly?"
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                            } else {
                                                                if ctx.var("mao_request").get()? == 21 {
                                                                    ctx.call(
                                                                        Function::Cutin,
                                                                        vec![Val::from("mocseal_dan01.bmp"), Val::from(1)],
                                                                    )?;
                                                                    ctx.lines_as(
                                                                        "Dandelion Member",
                                                                        args![
                                                                            "Raiyan Moore...",
                                                                            "The missing children...",
                                                                            "Satan Morocc and Thanatos",
                                                                            "Tower. They couldn't all be",
                                                                            "connected somehow, can they?"
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                } else {
                                                                    if ctx.var("mao_request").get()? == 22 {
                                                                        ctx.call(
                                                                            Function::Cutin,
                                                                            vec![Val::from("mocseal_dan01.bmp"), Val::from(1)],
                                                                        )?;
                                                                        ctx.lines_as(
                                                                            "Dandelion Member",
                                                                            args![
                                                                                "I hope that you'll be",
                                                                                "able to find Raiyan Moore",
                                                                                "at Thanatos Tower. Though,",
                                                                                "it's a public place, so I don't",
                                                                                "think he'd linger there for",
                                                                                "too long. Good luck..."
                                                                            ],
                                                                        )?;
                                                                        ctx.close_window()?;
                                                                    } else {
                                                                        if ctx.var("mao_request").get()? == 23 {
                                                                            ctx.call(
                                                                                Function::Cutin,
                                                                                vec![Val::from("mocseal_dan01.bmp"), Val::from(1)],
                                                                            )?;
                                                                            ctx.lines_as(
                                                                                "Dandelion Member",
                                                                                args![
                                                                                    "That monster...",
                                                                                    "So he kidnapped the",
                                                                                    "children to-- to sacrifice",
                                                                                    "them?! That's... unthinkable.",
                                                                                    "All to bring back some dead",
                                                                                    "demon? Why would he do that?!"
                                                                                ],
                                                                            )?;
                                                                            ctx.close_window()?;
                                                                        } else {
                                                                            if ctx.var("mao_request").get()? == 24 {
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("mocseal_dan01.bmp"), Val::from(0)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Dandelion Member",
                                                                                    args![
                                                                                        "Raiyan Moore kidnapped",
                                                                                        "the children to revive Satan",
                                                                                        "Morocc, and had the nerve",
                                                                                        "to hire the Assassin Guild",
                                                                                        "to protect him?! He's truly",
                                                                                        "a devious mastermind..."
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
                                                                                        "Being duped by that guy...",
                                                                                        "I've never been so insulted!",
                                                                                        "It's decided: that Raiyan",
                                                                                        "Moore doesn't deserve",
                                                                                        "to take another breath."
                                                                                    ],
                                                                                )?;
                                                                                ctx.close_window()?;
                                                                            } else if ctx.var("mao_request").get()? == 25 {
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("mocseal_dan01.bmp"), Val::from(0)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Dandelion Member",
                                                                                    args![
                                                                                        "I can't believe that",
                                                                                        "Raiyan Moore was under",
                                                                                        "our noses this whole time...",
                                                                                        "Do you know where he is now?"
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
                                                                                        "Yeah. Yeah...",
                                                                                        "Don't get too",
                                                                                        "worked up, he'll",
                                                                                        "be right here soon."
                                                                                    ],
                                                                                )?;
                                                                                ctx.close_window()?;
                                                                            } else if (ctx.var("mao_request").get()? == 26
                                                                                || ctx.var("mao_request").get()? == 27)
                                                                            {
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("mocseal_dan01.bmp"), Val::from(1)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Dandelion Member",
                                                                                    args![
                                                                                        "This is bad news...",
                                                                                        "It looks like Raiyan Moore",
                                                                                        "is performing the ritual to",
                                                                                        "revive Satan Morocc. We need",
                                                                                        "to go to Morocc Castle, where",
                                                                                        "Satan Morocc is sealed..."
                                                                                    ],
                                                                                )?;
                                                                                ctx.close_window()?;
                                                                            } else if (((ctx.var("mao_request").get()? == 28
                                                                                || ctx.var("mao_request").get()? == 29)
                                                                                || ctx.var("mao_request").get()? == 126)
                                                                                || ctx.var("mao_request").get()? == 127)
                                                                            {
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("mocseal_dan01.bmp"), Val::from(1)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Dandelion Member",
                                                                                    args![
                                                                                        "Well, we've sent some",
                                                                                        "people to purse Raiyan,",
                                                                                        "though I honestly doubt",
                                                                                        "that they'll be able to get",
                                                                                        "him. Luckily, though, you",
                                                                                        "seem to be alright..."
                                                                                    ],
                                                                                )?;
                                                                                ctx.close_window()?;
                                                                            } else if (ctx.var("mao_request").get()?.number()? > 102
                                                                                && ctx.var("mao_request").get()?.number()? < 126)
                                                                            {
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("mocseal_dan01.bmp"), Val::from(1)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Dandelion Member",
                                                                                    args![
                                                                                        "I'm sorry...",
                                                                                        "But I'm far too busy",
                                                                                        "to speak with you now.",
                                                                                        "This matter requires",
                                                                                        "my full attention!"
                                                                                    ],
                                                                                )?;
                                                                                ctx.close_window()?;
                                                                            } else {
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("mocseal_dan01.bmp"), Val::from(0)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Dandelion Member",
                                                                                    args![
                                                                                        "Those poor children...",
                                                                                        "And what are we going",
                                                                                        "to tell their parents?",
                                                                                        "Still, perhaps this is",
                                                                                        "the will of ^4D4DFFFreya^000000."
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as(
                                                                                    "Dandelion Member",
                                                                                    args![
                                                                                        "Their deaths may have",
                                                                                        "been horrific, but I believe",
                                                                                        "that those children are now",
                                                                                        "safe in Freya's loving arms.",
                                                                                        "Who can understand the ",
                                                                                        "divine will of Freya?"
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
                                                                                        "Freya...?",
                                                                                        "I thought most",
                                                                                        "people around here",
                                                                                        "prayed to Odin. Well,",
                                                                                        "I suppose that explains",
                                                                                        "a couple things."
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
    ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(255)])?;
    ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn dandelion_member(ctx: &Ctx) -> Script {
    dandelion_member_body(ctx, Vec::new()).map(|_| ())
}

fn reading_girl_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mao_request").get()? == 5 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Excuse me, but may I ask",
                "you a question? I'm hoping",
                "that you might be able to",
                "help me with something."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Yunia",
            args![
                "Eh? Oh, I'm sorry, I was",
                "so busy reading this book!",
                "So, uh, what exactly did",
                "you want to ask me?"
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("Do you know Raiyan Moore?"), Val::from("What are you reading?")],
            )?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Yunia",
                    args![
                        "Raiyan Moore?",
                        "I don't know any--",
                        "Oh, you must mean",
                        "Mr. R. Moore. Yes,",
                        "I suppose I do if his",
                        "first name is Raiyan."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Yunia",
                    args![
                        "I'm Yunia, Mr. Moore's",
                        "temporary assistant while",
                        "he's working on his current",
                        "research project. I can't make",
                        "heads or tails out of whatever",
                        "he's been studying, though..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Great, when was the",
                        "last time you've seen",
                        "him? You see, we're",
                        "actually looking for him..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Yunia",
                    args![
                        "I actually saw him here",
                        "everyday in the library up",
                        "until a couple days ago. That's",
                        "when these strange men tried",
                        "to capture him, but luckily he",
                        "was able to escape."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Yunia",
                    args![
                        "I remember that things",
                        "were so crazy that day,",
                        "and Mr. Moore even forgot",
                        "to bring his documents with",
                        "him while he was running away."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Documents?"), Val::from("Where did he go?")])? {
                    1 => {
                        ctx.lines_as(
                            "Yunia",
                            args![
                                "Yes, he was perusing",
                                "some historic documents",
                                "before he ran off. They're",
                                "all about history, so I couldn't really understand them at all..."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(6))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Yunia",
                            args![
                                "Mm... I really don't",
                                "know? But I did keep and",
                                "organize the documents that",
                                "he was researching so that",
                                "they'll be ready when he",
                                "comes back to the library."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(6))?;
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
                ctx.lines_as(
                    "Yunia",
                    args![
                        "Oh... you know.",
                        "It's just this novel",
                        "About a boy who becomes.",
                        "a slave and he needs to",
                        "pay off his parents' debts.",
                        "But then, he falls in love..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Yunia",
                    args![
                        "And the girl he falls",
                        "in love with? He doesn't",
                        "know it yet, but she's the",
                        "dark overlord of darkness!",
                        "At least, I think she is. They",
                        "keep hinting at it, though."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if ctx.var("mao_request").get()? == 6 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I was wondering if", "I could look through", "that Mr. Moore was studying..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Yunia",
            args![
                "Well, I gave most of the",
                "documents that I organized",
                "to someone else already, but",
                "now that I think about it, there are a few leftover, unorganized",
                "files that you can check out."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Yunia",
            args![
                "Actually, the person that took",
                "the organized documents did",
                "so on Mr. Moore's behalf. Then,",
                "she quickly vanished before",
                "I could ask her for her name or Mr. Moore's contact information."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Damn! That information",
                "would have been really",
                "helpful! (^333333I better not let",
                "her know that I'm actually",
                "trying to hunt Raiyan Moore^FFFFFF ^333333 down, or that he's a kidnapper.^000000)"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Yunia",
            args![
                "I hope Mr. Moore is",
                "alright. Oh, why don't",
                "you try reading his notes",
                "and his journal on his desk?",
                "That might be really helpful."
            ],
        )?;
        ctx.var("mao_request").set(Val::from(7))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mao_request").get()? == 7 {
        ctx.lines_as(
            "Yunia",
            args![
                "Mr. Moore's desk?",
                "Just go upstairs and",
                "look for it in the corner.",
                "You should be able to",
                "easily find his notes and",
                "journal right on top of it."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mao_request").get()? == 105 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Excuse me, but I'm", "looking for a Ms. Yunia?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Yunia", args!["Oh, that's me!", "So how can I help you?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Mr. R's Documents"), Val::from("What are you reading?")])? {
            1 => {
                ctx.lines_as(
                    "Yunia",
                    args![
                        "Mr. R? Ohhhh...",
                        "Mr. R. Moore. Is...",
                        "Is he alright? I was",
                        "so scared when those",
                        "strange men attacked him",
                        "right here in Juno Library!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Oh, he's perfectly safe.",
                        "We've got a professional to",
                        "ensure nothing happens to him.",
                        "But yes, he wanted me to come",
                        "here to pick up some research documents. Do you know about that?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Yunia",
                    args![
                        "Ah, yes, I do!",
                        "I've been organizing",
                        "them for his return.",
                        "Would you like to take",
                        "a look before delivering",
                        "them to Mr. R. Moore?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("May I?"), Val::from("No, thanks...")])? {
                    1 => {}
                    2 => {}
                    _ => {}
                }
                ctx.lines_as(
                    "Yunia",
                    args![
                        "Heh heh~ Alright~",
                        "First you should make sure",
                        "that the documents you're",
                        "delivering are the ones",
                        "that he needs, am I right?"
                    ],
                )?;
                ctx.var("mao_request").set(Val::from(106))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Yunia",
                    args![
                        "Oh... You know. Just this",
                        "story about this guy who's",
                        "cursed so that he transforms",
                        "into a fat dork around beautiful girls, and into a svelte, handsome",
                        "man around dorky women."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Yunia",
                    args![
                        "So then he gets into this",
                        "crazy love triangle, and now",
                        "I'm at the part when he has to",
                        "go on a date with both a gorgeous girl AND a geeky girl. How's",
                        "he going to transform next...?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("mao_request").get()? == 106 {
        ctx.lines_as(
            "Yunia",
            args![
                "Okay, these should",
                "be the documents that",
                "Mr. Moore wants to read.",
                "Just read through them",
                "quickly to make sure that",
                "I gave you the right ones."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Let's see here...",
                "Some of these sentences",
                "are underlined... Ah, and here",
                "are some notes in the margins.",
                "Maybe this'll tell me about",
                "Mr. R. Moore's research..."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^4d4dffSatan Morocc appeared,",
            "turning the world into a",
            "living hell. Somehow, the",
            "monster was sealed, and a",
            "castle and town was built over^FFFFFF ^4d4dff its prison. This place is Morocc."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Whoa, that's actually",
                "pretty interesting. But",
                "I need to do my job first.",
                "Perhaps I'll ask Mr. R. Moore",
                "about this later. Hey, Yunia,",
                "thanks for all of your help."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Yunia",
            args![
                "You're welcome~",
                "Oh, and please give",
                "my regards to Mr. Moore",
                "when you see him, okay?"
            ],
        )?;
        ctx.var("mao_request").set(Val::from(107))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Excuse me...", "But what are", "you reading?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Yunia",
            args![
                "Oh... You know.",
                "This story about this girl",
                "who becomes a princess.",
                "And then she owns this harem",
                "of handsome boys, see? But",
                "then, she meets this one boy..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Yunia",
            args![
                "This boy refuses to join",
                "her harem, and it's, like, so",
                "ironic because he's the one she",
                "really wants. Even though any",
                "other boy in the world would",
                "join her harem ^FF0000willingly^000000."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Yunia",
            args![
                "Anyway, I'm reading the part",
                "where he-- his name's Extopher-- has to defeat Count Guillermo",
                "in a sword duel for the right",
                "to ride the unicorn pegasus.",
                "Ooh, what'll happen next?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn reading_girl(ctx: &Ctx) -> Script {
    reading_girl_body(ctx, Vec::new()).map(|_| ())
}

fn workbook_mao_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mao_request").get()? == 7 {
        if !(ctx.var(".mao_book").get()?.is_true()) {
            ctx.var(".mao_book").set((ctx.var(".mao_book").get()? + Val::from(1)))?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("#maobooktimer::OnEnter")])?;
            ctx.lines(args![
                "^3355FFYou find a crumpled piece",
                "of paper filled with scribbles.",
                "Apparently, this was ripped",
                "from Moore's journal.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Nuts...", "I came all the", "way to Juno to look", "through some trash?"],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(0)])?;
            ctx.lines_as(
                "Kidd",
                args![
                    "For the sake of those",
                    "missing kids, we're duty",
                    "bound to sift through every",
                    "piece of possible evidence,",
                    "not matter how useless it may",
                    "look! Don't throw it away yet!"
                ],
            )?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#book::OnEnter")])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Holy--!", "You just came", "out of nowhere!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kidd",
                args![
                    "Sorry. I did tell",
                    "you I'd meet you here",
                    "in Juno. Anyway, let's",
                    "both take a careful look at",
                    "this paper from Moore's desk."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "...Morocc...",
                    "Were summoned...",
                    "Through the tower...?",
                    "This handwriting is so",
                    "bad, it's barely legible.",
                    "Can you read any of this?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kidd",
                args![
                    "Uhhh... Tower...",
                    "Prontera... Impact created...",
                    "Desert? Must... four directions",
                    "in Morocc... Oh! Balance the",
                    "four directions in Morocc",
                    "to prevent a great evil."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kidd",
                args![
                    "Sorry, that's all I can read, but it must be important: if Moore",
                    "spent the time to write that",
                    "part clearly, then he must be",
                    "planning to investigate the",
                    "four directions in Morocc."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kidd",
                args![
                    "I'm not sure what it",
                    "could all mean, but I'm",
                    "gonna bring this part of",
                    "his journal with me. I'll",
                    "meet you back in Morocc."
                ],
            )?;
            ctx.var("mao_request").set(Val::from(8))?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(255)])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#book::OnInit")])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("#maobooktimer::OnStop")])?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "^3355FFYou find a journal filled",
                "with illegible scribbles.",
                "It's impossible for you to",
                "decipher the writing inside.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("mao_request").get()?.number()? > 7 {
            ctx.lines(args![
                "^3355FFYou find a journal that is",
                "missing many of its pages",
                "and filled with illegible",
                "scribbles. It's impossible",
                "for you to decipher what",
                "could be written inside.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "^3355FFYou find what appears to",
                "be somebody's research",
                "journal. It's best not to",
                "touch it for now.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn workbook_mao(ctx: &Ctx) -> Script {
    workbook_mao_body(ctx, Vec::new()).map(|_| ())
}

fn workbook_mao_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var(".mao_book").set(Val::from(0))?;
    return Err(Stop::End);
}

pub fn workbook_mao_oninit(ctx: &Ctx) -> Script {
    workbook_mao_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn kidd_book_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Kidd", args!["......", "........."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn kidd_book(ctx: &Ctx) -> Script {
    kidd_book_body(ctx, Vec::new()).map(|_| ())
}

fn kidd_book_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Kidd#book")])?;
    return Err(Stop::End);
}

pub fn kidd_book_oninit(ctx: &Ctx) -> Script {
    kidd_book_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn kidd_book_onenter_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Kidd#book")])?;
    return Err(Stop::End);
}

pub fn kidd_book_onenter(ctx: &Ctx) -> Script {
    kidd_book_onenter_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MaobooktimerStep {
    Start,
    OnEnter,
    OnStop,
    OnTimer180000,
}

fn maobooktimer_run(ctx: &Ctx, mut step: MaobooktimerStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MaobooktimerStep::Start => {
                step = MaobooktimerStep::OnEnter;
                continue 'machine;
            }
            MaobooktimerStep::OnEnter => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            MaobooktimerStep::OnStop => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Workbook#mao::OnInit")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                step = MaobooktimerStep::OnTimer180000;
                continue 'machine;
            }
            MaobooktimerStep::OnTimer180000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Workbook#mao::OnInit")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn maobooktimer(ctx: &Ctx) -> Script {
    maobooktimer_run(ctx, MaobooktimerStep::Start, Vec::new()).map(|_| ())
}

pub fn maobooktimer_onenter(ctx: &Ctx) -> Script {
    maobooktimer_run(ctx, MaobooktimerStep::OnEnter, Vec::new()).map(|_| ())
}

pub fn maobooktimer_onstop(ctx: &Ctx) -> Script {
    maobooktimer_run(ctx, MaobooktimerStep::OnStop, Vec::new()).map(|_| ())
}

pub fn maobooktimer_ontimer180000(ctx: &Ctx) -> Script {
    maobooktimer_run(ctx, MaobooktimerStep::OnTimer180000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum LinStairsStep {
    Start,
    OnInit,
    OnEnter,
}

fn lin_stairs_run(ctx: &Ctx, mut step: LinStairsStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            LinStairsStep::Start => {
                step = LinStairsStep::OnInit;
                continue 'machine;
            }
            LinStairsStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Lin#stairs")])?;
                return Err(Stop::End);
            }
            LinStairsStep::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Lin#stairs")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn lin_stairs(ctx: &Ctx) -> Script {
    lin_stairs_run(ctx, LinStairsStep::Start, Vec::new()).map(|_| ())
}

pub fn lin_stairs_oninit(ctx: &Ctx) -> Script {
    lin_stairs_run(ctx, LinStairsStep::OnInit, Vec::new()).map(|_| ())
}

pub fn lin_stairs_onenter(ctx: &Ctx) -> Script {
    lin_stairs_run(ctx, LinStairsStep::OnEnter, Vec::new()).map(|_| ())
}

fn linstairs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mao_request").get()? == 109 {
        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Lin",
            args!["Alright, we need", "to talk for a second", "about this assignment."],
        )?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#stairs::OnEnter")])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Waaa-!", "You came out", "of nowhere!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lin",
            args![
                "Sorry for scaring you.",
                "We don't have time, so I'll",
                "get straight to the point.",
                "I don't trust our client.",
                "However, for the time being,",
                "we should do what he says."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lin",
            args![
                "In the meantime, keep",
                "an eye out. If you find",
                "anything that might suggest",
                "that Mr. R isn't telling us the",
                "truth for some reason, let",
                "me know. I'll see you soon."
            ],
        )?;
        ctx.var("mao_request").set(Val::from(110))?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(255)])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#stairs::OnInit")])?;
        return Err(Stop::End);
    } else if ctx.var("mao_request").get()? == 115 {
        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Lin",
            args![
                "Hey, I gotta talk to you",
                "real quick without Mr. R",
                "overhearing... I'm not",
                "taking any chances with him!"
            ],
        )?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#stairs::OnEnter")])?;
        ctx.next()?;
        ctx.lines_as(
            "Lin",
            args![
                "I tried to visit our local",
                "historian to learn more about",
                "Mr. R's research. To do that,",
                "I left Mr. R alone for a little",
                "while under our magic security system. However, I kinda failed."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lin",
            args![
                "I got a little peeved at",
                "the historian guy, and he",
                "got a little intimidated. Now",
                "he's holed up somewhere",
                "in the Morocc Inn, and I don't",
                "think he's seeing anybody."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lin",
            args![
                "I want you to find Morocc's",
                "historian and find out what",
                "you can about Satan Morocc,",
                "Morocc, and Thanatos Tower.",
                "We need to know how they're",
                "linked to the missing kids."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "I guess since he hasn't",
                "met me yet, maybe he'll",
                "talk to me. Alright, I'll do it. "
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lin",
            args![
                "Great, thanks a lot.",
                "Okay then, I'll see",
                "you later. Remember,",
                "the Morocc Inn, alright?"
            ],
        )?;
        ctx.var("mao_request").set(Val::from(116))?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(255)])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#stairs::OnInit")])?;
        return Err(Stop::End);
    } else if ctx.var("mao_request").get()? == 117 {
        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["L-Lin...?", "You there?"],
        )?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#stairs::OnEnter")])?;
        ctx.next()?;
        ctx.lines_as(
            "Lin",
            args![
                "Yeah, I'm here.",
                "So, were you able",
                "to find that historian?",
                "What exactly did you learn?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Well, I was only able to",
                "speak to the historian's",
                "assistant. Let's see...",
                "I found out about the",
                "origin of this city's name..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Thanatos Tower was used",
                "by Morocc Satan to summon",
                "his own monsters into our",
                "world. Now, it's being rebuilt,",
                "even though it houses these",
                "demons that look like angels..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lin",
            args![
                "Huh. That's strange.",
                "I heard that those are",
                "actually real angels that",
                "are guarding the tower for",
                "some reason. Hm. What else",
                "did you manage to learn?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Well, Satan Morocc, if it",
                "exists, might be able to",
                "come back into our world",
                "if the seal is broken by",
                "sacrificing children..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lin",
            args![
                "That... That sounds",
                "really bad, especially",
                "with all of those children",
                "missing from Morocc lately.",
                "What'll be our next move?",
                "I guess we'll talk to Mr. R."
            ],
        )?;
        ctx.var("mao_request").set(Val::from(118))?;
        ctx.next()?;
        ctx.lines_as(
            "Lin",
            args![
                "The way things are going,",
                "he's probably gonna ask us",
                "to investigate Thanatos Tower.",
                "For now, we'll see what he wants. Ah, and not a word of anything",
                "that we've discussed out here."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(255)])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#stairs::OnInit")])?;
        return Err(Stop::End);
    } else if (ctx.var("mao_request").get()? == 121 && ctx.var("thana_quest").get()?.number()? > 1) {
        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Lin",
            args!["Report to me first.", "Did you learn anything", "about Thanatos Tower?"],
        )?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#stairs::OnEnter")])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
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
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "The Rekenber Corporation",
                "is also responsible for the",
                "tower's reconstruction, and",
                "they ultimately plan to repair",
                "all tweleve of its levels."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lin",
            args![
                "Why would they want to do",
                "something crazy like that?",
                "If Mr. R is really preventing",
                "Morocc Satan's return, then",
                "his attackers must want",
                "to revive Morocc Satan..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lin",
            args![
                "But if he's lying, and",
                "he's trying to bring Morocc",
                "Satan into our world, then",
                "he's wanted by the Dandelion",
                "group... meaning, he may be",
                "the target for Kidd's mission."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lin",
            args![
                "Crap! This could be",
                "really bad. I need to",
                "talk about this to our",
                "commanding officer Valdes",
                "about this. While I do that,",
                "you go and check on Mr. R."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lin",
            args![
                "Why did Valdes accept",
                "both of these missions?!",
                "In the worst case scenario,",
                "we might be the ones who'll",
                "have to keep Satan Morocc",
                "from returning to this world..."
            ],
        )?;
        ctx.var("mao_request").set(Val::from(122))?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(255)])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#stairs::OnInit")])?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn linstairs(ctx: &Ctx) -> Script {
    linstairs_body(ctx, Vec::new()).map(|_| ())
}

fn upturned_spot_water_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mao_request").get()? == 14 {
        ctx.call(Function::Cutin, vec![Val::from("mocseal_ice01.bmp"), Val::from(2)])?;
        ctx.lines(args![
            "^3355FFYou find a gleaming",
            "crest that looks like a",
            "splashing wave of water.^000000"
        ])?;
        if ctx.call(Function::CountItem, vec![Val::from(996)])?.is_true() {
            ctx.lines(args![
                "^3355FFYou take out a Rough Wind",
                "to counter the crest's power.^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFThe crest responds to",
                "the Rough Wind, and both",
                "objects violent glow in",
                "conflict against each other.^000000"
            ])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL2")?])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL4")?])?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_ice02.bmp"), Val::from(2)])?;
            ctx.lines(args![
                "^3355FFSuddenly, the crest sprayed",
                "some water which evaporated",
                "quickly in mid-air: it seems",
                "the Wind element neutralized",
                "this crest's Water element.^000000"
            ])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SPHERE")?])?;
            ctx.call(Function::DelItem, vec![Val::from(996), Val::from(1)])?;
            ctx.var("mao_request").set(Val::from(15))?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Alright...",
                    "Now I should look for",
                    "the crest hidden in the",
                    "field north of Morocc."
                ],
            )?;
            ctx.close_window()?;
        } else {
            ctx.lines(args!["^3355FFYou probably need a Rough", "Wind to counter its power.^000000"])?;
            ctx.close_window()?;
        }
    } else if (ctx.var("mao_request").get()?.number()? > 14 && ctx.var("mao_request").get()?.number()? < 100) {
        ctx.lines(args![
            "^3355FFYou find a gleaming",
            "crest that looks like a",
            "splashing wave of water.^000000",
            "^3355FFHowever, you don't sense",
            "anything strange about it.^000000"
        ])?;
        ctx.call(Function::Cutin, vec![Val::from("mocseal_ice02.bmp"), Val::from(2)])?;
        ctx.close_window()?;
    } else if ctx.var("mao_request").get()? == 113 {
        ctx.lines(args![
            "^3355FFYou find a gleaming",
            "crest that looks like a",
            "splashing wave of water.^000000"
        ])?;
        ctx.call(Function::Cutin, vec![Val::from("mocseal_ice02.bmp"), Val::from(2)])?;
        if ctx.call(Function::CountItem, vec![Val::from(995)])?.is_true() {
            ctx.lines(args![
                "^3355FFYou bring out a Mystic Frozen",
                "to enhance the crest's power.^000000"
            ])?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_ice01.bmp"), Val::from(2)])?;
            ctx.lines(args![
                "^3355FFThe crest resonates",
                "with the Mystic Frozen,",
                "suddenly causing the air",
                "to chill and raising the",
                "waves in the oasis.^000000"
            ])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL2")?])?;
            ctx.call(Function::DelItem, vec![Val::from(995), Val::from(1)])?;
            ctx.var("mao_request").set(Val::from(114))?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Alright...",
                    "Now I should look for",
                    "the crest hidden in the",
                    "field north of Morocc."
                ],
            )?;
            ctx.close_window()?;
        } else {
            ctx.lines(args!["^3355FFYou probably need a Mystic", "Frozen to enhance its power."])?;
            ctx.close_window()?;
        }
    } else if ctx.var("mao_request").get()?.number()? > 112 {
        ctx.lines(args![
            "^3355FFYou find a gleaming",
            "crest that looks like a",
            "splashing wave of water.^000000"
        ])?;
        ctx.call(Function::Cutin, vec![Val::from("mocseal_ice01.bmp"), Val::from(2)])?;
        ctx.close_window()?;
    }
    ctx.call(Function::Cutin, vec![Val::from("mocseal_ice01.bmp"), Val::from(255)])?;
    ctx.call(Function::Cutin, vec![Val::from("mocseal_ice02.bmp"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn upturned_spot_water(ctx: &Ctx) -> Script {
    upturned_spot_water_body(ctx, Vec::new()).map(|_| ())
}
