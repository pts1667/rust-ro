use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn kaci_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_a = Val::from(0);
    let mut l_b = Val::from(0);
    let mut l_c = Val::from(0);
    ctx.mes("[Kaci]")?;
    if ctx.var("hg_ma1").get()? == 3 {
        ctx.lines(args![
            "You must be sooo",
            "bored, adventurer~",
            "How would you like",
            "to play a game of Dice?"
        ])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Do you know a guy named Thierry?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Kaci", args!["Thierry? Um...that sounds familiar..."])?;
        ctx.next()?;
        ctx.lines_as("Kaci", args!["....................", ".............", ".......", "..."])?;
        ctx.next()?;
        ctx.lines_as("Kaci", args!["Oh, right! Thierry, Thierry, Thierry! Right?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Kaci",
            args![
                "- She suddenly called out the name several times,  -",
                "- so you were startled and dropped your half eaten apple. -"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Kaci", args!["Oops, sorry, if I startled you. I could not remember the name because it has been a while since I heard of his name the last time."])?;
        ctx.next()?;
        ctx.lines_as(
            "Kaci",
            args![
                "Thierry was a guy living in the next door while I was learning games and tricks from Mr. Mawong.",
                "I remember that he was a very smart guy and was doing many research stuffs."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kaci",
            args![
                "Oh yeah, and he was in love with Eukran's sister.",
                "And Eukran and I were studying games together under Mr. Mawong at that time.",
                "Hmm...I wonder how they have been doing."
            ],
        )?;
        ctx.next()?;
        ctx.mes("- You told her that Euslan and Thierry got engaged, but Thierry had to leave her to find medicine for her illness. -")?;
        ctx.next()?;
        ctx.lines_as(
            "Kaci",
            args!["Oh...gosh...I can't believe that Euslan is has become ill....Oh......"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "- Kaci seemed to be shocked by such devastating news. -",
            "- She was mumbling something for a while, and then came back to her senses again. -"
        ])?;
        ctx.next()?;
        ctx.lines_as("Kaci", args!["I am so sorry for her...I really want to give here some help...but, it has been a while for me since I saw them. So, I don't know where he is."])?;
        ctx.next()?;
        ctx.lines_as("Kaci", args!["But, let me think if..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Kaci",
            args![
                "Oh, right! The airship captain might be able to help you, because he is the one who is in charge of the passenger list."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kaci",
            args!["Although the passenger list is classified and strictly restricted from releasing to public..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kaci",
            args!["But, if you tell him the story, he might be able to help you. Please go ask him if you can."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kaci",
            args!["And if you see Euslan again, please send my regard and blessing to her."],
        )?;
        ctx.next()?;
        ctx.mes("- You promised her to send her regards and blessings to Euslan. -")?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8044), Val::from(8045)])?;
        ctx.var("hg_ma1").set(Val::from(4))?;
    } else if ctx.var("hg_ma1").get()? == 4 {
        ctx.mes("The airship captain might be able to help you in finding Thierry's whereabouts.")?;
    } else if ctx.var("lhz_heart").get()? == 2 {
        ctx.lines(args![
            "You must be sooo",
            "bored, adventurer~",
            "How would you like",
            "to play a game of Dice?"
        ])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Give her the letter from Hallen.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Kaci",
            args![
                "Oh, Hallen must have",
                "asked you to deliver this.",
                "He should be doing this sort",
                "of thing himself, instead of",
                "asking customers for favors!",
                "But still, he's really busy..."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8036), Val::from(8037)])?;
        ctx.var("lhz_heart").set(Val::from(3))?;
        ctx.lines_as(
            "Kaci",
            args![
                "Thank you for bringing",
                "this to me. If you ever",
                "need a little favor from",
                "me, feel free to ask me,",
                "alright? See you later~"
            ],
        )?;
    } else if ctx.var("lhz_heart").get()? == 3 {
        ctx.lines(args![
            "Oh, hello. Thanks for",
            "delivering that letter from",
            "Mr. Mawong to me, especially",
            "since Hallen was too busy to",
            "do it himself. Reading that",
            "letter really made my day~"
        ])?;
        ctx.next()?;
        'l1: loop {
            if !((l_a.clone() == 0 || l_b.clone() == 0) || l_c.clone() == 0) {
                break 'l1;
            }
            'b1: {
                if ((l_a.clone().is_true() || l_b.clone().is_true()) || l_c.clone().is_true()) {
                    ctx.lines_as(
                        "Kaci",
                        args![
                            "Please don't hesitate",
                            "to ask me anything if",
                            "you need a small favor,",
                            "or if you're just curious",
                            "about me in general~"
                        ],
                    )?;
                    ctx.next()?;
                }
                match runtime::select_values(ctx, &[Val::from("Ask about Hallen:Ask about Mawong:Ask about the Airship")])? {
                    1 => {
                        l_a = Val::from(1);
                        ctx.lines_as(
                            "Kaci",
                            args![
                                "Hallen? Well, he and I are",
                                "pretty close. Did you know",
                                "that we're cousins? In fact,",
                                "his dad, my uncle, brought",
                                "me as a child after my father",
                                "passed away 20 years ago."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kaci",
                            args![
                                "There was some kind of",
                                "mine explosion accident in",
                                "Einbech. I don't remember.",
                                "Anyway, me and Hallen are",
                                "like sister and brother, and",
                                "we always stick together."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kaci",
                            args![
                                "When Captain Ferlock saw me",
                                "host a Dice game by coincidence",
                                "and then hired me to work on his Airship, Hallen insisted on coming",
                                "along. He's stubborn like that,",
                                "but he's also very sweet."
                            ],
                        )?;
                    }
                    2 => {
                        l_b = Val::from(1);
                        ctx.lines_as(
                            "Kaci",
                            args![
                                "Mr. Mawong? Oh, don't get",
                                "the wrong idea! He's my",
                                "mentor, the one who taught",
                                "me how to play all of these",
                                "wonderful games. He's pretty",
                                "famous around Juno, you know."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kaci",
                            args![
                                "I used to be so depressed,",
                                "not caring about the world",
                                "at all, after my father died.",
                                "Then, I ran into Mr. Mawong,",
                                "and he taught me how to find",
                                "the joy in life once again."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kaci",
                            args![
                                "Because of him, I decided",
                                "to learn his games and to",
                                "help people forget their",
                                "worries and just enjoy",
                                "themselves, even if it's",
                                "just for a little while."
                            ],
                        )?;
                    }
                    3 => {
                        l_c = Val::from(1);
                        ctx.lines_as(
                            "Kaci",
                            args![
                                "The Airship? It's so",
                                "huge and beautiful, and",
                                "I'm always amazed that it",
                                "can fly so gracefully through",
                                "the air. I've always wanted to live someplace close to the sky..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kaci",
                            args![
                                "A place where I can see",
                                "the clouds and bask in the",
                                "sun. So, when Captain Ferlock",
                                "invited me to work here, it was",
                                "like a dream come true~"
                            ],
                        )?;
                    }
                    _ => {}
                }
                ctx.next()?;
            }
        }
        ctx.lines_as(
            "Kaci",
            args![
                "By the way, if Hallen",
                "bothers you with another",
                "request, please refuse to",
                "help him next time, okay?",
                "I won't allow him to get",
                "spoiled! Ho ho ho ho~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kaci",
            args![
                "Well, I better get",
                "back to work now.",
                "Have a nice day,",
                "and I'll see you later~"
            ],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8037), Val::from(8038)])?;
        ctx.var("lhz_heart").set(Val::from(4))?;
    } else if ctx.var("lhz_heart").get()? == 8 {
        ctx.lines(args![
            ((Val::from("Hello, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
            "It's nice to see you again.",
            "So is there anything I can",
            "help you with today?"
        ])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Who's that drunk over there?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Kaci",
            args![
                "Oh, him? He's one of my",
                "regulars, some guy who's",
                "supposed to be an Einbroch",
                "Lab Director. He keeps losing",
                "Apples whenever he plays Dice:",
                "I think he's a gambling addict~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kaci",
            args![
                "In fact, his losses make up",
                "for all the Apples I lost when",
                "that Apple Merchant played here. Now, it's a legendary tale that",
                "we all think fondly of, but at",
                "the time, it nearly broke me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kaci",
            args![
                "Anyway, aside from the fact",
                "that the Einbroch Lab Director",
                "is kind of a whiny person when",
                "he's drunk, and a poor gambler,",
                "I don't know much about him."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "(^333333Einbroch Laboratory...",
                "It's so suspicious. That",
                "director knows about Ymir's",
                "Heart Piece, so they must be",
                "keeping something really",
                "important over there.^000000)"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "(^333333I better investigate",
                "that laboratory in Einbroch",
                "as soon as I can, even if it's",
                "a heavily restricted area.^000000)"
            ],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8041), Val::from(8042)])?;
        ctx.var("lhz_heart").set(Val::from(9))?;
    } else {
        ctx.lines(args![
            "You must be sooo",
            "bored, adventurer~",
            "How would you like",
            "to play a game of Dice?"
        ])?;
        ctx.next()?;
        shared::airports_airships::applegamble(ctx, vec![Val::from("Kaci")])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn kaci(ctx: &Ctx) -> Script {
    kaci_body(ctx, Vec::new()).map(|_| ())
}

fn lab01_heart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn lab01_heart(ctx: &Ctx) -> Script {
    lab01_heart_body(ctx, Vec::new()).map(|_| ())
}

fn lab01_heart_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_heart").get()? == 9 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "(^333333I've... I've just",
                "got to investigate",
                "that laboratory and",
                "see if anything funny",
                "is going on in there.^000000)"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn lab01_heart_ontouch(ctx: &Ctx) -> Script {
    lab01_heart_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn lab02_heart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn lab02_heart(ctx: &Ctx) -> Script {
    lab02_heart_body(ctx, Vec::new()).map(|_| ())
}

fn lab02_heart_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_heart").get()? == 9 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "What the...",
                "It's some sort of",
                "weird device. The label",
                "here says, ''Password",
                "Checker?'' Well, this",
                "might come in handy later."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou decide to bring",
            "the Password Checker",
            "device with you into",
            "the laboratory.^000000"
        ])?;
        ctx.var("lhz_heart").set(Val::from(10))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn lab02_heart_ontouch(ctx: &Ctx) -> Script {
    lab02_heart_ontouch_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum FerlockLabStep {
    Start,
    OnEnter,
    OnTimer120000,
    OnInit,
}

fn ferlock_lab_run(ctx: &Ctx, mut step: FerlockLabStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            FerlockLabStep::Start => {
                return Err(Stop::End);
            }
            FerlockLabStep::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Ferlock#lab")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            FerlockLabStep::OnTimer120000 => {
                step = FerlockLabStep::OnInit;
                continue 'machine;
            }
            FerlockLabStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Ferlock#lab")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ferlock_lab(ctx: &Ctx) -> Script {
    ferlock_lab_run(ctx, FerlockLabStep::Start, Vec::new()).map(|_| ())
}

pub fn ferlock_lab_onenter(ctx: &Ctx) -> Script {
    ferlock_lab_run(ctx, FerlockLabStep::OnEnter, Vec::new()).map(|_| ())
}

pub fn ferlock_lab_ontimer120000(ctx: &Ctx) -> Script {
    ferlock_lab_run(ctx, FerlockLabStep::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn ferlock_lab_oninit(ctx: &Ctx) -> Script {
    ferlock_lab_run(ctx, FerlockLabStep::OnInit, Vec::new()).map(|_| ())
}

fn lab03_heart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn lab03_heart(ctx: &Ctx) -> Script {
    lab03_heart_body(ctx, Vec::new()).map(|_| ())
}

fn lab03_heart_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_ball = Val::from(0);
    let mut l_input = Val::from(0);
    let mut l_input1 = Val::from(0);
    let mut l_input10 = Val::from(0);
    let mut l_input100 = Val::from(0);
    let mut l_retry = Val::from(0);
    let mut l_strike = Val::from(0);
    let mut l_yagu1 = Val::from(0);
    let mut l_yagu10 = Val::from(0);
    let mut l_yagu100 = Val::from(0);
    if ctx.var("lhz_heart").get()? == 9 {
        ctx.lines_as(
            "Security System",
            args![
                "^FF0000Unauthorized person",
                "detected. Password",
                "Checker not detected.",
                "Access denied.^000000"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lhz_heart").get()? == 10 {
        ctx.lines_as(
            "Security System",
            args![
                "^FF0000Enter the 3 digit password.",
                "You will be allowed 5 tries",
                "within 3 minutes to enter",
                "the correct password, or",
                "the password will reset.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Password Checker",
            args![
                "^333333Hint:",
                "Do not use the",
                "number 0, and do",
                "not enter any number",
                "more than once. Make",
                "sure the password is 3 digits."
            ],
        )?;
        'l1: loop {
            if !((l_yagu100.clone().loosely_equals(&l_yagu10.clone()) || l_yagu100.clone().loosely_equals(&l_yagu1.clone()))
                || l_yagu10.clone().loosely_equals(&l_yagu1.clone()))
            {
                break 'l1;
            }
            'b1: {
                l_yagu100 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(9)])?;
                l_yagu10 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(9)])?;
                l_yagu1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(9)])?;
            }
        }
        'l2: loop {
            if !(true) {
                break 'l2;
            }
            'b2: {
                'l3: loop {
                    if !(true) {
                        break 'l3;
                    }
                    'b3: {
                        ctx.next()?;
                        let (input, status) = runtime::input_number(ctx, None, None)?;
                        l_input = input;
                        if (l_input.clone().number()? < 100 || l_input.clone().number()? > 999) {
                            ctx.lines_as(
                                "Security System",
                                args![
                                    "^FF0000Error.",
                                    "The password entered",
                                    "exceeds the number digit",
                                    "limit. You must only enter",
                                    "3 digit passwords.^000000"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        l_input100 = (l_input.clone().try_div(Val::from(100))?);
                        l_input10 = ((l_input.clone().try_rem(Val::from(100))?).try_div(Val::from(10))?);
                        l_input1 = (l_input.clone().try_rem(Val::from(10))?);
                        if (((((l_input100.clone() + l_input10.clone()) + l_input1.clone()) == 0
                            || l_input100.clone().loosely_equals(&l_input10.clone()))
                            || l_input100.clone().loosely_equals(&l_input1.clone()))
                            || l_input10.clone().loosely_equals(&l_input1.clone()))
                        {
                            ctx.lines_as(
                                "Security System",
                                args![
                                    "^FF0000Error.",
                                    "You cannot enter the",
                                    "number 0, or use any",
                                    "other number more than",
                                    "once. Please try again.^000000"
                                ],
                            )?;
                            break 'b3;
                        }
                        break 'l3;
                    }
                }
                l_retry = (l_retry.clone() + Val::from(1));
                ctx.lines_as(
                    "Security System",
                    args![
                        ((Val::from("^ff0000") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("^000000")),
                        "has entered the following:",
                        ((((((Val::from("^0000ff") + l_input100.clone()) + Val::from("^000000 - ^0000ff")) + l_input10.clone())
                            + Val::from("^000000 - ^0000ff"))
                            + l_input1.clone())
                            + Val::from("^000000.")),
                        "Please wait for authorization to complete."
                    ],
                )?;
                ctx.next()?;
                l_strike = ((Val::from(l_yagu100.clone().loosely_equals(&l_input100.clone()))
                    + Val::from(l_yagu10.clone().loosely_equals(&l_input10.clone())))
                    + Val::from(l_yagu1.clone().loosely_equals(&l_input1.clone())));
                l_ball = ((Val::from(
                    (l_yagu100.clone().loosely_equals(&l_input10.clone()) || l_yagu100.clone().loosely_equals(&l_input1.clone())),
                ) + Val::from(
                    (l_yagu10.clone().loosely_equals(&l_input100.clone()) || l_yagu10.clone().loosely_equals(&l_input1.clone())),
                )) + Val::from(
                    (l_yagu1.clone().loosely_equals(&l_input100.clone()) || l_yagu1.clone().loosely_equals(&l_input10.clone())),
                ));
                if l_strike.clone() == 3 {
                    ctx.lines_as("Security System", args!["Authorization complete.", "Archive access granted."])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFThe archive door opens,",
                        "revealing a series of",
                        "filed documents. Out",
                        "of all of them, one of the",
                        "files grabs your attention.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "Varmunt Project No. 3",
                        "Security Level : Grade 1-C",
                        " ",
                        "Caution: Only project members",
                        "of the Ymir Heart Synthesization project are authorized to view",
                        "this classified document."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "Varmunt Project No. 3",
                        "Security Level : Grade 1-C",
                        " ",
                        "All others found reading",
                        "this document are subject",
                        "to punishment or dismissal."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou perceive the sound of",
                        "footsteps and quickly try to",
                        "return the document to its",
                        "original place. However,",
                        "the person that has come",
                        "to the archive has already",
                        "seen you holding the file.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Ferlock#lab::OnEnter")])?;
                    ctx.lines_as(
                        "Ferlock",
                        args![
                            "Excuse me, but are",
                            "you an employee here?",
                            "I've come to pick up a",
                            "new Rune Stone for the",
                            "Airship. Wait, wait...",
                            "You look familiar..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_HUK")?,
                            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Ferlock#lab")])?,
                        ],
                    )?;
                    ctx.lines_as(
                        "Ferlock",
                        args![
                            "That's right, aren't you the",
                            "one who brought me my",
                            "brother's letter? Yeah, you",
                            "were asking me about the",
                            "Airship's flight mechanics.",
                            "So you're not an employee..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ferlock",
                        args![
                            "Boy, you must be more than",
                            "curious to be looking up that",
                            "information in a restricted",
                            "area. But don't worry, you",
                            "must have your reasons,",
                            "so I won't call the guards."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ferlock",
                        args![
                            "Still, you better get out",
                            "of here. The guards patrol",
                            "this place regularly, so you're",
                            "almost sure to get caught. Hmm,",
                            "come and see me later at the",
                            "Airship, alright? Now hurry!"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Ferlock#lab::OnInit")])?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(8042), Val::from(8043)])?;
                    ctx.var("lhz_heart").set(Val::from(11))?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Password Checker",
                    args![
                        " ",
                        "^333333Total of correct numbers",
                        ((Val::from("in correct sequence: ^ff0000") + l_strike.clone()) + Val::from("^333333")),
                        ((Val::from("Total of correct numbers misplaced: ^ff0000") + l_ball.clone()) + Val::from("^333333.")),
                        "Please use these results",
                        "to make a more accurate guess.^000000"
                    ],
                )?;
                ctx.next()?;
                if l_retry.clone().number()? > 4 {
                    ctx.lines_as(
                        "Security System",
                        args![
                            "^FF0000The correct password",
                            "for this session was",
                            ((((((Val::from("^000000") + l_yagu100.clone()) + Val::from("^FF0000 - ^000000")) + l_yagu10.clone())
                                + Val::from("^000000 - ^000000"))
                                + l_yagu1.clone())
                                + Val::from("^FF0000.")),
                            "Password will now be reset.^000000"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Security System", args!["Beeeeep!", "Incorrect password.", " "])?;
                let subject4 = l_retry.clone();
                if subject4 == 1 {
                    ctx.mes("Initialing 2nd attempt...^000000")?;
                } else if subject4 == 2 {
                    ctx.mes("Initialing 3rd attempt...^000000")?;
                } else if subject4 == 3 {
                    ctx.mes("Initialing 4th attempt...^000000")?;
                } else if subject4 == 4 {
                    ctx.mes("Initialing final attempt...^000000")?;
                }
            }
        }
    }
    return Err(Stop::End);
}

pub fn lab03_heart_ontouch(ctx: &Ctx) -> Script {
    lab03_heart_ontouch_body(ctx, Vec::new()).map(|_| ())
}
