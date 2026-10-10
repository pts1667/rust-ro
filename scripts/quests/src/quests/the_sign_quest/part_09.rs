use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum WitchStep {
    Start,
    OnTouch,
}

fn witch_run(ctx: &Ctx, mut step: WitchStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            WitchStep::Start => {
                step = WitchStep::OnTouch;
                continue 'machine;
            }
            WitchStep::OnTouch => {
                shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
                if ctx.var("sign_q").get()? == 83 {
                    if ctx.var("sign_sq").get()? == 2 {
                        if ctx.call(Function::CountItem, vec![Val::from(7304)])?.number()? > 0 {
                            ctx.lines_as(
                                "Kirkena",
                                args!["W-what's this?", "Why is it that", "you have one of", "my lost spell books?"],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("From a cursed soul...")])? {
                                1 => {
                                    ctx.lines_as("Kirkena", args!["Oh. That bastard must have", "stolen my spell books and got cursed when he cast the spells without my permission. Still, it's a relief to have my spells back."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Kirkena",
                                        args!["So mortal...", "You must be here for", "some reason. Speak", "your mind."],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(
                                        ctx,
                                        &[Val::from(
                                            "Send me back to my world.:There's a lost child here that I want to help...",
                                        )],
                                    )? {
                                        1 => {
                                            ctx.lines_as(
                                                "Kirkena",
                                                args![
                                                    "Understood.",
                                                    "Let me send",
                                                    "you back to the",
                                                    "realm of the living",
                                                    "where you belong..."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            ctx.call(Function::Warp, vec![Val::from("umbala"), Val::from(132), Val::from(203)])?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as(
                                                "Kirkena",
                                                args![
                                                    "There's a living child?",
                                                    "Here in Niflheim? That's",
                                                    "most peculiar. Well, let me",
                                                    "give you this. It will send",
                                                    "a living human back to one",
                                                    "of the towns in your realm."
                                                ],
                                            )?;
                                            ctx.call(Function::DelItem, vec![Val::from(7304), Val::from(1)])?;
                                            ctx.var("sign_sq").set(Val::from(3))?;
                                            ctx.call(Function::GetItem, vec![Val::from(7309), Val::from(1)])?;
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
                                "Kirkena",
                                args![
                                    "A mortal? What are you doing",
                                    "here? I'm not sure how more",
                                    "of you are able to get here,",
                                    "or your reasons for coming,",
                                    "but this place is dangerous",
                                    "for the living."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kirkena",
                                args![
                                    "I'm sending you back",
                                    "to the realm of the living.",
                                    "If you can help it, you should",
                                    "probably avoid coming back",
                                    "to Niflheim..."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("umbala"), Val::from(132), Val::from(203)])?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ctx.var("sign_sq").get()? == 3 {
                            if ctx.call(Function::CountItem, vec![Val::from(7309)])?.number()? < 1 {
                                ctx.lines_as(
                                    "Kirkena",
                                    args![
                                        "You lost the wing?",
                                        "Here, let me give you",
                                        "my last one. Now, hurry",
                                        "up and save that lost child!"
                                    ],
                                )?;
                                ctx.var("sign_sq").set(Val::from(4))?;
                                ctx.call(Function::GetItem, vec![Val::from(7309), Val::from(1)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Kirkena",
                                    args![
                                        "I just gave you",
                                        "that wing, right?",
                                        "Hurry and save that",
                                        "poor child stuck in",
                                        "Niflheim!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else {
                            if ctx.var("sign_sq").get()?.number()? < 6 {
                                ctx.lines_as(
                                    "Kirkena",
                                    args![
                                        "Hmm...",
                                        "If you don't have",
                                        "anything important to",
                                        "accomplish in Niflheim,",
                                        "you should probably return",
                                        "to the realm of the living..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("sign_sq").get()? == 6 {
                                ctx.lines_as(
                                    "Kirkena",
                                    args!["So did you already", "use the wing to save", "that poor, lost child?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Kirkena",
                                    args![
                                        "What...?!",
                                        "It didn't work?",
                                        "That's impossible!",
                                        "Wait, give me a moment to",
                                        "think. What could be wrong?"
                                    ],
                                )?;
                                ctx.var("sign_sq").set(Val::from(7))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("sign_sq").get()? == 7 {
                                ctx.lines_as(
                                    "Kirkena",
                                    args!["Alright.", "Now I understand", "why the wing didn't", "work for that poor girl..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Kirkena",
                                    args![
                                        "She's so young that",
                                        "she doesn't even know",
                                        "that she's dead. It's tragic,",
                                        "but there's no way for her",
                                        "to get out of here..."
                                    ],
                                )?;
                                ctx.var("sign_sq").set(Val::from(8))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Kirkena",
                                    args![
                                        "When people don't fully",
                                        "know the situation that",
                                        "they're in, they become more",
                                        "willing to take their chances."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                } else {
                    if ctx.var("sign_q").get()?.number()? < 88 {
                        ctx.lines_as(
                            "Kirkena",
                            args![
                                "When people don't fully",
                                "know the situation that",
                                "they're in, they become more",
                                "willing to take their chances."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("sign_q").get()? == 88 {
                        if ctx.call(Function::CountItem, vec![Val::from(2642)])?.number()? > 0 {
                            ctx.lines_as(
                                "Kirkena",
                                args![
                                    "As I expected,",
                                    "there is something behind",
                                    "all of this. Now, you may",
                                    "know where you are, but",
                                    "do you understand why",
                                    "the dead are here?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kirkena",
                                args![
                                    "This realm is a place",
                                    "for warriors that have failed",
                                    "to prove their courage. Keep in",
                                    "mind that it's not too late for",
                                    "you to join their ranks."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kirkena",
                                args![
                                    "The realm of Niflheim",
                                    "is ruled by the Queen of the",
                                    "Dead. Sometimes she appears",
                                    "in her shining armor and makes the rounds. Everyone who sees her is stunned by the image of authority."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kirkena",
                                args![
                                    "Now...",
                                    "What I need you to do is",
                                    "ask the Queen of the Dead",
                                    "for the Symbol of Nine Realms."
                                ],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("What is that?")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                "Kirkena",
                                args![
                                    "The symbol acts as",
                                    "a voucher of the queen's",
                                    "authority and represents her",
                                    "undeniable right to rule over",
                                    "the dead. But right now, I can't",
                                    "explain why I need it."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kirkena",
                                args![
                                    "I'll tell you why I need",
                                    "it once you bring me the",
                                    "queen's symbol. Now, please",
                                    "keep this secret and tell no one that I asked you to bring the symbol to me."
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Alright, I understand.:How am I supposed to get the symbol?")])?
                            {
                                1 => {
                                    ctx.var("sign_q").set(Val::from(89))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Kirkena",
                                        args![
                                            "Taking the symbol",
                                            "by force is out of the",
                                            "question. Not even the gods",
                                            "would consider battling the",
                                            "Queen of the Dead."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Kirkena",
                                        args![
                                            "You will need to",
                                            "earn the queen's favor",
                                            "in order to even have",
                                            "a chance of obtaining",
                                            "the Symbol of Nine Realms."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Kirkena",
                                        args![
                                            "Now, there is a rumor",
                                            "that the Queen of the Dead",
                                            "is searching for her lost mother, Angrboda. Now, if you could find where Angrboda has been",
                                            "sealed away..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Kirkena",
                                        args![
                                            "When you need to meet",
                                            "me from now on, go ahead",
                                            "and take the ^FF0000secret passage",
                                            "through the right side of the",
                                            "portrait on the second floor^000000",
                                            ((Val::from("of this castle, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                + Val::from("."))
                                        ],
                                    )?;
                                    ctx.var("sign_q").set(Val::from(92))?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Kirkena",
                                        args![
                                            "If you use that piano",
                                            "to find me, I'll probably",
                                            "mistake you for one of the",
                                            "humans that need guidance",
                                            "out of Niflheim. So don't",
                                            "forget to use that passage."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        } else {
                            ctx.lines_as("Kirkena", args!["Did you need my help?", "I know that I'm one of the few in Niflheim that are sympathetic with mortals, I've got a problem of my own that I need to deal with..."])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Never mind, sorry!:What happened...?")])? {
                                1 => {
                                    ctx.lines_as("Kirkena", args!["...", "Hrrmmmpf..."])?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Kirkena",
                                        args![
                                            "Recently, I've found that",
                                            "two of my spell books are",
                                            "missing. They contain some",
                                            "pretty potent spells that",
                                            "could cause disaster in",
                                            "incompetent hands..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Kirkena",
                                        args![
                                            "I'm pretty sure they",
                                            "were stolen, so if you'd",
                                            "retrieve them for me from",
                                            "the thief, I'd be truly grateful. Then, I'd have my hands free",
                                            "to give the help you came for."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Kirkena",
                                        args![
                                            "Oh, and next time you come",
                                            "to see me, go ahead and use",
                                            "the passage to the right of the portrait on the second floor",
                                            "of the castle."
                                        ],
                                    )?;
                                    ctx.var("sign_q").set(Val::from(90))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        }
                    } else if ctx.var("sign_q").get()? == 89 {
                        ctx.lines_as("Kirkena", args!["...Hm?", "Is there", "something that", "you need to ask me?"])?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("How can I get the symbol?")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Kirkena",
                            args![
                                "Taking the symbol",
                                "by force is out of the",
                                "question. Not even the gods",
                                "would consider battling the",
                                "Queen of the Dead."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kirkena",
                            args![
                                "You will need to",
                                "earn the queen's favor",
                                "in order to even have",
                                "a chance of obtaining",
                                "the Symbol of Nine Realms."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kirkena",
                            args![
                                "Now, there is a rumor",
                                "that the Queen of the Dead",
                                "is searching for her lost mother, Angrboda. Now, if you could find where Angrboda has been",
                                "sealed away..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kirkena",
                            args![
                                "When you need to",
                                "meet me from now on,",
                                "go ahead and take passage",
                                "through the right side of the",
                                "portrait on the second",
                                "floor of this castle."
                            ],
                        )?;
                        ctx.var("sign_q").set(Val::from(92))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn witch(ctx: &Ctx) -> Script {
    witch_run(ctx, WitchStep::Start, Vec::new()).map(|_| ())
}

pub fn witch_ontouch(ctx: &Ctx) -> Script {
    witch_run(ctx, WitchStep::OnTouch, Vec::new()).map(|_| ())
}

fn queen_of_the_dead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Lady Hell]")?;
    if ctx.var("sign_q").get()?.number()? < 117 {
        ctx.lines(args![
            "^8C1717You wish to have",
            "an audience with the",
            "Queen of the Dead without",
            "invitation? Insolent mortal!",
            "Go back to where you belong!^000000"
        ])?;
        ctx.close_window()?;
        ctx.call(Function::PercentHeal, vec![Val::from(-100), Val::from(0)])?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 117 {
        ctx.lines(args![
            "^8C1717I have been told",
            "by Ganglati that you",
            "are the mortal that has",
            "guided Mother's soul to me.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Lady Hell",
            args![
                "^8C1717Do not fear me,",
                "hero of Midgard.",
                "You have won the favor",
                "of the Queen of the Dead",
                "and may ask me of any",
                "reward if I deem it fair.^000000"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("The Symbol of the Nine Realms...")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Lady Hell",
            args![
                "^8C1717The true Symbol of the",
                "Nine Realms cannot be freely",
                "given or lent. However, I sense",
                "your purpose and will give you",
                "a symbol imbued with enough",
                "power to be used only once.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lady Hell",
            args![
                "^8C1717With the symbol I give you,",
                "you can command the dead to",
                "carry out your will without",
                "question. However, keep in",
                "mind that after one use,",
                "its power will be consumed.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lady Hell",
            args![
                "^8C1717I have also decided",
                "to reward you with one",
                "more special permission.",
                "You may now freely draw",
                "water from the fountain",
                "in my mansion, brave hero.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args!["^3355FFYou received the", "Symbol of the Nine Realms.^000000"])?;
        ctx.call(Function::DelItem, vec![Val::from(7307), Val::from(1)])?;
        ctx.var("sign_q").set(Val::from(118))?;
        ctx.call(Function::GetItem, vec![Val::from(7305), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()?.number()? < 142 {
        ctx.lines(args![
            "^8C1717Greetings, mortal.",
            "Make sure that you",
            "use the power of the",
            "symbol wisely. You will",
            "be responsible for the",
            "consequences...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 142 {
        ctx.lines(args![
            "^8C1717Ah.",
            "You are the",
            "mortal called",
            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", are you not?")),
            "Yes, you are known to me.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Lady Hell",
            args![
                "^8C1717I've wanted to see you",
                "as I've sensed that you",
                "carry something which feels",
                "very familiar to me. Do you",
                "have something extraordinarily",
                "special in your possession?^000000"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("The Sign:Sobbing Starlight")])? {
            1 => {
                ctx.lines_as(
                    "Lady Hell",
                    args![
                        "^8C1717The Sign...?",
                        "Hmm, no, that's not",
                        "what I sensed. It's actually",
                        "something quite different...^000000"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Lady Hell",
                    args![
                        "^8C1717Yes, that's it!",
                        "I never expected a mortal",
                        "to have such an interesting",
                        ((Val::from("artifact in ")
                            + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                Val::from("his")
                            } else {
                                Val::from("her")
                            }))
                            + Val::from(" possession.^000000"))
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lady Hell",
                    args![
                        "^8C1717As queen of Niflheim,",
                        "I command you to lend the",
                        "Sobbing Starlight to me! Fear",
                        "not, I shall quickly return it. For a mortal, it must have",
                        "taken great pains to obtain this...^000000"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Please take it...:N-no! Don't take it!")])? {
                    1 => {
                        ctx.lines_as(
                            "Lady Hell",
                            args![
                                "^8C1717Thank you mortal.",
                                "Now, I shall show",
                                "you something truly",
                                "interesting...^000000"
                            ],
                        )?;
                        ctx.call(Function::DelItem, vec![Val::from(7178), Val::from(1)])?;
                        ctx.var("sign_q").set(Val::from(143))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Lady Hell",
                            args![
                                "^8C1717Ha ha ha!",
                                "You amuse me,",
                                "mortal! To think,",
                                "you even have courage",
                                "to refuse the queen of",
                                "Niflheim! Ha ha ha!^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Lady Hell",
                            args![
                                "^8C1717Mercy is not a quality",
                                "that I am known for, but",
                                "since you are my favored",
                                "mortal, I shall not kill you.",
                                "Still, it would be unwise",
                                "to displease me~^000000"
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("niflheim"), Val::from(29), Val::from(154)])?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    } else if ctx.var("sign_q").get()? == 143 {
        ctx.lines(args![
            "^8C1717As queen of this realm,",
            "I am unaccustomed to labor.",
            "But give me a moment to finish",
            "my work on this priceless",
            "artifact, mortal.^000000"
        ])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_GLASSWALL")?])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_NAPALMBEAT")?])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FIREPILLARBOMB")?])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LORD")?])?;
        ctx.next()?;
        ctx.lines_as(
            "Lady Hell",
            args![
                "^8C1717It is done.",
                "This is the true",
                "form of the object",
                "you humans call the",
                "Sobbing Starlight.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.var("sign_q").set(Val::from(144))?;
        ctx.call(Function::GetItem, vec![Val::from(7025), Val::from(1)])?;
        ctx.lines_as("Lady Hell", args!["^8C1717Although you may also know", "this object as God's Tear Drop, keep in mind that history, as you humans know it, may actually be different than the truth.^000000"])?;
        ctx.next()?;
        ctx.lines_as(
            "Lady Hell",
            args![
                "^8C1717In other words, there",
                "are some older tales about",
                "the gods and their enemies that",
                "may have been confused and twisted as they were handed down from generation to generation.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lady Hell",
            args![
                "^8C1717That is all I can tell",
                "you for now, mortal. It will",
                "be your job to discover the",
                "truth of the legends...^000000"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^8C1717Greetings, mortal.",
            "Is the realm of the living",
            "that boring and tedious?",
            "Ha ha ha! Well, there shall",
            "always be a place for you",
            "here in Niflheim.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn queen_of_the_dead(ctx: &Ctx) -> Script {
    queen_of_the_dead_body(ctx, Vec::new()).map(|_| ())
}

fn depressing_man_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    if ctx.var("sign_q").get()?.number()? < 80 {
        ctx.lines_as(
            "????",
            args!["Leave me be,", "ruffian! I'm disinclined", "towards conversation", "at the moment."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("sign_q").get()?.number()? < 87 {
            ctx.lines_as(
                "????",
                args![
                    "Don't go judging",
                    "people based on just",
                    "how they look. Try to look",
                    "inside and find the truth",
                    "within their hearts."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("sign_q").get()? == 87 {
                if ctx.call(Function::CountItem, vec![Val::from(2642)])?.number()? > 1 {
                    ctx.lines_as(
                        "Gen",
                        args![
                            "You lookin' for",
                            "something? Crayu",
                            "must have sent you.",
                            "Alright then, let's get",
                            "straight to the point."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gen",
                        args!["So, why is it that you", "wanna become one", "of Valkyrie's chosen?"],
                    )?;
                    ctx.next()?;
                    'b1: {
                        let subject1 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("To prove my courage:For honor:To help people")],
                        )?);
                        let mut matched1 = false;
                        let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                            matched1 = true;
                        }
                        if matched1 {
                            if ctx.var("sign_sq").get()?.number()? > 2 {
                                ctx.var("sign_sq").set(Val::from(0))?;
                            } else {
                                ctx.var("sign_sq").set((ctx.var("sign_sq").get()? + Val::from(1)))?;
                            }
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                            matched1 = true;
                        }
                        if matched1 {
                            break 'b1;
                        }
                    }
                    ctx.lines_as("Gen", args!["Alright...", "Now, did Crayu", "tell you why I'm here?"])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
                        1 => {
                            ctx.lines_as(
                                "Gen",
                                args![
                                    "Good, then you",
                                    "already know what",
                                    "you need to do. Now go",
                                    "see the witch of Niflheim."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Gen",
                                args![
                                    "And, off the record,",
                                    "I think you should be",
                                    "really careful if you find",
                                    "yourself dealing with Serin..."
                                ],
                            )?;
                            ctx.var("sign_q").set(Val::from(88))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Gen",
                                args![
                                    "You know, I was an",
                                    "adventurer myself. In fact,",
                                    "I even came here to Niflheim",
                                    "for the ordeals set by the gods. However, I failed and ended up being bound to this realm..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Gen",
                                args![
                                    "I just wanted to warn you.",
                                    "Be careful and don't justify",
                                    "your greed for any sort of reward using the excuse that you're just gonna prove your courage.",
                                    "That's what I did..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Gen",
                                args![
                                    "Anyway, I've already talked",
                                    "too much. First, go and see",
                                    "the witch and ask her to",
                                    "help you out, okay?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Gen",
                                args![
                                    "One last thing. Off the",
                                    "record, I think you should",
                                    "be really careful if you find",
                                    "yourself dealing with Serin.",
                                    "She seems nice enough, but",
                                    "something's weird about her."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Gen",
                                args![
                                    "She's awfully persuasive,",
                                    "and the fact that she used",
                                    "to be a great wizard when she",
                                    "was alive bothers me. She's a lot different than the rest of the guys stuck in Niflheim..."
                                ],
                            )?;
                            ctx.var("sign_q").set(Val::from(88))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.lines_as("Gen", args!["Are you here", "to prove your", "courage to the gods?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gen",
                        args![
                            "You know, I was an",
                            "adventurer myself. In fact,",
                            "I even came here to Niflheim",
                            "for the ordeals set by the gods. However, I failed and ended up being bound to this realm..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gen",
                        args![
                            "I just wanted to warn you.",
                            "Be careful and don't justify",
                            "your greed for any sort of reward using the excuse that you're just gonna prove your courage.",
                            "That's what I did..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gen",
                        args![
                            "Anyway, I've already talked",
                            "too much. First, go and see",
                            "the witch and ask her to",
                            "help you out, okay?"
                        ],
                    )?;
                    ctx.var("sign_q").set(Val::from(88))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("sign_q").get()?.number()? < 117 {
                    ctx.lines_as(
                        "Gen",
                        args![
                            "Huh... I hear you're",
                            "looking for something",
                            "pretty important. It's going",
                            "to be a dangerous search."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gen",
                        args![
                            "They are plenty of",
                            "other parties that are",
                            "interested in the thing",
                            "which you're seeking..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("sign_q").get()? == 117 {
                    ctx.lines_as(
                        "Gen",
                        args![
                            "You must be a very",
                            "resourceful mortal.",
                            "Not even Lady Hell, with",
                            "all her power, could find",
                            "the soul of her mother",
                            "for a long time."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gen",
                        args![
                            "There's no need",
                            "for me to hide my",
                            "true identity any longer.",
                            "My real name is Ganglati,",
                            "servant of Lady Hell, the",
                            "ruler of Niflheim."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gen",
                        args![
                            "Her highness would",
                            "have words with you.",
                            "Shall I send you to her",
                            "mansion, Eljudnir,",
                            "right now?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
                        1 => {
                            ctx.lines_as(
                                "Gen",
                                args![
                                    "Remember to be",
                                    "careful and especially",
                                    "polite when you speak to",
                                    "Lady Hell. Otherwise, the",
                                    "consequences will be severe..."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("que_sign01"), Val::from(45), Val::from(20)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Gen",
                                args![
                                    "I understand.",
                                    "Meeting with a deity is",
                                    "no small matter. However,",
                                    "I advise you to make haste",
                                    "as her highness is eager",
                                    "to see you soon."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("sign_q").get()? == 118 {
                    ctx.lines_as(
                        "Gen",
                        args![
                            "You must be most favored",
                            "to receive the Symbol of the",
                            "Nine Realms. Of course, it",
                            "can be used only once, but",
                            "it is still a great honor."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gen",
                        args![
                            "Once again, I ask that",
                            "you be on guard against",
                            "Serin. She may already be",
                            "aware of what you plan to",
                            "do with this symbol..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("sign_q").get()?.number()? < 129 {
                    ctx.lines_as(
                        "Gen",
                        args![
                            "You must hurry and",
                            "stop the summoning",
                            "before your world in",
                            "cast into immense peril!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gen",
                        args![
                            "Although Lady Hell",
                            "is being rather blase",
                            "about this matter, I beseech",
                            "you to do what you can for",
                            "the realm of the living!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("sign_q").get()?.number()? > 200 {
                    ctx.lines_as(
                        "Gen",
                        args![
                            "I should have",
                            "known you couldn't",
                            "stop her. Still, I had",
                            "a little hope that you'd",
                            "be able to do it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gen",
                        args!["Perhaps I expected", "far too much of you.", "Goodbye for now, mortal."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Gen",
                        args![
                            "You've done a great",
                            "job of stopping Serin.",
                            "Excellent! Such great",
                            "service on behalf of",
                            "the realm of the living!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gen",
                        args![
                            "I'm pleased to inform",
                            "you that you've been",
                            "invited to Lady Hell's",
                            "mansion once again.",
                            "Are you ready to go?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("No, thanks.:Yes.")])? {
                        1 => {
                            ctx.lines_as(
                                "Gen",
                                args!["I see.", "Well then, come", "back to me when you", "feel fully prepared."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Gen",
                                args![
                                    "Good, good.",
                                    "Don't forget to",
                                    "speak to her highness",
                                    "with the utmost respect!"
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("que_sign01"), Val::from(45), Val::from(20)])?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn depressing_man(ctx: &Ctx) -> Script {
    depressing_man_body(ctx, Vec::new()).map(|_| ())
}

fn switch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    if ctx.call(Function::CountItem, vec![Val::from(7313)])? == 1 {
        ctx.call(Function::Warp, vec![Val::from("que_sign01"), Val::from(115), Val::from(135)])?;
        return Err(Stop::End);
    }
    if ctx.var("sign_q").get()?.number()? > 89 {
        if ctx.var("sign_q").get()? == 126 {
            ctx.var("sign_q").set(Val::from(198))?;
        }
        ctx.call(Function::Warp, vec![Val::from("que_sign01"), Val::from(115), Val::from(135)])?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn switch(ctx: &Ctx) -> Script {
    switch_body(ctx, Vec::new()).map(|_| ())
}

fn mad_man_s_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_select_s = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Laichin]")?;
    if ctx.var("sign_q").get()?.number()? < 82 {
        ctx.lines(args![
            "What? I look familiar to you?",
            "Eh, you know what they say.",
            "Everyone has a lookalike in the",
            "world somewhere. Am I right",
            "or am I right?"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("sign_q").get()?.number()? < 96 {
            ctx.lines(args![
                "Niflheim ain't such a bad",
                "place to live. Sure, Asgard's",
                "all pretty and stuff, but there's too many rules. In this place, you",
                "can enjoy a lot more freedom. After you get used to all the stink."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Laichin",
                args![
                    "Did I say 'stink?'",
                    "I meant... 'fragrance.'",
                    "Alright, so the gods expelled",
                    "me, but I really oughtta thank",
                    "'em for sending me here!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Laichin",
                args![
                    "What's that look for...?",
                    "What, you can't believe",
                    "that someone like me used",
                    "to live in Valhalla? Me neither! They really made a mistake when they put me up to live in Asgard!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("sign_q").get()? == 96 {
                ctx.lines(args![
                    "Angrboda...?",
                    "Yeah, yeah!",
                    "I remember hearing something",
                    "about her when I usedta hang",
                    "with the gods up in Asgard."
                ])?;
                ctx.next()?;
                ctx.lines_as("Laichin", args!["Yeah, those guys were pretty", "gangster about it. I mean, they took her soul, broke it in four pieces and stashed them in the deepest hole in Midgard!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Laichin",
                    args![
                        "Sure, she was a giant, but",
                        "they were pretty scared of her.",
                        "All her children ended up to be legendary monsters! Well, I'm",
                        "not sure if Lady Hell counts."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laichin",
                    args![
                        "That's pretty harsh.",
                        "I mean, even if you die,",
                        "your soul has no place to",
                        "rest. Angrboda's soul is",
                        "all cut up and stuff!"
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("So where are the soul pieces again?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Laichin",
                    args![
                        "What...? I just said,",
                        "the deepest underground",
                        "place in Midgard.",
                        "It's a dangerous joint",
                        "protected by gangs",
                        "of monsters."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laichin",
                    args![
                        "Wait, you tellin' me",
                        "that you're gonna find",
                        "all of Angrboda's soul?!",
                        "You're crazy! If they end up",
                        "missing, the gods'll know I was",
                        "the one who tipped you off!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laichin",
                    args![
                        "If I was gonna risk my soul,",
                        "I'd wanna enjoy the rest of my",
                        "afterlife as my much as I could",
                        "before the gods offed me. Man,",
                        "I'd need at least 40,000 Zeny to even have a decent time..."
                    ],
                )?;
                ctx.next()?;
                'b1: {
                    let subject1 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Pay Laichin 40,000 Zeny:Don't pay Laichin.")],
                    )?);
                    let mut matched1 = false;
                    let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                    if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                        matched1 = true;
                    }
                    if matched1 {
                        if ctx.var("Zeny").get()?.number()? < 40000 {
                            ctx.lines_as(
                                "Laichin",
                                args![
                                    "Hey...",
                                    "This is sooo",
                                    "not enough money",
                                    "for me to enjoy myself",
                                    "if my soul ends up getting",
                                    "busted by the gods."
                                ],
                            )?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Laichin",
                                args![
                                    "Now we're talkin'!",
                                    "You're a true hero!",
                                    "Right, you gotta go to the",
                                    "lowest part of Glast Heim to",
                                    "find Angrboda's soul pieces."
                                ],
                            )?;
                            ctx.next()?;
                            l_select_s = ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?;
                            if l_select_s.clone() == 1 {
                                ctx.lines_as(
                                    "Laichin",
                                    args![
                                        "Check out the",
                                        "big 1 o' clock,",
                                        "big 5 o' clock,",
                                        "big 7 o' clock and",
                                        "the small 6 o' clock."
                                    ],
                                )?;
                                ctx.next()?;
                            } else if l_select_s.clone() == 2 {
                                ctx.lines_as(
                                    "Laichin",
                                    args![
                                        "Check out the",
                                        "big 4 o' clock,",
                                        "big 10 o' clock,",
                                        "small 6 o' clock and the",
                                        "very small 11 o' clock."
                                    ],
                                )?;
                                ctx.next()?;
                            } else if l_select_s.clone() == 3 {
                                ctx.lines_as(
                                    "Laichin",
                                    args![
                                        "Check out the",
                                        "big 5 o' clock,",
                                        "small 11 o' clock,",
                                        "small 6 o' clock and the",
                                        "very small 11 o' clock."
                                    ],
                                )?;
                                ctx.next()?;
                            } else if l_select_s.clone() == 4 {
                                ctx.lines_as(
                                    "Laichin",
                                    args![
                                        "Check out the",
                                        "big 4 o' clock,",
                                        "big 5 o' clock,",
                                        "big 10 o' clock and",
                                        "the small 11 o' clock."
                                    ],
                                )?;
                                ctx.next()?;
                            } else {
                                ctx.lines_as("Laichin", args!["Heh heh!"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Laichin",
                                args![
                                    "Her soul pieces are really",
                                    "well hidden, so you gotta look around, even if you know the general location. And don't",
                                    "tell anyone else about these locations or there'll be trouble."
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("I promise.:What do you mean by big and small?")])? {
                                1 => {
                                    ctx.lines_as("Laichin", args!["Alright, kid.", "Have fun!"])?;
                                    if l_select_s.clone() == 1 {
                                        ctx.var("sign_q").set(Val::from(97))?;
                                    } else if l_select_s.clone() == 2 {
                                        ctx.var("sign_q").set(Val::from(98))?;
                                    } else if l_select_s.clone() == 3 {
                                        ctx.var("sign_q").set(Val::from(99))?;
                                    } else if l_select_s.clone() == 4 {
                                        ctx.var("sign_q").set(Val::from(100))?;
                                    } else {
                                        ctx.lines_as("Laichin", args!["Heh heh!"])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(40000))?))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Laichin",
                                        args![
                                            "What do I mean by",
                                            "big and small? Hey man,",
                                            "the answer to that is worth",
                                            "at least... 20,000 Zeny.",
                                            "If you wanna know,",
                                            "cough up the cash!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Don't pay him.:Pay him.")])? {
                                        1 => {
                                            ctx.lines_as(
                                                "Laichin",
                                                args![
                                                    "Fine, fine.",
                                                    "Just don't come",
                                                    "crawling back when",
                                                    "you can't figure out",
                                                    "where the soul pieces are!"
                                                ],
                                            )?;
                                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(40000))?))?;
                                            if l_select_s.clone() == 1 {
                                                ctx.var("sign_q").set(Val::from(97))?;
                                            } else if l_select_s.clone() == 2 {
                                                ctx.var("sign_q").set(Val::from(98))?;
                                            } else if l_select_s.clone() == 3 {
                                                ctx.var("sign_q").set(Val::from(99))?;
                                            } else if l_select_s.clone() == 4 {
                                                ctx.var("sign_q").set(Val::from(100))?;
                                            } else {
                                                ctx.lines_as("Laichin", args!["Hehehehe~"])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            if ctx.var("Zeny").get()?.number()? < 60000 {
                                                ctx.lines_as("Laichin", args!["What is this?", "You tryin to welch", "me or somethin'?"])?;
                                                ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                ctx.lines_as(
                                                    "Laichin",
                                                    args![
                                                        "Alright, listen up.",
                                                        "Big, small and very small",
                                                        "mean the distances from the",
                                                        "center of the map. So for big,",
                                                        "I mean look near the border of",
                                                        "the map. Easy, huh?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Laichin",
                                                    args![
                                                        "When I say small, you",
                                                        "gotta look in areas closer",
                                                        "than the borders of the map.",
                                                        "For very small, you gotta",
                                                        "look near the center. Got it?"
                                                    ],
                                                )?;
                                                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(60000))?))?;
                                                if l_select_s.clone() == 1 {
                                                    ctx.var("sign_q").set(Val::from(101))?;
                                                } else if l_select_s.clone() == 2 {
                                                    ctx.var("sign_q").set(Val::from(102))?;
                                                } else if l_select_s.clone() == 3 {
                                                    ctx.var("sign_q").set(Val::from(103))?;
                                                } else if l_select_s.clone() == 4 {
                                                    ctx.var("sign_q").set(Val::from(104))?;
                                                } else {
                                                    ctx.lines_as("Laichin", args!["Eh heh", "heh heh heh!"])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as("Laichin", args!["Huh...?", "Okay pal."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            } else {
                if (((ctx.var("sign_q").get()? == 97 || ctx.var("sign_q").get()? == 98) || ctx.var("sign_q").get()? == 99)
                    || ctx.var("sign_q").get()? == 100)
                {
                    ctx.lines(args![
                        "Oh yeah, about those",
                        "directions I gave you",
                        "last time? They work, but",
                        "you gotta know what I mean",
                        "by 'big' or 'small' whatever",
                        "o' clock means. That's right... "
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Laichin",
                        args![
                            "Matter of fact, I'll",
                            "tell you all about it.",
                            "Right after you pay me,",
                            "oh, I don't know, ^FF000020,000 zeny^000000."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Pay him.:Don't pay him.")])? {
                        1 => {
                            if ctx.var("Zeny").get()?.number()? < 20000 {
                                ctx.lines_as("Laichin", args!["What is this?", "You tryin to welch", "me or somethin'?"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Laichin",
                                    args![
                                        "Alright, listen up.",
                                        "Big, small and very small",
                                        "mean the distances from the",
                                        "center of the map. So for big,",
                                        "I mean look near the border of",
                                        "the map. Easy, huh?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Laichin",
                                    args![
                                        "When I say small, you",
                                        "gotta look in areas closer",
                                        "than the borders of the map.",
                                        "For very small, you gotta",
                                        "look near the center. Got it?"
                                    ],
                                )?;
                                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(20000))?))?;
                                if ctx.var("sign_q").get()? == 97 {
                                    ctx.var("sign_q").set(Val::from(101))?;
                                } else if ctx.var("sign_q").get()? == 98 {
                                    ctx.var("sign_q").set(Val::from(102))?;
                                } else if ctx.var("sign_q").get()? == 99 {
                                    ctx.var("sign_q").set(Val::from(103))?;
                                } else if ctx.var("sign_q").get()? == 100 {
                                    ctx.var("sign_q").set(Val::from(104))?;
                                } else {
                                    ctx.lines_as("Laichin", args!["Bwahahaha!"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        2 => {
                            ctx.lines_as("Laichin", args!["Whatever, dude!", "Freakin' cheapskate..."])?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("sign_q").get()?.number()? < 105 {
                    ctx.lines(args![
                        "Oh yeah. Uh, I forgot to tell",
                        "you that Angrboda's soul pieces are sealed with the power of the gods. You can't just smash them open."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Laichin",
                        args![
                            "...Or maybe you could.",
                            "Anyway, it'll be better",
                            "if you had a weapon that",
                            "was solid, heavy and powerful."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Laichin",
                        args![
                            "But yeah, think about",
                            "what it means to break",
                            "a godly seal. Remember there's",
                            "^FF0000some kinda rule^000000 that the gods imposed which you gotta follow",
                            "to release Angrboda's soul."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Laichin",
                        args![
                            "Ah, right. If you wanna",
                            "try different weapons for",
                            "breaking those seals, make",
                            "sure you got 'em in your",
                            "inventory and that they're",
                            "not equipped, alright?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Thanks~!:Where am I supposed to go again?")])? {
                        1 => {
                            ctx.lines_as(
                                "Laichin",
                                args![
                                    "Oh no...",
                                    "Thank you for all",
                                    "of this zeny! Money",
                                    "might not be able to",
                                    "buy happiness, but it",
                                    "sure comes real close!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Laichin",
                                args![
                                    "Hey... You gotta",
                                    "go to the lowest part",
                                    "of Glast Heim to find all",
                                    "of Angrboda's soul pieces.",
                                    "Once you get there..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.mes("[Laichin]")?;
                            if ctx.var("sign_q").get()? == 101 {
                                ctx.lines(args![
                                    "Check out the",
                                    "big 1 o' clock,",
                                    "big 5 o' clock,",
                                    "big 7 o' clock and",
                                    "the small 6 o' clock."
                                ])?;
                            } else if ctx.var("sign_q").get()? == 102 {
                                ctx.lines(args![
                                    "Check out the",
                                    "big 4 o' clock,",
                                    "big 10 o' clock,",
                                    "small 6 o' clock and the",
                                    "very small 11 o' clock."
                                ])?;
                            } else if ctx.var("sign_q").get()? == 103 {
                                ctx.lines(args![
                                    "Check out the",
                                    "big 5 o' clock,",
                                    "small 11 o' clock,",
                                    "small 6 o' clock and the",
                                    "very small 11 o' clock."
                                ])?;
                            } else if ctx.var("sign_q").get()? == 104 {
                                ctx.lines(args![
                                    "Check out the",
                                    "big 4 o' clock,",
                                    "big 5 o' clock,",
                                    "big 10 o' clock and",
                                    "the small 11 o' clock."
                                ])?;
                            }
                            ctx.lines(args!["And break those seals", "in that order, okay?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Laichin",
                                args![
                                    "Big, small and very small",
                                    "mean the distances from the",
                                    "center of the map. So for big,",
                                    "I mean look near the border of",
                                    "the map. Easy, huh?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Laichin",
                                args![
                                    "When I say small, you",
                                    "gotta look in areas closer",
                                    "than the borders of the map.",
                                    "For very small, you gotta",
                                    "look near the center. Got it?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("sign_q").get()?.number()? < 150 {
                    ctx.lines(args![
                        "Dude, Niflheim rocks.",
                        "It's like, a million times",
                        "better than that boring",
                        "old Asgard. Hell yeah!"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Laichin",
                        args![
                            "Eh, I don't even wanna",
                            "know if you manage to",
                            "find all of Angrboda's soul",
                            "pieces, but if you do, don't",
                            "go bringin' close to me. If the",
                            "gods find out you have 'em... "
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Laichin",
                        args![
                            "But for the sake of",
                            "argument, let's say you",
                            "do happen to get them all.",
                            "In that case, you oughta get",
                            "Angrboda's soul to the Queen",
                            "of Hell right away, alright?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Laichin",
                        args![
                            "Ooh.. But not just any",
                            "yahoo can waltz up to her.",
                            "The best thing would be to",
                            "give it to one of her really",
                            "trusted retainers, I guess.",
                            "Now, who was he again?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Laichin",
                        args![
                            "I know he's always in",
                            "disguise as some sorta",
                            "really ^666666despressing guy^000000",
                            "and I can't remember his",
                            "name for the life of me, but",
                            "he's around here somewhere..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("sign_q").get()? == 200 {
                    ctx.lines(args!["Whoa...", ".................", "My freakin' head hurts."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines(args!["Whoa...", "................."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn mad_man_s(ctx: &Ctx) -> Script {
    mad_man_s_body(ctx, Vec::new()).map(|_| ())
}
