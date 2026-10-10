use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn delna_li_reken_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Delna",
        args![
            "Sometimes the simple",
            "pleasures can give you",
            "the most happiness. For me,",
            "going outside and basking in",
            "the sun is the greatest thing~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Delna",
        args![
            "Yes, sunbathing in a quiet",
            "and relaxing place can be",
            "so refreshing. And if you're",
            "careful about not getting a",
            "sunburn or a tan, a little sun",
            "can be really good for you."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn delna_li_reken(ctx: &Ctx) -> Script {
    delna_li_reken_body(ctx, Vec::new()).map(|_| ())
}

fn martial_artist_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Martial Artist",
        args![
            "Curses...",
            "I've come to the",
            "wrong place to seek",
            "out a challenge. No",
            "one here is really all",
            "that mighty or competitive!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Martial Artist",
        args![
            "This whole city thinks",
            "it can buy power and safety",
            "with money. They don't know",
            "the value of a nice, friendly",
            "brawl. Hopefully, I'll find a",
            "rival around here soon..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn martial_artist_1(ctx: &Ctx) -> Script {
    martial_artist_1_body(ctx, Vec::new()).map(|_| ())
}

fn kosit_zen1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Kosit",
        args![
            "This city might have",
            "more guards and rules",
            "than other places, but",
            "I still don't know if it's",
            "really safe to live here."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kosit",
        args![
            "I mean, the reason we have",
            "these rules is because of all",
            "the unruly gangsters that can",
            "sometimes get into the city.",
            "I mean, it's relatively peaceful and all. But these rules..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kosit",
        args![
            "It's good to be safe,",
            "but I don't know if it's",
            "a good idea to sacrifice",
            "our freedoms or standard",
            "of living, you know?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn kosit_zen1(ctx: &Ctx) -> Script {
    kosit_zen1_body(ctx, Vec::new()).map(|_| ())
}

fn sopheap_zen1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Sopheap",
        args![
            "Oh, you youngsters.",
            "Always traveling around",
            "and having adventures and",
            "fighting monsters. I certainly",
            "had my fill of excitement back",
            "when I was your age, long ago."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Sopheap",
        args![
            "Sure, I miss doing all",
            "of that, but now I'm content",
            "with just relaxing and resting.",
            "Still, there are a lot of old folk who refuse to sit still like this~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn sopheap_zen1(ctx: &Ctx) -> Script {
    sopheap_zen1_body(ctx, Vec::new()).map(|_| ())
}

fn gopal_zen4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Gopal",
        args![
            "Granny may be happy",
            "just sitting around and",
            "enjoying the peaceful life,",
            "but I'm not! I'm too young",
            "to just lay down and let",
            "these days just pass by!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Gopal",
        args![
            "I wanna make something",
            "of myself. Maybe someday,",
            "I'll found a company as big",
            "as the Rekenber Corporation!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn gopal_zen4(ctx: &Ctx) -> Script {
    gopal_zen4_body(ctx, Vec::new()).map(|_| ())
}

fn kimmy_zen3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Kimmy",
        args![
            "Unlike most places,",
            "Lighthalzen has many",
            "beautiful clothing and",
            "accessory shops. This",
            "place is heaven to a",
            "trend-setter like me~!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kimmy",
        args![
            "I don't know if you",
            "adventurers are interested",
            "in fashion, but you can trash",
            "your old clothes and get some",
            "new, unique and trendy gear",
            "over here in Lighthalzen~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn kimmy_zen3(ctx: &Ctx) -> Script {
    kimmy_zen3_body(ctx, Vec::new()).map(|_| ())
}

fn bodger_zen5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Bodger",
        args![
            "Another hungry day...",
            "I don't have any money",
            "and even if I did, there's",
            "no place that sells food",
            "I'd actually eat. Oh, man.",
            "I'm barely living as it is."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Bodger",
        args![
            "I hear that the people",
            "who live Uptown eat totally",
            "delicious, gourmet food eight",
            "times a day! Hopefully it's just an exaggeration. 'Cuz if it",
            "wasn't, I'd be so mad..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bodger_zen5(ctx: &Ctx) -> Script {
    bodger_zen5_body(ctx, Vec::new()).map(|_| ())
}

fn avetis_zen10_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Avetis",
        args![
            "A-ack...",
            "^333333*Cough cough*^000000",
            "Would you give me",
            "some m-medicine?!",
            "^333333*Cough cough haack*^000000",
            "Sweet Christmas, it hurts..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Avetis",
        args![
            "I sk-skipped work",
            "because I've been too",
            "sick t-to go. ^333333*Cough*^000000",
            "But now I don't have",
            "the money to ^333333*Haack*^000000",
            "buy med-medicine... "
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn avetis_zen10(ctx: &Ctx) -> Script {
    avetis_zen10_body(ctx, Vec::new()).map(|_| ())
}

fn diana_npc_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Diana",
        args![
            "Oh wow, that weapon",
            "is fantastic! But I'm sure",
            "that it must be really",
            "expensive. Huh..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Shop Assistant",
        args![
            "Ma'am, are you talking",
            "about this Stiletto? You",
            "certainly have an eye for",
            "quality weapons. If you",
            "don't mind me asking,",
            "where are you from?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Diana", args!["Oh, I was born", "and raised in Morocc."])?;
    ctx.next()?;
    ctx.lines_as(
        "Shop Assistant",
        args![
            "Ah yes, I've heard many",
            "good things about that town.",
            "You've certainly proven that",
            "people from Morocc truly have",
            "good taste. Now, this Stiletto",
            "is a steal at 39,800 zeny..."
        ],
    )?;
    ctx.next()?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
    ctx.lines_as(
        "Diana",
        args![
            "Huh...?!",
            "That's ridiculous!",
            "Never mind that, let",
            "me take a look at that",
            "Gladius in the corner."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Shop Assistant",
        args!["The Gladius?", "Ah, that would be", "39,800 zeny, ma'am."],
    )?;
    ctx.next()?;
    ctx.lines_as("Diana", args!["Oh, that's a really", "good price. I'll take it!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Shop Assistant",
        args![
            "Yes, this replica really",
            "does look just like an",
            "actual Gladius, does it",
            "not? Although not a true",
            "weapon, it's quite capable of",
            "opening the toughest envelopes."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Diana", args!["...", "......", "I take that back.", "This shop really sucks."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn diana_npc(ctx: &Ctx) -> Script {
    diana_npc_body(ctx, Vec::new()).map(|_| ())
}

fn shop_assistant_cobo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Shop Assistant",
        args![
            "Welcome to our",
            "store where we offer",
            "many unique products that",
            "you can't find anywhere else."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Shop Assistant",
        args![
            "However, shopping is only available to our members. There's an annual",
            "membership fee that's waived when you spend a certain amount every",
            "month in our store. If you invite your friends, you'll receive spe--"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["N-no thank you!", "I'm not interested!"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn shop_assistant_cobo(ctx: &Ctx) -> Script {
    shop_assistant_cobo_body(ctx, Vec::new()).map(|_| ())
}

fn sergei_zen1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Sergei",
        args![
            "You know, there's an",
            "interesting story about",
            "the axe that's hanging",
            "over there. Would you",
            "like me to tell you?"
        ],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Sure.:No, thanks.")])?) == 1 {
        ctx.lines_as(
            "Sergei",
            args![
                "This previous owner of",
                "this Weapon Shop was",
                "a convicted serial killer.",
                "Each night, he'd take that",
                "axe and cruelly murder",
                "beautiful ladies like me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sergei",
            args![
                "When he was finally",
                "caught, they beheaded",
                "him with his own axe.",
                "Since then, they say that",
                "his ghost still lingers and",
                "sharpens this axe at night."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sergei",
            args![
                "Just thinking about",
                "it gives me goosebumps!",
                "And I'm supposed to work",
                "here! It's so creepy!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Sergei",
        args![
            "Oh, how disappointing~",
            "It's the perfect story for",
            "the season. Well, now that",
            "I think about it, that story is",
            "actually pretty creepy..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn sergei_zen1(ctx: &Ctx) -> Script {
    sergei_zen1_body(ctx, Vec::new()).map(|_| ())
}

fn srinivas_zen4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Srinivas",
        args![
            "Those rundown buildings",
            "in the slums are an eyesore",
            "that offend the entire city!",
            "I just wish they would wreck",
            "them down. What do I care",
            "about the poor and needy?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn srinivas_zen4(ctx: &Ctx) -> Script {
    srinivas_zen4_body(ctx, Vec::new()).map(|_| ())
}

fn victor_perfecto_zen9_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Victor Perfecto",
        args![
            "I've heard that the",
            "Rekenber Corporation",
            "actually created the",
            "environment in Lighthalzen",
            "through artificial means."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Victor Perfecto",
        args![
            "It seems like it'd take",
            "a lot of investment, but",
            "artificially creating an",
            "environment isn't impossible",
            "with the means available to",
            "the Rekenber Corporation."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Victor Perfecto",
        args![
            "^333333*Sigh...*^000000",
            "Still, it's pretty",
            "depressing to think",
            "that the beauty of nature",
            "can be man-made and",
            "equated to zeny, you know?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn victor_perfecto_zen9(ctx: &Ctx) -> Script {
    victor_perfecto_zen9_body(ctx, Vec::new()).map(|_| ())
}

fn vergil_zen4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Vergil",
        args![
            "The weather is so",
            "nice today, like always.",
            "I just want to ditch work,",
            "run outside and work out."
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(
        ctx,
        &[Val::from(
            "Where do you want to go?:But shouldn't you go to work?:Have you heard about the serial killer?",
        )],
    )? {
        1 => {
            ctx.lines_as(
                "Vergil",
                args![
                    "Well, those guys in",
                    "black suits, not to mention",
                    "the ruffians that manage to",
                    "invade town, sometimes sort",
                    "of make it kind of unsafe to",
                    "go out all by yourself."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Vergil",
                args![
                    "But me and a buddy are",
                    "planning to head to the",
                    "Al De Baran Turbo Track",
                    "one of these days. Boy,",
                    "the last time we went,",
                    "he wasted a lot of zeny."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Vergil",
                args![
                    "He actually won some",
                    "kind of potion as a prize",
                    "and let me have it. I drank",
                    "it and it made me move really",
                    "slowly. Now what kind of prize",
                    "is that supposed to be?!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Vergil",
                args![
                    "I think my buddy",
                    "was totally scammed.",
                    "That, or I was totally",
                    "tricked by him to drink it.",
                    "How they came up with such a ridiculous idea for a prize!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Vergil",
                args![
                    "Wha--?! I didn't say",
                    "I was going to ditch work,",
                    "I just said I wanted to!",
                    "But just to spite you, maybe",
                    "just maybe, I won't go today!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Vergil",
                args![
                    "Then again, I don't",
                    "think I can slack on",
                    "this project. Curses...",
                    "The weekend certainly",
                    "doesn't come fast enough!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            ctx.lines_as(
                "Vergil",
                args![
                    "What, you mean that",
                    "axe murderer from a long",
                    "time ago? Well, I heard a",
                    "rumor that it actually wasn't",
                    "like that. Let's see, how",
                    "did the story go?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Vergil",
                args![
                    "I heard that some hat",
                    "maker, the one who makes",
                    "the Smokie Hat, accidentally",
                    "made a Person Headgear",
                    "instead of, like, a hat made",
                    "of monsters. Don't ask me how."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Vergil",
                args![
                    "Yeah, I think relatives",
                    "of that Airship Captain...",
                    "They were totally made into",
                    "a hat on accident. Supposedly,",
                    "it looks like a Reindeer Head,",
                    "but now that's just too weird."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn vergil_zen4(ctx: &Ctx) -> Script {
    vergil_zen4_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum JorjeZeroStep {
    Start,
    OnTouch,
}

fn jorje_zero_run(ctx: &Ctx, mut step: JorjeZeroStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            JorjeZeroStep::Start => {
                step = JorjeZeroStep::OnTouch;
                continue 'machine;
            }
            JorjeZeroStep::OnTouch => {
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                if subject1 == 1 {
                    ctx.lines_as(
                        "Jorje",
                        args![
                            "Arrrgh, I don't",
                            "have any time for",
                            "talking! I'm in the",
                            "middle of an important",
                            "task! H-hold on a second!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject1 == 2 {
                    ctx.lines_as(
                        "Jorje",
                        args![
                            "D-don't come any",
                            "closer! Anyone who",
                            "comes near me might",
                            "just screw me up! Back off!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject1 == 3 {
                    ctx.lines_as(
                        "Jorje",
                        args![
                            "Oh man...",
                            "I've been working so",
                            "hard and haven't taken",
                            "any breaks. I think I'll",
                            "reward myself and buy",
                            "something like maybe--"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jorje",
                        args![
                            "No! No, I'm not",
                            "gonna buy anything!",
                            "I've got my future wife",
                            "to think about! Must...",
                            "Save... More... Money!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn jorje_zero(ctx: &Ctx) -> Script {
    jorje_zero_run(ctx, JorjeZeroStep::Start, Vec::new()).map(|_| ())
}

pub fn jorje_zero_ontouch(ctx: &Ctx) -> Script {
    jorje_zero_run(ctx, JorjeZeroStep::OnTouch, Vec::new()).map(|_| ())
}

fn leimi_mimir_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Leimi", args!["...", "......"])?;
    ctx.next()?;
    ctx.lines_as("Leimi", args!["Oh...!", "Good heavens!", "Um, may I help you?"])?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
    if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?) {
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
            ctx.next()?;
            ctx.lines_as(
                "Leimi",
                args![
                    "Oh, you're an Assassin!",
                    "Oh, you boys are soooo cute!",
                    "And so cool and so mysterious all at the same time! I love you!"
                ],
            )?;
        } else {
            ctx.next()?;
            ctx.lines_as(
                "Leimi",
                args![
                    "An Assassin...?",
                    "Oh, you wouldn't happen",
                    "to know any Assassin boys",
                    "that might be single, do you?",
                    "Oh-my-god, they're hunky-hot~"
                ],
            )?;
        }
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn leimi_mimir(ctx: &Ctx) -> Script {
    leimi_mimir_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MimirCameraStep {
    Start,
    OnTouch,
}

fn mimir_camera_run(ctx: &Ctx, mut step: MimirCameraStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MimirCameraStep::Start => {
                step = MimirCameraStep::OnTouch;
                continue 'machine;
            }
            MimirCameraStep::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                    && ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?)
                {
                    ctx.mes("^3355FF*Click*^000000")?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["What the...?", "That sound. Did...", "Did someone just", "take my picture?"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn mimir_camera(ctx: &Ctx) -> Script {
    mimir_camera_run(ctx, MimirCameraStep::Start, Vec::new()).map(|_| ())
}

pub fn mimir_camera_ontouch(ctx: &Ctx) -> Script {
    mimir_camera_run(ctx, MimirCameraStep::OnTouch, Vec::new()).map(|_| ())
}

fn cool_event_staff_saera_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Saera",
        args![
            "Welcome to the",
            "temporary headquarters",
            "of Cool Event Corporation.",
            "How may I help you today?"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Temporary headquarters?:Voting:No, thanks.")])? {
        1 => {
            ctx.lines_as(
                "Saera",
                args![
                    "Our headquarters building",
                    "is currently undergoing",
                    "reconstruction, so we are",
                    "basing our operations in",
                    "this place for the meantime."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            if ctx.var("lhz_boss").get()?.number()? < 17 {
                ctx.lines_as(
                    "Saera",
                    args![
                        "Currently, Kafra Corporation",
                        "and Cool Event Corp are working",
                        "on a collaborative program that",
                        "will provide direct teleport",
                        "services to dungeons."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Saera",
                    args![
                        "Due to technical issues,",
                        "both companies cannot provide",
                        "teleport services to the same",
                        "dungeon. Therefore, we will be",
                        "selecting our valued customers to choose the company they want."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Saera",
                    args![
                        "Only a limited number of",
                        "voters will be chosen, so",
                        "you can check your voting",
                        "eligibility at the headquarters",
                        "of both participating companies. Thank you for your patronage~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Saera",
                    args![
                        "Currently, Kafra Corporation",
                        "and Cool Event Corp are working",
                        "on a collaborative program that",
                        "will provide direct teleport",
                        "services to dungeons."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Saera",
                    args![
                        "Due to technical issues,",
                        "both companies cannot provide",
                        "teleport services to the same",
                        "dungeon. Therefore, we will be",
                        "selecting a number of valued customers to vote for their choice."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Saera",
                    args![
                        "I've just reviewed your",
                        "information and would like",
                        "to inform you that you are",
                        "indeed eligible to vote.",
                        "Your participation in this",
                        "election is much appreciated."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Saera",
                    args![
                        "Remember that the",
                        "election polls can be",
                        "found in either Prontera",
                        "or Juno. Thank you very much."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        3 => {
            ctx.lines_as("Saera", args!["Thank you.", "Have a good day."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn cool_event_staff_saera(ctx: &Ctx) -> Script {
    cool_event_staff_saera_body(ctx, Vec::new()).map(|_| ())
}

fn event_planner_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Jellarin",
        args![
            "I don't like this.",
            "But I don't like that",
            "idea either. What will",
            "I do for a new event, eh?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jellarin",
        args![
            "I need something",
            "major, something that'll",
            "really shake up the world,",
            "something epochal, but what?",
            "Hey, do you have any ideas?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn event_planner(ctx: &Ctx) -> Script {
    event_planner_body(ctx, Vec::new()).map(|_| ())
}

fn cool_event_manager_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Baoto",
        args![
            "Hmmm...",
            "The employees seem",
            "to be having too much",
            "fun amongst themselves",
            "recently. This does not",
            "bode well at all..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Baoto",
        args![
            "It looks like I'm",
            "just going to have to",
            "start cracking that whip",
            "more often and much",
            "harder. Ha ha ha ha!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn cool_event_manager(ctx: &Ctx) -> Script {
    cool_event_manager_body(ctx, Vec::new()).map(|_| ())
}

fn cool_event_staff_cesuna_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Cesuna",
        args![
            "Ack! I'm totally",
            "swamped with all this",
            "work! But I don't wanna",
            "do any of it. That's it!",
            "I totally need a break."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Cesuna",
        args![
            "^333333*Sigh...*^000000",
            "I wonder if Saera",
            "would ever consider",
            "going out with me?",
            "That would be nice~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn cool_event_staff_cesuna(ctx: &Ctx) -> Script {
    cool_event_staff_cesuna_body(ctx, Vec::new()).map(|_| ())
}

fn rekenber_employee_li_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_tre").get()?.number()? > 54 {
        ctx.lines_as(
            "Rekenber Employee",
            args![
                "Greetings. As part of our",
                "effort to relieve the poor,",
                "Rekenber is providing job",
                "opportunities targeted for",
                "citizens of the slum areas."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rekenber Employee",
            args![
                "You can choose to work",
                "from home, or undergo a",
                "little bit of training for more",
                "professional positions. This",
                "is a great chance to make a",
                "difference... and some money~"
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn rekenber_employee_li_2(ctx: &Ctx) -> Script {
    rekenber_employee_li_2_body(ctx, Vec::new()).map(|_| ())
}

fn scientist_li_01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::IsEquipped, vec![Val::from(2241)])?.is_true()
        && ctx.call(Function::IsEquipped, vec![Val::from(2243)])?.is_true())
    {
        if ctx.var("hg_tre").get()?.number()? > 54 {
            ctx.lines_as("A Scientist", args!["What happened? All the machines are ruined and the research report are gone! The history of Regenschirm has been hacked!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Scientist",
                args![
                    "It takes so long for",
                    "this device to process",
                    "all the data and give me",
                    "the results. Still, the wait",
                    "heightens my anticipation..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Scientist",
            args!["What?! Guards!", "Hurry, there's an", "intruder right here!"],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(33), Val::from(224)])?;
        return Err(Stop::End);
    }
}

pub fn scientist_li_01(ctx: &Ctx) -> Script {
    scientist_li_01_body(ctx, Vec::new()).map(|_| ())
}
