use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn unturned_spot_wind_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mao_request").get()? == 10 {
        ctx.call(Function::Cutin, vec![Val::from("mocseal_wind01.bmp"), Val::from(2)])?;
        ctx.lines(args![
            "^3355FFYou find a shining crest",
            "that looks like a symbol of",
            "the Wind. As you approach it,",
            "you can feel the wind blowing",
            "strongly against your skin.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "This must be what",
                "Raiyan Moore is so",
                "interested in. Well,",
                "I better go inform Kidd",
                "about what I found here."
            ],
        )?;
        ctx.var("mao_request").set(Val::from(11))?;
        ctx.close_window()?;
    } else {
        if ctx.var("mao_request").get()? == 11 {
            ctx.call(Function::Cutin, vec![Val::from("mocseal_wind01.bmp"), Val::from(2)])?;
            ctx.lines(args![
                "^3355FFYou find a shining crest",
                "that looks like a symbol of",
                "the Wind. As you approach it,",
                "you can feel the wind blowing",
                "strongly against your skin.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Great, now that I found",
                    "this, I really need to tell",
                    "Kidd about it. Hopefully, he",
                    "found something just like this."
                ],
            )?;
            ctx.close_window()?;
        } else if ctx.var("mao_request").get()? == 12 {
            if ctx.call(Function::CountItem, vec![Val::from(997)])?.is_true() {
                ctx.call(Function::Cutin, vec![Val::from("mocseal_wind01.bmp"), Val::from(2)])?;
                ctx.lines(args![
                    "^3355FFYou find a shining crest",
                    "that looks like a symbol of",
                    "the Wind. As you approach it,",
                    "you can feel the wind blowing",
                    "strongly against your skin.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Okay, the power of the",
                        "Earth counteracts the ",
                        "Wind. I'll just pull out this",
                        "Great Nature and... Eh?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFThe Wind elemental crest",
                    "quickly responds to the",
                    "Great Nature stone.^000000"
                ])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL4")?])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL5")?])?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_wind02.bmp"), Val::from(2)])?;
                ctx.lines(args![
                    "^3355FFThe Wind and Earth",
                    "neutralized each other,",
                    "causing the power of the",
                    "Wind in this area to stabilize.^000000"
                ])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SPHERE")?])?;
                ctx.call(Function::DelItem, vec![Val::from(997), Val::from(1)])?;
                ctx.var("mao_request").set(Val::from(13))?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Great, now that I'm done",
                        "with this crest, I need to",
                        "find the next one. Let's see,",
                        "I need to find the one to the",
                        "south that Kidd already found."
                    ],
                )?;
                ctx.close_window()?;
            } else {
                ctx.call(Function::Cutin, vec![Val::from("mocseal_wind01.bmp"), Val::from(2)])?;
                ctx.lines(args![
                    "^3355FFYou find a shining crest",
                    "that looks like a symbol of",
                    "the Wind. As you approach it,",
                    "you can feel the wind blowing",
                    "strongly against your skin.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Wind, wind, wind...", "What do I use to counteract", "the Wind property? It was..."],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("Fire"), Val::from("Ice"), Val::from("Wind"), Val::from("Earth")],
                )? {
                    1 => {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Fire...? No, that's...", "Wind and Fire sort of", "go together, don't they?"],
                        )?;
                        ctx.close_window()?;
                    }
                    2 => {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Ice...? No...",
                                "The power of Wind, of",
                                "lightning, supersedes",
                                "the power of Ice and water..."
                            ],
                        )?;
                        ctx.close_window()?;
                    }
                    3 => {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["I got it...!", "I'll fight Wind", "with Wind! No...", "Don't be ridiculous."],
                        )?;
                        ctx.close_window()?;
                    }
                    4 => {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Earth...?", "That's it! I need a", "Great Nature to use", "on this Wind crest!"],
                        )?;
                        ctx.close_window()?;
                    }
                    _ => {}
                }
            }
        } else {
            if (ctx.var("mao_request").get()?.number()? > 12 && ctx.var("mao_request").get()?.number()? < 100) {
                ctx.lines(args![
                    "^3355FFYou find a shining crest",
                    "that looks like a symbol of",
                    "the Wind. However, you don't",
                    "think that it's particularly",
                    "worthy of an investigation.^000000"
                ])?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_wind02.bmp"), Val::from(2)])?;
                ctx.close_window()?;
            } else if ctx.var("mao_request").get()? == 110 {
                ctx.call(Function::Cutin, vec![Val::from("mocseal_wind02.bmp"), Val::from(2)])?;
                ctx.lines(args![
                    "^3355FFYou find a shining crest",
                    "that looks like a symbol of",
                    "the Wind. However, the air",
                    "flow around it seems weak.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "This must be what I'm looking",
                        "for. It looks like an artifact of the Wind element, but the wind",
                        "around here isn't as strong as it should be. I think I might need",
                        "to bring a ^4D4DFFRough Wind^000000 here..."
                    ],
                )?;
                ctx.var("mao_request").set(Val::from(111))?;
                ctx.close_window()?;
            } else if ctx.var("mao_request").get()? == 111 {
                if ctx.call(Function::CountItem, vec![Val::from(996)])?.is_true() {
                    ctx.call(Function::Cutin, vec![Val::from("mocseal_wind02.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Let's see...",
                            "Hopefully this",
                            "Rough Wind will do",
                            "the trick. Whoa. Um...",
                            "Something's happening..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFThe elemental crest",
                        "quickly responds to the",
                        "Rough Wind, causing the",
                        "crest to shine brighter and",
                        "the wind to blow stronger.^000000"
                    ])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL4")?])?;
                    ctx.call(Function::DelItem, vec![Val::from(996), Val::from(1)])?;
                    ctx.var("mao_request").set(Val::from(112))?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("mocseal_wind01.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Well, I guess that's that.",
                            "Now I need to talk to Lin",
                            "and figure out if there are",
                            "more of these things to be",
                            "found outside of Morocc."
                        ],
                    )?;
                    ctx.close_window()?;
                } else {
                    ctx.lines(args![
                        "^3355FFYou need to bring",
                        "a Rough Wind to activate",
                        "this Wind elemental crest.^000000"
                    ])?;
                    ctx.call(Function::Cutin, vec![Val::from("mocseal_wind02.bmp"), Val::from(2)])?;
                    ctx.close_window()?;
                }
            } else if ctx.var("mao_request").get()?.number()? > 111 {
                ctx.lines(args![
                    "^3355FFYou find a shining crest",
                    "that looks like a symbol of",
                    "the Wind. As you approach it,",
                    "you can feel the wind blowing",
                    "strongly against your skin.^000000"
                ])?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_wind01.bmp"), Val::from(2)])?;
                ctx.close_window()?;
            }
        }
    }
    ctx.call(Function::Cutin, vec![Val::from("mocseal_wind01.bmp"), Val::from(255)])?;
    ctx.call(Function::Cutin, vec![Val::from("mocseal_wind02.bmp"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn unturned_spot_wind(ctx: &Ctx) -> Script {
    unturned_spot_wind_body(ctx, Vec::new()).map(|_| ())
}

fn unturned_spot_earth_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mao_request").get()? == 13 {
        if ctx.call(Function::CountItem, vec![Val::from(994)])?.is_true() {
            ctx.lines(args![
                "^3355FFYou find a shimmering",
                "crest that symbolizes",
                "the power of the Earth.^000000"
            ])?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_earth01.bmp"), Val::from(2)])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Let's see...", "I should use this", "Flame Heart to", "stabilize this crest."],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFAs soon as you pull out",
                "the Flame Heart, the stone",
                "and the crest begin to glow",
                "intensely, as if their powers",
                "were conflicting.^000000"
            ])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL5")?])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL3")?])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFThe Flame Heart then",
                "quickly vanishes with",
                "a burst of heated vapor.",
                "This power of this crest",
                "is now stabilized.^000000"
            ])?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_earth02.bmp"), Val::from(2)])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SPHERE")?])?;
            ctx.call(Function::DelItem, vec![Val::from(994), Val::from(1)])?;
            ctx.var("mao_request").set(Val::from(14))?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "That was simple.",
                    "Alright, now I should",
                    "try to find the crest",
                    "hidden east of Morocc."
                ],
            )?;
            ctx.close_window()?;
        } else {
            ctx.lines(args![
                "^3355FFYou find a shimmering",
                "crest that symbolizes",
                "the power of the Earth.^000000"
            ])?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_earth01.bmp"), Val::from(2)])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFTo counteract the",
                "crest's Earth power,",
                "you'll probably need",
                "a Flame Heart stone.^000000"
            ])?;
            ctx.close_window()?;
        }
    } else {
        if (ctx.var("mao_request").get()?.number()? > 13 && ctx.var("mao_request").get()?.number()? < 100) {
            ctx.lines(args![
                "^3355FFYou find a shimmering",
                "crest that symbolizes",
                "the power of the Earth.",
                "However, you don't sense",
                "anything peculiar from it."
            ])?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_earth02.bmp"), Val::from(2)])?;
            ctx.close_window()?;
        } else if ctx.var("mao_request").get()? == 112 {
            ctx.lines(args![
                "^3355FFYou find a shimmering",
                "crest that symbolizes",
                "the power of the Earth.",
                "You'll need a Great Nature",
                "stone to enhance its power.^000000"
            ])?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_earth02.bmp"), Val::from(2)])?;
            ctx.next()?;
            if ctx.call(Function::CountItem, vec![Val::from(997)])?.is_true() {
                ctx.lines(args![
                    "^3355FFYou pull out a",
                    "Great Nature, which",
                    "causes tremors in the",
                    "ground and sand to flow",
                    "towards the crest.^000000"
                ])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL5")?])?;
                ctx.call(Function::DelItem, vec![Val::from(997), Val::from(1)])?;
                ctx.var("mao_request").set(Val::from(113))?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Great, I think that actually",
                        "worked! Now, I should try",
                        "to find the crest hidden to",
                        "the east of Morocc."
                    ],
                )?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_earth01.bmp"), Val::from(2)])?;
                ctx.close_window()?;
            } else {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["The next time that", "I come here, I better", "have a Great Nature ready..."],
                )?;
                ctx.close_window()?;
            }
        } else if ctx.var("mao_request").get()?.number()? > 111 {
            ctx.lines(args![
                "^3355FFYou find a shimmering",
                "crest that symbolizes",
                "the power of the Earth.^000000"
            ])?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_earth01.bmp"), Val::from(2)])?;
            ctx.close_window()?;
        }
    }
    ctx.call(Function::Cutin, vec![Val::from("mocseal_earth01.bmp"), Val::from(255)])?;
    ctx.call(Function::Cutin, vec![Val::from("mocseal_earth02.bmp"), Val::from(255)])?;
    Ok(Val::from(0))
}

pub fn unturned_spot_earth(ctx: &Ctx) -> Script {
    unturned_spot_earth_body(ctx, Vec::new()).map(|_| ())
}

fn unturned_spot_fire_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mao_request").get()? == 15 {
        if ctx.call(Function::CountItem, vec![Val::from(995)])?.is_true() {
            ctx.lines(args!["^3355FFYou find a gleaming", "crest that symbolizes Fire.^000000"])?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_fire01.bmp"), Val::from(2)])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "This crest must be",
                    "imbued with the Fire",
                    "element. I better see",
                    "if this Mystic Frozen",
                    "can stabilize it."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFAs soon as you take",
                "out the Mystic Frozen,",
                "it begins to pulse with",
                "light as the crest glows",
                "brighter and brighter.^000000"
            ])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL3")?])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL2")?])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFThe air in the area",
                "suddenly chills, and",
                "your Mystic Frozen",
                "bursts into cold vapor.^000000"
            ])?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_fire02.bmp"), Val::from(2)])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SPHERE")?])?;
            ctx.call(Function::DelItem, vec![Val::from(995), Val::from(1)])?;
            ctx.var("mao_request").set(Val::from(16))?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "It looks like my work",
                    "here is done. Now, I better",
                    "go back and report to Kidd."
                ],
            )?;
            ctx.close_window()?;
        } else {
            ctx.call(Function::Cutin, vec![Val::from("mocseal_fire01.bmp"), Val::from(2)])?;
            ctx.lines(args![
                "^3355FFYou find a gleaming",
                "crest that symbolizes Fire.",
                "You'll need a Mystic Frozen",
                "to stabilize its power.^000000"
            ])?;
            ctx.close_window()?;
        }
    } else {
        if (ctx.var("mao_request").get()?.number()? > 15 && ctx.var("mao_request").get()?.number()? < 100) {
            ctx.lines(args![
                "^3355FFYou find a gleaming",
                "crest that symbolizes Fire.",
                "However, you don't think it's",
                "worth investigating for now.^000000"
            ])?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_fire02.bmp"), Val::from(2)])?;
            ctx.close_window()?;
        } else if ctx.var("mao_request").get()? == 114 {
            ctx.call(Function::Cutin, vec![Val::from("mocseal_fire02.bmp"), Val::from(2)])?;
            ctx.lines(args!["^3355FFYou find a gleaming", "crest that symbolizes Fire."])?;
            if ctx.call(Function::CountItem, vec![Val::from(994)])?.is_true() {
                ctx.lines(args!["You'll need a Flame Heart", "in order to enhance its power.^000000"])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFYou pull out a Flame",
                    "Heart, and the crest",
                    "begins to shine as the",
                    "air around you heats up.^000000"
                ])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL3")?])?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_fire01.bmp"), Val::from(2)])?;
                ctx.call(Function::DelItem, vec![Val::from(994), Val::from(1)])?;
                ctx.var("mao_request").set(Val::from(115))?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Well, that's the",
                        "last elemental crest.",
                        "Now I better go back",
                        "and report to Lin."
                    ],
                )?;
                ctx.close_window()?;
            } else {
                ctx.lines(args!["You'll need a Flame Heart", "in order to enhance its power.^000000"])?;
                ctx.close_window()?;
            }
        } else if ctx.var("mao_request").get()?.number()? > 113 {
            ctx.lines(args!["^3355FFYou find a gleaming", "crest that symbolizes Fire.^000000"])?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_fire01.bmp"), Val::from(2)])?;
            ctx.close_window()?;
        }
    }
    ctx.call(Function::Cutin, vec![Val::from("mocseal_fire01.bmp"), Val::from(255)])?;
    ctx.call(Function::Cutin, vec![Val::from("mocseal_fire02.bmp"), Val::from(255)])?;
    Ok(Val::from(0))
}

pub fn unturned_spot_fire(ctx: &Ctx) -> Script {
    unturned_spot_fire_body(ctx, Vec::new()).map(|_| ())
}

fn assistant_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("mao_request").get()?.number()? > 18 && ctx.var("mao_request").get()?.number()? < 100) {
        ctx.lines_as(
            "Sephit",
            args![
                "Hopefully, I was able to",
                "help you with whatever",
                "information that you needed.",
                "Morocc has a much richer",
                "history than most people",
                "realize, don't you think?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mao_request").get()?.number()? > 116 {
        ctx.lines_as(
            "Sephit",
            args![
                "Hopefully, I was able to",
                "help you with whatever",
                "information that you needed.",
                "Morocc has a much richer",
                "history than most people",
                "realize, don't you think?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "When I get some time,",
                "I really want to investigate",
                "that Thanatos Tower. I get",
                "the feeling that there's so",
                "much I can learn there~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mao_request").get()? == 18 {
        ctx.lines_as(
            "Sephit",
            args![
                "Oh, we usually don't",
                "have many visitors here.",
                "Are you here to speak to",
                "our local historian? He's",
                "pretty busy right now, so",
                "I hope you can come back later."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "Though, to be honest,",
                "he's kind of hiding under",
                "the covers at the moment.",
                "Some Assassin came to",
                "request some information,",
                "but he scared him off..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "It was pretty funny, actually.",
                "The guy walked in, flashed",
                "his dagger, and declared that",
                "he needed some important",
                "information. I guess my boss",
                "was pretty intimidated by him."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "If it's really important,",
                "then I might be able to",
                "answer your questions if",
                "they're about Morocc's most",
                "ancient histories and legends."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou ask Sephit about the",
            "four elemental crests around",
            "Morocc, their significance, and",
            "about Raiyan Moore. You also",
            "inform her that you've already",
            "stabilized the crests' power.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "Moore... Moore...",
                "His work sounds really",
                "important, so I'm surprised",
                "I haven't heard of him. I'd ask",
                "my boss, but I can't disturb",
                "him right now. Ah, well..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "Oh, first of all, not too",
                "many people know about",
                "those crests. Still, you did",
                "a great service by stabilizing",
                "them. Otherwise, the seal under",
                "Morocc Castle would break."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "The seal beneath Morocc",
                "Castle actually keeps Satan",
                "Morocc from entering our world.",
                "If he ever returned, he might",
                "repeat the mass destruction"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "When Satan Morocc",
                "was terrorizing the human",
                "world, he used Thanatos",
                "Tower as his power base.",
                "There, he would summon",
                "countless hordes of minions."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "That tower has been in ruins",
                "for years, but recently some",
                "company started reconstructing",
                "it, even though demons, well,",
                "disguised as angels, still",
                "roam that place freely."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "Satan Morocc may have been",
                "unimaginably powerful, but",
                "it would take a lot of work",
                "to bring him back into our",
                "world. Let's see, you could",
                "destroy Morocc's seal..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "There was also... Oh, God.",
                "Long ago, someone actually",
                "sacrificed children in a failed",
                "attempt to revive Morocc Satan.",
                "The children missing here in",
                "Morocc-- Y-you don't think..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "But who really knows?",
                "I mean we have historical",
                "records of Satan Morocc, ",
                "but maybe it's just a legend.",
                "Aside from that, we have no",
                "proof that he really exists."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "...Well, aside from those",
                "elemental crests, I mean.",
                "Then again, maybe they just",
                "regulate this region's elements",
                "to make it possible for people",
                "to live here in the desert."
            ],
        )?;
        ctx.var("mao_request").set(Val::from(19))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mao_request").get()? == 116 {
        ctx.lines_as(
            "Sephit",
            args![
                "Oh, we usually don't",
                "have many visitors here.",
                "Are you here to speak to",
                "our local historian? He's",
                "pretty busy right now, so",
                "I hope you can come back later."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "Though, to be honest,",
                "he's kind of hiding under",
                "the covers at the moment.",
                "Some Assassin came to",
                "request some information,",
                "but she scared him off..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "If it's really important,",
                "then I might be able to",
                "answer your questions if",
                "they're about Morocc's most",
                "ancient histories and legends."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou ask Sephit for any",
            "information related to",
            "Satan Morocc and Thanatos",
            "Tower, particularly their",
            "significance and how",
            "they might be related.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou also inform her",
            "about Moore's research,",
            "and about the elemental",
            "crests hidden throughout",
            "Morocc that you've balanced.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "Moore... Moore...",
                "His work sounds really",
                "important, so I'm surprised",
                "I haven't heard of him. I'd ask",
                "my boss, but I can't disturb",
                "him right now. Ah, well..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "Oh, first of all, not too",
                "many people know about",
                "those crests. Still, you did",
                "a great service by stabilizing",
                "them. Otherwise, the seal under",
                "Morocc Castle would break."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "The seal beneath Morocc",
                "Castle actually keeps Satan",
                "Morocc from entering our world.",
                "If he ever returned, he might",
                "repeat the mass destruction",
                "that he caused in the past."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "It's funny that you should ask",
                "about Thanatos Tower. When",
                "Satan Morocc was terrorizing",
                "our world, he used that place",
                "to summon hordes of minions",
                "that would menace us humans."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "That tower has been in ruins",
                "for years, but recently some",
                "company started reconstructing",
                "it, even though demons, well,",
                "disguised as angels, still",
                "roam that place freely."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "Satan Morocc may have been",
                "unimaginably powerful, but",
                "it would take a lot of work",
                "to bring him back into our",
                "world. Let's see, you could",
                "destroy Morocc's seal..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "There was also... Oh, God.",
                "Long ago, someone actually",
                "sacrificed children in a failed",
                "attempt to revive Morocc Satan.",
                "The children missing here in",
                "Morocc-- Y-you don't think..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "But who really knows?",
                "I mean we have historical",
                "records of Satan Morocc, ",
                "but maybe it's just a legend.",
                "Aside from that, we have no",
                "proof that he really exists."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "...Well, aside from those",
                "elemental crests, I mean.",
                "Then again, maybe they just",
                "regulate this region's elements",
                "to make it possible for people",
                "to live here in the desert."
            ],
        )?;
        ctx.var("mao_request").set(Val::from(117))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Sephit",
            args![
                "Oh, we usually don't",
                "have many visitors here.",
                "Are you here to speak to",
                "our local historian? He's",
                "pretty busy right now, so",
                "I hope you can come back later."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "Lately, I've been",
                "digging through some",
                "old historical records and",
                "learned something about",
                "a monster called Satan Morocc."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sephit",
            args![
                "According to the legends,",
                "he's sealed beneath Morocc",
                "Castle, and our town gets its",
                "name from him. That sounds",
                "pretty grotesque, don't you",
                "think? Too weird to be true..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn assistant(ctx: &Ctx) -> Script {
    assistant_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MaoTableStep {
    Start,
    OnTouch,
    OnInit,
    OnEnter,
}

fn mao_table_run(ctx: &Ctx, mut step: MaoTableStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MaoTableStep::Start => {
                step = MaoTableStep::OnTouch;
                continue 'machine;
            }
            MaoTableStep::OnTouch => {
                if ctx.var("mao_request").get()? == 24 {
                    ctx.lines_as(
                        "Valdes",
                        args![
                            "That's grave news...",
                            "I had my suspicions",
                            "about Mr. R, but I never",
                            "thought he was capable",
                            "of such audacity..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kidd",
                        args![
                            "Damn it! I know you had",
                            "to accept both Mr. R and",
                            "the Dandelion Organization",
                            "because we didn't know the",
                            "truth beforehand, but does",
                            "Lin know about this now?!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Valdes",
                        args![
                            "I doubt it. I only learned",
                            "that Mr. R was the true",
                            "kidnapper when you told",
                            "me just now. But to sacrifice",
                            "them in order to revive Satan",
                            "Morocc... That's beyond evil."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Valdes",
                        args![
                            "Mr. R... If he is Raiyan",
                            "Moore, then you can find him",
                            "in the other room. Go and",
                            "interrogate him, confirm",
                            "everything you have learned."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Valdes", args!["Come...", "This way."])?;
                    ctx.next()?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Valdes#2::OnInit")])?;
                    ctx.lines_as(
                        "Kidd",
                        args![
                            "This is getting",
                            "too serious. Valdes,",
                            "wait, I'm coming with you!",
                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", come on, quickly!"))
                        ],
                    )?;
                    ctx.var("mao_request").set(Val::from(25))?;
                    ctx.close_window()?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#2::OnInit")])?;
                    return Err(Stop::End);
                } else if ctx.var("mao_request").get()? == 123 {
                    ctx.lines_as(
                        "Lin",
                        args![
                            "Master, did you know",
                            "from the beginning?!",
                            "You knew that the person",
                            "I was supposed to protect",
                            "is the same person that",
                            "Kidd is supposed to find?!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Valdes",
                        args![
                            "Lin, calm down. Yes,",
                            "I suspected as such from",
                            "the start. However, I did",
                            "know whether to trust Mr. R",
                            "or the Dandelion organization."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Valdes",
                        args![
                            "Since both parties claimed",
                            "to be able to help the missing",
                            "children, I took the chance.",
                            "For now, the best thing to",
                            "do would be to ask if Mr. R's",
                            "attackers are from Dandelion..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lin",
                        args![
                            "Alright... I can't believe",
                            "it... Mr. R. Moore... Kidd",
                            "is supposed to hunt down",
                            "Raiyan Moore... It's too",
                            "much of a coincidence..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Lin! I'm sorry to",
                            "interrupt, but Mr. R",
                            "is missing... I don't",
                            "know where he is!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lin",
                        args![
                            "What?! What do yo--",
                            "Wh-what's going on?!",
                            "What's this noise in",
                            "my f-freakin' head?!"
                        ],
                    )?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("que_job01"),
                            Val::from("...Blood... is the currency... of the soul..."),
                            Val::from(1),
                            Val::from(8087790),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Lin", args!["Oh no...", "This is what", "I feared the most..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["What's...", "What's going on?"],
                    )?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("que_job01"),
                            Val::from("...We... need... blood... of... innocence..."),
                            Val::from(1),
                            Val::from(8087790),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lin",
                        args![
                            "I think...",
                            "I think it's the",
                            "ceremony to revive",
                            "Satan Morocc! Mr. R must",
                            "have went there to stop",
                            "them... or to join them!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Valdes",
                        args![
                            "Lin, go call everyone",
                            "in the guild! And you,",
                            "try to find the source",
                            "of that weird echo!",
                            "Hurry, there's no time!"
                        ],
                    )?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("que_job01"),
                            Val::from("...Grant... us... immortality... Satan Morocc..."),
                            Val::from(1),
                            Val::from(8087790),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Lin", args!["Yes, sir!"])?;
                    ctx.next()?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#2::OnInit")])?;
                    ctx.lines_as(
                        "Valdes",
                        args![
                            "No matter what the ",
                            "cost, we can't let",
                            "that ritual finish...!",
                            "If Satan Morocc really",
                            "exists, we can't let",
                            "him enter our world!"
                        ],
                    )?;
                    ctx.var("mao_request").set(Val::from(124))?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(11), Val::from(4)])?;
                    return Err(Stop::End);
                }
                step = MaoTableStep::OnInit;
                continue 'machine;
            }
            MaoTableStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#mao_table")])?;
                return Err(Stop::End);
            }
            MaoTableStep::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#mao_table")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mao_table(ctx: &Ctx) -> Script {
    mao_table_run(ctx, MaoTableStep::Start, Vec::new()).map(|_| ())
}

pub fn mao_table_ontouch(ctx: &Ctx) -> Script {
    mao_table_run(ctx, MaoTableStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn mao_table_oninit(ctx: &Ctx) -> Script {
    mao_table_run(ctx, MaoTableStep::OnInit, Vec::new()).map(|_| ())
}

pub fn mao_table_onenter(ctx: &Ctx) -> Script {
    mao_table_run(ctx, MaoTableStep::OnEnter, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MaoEmptyStep {
    Start,
    OnTouch,
    OnInit,
    OnEnter,
}

fn mao_empty_run(ctx: &Ctx, mut step: MaoEmptyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MaoEmptyStep::Start => {
                step = MaoEmptyStep::OnTouch;
                continue 'machine;
            }
            MaoEmptyStep::OnTouch => {
                if ctx.var("mao_request").get()? == 122 {
                    ctx.lines(args!["^333333...........No........", "This.........can't...^000000"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Huh?! That sounds",
                            "like Lin's voice...",
                            "Where are they?",
                            "Where have they gone?!"
                        ],
                    )?;
                    ctx.var("mao_request").set(Val::from(123))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = MaoEmptyStep::OnInit;
                continue 'machine;
            }
            MaoEmptyStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#mao_empty")])?;
                return Err(Stop::End);
            }
            MaoEmptyStep::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#mao_empty")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mao_empty(ctx: &Ctx) -> Script {
    mao_empty_run(ctx, MaoEmptyStep::Start, Vec::new()).map(|_| ())
}

pub fn mao_empty_ontouch(ctx: &Ctx) -> Script {
    mao_empty_run(ctx, MaoEmptyStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn mao_empty_oninit(ctx: &Ctx) -> Script {
    mao_empty_run(ctx, MaoEmptyStep::OnInit, Vec::new()).map(|_| ())
}

pub fn mao_empty_onenter(ctx: &Ctx) -> Script {
    mao_empty_run(ctx, MaoEmptyStep::OnEnter, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RabsentStep {
    Start,
    OnTouch,
    OnInit,
    OnEnter,
}

fn rabsent_run(ctx: &Ctx, mut step: RabsentStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RabsentStep::Start => {
                step = RabsentStep::OnTouch;
                continue 'machine;
            }
            RabsentStep::OnTouch => {
                if ctx.var("mao_request").get()? == 25 {
                    ctx.lines_as("Kidd", args!["Raiyan Moore...!", "Wh-where is he?!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Valdes",
                        args![
                            "What the deuce?",
                            "He should be just",
                            "in this room. Wait,",
                            "how could he get past",
                            "our security magic...?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Kidd", args!["Where's Lin?", "Lin?! Lin, where", "are you? Answer me!"])?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("que_job01"),
                            Val::from("...Blood... is the currency... of the soul..."),
                            Val::from(1),
                            Val::from(8087790),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kidd",
                        args![
                            "God! Wh-what's",
                            "that--where is that",
                            "voice coming from?!",
                            "My head feels like",
                            "it's gonna split open!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["What...", "What's happening?"],
                    )?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("que_job01"),
                            Val::from("...We... need... holier... blood... the ritual..."),
                            Val::from(1),
                            Val::from(8087790),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kidd",
                        args![
                            "Damn, that has to be him!",
                            "He's trying to revive Satan",
                            "Morocc right now! And where's",
                            "Lin?! What happened to her?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Valdes",
                        args![
                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", I'm going")),
                            "to call every member of the",
                            "Assassin Guild. You and Kidd",
                            "go try to find the source of",
                            "that voice and stop this!"
                        ],
                    )?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("que_job01"),
                            Val::from("...By Satan Morocc's blessings... Grant me... Immortality!"),
                            Val::from(1),
                            Val::from(8087790),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Kidd", args!["Yes, sir!"])?;
                    ctx.next()?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#3::OnInit")])?;
                    ctx.lines_as(
                        "Valdes",
                        args![
                            "I only hope that",
                            "we're not too late",
                            "to stop this from",
                            "happening. Hurry,",
                            "the fate of the world",
                            "hangs in the balance!"
                        ],
                    )?;
                    ctx.var("mao_request").set(Val::from(26))?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(144), Val::from(61)])?;
                    return Err(Stop::End);
                }
                step = RabsentStep::OnInit;
                continue 'machine;
            }
            RabsentStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#Rabsent")])?;
                return Err(Stop::End);
            }
            RabsentStep::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#Rabsent")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn rabsent(ctx: &Ctx) -> Script {
    rabsent_run(ctx, RabsentStep::Start, Vec::new()).map(|_| ())
}

pub fn rabsent_ontouch(ctx: &Ctx) -> Script {
    rabsent_run(ctx, RabsentStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn rabsent_oninit(ctx: &Ctx) -> Script {
    rabsent_run(ctx, RabsentStep::OnInit, Vec::new()).map(|_| ())
}

pub fn rabsent_onenter(ctx: &Ctx) -> Script {
    rabsent_run(ctx, RabsentStep::OnEnter, Vec::new()).map(|_| ())
}

fn man_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("mao_request").get()?.number()? > 27 && ctx.var("mao_request").get()?.number()? < 31)
        || (ctx.var("mao_request").get()?.number()? > 125 && ctx.var("mao_request").get()?.number()? < 129))
    {
        ctx.lines(args!["^3355FFYou find the body", "of a dead man.^000000"])?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_SCRATCH")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFYou find a man lying",
        "on the floor. He seems",
        "very close to dying, and",
        "is mumbling deliriously.^000000"
    ])?;
    if (ctx.var("mao_request").get()? == 26 || ctx.var("mao_request").get()? == 27) {
        ctx.next()?;
        ctx.lines_as(
            "Man",
            args![
                "Ghhhk~! Fr-fresh...",
                "B-blood! Hee hee hee~",
                "For th-the ritual, I-I'll",
                "d-dedicate... Myself...",
                "For the s-sacrifice!",
                "^333333*Cough Cough!*^000000"
            ],
        )?;
        if !(ctx.var("$mao_gate1").get()?.is_true()) {
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("que_job02"), Val::from(14), Val::from(182)])?;
            ctx.var("$mao_gate1").set(Val::from(1))?;
            return Err(Stop::End);
        }
    } else if (ctx.var("mao_request").get()? == 124 || ctx.var("mao_request").get()? == 125) {
        ctx.next()?;
        ctx.lines_as(
            "Man",
            args![
                "Ghhhk~! Fr-fresh...",
                "B-blood! Hee hee hee~",
                "For th-the ritual, I-I'll",
                "d-dedicate... Myself...",
                "For the s-sacrifice!",
                "^333333*Cough Cough!*^000000"
            ],
        )?;
        if !(ctx.var("$mao_gate2").get()?.is_true()) {
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("que_job03"), Val::from(14), Val::from(182)])?;
            ctx.var("$mao_gate2").set(Val::from(1))?;
            return Err(Stop::End);
        }
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn man(ctx: &Ctx) -> Script {
    man_body(ctx, Vec::new()).map(|_| ())
}

fn man_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$mao_gate1").set(Val::from(0))?;
    ctx.var("$mao_gate2").set(Val::from(0))?;
    return Err(Stop::End);
}

pub fn man_oninit(ctx: &Ctx) -> Script {
    man_oninit_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate1Step {
    Start,
    OnEnter,
    OnStop,
    OnTouch,
    OnTimer580000,
    OnTimer590000,
    OnTimer595000,
    OnTimer596000,
    OnTimer597000,
}

fn maogate1_run(ctx: &Ctx, mut step: Maogate1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate1Step::Start => {
                step = Maogate1Step::OnEnter;
                continue 'machine;
            }
            Maogate1Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate1")])?;
                return Err(Stop::End);
            }
            Maogate1Step::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1")])?;
                return Err(Stop::End);
            }
            Maogate1Step::OnTouch => {
                if (ctx.var("mao_request").get()? == 26 || ctx.var("mao_request").get()? == 27) {
                    ctx.call(Function::InitNpcTimer, vec![])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("#maogate1")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_talk1::OnEnter")])?;
                } else {
                    ctx.lines(args!["^3355FFYou will now be", "teleported outside.^000000"])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("morocc"), Val::from(100), Val::from(100)])?;
                    ctx.var("$mao_gate1").set(Val::from(0))?;
                }
                return Err(Stop::End);
            }
            Maogate1Step::OnTimer580000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job02"),
                        Val::from("...Need... more blood... before... gate closes..."),
                        Val::from(1),
                        Val::from(14524637),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate1Step::OnTimer590000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job02"),
                        Val::from("...Grrrr... can't... fail this time... must revive..."),
                        Val::from(1),
                        Val::from(14524637),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maogate1Step::OnTimer595000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("que_job02"), Val::from("morocc"), Val::from(160), Val::from(129)],
                )?;
                return Err(Stop::End);
            }
            Maogate1Step::OnTimer596000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_talk1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_talk2::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_talk3::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_talk4::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_talk5::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_talk6::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_talk7::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#maogate1_1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#maogate1_2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#maogate1_1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#maogate1_2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("R#maogate1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_end::OnStop")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_end::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_end2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_3::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate1_4::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_setting::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#1_bt::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_battle::OnStop2")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate1_battle::OnEnter")])?;
                return Err(Stop::End);
            }
            Maogate1Step::OnTimer597000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate1")])?;
                ctx.var("$mao_gate1").set(Val::from(0))?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate1(ctx: &Ctx) -> Script {
    maogate1_run(ctx, Maogate1Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate1_onenter(ctx: &Ctx) -> Script {
    maogate1_run(ctx, Maogate1Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate1_onstop(ctx: &Ctx) -> Script {
    maogate1_run(ctx, Maogate1Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate1_ontouch(ctx: &Ctx) -> Script {
    maogate1_run(ctx, Maogate1Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn maogate1_ontimer580000(ctx: &Ctx) -> Script {
    maogate1_run(ctx, Maogate1Step::OnTimer580000, Vec::new()).map(|_| ())
}

pub fn maogate1_ontimer590000(ctx: &Ctx) -> Script {
    maogate1_run(ctx, Maogate1Step::OnTimer590000, Vec::new()).map(|_| ())
}

pub fn maogate1_ontimer595000(ctx: &Ctx) -> Script {
    maogate1_run(ctx, Maogate1Step::OnTimer595000, Vec::new()).map(|_| ())
}

pub fn maogate1_ontimer596000(ctx: &Ctx) -> Script {
    maogate1_run(ctx, Maogate1Step::OnTimer596000, Vec::new()).map(|_| ())
}

pub fn maogate1_ontimer597000(ctx: &Ctx) -> Script {
    maogate1_run(ctx, Maogate1Step::OnTimer597000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate1Talk1Step {
    Start,
    OnInit,
    OnEnter,
    OnTouch,
}

fn maogate1_talk1_run(ctx: &Ctx, mut step: Maogate1Talk1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate1Talk1Step::Start => {
                step = Maogate1Talk1Step::OnInit;
                continue 'machine;
            }
            Maogate1Talk1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_talk1")])?;
                return Err(Stop::End);
            }
            Maogate1Talk1Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate1_talk1")])?;
                return Err(Stop::End);
            }
            Maogate1Talk1Step::OnTouch => {
                ctx.lines(args![
                    "^333333Holy... ritual... of...",
                    "blood... I am proud...",
                    "to b-be... the last...",
                    "sacrifice... Heh heh heh...^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFThe maniacal laughter",
                    "from that strange voice",
                    "echoes in your head as you",
                    "wake up in some strange place.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Where am I...?",
                        "What is this place?",
                        "Arrgh, my head hurts,",
                        "but I've got to stop",
                        "Satan Morocc's revival..."
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job02"),
                        Val::from("...Blood... is the currency... of the soul..."),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "There's that voice",
                        "again! Kidd? Valdes?",
                        "Are any of you here?",
                        "Damn, I must be all alone.",
                        "I guess I have to stop this",
                        "weird ritual all by myself!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_talk1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate1_talk1(ctx: &Ctx) -> Script {
    maogate1_talk1_run(ctx, Maogate1Talk1Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate1_talk1_oninit(ctx: &Ctx) -> Script {
    maogate1_talk1_run(ctx, Maogate1Talk1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn maogate1_talk1_onenter(ctx: &Ctx) -> Script {
    maogate1_talk1_run(ctx, Maogate1Talk1Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate1_talk1_ontouch(ctx: &Ctx) -> Script {
    maogate1_talk1_run(ctx, Maogate1Talk1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate1Talk2Step {
    Start,
    OnStop,
    OnEnter,
    OnTouch,
}

fn maogate1_talk2_run(ctx: &Ctx, mut step: Maogate1Talk2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate1Talk2Step::Start => {
                step = Maogate1Talk2Step::OnStop;
                continue 'machine;
            }
            Maogate1Talk2Step::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_talk2")])?;
                return Err(Stop::End);
            }
            Maogate1Talk2Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate1_talk2")])?;
                return Err(Stop::End);
            }
            Maogate1Talk2Step::OnTouch => {
                ctx.lines(args!["...", "......", "........."])?;
                ctx.next()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(
                    Function::StartStatus,
                    vec![ctx.constant("SC_BLIND")?, Val::from(60000), Val::from(0)],
                )?;
                ctx.lines_as("?????", args!["...Isn't the moon", "so beautiful tonight?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "?????",
                    args!["...But I wonder...", "Why are there twice as many", "guards when the moon is full?"],
                )?;
                ctx.next()?;
                ctx.lines_as("???????", args!["The moon is so red...", "Yes, it's been a year..."])?;
                ctx.next()?;
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["What's going on?", "Who's there? Kidd?", "Valdes? Anybody?", "What was that?"],
                )?;
                ctx.close_window()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(Function::EndStatus, vec![ctx.constant("SC_ALL")?])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_talk2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate1_talk2(ctx: &Ctx) -> Script {
    maogate1_talk2_run(ctx, Maogate1Talk2Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate1_talk2_onstop(ctx: &Ctx) -> Script {
    maogate1_talk2_run(ctx, Maogate1Talk2Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate1_talk2_onenter(ctx: &Ctx) -> Script {
    maogate1_talk2_run(ctx, Maogate1Talk2Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate1_talk2_ontouch(ctx: &Ctx) -> Script {
    maogate1_talk2_run(ctx, Maogate1Talk2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate1Talk3Step {
    Start,
    OnStop,
    OnEnter,
    OnTouch,
}

fn maogate1_talk3_run(ctx: &Ctx, mut step: Maogate1Talk3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate1Talk3Step::Start => {
                step = Maogate1Talk3Step::OnStop;
                continue 'machine;
            }
            Maogate1Talk3Step::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_talk3")])?;
                return Err(Stop::End);
            }
            Maogate1Talk3Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate1_talk3")])?;
                return Err(Stop::End);
            }
            Maogate1Talk3Step::OnTouch => {
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(
                    Function::StartStatus,
                    vec![ctx.constant("SC_BLIND")?, Val::from(60000), Val::from(0)],
                )?;
                ctx.lines_as(
                    "?????",
                    args![
                        "So, you telling me you",
                        "don't know how this town",
                        "got its name? Well, it's",
                        "kind of a scary tale..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "?????",
                    args![
                        "An evil demon invaded our",
                        "world, summoning troops of",
                        "his minions from the darkness.",
                        "However, he was defeated and",
                        "sealed beneath this castle",
                        "by a legendary hero."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "???????",
                    args![
                        "But the hatred of Satan",
                        "Morocc has not ebbed with",
                        "time. If you look at the full",
                        "moon from here, it is colored",
                        "red with Satan Morocc's rage..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["...I keep hearing", "things! Where the hell", "is it all coming from?!"],
                )?;
                ctx.next()?;
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.close_window()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(Function::EndStatus, vec![ctx.constant("SC_ALL")?])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_talk3")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate1_talk3(ctx: &Ctx) -> Script {
    maogate1_talk3_run(ctx, Maogate1Talk3Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate1_talk3_onstop(ctx: &Ctx) -> Script {
    maogate1_talk3_run(ctx, Maogate1Talk3Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate1_talk3_onenter(ctx: &Ctx) -> Script {
    maogate1_talk3_run(ctx, Maogate1Talk3Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate1_talk3_ontouch(ctx: &Ctx) -> Script {
    maogate1_talk3_run(ctx, Maogate1Talk3Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate1Talk4Step {
    Start,
    OnStop,
    OnEnter,
    OnTouch,
}

fn maogate1_talk4_run(ctx: &Ctx, mut step: Maogate1Talk4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate1Talk4Step::Start => {
                step = Maogate1Talk4Step::OnStop;
                continue 'machine;
            }
            Maogate1Talk4Step::OnStop => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_talk4")])?;
                return Err(Stop::End);
            }
            Maogate1Talk4Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate1_talk4")])?;
                return Err(Stop::End);
            }
            Maogate1Talk4Step::OnTouch => {
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(
                    Function::StartStatus,
                    vec![ctx.constant("SC_BLIND")?, Val::from(60000), Val::from(0)],
                )?;
                ctx.lines_as("?????", args!["Hmpf.", "It has begun."])?;
                ctx.next()?;
                ctx.lines_as(
                    "?????",
                    args!["That smoke...?", "What exactly is going", "on over there every", "full moon?"],
                )?;
                ctx.next()?;
                ctx.lines_as("???????", args!["Well...", "Who's to say?"])?;
                ctx.next()?;
                ctx.lines_as("????", args!["B-boss...!"])?;
                ctx.next()?;
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job02"),
                        Val::from("...Moooore... Give me... more blood..."),
                        Val::from(1),
                        Val::from(14524637),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "These voices...",
                        "They're like past",
                        "memories of something",
                        "that's happened here",
                        "in Castle Morocc..."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::SoundEffect, vec![Val::from("wander_man_move.wav"), Val::from(0)])?;
                ctx.call(Function::EndStatus, vec![ctx.constant("SC_ALL")?])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate1_talk4")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maogate1_talk4(ctx: &Ctx) -> Script {
    maogate1_talk4_run(ctx, Maogate1Talk4Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate1_talk4_onstop(ctx: &Ctx) -> Script {
    maogate1_talk4_run(ctx, Maogate1Talk4Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maogate1_talk4_onenter(ctx: &Ctx) -> Script {
    maogate1_talk4_run(ctx, Maogate1Talk4Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate1_talk4_ontouch(ctx: &Ctx) -> Script {
    maogate1_talk4_run(ctx, Maogate1Talk4Step::OnTouch, Vec::new()).map(|_| ())
}
