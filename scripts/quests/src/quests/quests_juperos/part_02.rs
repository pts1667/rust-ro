use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn bundle_of_files_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^8B6914*Thesis: The Fall of Juperos*",
        "By Fayruz Khrhiyha",
        "Sage Castle Researcher^000000"
    ])?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Table of Contents.:Leave it alone.")])?);
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            if ctx.var("yuno_hist").get()?.number()? > 4 {
                ctx.lines(args![
                    "^8B6914 1. Preface",
                    " 2. Juperos Background",
                    " 3. Theory Behind Its Fall^000000"
                ])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("Preface:Juperos Background:Theory Behind Its Fall:Leave it alone.")],
                )? {
                    1 => {
                        ctx.lines(args![
                            "^8B6914Scholars are certain",
                            "that the Juperos civilization",
                            "used to be located above the",
                            "ground, but it is now buried",
                            "beneath the El Mes Plateau.",
                            "The reasons for the city's"
                        ])?;
                        if ctx.var("yuno_hist").get()?.number()? < 9 {
                            ctx.mes("ruin are still nebulous...^000000")?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^8B6914There is much speculation",
                                "about the reasons for Jupero's",
                                "downfall, but any documentation",
                                "from that time period has been",
                                "difficult to find. As for now, any evidence we have regarding",
                                "Juperos is inconclusive.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^8B6914In spite of this lack",
                                "of empirical or concrete",
                                "data on the civilization of",
                                "Juperos, our modern world",
                                "may be able to learn much",
                                "from that ancient city's ruins.",
                                "..................^000000"
                            ])?;
                            if ctx.var("yuno_hist").get()?.number()? < 7 {
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Hmmm...",
                                        "A whole chapter",
                                        "dedicated to saying,",
                                        "''We know absolutely",
                                        "nothing about something.''",
                                        "I should write a book~"
                                    ],
                                )?;
                            }
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.mes("ruin are still in debate...^000000")?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^8B6914However, new findings",
                            "regarding the history",
                            "of Juperos have allowed",
                            "us to make a few conclusions.^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        if ctx.var("yuno_hist").get()?.number()? < 9 {
                            ctx.lines(args!["^8B6914...", "......", "..........^000000"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "This is all just idle",
                                    "conjecture! This paper",
                                    "isn't developed enough",
                                    "yet to be a real thesis..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("yuno_hist").get()? == 9 {
                            ctx.lines(args![
                                "^8B6914Juperos was built over",
                                "a thousand years ago in",
                                "an era of peace just after",
                                "a major war. Contrary to",
                                "popular belief, there isn't any",
                                "evidence proving that Juno may",
                                "have descended from Juperos.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^8B6914In fact, there is a",
                                "strong possibility that",
                                "another war, between Juno",
                                "and Juperos, resulted in Juno's",
                                "independence from Juperos and",
                                "the destruction of any existing",
                                "documentation from that era.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Yes... Of course!", "It all makes sense now!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines(args![
                            "^8B6914Juperos was built over",
                            "a thousand years ago in",
                            "an era of peace just after",
                            "a major war. There is now",
                            "direct evidence linking Juno",
                            "to Juperos proving that Juno",
                            "was but a part of Juperos.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^8B6914Just like Juno, Juperos",
                            "was a society that prided",
                            "itself on its advancement",
                            "in the sciences which played",
                            "a permeating role in civilized",
                            "life. Science was reponsible for Juperos's rise and downfall."
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        if ctx.var("yuno_hist").get()?.number()? < 10 {
                            ctx.lines(args!["^8B6914...", "......", "..........^000000"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Huh. This writer keeps",
                                    "talking about theories,",
                                    "but the more I read, the",
                                    "less clear I am on what",
                                    "the theory actually is.",
                                    "I don't think there is one..."
                                ],
                            )?;
                            ctx.call(
                                Function::Emotion,
                                vec![
                                    ctx.constant("ET_SWEAT")?,
                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines(args![
                            "^8B6914What is most unsettling",
                            "is recent evidence, including",
                            "a first hand written account,",
                            "regarding the role of one of",
                            "Jupero's foremost scientists",
                            "in that city's rise and fall."
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^8B6914If these writings are",
                            "authentic, then what actually",
                            "happened was that a scientific",
                            "revolution occurred as a direct",
                            "result of one scientist's effort to manipulate the energies of the",
                            "artifact known as Ymir's heart."
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^8B6914This one man and Ymir's",
                            "Heart are credited with",
                            "the success and prosperity",
                            "of the Jupero's civilization.",
                            "However, there are various accounts prior to Jupero's fall",
                            "detailing his work with chimera...^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^8B6914This scientist, supposedly",
                            "in his passion to benefit his",
                            "people by finding a scientific",
                            "method for immortality by using",
                            "chimera for testing, was driven",
                            "insane. He experimented on",
                            "himself with disatrous results.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^8B6914There was an error in the",
                            "energy calibration of Ymir's",
                            "Heart and the scientist was",
                            "transformed into the monster",
                            "we now know as Chimera. He",
                            "and his test subjects were set",
                            "loose into the city of Juperos.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^8B6914These immortal Chimeras",
                            "razed the entire city, killing",
                            "countless people. Apparently,",
                            "a team of scientists were able",
                            "to salvage a fragment of Ymir's",
                            "Heart, and use it to lauch part",
                            "of Juperos into the sky.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^8B6914That section of Juperos",
                            "eventually developed into",
                            "the city of Juno. Since the",
                            "scientists who launched Juno into the sky all immediately died",
                            "afterwards from an unknown cause, they left no documentation.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^8B6914The Chimera, and the",
                            "laboratory in which it",
                            "was created, is rumored to",
                            "remain beneath the ruins of",
                            "the once great city of Juperos.^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    4 => {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Ugh... My head hurts",
                                "too much from reading",
                                "a book for smarty people.",
                                "I know! I'll play videogames!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            match runtime::select_values(ctx, &[Val::from("Preface:Close the file.")])? {
                1 => {
                    ctx.lines(args![
                        "^8B6914Scholars are certain",
                        "that the Juperos civilization",
                        "used to be located above the",
                        "ground, but it is now buried",
                        "beneath the El Mes Plateau.",
                        "The reasons for the city's",
                        "ruin are still nebulous...^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^8B6914There is much speculation",
                        "about the reasons for Jupero's",
                        "downfall, but any documentation",
                        "from that time period has been",
                        "difficult to find. As for now, any evidence we have regarding",
                        "Juperos is inconclusive.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^8B6914In spite of this lack",
                        "of empirical or concrete",
                        "data on the civilization of",
                        "Juperos, our modern world",
                        "may be able to learn much",
                        "from that ancient city's ruins.",
                        "..................^000000"
                    ])?;
                    if !(ctx.var("yuno_hist").get()?.is_true()) {
                        ctx.var("yuno_hist").set(Val::from(1))?;
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "This...",
                            "This is supposed to",
                            "be a research thesis?",
                            "There's barely any",
                            "research in it..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Hmm...",
                            "This isn't heavy",
                            "enough to be a real",
                            "academic work. It must",
                            "not even be finished yet."
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
            if ctx.var("yuno_hist").get()?.number()? < 5 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Nah...", "I'm tired of reading.", "Ironically enough."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("yuno_hist").get()?.number()? < 7 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["I don't feel like", "reading this. Not", "enough pictures..."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("yuno_hist").get()?.number()? < 9 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["It looks very sophisticated..."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("yuno_hist").get()?.number()? < 10 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["I know this book...", "But I don't feel like", "reading it right now."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "This thesis is",
                        "looking pretty good~",
                        "Of course, I did have",
                        "a hand in making it..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn bundle_of_files(ctx: &Ctx) -> Script {
    bundle_of_files_body(ctx, Vec::new()).map(|_| ())
}

fn book_juperos_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("yuno_hist").get()?.number()? > 7 {
        ctx.lines(args![
            "^8B6914*Self-Honesty*",
            "*'Benefits Fo' Life!'*",
            "By Stephen Oyoung",
            " ",
            " ",
            "Publisher:",
            "Wushu Publishing, Co.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["W-whoa!", "This book wasn't", "here before! It looks", "pretty interesting..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args![
            "You know what?",
            "I think I'll just flip",
            "through some pages",
            "from a random book.",
            "Hmmm, let's see..."
        ],
    )?;
    ctx.next()?;
    ctx.lines(args![
        "^8B6914''Admiral, the Kylorians are",
        "still advancing!'' Commander",
        "McKenrick announced without",
        "his usual swagger. ''They're...",
        "They're not stopping!'' But",
        "Admiral Leh's eyes were a cold,",
        "unfeeling shade of sternness.^000000"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "^8B6914''Let the goddamned space",
        "aliens come,'' hissed Leh.",
        "''We don't stand a chance",
        "without the Zenoi Sword",
        "to summon the power of",
        "GOD-POING. It's... It's",
        "all over. Damn it all...''^000000"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "^8B6914The doors burst open as",
        "Bucky flew into the room.",
        "''The Zenoi Sword! The",
        "Zenoi Sword! Someone's",
        "found it!'' the boy yelled.",
        "''Really?! We better hurry:",
        "Earth doesn't have much time!''^000000"
    ])?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args![
            "...",
            "......",
            "Whoa. I really",
            "should have read",
            "this masterpiece",
            "from the beginning..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn book_juperos(ctx: &Ctx) -> Script {
    book_juperos_body(ctx, Vec::new()).map(|_| ())
}

fn bronze_statue_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("yuno_hist").get()?.number()? > 3 {
        ctx.lines(args![
            "^3355FF''Do you wish to see",
            "the end of the madness?",
            "He is waiting where the three",
            "columns were destroyed, where",
            "two hundred illusions wander.''^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("yuno_hist").get()? == 3 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "I better go and",
                "see Fayruz in the",
                "Juno Library and tell",
                "her about the inscription."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFThere's a peculiar",
            "engraving on the",
            "Bronze Statue's rod.^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Investigate:Ignore it")])? {
            1 => {
                ctx.lines(args![
                    "^3355FFIt's an inscription that's",
                    "written in an old language",
                    "that you can't understand,",
                    "but have no problem reading",
                    "and making out the sounds",
                    "for some weird reason.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "This is creepy!",
                        "I know that I'm not",
                        "supposed to be able",
                        "to read this, but here",
                        "I am. I know what sounds",
                        "all of these letters make..."
                    ],
                )?;
                ctx.next()?;
                if ctx.var("yuno_hist").get()? == 2 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Then again, Fayruz did",
                            "say this was enchanted.",
                            "Okay, I guess I'll go back",
                            "to the Juno Library and",
                            "tell her what I found."
                        ],
                    )?;
                    ctx.var("yuno_hist").set(Val::from(3))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(11017), Val::from(11018)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Huh. Only a scholar,",
                        "maybe someone even in",
                        "Juno, could make sense",
                        "of what this stuff says."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn bronze_statue_1(ctx: &Ctx) -> Script {
    bronze_statue_1_body(ctx, Vec::new()).map(|_| ())
}

fn ambitious_hollgrehenn_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "*Hollgrehenn: The Ambition*",
        "By Aragham Caul*",
        " ",
        " ",
        " ",
        " ",
        "Publisher:",
        "Muha Books, Co."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "...",
        "He would stop at nothing",
        "to have the greatest weapon",
        "in the world in his possession.",
        "He became a smith so that he",
        "could discern which weapons",
        "were the most powerful..."
    ])?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["That's strange...", "The next page", "has been torn out."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn ambitious_hollgrehenn(ctx: &Ctx) -> Script {
    ambitious_hollgrehenn_body(ctx, Vec::new()).map(|_| ())
}

fn penniless_hollgrehenn_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^8B6914*Penniless Hollgrehenn*",
        " * Pennyless Hollgrehenn * ",
        "By Hollgrehenn",
        " ",
        " ",
        "Publisher:",
        "Muha Books, Co.^000000"
    ])?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args![
            "This book looks",
            "like a total piece of",
            "crap. I'd have more",
            "fun getting my teeth",
            "drilled by a blindfolded",
            "dentist. Or would I...?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn penniless_hollgrehenn(ctx: &Ctx) -> Script {
    penniless_hollgrehenn_body(ctx, Vec::new()).map(|_| ())
}

fn popular_feasts_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^8B6914*Popular Feasts*",
        "By Cabbage Pickle Community",
        " ",
        " ",
        " ",
        "Publisher:",
        "Muha Books, Co.^000000"
    ])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from(" 1. Fried Yoyo Tails: 14. Poring Salad: 252. Beak Soup")])? {
        1 => {
            ctx.lines(args![
                "^8B6914...",
                "If possible, try",
                "to use tails cut",
                "from live Yoyos.",
                "Now, as for skinning...^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Um...", "Barf?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Hey, this page is gone!",
                    "I guess Poring Salad is",
                    "the most popular feast",
                    "in this entire book."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            ctx.lines(args![
                "^8B6914...",
                "Fry the cut beaks",
                "using herbal oil until",
                "crisp. Then, pour the",
                "feathers into a blender...^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Hmm...",
                    "Sounds a little",
                    "too gourmet for my",
                    "taste. And by ''gourmet,''",
                    "I mean, ''totally gross.''"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn popular_feasts(ctx: &Ctx) -> Script {
    popular_feasts_body(ctx, Vec::new()).map(|_| ())
}

fn hamerun_rat_hunter_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["I can't...", "reach it...!"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hamerun_rat_hunter(ctx: &Ctx) -> Script {
    hamerun_rat_hunter_body(ctx, Vec::new()).map(|_| ())
}

fn red_book_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args!["^3355FFYou find a book", "with red binding.^000000"])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Read.:Leave it alone.")])? {
        1 => {
            ctx.mes("^8B6914...^000000")?;
            ctx.next()?;
            ctx.lines(args!["^8B6914...", "......^000000"])?;
            ctx.next()?;
            ctx.lines(args!["^8B6914...", "......", ".........^000000"])?;
            ctx.next()?;
            ctx.lines(args!["^8B6914...", "......", ".........", "............^000000"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["This is...", "A compilation of", "Shakespeare in ", "coloring book format?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Wait...", "Why would fans of", "Shakespeare even", "want a coloring book?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Hmm... Well...",
                    "I suppose little kids who",
                    "read Shakespeare would",
                    "appreciate something like that."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Hold on...",
                    "Do little kids who",
                    "are able to read the",
                    "works of Shakespeare",
                    "even exist? I hope not..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "What a strange",
                    "and mysterious book.",
                    "I'll never know what's",
                    "inside unless I read it!",
                    "Come on! Doesn't the red",
                    "binding mean something?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn red_book(ctx: &Ctx) -> Script {
    red_book_body(ctx, Vec::new()).map(|_| ())
}

fn scroll_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFYou've found an",
        "antiquated scroll",
        "that's collected a",
        "layer of fine dust.^000000"
    ])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Peruse:Leave it alone")])? {
        1 => {
            ctx.lines(args![
                "^8B6914Item Upgrade Introduction",
                " ",
                " ",
                " ",
                "1. Item Upgrade Definition",
                " ",
                "The key to success when",
                "upgrading items comes from",
                "only one place: Your ''Mind.''",
                " ",
                " ",
                " ",
                "2. Power of a Positive Attitude",
                "Before trying to upgrade",
                "an item, plan out how high",
                "you want to upgrade and how",
                "much you'll spend beforehand.",
                "But like all ladies, Lady Luck",
                "smiles when you fully splurge.^000000",
                " ",
                " "
            ])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Wait, wait...",
                    "This thing IS old.",
                    "I mean, it's obviously",
                    "written from a patriarchal",
                    "standpoint that promotes",
                    "bipartisan gender roles."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["I'm...", "I'm so offended."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "This scroll is far too",
                    "primitive. There's been",
                    "all sorts of technological",
                    "reading advances that I can't",
                    "live without... Like pages."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn scroll(ctx: &Ctx) -> Script {
    scroll_body(ctx, Vec::new()).map(|_| ())
}

fn paper_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFIt's a piece of",
        "paper that looks",
        "like a personal letter.^000000"
    ])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Read it.:Leave it alone.")])? {
        1 => {
            if ctx.var("yuno_hist").get()?.number()? > 7 {
                ctx.lines(args![
                    "^8B6914P.S.",
                    "Please...",
                    "Come back to me.",
                    " ",
                    " ",
                    "Love,",
                    "Fayruz^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines(args![
                "^8B6914...",
                "I can't forget your smile.",
                "No matter what, even if you",
                "hate me, I'll always have these",
                "feelings just for you. You are",
                "the one who is most special,",
                "who means the most to my heart.^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^8B6914I know we've had our",
                "differences, but please",
                "don't refuse my love. By",
                "your hands, I hope that",
                "you can forgive me for us.",
                "- Love, Fayruz^000000"
            ])?;
            ctx.next()?;
            if ctx.call(Function::GetPartnerId, vec![])?.is_true() {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Ahhhhhh~", "Love sure is nice!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["What th--?!", "Booooooooo!", "Love stinks!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["I guess I better", "not look at this.", "I mean, I might", "regret reading it."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn paper(ctx: &Ctx) -> Script {
    paper_body(ctx, Vec::new()).map(|_| ())
}

fn stone_statue_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("yuno_hist").get()?.is_true() {
        ctx.lines(args![
            "^3355FFIt's a stone statue",
            "that looks exactly like",
            "the one in the Juno Library.",
            "However, it has sculptures",
            "of books instead of real ones.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFIt's possible that the",
            "statue in the Juno Library",
            "was made after this one.",
            "But who can be sure?^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args!["^3355FFIt's an old", "statue sculpted", "out of stone.^000000"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn stone_statue(ctx: &Ctx) -> Script {
    stone_statue_body(ctx, Vec::new()).map(|_| ())
}

fn bronze_statue_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFIt's an elaborate",
        "bronze statue that",
        "is twice the height of",
        "a normal human being.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bronze_statue_2(ctx: &Ctx) -> Script {
    bronze_statue_2_body(ctx, Vec::new()).map(|_| ())
}

fn sculpture_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args!["^3355FFIt's a sculpture that", "looks familiar to you.^000000"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn sculpture(ctx: &Ctx) -> Script {
    sculpture_body(ctx, Vec::new()).map(|_| ())
}

fn machine_statue_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFThis is the bust of a",
        "humanoid machine with",
        "a familiar Crest Piece",
        "carved into the middle.^000000"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "^3355FFThe statue's entire",
        "form is mind boggling,",
        "but you manage to note",
        "that its outstretched",
        "arm points westward.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn machine_statue(ctx: &Ctx) -> Script {
    machine_statue_body(ctx, Vec::new()).map(|_| ())
}

fn jupe_goto_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn jupe_goto_1(ctx: &Ctx) -> Script {
    jupe_goto_1_body(ctx, Vec::new()).map(|_| ())
}

fn jupe_goto_1_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![Val::from("jupe_goto#1"), Val::from(1)])?;
    ctx.lines(args![
        ((Val::from("^777777[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]^000000")),
        "This light...",
        "It feels like...",
        "Its warmth is...",
        "Wrapping all over me..."
    ])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Ah, it's so nice...:No! This is wrong!")])? {
        1 => {
            ctx.lines(args![
                ((Val::from("^777777[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]^000000")),
                "Ahhhh...",
                "It feels like",
                "I'm floating..."
            ])?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_LIGHTSPHERE")?])?;
            ctx.close_window()?;
            ctx.call(Function::StopNpcTimer, vec![])?;
            ctx.call(Function::Warp, vec![Val::from("juperos_02"), Val::from(128), Val::from(278)])?;
        }
        2 => {
            ctx.lines(args![
                ((Val::from("^777777[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]^000000")),
                "N-No! This is",
                "wrong! Something",
                "weird's happening!",
                "I gotta get away!"
            ])?;
            ctx.close_window()?;
            ctx.call(Function::StopNpcTimer, vec![])?;
            ctx.call(Function::Warp, vec![Val::from("juperos_01"), Val::from(96), Val::from(91)])?;
        }
        _ => {}
    }
    return Err(Stop::End);
}

pub fn jupe_goto_1_ontouch(ctx: &Ctx) -> Script {
    jupe_goto_1_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn jupe_goto_1_ontimer10000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::Warp, vec![Val::from("juperos_02"), Val::from(128), Val::from(278)])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("jupe_goto#2::OnEnable")])?;
    return Err(Stop::End);
}

pub fn jupe_goto_1_ontimer10000(ctx: &Ctx) -> Script {
    jupe_goto_1_ontimer10000_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum JupeGoto2Step {
    Start,
    OnInit,
    OnEnable,
    OnTouch,
    OnTimer2000,
}

fn jupe_goto_2_run(ctx: &Ctx, mut step: JupeGoto2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            JupeGoto2Step::Start => {
                step = JupeGoto2Step::OnInit;
                continue 'machine;
            }
            JupeGoto2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("jupe_goto#2")])?;
                return Err(Stop::End);
            }
            JupeGoto2Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("jupe_goto#2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("jupe_goto#1")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            JupeGoto2Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("juperos_02"), Val::from(128), Val::from(278)])?;
                return Err(Stop::End);
            }
            JupeGoto2Step::OnTimer2000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("jupe_goto#1")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("jupe_goto#2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn jupe_goto_2(ctx: &Ctx) -> Script {
    jupe_goto_2_run(ctx, JupeGoto2Step::Start, Vec::new()).map(|_| ())
}

pub fn jupe_goto_2_oninit(ctx: &Ctx) -> Script {
    jupe_goto_2_run(ctx, JupeGoto2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn jupe_goto_2_onenable(ctx: &Ctx) -> Script {
    jupe_goto_2_run(ctx, JupeGoto2Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn jupe_goto_2_ontouch(ctx: &Ctx) -> Script {
    jupe_goto_2_run(ctx, JupeGoto2Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn jupe_goto_2_ontimer2000(ctx: &Ctx) -> Script {
    jupe_goto_2_run(ctx, JupeGoto2Step::OnTimer2000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum S3fGateSwitchJupeStep {
    Start,
    OnInit,
    OnReset,
    OnEnable,
    OnTimer5000,
    OnMyMobDead,
}

fn s_3f_gate_switch_jupe_run(ctx: &Ctx, mut step: S3fGateSwitchJupeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            S3fGateSwitchJupeStep::Start => {
                step = S3fGateSwitchJupeStep::OnInit;
                continue 'machine;
            }
            S3fGateSwitchJupeStep::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            S3fGateSwitchJupeStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("juperos_02"), Val::from("3F Gate Switch#jupe::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            S3fGateSwitchJupeStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("3F Gate Switch#jupe")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            S3fGateSwitchJupeStep::OnTimer5000 => {
                ctx.var(".mymobs").set(Val::from(3))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("juperos_02"),
                        Val::from(24),
                        Val::from(275),
                        Val::from("1st Gate Switch"),
                        Val::from(1674),
                        Val::from(1),
                        Val::from("3F Gate Switch#jupe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("juperos_02"),
                        Val::from(240),
                        Val::from(29),
                        Val::from("2nd Gate Switch"),
                        Val::from(1674),
                        Val::from(1),
                        Val::from("3F Gate Switch#jupe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("juperos_02"),
                        Val::from(282),
                        Val::from(183),
                        Val::from("3rd Gate Switch"),
                        Val::from(1674),
                        Val::from(1),
                        Val::from("3F Gate Switch#jupe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            S3fGateSwitchJupeStep::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()? == 2 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("juperos_02"),
                            Val::from("Who are you to come here?"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xFF0000"),
                        ],
                    )?;
                } else if ctx.var(".mymobs").get()? == 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("juperos_02"),
                            Val::from("Have you come seeking Juperos?! It no longer exists..."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xFF0000"),
                        ],
                    )?;
                } else if ctx.var(".mymobs").get()? == 0 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("juperos_02"),
                            Val::from("Have you come to see me? Fine! Find me first!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xFF0000"),
                        ],
                    )?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SCREEN_QUAKE")?])?;
                    ctx.call(Function::SoundEffectAll, vec![Val::from("earth_quake.wav"), Val::from(0)])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("3F Gate Switch#jupe")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Restricted Area#jupe::OnEnable")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn s_3f_gate_switch_jupe(ctx: &Ctx) -> Script {
    s_3f_gate_switch_jupe_run(ctx, S3fGateSwitchJupeStep::Start, Vec::new()).map(|_| ())
}

pub fn s_3f_gate_switch_jupe_oninit(ctx: &Ctx) -> Script {
    s_3f_gate_switch_jupe_run(ctx, S3fGateSwitchJupeStep::OnInit, Vec::new()).map(|_| ())
}

pub fn s_3f_gate_switch_jupe_onreset(ctx: &Ctx) -> Script {
    s_3f_gate_switch_jupe_run(ctx, S3fGateSwitchJupeStep::OnReset, Vec::new()).map(|_| ())
}

pub fn s_3f_gate_switch_jupe_onenable(ctx: &Ctx) -> Script {
    s_3f_gate_switch_jupe_run(ctx, S3fGateSwitchJupeStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn s_3f_gate_switch_jupe_ontimer5000(ctx: &Ctx) -> Script {
    s_3f_gate_switch_jupe_run(ctx, S3fGateSwitchJupeStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn s_3f_gate_switch_jupe_onmymobdead(ctx: &Ctx) -> Script {
    s_3f_gate_switch_jupe_run(ctx, S3fGateSwitchJupeStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::Start, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_oninit(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnInit, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_onenable(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer5000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer7000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer9000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer9000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer9001(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer9001, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer23000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer23000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer46000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer46000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer69000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer69000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer92000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer92000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer115000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer115000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer161000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer161000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer184000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer184000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer207000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer207000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer230000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer230000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer253000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer253000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer276000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer276000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer299000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer299000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer322000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer322000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer345000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer345000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer368000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer368000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer391000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer391000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer414000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer414000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer460000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer460000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer483000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer483000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer506000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer506000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer529000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer529000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer552000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer552000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer556000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer556000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer561000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer561000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer598000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer598000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer600000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer600000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer603000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer603000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer621000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer621000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontimer1200000(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTimer1200000, Vec::new()).map(|_| ())
}

pub fn restricted_area_jupe_ontouch(ctx: &Ctx) -> Script {
    restricted_area_jupe_run(ctx, RestrictedAreaJupeStep::OnTouch, Vec::new()).map(|_| ())
}

fn hole_1_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("1"), Val::from(2)])?;
    if ctx.var("$@juprearea1inuse").get()? == 1 {
        ctx.lines(args![
            "^3355FFThis seems like",
            "some kind of device",
            "that will allow you to",
            "pass to the other side.",
            "There's a slot where you",
            "probably need to insert",
            "some kind of object...^000000"
        ])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("1"), Val::from(255)])?;
        return Err(Stop::End);
    } else if (((ctx.call(Function::CountItem, vec![Val::from(7356)])?.number()? > 0
        || ctx.call(Function::CountItem, vec![Val::from(7359)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7357)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7358)])?.number()? > 0)
    {
        ctx.lines(args![
            "^3355FFThis seems like",
            "some kind of device",
            "that will allow you to",
            "pass to the other side.",
            "There's a slot where you",
            "probably need to insert",
            "some kind of object...^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Insert a Crest Piece.:Cancel.")])? {
            1 => {
                if ctx.call(Function::CountItem, vec![Val::from(7356)])?.number()? > 0 {
                    ctx.lines(args![
                        "^3355FFYou take out your",
                        "Crest Piece and place",
                        "it into the slot where it",
                        "happens to fit perfectly.^000000"
                    ])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_TOPRANK")?])?;
                    ctx.call(Function::Cutin, vec![Val::from("1-1"), Val::from(2)])?;
                    ctx.next()?;
                    if ctx.var("$@juprearea1inuse").get()? == 1 {
                        ctx.lines(args![
                            "^3355FFNothing happens.",
                            "Perhaps an alarm or",
                            "some other safety measure",
                            "was activated to keep the",
                            "Crest Piece from activating",
                            "this transportation device.",
                            "You retrieve the Crest Piece.^000000"
                        ])?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from("1-1"), Val::from(255)])?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines(args![
                            "^3355FFThe slot rotates and",
                            "the Crest Piece moves as",
                            "if it were turning a key. You",
                            "feel a weak tremor as a Warp",
                            "Portal to the other side is",
                            "activated. You then retrieve",
                            "your Crest Piece.^000000"
                        ])?;
                        ctx.call(Function::InitNpcTimer, vec![])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Warp#1-1::OnEnable")])?;
                        ctx.call(Function::EnableNpc, vec![Val::from("Red Alarm#1-1")])?;
                        ctx.call(Function::DisableNpc, vec![Val::from("#hole#1-1")])?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from("1-1"), Val::from(255)])?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.lines(args![
                        "^3355FFUnfortunately, you're",
                        "not carrying anything",
                        "that might be able to fit",
                        "into the slot and activate",
                        "this mechanical device.^000000"
                    ])?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from("1"), Val::from(255)])?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Hmmm...", "Do I have anything", "that might make this", "weird machine work?"],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from("1"), Val::from(255)])?;
                return Err(Stop::End);
            }
            _ => {}
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFThis seems like",
            "some kind of device",
            "that will allow you to",
            "pass to the other side.",
            "There's a slot where you",
            "probably need to insert",
            "some kind of object...^000000"
        ])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("1"), Val::from(255)])?;
        return Err(Stop::End);
    }
}

pub fn hole_1_1(ctx: &Ctx) -> Script {
    hole_1_1_body(ctx, Vec::new()).map(|_| ())
}

fn hole_1_1_onstop_timer_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn hole_1_1_onstop_timer(ctx: &Ctx) -> Script {
    hole_1_1_onstop_timer_body(ctx, Vec::new()).map(|_| ())
}

fn hole_1_1_ontimer22500_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::EnableNpc, vec![Val::from("#hole#1-1")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#1-1")])?;
    return Err(Stop::End);
}

pub fn hole_1_1_ontimer22500(ctx: &Ctx) -> Script {
    hole_1_1_ontimer22500_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Warp11Step {
    Start,
    OnInit,
    OnEnable,
    OnTouch,
    OnTimer22500,
}

fn warp_1_1_run(ctx: &Ctx, mut step: Warp11Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Warp11Step::Start => {
                step = Warp11Step::OnInit;
                continue 'machine;
            }
            Warp11Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#1-1")])?;
                return Err(Stop::End);
            }
            Warp11Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Warp#1-1")])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BIG_PORTAL")?])?;
                ctx.call(Function::SoundEffectAll, vec![Val::from("jupe_warp.wav"), Val::from(0)])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Warp11Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("jupe_area1"), Val::from(47), Val::from(259)])?;
                return Err(Stop::End);
            }
            Warp11Step::OnTimer22500 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#1-1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn warp_1_1(ctx: &Ctx) -> Script {
    warp_1_1_run(ctx, Warp11Step::Start, Vec::new()).map(|_| ())
}

pub fn warp_1_1_oninit(ctx: &Ctx) -> Script {
    warp_1_1_run(ctx, Warp11Step::OnInit, Vec::new()).map(|_| ())
}

pub fn warp_1_1_onenable(ctx: &Ctx) -> Script {
    warp_1_1_run(ctx, Warp11Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn warp_1_1_ontouch(ctx: &Ctx) -> Script {
    warp_1_1_run(ctx, Warp11Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn warp_1_1_ontimer22500(ctx: &Ctx) -> Script {
    warp_1_1_run(ctx, Warp11Step::OnTimer22500, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RedAlarm11Step {
    Start,
    OnInit,
    OnTouch,
}

fn red_alarm_1_1_run(ctx: &Ctx, mut step: RedAlarm11Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RedAlarm11Step::Start => {
                step = RedAlarm11Step::OnInit;
                continue 'machine;
            }
            RedAlarm11Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#1-1")])?;
                return Err(Stop::End);
            }
            RedAlarm11Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Red Alarm On#1-1::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm#1-1")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#hole#1-1::OnStop_Timer")])?;
                ctx.var("$@juprearea1inuse").set(Val::from(1))?;
                ctx.call(Function::DisableNpc, vec![Val::from("#hole#1-1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn red_alarm_1_1(ctx: &Ctx) -> Script {
    red_alarm_1_1_run(ctx, RedAlarm11Step::Start, Vec::new()).map(|_| ())
}

pub fn red_alarm_1_1_oninit(ctx: &Ctx) -> Script {
    red_alarm_1_1_run(ctx, RedAlarm11Step::OnInit, Vec::new()).map(|_| ())
}

pub fn red_alarm_1_1_ontouch(ctx: &Ctx) -> Script {
    red_alarm_1_1_run(ctx, RedAlarm11Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RedAlarmOn11Step {
    Start,
    OnInit,
    OnEnable,
    OnTimer1000,
    OnTimer3000,
    OnTimer5000,
    OnTimer7000,
    OnTimer8000,
}

fn red_alarm_on_1_1_run(ctx: &Ctx, mut step: RedAlarmOn11Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RedAlarmOn11Step::Start => {
                step = RedAlarmOn11Step::OnInit;
                continue 'machine;
            }
            RedAlarmOn11Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm On#1-1")])?;
                return Err(Stop::End);
            }
            RedAlarmOn11Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Red Alarm On#1-1")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RedAlarmOn11Step::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("Those of you who have come here..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn11Step::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("I do not intend to stop you."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.call(
                    Function::SoundEffectAll,
                    vec![Val::from("jupe_warning.wav"), Val::from(0), Val::from("jupe_area1")],
                )?;
                ctx.call(
                    Function::SoundEffectAll,
                    vec![Val::from("jupe_warning.wav"), Val::from(0), Val::from("jupe_area1")],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn11Step::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("But I assume you are prepared for a few obstacles..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster1#1-1::OnEnable")])?;
                return Err(Stop::End);
            }
            RedAlarmOn11Step::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("After all, you are venturing through a forbidden area!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RedAlarmOn11Step::OnTimer8000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Red Alarm On#1-1")])?;
                ctx.call(
                    Function::SoundEffectAll,
                    vec![Val::from("jupe_warning.wav"), Val::from(0), Val::from("jupe_area1")],
                )?;
                ctx.call(
                    Function::SoundEffectAll,
                    vec![Val::from("jupe_warning.wav"), Val::from(0), Val::from("jupe_area1")],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn red_alarm_on_1_1(ctx: &Ctx) -> Script {
    red_alarm_on_1_1_run(ctx, RedAlarmOn11Step::Start, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_1_oninit(ctx: &Ctx) -> Script {
    red_alarm_on_1_1_run(ctx, RedAlarmOn11Step::OnInit, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_1_onenable(ctx: &Ctx) -> Script {
    red_alarm_on_1_1_run(ctx, RedAlarmOn11Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_1_ontimer1000(ctx: &Ctx) -> Script {
    red_alarm_on_1_1_run(ctx, RedAlarmOn11Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_1_ontimer3000(ctx: &Ctx) -> Script {
    red_alarm_on_1_1_run(ctx, RedAlarmOn11Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_1_ontimer5000(ctx: &Ctx) -> Script {
    red_alarm_on_1_1_run(ctx, RedAlarmOn11Step::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_1_ontimer7000(ctx: &Ctx) -> Script {
    red_alarm_on_1_1_run(ctx, RedAlarmOn11Step::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn red_alarm_on_1_1_ontimer8000(ctx: &Ctx) -> Script {
    red_alarm_on_1_1_run(ctx, RedAlarmOn11Step::OnTimer8000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Monster111Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTimer300000,
    OnTimer300002,
    OnMyMobDead,
}

fn monster1_1_1_run(ctx: &Ctx, mut step: Monster111Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Monster111Step::Start => {
                step = Monster111Step::OnInit;
                continue 'machine;
            }
            Monster111Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster1#1-1")])?;
                return Err(Stop::End);
            }
            Monster111Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster1#1-1")])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("jupe_area1"), Val::from("Monster1#1-1::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Monster111Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Monster1#1-1")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.var(".mymobs").set(Val::from(8))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(30),
                        Val::from(263),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#1-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(30),
                        Val::from(262),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#1-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(30),
                        Val::from(261),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#1-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(30),
                        Val::from(260),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#1-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(30),
                        Val::from(259),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#1-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(30),
                        Val::from(258),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#1-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(30),
                        Val::from(257),
                        Val::from("Security Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Monster1#1-1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from(30),
                        Val::from(256),
                        Val::from("Security Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Monster1#1-1::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Monster111Step::OnTimer300000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_area1"),
                        Val::from("Do you realize this is a hallucination?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                ctx.var("$@juprearea1inuse").set(Val::from(0))?;
                return Err(Stop::End);
            }
            Monster111Step::OnTimer300002 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#hole#1-1")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster1#1-1::OnDisable")])?;
                return Err(Stop::End);
            }
            Monster111Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Monster2#1-1::OnEnable")])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Monster1#1-1")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster1_1_1(ctx: &Ctx) -> Script {
    monster1_1_1_run(ctx, Monster111Step::Start, Vec::new()).map(|_| ())
}

pub fn monster1_1_1_oninit(ctx: &Ctx) -> Script {
    monster1_1_1_run(ctx, Monster111Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster1_1_1_ondisable(ctx: &Ctx) -> Script {
    monster1_1_1_run(ctx, Monster111Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn monster1_1_1_onenable(ctx: &Ctx) -> Script {
    monster1_1_1_run(ctx, Monster111Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster1_1_1_ontimer300000(ctx: &Ctx) -> Script {
    monster1_1_1_run(ctx, Monster111Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn monster1_1_1_ontimer300002(ctx: &Ctx) -> Script {
    monster1_1_1_run(ctx, Monster111Step::OnTimer300002, Vec::new()).map(|_| ())
}

pub fn monster1_1_1_onmymobdead(ctx: &Ctx) -> Script {
    monster1_1_1_run(ctx, Monster111Step::OnMyMobDead, Vec::new()).map(|_| ())
}
