use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum PowerfulLookingWomanStep {
    Start,
    OnTouch,
}

fn powerful_looking_woman_run(ctx: &Ctx, mut step: PowerfulLookingWomanStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PowerfulLookingWomanStep::Start => {
                if ctx.var("thai_find").get()? == 14 {
                    ctx.lines_as("Shuda", args!["Have you", "found Annon?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shuda",
                        args!["You haven't", "found him yet?", "Then where the", "is he, then?!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Shuda", args!["Oh well...", "I'm not sure if I can believe everything you said, but since you found the ring for me, that's good enough. Thank you for your help."])?;
                    ctx.next()?;
                    ctx.lines_as("Shuda", args!["If you happen to meet Annon during your travels, tell him to come back right away. You can go ahead and continue your adventures now~"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shuda",
                        args![
                            "If I ever",
                            "see him again...",
                            "He will be ^660000punished^000000...",
                            "^666666*grinds teeth*^000000"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("thai_find").get()?.number()? > 2 && ctx.var("thai_find").get()?.number()? < 14) {
                    ctx.lines_as(
                        "Shuda",
                        args![
                            "^666666*Sigh...*^000000",
                            "You haven't",
                            "found Annon yet?",
                            "I see. Oh well, it's alright.",
                            "Time isn't an issue for us."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Shuda", args!["And by 'us,' I mean '^660000you^000000.' I don't care about your plans or whatever else you may have scheduled, because you're going to make", "time for me! Oho ho ho ho ho!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("thai_find").get()? == 2 {
                    if ctx.call(Function::CountItem, vec![Val::from(7288)])?.is_true() {
                        ctx.lines_as(
                            "Shuda",
                            args![
                                "Ah... Is it...?!!",
                                "Yes, that's it!",
                                "My engagement ring that was",
                                "thrown into the water by that stupid Annon!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Shuda",
                            args!["Thank you so much! In your honor, our first seven kids will be named after you! Oho ho ho ho ho ho ho~"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Shuda",
                            args![
                                "Now...",
                                "All I gotta do",
                                "is find Annon.",
                                "Where the hell",
                                "did that moron go?!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Shuda",
                            args![
                                "^666666*Ahem*^000000",
                                "I-I didn't mean",
                                "to scare him, all I wanted was",
                                "my engagement ring back."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Shuda",
                            args![
                                "^666666*Sigh*^000000",
                                "I guess the days",
                                "of being a happy bride",
                                "are still far away from me..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args!["^3355FFShuda gazes directly", "at you with a puppy eyed look.^000000"])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("D-Don't look at me that way!:Alright, alright.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Shuda",
                                    args![
                                        "Oh come on, you know everything that happened, and I know you're gonna continue adventuring",
                                        "around this area."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Shuda", args!["I was just asking you to find Annon while you do that. Now, isn't that such a small favor? Thank you for doing this for me~"])?;
                                ctx.next()?;
                                ctx.lines_as("Shuda", args!["In the meantime, I'll think of how I'm gonna pay back Annon for leaving me in misery! So hurry and find Annon! Hoo ho ho ho ho!"])?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                                ctx.next()?;
                                ctx.call(
                                    Function::Emotion,
                                    vec![
                                        ctx.constant("ET_THINK")?,
                                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                    ],
                                )?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["(She's so scary!", "Making me do stuff", "for her again...)"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["(Why am I always", "doing things for other", "people anyway?!)"],
                                )?;
                            }
                            2 => {
                                ctx.lines_as("Shuda", args!["Yes, I knew I could count on you! So find Annon! It's my destiny to make him ^660000mine^000000. Hoo ho ho ho ho!"])?;
                                ctx.next()?;
                                ctx.lines_as("Shuda", args!["I haven't seen him around the village for a while. And I don't think he jumped into the water to find my ring, since he's too sissy for that. He probably doesn't have the guts to see me without it."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Shuda",
                                    args![
                                        "Will you please tell Annon that",
                                        "I got my ring back so he can come back to me now? I hope you'll remember that for me, adventurer."
                                    ],
                                )?;
                            }
                            _ => {}
                        }
                        ctx.call(Function::DelItem, vec![Val::from(7288), Val::from(1)])?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(12030), Val::from(12031)])?;
                        ctx.var("thai_find").set(Val::from(3))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Shuda", args!["Huh...?", "Did you find my ring?", "Show me, show it to me!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Shuda",
                            args!["What the...", "You got nothing!", "Don't lie to me,", "bring me my ring!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Shuda",
                            args![
                                "Hurry up and find",
                                "my ring. I'll be waiting",
                                "here for the good news.",
                                "Oho ho ho ho ho~!"
                            ],
                        )?;
                        if ctx.call(Function::CheckQuest, vec![Val::from(12029)])? == -1 {
                            ctx.call(Function::SetQuest, vec![Val::from(12029)])?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("thai_find").get()? == 1 {
                        ctx.lines_as(
                            "Shuda",
                            args!["I feel a little bad asking you this, especially since you're", "an outsider."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Shuda", args!["But that ring is really important to me. If you could find it for me, I'd be the happiest woman in the whole world."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Shuda",
                            args![
                                "So please, I beg you.",
                                "Please bring my ring back to me.",
                                "I want to live with Annon as a happy couple forever. Can't you understand that?!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Powerful-Looking Woman", args!["Umm...?", "Ah, you must be one of the tourists. I'd guide you around and listen to your stories about the outside world, but..."])?;
                        ctx.next()?;
                        if ctx.call(Function::Rand, vec![Val::from(0), Val::from(1)])?.is_true() {
                            ctx.lines_as(
                                "Powerful-Looking Woman",
                                args!["I'm reeeally", "busy right now.", "So very busy."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Powerful-Looking Woman", args!["Huh...?", "You don't think I look busy just because I'm standing on one spot and not moving at all?! Oh. My. God. You're so rude!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Powerful-Looking Woman",
                                args![
                                    "Now, I've got to find my",
                                    "engagement ring. If only",
                                    "my fiancee hadn't",
                                    "thrown it..."
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("What...?:Lies~!")])? {
                                1 => {
                                    ctx.lines_as(
                                        "Powerful-Looking Woman",
                                        args![
                                            "What 'what?'",
                                            "You still think I'm",
                                            "standing around for my",
                                            "own amusement or something?!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(
                                        Function::Emotion,
                                        vec![
                                            ctx.constant("ET_KIK")?,
                                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                        ],
                                    )?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["No no no no...", "I meant, I can't", "believe that you're", "engaged to someone!"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Powerful-Looking Woman",
                                        args![
                                            "Wwwwaaaaaahhhhh!",
                                            "Wh-who are you to say",
                                            "something so rude to such a nice lady like myself. Go away, I don't wanna talk to you!"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Powerful-Looking Woman",
                                        args![
                                            "What lies?",
                                            "I'm not gonna put up with crap",
                                            "if you're gonna say something like 'I can't believe you're engaged to someone!'"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "Whoa whoa whoa~",
                                            "Calm down, lady.",
                                            "Hear me out first.",
                                            "What I'm wondering is..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Powerful-Looking Woman",
                                        args![
                                            "I know, I know.",
                                            "You're wondering why I'm",
                                            "standing near the water, right? Well, it's because my ring was thrown there and I've gotta",
                                            "get it back."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Powerful-Looking Woman", args!["What am I doing, talking to a tourist like this? ^666666*Sigh...*^000000 Well, you learned what you wanted to know. I hope you're satisfied now."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Powerful-Looking Woman", args!["..."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Powerful-Looking Woman", args!["...", "......"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Powerful-Looking Woman", args!["...", "......", "........Hey."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Powerful-Looking Woman",
                                        args!["Why are you staring at me?", "Don't you have anything to do?"],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("No.:No~ and you are an interesting girl.")])? {
                                        1 => {
                                            ctx.lines_as("Powerful-Looking Woman", args!["I see. Hmmm...", "You're kind of my type."])?;
                                            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                                ctx.lines(args![
                                                    "No no, that doesn't mean I'm interested in you. Plus I'm",
                                                    "already engaged."
                                                ])?;
                                            } else {
                                                ctx.mes(
                                                    "Before you think of anything weird, I meant that I think you and me are a lot alike.",
                                                )?;
                                            }
                                            ctx.next()?;
                                            ctx.lines_as("Powerful-Looking Woman", args!["Let me introduce myself.", "My name is Shuda, I've lived in Ayothaya all my life. Since you don't have anything to do, I have", "a favor to ask of you."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Shuda", args!["My fiance Annon is always interested in things other than me. I don't understand why, and I don't think I deserve to be ignored!"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Shuda",
                                                args![
                                                    "^666666*Sigh...*^000000",
                                                    "Anyways, we're engaged",
                                                    "and we got rings for each other..."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Shuda", args!["I thought being engaged would improve our relationship. But he just took my ring and threw it into the water."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Shuda",
                                                args![
                                                    "I got upset and yelled at him to get my ring back for me, and then",
                                                    "I never saw him after that.",
                                                    "So what I want to ask is..."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["You want me", "to find your", "ring, don't you?"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Shuda",
                                                args!["Ah...", "So you know", "what I want.", "That makes it", "much simpler."],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Shuda",
                                                args![
                                                    "And once I retrieve",
                                                    "the ring, I'll make",
                                                    "him treat me to a much",
                                                    "deserved vacation in Jawaii..."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Shuda",
                                                args!["...Where I will", "^660000enslave^000000 him.", "^666666*Grinds teeth*^000000"],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(
                                                Function::Emotion,
                                                vec![
                                                    ctx.constant("ET_SURPRISE")?,
                                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                                ],
                                            )?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["(...H-Horrifying!)"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Shuda", args!["So anyway, I want you to go out and find my engagement ring. Since the water is calm and still around here, the ring is probably still under the water, and hasn't been swept away yet."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Shuda", args!["And so, that is my quest for you. It should keep you entertained for a while. So get into the water, and I'll supervise from here.", "Oho ho ho ho ho!"])?;
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                                            ctx.var("thai_find").set(Val::from(1))?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as(
                                                "Powerful-Looking Woman",
                                                args![
                                                    "^666666*Sigh...*^000000",
                                                    "Are you gonna just stare and",
                                                    "not help at all? I thought the word adventurer was synonymous",
                                                    "with 'hero,' not 'lazy.'"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Powerful-Looking Woman", args!["Oh I see...", "Don't help strangers, do you?", "Well, my name is Shuda, a beautiful damsel waiting for her engagement ring to rise up from the water."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Powerful-Looking Woman",
                                                args!["There...!", "We're not", "strangers anymore!", "Oho ho ho ho ho!"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Shuda", args!["Don't you pity me, and my tragic situation? If someone were to help this precious maiden find her ring, she'd be the happiest bride in the whole world."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Shuda", args!["And her savior", "would be known as", "the hero of Ayothaya."])?;
                                            ctx.next()?;
                                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args![" ...Say what?!"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Shuda", args!["Basically, you're gonna find my engagement ring, and I'll be the happiest bride in the whole world and I'll live happily ever after. What's so hard to understand", "about that?"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Shuda", args!["Sure, Annon, he's my fiancee, is spineless and disappears whenever", "I say something a little too harsh. What a little girl! Can't he see I'm trying to make him a better man?!"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["He ran away", "from you, didn't he?"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Shuda", args!["Eh...?", "What was that?"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "^666666*Ahem*^000000",
                                                    "You're way",
                                                    "too good for him.",
                                                    "(I'd have run away myself!)"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Shuda", args!["Yes, yes, I know.", "But what's important", "is that once I get my ring back, Annon will be permanently mine. Till ^660000death do we part^000000."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Shuda",
                                                args![
                                                    "So, are you",
                                                    "gonna help me or what?!",
                                                    "If you do, me and Annon are gonna name our first seven children after you."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Shuda", args!["So go and look under the water! There's no water current, so I'm sure it's still there! What are you waiting for, it's should be an easy swim for you! Oho ho ho ho ho!"])?;
                                            if ctx.call(Function::CheckQuest, vec![Val::from(12029)])? == -1 {
                                                ctx.call(Function::SetQuest, vec![Val::from(12029)])?;
                                            }
                                            ctx.var("thai_find").set(Val::from(1))?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        } else {
                            ctx.lines_as(
                                "Powerful-Looking Woman",
                                args![
                                    "^666666*Sigh...*^000000",
                                    "Why isn't it coming up from the water? Maybe if I stirred around with a wooden stick...?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Powerful-Looking Woman",
                                args![
                                    "...?!",
                                    "What are you",
                                    "looking at?!",
                                    "I'm busy, so if you",
                                    "want something",
                                    "I can't help you!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Powerful-Looking Woman",
                                args![
                                    "^666666Stupid tourists, running",
                                    "around all over Ayothaya,",
                                    "pestering and bothering",
                                    "beautiful girls like me...^000000"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
                step = PowerfulLookingWomanStep::OnTouch;
                continue 'machine;
            }
            PowerfulLookingWomanStep::OnTouch => {
                if ctx.var("thai_find").get()? == 0 {
                    ctx.lines_as(
                        "Powerful-Looking Woman",
                        args!["^666666*Sigh...*^000000", "Where the", "hell is it!?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Powerful-Looking Woman",
                        args![
                            "What are",
                            "you looking at?",
                            "Haven't you ever seen a delicate damsel sigh before? Leave me the hell alone!"
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

pub fn powerful_looking_woman(ctx: &Ctx) -> Script {
    powerful_looking_woman_run(ctx, PowerfulLookingWomanStep::Start, Vec::new()).map(|_| ())
}

pub fn powerful_looking_woman_ontouch(ctx: &Ctx) -> Script {
    powerful_looking_woman_run(ctx, PowerfulLookingWomanStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AnnonbloodStep {
    Start,
    OnTouch,
    HoistEnd1,
}

fn annonblood_run(ctx: &Ctx, mut step: AnnonbloodStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AnnonbloodStep::Start => {
                if !(ctx.var("$@annonactive").get()?.is_true()) {
                    if ctx.var("thai_find").get()? == 13 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["A blood stain?", "What's going on?", "I should take a look..."],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFUnder a giant tree,",
                            "you find some bushes with",
                            "blood smudged on some",
                            "of the leaves.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFWithin the bushes, you see",
                            "that the trail of blood continues, almost as if a bleeding animal struggled to find refuge within",
                            "the bushes.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Hmmm...."])?;
                        ctx.next()?;
                        'b1: {
                            let subject1 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Scatter the bushes.:Dig up the bushes.")],
                            )?);
                            let mut matched1 = false;
                            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                                matched1 = true;
                            }
                            if matched1 {
                                if !(ctx.call(Function::Rand, vec![Val::from(0), Val::from(2)])?.is_true()) {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["^666666*Cough cough*^000000", "It's so dusty!"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("?", args!["...Awwww", "Oh... Oh God...!"])?;
                                    ctx.next()?;
                                    ctx.call(
                                        Function::Emotion,
                                        vec![
                                            ctx.constant("ET_SURPRISE")?,
                                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                        ],
                                    )?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["Huh...?", "I heard something", "from below! Let's", "dig it up!"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^3355FFYou pull up the bushes,",
                                        "^3355FFand beneath the dust and",
                                        "^3355FFshrubbery, you find",
                                        "a small burrow.^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.var("$@annonactive").set(Val::from(1))?;
                                    ctx.call(Function::EnableNpc, vec![Val::from("Haggard Man")])?;
                                    ctx.lines_as(
                                        "Haggard Man",
                                        args![
                                            "^666666*Cough cough!*^000000",
                                            "Wh...who's there?",
                                            "Shuda?! If that's you,",
                                            "stay back, or I'll kill myself!"
                                        ],
                                    )?;
                                    ctx.call(
                                        Function::Emotion,
                                        vec![
                                            ctx.constant("ET_HUK")?,
                                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                        ],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("What are you doing here?:I'm not Shuda!")])? {
                                        1 => {
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["Hey...", "What are you doi--"],
                                            )?;
                                            ctx.next()?;
                                            ctx.var("$@annonactive").set(Val::from(0))?;
                                            ctx.call(Function::DisableNpc, vec![Val::from("Haggard Man")])?;
                                            ctx.mes("^3355FFHe vanished!^000000")?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as("Haggard Man", args!["Hmmm...?", "I sense kindess, sincerity and a general love for mankind in your voice. Verily, you are not Shuda."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Haggard Man", args!["You are unfamiliar to me, outsider. I'm unsure of how you found my hiding place, but if you have something to say, make it short!"])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                } else {
                                    ctx.lines(args![
                                        "^3355FFYou dig up the bushes, stirring",
                                        "up a large, choking cloud of dust.^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["^666666*Cough cough!*^000000", "What a large,", "choking cloud", "of dust!"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                                matched1 = true;
                            }
                            if matched1 {
                                ctx.lines(args![
                                    "^3355FFAs you dig up the bushes, the ground beneath one of the bushes crumbles and you hear a scream",
                                    "from beneath.^000000"
                                ])?;
                                ctx.next()?;
                                if !(ctx.call(Function::Rand, vec![Val::from(0), Val::from(2)])?.is_true()) {
                                    ctx.var("$@annonactive").set(Val::from(1))?;
                                    ctx.call(Function::EnableNpc, vec![Val::from("Haggard Man")])?;
                                    ctx.lines_as("?", args!["^666666*Cough cough*^000000", "...M-my legs!"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "Hmmm...?",
                                            "What was that noise?",
                                            "Ugh, it's awfully dusty.",
                                            "I better stop digging.",
                                            "^666666*cough cough cough*^000000"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                        }
                    } else if ctx.var("thai_find").get()? == 14 {
                        ctx.mes("^3355FFYou find that the burrow is empty. It seems Annon has found a new hiding place.^000000")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.mes("^3355FFYou find a pile of dead bushes under a large tree. The area around the bushes seems awfully dusty.^000000")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    step = AnnonbloodStep::OnTouch;
                    continue 'machine;
                }
                step = AnnonbloodStep::HoistEnd1;
                continue 'machine;
            }
            AnnonbloodStep::OnTouch => {
                if ctx.var("thai_find").get()? == 12 && !(ctx.call(Function::Rand, vec![Val::from(0), Val::from(2)])?.is_true()) {
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_SURPRISE")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Huh...?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["That looks", "like blood!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Hmm...", "Yes...", "That's definitely blood."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Strange to see", "a blood stain in", "this kind of place..."],
                    )?;
                    ctx.var("thai_find").set(Val::from(13))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(12033), Val::from(12034)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = AnnonbloodStep::HoistEnd1;
                continue 'machine;
            }
            AnnonbloodStep::HoistEnd1 => {
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn annonblood(ctx: &Ctx) -> Script {
    annonblood_run(ctx, AnnonbloodStep::Start, Vec::new()).map(|_| ())
}

pub fn annonblood_ontouch(ctx: &Ctx) -> Script {
    annonblood_run(ctx, AnnonbloodStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum HaggardManStep {
    Start,
    OnInit,
}

fn haggard_man_run(ctx: &Ctx, mut step: HaggardManStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HaggardManStep::Start => {
                if ctx.var("thai_find").get()? == 13 {
                    ctx.lines_as(
                        "Haggard Man",
                        args!["^666666*...Cough cough*^000000", "No...! My hiding place!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Haggard Man",
                        args![
                            "Hmmm...?",
                            "I'm bleeding?!",
                            "Oh, how did this happen?",
                            "My spiritual training wasn't supposed to go like this...!"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Treat his wound first.:Is that you, Annon?")])? {
                        1 => {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Anyway, I think you need to be treated. Let me check if I have some potions."],
                            )?;
                            ctx.next()?;
                            if ctx.call(Function::CountItem, vec![Val::from(504)])?.is_true() {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Ah...", "Here we are,", "a White Potion."],
                                )?;
                                ctx.call(Function::DelItem, vec![Val::from(504), Val::from(1)])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Haggard Man",
                                    args![
                                        "You mean the White Potion,",
                                        "the ultimate drink for healing?",
                                        "I appreciate you doing me this favor. ^666666*Phew*^000000 Now, I feel much better."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Annon",
                                    args![
                                        "Ah, my apologies for not introducing myself earlier.",
                                        "I am called Annon. You must wondering what I must be doing",
                                        "here under the ground."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Annon", args!["Well, I'm performing a special exercise for my spiritual training. Though, it may be hard to believe."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Annon",
                                    args![
                                        "What...?",
                                        "Shuda sent you?! Oh well...",
                                        "I thank you for curing me. Now, it's time for me to resume my training."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines(args![
                                    ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
                                ])?;
                                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_CLOAKING")?])?;
                                ctx.mes(
                                    "Wait, I can't let you go! You don't know how hard it was for me to find you! You can't just leave!",
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Annon", args!["Hm? Then what is it that you want? I implore you, do not tell Shuda that you have found me. No matter what, I am not going back to her!"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Annon",
                                    args![
                                        "Eh...?",
                                        "Did you find",
                                        "her ring already?",
                                        "Ooh, I was hoping it",
                                        "would never be found."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Annon", args!["She probably told you that I ran away out of shame for losing the ring, and that if you got the ring back, I'd be happy to return, right?"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Annon",
                                    args![
                                        "Well...",
                                        "I threw away",
                                        "that ring on purpose!",
                                        "I'm serious about not",
                                        "going back to Shuda."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Annon",
                                    args![
                                        "It looks like you wish to know more. If you promise me that you will not mention my whereabouts",
                                        "to Shuda, then I'll explain."
                                    ],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(
                                    ctx,
                                    &[Val::from("You think the world revolves around you?:What do you want me to do?")],
                                )? {
                                    1 => {
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["You think the world", "revolves around you?", "You can't just run away!"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Annon",
                                            args!["Oh, I know, I know!", "But you don't understand. Obviously, Shuda is beautiful."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Annon",
                                            args![
                                                "But our parents arranged our marriage when we were babies.",
                                                "That kind of arrangement doesn't matter to me, but she obviously cares about it."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Annon", args!["In all honesty, she scares me", "to death. Whenever I'm with her,", "I can't help but break into a cold sweat. And when she's near, I have nightmares about her. ^666666*Brrr...*^000000"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Annon", args!["She thinks it's funny to electrocute me. and ties me up in", "a darkened room and feeds me only Jellopy whenever I don't listen to her. Now can you see why I crave freedom?"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Annon", args!["Of course, if you reveal to her that you've found me, I'll repay your favor. I implore you, just pretend you haven't found me, please?"])?;
                                        ctx.next()?;
                                        let choice = runtime::select_values(ctx, &[Val::from("...Alright, alright.")])?;
                                        ctx.var("@menu").set(choice)?;
                                        ctx.lines_as(
                                            "Annon",
                                            args![
                                                "Thank you so much!",
                                                "Now, to pay you back, let me use one of the skills I've learned in my spiritual training."
                                            ],
                                        )?;
                                    }
                                    2 => {
                                        ctx.lines_as("Annon", args!["Of course, what I want you to do is to keep me a secret from Shuda. Just forget that you've ever met me, and let Shuda know it was impossible to find me."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Annon", args!["Besides, even if you help her, all she'll do is offer to name her kids after you. Now... Do you really want that? I know for sure that she and I won't be having children. There's enough evil in the world."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Annon", args!["Now, you must be wondering", "how I would repay you for your silence. Well, let me show you one of the skills I've learned in my spiritual training."])?;
                                    }
                                    _ => {}
                                }
                                ctx.next()?;
                                ctx.lines_as(
                                    "Annon",
                                    args!["Let's see...", "Let me twist your", "arm this way, and", "apply pressure here..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Wahhh~!", "What the hell", "are you doing!", "You're hurting me, man!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Annon",
                                    args![
                                        "Oh, this is a special kind",
                                        "of massage. Don't worry, it'll",
                                        "relieve you of fatigue. It's quite heavenly, actually. You'll see~"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FF*Snap snap snap*",
                                    "*Snap snap snap*",
                                    "*Crack crack crack*",
                                    "*Crack crack crack*^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines(args!["^3355FFYou feel as though Annon", "realigned every bone in your skeleton, placing them in the right spots. Strangely, you feel intense relaxation and refreshment instead of excruciating pain."])?;
                                ctx.var("thai_find").set(Val::from(14))?;
                                ctx.call(Function::CompleteQuest, vec![Val::from(12034)])?;
                                {
                                    if ctx.var("BaseLevel").get()?.number()? < 56 {
                                        ctx.call(Function::GetExperience, vec![Val::from(9000), Val::from(0)])?;
                                    } else {
                                        if (ctx.var("BaseLevel").get()?.number()? > 55 && ctx.var("BaseLevel").get()?.number()? < 61) {
                                            ctx.call(Function::GetExperience, vec![Val::from(10500), Val::from(0)])?;
                                        } else {
                                            if (ctx.var("BaseLevel").get()?.number()? > 60 && ctx.var("BaseLevel").get()?.number()? < 66) {
                                                ctx.call(Function::GetExperience, vec![Val::from(18684), Val::from(0)])?;
                                            } else {
                                                if (ctx.var("BaseLevel").get()?.number()? > 65
                                                    && ctx.var("BaseLevel").get()?.number()? < 71)
                                                {
                                                    ctx.call(Function::GetExperience, vec![Val::from(27411), Val::from(0)])?;
                                                } else if (ctx.var("BaseLevel").get()?.number()? > 70
                                                    && ctx.var("BaseLevel").get()?.number()? < 76)
                                                {
                                                    ctx.call(Function::GetExperience, vec![Val::from(70757), Val::from(0)])?;
                                                } else if (ctx.var("BaseLevel").get()?.number()? > 75
                                                    && ctx.var("BaseLevel").get()?.number()? < 81)
                                                {
                                                    ctx.call(Function::GetExperience, vec![Val::from(130246), Val::from(0)])?;
                                                } else if (ctx.var("BaseLevel").get()?.number()? > 80
                                                    && ctx.var("BaseLevel").get()?.number()? < 86)
                                                {
                                                    ctx.call(Function::GetExperience, vec![Val::from(150340), Val::from(0)])?;
                                                } else if (ctx.var("BaseLevel").get()?.number()? > 85
                                                    && ctx.var("BaseLevel").get()?.number()? < 91)
                                                {
                                                    ctx.call(Function::GetExperience, vec![Val::from(182052), Val::from(0)])?;
                                                } else {
                                                    ctx.call(Function::GetExperience, vec![Val::from(406786), Val::from(0)])?;
                                                }
                                            }
                                        }
                                    }
                                }
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Whoa...?",
                                        "That felt was so great~",
                                        "It even feels like my mind",
                                        "has been cleansed too!",
                                        "You've got a deal!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Annon",
                                    args![
                                        "Thank you, thank you so much.",
                                        "Now I can avoid being tied to Shuda, at least for a little longer."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Annon",
                                    args![
                                        "Now, if you'll excuse me,",
                                        "I must make good my escape",
                                        "before others find me.",
                                        "Good day~!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.var("$@annonactive").set(Val::from(0))?;
                                ctx.call(Function::DisableNpc, vec![Val::from("Haggard Man")])?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "No way, no way.",
                                        "He's too seriously",
                                        "wounded to be cured",
                                        "with the normal potions",
                                        "that I have. I'll need",
                                        "a stronger potion..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Okay, you stay",
                                        "here for a while.",
                                        "I'll bring you some",
                                        "medicine and treat",
                                        "your wound."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Haggard Man",
                                    args![
                                        "No, you don't have to...",
                                        "^666666*Cough cough cough*^000000",
                                        "W-wait, don't push",
                                        "me back inside!",
                                        "Wahhhhhh~!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.var("$@annonactive").set(Val::from(0))?;
                                ctx.call(Function::DisableNpc, vec![Val::from("Haggard Man")])?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Okay, it's time",
                                        "for me to play doctor!",
                                        "I don't have much time!",
                                        "My mission: Bring back an",
                                        "ultra powerful potion for",
                                        "this poor guy~"
                                    ],
                                )?;
                                ctx.call(
                                    Function::Emotion,
                                    vec![
                                        ctx.constant("ET_BEST")?,
                                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Haggard Man",
                                args![
                                    "Annon...?",
                                    "Who's that?",
                                    "I've never heard of him!",
                                    "I don't know him, okay!",
                                    "Stop disturbing my training!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Uh...",
                                    "Hmm, it doesn't seem like you should be yelling with that kind of wound. You sure you're not Annon?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Haggard Man",
                                args!["I told you!", "I've never", "heard of him!", "L-leave me alone!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Haggard Man", args!["I have to", "hide from Shuda.", "^666666*Cries*^000000"])?;
                            ctx.close_window()?;
                            ctx.var("$@annonactive").set(Val::from(0))?;
                            ctx.call(Function::DisableNpc, vec![Val::from("Haggard Man")])?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.lines_as(
                        "Haggard Man",
                        args![
                            "Huh? Who are you?",
                            "This isn't a safe place for a tourists! You should leave right away!"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.var("$@annonactive").set(Val::from(0))?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Haggard Man")])?;
                    return Err(Stop::End);
                }
                step = HaggardManStep::OnInit;
                continue 'machine;
            }
            HaggardManStep::OnInit => {
                ctx.var("$@annonactive").set(Val::from(0))?;
                ctx.call(Function::DisableNpc, vec![Val::from("Haggard Man")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn haggard_man(ctx: &Ctx) -> Script {
    haggard_man_run(ctx, HaggardManStep::Start, Vec::new()).map(|_| ())
}

pub fn haggard_man_oninit(ctx: &Ctx) -> Script {
    haggard_man_run(ctx, HaggardManStep::OnInit, Vec::new()).map(|_| ())
}

fn fisherman_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_randfish = Val::from(0);
    if ctx.var("thai_find").get()? == 1 {
        ctx.lines_as(
            "Dannai",
            args![
                "This place is known to be",
                "teeming with fish. The fish here tend to eat anything they find,",
                "so it's easy to catch them."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Dannai", args!["We are providing a fishing rod rental service. Every time you fish, you'll need ^3355FF 1 Monster's Feed^000000 to use as bait, and pay", "a rod rental fee of ^3355FF50 Zeny^000000."])?;
        ctx.next()?;
        ctx.lines_as("Dannai", args!["Would you", "like to try?"])?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No, thanks.")])?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                if (ctx.call(Function::CountItem, vec![Val::from(528)])?.is_true() && ctx.var("Zeny").get()?.number()? > 49) {
                    ctx.lines(args!["^3355FFYou cast your", "fishing line", "into the water.^000000"])?;
                    ctx.next()?;
                    ctx.mes("...")?;
                    ctx.next()?;
                    ctx.lines(args!["...", "......"])?;
                    ctx.next()?;
                    ctx.mes("^3355FFYou've hooked a Phen!^000000")?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(528), Val::from(1)])?;
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(50))?))?;
                    l_randfish = ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?;
                    if (l_randfish.clone().number()? > 0 && l_randfish.clone().number()? < 40) {
                        ctx.lines(args![
                            "^3355FFWhile cooking the Phen, you",
                            "find that it's swallowed a Stone.^000000"
                        ])?;
                        ctx.close_window()?;
                        ctx.call(Function::GetItem, vec![Val::from(7049), Val::from(1)])?;
                        return Err(Stop::End);
                    } else {
                        if (l_randfish.clone().number()? > 39 && l_randfish.clone().number()? < 60) {
                            ctx.lines(args!["^3355FFInside of the Phen,", "you find a Red Potion.^000000"])?;
                            ctx.close_window()?;
                            ctx.call(Function::GetItem, vec![Val::from(501), Val::from(1)])?;
                            return Err(Stop::End);
                        } else {
                            if (l_randfish.clone().number()? > 59 && l_randfish.clone().number()? < 70) {
                                ctx.lines(args!["^3355FFInside of the Phen,", "you find a Yellow Potion.^000000"])?;
                                ctx.close_window()?;
                                ctx.call(Function::GetItem, vec![Val::from(503), Val::from(1)])?;
                                return Err(Stop::End);
                            } else if (l_randfish.clone().number()? > 69 && l_randfish.clone().number()? < 80) {
                                ctx.lines(args!["^3355FFThere's nothing inside...", "But now you can eat Sushi!^000000"])?;
                                ctx.close_window()?;
                                ctx.call(Function::GetItem, vec![Val::from(551), Val::from(1)])?;
                                return Err(Stop::End);
                            } else if (l_randfish.clone().number()? > 79 && l_randfish.clone().number()? < 90) {
                                ctx.lines(args!["^3355FFYou find", "a Flower Ring", "inside the Phen.^000000"])?;
                                ctx.close_window()?;
                                ctx.call(Function::GetItem, vec![Val::from(2612), Val::from(1)])?;
                                return Err(Stop::End);
                            } else if l_randfish.clone() == 90 {
                                ctx.lines(args!["^3355FFYou find", "a Diamond Ring", "inside the Phen.^000000"])?;
                                ctx.close_window()?;
                                ctx.call(Function::GetItem, vec![Val::from(2613), Val::from(1)])?;
                                return Err(Stop::End);
                            } else if (l_randfish.clone().number()? > 90 && l_randfish.clone().number()? < 101) {
                                ctx.lines(args![
                                    "^3355FFYou find a ring",
                                    "with a name engraved",
                                    "inside the band.^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines(args!["^3355FFYou take the", "engagement ring.^000000"])?;
                                ctx.close_window()?;
                                ctx.var("thai_find").set(Val::from(2))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(12029), Val::from(12030)])?;
                                ctx.call(Function::GetItem, vec![Val::from(7288), Val::from(1)])?;
                                return Err(Stop::End);
                            }
                        }
                    }
                } else {
                    ctx.lines_as(
                        "Dannai",
                        args!["Remember, you need ^3355FF1 Monster's Feed^000000 to use as bait and ^3355FF50 zeny^000000 to rend a rod."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Dannai",
                    args![
                        "No problem,",
                        "come back anytime you",
                        "want. Fishing relaxes",
                        "the mind and makes you",
                        "feel at peace..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("thai_find").get()?.number()? < 1 {
        ctx.lines_as(
            "Dannai",
            args![
                "The fish here tend",
                "to eat anything they find.",
                "When you gut them, you can",
                "often find interesting things inside."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dannai",
            args![
                "So when you get a chance,",
                "why don't you sit down, relax",
                "and enjoy some quiet fishing?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Dannai", args!["Welcome, long time no see!", "It seems people learned that you found some kind of fortune from fishing here. After word got around, I've been getting lots", "of customers."])?;
        ctx.next()?;
        ctx.lines_as(
            "Dannai",
            args![
                "I understand that you want",
                "fish, but right now there",
                "aren't any fishing spots",
                "for you at the moment."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dannai",
            args!["Please let other", "people have their", "chance to find their", "own fortunes."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn fisherman(ctx: &Ctx) -> Script {
    fisherman_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ayofootprint1Step {
    Start,
    OnTouch,
}

fn ayofootprint1_run(ctx: &Ctx, mut step: Ayofootprint1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ayofootprint1Step::Start => {
                step = Ayofootprint1Step::OnTouch;
                continue 'machine;
            }
            Ayofootprint1Step::OnTouch => {
                if ctx.var("thai_find").get()? == 4 && ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?.number()? < 2 {
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_SURPRISE")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.mes("^3355FFYou find footprints heading to the ^5C3317North^3355FF. It looks like somebody was in quite a hurry, just like the villager said!^000000")?;
                    ctx.var("thai_find").set(Val::from(5))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(12032), Val::from(12033)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn ayofootprint1(ctx: &Ctx) -> Script {
    ayofootprint1_run(ctx, Ayofootprint1Step::Start, Vec::new()).map(|_| ())
}

pub fn ayofootprint1_ontouch(ctx: &Ctx) -> Script {
    ayofootprint1_run(ctx, Ayofootprint1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ayofootprint2Step {
    Start,
    OnTouch,
}

fn ayofootprint2_run(ctx: &Ctx, mut step: Ayofootprint2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ayofootprint2Step::Start => {
                step = Ayofootprint2Step::OnTouch;
                continue 'machine;
            }
            Ayofootprint2Step::OnTouch => {
                if ctx.var("thai_find").get()? == 5 && ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?.number()? < 2 {
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_THINK")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.lines(args!["^3355FFYou find another set of footprints. It seems that somebody was running away from something. These prints are heading to the ^5C3317East^3355FF. You'd better follow them to see what", "you can find.^000000"])?;
                    ctx.var("thai_find").set(Val::from(6))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn ayofootprint2(ctx: &Ctx) -> Script {
    ayofootprint2_run(ctx, Ayofootprint2Step::Start, Vec::new()).map(|_| ())
}

pub fn ayofootprint2_ontouch(ctx: &Ctx) -> Script {
    ayofootprint2_run(ctx, Ayofootprint2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ayofootprint3Step {
    Start,
    OnTouch,
}

fn ayofootprint3_run(ctx: &Ctx, mut step: Ayofootprint3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ayofootprint3Step::Start => {
                step = Ayofootprint3Step::OnTouch;
                continue 'machine;
            }
            Ayofootprint3Step::OnTouch => {
                if ctx.var("thai_find").get()? == 6 && ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?.number()? < 2 {
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_STARE_ABOUT")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.lines(args![
                        "^3355FFThe footprints end around this area. Whoever was running must",
                        "have been exhausted and stumbled on something. Perhaps the owner of the footprints is nearby...^000000"
                    ])?;
                    ctx.var("thai_find").set(Val::from(7))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn ayofootprint3(ctx: &Ctx) -> Script {
    ayofootprint3_run(ctx, Ayofootprint3Step::Start, Vec::new()).map(|_| ())
}

pub fn ayofootprint3_ontouch(ctx: &Ctx) -> Script {
    ayofootprint3_run(ctx, Ayofootprint3Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ayofootprint4Step {
    Start,
    OnTouch,
}

fn ayofootprint4_run(ctx: &Ctx, mut step: Ayofootprint4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ayofootprint4Step::Start => {
                step = Ayofootprint4Step::OnTouch;
                continue 'machine;
            }
            Ayofootprint4Step::OnTouch => {
                if ctx.var("thai_find").get()? == 7 && ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?.number()? < 2 {
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_AHA")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.lines(args!["^3355FFAs you follow the footprints,", "you arrive at the '^5C3317Entrance of the Shrine^3355FF.' Whoever this person was, he just ran into the second level.^000000"])?;
                    ctx.var("thai_find").set(Val::from(8))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn ayofootprint4(ctx: &Ctx) -> Script {
    ayofootprint4_run(ctx, Ayofootprint4Step::Start, Vec::new()).map(|_| ())
}

pub fn ayofootprint4_ontouch(ctx: &Ctx) -> Script {
    ayofootprint4_run(ctx, Ayofootprint4Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ayofootprint5Step {
    Start,
    OnTouch,
}

fn ayofootprint5_run(ctx: &Ctx, mut step: Ayofootprint5Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ayofootprint5Step::Start => {
                step = Ayofootprint5Step::OnTouch;
                continue 'machine;
            }
            Ayofootprint5Step::OnTouch => {
                if ctx.var("thai_find").get()? == 8 && ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?.number()? < 2 {
                    ctx.lines(args![
                        "^3355FFYou find the same footprints",
                        "in this place. It seems pretty",
                        "dangerous, and you wonder why",
                        "anyone would come here. The",
                        "footprints continue towards",
                        "the ^5C3317North^3355FF.^000000"
                    ])?;
                    ctx.var("thai_find").set(Val::from(9))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn ayofootprint5(ctx: &Ctx) -> Script {
    ayofootprint5_run(ctx, Ayofootprint5Step::Start, Vec::new()).map(|_| ())
}

pub fn ayofootprint5_ontouch(ctx: &Ctx) -> Script {
    ayofootprint5_run(ctx, Ayofootprint5Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ayofootprint6Step {
    Start,
    OnTouch,
}

fn ayofootprint6_run(ctx: &Ctx, mut step: Ayofootprint6Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_randayo = Val::from(0);
    'machine: loop {
        match step {
            Ayofootprint6Step::Start => {
                step = Ayofootprint6Step::OnTouch;
                continue 'machine;
            }
            Ayofootprint6Step::OnTouch => {
                if ctx.var("thai_find").get()? == 9 && ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 1 {
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_SURPRISE")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.lines(args![
                        "^3355FFThe trail of footprints",
                        "continue here, and seem",
                        "to look more freshly made.^000000."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args!["^3355FFHmm...?", "There's something here...^000000"])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Ignore.:^5C3317Investigate.^000000")])? {
                        1 => {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Well, it's probably nothing. But which way should I go now?"],
                            )?;
                            ctx.var("thai_find").set(Val::from(10))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            l_randayo = ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?;
                            if l_randayo.clone().number()? < 2 {
                                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Why, this is a pack of White Potions. Whoever owned these must have dropped these by accident. Hmm, I could use these for later..."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Wait...",
                                        "There are more",
                                        "footprints heading",
                                        "toward the ^5C3317East^000000.",
                                        "I better look",
                                        "into this."
                                    ],
                                )?;
                                ctx.var("thai_find").set(Val::from(10))?;
                                ctx.call(Function::GetItem, vec![Val::from(504), Val::from(10)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (l_randayo.clone().number()? > 2 && l_randayo.clone().number()? < 5) {
                                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Why, this is a pack of Yellow Potions. Whoever owned these must have dropped these by accident. Hmm, I could use these for later..."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Wait...",
                                        "There are more",
                                        "footprints heading",
                                        "toward the ^5C3317East^000000.",
                                        "I better look",
                                        "into this."
                                    ],
                                )?;
                                ctx.var("thai_find").set(Val::from(10))?;
                                ctx.call(Function::GetItem, vec![Val::from(503), Val::from(10)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as("Rat", args!["^FF0000Squeak!", "^FF0000Squeak Squeak!^000000"])?;
                                ctx.call(
                                    Function::Emotion,
                                    vec![
                                        ctx.constant("ET_HUK")?,
                                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["^8C1717Wah^000000!", "What was that?!"],
                                )?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFIt was just a rat.",
                                    "It feels awful silly to be scared by just a little rat, does it not?^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFYou find another trail",
                                    "of footprints that heads",
                                    "towards the ^5C3317East^3355FF.^000000"
                                ])?;
                                ctx.var("thai_find").set(Val::from(10))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        _ => {}
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn ayofootprint6(ctx: &Ctx) -> Script {
    ayofootprint6_run(ctx, Ayofootprint6Step::Start, Vec::new()).map(|_| ())
}

pub fn ayofootprint6_ontouch(ctx: &Ctx) -> Script {
    ayofootprint6_run(ctx, Ayofootprint6Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ayofootprint7Step {
    Start,
    OnTouch,
}

fn ayofootprint7_run(ctx: &Ctx, mut step: Ayofootprint7Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ayofootprint7Step::Start => {
                step = Ayofootprint7Step::OnTouch;
                continue 'machine;
            }
            Ayofootprint7Step::OnTouch => {
                if ctx.var("thai_find").get()? == 10 && ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?.number()? < 2 {
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_SURPRISE")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.lines(args![
                        "^3355FFWhere is he...?",
                        "He seems to have gone",
                        "deep into the dungeon.",
                        "Just looking for this",
                        "guy is exhausting...^000000"
                    ])?;
                    ctx.var("thai_find").set(Val::from(11))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn ayofootprint7(ctx: &Ctx) -> Script {
    ayofootprint7_run(ctx, Ayofootprint7Step::Start, Vec::new()).map(|_| ())
}

pub fn ayofootprint7_ontouch(ctx: &Ctx) -> Script {
    ayofootprint7_run(ctx, Ayofootprint7Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ayofootprint8Step {
    Start,
    OnTouch,
}

fn ayofootprint8_run(ctx: &Ctx, mut step: Ayofootprint8Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ayofootprint8Step::Start => {
                step = Ayofootprint8Step::OnTouch;
                continue 'machine;
            }
            Ayofootprint8Step::OnTouch => {
                if ctx.var("thai_find").get()? == 11 && ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?.number()? < 2 {
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_SURPRISE")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.mes("^3355FFYou find traces of blood. Whoever was running in this direction may have been wounded. The trail of blood leads to the ^5C3317North^3355FF.^000000")?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(12032), Val::from(12033)])?;
                    ctx.var("thai_find").set(Val::from(12))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn ayofootprint8(ctx: &Ctx) -> Script {
    ayofootprint8_run(ctx, Ayofootprint8Step::Start, Vec::new()).map(|_| ())
}

pub fn ayofootprint8_ontouch(ctx: &Ctx) -> Script {
    ayofootprint8_run(ctx, Ayofootprint8Step::OnTouch, Vec::new()).map(|_| ())
}

fn old_man_02_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_randayo = Val::from(0);
    if ctx.var("thai_find").get()? == 3 {
        ctx.lines_as(
            "Tham",
            args![
                "Weird...",
                "Why was that guy",
                "running into the",
                "shrine like that?",
                "That really",
                "bothers me..."
            ],
        )?;
        ctx.next()?;
        ctx.mes("[Tham]")?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
        ctx.lines(args!["Oh, aren't you", "from Midgard?", "Hello there~"])?;
        ctx.next()?;
        ctx.lines_as("Tham", args!["Ah...", "You heard me talking to myself? Well, I didn't see who he was but, some guy ran into the forest which leads to the shrine. He seemed really terrified of something."])?;
        ctx.next()?;
        ctx.lines_as(
            "Tham",
            args!["He came from out of nowhere, so he scared the crap out of me. Do you think he could be a criminal?"],
        )?;
        ctx.next()?;
        ctx.mes("[Tham]")?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
        ctx.mes("Ah! I think, in his intense fear, he was screaming something about some kind of '^FF0000Shuda^000000,' or whatever. But what could it possibly mean?")?;
        ctx.next()?;
        ctx.lines_as(
            "Tham",
            args!["Oh well, don't worry. All of our villagers aren't like that. Anyway, I hope you enjoy your travels in Ayothaya."],
        )?;
        ctx.var("thai_find").set(Val::from(4))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(12031), Val::from(12032)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        l_randayo = ctx.call(Function::Rand, vec![Val::from(1), Val::from(7)])?;
        if l_randayo.clone().number()? < 5 {
            ctx.lines_as(
                "Tham",
                args!["Oh hello~", "I can tell from your clothes that you must be from Midgard."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tham",
                args![
                    "Recently, it seems that",
                    "our village has been receiving",
                    "more tourism, and I can see that",
                    "our village is being influenced",
                    "in a positive way."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tham",
                args![
                    "Ah...",
                    "While you're here,",
                    "why don't you taste some",
                    "authentic Ayothaya cuisine",
                    "for yourself?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tham",
                args![
                    "We're known for our",
                    "spicy and sweet seafood,",
                    "and there are some nice",
                    "restaurants around here."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tham",
                args!["Anyway, I'm sure that you'll like it. I hope that you come visit Ayothaya as often as you can~"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (l_randayo.clone().number()? > 4 && l_randayo.clone().number()? < 7) {
            ctx.lines_as(
                "Tham",
                args![
                    "Ah~",
                    "I've got this craving for Ms. Mali the Spicy's food, especially her 'Tom Yum Goong.'"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Tham", args!["I think it's the", "most delicious dish", "in the entire world!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Tham",
                args!["Oh hello~", "I can tell from your clothes that you must be from Midgard."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tham",
                args!["I hope that you enjoy your stay here, and that you come visit Ayothaya as often as you can~"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn old_man_02(ctx: &Ctx) -> Script {
    old_man_02_body(ctx, Vec::new()).map(|_| ())
}

fn dusit_thai_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ayodunquest").get()? == 0 {
        ctx.lines_as(
            "Dusit",
            args![
                "Oh...!",
                "You must be a traveller.",
                "I have a tale you may wish to hear if you're interested in listening."
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("What is it?:I know what the story is.")],
        )?) == 1
        {
            ctx.lines_as("Dusit", args!["Ayothaya has a long history. Perhaps it looks calm and peaceful now, but in the past our village was terrorized by a terrible creature."])?;
            ctx.next()?;
            ctx.lines_as(
                "Dusit",
                args![
                    "This beast is known to us as",
                    "the ^660000Sa-mhing Tiger^000000, which used",
                    "to dwell deep in the ancient ruins."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Dusit", args!["The Sa-mhing Tiger is addicted", "to the taste of human flesh. It has been known to transform into the shape of a person it has eaten in order to lure out his family and friends."])?;
            ctx.next()?;
            ctx.lines_as("Dusit", args!["The local people are still afraid that the creature might transform into someone they know, and sneak into their homes to eat them."])?;
            ctx.next()?;
            ctx.lines_as("Dusit", args!["However, there is one way you can identify the Sa-mhing Tiger from a human being. Since it's a tiger, it cannot do some of the things", "humans are taught to do,", "such as light a match."])?;
            ctx.next()?;
            ctx.lines_as(
                "Dusit",
                args![
                    "So, if by any chance you see",
                    "a friend acting strangely all",
                    "of a sudden, ask him to strike",
                    "a match. Make sure he is not",
                    "the Sa-mhing Tiger."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Dusit", args!["However...", "I'm not sure if this actually works or not. I guess the best thing to do is just run for your life if you even suspect someone of being", "the Sa-mhing Tiger."])?;
            ctx.next()?;
            ctx.lines_as("Dusit", args!["Do you believe", "in the existance", "of man-eating tigers?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Well, not really.:Yes, I do and I think they're scary!")],
            )?) == 1
            {
                ctx.lines_as(
                    "Dusit",
                    args![
                        "Umm...",
                        "Oh well. My late grandfather told me about the Sa-mhing Tiger when",
                        "I was a little kid. Of course,",
                        "I haven't seen it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dusit",
                    args![
                        "The closest thing I've seen to the Sa-mhing Tiger was a golden cat with a bell. Somehow, I don't think it ate men."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Dusit",
                args![
                    "Run for your life if you ever encounter the Sa-mhing Tiger!",
                    "It'll eat you alive!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dusit",
                args![
                    "If you're interested in finding",
                    "out more about the story, I can introduce you to someone who",
                    "knows that creature more than",
                    "anyone else."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dusit",
                args!["Why don't you talk to Boonthom? He's a pretty strange person, but there's something about him..."],
            )?;
            ctx.next()?;
            ctx.lines_as("Dusit", args!["...", "......"])?;
            ctx.next()?;
            ctx.lines_as(
                "Dusit",
                args!["Ah well, you'll see what I'm talking about. I hope he'll help you learn more about that evil creature."],
            )?;
            ctx.var("ayodunquest").set(Val::from(1))?;
            ctx.call(Function::SetQuest, vec![Val::from(12035)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Dusit", args!["You do now?", "Then be careful!", "It'll eat you alive!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ayodunquest").get()?.number()? > 0 {
        ctx.lines_as(
            "Dusit",
            args![
                "How are you?",
                "If you're interested",
                "in the Sa-mhing Tiger,",
                "please go talk to Boonthom."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dusit",
            args!["But beware!", "The Sa-mhing Tiger might be", "closer to you than you expect!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn dusit_thai(ctx: &Ctx) -> Script {
    dusit_thai_body(ctx, Vec::new()).map(|_| ())
}
