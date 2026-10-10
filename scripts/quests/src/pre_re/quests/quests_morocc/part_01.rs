use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn messenger_prince1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("nk_prince").get()? == 0 {
        if ctx.var("rebirth_moc_edq").get()?.number()? > 1 {
            ctx.lines_as("Messenger", args!["I am a messenger from the Royal court, sent to find a reliable adventurer. This is all about the very important issues of our nation."])?;
            ctx.next()?;
            if ctx.var("aru_monas").get()?.number()? > 23 {
                ctx.lines_as("Messenger", args!["It might have brought national disorder, but people are all wary about the rebirth of the devil. We are in the perfect times to process this issue without making any additional fuss."])?;
                ctx.next()?;
                ctx.lines_as("Messenger", args!["Oh, I don't want you to panic too much... Hmm... what I want to say is, Tristan the third, the ruler of Rune-Midgarts..."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Passed away?:Came for a celebration again?")])? {
                    1 => {
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                        ctx.lines_as(
                            "Messenger",
                            args![
                                "You already know about that...",
                                "Then, I don't need to give",
                                "you any further explanation."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Messenger", args!["The King's position cannot", "be empty for too long.", "Now, the debate over selecting our next King is actively in progress. Please give us a hand with this, for the tomorrow of Rune-Midgarts."])?;
                        ctx.var("nk_prince").set(Val::from(1))?;
                        ctx.call(Function::SetQuest, vec![Val::from(10000)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Messenger", args!["..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Messenger",
                            args![
                                "What are you thinking?",
                                "Do you really think a celebration is momentous for a nation right now?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Messenger", args!["Yeah...", "It's quite natural for you to expect nothing. I still don't know how to express this to you. Don't be too perplexed."])?;
                        ctx.next()?;
                        ctx.lines_as("Messenger", args!["Tristan the 3rd, the ruler of Rune-Midgarts..."])?;
                        ctx.next()?;
                        ctx.lines_as("Messenger", args!["...", "has passed away."])?;
                        ctx.next()?;
                        ctx.lines_as("Messenger", args!["To prevent the people's disorder, it won't yet be announced. The Court is focused on selecting the next king right now. They are very busy considering it."])?;
                        ctx.next()?;
                        ctx.lines_as("Messenger", args!["You can learn more details in court. My mission is only to deliver this message to a reliable and brave adventurer."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Messenger",
                            args!["Please...Please...", "Lend a hand for the future", "of the Rune-Midgarts kingdom."],
                        )?;
                        ctx.var("nk_prince").set(Val::from(1))?;
                        ctx.call(Function::SetQuest, vec![Val::from(10000)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.lines_as("Messenger", args!["It might have brought national disorder, but people are all wary about the rebirth of the devil. We are in the perfect times to process this issue without making any additional fuss."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Messenger",
                    args![
                        "The King hasn't appeared",
                        "in public for some time.",
                        "Many are curious about the situation."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Messenger",
                    args![
                        "I hate my duty to deliver",
                        "this sorrowful reality to others.",
                        "I think you already have a clue from my words..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Messenger", args!["King Tristan..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Messenger",
                    args![
                        "King Tristan...",
                        "...passed away...",
                        "...after a long time of suffering acutely from illness."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Messenger", args!["The Royal Court is now in the process of selecting the next king. My mission is to find a devoted adventurer and send them to court."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Messenger",
                    args![
                        "They will let you know what",
                        "you have to do in court.",
                        "I hope you will help bring",
                        "peace to Rune-Midgarts."
                    ],
                )?;
                ctx.var("nk_prince").set(Val::from(1))?;
                ctx.call(Function::SetQuest, vec![Val::from(10000)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as(
                "Messenger",
                args![
                    "I am a messenger of Rune-Midgarts.",
                    "Is there something special",
                    "in this country?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Messenger",
                args!["Haven't you seen something mysterious? Or heard some gossip?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("nk_prince").get()? == 1 {
            ctx.lines_as(
                "Messenger",
                args!["I want you to go to the Prontera Royal Court to help in the next King's selection."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Messenger",
                args!["I will stay here to find other adventurers. May your heart bring great glory to Rune-Midgarts!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Messenger",
                args!["I am on a mission now. I think you may be looking for a different location. Ask others."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn messenger_prince1(ctx: &Ctx) -> Script {
    messenger_prince1_body(ctx, Vec::new()).map(|_| ())
}

fn inspector_prince_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_brave = Val::from(0);
    let mut l_int = Val::from(0);
    let mut l_prin1 = Val::from(0);
    let mut l_prin2 = Val::from(0);
    let mut l_prin3 = Val::from(0);
    let mut l_prin4 = Val::from(0);
    let mut l_prin5 = Val::from(0);
    let mut l_prin6 = Val::from(0);
    let mut l_prin7 = Val::from(0);
    let mut l_prince = Val::from(0);
    let mut l_solid = Val::from(0);
    if (ctx.call(Function::CheckQuest, vec![Val::from(10004)])? == 0 || ctx.call(Function::CheckQuest, vec![Val::from(10004)])? == 1) {
        ctx.lines_as(
            "Inspector",
            args![
                "Judge!",
                "How goes it? I don't think it is easy to meet seven candidates and appraise them."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Inspector",
            args!["Ha. Are you done?", "You are a fast worker, getting the job done!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Inspector", args!["Let me have the scoop on the candidates~"])?;
        ctx.next()?;
        ctx.lines(args![
            "-I talked about my impression",
            "of the candidates for prince,",
            "to the Inspector, right away.",
            "I'm sure, I only give clear",
            "and concise facts to him.-"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Inspector",
            args!["Hmm... is that so? What you've said has helped me a lot. Thank you for your hard work."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Inspector",
            args![
                "By the way, I happened",
                "to hear a rumor about",
                "Prince Eigen Ahrum of the Walter family. Could you appraise all the candidates, one more time? I need this done urgently."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Inspector", args!["Now the whole picture may change... Others must probably be the same. But I worry about Eigen Ahrum because I can't neglect any rumors about any candidate."])?;
        ctx.next()?;
        ctx.lines_as("Inspector", args!["Okay, so... please go in there one more time."])?;
        ctx.call(Function::CompleteQuest, vec![Val::from(10004)])?;
        ctx.call(Function::SetQuest, vec![Val::from(10018)])?;
        ctx.call(Function::SetQuest, vec![Val::from(10019)])?;
        ctx.call(Function::SetQuest, vec![Val::from(10020)])?;
        ctx.call(Function::SetQuest, vec![Val::from(10021)])?;
        ctx.call(Function::SetQuest, vec![Val::from(10022)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("nk_prince").get()?.number()? < 2 {
        ctx.lines_as("Inspector", args!["What's wrong with you?", "This is not an open area!"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Alright, Alright.:The Messenger directed me here.")])? {
            1 => {
                ctx.lines_as("Inspector", args!["If you know it, why are you here? Get out of here!"])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("prontera"), Val::from(155), Val::from(353)])?;
                return Err(Stop::End);
            }
            2 => {
                if ctx.var("nk_prince").get()? == 0 {
                    ctx.lines_as("Inspector", args!["I don't think", "that he is an experienced appraiser..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Inspector", args!["......"])?;
                ctx.next()?;
                if ctx.var("BaseLevel").get()?.number()? > 89 {
                    ctx.lines_as("Inspector", args!["The Messenger checked only for your physical aptitude, and simply sent a warrior to Court. That's what his duty is all about."])?;
                    ctx.next()?;
                    ctx.lines_as("Inspector", args!["Checking your qualification is a matter that concerns me."])?;
                    ctx.next()?;
                    ctx.lines_as("Inspector", args!["Hmm..."])?;
                    ctx.next()?;
                    ctx.lines_as("Inspector", args!["You are an experienced traveler. You have enough strength, but... The Court doesn't choose a person based only on his or her great strength and battle experience."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Inspector",
                        args!["To check someone's quality", "you have to have enough", "personality about you."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Inspector",
                        args!["In that sense, I want to do a simple experiment.", "Let's test you."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Inspector", args!["This won't be complicated. You are just needed to give a quick answer, frankly please, after each given situation."])?;
                    ctx.next()?;
                    ctx.lines_as("Inspector", args!["kay, let's start. Get ready. Listen carefully and answer."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Inspector",
                        args![
                            "What do you think is the",
                            "most important thing to consider",
                            "as a leader of a hunt party?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "Strength and characteristics of the monster.:Efficiency of the hunt.:What we can get from the hunt.",
                        )],
                    )? {
                        1 => {
                            l_int = (l_int.clone() + Val::from(10));
                        }
                        2 => {
                            l_solid = (l_solid.clone() + Val::from(10));
                        }
                        3 => {
                            l_brave = (l_brave.clone() + Val::from(10));
                        }
                        _ => {}
                    }
                    ctx.lines_as("Inspector", args!["What if we lose a party member", "during battle?"])?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "Keep fighting until we can do it.:Find a new place and try differently.:Stop hunting and replenish.",
                        )],
                    )? {
                        1 => {
                            l_brave = (l_brave.clone() + Val::from(10));
                        }
                        2 => {
                            l_int = (l_int.clone() + Val::from(10));
                        }
                        3 => {
                            l_solid = (l_solid.clone() + Val::from(10));
                        }
                        _ => {}
                    }
                    ctx.lines_as("Inspector", args!["What you get as a result of the hunt..."])?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "Share it as agreed prior.:I don't want to care for sharing.:Give it to the member that needs it.",
                        )],
                    )? {
                        1 => {
                            l_int = (l_int.clone() + Val::from(10));
                        }
                        2 => {
                            l_solid = (l_solid.clone() + Val::from(10));
                        }
                        3 => {
                            l_brave = (l_brave.clone() + Val::from(10));
                        }
                        _ => {}
                    }
                    ctx.lines_as(
                        "Inspector",
                        args!["What type of job class member", "has to be cured while the enemy still lives?"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Priest, healer class.:Hunter, damage dealer.:Lord Knight, tanker.")],
                    )? {
                        1 => {
                            l_int = (l_int.clone() + Val::from(10));
                        }
                        2 => {
                            l_brave = (l_brave.clone() + Val::from(10));
                        }
                        3 => {
                            l_solid = (l_solid.clone() + Val::from(10));
                        }
                        _ => {}
                    }
                    ctx.lines_as("Inspector", args!["Whom do you want to befriend the most?"])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("A strong person.:An experienced person.:A kind person.")])? {
                        1 => {
                            l_brave = (l_brave.clone() + Val::from(10));
                        }
                        2 => {
                            l_int = (l_int.clone() + Val::from(10));
                        }
                        3 => {
                            l_solid = (l_solid.clone() + Val::from(10));
                        }
                        _ => {}
                    }
                    ctx.lines_as(
                        "Inspector",
                        args!["What was your ability that helped", "you the most during your training?"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "Relationships with people.:Plenty of information gathering.:Discernment and driving force.",
                        )],
                    )? {
                        1 => {
                            l_solid = (l_solid.clone() + Val::from(10));
                        }
                        2 => {
                            l_int = (l_int.clone() + Val::from(10));
                        }
                        3 => {
                            l_brave = (l_brave.clone() + Val::from(10));
                        }
                        _ => {}
                    }
                    ctx.lines_as("Inspector", args!["How do you give encouragement to others, usually?"])?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "Refer to a past failure.:Hesitating is the worst thing.:You are not alone.",
                        )],
                    )? {
                        1 => {
                            l_int = (l_int.clone() + Val::from(10));
                        }
                        2 => {
                            l_brave = (l_brave.clone() + Val::from(10));
                        }
                        3 => {
                            l_solid = (l_solid.clone() + Val::from(10));
                        }
                        _ => {}
                    }
                    ctx.lines_as("Inspector", args!["What is an ultimate virtue", "in your life?"])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Growth of oneself.:World peace.:The pursuit of truth.")])? {
                        1 => {
                            l_brave = (l_brave.clone() + Val::from(10));
                        }
                        2 => {
                            l_solid = (l_solid.clone() + Val::from(10));
                        }
                        3 => {
                            l_int = (l_int.clone() + Val::from(10));
                        }
                        _ => {}
                    }
                    ctx.lines_as(
                        "Inspector",
                        args!["What would you want to keep", "the most if you were on a ", "deserted island?"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Map:Flint:Weapon")])? {
                        1 => {
                            l_int = (l_int.clone() + Val::from(10));
                        }
                        2 => {
                            l_solid = (l_solid.clone() + Val::from(10));
                        }
                        3 => {
                            l_brave = (l_brave.clone() + Val::from(10));
                        }
                        _ => {}
                    }
                    ctx.lines_as("Inspector", args!["What type of story do you like the most?"])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Love story.:Heroic epic.:Religious tale.")])? {
                        1 => {
                            l_solid = (l_solid.clone() + Val::from(10));
                        }
                        2 => {
                            l_brave = (l_brave.clone() + Val::from(10));
                        }
                        3 => {
                            l_int = (l_int.clone() + Val::from(10));
                        }
                        _ => {}
                    }
                    ctx.lines_as(
                        "Inspector",
                        args!["You've come to engage the", "new monster in a new area.", "What do you do?"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Attack first.:Observe from a distance.:Flee away.")])? {
                        1 => {
                            l_brave = (l_brave.clone() + Val::from(10));
                        }
                        2 => {
                            l_int = (l_int.clone() + Val::from(10));
                        }
                        3 => {
                            l_solid = (l_solid.clone() + Val::from(10));
                        }
                        _ => {}
                    }
                    ctx.lines_as(
                        "Inspector",
                        args!["What has to be done first when", "ruling the Rune-Midgarts kingdom?"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "Dominate countries by reinforcing militia.:Advancement of economy and technology.:Keep public security firm.",
                        )],
                    )? {
                        1 => {
                            l_brave = (l_brave.clone() + Val::from(10));
                        }
                        2 => {
                            l_int = (l_int.clone() + Val::from(10));
                        }
                        3 => {
                            l_solid = (l_solid.clone() + Val::from(10));
                        }
                        _ => {}
                    }
                    ctx.lines_as("Inspector", args!["Your humanity test is done.", "You have been living..."])?;
                    ctx.next()?;
                    if l_brave.clone().number()? > 50 {
                        if l_int.clone().number()? > 30 {
                            ctx.lines_as(
                                "Inspector",
                                args!["with strong conviction,", "and reasonable judgement,", "so far."],
                            )?;
                            ctx.next()?;
                        } else if l_solid.clone().number()? > 30 {
                            ctx.lines_as(
                                "Inspector",
                                args!["with strong bravery,", "but also you've had a", "temperate life."],
                            )?;
                            ctx.next()?;
                        } else {
                            ctx.lines_as(
                                "Inspector",
                                args!["with a decisive mind.", "You could overcome hardships with it."],
                            )?;
                            ctx.next()?;
                        }
                    } else {
                        if l_int.clone().number()? > 50 {
                            if l_brave.clone().number()? > 30 {
                                ctx.lines_as("Inspector", args!["with firmness,", "and reasonable judgement,", "so far."])?;
                                ctx.next()?;
                            } else if l_solid.clone().number()? > 30 {
                                ctx.lines_as(
                                    "Inspector",
                                    args!["with reasonable judgement", "and a harmonic sensibility,", "so far."],
                                )?;
                                ctx.next()?;
                            } else {
                                ctx.lines_as(
                                    "Inspector",
                                    args![
                                        "with a calm and prudent decision.",
                                        "I think you could also have had many good experiences."
                                    ],
                                )?;
                                ctx.next()?;
                            }
                        } else if l_solid.clone().number()? > 50 {
                            if l_brave.clone().number()? > 30 {
                                ctx.lines_as(
                                    "Inspector",
                                    args!["with strong bravery,", "but also you've had a", "temperate life."],
                                )?;
                                ctx.next()?;
                            } else if l_int.clone().number()? > 30 {
                                ctx.lines_as(
                                    "Inspector",
                                    args!["with reasonable judgement", "and peaceful sensibility,", "so far."],
                                )?;
                                ctx.next()?;
                            } else {
                                ctx.lines_as(
                                    "Inspector",
                                    args![
                                        "...Erm, actually...",
                                        "You didn't distort to either way...",
                                        "and stability and harmony were",
                                        "an important virtue in your life."
                                    ],
                                )?;
                                ctx.next()?;
                            }
                        }
                    }
                    ctx.lines_as(
                        "Inspector",
                        args![
                            "I think you are",
                            "qualified enough to evaluate others. I apologize for my rude behavior to you in the beginning."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Inspector",
                        args!["So, I ask you to do us a favor. You have become a regular adventurer appraiser."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Inspector", args!["Ah, you've spent a long time with me. You must be very tired. Before starting work, if you need some preparation, take off now and come back to me later. I will be waiting here for you."])?;
                    ctx.next()?;
                    ctx.var("nk_prince").set(Val::from(4))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(10000), Val::from(10003)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Inspector",
                        args![
                            "I don't think you are",
                            "an experienced adventurer...",
                            "And this is an important issue for the nation. I can't give this job to anyone like you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Inspector",
                        args!["But, if you prove yourself a strong adventurer, then I will let you pass the first qualification test."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Inspector",
                        args!["Have you heard of Glastheim? In that area, there are monsters named Knights of Abyss."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Inspector",
                        args![
                            "After beating the knights of abyss",
                            "bring 2 ^ff0000Reins^000000.",
                            "If you can do this mission,",
                            "I will respect your strength."
                        ],
                    )?;
                    ctx.var("nk_prince").set(Val::from(2))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(10000), Val::from(10001)])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Inspector",
                        args!["Let's continue our talk", "after getting the 2 Reins.", "Ciao..."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            _ => {}
        }
    } else {
        if ctx.var("nk_prince").get()? == 2 {
            ctx.lines_as(
                "Inspector",
                args!["Ahh, you came to meet me before huh? I guess you've already been to Glastheim."],
            )?;
            ctx.next()?;
            ctx.lines_as("Inspector", args!["Do you have the 2 reins I mentioned?"])?;
            ctx.next()?;
            if ctx.call(Function::CountItem, vec![Val::from(1064)])?.number()? > 1 {
                ctx.lines_as(
                    "Inspector",
                    args!["Ooh!", "Frankly, I didn't expect you to bring them. You are great."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Inspector",
                    args!["Alright, good. You proved your strength for yourself. Now you passed one exam. All but one."],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(1064), Val::from(2)])?;
                ctx.var("nk_prince").set(Val::from(3))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(10001), Val::from(10002)])?;
                ctx.next()?;
                ctx.lines_as("Inspector", args!["The judge to appraise princes should build up his character too. Strength cannot cover everything. Personality should come along with it"])?;
                ctx.next()?;
                ctx.lines_as("Inspector", args!["In a sense, the next examination is to test your character. It is just a series of simple questions, but relax first, and be prepared mentally... then come and talk to me again."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as("Inspector", args!["You've never even taken such a dangerous mission before! How dare you think you could come to this place? I don't think you can take the job of evaluating princes."])?;
                ctx.next()?;
                ctx.lines_as("Inspector", args!["Now, you can come back later."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("nk_prince").get()? == 3 {
                ctx.lines_as("Inspector", args!["Now are you ready?", "This test is not that hard though..."])?;
                ctx.next()?;
                ctx.lines_as("Inspector", args!["This is done to know your attitude and behavior in certain situations. Listen carefully to my questions, and answer frankly in regards to the episodes of your trip."])?;
                ctx.next()?;
                ctx.lines_as("Inspector", args!["Now, let me begin."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Inspector",
                    args![
                        "What do you think is the",
                        "most important thing to consider",
                        "as a leader of a hunt party?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Strength and characteristics of the monster.:Efficiency of the hunt.:What we can get from the hunt.",
                    )],
                )? {
                    1 => {
                        l_int = (l_int.clone() + Val::from(10));
                    }
                    2 => {
                        l_solid = (l_solid.clone() + Val::from(10));
                    }
                    3 => {
                        l_brave = (l_brave.clone() + Val::from(10));
                    }
                    _ => {}
                }
                ctx.lines_as("Inspector", args!["What if we lose a party member", "during battle?"])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Keep fighting until we can do it.:Find a new place and try differently.:Stop hunting and replenish.",
                    )],
                )? {
                    1 => {
                        l_brave = (l_brave.clone() + Val::from(10));
                    }
                    2 => {
                        l_int = (l_int.clone() + Val::from(10));
                    }
                    3 => {
                        l_solid = (l_solid.clone() + Val::from(10));
                    }
                    _ => {}
                }
                ctx.lines_as("Inspector", args!["What you get as a result of the hunt..."])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Share it as agreed prior.:I don't want to care for sharing.:Give it to the member that needs it.",
                    )],
                )? {
                    1 => {
                        l_int = (l_int.clone() + Val::from(10));
                    }
                    2 => {
                        l_solid = (l_solid.clone() + Val::from(10));
                    }
                    3 => {
                        l_brave = (l_brave.clone() + Val::from(10));
                    }
                    _ => {}
                }
                ctx.lines_as(
                    "Inspector",
                    args!["What type of job class member", "has to be cured while the enemy still lives?"],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("Priest, healer class.:Hunter, damage dealer.:Lord Knight, tanker.")],
                )? {
                    1 => {
                        l_int = (l_int.clone() + Val::from(10));
                    }
                    2 => {
                        l_brave = (l_brave.clone() + Val::from(10));
                    }
                    3 => {
                        l_solid = (l_solid.clone() + Val::from(10));
                    }
                    _ => {}
                }
                ctx.lines_as("Inspector", args!["Whom do you want to befriend the most?"])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("A strong person.:An experienced person.:A kind person.")])? {
                    1 => {
                        l_brave = (l_brave.clone() + Val::from(10));
                    }
                    2 => {
                        l_int = (l_int.clone() + Val::from(10));
                    }
                    3 => {
                        l_solid = (l_solid.clone() + Val::from(10));
                    }
                    _ => {}
                }
                ctx.lines_as(
                    "Inspector",
                    args!["What was your ability that helped", "you the most during your training?"],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Relationships with people.:Plenty of information gathering.:Discernment and driving force.",
                    )],
                )? {
                    1 => {
                        l_solid = (l_solid.clone() + Val::from(10));
                    }
                    2 => {
                        l_int = (l_int.clone() + Val::from(10));
                    }
                    3 => {
                        l_brave = (l_brave.clone() + Val::from(10));
                    }
                    _ => {}
                }
                ctx.lines_as("Inspector", args!["How do you give encouragement to others, usually?"])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Refer to a past failure.:Hesitating is the worst thing.:You are not alone.",
                    )],
                )? {
                    1 => {
                        l_int = (l_int.clone() + Val::from(10));
                    }
                    2 => {
                        l_brave = (l_brave.clone() + Val::from(10));
                    }
                    3 => {
                        l_solid = (l_solid.clone() + Val::from(10));
                    }
                    _ => {}
                }
                ctx.lines_as("Inspector", args!["What is an ultimate virtue", "in your life?"])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Growth of oneself.:World peace.:The pursuit of truth.")])? {
                    1 => {
                        l_brave = (l_brave.clone() + Val::from(10));
                    }
                    2 => {
                        l_solid = (l_solid.clone() + Val::from(10));
                    }
                    3 => {
                        l_int = (l_int.clone() + Val::from(10));
                    }
                    _ => {}
                }
                ctx.lines_as(
                    "Inspector",
                    args!["What would you want to keep", "the most if you were on a ", "deserted island?"],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Map:Flint:Weapon")])? {
                    1 => {
                        l_int = (l_int.clone() + Val::from(10));
                    }
                    2 => {
                        l_solid = (l_solid.clone() + Val::from(10));
                    }
                    3 => {
                        l_brave = (l_brave.clone() + Val::from(10));
                    }
                    _ => {}
                }
                ctx.lines_as("Inspector", args!["What type of story do you like the most?"])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Love story.:Heroic epic.:Religious tale.")])? {
                    1 => {
                        l_solid = (l_solid.clone() + Val::from(10));
                    }
                    2 => {
                        l_brave = (l_brave.clone() + Val::from(10));
                    }
                    3 => {
                        l_int = (l_int.clone() + Val::from(10));
                    }
                    _ => {}
                }
                ctx.lines_as(
                    "Inspector",
                    args!["You've come to engage the", "new monster in a new area.", "What do you do?"],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Attack first.:Observe from a distance.:Flee away.")])? {
                    1 => {
                        l_brave = (l_brave.clone() + Val::from(10));
                    }
                    2 => {
                        l_int = (l_int.clone() + Val::from(10));
                    }
                    3 => {
                        l_solid = (l_solid.clone() + Val::from(10));
                    }
                    _ => {}
                }
                ctx.lines_as(
                    "Inspector",
                    args!["What has to be done first when", "ruling the Rune-Midgarts kingdom?"],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Dominate countries by reinforcing militia.:Advancement of economy and technology.:Keep public security firm.",
                    )],
                )? {
                    1 => {
                        l_brave = (l_brave.clone() + Val::from(10));
                    }
                    2 => {
                        l_int = (l_int.clone() + Val::from(10));
                    }
                    3 => {
                        l_solid = (l_solid.clone() + Val::from(10));
                    }
                    _ => {}
                }
                ctx.lines_as("Inspector", args!["Your humanity test is done.", "You have been living..."])?;
                ctx.next()?;
                if l_brave.clone().number()? > 50 {
                    if l_int.clone().number()? > 30 {
                        ctx.lines_as(
                            "Inspector",
                            args!["with strong conviction,", "and reasonable judgement,", "so far."],
                        )?;
                        ctx.next()?;
                    } else if l_solid.clone().number()? > 30 {
                        ctx.lines_as(
                            "Inspector",
                            args!["with strong bravery,", "but also you've had a", "temperate life."],
                        )?;
                        ctx.next()?;
                    } else {
                        ctx.lines_as(
                            "Inspector",
                            args!["with a decisive mind.", "You could overcome hardships with it."],
                        )?;
                        ctx.next()?;
                    }
                } else {
                    if l_int.clone().number()? > 50 {
                        if l_brave.clone().number()? > 30 {
                            ctx.lines_as("Inspector", args!["with firmness,", "and reasonable judgement,", "so far."])?;
                            ctx.next()?;
                        } else if l_solid.clone().number()? > 30 {
                            ctx.lines_as(
                                "Inspector",
                                args!["with reasonable judgement", "and a harmonic sensibility,", "so far."],
                            )?;
                            ctx.next()?;
                        } else {
                            ctx.lines_as(
                                "Inspector",
                                args![
                                    "with a calm and prudent decision.",
                                    "I think you could also have had many good experiences."
                                ],
                            )?;
                            ctx.next()?;
                        }
                    } else if l_solid.clone().number()? > 50 {
                        if l_brave.clone().number()? > 30 {
                            ctx.lines_as(
                                "Inspector",
                                args!["with strong bravery,", "but also you've had a", "temperate life."],
                            )?;
                            ctx.next()?;
                        } else if l_int.clone().number()? > 30 {
                            ctx.lines_as(
                                "Inspector",
                                args!["with reasonable judgement", "and peaceful sensibility,", "so far."],
                            )?;
                            ctx.next()?;
                        } else {
                            ctx.lines_as(
                                "Inspector",
                                args![
                                    "...Erm, actually...",
                                    "You didn't distort to either way...",
                                    "and stability and harmony were",
                                    "an important virtue in your life."
                                ],
                            )?;
                            ctx.next()?;
                        }
                    }
                }
                ctx.lines_as(
                    "Inspector",
                    args![
                        "I think you are",
                        "qualified enough to evaluate others. I apologize for my rude behavior to you in the beginning."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Inspector",
                    args!["So, I ask you to do us a favor. You have become a regular adventurer appraiser."],
                )?;
                ctx.next()?;
                ctx.lines_as("Inspector", args!["Ah, you've spent a long time with me. You must be very tired. Before starting work, if you need some preparation, take off now and come back to me later. I will be waiting here for you."])?;
                ctx.next()?;
                ctx.var("nk_prince").set(Val::from(4))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(10002), Val::from(10003)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("nk_prince").get()? == 4 {
                    ctx.lines_as(
                        "Inspector",
                        args![
                            "Welcome...",
                            "I will give you some",
                            "instructions on what to do,",
                            "and what one has to be careful with, when dealing with princes. Pay attention to me."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Inspector", args!["I will answer your questions first."])?;
                    ctx.next()?;
                    'l26: loop {
                        if !(true) {
                            break 'l26;
                        }
                        'b26: {
                            match runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "What should I do, exactly?:The King died because of his illness?:Is my mission confidential??:I have no more questions.",
                                )],
                            )? {
                                1 => {
                                    ctx.lines_as("Inspector", args!["We have seven candidates from seven families. One per each family. The candidates are all in the same place, where I will introduce them to you."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Inspector", args!["You have to meet the candidates one by one, and ask some questions about their quality; or observe their actions, then come back to me and explain all that you observed in detail."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Inspector", args!["The original rule is that the quality of princes has to be appraised by royal families for a long time, and one has to be elected by internal ordinance."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Inspector", args!["Because of the resurrection of the devil Satan Morocc, people are too uneasy to follow regular announcements, and the atmosphere of the kingdom is not stable."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Inspector",
                                        args!["Each kingdom is busy preventing the interference of Satan Morocc too."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Inspector", args!["So this time, the Court decided to change its ways, giving the opportunity to appraise the candidates to famous and devoted adventurers like yourself. That's why you are here."])?;
                                    ctx.next()?;
                                    l_prince = Val::from(1);
                                }
                                2 => {
                                    ctx.lines_as("Inspector", args!["How absurd that you ask that!! I already told you clearly that King Tristan died of illness, didn't I?"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Inspector", args!["Don't you have anything else to be curious about?"])?;
                                    ctx.next()?;
                                }
                                3 => {
                                    ctx.lines_as("Inspector", args!["Sure...", "It's natural to announce the death of the King. But the atmosphere of the nation is not ripe for it. This is our only alternative for now."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Inspector", args!["Princes cannot go outside on their own will. People cannot go to the place where the princes are."])?;
                                    ctx.next()?;
                                }
                                4 => {
                                    if l_prince.clone() == 1 {
                                        ctx.lines_as(
                                            "Inspector",
                                            args![
                                                "I think this is enough",
                                                "instruction for this.",
                                                "Now I will let you know",
                                                "where you can meet the princes."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Inspector", args!["When you get to the medic officer's room, a soldier will be standing on a strange spot. You can find that there isn't a door there."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Inspector", args!["Behind the soldier, you can find a secret door. If you talk to the soldier, the soldier will allow you to get in."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Inspector", args!["As I have expressed, this is a very important issue for the Rune-Midgarts kingdom. Although I've stressed this point repeateadly, I cannot stress it enough."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Inspector", args!["Please behave accordingly.", "Thanks for your effort."])?;
                                        ctx.var("nk_prince").set(Val::from(5))?;
                                        ctx.call(Function::CompleteQuest, vec![Val::from(10003)])?;
                                        ctx.call(Function::SetQuest, vec![Val::from(10005)])?;
                                        ctx.call(Function::SetQuest, vec![Val::from(10006)])?;
                                        ctx.call(Function::SetQuest, vec![Val::from(10007)])?;
                                        ctx.call(Function::SetQuest, vec![Val::from(10008)])?;
                                        ctx.call(Function::SetQuest, vec![Val::from(10009)])?;
                                        ctx.call(Function::SetQuest, vec![Val::from(10010)])?;
                                        ctx.call(Function::SetQuest, vec![Val::from(10011)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as("Inspector", args!["You got instructions as to what you should do, didn't you? This issue is not that worthless that you can take it with ease! Pay attention to this!"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Inspector", args!["If you are not motivated to do it, I will find another adventurer. You can go back to your business, if you want to."])?;
                                        ctx.next()?;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                } else {
                    if ctx.var("nk_prince").get()? == 7 {
                        ctx.lines_as(
                            "Inspector",
                            args![
                                "How is it going?",
                                "Grading someone is not a piece of cake. It's real hard work indeed. You do the hard work for our country."
                            ],
                        )?;
                        ctx.next()?;
                        if (ctx.var("nkprince_eisen").get()? == 15
                            && (ctx.call(Function::CheckQuest, vec![Val::from(10025)])? == 0
                                || ctx.call(Function::CheckQuest, vec![Val::from(10025)])? == 1))
                        {
                            ctx.mes("-I tell him about the Ahrum and Ernst accident.-")?;
                            ctx.next()?;
                            ctx.lines_as("Inspector", args!["What!? I can't believe it!", "Wha-, that's so..."])?;
                            ctx.next()?;
                            ctx.lines_as("Inspector", args!["By Ernst's hand...?", "Yes...?"])?;
                            ctx.next()?;
                            ctx.lines_as("Inspector", args!["You've fulfilled the job as", "adventurer appraiser fully.", "I really appreciate your assistance thus far. Due to your help, the future of the kingdom is indebted to you."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["(He doesn't seem to be suprised too much.. Could there be a reason? Now I am suspicious...)"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Inspector", args!["Hey, the things you experienced here cannot be disclosed to others. You understand that this situation is fully confidential."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Inspector",
                                args!["So, keep up the good work.", "For the Kingdom of Rune-Midgarts!"],
                            )?;
                            {
                                if (ctx.var("BaseLevel").get()?.number()? > 80 && ctx.var("BaseLevel").get()?.number()? < 86) {
                                    ctx.call(Function::GetExperience, vec![Val::from(400000), Val::from(0)])?;
                                } else if (ctx.var("BaseLevel").get()?.number()? > 85 && ctx.var("BaseLevel").get()?.number()? < 91) {
                                    ctx.call(Function::GetExperience, vec![Val::from(450000), Val::from(0)])?;
                                } else if (ctx.var("BaseLevel").get()?.number()? > 90 && ctx.var("BaseLevel").get()?.number()? < 96) {
                                    ctx.call(Function::GetExperience, vec![Val::from(500000), Val::from(0)])?;
                                } else if (ctx.var("BaseLevel").get()?.number()? > 95 && ctx.var("BaseLevel").get()?.number()? < 99) {
                                    ctx.call(Function::GetExperience, vec![Val::from(550000), Val::from(0)])?;
                                } else if ctx.var("BaseLevel").get()?.number()? >= 99 {
                                    ctx.call(Function::GetExperience, vec![Val::from(1100000), Val::from(0)])?;
                                } else {
                                    ctx.call(Function::GetExperience, vec![Val::from(300000), Val::from(0)])?;
                                }
                            }
                            ctx.var("nk_prince").set(Val::from(8))?;
                            ctx.call(Function::CompleteQuest, vec![Val::from(10025)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as("Inspector", args!["Then, let me hit the road.."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else if (ctx.var("nk_prince").get()? == 8 || ctx.var("nk_prince").get()? == 9) {
                        ctx.lines_as("Inspector", args!["All for the glory", "of Rune-Midgarts!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Inspector", args!["Hello, appraiser.", "I hope you are able to finish your mission perfectly. Have you met with all the princes, like I told you to?"])?;
                        if ctx.var("nkprince_eisen").get()? != 10 {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
                            1 => {
                                l_prin1 = ctx.call(Function::CheckQuest, vec![Val::from(10005)])?;
                                l_prin2 = ctx.call(Function::CheckQuest, vec![Val::from(10006)])?;
                                l_prin3 = ctx.call(Function::CheckQuest, vec![Val::from(10007)])?;
                                l_prin4 = ctx.call(Function::CheckQuest, vec![Val::from(10008)])?;
                                l_prin5 = ctx.call(Function::CheckQuest, vec![Val::from(10009)])?;
                                l_prin6 = ctx.call(Function::CheckQuest, vec![Val::from(10010)])?;
                                l_prin7 = ctx.call(Function::CheckQuest, vec![Val::from(10011)])?;
                                if ((((((l_prin1.clone() + l_prin2.clone()) + l_prin3.clone()) + l_prin4.clone()) + l_prin5.clone())
                                    + l_prin6.clone())
                                    + l_prin7.clone())
                                    == 14
                                {
                                    ctx.lines_as("Inspector", args!["Very well.", "I like hearing about the princes."])?;
                                    ctx.call(Function::CompleteQuest, vec![Val::from(10004)])?;
                                    ctx.call(Function::SetQuest, vec![Val::from(10004)])?;
                                } else {
                                    ctx.lines_as("Inspector", args!["Are you sure?", "Please check on all the princes."])?;
                                }
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                l_prin1 = ctx.call(Function::CheckQuest, vec![Val::from(10005)])?;
                                l_prin2 = ctx.call(Function::CheckQuest, vec![Val::from(10006)])?;
                                l_prin3 = ctx.call(Function::CheckQuest, vec![Val::from(10007)])?;
                                l_prin4 = ctx.call(Function::CheckQuest, vec![Val::from(10008)])?;
                                l_prin5 = ctx.call(Function::CheckQuest, vec![Val::from(10009)])?;
                                l_prin6 = ctx.call(Function::CheckQuest, vec![Val::from(10010)])?;
                                l_prin7 = ctx.call(Function::CheckQuest, vec![Val::from(10011)])?;
                                if ((((((l_prin1.clone() + l_prin2.clone()) + l_prin3.clone()) + l_prin4.clone()) + l_prin5.clone())
                                    + l_prin6.clone())
                                    + l_prin7.clone())
                                    == 14
                                {
                                    ctx.lines_as("Inspector", args!["Don't be coy. I'm sure you have done it already."])?;
                                    ctx.call(Function::CompleteQuest, vec![Val::from(10004)])?;
                                    ctx.call(Function::SetQuest, vec![Val::from(10004)])?;
                                } else {
                                    ctx.lines_as("Inspector", args!["My investigations on all the princes are done."])?;
                                }
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn inspector_prince(ctx: &Ctx) -> Script {
    inspector_prince_body(ctx, Vec::new()).map(|_| ())
}

fn prince_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_prin1 = Val::from(0);
    let mut l_prin2 = Val::from(0);
    let mut l_prin3 = Val::from(0);
    let mut l_prin4 = Val::from(0);
    let mut l_prin5 = Val::from(0);
    let mut l_prin6 = Val::from(0);
    let mut l_prin7 = Val::from(0);
    if ctx.var("nk_prince").get()?.number()? > 6 {
        ctx.lines(args!["-Obssessed with making", "lock and key.-"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CheckQuest, vec![Val::from(10020)])? == 2 {
        ctx.lines_as(
            "Erich",
            args![
                "...What bad luck I have!",
                "But he insists me to be corrupted, knowing how it would be..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.call(Function::CheckQuest, vec![Val::from(10020)])? == 0 || ctx.call(Function::CheckQuest, vec![Val::from(10020)])? == 1)
    {
        ctx.lines_as(
            "Erich",
            args!["These days, I have bad luck... Only harrassments happen to me..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("nk_prince").get()?.number()? < 5 {
        ctx.lines_as(
            "Prince",
            args!["Who are you?", "I don't think you are eligible", "to be entering this room?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Prince", args!["I order you out of", "my sight."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("nk_prince").get()? == 5 {
        ctx.lines_as(
            "Prince",
            args![
                "Who are you?",
                "Someone must have been instructed about the presence of the hidden entrance. Unless... Are you the adventurer appraiser?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Yes, I am.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Prince", args!["Are you?... Do me a favor then. I am a legitimate son from the Nerius family. My name is Erich. You can call me Prince Erich."])?;
        ctx.next()?;
        ctx.lines_as("Erich", args!["My full name is..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Erich",
            args![
                "That's enough.",
                "You can ask information about me to my servant. I will take a rest."
            ],
        )?;
        ctx.var("nk_prince").set(Val::from(6))?;
        ctx.call(Function::CompleteQuest, vec![Val::from(10011)])?;
        l_prin1 = ctx.call(Function::CheckQuest, vec![Val::from(10005)])?;
        l_prin2 = ctx.call(Function::CheckQuest, vec![Val::from(10006)])?;
        l_prin3 = ctx.call(Function::CheckQuest, vec![Val::from(10007)])?;
        l_prin4 = ctx.call(Function::CheckQuest, vec![Val::from(10008)])?;
        l_prin5 = ctx.call(Function::CheckQuest, vec![Val::from(10009)])?;
        l_prin6 = ctx.call(Function::CheckQuest, vec![Val::from(10010)])?;
        l_prin7 = ctx.call(Function::CheckQuest, vec![Val::from(10011)])?;
        if ((((((l_prin1.clone() + l_prin2.clone()) + l_prin3.clone()) + l_prin4.clone()) + l_prin5.clone()) + l_prin6.clone())
            + l_prin7.clone())
            == 14
        {
            ctx.call(Function::SetQuest, vec![Val::from(10012)])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Erich", args!["......My conscience!", "It doesn't work well..."])?;
        ctx.next()?;
        ctx.mes("-He seems to be so obssessed to care about anything else.-")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn prince(ctx: &Ctx) -> Script {
    prince_body(ctx, Vec::new()).map(|_| ())
}

fn servant_hans_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckQuest, vec![Val::from(10020)])? == 2 {
        ctx.lines_as(
            "Hans",
            args![
                "Ahh... mmm... I...",
                "I... am so, sorry!",
                "I... think I made a big mistake with the appraiser!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hans",
            args!["My prince didn't do anything wrong but, anyways, I apologize for anything to you!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.call(Function::CheckQuest, vec![Val::from(10020)])? == 0 || ctx.call(Function::CheckQuest, vec![Val::from(10020)])? == 1)
    {
        ctx.lines(args![
            "Incessantly...-",
            "-He has jitters whenever I react to him. His actions give me a feeling of pity.-"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("nk_prince").get()? == 6 {
        ctx.lines_as(
            "Hans",
            args!["How are you doing, sir?", "My name is Hans, servant of Prince Erich."],
        )?;
        ctx.next()?;
        ctx.lines_as("Hans", args!["What can I do for you?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Prince Erich told me to meet you.:Nothing much.")])? {
            1 => {
                ctx.lines_as("Hans", args!["Did Prince Erich...?"])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("I want to listen to a story of him.")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Hans",
                    args!["Ahh, I see.", "Frankly, Prince Erich is not interested in regime that much."],
                )?;
                ctx.next()?;
                ctx.lines_as("Hans", args!["He writes in his journal daily. On his wedding day, he jotted down: 'Like, nothing special happened today'. He doesn't care about current events or human relationships."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Hans",
                    args![
                        "What he likes is making locks.",
                        "He is eccentric, isn't he?",
                        "He is a kind of jester."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Hans", args!["He doesn't like classy costumes either. He didn't even bring a dress suit here. And as soon as he got here, he was immersed in making a lock and key, like that."])?;
                ctx.next()?;
                ctx.lines_as("Hans", args!["I don't think I have the proper expression for you... I don't think I have something special to say about him to you. You can just appraise him as you see him..."])?;
                ctx.next()?;
                ctx.lines_as("Hans", args!["That's all I can say about this.", "Huh...~"])?;
                ctx.next()?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                ctx.lines_as("Hans", args!["Ah, Prince!", "You can cut your finger!"])?;
                ctx.next()?;
                ctx.lines_as("Erich", args!["........."])?;
                ctx.next()?;
                ctx.lines_as("Hans", args!["...Hmm he cannot even hear...", "Oooooh..."])?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["......"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Hans", args!["Did you meet the other princes?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Hans",
                    args!["Although you may ask something, you will not hear anything special from him now."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines_as("Hans", args!["Hello?", "Are you the new adventurer appraiser??"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn servant_hans(ctx: &Ctx) -> Script {
    servant_hans_body(ctx, Vec::new()).map(|_| ())
}

fn prince_urgen_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_p_a = Val::from(0);
    let mut l_p_b = Val::from(0);
    let mut l_p_c = Val::from(0);
    let mut l_prin1 = Val::from(0);
    let mut l_prin2 = Val::from(0);
    let mut l_prin3 = Val::from(0);
    let mut l_prin4 = Val::from(0);
    let mut l_prin5 = Val::from(0);
    let mut l_prin6 = Val::from(0);
    let mut l_prin7 = Val::from(0);
    if ctx.var("nk_prince").get()?.number()? > 6 {
        ctx.lines_as("Urugen", args!["...It is not beautiful.", "It discomforts me..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CheckQuest, vec![Val::from(10021)])? == 2 {
        ctx.lines_as(
            "Urugen",
            args![" feel very much displeased. Hey, what are you looking at? Get out!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Urugen",
            args!["My beautiful body isn't in perfect condition right now. I don't want to show it to anyone today."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.call(Function::CheckQuest, vec![Val::from(10021)])? == 0 || ctx.call(Function::CheckQuest, vec![Val::from(10021)])? == 1)
    {
        ctx.lines_as("Urugen", args!["...What? What did you...", "just say to me?... Huh?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("nk_prince").get()?.number()? < 5 {
        ctx.lines_as("Prince", args!["Huuuuuuu", "The position is not suitable for you."])?;
        ctx.next()?;
        ctx.lines_as("Prince", args!["Get away from me, as soon as you can."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("nk_prince").get()?.number()? > 4 {
        ctx.lines_as(
            "Prince",
            args![
                "La~ lalalala~ lalala~",
                "Are you the person, supposed to look me over and appraise my quality?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Prince",
            args!["The daffodil you are gazing at is called Urugen. It bloomed at Wigner family."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Urugen",
            args!["I usually don't let anyone hear my beautiful voice, but this time, I will give a special service for you."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Urugen",
            args!["listen...", "Let me answer with my", "unforgettable, clear and beautiful voice."],
        )?;
        ctx.next()?;
        'l1: loop {
            if !(true) {
                break 'l1;
            }
            'b1: {
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "I want to know your background.:Let me know your view of the nation.:What are your hobbies or tastes?:Let me leave.",
                    )],
                )? {
                    1 => {
                        ctx.lines_as("Urugen", args!["My dashing face from", "childhood brought envy and jealousy from men, and endless proposals from women. I felt sick with it, so I came to be away from people."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Urugen",
                            args!["My beautiful person", "shouldn't bear stuff like that sometimes."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Urugen",
                            args!["I became timid, gradually, in phases. I feel awe from men and women both, regardless of sexuality."],
                        )?;
                        l_p_a = Val::from(1);
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as("Urugen", args!["People should do what they are supposed to do. That's the source of drive for a nation. For me, my existence will be enough, for the nation."])?;
                        ctx.next()?;
                        ctx.lines_as("Urugen", args!["The presence of such a gorgeous king like me will be the light for people and the hope and reason for their lives."])?;
                        l_p_b = Val::from(1);
                        ctx.next()?;
                    }
                    3 => {
                        ctx.lines_as(
                            "Urugen",
                            args!["For sure, taking care of my body. Humans should pursue beauty. It's quite natural, isn't it?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Urugen", args!["In every case, there is an exception, like you in this court. Can you stand away from me a bit more? Because of your odor, I can hardly breathe."])?;
                        l_p_c = Val::from(1);
                        ctx.next()?;
                    }
                    4 => {
                        ctx.lines_as(
                            "Urugen",
                            args![
                                "Your spirit must be so strong. Looking over my beauty so long a time, you haven't lost your spirit yet."
                            ],
                        )?;
                        if ((l_p_a.clone() + l_p_b.clone()) + l_p_c.clone()) == 3 {
                            ctx.call(Function::CompleteQuest, vec![Val::from(10009)])?;
                        }
                        ctx.call(Function::CompleteQuest, vec![Val::from(10011)])?;
                        l_prin1 = ctx.call(Function::CheckQuest, vec![Val::from(10005)])?;
                        l_prin2 = ctx.call(Function::CheckQuest, vec![Val::from(10006)])?;
                        l_prin3 = ctx.call(Function::CheckQuest, vec![Val::from(10007)])?;
                        l_prin4 = ctx.call(Function::CheckQuest, vec![Val::from(10008)])?;
                        l_prin5 = ctx.call(Function::CheckQuest, vec![Val::from(10009)])?;
                        l_prin6 = ctx.call(Function::CheckQuest, vec![Val::from(10010)])?;
                        l_prin7 = ctx.call(Function::CheckQuest, vec![Val::from(10011)])?;
                        if ((((((l_prin1.clone() + l_prin2.clone()) + l_prin3.clone()) + l_prin4.clone()) + l_prin5.clone())
                            + l_prin6.clone())
                            + l_prin7.clone())
                            == 14
                        {
                            ctx.call(Function::SetQuest, vec![Val::from(10012)])?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn prince_urgen(ctx: &Ctx) -> Script {
    prince_urgen_body(ctx, Vec::new()).map(|_| ())
}

fn prince_helmut_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_prin1 = Val::from(0);
    let mut l_prin2 = Val::from(0);
    let mut l_prin3 = Val::from(0);
    let mut l_prin4 = Val::from(0);
    let mut l_prin5 = Val::from(0);
    let mut l_prin6 = Val::from(0);
    let mut l_prin7 = Val::from(0);
    if ctx.var("nk_prince").get()?.number()? > 6 {
        ctx.lines_as(
            "Helmut",
            args!["Such an idiot. I should have killed him earlier. Now I feel relieved."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CheckQuest, vec![Val::from(10022)])? == 2 {
        ctx.lines_as(
            "Helmut",
            args![
                "Damn... Damn it!",
                "Novice of Walter!!",
                "How can I deal with this stress? Damn! Hell!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.call(Function::CheckQuest, vec![Val::from(10022)])? == 0 || ctx.call(Function::CheckQuest, vec![Val::from(10022)])? == 1)
    {
        ctx.lines(args!["-He is so blushed,", "evidently shown on his face.-"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("nk_prince").get()?.number()? < 5 {
        ctx.lines_as(
            "Helmut",
            args!["You are not supposed to be here!", "Can't you keep away from me!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("nk_prince").get()?.number()? > 4 {
        ctx.lines_as(
            "Helmut",
            args!["Are you the new appraiser? I am fed up with the many visitors! Let's take up the main subject!"],
        )?;
        ctx.next()?;
        ctx.mes("-What subject should I start with?-")?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from(
                "Your background...:Your ambition...:Your view of the nation...:I want to meet others first.",
            )],
        )? {
            1 => {
                ctx.lines_as("Helmut", args!["I don't know how others react but, I feel very uptight with your question. I shouldn't, but, I don't have a different view!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Helmut",
                    args!["I am the prince; if you grumble, you become prince! I feel tiresome with this kind of questioning."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["............", "Yes, I understand. Go ahead..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Helmut",
                    args![
                        "I am Helmut from Roewenburg.",
                        "What I like is smelling blood in the battlefield, and I enjoy festivals too."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Helmut", args!["What I hate is sticking to formality, and talking much. As for ambition? What am I going to do when I become... king...?? I am asked this question many times."])?;
                ctx.next()?;
                ctx.lines_as("Helmut", args!["When I become king, I will reinforce our troops and conquer continent after unification of continent; I will eradicate all the monsters that are harmful to my people!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Helmut",
                    args!["~Kuffkuff~! I have a sore throat! It's been a while since I've used my throat. ~Kuffkuff~!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Helmut", args!["Hey Calbern!", "Bring beer! Beer!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Helmut",
                    args!["Why are you still standing there? I have nothing else to say to you! Stand back!"],
                )?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["......"])?;
                ctx.call(Function::CompleteQuest, vec![Val::from(10010)])?;
                l_prin1 = ctx.call(Function::CheckQuest, vec![Val::from(10005)])?;
                l_prin2 = ctx.call(Function::CheckQuest, vec![Val::from(10006)])?;
                l_prin3 = ctx.call(Function::CheckQuest, vec![Val::from(10007)])?;
                l_prin4 = ctx.call(Function::CheckQuest, vec![Val::from(10008)])?;
                l_prin5 = ctx.call(Function::CheckQuest, vec![Val::from(10009)])?;
                l_prin6 = ctx.call(Function::CheckQuest, vec![Val::from(10010)])?;
                l_prin7 = ctx.call(Function::CheckQuest, vec![Val::from(10011)])?;
                if ((((((l_prin1.clone() + l_prin2.clone()) + l_prin3.clone()) + l_prin4.clone()) + l_prin5.clone()) + l_prin6.clone())
                    + l_prin7.clone())
                    == 14
                {
                    ctx.call(Function::SetQuest, vec![Val::from(10012)])?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Helmut", args!["I don't know how others react but, I feel very uptight with your question. I shouldn't, but, I don't have a different view!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Helmut",
                    args!["I am the prince; if you grumble, you become prince! I feel tiresome with this kind of questioning."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["............", "Yes, I understand. Go ahead..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Helmut",
                    args![
                        "I am Helmut from Roewenburg.",
                        "What I like is smelling blood in the battlefield, and I enjoy festivals too."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Helmut", args!["What I hate is sticking to formality, and talking much. As for ambition? What am I going to do when I become... king...?? I am asked this question many times."])?;
                ctx.next()?;
                ctx.lines_as("Helmut", args!["When I become king, I will reinforce our troops and conquer continent after unification of continent; I will eradicate all the monsters that are harmful to my people!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Helmut",
                    args!["~Kuffkuff~! I have a sore throat! It's been a while since I've used my throat. ~Kuffkuff~!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Helmut", args!["Hey Calbern!", "Bring beer! Beer!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Helmut",
                    args!["Why are you still standing there? I have nothing else to say to you! Stand back!"],
                )?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["......"])?;
                ctx.call(Function::CompleteQuest, vec![Val::from(10010)])?;
                l_prin1 = ctx.call(Function::CheckQuest, vec![Val::from(10005)])?;
                l_prin2 = ctx.call(Function::CheckQuest, vec![Val::from(10006)])?;
                l_prin3 = ctx.call(Function::CheckQuest, vec![Val::from(10007)])?;
                l_prin4 = ctx.call(Function::CheckQuest, vec![Val::from(10008)])?;
                l_prin5 = ctx.call(Function::CheckQuest, vec![Val::from(10009)])?;
                l_prin6 = ctx.call(Function::CheckQuest, vec![Val::from(10010)])?;
                l_prin7 = ctx.call(Function::CheckQuest, vec![Val::from(10011)])?;
                if ((((((l_prin1.clone() + l_prin2.clone()) + l_prin3.clone()) + l_prin4.clone()) + l_prin5.clone()) + l_prin6.clone())
                    + l_prin7.clone())
                    == 14
                {
                    ctx.call(Function::SetQuest, vec![Val::from(10012)])?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as("Helmut", args!["I don't know how others react but, I feel very uptight with your question. I shouldn't, but, I don't have a different view!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Helmut",
                    args!["I am the prince; if you grumble, you become prince! I feel tiresome with this kind of questioning."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["............", "Yes, I understand. Go ahead..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Helmut",
                    args![
                        "I am Helmut from Roewenburg.",
                        "What I like is smelling blood in the battlefield, and I enjoy festivals too."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Helmut", args!["What I hate is sticking to formality, and talking much. As for ambition? What am I going to do when I become... king...?? I am asked this question many times."])?;
                ctx.next()?;
                ctx.lines_as("Helmut", args!["When I become king, I will reinforce our troops and conquer continent after unification of continent; I will eradicate all the monsters that are harmful to my people!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Helmut",
                    args!["~Kuffkuff~! I have a sore throat! It's been a while since I've used my throat. ~Kuffkuff~!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Helmut", args!["Hey Calbern!", "Bring beer! Beer!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Helmut",
                    args!["Why are you still standing there? I have nothing else to say to you! Stand back!"],
                )?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["......"])?;
                ctx.call(Function::CompleteQuest, vec![Val::from(10010)])?;
                l_prin1 = ctx.call(Function::CheckQuest, vec![Val::from(10005)])?;
                l_prin2 = ctx.call(Function::CheckQuest, vec![Val::from(10006)])?;
                l_prin3 = ctx.call(Function::CheckQuest, vec![Val::from(10007)])?;
                l_prin4 = ctx.call(Function::CheckQuest, vec![Val::from(10008)])?;
                l_prin5 = ctx.call(Function::CheckQuest, vec![Val::from(10009)])?;
                l_prin6 = ctx.call(Function::CheckQuest, vec![Val::from(10010)])?;
                l_prin7 = ctx.call(Function::CheckQuest, vec![Val::from(10011)])?;
                if ((((((l_prin1.clone() + l_prin2.clone()) + l_prin3.clone()) + l_prin4.clone()) + l_prin5.clone()) + l_prin6.clone())
                    + l_prin7.clone())
                    == 14
                {
                    ctx.call(Function::SetQuest, vec![Val::from(10012)])?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            4 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["No, Prince,", "I will be back later."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Helmut",
                    args![
                        "Alright, up to you.",
                        "Hey! Hey Calbern!",
                        "Move your ass here with beer!",
                        "Do you want to be beaten down?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn prince_helmut(ctx: &Ctx) -> Script {
    prince_helmut_body(ctx, Vec::new()).map(|_| ())
}

fn calbern_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Calbern",
        args!["How are you doing, sir?", "I am the servant of Prince Helmut...", "Calbern."],
    )?;
    ctx.next()?;
    ctx.lines_as("Helmut", args!["Hey Calbern!", "Bring beer! Beer!"])?;
    ctx.next()?;
    ctx.lines_as("Calbern", args!["Oops, I am so sorry.", "I got business..."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn calbern(ctx: &Ctx) -> Script {
    calbern_body(ctx, Vec::new()).map(|_| ())
}
