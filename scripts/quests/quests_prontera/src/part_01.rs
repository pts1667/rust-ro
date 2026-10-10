use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum RecruiterStep {
    Start,
    SVolunteer,
}

fn recruiter_run(ctx: &Ctx, mut step: RecruiterStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RecruiterStep::Start => {
                if runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(8))?.is_true() {
                    ctx.lines_as(
                        "Recruiter",
                        args!["Ah...", "I know those eyes.", "Full of compassion", "...and courage."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Recruiter",
                        args![
                            "Of course you're a volunteer for campaign to reclaim the Prontera Culvert. Would you let me warp you there?"
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Warp to Culvert Entrance.:Quit.")])?) == 1 {
                        ctx.call(Function::Warp, vec![Val::from("prt_fild05"), Val::from(274), Val::from(208)])?;
                        return Err(Stop::End);
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Recruiter", args!["Ah, let me inform you that the Capital Defense Headquarter of the Rune-Midgarts Kingdom has now decided to recruit a punitive force due to the Prontera Culvert's situation."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Volunteer.:Situation...?:Quit.")])? {
                    1 => {
                        recruiter_run(ctx, RecruiterStep::SVolunteer, vec![])?;
                        ctx.lines_as(
                            "Recruiter",
                            args!["Are you ready, hero?", "I will now warp you", "to the Culvert."],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Warp, vec![Val::from("prt_fild05"), Val::from(274), Val::from(208)])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Recruiter",
                            args![
                                "Haven't you heard...?",
                                "The Prontera Culvert is infested with all kinds of filthy vermin!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Recruiter", args!["Due to the serious risk of water contamination, and the safety of Rune-Midgartsians, the Culvert has been quarantined by royal decree."])?;
                        ctx.next()?;
                        ctx.lines_as("Recruiter", args!["Needless to say, this has caused shortages in the water supply in our kingdom. Indeed, such a crisis in these dark times..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Recruiter",
                            args![
                                "Our people",
                                "thirst for water...",
                                "But they thirst",
                                "even more...",
                                "For a hero!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Recruiter", args!["Will you stand idly as the children of Rune-Midgarts suffer from parched throats and shed tears of dryness?! Would you bury our children?!"])?;
                        ctx.next()?;
                        ctx.lines_as("Recruiter", args!["Warriors! Rune-Midgarts is calling you! Cleanse this land's pestilence and cast out the vermin in the Culvert! Only you... Can make a difference."])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Volunteer.:Quit.")])?) == 1 {
                            recruiter_run(ctx, RecruiterStep::SVolunteer, vec![])?;
                            ctx.lines_as(
                                "Recruiter",
                                args!["Adventurer...", "I will now warp you", "to the Prontera Culvert."],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Warp, vec![Val::from("prt_fild05"), Val::from(274), Val::from(208)])?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Recruiter", args!["Hmpf. Well. The next time you take a drink of water, just remember that an average of nine and a half children just died... ^990000Of thirst^000000."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                step = RecruiterStep::SVolunteer;
                continue 'machine;
            }
            RecruiterStep::SVolunteer => {
                ctx.var("misc_quest")
                    .set(runtime::op(&ctx.var("misc_quest").get()?, "|", &Val::from(8))?)?;
                ctx.lines_as("Recruiter", args!["Your registration...", "is now complete."])?;
                ctx.next()?;
                ctx.lines_as("Recruiter", args!["I would like to thank you for volunteering to do your part for our great kingdom. Here, take these provisions for your battles against the forces of darkness."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Recruiter",
                    args!["3 Red Potions, 1 Milk,", "and 1 Orange Potion", "...to give you hope."],
                )?;
                ctx.call(Function::GetItem, vec![Val::from(501), Val::from(3)])?;
                ctx.call(Function::GetItem, vec![Val::from(519), Val::from(1)])?;
                ctx.call(Function::GetItem, vec![Val::from(502), Val::from(1)])?;
                ctx.next()?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn recruiter(ctx: &Ctx) -> Script {
    recruiter_run(ctx, RecruiterStep::Start, Vec::new()).map(|_| ())
}

fn culvert_guardian_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(8))?.is_true() {
        ctx.lines_as(
            "Culvert Guardian",
            args![
                "Ah, you're one of our volunteers. This is the entrance of the Prontera Culvert.",
                "Do you wish to",
                "go inside?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Sure.:Quit.")])?) == 1 {
            ctx.call(Function::Warp, vec![Val::from("prt_sewb1"), Val::from(131), Val::from(247)])?;
            return Err(Stop::End);
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Culvert Guardian",
        args!["I'm sorry, but we can only allow volunteers for the Culvert Campaign to enter."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Culvert Guardian",
        args![
            "If you'd like to volunteer, please visit the ^000077Culvert Registrar^000000 located in the 11 O'clock direction of Prontera."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn culvert_guardian(ctx: &Ctx) -> Script {
    culvert_guardian_body(ctx, Vec::new()).map(|_| ())
}

fn teacher_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (((ctx.call(Function::CountItem, vec![Val::from(710)])?.number()? > 0
        && ctx.call(Function::CountItem, vec![Val::from(703)])?.number()? > 0)
        && ctx.call(Function::CountItem, vec![Val::from(704)])?.number()? > 0)
        && ctx.call(Function::CountItem, vec![Val::from(708)])?.number()? > 0)
    {
        ctx.lines_as("Teacher", args!["Oh...", "Those Flowers in your hand are..."])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Show Flowers:Present Flowers:Cancel")])? {
            1 => {
                ctx.lines_as(
                    "Teacher",
                    args![
                        "Ah...",
                        "Those are definitely the 4 kinds of Flowers I was looking for! Would you give them to me?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Teacher", args!["If you would...", "I'll give you my precious item."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                if (((ctx.call(Function::CountItem, vec![Val::from(710)])?.number()? > 0
                    && ctx.call(Function::CountItem, vec![Val::from(703)])?.number()? > 0)
                    && ctx.call(Function::CountItem, vec![Val::from(704)])?.number()? > 0)
                    && ctx.call(Function::CountItem, vec![Val::from(708)])?.number()? > 0)
                {
                    ctx.call(Function::DelItem, vec![Val::from(710), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(703), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(704), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(708), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(5012), Val::from(1)])?;
                    ctx.lines_as("Teacher", args!["I really really appreciate you what you've done for me. I'm truly grateful. I will give my precious item to you as promised."])?;
                    ctx.next()?;
                    ctx.lines_as("Teacher", args!["This is the Hat I've worn on my Graduation from my University. It reminds me of my happy school days. Please take this..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Teacher",
                        args![
                            "... Oh, I'm Sorry.",
                            "But that's not what I'm looking for. Maybe you need to study flowers a little bit?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            _ => {}
        }
    } else {
        ctx.lines_as("Teacher", args!["Don't you think...", "Flowers are pretty?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Talk:Quit")])? {
            1 => {
                ctx.lines_as(
                    "Teacher",
                    args!["When I was young,", "I had no friends and", "studied all the time."],
                )?;
                ctx.next()?;
                ctx.lines_as("Teacher", args!["Sometimes I regret what I did when I was younger, but it's not a big deal now. Through hard study, I finished entire educational courses earlier than all the other students."])?;
                ctx.next()?;
                ctx.lines_as("Teacher", args!["However...", "I was sad and sometimes lonely. Whenever the studying got too hard, I needed someone who would listen to me. Finally, one day, I met my best friend."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Teacher",
                    args!["When I saw a Pretty Flower blooming in the abandoned Garden, I realized It was my friend which shared my fate."],
                )?;
                ctx.next()?;
                ctx.lines_as("Teacher", args!["Although it is a Common and Normal Flower to others, she gave me the reason to study again. I could achieve my goals because of her."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Teacher",
                    args![
                        "So recently...",
                        "I am trying to pay her back. Now I am studying Flowers, and plan to over the whole world with them."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Teacher", args!["To accomplish my work...", "I need bunches of flowers. But for someone who rarely goes outside, it is really hard to find all the flowers I need."])?;
                ctx.next()?;
                ctx.lines_as("Teacher", args!["I need 1 ^3355FFIllusion Flower^000000 ,1 ^3355FFHinalle^000000,1 ^3355FFAloe^000000 and 1 ^3355FFMent^000000. If you can bring them to me, I'd give you my treasure..."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Teacher",
                    args!["One of these days I will cover this whole world with Flowers."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn teacher(ctx: &Ctx) -> Script {
    teacher_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum BusyBoyPrtStep {
    Start,
    OnTouch,
    SGetBooks,
    SCheckWeight,
}

fn busy_boy_prt_run(ctx: &Ctx, mut step: BusyBoyPrtStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BusyBoyPrtStep::Start => {
                busy_boy_prt_run(ctx, BusyBoyPrtStep::SCheckWeight, vec![])?;
                if ctx.var("BaseLevel").get()?.number()? > 59 {
                    if ctx.var("prt_curse").get()? == 0 {
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args![
                                "Aw nuts...",
                                "What am I gonna do?",
                                "I have to deliver these",
                                "books, but... I... Oh man,",
                                "I can't get scared now!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("prt_curse").get()? == 1 {
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args![
                                "Aw nuts...",
                                "What am I gonna do?",
                                "I have to deliver these",
                                "books, but... I... Oh man,",
                                "I can't get scared now!"
                            ],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Speak with him:Ignore him")])?) == 1 {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Say, what seems", "to be the problem?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Busy-Looking Boy", args![".........?"])?;
                            ctx.next()?;
                            ctx.mes("[Busy-Looking Boy]")?;
                            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                ctx.lines(args!["Ack! You're that clumsy", "dude who made me drop"])?;
                            } else {
                                ctx.lines(args!["Ack! You're that ditzy", "chick who made me drop"])?;
                            }
                            ctx.lines(args![
                                "all of those books earlier!",
                                "Wait, you ditched me before,",
                                "so why act all concerned now?"
                            ])?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                        } else {
                            ctx.lines_as(
                                "Busy-Looking Boy",
                                args![
                                    "Okay, okay...",
                                    "Don't even think",
                                    "about floating in",
                                    "the sky. You're like,",
                                    "so stable. Don't think...",
                                    "Just... Just board that ship..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Well, I, um...")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.mes("[Busy-Looking Boy]")?;
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            ctx.lines(args![
                                "Yeah... A real",
                                "man wouldn't have",
                                "ditched back then",
                                "without helping me.",
                                "You're a real creepo,",
                                "you know that?"
                            ])?;
                        } else {
                            ctx.lines(args![
                                "Yeah... If you were",
                                "a graceful and considerate",
                                "lady, you woulda helped me",
                                "out before. I'm right, huh?"
                            ])?;
                        }
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFYou instinctively",
                            "kick over one of the",
                            "piles of books next",
                            "to the young boy.",
                            "You couldn't help it:",
                            "it was a natural reflex!^000000"
                        ])?;
                        ctx.next()?;
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HITDARK")?])?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                        ctx.mes("^3355FF*BAM!*^000000")?;
                        ctx.next()?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args!["Ack! Those books...!", "It took me so long to", "stack all of those!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Oh no, what a mess!",
                                "Here, let me help you",
                                "arrange these nicely",
                                "out of the bottom of",
                                "my freakin' heart."
                            ],
                        )?;
                        ctx.call(
                            Function::Emotion,
                            vec![
                                ctx.constant("ET_THROB")?,
                                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Busy-Looking Boy", args!["...", "......", "........."])?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args!["Huh...?", "Oh, thanks for", "helping me out here.", "I really appreciate it."],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Are these all yours?")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args![
                                "These? Oh, I'm supposed",
                                "to deliver these for my job.",
                                "I need to take these to Juno",
                                "from the Prontera Library",
                                "for a client. However, um..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args![
                                "Well...",
                                "I get motion sick really",
                                "easily, so it scares me to",
                                "death to ride the Airship",
                                "all the way to Juno."
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args![
                                "Now I'm in trouble!",
                                "I'm never late, but this",
                                "time I just can't help it.",
                                "There's nothing I can do!",
                                "Oh, I'm gonna lose this job!"
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args![
                                "Wait... You adventurers",
                                "do a lot of traveling, right?",
                                "If you're traveling to Juno,",
                                "would you please deliver this",
                                "for me? I'll be in real trouble",
                                "if I don't send these books..."
                            ],
                        )?;
                        ctx.next()?;
                        busy_boy_prt_run(ctx, BusyBoyPrtStep::SGetBooks, vec![])?;
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args![
                                "I-I understand.",
                                "It's none of your ",
                                "business, I know, but",
                                "I'm just so desperate..."
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                        ctx.var("prt_curse").set(Val::from(2))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("prt_curse").get()? == 2 {
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args![
                                "Oh, hello again.",
                                "Sorry, but I'm trying to",
                                "concentrate here. Gonna...",
                                "Summon all my courage...",
                                "and b-board that Airship!",
                                "Argh! No, I can't do it!"
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                        ctx.next()?;
                        busy_boy_prt_run(ctx, BusyBoyPrtStep::SGetBooks, vec![])?;
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args![
                                "^333333*Sob...*^000000",
                                "What am I gonna do?",
                                "That guy's been waiting",
                                "for me to deliver his books",
                                "for quite a while now..."
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("prt_curse").get()? == 3 {
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args![
                                "Oh, please deliver",
                                "those books I gave",
                                "you to ^FF0000Mr. Karlomoff^000000, who",
                                "should be waiting around",
                                "the Juno Library. Thanks",
                                "again for your help~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Busy-Looking Boy",
                        args![
                            "Ack! Would you leave",
                            "me alone and let me work?",
                            "I've got something important",
                            "to do! When I finish arranging",
                            "these books,  need to... I need to make some preparations!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Busy-Looking Boy",
                    args![
                        "Ack! Would you leave",
                        "me alone and let me work?",
                        "I've got something important",
                        "to do! When I finish arranging",
                        "these books, I need to... I need to make some preparations!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            BusyBoyPrtStep::OnTouch => {
                busy_boy_prt_run(ctx, BusyBoyPrtStep::SCheckWeight, vec![])?;
                if ctx.var("BaseLevel").get()?.number()? > 59 {
                    if ctx.var("prt_curse").get()? == 0 {
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HITDARK")?])?;
                        ctx.mes("^3355FF*BAM!*^000000")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Busy Looking Boy",
                            args!["Hey, look out!", "Can't you be more", "careful?! Geeeeez!"],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Oops, I'm so sorry.", "Are you alright?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Busy Looking Boy",
                            args!["Yeah, no thanks to you!", "Oh... Don't worry, I'm fine."],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFYou notice that the boy",
                            "dropped many hardcover",
                            "books that are probably about",
                            "ruins and their legends, based",
                            "on their titles. The boy dusted",
                            "himself off and began to",
                            "carefully pile the books.^000000"
                        ])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Leave:Help him pile the books")])?) == 1 {
                            ctx.lines_as(
                                "Busy-Looking Boy",
                                args![
                                    "Next time, look",
                                    "where you're going,",
                                    "alright? I mean, you",
                                    "might really break",
                                    "something if you're",
                                    "always that careless!"
                                ],
                            )?;
                            ctx.var("prt_curse").set(Val::from(1))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args!["Huh...?", "Oh, thanks for", "helping me out here.", "I really appreciate it."],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Are these all yours?")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args![
                                "These? Oh, I'm supposed",
                                "to deliver these for my job.",
                                "I need to take these to Juno",
                                "from the Prontera Library",
                                "for a client. However, um..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args![
                                "Well...",
                                "I get motion sick really",
                                "easily, so it scares me to",
                                "death to ride the Airship",
                                "all the way to Juno."
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args![
                                "Now I'm in trouble!",
                                "I'm never late, but this",
                                "time I just can't help it.",
                                "There's nothing I can do!",
                                "Oh, I'm gonna lose this job!"
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args![
                                "Wait... You adventurers",
                                "do a lot of traveling, right?",
                                "If you're traveling to Juno,",
                                "would you please deliver this",
                                "for me? I'll be in real trouble",
                                "if I don't send these books..."
                            ],
                        )?;
                        ctx.next()?;
                        busy_boy_prt_run(ctx, BusyBoyPrtStep::SGetBooks, vec![])?;
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args![
                                "I-I understand.",
                                "It's none of your ",
                                "business, I know, but",
                                "I'm just so desperate..."
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                        ctx.var("prt_curse").set(Val::from(2))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("prt_curse").get()? == 3 {
                        ctx.lines_as(
                            "Busy-Looking Boy",
                            args![
                                "Oh, please deliver",
                                "those books I gave",
                                "you to ^FF0000Mr. Karlomoff^000000, who",
                                "should be waiting around",
                                "the Juno Library. Thanks",
                                "again for your help~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                return Err(Stop::End);
            }
            BusyBoyPrtStep::SGetBooks => {
                if Val::from(runtime::select_values(ctx, &[Val::from("Help him:Don't help him")])?) == 1 {
                    ctx.lines_as(
                        "Busy-Looking Boy",
                        args![
                            "Oh, thank you so much!",
                            "You don't understand how",
                            "much I dread those Airships.",
                            "Now, would you please deliver",
                            "these books to ^FF0000Mr. Karlomoff^000000",
                            "near the Juno Library?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Busy-Looking Boy",
                        args![
                            "I know these books are",
                            "pretty heavy, but be really",
                            "careful with them! Anyway,",
                            "thanks for doing this for me.",
                            "I was really at my wit's end..."
                        ],
                    )?;
                    ctx.var("prt_curse").set(Val::from(3))?;
                    ctx.call(Function::GetItem, vec![Val::from(7431), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
            BusyBoyPrtStep::SCheckWeight => {
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
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn busy_boy_prt(ctx: &Ctx) -> Script {
    busy_boy_prt_run(ctx, BusyBoyPrtStep::Start, Vec::new()).map(|_| ())
}

pub fn busy_boy_prt_ontouch(ctx: &Ctx) -> Script {
    busy_boy_prt_run(ctx, BusyBoyPrtStep::OnTouch, Vec::new()).map(|_| ())
}

fn historian_prt01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
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
    if ctx.var("prt_curse").get()?.number()? < 3 {
        ctx.lines_as(
            "Historian",
            args![
                "Juno's mysterious past",
                "holds some great secret.",
                "I'm sure of it! It excites me",
                "to know that my research",
                "brings me that much closer",
                "to finally unveiling it!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Historian",
            args![
                "But I'll admit that even I don't know how much digging I'll",
                "have to do to learn what I want. It's our duty as historians to",
                "find out the truth of the past, but it definitely won't be easy."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Historian",
            args![
                "To understand, to see the",
                "truth of the past with my",
                "own eyes... I'd even sell",
                "my soul for the opportunity."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("prt_curse").get()? == 3 {
            ctx.lines(args![
                "^3355FFAs you approached, this",
                "historian suddenly closed",
                "the book that he was reading,",
                "looked to the heavens and",
                "let out a deep breath in a vain",
                "attempt to relieve his tension.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as("Historian", args!["GRRRRRRRRR!", "Where are my books?!"])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Are you alright?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Historian",
                args![
                    "Er? Oh. I'm sorry you",
                    "had to see that. I'm just",
                    "extremely upset. You see,",
                    "I'm expecting a delivery of",
                    "research books from Prontera,",
                    "but they haven't arrived yet."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "In fact, the delivery boy is",
                    "very late. This will delay my",
                    "research considerably since",
                    "I cannot proceed without more",
                    "new material to study."
                ],
            )?;
            ctx.next()?;
            if ctx.call(Function::CountItem, vec![Val::from(7431)])?.number()? > 0 {
                let choice = runtime::select_values(ctx, &[Val::from("Excuse me, but what's your name?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Historian",
                    args![
                        "Hm? I'm Karlomoff, the",
                        "1st scholar of the Rekenber",
                        "Historical Research Group.",
                        "Did you need something, or",
                        "were you looking around here",
                        "for somebody in particular?"
                    ],
                )?;
                ctx.next()?;
            } else {
                let choice = runtime::select_values(ctx, &[Val::from("Oh, I'm delivering the books for him.")])?;
                ctx.var("@menu").set(choice)?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_QUESTION")?])?;
                ctx.lines_as(
                    "Historian",
                    args!["Ah, really?", "Great, you're here!", "So, where are the books?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["I...", "I don't have", "them right now."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Historian",
                    args![
                        "What...?",
                        "Don't tell me that",
                        "you lost them! Those",
                        "books were priceless!",
                        "Did you come all this",
                        "way just to tell me that?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Historian",
                    args![
                        "Well, I suppose you'll",
                        "have to report the loss of",
                        "the books and get some new",
                        "copies of those books that",
                        "you were supposed to deliver.",
                        "Now hurry, I need to research!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            let choice = runtime::select_values(ctx, &[Val::from("Right, I brought your books.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Historian",
                args![
                    "What? But you're not the",
                    "delivery boy. Well, let me see",
                    "the books you've brought. Hm...",
                    "Just as I thought: the Rune-",
                    "Midgarts Kingdom has a great",
                    "wealth of ancient information."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "Ah, thank you so much for",
                    "bringing these. However, by",
                    "your mode of dress, I can tell",
                    "that you are an adventurer.",
                    "What happened to the delivery boy that was supposed to bring these?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "..........?",
                    "Afraid to ride the Airship?",
                    "Motion sickness? Well, that's",
                    "quite understandable. The poor",
                    "boy should have contacted me",
                    "about that beforehand."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "Goodness, it's been ",
                    "a while since I've had",
                    "a visitor. Would you like",
                    "to stay and chat for a bit?",
                    "Just give me a minute to get",
                    "a cool drink of water first~"
                ],
            )?;
            ctx.var("prt_curse").set(Val::from(4))?;
            ctx.call(Function::DelItem, vec![Val::from(7431), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("prt_curse").get()? == 4 {
            ctx.lines_as(
                "Historian",
                args![
                    "Ah, that glass of",
                    "water was just what",
                    "I needed to refresh",
                    "myself. Let me tell you",
                    "a little bit about my work."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "I might have it mentioned before, but my name is Karlomoff and",
                    "I work in the Rekenber Historical Research as its 1st scholar. We",
                    "recently finished our project on the Schwarzwald Republic."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "Now, we're trying to learn",
                    "more about the Rune-Midgarts",
                    "Kingdom's history. We believe",
                    "it's linked to our Schwarzwald",
                    "Republic because they share the",
                    "continent. Makes sense, right?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "Ultimately, we hope that",
                    "new information from our",
                    "research of Rune-Midgarts will",
                    "shed some new light on our",
                    "current understanding of the",
                    "Schwarzwald Republic's past."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "Including myself, there are",
                    "^3131FF3 members of the Rekenber",
                    "Historical Research Group^000000",
                    "that are studying the Rune-",
                    "Midgarts Kingdom's history."
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Rekenber Historical Research Group?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Historian",
                args![
                    "You adventurers from Rune-",
                    "Midgarts may not know it, but",
                    "the Rekenber Corporation has",
                    "unofficial control over our",
                    "Schwarzwald Republic. Some",
                    "hate it, others don't care."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "Anyway, Rekenber has its own",
                    "Historical Research Group since",
                    "rediscovering ancient technologies has been key to its success, well,",
                    "so far as I can tell. Personally, I enjoy the pursuit of knowledge."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "Oh, here's an interesting",
                    "fact! Did you know that the",
                    "title of ruler of Rune-Midgarts",
                    "isn't always passed down",
                    "through the same family?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "Actually, there are several",
                    "royal families that hold a",
                    "special competition to decide",
                    "which prince becomes the",
                    "next king. Fascinating..."
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Several royal families?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Historian",
                args![
                    "Yes, I'll try to explain it",
                    "briefly. There are a total",
                    "of 7 royal families. Each",
                    "family is descended from one",
                    "of the 7 warriors that founded",
                    "the Rune-Midgarts Kingdom."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "According to the records,",
                    "Jormungand, the snake of the",
                    "earth, appeared and brought",
                    "chaos to the entire continent.",
                    "7 warriors appeared and drove the serpent away, saving the world."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "With the return of peace,",
                    "the 7 warriors established",
                    "the Rune-Midgarts Kingdom,",
                    "choosing Tristram Geoborg III",
                    "as the kingdom's first ruler. "
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "Knowing that their descendents",
                    "may not always be deserving of",
                    "ruling the kingdom, the 7 warriors agreed to hold a contest amongst",
                    "their families each generation to prevent royal corruption."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
            ctx.lines_as(
                "Historian",
                args![
                    "Oh... Not too excited",
                    "about history, huh? Well,",
                    "maybe if I sing the ancient",
                    "song of this myth, you'd be",
                    "better able to understand..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "^FF0000*The great serpent*^000000",
                    "^FF0000*swallowed the sea.*^000000",
                    "^FF0000*The eagle of the rainbow*^000000",
                    "^FF0000*swallowed the serpent.*^000000",
                    "^FF0000*Then the eagle built its nest.*^000000",
                    "^FF0000*A nest upon the swallowed sea.*^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "^333333*Ahem*^000000 As you see, I'm",
                    "quite tone deaf. But the",
                    "point is that people still",
                    "praise the 7 warriors' exploits",
                    "through this song. Isn't that",
                    "interesting to know about?"
                ],
            )?;
            ctx.var("prt_curse").set(Val::from(5))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("prt_curse").get()? == 5 {
            ctx.lines_as(
                "Historian",
                args![
                    "I'm sorry that I let my",
                    "mouth run while talking at",
                    "great length about Rune-",
                    "Midgart's history. Still,",
                    "I hope you found that tale",
                    "at least a little enjoyable."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "Oh! Will you be returning to",
                    "the Rune-Midgarts Kingdom?",
                    "If so, then I have a favor to",
                    "ask you. Would you please",
                    "deliver this report I've written to my colleague in Morocc?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "It would be a great help",
                    "to me if you could get this",
                    "report to her right away.",
                    "Ah, her name is Rodafrian.",
                    "I'm certain you can find her",
                    "somewhere in that desert town."
                ],
            )?;
            ctx.var("prt_curse").set(Val::from(6))?;
            ctx.call(Function::GetItem, vec![Val::from(7342), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("prt_curse").get()? == 6 {
            ctx.lines_as(
                "Historian",
                args![
                    "Please find my colleague,",
                    "Rodafrian, in Morocc and",
                    "deliver my report to her.",
                    "You should be able to find",
                    "her there doing research."
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("I will:Can I listen to that song again?")],
            )?) == 1
            {
                ctx.lines_as(
                    "Historian",
                    args![
                        "Once again, thank",
                        "you for your help.",
                        "It will really speed the",
                        "progress of my research,",
                        "especially since those books",
                        "were delivered fairly late..."
                    ],
                )?;
            } else {
                ctx.lines_as(
                    "Historian",
                    args![
                        "Song? Oh, you mean the",
                        "one praising the 7 who",
                        "founded the Rune-Midgarts",
                        "Kingdom? Sure, let's see now..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Historian",
                    args![
                        "^FF0000*The great serpent*^000000",
                        "^FF0000*swallowed the sea.*^000000",
                        "^FF0000*The eagle of the rainbow*^000000",
                        "^FF0000*swallowed the serpent.*^000000",
                        "^FF0000*Then the eagle built its nest.*^000000",
                        "^FF0000*A nest upon the swallowed sea.*^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Historian",
                    args![
                        "You must be more interested",
                        "in history than I suspected.",
                        "If you'd like, I'll write you a",
                        "letter or recommendation",
                        "for the Rekenber Historical",
                        "Research Group. Ha ha ha~"
                    ],
                )?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("prt_curse").get()?.number()? > 55 {
            ctx.lines_as(
                "Historian",
                args![
                    "Ah, it's been a while",
                    "since the last time I saw",
                    "you. Rodafrian actually came",
                    "to visit me a few days ago.",
                    "I believe she came here",
                    "to gloat or threaten me..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Historian",
                args![
                    "It was very surreal.",
                    "She kept raving about",
                    "some incredible revelation,",
                    "and about finally putting me",
                    "in my place. I didn't know she",
                    "could be so competitive!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Historian", args!["ZzzzZZz....", "ZzzzZZz....ZZZzzzz..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn historian_prt01(ctx: &Ctx) -> Script {
    historian_prt01_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum HistorianPrt02Step {
    Start,
    SGiveName,
}

fn historian_prt02_run(ctx: &Ctx, mut step: HistorianPrt02Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_answer_s = Val::from("");
    let mut l_line_s = Val::from("");
    let mut l_total = Val::from(0);
    'machine: loop {
        match step {
            HistorianPrt02Step::Start => {
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
                if ctx.var("prt_curse").get()? == 6 {
                    if ctx.call(Function::CountItem, vec![Val::from(7342)])?.number()? < 0 {
                        ctx.lines(args![
                            "^3355FFYou seem to have lost",
                            "Karlomoff's Report. You",
                            "needed to deliver here to",
                            "one of his colleagues..."
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Historian Rodafrian",
                        args![
                            "Oh, how does this place",
                            "have to be so hot? This",
                            "can't be good for my skin...",
                            "Ooh, I wish I were back home",
                            "in the Schwarzwald Republic~"
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Rodafrian",
                        args!["Hello there.", "E-excuse me, but,", "um, may I help you?"],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Mr. Karlomoff has sent me.")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Historian Rodafrian",
                        args!["Karlomoff...?", "Him? Alright...", "What did he send", "you to me for?"],
                    )?;
                    ctx.next()?;
                    ctx.lines(args!["^3355FFYou hand Karlomoff's", "report to Rodafrian.^000000"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Historian Rodafrian",
                        args![
                            "A report? What does he",
                            "want to show off this time?",
                            "^333333*Sigh*^000000 Ah well, thanks for",
                            "the trouble. I'll look at it,",
                            "though I'm not expecting",
                            "much if Karlomoff wrote it."
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Ask her about her research:End conversation")],
                    )?) == 1
                    {
                        ctx.lines_as(
                            "Historian Rodafrian",
                            args![
                                "Oh, you know about the",
                                "Rekenber Historical Research",
                                "Group? Ah, right. Karlomoff",
                                "must have told you already.",
                                "Currently we're researching",
                                "the Rune-Midgarts Kingdom."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Historian Rodafrian",
                            args![
                                "Now, while I understand the",
                                "value of ancient books and",
                                "records, I think Karlomoff",
                                "relies on them too much.",
                                "I prefer more active research",
                                "in the vein of archaelogy."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Historian Rodafrian",
                            args![
                                "Right now, I'm spending",
                                "time in Morocc to visit the",
                                "Sphinx and Pyramids and see",
                                "if I can excavage some relics.",
                                "Hopefully I can uncover some new historical evidence that way."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Historian Rodafrian",
                            args![
                                "If you're interested in",
                                "learning more about Morocc's",
                                "history, please go ahead and",
                                "talk to my assistance. He's very passionate about studying this",
                                "town's culture and background."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Historian Rodafrian",
                            args![
                                "Actually, the people of",
                                "Morocc also appreciate their",
                                "town's history, and pass down",
                                "songs about ancient times",
                                "through the generations. Let's",
                                "see, how did that one go?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Historian Rodafrian",
                            args![
                                "^FF0000*The great serpent*^000000",
                                "^FF0000*swallowed the sea.*^000000",
                                "^FF0000*The eagle of the rainbow*^000000",
                                "^FF0000*swallowed the serpent, and*^000000",
                                "and... and... Um. Oh dear."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Historian Rodafrian",
                            args!["I can't remember the next", "line! Actually, have you", "heard this song before?"],
                        )?;
                        ctx.next()?;
                        'b1: {
                            let subject1 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Yes, I have.:Yes, but it is different.:No, sorry.")],
                            )?);
                            let mut matched1 = false;
                            let no_case1 = !subject1.loosely_equals(&Val::from(1))
                                && !subject1.loosely_equals(&Val::from(2))
                                && !subject1.loosely_equals(&Val::from(3));
                            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                                matched1 = true;
                            }
                            if matched1 {
                                ctx.lines_as(
                                    "Historian Rodafrian",
                                    args![
                                        "Oh, that's great!",
                                        "Would you provide",
                                        "me with the line that",
                                        "follows these lyrics?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^FF0000*The great serpent*^000000",
                                    "^FF0000*swallowed the sea.*^000000",
                                    "^FF0000*The eagle of the rainbow*^000000",
                                    "^FF0000*swallowed the serpent.*^000000",
                                    "......................"
                                ])?;
                                ctx.next()?;
                                let (input, status) = runtime::input_text(ctx, None, None)?;
                                l_answer_s = input;
                                if l_answer_s.clone() == "Then the eagle built its nest." {
                                    ctx.lines_as(
                                        "Historian Rodafrian",
                                        args![
                                            "Really? That makes",
                                            "sense, but those lyrics",
                                            "sound different than the",
                                            "ones I had heard. Hmmmm...",
                                            "If you don't mind, would you",
                                            "please tell me your name?"
                                        ],
                                    )?;
                                    historian_prt02_run(ctx, HistorianPrt02Step::SGiveName, vec![Val::from(1)])?;
                                } else {
                                    ctx.lines_as(
                                        "Historian Rodafrian",
                                        args![
                                            "Huh...?",
                                            "That doesn't sound",
                                            "right at all. Are you sure",
                                            "that's the lyric you heard?"
                                        ],
                                    )?;
                                    ctx.call(Function::DelItem, vec![Val::from(7342), Val::from(1)])?;
                                    ctx.var("prt_curse").set(Val::from(7))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                                matched1 = true;
                            }
                            if matched1 {
                                ctx.lines_as(
                                    "Historian Rodafrian",
                                    args![
                                        "What was that?",
                                        "The song is different?",
                                        "Hmm. Then, would you",
                                        "please sing the version",
                                        "that you know to me?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFYou clear your throat",
                                    "and begin to sing the song",
                                    "that you heard from Karlomoff.^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
                                ])?;
                                let (input, status) = runtime::input_text(ctx, None, None)?;
                                l_line_s = input;
                                if l_line_s.clone() == "The great serpent swallowed the sea." {
                                    l_total = (l_total.clone() + Val::from(1));
                                    ctx.lines(args!["*The great serpent*", "*swallowed the sea.*"])?;
                                } else {
                                    ctx.lines(args![((Val::from("* ") + l_line_s.clone()) + Val::from("*"))])?;
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
                                    ctx.mes("*Then the eagle built its nest.*")?;
                                } else {
                                    ctx.lines(args![((Val::from("*") + l_line_s.clone()) + Val::from("*"))])?;
                                }
                                let (input, status) = runtime::input_text(ctx, None, None)?;
                                l_line_s = input;
                                if l_line_s.clone() == "A nest upon the swallowed sea." {
                                    l_total = (l_total.clone() + Val::from(1));
                                    ctx.mes("*A nest upon the swallowed sea.*")?;
                                } else {
                                    ctx.lines(args![((Val::from("*") + l_line_s.clone()) + Val::from("*"))])?;
                                }
                                ctx.next()?;
                                if l_total.clone() == 4 {
                                    ctx.lines_as(
                                        "Historian Rodafrian",
                                        args![
                                            "Really? That makes",
                                            "sense, but those lyrics",
                                            "sound different than the",
                                            "ones I had heard. Hmmmm...",
                                            "If you don't mind, would you",
                                            "please tell me your name?"
                                        ],
                                    )?;
                                    historian_prt02_run(ctx, HistorianPrt02Step::SGiveName, vec![Val::from(1)])?;
                                } else {
                                    ctx.lines_as(
                                        "Historian Rodafrian",
                                        args![
                                            "Huh...?",
                                            "That doesn't sound right",
                                            "at all. Are you sure those",
                                            "are the lyrics you heard?"
                                        ],
                                    )?;
                                    ctx.call(Function::DelItem, vec![Val::from(7342), Val::from(1)])?;
                                    ctx.var("prt_curse").set(Val::from(7))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                                matched1 = true;
                            }
                            if matched1 {
                                ctx.lines_as(
                                    "Historian Rodafrian",
                                    args![
                                        "............",
                                        "........................",
                                        "Well then. Would you",
                                        "mind if I ask you for",
                                        "your name, adventurer?"
                                    ],
                                )?;
                                historian_prt02_run(ctx, HistorianPrt02Step::SGiveName, vec![Val::from(1)])?;
                            }
                        }
                    }
                    ctx.lines_as(
                        "Historian",
                        args![
                            "Although I love doing my",
                            "research and uncovering",
                            "new information regarding",
                            "history, I feel a little lonely",
                            "sometimes. Not many people",
                            "share my interests, you know..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("prt_curse").get()? == 7 {
                        ctx.lines(args![
                            "^3355FFArms folded, Rodafrian",
                            "seems to be deep in thought.",
                            "Then, as if making making",
                            "an important decision, she",
                            "looks directly into your eyes",
                            "and begins to speak.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as("Historian Rodafrian", args!["Adventurer...", "I want to know your name."])?;
                        historian_prt02_run(ctx, HistorianPrt02Step::SGiveName, vec![Val::from(0)])?;
                    } else {
                        if (ctx.var("prt_curse").get()?.number()? > 7 && ctx.var("prt_curse").get()?.number()? < 30) {
                            ctx.lines_as(
                                "Historian Rodafrian",
                                args![
                                    "As I recall, Mondo should",
                                    "be at Mount Mjolnir, located",
                                    "in the northern region of the",
                                    "Rune-Midgarts Kingdom. Ask",
                                    "him for the lyrics of the song that I can't seem to remember..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (ctx.var("prt_curse").get()? == 30 || ctx.var("prt_curse").get()? == 55) {
                            ctx.lines_as(
                                "Historian Rodafrian",
                                args![
                                    "Oh, you've returned.",
                                    "Have you met with Mondo",
                                    "and figured out the lyrics of",
                                    "that song I was looking for?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
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
                            ctx.lines_as(
                                "Historian Rodafrian",
                                args![
                                    "Oh, yes!",
                                    "Yes, that was it! Now",
                                    "I remember, thank you",
                                    "so much! Ah, back to work..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "^333333(As a historian, Rodafrian",
                                    "might be able to help me in",
                                    "investigating the curse of the",
                                    "Geoborgs. The priests told me",
                                    "not to tell anybody, though.",
                                    "Should I take this risk?)^000000"
                                ],
                            )?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Tell her about Jormungand's curse:Don't tell her")],
                            )?) == 1
                            {
                                ctx.lines(args![
                                    "^3355FFYou explain everything",
                                    "that you have learned to",
                                    "Rodafrian, choosing not",
                                    "to withhold any secrets.^000000"
                                ])?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
                                ctx.next()?;
                                ctx.lines_as("Historian Rodafrian", args![".....................!"])?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Historian Rodafrian",
                                    args![
                                        "Thank you for sharing that",
                                        "with me. I hope you realize",
                                        "how precious that information",
                                        "is. I had no idea the royal",
                                        "family was keeping that kind",
                                        "of secret. Goodness, me..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Historian Rodafrian",
                                    args![
                                        "You know, I don't really",
                                        "know much about poison, but",
                                        "I do know that, aside from our",
                                        "own Assassins, there are poison",
                                        "experts living in some strange",
                                        "land located to the west."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Historian Rodafrian",
                                    args![
                                        "Anyway, your report about",
                                        "the Geoborg family will be",
                                        "greatly appreciated by the",
                                        "Rekenber Historical Research",
                                        "Group. But first, I need to",
                                        "finish this Morocc project..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Historian Rodafrian",
                                    args![
                                        "Anyway, keep this information",
                                        "a secret between me and you",
                                        "for now. Then, when I reveal the secret curse of the Geoborg royal",
                                        "family, I'll finally outshine that Karlomoff! Bwahahahahaha!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "^333333(Drat, she didn't give me",
                                        "any help... All I did was",
                                        "reveal a huge secret to her",
                                        "that she might spread around!",
                                        "Oh well, I guess I better head",
                                        "back to the priests...)^000000"
                                    ],
                                )?;
                                if ctx.var("prt_curse").get()? == 30 {
                                    ctx.var("prt_curse").set(Val::from(31))?;
                                } else {
                                    ctx.var("prt_curse").set(Val::from(60))?;
                                }
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Historian Rodafrian",
                                args![
                                    "Oh, let me thank you",
                                    "once again for going",
                                    "through the trouble of",
                                    "getting that lyric for me~",
                                    "I put my assistant through",
                                    "enough trouble already..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Historian Rodafrian",
                                args![
                                    "I also better not forget",
                                    "to go through Karlomoff's",
                                    "report. I'll have to have a",
                                    "debate with him sooner or",
                                    "later, and I really want to",
                                    "put that guy in his place!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^3355FFRodafrian seemed very",
                                "happy and began to read",
                                "through Karlomoff's report.",
                                "For now, it would be best",
                                "to return to Father Bamph.^000000"
                            ])?;
                            if ctx.var("prt_curse").get()? == 30 {
                                ctx.var("prt_curse").set(Val::from(40))?;
                            } else {
                                ctx.var("prt_curse").set(Val::from(56))?;
                            }
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (ctx.var("prt_curse").get()? == 31 || ctx.var("prt_curse").get()? == 40) {
                            ctx.lines(args![
                                "^3355FFRodafrian seemed very",
                                "happy and began to read",
                                "through Karlomoff's report.",
                                "For now, it would be best",
                                "to return to Father Bamph.^000000"
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("prt_curse").get()? == 56 {
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                            ctx.lines_as(
                                "Historian Rodafrian",
                                args![
                                    "I just paid a visit to",
                                    "Karlomoff and gave him",
                                    "a piece of my mind! I think...",
                                    "I think I put him in his place.",
                                    "But I can never really tell",
                                    "with that sneaky guy..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("prt_curse").get()? == 60 {
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                            ctx.lines_as(
                                "Historian Rodafrian",
                                args![
                                    "I just aid a visit to",
                                    "Karlomoff and gave him",
                                    "a piece of my mind! I think...",
                                    "I think I put him in his place.",
                                    "But I can never really tell",
                                    "with that sneaky guy..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines(args![
                                "It doesn't look like",
                                "Rodafrian can offer you",
                                "any more information. For",
                                "now, it would be best to",
                                "go to ^3355FFProntera Church.^000000"
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Historian Rodafrian",
                                args![
                                    "I'm sorry, but I'm",
                                    "really very busy with my",
                                    "research at the moment.",
                                    "Perhaps we can talk later",
                                    "once I've completed this?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
                step = HistorianPrt02Step::SGiveName;
                continue 'machine;
            }
            HistorianPrt02Step::SGiveName => {
                ctx.next()?;
                let choice = runtime::select_values(
                    ctx,
                    &[((Val::from("My name is ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))],
                )?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Historian Rodafrian",
                    args![
                        ((Val::from("Ah ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", eh?")),
                        "Do you mind if I ask you a favor? I really need to verify the true",
                        "lyrics for this song. However,",
                        "I need to stay here in Morocc",
                        "to complete my research."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Historian Rodafrian",
                    args![
                        "Would you please find my",
                        "colleague Mondo and ask him",
                        "for this song's lyrics? You can",
                        "find him in the ancient ruins",
                        "in Mount Mjolnir, north in the",
                        "Rune-Midgarts Kingdom."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Historian Rodafrian",
                    args![
                        "I apologize for asking",
                        "you to do what sounds like",
                        "a silly errand, but I actually",
                        "need to know this song's lyrics",
                        "for the sake of my research..."
                    ],
                )?;
                if runtime::arg(&args, 0, Val::from(0)) == 1 {
                    ctx.call(Function::DelItem, vec![Val::from(7342), Val::from(1)])?;
                }
                ctx.var("prt_curse").set(Val::from(8))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn historian_prt02(ctx: &Ctx) -> Script {
    historian_prt02_run(ctx, HistorianPrt02Step::Start, Vec::new()).map(|_| ())
}
