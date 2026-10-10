use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum HistorianPrt03Step {
    Start,
    SSong,
}

fn historian_prt03_run(ctx: &Ctx, mut step: HistorianPrt03Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_line_s = Val::from("");
    let mut l_total = Val::from(0);
    'machine: loop {
        match step {
            HistorianPrt03Step::Start => {
                if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
                    || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
                {
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
                if ctx.var("prt_curse").get()? == 8 {
                    ctx.lines_as(
                        "Historian Mondo",
                        args![
                            "Excuse me...",
                            "But if you don't have",
                            "anything important for",
                            "me, then would you",
                            "please leave? I have",
                            "research to attend to..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "I'm sorry, but Ms. Rodafrian",
                            "wanted me to ask you for the",
                            "lyrics of some song describing",
                            "the Rune-Midgarts Kingdom's",
                            "founding. Would you tell",
                            "them to me for her?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Mondo",
                        args![
                            "Rodafrian? Oh, she must",
                            "mean that old children's",
                            "song. How did it go now?",
                            "Ah, now I remember..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Mondo",
                        args![
                            "^FF0000*The great serpent*^000000",
                            "^FF0000*swallowed the sea.*^000000",
                            "^FF0000*The eagle of the rainbow*^000000",
                            "^FF0000*swallowed the serpent.*^000000",
                            "^FF0000*Then snake scales grew on*^000000",
                            "^FF0000*the eagle, and it slowly died.*^000000"
                        ],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("...........?")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Historian Mondo",
                        args![
                            "Yes, I know: the song",
                            "is rather morbid, despite",
                            "the fact that it's sung like",
                            "such a happy song. Sometimes",
                            "these little songs are supposed",
                            "to scare kids into doing good."
                        ],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Excuse me...")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Historian Mondo",
                        args!["Hm? You look confused.", "Did you have a question", "to ask me about the song?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Actually, I heard",
                            "a different version",
                            "of this song, so I was",
                            "wondering why the lyrics",
                            "would be different..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Mondo",
                        args![
                            "Really now?",
                            "Well then, let me",
                            "hear the version that",
                            "you've heard, starting",
                            "from the beginning."
                        ],
                    )?;
                    ctx.next()?;
                    historian_prt03_run(ctx, HistorianPrt03Step::SSong, vec![])?;
                } else if ctx.var("prt_curse").get()? == 9 {
                    ctx.lines_as(
                        "Historian Mondo",
                        args![
                            "Ah, would you tell me",
                            "the lyrics of the version of",
                            "the Rune-Midgarts Kingdom",
                            "founding myth song that",
                            "you happened to hear?"
                        ],
                    )?;
                    ctx.next()?;
                    historian_prt03_run(ctx, HistorianPrt03Step::SSong, vec![])?;
                } else if ctx.var("prt_curse").get()? == 10 {
                    ctx.lines_as(
                        "Historian Mondo",
                        args![
                            "Interesting...",
                            "The version you heard",
                            "from Karlomoff is different",
                            "than the one I know. It's",
                            "possible that his version",
                            "is the most authentic."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Mondo",
                        args![
                            "After all, I learned",
                            "the version I know",
                            "by listening to a",
                            "little kid sing it."
                        ],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("A kid?")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Historian Mondo",
                        args![
                            "Yeah. My guess? The version",
                            "that the kid sang might have",
                            "changed as it was transmitted",
                            "by mouth through the generations. If you want to learn more, you",
                            "should probably find that kid."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Mondo",
                        args![
                            "Anyway, Karlomoff may have",
                            "the most accurate version of",
                            "the song since he's very good",
                            "at procuring and authenticating",
                            "historical written records and",
                            "documents. It's his specialty."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Mondo",
                        args![
                            "Ah, if you want to learn more,",
                            "then you should try to find the",
                            "kid that was singing the song.",
                            "Hmm, he might be around ^3131DDthe",
                            "river near here^000000. Hopefully it",
                            "won't be too hard to find him."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Mondo",
                        args![
                            "Once you're able to talk",
                            "to that kid, go ahead and",
                            "report back to Rodafrian.",
                            "She probably needs to know",
                            "soon, and I can talk to that",
                            "little kid at my leisure, so..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Mondo",
                        args![
                            "Well then, I hope you",
                            "find what you're looking",
                            "for. Good luck in your",
                            "travels, adventurer."
                        ],
                    )?;
                    ctx.var("prt_curse").set(Val::from(11))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Historian Mondo",
                        args![
                            "Isn't this such",
                            "a beautiful place?",
                            "It's no wonder that",
                            "ancient peoples chose",
                            "to live around here..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = HistorianPrt03Step::SSong;
                continue 'machine;
            }
            HistorianPrt03Step::SSong => {
                ctx.lines(args![
                    ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
                ])?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_line_s = input;
                if l_line_s.clone() == "The great serpent swallowed the sea." {
                    l_total = (l_total.clone() + Val::from(1));
                    ctx.lines(args!["*The great serpent*", "*swallowed the sea.*"])?;
                } else {
                    ctx.lines(args![((Val::from("*") + l_line_s.clone()) + Val::from("*"))])?;
                }
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_line_s = input;
                if l_line_s.clone() == "The eagle of the rainbow swallowed the serpent." {
                    l_total = (l_total.clone() + Val::from(1));
                    ctx.lines(args!["*The eagle of the rainbow*", "*swallowed the serpent.*"])?;
                } else {
                    ctx.lines(args![((Val::from("*") + l_line_s.clone()) + Val::from("*"))])?;
                }
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_line_s = input;
                if l_line_s.clone() == "Then the eagle built its nest." {
                    l_total = (l_total.clone() + Val::from(1));
                }
                ctx.lines(args![((Val::from("*") + l_line_s.clone()) + Val::from("*"))])?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_line_s = input;
                if l_line_s.clone() == "A nest upon the swallowed sea." {
                    l_total = (l_total.clone() + Val::from(1));
                }
                ctx.lines(args![((Val::from("*") + l_line_s.clone()) + Val::from("*"))])?;
                if l_total.clone() == 4 {
                    ctx.var("prt_curse").set(Val::from(10))?;
                } else {
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Wait, wait...", "I think I messed up!", "(Those weren't the", "correct lyrics...)"],
                    )?;
                    ctx.var("prt_curse").set(Val::from(9))?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn historian_prt03(ctx: &Ctx) -> Script {
    historian_prt03_run(ctx, HistorianPrt03Step::Start, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum PrtPoem01Step {
    Start,
    OnTouch,
}

fn prt_poem01_run(ctx: &Ctx, mut step: PrtPoem01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PrtPoem01Step::Start => {
                step = PrtPoem01Step::OnTouch;
                continue 'machine;
            }
            PrtPoem01Step::OnTouch => {
                if ctx.var("prt_curse").get()? == 11 {
                    ctx.lines(args!["^FF0000*The great serpent*^000000", "^FF0000*swallowed the sea.*^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn prt_poem01(ctx: &Ctx) -> Script {
    prt_poem01_run(ctx, PrtPoem01Step::Start, Vec::new()).map(|_| ())
}

pub fn prt_poem01_ontouch(ctx: &Ctx) -> Script {
    prt_poem01_run(ctx, PrtPoem01Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum PrtPoem02Step {
    Start,
    OnTouch,
}

fn prt_poem02_run(ctx: &Ctx, mut step: PrtPoem02Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PrtPoem02Step::Start => {
                step = PrtPoem02Step::OnTouch;
                continue 'machine;
            }
            PrtPoem02Step::OnTouch => {
                if ctx.var("prt_curse").get()? == 11 {
                    ctx.lines(args!["^FF0000*The great serpent*^000000", "^FF0000*swallowed the sea.*^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn prt_poem02(ctx: &Ctx) -> Script {
    prt_poem02_run(ctx, PrtPoem02Step::Start, Vec::new()).map(|_| ())
}

pub fn prt_poem02_ontouch(ctx: &Ctx) -> Script {
    prt_poem02_run(ctx, PrtPoem02Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum PrtPoem03Step {
    Start,
    OnTouch,
}

fn prt_poem03_run(ctx: &Ctx, mut step: PrtPoem03Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PrtPoem03Step::Start => {
                step = PrtPoem03Step::OnTouch;
                continue 'machine;
            }
            PrtPoem03Step::OnTouch => {
                if ctx.var("prt_curse").get()? == 11 {
                    ctx.lines(args![
                        "^FF0000*The eagle of the rainbow*^000000",
                        "^FF0000*swallowed the serpent.*^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn prt_poem03(ctx: &Ctx) -> Script {
    prt_poem03_run(ctx, PrtPoem03Step::Start, Vec::new()).map(|_| ())
}

pub fn prt_poem03_ontouch(ctx: &Ctx) -> Script {
    prt_poem03_run(ctx, PrtPoem03Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum PrtPoem04Step {
    Start,
    OnTouch,
}

fn prt_poem04_run(ctx: &Ctx, mut step: PrtPoem04Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PrtPoem04Step::Start => {
                step = PrtPoem04Step::OnTouch;
                continue 'machine;
            }
            PrtPoem04Step::OnTouch => {
                if ctx.var("prt_curse").get()? == 11 {
                    ctx.lines(args![
                        "^FF0000*Then snake scales*^000000",
                        "^FF0000*grew on the eagle...*^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn prt_poem04(ctx: &Ctx) -> Script {
    prt_poem04_run(ctx, PrtPoem04Step::Start, Vec::new()).map(|_| ())
}

pub fn prt_poem04_ontouch(ctx: &Ctx) -> Script {
    prt_poem04_run(ctx, PrtPoem04Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum PrtPoem05Step {
    Start,
    OnTouch,
}

fn prt_poem05_run(ctx: &Ctx, mut step: PrtPoem05Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PrtPoem05Step::Start => {
                step = PrtPoem05Step::OnTouch;
                continue 'machine;
            }
            PrtPoem05Step::OnTouch => {
                if ctx.var("prt_curse").get()? == 11 {
                    ctx.mes("^FF0000*And it slowly died...*^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn prt_poem05(ctx: &Ctx) -> Script {
    prt_poem05_run(ctx, PrtPoem05Step::Start, Vec::new()).map(|_| ())
}

pub fn prt_poem05_ontouch(ctx: &Ctx) -> Script {
    prt_poem05_run(ctx, PrtPoem05Step::OnTouch, Vec::new()).map(|_| ())
}

fn dog_prt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Dog", args!["Bow Wow!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn dog_prt(ctx: &Ctx) -> Script {
    dog_prt_body(ctx, Vec::new()).map(|_| ())
}

fn dazed_boy_prt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("prt_curse").get()? == 11 {
        ctx.lines_as(
            "Absent-Minded Boy",
            args![
                "The great serpent",
                "swallowed the sea.",
                "The eagle of the rainbow",
                "swallowed the serpent.",
                "Then snake scales grew on",
                "the eagle, and it slowly died."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Exhausted-Looking Woman",
            args!["Goodness...!", "Will you please stop", "singing that song?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Absent-Minded Boy", args![".................."])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThe little boy quietly",
            "stared at the woman for a",
            "little bit, and then crouched",
            "down and began petting a",
            "dog sitting next to him.^000000"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("What did you just sing?:Ignore them")],
        )?) == 1
        {
            ctx.lines_as("Absent-Minded Boy", args!["...", "......", "........."])?;
            ctx.next()?;
            ctx.lines_as(
                "Absent-Minded Boy",
                args![
                    "Woof-woof, have you",
                    "heard this song? Mommy",
                    "taught me to sing it, you",
                    "know. She says it was sung",
                    "by people for a long time~"
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFThe boy continued speaking",
                "to the dog like an old friend,",
                "ignoring your presence and",
                "acting as if you didn't exist.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Exhausted-Looking Woman",
                args!["Kaanu...", "What did I tell", "you about ignoring", "people's questions!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kaanu",
                args![
                    ".........",
                    "..................",
                    "Uh oh, Woof-woof.",
                    "Mommy seems upset",
                    "for some reason."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Exhausted-Looking Woman",
                args![
                    "I'm sorry, my kid is...",
                    "Look, did you want to know",
                    "more about that song? Try",
                    "not to worry about it: it's just an old children's tune."
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Why are you here?:May I hear the song again?")],
            )?) == 1
            {
                ctx.lines_as("Exhausted-Looking Woman", args!["................."])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFThe woman's lip tightens",
                    "with shame. It's obvious",
                    "that this topic makes her",
                    "feel very uncomfortable.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as("Kaanu", args!["...", "......", "........."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kaanu",
                    args![
                        "Woof-woof, do you know",
                        "why Mommy is always",
                        "so sad? I wish I knew",
                        "why Mommy never smiles..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Exhausted-Looking Woman", args!["........"])?;
                ctx.var("prt_curse").set(Val::from(12))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Exhausted-Looking Woman", args![".................."])?;
            ctx.next()?;
            ctx.lines_as(
                "Exhausted-Looking Woman",
                args!["Okay. I can do this", "simple thing for you.", "Let me recite this song..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Exhausted-Looking Woman",
                args![
                    "The great serpent",
                    "swallowed the sea.",
                    "The eagle of the rainbow",
                    "swallowed the serpent.",
                    "Then snake scales grew on",
                    "the eagle, and it slowly died."
                ],
            )?;
            ctx.var("prt_curse").set(Val::from(12))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Absent-Minded Boy", args!["...", "......", "........."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("prt_curse").get()? == 36 {
        ctx.lines(args![
            "^3355FFThe little boy looked",
            "very worn and weary of",
            "life, a look that is very",
            "unsettling on the face",
            "of a young child.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("prt_curse").get()? == 45 {
        ctx.lines(args![
            "^3355FFThe little boy still",
            "won't talk to you, but",
            "he acknowledges your",
            "presense by making eye",
            "contact and smiling.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("prt_curse").get()?.number()? > 54 {
        ctx.lines(args![
            "^3355FFThe little boy still",
            "won't talk to you, but he",
            "make you feel welcome",
            "by giving you a warm smile.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Absent-Minded Boy", args!["...", "......", "........."])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou find it difficult",
            "to ignore the little boy's",
            "probing, wary eyes directed",
            "at you. It's very clear that",
            "he doesn't trust you.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn dazed_boy_prt(ctx: &Ctx) -> Script {
    dazed_boy_prt_body(ctx, Vec::new()).map(|_| ())
}

fn exhausted_looking_woman_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
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
    if ctx.var("prt_curse").get()? == 11 {
        ctx.lines_as(
            "Absent-Minded Boy",
            args![
                "The great serpent",
                "swallowed the sea.",
                "The eagle of the rainbow",
                "swallowed the serpent.",
                "Then snake scales grew on",
                "the eagle, and it slowly died."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Exhausted-Looking Woman",
            args!["Goodness...!", "Will you please stop", "singing that song?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Absent-Minded Boy", args![".................."])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThe little boy quietly",
            "stared at the woman for a",
            "little bit, and then crouched",
            "down and began petting a",
            "dog sitting next to him.^000000"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("What did you just sing?:Ignore them")],
        )?) == 1
        {
            ctx.lines_as("Absent-Minded Boy", args!["...", "......", "........."])?;
            ctx.next()?;
            ctx.lines_as(
                "Absent-Minded Boy",
                args![
                    "Woof-woof, have you",
                    "heard this song? Mommy",
                    "taught me to sing it, you",
                    "know. She says it was sung",
                    "by people for a long time~"
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFThe boy continued speaking",
                "to the dog like an old friend,",
                "ignoring your presence and",
                "acting as if you didn't exist.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Exhausted-Looking Woman",
                args!["Kaanu...", "What did I tell", "you about ignoring", "people's questions!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kaanu",
                args![
                    ".........",
                    "..................",
                    "Uh oh, Woof-woof.",
                    "Mommy seems upset",
                    "for some reason."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Exhausted-Looking Woman",
                args![
                    "I'm sorry, my kid is...",
                    "Look, did you want to know",
                    "more about that song? Try",
                    "not to worry about it: it's just an old children's tune."
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Why are you here?:May I hear the song again?")],
            )?) == 1
            {
                ctx.lines_as("Exhausted-Looking Woman", args!["................."])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFThe woman's lip tightens",
                    "with shame. It's obvious",
                    "that this topic makes her",
                    "feel very uncomfortable.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as("Kaanu", args!["...", "......", "........."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kaanu",
                    args![
                        "Woof-woof, do you know",
                        "why Mommy is always",
                        "so sad? I wish I knew",
                        "why Mommy never smiles..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Exhausted-Looking Woman", args!["........"])?;
                ctx.var("prt_curse").set(Val::from(12))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Exhausted-Looking Woman", args![".................."])?;
            ctx.next()?;
            ctx.lines_as(
                "Exhausted-Looking Woman",
                args!["Okay. I can do this", "simple thing for you.", "Let me recite this song..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Exhausted-Looking Woman",
                args![
                    "The great serpent",
                    "swallowed the sea.",
                    "The eagle of the rainbow",
                    "swallowed the serpent.",
                    "Then snake scales grew on",
                    "the eagle, and it slowly died."
                ],
            )?;
            ctx.var("prt_curse").set(Val::from(12))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Exhausted-Looking Woman", args!["..........."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("prt_curse").get()? == 12 {
            ctx.lines_as(
                "Exhausted-Looking Woman",
                args![
                    "Not too many people",
                    "come to this remote area.",
                    "What brings you all the",
                    "way over here, I wonder?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Actually, I'm just",
                    "trying to learn the",
                    "real lyrics of that",
                    "song that little boy",
                    "has been singing."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Exhausted-Looking Woman", args!["........."])?;
            ctx.next()?;
            ctx.lines_as(
                "Exhausted-Looking Woman",
                args![
                    "That song's lyrics, and",
                    "its meaning, can change",
                    "from person to person as",
                    "time goes on. I suppose part",
                    "of the reason is because it's",
                    "been handed down orally..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Exhausted-Looking Woman",
                args![
                    "It sounds like such a happy",
                    "song, but it's meaning had",
                    "changed for me ever since",
                    "the accident had happened...",
                    "It's been so long, but I don't",
                    "think the pain will ever heal."
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Accident?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Exhausted-Looking Woman",
                args![
                    "I'm... I'm sorry.",
                    "I don't think I can talk",
                    "about it. If I try, I know I'll",
                    "relive the horror. Every day",
                    "is already a struggle to cope",
                    "with the sins I've committed."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Exhausted-Looking Woman",
                args![
                    "I know it's selfish, but",
                    "I have a favor to ask of you.",
                    "I can't really leave this area,",
                    "and seldom do I receive visitors. Would you be so kind to deliver",
                    "something to Prontera for me?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Exhausted-Looking Woman",
                args![
                    "I used to live in Prontera,",
                    "and ever since the accident,",
                    "Father Bamph has been so very",
                    "supportive of me. It's not enough to repay his kindness, but please",
                    "give these herbs to him for me."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Exhausted-Looking Woman",
                args![
                    "You can find ^3131FFFather Bamph^000000",
                    "in ^3131FFProntera Church^000000. When you ",
                    "see him, please give him my",
                    "warmest regards. I would do it",
                    "myself, but because of what I did, I can't return to Prontera..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "^333333(Alright then. I guess",
                    "I should deliver these",
                    "herbs to Prontera Church",
                    "before I talk to Rodafrian.)^000000"
                ],
            )?;
            ctx.call(Function::GetItem, vec![Val::from(7432), Val::from(1)])?;
            ctx.var("prt_curse").set(Val::from(13))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if (ctx.var("prt_curse").get()?.number()? > 12 && ctx.var("prt_curse").get()?.number()? < 16) {
                ctx.lines_as(
                    "Exhausted-Looking Woman",
                    args![
                        "Father Bamph has helped",
                        "me so much. Please bring",
                        "that herb pouch to him in",
                        "Prontera Church, and give",
                        "him my warmest regards."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("prt_curse").get()? == 16 {
                    ctx.lines_as(
                        "Bonnie Imbullea",
                        args![
                            "Oh, you've come back?",
                            "Were you able to deliver",
                            "that pouch of herbs that",
                            "I gave you to Father Bamph?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Yes, I did. Actually,",
                            "Father Bamph sent me",
                            "here to ask you a few",
                            "things. You see, he told",
                            "me all about the royal secret",
                            "and, well, your exorcism."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bonnie Imbullea",
                        args![
                            "Oh. I see.",
                            "You must think",
                            "me a monster...",
                            "Those poor children",
                            "are dead because of me..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "N-no! Not at all!",
                            "I just wanted to ask if there",
                            "was a connection between",
                            "the song that Kaanu has been",
                            "singing and Jormungand's curse."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Bonnie Imbullea", args!["...", "......", "........."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bonnie Imbullea",
                        args![
                            "After the exorcism failed,",
                            "I should have been punished",
                            "with death. But the royal family and the Prontera Church took",
                            "mercy on me, and allowed",
                            "me to live here in exile."
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bonnie Imbullea",
                        args![
                            "I began to sing this song",
                            "to keep myself distracted,",
                            "even if it was related to",
                            "the Jormungand curse."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bonnie Imbullea",
                        args![
                            "You know, now that I think",
                            "about it, I do remember hearing",
                            "that after the exorcism failed,",
                            "a maid found a fragment of a",
                            "Red Gemstone while cleaning",
                            "the secret ceremonial grounds."
                        ],
                    )?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_SURPRISE")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bonnie Imbullea",
                        args![
                            "I always thought it",
                            "was a little strange.",
                            "I wanted to investigate",
                            "it, but then I was discharged",
                            "from the Prontera Church",
                            "after my failure..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "A fragment from",
                            "a Red Gemstone?",
                            "Maybe... maybe it's",
                            "evidence that someone",
                            "sabotaged the exorcism!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Bonnie Imbullea",
                        args![
                            "What happened was tragic,",
                            "but it would make me feel",
                            "so much better if that were",
                            "really the truth. Even if the",
                            "exorcism was sabotaged,",
                            "we can't turn back time..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "^333333(Hmm...",
                            "I better head back",
                            "to Prontera Church",
                            "and let Father Bamph",
                            "know about this information.)^000000"
                        ],
                    )?;
                    ctx.var("prt_curse").set(Val::from(17))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("prt_curse").get()? == 17 {
                        ctx.lines_as(
                            "Bonnie Imbullea",
                            args![
                                "It's really very",
                                "difficult to live",
                                "with this guilt.",
                                "I was trained to help",
                                "people, not kill them!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFYou tried to comfort",
                            "Bonnie Imbullea for a",
                            "little while before you",
                            "return to Prontera Church."
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ((ctx.var("prt_curse").get()? == 35 || ctx.var("prt_curse").get()? == 44)
                        || ctx.var("prt_curse").get()? == 54)
                    {
                        ctx.lines(args![
                            "^3355FFYou tell Bonnie Imbullea",
                            "that the deaths of the princes",
                            "were not her fault and that the exorcism was probably sabotaged.",
                            "However, you keep specifics, like the use of poison, to yourself.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Bonnie Imbullea",
                            args![
                                "Is that really true...?",
                                "Oh, I'm so happy! I really",
                                "thought I'd have to live the",
                                "rest of my in seclusion with",
                                "this horrible shame. Oh, thank",
                                "you so much for your help!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Bonnie Imbullea",
                            args![
                                "Kaanu will be able to",
                                "meet other kids his own",
                                "age... We can finally live",
                                "a normal life. How wonderful..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Bonnie Imbullea",
                            args![
                                "But wait...",
                                "How exactly did the",
                                "princes die, then?",
                                "There weren't any",
                                "wounds on the bodies",
                                "from what I remember..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Well, um, I wouldn't know",
                                "anything about that, but",
                                "Father Bamph and Father",
                                "Biscuss are handling the",
                                "investigation, and they're",
                                "doing their best to find out."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Bonnie Imbullea",
                            args![
                                "I see. Well, I believe in",
                                "them: they're great priests,",
                                "after all. Although I may be",
                                "forgiven for my failure, I still fear for the royal family...",
                                "All I can do for now is pray."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Bonnie Imbullea",
                            args![
                                "Anyway, I promise that",
                                "I won't reveal what I know",
                                "about this incident to the",
                                "public. And if you ever come",
                                "by the mountain again, I hope",
                                "you stop by to visit us."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Bonnie Imbullea",
                            args![
                                "You don't understand",
                                "how much the news you've",
                                "brought really means to me.",
                                "I can finally free myself from",
                                "this burdensome guilt...",
                                "Thank you, kind adventurer~"
                            ],
                        )?;
                        if ctx.var("prt_curse").get()? == 35 {
                            ctx.var("prt_curse").set(Val::from(36))?;
                        } else if ctx.var("prt_curse").get()? == 44 {
                            ctx.var("prt_curse").set(Val::from(45))?;
                        } else {
                            ctx.next()?;
                            ctx.lines(args![
                                "^3355FFNow, you can return to",
                                "Rodafrian, the historian",
                                "stationed in Morocc.^000000"
                            ])?;
                            ctx.var("prt_curse").set(Val::from(55))?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("prt_curse").get()? == 36 {
                        ctx.lines_as(
                            "Bonnie Imbullea",
                            args![
                                "By your grace and mercy,",
                                "please pity the poor souls",
                                "and protect the royal family...",
                                "Bless us with your light and",
                                "may your wisdom guide us..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("prt_curse").get()? == 45 {
                        ctx.lines_as(
                            "Bonnie Imbullea",
                            args![
                                "I guess we've gotten",
                                "too used to the peace",
                                "and serenity of Mount",
                                "Mjolnir. It won't be easy",
                                "to leave, even if we're welcome to return to Prontera..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("prt_curse").get()?.number()? > 54 {
                        ctx.lines_as(
                            "Bonnie Imbullea",
                            args![
                                "Thank you so much",
                                "for helping out in the",
                                "investigation of the",
                                "princes. I'd never know",
                                "happiness again if it",
                                "weren't for your efforts."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Bonnie Imbullea", args!["^333333*Sigh...*^000000"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
}

pub fn exhausted_looking_woman(ctx: &Ctx) -> Script {
    exhausted_looking_woman_body(ctx, Vec::new()).map(|_| ())
}
