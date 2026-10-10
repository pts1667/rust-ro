use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn diego_ep13bs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_newbs").get()?.number()? < 5 {
        ctx.lines_as("Diego", args!["I'm busy right now!"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_newbs").get()? == 5 {
        ctx.lines_as(
            "Diego",
            args!["Wow, this is tent is big!", "Hey, you there, adventurer. Please help me."],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Help him.:Don't help.")])? {
            1 => {
                ctx.lines_as("Diego", args!["Thanks."])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_THANKS")?])?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["What do you need?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Diego",
                    args![
                        "Ah...",
                        "When I passed by here...",
                        "I kicked the post by mistake",
                        "and now it's broken."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Diego", args!["I need to ask others for help", "while I hold this post up."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Diego",
                    args![
                        "Well, let me see...",
                        "I'm really lucky to see you,",
                        ((Val::from("passing by here, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Hah!! How did you know my name?!"],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_HUK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Diego", args!["It's on your nameplate."])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("So how can I help you.")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as("Diego", args!["While I hold this post up,", "please get materials to fix it."])?;
                ctx.next()?;
                ctx.lines_as("Diego", args!["I need the bar to sustain this", "post and a rope to tie it."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Diego",
                    args![
                        "You can find many trees",
                        "west of the camp for branches",
                        "that we can use as a post."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Diego",
                    args![
                        "And you can find the weird",
                        "plant called ^0000FFNepenthes^000000",
                        "at the field east of the camp it's vines are very strong."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Diego",
                    args![
                        "I think we should be",
                        "able to fix this with",
                        "^0000FF20 Ordinary Branches and 20 Strong Vine^000000.",
                        "Please bring them to me!!!"
                    ],
                )?;
                ctx.var("ep13_newbs").set(Val::from(6))?;
                ctx.call(Function::SetQuest, vec![Val::from(11087)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Diego", args!["...", "You're too harsh.", "Sob..."])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("ep13_newbs").get()? == 6 {
        if (ctx.call(Function::CountItem, vec![Val::from(6041)])?.number()? > 19
            && ctx.call(Function::CountItem, vec![Val::from(6042)])?.number()? > 19)
        {
            ctx.lines_as(
                "Diego",
                args!["Oh!!", "You finally brought", "the materials!", "I'm so grateful!!"],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
            ctx.next()?;
            ctx.lines_as(
                "Diego",
                args![
                    "Ok, so while I hold this post,",
                    "please attach the branches",
                    "and tie it up with the vines."
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(6041), Val::from(20)])?;
            ctx.call(Function::DelItem, vec![Val::from(6042), Val::from(20)])?;
            ctx.var("ep13_newbs").set(Val::from(7))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(11087), Val::from(11088)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Diego",
                args![
                    "I think we should be",
                    "able to fix this with",
                    "^0000FF20 Ordinary Branches and 20 Strong Vine^000000.",
                    "Please bring them to me!!!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ep13_newbs").get()? == 7 {
        ctx.lines_as(
            "Diego",
            args![
                "Ok, so while I hold this post,",
                "please attach the branches",
                "and tie it up with the vines."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_newbs").get()? == 8 {
        ctx.lines_as("Diego", args!["Please do the same for the other one."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_newbs").get()? == 9 {
        if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
            ctx.lines(args![
                "- Wait a minute !! -",
                "- Currently you're carrying -",
                "- too many items with you. -",
                "- Please try again -",
                "- after you lose some weight. -"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Diego",
                args![
                    "Thanks to your help,",
                    "I can protect the barracks.",
                    "When I was in trouble,",
                    "you came along.",
                    "It's got to be fate."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["No, I just...", "I was assigned to stay", "in these barracks."],
            )?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_PROFUSELY_SWEAT")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Diego",
                args![
                    "Oh!!!",
                    "Then it this really is fate!!!",
                    "And this is the special",
                    "product of this camp,",
                    "^0000FFChocolate Pie^000000."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Diego",
                args!["Please come in and", "take some rest for the", "expedition tomorrow!!"],
            )?;
            ctx.var("ep13_newbs").set(Val::from(10))?;
            ctx.call(Function::GetItem, vec![Val::from(12322), Val::from(5)])?;
            ctx.call(Function::CompleteQuest, vec![Val::from(11090)])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("mid_campin"), Val::from(291), Val::from(128)])?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as("Diego", args!["I'm busy right now!"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn diego_ep13bs(ctx: &Ctx) -> Script {
    diego_ep13bs_body(ctx, Vec::new()).map(|_| ())
}

fn ep13bs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn ep13bs(ctx: &Ctx) -> Script {
    ep13bs_body(ctx, Vec::new()).map(|_| ())
}

fn ep13bs_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_newbs").get()?.number()? < 4 {
        ctx.lines(args!["- It's a neat and tidy bed -", "- It seems that nobody used it. -"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_newbs").get()?.number()? > 3 && ctx.var("ep13_newbs").get()?.number()? < 10) {
        ctx.lines(args!["- It's a neat and tidy bed -", "- It seems that I will use this one. -"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_newbs").get()? == 10 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Haaaam~~~~~", "what a good night!!"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "- I slept well -",
            "- and feel better. -",
            "- I think I need to go -",
            "- to Instructor Lugen -"
        ])?;
        ctx.var("ep13_newbs").set(Val::from(11))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args!["- The bed that I use -", "- It is comfortable. -."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn ep13bs_ontouch(ctx: &Ctx) -> Script {
    ep13bs_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn lucas_ep13bs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_newbs").get()?.number()? < 11 {
        ctx.lines_as("Lucas", args!["......"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_newbs").get()? == 11 {
        ctx.lines_as("Lucas", args!["......"])?;
        ctx.next()?;
        ctx.lines_as("Lucas", args!["...Hmm...", "Replacement?", "......"])?;
        ctx.next()?;
        ctx.lines_as(
            "Lucas",
            args!["The Federal team's still...", "send us people using the", "dangerous way."],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Dangerous way?:Disregard it.")])? {
            1 => {
                ctx.lines_as("Lucas", args!["......", "I have been here for a long time."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Lucas",
                    args!["To breach the dimensional crack, the assassin team was deployed first..."],
                )?;
                ctx.next()?;
                ctx.lines_as("Lucas", args!["However, most of my men...", "......"])?;
                ctx.next()?;
                ctx.lines_as("Lucas", args!["No, just ignore what I said.", "......"])?;
                ctx.var("ep13_newbs").set(Val::from(12))?;
                ctx.call(Function::GetItem, vec![Val::from(12322), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Lucas", args!["......"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if (ctx.var("ep13_newbs").get()?.number()? > 11 && ctx.var("ep13_newbs").get()?.number()? < 100) {
        ctx.lines_as("Lucas", args!["You seem to live hard all the time..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Lucas",
            args!["The motherland constantly", "sends us people...", "Is that passage safe?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn lucas_ep13bs(ctx: &Ctx) -> Script {
    lucas_ep13bs_body(ctx, Vec::new()).map(|_| ())
}

fn davi_ep13bs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Davi", args!["Ahhh, my body...", "I walked too much...", "and my body hurts."])?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn davi_ep13bs(ctx: &Ctx) -> Script {
    davi_ep13bs_body(ctx, Vec::new()).map(|_| ())
}

fn jan_ep13bs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_newbs").get()? == 15 {
        if ctx.call(Function::CountItem, vec![Val::from(6045)])?.number()? > 0 {
            ctx.lines_as("Jan", args!["Wawa, what is this~", "Hey, is that for me?"])?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Yes..."])?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_SWEAT")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Jan", args!["Ahhh, it's exciting~", "I love it."])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_COOL")?])?;
            ctx.next()?;
            ctx.lines_as(
                "Jan",
                args![
                    "I want to unwrap it right now~",
                    "Please tell the Instructor",
                    "I'm grateful~",
                    "Uhuhu."
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(6045), Val::from(1)])?;
            ctx.var("ep13_newbs").set(Val::from(16))?;
            ctx.call(Function::GetItem, vec![Val::from(12322), Val::from(1)])?;
            ctx.call(Function::ChangeQuest, vec![Val::from(11093), Val::from(11094)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Jan",
                args!["Why hasn't the product", "that I ordered one month", "ago come yet?"],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("ep13_newbs").get()? == 16 {
            ctx.lines_as("Jan", args!["I really wanted it.", "I really love it~~"])?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["What is... it~~?"])?;
            ctx.next()?;
            ctx.lines_as("Jan", args!["......", "......"])?;
            ctx.next()?;
            ctx.lines_as("Jan", args!["It, it's a secret..."])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SHY")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Jan", args!["The more I study around here,", "the more I find strange things."])?;
            ctx.next()?;
            ctx.lines_as("Jan", args!["It seems to stimulate my", "passion as a scientist?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn jan_ep13bs(ctx: &Ctx) -> Script {
    jan_ep13bs_body(ctx, Vec::new()).map(|_| ())
}

fn gerard_ep13bs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_newbs").get()? == 17 {
        if ctx.call(Function::CountItem, vec![Val::from(6045)])?.number()? > 0 {
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Are you Gerard?"])?;
            ctx.next()?;
            ctx.lines_as("Gerard", args!["Yes, I am.", "What brings you here?"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Instructor Lugen sent", "me for an errand."],
            )?;
            ctx.next()?;
            ctx.lines_as("Gerard", args!["Ah! Supplies.", "The supplies that ran out."])?;
            ctx.next()?;
            ctx.lines_as(
                "Gerard",
                args![
                    "I am so hungry that",
                    "I was considering",
                    "going back to the camp.",
                    "I really appreciate this."
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_HUNGRY")?])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["...", "You ran out of supplies, but", "you didn't go back to the camp?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Gerard",
                args!["Yes.", "This place is really interesting", "and I don't want to leave at all."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Gerard",
                args!["Please tell the instructor that", "I won't be back for some time."],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
            ctx.call(Function::DelItem, vec![Val::from(6045), Val::from(1)])?;
            ctx.var("ep13_newbs").set(Val::from(18))?;
            ctx.call(Function::GetItem, vec![Val::from(12322), Val::from(1)])?;
            ctx.call(Function::ChangeQuest, vec![Val::from(11095), Val::from(11096)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Gerard",
                args!["Hhhh, I am starving.", "When will the supplies come?", "It is killing me..."],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("ep13_newbs").get()? == 18 {
            ctx.lines_as(
                "Gerard",
                args!["Hu, I am full now.", "Time to continue on", "with the exploration!!!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Gerard",
                args![
                    "I was born in a desert",
                    "and have never seen",
                    "a field so green.",
                    "So, to study here.",
                    "is very interesting."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Gerard", args!["By the way... is this a", "mushroom or a tree?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn gerard_ep13bs(ctx: &Ctx) -> Script {
    gerard_ep13bs_body(ctx, Vec::new()).map(|_| ())
}

fn alberto_ep13bs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_newbs").get()? == 19 {
        if ctx.call(Function::CountItem, vec![Val::from(6045)])?.number()? > 0 {
            ctx.lines_as("Alberto", args!["Hhh, I am cold..."])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Mr. Alberto, take this.", "The Instructor sent", "me on an errand."],
            )?;
            ctx.next()?;
            ctx.lines_as("Alberto", args!["I finally got it.", "My coat...sniff.", "It's freezing here."])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
            ctx.next()?;
            ctx.lines_as("Alberto", args!["Sniff...", "- rustling sound -"])?;
            ctx.next()?;
            ctx.lines_as(
                "Alberto",
                args![
                    "Ah, I am sorry but...",
                    "My hands are so cold",
                    "that I cannot open it.",
                    "Can you open it for me?."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "- rustling sound -",
                "- Opening someone -",
                "- else's stuff makes -",
                "- me feel strange. -"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Here it is.", "Hopefully it'll warm you up."],
            )?;
            ctx.next()?;
            ctx.lines_as("Alberto", args!["Yes I really appreciate it.", "It makes me feel alive."])?;
            ctx.next()?;
            ctx.lines_as(
                "Alberto",
                args!["Please tell the Instructor", "that I am still alive.", "Sniff."],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
            ctx.call(Function::DelItem, vec![Val::from(6045), Val::from(1)])?;
            ctx.var("ep13_newbs").set(Val::from(20))?;
            ctx.call(Function::GetItem, vec![Val::from(12322), Val::from(1)])?;
            ctx.call(Function::ChangeQuest, vec![Val::from(11097), Val::from(11098)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Alberto", args!["It's so cold here~~", "the wind chills me to the bones~"])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("ep13_newbs").get()? == 20 {
            ctx.lines_as(
                "Alberto",
                args!["I am still cold even", "with this heavy coat.", "I need to order more clothes."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Alberto", args!["What are these weird", "structures set up", "here and there?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn alberto_ep13bs(ctx: &Ctx) -> Script {
    alberto_ep13bs_body(ctx, Vec::new()).map(|_| ())
}

fn cooking_soldier_ep13bs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Alix",
        args![
            "Huu, I made this and",
            "this is really great.",
            "When I am back",
            "to the motherland,",
            "I will open a shop for it!!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Alix",
        args!["Have you had it?", "My chocolate pie!!!", "This food is a real miracle!!"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn cooking_soldier_ep13bs(ctx: &Ctx) -> Script {
    cooking_soldier_ep13bs_body(ctx, Vec::new()).map(|_| ())
}

fn sorcerer_ep13bs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Biolay", args!["Sob...", "Where is she?"])?;
    ctx.next()?;
    ctx.lines_as(
        "Biolay",
        args![
            "I was just wandering",
            "around Morocc and",
            "became faint. then",
            "I found myself here."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Biolay", args!["I want to go back to my house."])?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn sorcerer_ep13bs(ctx: &Ctx) -> Script {
    sorcerer_ep13bs_body(ctx, Vec::new()).map(|_| ())
}

fn tree_ep13bs1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a minute !! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again -",
            "- after you lose some weight. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_newbs").get()? == 6 {
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 3 {
            ctx.lines(args!["- rustling sound -", "- You find a branch -"])?;
            ctx.next()?;
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 2 {
                ctx.lines(args!["- Break! -", "- Bomb, bomb~~ -", "- You got the branch. -"])?;
                ctx.call(Function::GetItem, vec![Val::from(6042), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines(args!["- Break! -", "- Bomb, bomb~~ -", "- This branch is decayed. -"])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["This is useless.", "I need to find others..."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines(args!["- rustling sound -", "- There's nothing useful. -"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    return Err(Stop::End);
}

pub fn tree_ep13bs1(ctx: &Ctx) -> Script {
    tree_ep13bs1_body(ctx, Vec::new()).map(|_| ())
}

fn post_ep13bs1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_newbs").get()?.number()? < 6 {
        ctx.lines(args!["- The posts of the -", "- barracks are broken. -"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ep13_newbs").get()? == 6 {
        ctx.lines(args![
            "- The posts of the -",
            "- barracks are broken. -",
            "- You'd better go out and -",
            "- find the materials to fix it. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ep13_newbs").get()? == 7 {
        ctx.lines(args![
            "- There are broken posts. -",
            "- Unless they are fixed soon, -",
            "- the barracks will collapse. -"
        ])?;
        ctx.next()?;
        'l1: loop {
            if !(true) {
                break 'l1;
            }
            'b1: {
                match runtime::select_values(ctx, &[Val::from("Attach the tree bar.:Tie it with the vines.")])? {
                    1 => {
                        ctx.lines(args!["- You attach the tree bar -", "- to the broken post -"])?;
                        ctx.next()?;
                    }
                    2 => {
                        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])? == 2 {
                            ctx.mes("- You tie the broken post -")?;
                            ctx.next()?;
                            ctx.mes("- It seems that it's fixed -")?;
                            ctx.next()?;
                            ctx.lines_as("Diego", args!["Oh! Now you need to", "knot it and finish it."])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Knot it.:Leave it.")])? {
                                1 => {
                                    ctx.mes("- You tie a perfect knot -")?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Diego",
                                        args!["And this post should be knotted.", "Then please do it to that post."],
                                    )?;
                                    ctx.var("ep13_newbs").set(Val::from(8))?;
                                    ctx.call(Function::ChangeQuest, vec![Val::from(11088), Val::from(11089)])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.mes("- You decide to leave -")?;
                                    ctx.next()?;
                                    ctx.lines(args!["- The branches slide down. -", "- You failed to fix it. -"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Diego", args!["...", "Why aren't you helping?"])?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        } else {
                            ctx.lines(args![
                                "- You fix the broken post -",
                                "- with the vines. -",
                                "- It seems that you-",
                                "- need more branches. -"
                            ])?;
                            ctx.next()?;
                        }
                    }
                    _ => {}
                }
            }
        }
    } else if ctx.var("ep13_newbs").get()? == 8 {
        ctx.lines_as(
            "Diego",
            args!["This post is repaired.", "Please do the same thing", "on the other posts."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "- The post tied by the -",
            "- vines seems unsteady -",
            "- but my repair is perfect. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn post_ep13bs1(ctx: &Ctx) -> Script {
    post_ep13bs1_body(ctx, Vec::new()).map(|_| ())
}

fn post_ep13bs2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_newbs").get()?.number()? < 6 {
        ctx.lines(args!["- The posts of the -", "- barracks are broken. -"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("ep13_newbs").get()? == 6 || ctx.var("ep13_newbs").get()? == 7) {
        ctx.lines(args![
            "- The posts of the -",
            "- barracks are broken. -",
            "- You'd better go out and -",
            "- find the materials to fix it. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ep13_newbs").get()? == 8 {
        ctx.lines(args![
            "- There are broken posts. -",
            "- Unless they are fixed soon, -",
            "- the barracks will collapse. -"
        ])?;
        ctx.next()?;
        'l1: loop {
            if !(true) {
                break 'l1;
            }
            'b1: {
                match runtime::select_values(ctx, &[Val::from("Attach the tree bar.:Tie it with the vines.")])? {
                    1 => {
                        ctx.lines(args!["- You attach the tree bar -", "- to the broken post -"])?;
                        ctx.next()?;
                    }
                    2 => {
                        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])? == 4 {
                            ctx.mes("- You tie the broken post -")?;
                            ctx.next()?;
                            ctx.mes("- It seems that it's fixed -")?;
                            ctx.next()?;
                            ctx.lines_as("Diego", args!["Oh! Now you need to knot it and", "finish it."])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Knot it.:Leave it.")])? {
                                1 => {
                                    ctx.mes("- You tie a perfect knot -")?;
                                    ctx.next()?;
                                    ctx.lines_as("Diego", args!["Ohhh!!!!", "The broken posts are perfectly fixed!"])?;
                                    ctx.var("ep13_newbs").set(Val::from(9))?;
                                    ctx.call(Function::ChangeQuest, vec![Val::from(11089), Val::from(11090)])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.mes("- You decide to leave -")?;
                                    ctx.next()?;
                                    ctx.lines(args!["- The branches slide down. -", "- You failed to fix it. -"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Diego", args!["...", "Why aren't you helping?"])?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        } else {
                            ctx.lines(args![
                                "- You fix the broken post -",
                                "- with the vines. -",
                                "- It seems that you-",
                                "- need more branches. -"
                            ])?;
                            ctx.next()?;
                        }
                    }
                    _ => {}
                }
            }
        }
    } else {
        ctx.lines(args![
            "- The post tied by the -",
            "- vines seems unsteady -",
            "- but my repair is perfect. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn post_ep13bs2(ctx: &Ctx) -> Script {
    post_ep13bs2_body(ctx, Vec::new()).map(|_| ())
}

fn monster_scholar_ep13_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if (ctx.var("ep13_ryu").get()?.number()? < 100 && ctx.var("ep13_start").get()?.number()? < 100) {
        ctx.lines_as(
            "Monster Scholar",
            args![
                "Who... Who are you?",
                "Are you from the other",
                "side of the space gap?",
                "Say, are you a scholar,",
                "soldier, or village",
                "representative?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Monster Scholar",
            args!["Oh, I'm sorry.", "You don't have to tell", "me if you don't want to..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("ep13_animal").get()? == 0 {
            ctx.lines_as(
                "Monster Scholar",
                args![
                    "Who... Who are you?",
                    "Are you from the other",
                    "side of the space gap?",
                    "Say, are you a scholar,",
                    "soldier, or village",
                    "representative?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monster Scholar",
                args!["Oh, I'm sorry.", "You don't have to tell", "me if you don't want to..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monster Scholar",
                args![
                    "Please go ahead and do",
                    "what you have to do.",
                    "Oh, me? Don't worry.",
                    "I'm fine by myself."
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Who are you?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Monster Scholar",
                args![
                    "I'm Rumis Block, and",
                    "I'm working here on behalf of",
                    "the monsternomics academy",
                    "from the Schwarzwald Republic.",
                    "Botanist Terris Block is",
                    "my older twin brother."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "This place is freezing. Not only",
                    "that, it's deserted and too",
                    "quiet... Everything around us is",
                    "watching and threatening us.",
                    "You know, I didn't want to",
                    "come here..."
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("How did you get here?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "In fact, none of us",
                    "wanted to come here.",
                    "I mean, no one knows",
                    "what kind of danger lurks",
                    "in this unknown world."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "It was my brother Terris who",
                    "wanted me to come along.",
                    "He was selected first as the",
                    "botanist of the continental",
                    "expedition. He had to drag me",
                    "here 'cuz I didn't want to come."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "I guess he didn't want to be",
                    "alone in this world far away",
                    "from civilization. Instead, he",
                    "decided to drag me here and",
                    "force me to join the expedition",
                    "so he doesn't feel lonely!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "What's with that dirty look?",
                    "Why, do I sound like",
                    "someone blaming someone",
                    "else for the misery in his life?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "I'm telling you, it's the truth.",
                    "There are people that want ",
                    "to escape the expedition by",
                    "getting themselves sick on purpose."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Rumis Block", args!["I know it's too late to", "complain now, but..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "Every day I pray to God to",
                    "let me go back home safely.",
                    "That's the only thing that",
                    "keeps me going. Everything",
                    "around here is watching me.",
                    "It's suffocating, you know?"
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("When do you go back?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "That's the problem.",
                    "We Specialists are required",
                    "to submit our study reports",
                    "to the expedition's leaders.",
                    "We are here to explore",
                    "and study this world."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "I'm sure the management will",
                    "tell me when to go back, but",
                    "I don't expect it to be any time",
                    "soon. It'll be at least after",
                    "I submit my report."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args!["I must tell you,", "I hate this place!", "It gives me the creeps!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "Yes, I'm a monster scholar",
                    "But I've been afraid of creatures even when I was on the Midgard Continent..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "No, but can't you see?",
                    "This place is totally new to me,",
                    "and everything is so unknown..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args!["I may be a monster scholar,", "but I'm afraid...", "Of many things..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "The president of United",
                    "Midgard Corporation has",
                    "asked me to find some",
                    "monsters that are edible."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "But look outside. Look!",
                    "Do you see Mandragoras",
                    "hiding in the icy bushes?",
                    "Those are called ^3131FFNepentheses^000000."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "Aren't they quite savage-",
                    "looking and intimidating?",
                    "They look as if they",
                    "want to sting me with",
                    "those sharp thorns!",
                    "Argh..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "I'd rather starve to",
                    "death than study",
                    "those dangerous and",
                    "grotesque monsters!",
                    "I'm not going to go",
                    "outside of my tent."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rumis Block",
                args![
                    "Not ever.",
                    "No one can dare",
                    "venture through",
                    "such a threatening",
                    "environment!"
                ],
            )?;
            ctx.var("ep13_animal").set(Val::from(1))?;
            ctx.call(Function::SetQuest, vec![Val::from(2147)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("ep13_animal").get()? == 1 {
                ctx.lines_as("Rumis Block", args!["I'm not going to go", "outside of my tent.", "Not ever."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("ep13_animal").get()? == 2 {
                    if ctx.call(Function::CountItem, vec![Val::from(6041)])?.number()? > 0 {
                        ctx.lines_as(
                            "Rumis Block",
                            args![
                                "I'm scared of this place.",
                                "But I'm not going to ask",
                                "my brother for help",
                                "He hates me."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rumis Block",
                            args!["If a Nepenthes", "approaches me", "with a smile and", "speaks to me kindly..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rumis Block",
                            args![
                                "*Sigh* I sound crazy, don't I?",
                                "...This is why I didn't want",
                                "to leave my lab..."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Show loot from Nepentheses.:Scold him.:Comfort him.")])? {
                            1 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Hey, take a look here...",
                                        "These Nepentheses aren't",
                                        "any more brutal or grotesque",
                                        "than the ones back home."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Rumis Block", args!["...What's this?", "Isn't this from", "the Nepentheses?"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["That's right!", "I took care of them.", "It was easy, haha!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rumis Block",
                                    args![
                                        "I can't believe you really",
                                        "took care of them! Tell me,",
                                        "who are you really?",
                                        "Are you a soldier?",
                                        "Captain? Or agent?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "No, I'm just a wandering",
                                        "adventurer. I've come here",
                                        "to challenge this new place",
                                        "and to help people out~"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rumis Block",
                                    args![
                                        "I... I see.",
                                        "I've heard about adventurers",
                                        "that have made positive",
                                        "impacts on Midgard.",
                                        "You're one of them, huh?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rumis Block",
                                    args![
                                        "...That's great!",
                                        "I think I've finally found",
                                        "a silver lining. God has",
                                        "sent me a savior.",
                                        "And I think that's you!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rumis Block",
                                    args![
                                        "Please, don't say you",
                                        "don't want to help me.",
                                        "Can't you see that.",
                                        "I'm shivering in fear?",
                                        "Please help me. Will you?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rumis Block",
                                    args!["As I told you earlier,", "I have an elder twin", "brother, Terris Block..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rumis Block",
                                    args![
                                        "He's a botanist. He's",
                                        "extremely strong and",
                                        "extroverted, unlike me.",
                                        "I think he hates me because",
                                        "I represent weakness, and",
                                        "he is the complete opposite."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rumis Block",
                                    args![
                                        "...Oh, that's not what I wanted",
                                        "to say. I wanted to show my",
                                        "discovery regarding the",
                                        "Nepentheses to my brother."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rumis Block",
                                    args![
                                        "I need a Nepenthes specimen,",
                                        "but I don't have the tools to",
                                        "make one, my luggage is still",
                                        "on transfer. And it's not like",
                                        "I've packed enough tools, so..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rumis Block",
                                    args![
                                        "To make my specimen tools,",
                                        "I need ^3131FF1 Empty Bottle,",
                                        "5 Holy Waters, and",
                                        "30 Sticky Mucus^000000."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rumis Block",
                                    args!["You know what I'm.", "asking here, don't you?", "Help me, help me, please."],
                                )?;
                                ctx.call(Function::DelItem, vec![Val::from(6041), Val::from(1)])?;
                                ctx.var("ep13_animal").set(Val::from(3))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(2147), Val::from(2148)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "You call yourself a monster",
                                        "scholar, and yet you're afraid",
                                        "of practicing your expertise.",
                                        "You're nothing but a coward."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rumis Block",
                                    args!["I can take your criticism,", "but I can't hide my true feelings."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            3 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Come on, you're a respected",
                                        "monster scholar. You'll find",
                                        "that the monsters in this world",
                                        "aren't that different from the",
                                        "ones in Midgard. Why don't",
                                        "you see for yourself?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "- Rumis didn't seem -",
                                    "- to be convinced. -",
                                    "- He shook his head in refusal. -"
                                ])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else {
                        ctx.lines_as("Rumis Block", args!["I'm not going to go", "outside of my tent.", "Not ever."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("ep13_animal").get()? == 3 {
                        if ((ctx.call(Function::CountItem, vec![Val::from(713)])?.number()? > 0
                            && ctx.call(Function::CountItem, vec![Val::from(523)])?.number()? > 4)
                            && ctx.call(Function::CountItem, vec![Val::from(938)])?.number()? > 29)
                        {
                            ctx.lines_as("Rumis Block", args!["Thank you.", "you've brought the materials."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rumis Block",
                                args![
                                    "While you were collecting the",
                                    "materials, I've sent out soldiers",
                                    "to catch a few Nepentheses."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rumis Block",
                                args!["Nepenthes looks extremely", "similar to Mandragora on", "the Midgard Continent."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rumis Block",
                                args![
                                    "Both of them must share the",
                                    "same ancestor, but apparently",
                                    "Nepenthes has evolved to",
                                    "become stronger and more",
                                    "aggressive due to this",
                                    "barren environment."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rumis Block",
                                args!["I wonder how the ancestors", "were scattered over", "two different worlds..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rumis Block",
                                args![
                                    "Maybe they went through",
                                    "the World Tree Yggdrasil?",
                                    "Maybe it came along with",
                                    "other creatures such as",
                                    "Satan Morocc and traversed",
                                    "through time and space."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Rumis Block", args!["The characteristics of this species are..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rumis Block",
                                args![
                                    "Oh, where are my manners?",
                                    "I'm sorry if I bored you by",
                                    "talking academic nonsense...",
                                    "Let's talk about something",
                                    "else, shall we?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rumis Block",
                                args![
                                    "Do you think...",
                                    "My brother will be happy to",
                                    "see this Nepenthes specimen?",
                                    "You know, Nepenthes is a",
                                    "plant monster, and that's",
                                    "my brother's specialty."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rumis Block",
                                args![
                                    "Please deliver this specimen to Terris Block",
                                    "He's near the expedition camp.",
                                    "No, you won't have to sit through my brother's reaction... If it's bad."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rumis Block",
                                args!["- You have received a-", "- Nepenthes Specimen -", "- from Rumis Block. -"],
                            )?;
                            ctx.call(Function::DelItem, vec![Val::from(713), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(523), Val::from(5)])?;
                            ctx.call(Function::DelItem, vec![Val::from(938), Val::from(30)])?;
                            ctx.var("ep13_animal").set(Val::from(4))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(2148), Val::from(2149)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Rumis Block",
                                args![
                                    "To make my specimen tools,",
                                    "I need ^3131FF1 Empty Bottle, 5 Holy Waters, and 30 Sticky Mucus^000000."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Rumis Block",
                                args!["You know what I'm asking here, don't you? Thank you in advance."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if (ctx.var("ep13_animal").get()?.number()? > 3 && ctx.var("ep13_animal").get()?.number()? < 10) {
                            ctx.lines_as(
                                "Rumis Block",
                                args![
                                    "Did you deliver the Nepenthes Specimen to my brother Terris?",
                                    "He's at the east of the camp."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("ep13_animal").get()? == 10 {
                                ctx.lines_as(
                                    "Rumis Block",
                                    args![
                                        "...Welcome back.",
                                        "Oh, you don't have to tell me what my brother said,",
                                        "because someone from his side already came by. His report really impressed management, huh?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Rumis Block", args!["Yes, that's my brother......"])?;
                                ctx.next()?;
                                ctx.lines_as("Rumis Block", args!["...Anyways, one of the soldiers that captured Nepentheses for me has given me some interesting information."])?;
                                ctx.next()?;
                                ctx.lines_as("Rumis Block", args!["Over the eastern bridge, he found a new type of monster."])?;
                                ctx.next()?;
                                ctx.lines_as("Rumis Block", args!["According to his description, the monster is suspected to be a member of the long-haired Cat family. I'd like to see it with my own eyes, but..."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rumis Block",
                                    args!["As you know, I'm not yet ready to face any types of monsters in this world..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rumis Block",
                                    args!["Can you escort me while I'm studying that species?", "Please, I'm begging you!"],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("Okay.:No.")])? {
                                    1 => {
                                        ctx.lines_as(
                                            "Rumis Block",
                                            args!["Wow, thanks!", "Then I'd better pack my stuff right away..."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Rumis Block",
                                            args![
                                                "It won't take long. Can you please go wait for me over the eastern bridge?",
                                                "Don't... Don't go too far from the bridge!"
                                            ],
                                        )?;
                                        ctx.var("ep13_animal").set(Val::from(11))?;
                                        ctx.call(Function::ChangeQuest, vec![Val::from(2153), Val::from(2154)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                        ctx.lines_as("Rumis Block", args!["Oh........."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            } else if ctx.var("ep13_animal").get()? == 11 {
                                ctx.lines_as(
                                    "Rumis Block",
                                    args![
                                        "I'll pack my stuff and follow you. Can you please go wait for me over the rightward bridge?",
                                        "Don't... Don't go too far from the bridge!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("ep13_animal").get()? == 12 {
                                ctx.mes("- Rumis is hurriedly packing his stuff. -")?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("ep13_animal").get()? == 13 {
                                ctx.lines_as("Rumis Block", args!["......"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rumis Block",
                                    args!["Apparently, an extremely intelligent creature inhabits Ash Vaccum."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Rumis Block", args!["An explorer went over the rightward bridge to find some wood. In the darkness, something really big rushed at him from far away."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rumis Block",
                                    args!["When he came back to his senses, there was nothing but a tricorn hat left on the ground."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Rumis Block", args!["The tricorn is woven in a way that was never introduced to Midgard, and its materials are also unidentifiable."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rumis Block",
                                    args!["Does it mean another race of human resides here? Would they be fairies or something else?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Rumis Block",
                                    args!["What if they've detected us and tried to drive us away... Wah!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Rumis Block", args!["Even thinking of them send chills down on my spine! Can you please go check what kind of races reside at the end of the land over the rightward bridge?", "I want to know if they're friendly or hostile to us."])?;
                                ctx.var("ep13_animal").set(Val::from(14))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(2156), Val::from(2157)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("ep13_animal").get()? == 14 {
                                if ctx.call(Function::CheckQuest, vec![Val::from(2157), ctx.constant("HUNTING")?])? == 2 {
                                    ctx.lines_as("Rumis Block", args!["Welcome back. Did you find out about them?"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["Well, let's see..."],
                                    )?;
                                    ctx.next()?;
                                    'l3: loop {
                                        if !(true) {
                                            break 'l3;
                                        }
                                        'b3: {
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["The owner of the tricorn hat", "is a monster called..."],
                                            )?;
                                            let (input, status) = runtime::input_text(ctx, None, None)?;
                                            l_input_s = input;
                                            if runtime::compare(&l_input_s.clone(), &Val::from("Tatacho")).is_true() {
                                                ctx.mes("^FF0000Tatacho^000000")?;
                                                ctx.next()?;
                                                break 'l3;
                                            } else {
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args![
                                                        ((Val::from("") + l_input_s.clone())
                                                            + Val::from("...? I don't think that was the name..."))
                                                    ],
                                                )?;
                                                ctx.next()?;
                                            }
                                        }
                                    }
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["The size of Tatacho is about..."],
                                    )?;
                                    let (input, status) = runtime::input_text(ctx, None, None)?;
                                    l_input_s = input;
                                    ctx.lines(args![
                                        ((Val::from("^3131FF") + l_input_s.clone()) + Val::from("^000000, I guess..."))
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Tatachos are usually sitting on the ground, but they roll around if their body temperature goes down too low because of the cold weather."])?;
                                    ctx.next()?;
                                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["About that tricorn hat, I've concluded, based on my opinion about their appearance and their movement patterns, that it is not active or productive at all:"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["They are not intelligent enough to produce a sophisticated hat like that."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Rumis Block", args!["......", "You now sound like me... I guess I've stained you with my academic manner of speech. Oh, I'm not saying it is bad, but..."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Rumis Block", args!["Anyways, according to your conclusion, the hat must have been given to them by someone else, or they could have picked it up after another creature dropped it.", "Thank you for such valuable information.", "It's enough to write a report."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Rumis Block", args!["I should make a brief report and then submit it to management. Hopefully, a new monster scholar after me will do a better job in actually studying them."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Rumis Block", args!["There are many cold places in my country, the Schwarzwald Republic, but none are as cold as this place. This weather is freezing my senses, dulling my passion toward my studies.", "I shall go back home faster than anyone else in this camp. I'm tired of this world..."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Rumis Block",
                                        args![
                                            ((Val::from("...") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                + Val::from(", I thank you so much for helping me despite my self-centered behavior.")),
                                            "I really appreciate it"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Rumis Block",
                                        args![
                                            "I'll repay your favor as soon as I go back home.",
                                            "I hope you'll make yourself known in Ash Vacuum with your great accomplishments."
                                        ],
                                    )?;
                                    ctx.var("ep13_animal").set(Val::from(15))?;
                                    ctx.call(Function::CompleteQuest, vec![Val::from(2157)])?;
                                    ctx.call(Function::GetExperience, vec![Val::from(150000), Val::from(0)])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as("Rumis Block", args!["Can you please go check what kind of races reside at the end of the land over the rightward bridge? I want to know if they're friendly or hostile to us."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            } else {
                                if ctx.var("ep13_animal").get()? == 15 {
                                    if ctx.call(Function::CountItem, vec![Val::from(6033)])?.number()? > 0 {
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["Mr. Rumis! Take a look at this!"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Rumis Block", args!["...Oh, hello.", "I'm extremely frustrated. I feel helpless because I wasn't able to go back home last time..."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args![
                                                "I've found this horn from a monster in the Splendide Area.",
                                                "That monster was very mysterious, and its body was covered with grass."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Rumis Block",
                                            args![
                                                "Ho... Covered with grass?",
                                                "...I know you're excited, but that doesn't sound interesting to me. Sorry."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Rumis Block", args!["Let me take a look at the horn."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Rumis Block", args!["Hmm."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Rumis Block", args!["Uhmm..."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Rumis Block", args!["Huh...?"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Rumis Block",
                                            args![
                                                "...This is remarkable.",
                                                "...This horn looks very similar to that of the Hillsrions that inhabit the Manuk area."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Rumis Block",
                                            args!["I wonder if they're related,", "or if this is an evolved Hillsrion."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["Ah, and there's the possibility of mutation caused by the environment."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Rumis Block",
                                            args![
                                                "...What do you mean?",
                                                "Are you saying the monster has been mutated by environmental causes?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["Well, that's possible, isn't it? Haha!"],
                                        )?;
                                        ctx.next()?;
                                        ctx.mes("- You told Rumis what you've heard from Botanist Terris: some plants are already showing signs of mutation. -")?;
                                        ctx.next()?;
                                        ctx.lines_as("Rumis Block", args!["Oh, that's interesting.", "If such a device really exists, it's possible to cause abnormal growth to monsters by injecting them with special energy."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Rumis Block", args!["By the way, who came up with the idea of that device?"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["I have no idea. Haha!"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Rumis Block", args!["I see...", "Mutants caused by a man-made device... Then there might be more mutated creatures in addition to Tendrillion."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Rumis Block", args!["...Now I'm very curious.", "But... I want to go back home... It's no use studying them if the management will order me to return."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Rumis Block",
                                            args![
                                                "I guess my brother really likes this place.",
                                                "I mean, he's working so hard to figure things out..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Rumis Block",
                                            args![
                                                "Well, not everyone can live the same life.",
                                                "I just hope he'll have a better understanding about me..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Rumis Block",
                                            args![
                                                "I don't know if people in Midgard will welcome me back.",
                                                "Thanks to you, I at least have a few interesting stories to tell them."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Rumis Block",
                                            args![
                                                "If you're going to stick around here longer, please help my brother with his study.",
                                                "I might want to come back later once this area is fully explored and developed..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.mes("- With a shy smile on the face, Rumis Block asked you to shake hands, and then turned around hurriedly. -")?;
                                        ctx.next()?;
                                        ctx.mes("- You really hope that Rumis will be able to go back home. -")?;
                                        ctx.var("ep13_animal").set(Val::from(100))?;
                                        ctx.call(Function::DelItem, vec![Val::from(6033), Val::from(1)])?;
                                        ctx.call(Function::GetExperience, vec![Val::from(100000), Val::from(0)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as("Rumis Block", args!["...I submitted the report to the management, but they have not yet ordered me to go back home...", "...*Sigh* I guess nothing's as easy as I'd hoped."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                } else {
                                    if ctx.var("ep13_animal").get()? == 100 {
                                        ctx.mes("- Rumis seems anxious and scatterbrained. -")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as("Rumis Block", args!["When can I go back to Midgard...? *Sigh*"])?;
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
    Ok(Val::from(0))
}

pub fn monster_scholar_ep13(ctx: &Ctx) -> Script {
    monster_scholar_ep13_body(ctx, Vec::new()).map(|_| ())
}
