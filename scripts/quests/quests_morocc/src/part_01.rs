use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn william_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("William", args!["Welcome to", "MacMillan's ^3355FFPost^000000 Workshop."])?;
    ctx.next()?;
    ctx.lines_as("William", args!["My family, the MacMillan Clan, has been producing Professional Traffic Signal Posts for more than 250 years. Nowadays, we are booking Special Orders for our unique ornament, ^3355FFStop Post^000000."])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Could I order one ^3355FFStop Post^000000?:Nah...")])? {
        1 => {
            ctx.lines_as("William", args!["This unique ornament, ^3355FFStop Post^000000, is a traffic signal on the road, and doubles as a hair ornament! This, we solemnly promise on the strength of a 100-year guarantee."])?;
            ctx.next()?;
            ctx.lines_as("William", args!["To produce a ^3355FFStop Post^000000, we need ^3355FF91100 Zeny^000000, ^3355FF50 Trunk^000000 and ^3355FF1 Black Dyestuffs^000000."])?;
            ctx.next()?;
            if ((ctx.call(Function::CountItem, vec![Val::from(1019)])?.number()? > 49
                && ctx.call(Function::CountItem, vec![Val::from(983)])?.number()? > 0)
                && ctx.var("Zeny").get()?.number()? > 91099)
            {
                ctx.call(Function::DelItem, vec![Val::from(1019), Val::from(50)])?;
                ctx.call(Function::DelItem, vec![Val::from(983), Val::from(1)])?;
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(91100))?))?;
                ctx.lines_as(
                    "William",
                    args![
                        "Here you are~!",
                        "This ^3355FFStop Post^000000 has",
                        "been especially made",
                        "just for you!",
                        "Thank you for stopping by!",
                        "...Get the joke?"
                    ],
                )?;
                ctx.call(Function::GetItem, vec![Val::from(2272), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "William",
                    args!["So...", "When you get those items, swing on by and we'll give you a Stop Post."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        2 => {
            ctx.lines_as(
                "William",
                args!["Anyway, thank you for coming by 'MacMillan's Workshop.' But think about buying something next time, will ya?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn william(ctx: &Ctx) -> Script {
    william_body(ctx, Vec::new()).map(|_| ())
}

fn alchemist_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Marius",
        args![
            "Howdy, new customer!!",
            "I know what you're gonna talk about. You want the Magic Glasses from me, riiiiight?"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("What is that?:Manufacture:Ignore him")])? {
        1 => {
            ctx.lines_as("Marius", args!["Hmm~!", "Well, the official name for them is ^3355FFBinoculars^000000! An optical device that works like a pair of field glasses, they're designed for simultaneous use by both eyes!"])?;
            ctx.next()?;
            ctx.lines_as("Marius", args!["They're made up of two small telescopes joined with a single focusing device. You can arrange the lenses to produce stereoscopic vision."])?;
            ctx.next()?;
            ctx.lines_as(
                "Marius",
                args!["So?", "Ain't that", "something, huh?", "Muhahahahahaha!", ". . . . ."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Marius",
                args!["Hm? You don't seem to", "believe what I just said?", "Oh c'mon, jerk. It's real!!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Marius",
                args![
                    "I will let you",
                    "know the items I need...",
                    "1 ^3355FFGeek Glasses^000000! 100 ^3355FFSteel^000000!",
                    "And ^3355FF50000 Zeny^000000!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            if ((ctx.call(Function::CountItem, vec![Val::from(2243)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(999)])?.number()? > 99)
                && ctx.var("Zeny").get()?.number()? > 49999)
            {
                ctx.lines_as(
                    "Marius",
                    args!["Perfect, perfect !", "Now my masterpiece will be complete!", "Muhahahaha !"],
                )?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(2243), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(999), Val::from(100)])?;
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(50000))?))?;
                ctx.lines_as("Marius", args!["Here you are!", "Binoculars !"])?;
                ctx.call(Function::GetItem, vec![Val::from(2296), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Marius",
                    args![
                        "Just remember...",
                        "Don't peep at something you shouldn't look at. Well, at least try not to."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Marius",
                    args![
                        "Argggghhhhhhh!!",
                        "You didn't bring",
                        "enough items!!!!",
                        "How dare you",
                        "disgrace me!!!",
                        "Baaaadddd !!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        3 => {
            ctx.lines_as(
                "Marius",
                args![
                    "Hey you~!",
                    "Now you're in front of Marius, an Alchemist among Alchemists. Ignoring me, eh? Come on, I'll beat your ass!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn alchemist(ctx: &Ctx) -> Script {
    alchemist_body(ctx, Vec::new()).map(|_| ())
}

fn reading_girl_moc_girl1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["Excuse me, but may I ask", "you a question?"],
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
    let choice = runtime::select_values(ctx, &[Val::from("What are you reading?")])?;
    ctx.var("@menu").set(choice)?;
    ctx.lines_as(
        "Yunia",
        args![
            "Ah! This is one of the best sellers of Joshua Vansei's.",
            "The main character is a writer, and it's so touching that this guy writes a story of his mistress..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Yunia",
        args![
            "Oh.. I can't wait to see Hoein Special..",
            "And this, you could have a look if you want to."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args![
            "Oh, now, I shouldn't.",
            "Looks like the cover's rounded with a red strap,",
            "which..I really think is for adults only.",
            "ex..cuse me~!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn reading_girl_moc_girl1(ctx: &Ctx) -> Script {
    reading_girl_moc_girl1_body(ctx, Vec::new()).map(|_| ())
}

fn assistant_moc_ex1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Sephit",
        args![
            "I've always believed in Satan..",
            "Haah... Guess it's good to be living, after all."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Sephit",
        args!["Who would've thought to actually see the Satan while alive.....?"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn assistant_moc_ex1(ctx: &Ctx) -> Script {
    assistant_moc_ex1_body(ctx, Vec::new()).map(|_| ())
}

fn girl_moc_ex002_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Girl",
        args![
            "Oh, the world.. the world is doomed..",
            "-sobbing-",
            "Nothing seems to be working now.."
        ],
    )?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args![
            "....Tsk..tsk...",
            "Good heavens.. I can't believe this young girl's drinking with sadness...."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn girl_moc_ex002(ctx: &Ctx) -> Script {
    girl_moc_ex002_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ContinentalGuard01Step {
    Start,
    OnTouch,
}

fn continental_guard_01_run(ctx: &Ctx, mut step: ContinentalGuard01Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_onlinemembers = Val::from(0);
    let mut l_partymembercid: Vec<Val> = Vec::new();
    let mut l_partymembercount = Val::from(0);
    'machine: loop {
        match step {
            ContinentalGuard01Step::Start => {
                if (ctx.var("rebirth_moc_edq").get()? == 0 && ctx.var("rebirth_moc_edq").get()?.number()? < 4) {
                    ctx.lines_as(
                        "Continental Guard",
                        args![
                            "No commoners are allowed in the area beyond this point.",
                            "This place is extremely dangerous so you are restricted from entering."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Cancel Conversation:Ask What Happened")])? {
                        1 => {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Continental Guard",
                                args![
                                    "Didn't you know? Satan Morocc has resurrected and broke out of Morocc Village where he was confined."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Continental Guard", args!["His resurrection has caused irreparable damage to the village and to the desert around it, and now he has moved to the Sograt Desert."])?;
                            ctx.next()?;
                            ctx.lines_as("Continental Guard", args!["We are here to carry out the orders of the Prontera Kingdom by preventing commoners, aside from the members of the Morocc Subjugation, from accessing the area."])?;
                            ctx.next()?;
                            ctx.lines_as("Continental Guard", args!["If you'd like to know more information, I suggest that you speak to the Continental Guard in charge of the accident site in Morocc Village."])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("End Conversation:Ask About Guard's Location")])? {
                                1 => {
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Continental Guard",
                                        args!["The guard that you want to talk to is at a camp built in the center of Morocc Village."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Continental Guard", args!["If you'd like, I can send you there directly."])?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("No, thanks.:Please do.")])? {
                                        1 => {
                                            ctx.lines_as("Continental Guard", args!["I see. Well then, for your safety, please leave this dangerous area as soon as possible."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as(
                                                "Continental Guard",
                                                args!["Great. I'll send you to Morocc Village's accident site shortly."],
                                            )?;
                                            ctx.close_window()?;
                                            ctx.call(Function::Warp, vec![Val::from("morocc"), Val::from(160), Val::from(61)])?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                } else if (ctx.var("rebirth_moc_edq").get()?.number()? > 3 && ctx.var("rebirth_moc_edq").get()?.number()? < 8) {
                    ctx.lines_as(
                        "Continental Guard",
                        args![
                            "No commoners are allowed in the area beyond this point.",
                            "This place is extremely dangerous so you are restricted from entering."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "Cancel Conversation:Enter the Field to Investigate:Move to Morocc's Accident Site",
                        )],
                    )? {
                        1 => {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            if ctx.var("$@re_moc").get()?.number()? < 3 {
                                runtime::party_members(ctx, ctx.call(Function::GetCharacterId, vec![Val::from(1)])?, Val::from(1))?;
                                l_partymembercount = ctx.var("$@partymembercount").get()?;
                                l_i = Val::from(0);
                                'l5: loop {
                                    if !(runtime::op(&l_i.clone(), "<", &l_partymembercount.clone())?.is_true()) {
                                        break 'l5;
                                    }
                                    'b5: {
                                        if ctx
                                            .call(
                                                Function::ConvertPcInfo,
                                                vec![
                                                    runtime::local_get(&l_partymembercid, &l_i.clone(), false),
                                                    ctx.constant("CPC_ACCOUNT")?,
                                                ],
                                            )?
                                            .is_true()
                                        {
                                            l_onlinemembers = (l_onlinemembers.clone() + Val::from(1));
                                        }
                                    }
                                    l_i = (l_i.clone() + Val::from(1));
                                }
                                if (l_onlinemembers.clone().number()? > 1
                                    && ctx.call(Function::CountItem, vec![Val::from(7826)])?.number()? > 0)
                                {
                                    ctx.lines_as("Continental Guard", args!["......"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Continental Guard", args!["Yes, I've confirmed that you're a member of the Continental Guards. I wish you good luck in accomplishing your mission."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Continental Guard", args!["I'll send you to the field shortly. Please use the warp at the field entrance to come back to this area."])?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Warp, vec![Val::from("moc_fild21"), Val::from(38), Val::from(193)])?;
                                    return Err(Stop::End);
                                } else if (l_onlinemembers.clone().number()? < 2
                                    && ctx.call(Function::CountItem, vec![Val::from(7826)])?.number()? > 0)
                                {
                                    ctx.lines_as("Continental Guard", args!["Welcome, members of the Continental Guards. I have a special order from Chief Balrog for you, so let me read it for you."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Continental Guard", args!["'Due to the dangers of this area, I hereby prohibit members of the Continental Guard to investigate the area alone. You must organize a party of at least 2 members to carry out your missions from now on.'"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Continental Guard", args!["...You understand that, right? Please go back to the site, and come back in a party of at least 2 members. Thank you."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.call(Function::CountItem, vec![Val::from(7826)])?.number()? < 1 {
                                    ctx.lines_as("Continental Guard", args!["Only members of the Continental Guards with Continental Guard Certificates are allowed to proceed beyond this point."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            } else {
                                ctx.lines_as("Continental Guard", args!["We've received orders from Headquarters to block access to this area since an unusual space-time phenomenon has been detected from the Morocc field."])?;
                                ctx.next()?;
                                ctx.lines_as("Continental Guard", args!["We need to wait until the phenomenon is over, and then we'll let you proceed with your investigation."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        3 => {
                            ctx.lines_as(
                                "Continental Guard",
                                args!["Great. I'll send you to Morocc Village's accident site shortly."],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("morocc"), Val::from(160), Val::from(61)])?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("rebirth_moc_edq").get()? == 8 {
                    ctx.lines_as("Continental Guard", args!["Ah, you're an adventurer working for the Continental Guard. Nice to meet you. Feel free to ask me if you need my assistance."])?;
                    ctx.next()?;
                    runtime::party_members(ctx, ctx.call(Function::GetCharacterId, vec![Val::from(1)])?, Val::from(1))?;
                    l_partymembercount = ctx.var("$@partymembercount").get()?;
                    l_i = Val::from(0);
                    'l6: loop {
                        if !(runtime::op(&l_i.clone(), "<", &l_partymembercount.clone())?.is_true()) {
                            break 'l6;
                        }
                        'b6: {
                            if ctx
                                .call(
                                    Function::ConvertPcInfo,
                                    vec![
                                        runtime::local_get(&l_partymembercid, &l_i.clone(), false),
                                        ctx.constant("CPC_ACCOUNT")?,
                                    ],
                                )?
                                .is_true()
                            {
                                l_onlinemembers = (l_onlinemembers.clone() + Val::from(1));
                            }
                        }
                        l_i = (l_i.clone() + Val::from(1));
                    }
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "Enter the First Field to Investigate:Enter the Second Field to Investigate:Return to Morocc's Accident Site:Cancel Conversation",
                        )],
                    )? {
                        1 => {
                            if (l_onlinemembers.clone().number()? > 1
                                && ctx.call(Function::CountItem, vec![Val::from(7826)])?.number()? > 0)
                            {
                                ctx.lines_as("Continental Guard", args!["......"])?;
                                ctx.next()?;
                                ctx.lines_as("Continental Guard", args!["Yes, I've confirmed that you're a member of the Continental Guards. I wish you good luck in accomplishing your mission."])?;
                                ctx.next()?;
                                ctx.lines_as("Continental Guard", args!["I'll send you to the field shortly. Please use the warp at the field entrance to come back to this area."])?;
                                ctx.close_window()?;
                                ctx.call(Function::Warp, vec![Val::from("moc_fild21"), Val::from(38), Val::from(193)])?;
                                return Err(Stop::End);
                            } else if (l_onlinemembers.clone().number()? < 2
                                && ctx.call(Function::CountItem, vec![Val::from(7826)])?.number()? > 0)
                            {
                                ctx.lines_as("Continental Guard", args!["Welcome, members of the Continental Guards. I have a special order from Chief Balrog for you, so let me read it for you."])?;
                                ctx.next()?;
                                ctx.lines_as("Continental Guard", args!["'Due to the dangers of this area, I hereby prohibit members of the Continental Guard to investigate the area alone. You must organize a party of at least 2 members to carry out your missions from now on.'"])?;
                                ctx.next()?;
                                ctx.lines_as("Continental Guard", args!["...You understand that, right? Please go back to the site, and come back in a party of at least 2 members. Thank you."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.call(Function::CountItem, vec![Val::from(7826)])?.number()? < 1 {
                                ctx.lines_as("Continental Guard", args!["Only members of the Continental Guards with Continental Guard Certificates are allowed to proceed beyond this point."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        2 => {
                            if (l_onlinemembers.clone().number()? > 1
                                && ctx.call(Function::CountItem, vec![Val::from(7826)])?.number()? > 0)
                            {
                                ctx.lines_as("Continental Guard", args!["......"])?;
                                ctx.next()?;
                                ctx.lines_as("Continental Guard", args!["Yes, I've confirmed that you're a member of the Continental Guards. I wish you good luck in accomplishing your mission."])?;
                                ctx.next()?;
                                ctx.lines_as("Continental Guard", args!["I'll send you to the field shortly. Please use the warp at the field entrance to come back to this area."])?;
                                ctx.close_window()?;
                                ctx.call(Function::Warp, vec![Val::from("moc_fild22"), Val::from(38), Val::from(193)])?;
                                return Err(Stop::End);
                            } else if (l_onlinemembers.clone().number()? < 2
                                && ctx.call(Function::CountItem, vec![Val::from(7826)])?.number()? > 0)
                            {
                                ctx.lines_as("Continental Guard", args!["Welcome, members of the Continental Guards. I have a special order from Chief Balrog for you, so let me read it for you."])?;
                                ctx.next()?;
                                ctx.lines_as("Continental Guard", args!["'Due to the dangers of this area, I hereby prohibit members of the Continental Guard to investigate the area alone. You must organize a party of at least 2 members to carry out your missions from now on.'"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Continental Guard",
                                    args!["...So please go back to the site, and come back in a party of at least 2 members. Thank you."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.call(Function::CountItem, vec![Val::from(7826)])?.number()? < 1 {
                                ctx.lines_as("Continental Guard", args!["Only members of the Continental Guards with Continental Guard Certificates are allowed to proceed beyond this point."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        3 => {
                            ctx.lines_as(
                                "Continental Guard",
                                args!["Great. I'll send you to Morocc Village's accident site shortly."],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("morocc"), Val::from(160), Val::from(61)])?;
                            return Err(Stop::End);
                        }
                        4 => {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    return Err(Stop::End);
                }
                step = ContinentalGuard01Step::OnTouch;
                continue 'machine;
            }
            ContinentalGuard01Step::OnTouch => {
                ctx.lines_as(
                    "Continental Guard",
                    args![
                        "No commoners are allowed in the area beyond this point.",
                        "This place is extremely dangerous so you are restricted from entering."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
                return Err(Stop::End);
            }
        }
    }
}

pub fn continental_guard_01(ctx: &Ctx) -> Script {
    continental_guard_01_run(ctx, ContinentalGuard01Step::Start, Vec::new()).map(|_| ())
}

pub fn continental_guard_01_ontouch(ctx: &Ctx) -> Script {
    continental_guard_01_run(ctx, ContinentalGuard01Step::OnTouch, Vec::new()).map(|_| ())
}

fn continental_messenger_00_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_area_s = Val::from("");
    l_area_s = ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?;
    if l_area_s.clone() == "01" {
        l_area_s = Val::from("Prontera");
    } else if l_area_s.clone() == "02" {
        l_area_s = Val::from("Geffen");
    } else if l_area_s.clone() == "03" {
        l_area_s = Val::from("Payon");
    } else if l_area_s.clone() == "04" {
        l_area_s = Val::from("Alberta");
    } else if l_area_s.clone() == "05" {
        l_area_s = Val::from("Al De Baran");
    }
    if ctx.var("BaseLevel").get()?.number()? > 79 {
        ctx.lines_as(
            "Continental Guard Messenger",
            args![
                ((Val::from("Good day, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from(". We don't have the luxury of time in this dire situation so I'll try to make this quick."))
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Continental Guard Messenger", args![((((Val::from("I'm a messenger dispatched here to ") + l_area_s.clone()) + Val::from(" from the Morocc Continental Guard Headquarters. My duty is to deliver this important message to as many renowned adventurers as I can. I'm glad that I was finally able to find you, ")) + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Morocc Continental Guard Headquarters?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Continental Guard Messenger",
            args!["Time's running out, so I can only give you a brief explanation."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Continental Guard Messenger",
            args![
                "Satan Morocc, the heinous demon imprisoned deep in Morocc's underground for centuries, has resurrected and broken free."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Continental Guard Messenger",
            args!["The revival of Satan Morocc has completely devastated Morocc Village and the area surrounding it."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Continental Guard Messenger",
            args![
                "All of the death and suffering... Anywhere else must seem like heaven compared to what's happening in and around Morocc."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Continental Guard Messenger", args!["The Morocc Continental Guard was immediately organized to fight Satan Morocc and bring relief to this dire situation. I'm here to inform everyone in the Rune-Midgarts Kingdom about this catastrophe."])?;
        ctx.next()?;
        ctx.lines_as(
            "Continental Guard Messenger",
            args![
                ((Val::from("We need your help, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from(". We need reputable adventurers like you to seal away Satan Morocc once again."))
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Continental Guard Messenger",
            args![
                ((Val::from("For more information, please speak to the captain of the Morocc Continental Guard. He is waiting for you, ")
                    + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from("."))
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Continental Guard Messenger", args!["The Continental Guard Headquarters is located near the center palace of Morocc, so please go speak to the chief as soon as you can."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Continental Guard Messenger",
            args![
                ((Val::from("I'm a messenger from the Morocc Continental Guards Headquarters, and I've come here to ") + l_area_s.clone())
                    + Val::from(" with an urgent message for everyone here in Rune-Midgarts Kingdom."))
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Morocc Continental Guard Headquarters?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Continental Guard Messenger",
            args!["Time's running out, so I can only give you a brief explanation."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Continental Guard Messenger",
            args![
                "Satan Morocc, the heinous demon imprisoned deep in Morocc's underground for centuries, has resurrected and broken free."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Continental Guard Messenger",
            args!["The revival of Satan Morocc has completely devastated Morocc Village and the area surrounding it."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Continental Guard Messenger",
            args![
                "All of the death and suffering... Anywhere else must seem like heaven compared to what's happening in and around Morocc."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Continental Guard Messenger", args!["The Morocc Continental Guard was immediately organized to fight Satan Morocc and bring relief to this dire situation. I'm here to inform everyone in the Rune-Midgarts Kingdom about this catastrophe."])?;
        ctx.next()?;
        ctx.lines_as("Continental Guard Messenger", args!["Part of my duty is to help recruit adventurers from around the world to help drive Satan Morocc back into confinement. It won't be easy, but he was defeated once before. Satan Morocc can be sealed away again."])?;
        ctx.next()?;
        ctx.lines_as(
            "Continental Guard Messenger",
            args!["At least 70% of Morocc Village was destroyed by the fallout of Satan Morocc's resurrection."],
        )?;
        ctx.next()?;
        ctx.lines_as("Continental Guard Messenger", args!["The desert around Morocc has also turned into a land of death after it was claimed by Satan Morocc and his powerful doppelgangers and underlings.", "So heed my warning, stay away from the Morocc area unless you're on a mission organized by the Morocc Continental Guard."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn continental_messenger_00(ctx: &Ctx) -> Script {
    continental_messenger_00_body(ctx, Vec::new()).map(|_| ())
}

fn continental_messenger_00_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("BaseLevel").get()?.number()? > 79 {
        ctx.lines_as(
            "Continental Guard Messenger",
            args![
                ((Val::from("Are you... ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
                "Ah, very well. I have an extremely important message for you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Continental Guard Messenger",
            args!["The situation is urgent, so please listen to me carefully."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn continental_messenger_00_ontouch(ctx: &Ctx) -> Script {
    continental_messenger_00_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn continental_official_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rebirth_moc_edq").get()?.number()? > 3 {
        ctx.lines_as("Continental Guard Official", args!["On behalf of the Continental Guard, I thank you for your efforts. Now, let me see if I've received any news from Headquarters that you should know..."])?;
        ctx.next()?;
        ctx.lines_as("Continental Guard Official", args!["Hmm..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Continental Guard Official",
            args!["Nothing yet. Please carry on with your current mission. Once again, thank you."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rebirth_moc_edq").get()? == 0 {
        ctx.lines_as(
            "Continental Guard Official",
            args!["Welcome to the Morocc Subjugation Information Center. How may I help you?"],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from(
                "What is this place for?:I'm here to volunteer.:Tell me about the village situation.",
            )],
        )? {
            1 => {
                ctx.lines_as(
                    "Continental Guard Official",
                    args![
                        "I'm stationed here to assist adventurers who wish to volunteer and help the Continental Guard fight Satan Morocc."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Continental Guard Official", args!["I am sure you are already aware that Satan Morocc's revival threatens the peace of the Rune-Midgarts kingdom, and if Satan Morocc is allowed to roam free, it will devour the entire world."])?;
                ctx.next()?;
                ctx.lines_as("Continental Guard Official", args!["The Continental Guard is currently planning an array of countermeasures to suppress Satan Morocc under the order of the kingdom."])?;
                ctx.next()?;
                ctx.lines_as("Continental Guard Official", args!["If you have been invited by our messenger, or are confident in your skills, we encourage you to volunteer for the Continental Guards and bring peace back to this continent."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                if ctx.var("BaseLevel").get()?.number()? > 79 {
                    ctx.lines_as(
                        "Continental Guard Official",
                        args![
                            ((Val::from("Welcome, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(". I've been waiting for you. I assume our messenger informed you of our situation."))
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Continental Guard Official",
                        args!["Let me process your application immediately. Please wait."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Continental Guard Official",
                        args!["Now please go speak to Chief Balrog of the Continental Guard. You can find him in the center."],
                    )?;
                    ctx.var("rebirth_moc_edq").set(Val::from(1))?;
                    ctx.call(Function::SetQuest, vec![Val::from(3050)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Continental Guard Official",
                        args!["I applaud you for your courage, but you will need more than just courage to help us."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Continental Guard Official", args!["Satan Morocc is most powerful evil that exists in the mortal world. Snuffing out your life would be so trivial to him."])?;
                    ctx.next()?;
                    ctx.lines_as("Continental Guard Official", args!["I strongly recommend that you just think about your own safety for now. When the final battle comes, I am sure that no place will be safe."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            3 => {
                ctx.lines_as("Continental Guard Official", args!["As you can see, the situation can't be worse. The village and the surrounding area were irreparably damaged by Satan Morocc's resurrection."])?;
                ctx.next()?;
                ctx.lines_as("Continental Guard Official", args!["Since the kingdom has dispatched Continental Guard Messengers everywhere, many able adventurers have flocked to this place, but... I'm afraid they still might not be enough to defeat the demon."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Continental Guard Official",
                    args!["Please follow our instructions, at least around this area, and stay out of danger for now."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines_as(
            "Continental Guard Official",
            args!["Your application already has been registered. Please go speak to Chief Balrog."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn continental_official(ctx: &Ctx) -> Script {
    continental_official_body(ctx, Vec::new()).map(|_| ())
}

fn chief_balrog_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rebirth_moc_edq").get()? == 0 {
        ctx.lines_as("Chief Balrog", args!["You've come here at a bad time, but it's nice to meet you. I'm Continental Guard Chief Balrog. We've been dispatched to Morocc in order to suppress Satan Morocc... We'll need all the strength and luck we can gather."])?;
        ctx.next()?;
        ctx.lines_as("Chief Balrog", args!["I'm sorry, but I'm too busy checking through all these applications for future Continental Guards to greet you adventurers one by one."])?;
        ctx.next()?;
        ctx.lines_as("Chief Balrog", args!["Listen, it might be more helpful if you talk to some other people first. I'm sure one of the other Continental Guards or our messengers will be better equipped to help you out."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("rebirth_moc_edq").get()? == 1 {
            ctx.lines_as("Chief Balrog", args!["You've come here at a bad time, but it's nice to meet you. I'm Continental Guard Chief Balrog. We've been dispatched to Morocc in order to suppress Satan Morocc... We'll need all the strength and luck we can gather."])?;
            ctx.next()?;
            ctx.lines_as("Chief Balrog", args!["It looks like you have business with me. Please make it brief since I don't have a lot of time on my hands. There's lots of things I need to take care of..."])?;
            ctx.next()?;
            'b1: {
                let subject1 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Ask about Continental Guards:I want to join the Continental Guard.:End Conversation",
                    )],
                )?);
                let mut matched1 = false;
                let no_case1 = !subject1.loosely_equals(&Val::from(1))
                    && !subject1.loosely_equals(&Val::from(2))
                    && !subject1.loosely_equals(&Val::from(3));
                if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as("Chief Balrog", args!["The damage that Satan Morocc's resurrection has caused is obvious just by taking a look around this area.", "What's scary is that all this was caused just by breaking out from it's prison. Just think of the damage it could do if it was left to roam around freely..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chief Balrog",
                        args!["Morocc Village is completely devastated, and Satan Morocc has now claimed possession of the Sograt Desert."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Chief Balrog", args!["I've no doubt that if left unchecked, Satan Morocc will take over the entire Midgard Continent. We've got to stop him right now before the entire world suffers."])?;
                    ctx.next()?;
                    ctx.lines_as("Chief Balrog", args!["That's why the kingdom has ordered us elite soldiers to form the Continental Guard, and recruit reputable adventurers around the world so that we can make a united stand against Satan Morocc."])?;
                    ctx.next()?;
                    ctx.lines_as("Chief Balrog", args!["Of course, it'll be incredibly difficult... We'll need to make sacrifices... I'm not even sure if we can win. Still, the fate of the world is at stake, and we've got to do something. Dark times have truly fallen upon us..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Chief Balrog",
                        args!["Huh? Do you want to join us and volunteer for the Continental Guard?"],
                    )?;
                    ctx.next()?;
                    if ctx.var("BaseLevel").get()?.number()? > 79 {
                        ctx.lines_as(
                            "Chief Balrog",
                            args![
                                ((Val::from("What was your name? ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                    + Val::from("? Oh yes, I've heard of you."))
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["Hmm, it occurs to me that you don't understand the danger involved in all this. Do you have any idea how powerful Satan Morocc is?"])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["You won't be able to help us if you don't fully understand the risks. I've seen thousands of foolhardy adventurers throw their lives away by thinking they could defeat Satan Morocc with their own strength."])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["We are facing a threat that will determine the world's future, and fools that can't work in a team will be liabilities, not assets."])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["I'm sorry, but inexperienced adventurers would just get in our way. I hope you understand. If you really want to help us, then please focus on your training for now."])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("End Conversation:Ask Again")])? {
                            1 => {
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as("Chief Balrog", args!["Look, I understand your enthusiasm, but you're mistaken if you think you can help us. I don't have time to fully explain the danger that we're all facing."])?;
                                ctx.next()?;
                                ctx.lines_as("Chief Balrog", args!["Satan Morocc is nothing like the monsters you may have encountered. At best you'll throw your life away, but there's the chance that you might get one of my men killed by your mistakes and incompetence. I can't have that!"])?;
                                ctx.next()?;
                                ctx.lines_as("Chief Balrog", args!["Please understand that this is for your own good. Fight some Porings or whatever else might be a good match for your level."])?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("End Conversation:You've just got to let me join!")])? {
                                    1 => {
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.lines_as("Chief Balrog", args!["Sigh... Why are you so persistent? Can't you understand that no means no? Let me say this one more time."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Chief Balrog", args!["We're too busy fighting Satan Morocc to watch after rookies like you. You'd just be throwing your life away."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Chief Balrog", args!["Stop bothering me. You'd be nothing but a burden."])?;
                                        ctx.next()?;
                                        match runtime::select_values(ctx, &[Val::from("Give Up:Give me a chance to prove myself!")])? {
                                            1 => {
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                ctx.lines_as("Chief Balrog", args!["Huh? You want a chance to prove yourself?"])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args!["Yes, I'll do whatever it takes to join the Continental Guard!"],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Chief Balrog", args!["Hmm..."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Chief Balrog", args!["Interesting. You have my attention. Alright, I guess it's only fair that I acknowledge your strength if you can handle something for me."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Chief Balrog", args!["The Continental Guard has been on full alert around this village and the desert. We need to be prepared in case Satan Morocc storms our defenses."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Chief Balrog", args!["Soldiers need firewood to keep the bonfires burning all night, but we've been suffering a firewood shortage."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Chief Balrog", args!["I happened to hear that the monsters in the Lava Dungeon have everlasting flame which would solve our bonfire problems. I want you to gather those flames for us."])?;
                                                ctx.next()?;
                                                match runtime::select_values(
                                                    ctx,
                                                    &[Val::from("That's too hard! Let me think about it!:No problem.")],
                                                )? {
                                                    1 => {
                                                        ctx.lines_as("Chief Balrog", args!["Hmpf, I was right. I knew you wouldn't be able to handle such a simple task. I shouldn't have wasted my time with you."])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    2 => {
                                                        ctx.lines_as("Chief Balrog", args!["Excellent! Bring back ^0000FF30 Live Coals^000000. I'll be waiting for your return."])?;
                                                        ctx.var("rebirth_moc_edq").set(Val::from(2))?;
                                                        ctx.call(Function::ChangeQuest, vec![Val::from(3050), Val::from(3051)])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    } else {
                        ctx.lines_as(
                            "Chief Balrog",
                            args!["What? How can you even think of joining us when you don't have any real skills to offer?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["I guarantee that you'd just drag my soldiers down with you. You'd only be a threat to Satan Morocc in your dreams. Just give it up."])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["You won't be able to help us if you don't fully understand the risks. I've seen thousands of foolhardy adventurers throw their lives away by thinking they could defeat Satan Morocc with their own strength."])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["We are facing a threat that will determine the world's future, and fools that can't work in a team will be liabilities, not assets."])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["I'm sorry, but inexperienced adventurers would just get in our way. I hope you understand. If you really want to help us, then please focus on your training for now."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        } else {
            if ctx.var("rebirth_moc_edq").get()? == 2 {
                ctx.lines_as(
                    "Chief Balrog",
                    args!["So, did you find ^0000FF30 Live Coals^000000? You didn't come empty-handed, did you?"],
                )?;
                ctx.next()?;
                if ctx.call(Function::CountItem, vec![Val::from(7098)])?.number()? > 29 {
                    ctx.lines_as("Chief Balrog", args!["Let's see... One, two, three... Thirty, you've brought them all. Well, this was a pretty simple task. All it takes is time and a little effort. Anyone could do it."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chief Balrog",
                        args!["Anyways, thank you for bringing the Live Coals. I guess you're stronger than I thought."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Now will you let me join the Continental Guard?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chief Balrog",
                        args!["No, gathering these flames is a piece of cake compared to what we're going to do."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chief Balrog",
                        args!["I'm still not convinced that you're good enough to join us. Let me think... Hmm..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I didn't think you would accept me so easily. I'm ready. Ask me whatever you want."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Chief Balrog", args!["Ah, I've got an idea. Now, because this is an allied operation, we've accepted some mercenary soldiers from the Schwarzwald Republic to join the Continental Guard."])?;
                    ctx.next()?;
                    ctx.lines_as("Chief Balrog", args!["These mercenaries have never seen the desert, and they're having trouble carrying out operations in the heat, dry air, and sandstorms. It's not their fault, but they could use a little help since they're out of their element."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["(This guy seems threatening, but he seems to have a good heart.)"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Chief Balrog", args!["Now, I remember hearing that the monsters in the Ice Cave to the north have frozen hearts. Those hearts might be able to relieve those soldiers from the heat."])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("That's too hard!:No problem. How many do you want?")])? {
                        1 => {
                            ctx.lines_as("Chief Balrog", args!["I knew that'd be too tough for you. I'm glad you finally realized your limits before it was too late. It takes wisdom to recognize your weakness."])?;
                            ctx.next()?;
                            ctx.lines_as("Chief Balrog", args!["You'd better pack up and return where you came from. You want to be far away from here when Satan Morocc attacks."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as("Chief Balrog", args!["Huh?! Are you sure you can bring those? Hmm..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Chief Balrog",
                                args!["Well... If you insist on giving it a try, then... I'll need at least 50 of those frozen hearts."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Chief Balrog",
                                args!["Please bring me ^0000FF50 Glacial Hearts^000000. The faster you get them here, the better."],
                            )?;
                            ctx.call(Function::DelItem, vec![Val::from(7098), Val::from(30)])?;
                            ctx.var("rebirth_moc_edq").set(Val::from(3))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(3051), Val::from(3052)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.lines_as(
                        "Chief Balrog",
                        args!["Umm... Didn't you hear what I said? I said 30 Live Coals, 30."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chief Balrog",
                        args!["Now hurry up. If you feel like giving up, it's no problem. Let me know, though!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("rebirth_moc_edq").get()? == 3 {
                    ctx.lines_as("Chief Balrog", args!["So, did you find 50 Glacial Hearts?"])?;
                    ctx.next()?;
                    if ctx.call(Function::CountItem, vec![Val::from(7561)])?.number()? > 49 {
                        ctx.lines_as("Chief Balrog", args!["Thank you. I'm sure that my soldiers will appreciate these... Those guys aren't used to the desert, and could use the relief."])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["Well... It looks like you earned this."])?;
                        ctx.call(Function::DelItem, vec![Val::from(7561), Val::from(50)])?;
                        ctx.call(Function::GetItem, vec![Val::from(7826), Val::from(1)])?;
                        ctx.var("rebirth_moc_edq").set(Val::from(4))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(3052), Val::from(3053)])?;
                        ctx.next()?;
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["What's this?"])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["You'd know if you read it. It's a certificate that proves that you're a member of the Continental Guard. I admit that I'm impressed by your skills and gumption."])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["Welcome to the Continental Guard, my friend. Are you ready to risk your life for the sake of peace in the world?"])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["Your mission from here on will be simple, but extremely difficult: you will join the rest of the Continental Guard to keep Satan Morocc from fully reviving its true power."])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["My soldiers are risking their lives to fight Morocc's doppelgangers and underlings in order to make it safer for their comrades to travel to Morocc's lair."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Chief Balrog",
                            args!["It looks like your help could be useful after all. It's time for you to pitch in. Good luck."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Chief Balrog",
                            args!["Umm... Didn't you hear what I said? I said 50 Glacial Hearts, 50."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Chief Balrog",
                            args!["Now hurry up. If you feel like giving up, it's no problem. Let me know, though!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("rebirth_moc_edq").get()? == 4 {
                        ctx.lines_as(
                            "Chief Balrog",
                            args!["I guess you'd benefit from a full situational briefing. Shall I brief you now?"],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
                            1 => {
                                ctx.lines_as("Chief Balrog", args!["I'm going to tell you some basic information about fighting Satan Morocc as a member of the Continental Guard."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Chief Balrog",
                                    args![
                                        "Do you remember what Morocc was like before this happened? You would if you've been here before."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Chief Balrog", args!["Right now, all of Morocc's entrances and the outer fields leading to other villages are currently under our control."])?;
                                ctx.next()?;
                                ctx.lines_as("Chief Balrog", args!["The main reason for this is because Satan Morocc has been sighted in Sograt Desert, but there's another important reason."])?;
                                ctx.next()?;
                                ctx.lines_as("Chief Balrog", args!["Satan Morocc's power has been causing unnatural gaps in our time-space continuum. The gaps are isolated to this area for now, but we can't be sure if they'll spread."])?;
                                ctx.next()?;
                                ctx.lines_as("Chief Balrog", args!["Due to the time-space gaps, Sograt Desert's terrain has changed, and some parts of the desert have disappeared."])?;
                                ctx.next()?;
                                ctx.lines_as("Chief Balrog", args!["Furthermore, the situation's gotten worse since Satan Morocc's doppelgangers started appearing, and other monsters have become influenced by its power."])?;
                                ctx.next()?;
                                ctx.lines_as("Chief Balrog", args!["That's all we know so far. If you go out, you'll see what I mean... You may even see horrors that we haven't discovered yet. If you need more information, please speak to the Continental Official."])?;
                                ctx.next()?;
                                ctx.lines_as("Chief Balrog", args!["Please do your best to hold back Satan Morocc. Don't forget that the future of the continent is in our hands. Dismissed."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as("Chief Balrog", args!["I see. Keep up your good work in fighting Satan Morocc. Don't forget that the future of the continent is in our hands."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else if ctx.var("rebirth_moc_edq").get()? == 5 {
                        ctx.lines_as("Chief Balrog", args!["Wah... What? Did you really defeat Satan Morocc?"])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["How? Do you have any proof of your victory?"])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["...No, I can't just accept and trust your verbal account. I mean, if you've completed such an important mission, you must bring me some evidence."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Chief Balrog",
                            args!["Go back, bring proof of your victory, and then come back."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("rebirth_moc_edq").get()? == 6 {
                        ctx.lines_as("Chief Balrog", args!["What? Did you really defeat Satan Morocc?"])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["How? Do you have any proof of your victory?"])?;
                        ctx.next()?;
                        if ctx.call(Function::CountItem, vec![Val::from(7820)])?.number()? > 0 {
                            ctx.lines_as("Chief Balrog", args!["Is... Isn't this?"])?;
                            ctx.next()?;
                            ctx.lines_as("Chief Balrog", args!["Wow... I think this is really the skin of Satan Morocc. Congratulations, soldier. You just saved this world and people from being consumed by destruction and evil."])?;
                            ctx.next()?;
                            ctx.lines_as("Chief Balrog", args!["I guess you seriously wounded Satan Morocc, and it retreated to a time-space gap. No doubt it's trying to recoup its strength."])?;
                            ctx.next()?;
                            ctx.lines_as("Chief Balrog", args!["My only regret is that we can't pursue Morocc beyond this dimension. I'm glad, however, that you defeated Satan Morocc and kept it from regaining its full strength. For now, anyway..."])?;
                            ctx.next()?;
                            ctx.lines_as("Chief Balrog", args!["Our researchers might be able to learn some important new facts from this piece of skin... We need to learn all we can about that monster if peace is to be possible in our world's future."])?;
                            ctx.next()?;
                            ctx.lines_as("Chief Balrog", args!["Once again, I thank you for your distinguished service on behalf of the Continental Guard and the Rune-Midgarts Kingdom. I'll report your great achievement to His Majesty right away."])?;
                            ctx.next()?;
                            ctx.call(Function::DelItem, vec![Val::from(7820), Val::from(1)])?;
                            ctx.call(Function::GetExperience, vec![Val::from(2000000), Val::from(0)])?;
                            ctx.var("rebirth_moc_edq").set(Val::from(7))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(3055), Val::from(3056)])?;
                            ctx.lines_as("Chief Balrog", args!["I've prepared a few things to give as a reward for you. Let's see... I have three items. Which one do you like to receive?"])?;
                            ctx.next()?;
                            'b8: {
                                let subject8 = Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from("1. Seal of Continental Guard:2. Morocc Charm Stone:3. Morocc Ring")],
                                )?);
                                let mut matched8 = false;
                                let no_case8 = !subject8.loosely_equals(&Val::from(1))
                                    && !subject8.loosely_equals(&Val::from(2))
                                    && !subject8.loosely_equals(&Val::from(3));
                                if !matched8 && subject8.loosely_equals(&Val::from(1)) {
                                    matched8 = true;
                                }
                                if matched8 {
                                    ctx.lines_as("Chief Balrog", args!["The Seal of Continental Guard is an extremely valuable reward given directly from the kingdom court. It is a symbol of strength."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Chief Balrog",
                                        args!["The accessory's options are: ^0000FF MHP+50, +3% Attack Speed^000000."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Chief Balrog", args!["Do you really want the Seal of Continental Guard?"])?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("1. Yes.:2. No.")])? {
                                        1 => {
                                            ctx.lines_as(
                                                "Chief Balrog",
                                                args!["Great, then I'll reward you with the Seal of Continental Guard. Congratulations."],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::GetItem, vec![Val::from(2730), Val::from(1)])?;
                                            ctx.var("rebirth_moc_edq").set(Val::from(8))?;
                                            ctx.call(Function::CompleteQuest, vec![Val::from(3056)])?;
                                            ctx.lines_as("Chief Balrog", args!["I hope you keep in mind that our battle is far from over. Our enemy is the king of demons... I fear that Satan Morocc will return someday soon."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Chief Balrog", args!["Enjoy your victory, but be ever watchful and vigilant. I will always be here to help and reward you for your service."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Chief Balrog",
                                                args!["You should go rest now. Don't worry, we'll take care of everything else here."],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as(
                                                "Chief Balrog",
                                                args!["No problem. Take your time to think, and then speak to me again."],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                                if !matched8 && subject8.loosely_equals(&Val::from(2)) {
                                    matched8 = true;
                                }
                                if matched8 {
                                    ctx.lines_as("Chief Balrog", args!["The Morocc Charm Stone is an extremely valuable reward given directly from the kingdom court, and it is a symbol of prosperity and mana."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Chief Balrog",
                                        args!["The accessory's options are: ^0000FF MSP+50, -1% Casting Speed^000000."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Chief Balrog", args!["Do you really want a Morocc Charm Stone?"])?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("1. Yes.:2. No.")])? {
                                        1 => {
                                            ctx.lines_as(
                                                "Chief Balrog",
                                                args!["Great, then I'll reward you with a Morocc Charm Stone. Congratulations."],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::GetItem, vec![Val::from(2731), Val::from(1)])?;
                                            ctx.var("rebirth_moc_edq").set(Val::from(8))?;
                                            ctx.call(Function::CompleteQuest, vec![Val::from(3056)])?;
                                            ctx.lines_as("Chief Balrog", args!["I hope you keep in mind that our battle is far from over. Our enemy is the king of demons... I fear that Satan Morocc will return someday soon."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Chief Balrog", args!["Enjoy your victory, but be ever watchful and vigilant. I will always be here to help and reward you for your service."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Chief Balrog",
                                                args!["You should go rest now. Don't worry, we'll take care of everything else here."],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as(
                                                "Chief Balrog",
                                                args!["No problem. Take your time to think, and then speak to me again."],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                                if !matched8 && subject8.loosely_equals(&Val::from(3)) {
                                    matched8 = true;
                                }
                                if matched8 {
                                    ctx.lines_as("Chief Balrog", args!["The Morocc Ring is an extremely valuable reward given directly from the kingdom court that symbolizes critical power."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Chief Balrog", args!["The accessory's option is: ^0000FF CRI + 5^000000."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Chief Balrog", args!["Do you really want a Morocc Ring?"])?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("1. Yes.:2. No.")])? {
                                        1 => {
                                            ctx.lines_as(
                                                "Chief Balrog",
                                                args!["Great, then I'll reward you with a Morocc Ring. Congratulations."],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::GetItem, vec![Val::from(2732), Val::from(1)])?;
                                            ctx.var("rebirth_moc_edq").set(Val::from(8))?;
                                            ctx.call(Function::CompleteQuest, vec![Val::from(3056)])?;
                                            ctx.lines_as("Chief Balrog", args!["I hope you keep in mind that our battle is far from over. Our enemy is the king of demons... I fear that Satan Morocc will return someday soon."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Chief Balrog", args!["Enjoy your victory, but be ever watchful and vigilant. I will always be here to help and reward you for your service."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Chief Balrog",
                                                args!["You should go rest now. Don't worry, we'll take care of everything else here."],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as(
                                                "Chief Balrog",
                                                args!["No problem. Take your time to think about it, and then speak to me again."],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        } else {
                            ctx.lines_as("Chief Balrog", args!["...No, I can't just accept and trust your verbal account. I mean, if you've completed such an important mission, you must bring me some evidence."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Chief Balrog",
                                args!["Go back, bring proof of your victory, and then come back."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else if ctx.var("rebirth_moc_edq").get()? == 7 {
                        ctx.lines_as("Chief Balrog", args!["I've prepared a few things to give as a reward for you. Let's see... I have three items. Which one do you like to receive?"])?;
                        ctx.next()?;
                        'b12: {
                            let subject12 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("1. Seal of Continental Guard:2. Morocc Charm Stone:3. Morocc Ring")],
                            )?);
                            let mut matched12 = false;
                            let no_case12 = !subject12.loosely_equals(&Val::from(1))
                                && !subject12.loosely_equals(&Val::from(2))
                                && !subject12.loosely_equals(&Val::from(3));
                            if !matched12 && subject12.loosely_equals(&Val::from(1)) {
                                matched12 = true;
                            }
                            if matched12 {
                                ctx.lines_as("Chief Balrog", args!["The Seal of Continental Guard is an extremely valuable reward given directly from the kingdom court. It is a symbol of strength."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Chief Balrog",
                                    args!["The accessory's options are: ^0000FF MHP+50, +3% Attack Speed^000000."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Chief Balrog", args!["Do you really want the Seal of Continental Guard?"])?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("1. Yes.:2. No.")])? {
                                    1 => {
                                        ctx.lines_as(
                                            "Chief Balrog",
                                            args!["Great, then I'll reward you with the Seal of Continental Guard. Congratulations."],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::GetItem, vec![Val::from(2730), Val::from(1)])?;
                                        ctx.var("rebirth_moc_edq").set(Val::from(8))?;
                                        ctx.call(Function::CompleteQuest, vec![Val::from(3056)])?;
                                        ctx.lines_as("Chief Balrog", args!["I hope you keep in mind that our battle is far from over. Our enemy is the king of demons... I fear that Satan Morocc will return someday soon."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Chief Balrog", args!["Enjoy your victory, but be ever watchful and vigilant. I will always be here to help and reward you for your service."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Chief Balrog",
                                            args!["You should go rest now. Don't worry, we'll take care of everything else here."],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.lines_as(
                                            "Chief Balrog",
                                            args!["No problem. Take your time to think, and then speak to me again."],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            }
                            if !matched12 && subject12.loosely_equals(&Val::from(2)) {
                                matched12 = true;
                            }
                            if matched12 {
                                ctx.lines_as("Chief Balrog", args!["The Morocc Charm Stone is an extremely valuable reward given directly from the kingdom court, and it is a symbol of prosperity and mana."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Chief Balrog",
                                    args!["The accessory's options are: ^0000FF MSP+50, -1% Casting Speed^000000."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Chief Balrog", args!["Do you really want a Morocc Charm Stone?"])?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("1. Yes.:2. No.")])? {
                                    1 => {
                                        ctx.lines_as(
                                            "Chief Balrog",
                                            args!["Great, then I'll reward you with a Morocc Charm Stone. Congratulations."],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::GetItem, vec![Val::from(2731), Val::from(1)])?;
                                        ctx.var("rebirth_moc_edq").set(Val::from(8))?;
                                        ctx.call(Function::CompleteQuest, vec![Val::from(3056)])?;
                                        ctx.lines_as("Chief Balrog", args!["I hope you keep in mind that our battle is far from over. Our enemy is the king of demons... I fear that Satan Morocc will return someday soon."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Chief Balrog", args!["Enjoy your victory, but be ever watchful and vigilant. I will always be here to help and reward you for your service."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Chief Balrog",
                                            args!["You should go rest now. Don't worry, we'll take care of everything else here."],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.lines_as(
                                            "Chief Balrog",
                                            args!["No problem. Take your time to think, and then speak to me again."],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            }
                            if !matched12 && subject12.loosely_equals(&Val::from(3)) {
                                matched12 = true;
                            }
                            if matched12 {
                                ctx.lines_as("Chief Balrog", args!["The Morocc Ring is an extremely valuable reward given directly from the kingdom court that symbolizes critical power."])?;
                                ctx.next()?;
                                ctx.lines_as("Chief Balrog", args!["The accessory's option is: ^0000FF CRI + 5^000000."])?;
                                ctx.next()?;
                                ctx.lines_as("Chief Balrog", args!["Do you really want a Morocc Ring?"])?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("1. Yes.:2. No.")])? {
                                    1 => {
                                        ctx.lines_as(
                                            "Chief Balrog",
                                            args!["Great, then I'll reward you with a Morocc Ring. Congratulations."],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::GetItem, vec![Val::from(2732), Val::from(1)])?;
                                        ctx.var("rebirth_moc_edq").set(Val::from(8))?;
                                        ctx.call(Function::CompleteQuest, vec![Val::from(3056)])?;
                                        ctx.lines_as("Chief Balrog", args!["I hope you keep in mind that our battle is far from over. Our enemy is the king of demons... I fear that Satan Morocc will return someday soon."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Chief Balrog", args!["Enjoy your victory, but be ever watchful and vigilant. I will always be here to help and reward you for your service."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Chief Balrog",
                                            args!["You should go rest now. Don't worry, we'll take care of everything else here."],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.lines_as(
                                            "Chief Balrog",
                                            args!["No problem. Take your time to think about it, and then speak to me again."],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            }
                        }
                    } else if ctx.var("rebirth_moc_edq").get()? == 8 {
                        ctx.lines_as("Chief Balrog", args!["I've heard that the kingdom is planning to send out a large group of researchers to investigate the other world to which Satan Morocc has escaped."])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["That means it's time for both of us -- you and I -- to get to work. Who knows when Morocc will return to plague us?"])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["Please keep up the good work, and don't forget that the future of the continent and the kingdom relies on us."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Chief Balrog", args!["You've come here at a bad time, but it's nice to meet you. I'm Continental Guard Chief Balrog. We've been dispatched to Morocc in order to suppress Satan Morocc... We'll need all the strength and luck we can gather."])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["I'm sorry, but I'm too busy checking through all these applications for future Continental Guards to greet you adventurers one by one."])?;
                        ctx.next()?;
                        ctx.lines_as("Chief Balrog", args!["Listen, it might be more helpful if you talk to some other people first. I'm sure one of the other Continental Guards or our messengers will be better equipped to help you out."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn chief_balrog(ctx: &Ctx) -> Script {
    chief_balrog_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MoroccTimerEdqStep {
    Start,
    OnTouch,
}

fn morocc_timer_edq_run(ctx: &Ctx, mut step: MoroccTimerEdqStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MoroccTimerEdqStep::Start => {
                step = MoroccTimerEdqStep::OnTouch;
                continue 'machine;
            }
            MoroccTimerEdqStep::OnTouch => {
                if ctx.var("$@re_moc").get()? == 0 {
                    ctx.var("$@re_moc").set(Val::from(1))?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Satan Broadcast#edq::OnEnable")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn morocc_timer_edq(ctx: &Ctx) -> Script {
    morocc_timer_edq_run(ctx, MoroccTimerEdqStep::Start, Vec::new()).map(|_| ())
}

pub fn morocc_timer_edq_ontouch(ctx: &Ctx) -> Script {
    morocc_timer_edq_run(ctx, MoroccTimerEdqStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn satan_broadcast_edq(ctx: &Ctx) -> Script {
    satan_broadcast_edq_run(ctx, SatanBroadcastEdqStep::Start, Vec::new()).map(|_| ())
}

pub fn satan_broadcast_edq_oninit(ctx: &Ctx) -> Script {
    satan_broadcast_edq_run(ctx, SatanBroadcastEdqStep::OnInit, Vec::new()).map(|_| ())
}

pub fn satan_broadcast_edq_onenable(ctx: &Ctx) -> Script {
    satan_broadcast_edq_run(ctx, SatanBroadcastEdqStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn satan_broadcast_edq_ontimer5000(ctx: &Ctx) -> Script {
    satan_broadcast_edq_run(ctx, SatanBroadcastEdqStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn satan_broadcast_edq_ontimer15000(ctx: &Ctx) -> Script {
    satan_broadcast_edq_run(ctx, SatanBroadcastEdqStep::OnTimer15000, Vec::new()).map(|_| ())
}

pub fn satan_broadcast_edq_ontimer25000(ctx: &Ctx) -> Script {
    satan_broadcast_edq_run(ctx, SatanBroadcastEdqStep::OnTimer25000, Vec::new()).map(|_| ())
}

pub fn satan_broadcast_edq_ondisable(ctx: &Ctx) -> Script {
    satan_broadcast_edq_run(ctx, SatanBroadcastEdqStep::OnDisable, Vec::new()).map(|_| ())
}

fn group_of_evil_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("rebirth_moc_edq").get()? == 4 || ctx.var("rebirth_moc_edq").get()? == 7) && ctx.var("$@re_moc").get()? == 1) {
        ctx.mes("Awed by the time-space gap where darkness is given life, you instinctively step back.")?;
        ctx.next()?;
        ctx.mes("You can feel the power of the darkness rise from the gap where light and darkness are mingled.")?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Wah...!"])?;
        ctx.close_window()?;
        if ctx.var("$@re_moc").get()? == 1 {
            ctx.var("$@re_moc").set(Val::from(2))?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Satan Summon#edq::OnEnable")])?;
        }
        return Err(Stop::End);
    } else {
        ctx.mes("Awed by the time-space gap where darkness is given life, you instinctively step back.")?;
        ctx.next()?;
        ctx.mes("You can feel the power of the darkness rise from the gap where light and darkness are mingled.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn group_of_evil_edq(ctx: &Ctx) -> Script {
    group_of_evil_edq_body(ctx, Vec::new()).map(|_| ())
}

fn group_of_evil_edq_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Group of Evil#edq")])?;
    ctx.var("$@re_moc_time$").set(Val::from(""))?;
    return Err(Stop::End);
}

pub fn group_of_evil_edq_onenable(ctx: &Ctx) -> Script {
    group_of_evil_edq_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn time_space_gap_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("rebirth_moc_edq").get()? == 4 && ctx.var("$@re_moc").get()? == 3) {
        ctx.mes("The time-space gap's frightening darkness seems to dissipate as Satan Morocc fades away.")?;
        ctx.next()?;
        ctx.mes("You can see the fragments floating in the gap, and the radiating lights are now slowing their movement.")?;
        ctx.next()?;
        ctx.mes("You stretch your hand and pick up a lusterless fragment.")?;
        ctx.call(Function::GetItem, vec![Val::from(7820), Val::from(1)])?;
        ctx.var("rebirth_moc_edq").set(Val::from(6))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3053), Val::from(3055)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.mes("Slowly, erratically, the mysterious curtain of darkness casts over your eyes as you stare at the time-space gap.")?;
        ctx.next()?;
        ctx.mes("You feel like it's too dangerous to come any closer.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn time_space_gap_edq(ctx: &Ctx) -> Script {
    time_space_gap_edq_body(ctx, Vec::new()).map(|_| ())
}

fn time_space_gap_edq_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Time-Space Gap#edq")])?;
    return Err(Stop::End);
}

pub fn time_space_gap_edq_oninit(ctx: &Ctx) -> Script {
    time_space_gap_edq_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn time_space_gap_edq_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::EnableNpc, vec![Val::from("Time-Space Gap#edq")])?;
    return Err(Stop::End);
}

pub fn time_space_gap_edq_onenable(ctx: &Ctx) -> Script {
    time_space_gap_edq_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn time_space_gap_edq_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Time-Space Gap#edq")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Group of Evil#edq::OnEnable")])?;
    ctx.var("$@re_moc").set(Val::from(0))?;
    return Err(Stop::End);
}

pub fn time_space_gap_edq_ondisable(ctx: &Ctx) -> Script {
    time_space_gap_edq_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn time_space_gap_edq_ontimer1800000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$@re_moc").set(Val::from(4))?;
    return Err(Stop::End);
}

pub fn time_space_gap_edq_ontimer1800000(ctx: &Ctx) -> Script {
    time_space_gap_edq_ontimer1800000_body(ctx, Vec::new()).map(|_| ())
}

fn time_space_gap_edq_ontimer21600000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DoNpcEvent, vec![Val::from("Time-Space Gap#edq::OnDisable")])?;
    return Err(Stop::End);
}

pub fn time_space_gap_edq_ontimer21600000(ctx: &Ctx) -> Script {
    time_space_gap_edq_ontimer21600000_body(ctx, Vec::new()).map(|_| ())
}

pub fn satan_summon_edq(ctx: &Ctx) -> Script {
    satan_summon_edq_run(ctx, SatanSummonEdqStep::Start, Vec::new()).map(|_| ())
}

pub fn satan_summon_edq_oninit(ctx: &Ctx) -> Script {
    satan_summon_edq_run(ctx, SatanSummonEdqStep::OnInit, Vec::new()).map(|_| ())
}

pub fn satan_summon_edq_onenable(ctx: &Ctx) -> Script {
    satan_summon_edq_run(ctx, SatanSummonEdqStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn satan_summon_edq_ondisable(ctx: &Ctx) -> Script {
    satan_summon_edq_run(ctx, SatanSummonEdqStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn satan_summon_edq_onmymobdead(ctx: &Ctx) -> Script {
    satan_summon_edq_run(ctx, SatanSummonEdqStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn satan_summon_edq_ontimer5400000(ctx: &Ctx) -> Script {
    satan_summon_edq_run(ctx, SatanSummonEdqStep::OnTimer5400000, Vec::new()).map(|_| ())
}

pub fn satan_summon_edq_ontimer5415000(ctx: &Ctx) -> Script {
    satan_summon_edq_run(ctx, SatanSummonEdqStep::OnTimer5415000, Vec::new()).map(|_| ())
}

fn morocc_globalvar_admin_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.lines_as("Helper", args!["Please enter the password and # button."])?;
    ctx.next()?;
    if shared::other_gm_npcs::f_gm_npc(ctx, vec![Val::from(1854), Val::from(0)])?.number()? < 1 {
        ctx.lines_as("Helper", args!["Please press the numbers we always sing about."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Helper",
            args!["Hello. I am a post office.", "I am currently checking Morocc status."],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "=============",
            "Current State",
            "=============",
            "^3131FF<Morocc>^000000"
        ])?;
        if ctx.var("$@re_moc").get()? == 0 {
            ctx.lines(args![
                "Reset. Enable to summon Morocc.",
                "^3131FF<Continental Guards>^000000",
                "Enable to enter to moc_fild21 field."
            ])?;
        } else if ctx.var("$@re_moc").get()? == 1 {
            ctx.lines(args![
                "^3131FF<Morocc>^000000",
                "Some warrior entered after the reset. However, the warrior hasn't started the quest yet.",
                "The warrior who has rebirth_moc_edq 4 and 7 is now enabled to summon Morocc.",
                "^3131FF<Continental Guards>^000000",
                "The warrior who has rebirth_moc_edq 4 ~ 7 is now enabled to enter to moc_fild21."
            ])?;
        } else if ctx.var("$@re_moc").get()? == 2 {
            ctx.lines(args![
                "^3131FF<Morocc>^000000",
                "Morocc has been summoned. After 90 minutes has passed, it will revert back to reset status.",
                "^3131FF<Continental Guards>^000000",
                "The warrior who has rebirth_moc_edq 4 ~ 7 is now enabled to enter to moc_fild21."
            ])?;
        } else if ctx.var("$@re_moc").get()? == 3 {
            ctx.lines(args![
                "^3131FF<Morocc>^000000",
                ((Val::from("Morocc has been killed. Death time is ") + ctx.var("$@re_moc_time$").get()?)
                    + Val::from("(00 hr/00 min/00 sec).")),
                "After 6 hours later since the death time, it will be reset.",
                "The warriors whom has remained in the field can continue the quest via Time Space Gap.",
                "^3131FF<Continental Guards>^000000",
                "Disabled to enter to moc_fild21 from outside of the field."
            ])?;
        } else {
            ctx.lines(args!["^3131FF<Morocc>^000000", ((Val::from("Morocc has been killed. Death time is ") + ctx.var("$@re_moc_time$").get()?) + Val::from("(00 hr/00 min/00 sec).")), "After 6 hours later since the death time, it will be reset.", "It has been already passed 30 minutes after Morocc's death, so warriors can not continue the quest even if they click the Time Space Gap.", "^3131FF<Continental Guards>^000000", "Disabled to enter to moc_fild21 from outside of the field."])?;
        }
        ctx.next()?;
        ctx.lines_as("Helper", args!["What do you want?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Cancel.:Reset")])? {
            1 => {
                ctx.lines_as("Helper", args!["Alright."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                if ctx.call(Function::StrNpcInfo, vec![Val::from(4)])? == "sec_in02" {
                    ctx.mes("You can reset at moc_fild21 5 5.")?;
                } else {
                    ctx.mes("Reset starts.")?;
                    ctx.next()?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Group of Evil#edq::OnEnable")])?;
                    ctx.lines(args![
                        "Group of Evil NPC is appeared.",
                        "Time-Space Gap NPC is disappeared.",
                        "6 hours term timer is stopped."
                    ])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Satan Summon#edq::OnDisable")])?;
                    ctx.lines(args![
                        "Morocc 90 minutes survival timer is stopped.",
                        "Morocc is being killed.",
                        "Now, Continental Guards will let players enter."
                    ])?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn morocc_globalvar_admin(ctx: &Ctx) -> Script {
    morocc_globalvar_admin_body(ctx, Vec::new()).map(|_| ())
}
