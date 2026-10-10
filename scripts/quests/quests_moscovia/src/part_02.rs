use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum AgedStrangerNpcStep {
    Start,
    SAS1,
    SAS2,
    SAS3,
}

fn aged_stranger_npc_run(ctx: &Ctx, mut step: AgedStrangerNpcStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AgedStrangerNpcStep::Start => {
                if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
                    ctx.lines_as(
                        "Aged Stranger",
                        args!["You're carrying too many items.", "Please come after using kafra service."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("mos_whale_edq").get()? == 13 {
                    ctx.lines_as("Aged Stranger", args!["Oh, your awake.", "How do you feel? Are you ok??"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Hm... where..is...", "Eck! What happened...?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aged Stranger",
                        args![
                            "Well... You were a bit",
                            "of a surprise! I haven't",
                            "talked for a long time...",
                            "with a person who lives on dry",
                            "land."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Where am I?", "Who are you old man?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aged Stranger",
                        args![
                            "This is... well...",
                            "What can I say...",
                            "This is called a moving island, by",
                            "most people."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Aged Stranger", args!["This is a small island that only I inhabit."])?;
                    ctx.next()?;
                    ctx.lines_as("Aged Stranger", args!["I call it...", "Whale Island."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "This place is very famously known",
                            "as The Moving Island! But... This",
                            "island is... Maybe..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aged Stranger",
                        args![
                            "Now do you get it? Hahaha!",
                            "Right. We are standing on",
                            "the back of a gigantic whale!",
                            "That's why I call it Whale Island!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "How...? How is it possible",
                            "that the water flows in streams",
                            "here? And trees grow! On the",
                            "whale's back!?!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aged Stranger",
                        args![
                            "Heheheh. This is an island",
                            "of legend. Right now you are",
                            "experiencing greatness! Hahahaha!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Are you a real person?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aged Stranger",
                        args![
                            "Heheh, as you can see,",
                            "I am just an aged man.",
                            "I've simply spent my lifetime here",
                            "leisurely, by breaking off",
                            "relations with any unworldly life."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aged Stranger",
                        args![
                            "Okay, now you get yourself",
                            "together. I will send you back to",
                            "dry land. If you don't go back",
                            "soon, some people may worry."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::SoundEffect, vec![Val::from("mos_gusli1.wav"), Val::from(0)])?;
                    ctx.lines(args![
                        "-The island starts to move slowly",
                        "when the old man plays an",
                        "unfamiliar instrument...-"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aged Stranger",
                        args![
                            "In a few minutes, you will go back",
                            "to the land from whence you came.",
                            "You can relax."
                        ],
                    )?;
                    ctx.next()?;
                    aged_stranger_npc_run(ctx, AgedStrangerNpcStep::SAS3, vec![])?;
                    ctx.var("mos_whale_edq").set(Val::from(14))?;
                    if ctx.call(Function::IsBeginQuest, vec![Val::from(18109)])?.is_true() {
                        ctx.call(Function::ChangeQuest, vec![Val::from(18109), Val::from(18110)])?;
                    }
                    ctx.close_window()?;
                    ctx.call(Function::Sleep, vec![Val::from(20000)])?;
                    ctx.var("mos_whale_edq").set(Val::from(15))?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("mos_whale_edq").get()? == 14 {
                        aged_stranger_npc_run(ctx, AgedStrangerNpcStep::SAS3, vec![])?;
                        ctx.var("mos_whale_edq").set(Val::from(15))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("mos_whale_edq").get()? == 15 {
                            ctx.lines_as(
                                "Aged Stranger",
                                args![
                                    "Hm... Almost there...",
                                    "Young man, you will arrive back on",
                                    "the mainland in a few minutes. Go carefully."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Old man, if I want to come back", "here again, how can I do it?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Aged Stranger",
                                args![
                                    "Hehe... Do you want to come here",
                                    "again? Here? Where I live alone? In",
                                    "this small island? You are",
                                    "funny..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Aged Stranger",
                                args!["If we are preordained to meet,", "we will meet here again,", "won't we? Heheheh..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Aged Stranger",
                                args![
                                    "It's time to... well,",
                                    "go back safely. Someday, if you",
                                    "want to come again to meet me, you",
                                    "can endure the same seas as last",
                                    "time. I won't stop you. Hehehe."
                                ],
                            )?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(18110), Val::from(18111)])?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("moscovia"), Val::from(163), Val::from(54)])?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("mos_whale_edq").get()? == 30 {
                                ctx.lines_as(
                                    "Aged Stranger",
                                    args!["Hahaha... I didn't know that I", "would meet you again! How did you", "return?"],
                                )?;
                                ctx.next()?;
                                let choice = runtime::select_values(ctx, &[Val::from("Explain the whole story.")])?;
                                ctx.var("@menu").set(choice)?;
                                ctx.lines_as(
                                    "Aged Stranger",
                                    args![
                                        "Hm... just like that?",
                                        "Whoever hasn't come here directly",
                                        "would expect you to return like",
                                        "that. Well."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Aged Stranger",
                                    args![
                                        "I just wanted to spend the rest of",
                                        "my life here, leisurely... I'm",
                                        "interested in the world... but I'm",
                                        "no match for it..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Aged Stranger",
                                    args![
                                        "Do not look sorry for me.",
                                        "When you went back I didn't ask you",
                                        "not to say anything about this",
                                        "island to others."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Aged Stranger",
                                    args![
                                        "If I had decided to hide where",
                                        "nobody can find... Well, you",
                                        "needn't worry too much about that",
                                        "now."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Aged Stranger", args!["Anyway, are you troubled?", "What are you going to do?"])?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(ctx, &[Val::from("Think more.:Don't you have a way?")])?) == 1 {
                                    ctx.lines_as(
                                        "Aged Stranger",
                                        args!["Do that. Think carefully, the", "answer is to be found...", "Hoohoohoo!"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Aged Stranger",
                                    args!["To bring the Csar here and show him", "everything?!? It's difficult..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Aged Stranger",
                                    args![
                                        "An evidence of whale island...",
                                        "That's all you need if only it",
                                        "existed in this island..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::SoundEffect, vec![Val::from("mos_gusli1.wav"), Val::from(0)])?;
                                ctx.lines(args![
                                    "-The aged man closes his eyes and",
                                    "starts to play his instrument, as",
                                    "he falls in thought.-"
                                ])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Old man, could you make one of",
                                        "those instruments for me? That is a",
                                        "marvelous thing that I've seen only",
                                        "here."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Aged Stranger",
                                    args![
                                        "Oh... That's a good idea.",
                                        "There isn't an instrument like this",
                                        "in all Moscovia! Absolutely..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Aged Stranger",
                                    args!["Hm... If you grant my request, I", "will make this instrument for you."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["What is your request?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Aged Stranger",
                                    args![
                                        "Just in time. I have always used",
                                        "this instrument which is already",
                                        "over 50 years old. The body is so",
                                        "worn. It's not surprising that it's",
                                        "falling apart..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Aged Stranger",
                                    args![
                                        "It is made with strings that only",
                                        "exist in this place, but you can",
                                        "get the other materials for it in",
                                        "the mainland."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Aged Stranger", args!["If you bring the materials, I can", "make a new one."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Aged Stranger",
                                    args!["The materials are ^0000FFLog 30, Tough Vines 20, Antelope Horn 20, Sea-otter Fur 10^000000."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Aged Stranger",
                                    args![
                                        "I won't move the island, so you can",
                                        "find it when you come back with the",
                                        "materials."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Aged Stranger",
                                    args!["Tell me whenever you're ready, and", "I'll send you back to the", "mainland."],
                                )?;
                                if ctx.call(Function::IsBeginQuest, vec![Val::from(18116)])?.is_true() {
                                    ctx.call(Function::ChangeQuest, vec![Val::from(18116), Val::from(18117)])?;
                                }
                                ctx.var("mos_whale_edq").set(Val::from(31))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("mos_whale_edq").get()? == 31 {
                                    ctx.lines_as("Aged Stranger", args!["Okay, are you ready to go back to the mainland?"])?;
                                    ctx.next()?;
                                    if Val::from(runtime::select_values(
                                        ctx,
                                        &[Val::from("What are the materials?:I am ready.")],
                                    )?) == 1
                                    {
                                        ctx.lines_as("Aged Stranger", args!["You should bring these materials:"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Aged Stranger",
                                            args![
                                                "You should bring these materials:",
                                                "^0000FFLog 30, Tough Vines 20, Antelope Horn 20, Sea-otter Fur 10^000000."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines_as("Aged Stranger", args!["I see. If so, let's go."])?;
                                    ctx.next()?;
                                    ctx.var("mos_whale_edq").set(Val::from(32))?;
                                    aged_stranger_npc_run(ctx, AgedStrangerNpcStep::SAS3, vec![])?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Warp, vec![Val::from("moscovia"), Val::from(162), Val::from(56)])?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("mos_whale_edq").get()? == 33 {
                                        ctx.lines_as("Aged Stranger", args!["Oh... Did you get all the", "materials?"])?;
                                        ctx.next()?;
                                        if (((ctx.call(Function::CountItem, vec![Val::from(7201)])?.number()? > 29
                                            && ctx.call(Function::CountItem, vec![Val::from(7197)])?.number()? > 19)
                                            && ctx.call(Function::CountItem, vec![Val::from(7106)])?.number()? > 19)
                                            && ctx.call(Function::CountItem, vec![Val::from(7065)])?.number()? > 9)
                                        {
                                            ctx.lines_as(
                                                "Aged Stranger",
                                                args!["You found the right materials.", "Okay, I will start to make the", "instrument."],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::DelItem, vec![Val::from(7201), Val::from(30)])?;
                                            ctx.call(Function::DelItem, vec![Val::from(7197), Val::from(20)])?;
                                            ctx.call(Function::DelItem, vec![Val::from(7106), Val::from(20)])?;
                                            ctx.call(Function::DelItem, vec![Val::from(7065), Val::from(10)])?;
                                            ctx.var("mos_whale_edq").set(Val::from(34))?;
                                            if ctx.call(Function::IsBeginQuest, vec![Val::from(18117)])?.is_true() {
                                                ctx.call(Function::ChangeQuest, vec![Val::from(18117), Val::from(18118)])?;
                                            }
                                            ctx.lines_as(
                                                "Aged Stranger",
                                                args!["Wait for a moment until I make the", "instrument successfully."],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as(
                                            "Aged Stranger",
                                            args![
                                                "Ugh... We need more materials...",
                                                "We need ^0000FFLog 30, Tough Vines 20, Antelope Horn 20, Sea-otter Fur 10^000000."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Aged Stranger",
                                            args!["What will you do? Will you go back", "to the mainland to find the", "materials?"],
                                        )?;
                                        ctx.next()?;
                                        if Val::from(runtime::select_values(
                                            ctx,
                                            &[Val::from("What are the materials?:I am ready.")],
                                        )?) == 1
                                        {
                                            ctx.lines_as("Aged Stranger", args!["You should bring these materials:"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Aged Stranger",
                                                args![
                                                    "You should bring these materials:",
                                                    "^0000FFLog 30, Tough Vines 20, Antelope Horn 20, Sea-otter Fur 10^000000."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as("Aged Stranger", args!["I see. If so, let's go."])?;
                                        ctx.next()?;
                                        aged_stranger_npc_run(ctx, AgedStrangerNpcStep::SAS3, vec![])?;
                                        ctx.close_window()?;
                                        ctx.call(Function::Warp, vec![Val::from("moscovia"), Val::from(162), Val::from(56)])?;
                                        return Err(Stop::End);
                                    } else {
                                        if (ctx.var("mos_whale_edq").get()? == 34 || ctx.var("mos_whale_edq").get()? == 35) {
                                            if ctx.var("mos_whale_edq").get()? == 34 {
                                                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? != 2 {
                                                    ctx.lines_as(
                                                        "Aged Stranger",
                                                        args!["It's not completed yet. Please wait a little longer."],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                ctx.lines_as(
                                                    "Aged Stranger",
                                                    args![
                                                        "I have succeeded in making the",
                                                        "instrument! Here you are. It's yours.",
                                                        "This instrument is called a Gusli.",
                                                        "This is a traditional instrument",
                                                        "which comes from a faraway land of",
                                                        "the ancestors of Moscovia."
                                                    ],
                                                )?;
                                                ctx.call(Function::GetItem, vec![Val::from(2707), Val::from(1)])?;
                                                ctx.var("mos_whale_edq").set(Val::from(35))?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Aged Stranger",
                                                    args!["I am the last person who has", "learned it in this land."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Aged Stranger",
                                                    args!["So, indeed... This is a very", "special item here, that only we", "have."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Aged Stranger",
                                                    args![
                                                        "If you show this to the good Csar,",
                                                        "he can believe your story about the",
                                                        "island. Everything is going to be",
                                                        "well."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                if ctx.call(Function::IsBeginQuest, vec![Val::from(18118)])?.is_true() {
                                                    ctx.call(Function::ChangeQuest, vec![Val::from(18118), Val::from(18119)])?;
                                                }
                                            }
                                            aged_stranger_npc_run(ctx, AgedStrangerNpcStep::SAS2, vec![Val::from(1)])?;
                                        } else if ctx.var("mos_whale_edq").get()? == 36 {
                                            ctx.lines_as(
                                                "Aged Stranger",
                                                args!["Are you ready to learn the Gusli?", "Please equip the Gusli."],
                                            )?;
                                            ctx.next()?;
                                            if ctx.call(Function::IsEquipped, vec![Val::from(2707)])?.is_true() {
                                                ctx.lines_as(
                                                    "Aged Stranger",
                                                    args![
                                                        "Hm... Very well.",
                                                        "At first, look at me how I play,",
                                                        "then you play it slowly."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::SoundEffect, vec![Val::from("mos_gusli1.wav"), Val::from(0)])?;
                                                ctx.lines_as(
                                                    "Aged Stranger",
                                                    args![
                                                        "Do not hurry,",
                                                        "keep your composure,",
                                                        "just look carefully,",
                                                        "and follow me slowly."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_BARD")?)
                                                    || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_CLOWN")?))
                                                {
                                                    ctx.lines_as("Aged Stranger", args!["Oh! Ooh... Your hands are really good. You are surely talented with a music instrument."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Aged Stranger", args!["I heard that there are some adventurers in the mainland... that like playing instruments and singing songs! You must be one of them..."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Aged Stranger", args!["This may go faster than I expected."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Aged Stranger",
                                                        args!["Okay, it's time for your turn. Play it. Play the song that I played."],
                                                    )?;
                                                    ctx.next()?;
                                                    aged_stranger_npc_run(ctx, AgedStrangerNpcStep::SAS1, vec![Val::from(3)])?;
                                                } else {
                                                    ctx.lines_as(
                                                        "Aged Stranger",
                                                        args!["Okay, it's time for your turn. Play it. Play the song that I played."],
                                                    )?;
                                                    ctx.next()?;
                                                    aged_stranger_npc_run(ctx, AgedStrangerNpcStep::SAS1, vec![Val::from(6)])?;
                                                }
                                            }
                                            ctx.lines_as(
                                                "Aged Stranger",
                                                args![
                                                    "Um... Your preparations are not",
                                                    "good. You're not holding the",
                                                    "instrument correctly. Equip it",
                                                    "properly, and then you can learn."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Aged Stranger",
                                                args!["Tell me again when you are prepared", "to play the Gusli correctly."],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if ctx.var("mos_whale_edq").get()? == 37 {
                                            ctx.lines_as("Aged Stranger", args!["Are you ready to learn the Gusli?"])?;
                                            ctx.next()?;
                                            if ctx.call(Function::IsEquipped, vec![Val::from(2707)])?.is_true() {
                                                ctx.lines_as("Aged Stranger", args!["Um... You did well."])?;
                                                ctx.next()?;
                                                if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_BARD")?)
                                                    || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_CLOWN")?))
                                                {
                                                    ctx.lines_as(
                                                        "Aged Stranger",
                                                        args!["with your ability, you can", "absolutely play it wonderfully.", "Cheer up."],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Aged Stranger",
                                                        args!["Good. Let's play again. Play the", "song that I played."],
                                                    )?;
                                                    ctx.next()?;
                                                    aged_stranger_npc_run(ctx, AgedStrangerNpcStep::SAS1, vec![Val::from(3)])?;
                                                } else {
                                                    ctx.lines_as(
                                                        "Aged Stranger",
                                                        args!["Good. Let's play again. Play the", "song that I played."],
                                                    )?;
                                                    ctx.next()?;
                                                    aged_stranger_npc_run(ctx, AgedStrangerNpcStep::SAS1, vec![Val::from(6)])?;
                                                }
                                            }
                                            ctx.lines_as(
                                                "Aged Stranger",
                                                args![
                                                    "Um... Your preparations are not",
                                                    "good. You're not holding the",
                                                    "instrument correctly. Equip it",
                                                    "properly, and then you can learn."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Aged Stranger",
                                                args!["Tell me again when you are prepared", "to play the Gusli correctly."],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if ctx.var("mos_whale_edq").get()? == 38 {
                                            aged_stranger_npc_run(ctx, AgedStrangerNpcStep::SAS2, vec![Val::from(0)])?;
                                        } else if ctx.var("mos_whale_edq").get()?.number()? > 38 {
                                            ctx.lines_as(
                                                "Aged Stranger",
                                                args!["This is the Whale Island.", "I don't know how you came here."],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Aged Stranger",
                                                args!["If you want, I can send you back", "to the mainland. What'll it be?"],
                                            )?;
                                            ctx.next()?;
                                            match runtime::select_values(
                                                ctx,
                                                &[Val::from("Look around.:Go back to the mainland.:Venture into the unknown.")],
                                            )? {
                                                1 => {
                                                    ctx.lines_as(
                                                        "Aged Stranger",
                                                        args!["Well, well... Do as you please.", "If so, I will take a rest."],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                2 => {
                                                    ctx.lines_as("Aged Stranger", args!["I see. If so, let's go."])?;
                                                    ctx.next()?;
                                                    aged_stranger_npc_run(ctx, AgedStrangerNpcStep::SAS3, vec![])?;
                                                    ctx.close_window()?;
                                                    ctx.call(Function::Warp, vec![Val::from("moscovia"), Val::from(162), Val::from(56)])?;
                                                    return Err(Stop::End);
                                                }
                                                3 => {
                                                    ctx.lines_as(
                                                        "Aged Stranger",
                                                        args!["Oh, if you want, I can", "guide you to a good place for you."],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Aged Stranger",
                                                        args![
                                                            "There are some lands...",
                                                            "untouched and mysterious...",
                                                            "around Moscovia..."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Aged Stranger",
                                                        args!["If you want, I'll send you there. What do you think of that?"],
                                                    )?;
                                                    ctx.next()?;
                                                    if Val::from(runtime::select_values(
                                                        ctx,
                                                        &[Val::from("Consider it.:Ok, please send me there.")],
                                                    )?) == 1
                                                    {
                                                        ctx.lines_as(
                                                            "Aged Stranger",
                                                            args!["Well, well... Do as you please.", "If so, I will take a rest."],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    ctx.lines_as("Aged Stranger", args!["I see. If so, let's go."])?;
                                                    ctx.next()?;
                                                    aged_stranger_npc_run(ctx, AgedStrangerNpcStep::SAS3, vec![])?;
                                                    ctx.close_window()?;
                                                    ctx.call(
                                                        Function::Warp,
                                                        vec![Val::from("mosk_fild02"), Val::from(204), Val::from(54)],
                                                    )?;
                                                    return Err(Stop::End);
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                ctx.lines_as(
                    "Aged Stranger",
                    args![
                        "Long time, no see!",
                        "You probably like the island, don't you?",
                        "Please stay and take a rest."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aged Stranger",
                    args!["Or if you want, I can send you back", "to the mainland. What'll it be?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Look around.:Go back to the mainland.")],
                )?) == 1
                {
                    ctx.lines_as(
                        "Aged Stranger",
                        args!["Well, well... Do as you please.", "If so, I will take a rest."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Aged Stranger", args!["Good. I will go to the place", "right way."])?;
                ctx.next()?;
                ctx.call(Function::SoundEffect, vec![Val::from("mos_gusli1.wav"), Val::from(0)])?;
                ctx.lines(args![
                    "A old man started to play a instrument",
                    "closing his eyes.",
                    "And then he didn't any answer so, looks like",
                    "falling down to only his world."
                ])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("moscovia"), Val::from(162), Val::from(56)])?;
                return Err(Stop::End);
            }
            AgedStrangerNpcStep::SAS1 => {
                if ctx.call(Function::Rand, vec![Val::from(1), runtime::arg(&args, 0, Val::from(0))])? == 2 {
                    ctx.call(Function::SoundEffect, vec![Val::from("mos_gusli1.wav"), Val::from(0)])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_EXIT")?])?;
                    ctx.lines_as(
                        "Aged Stranger",
                        args![
                            "Oh! You are good at playing the",
                            "Gusli! In such a short time... you",
                            "are great!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aged Stranger",
                        args![
                            "You've done well. I don't need to",
                            "teach you anymore. Your ability",
                            "will be improved with more practice."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aged Stranger",
                        args!["Now go back to the mainland and", "play the Gusli for our dear Csar."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aged Stranger",
                        args!["With your ability to play, surely", "you can win the admiration of all."],
                    )?;
                    ctx.var("mos_whale_edq").set(Val::from(38))?;
                    if ctx.call(Function::IsBeginQuest, vec![Val::from(18119)])?.is_true() {
                        ctx.call(Function::ChangeQuest, vec![Val::from(18119), Val::from(18120)])?;
                    } else if ctx.call(Function::IsBeginQuest, vec![Val::from(18118)])?.is_true() {
                        ctx.call(Function::ChangeQuest, vec![Val::from(18118), Val::from(18120)])?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Aged Stranger",
                    args![
                        "Um, a little bit greenhorn...",
                        "Do not play that way.",
                        "Take more time to concentrate while",
                        "you play."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aged Stranger",
                    args![
                        "Practice more on your own.",
                        "I will check on you again.",
                        "If you practice, you will be good",
                        "at it in no time."
                    ],
                )?;
                ctx.var("mos_whale_edq").set(Val::from(37))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            AgedStrangerNpcStep::SAS2 => {
                ctx.lines_as("Aged Stranger", args!["So, what will you do now?"])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[((Val::from("How would I get back here?:")
                        + (if runtime::arg(&args, 0, Val::from(0)).is_true() {
                            Val::from("I want to learn the Gusli")
                        } else {
                            Val::from("")
                        }))
                        + Val::from(":Go back to the mainland."))],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Aged Stranger",
                            args![
                                "You are my friend now,",
                                "and you have this instrument.",
                                "Whenever you want to come back",
                                "here again, just play the Gusli",
                                "at the docks of Moscovia."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aged Stranger",
                            args![
                                "Even if it's difficult to play it",
                                "well, because you may not yet know",
                                "how to play it, it's not a problem.",
                                "Just make a sound."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aged Stranger",
                            args![
                                "This island... this whale... can",
                                "perceive the sound of a Gusli from",
                                "an endless distance away."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aged Stranger",
                            args![
                                "If you play this instrument,",
                                "wherever you are, I'll go to you",
                                "with this island. Only if you are a",
                                "friend... heheh."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Aged Stranger",
                            args![
                                "You really want to...",
                                "learn this instrument...",
                                "don't you?",
                                "Well... it's hard to learn",
                                "in such a short time..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aged Stranger",
                            args!["It's okay if you can't play this", "instrument. I already made it for you."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aged Stranger",
                            args![
                                "But I can teach you how",
                                "to play it anyway.",
                                "I can't be too sure, though",
                                "because I have never taught others."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aged Stranger",
                            args!["Okay. If you are ready to play the", "Gusli, let me know."],
                        )?;
                        ctx.var("mos_whale_edq").set(Val::from(36))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.lines_as("Aged Stranger", args!["I see. Okay, let's go."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aged Stranger",
                            args![
                                "Whenever you want to come back here",
                                "again, play the Gusli at the docks",
                                "of Moscovia."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aged Stranger",
                            args![
                                "Even if it's difficult to play it",
                                "well, because you may not yet know",
                                "how to play it, it's not a problem.",
                                "Just make a sound."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aged Stranger",
                            args![
                                "This island... this whale... can",
                                "perceive the sound of a Gusli from",
                                "an endless distance away."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aged Stranger",
                            args![
                                "If you play this instrument,",
                                "wherever you are, I'll go to you",
                                "with this island. Only if you are a",
                                "friend... heheh."
                            ],
                        )?;
                        ctx.next()?;
                        aged_stranger_npc_run(ctx, AgedStrangerNpcStep::SAS3, vec![])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("moscovia"), Val::from(162), Val::from(56)])?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                step = AgedStrangerNpcStep::SAS3;
                continue 'machine;
            }
            AgedStrangerNpcStep::SAS3 => {
                ctx.call(Function::SoundEffect, vec![Val::from("mos_gusli1.wav"), Val::from(0)])?;
                ctx.lines(args![
                    "-The old man starts to play",
                    "his instrument, with eyes closed.",
                    "He is unresponsive...",
                    "like having fallen into a world",
                    "only his own.-"
                ])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn aged_stranger_npc(ctx: &Ctx) -> Script {
    aged_stranger_npc_run(ctx, AgedStrangerNpcStep::Start, Vec::new()).map(|_| ())
}

fn csar_alexsay_iii_npc_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
        ctx.lines_as(
            "Csar Alexsay III",
            args!["You're carrying too many items.", "Please come after using kafra service."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("mos_nowinter").get()? == 12 {
        ctx.lines_as(
            "Csar Alexsay III",
            args!["You!!!", "So many people saw you", "meet with Baba Yaga!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Csar Alexsay III",
            args!["The guilt which must be felt, when", "meeting secretly with a witch..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Csar Alexsay III",
            args!["For that, we indict capital", "punishment without any just trial!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Csar Alexsay III",
            args!["But you are a stranger to these", "lands... So I will hold you in special trial."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Csar Alexsay III",
            args!["If you have anything to say...", "spare me no detail."],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Explain the circumstances.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines(args![
            "-Talk about what happened with Baba",
            "Yaga, and move forward with the plan.-"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Csar Alexsay III",
            args![
                "Hm-hm, that's an embarrassing story...",
                "But if you are telling the",
                "truth, my people will be pleased."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Csar Alexsay III",
            args!["Okay, Bring to me", "any evidence, to believe", "what you are saying."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Csar Alexsay III",
            args![
                "You killed the Baba Yaga!!",
                "If this is true, bring",
                "me Yaga's Pestles.",
                "If you do, I will make",
                "sure no one ever doubts you."
            ],
        )?;
        ctx.next()?;
        if ctx.call(Function::CountItem, vec![Val::from(7762)])?.number()? > 39
            && Val::from(runtime::select_values(
                ctx,
                &[Val::from("Show the Yaga's Pestles.:Do nothing.")],
            )?) == 1
        {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Yes, Here you are."],
            )?;
            ctx.next()?;
            ctx.mes("-Offered the Yaga's Pestles.-")?;
            ctx.call(Function::DelItem, vec![Val::from(7762), Val::from(40)])?;
            ctx.next()?;
            ctx.lines_as(
                "Csar Alexsay III",
                args![
                    "Hm.. You do have them.",
                    "For the time being, I will admit",
                    "that you are coming and going to",
                    "hunt Baba Yaga."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Csar Alexsay III",
                args![
                    "But do not engage in doubtable",
                    "behavior that would instigate my",
                    "people to act in a strange way!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Csar Alexsay III",
                args!["If you do that, I will arrest you immediately!", "So take care of yourself."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Csar Alexsay III",
                args![
                    "And, when you succeed in",
                    "banishing winter with magic,",
                    "announce that to me immediately."
                ],
            )?;
            ctx.var("mos_nowinter").set(Val::from(14))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(18076), Val::from(18077)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Csar Alexsay III",
            args!["I said to bring me", "40 Yaga's Pestles", "from the Baba Yaga."],
        )?;
        ctx.var("mos_nowinter").set(Val::from(13))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("mos_nowinter").get()? == 13 {
            ctx.lines_as(
                "Csar Alexsay III",
                args!["Did you bring some evidence to", "resolve your issue of doubt, traveler?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Yes, I did.:I am confused.")])?) == 1 {
                if ctx.call(Function::CountItem, vec![Val::from(7762)])?.number()? > 39 {
                    ctx.call(Function::DelItem, vec![Val::from(7762), Val::from(40)])?;
                    ctx.lines_as(
                        "Csar Alexsay III",
                        args![
                            "Good,",
                            "I won't doubt you anymore.",
                            "Because you've proven yourself,",
                            "you can remain a free traveler."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Csar Alexsay III",
                        args![
                            "Surely, I hate cooperating with Baba Yaga.",
                            "But if my people are happy",
                            "after the work you do,",
                            "I am also pleased."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Csar Alexsay III",
                        args![
                            "For the time being, I will admit",
                            "that you are coming and going to",
                            "hunt Baba Yaga."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Csar Alexsay III",
                        args![
                            "But do not engage in doubtable",
                            "behavior that would instigate my",
                            "people to act in a strange way!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Csar Alexsay III",
                        args!["If you do that, I will arrest you immediately!", "So take care of yourself."],
                    )?;
                    ctx.var("mos_nowinter").set(Val::from(14))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(18076), Val::from(18077)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Csar Alexsay III",
                    args![
                        "Your items do not match the",
                        "count of 40 Yaga's Pestles.",
                        "Did you forget the amount,",
                        "or did you wish to lie to me..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Csar Alexsay III",
                    args![
                        "I will treat that as a joke!",
                        "A kind of sarcasm!",
                        "You go away now and bring some",
                        "evidence to certify your innocence."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Csar Alexsay III",
                args!["I said to bring me", "40 Yaga's Pestles", "from the Baba Yaga."],
            )?;
            ctx.next()?;
            ctx.lines_as("Csar Alexsay III", args!["Are you here to tease me?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("mos_nowinter").get()? == 14 {
                ctx.lines_as(
                    "Csar Alexsay III",
                    args![
                        "It'd be great if I stayed put, for",
                        "the good of both me and my people.",
                        "I don't want to see the Baba Yaga",
                        "personally."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Csar Alexsay III", args!["So, I want you to ", "take my place for that."])?;
                ctx.next()?;
                ctx.lines_as("Csar Alexsay III", args!["Please help Baba Yaga", "to seize the summer."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("mos_nowinter").get()? == 20 {
                    ctx.lines_as("Csar Alexsay III", args!["Are you here for..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Csar Alexsay III",
                        args!["I already heard about the", "weather from the minister."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Csar Alexsay III",
                        args![
                            "Actually, I was a little",
                            "dissatisfied with the doing",
                            "of witchcraft at the center",
                            "of the square..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Csar Alexsay III", args!["But... I can admit it was good work."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Csar Alexsay III",
                        args![
                            "Everyone hates the cold winter.",
                            "Now, the winter will not",
                            "come anymore! So, I'm very pleased."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Csar Alexsay III",
                        args!["Right.", "I want to give you something", "in the name of the people."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Csar Alexsay III",
                        args!["Here, take it.", "I give it as an atonement", "to make my people happy."],
                    )?;
                    ctx.var("mos_nowinter").set(Val::from(21))?;
                    ctx.call(Function::CompleteQuest, vec![Val::from(18079)])?;
                    ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Csar Alexsay III",
                        args!["Stay here as long as you want, and", "enjoy yourself to the fullest this summer."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("mos_whale_edq").get()?.number()? < 16 {
                        ctx.lines_as(
                            "Csar Alexsay III",
                            args!["Welcome to Moscovia!", "I am the ruler, Csar Aleksay III, of Moscovia."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Csar Alexsay III", args!["Go back to your hometown and talk about the beauty of Moscovia! Talk about my great government to all the people of the lands!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("mos_whale_edq").get()? == 16 {
                            ctx.lines_as(
                                "Csar Alexsay III",
                                args!["A foreign traveler...?", "Do you have something to tell me?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Csar Alexsay III",
                                args!["If it is not important,", "have an audience with the Prime Minister first."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("mos_whale_edq").get()? == 17 {
                                ctx.lines_as(
                                    "Csar Alexsay III",
                                    args!["Oh, are you the traveler who told me about an interesting adventure story..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Csar Alexsay III", args!["Not only are you not from Moscovia, but also... you have found the moving island, which only existed in legends! I want to hear details..."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Csar Alexsay III",
                                    args![
                                        "Let's hear it...",
                                        "Tell me quickly.",
                                        "I'm anxious to know the truth about the island..."
                                    ],
                                )?;
                                ctx.next()?;
                                let choice = runtime::select_values(ctx, &[Val::from("Tell the story all the while.")])?;
                                ctx.var("@menu").set(choice)?;
                                ctx.mes("...")?;
                                ctx.next()?;
                                ctx.mes("... ...")?;
                                ctx.next()?;
                                ctx.mes("... ... ...")?;
                                ctx.next()?;
                                ctx.mes("... ... ... ...")?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Csar Alexsay III",
                                    args![
                                        "Surely, an interesting story... But it sounds so unbelievable! What do you think, Prime Minister?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Prime Minister Dmitree",
                                    args![
                                        "It doesn't sound like a lie,",
                                        "but it's difficult to believe",
                                        "completely. I mean, I've known",
                                        "a whale to be huge, but..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Prime Minister Dmitree",
                                    args![
                                        "a person living there...",
                                        "water streaming...",
                                        "and a tree growing on it...!!!",
                                        "Hmm..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Csar Alexsay III", args!["I agree. I have heard many kinds of mysterious adventure stories, but this sounds like an absurd story!"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Csar Alexsay III",
                                    args!["Not to mention that it's happened near my nation. Unbelievable!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Prime Minister Dmitree",
                                    args![
                                        "But Csar... In my memory,",
                                        "I heard that exactly, the",
                                        "last time a story was told",
                                        "about the mysterious moving island. But there was a musical instrument...which an old man played."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Csar Alexsay III", args!["Really? Tell me the details."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Prime Minister Dmitree",
                                    args![
                                        "Surely, I have heard the same",
                                        "story, as told in the legends",
                                        "of our ancestors; about an old man who lived there."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Csar Alexsay III", args!["Hm. There are many doubtful points. Ordinarily, I would give you a big prize, but in this case, it is difficult to believe."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Csar Alexsay III",
                                    args![
                                        "Bring me something to prove",
                                        "the existence of the whale.",
                                        "If you do, I will give you a big prize."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Csar Alexsay III",
                                    args![
                                        "But if you don't,",
                                        "I will punish you for your lies! Bring this instrument that supposedly an old man has."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Prime Minister Dmitree",
                                    args![
                                        "Csar... This is a foreigner...",
                                        "No disrespect was meant to you.",
                                        "Perhaps a little more generosity is in order?."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Csar Alexsay III", args!["No way! If the story is not true, many other foreign travelers who pass by this land will disrespect me like this... What would you have me do?"])?;
                                ctx.next()?;
                                ctx.lines_as("Prime Minister Dmitree", args!["......"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Csar Alexsay III",
                                    args![
                                        "You got it, traveler?",
                                        "You have a heavy responsibility.",
                                        "Bring evidence of this whale island to me, to provide me with some relief. Now go."
                                    ],
                                )?;
                                ctx.var("mos_whale_edq").set(Val::from(18))?;
                                if ctx.call(Function::IsBeginQuest, vec![Val::from(18112)])?.is_true() {
                                    ctx.call(Function::ChangeQuest, vec![Val::from(18112), Val::from(18113)])?;
                                }
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if (ctx.var("mos_whale_edq").get()?.number()? > 17 && ctx.var("mos_whale_edq").get()?.number()? < 35) {
                                    ctx.lines_as("Csar Alexsay III", args!["I'm tired... I want to take a rest, so... leave."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Csar Alexsay III", args!["Just think about finding the whale island..."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("mos_whale_edq").get()? == 35 {
                                        ctx.lines_as(
                                            "Csar Alexsay III",
                                            args![
                                                "Oh. You've come back...",
                                                "Hm. Did you find something to bring",
                                                "to me from the whale island?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Csar Alexsay III",
                                            args![
                                                "Did you bring the instrument?",
                                                "A Gooselri? Which only exists",
                                                "in the whale island? Let's see."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        if ctx.call(Function::CountItem, vec![Val::from(2707)])?.number()? > 0 {
                                            ctx.lines_as("Csar Alexsay III", args!["Oh... Is this instrument... a Gooselri?"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Prime Minister Dmitree",
                                                args!["Actually, Csar, I believe it is", "called a Gusli."],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Csar Alexsay III",
                                                args!["Oh. Prime Minister, is this the", "correct instrument from the", "legends, then?"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Prime Minister Dmitree",
                                                args![
                                                    "I can't be sure, but this is",
                                                    "definitely an instrument never seen",
                                                    "in our lands before."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Csar Alexsay III",
                                                args![
                                                    "Hm. This is also my first time",
                                                    "seeing this kind of musical",
                                                    "instrument. So mysterious... Hey!",
                                                    "Can you play it?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            if Val::from(runtime::select_values(ctx, &[Val::from("I can play.:I can't play it.")])?) == 1 {
                                                ctx.lines_as("Csar Alexsay III", args!["You can play it!", "Good, play it right away!"])?;
                                                ctx.next()?;
                                                ctx.call(Function::SoundEffect, vec![Val::from("mos_gusli2.wav"), Val::from(0)])?;
                                                ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
                                                ctx.call(
                                                    Function::Emotion,
                                                    vec![
                                                        ctx.constant("ET_ANGER")?,
                                                        ctx.call(
                                                            Function::GetNpcId,
                                                            vec![Val::from(0), Val::from("Prime Minister Dmitree#m")],
                                                        )?,
                                                    ],
                                                )?;
                                                ctx.lines_as(
                                                    "Csar Alexsay III",
                                                    args!["Um... What is this? Do you mock me?", "You are so impudent... You...!"],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Prime Minister Dmitree",
                                                    args![
                                                        "Csar... Calm down, please.",
                                                        "If you are quick to anger by such a",
                                                        "lowly person, it will become a",
                                                        "problem of prestige for you."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Csar Alexsay III",
                                                    args![
                                                        "Agh... I agree with you.",
                                                        "You. Give a proper prize to the",
                                                        "adventurer and send them on their way."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Prime Minister Dmitree", args!["How can I re...reward...?"])?;
                                                ctx.next()?;
                                                ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                                                ctx.lines_as(
                                                    "Csar Alexsay III",
                                                    args![
                                                        "I ordered you instead Prime",
                                                        "Minister, to make this poor player",
                                                        "disappear from in front of my eyes,",
                                                        "right now."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Prime Minister Dmitree",
                                                    args!["You should be grateful for the", "Csar's mercy... impudent", "traveler..."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Prime Minister Dmitree", args!["Even though I regard as your effort for the time so, award you. Take it and go out."])?;
                                                ctx.call(Function::GetItem, vec![Val::from(12702), Val::from(1)])?;
                                                ctx.call(Function::GetExperience, vec![Val::from(500000), Val::from(0)])?;
                                                ctx.var("mos_whale_edq").set(Val::from(39))?;
                                                if ctx.call(Function::IsBeginQuest, vec![Val::from(18118)])?.is_true() {
                                                    ctx.call(Function::ChangeQuest, vec![Val::from(18118), Val::from(18119)])?;
                                                }
                                                ctx.call(Function::CompleteQuest, vec![Val::from(18119)])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            ctx.lines_as(
                                                "Csar Alexsay III",
                                                args!["Um... That's too bad.", "I see... Will I ever believe..."],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Csar Alexsay III",
                                                args!["Thanks for your efforts.", "Hey, Prime Minister,", "reward this traveler."],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Prime Minister Dmitree", args!["How can I reward the traveler?"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Csar Alexsay III",
                                                args!["I leave the matter in your hands;", "it's up to you. I will take a", "rest."],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Prime Minister Dmitree",
                                                args!["The Csar didn't take pleasure in", "your story, as I expected."],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Prime Minister Dmitree",
                                                args!["Even though I regard as your effort for the time so, award you. Take it."],
                                            )?;
                                            ctx.call(Function::GetItem, vec![Val::from(12702), Val::from(1)])?;
                                            ctx.call(Function::GetExperience, vec![Val::from(700000), Val::from(0)])?;
                                            ctx.var("mos_whale_edq").set(Val::from(40))?;
                                            if ctx.call(Function::IsBeginQuest, vec![Val::from(18118)])?.is_true() {
                                                ctx.call(Function::ChangeQuest, vec![Val::from(18118), Val::from(18119)])?;
                                            }
                                            ctx.call(Function::CompleteQuest, vec![Val::from(18119)])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as(
                                            "Csar Alexsay III",
                                            args![
                                                "What are you doing? Can you play it",
                                                "without even holding the musical instrument???"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Csar Alexsay III", args!["Don't be impudent with me! Do it right!"])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("mos_whale_edq").get()? == 38 {
                                        ctx.lines_as(
                                            "Csar Alexsay III",
                                            args!["Oh you come...hum did you find", "something to satisfy me", "at the whale island?"],
                                        )?;
                                        ctx.next()?;
                                        if ctx.call(Function::CountItem, vec![Val::from(2707)])?.is_true() {
                                            ctx.lines_as("Csar Alexsay III", args!["Oh... Is this instrument... a Gooselri?"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Prime Minister Dmitree",
                                                args!["Actually, Csar, I believe it is called a Gusli."],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Csar Alexsay III",
                                                args!["Oh. Prime Minister, is this the", "correct instrument from the", "legends, then?"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Prime Minister Dmitree",
                                                args![
                                                    "I can't be sure, but this is",
                                                    "definitely an instrument never seen",
                                                    "in our lands before."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Csar Alexsay III",
                                                args![
                                                    "Hm. This is also my first time",
                                                    "seeing this kind of musical",
                                                    "instrument. So mysterious... Hey!",
                                                    "Can you play it?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["Yes. I learned how to play it at Whale Island."],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Csar Alexsay III",
                                                args!["Oh! Oh!... You can play it!", "Play it right away. I wonder about its sound."],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::SoundEffect, vec![Val::from("mos_gusli1.wav"), Val::from(0)])?;
                                            ctx.lines(args![
                                                "-When the music of the Gusli is",
                                                "played, all the people in the",
                                                "Csar's Palace fall in with the tune.-"
                                            ])?;
                                            ctx.next()?;
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                            ctx.call(
                                                Function::Emotion,
                                                vec![
                                                    ctx.constant("ET_CRY")?,
                                                    ctx.call(
                                                        Function::GetNpcId,
                                                        vec![Val::from(0), Val::from("Prime Minister Dmitree#m")],
                                                    )?,
                                                ],
                                            )?;
                                            ctx.lines_as(
                                                "Csar Alexsay III",
                                                args!["Oh! I can't hear without tears.", "That's a sad tune."],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Prime Minister Dmitree",
                                                args![
                                                    "That's right, Csar.",
                                                    "I have never heard a sad tune such as this.",
                                                    "Kh-huk. Sniff."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Csar Alexsay III", args!["Huk... You. Give a big prize to", "this traveler!"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Prime Minister Dmitree", args!["Hukhuk... Sniff. How can I reward this?"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Csar Alexsay III",
                                                args![
                                                    "I leave this matter in your hands.",
                                                    "Give a proper prize to whom has",
                                                    "Shown me a great story and",
                                                    "beautiful music."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Prime Minister Dmitree",
                                                args![
                                                    "I express thanks to you on behalf",
                                                    "of our dear Csar and all the people",
                                                    "in his palace. I will reward your",
                                                    "efforts, in the name of the Csar."
                                                ],
                                            )?;
                                            ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
                                            ctx.call(Function::GetExperience, vec![Val::from(1200000), Val::from(0)])?;
                                            ctx.var("mos_whale_edq").set(Val::from(41))?;
                                            ctx.call(Function::CompleteQuest, vec![Val::from(18120)])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as(
                                            "Csar Alexsay III",
                                            args!["But you... Where do you keep the", "instrument which you are to show me?"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Csar Alexsay III",
                                            args!["Don't you need to be holding the instrument, in order to play it???"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["I'm sorry. I will be ready now, and try again to play it."],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("mos_whale_edq").get()? == 39 {
                                        ctx.lines_as("Csar Alexsay III", args!["What happen... If you have special things go out."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("mos_whale_edq").get()? == 40 {
                                        ctx.lines_as(
                                            "Csar Alexsay III",
                                            args![
                                                "Um... You are a traveler as I saw.",
                                                "If you have special things, don't interfere my rest."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("mos_whale_edq").get()? == 41 {
                                        ctx.lines_as(
                                            "Csar Alexsay III",
                                            args![
                                                "Oh... You. ~",
                                                "Nice to see you again.",
                                                "I want for you to sometimes stop by here and play some music for me. ~"
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
    ctx.lines_as(
        "Csar Alexsay III",
        args!["Welcome to Moscovia,", "I am Csar Alexsay the Third."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Csar Alexsay III",
        args![
            "Go back to your hometown and tell everyone about the beauty of Moscovia,",
            "and my great government."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn csar_alexsay_iii_npc(ctx: &Ctx) -> Script {
    csar_alexsay_iii_npc_body(ctx, Vec::new()).map(|_| ())
}

fn csar_alexsay_iii_npc_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mos_nowinter").get()? != 12 {
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Csar Alexsay III",
        args!["You!!!", "So many people saw you", "meet with Baba Yaga!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Csar Alexsay III",
        args!["The guilt which must be felt, when", "meeting secretly with a witch..."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Csar Alexsay III",
        args!["For that, we indict capital", "punishment without any just trial!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Csar Alexsay III",
        args!["But you are a stranger to these", "lands... So I will hold you in special trial."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Csar Alexsay III",
        args!["If you have anything to say...", "spare me no detail."],
    )?;
    ctx.next()?;
    let choice = runtime::select_values(ctx, &[Val::from("Explain the circumstances.")])?;
    ctx.var("@menu").set(choice)?;
    ctx.lines(args![
        "-Talk about what happened with Baba",
        "Yaga, and move forward with the plan.-"
    ])?;
    ctx.next()?;
    ctx.lines_as(
        "Csar Alexsay III",
        args![
            "Hm-hm, that's an embarrassing story...",
            "But if you are telling the",
            "truth, my people will be pleased."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Csar Alexsay III",
        args!["Okay, Bring to me", "any evidence, to believe", "what you are saying."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Csar Alexsay III",
        args![
            "You killed the Baba Yaga!!",
            "If this is true, bring",
            "me Yaga's Pestles.",
            "If you do, I will make",
            "sure no one ever doubts you."
        ],
    )?;
    ctx.next()?;
    if ctx.call(Function::CountItem, vec![Val::from(7762)])?.number()? > 39
        && Val::from(runtime::select_values(
            ctx,
            &[Val::from("Show the Yaga's Pestles.:Do nothing.")],
        )?) == 1
    {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Yes, Here you are."],
        )?;
        ctx.next()?;
        ctx.mes("-Offered the Yaga's Pestles.-")?;
        ctx.call(Function::DelItem, vec![Val::from(7762), Val::from(40)])?;
        ctx.next()?;
        ctx.lines_as(
            "Csar Alexsay III",
            args![
                "Hm.. You do have them.",
                "For the time being, I will admit",
                "that you are coming and going to",
                "hunt Baba Yaga."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Csar Alexsay III",
            args![
                "But do not engage in doubtable",
                "behavior that would instigate my",
                "people to act in a strange way!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Csar Alexsay III",
            args!["If you do that, I will arrest you immediately!", "So take care of yourself."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Csar Alexsay III",
            args![
                "And, when you succeed in",
                "banishing winter with magic,",
                "announce that to me immediately."
            ],
        )?;
        ctx.var("mos_nowinter").set(Val::from(14))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(18076), Val::from(18077)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Csar Alexsay III",
        args!["I said to bring me", "40 Yaga's Pestles", "from the Baba Yaga."],
    )?;
    ctx.var("mos_nowinter").set(Val::from(13))?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn csar_alexsay_iii_npc_ontouch(ctx: &Ctx) -> Script {
    csar_alexsay_iii_npc_ontouch_body(ctx, Vec::new()).map(|_| ())
}
