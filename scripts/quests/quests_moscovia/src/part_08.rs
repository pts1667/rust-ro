use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn occult_apple_tree_rus29_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rhea_rus_main").get()? == 8 && ctx.var("rhea_rus_quiz").get()? == 2 {
        if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2429
            && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2430)
        {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["...Hmm, did I forget to wear something...?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "- There are Ripe Apples -",
            "- growing in clusters. -",
            "- But they are higher than -",
            "- you expected them to be -"
        ])?;
        ctx.next()?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])? == 2 {
            ctx.lines(args!["- You hit the tree firmly, -", "- and the apples fall down! -"])?;
            ctx.next()?;
        } else {
            ctx.lines(args!["- You hit the tree firmly, -", "- worms fall down! -"])?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Ahhhhhhhh!!!"])?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_HUK")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.mes("- ^0000ff You get 100 Apples !!^000000 -")?;
        ctx.call(
            Function::Announce,
            vec![
                Val::from("Mazroka : You are truly brave. When you get the cookies and apples, come to see me."),
                ctx.constant("BC_MAP")?,
                Val::from(8900331),
            ],
        )?;
        ctx.var("rhea_rus_quiz").set(Val::from(3))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_QUESTION")?,
            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
        ],
    )?;
    return Err(Stop::End);
}

pub fn occult_apple_tree_rus29(ctx: &Ctx) -> Script {
    occult_apple_tree_rus29_body(ctx, Vec::new()).map(|_| ())
}

fn marozka_rus31_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_j = Val::from(0);
    let mut l_rus_quiz01 = Val::from(0);
    if ctx.var("rhea_rus_main").get()?.number()? < 8 {
        ctx.lines_as("Marozka", args!["..........................."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("rhea_rus_main").get()? == 8 {
            if ctx.var("rhea_rus_quiz").get()?.number()? < 3 {
                ctx.lines_as("Marozka", args!["..........................."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("rhea_rus_quiz").get()? == 3 {
                if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2429
                    && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2430)
                {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["...Hmm, did I forget to wear something...?"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Marozka", args![".....................you came."])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Are you Marozka? Here, I've proven to you my strength now can you make me the 'Golden Thread'?",
                        "I need it to get Maria out of that wall."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "- You put the cookies and -",
                    "- apples in front of him -",
                    "- and he slowly utters -"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Marozka",
                    args!["As you've heard, to make the 'Golden Thread' is very difficult and takes a very long time."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Marozka",
                    args!["I will begin making it now... let me see... Could you please come back to me in an hour?"],
                )?;
                ctx.var("rhea_rus_quiz").set(Val::from(4))?;
                ctx.var("rus_time01")
                    .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?)?;
                ctx.var("rus_time02")
                    .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_DAYOFWEEK")?])?)?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("rhea_rus_quiz").get()? == 4 {
                if (!ctx
                    .var("rus_time01")
                    .get()?
                    .loosely_equals(&ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?)
                    || !ctx
                        .var("rus_time02")
                        .get()?
                        .loosely_equals(&ctx.call(Function::GetTime, vec![ctx.constant("DT_DAYOFWEEK")?])?))
                {
                    ctx.lines_as(
                        "Marozka",
                        args![
                            "Ah, just in time.",
                            "I have finally finished making the 'Golden Thread'. Just wait one more second and it'll be ready."
                        ],
                    )?;
                    ctx.var("rhea_rus_quiz").set(Val::from(28))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Marozka",
                    args!["As you've heard, to make the 'Golden Thread' is very difficult and takes a very long time."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Marozka",
                    args!["I will begin making it now... let me see... Could you please come back to me in an hour?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("rhea_rus_quiz").get()? == 28 {
                ctx.lines_as(
                    "Marozka",
                    args![
                        "As promised, here's the 'Golden Thread'.",
                        "So, you are really going to fight Koshei the Immportal, eh?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["I don't know but I promised to get Maria out of there."],
                )?;
                ctx.next()?;
                ctx.lines_as("Marozka", args!["Huh, Koshei is not your average adversary. Strength alone cannot beat him. You will need wisdom and an unfaltering spirit in order to win."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Marozka",
                    args!["Koshei will be extremely hostile to you because you are helping Maria."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Marozka",
                    args!["Do you still want the Golden Thread? Ok, I will give it to you! But, there is one more test."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["One more test?", "Didn't I already prove my worth?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Marozka",
                    args!["That was a test of your strength. Now you must pass the test of mind and wisdom."],
                )?;
                ctx.var("rhea_rus_quiz").set(Val::from(29))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("rhea_rus_quiz").get()? == 29 {
                if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2429
                    && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2430)
                {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["...Hmm, did I forget to wear something...?"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Marozka", args!["...So? Have you decided?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("I need more time...:I will take it!")],
                )?) == 1
                {
                    ctx.lines_as("Marozka", args!["Ok then. Come back when you are ready."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Marozka",
                    args!["You will... Ok, let's get to it. You must answer all of these questions correctly in order to pass the test."],
                )?;
                ctx.next()?;
                ctx.lines_as("Marozka", args!["What is deaf, dumb, and blind and always tells the truth?"])?;
                ctx.next()?;
                l_j = Val::from(runtime::select_values(ctx, &[Val::from("A Poring:A Picky:A Mirror:A Tree")])?);
                l_rus_quiz01 = (if l_j.clone() == 3 { Val::from(1) } else { Val::from(0) });
                ctx.lines_as("Marozka", args!["If 4 cats can catch 4 mice every 4 minutes, what is the minimum number of cats needed to get 10 mice in 10 minutes?"])?;
                ctx.next()?;
                l_j = Val::from(runtime::select_values(ctx, &[Val::from("4:5:6:10")])?);
                l_rus_quiz01 = (if l_j.clone() == 1 {
                    (l_rus_quiz01.clone() + Val::from(1))
                } else {
                    l_rus_quiz01.clone()
                });
                ctx.lines_as(
                    "Marozka",
                    args!["Which of these gets shorter during winter and longer during summer?"],
                )?;
                ctx.next()?;
                l_j = Val::from(runtime::select_values(ctx, &[Val::from("Sky:Day:Waves:Wind")])?);
                l_rus_quiz01 = (if l_j.clone() == 2 {
                    (l_rus_quiz01.clone() + Val::from(1))
                } else {
                    l_rus_quiz01.clone()
                });
                ctx.lines_as(
                    "Marozka",
                    args![
                        "Doris Etticoat, wears a petticoat and has a red nose; the longer she stands, the shorter she grows. What is she?"
                    ],
                )?;
                ctx.next()?;
                l_j = Val::from(runtime::select_values(ctx, &[Val::from("A Star:A Candle:A Sword:The Moon")])?);
                l_rus_quiz01 = (if l_j.clone() == 2 {
                    (l_rus_quiz01.clone() + Val::from(1))
                } else {
                    l_rus_quiz01.clone()
                });
                ctx.lines_as("Marozka", args!["My top and bottom are twins of a kind. The middle of me makes one body combined. If I stand tall and still, run faster I will. What am I?"])?;
                ctx.next()?;
                l_j = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("A Woman:Valkyrie:The Moon:An Hourglass")],
                )?);
                l_rus_quiz01 = (if l_j.clone() == 4 {
                    (l_rus_quiz01.clone() + Val::from(1))
                } else {
                    l_rus_quiz01.clone()
                });
                ctx.lines_as("Marozka", args!["Now let's see how you did."])?;
                ctx.next()?;
                if l_rus_quiz01.clone().number()? > 4 {
                    ctx.lines_as("Marozka", args!["You got all of them correct."])?;
                    ctx.next()?;
                } else {
                    ctx.lines_as(
                        "Marozka",
                        args!["I told you before that I wouldn't give you the Golden Thread until you got all questions correct."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Marozka", args!["It is not too late to try again. With wisdom comes patience."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Marozka", args!["I know that a quiz is a strange way of evaluating a person's wisdom. But, it is my way of knowing if you are truly committed to fighting Koshei."])?;
                ctx.next()?;
                ctx.lines_as("Marozka", args!["I hope you help Maria with your strength and kindness."])?;
                ctx.var("rhea_rus_quiz").set(Val::from(30))?;
                ctx.call(Function::GetItem, vec![Val::from(7879), Val::from(10)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    ctx.lines_as("Marozka", args!["..............................."])?;
    ctx.next()?;
    ctx.lines(args!["- He closes his eyes -", "- and stands still -"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn marozka_rus31(ctx: &Ctx) -> Script {
    marozka_rus31_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum BabaYagaRus32Step {
    Start,
    OnDisable,
    OnTimer180000,
    OnMyMobDead,
}

fn baba_yaga_rus32_run(ctx: &Ctx, mut step: BabaYagaRus32Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_amount: Vec<Val> = Vec::new();
    let mut l_input = Val::from(0);
    let mut l_input_s = Val::from("");
    let mut l_item = Val::from(0);
    let mut l_item_req: Vec<Val> = Vec::new();
    let mut l_player_name_s = Val::from("");
    let mut l_string_s = Val::from("");
    let mut l_total = Val::from(0);
    let mut l_zeny_req = Val::from(0);
    'machine: loop {
        match step {
            BabaYagaRus32Step::Start => {
                if ctx.var("rhea_rus_main").get()?.number()? < 9 {
                    ctx.lines_as(
                        "Baba Yaga",
                        args![
                            "...........................",
                            "If you lotter around here any longer, I will make myself some tasty human soup! Hehehehehe."
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_KIK")?])?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_HUK")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("rhea_rus_main").get()? == 9 {
                        l_player_name_s = ctx.call(Function::StrCharInfo, vec![Val::from(0)])?;
                        ctx.lines_as("Baba Yaga", args!["Why are you here, you yummy looking human? If you lotter around here any longer, I will make myself some tasty human soup! Hehehehehe"])?;
                        ctx.next()?;
                        ctx.lines_as(l_player_name_s.clone(), args!["I, ah, I.. Gold.. golden..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Baba Yaga",
                            args![
                                "What are you babbling about?",
                                "Do you want me to transform you into a savage beast?!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Baba Yaga", args!["'Presto Change-o!!'", "'Turn into a pig!!'"])?;
                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BARRIER")?])?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Baba Yaga",
                            args![
                                "Hmm? You are protected by a Protection Spell?",
                                "But, it was weake. My spell destroyed it."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Baba Yaga", args!["Leave now, or I will curse you again! 'Presto...'"])?;
                        ctx.next()?;
                        ctx.lines_as(l_player_name_s.clone(), args!["Eh, eh.. I mean.. I say.. spell..."])?;
                        ctx.next()?;
                        let (input, status) = runtime::input_text(ctx, None, None)?;
                        l_input_s = input;
                        if l_input_s.clone() == "Spellshield Protection" {
                            ctx.lines_as(
                                l_player_name_s.clone(),
                                args![
                                    "Eh, eh.. I mean.. I say.. spell...",
                                    ((Val::from("") + l_input_s.clone()) + Val::from(" !!!"))
                                ],
                            )?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_ABSORBSPIRITS")?])?;
                            ctx.next()?;
                        } else {
                            ctx.lines_as(
                                l_player_name_s.clone(),
                                args![
                                    "Eh, eh.. I mean.. I say.. spell...",
                                    ((Val::from("") + l_input_s.clone()) + Val::from(" !!!"))
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Baba Yaga",
                                args!["What are you planning to do with that weak spell!?", "Get away, child!"],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("mosk_dun02"), Val::from(135), Val::from(163)])?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Baba Yaga",
                            args!["Ho, you are protected by a Protection Spell. You are no ordinary kid."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            l_player_name_s.clone(),
                            args!["I heard that you are able to make the 'Golden Key' and that's why I am here!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Baba Yaga", args!["'Golden Key'? Why do you need it?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            l_player_name_s.clone(),
                            args!["I need it to release Maria Mobrena from her dark wall prison."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Baba Yaga", args!["...Maria Morebna...", "Are you fighting against Koshei?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            l_player_name_s.clone(),
                            args!["If he gets in my way of keeping my promise to her, I guess that I will fight him."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Baba Yaga",
                            args![
                                "Ho, kiheeheheheheheh. Ehehehehehehe.",
                                "You are interesting. You don't seem to be scared of Koshei."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Baba Yaga", args!["Ok, the materials for the key are..."])?;
                        ctx.next()?;
                        if ((((ctx.call(Function::CountItem, vec![Val::from(724)])?.number()? > 1
                            && ctx.call(Function::CountItem, vec![Val::from(969)])?.number()? > 2)
                            && ctx.call(Function::CountItem, vec![Val::from(7877)])?.is_true())
                            && ctx.call(Function::CountItem, vec![Val::from(7878)])?.number()? > 1)
                            && ctx.call(Function::CountItem, vec![Val::from(7879)])?.number()? > 9)
                        {
                            ctx.lines_as("Baba Yaga", args!["You have already gatered all the materials?"])?;
                            ctx.next()?;
                        } else {
                            ctx.lines_as(
                                "Baba Yaga",
                                args!["What? I cannot make you the 'Golden Key' without the materials."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Baba Yaga",
                                args!["Go and find ^0000ff2 Cursed Ruby, 3 Gold, 1 Red Ring, 2 Lusalka's Hair, 10 Golden Thread^000000!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Baba Yaga", args!["Ok, let's begin. Ah, before that... Wait here!"])?;
                        ctx.call(Function::DelItem, vec![Val::from(724), Val::from(2)])?;
                        ctx.call(Function::DelItem, vec![Val::from(969), Val::from(3)])?;
                        ctx.call(Function::DelItem, vec![Val::from(7877), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(7878), Val::from(2)])?;
                        ctx.call(Function::DelItem, vec![Val::from(7879), Val::from(10)])?;
                        ctx.var("rhea_rus_main").set(Val::from(10))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("rhea_rus_main").get()? == 10 {
                            ctx.lines_as("Baba Yaga", args!["You come here!"])?;
                            ctx.next()?;
                            ctx.lines(args!["- Baba Yaga looks -", "- at you carefully -"])?;
                            ctx.next()?;
                            ctx.lines_as("Baba Yaga", args!["Hmmm, you look energetic and strong. Ok, while I make the 'Golden Key', you must first do a favor for me!"])?;
                            ctx.next()?;
                            if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ARCHER")?) {
                                ctx.lines_as("Baba Yaga", args!["My cow has run away. Find her and cast a Return Spell on her. The spell is '^ff0000Good feed is orange-flavored^000000'. You sould remember it."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Baba Yaga",
                                    args!["Ok! Move, move! You better be quick or I will find a way to punish you. Ehehehehehe."],
                                )?;
                                ctx.var("rhea_rus_main").set(Val::from(11))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                                ctx.lines_as("Baba Yaga", args!["Behind my house, there is noisy coffin. It has been so nosiy recently that I can't sleep. Silence it."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Baba Yaga",
                                    args!["Ok! Move, move! You better be quick or I will find a way to punish you. Ehehehehehe."],
                                )?;
                                ctx.var("rhea_rus_main").set(Val::from(16))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                                ctx.lines_as("Baba Yaga", args!["Please find my silver spoons. Those bad pirates stole them. I can feel them around the wrecked ship. Bring them to me."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Baba Yaga",
                                    args!["Ok! Move, move! You better be quick or I will find a way to punish you. Ehehehehehe."],
                                )?;
                                ctx.var("rhea_rus_main").set(Val::from(21))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?) {
                                ctx.lines_as(
                                    "Baba Yaga",
                                    args!["Buy me a magic book. It is published by Momotaro in Amatsu. Go there and buy it for me."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Baba Yaga",
                                    args!["Ok! Move, move! You better be quick or I will find a way to punish you. Ehehehehehe."],
                                )?;
                                ctx.var("rhea_rus_main").set(Val::from(26))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
                                ctx.lines_as("Baba Yaga", args!["Can you see a jar next to the cabin? There is a House ghost living in there. Drive him out of there. He irritates me a lot."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Baba Yaga",
                                    args!["Ok! Move, move! You better be quick or I will find a way to punish you. Ehehehehehe."],
                                )?;
                                ctx.var("rhea_rus_main").set(Val::from(31))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as("Baba Yaga", args!["Go to the Broom Grandmother in Payon and buy me a broom. It is best for cleaning but I don't have time to go there."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Baba Yaga",
                                args!["Ok! Move, move! You better be quick or I will find a way to punish you. Ehehehehehe."],
                            )?;
                            ctx.var("rhea_rus_main").set(Val::from(36))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if (ctx.var("rhea_rus_main").get()?.number()? > 10 && ctx.var("rhea_rus_main").get()?.number()? < 16) {
                                ctx.lines_as("Baba Yaga", args!["My cow has run away. Find her and cast a Return Spell on her. The spell is '^ff0000Good feed is orange-flavored^000000'. You sould remember it."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Baba Yaga",
                                    args!["Ok! Move, move! You better be quick or I will find a way to punish you. Ehehehehehe."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if (ctx.var("rhea_rus_main").get()?.number()? > 15 && ctx.var("rhea_rus_main").get()?.number()? < 21) {
                                    ctx.lines_as("Baba Yaga", args!["Can you see a jar next to the cabin? There is a House ghost living in there. Drive him out of there. He irritates me a lot."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Baba Yaga",
                                        args!["Ok! Move, move! You better be quick or I will find a way to punish you. Ehehehehehe."],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if (ctx.var("rhea_rus_main").get()?.number()? > 20 && ctx.var("rhea_rus_main").get()?.number()? < 26) {
                                        ctx.lines_as("Baba Yaga", args!["Please find my silver spoons. Those bad pirates stole them. I can feel them around the wrecked ship. Bring them to me."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Baba Yaga",
                                            args!["Ok! Move, move! You better be quick or I will find a way to punish you. Ehehehehehe."],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.var("rhea_rus_main").get()? == 26 {
                                            ctx.lines_as("Baba Yaga", args!["Buy me a magic book. It is published by Momotaro in Amatsu. Go there and buy it for me."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Baba Yaga",
                                                args![
                                                    "Ok! Move, move! You better be quick or I will find a way to punish you. Ehehehehehe."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.var("rhea_rus_main").get()? == 27 {
                                                if ctx.call(Function::CountItem, vec![Val::from(7881)])?.number()? > 0 {
                                                    ctx.lines_as(
                                                        "Baba Yaga",
                                                        args!["Ho, did you buy the magic book? Give it to me. I will try it out."],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "- Baba Yaga takes the book, -",
                                                        "reads one of the pages, -",
                                                        "and casts something -"
                                                    ])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Baba Yaga", args!["Ok.. Let's do this.", "'Keep off the grass!!!'"])?;
                                                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL4")?])?;
                                                    ctx.call(Function::DelItem, vec![Val::from(7881), Val::from(1)])?;
                                                    ctx.var("rhea_rus_main").set(Val::from(28))?;
                                                    ctx.call(
                                                        Function::Monster,
                                                        vec![
                                                            Val::from("mosk_dun02"),
                                                            Val::from(52),
                                                            Val::from(210),
                                                            Val::from("Violent Gardener"),
                                                            Val::from(1493),
                                                            Val::from(1),
                                                            Val::from("Baba Yaga#rus32::OnMyMobDead"),
                                                        ],
                                                    )?;
                                                    ctx.call(
                                                        Function::Monster,
                                                        vec![
                                                            Val::from("mosk_dun02"),
                                                            Val::from(53),
                                                            Val::from(210),
                                                            Val::from("Dangerous Gardener"),
                                                            Val::from(1500),
                                                            Val::from(1),
                                                            Val::from("Baba Yaga#rus32::OnMyMobDead"),
                                                        ],
                                                    )?;
                                                    ctx.call(
                                                        Function::Monster,
                                                        vec![
                                                            Val::from("mosk_dun02"),
                                                            Val::from(54),
                                                            Val::from(210),
                                                            Val::from("Brutal Gardener"),
                                                            Val::from(1497),
                                                            Val::from(1),
                                                            Val::from("Baba Yaga#rus32::OnMyMobDead"),
                                                        ],
                                                    )?;
                                                    ctx.call(Function::DoNpcEvent, vec![Val::from("Baba Yaga#rus32::OnDisable")])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                ctx.lines_as("Baba Yaga", args!["Buy me a magic book. It is published by Momotaro in Amatsu. Go there and buy it for me."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Baba Yaga", args!["Ok! Move, move! You better be quick or I will find a way to punish you. Ehehehehehe."])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                if ctx.var("rhea_rus_main").get()? == 28 {
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args!["W, what is this, suddenly?!"],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Baba Yaga", args!["Ho.. this is for a beautiful garden. This is better than I expected. Ok then, let's do this."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args!["Hey, are you listening to me?"],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Baba Yaga", args!["'There is an order for you to open your eyes!!!'"])?;
                                                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BEGINSPELL3")?])?;
                                                    ctx.var("rhea_rus_main").set(Val::from(29))?;
                                                    ctx.call(
                                                        Function::Monster,
                                                        vec![
                                                            Val::from("mosk_dun02"),
                                                            Val::from(52),
                                                            Val::from(210),
                                                            Val::from("Alarm to 5 minutes"),
                                                            Val::from(1193),
                                                            Val::from(1),
                                                            Val::from("Baba Yaga#rus32::OnMyMobDead"),
                                                        ],
                                                    )?;
                                                    ctx.call(
                                                        Function::Monster,
                                                        vec![
                                                            Val::from("mosk_dun02"),
                                                            Val::from(53),
                                                            Val::from(210),
                                                            Val::from("Alarm on time"),
                                                            Val::from(1193),
                                                            Val::from(1),
                                                            Val::from("Baba Yaga#rus32::OnMyMobDead"),
                                                        ],
                                                    )?;
                                                    ctx.call(
                                                        Function::Monster,
                                                        vec![
                                                            Val::from("mosk_dun02"),
                                                            Val::from(54),
                                                            Val::from(210),
                                                            Val::from("Alarm past 5 minutes"),
                                                            Val::from(1193),
                                                            Val::from(1),
                                                            Val::from("Baba Yaga#rus32::OnMyMobDead"),
                                                        ],
                                                    )?;
                                                    ctx.call(Function::DoNpcEvent, vec![Val::from("Baba Yaga#rus32::OnDisable")])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    if ctx.var("rhea_rus_main").get()? == 29 {
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args!["What are you doing!!"],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Baba Yaga", args!["Hmm, this one makes people deligent and doesn't look effective. Ok then, next is a House Ghost..."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args!["Hey, I am talking..."],
                                                        )?;
                                                        ctx.call(
                                                            Function::Emotion,
                                                            vec![
                                                                ctx.constant("ET_CRY")?,
                                                                Val::from(
                                                                    ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true(),
                                                                ),
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Baba Yaga", args!["'In the corner...'"])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args!["!!!!!!!!!!!!!!!!!!!!!!!!!"],
                                                        )?;
                                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Baba Yaga", args!["What, child? Do you feel bad?"])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args!["When will you give me the key?!"],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Baba Yaga",
                                                            args!["Huh, I can make it for you right now. You have a violent temper."],
                                                        )?;
                                                        ctx.var("rhea_rus_main").set(Val::from(44))?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else {
                                                        if (ctx.var("rhea_rus_main").get()?.number()? > 30
                                                            && ctx.var("rhea_rus_main").get()?.number()? < 36)
                                                        {
                                                            ctx.lines_as("Baba Yaga", args!["Can you see a jar next to the cabin? There is a House ghost living in there. Drive him out of there. He irritates me a lot."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Baba Yaga", args!["Ok! Move, move! You better be quick or I will find a way to punish you. Ehehehehehe."])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else {
                                                            if (ctx.var("rhea_rus_main").get()?.number()? > 35
                                                                && ctx.var("rhea_rus_main").get()?.number()? < 41)
                                                            {
                                                                ctx.lines_as("Baba Yaga", args!["Go to the Broom Grandmother in Payon and buy me a broom. It is best for cleaning but I don't have time to go there."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Baba Yaga", args!["Ok! Move, move! You better be quick or I will find a way to punish you. Ehehehehehe."])?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else {
                                                                if ctx.var("rhea_rus_main").get()? == 41 {
                                                                    ctx.lines_as(
                                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                        args!["I have done your favor! I got the cow!"],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Baba Yaga",
                                                                        args!["Oh, great. I just finished making the 'Golden Key'!"],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Baba Yaga", args!["Here, help Maria with this key and watch out for Koshei. He is very dangerous. Kehehehehehe."])?;
                                                                    ctx.var("rhea_rus_main").set(Val::from(47))?;
                                                                    ctx.call(Function::GetItem, vec![Val::from(7876), Val::from(1)])?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else {
                                                                    if ctx.var("rhea_rus_main").get()? == 42 {
                                                                        ctx.lines_as(
                                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                            args!["I have done your favor! The coffin is now silenced!"],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Baba Yaga",
                                                                            args!["Oh, yes. I can sleep at night now. Kehehehehe."],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Baba Yaga",
                                                                            args![
                                                                                "You are on time. I just finished making the 'Golden Key'!"
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Baba Yaga", args!["Here, help Maria with this key and watch out for Koshei. He is very dangerous. Kehehehehehe."])?;
                                                                        ctx.var("rhea_rus_main").set(Val::from(47))?;
                                                                        ctx.call(Function::GetItem, vec![Val::from(7876), Val::from(1)])?;
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    } else {
                                                                        if ctx.var("rhea_rus_main").get()? == 43 {
                                                                            if ctx
                                                                                .call(Function::CountItem, vec![Val::from(7880)])?
                                                                                .is_true()
                                                                            {
                                                                                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I have done your favor! I found and got the silver spoons!"])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Baba Yaga", args!["Oh, yes. They are expensive ones. Those bad pirates.. Kehehehehe."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Baba Yaga", args!["You are on time. I just finished making the 'Golden Key'!"])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Baba Yaga", args!["Here, help Maria with this key and watch out for Koshei. He is very dangerous. Kehehehehehe."])?;
                                                                                ctx.call(
                                                                                    Function::DelItem,
                                                                                    vec![Val::from(7880), Val::from(1)],
                                                                                )?;
                                                                                ctx.var("rhea_rus_main").set(Val::from(47))?;
                                                                                ctx.call(
                                                                                    Function::GetItem,
                                                                                    vec![Val::from(7876), Val::from(1)],
                                                                                )?;
                                                                                ctx.close_window()?;
                                                                                return Err(Stop::End);
                                                                            }
                                                                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I have done your favor! I found and got the silver spoons!"])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as(
                                                                                "Baba Yaga",
                                                                                args!["Oh, yes, yes. Give them to me."],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as(
                                                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                                args!["Here they.. They..?!"],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Baba Yaga", args!["You fool! Where did you sell my spoons!? Get them for me right now!"])?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        } else {
                                                                            if ctx.var("rhea_rus_main").get()? == 44 {
                                                                                ctx.lines_as(
                                                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                                    args!["I have done your favor. Give me the key!"],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Baba Yaga", args!["Oh, yes, you violent tempered child. I just finished making the 'Golden Key'."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Baba Yaga", args!["Here, help Maria with this key and watch out for Koshei. He is very dangerous. Kehehehehehe."])?;
                                                                                ctx.var("rhea_rus_main").set(Val::from(47))?;
                                                                                ctx.call(
                                                                                    Function::GetItem,
                                                                                    vec![Val::from(7876), Val::from(1)],
                                                                                )?;
                                                                                ctx.close_window()?;
                                                                                return Err(Stop::End);
                                                                            } else {
                                                                                if ctx.var("rhea_rus_main").get()? == 45 {
                                                                                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I have done your favor! The House ghost is now quiet!"])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as("Baba Yaga", args!["Oh, yes. That irritating ghost is gone. Kehehehehe."])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as("Baba Yaga", args!["You are on time. I just finished making the 'Golden Key'!"])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as("Baba Yaga", args!["Here, help Maria with this key and watch out for Koshei. He is very dangerous. Kehehehehehe."])?;
                                                                                    ctx.var("rhea_rus_main").set(Val::from(47))?;
                                                                                    ctx.call(
                                                                                        Function::GetItem,
                                                                                        vec![Val::from(7876), Val::from(1)],
                                                                                    )?;
                                                                                    ctx.close_window()?;
                                                                                    return Err(Stop::End);
                                                                                } else {
                                                                                    if ctx.var("rhea_rus_main").get()? == 46 {
                                                                                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I have done your favor! Here, the best broom from Payon!"])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as("Baba Yaga", args!["Ooh, yes. Well done. My flying broom was getting old. Kehehehehe."])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as("Baba Yaga", args!["You are on time. I just finished making the 'Golden Key'!"])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as("Baba Yaga", args!["Here, help Maria with this key and watch out for Koshei. He is very dangerous. Kehehehehehe."])?;
                                                                                        ctx.var("rhea_rus_main").set(Val::from(47))?;
                                                                                        ctx.call(
                                                                                            Function::GetItem,
                                                                                            vec![Val::from(7876), Val::from(1)],
                                                                                        )?;
                                                                                        ctx.close_window()?;
                                                                                        return Err(Stop::End);
                                                                                    } else {
                                                                                        if (ctx.var("rhea_rus_main").get()?.number()? > 46
                                                                                            && ctx.var("rhea_rus_main").get()?.number()?
                                                                                                < 49)
                                                                                        {
                                                                                            ctx.lines_as("Baba Yaga", args!["You got the key and went to Maria immedately, right?"])?;
                                                                                            ctx.next()?;
                                                                                            ctx.lines_as("Baba Yaga", args!["Take care of yourself. When you release her, Koshei will come out and bring destruction."])?;
                                                                                            ctx.close_window()?;
                                                                                            return Err(Stop::End);
                                                                                        } else {
                                                                                            if ctx.var("rhea_rus_main").get()? == 49 {
                                                                                                ctx.lines_as(
                                                                                                    "Baba Yaga",
                                                                                                    args![
                                                                                                        "What are you doing here? Kehehehe."
                                                                                                    ],
                                                                                                )?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Koshei... suddenly appeared.. and Maria.. Wolf... killed..."])?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines_as("Baba Yaga", args!["Calm down and speak slowly. I cannot hear you at all. Drink some water."])?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines(args![
                                                                                                    "- You drink some water and -",
                                                                                                    "- tell her the story slowly -"
                                                                                                ])?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines_as(
                                                                                                    "Baba Yaga",
                                                                                                    args![
                                                                                                        "You beat Koshei? Did you?",
                                                                                                        "Kehehe, you are no ordinary kid."
                                                                                                    ],
                                                                                                )?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines_as(
                                                                                                    "Baba Yaga",
                                                                                                    args!["Water to enliven Maria..."],
                                                                                                )?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines_as("Baba Yaga", args!["'Living Water'... I just need the ingredients.  I need ^0000ff1 Holy Water^000000 and ^0000ff2 Yggdrasil Leaves^000000."])?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines_as("Baba Yaga", args!["And 'Dead Water'.. I need ^0000ff1 Cursed Water^000000 and ^0000ff10 Hinalle Leaflets^000000."])?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines_as("Baba Yaga", args!["Bring them to me quickly! Time is running out!"])?;
                                                                                                ctx.var("rhea_rus_main")
                                                                                                    .set(Val::from(50))?;
                                                                                                ctx.close_window()?;
                                                                                                return Err(Stop::End);
                                                                                            } else {
                                                                                                if ctx.var("rhea_rus_main").get()? == 50 {
                                                                                                    if (((ctx
                                                                                                        .call(
                                                                                                            Function::CountItem,
                                                                                                            vec![Val::from(523)],
                                                                                                        )?
                                                                                                        .is_true()
                                                                                                        && ctx
                                                                                                            .call(
                                                                                                                Function::CountItem,
                                                                                                                vec![Val::from(12020)],
                                                                                                            )?
                                                                                                            .is_true())
                                                                                                        && ctx
                                                                                                            .call(
                                                                                                                Function::CountItem,
                                                                                                                vec![Val::from(610)],
                                                                                                            )?
                                                                                                            .number()?
                                                                                                            > 1)
                                                                                                        && ctx
                                                                                                            .call(
                                                                                                                Function::CountItem,
                                                                                                                vec![Val::from(520)],
                                                                                                            )?
                                                                                                            .number()?
                                                                                                            > 9)
                                                                                                    {
                                                                                                        ctx.lines_as(
                                                                                                            "Baba Yaga",
                                                                                                            args![
                                                                                                                "Kehe, you are very quick."
                                                                                                            ],
                                                                                                        )?;
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines_as("Baba Yaga", args!["Ok, let's make the 'Living Water' first..."])?;
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines(args!["- Baba Yaga brings a big pot, -", "- pours the Holy Water in it, -", "- puts the Yggdrasil Leaves -", "- in it and stirs them up -", "- as it casts strange spells -"])?;
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines_as("Baba Yaga", args!["Yg~ Yg~ Yggdrasil~ Yggdrasil has lots of iron~"])?;
                                                                                                        ctx.call(
                                                                                                            Function::NpcSpecialEffect,
                                                                                                            vec![ctx.constant(
                                                                                                                "EF_PHARMACY_OK",
                                                                                                            )?],
                                                                                                        )?;
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines_as("Baba Yaga", args!["Here, I made the 'Living Water' so now let's make the 'Dead Water'..."])?;
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines(args!["- Baba Yaga pours the cursed -", "- water in another pot, then -", "- pounds the Hinalle leaflets, -", "- puts their juice in the pot -", "- and stirs them together -", "- as it casts strange spells -"])?;
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines_as("Baba Yaga", args!["Hi~ hi~ hi~ hi~ na~ lle~ Stir with right hand~ Stir with left hand~ Both hands will be ok~"])?;
                                                                                                        ctx.call(
                                                                                                            Function::NpcSpecialEffect,
                                                                                                            vec![ctx.constant(
                                                                                                                "EF_PHARMACY_OK",
                                                                                                            )?],
                                                                                                        )?;
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines_as(
                                                                                                            "Baba Yaga",
                                                                                                            args!["Here, it's done."],
                                                                                                        )?;
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines_as("Baba Yaga", args!["Take this and help Maria. Kehehehehehe."])?;
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines_as("Baba Yaga", args!["First, pour the 'Dead Water' to remove the wound and curse on her and then pour the 'Living Water' to enliven her."])?;
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines_as("Baba Yaga", args!["Go there right now! Kehehehehe"])?;
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines(args!["- ^0000ffYou get the 'Living Water'^000000 -", "- ^0000ffand the 'Dead Water' !!^000000 -"])?;
                                                                                                        ctx.call(
                                                                                                            Function::DelItem,
                                                                                                            vec![
                                                                                                                Val::from(523),
                                                                                                                Val::from(1),
                                                                                                            ],
                                                                                                        )?;
                                                                                                        ctx.call(
                                                                                                            Function::DelItem,
                                                                                                            vec![
                                                                                                                Val::from(12020),
                                                                                                                Val::from(1),
                                                                                                            ],
                                                                                                        )?;
                                                                                                        ctx.call(
                                                                                                            Function::DelItem,
                                                                                                            vec![
                                                                                                                Val::from(610),
                                                                                                                Val::from(2),
                                                                                                            ],
                                                                                                        )?;
                                                                                                        ctx.call(
                                                                                                            Function::DelItem,
                                                                                                            vec![
                                                                                                                Val::from(520),
                                                                                                                Val::from(10),
                                                                                                            ],
                                                                                                        )?;
                                                                                                        ctx.var("rhea_rus_main")
                                                                                                            .set(Val::from(51))?;
                                                                                                        ctx.close_window()?;
                                                                                                        return Err(Stop::End);
                                                                                                    }
                                                                                                    ctx.lines_as("Baba Yaga", args!["'Living Water'... I just need the ingredients.  I need ^0000ff1 Holy Water^000000 and ^0000ff2 Yggdrasil Leaves^000000."])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as("Baba Yaga", args!["And 'Dead Water'.. I need ^0000ff1 Cursed Water^000000 and ^0000ff10 Hinalle Leaflets^000000."])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as("Baba Yaga", args!["Bring them to me quickly! Time is running out!"])?;
                                                                                                    ctx.close_window()?;
                                                                                                    return Err(Stop::End);
                                                                                                } else if ctx.var("rhea_rus_main").get()?
                                                                                                    == 51
                                                                                                {
                                                                                                    ctx.lines_as(
                                                                                                        "Baba Yaga",
                                                                                                        args!["Go to Maria to help her."],
                                                                                                    )?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as("Baba Yaga", args!["First, pour the 'Dead Water' to remove the wound and curse on her and then pour the 'Living Water' to enliven her."])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as(
                                                                                                        "Baba Yaga",
                                                                                                        args![
                                                                                                            "Go there right now! Kehehehehe"
                                                                                                        ],
                                                                                                    )?;
                                                                                                    ctx.close_window()?;
                                                                                                    return Err(Stop::End);
                                                                                                } else if ctx.var("rhea_rus_main").get()?
                                                                                                    == 52
                                                                                                {
                                                                                                    ctx.lines_as("Baba Yaga", args!["You are more useful than I thought, kid."])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as("Baba Yaga", args!["I thank you for helping Maria. To tell the truth, it was me that connected her with Koshei. In order to seal Koshei, I needed a sacrfice..."])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as("Baba Yaga", args!["I succeeded in sealing Koshei away by using a girl who was magically strong. Sadly the girl also had to remain in the same condition as Koshei in order to maintain the seal."])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as("Baba Yaga", args!["However, after so many years it was impossible to keep him sealed with Maria's power alone."])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as("Baba Yaga", args!["So that's when I decided that if you could subdue Koshei, it would be the only way to release Maria instead of using her to maintain the seal."])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as("Baba Yaga", args!["So, that is why I helped you out by making the 'Golden Key'."])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as("Baba Yaga", args!["Koshei is still alive, but he is weaker than before. So I don't need to use Maria anymore to seal Koshei."])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as("Baba Yaga", args!["Thank you so much for helping Maria..."])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as("Baba Yaga", args!["I want to give anything helpful for you."])?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines_as("Baba Yaga", args!["Whenever you come to me, I will make you potions with just a few materials and a small fee. I hope that this will be of help to you. Kehehehehehe."])?;
                                                                                                    ctx.var("rhea_rus_main")
                                                                                                        .set(Val::from(53))?;
                                                                                                    ctx.close_window()?;
                                                                                                    return Err(Stop::End);
                                                                                                } else if ctx
                                                                                                    .var("rhea_rus_main")
                                                                                                    .get()?
                                                                                                    .number()?
                                                                                                    > 52
                                                                                                {
                                                                                                    ctx.lines_as("Baba Yaga", args!["Oh, you are back. What can I do for you?"])?;
                                                                                                    ctx.next()?;
                                                                                                    match runtime::select_values(
                                                                                                        ctx,
                                                                                                        &[Val::from(
                                                                                                            "Condensed Red Potion:Condensed Yellow Potion:Cancel",
                                                                                                        )],
                                                                                                    )? {
                                                                                                        1 => {
                                                                                                            l_item = Val::from(545);
                                                                                                            l_zeny_req = Val::from(20);
                                                                                                            let base =
                                                                                                                Val::from(0).number()?;
                                                                                                            runtime::local_set(
                                                                                                                &mut l_item_req,
                                                                                                                &Val::from(base + 0),
                                                                                                                Val::from(501),
                                                                                                                false,
                                                                                                            );
                                                                                                            runtime::local_set(
                                                                                                                &mut l_item_req,
                                                                                                                &Val::from(base + 1),
                                                                                                                Val::from(1092),
                                                                                                                false,
                                                                                                            );
                                                                                                            runtime::local_set(
                                                                                                                &mut l_item_req,
                                                                                                                &Val::from(base + 2),
                                                                                                                Val::from(7134),
                                                                                                                false,
                                                                                                            );
                                                                                                            runtime::local_set(
                                                                                                                &mut l_item_req,
                                                                                                                &Val::from(base + 3),
                                                                                                                Val::from(512),
                                                                                                                false,
                                                                                                            );
                                                                                                            let base =
                                                                                                                Val::from(0).number()?;
                                                                                                            runtime::local_set(
                                                                                                                &mut l_amount,
                                                                                                                &Val::from(base + 0),
                                                                                                                Val::from(1),
                                                                                                                false,
                                                                                                            );
                                                                                                            runtime::local_set(
                                                                                                                &mut l_amount,
                                                                                                                &Val::from(base + 1),
                                                                                                                Val::from(1),
                                                                                                                false,
                                                                                                            );
                                                                                                            runtime::local_set(
                                                                                                                &mut l_amount,
                                                                                                                &Val::from(base + 2),
                                                                                                                Val::from(1),
                                                                                                                false,
                                                                                                            );
                                                                                                            runtime::local_set(
                                                                                                                &mut l_amount,
                                                                                                                &Val::from(base + 3),
                                                                                                                Val::from(5),
                                                                                                                false,
                                                                                                            );
                                                                                                            l_string_s = Val::from(
                                                                                                                "I can only make 100 at a time. If you don't want any, then just say '0'",
                                                                                                            );
                                                                                                        }
                                                                                                        2 => {
                                                                                                            l_item = Val::from(546);
                                                                                                            l_zeny_req = Val::from(50);
                                                                                                            let base =
                                                                                                                Val::from(0).number()?;
                                                                                                            runtime::local_set(
                                                                                                                &mut l_item_req,
                                                                                                                &Val::from(base + 0),
                                                                                                                Val::from(503),
                                                                                                                false,
                                                                                                            );
                                                                                                            runtime::local_set(
                                                                                                                &mut l_item_req,
                                                                                                                &Val::from(base + 1),
                                                                                                                Val::from(1092),
                                                                                                                false,
                                                                                                            );
                                                                                                            runtime::local_set(
                                                                                                                &mut l_item_req,
                                                                                                                &Val::from(base + 2),
                                                                                                                Val::from(7134),
                                                                                                                false,
                                                                                                            );
                                                                                                            runtime::local_set(
                                                                                                                &mut l_item_req,
                                                                                                                &Val::from(base + 3),
                                                                                                                Val::from(513),
                                                                                                                false,
                                                                                                            );
                                                                                                            let base =
                                                                                                                Val::from(0).number()?;
                                                                                                            runtime::local_set(
                                                                                                                &mut l_amount,
                                                                                                                &Val::from(base + 0),
                                                                                                                Val::from(1),
                                                                                                                false,
                                                                                                            );
                                                                                                            runtime::local_set(
                                                                                                                &mut l_amount,
                                                                                                                &Val::from(base + 1),
                                                                                                                Val::from(1),
                                                                                                                false,
                                                                                                            );
                                                                                                            runtime::local_set(
                                                                                                                &mut l_amount,
                                                                                                                &Val::from(base + 2),
                                                                                                                Val::from(1),
                                                                                                                false,
                                                                                                            );
                                                                                                            runtime::local_set(
                                                                                                                &mut l_amount,
                                                                                                                &Val::from(base + 3),
                                                                                                                Val::from(10),
                                                                                                                false,
                                                                                                            );
                                                                                                            l_string_s = Val::from(
                                                                                                                "Tell me the number less than 100. If you don't want, tell me zero",
                                                                                                            );
                                                                                                        }
                                                                                                        3 => {
                                                                                                            ctx.lines_as("Baba Yaga", args!["You don't want anything?"])?;
                                                                                                            ctx.next()?;
                                                                                                            ctx.lines_as("Baba Yaga", args!["I don't understand you. What do you want? kehehehehe."])?;
                                                                                                            ctx.close_window()?;
                                                                                                            return Err(Stop::End);
                                                                                                        }
                                                                                                        _ => {}
                                                                                                    }
                                                                                                    ctx.lines_as("Baba Yaga", args![runtime::sprintf(&Val::from("To make %s, I need ^0000ff%d %s, %d %s, %d %s, %d %s and %d Zeny^000000."), &[ctx.call(Function::GetItemName, vec![l_item.clone()])?, runtime::local_get(&l_amount, &Val::from(0), false), ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item_req, &Val::from(0), false)])?, runtime::local_get(&l_amount, &Val::from(1), false), ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item_req, &Val::from(1), false)])?, runtime::local_get(&l_amount, &Val::from(2), false), ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item_req, &Val::from(2), false)])?, runtime::local_get(&l_amount, &Val::from(3), false), ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item_req, &Val::from(3), false)])?, l_zeny_req.clone()])?])?;
                                                                                                    if ((((runtime::op(
                                                                                                        &ctx.var("Zeny").get()?,
                                                                                                        ">=",
                                                                                                        &l_zeny_req.clone(),
                                                                                                    )?
                                                                                                    .is_true()
                                                                                                        && runtime::op(
                                                                                                            &ctx.call(
                                                                                                                Function::CountItem,
                                                                                                                vec![runtime::local_get(
                                                                                                                    &l_item_req,
                                                                                                                    &Val::from(0),
                                                                                                                    false,
                                                                                                                )],
                                                                                                            )?,
                                                                                                            ">=",
                                                                                                            &runtime::local_get(
                                                                                                                &l_amount,
                                                                                                                &Val::from(0),
                                                                                                                false,
                                                                                                            ),
                                                                                                        )?
                                                                                                        .is_true())
                                                                                                        && runtime::op(
                                                                                                            &ctx.call(
                                                                                                                Function::CountItem,
                                                                                                                vec![runtime::local_get(
                                                                                                                    &l_item_req,
                                                                                                                    &Val::from(1),
                                                                                                                    false,
                                                                                                                )],
                                                                                                            )?,
                                                                                                            ">=",
                                                                                                            &runtime::local_get(
                                                                                                                &l_amount,
                                                                                                                &Val::from(1),
                                                                                                                false,
                                                                                                            ),
                                                                                                        )?
                                                                                                        .is_true())
                                                                                                        && runtime::op(
                                                                                                            &ctx.call(
                                                                                                                Function::CountItem,
                                                                                                                vec![runtime::local_get(
                                                                                                                    &l_item_req,
                                                                                                                    &Val::from(2),
                                                                                                                    false,
                                                                                                                )],
                                                                                                            )?,
                                                                                                            ">=",
                                                                                                            &runtime::local_get(
                                                                                                                &l_amount,
                                                                                                                &Val::from(2),
                                                                                                                false,
                                                                                                            ),
                                                                                                        )?
                                                                                                        .is_true())
                                                                                                        && runtime::op(
                                                                                                            &ctx.call(
                                                                                                                Function::CountItem,
                                                                                                                vec![runtime::local_get(
                                                                                                                    &l_item_req,
                                                                                                                    &Val::from(3),
                                                                                                                    false,
                                                                                                                )],
                                                                                                            )?,
                                                                                                            ">=",
                                                                                                            &runtime::local_get(
                                                                                                                &l_amount,
                                                                                                                &Val::from(3),
                                                                                                                false,
                                                                                                            ),
                                                                                                        )?
                                                                                                        .is_true())
                                                                                                    {
                                                                                                        l_total =
                                                                                                            (ctx.call(
                                                                                                                Function::CountItem,
                                                                                                                vec![runtime::local_get(
                                                                                                                    &l_item_req,
                                                                                                                    &Val::from(0),
                                                                                                                    false,
                                                                                                                )],
                                                                                                            )?
                                                                                                            .try_div(runtime::local_get(
                                                                                                                &l_amount,
                                                                                                                &Val::from(0),
                                                                                                                false,
                                                                                                            ))?);
                                                                                                        l_total = (if runtime::op(
                                                                                                            &l_total.clone(),
                                                                                                            "<=",
                                                                                                            &(ctx
                                                                                                                .call(
                                                                                                                    Function::CountItem,
                                                                                                                    vec![
                                                                                                                        runtime::local_get(
                                                                                                                            &l_item_req,
                                                                                                                            &Val::from(1),
                                                                                                                            false,
                                                                                                                        ),
                                                                                                                    ],
                                                                                                                )?
                                                                                                                .try_div(
                                                                                                                    runtime::local_get(
                                                                                                                        &l_amount,
                                                                                                                        &Val::from(1),
                                                                                                                        false,
                                                                                                                    ),
                                                                                                                )?),
                                                                                                        )?
                                                                                                        .is_true()
                                                                                                        {
                                                                                                            l_total.clone()
                                                                                                        } else {
                                                                                                            (ctx.call(
                                                                                                                Function::CountItem,
                                                                                                                vec![runtime::local_get(
                                                                                                                    &l_item_req,
                                                                                                                    &Val::from(1),
                                                                                                                    false,
                                                                                                                )],
                                                                                                            )?
                                                                                                            .try_div(runtime::local_get(
                                                                                                                &l_amount,
                                                                                                                &Val::from(1),
                                                                                                                false,
                                                                                                            ))?)
                                                                                                        });
                                                                                                        l_total = (if runtime::op(
                                                                                                            &l_total.clone(),
                                                                                                            "<=",
                                                                                                            &(ctx
                                                                                                                .call(
                                                                                                                    Function::CountItem,
                                                                                                                    vec![
                                                                                                                        runtime::local_get(
                                                                                                                            &l_item_req,
                                                                                                                            &Val::from(2),
                                                                                                                            false,
                                                                                                                        ),
                                                                                                                    ],
                                                                                                                )?
                                                                                                                .try_div(
                                                                                                                    runtime::local_get(
                                                                                                                        &l_amount,
                                                                                                                        &Val::from(2),
                                                                                                                        false,
                                                                                                                    ),
                                                                                                                )?),
                                                                                                        )?
                                                                                                        .is_true()
                                                                                                        {
                                                                                                            l_total.clone()
                                                                                                        } else {
                                                                                                            (ctx.call(
                                                                                                                Function::CountItem,
                                                                                                                vec![runtime::local_get(
                                                                                                                    &l_item_req,
                                                                                                                    &Val::from(2),
                                                                                                                    false,
                                                                                                                )],
                                                                                                            )?
                                                                                                            .try_div(runtime::local_get(
                                                                                                                &l_amount,
                                                                                                                &Val::from(2),
                                                                                                                false,
                                                                                                            ))?)
                                                                                                        });
                                                                                                        l_total = (if runtime::op(
                                                                                                            &l_total.clone(),
                                                                                                            "<=",
                                                                                                            &(ctx
                                                                                                                .call(
                                                                                                                    Function::CountItem,
                                                                                                                    vec![
                                                                                                                        runtime::local_get(
                                                                                                                            &l_item_req,
                                                                                                                            &Val::from(3),
                                                                                                                            false,
                                                                                                                        ),
                                                                                                                    ],
                                                                                                                )?
                                                                                                                .try_div(
                                                                                                                    runtime::local_get(
                                                                                                                        &l_amount,
                                                                                                                        &Val::from(3),
                                                                                                                        false,
                                                                                                                    ),
                                                                                                                )?),
                                                                                                        )?
                                                                                                        .is_true()
                                                                                                        {
                                                                                                            l_total.clone()
                                                                                                        } else {
                                                                                                            (ctx.call(
                                                                                                                Function::CountItem,
                                                                                                                vec![runtime::local_get(
                                                                                                                    &l_item_req,
                                                                                                                    &Val::from(3),
                                                                                                                    false,
                                                                                                                )],
                                                                                                            )?
                                                                                                            .try_div(runtime::local_get(
                                                                                                                &l_amount,
                                                                                                                &Val::from(3),
                                                                                                                false,
                                                                                                            ))?)
                                                                                                        });
                                                                                                        l_total = (if runtime::op(
                                                                                                            &l_total.clone(),
                                                                                                            "<=",
                                                                                                            &(ctx
                                                                                                                .var("Zeny")
                                                                                                                .get()?
                                                                                                                .try_div(
                                                                                                                    l_zeny_req.clone(),
                                                                                                                )?),
                                                                                                        )?
                                                                                                        .is_true()
                                                                                                        {
                                                                                                            l_total.clone()
                                                                                                        } else {
                                                                                                            (ctx.var("Zeny")
                                                                                                                .get()?
                                                                                                                .try_div(
                                                                                                                    l_zeny_req.clone(),
                                                                                                                )?)
                                                                                                        });
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines_as("Baba Yaga", args![((((Val::from("With your materials, I can make ^0000ff") + l_total.clone()) + Val::from("^000000 ")) + ctx.call(Function::GetItemName, vec![l_item.clone()])?) + Val::from("."))])?;
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines_as("Baba Yaga", args![((Val::from("How many do you want me to make? ") + l_string_s.clone()) + Val::from(". Kehehehehe."))])?;
                                                                                                        ctx.next()?;
                                                                                                        'l2: loop {
                                                                                                            if !(true) {
                                                                                                                break 'l2;
                                                                                                            }
                                                                                                            'b2: {
                                                                                                                let (input, status) =
                                                                                                                    runtime::input_number(
                                                                                                                        ctx, None, None,
                                                                                                                    )?;
                                                                                                                l_input = input;
                                                                                                                ctx.mes("[Baba Yaga]")?;
                                                                                                                if !(l_input
                                                                                                                    .clone()
                                                                                                                    .is_true())
                                                                                                                {
                                                                                                                    ctx.mes("You don't want it?")?;
                                                                                                                    ctx.next()?;
                                                                                                                    ctx.lines_as("Baba Yaga", args!["I don't understand you. What do you want? Kehehehehe."])?;
                                                                                                                    ctx.close_window()?;
                                                                                                                    return Err(Stop::End);
                                                                                                                } else if l_input
                                                                                                                    .clone()
                                                                                                                    .number()?
                                                                                                                    > 100
                                                                                                                {
                                                                                                                    ctx.mes("I said no more than 100 at a time. Kehehehehe.")?;
                                                                                                                    ctx.next()?;
                                                                                                                } else {
                                                                                                                    break 'l2;
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        if ((((runtime::op(
                                                                                                            &ctx.var("Zeny").get()?,
                                                                                                            "<",
                                                                                                            &(l_zeny_req.clone().try_mul(
                                                                                                                l_input.clone(),
                                                                                                            )?),
                                                                                                        )?
                                                                                                        .is_true()
                                                                                                            || runtime::op(
                                                                                                                &ctx.call(
                                                                                                                    Function::CountItem,
                                                                                                                    vec![
                                                                                                                        runtime::local_get(
                                                                                                                            &l_item_req,
                                                                                                                            &Val::from(0),
                                                                                                                            false,
                                                                                                                        ),
                                                                                                                    ],
                                                                                                                )?,
                                                                                                                "<",
                                                                                                                &(runtime::local_get(
                                                                                                                    &l_amount,
                                                                                                                    &Val::from(0),
                                                                                                                    false,
                                                                                                                )
                                                                                                                .try_mul(l_input.clone())?),
                                                                                                            )?
                                                                                                            .is_true())
                                                                                                            || runtime::op(
                                                                                                                &ctx.call(
                                                                                                                    Function::CountItem,
                                                                                                                    vec![
                                                                                                                        runtime::local_get(
                                                                                                                            &l_item_req,
                                                                                                                            &Val::from(1),
                                                                                                                            false,
                                                                                                                        ),
                                                                                                                    ],
                                                                                                                )?,
                                                                                                                "<",
                                                                                                                &(runtime::local_get(
                                                                                                                    &l_amount,
                                                                                                                    &Val::from(1),
                                                                                                                    false,
                                                                                                                )
                                                                                                                .try_mul(l_input.clone())?),
                                                                                                            )?
                                                                                                            .is_true())
                                                                                                            || runtime::op(
                                                                                                                &ctx.call(
                                                                                                                    Function::CountItem,
                                                                                                                    vec![
                                                                                                                        runtime::local_get(
                                                                                                                            &l_item_req,
                                                                                                                            &Val::from(2),
                                                                                                                            false,
                                                                                                                        ),
                                                                                                                    ],
                                                                                                                )?,
                                                                                                                "<",
                                                                                                                &(runtime::local_get(
                                                                                                                    &l_amount,
                                                                                                                    &Val::from(2),
                                                                                                                    false,
                                                                                                                )
                                                                                                                .try_mul(l_input.clone())?),
                                                                                                            )?
                                                                                                            .is_true())
                                                                                                            || runtime::op(
                                                                                                                &ctx.call(
                                                                                                                    Function::CountItem,
                                                                                                                    vec![
                                                                                                                        runtime::local_get(
                                                                                                                            &l_item_req,
                                                                                                                            &Val::from(3),
                                                                                                                            false,
                                                                                                                        ),
                                                                                                                    ],
                                                                                                                )?,
                                                                                                                "<",
                                                                                                                &(runtime::local_get(
                                                                                                                    &l_amount,
                                                                                                                    &Val::from(3),
                                                                                                                    false,
                                                                                                                )
                                                                                                                .try_mul(l_input.clone())?),
                                                                                                            )?
                                                                                                            .is_true())
                                                                                                        {
                                                                                                            ctx.lines_as("Baba Yaga", args!["Where are the materials?", "They are not enough! Check them and come back again! Kehehehehe."])?;
                                                                                                            ctx.close_window()?;
                                                                                                            return Err(Stop::End);
                                                                                                        }
                                                                                                        ctx.lines_as("Baba Yaga", args![((Val::from("Ho, you want ") + l_input.clone()) + Val::from(". Ok, wait here for a while."))])?;
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines(args![((Val::from("- Baba Yaga grinds the ") + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item_req, &Val::from(3), false)])?) + Val::from(" -")), ((Val::from("- and pours the ") + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_item_req, &Val::from(0), false)])?) + Val::from(" -")), "- into the Medicine Bowl, -", "- boils and cools it down -", "- and puts it in the Test Tube -"])?;
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines_as(
                                                                                                            "Baba Yaga",
                                                                                                            args!["Here, done!"],
                                                                                                        )?;
                                                                                                        ctx.call(
                                                                                                            Function::NpcSpecialEffect,
                                                                                                            vec![ctx.constant(
                                                                                                                "EF_PHARMACY_OK",
                                                                                                            )?],
                                                                                                        )?;
                                                                                                        ctx.next()?;
                                                                                                        ctx.lines_as("Baba Yaga", args!["Whenever you want more, come to me."])?;
                                                                                                        ctx.call(
                                                                                                            Function::DelItem,
                                                                                                            vec![
                                                                                                                runtime::local_get(
                                                                                                                    &l_item_req,
                                                                                                                    &Val::from(0),
                                                                                                                    false,
                                                                                                                ),
                                                                                                                (runtime::local_get(
                                                                                                                    &l_amount,
                                                                                                                    &Val::from(0),
                                                                                                                    false,
                                                                                                                )
                                                                                                                .try_mul(l_input.clone())?),
                                                                                                            ],
                                                                                                        )?;
                                                                                                        ctx.call(
                                                                                                            Function::DelItem,
                                                                                                            vec![
                                                                                                                runtime::local_get(
                                                                                                                    &l_item_req,
                                                                                                                    &Val::from(1),
                                                                                                                    false,
                                                                                                                ),
                                                                                                                (runtime::local_get(
                                                                                                                    &l_amount,
                                                                                                                    &Val::from(1),
                                                                                                                    false,
                                                                                                                )
                                                                                                                .try_mul(l_input.clone())?),
                                                                                                            ],
                                                                                                        )?;
                                                                                                        ctx.call(
                                                                                                            Function::DelItem,
                                                                                                            vec![
                                                                                                                runtime::local_get(
                                                                                                                    &l_item_req,
                                                                                                                    &Val::from(2),
                                                                                                                    false,
                                                                                                                ),
                                                                                                                (runtime::local_get(
                                                                                                                    &l_amount,
                                                                                                                    &Val::from(2),
                                                                                                                    false,
                                                                                                                )
                                                                                                                .try_mul(l_input.clone())?),
                                                                                                            ],
                                                                                                        )?;
                                                                                                        ctx.call(
                                                                                                            Function::DelItem,
                                                                                                            vec![
                                                                                                                runtime::local_get(
                                                                                                                    &l_item_req,
                                                                                                                    &Val::from(3),
                                                                                                                    false,
                                                                                                                ),
                                                                                                                (runtime::local_get(
                                                                                                                    &l_amount,
                                                                                                                    &Val::from(3),
                                                                                                                    false,
                                                                                                                )
                                                                                                                .try_mul(l_input.clone())?),
                                                                                                            ],
                                                                                                        )?;
                                                                                                        ctx.var("Zeny").set(
                                                                                                            (ctx.var("Zeny")
                                                                                                                .get()?
                                                                                                                .try_sub(
                                                                                                                    (l_input
                                                                                                                        .clone()
                                                                                                                        .try_mul(
                                                                                                                            l_zeny_req
                                                                                                                                .clone(),
                                                                                                                        )?),
                                                                                                                )?),
                                                                                                        )?;
                                                                                                        ctx.call(
                                                                                                            Function::GetItem,
                                                                                                            vec![
                                                                                                                l_item.clone(),
                                                                                                                l_input.clone(),
                                                                                                            ],
                                                                                                        )?;
                                                                                                    }
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
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                ctx.lines_as("Baba Yaga", args!["What are you, human child."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            BabaYagaRus32Step::OnDisable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Baba Yaga#rus32")])?;
                return Err(Stop::End);
            }
            BabaYagaRus32Step::OnTimer180000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("mosk_dun02"), Val::from("Baba Yaga#rus32::OnMyMobDead")],
                )?;
                step = BabaYagaRus32Step::OnMyMobDead;
                continue 'machine;
            }
            BabaYagaRus32Step::OnMyMobDead => {
                ctx.call(Function::EnableNpc, vec![Val::from("Baba Yaga#rus32")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn baba_yaga_rus32(ctx: &Ctx) -> Script {
    baba_yaga_rus32_run(ctx, BabaYagaRus32Step::Start, Vec::new()).map(|_| ())
}

pub fn baba_yaga_rus32_ondisable(ctx: &Ctx) -> Script {
    baba_yaga_rus32_run(ctx, BabaYagaRus32Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn baba_yaga_rus32_ontimer180000(ctx: &Ctx) -> Script {
    baba_yaga_rus32_run(ctx, BabaYagaRus32Step::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn baba_yaga_rus32_onmymobdead(ctx: &Ctx) -> Script {
    baba_yaga_rus32_run(ctx, BabaYagaRus32Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn cow_rus33_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn cow_rus33(ctx: &Ctx) -> Script {
    cow_rus33_body(ctx, Vec::new()).map(|_| ())
}

fn cow_rus33_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    let mut l_npc_num = Val::from(0);
    let mut l_other_npc: Vec<Val> = Vec::new();
    ctx.lines_as("Cow", args!["Moo..."])?;
    if ctx.var("rhea_rus_main").get()? == 11 {
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Hah, t, that is Baba Yaga's cow?!", "Y, you..."],
        )?;
        ctx.next()?;
        ctx.mes("- You approach to the cow carefully and hold its nape -")?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["You! Go home now!"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
        ctx.next()?;
        ctx.mes("- The cow seems surprised, jumps and tries to attack you !! -")?;
        ctx.next()?;
        l_npc_num = runtime::atoi(&runtime::charat(
            &ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?,
            &Val::from(4),
        )?);
        if l_npc_num.clone() == 3 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_other_npc, &Val::from(base + 0), Val::from(4), false);
            runtime::local_set(&mut l_other_npc, &Val::from(base + 1), Val::from(5), false);
        } else if l_npc_num.clone() == 4 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_other_npc, &Val::from(base + 0), Val::from(3), false);
            runtime::local_set(&mut l_other_npc, &Val::from(base + 1), Val::from(5), false);
        } else {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_other_npc, &Val::from(base + 0), Val::from(3), false);
            runtime::local_set(&mut l_other_npc, &Val::from(base + 1), Val::from(4), false);
        }
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])? == 3 {
            ctx.mes("- You almost get hit and dodge its attack and cast the spell !! -")?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_input_s = input;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![((Val::from("") + l_input_s.clone()) + Val::from(" !!!"))],
            )?;
            if l_input_s.clone() == "Good feed is orange-flavored" {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_TELEPORTATION2")?])?;
                ctx.next()?;
                ctx.mes("- You cast the spell in a hurry and the cow is surronded by the light and disappears !! -")?;
                ctx.call(Function::DisableNpc, vec![])?;
                ctx.call(
                    Function::EnableNpc,
                    vec![(Val::from("Cow#rus3") + runtime::local_get(&l_other_npc, &Val::from(0), false))],
                )?;
                ctx.next()?;
            } else {
                ctx.next()?;
                ctx.lines_as("Cow", args!["...Moo..."])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                ctx.next()?;
                ctx.mes("- The cow, seems to gibe at you, looks at you quickly and runs to bushes !! -")?;
                ctx.call(Function::DisableNpc, vec![])?;
                if ctx.call(Function::Rand, vec![Val::from(2)])?.is_true() {
                    ctx.call(
                        Function::EnableNpc,
                        vec![(Val::from("Cow#rus3") + runtime::local_get(&l_other_npc, &Val::from(0), false))],
                    )?;
                } else {
                    ctx.call(
                        Function::EnableNpc,
                        vec![(Val::from("Cow#rus3") + runtime::local_get(&l_other_npc, &Val::from(1), false))],
                    )?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Akkkk?! Eh!? The s-spell..."],
            )?;
            ctx.next()?;
            ctx.mes("- You hesitate, the cow, seems to gibe at you, passes by you and runs to bushes !! -")?;
            ctx.call(Function::DisableNpc, vec![])?;
            if ctx.call(Function::Rand, vec![Val::from(2)])?.is_true() {
                ctx.call(
                    Function::EnableNpc,
                    vec![(Val::from("Cow#rus3") + runtime::local_get(&l_other_npc, &Val::from(0), false))],
                )?;
            } else {
                ctx.call(
                    Function::EnableNpc,
                    vec![(Val::from("Cow#rus3") + runtime::local_get(&l_other_npc, &Val::from(1), false))],
                )?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["What a surprise. The cow is bad like its owner."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Ok, then. Let's get back to Baba Yaga..."],
        )?;
        ctx.var("rhea_rus_main").set(Val::from(41))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rhea_rus_main").get()? == 41 {
        ctx.next()?;
        ctx.mes("- The cow is tied around its neck and eats grass comfortably -")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I must return to Baba Yaga..."],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn cow_rus33_ontouch(ctx: &Ctx) -> Script {
    cow_rus33_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn noisy_coffin_rus36_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_break_while = Val::from(0);
    if ctx.var("rhea_rus_main").get()?.number()? < 19 {
        ctx.lines_as("Noisy Coffin", args!["Isn't it good to be alive? Isn't it to live?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Noisy Coffin",
            args!["Don't you want to exchange lives with me? Exchange with me, exchange!"],
        )?;
        if ctx.var("rhea_rus_main").get()?.number()? < 16 {
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Ehh, that's one noisy coffin."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Noisy Coffin",
            args![
                "I was alive! I was alive!",
                "What is different between you and me? Why are you still alive? Why didn't you die? Why did I die?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["... How can I silence him..."],
        )?;
        ctx.next()?;
        'l1: loop {
            if !(l_break_while.clone() == 0) {
                break 'l1;
            }
            'b1: {
                match runtime::select_values(ctx, &[Val::from("Pray:Sing a hymn:Pour Holy Water on him:Kick him")])? {
                    1 => {
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...I'll pray."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Heaven, earth, and wind belong to God, there is no place for evil."],
                        )?;
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BLESSING")?])?;
                        ctx.next()?;
                        ctx.mes("[Noisy Coffin]")?;
                        if ctx.var("rhea_rus_main").get()? == 17 {
                            ctx.mes("Heek, heeee! Heeeeeek!")?;
                            ctx.var("rhea_rus_main").set(Val::from(18))?;
                        } else {
                            ctx.mes("What was that? What was that for? Was it fun?")?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["......................................"],
                            )?;
                            l_break_while = Val::from(1);
                        }
                    }
                    2 => {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Hmm, hmm! I'll sing a hymn. The first phrase 'Holy god against evils'!!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "- The covetous, the perverse -",
                                "- All the evils hateful -",
                                "- Will be droven off this land -"
                            ],
                        )?;
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_GLORIA")?])?;
                        ctx.next()?;
                        ctx.mes("[Noisy Coffin]")?;
                        if ctx.var("rhea_rus_main").get()? == 18 {
                            ctx.mes("Heek, heeee! Heeeeeek!")?;
                            ctx.var("rhea_rus_main").set(Val::from(19))?;
                        } else {
                            ctx.mes("Perverse! The perverse!! Drive away!! What was that song?! Was it fun?!")?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["......................................"],
                            )?;
                            l_break_while = Val::from(1);
                        }
                    }
                    3 => {
                        if ctx.call(Function::CountItem, vec![Val::from(523)])?.is_true() {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["If you don't shut up, I will pour Holy Water on you!"],
                            )?;
                            ctx.next()?;
                            ctx.mes("- You open the bottle of Holy Water and pour it around the coffin carefully !! -")?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SPHERE")?])?;
                            ctx.next()?;
                            ctx.mes("[Noisy Coffin]")?;
                            if ctx.var("rhea_rus_main").get()? == 16 {
                                ctx.mes("Heek, heeee! Heeeeeek!")?;
                                ctx.call(Function::DelItem, vec![Val::from(523), Val::from(1)])?;
                                ctx.var("rhea_rus_main").set(Val::from(17))?;
                            } else {
                                ctx.mes("Ah, cold! No, it's cool! Was it fun? Was it fun?!")?;
                                ctx.call(Function::DelItem, vec![Val::from(523), Val::from(1)])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["......................................"],
                                )?;
                                l_break_while = Val::from(1);
                            }
                        } else {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Yeah, where's the Holy water?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["...... I don't have any..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["......................................"],
                            )?;
                            l_break_while = Val::from(1);
                        }
                    }
                    4 => {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["...If you don't shut up...", "......I will kick you!!"],
                        )?;
                        ctx.next()?;
                        ctx.mes("- You close your eyes, take a deep breathe and kick the coffin with the might of God !! -")?;
                        ctx.next()?;
                        ctx.mes("[Noisy Coffin]")?;
                        if ctx.var("rhea_rus_main").get()? == 19 {
                            ctx.mes("Heeeek! Heeeeeee!!!! I am scared! Stop it!!")?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HOLYHIT")?])?;
                            ctx.var("rhea_rus_main").set(Val::from(20))?;
                        } else {
                            ctx.mes("Slam! Slam! I like noise! Do it more, more, more!")?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["......................................"],
                            )?;
                        }
                        l_break_while = Val::from(1);
                    }
                    _ => {}
                }
                ctx.next()?;
            }
        }
        if ctx.var("rhea_rus_main").get()?.number()? < 20 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["... Not effective..."],
            )?;
            ctx.var("rhea_rus_main").set(Val::from(16))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![".............If you make noise once more, you will see more than you imagine..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Noisy Coffin", args!["................................"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Hu, the job has been done.", "Let's get back to Baba Yaga."],
        )?;
        ctx.var("rhea_rus_main").set(Val::from(42))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rhea_rus_main").get()? == 20 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![".............If you make noise once more, you will see more than you imagine..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Noisy Coffin", args!["................................"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Hu, the job has been done.", "Let's get back to Baba Yaga."],
        )?;
        ctx.var("rhea_rus_main").set(Val::from(42))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rhea_rus_main").get()? == 42 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![".............If you make noise once more, you will see more than you imagine..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Noisy Coffin", args!["................................"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Hu, the job has been done.", "Let's get back to Baba Yaga."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Noisy Coffin", args!["............................."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn noisy_coffin_rus36(ctx: &Ctx) -> Script {
    noisy_coffin_rus36_body(ctx, Vec::new()).map(|_| ())
}

fn old_treasure_box_rus37_run(ctx: &Ctx, mut step: OldTreasureBoxRus37Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            OldTreasureBoxRus37Step::Start => {
                if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 3500 {
                    ctx.lines(args!["You are carrying too much!", "Lose some weight and try again."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if (ctx.var("rhea_rus_main").get()?.number()? > 20 && ctx.var("rhea_rus_main").get()?.number()? < 25) {
                    ctx.mes("- It is locked -")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("rhea_rus_main").get()? == 25 {
                    ctx.mes("- Around the locked box, a women having snakes on her head is engraved -")?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Does this work..."])?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Insert the key into the lock")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.mes("- As soon as you insert the key into the lock, the eyes of the woman moves and she looks at you -")?;
                    ctx.next()?;
                    ctx.mes("- Her eyes are made from jewels !! -")?;
                    if ctx.call(Function::CountItem, vec![Val::from(747)])?.is_true() {
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Arrrrrk? What is this!!"],
                        )?;
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FLASHER")?])?;
                        ctx.next()?;
                    } else {
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_BLIND")?, Val::from(60000), Val::from(0)],
                        )?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Old Treasure Box#rus37::OnCall")])?;
                        ctx.close_window()?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Old Treasure Box#rus37::OnDisable")])?;
                        return Err(Stop::End);
                    }
                    ctx.mes("- You wave your arms and cover your eyes. The mirror inside your bag reflects the light from her eyes accidentally !! -")?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FLASHER")?])?;
                    ctx.next()?;
                    ctx.mes("- The light reflected goes to the woman shape and fumes comes from the box and it opens !! -")?;
                    ctx.next()?;
                    ctx.mes("- ^0000ff You find Baba Yaga's spoon !!^000000 -")?;
                    ctx.var("rhea_rus_main").set(Val::from(43))?;
                    ctx.call(Function::GetItem, vec![Val::from(7880), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("rhea_rus_main").get()? == 43 {
                    ctx.mes("- The treasure box where you found the silver spoons -")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
            OldTreasureBoxRus37Step::OnCall => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("treasure01"),
                        Val::from(164),
                        Val::from(58),
                        Val::from("Medusa"),
                        Val::from(1148),
                        Val::from(1),
                        Val::from("Old Treasure Box#rus37::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("treasure01"),
                        Val::from(157),
                        Val::from(59),
                        Val::from("Obeaune"),
                        Val::from(1044),
                        Val::from(1),
                        Val::from("Old Treasure Box#rus37::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("treasure01"),
                        Val::from(164),
                        Val::from(54),
                        Val::from("Obeaune"),
                        Val::from(1044),
                        Val::from(1),
                        Val::from("Old Treasure Box#rus37::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("treasure01"),
                        Val::from(163),
                        Val::from(63),
                        Val::from("Obeaune"),
                        Val::from(1044),
                        Val::from(1),
                        Val::from("Old Treasure Box#rus37::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("treasure01"),
                        Val::from(168),
                        Val::from(59),
                        Val::from("Obeaune"),
                        Val::from(1044),
                        Val::from(1),
                        Val::from("Old Treasure Box#rus37::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            OldTreasureBoxRus37Step::OnDisable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Old Treasure Box#rus37")])?;
                return Err(Stop::End);
            }
            OldTreasureBoxRus37Step::OnTimer180000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("treasure01"), Val::from("Old Treasure Box#rus37::OnMyMobDead")],
                )?;
                step = OldTreasureBoxRus37Step::OnMyMobDead;
                continue 'machine;
            }
            OldTreasureBoxRus37Step::OnMyMobDead => {
                ctx.call(Function::EnableNpc, vec![Val::from("Old Treasure Box#rus37")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn old_treasure_box_rus37(ctx: &Ctx) -> Script {
    old_treasure_box_rus37_run(ctx, OldTreasureBoxRus37Step::Start, Vec::new()).map(|_| ())
}

pub fn old_treasure_box_rus37_oncall(ctx: &Ctx) -> Script {
    old_treasure_box_rus37_run(ctx, OldTreasureBoxRus37Step::OnCall, Vec::new()).map(|_| ())
}

pub fn old_treasure_box_rus37_ondisable(ctx: &Ctx) -> Script {
    old_treasure_box_rus37_run(ctx, OldTreasureBoxRus37Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn old_treasure_box_rus37_ontimer180000(ctx: &Ctx) -> Script {
    old_treasure_box_rus37_run(ctx, OldTreasureBoxRus37Step::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn old_treasure_box_rus37_onmymobdead(ctx: &Ctx) -> Script {
    old_treasure_box_rus37_run(ctx, OldTreasureBoxRus37Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn old_bed_rus38_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("rhea_rus_main").get()?.number()? >= 21 && ctx.var("rhea_rus_main").get()?.number()? < 26) {
        ctx.mes("- An old bed covered with dust and must -")?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("On the bed:Under the bed:Bed sheet")])? {
            1 => {
                ctx.mes("- Mushrooms grow on the bed -")?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...Life......"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.mes("- You look at the under the bed -")?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Ouch?!"])?;
                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
                ctx.call(Function::PercentHeal, vec![Val::from(-5), Val::from(0)])?;
                ctx.next()?;
                ctx.mes("- Something unidentified bites your hand !! -")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
        if ctx.var("rhea_rus_main").get()? != 21 {
            ctx.mes("- While running in a hurry, the sheet is a bit torn -")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.mes("- You raise the dirty sheet that has many holes -")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["...?! What is this?"],
        )?;
        ctx.next()?;
        ctx.mes("- There is a scar on the sheet that seems to have the location !! -")?;
        ctx.call(
            Function::ViewPoint,
            vec![Val::from(1), Val::from(165), Val::from(58), Val::from(1), Val::from(16711680)],
        )?;
        ctx.call(
            Function::ViewPoint,
            vec![Val::from(1), Val::from(61), Val::from(183), Val::from(2), Val::from(16711680)],
        )?;
        ctx.call(
            Function::ViewPoint,
            vec![Val::from(1), Val::from(98), Val::from(118), Val::from(3), Val::from(16711680)],
        )?;
        ctx.call(
            Function::ViewPoint,
            vec![Val::from(1), Val::from(27), Val::from(115), Val::from(4), Val::from(16711680)],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...This may be?!"])?;
        ctx.next()?;
        ctx.lines_as("Voice unidentified", args!["Who is there!?"])?;
        ctx.call(
            Function::ViewPoint,
            vec![Val::from(2), Val::from(165), Val::from(58), Val::from(1), Val::from(65280)],
        )?;
        ctx.call(
            Function::ViewPoint,
            vec![Val::from(2), Val::from(61), Val::from(183), Val::from(2), Val::from(65280)],
        )?;
        ctx.call(
            Function::ViewPoint,
            vec![Val::from(2), Val::from(98), Val::from(118), Val::from(3), Val::from(65280)],
        )?;
        ctx.call(
            Function::ViewPoint,
            vec![Val::from(2), Val::from(27), Val::from(115), Val::from(4), Val::from(65280)],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HUK")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Eek, it would be best to run away now!"],
        )?;
        ctx.var("rhea_rus_main").set(Val::from(22))?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("treasure01"), Val::from(68), Val::from(28)])?;
    }
    return Err(Stop::End);
}

pub fn old_bed_rus38(ctx: &Ctx) -> Script {
    old_bed_rus38_body(ctx, Vec::new()).map(|_| ())
}
