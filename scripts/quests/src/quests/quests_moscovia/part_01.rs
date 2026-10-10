use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn berbayeff_npc_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_speak = Val::from(0);
    if !(ctx.var("mos_whale_edq").get()?.is_true()) {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
        ctx.lines_as(
            "Berbayeff",
            args!["This..and..that...", "All guys...are distrustful.", "Makes me a braggart."],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Do you have a problem?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "I haven't seen you before.",
                "Are you a traveler?",
                "If so, then you may have",
                "seen many marvelous things",
                "from near and far."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "I'm sure you'll probably",
                "be interested in my story.",
                "All the villagers are blockheads,",
                "so they treat me like an idiot."
            ],
        )?;
        if ctx.call(Function::IsBeginQuest, vec![Val::from(18100)])? == 0 {
            ctx.call(Function::SetQuest, vec![Val::from(18100)])?;
        }
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("I'm not interested in any story.:Tell me.")],
        )?) == 1
        {
            ctx.lines_as("Berbayeff", args!["Indeed... You are not interested...", "That's ok."])?;
            ctx.next()?;
            ctx.lines_as("Berbayeff", args!["If you change your mind,", "you can talk to me whenever."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
        ctx.lines_as(
            "Berbayeff",
            args!["Good, I have been thinking that", "I would tell you this story from the start."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "You seem to have spent",
                "only a few days here.",
                "Have you ever heard",
                "about The Moving Island?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args!["In Moscovia,", "a story is handed down", "from the ancients, like legends."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "It is said that",
                "there is an island",
                "which sometimes moves,",
                "not far out at sea."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "But, nobody has seen",
                "the island before.",
                "So, few people have",
                "believed the legend."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "Actually, I thought that",
                "it might be a kind of",
                "old story... nothing more.",
                "But then... I saw..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "Actually, I thought that",
                "it might be a kind of",
                "old story... nothing more.",
                "But then... I saw...",
                "The Moving Island!!!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "Surprised? Right...",
                "It is so surprising.",
                "Frankly, you don't believe",
                "any of this story either."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args!["I saw the island", "from the middle of", "the village's south shore."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "At first, I doubted my eyes.",
                "I have never heard that that island",
                "really existed here, even though I",
                "have lived here all my life!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "I tried to go up",
                "to look at it from",
                "higher ground, but",
                "the island receded",
                "and disappeared...",
                "back to the sea."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("The island doesn't appear anymore?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Berbayeff",
            args!["If it does,", "one might just think", "that they've looked", "at an apparition."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "But a few nights ago...",
                "Finally, the island appeared",
                "in front of me again,",
                "while I was driving my ship to go home."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "At that time, I tried to",
                "get near the island again,",
                "slowly, but it disappeared...",
                "beyond the sea once more."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "It was then, I couldn't help",
                "to finally believe",
                "in the existence of",
                "The Moving Island!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "Surely, this must be",
                "a great discovery!",
                "Don't you think??",
                "Doesn't your heart shout out in",
                "hopes of adventuring out to the island?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args!["I haven't given up yet!", "Certainly, it'll appear again", "right in front of us!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "And on that day,",
                "I'll definitely go up to that",
                "island... and verify it with my own eyes!"
            ],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(18100), Val::from(18101)])?;
        ctx.var("mos_whale_edq").set(Val::from(1))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mos_whale_edq").get()? == 1 {
        l_speak = ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?;
        if l_speak.clone().number()? < 3 {
            ctx.lines_as(
                "Berbayeff",
                args!["Clearly, I want to prove", "the existence of that island to everyone..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Berbayeff",
                args![
                    "If the island appears again,",
                    "I should certify its identity.",
                    "Would you like to try it?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Berbayeff",
            args!["The best way to find the", "island is by direct contact", "with a ship in the sea."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args!["Me?", "Of course I have a ship.", "I am a fisherman after all..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args!["Ah, you do? You need...", "a ship? Hm...too bad.", "I can't lend my ship to you..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "But maybe Mr. Ibanoff",
                "can lend his ship to you",
                "He likes going around",
                "adventuring like me.",
                "He seems to enjoy his youth..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "Mr. Ibanoff also traveled and",
                "adventured all over the world",
                "like you, so he might understand",
                "your sentiments well."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berbayeff",
            args![
                "If you want to find the island",
                "by ship, go to Mr. Ibanoff",
                "and ask for a favor."
            ],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(18101), Val::from(18102)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("mos_whale_edq").get()?.number()? > 2 && ctx.var("mos_whale_edq").get()?.number()? < 42) {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
        ctx.lines_as(
            "Berbayeff",
            args!["Are you serious? Did you say", "you've found The Moving Island?"],
        )?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
        ctx.lines_as(
            "Berbayeff",
            args!["The legend is true!!", "I was right, wasn't I?", "You must be a great adventurer."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Berbayeff",
        args!["Welcome to Moscovia.", "What's your impression thus far?", "Looks gorgeous, huh?"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Berbayeff",
        args![
            "Most travelers and strangers",
            "like our village.",
            "I want you to visit places here and",
            "there. You'll probably love this town."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn berbayeff_npc(ctx: &Ctx) -> Script {
    berbayeff_npc_body(ctx, Vec::new()).map(|_| ())
}

fn bulletin_board_npc_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Charabel Schedule",
        args![
            "NOTICE, due to the varying",
            "tides here in Moscovia,",
            "the charabel can only",
            "depart when the tide is low."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Charabel Schedule",
        args![
            "The Charabel can depart",
            "between the following times:",
            "^0000FF12:00am PST^000000 to ^0000FF3:00am PST",
            "^0000FF6:00am PST^000000 to ^0000FF9:00am PST",
            "^0000FF12:00pm PST^000000 to ^0000FF3:00pm PST",
            "^0000FF6:00pm PST^000000 to ^0000FF9:00pm PST"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bulletin_board_npc(ctx: &Ctx) -> Script {
    bulletin_board_npc_body(ctx, Vec::new()).map(|_| ())
}

fn mr_ibanoff_npc_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mos_whale_edq").get()? == 0 {
        ctx.lines_as(
            "Mr. Ibanoff",
            args!["You are also an adventurer from another province. I also was a great adventurer."],
        )?;
        ctx.next()?;
        ctx.lines_as("Mr. Ibanoff", args!["Uf...I just want to be 20 years younger, so I can travel here and there with invigorating youth such as you... Time is an enemy. Hahahahaha!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if (ctx.var("mos_whale_edq").get()? == 1 || ctx.var("mos_whale_edq").get()? == 2) {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Hello. Are you Mr. Ibanoff?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Mr. Ibanoff", args!["Yes. I am Mr. Ibanoff.", "What's up?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Tell me an adventure story.:Lend me your ship.")],
            )?) == 1
            {
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "You are an adventurer, right?",
                        "You come from a strange land?",
                        "Well, well, well...",
                        "You are welcome here."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "When I adventured around",
                        "Rune-Midgarts, maybe it was",
                        "well before you were born.",
                        "Ah... I miss those days..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "Now, I'm so tired;",
                        "my body and my heart.",
                        "I get reminded of fond",
                        "memories just by looking at the sea..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "Um... Stories...",
                        "What to talk about first?",
                        "Right, I'll talk about that time I",
                        "fought against a huge flock of Porings..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Hahaha..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "You'll be happy that you",
                        "listened to the adventure story.",
                        "Well, well, well...",
                        "When I was 24 years old...",
                        "My colleagues and I were passing by",
                        "a forest near Payon..."
                    ],
                )?;
                ctx.next()?;
                ctx.mes("when...")?;
                ctx.next()?;
                ctx.mes("and....")?;
                ctx.next()?;
                ctx.mes("then.....")?;
                ctx.next()?;
                ctx.mes("Finally......")?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "I did a very good",
                        "job in the fight, so",
                        "my colleagues and I were",
                        "able to get back safely."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "What did you think?",
                        "Next... Let's talk about",
                        "the haunting of a Goblin mob...!"
                    ],
                )?;
                if ctx.call(Function::IsBeginQuest, vec![Val::from(18102)])?.is_true() {
                    ctx.call(Function::ChangeQuest, vec![Val::from(18102), Val::from(18103)])?;
                } else if ctx.call(Function::IsBeginQuest, vec![Val::from(18101)])?.is_true() {
                    ctx.call(Function::ChangeQuest, vec![Val::from(18101), Val::from(18103)])?;
                }
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("I'll listen next time...:Interesting. Keep talking.")],
                )?) == 1
                {
                    ctx.lines_as(
                        "Mr. Ibanoff",
                        args![
                            "If not...Ooo, I can just",
                            "keep going and going...",
                            "No? Too bad... Hehe...",
                            "Let's talk next time."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args!["Yeah? Ohohoh... my friend,", "we are in sync with each other...!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "Ok, this time I'll talk",
                        "about the bloody fight at the",
                        "haunting of a Goblin mob..."
                    ],
                )?;
                ctx.next()?;
                ctx.mes("You see...")?;
                ctx.next()?;
                ctx.mes("and....")?;
                ctx.next()?;
                ctx.mes("also.....")?;
                ctx.next()?;
                ctx.mes("Lastly......")?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args!["So, the day's great bloody", "fight was over like that...", "Ohwee..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "Hah! You are the first",
                        "to listen carefully to",
                        "my stories, for a long time.",
                        "Feels so good..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "You absolutely have talent,",
                        "as a great adventurer.",
                        "You can become a good friend to me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "Next time, we talk about your",
                        "adventure story... eh? ~",
                        "If you ever need anything,",
                        "come talk to me whenever, hahaha!"
                    ],
                )?;
                ctx.var("mos_whale_edq").set(Val::from(2))?;
                if ctx.call(Function::IsBeginQuest, vec![Val::from(18103)])?.is_true() {
                    ctx.call(Function::ChangeQuest, vec![Val::from(18103), Val::from(18104)])?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ctx.var("mos_whale_edq").get()? == 2 {
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args!["Ah! The good listener!", "What will you do if you", "borrow my ship? I wonder..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "I'm going to adventure out",
                        "to sea, to find a moving",
                        "island which has brought",
                        "many a legend to this neighboring village."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "The Moving Island?",
                        "I have never seen that really...",
                        "You are young! Hawhawhaw!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "I hope that you find",
                        "the island without fail,",
                        "and that you bring back",
                        "surprising news to us!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "Let it be so!",
                        "First, I should check",
                        "the state of the ship...",
                        "You have some work to do,",
                        "if you want to help me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "She has not floated",
                        "for a long time,",
                        "so we should check",
                        "and repair the parts",
                        "that power her up."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "Preparing the tools for repairs, in",
                        "this place, takes several days...",
                        "if you can possibly prepare them."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "The materials needed are:",
                        "^0000FFStrange Steel Piece 10, Rusty Screw 10, Flexible Tube 5, Jubilee 10^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "When you've prepared all the",
                        "materials, bring them to me. I'll",
                        "repair the ship."
                    ],
                )?;
                ctx.var("mos_whale_edq").set(Val::from(3))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(18104), Val::from(18105)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
            ctx.lines_as(
                "Mr. Ibanoff",
                args![
                    "You want to borrow",
                    "my ship so spontaneously??",
                    "I don't even know you!!",
                    "You are more rude than you look."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mr. Ibanoff",
                args![
                    "I don't know, how you know",
                    "that I have a ship...",
                    "Anyway, I can't lend my ship",
                    "willingly to strangers."
                ],
            )?;
            if ctx.call(Function::IsBeginQuest, vec![Val::from(18102)])?.is_true() {
                ctx.call(Function::ChangeQuest, vec![Val::from(18102), Val::from(18103)])?;
            } else if ctx.call(Function::IsBeginQuest, vec![Val::from(18101)])?.is_true() {
                ctx.call(Function::ChangeQuest, vec![Val::from(18101), Val::from(18103)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("mos_whale_edq").get()? == 3 {
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "Did you bring all the materials?",
                        "If we have all the materials,",
                        "we can prepare for departure!"
                    ],
                )?;
                ctx.next()?;
                if (((ctx.call(Function::CountItem, vec![Val::from(7167)])?.number()? > 9
                    && ctx.call(Function::CountItem, vec![Val::from(7317)])?.number()? > 9)
                    && ctx.call(Function::CountItem, vec![Val::from(7325)])?.number()? > 4)
                    && ctx.call(Function::CountItem, vec![Val::from(7312)])?.number()? > 9)
                {
                    ctx.lines_as("Mr. Ibanoff", args!["Oh! You got the all materials.", "It's enough."])?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7167), Val::from(10)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7317), Val::from(10)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7325), Val::from(5)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7312), Val::from(10)])?;
                    ctx.var("mos_whale_edq").set(Val::from(4))?;
                    ctx.lines_as(
                        "Mr. Ibanoff",
                        args![
                            "Okay, we are at the ready.",
                            "When would you like to depart?",
                            "I'll say one thing...",
                            "It's difficult to keep this",
                            "ship afloat after the sun has set."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Ibanoff",
                        args![
                            "In these neighboring waters,",
                            "there are many raging waves.",
                            "After the sun sets, the tide does",
                            "not get any easier... even for an",
                            "expert seaman."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Ibanoff",
                        args![
                            "Remember, the poor condition of",
                            "this ship also factors when",
                            "attempting to plow through the",
                            "raging waves."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Ibanoff",
                        args!["If you are ready to depart, tell", "me, and we can check the tides."],
                    )?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(18105), Val::from(18106)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args!["These materials are insufficient.", "Perhaps you forgot some of them?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "The materials needed are:",
                        "^0000FFStrange Steel Piece 10, Rusty Screw 10, Flexible Tube 5, Jubilee 10^000000."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("mos_whale_edq").get()? == 4 {
                    ctx.lines_as("Mr. Ibanoff", args!["Oh. Are you ready to depart?", "Good, let's see..."])?;
                    ctx.next()?;
                    if ((((ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 0
                        && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 3)
                        || (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 6
                            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 9))
                        || (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 12
                            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 15))
                        || (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 18
                            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 21))
                    {
                        ctx.lines_as(
                            "Mr. Ibanoff",
                            args![
                                "Hmm. It's not a bad time.",
                                "We should hurry up",
                                "while the tides are low.",
                                "The ship is prepared already",
                                "at a dock nearby, so we can depart",
                                "right away."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("mosk_ship"), Val::from(94), Val::from(110)])?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("mos_whale_edq").get()?.number()? < 15 {
                        ctx.lines_as(
                            "Mr. Ibanoff",
                            args![
                                "That last sailing was tough...",
                                "But, all adventures",
                                "are like that...",
                                "Hahaha!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Ibanoff",
                            args!["If you keep a mind to", "depart again... tell me", "and we can check the tides."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Mr. Ibanoff", args!["Will you depart?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("I'm not ready.:Let's go.")])?) == 1 {
                            ctx.lines_as("Mr. Ibanoff", args!["When you are ready to depart, tell me."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if ((((ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 0
                            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 3)
                            || (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 6
                                && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 9))
                            || (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 12
                                && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 15))
                            || (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 18
                                && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 21))
                        {
                            ctx.var("mos_whale_edq").set(Val::from(4))?;
                            ctx.lines_as(
                                "Mr. Ibanoff",
                                args![
                                    "Hmm. It's not a bad time.",
                                    "We should hurry up",
                                    "while the tides are low.",
                                    "The ship is prepared already",
                                    "at a dock nearby, so we can depart",
                                    "right away."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("mosk_ship"), Val::from(94), Val::from(110)])?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ctx.var("mos_whale_edq").get()? == 15 {
                            ctx.lines_as(
                                "Mr. Ibanoff",
                                args![
                                    "Hey you... You are alive!",
                                    "When you were swept away by the",
                                    "waves, I thought that was the last",
                                    "I would ever see of you!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Mr. Ibanoff", args!["Anyway, where were you all this time?"])?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("Explain the whole story.")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                "Mr. Ibanoff",
                                args!["Oh... that's really marvelous.", "Well I never...", "The island truly exists...?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mr. Ibanoff",
                                args!["What's more...", "The island is not an island?!?", "It's a gigantic whale???"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mr. Ibanoff",
                                args!["And trees grow... and water", "streams... along the whale's", "back!!?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mr. Ibanoff",
                                args![
                                    "That's unbelievable!",
                                    "The most marvelous incident I have",
                                    "ever heard in my lifetime!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mr. Ibanoff",
                                args!["Also, I am surprised at the", "presence of an aged person on Whale", "Island!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mr. Ibanoff",
                                args!["He must be a man strong in", "spirit... with experience and high", "discipline."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mr. Ibanoff",
                                args![
                                    "The story of the island has",
                                    "advanced enough until this part.",
                                    "Why don't you announce your",
                                    "findings to our dearest Csar",
                                    "and receive aid to find Whale",
                                    "Island?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mr. Ibanoff",
                                args![
                                    "Our dear Csar likes it very",
                                    "much to hear stories about",
                                    "valuable things, so if he",
                                    "listens to your story, he may give",
                                    "you a prize..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Mr. Ibanoff",
                                args![
                                    "And if you have any plans to go",
                                    "back to the island, I want to go",
                                    "together with you! Eh? Hahaha!"
                                ],
                            )?;
                            ctx.var("mos_whale_edq").set(Val::from(16))?;
                            if ctx.call(Function::IsBeginQuest, vec![Val::from(18111)])?.is_true() {
                                ctx.call(Function::ChangeQuest, vec![Val::from(18111), Val::from(18112)])?;
                            }
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("mos_whale_edq").get()?.number()? < 18 {
                                ctx.lines_as(
                                    "Mr. Ibanoff",
                                    args!["If our dear Csar listens to this", "kind of story, he would find it a", "pleasure."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Mr. Ibanoff", args!["He might even give you a prize..."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("mos_whale_edq").get()? == 18 {
                                    ctx.lines_as(
                                        "Mr. Ibanoff",
                                        args!["Hm. Our dear Csar requested", "evidence of the whale island?..."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Mr. Ibanoff", args!["That may prove a little", "difficult..."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Mr. Ibanoff",
                                        args!["Do you have a guarantee that you", "can find Whale Island again?"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Mr. Ibanoff",
                                        args![
                                            "I think you won't give up.",
                                            "How about if we depart",
                                            "once more, and I will gladly help",
                                            "you find Whale Island!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    if Val::from(runtime::select_values(
                                        ctx,
                                        &[Val::from("Try again to find Whale Island.:Give up.")],
                                    )?) == 1
                                    {
                                        ctx.lines_as(
                                            "Mr. Ibanoff",
                                            args![
                                                "That does it! I expected it.",
                                                "I knew you weren't the type to give up an adventure!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Mr. Ibanoff",
                                            args!["Again, let's get the ship ready and sail before the sun sets!"],
                                        )?;
                                        ctx.var("mos_whale_edq").set(Val::from(19))?;
                                        if ctx.call(Function::IsBeginQuest, vec![Val::from(18113)])?.is_true() {
                                            ctx.call(Function::ChangeQuest, vec![Val::from(18113), Val::from(18114)])?;
                                        }
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines_as(
                                        "Mr. Ibanoff",
                                        args![
                                            "I see...",
                                            "Even though you like adventure,",
                                            "it might be impossible to find",
                                            "Whale Island again..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Mr. Ibanoff",
                                        args![
                                            "If you change your mind,",
                                            "anytime, come tell me.",
                                            "Frankly, I would love to see you",
                                            "prove the existence of Whale Island",
                                            "to our dear Csar."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("mos_whale_edq").get()? == 19 {
                                        ctx.lines_as("Mr. Ibanoff", args!["Oh! Are you ready to depart?"])?;
                                        ctx.next()?;
                                        if Val::from(runtime::select_values(ctx, &[Val::from("I'm not ready.:Let's go.")])?) == 1 {
                                            ctx.lines_as("Mr. Ibanoff", args!["When you are ready to depart, tell me."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        if ((((ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 0
                                            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 3)
                                            || (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 6
                                                && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 9))
                                            || (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 12
                                                && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 15))
                                            || (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 18
                                                && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 21))
                                        {
                                            ctx.lines_as(
                                                "Mr. Ibanoff",
                                                args![
                                                    "Hmm. It's not a bad time.",
                                                    "We should hurry up",
                                                    "while the tides are low.",
                                                    "The ship is prepared already",
                                                    "at a dock nearby, so we can depart",
                                                    "right away."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            ctx.call(Function::Warp, vec![Val::from("mosk_ship"), Val::from(94), Val::from(110)])?;
                                            return Err(Stop::End);
                                        }
                                    } else {
                                        if ctx.var("mos_whale_edq").get()?.number()? < 32 {
                                            ctx.lines_as(
                                                "Mr. Ibanoff",
                                                args![
                                                    "What a surprise!!",
                                                    "Where have you been??",
                                                    "I was worried that you had",
                                                    "disappeared forever!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Mr. Ibanoff",
                                                args!["I don't care... Wherever you have", "been, it's very good to see you", "again."],
                                            )?;
                                            ctx.var("mos_whale_edq").set(Val::from(19))?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Mr. Ibanoff",
                                                args!["I can guess you'd like to go to", "Whale Island once again..."],
                                            )?;
                                            ctx.next()?;
                                            if ((((ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 0
                                                && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 3)
                                                || (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 6
                                                    && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 9))
                                                || (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 12
                                                    && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 15))
                                                || (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 18
                                                    && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 21))
                                            {
                                                ctx.lines_as(
                                                    "Mr. Ibanoff",
                                                    args![
                                                        "Hmm. It's not a bad time.",
                                                        "We should hurry up",
                                                        "while the tides are low.",
                                                        "The ship is prepared already",
                                                        "at a dock nearby, so we can depart",
                                                        "right away."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Warp, vec![Val::from("mosk_ship"), Val::from(94), Val::from(110)])?;
                                                return Err(Stop::End);
                                            }
                                        } else {
                                            if ctx.var("mos_whale_edq").get()? == 32 {
                                                ctx.lines_as("Mr. Ibanoff", args!["Oh. You came back!", "So, how did you do?"])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Mr. Ibanoff",
                                                    args![
                                                        "You say...",
                                                        "If you bring the materials, the old",
                                                        "man makes the instrument..."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Mr. Ibanoff",
                                                    args!["And he promised that he won't move", "the island until you go back", "again..."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Mr. Ibanoff",
                                                    args![
                                                        "I see. I remember the exact",
                                                        "directions. Let's leave",
                                                        "immediately, as soon as you are",
                                                        "ready."
                                                    ],
                                                )?;
                                                ctx.var("mos_whale_edq").set(Val::from(33))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                if ctx.var("mos_whale_edq").get()?.number()? < 39 {
                                                    ctx.lines_as("Mr. Ibanoff", args!["Oh. Did you get all the", "materials you needed?"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Ibanoff", args!["If so, do you want to go now?"])?;
                                                    ctx.next()?;
                                                    if Val::from(runtime::select_values(ctx, &[Val::from("Not yet.:Let's go.")])?) == 1 {
                                                        ctx.lines_as("Mr. Ibanoff", args!["I see. I will wait."])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    ctx.lines_as("Mr. Ibanoff", args!["Ok, let's go!"])?;
                                                    ctx.close_window()?;
                                                    ctx.call(Function::Warp, vec![Val::from("mosk_fild01"), Val::from(93), Val::from(94)])?;
                                                    return Err(Stop::End);
                                                } else if ctx.var("mos_whale_edq").get()?.number()? < 41 {
                                                    ctx.lines_as(
                                                        "Mr. Ibanoff",
                                                        args![
                                                            "Finally, everything has ended",
                                                            "successfully. With this, I add 1",
                                                            "more page to my... no, OUR adventure story!"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Mr. Ibanoff",
                                                        args![
                                                            "Someday, I want to have another",
                                                            "chance to have another fantastic",
                                                            "adventure with a person like you!"
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if ctx.var("mos_whale_edq").get()? == 41 {
                                                    ctx.lines_as(
                                                        "Mr. Ibanoff",
                                                        args![
                                                            "I heard that you played a great",
                                                            "performance, with the instrument",
                                                            "from Whale Island, at the Imperial",
                                                            "Palace. Eh? You must have quite the talent!"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Mr. Ibanoff",
                                                        args![
                                                            "Finally, everything has ended",
                                                            "successfully. With this, I add 1",
                                                            "more page to my... no, OUR adventure story!"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Mr. Ibanoff",
                                                        args![
                                                            "Someday, I want to have another",
                                                            "chance to have another fantastic",
                                                            "adventure with a person like you!"
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if (ctx.var("mos_whale_edq").get()?.number()? > 90
                                                    && ctx.var("mos_whale_edq").get()?.number()? < 100)
                                                {
                                                    ctx.lines_as(
                                                        "Mr. Ibanoff",
                                                        args![
                                                            "What's going on?",
                                                            "You should ride a ship now? Let's ready to leave hurry up."
                                                        ],
                                                    )?;
                                                    ctx.var("mos_whale_edq").set(Val::from(4))?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if (ctx.var("mos_whale_edq").get()?.number()? > 200
                                                    && ctx.var("mos_whale_edq").get()?.number()? < 300)
                                                {
                                                    ctx.lines_as(
                                                        "Mr. Ibanoff",
                                                        args![
                                                            "What's going on?",
                                                            "You should ride a ship now? Let's ready to leave hurry up."
                                                        ],
                                                    )?;
                                                    ctx.var("mos_whale_edq").set(Val::from(19))?;
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
            }
        }
    }
    ctx.lines_as(
        "Mr. Ibanoff",
        args![
            "Gee. The tide is too high.",
            "It's too dangerous to depart",
            "right now because of the waves",
            "getting too rugged."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mr. Ibanoff",
        args![
            "When the sea calmes down a bit,",
            "you can come back. We can't depart",
            "right now."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mr_ibanoff_npc(ctx: &Ctx) -> Script {
    mr_ibanoff_npc_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MrIbanoffNpc2Step {
    Start,
    SD1,
    SD2,
}

fn mr_ibanoff_npc2_run(ctx: &Ctx, mut step: MrIbanoffNpc2Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_d_s: Vec<Val> = Vec::new();
    let mut l_r = Val::from(0);
    let mut l_ship2 = Val::from(0);
    'machine: loop {
        match step {
            MrIbanoffNpc2Step::Start => {
                if ctx.var("mos_whale_edq").get()? == 4 {
                    ctx.lines_as(
                        "Mr. Ibanoff",
                        args!["What a time for sailing!", "The wind of the sea is so cool."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["You didn't have to accompany me", "through this dangerous sailing..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Ibanoff",
                        args![
                            "Such as I am, I know the",
                            "sea a lot better than you!",
                            "And this ship has been my",
                            "friend through all my life!",
                            "I wouldn't abandon a friend.",
                            "Would you? ~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Mr. Ibanoff", args!["I will order you where to go, by", "watching the seas."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Ibanoff",
                        args!["You just adjust the direction of", "the rudder by following my orders."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Mr. Ibanoff", args!["At first, hold the rudder to go", "forward, to the east."])?;
                    ctx.var("mos_whale_edq").set(Val::from(5))?;
                    if ctx.call(Function::IsBeginQuest, vec![Val::from(18106)])?.is_true() {
                        ctx.call(Function::ChangeQuest, vec![Val::from(18106), Val::from(18107)])?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("mos_whale_edq").get()? == 5 {
                        ctx.lines_as(
                            "Mr. Ibanoff",
                            args!["Keep the direction by holding the", "rudder to move forward to the", "East."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if (ctx.var("mos_whale_edq").get()? == 6 || ctx.var("mos_whale_edq").get()? == 21) {
                            l_r = (if ctx.var("mos_whale_edq").get()? == 21 {
                                Val::from(3)
                            } else {
                                Val::from(4)
                            });
                            if ctx.call(Function::Rand, vec![Val::from(1), l_r.clone()])? == 3 {
                                ctx.lines_as(
                                    "Mr. Ibanoff",
                                    args![
                                        "Hm. The sea currents have changed.",
                                        "Adjust the rudder forward to the",
                                        "North, to follow the currents."
                                    ],
                                )?;
                                ctx.var("mos_whale_edq").set((ctx.var("mos_whale_edq").get()? + Val::from(1)))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Mr. Ibanoff",
                                args![
                                    "Nothing yet... Nothing has changed",
                                    "in the flow of water. There is",
                                    "nothing to note..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Mr. Ibanoff", args!["We had better keep sailing on this", "heading for now."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("$@mos1_edq").get()?.is_true() {
                                ctx.lines_as("Mr. Ibanoff", args!["We must first repulse the", "monsters!"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if (ctx.var("mos_whale_edq").get()? == 7 || ctx.var("mos_whale_edq").get()? == 22) {
                                    ctx.lines_as(
                                        "Mr. Ibanoff",
                                        args![
                                            "Hey!",
                                            "Did you not hear me?",
                                            "Change the rudder forward to the",
                                            "North, and follow the currents!"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if (ctx.var("mos_whale_edq").get()? == 8 || ctx.var("mos_whale_edq").get()? == 10) {
                                        mr_ibanoff_npc2_run(
                                            ctx,
                                            MrIbanoffNpc2Step::SD1,
                                            vec![Val::from(3), Val::from(91), Val::from(92), Val::from(93), Val::from(94)],
                                        )?;
                                    } else {
                                        if (ctx.var("mos_whale_edq").get()?.number()? >= 91
                                            && ctx.var("mos_whale_edq").get()?.number()? <= 94)
                                        {
                                            mr_ibanoff_npc2_run(
                                                ctx,
                                                MrIbanoffNpc2Step::SD2,
                                                vec![(ctx.var("mos_whale_edq").get()?.try_sub(Val::from(91))?)],
                                            )?;
                                        } else {
                                            if ctx.var("mos_whale_edq").get()? == 11 {
                                                ctx.lines_as(
                                                    "Mr. Ibanoff",
                                                    args!["Look... Beyond the sea!", "Do you see something moving", "mysteriously?"],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Mr. Ibanoff",
                                                    args![
                                                        "Heheh... What is...",
                                                        "that... Hey! You...",
                                                        "Go around the deck to look more",
                                                        "carefully! Go!"
                                                    ],
                                                )?;
                                                ctx.var("mos_whale_edq").set(Val::from(12))?;
                                                if ctx.call(Function::IsBeginQuest, vec![Val::from(18107)])?.is_true() {
                                                    ctx.call(Function::ChangeQuest, vec![Val::from(18107), Val::from(18108)])?;
                                                }
                                                ctx.call(Function::DoNpcEvent, vec![Val::from("#findship::OnEnable")])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                if ctx.var("mos_whale_edq").get()? == 12 {
                                                    ctx.lines_as(
                                                        "Mr. Ibanoff",
                                                        args![
                                                            "What are you doing...? Go around",
                                                            "the deck to look more carefully!",
                                                            "Go!"
                                                        ],
                                                    )?;
                                                    ctx.call(Function::DoNpcEvent, vec![Val::from("#findship::OnEnable")])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    if ctx.var("mos_whale_edq").get()? == 19 {
                                                        ctx.lines_as(
                                                            "Mr. Ibanoff",
                                                            args![
                                                                "This time, I hope to find that",
                                                                "whale island again without any",
                                                                "incidents..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Mr. Ibanoff", args!["The method is the same as before."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Mr. Ibanoff",
                                                            args!["I will order you where to go, by", "watching the seas."],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Mr. Ibanoff",
                                                            args!["You just adjust the direction of", "the rudder by following my orders."],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Mr. Ibanoff",
                                                            args!["At first, hold the rudder to go", "forward, to the east."],
                                                        )?;
                                                        ctx.var("mos_whale_edq").set(Val::from(20))?;
                                                        if ctx.call(Function::IsBeginQuest, vec![Val::from(18114)])?.is_true() {
                                                            ctx.call(Function::ChangeQuest, vec![Val::from(18114), Val::from(18115)])?;
                                                        }
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else if ctx.var("mos_whale_edq").get()? == 20 {
                                                        ctx.lines_as(
                                                            "Mr. Ibanoff",
                                                            args![
                                                                "Keep the direction by holding the",
                                                                "rudder to move forward to the",
                                                                "East."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else if (ctx.var("mos_whale_edq").get()? == 23
                                                        || ctx.var("mos_whale_edq").get()? == 25)
                                                    {
                                                        mr_ibanoff_npc2_run(
                                                            ctx,
                                                            MrIbanoffNpc2Step::SD1,
                                                            vec![
                                                                Val::from(5),
                                                                Val::from(241),
                                                                Val::from(242),
                                                                Val::from(243),
                                                                Val::from(244),
                                                            ],
                                                        )?;
                                                    } else if (ctx.var("mos_whale_edq").get()?.number()? >= 241
                                                        && ctx.var("mos_whale_edq").get()?.number()? <= 244)
                                                    {
                                                        mr_ibanoff_npc2_run(
                                                            ctx,
                                                            MrIbanoffNpc2Step::SD2,
                                                            vec![(ctx.var("mos_whale_edq").get()?.try_sub(Val::from(241))?)],
                                                        )?;
                                                    } else if ctx.var("mos_whale_edq").get()? == 26 {
                                                        ctx.lines_as(
                                                            "Mr. Ibanoff",
                                                            args!["Look there!", "There is a moving island!", "We have done well!"],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Mr. Ibanoff",
                                                            args![
                                                                "Oh...my...",
                                                                "It really is there...",
                                                                "A whale! It's unbelievable!",
                                                                "It is as true as that..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Mr. Ibanoff",
                                                            args![
                                                                "I feel I can go anywhere with you!",
                                                                "Even through impossible things!",
                                                                "Haha! You are a friend who brings",
                                                                "good luck."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Mr. Ibanoff",
                                                            args!["Okay, now that you have landed on", "the island, I have to go back..."],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Mr. Ibanoff",
                                                            args![
                                                                "If another wave crashes against",
                                                                "this boat, I feel she might break",
                                                                "into pieces!"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Mr. Ibanoff",
                                                            args![
                                                                "Good luck. I hope you",
                                                                "can return without problems.",
                                                                "I will pray for you.",
                                                                "See you next time."
                                                            ],
                                                        )?;
                                                        ctx.var("mos_whale_edq").set(Val::from(30))?;
                                                        ctx.close_window()?;
                                                        ctx.call(
                                                            Function::Warp,
                                                            vec![Val::from("mosk_fild01"), Val::from(93), Val::from(94)],
                                                        )?;
                                                        if ctx.call(Function::IsBeginQuest, vec![Val::from(18115)])?.is_true() {
                                                            ctx.call(Function::ChangeQuest, vec![Val::from(18115), Val::from(18116)])?;
                                                        }
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
                }
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "What? How did you get on",
                        "this ship?? You... No.",
                        "I'll forgive you this once, but go",
                        "back now."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("moscovia"), Val::from(163), Val::from(55)])?;
                return Err(Stop::End);
            }
            MrIbanoffNpc2Step::SD1 => {
                if ctx.call(Function::Rand, vec![Val::from(1), runtime::arg(&args, 0, Val::from(0))])? == 3 {
                    l_ship2 = ctx.call(Function::Rand, vec![Val::from(4)])?;
                    let base = Val::from(0).number()?;
                    runtime::local_set(&mut l_d_s, &Val::from(base + 0), Val::from("East"), true);
                    runtime::local_set(&mut l_d_s, &Val::from(base + 1), Val::from("West"), true);
                    runtime::local_set(&mut l_d_s, &Val::from(base + 2), Val::from("South"), true);
                    runtime::local_set(&mut l_d_s, &Val::from(base + 3), Val::from("North"), true);
                    ctx.lines_as(
                        "Mr. Ibanoff",
                        args![
                            "Hm. The sea currents have changed.",
                            "Adjust the rudder forward to the",
                            (runtime::local_get(&l_d_s, &l_ship2.clone(), true) + Val::from(", to follow the currents."))
                        ],
                    )?;
                    ctx.var("mos_whale_edq")
                        .set(runtime::arg(&args, (l_ship2.clone() + Val::from(1)).number()?, Val::from(0)))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "Nothing yet... Nothing has changed",
                        "in the flow of water. There is",
                        "nothing to note..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Mr. Ibanoff", args!["We had better keep sailing on this heading for now."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            MrIbanoffNpc2Step::SD2 => {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_d_s, &Val::from(base + 0), Val::from("East"), true);
                runtime::local_set(&mut l_d_s, &Val::from(base + 1), Val::from("West"), true);
                runtime::local_set(&mut l_d_s, &Val::from(base + 2), Val::from("South"), true);
                runtime::local_set(&mut l_d_s, &Val::from(base + 3), Val::from("North"), true);
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "Hey!",
                        "Did you not hear me?",
                        "Change the rudder forward to the",
                        (runtime::local_get(&l_d_s, &runtime::arg(&args, 0, Val::from(0)), true) + Val::from(", and follow the currents!"))
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mr_ibanoff_npc2(ctx: &Ctx) -> Script {
    mr_ibanoff_npc2_run(ctx, MrIbanoffNpc2Step::Start, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RudderShipStep {
    Start,
    SRud1,
}

fn rudder_ship_run(ctx: &Ctx, mut step: RudderShipStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_d_s: Vec<Val> = Vec::new();
    let mut l_direction = Val::from(0);
    let mut l_j = Val::from(0);
    let mut l_monster_setting = Val::from(0);
    let mut l_r = Val::from(0);
    'machine: loop {
        match step {
            RudderShipStep::Start => {
                if ctx.var("mos_whale_edq").get()? == 5 {
                    rudder_ship_run(ctx, RudderShipStep::SRud1, vec![Val::from(1), Val::from(0)])?;
                } else {
                    if ctx.var("mos_whale_edq").get()? == 4 {
                        ctx.lines_as(
                            "Mr. Ibanoff",
                            args!["You don't have to adjust the rudder for now.", "Wait for my direction."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("mos_whale_edq").get()? == 6 {
                            ctx.lines_as(
                                "Mr. Ibanoff",
                                args![
                                    "Do not yet adjust the rudder.",
                                    "Only when I order you to,",
                                    "you adjust the rudder."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("$@mos1_edq").get()?.number()? > 0 {
                                ctx.lines_as("Mr. Ibanoff", args!["We should make sure to kill any", "monsters onboard."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("mos_whale_edq").get()? == 7 {
                                    rudder_ship_run(ctx, RudderShipStep::SRud1, vec![Val::from(4), Val::from(1)])?;
                                } else {
                                    if ((ctx.var("mos_whale_edq").get()? == 8 || ctx.var("mos_whale_edq").get()? == 21)
                                        || ctx.var("mos_whale_edq").get()? == 23)
                                    {
                                        ctx.lines_as(
                                            "Mr. Ibanoff",
                                            args![
                                                "Do not yet adjust the rudder.",
                                                "Only when I order you to,",
                                                "you adjust the rudder."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if (ctx.var("mos_whale_edq").get()?.number()? >= 91
                                            && ctx.var("mos_whale_edq").get()?.number()? <= 94)
                                        {
                                            rudder_ship_run(
                                                ctx,
                                                RudderShipStep::SRud1,
                                                vec![(ctx.var("mos_whale_edq").get()?.try_sub(Val::from(90))?), Val::from(2)],
                                            )?;
                                        } else {
                                            if (ctx.var("mos_whale_edq").get()?.number()? > 10
                                                && ctx.var("mos_whale_edq").get()?.number()? < 13)
                                            {
                                                ctx.lines_as(
                                                    "Mr. Ibanoff",
                                                    args!["Look... Beyond the sea!", "Do you see something moving", "mysteriously?"],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Mr. Ibanoff",
                                                    args![
                                                        "Heheh... What is...",
                                                        "that... Hey! You...",
                                                        "Go around the deck to look more",
                                                        "carefully! Go!"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                if ctx.var("mos_whale_edq").get()? == 20 {
                                                    rudder_ship_run(ctx, RudderShipStep::SRud1, vec![Val::from(1), Val::from(0)])?;
                                                } else {
                                                    if ctx.var("mos_whale_edq").get()? == 22 {
                                                        rudder_ship_run(ctx, RudderShipStep::SRud1, vec![Val::from(4), Val::from(1)])?;
                                                    } else if (ctx.var("mos_whale_edq").get()?.number()? >= 241
                                                        && ctx.var("mos_whale_edq").get()?.number()? <= 243)
                                                    {
                                                        rudder_ship_run(
                                                            ctx,
                                                            RudderShipStep::SRud1,
                                                            vec![(ctx.var("mos_whale_edq").get()?.try_sub(Val::from(240))?), Val::from(2)],
                                                        )?;
                                                    } else if ctx.var("mos_whale_edq").get()? == 244 {
                                                        rudder_ship_run(ctx, RudderShipStep::SRud1, vec![Val::from(4), Val::from(1)])?;
                                                    } else if ctx.var("mos_whale_edq").get()? == 25 {
                                                        ctx.lines_as(
                                                            "Mr. Ibanoff",
                                                            args!["You can adjust the rudder,", "under my direction."],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else if ctx.var("mos_whale_edq").get()? == 26 {
                                                        ctx.lines_as(
                                                            "Mr. Ibanoff",
                                                            args![
                                                                "Hey! Listen to what I am saying.",
                                                                "How come you go there without my",
                                                                "permission..."
                                                            ],
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
                }
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        "What? How did you get on",
                        "this ship?? You... No.",
                        "I'll forgive you this once, but go",
                        "back now."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("moscovia"), Val::from(162), Val::from(56)])?;
                return Err(Stop::End);
            }
            RudderShipStep::SRud1 => {
                l_direction = runtime::arg(&args, 0, Val::from(0));
                l_monster_setting = runtime::arg(&args, 1, Val::from(0));
                ctx.mes("Which way?")?;
                ctx.next()?;
                let base = Val::from(1).number()?;
                runtime::local_set(&mut l_d_s, &Val::from(base + 0), Val::from("East"), true);
                runtime::local_set(&mut l_d_s, &Val::from(base + 1), Val::from("West"), true);
                runtime::local_set(&mut l_d_s, &Val::from(base + 2), Val::from("South"), true);
                runtime::local_set(&mut l_d_s, &Val::from(base + 3), Val::from("North"), true);
                l_j = (Val::from(runtime::select_values(ctx, &[runtime::implode(&l_d_s, &Val::from(":"))?])?).try_sub(Val::from(1))?);
                if l_j.clone().loosely_equals(&l_direction.clone()) {
                    ctx.lines_as(
                        "Mr. Ibanoff",
                        args![
                            "Good. Firstly, we should",
                            ((Val::from("keep heading ") + runtime::strtolower(&runtime::local_get(&l_d_s, &l_direction.clone(), true)))
                                + Val::from(" this way.")),
                            "When I give the order,",
                            "please adjust the rudder again."
                        ],
                    )?;
                    if l_monster_setting.clone() == 2 {
                        l_r = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                    }
                    if (l_monster_setting.clone() == 2 && l_r.clone() == 3) {
                        ctx.next()?;
                        ctx.lines_as("Mr. Ibanoff", args!["Wait! Something has appeared in front of us..."])?;
                        ctx.next()?;
                        ctx.lines_as("Mr. Ibanoff", args!["Monsters!!!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Ibanoff",
                            args![
                                "These monsters are like none I have",
                                "ever encountered! Be careful! We",
                                "must repulse these monsters!"
                            ],
                        )?;
                        ctx.var("$@mos1_edq").set((ctx.var("$@mos1_edq").get()? + Val::from(1)))?;
                        if ctx.var("mos_whale_edq").get()?.number()? >= 241 {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Baehideun4#ship::OnEnable")])?;
                        } else {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Baehideun3#ship::OnEnable")])?;
                        }
                    }
                    ctx.var("mos_whale_edq").set(
                        (if l_monster_setting.clone() == 2 {
                            (if ctx.var("mos_whale_edq").get()?.number()? >= 241 {
                                (if l_r.clone() != 3 { Val::from(26) } else { Val::from(25) })
                            } else {
                                (if l_r.clone() != 3 { Val::from(10) } else { Val::from(11) })
                            })
                        } else {
                            (ctx.var("mos_whale_edq").get()? + Val::from(1))
                        }),
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Mr. Ibanoff",
                    args![
                        ((Val::from("I said that we should go ") + runtime::local_get(&l_d_s, &l_direction.clone(), true))
                            + Val::from("!")),
                        "You should sail in the right",
                        ((Val::from("direction! To the ") + runtime::strtolower(&runtime::local_get(&l_d_s, &l_direction.clone(), true)))
                            + Val::from("!"))
                    ],
                )?;
                if l_monster_setting.clone().number()? >= 1 {
                    l_r = (if l_monster_setting.clone() == 1 {
                        Val::from(5)
                    } else {
                        Val::from(4)
                    });
                    if ctx.call(Function::Rand, vec![Val::from(1), l_r.clone()])?.number()? <= 2 {
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Ibanoff",
                            args!["Oh no! Monsters have appeared!", "Let's get ready to fight! Hurry!"],
                        )?;
                        ctx.var("$@mos1_edq").set((ctx.var("$@mos1_edq").get()? + Val::from(1)))?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Baehideun1#ship::OnEnable")])?;
                    }
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn rudder_ship(ctx: &Ctx) -> Script {
    rudder_ship_run(ctx, RudderShipStep::Start, Vec::new()).map(|_| ())
}
