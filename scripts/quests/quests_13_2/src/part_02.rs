use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn splendide_guard_ep13md01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.lines(args![
            "Wait a moment!!",
            "You have too many items.",
            "You can't receive this.",
            "Lighten your weight and",
            "try again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 && ctx.var("ep13_2_rhea").get()?.number()? > 99) {
        if ctx.var("ep13_mdrama").get()? == 0 {
            ctx.lines_as(
                "Splendide Guard",
                args!["Outsider?", "Outsiders have been coming more frequently."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Splendide Guard",
                args!["It seems like our superiors have permitted your entrance, so I won't stop you either."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arc",
                args!["My name is Arc.", "By the way, hmm... Can you understand what I'm saying?"],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Yes.:Shake my head")])? {
                1 => {
                    ctx.lines_as(
                        "Arc",
                        args![
                            "What? Did you just say 'Yes'?",
                            "Can't believe we understand eachother!",
                            "When did you learn our language?",
                            "What is your race called?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("- I show my ring to Arc and explain everything including how I ended up here... -")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Arc",
                        args![
                            "So...hmm, That's how it is...",
                            "The source of that huge shock and mysterious explosion..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Arc",
                        args![
                            "Quite an interesting story.",
                            "That's why you humans were exploring here and there..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Arc",
                        args![
                            "Now I clearly understand that you are not sent by the giants of Manuk.",
                            "Thank you for telling me these interesting stories."
                        ],
                    )?;
                    ctx.var("ep13_mdrama").set(Val::from(1))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Arc",
                        args![
                            "Oh? Is that so?",
                            "We use different languages as expected...",
                            "That's a bit frustrating."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            if ctx.var("ep13_mdrama").get()? == 1 {
                ctx.lines_as("Arc", args!["By the way, you...", "No, never mind...", "What is your name?"])?;
                ctx.next()?;
                let choice = runtime::select_values(
                    ctx,
                    &[((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(""))],
                )?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Arc",
                    args![
                        ((Val::from("Ah, right. ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                        "That's a strange pronunciation.",
                        "I might mispronunce it, so please understand..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Arc", args!["Anyways, I have a favor to ask you. Is it okay?"])?;
                ctx.next()?;
                'b2: {
                    let subject2 = Val::from(runtime::select_values(ctx, &[Val::from("What favor?:Not now.")])?);
                    let mut matched2 = false;
                    let no_case2 = !subject2.loosely_equals(&Val::from(1)) && !subject2.loosely_equals(&Val::from(2));
                    if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines_as("Arc", args!["Um...Ah..It's...", "A little...complicated..."])?;
                        ctx.next()?;
                        ctx.mes("- Arc looks around to check if someone else is around. Satisfied you're alone he continues with his story in a low tone. -")?;
                        ctx.next()?;
                        ctx.lines_as("Arc", args!["Frankly, I'm worried about a friend of mine who left without permission saying that she's going to check on your camp. She hasn't come back since then."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Arc",
                            args![
                                "But, I can't leave this post to look for her 'cause I have a duty to guard the research data stored here."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Arc", args!["Since she left without permission, I can't even report to my superiors... that will just end up as a bigger problem."])?;
                        ctx.next()?;
                        ctx.lines_as("Arc", args!["The thought of it just worries me...", "As a matter of fact, we are trying not to have any contact with the giants of Manuk so we stay away from each other's territory."])?;
                        ctx.next()?;
                        ctx.lines_as("Arc", args!["That camp of yours is located in the area that has been acting as the neutral zone between our two races..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Arc",
                            args![
                                "If she has gotten close to their side then she might have gone over to the snow fields...",
                                "That will make it even more difficult for us to look for her..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Arc",
                            args!["The Manuk giants might misunderstand this as provoking them...Couldn't they?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "So...the point is, you want me to look for your friend. Right?",
                                "Because you can't do it yourself from all these complicated situations?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Arc", args!["Exactly!", "You're quite observant."])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Sure, I will do it.:Sorry, can't help you.")])? {
                            1 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["But, I'm going to need more information if I'm going to look for her."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Arc",
                                    args![
                                        "You don't have to worry about that...",
                                        "^4d4dffTerra^000000 has a habit of making a knot out of plants to mark her destination."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Arc",
                                    args![
                                        "She does it so that she won't get lost.",
                                        "If you find a trail of knotted plants it should lead you to where she is."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Arc",
                                    args![
                                        "It can be hard to look for them, but it's better than doing nothing...",
                                        "Please, I beg of you."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Arc", args!["Ah, and please keep this a secret from the other Laphines."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Arc",
                                    args![
                                        ((Val::from("Remember ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                            + Val::from("... you must keep this to yourself...")),
                                        "Please, find her."
                                    ],
                                )?;
                                ctx.var("ep13_mdrama").set(Val::from(2))?;
                                ctx.call(Function::SetQuest, vec![Val::from(7056)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Arc",
                                    args![
                                        "It sure was an unreasonable favor...",
                                        "You too were sent here to carry on a mission...",
                                        "That was thoughtless of me..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines_as(
                            "Arc",
                            args!["Is that so? Of course.", "You were sent here to carry on your own mission..."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            } else {
                if ctx.var("ep13_mdrama").get()? == 2 {
                    ctx.lines_as(
                        "Arc",
                        args![
                            "^4d4dffTerra^000000 has a habit of making a knot out of plants to mark her destination.",
                            "Keep that in mind when searching for her. And don't tell any other Laphine's about her missing."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Arc",
                        args![
                            ((Val::from("You ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(" are the only one I could ask for help...")),
                            "Do me this favor please."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if (ctx.var("ep13_mdrama").get()?.number()? > 3 && ctx.var("ep13_mdrama").get()?.number()? < 7) {
                        ctx.lines_as(
                            "Arc",
                            args![
                                "Did you find anything?",
                                "Have you found any knotted plants?",
                                "Yeah, that's the mark",
                                "that Terra leaves."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Arc", args!["Keep it up and find her", "for me."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("ep13_mdrama").get()? == 7 {
                            ctx.lines_as("Arc", args!["Ah, Well met!", "What do I do now?!"])?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("What? What happened?")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.mes("- Arc looks like he was thrown into confusion about something. -")?;
                            ctx.next()?;
                            ctx.lines_as("Arc", args!["Terra..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Arc",
                                args!["Terra came back!!!", "but, she was covered all over with wounds..."],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("Covered with wounds?")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                "Arc",
                                args![
                                    "She didn't even get treatment!",
                                    "She just took the Bradium with that exhausted body!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Arc",
                                args![
                                    "Didn't even tell me the reason!",
                                    "I told her no, but she wouldn't listen to me!",
                                    "It happened all of a sudden."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Arc",
                                args![
                                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                                    "What shoud I do now?",
                                    "What do I have to do?",
                                    "Terra left again and she was hurt..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Arc", args!["And I just stood here...doing nothing."])?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("You have a strong sense of responsibility.")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                "Arc",
                                args![
                                    "......anyways.",
                                    "I couldn't either stop her, or follow her...",
                                    "What now...?",
                                    "Why did she take the Bradium...?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.mes("- You tell Arc about the Giant from the cave. -")?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Arc",
                                args![
                                    "What? Is that what happened?",
                                    "Bradium...is certainly a precious ore for Sapha.",
                                    "They use a refined Bradium ore..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Arc",
                                args![
                                    "You said that a Sapha was hurt?",
                                    "And its body was already stiff?",
                                    "Impossible!",
                                    "Terra!!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.mes("- Arc suddenly began gathering up his gear. -")?;
                            ctx.var("ep13_mdrama").set(Val::from(8))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("ep13_mdrama").get()? == 8 {
                                ctx.lines_as(
                                    "Arc",
                                    args![
                                        "Where was that cave where you found that Sapha?",
                                        "There was no such cave when we inspected our surrounding areas."
                                    ],
                                )?;
                                ctx.next()?;
                                let choice = runtime::select_values(ctx, &[Val::from("What are you trying to do?")])?;
                                ctx.var("@menu").set(choice)?;
                                ctx.lines_as("Arc", args!["Can't you see?", "I'm going out to look for Terra. She took the Bradium with her, it has got to be something to do with that Sapha."])?;
                                ctx.next()?;
                                let choice = runtime::select_values(ctx, &[Val::from("Is it okay for you to leave your post?")])?;
                                ctx.var("@menu").set(choice)?;
                                ctx.lines_as("Arc", args!["It's.. ...", "...No....", "Shit, then what should I do!!"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "I will go. I'm the one who knows exactly where that cave is.",
                                        "I will go and bring her back.",
                                        "If Terra headed for that cave, I should be able to find her."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Let's hear rest of the story from Terra.",
                                        "It is important for you to guard this place, isn't it?",
                                        "If Terra really took the Bradium without any permission..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Arc",
                                    args![
                                        "The higher ups won't just remain still...",
                                        "I'm sorry to ask you but...",
                                        "Please go and bring her back here..."
                                    ],
                                )?;
                                ctx.var("ep13_mdrama").set(Val::from(9))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(7058), Val::from(7059)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if (ctx.var("ep13_mdrama").get()?.number()? > 8 && ctx.var("ep13_mdrama").get()?.number()? < 13) {
                                    ctx.lines_as(
                                        "Arc",
                                        args!["I'm really sorry for getting you into this mess.....", "...Please, find Terra."],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("ep13_mdrama").get()? == 13 {
                                        ctx.lines_as(
                                            "Arc",
                                            args![
                                                "What? A wounded Laphine in the Sapha's village?",
                                                "Was it Terra?! Was she captured by those Sapha bastards?!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args![
                                                "Calm down. I couldn't check out exactly who the captured Laphine was.",
                                                "And they seem to have no intention of harming the Laphine."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args![
                                                "Also, they were looking for something that can cure a wounded Laphine...",
                                                "If we bring some medicine to them, we might have a chance to see who the Laphine is."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I understand how anxious you feel.. but that is the only way I can think of finding out who it is."])?;
                                        ctx.next()?;
                                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I will take some things to treat the wounds with me and if it turns out that the captured Laphine is Terra, I will bring her back."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Arc",
                                            args!["..........", "OK, that seems like the best thing we can do for this situation..."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Arc",
                                            args![
                                                "Do you by any chance know about the 'Yggdrasil'?",
                                                "There's nothing better than the Yggdrasilberry to treat wounds...",
                                                "Especially for us Laphine..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Arc",
                                            args![
                                                "If you have a Yggdrasilberry, you can use it to cure that Laphine...",
                                                ".Yggdrasil is the tree of life... its roots are in touch with everything in this world."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Arc", args!["I have...", "...only three Yggdrasilberries..."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Arc", args!["I will give you these but they won't be enough...", "Outside... there is a huge tree... where the root of the Yggdrasil is exposed.... it should be there."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Arc",
                                            args![
                                                "Sometimes you can find Yggdrasilberries around there.",
                                                "Go find few more and bring them with you. Hurry!"
                                            ],
                                        )?;
                                        ctx.var("ep13_mdrama").set(Val::from(14))?;
                                        ctx.call(Function::GetItem, vec![Val::from(607), Val::from(3)])?;
                                        ctx.call(Function::ChangeQuest, vec![Val::from(7062), Val::from(7063)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if (ctx.var("ep13_mdrama").get()?.number()? > 13 && ctx.var("ep13_mdrama").get()?.number()? < 18) {
                                            ctx.lines_as(
                                                "Arc",
                                                args![
                                                    "There's a huge root of the tree near a swamp at the outskirts of this area.",
                                                    "That's where the tree of life's root is exposed."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Arc", args!["You should be able to find some Yggdrasilberries there."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Arc",
                                                args!["You will need at least 6~7 of them...", ".... .. I hope she is safe.."],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Arc", args!["As soon as you are done collecting the berries, bring them to the Sapha Village right away!!!", "Terra... please be safe."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if (ctx.var("ep13_mdrama").get()?.number()? > 17
                                            && ctx.var("ep13_mdrama").get()?.number()? < 25)
                                        {
                                            ctx.lines_as("Arc", args!["...Oh where is Terra...?"])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if ctx.var("ep13_mdrama").get()? == 25 {
                                            ctx.lines_as(
                                                "Arc",
                                                args![
                                                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                        + Val::from("!!")),
                                                    "Terra's back!!!",
                                                    "She is back in one piece!",
                                                    "...She hasn't been saying a single word since her return, I told her to rest in Yai..."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Arc",
                                                args![
                                                    "Please come by if you don't mind...",
                                                    "Terra's Yai.",
                                                    "Which is her Private Residence... It's located at the Southeast direction from here.",
                                                    "I would like to hear a detailed account of what happened."
                                                ],
                                            )?;
                                            ctx.var("ep13_mdrama").set(Val::from(26))?;
                                            ctx.call(Function::ChangeQuest, vec![Val::from(7070), Val::from(7071)])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if ctx.var("ep13_mdrama").get()? == 26 {
                                            ctx.lines_as(
                                                "Arc",
                                                args![
                                                    "What happened....while she was gone?",
                                                    "I would like to hear a detailed account fn what happened...",
                                                    "Please go to Terra's Yai later."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if ctx.var("ep13_mdrama").get()?.number()? > 26 {
                                            ctx.lines_as(
                                                "Arc",
                                                args![
                                                    "I really appreciate what you've done for us.",
                                                    "But, I'm worried about Terra.",
                                                    "I hope she can be her old self..."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            ctx.lines_as(
                                                "Arc",
                                                args!["Ah~Ah~Stop right there...", "This area is off-limits to unauthorized personnel."],
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
    } else {
        if ctx.var("ep13_mdrama").get()?.number()? > 0 {
            ctx.lines_as(
                "Arc",
                args![
                    "Yur,Dur AnoVa?",
                    "Wha? Dieb OsaDur .. ",
                    "ah..RuffThus NeAsh. man nesAsh OdesAlah ?"
                ],
            )?;
            ctx.next()?;
            ctx.mes("- I can't understand what he's saying... Oh god how frustrating. -")?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Splendide Guard",
                args!["NeiVil !", "narNoth nesMush.", "AnuDur AmanDana Goth nar!"],
            )?;
            ctx.next()?;
            ctx.mes("- We don't seem to understand eachother. -")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    return Err(Stop::End);
}

pub fn splendide_guard_ep13md01(ctx: &Ctx) -> Script {
    splendide_guard_ep13md01_body(ctx, Vec::new()).map(|_| ())
}

fn splendide_guard_ep13md01_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        if ctx.var("ep13_mdrama").get()?.number()? < 1 {
            ctx.lines_as(
                "Splendide Guard",
                args![
                    "Halt.",
                    "This area is restricted unless you have been given permission.",
                    "Especially to outsiders like you."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as("Splendide Guard", args!["Tal-!", "AnuDur AmanDana Goth nar!", "Agoltas Me..."])?;
        ctx.next()?;
        ctx.mes("- Looks like we don't understand each other. This could be troublesome... -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn splendide_guard_ep13md01_ontouch(ctx: &Ctx) -> Script {
    splendide_guard_ep13md01_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn ep13_mdplant01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_mdrama").get()? == 2 {
        ctx.call(Function::Cutin, vec![Val::from("ep13_plant01"), Val::from(2)])?;
        ctx.lines(args![
            "Weeds are easy to find around here...",
            "A closer look reveals that there's one knotted stem...",
            "It looks like this is Terra's trace that Arc told me about."
        ])?;
        ctx.next()?;
        ctx.mes("A knotted leaf is pointing toward the^4d4dff Southern^000000 direction.")?;
        ctx.var("ep13_mdrama").set(Val::from(3))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(7056), Val::from(7057)])?;
        ctx.close_window()?;
    } else if ctx.var("ep13_mdrama").get()?.number()? > 2 {
        ctx.lines(args![
            "This is a marking Terra left to remember the way back home.",
            "A knotted leaf is pointing^4d4dff South^000000."
        ])?;
        ctx.close_window()?;
    } else {
        ctx.lines(args![
            "Weeds are easy to find around here...",
            "There's nothing too special about it."
        ])?;
        ctx.close_window()?;
    }
    ctx.call(Function::Cutin, vec![Val::from("ep13_plant01"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn ep13_mdplant01(ctx: &Ctx) -> Script {
    ep13_mdplant01_body(ctx, Vec::new()).map(|_| ())
}

fn ep13_mdplant02_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_mdrama").get()?.number()? > 2 {
        ctx.call(Function::Cutin, vec![Val::from("ep13_plant01"), Val::from(2)])?;
        ctx.lines(args![
            "Weeds are easy to find around here...",
            "A closer look reveals that there's one knotted stem...",
            "It looks like this is Terra's trace that Arc told me about."
        ])?;
        ctx.next()?;
        ctx.mes("A knotted leaf is pointing ^4d4dff East^000000.")?;
        ctx.close_window()?;
    } else {
        ctx.lines(args![
            "Weeds are easy to find around here...",
            "There's nothing too special about it."
        ])?;
        ctx.close_window()?;
    }
    ctx.call(Function::Cutin, vec![Val::from("ep13_plant01"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn ep13_mdplant02(ctx: &Ctx) -> Script {
    ep13_mdplant02_body(ctx, Vec::new()).map(|_| ())
}

fn ep13_mdplant03_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_mdrama").get()?.number()? > 2 {
        ctx.call(Function::Cutin, vec![Val::from("ep13_plant01"), Val::from(2)])?;
        ctx.lines(args![
            "Weeds are easy to find around here...",
            "A closer look reveals that there's one knotted stem...",
            "It looks like this is Terra's trace that Arc told me about."
        ])?;
        ctx.next()?;
        ctx.mes("A knotted leaf is pointing ^4d4dff North^000000.")?;
        ctx.close_window()?;
    } else {
        ctx.lines(args![
            "Weeds are easy to find around here...",
            "There's nothing too special about it."
        ])?;
        ctx.close_window()?;
    }
    ctx.call(Function::Cutin, vec![Val::from("ep13_plant01"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn ep13_mdplant03(ctx: &Ctx) -> Script {
    ep13_mdplant03_body(ctx, Vec::new()).map(|_| ())
}

fn ep13_mdplant04_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_mdrama").get()? == 3 {
        ctx.lines(args![
            "There are footprints here as well as signs that someone has fallen.",
            "It looks like someone was fighting here?",
            "But who?"
        ])?;
        ctx.next()?;
        ctx.mes("There are footprints heading ^4d4dffNorth^000000.")?;
        ctx.var("ep13_mdrama").set(Val::from(4))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ep13_mdrama").get()?.number()? > 3 {
        ctx.lines(args![
            "Someone must have had a fight here.",
            "There are footprints heading ^4d4dffNorth^000000."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.mes("There are footprints here that seem to be leading towards something.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn ep13_mdplant04(ctx: &Ctx) -> Script {
    ep13_mdplant04_body(ctx, Vec::new()).map(|_| ())
}

fn ep13_mdplant05_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_mdrama").get()? == 4 {
        ctx.lines(args![
            "There's evidence that there was a fight here too.",
            "These footsteps can't be a Laphine's..."
        ])?;
        ctx.next()?;
        ctx.mes("There are footprints that lead to^4d4dff the Root of a huge tree^000000.")?;
        ctx.var("ep13_mdrama").set(Val::from(5))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ep13_mdrama").get()?.number()? > 4 {
        ctx.mes("There are footprints that lead to^4d4dff the Root of a huge tree^000000.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.mes("There are footprints here that seem to be leading towards something.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn ep13_mdplant05(ctx: &Ctx) -> Script {
    ep13_mdplant05_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ToDun01Ep132Step {
    Start,
    OnTouch,
}

fn to_dun01_ep13_2_run(ctx: &Ctx, mut step: ToDun01Ep132Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ToDun01Ep132Step::Start => {
                step = ToDun01Ep132Step::OnTouch;
                continue 'machine;
            }
            ToDun01Ep132Step::OnTouch => {
                if ctx.var("ep13_mdrama").get()? == 5 {
                    ctx.lines(args![
                        "There's a stem entangled inside an opening in the roots of a huge tree...",
                        "It looks like something slipped here."
                    ])?;
                    ctx.next()?;
                    ctx.mes("A cold breeze is blowing out from deep inside the tree.")?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Follow the trace.:Looks dangerous... head back.")])? {
                        1 => {
                            ctx.mes("You take a one step forward carefully through the muddy roots and slip.")?;
                            ctx.var("ep13_mdrama").set(Val::from(6))?;
                            ctx.close_window()?;
                            ctx.call(Function::PercentHeal, vec![Val::from(-30), Val::from(0)])?;
                            ctx.call(Function::Warp, vec![Val::from("nyd_dun01"), Val::from(72), Val::from(125)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines(args![
                                "It's too dark to see anything inside...but it feels like there's a big hole at the bottom.",
                                "I'd better step back...It looks dangerous."
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("ep13_mdrama").get()?.number()? > 5 {
                    ctx.mes("Between huge roots, there is a hole leads to the underground cave.")?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Go inside.:I'm not going in there.")])? {
                        1 => {
                            ctx.mes("Again, you slip through the muddy roots.")?;
                            ctx.close_window()?;
                            ctx.call(Function::PercentHeal, vec![Val::from(-30), Val::from(0)])?;
                            ctx.call(Function::Warp, vec![Val::from("nyd_dun01"), Val::from(72), Val::from(125)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("You decide to come back later.")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.lines(args![
                        "Strange looking stems are entangled inside an opening of huge roots.",
                        "Surface looks unstable...I should step away before I slip on it."
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn to_dun01_ep13_2(ctx: &Ctx) -> Script {
    to_dun01_ep13_2_run(ctx, ToDun01Ep132Step::Start, Vec::new()).map(|_| ())
}

pub fn to_dun01_ep13_2_ontouch(ctx: &Ctx) -> Script {
    to_dun01_ep13_2_run(ctx, ToDun01Ep132Step::OnTouch, Vec::new()).map(|_| ())
}

fn petrified_sapha_ep13md03_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.lines(args![
            "Wait a moment!!",
            "You have too many items.",
            "You can't receive this.",
            "Lighten your weight and",
            "try again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_mdrama").get()? == 6 {
        ctx.mes("This is the only spot where sunlight is shining down through a hole in the ceiling.")?;
        ctx.next()?;
        ctx.mes("Looking up I see the roots of the tree and stems digging up a mud-plastered wall, so the sunlight could shine through....2.j...There is a statue of Giant on the ground.")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["A statue in a place like this...", "Interesting."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Hm, This resembles... that giant tribe in Manuk...",
                "Huh? It's wearing a muffler...",
                "...It can't be!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["It has a lot of scars on it... as if it had been in a severe fight..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "And it appears like it's holding something in its arms...",
                "Just what happened here?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "I can't imagine what happened here, but I don't see Terra's marks anymore...",
                "and this cave looks unsafe... I'd better go back to Arc for now."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Could it be... Terra was fighting this Giant?"],
        )?;
        ctx.var("ep13_mdrama").set(Val::from(7))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(7057), Val::from(7058)])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Anyways... How do I get out of here?",
                "That hole I came in is too slippery to climb back out..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Light is coming in from the ceiling above... Maybe I should just climb a stem around here..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if (ctx.var("ep13_mdrama").get()? == 7 || ctx.var("ep13_mdrama").get()? == 8) {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["I'm done with this place.", "I should go back to Arc...", "Let's find an exit."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("ep13_mdrama").get()? == 9 {
                ctx.lines(args!["I came back here, but there's no trace of Terra...", "There's only one thing different from the last time..Small particles of an ore are sprinkled around the Sapha's dead body."])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "So...Terra was here.....",
                        "Powdered Bradium...",
                        "What was she trying to do with this Sapha?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "I can't just go back to Arc without anything... he'd be so disappointed...",
                        "Hmm, Should I go to the Sapha's village?"
                    ],
                )?;
                ctx.var("ep13_mdrama").set(Val::from(10))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(7059), Val::from(7060)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("ep13_mdrama").get()? == 10 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "I can't find any traces of Terra anymore.",
                            "One thing that bugs me is...These particles of an ore."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I should go back to Arc and ask about it, or go to Sapha's Village and investigate some more."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("ep13_mdrama").get()?.number()? > 10 && ctx.var("ep13_mdrama").get()?.number()? < 21) {
                    ctx.lines(args![
                        "A giant from the race called Sapha is petrified here.",
                        "It looks like it's dead..."
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("ep13_mdrama").get()? == 21 {
                    ctx.lines(args![
                        "I need to bring back some evidence of this giant in the cave.",
                        "What should I bring with me?"
                    ])?;
                    ctx.next()?;
                    'b1: {
                        let subject1 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Hair:Muffler:Pants:Fragment of Bradium")],
                        )?);
                        let mut matched1 = false;
                        let no_case1 = !subject1.loosely_equals(&Val::from(1))
                            && !subject1.loosely_equals(&Val::from(2))
                            && !subject1.loosely_equals(&Val::from(3))
                            && !subject1.loosely_equals(&Val::from(4));
                        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.mes("It won't come off. It's as if it is a tree rooted on a rock.")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.lines(args![
                                "You carefully strip a worn muffler off of the Sapha's neck.",
                                "We will see if this muffler belongs to Ogen or not."
                            ])?;
                            ctx.next()?;
                            ctx.mes("And...what else should I take?")?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Hair:Pants:Fragment of Bradium")])? {
                                1 => {
                                    ctx.mes("It won't come off. It's as if it is a tree rooted on a rock.")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.mes("... I don't want to take off a dead MAN's pants...")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                3 => {
                                    ctx.lines(args![
                                        "You pick up a fragment of Bradium scattered on the ground.",
                                        "This should be enough."
                                    ])?;
                                    ctx.var("ep13_mdrama").set(Val::from(22))?;
                                    ctx.call(Function::GetItem, vec![Val::from(6085), Val::from(1)])?;
                                    ctx.call(Function::GetItem, vec![Val::from(6084), Val::from(1)])?;
                                    ctx.call(Function::ChangeQuest, vec![Val::from(7066), Val::from(7067)])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.mes("... I don't want to take off a dead MAN's pants...")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(4)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.lines(args![
                                "You pick up a fragment of Bradium scattered on the ground.",
                                "This will prove Terra's effort...",
                                "And what else?"
                            ])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Hair:Muffler:Pants")])? {
                                1 => {
                                    ctx.mes("It won't come off. It's as if it is a tree rooted on a rock.")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines(args![
                                        "You carefully strip a worn muffler off of the Sapha's neck.",
                                        "We will see if this muffler belongs to Ogen or not...",
                                        "This should be enough."
                                    ])?;
                                    ctx.var("ep13_mdrama").set(Val::from(22))?;
                                    ctx.call(Function::GetItem, vec![Val::from(6085), Val::from(1)])?;
                                    ctx.call(Function::GetItem, vec![Val::from(6084), Val::from(1)])?;
                                    ctx.call(Function::ChangeQuest, vec![Val::from(7066), Val::from(7067)])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                3 => {
                                    ctx.mes("... I don't want to take off a dead MAN's pants...")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        }
                    }
                } else if ctx.var("ep13_mdrama").get()? == 22 {
                    ctx.lines(args![
                        "Petrified Sapha.",
                        "Now I have the muffler, let's go back to Luik and Snorren."
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("ep13_mdrama").get()? == 23 {
                    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Snorren#ep13md17::OnEnable")])?;
                        ctx.lines_as("Snorren", args!["Ogen!!!!!!", "...Ogen... Ogen...", "....Ogeeen......"])?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("You don't mean..?")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Snorren",
                            args![
                                "This.. is.. worse than I could have imagined.....",
                                "Ogen... this doesn't look like it can be reversed..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Snorren",
                            args!["No matter how much bradium fluid we inject... Ogen can't come back alive..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Snorren",
                            args![
                                "Ogen... really was adament... about protecting that Laphine... to protect Terra.....",
                                "He did his best 'til the last..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Snorren",
                            args![
                                ((Val::from("Look at this, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                                "His petrifyied body...",
                                "protected Terra...like this."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Snorren", args!["Ogen... made himself an unbreakable shield..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Snorren",
                            args![
                                "Ogen... you can now go back to the bosom of the Motherland...",
                                "This statue will be a monument to your courage..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.mes("- - I can't begin to describe the sorrow of losing a close friend... -")?;
                        ctx.next()?;
                        ctx.mes("- All I could do was to keep tapping his shoulder... as he quietly sobbed... -")?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Let's.. go back...", "Ogen wouldn't want you to be so sad and depressed."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Snorren",
                            args![
                                "I guess you are right...",
                                "There's... sunlight shining here...",
                                "This... must be a comfort resting place for Ogen."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Snorren",
                            args!["I should go and tell Luik and the townspeople about this...", "I will go on ahead."],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Snorren#ep13md17::OnDisable")])?;
                        ctx.lines(args![
                            "- Snorren stood up with a bitter smile...",
                            "...I should go back too... I'm worried about Terra. -"
                        ])?;
                        ctx.var("ep13_mdrama").set(Val::from(24))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(7068), Val::from(7069)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Whoa, I almost forgot the ring."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("ep13_mdrama").get()?.number()? > 23 {
                        ctx.lines(args![
                            "Ogen... was petrified as he covered Terra...",
                            "What was going on between them that they would see past their racial differences...?"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.mes("Before you is a statue made of stone and wood. It's so life-like it's eerie.")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn petrified_sapha_ep13md03(ctx: &Ctx) -> Script {
    petrified_sapha_ep13md03_body(ctx, Vec::new()).map(|_| ())
}

pub fn snorren_ep13md17(ctx: &Ctx) -> Script {
    snorren_ep13md17_run(ctx, SnorrenEp13md17Step::Start, Vec::new()).map(|_| ())
}

pub fn snorren_ep13md17_oninit(ctx: &Ctx) -> Script {
    snorren_ep13md17_run(ctx, SnorrenEp13md17Step::OnInit, Vec::new()).map(|_| ())
}

pub fn snorren_ep13md17_ondisable(ctx: &Ctx) -> Script {
    snorren_ep13md17_run(ctx, SnorrenEp13md17Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn snorren_ep13md17_onenable(ctx: &Ctx) -> Script {
    snorren_ep13md17_run(ctx, SnorrenEp13md17Step::OnEnable, Vec::new()).map(|_| ())
}

fn trunk_of_a_tree_ep13_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "The trunk of a Tree moderately stretched upward.",
        "There are big thorns here that should be enough to use as footholds to climb up."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn trunk_of_a_tree_ep13(ctx: &Ctx) -> Script {
    trunk_of_a_tree_ep13_body(ctx, Vec::new()).map(|_| ())
}

fn trunk_of_a_tree_ep13_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "It's the trunk of a Tree stretched out toward a hole in the ceiling.",
        "Big thorns can be used as footholds."
    ])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Climb up.:Never mind.")])? {
        1 => {
            ctx.call(Function::Warp, vec![Val::from("spl_fild01"), Val::from(376), Val::from(65)])?;
            return Err(Stop::End);
        }
        2 => {
            ctx.mes("I'll come back later.")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn trunk_of_a_tree_ep13_ontouch(ctx: &Ctx) -> Script {
    trunk_of_a_tree_ep13_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn villager_ep13_11_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.lines(args![
            "Wait a moment!!",
            "You have too many items.",
            "You can't receive this.",
            "Lighten your weight and",
            "try again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        if ctx.var("ep13_mdrama").get()? == 10 {
            ctx.lines_as(
                "Villager Lawine",
                args!["By the way, how did that Fairy come this far?", "Is she captured?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Villager Rivier",
                args![
                    "No, She's not.",
                    "That Fairy came here by her own accord.",
                    "How impudent. She was horribly wounded when she got here."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["(Hmm? a Fairy? Wounded?", "Are they talking about Terra..?!)"],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Break into their conversation.:Keep quiet and listen.")])? {
                1 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Excuse me for interrupting your conversation.",
                            "You were talking about a Fairy, would you be kind to tell me the details?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Villager Lawine", args!["Outsider!!", "Why is he here?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Villager Rivier",
                        args![
                            "We weren't saying anything.",
                            "There's nothing to tell you.",
                            "Lawine, Come with me to the Refinery later."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "- The Sapha people are not willing to tell you anything. But the",
                        "captured Fairy may be Terra. -"
                    ])?;
                    ctx.var("ep13_mdrama").set(Val::from(11))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(7060), Val::from(7061)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Villager Lawine",
                        args!["I heard a fairy was crying and begging, she was badly hurt."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Villager Rivier",
                        args!["No one understand the words of fairy...", "In the end we had no idea what to do."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Villager Rivier",
                        args![
                            "Anyway, that fairy is now in prison.",
                            "By the way, pull within feather",
                            "Today we have to go to the refinery,",
                            "do not forget."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Villager Lawine", args!["You know, right?", "Ahh! A foreign visitor!"])?;
                    ctx.next()?;
                    ctx.lines_as("Villager Rivier", args!["Nevermind him, he can not understand our words."])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "- Fortunately, Sapha had thought I could not understand them,",
                        "but they must be talking about Terra... -"
                    ])?;
                    ctx.var("ep13_mdrama").set(Val::from(11))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(7060), Val::from(7061)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("ep13_mdrama").get()?.number()? > 10 {
            ctx.mes("- The Sapha are having a conversation and not paying attention to my movements. -")?;
            ctx.next()?;
            ctx.mes("- They are talking mostly about common town matters, not much useful information. -")?;
            ctx.next()?;
            ctx.mes("- I'd better move away from them before they get suspicious of my presence. -")?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Villager",
                args![
                    "...Outsider?",
                    "Do you have a permission to come into our village?",
                    "That's right..Do you even understand me?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as("Villager", args!["Das?", "idh sd!", "Dh apa sd!-is Das idh."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn villager_ep13_11(ctx: &Ctx) -> Script {
    villager_ep13_11_body(ctx, Vec::new()).map(|_| ())
}

fn villager_ep13_12_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.lines(args![
            "Wait a moment!!",
            "You have too many items.",
            "You can't receive this.",
            "Lighten your weight and",
            "try again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        if ctx.var("ep13_mdrama").get()? == 10 {
            ctx.lines_as(
                "Villager Lawine",
                args!["By the way, how did that Fairy come this far?", "Is she captured?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Villager Rivier",
                args![
                    "No, She's not.",
                    "That Fairy came here by her own accord.",
                    "How impudent. She was horribly wounded when she got here."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["(Hmm? a Fairy? Wounded?", "Are they talking about Terra..?!)"],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Break into their conversation.:Keep quiet and listen.")])? {
                1 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Excuse me for interrupting your conversation.",
                            "You were talking about a Fairy, would you be kind to tell me the details?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Villager Lawine", args!["Outsider!!", "Why is he here?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Villager Rivier",
                        args![
                            "We weren't saying anything.",
                            "There's nothing to tell you.",
                            "Lawine, Come with me to the Refinery later."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "- The Sapha people are not willing to tell you anything. But the",
                        "captured Fairy may be Terra. -"
                    ])?;
                    ctx.var("ep13_mdrama").set(Val::from(11))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(7060), Val::from(7061)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Villager Lawine",
                        args!["I heard a fairy was crying and begging, she was badly hurt."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Villager Rivier",
                        args!["No one understand the words of fairy...", "In the end we had no idea what to do."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Villager Rivier",
                        args![
                            "Anyway, that fairy is now in prison.",
                            "By the way, pull within feather",
                            "Today we have to go to the refinery,",
                            "do not forget."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Villager Lawine", args!["You know, right?", "Ahh! A foreign visitor!"])?;
                    ctx.next()?;
                    ctx.lines_as("Villager Rivier", args!["Nevermind him, he can not understand our words."])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "- Fortunately, Sapha had thought I could not understand them,",
                        "but they must be talking about Terra... -"
                    ])?;
                    ctx.var("ep13_mdrama").set(Val::from(11))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(7060), Val::from(7061)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("ep13_mdrama").get()?.number()? > 10 {
            ctx.mes("- The Sapha are having a conversation and not paying attention to my movements. -")?;
            ctx.next()?;
            ctx.mes("- They are talking mostly about common town matters, not much useful information. -")?;
            ctx.next()?;
            ctx.mes("- I'd better move away from them before they get suspicious of my presence. -")?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Villager",
                args![
                    "...Outsider, huh...",
                    "Since when did we allow outsiders to come and visit?",
                    "That's right..Do you even understand me?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Villager",
                args!["...I guess it doesn't matter....", "Ah, Do we even understand each other?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as("Villager", args!["Fsd a idh as?", "Nsf iu ai sd a sd!", "Asd fo sdj fso df."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn villager_ep13_12(ctx: &Ctx) -> Script {
    villager_ep13_12_body(ctx, Vec::new()).map(|_| ())
}

fn snorren_ep13_13_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_temp_ig = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.lines(args![
            "Wait a moment!!",
            "You have too many items.",
            "You can't receive this.",
            "Lighten your weight and",
            "try again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        if ctx.var("ep13_mdrama").get()? == 11 {
            ctx.lines(args![
                "He seems anxious as he wanders back and forth..",
                "Is he the guard of this prison?"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Snorren",
                args![
                    ".. Argh.. Why not?!",
                    "What a blockhead...",
                    "Hmm?",
                    "Who are you?!",
                    "How could an outsider be here...?"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("About the Captured Fairy.:What is this place?")])? {
                1 => {
                    ctx.lines_as(
                        "Snorren",
                        args![
                            "How does an outsider like you know that?",
                            "And..? What? How can you understand what I'm saying?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snorren",
                        args!["Helloo...?", "Can you understand the words coming out of my mouth?"],
                    )?;
                    ctx.next()?;
                    ctx.mes("- I show the ring to Snorren. -")?;
                    ctx.next()?;
                    ctx.lines_as("Snorren", args!["Aha! Now I understand...", "That's amazing. a ring capable of interpreting between different languages... and you were able to come all the way here."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snorren",
                        args!["I heard there are outsiders helping our people.", "You must be one of them!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snorren",
                        args![
                            "But where did you hear about that Fairy?",
                            "Frankly, I want to have a word with that Fairy.",
                            "But I don't understand their language."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Why do you want to talk to her?:Do you need an interpreter?")])? {
                        1 => {}
                        2 => {
                            ctx.lines_as(
                                "Snorren",
                                args![
                                    "An interpreter? Certainly!",
                                    "But I can't just do it arbitrarily...",
                                    "The reason why I want to have a word with that fairy, I mean Laphine..."
                                ],
                            )?;
                            ctx.next()?;
                        }
                        _ => {}
                    }
                    ctx.lines_as(
                        "Snorren",
                        args!["I... want to find out if she's got anything to do with my friend..."],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Your friend?")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Snorren",
                        args![
                            "Yeah. Ogen is missing.",
                            "He said he was curious about the area you are living in and went to check it out. How foolish of him."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snorren",
                        args!["And he never came back.", "After quite a while, that fairy came."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snorren",
                        args![
                            "I don't know what guts she had to just fly into our territory.",
                            "That fairy just kept on yelling 'a bradium!' 'a bradium!'"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snorren",
                        args![
                            "Our village's vigilantes are saying that fairy is affected by the power of bradium and went crazy.",
                            "That's what Laphine are all like!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snorren",
                        args![
                            "They just try to make accesories out of the ore which is our source of life.",
                            "They collect anything that possesses magical power."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["You should calm down a bit... Tell me about that captured Laphine."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snorren",
                        args![
                            "Right, I'm sorry.",
                            "Anyways, shortly after Ogen went missing the fairy appears... this is too suspicious...",
                            "and that fairy was severely wounded too."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Snorren", args!["I think..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snorren",
                        args![
                            "That fairy attacked Ogen as he was trying to peek on your camp.",
                            "And she was drawn by Ogen's bradium and came all the way here to find more bradium!"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("That could be right...:I don't see it that way...")])? {
                        1 => {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "But does that Ore called bradium really possess an overwhelming magical power?",
                                    "It didn't seem like that..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Snorren",
                                args![
                                    "Really? But, we Sapha can't live without bradium.",
                                    "This war with the Laphine... is to protect the bradium from them..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["I see...", "Your people and their people are fighting over the bradium..."],
                            )?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines_as("Snorren", args!["Is... Is that so?", "Then how do you see this situation?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["It just occurred to me...", "It's like some kind of forbidden area..."],
                            )?;
                            ctx.next()?;
                        }
                        _ => {}
                    }
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Anyhow, You are going to ask that fairy about Ogen, right?",
                            "If that's the case, I could interpret between you two..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snorren",
                        args!["Yeah?!", "Then just wait here for a sec!", "I will go and ask Luik about this!"],
                    )?;
                    ctx.var("ep13_mdrama").set(Val::from(12))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Snorren",
                        args![
                            "Uh, Ah~!",
                            "This is a prison.",
                            "You need to obey the rules to keep a peaceful group life."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Snorren", args!["This is where people who break the rules are kept overnight."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snorren",
                        args![
                            "Oh. I'm not one of them.",
                            "I just have some business here.",
                            "Snorren is a good Sapha."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            if ctx.var("ep13_mdrama").get()? == 12 {
                ctx.lines_as("Snorren", args!["Umm.. Hold on a sec...", "Luik. It's me. Snorren."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Voice through the door",
                    args![
                        "Ah, I told you it's not possible Snorren!",
                        "I can only put you in a cell for one night!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Snorren",
                    args![
                        "Yikes-! Luik, It's not that!",
                        "I brought someone who would interpret that fairy's words!",
                        "till, Can't I come in?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Luik's Voice",
                    args!["Shut up! Anyway, we have more things to worry about. This Laphine, we need to treat her wounds..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Luik's Voice",
                    args![
                        "Her body is completely different from ours. What should we do?",
                        "Hey, Snorren! Bring me anything. Anything like herbs!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Snorren",
                    args![
                        "Herbs? Is her condition that bad?",
                        "Can't we fix her with bradium?",
                        "Oh right, She's not Sapha, ey?"
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Nothing gets past you...")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as("Snorren", args!["Do you have anything on your mind?"])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "She's a Laphine, right? Then we should ask a Laphine about it...",
                        "Like what we need to treat her...",
                        "I can take care of it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Snorren",
                    args!["Really?", "Then hurry up and bring something back...", "We will wait."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["(I think I have no choice but to go back and ask Arc....)"],
                )?;
                ctx.var("ep13_mdrama").set(Val::from(13))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(7061), Val::from(7062)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if (ctx.var("ep13_mdrama").get()?.number()? > 12 && ctx.var("ep13_mdrama").get()?.number()? < 17) {
                    ctx.lines_as(
                        "Snorren",
                        args!["Hurry up and go find a way to cure this Laphine...", "Then Luik might let us in."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("ep13_mdrama").get()? == 17 {
                        if ctx.call(Function::CountItem, vec![Val::from(607)])?.number()? > 5 {
                            ctx.lines_as(
                                "Snorren",
                                args![
                                    "Did you find a cure?",
                                    "This.. is one strange looking fruit.",
                                    "I saw this fruit from time to time, but never thought this would be a cure..?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Snorren", args!["I will take everything you've brought with you!"])?;
                            l_temp_ig = ctx.call(Function::CountItem, vec![Val::from(607)])?;
                            ctx.call(Function::DelItem, vec![Val::from(607), l_temp_ig.clone()])?;
                            ctx.var("ep13_mdrama").set(Val::from(18))?;
                            ctx.next()?;
                            ctx.lines_as("Snorren", args!["Now, Let's go inside!"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Hmm...this won't be enough....", "I would need at least 6~7 of these..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Snorren", args!["Is something wrong?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Huh? Ah, I think we would need more than what I've brought here... I'll be back."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ctx.var("ep13_mdrama").get()? == 18 {
                            ctx.lines_as(
                                "Snorren",
                                args!["I already told Luik everything about you...", "Let's go inside."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ((ctx.var("ep13_mdrama").get()?.number()? > 18 && ctx.var("ep13_mdrama").get()?.number()? < 21)
                                || ctx.var("ep13_mdrama").get()? == 22)
                            {
                                ctx.lines_as("Snorren", args!["I've been waiting for you...", "Let's make haste."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("ep13_mdrama").get()? == 21 {
                                ctx.lines_as(
                                    "Snorren",
                                    args![
                                        "I'm really confused now...Should I believe your story or not.",
                                        "If it is true that cave really exists and Ogen is hurt..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("ep13_mdrama").get()? == 23 {
                                ctx.lines_as(
                                    "Snorren",
                                    args![
                                        "The approximate location is...",
                                        "Huge Roots of a tree. Got it.",
                                        "You go ahead first. I will stop by at a refinery and be right after you."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Snorren",
                                    args![
                                        "Since that place is Laphine's territory, I will go quitely so they won't notice me.",
                                        "You just go on ahead, I will be right behind you."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("ep13_mdrama").get()? == 24 {
                                ctx.lines_as(
                                    "Snorren",
                                    args![
                                        "You are back. .. Luik wanted me to tell you this...",
                                        "Terra, that Laphine, is free now."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Snorren",
                                    args![
                                        "We are tired of fighting. We didn't even want to start...",
                                        "And we saw her true intention before everything else."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Snorren",
                                    args![
                                        "Go and check if Terra went back to her home safely...",
                                        "Ogen... He went back to Mother Nature."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Snorren",
                                    args![
                                        "I really don't think what Ogen did was the right thing to do...",
                                        "But that was... Ogen..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Snorren",
                                    args![
                                        "Forgive me, I'm a bit depressed...",
                                        "I'd rather just rest here.",
                                        "I really appreciate what you've done."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Snorren", args!["Come back anytime...", "Ok?"])?;
                                ctx.var("ep13_mdrama").set(Val::from(25))?;
                                ctx.call(Function::GetItem, vec![Val::from(6080), Val::from(15)])?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(7069), Val::from(7070)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("ep13_mdrama").get()?.number()? > 24 {
                                ctx.lines_as(
                                    "Snorren",
                                    args![
                                        "What do you think of this muffler?",
                                        "This belonged to Ogen...",
                                        "Does it look good on me?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Snorren",
                                    args!["Outsiders shouldn't be here without permission.", "Please get out of here."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                }
            }
        }
    } else {
        ctx.lines_as("Snorren", args!["Ids ad?", "Ns ai dha si asd!", "Ms dfaa sd a."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn snorren_ep13_13(ctx: &Ctx) -> Script {
    snorren_ep13_13_body(ctx, Vec::new()).map(|_| ())
}
