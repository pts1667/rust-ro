use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn wall_rus04_run(ctx: &Ctx, mut step: WallRus04Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    let mut l_rus_food = Val::from(0);
    let mut l_speak01 = Val::from(0);
    'machine: loop {
        match step {
            WallRus04Step::Start => {
                if ctx.var("rhea_rus_main").get()?.number()? < 5 {
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...?!"])?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_QUESTION")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("rhea_rus_main").get()? == 5 {
                        l_speak01 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                        if l_speak01.clone() == 3 {
                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...?!"])?;
                            ctx.call(
                                Function::Emotion,
                                vec![
                                    ctx.constant("ET_QUESTION")?,
                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                ],
                            )?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Gray Wolf#rus05::OnEnable")])?;
                            ctx.next()?;
                        } else {
                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...?!"])?;
                            ctx.call(
                                Function::Emotion,
                                vec![
                                    ctx.constant("ET_QUESTION")?,
                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Girl's Voice", args!["Who is there? Gray Wolf?!", "Wolf, who are you with?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Gray Wolf",
                            args!["This is an adventurer from another land. He is not afraid of Koshei. He may be able to help you."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Girl's Voice", args!["Help me? Do you think he can stop Koshei?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Ah, hold on. Who is Koshei and who is talking?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Girl's Voice", args!["Koshei.. He is an immortal and endless evil. I fought against him a long tim ago. After a fierce fight, I had to seal him in the darkness because he is immortal."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Girl's Voice",
                            args![
                                "And as a result I became imprisoned in this wall.",
                                "He has found a way to break away from his seal and I fear for the worst."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Girl's Voice",
                            args![
                                "My name is Maria Morebna. Help me exterminate Koshei. I want to keep peace in Moscovia. Can you help me?"
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Ok, I can")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["So, how can I help you?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Gray Wolf",
                            args![
                                "First, you need to find a way to get her out of there.",
                                "I've heard about a '^0000ffGolden Key^000000' that can open anything. There is a keymaker in town."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Gray Wolf", args!["As you see, I look like a monster so I am not able to go there. I want you to find this keymaker and ask about the '^0000ffGolden Key^000000'."])?;
                        ctx.next()?;
                        ctx.lines_as("Girl's Voice", args!["Please don't forget about me."])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Gray Wolf#rus05::OnDisable")])?;
                        ctx.var("rhea_rus_main").set(Val::from(6))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if (ctx.var("rhea_rus_main").get()?.number()? > 5 && ctx.var("rhea_rus_main").get()?.number()? < 8) {
                            ctx.lines_as(
                                "Girl's Voice",
                                args![
                                    "Help me get out of here.",
                                    "I hope you can help me find that '^0000ffGolden Key^000000' that can open anything."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Girl's Voice",
                                args!["I've heard about a keymaker in Moscovia who knows how to make a '^0000ffGolden Key^000000'."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("rhea_rus_main").get()? == 8 {
                                if ctx.var("rhea_rus_quiz").get()?.number()? < 3 {
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
                                    ctx.lines_as("Girl's Voice", args!["Did you get the key?"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["...I need golden thread. I heard that I was able to get it from Marozka. He...."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Girl's Voice", args!["Marozka... He is the owner of winter and dwells in the deep underground. He won't let just anyone see him."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Girl's Voice",
                                        args![
                                            "He has monsters that guard his underground lair. It's very dangerous. But, I know where he is."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Girl's Voice",
                                        args!["What do you think? I can open a portal to the cave Do you want to go there now?"],
                                    )?;
                                    ctx.next()?;
                                    if Val::from(runtime::select_values(ctx, &[Val::from("Not yet...:Open the portal!")])?) == 1 {
                                        ctx.lines_as("Girl's Voice", args!["... When you're ready, just tell me."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines_as("Girl's Voice", args!["...Ok, be careful There are a lot of monsters there."])?;
                                    ctx.call(Function::DoNpcEvent, vec![Val::from("1#rus27::OnEnable")])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as("Girl's Voice", args!["Have you.. got the key?"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Not yet... Give me a second."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Girl's Voice", args!["If you gather all the necessary things, go to Moscovia and find the keymaker. Do me this favor, please."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if (ctx.var("rhea_rus_main").get()?.number()? > 8 && ctx.var("rhea_rus_main").get()?.number()? < 47) {
                                    ctx.lines_as("Girl's Voice", args!["Have you.. got the key?"])?;
                                    ctx.next()?;
                                    if ((ctx.var("rhea_rus_ring").get()?.number()? > 8 && ctx.var("rhea_rus_hair").get()?.number()? > 8)
                                        && ctx.var("rhea_rus_quiz").get()?.number()? > 29)
                                    {
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["I collected every meterial for the key, But have not yet got the key..."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Girl's Voice", args!["Go to Moscovia and find the keymaker."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["Not yet... Give me a second."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Girl's Voice", args!["If you gather all the things necessary, go to Moscovia and find the keymaker. Do me this favor, please."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("rhea_rus_main").get()? == 47 {
                                        if ctx.call(Function::CountItem, vec![Val::from(7876)])?.is_true() {
                                            if ctx.var("$@rus_req02").get()? == 1 {
                                                ctx.lines_as(
                                                    "Girl's Voice",
                                                    args!["I can feel Koshei's energy.. It is not a time to use the key..."],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            ctx.var("$@rus_req02").set(Val::from(1))?;
                                            ctx.lines_as("Girl's Voice", args!["Did you get the key?"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["I did. So, how do I use it?"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Girl's Voice", args!["Listen very carefully!", "Left the key up, and shout '^0000ffThe Free wind blows and gets you wherever you want^000000'."])?;
                                            ctx.next()?;
                                            let (input, status) = runtime::input_text(ctx, None, None)?;
                                            l_input_s = input;
                                            if l_input_s.clone() == "The Free wind blows and gets you wherever you want" {
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args![((Val::from("^0000ff ") + l_input_s.clone()) + Val::from(" !! ^000000"))],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines(args![
                                                    "- The key begins to glow -",
                                                    "- wind begins to blow -",
                                                    "- from somewhere -"
                                                ])?;
                                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BEGINSPELL")?])?;
                                                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FREEZED")?])?;
                                                ctx.next()?;
                                            } else {
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args![((Val::from(" ") + l_input_s.clone()) + Val::from(" !! "))],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Girl's Voice", args!["........................................................................................."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Girl's Voice", args!["You must've said it wrong..."])?;
                                                ctx.var("$@rus_req02").set(Val::from(0))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            ctx.lines(args!["- The wind disappears and -", "- a very young girl appears -"])?;
                                            ctx.call(Function::DoNpcEvent, vec![Val::from("Maria Morebna#rus46::OnEnable")])?;
                                            ctx.call(
                                                Function::NpcSpecialEffect,
                                                vec![ctx.constant("EF_FREEZED")?, ctx.constant("AREA")?, Val::from("Maria Morebna#rus46")],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Maria Morebna",
                                                args![
                                                    "Thank you!",
                                                    "I'm finally out of there! Now I can.....................aaaaak!!!...."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(
                                                Function::NpcSpecialEffect,
                                                vec![ctx.constant("EF_HIT2")?, ctx.constant("AREA")?, Val::from("Maria Morebna#rus46")],
                                            )?;
                                            ctx.call(
                                                Function::NpcSpecialEffect,
                                                vec![
                                                    ctx.constant("EF_DARKBREATH")?,
                                                    ctx.constant("AREA")?,
                                                    Val::from("Maria Morebna#rus46"),
                                                ],
                                            )?;
                                            ctx.call(
                                                Function::NpcSpecialEffect,
                                                vec![ctx.constant("EF_DEVIL")?, ctx.constant("AREA")?, Val::from("Maria Morebna#rus46")],
                                            )?;
                                            ctx.lines(args!["- Maria is attacked -", "- and falls down!! -"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["Maria?! What happened?!"],
                                            )?;
                                            ctx.call(
                                                Function::StartStatus,
                                                vec![ctx.constant("SC_CURSE")?, Val::from(60000), Val::from(0)],
                                            )?;
                                            ctx.call(
                                                Function::Emotion,
                                                vec![
                                                    ctx.constant("ET_HUK")?,
                                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Koshei, the Immortal", args!["I really want to thank you, human. Maria was imprisoned in the wall so I couldn't do anything!"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Koshei, the Immortal", args!["She'll thank me for giving her a quick death!!"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Koshei, the Immortal", args!["But you, human... You are here to disturb me?"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Koshei, the Immortal", args!["No one can stop me! Now, die!!!"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Gray Wolf", args!["Watch out!!"])?;
                                            ctx.next()?;
                                            ctx.call(Function::DoNpcEvent, vec![Val::from("Gray Wolf#rus05::OnEnable")])?;
                                            ctx.lines_as("Koshei, the Immortal", args!["Gray Wolf...", "You think you can fight me?!"])?;
                                            ctx.next()?;
                                            ctx.call(
                                                Function::NpcSpecialEffect,
                                                vec![ctx.constant("EF_FIREHIT")?, ctx.constant("AREA")?, Val::from("Gray Wolf#rus05")],
                                            )?;
                                            ctx.lines(args![
                                                "- Staggering from -",
                                                "- Koshei's flames, -",
                                                "- Gray Wolf quickly tell me -"
                                            ])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Gray Wolf",
                                                args![
                                                    "Listen, I can still help Maria if I treat her now.",
                                                    "I will flee with her and leave you to deal with Koshei!!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::DoNpcEvent, vec![Val::from("Gray Wolf#rus05::OnDisable")])?;
                                            ctx.call(Function::DoNpcEvent, vec![Val::from("Maria Morebna#rus46::OnDisable")])?;
                                            ctx.lines_as("Koshei, the Immortal", args!["Stop! Where do you think you're going?!!!"])?;
                                            ctx.call(Function::DelItem, vec![Val::from(7876), Val::from(1)])?;
                                            ctx.var("rhea_rus_main").set(Val::from(48))?;
                                            ctx.call(
                                                Function::Monster,
                                                vec![
                                                    Val::from("mosk_dun01"),
                                                    Val::from(45),
                                                    Val::from(256),
                                                    Val::from("Koshei, the Immortal"),
                                                    Val::from(1890),
                                                    Val::from(1),
                                                    Val::from("Wall#rus04::OnMyMobDead"),
                                                ],
                                            )?;
                                            ctx.call(Function::DoNpcEvent, vec![Val::from("Koshei#rus47::OnEnable")])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as("Girl's Voice", args!["Have you.. got the key?"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["Sure! Here it...", "...............Errr?"],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.var("rhea_rus_main").get()? == 48 {
                                            if ctx.var("$@rus_req02").get()? == 1 {
                                                ctx.lines_as("Gray Wolf's voice", args!["I can sense Koshei's presence."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Gray Wolf's voice",
                                                    args![
                                                        "I can still help Maria if I treat her now.",
                                                        "I will flee with her and leave you to deal with Koshei!!"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["...Maria...", "I... could do nothing..."],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Gray Wolf", args!["No, it's not too late!"])?;
                                            ctx.call(
                                                Function::Emotion,
                                                vec![
                                                    ctx.constant("ET_SURPRISE")?,
                                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                                ],
                                            )?;
                                            ctx.call(Function::DoNpcEvent, vec![Val::from("Gray Wolf#rus05::OnEnable")])?;
                                            ctx.next()?;
                                            ctx.lines_as("Gray Wolf", args!["She's alive.", "But, she's still unstable."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Gray Wolf", args!["To save her, I need 'Life Water' and 'Death Water'. Find Baba Yaga, she'll know what to do."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Gray Wolf",
                                                args!["I can't keep her alive for much longer. You must find Baba Yaga."],
                                            )?;
                                            ctx.var("rhea_rus_main").set(Val::from(49))?;
                                            ctx.close_window()?;
                                            ctx.call(Function::DoNpcEvent, vec![Val::from("Gray Wolf#rus05::OnDisable")])?;
                                            return Err(Stop::End);
                                        } else {
                                            if (ctx.var("rhea_rus_main").get()?.number()? > 48
                                                && ctx.var("rhea_rus_main").get()?.number()? < 51)
                                            {
                                                ctx.lines_as("Gray Wolf's voice", args!["But, she's still unstable."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Gray Wolf's voice", args!["To save her, I need 'Life Water' and 'Death Water'. Find Baba Yaga, she'll know what to do."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Gray Wolf's voice",
                                                    args!["I can't keep her alive for much longer. You must find Baba Yaga."],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                if ctx.var("rhea_rus_main").get()? == 51 {
                                                    if ctx.var("$@rus_req02").get()? == 1 {
                                                        ctx.lines_as("Gray Wolf's voice", args!["I can sense Koshei's presence."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Gray Wolf's voice",
                                                            args![
                                                                "I can still help Maria if I treat her now.",
                                                                "I will flee with her and leave you to deal with Koshei!!"
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    ctx.lines_as(
                                                        "Gray Wolf's voice",
                                                        args!["Did you get the 'Life Water' and 'Death Water'?"],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args!["I've got them! How's Maria?!"],
                                                    )?;
                                                    ctx.call(Function::DoNpcEvent, vec![Val::from("Gray Wolf#rus05::OnEnable")])?;
                                                    ctx.call(Function::DoNpcEvent, vec![Val::from("Maria Morebna#rus46::OnEnable")])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Gray Wolf", args!["I fear that her spirit has gone. Severe wounds and curses are hindering her spirit from coming back. But we can save her with that water."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args!["Ok... First..."],
                                                    )?;
                                                    ctx.next()?;
                                                    if Val::from(runtime::select_values(ctx, &[Val::from("Death Water:Life Water")])?) == 1
                                                    {
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args!["I hope this works..."],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines(args!["- I pour the Death Water -", "- on Maria's stiff body -"])?;
                                                        ctx.next()?;
                                                    } else {
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args!["I hope this works..."],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines(args!["- I pour the Life Water -", "- on Maria's stiff body -"])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Maria Morebna", args![".........................."])?;
                                                        ctx.next()?;
                                                        ctx.mes("- Nothing changed -")?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args!["Huk, was this wrong...?!"],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Gray Wolf", args!["What are you doing?! I told you we did not have much time! What are you going to do now?!"])?;
                                                        ctx.next()?;
                                                        ctx.call(Function::DoNpcEvent, vec![Val::from("Gray Wolf#rus05::OnDisable")])?;
                                                        ctx.call(Function::DoNpcEvent, vec![Val::from("Maria Morebna#rus46::OnDisable")])?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args!["I am so sorry. But some water is left..."],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    ctx.lines(args!["- The wounds and -", "- curses on her body -", "- are removed !! -"])?;
                                                    ctx.call(
                                                        Function::NpcSpecialEffect,
                                                        vec![
                                                            ctx.constant("EF_ABSORBSPIRITS")?,
                                                            ctx.constant("AREA")?,
                                                            Val::from("Maria Morebna#rus46"),
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args!["Ok! Next..."],
                                                    )?;
                                                    ctx.next()?;
                                                    if Val::from(runtime::select_values(ctx, &[Val::from("Death Water:Life Water")])?) == 1
                                                    {
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args!["Pour the 'Death Water' again..."],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args!["...!? No, not 'Death Water'.. I should pour 'Life Water'......."],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Gray Wolf",
                                                            args!["............................", "Can I trust you...?"],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args!["Hehehe, you can trust me. Next is the 'Life Water'."],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.mes("- I pour 'Life Water' on Maria -")?;
                                                        ctx.next()?;
                                                    } else {
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args!["Ok, next the 'Life Water'."],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines(args!["- I pour the Life Water -", "- on Maria's stiff body  -"])?;
                                                        ctx.next()?;
                                                    }
                                                    ctx.mes("- !! -")?;
                                                    ctx.call(
                                                        Function::NpcSpecialEffect,
                                                        vec![
                                                            ctx.constant("EF_RESURRECTION")?,
                                                            ctx.constant("AREA")?,
                                                            Val::from("Maria Morebna#rus46"),
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Gray Wolf", args!["Success!!"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Maria Morebna", args!["Ah, did I...?!", "You saved me. I really appreicate it. Thank you for helping us fight against Koshei."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Maria Morebna",
                                                        args!["And you, Gray Wolf.. Thank you for protecting me while I was helpless."],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Maria Morebna", args!["Koshei wasn't completely destroyed. But, his power has weakened, it shouldn't be hard to seal his power without me also being sealed."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Maria Morebna", args!["Koshei and I are bound by magic. If one's power should be sealed, the other must be sealed as well."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Maria Morebna", args!["But the original seal weakened over time and Koshei was able to get out of his confinement. That is why I asked you to get me out of here."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Maria Morebna", args!["He realized that I was trying to break free and waited here for anyone who would help me. I never ever expected it..."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Maria Morebna",
                                                        args!["Anyway, I'm so grateful for everything you did."],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Maria Morebna", args!["Please accept my poor reward, I hope that it will be of some help for such a brave adventurer. I hope that the gods will bless you on your journey."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Maria Morebna", args!["It's been a long time since I've felt the sunlight, fresh wind and the scent of grass. I'm so grateful."])?;
                                                    ctx.var("rhea_rus_main").set(Val::from(52))?;
                                                    l_rus_food = ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])?;
                                                    let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])?;
                                                    if subject1 == 1 {
                                                        ctx.call(Function::GetItem, vec![Val::from(12093), Val::from(1)])?;
                                                    } else if subject1 == 2 {
                                                        ctx.call(Function::GetItem, vec![Val::from(12088), Val::from(1)])?;
                                                    } else if subject1 == 3 {
                                                        ctx.call(Function::GetItem, vec![Val::from(12073), Val::from(1)])?;
                                                    } else if subject1 == 4 {
                                                        ctx.call(Function::GetItem, vec![Val::from(12078), Val::from(1)])?;
                                                    } else if subject1 == 5 {
                                                        ctx.call(Function::GetItem, vec![Val::from(12083), Val::from(1)])?;
                                                    } else if subject1 == 6 {
                                                        ctx.call(Function::GetItem, vec![Val::from(12098), Val::from(1)])?;
                                                    }
                                                    {
                                                        if ctx.var("BaseLevel").get()?.number()? < 56 {
                                                            ctx.call(Function::GetExperience, vec![Val::from(13000), Val::from(3500)])?;
                                                        } else {
                                                            if ctx.var("BaseLevel").get()?.number()? < 61 {
                                                                ctx.call(Function::GetExperience, vec![Val::from(24600), Val::from(6150)])?;
                                                            } else {
                                                                if ctx.var("BaseLevel").get()?.number()? < 66 {
                                                                    ctx.call(
                                                                        Function::GetExperience,
                                                                        vec![Val::from(42420), Val::from(10605)],
                                                                    )?;
                                                                } else {
                                                                    if ctx.var("BaseLevel").get()?.number()? < 71 {
                                                                        ctx.call(
                                                                            Function::GetExperience,
                                                                            vec![Val::from(64892), Val::from(16223)],
                                                                        )?;
                                                                    } else {
                                                                        if ctx.var("BaseLevel").get()?.number()? < 76 {
                                                                            ctx.call(
                                                                                Function::GetExperience,
                                                                                vec![Val::from(164908), Val::from(41227)],
                                                                            )?;
                                                                        } else if ctx.var("BaseLevel").get()?.number()? < 81 {
                                                                            ctx.call(
                                                                                Function::GetExperience,
                                                                                vec![Val::from(276292), Val::from(69073)],
                                                                            )?;
                                                                        } else if ctx.var("BaseLevel").get()?.number()? < 86 {
                                                                            ctx.call(
                                                                                Function::GetExperience,
                                                                                vec![Val::from(340408), Val::from(85102)],
                                                                            )?;
                                                                        } else if ctx.var("BaseLevel").get()?.number()? < 91 {
                                                                            ctx.call(
                                                                                Function::GetExperience,
                                                                                vec![Val::from(418460), Val::from(104615)],
                                                                            )?;
                                                                        } else if ctx.var("BaseLevel").get()?.number()? < 99 {
                                                                            ctx.call(
                                                                                Function::GetExperience,
                                                                                vec![Val::from(888140), Val::from(222035)],
                                                                            )?;
                                                                        } else {
                                                                            ctx.call(
                                                                                Function::GetItem,
                                                                                vec![Val::from(617), Val::from(1)],
                                                                            )?;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    ctx.call(Function::DoNpcEvent, vec![Val::from("Gray Wolf#rus05::OnDisable")])?;
                                                    ctx.call(Function::DoNpcEvent, vec![Val::from("Maria Morebna#rus46::OnDisable")])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if ctx.var("rhea_rus_main").get()?.number()? > 51 {
                                                    ctx.lines_as(
                                                        "Girl's Voice",
                                                        args!["... Koshei is immortal and will surely appear again someday..."],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Girl's Voice", args!["When that time comes, I will deal with him. I will use my time of waiting to train to make myself strong whenever he returns..."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Girl's Voice", args!["I'm grateful that you did your best for me and my village. I will never forget your kindness."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Girl's Voice", args!["Ah, Go to Baba Yaga in your free time. She seems to have something to tell you."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Girl's Voice", args!["Take care of yourself. It is better for you to stay away from here because, Koshei may try to use your power as his own, huhu."])?;
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
                step = WallRus04Step::OnInit;
                continue 'machine;
            }
            WallRus04Step::OnInit => {
                ctx.var("$@rus_req02").set(Val::from(0))?;
                return Err(Stop::End);
            }
            WallRus04Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Wall#rus04")])?;
                return Err(Stop::End);
            }
            WallRus04Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Wall#rus04")])?;
                return Err(Stop::End);
            }
            WallRus04Step::OnMyMobDead => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Koshei#rus47::OnDisable")])?;
                ctx.var("$@rus_req02").set(Val::from(0))?;
                ctx.call(Function::Announce, vec![Val::from("Koshei, the Immortal : Keeeek, you, cursed human.. I'll never give up!!! We'll see who's smiling in the end!!!"), ctx.constant("BC_MAP")?, Val::from(13513009)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn wall_rus04(ctx: &Ctx) -> Script {
    wall_rus04_run(ctx, WallRus04Step::Start, Vec::new()).map(|_| ())
}

pub fn wall_rus04_oninit(ctx: &Ctx) -> Script {
    wall_rus04_run(ctx, WallRus04Step::OnInit, Vec::new()).map(|_| ())
}

pub fn wall_rus04_onenable(ctx: &Ctx) -> Script {
    wall_rus04_run(ctx, WallRus04Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn wall_rus04_ondisable(ctx: &Ctx) -> Script {
    wall_rus04_run(ctx, WallRus04Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn wall_rus04_onmymobdead(ctx: &Ctx) -> Script {
    wall_rus04_run(ctx, WallRus04Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn koshei_rus47_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn koshei_rus47(ctx: &Ctx) -> Script {
    koshei_rus47_body(ctx, Vec::new()).map(|_| ())
}

fn koshei_rus47_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Koshei#rus47")])?;
    return Err(Stop::End);
}

pub fn koshei_rus47_oninit(ctx: &Ctx) -> Script {
    koshei_rus47_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn koshei_rus47_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::EnableNpc, vec![Val::from("Koshei#rus47")])?;
    return Err(Stop::End);
}

pub fn koshei_rus47_onenable(ctx: &Ctx) -> Script {
    koshei_rus47_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn koshei_rus47_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Koshei#rus47")])?;
    return Err(Stop::End);
}

pub fn koshei_rus47_ondisable(ctx: &Ctx) -> Script {
    koshei_rus47_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn koshei_rus47_ontimer3000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("mosk_dun01"),
            Val::from("Koshei, the Immortal : I will kill all who disturb me!! Cry in terror weak humans!!!"),
            ctx.constant("BC_MAP")?,
            Val::from(13513009),
        ],
    )?;
    return Err(Stop::End);
}

pub fn koshei_rus47_ontimer3000(ctx: &Ctx) -> Script {
    koshei_rus47_ontimer3000_body(ctx, Vec::new()).map(|_| ())
}

fn koshei_rus47_ontimer63000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("mosk_dun01"),
            Val::from("Koshei, the Immortal : You worms, you mere monsters... I will curse all who are in my way!!"),
            ctx.constant("BC_MAP")?,
            Val::from(13513009),
        ],
    )?;
    return Err(Stop::End);
}

pub fn koshei_rus47_ontimer63000(ctx: &Ctx) -> Script {
    koshei_rus47_ontimer63000_body(ctx, Vec::new()).map(|_| ())
}

fn koshei_rus47_ontimer150000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("mosk_dun01"),
            Val::from("Koshei, the Immortal : Mankind! Cry in terror!! Hahahahahahahhahahah!!!"),
            ctx.constant("BC_MAP")?,
            Val::from(13513009),
        ],
    )?;
    return Err(Stop::End);
}

pub fn koshei_rus47_ontimer150000(ctx: &Ctx) -> Script {
    koshei_rus47_ontimer150000_body(ctx, Vec::new()).map(|_| ())
}

fn koshei_rus47_ontimer300000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn koshei_rus47_ontimer300000(ctx: &Ctx) -> Script {
    koshei_rus47_ontimer300000_body(ctx, Vec::new()).map(|_| ())
}

fn mos_rus_main_run(ctx: &Ctx, mut step: MosRusMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MosRusMainStep::Start => {
                return Err(Stop::End);
            }
            MosRusMainStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![])?;
                return Err(Stop::End);
            }
            MosRusMainStep::OnTimer120000 => {
                step = MosRusMainStep::OnDisable;
                continue 'machine;
            }
            MosRusMainStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                step = MosRusMainStep::OnInit;
                continue 'machine;
            }
            MosRusMainStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mos_rus_main(ctx: &Ctx) -> Script {
    mos_rus_main_run(ctx, MosRusMainStep::Start, Vec::new()).map(|_| ())
}

pub fn mos_rus_main_onenable(ctx: &Ctx) -> Script {
    mos_rus_main_run(ctx, MosRusMainStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn mos_rus_main_ontimer120000(ctx: &Ctx) -> Script {
    mos_rus_main_run(ctx, MosRusMainStep::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn mos_rus_main_ondisable(ctx: &Ctx) -> Script {
    mos_rus_main_run(ctx, MosRusMainStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn mos_rus_main_oninit(ctx: &Ctx) -> Script {
    mos_rus_main_run(ctx, MosRusMainStep::OnInit, Vec::new()).map(|_| ())
}

fn the_blacksmith_rus06_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 3500 {
        ctx.lines_as(
            "The Blacksmith",
            args!["Why are you carrying that much?", "Are you training for something?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("rhea_rus_main").get()?.number()? < 6 {
        ctx.lines_as("The Blacksmith", args!["Bahaha~", "Good weather! Eh?", "Perfect for a picnic."])?;
        ctx.next()?;
        ctx.lines_as(
            "The Blacksmith",
            args!["But, I've heard that some people have become lost in the forest on the island near this village. What's going on?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("rhea_rus_main").get()? == 6 {
            ctx.lines_as("The Blacksmith", args!["Bahaha~", "Good weather! Eh?", "Perfect for a picnic."])?;
            ctx.next()?;
            ctx.lines_as(
                "The Blacksmith",
                args!["But, I've heard that some people have become lost in the forest on the island near this village. What's going on?"],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Excuse me...")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Excuse me, have you heard of a keymaker who knows how to make a '^0000ffGolden Key^000000'?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "The Blacksmith",
                args!["Hmm, are you an adventurer? Who told you about the '^0000ffGolden Key^000000'?"],
            )?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Ah, in fact..."])?;
            ctx.next()?;
            ctx.lines(args!["- You tell him about -", "- Maria Morebna -", "- and Gray Wolf -"])?;
            ctx.next()?;
            ctx.lines_as(
                "The Blacksmith",
                args![
                    "Well, then... The keymaker for the '^0000ffGolden Key^000000'.",
                    "You are very lucky to have asked me about this."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["W, what do you mean...?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "The Blacksmith",
                args!["I'm the only one in this town that knows what materials are needed to make the '^0000ffGolden Key^000000'! Bahaha!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "The Blacksmith",
                args!["I guess you think that getting the materials will be easy huh?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Hey I didn't say..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "The Blacksmith",
                args![
                    "Bahahaha~",
                    "It will take a lot of work.",
                    "A long journey awaits you",
                    "If you choose to take it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "The Blacksmith",
                args![
                    "Your boots will be worn",
                    "out after this journey.",
                    "What do you think?",
                    "Would you like to try?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Of course! I have to help Maria!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "The Blacksmith",
                args![
                    "Bahaha! I like you!",
                    "Ok, first, bring me ^0000ff25 Steel^000000. I'm not going anywhere so come back here when you have them."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Ok so that's the first thing you need for the 'Golden Key' right?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "The Blacksmith",
                args![
                    "Bahahaha~",
                    "I promise you, I'll explain when you bring the ^0000ff25 Steel^000000 to me. Please just get the Steel and come back."
                ],
            )?;
            ctx.var("rhea_rus_main").set(Val::from(7))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("rhea_rus_main").get()? == 7 {
                if ctx.call(Function::CountItem, vec![Val::from(999)])?.number()? > 24 {
                    ctx.lines_as("The Blacksmith", args!["Ohhh, that was faster than I expected!"])?;
                    ctx.next()?;
                    ctx.lines_as("The Blacksmith", args!["Ok then, wait here."])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "- He melts the steel -",
                        "- and begins to forge -",
                        "- it into something !! -"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "The Blacksmith",
                        args!["Bahaha~, it's not my", "best work, but it'll do.", "Here ya go!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Ah?! This is?!!"])?;
                    ctx.next()?;
                    ctx.lines_as("The Blacksmith", args!["Strong Steel Boots!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["The Golden!!.........................", "....................Eh, what?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "The Blacksmith",
                        args!["Bahaha~ Didn't I tell you? A long journey awaits you. These boots will help you on your long journey."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "The Blacksmith",
                        args!["When they have worn out, you will know that your journey is near it's end."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "The Blacksmith",
                        args!["^ff0000You must always wear these boots while gathering the materials for the 'Golden Key'.^000000"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "The Blacksmith",
                        args!["Ok, I'll tell you what materials you need to get. I can make the key anytime if you get them to me."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "The Blacksmith",
                        args![
                            "You need ^0000ff2 Cursed Ruby, 3 Gold, 1 Red Ring, 2 Lusalka's Hair, 10 Golden Thread^000000 to make the key."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "The Blacksmith",
                        args![
                            "Well, you know what to do now. I will be waiting here.",
                            "Bahahaha~",
                            "And don't forget to wear these!"
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(999), Val::from(25)])?;
                    ctx.var("rhea_rus_main").set(Val::from(8))?;
                    if (((ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT")?)
                        || ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?))
                        || ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?))
                        || ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ARCHER")?))
                    {
                        ctx.call(Function::GetItem, vec![Val::from(2429), Val::from(1)])?;
                    } else {
                        ctx.call(Function::GetItem, vec![Val::from(2430), Val::from(1)])?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "The Blacksmith",
                    args!["What are you doing? First, you must get me ^0000ff25 Steel^000000."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "The Blacksmith",
                    args!["The faster you get me the materials, The faster you can help Maria Morebna."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("rhea_rus_main").get()? == 8 {
                if ((((ctx.call(Function::CountItem, vec![Val::from(724)])?.number()? > 1
                    && ctx.call(Function::CountItem, vec![Val::from(969)])?.number()? > 2)
                    && ctx.call(Function::CountItem, vec![Val::from(7877)])?.is_true())
                    && ctx.call(Function::CountItem, vec![Val::from(7878)])?.number()? > 1)
                    && ctx.call(Function::CountItem, vec![Val::from(7879)])?.number()? > 9)
                    && ((ctx.var("rhea_rus_ring").get()?.number()? > 8 && ctx.var("rhea_rus_hair").get()?.number()? > 8)
                        && ctx.var("rhea_rus_quiz").get()?.number()? > 29)
                {
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
                    ctx.lines_as(
                        "The Blacksmith",
                        args![
                            "So, you got all the materials. I can't believe it.",
                            "You did your best until your boots were worn out, didn't you?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Yes, you know, it's not very comfortable walking around in Steel boots all day!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "The Blacksmith",
                        args![
                            "Bahaha~ Ok, well done.",
                            "Well.. While you were looking for the materials, I was searching for the keymaker who can make the key!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I thought you said that you knew how to make the key?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("The Blacksmith", args!["Hmm... Well I know the materials. And I know who makes the key. So I will tell you where to find the keymaker. Is that ok?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Of course! I have to know where this person is."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "The Blacksmith",
                        args!["Ok, listen carefully. The Keymaker is at a cabin deep inside of the forest."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("The Blacksmith", args!["Ah, wait. Take off your armor. I need to cast a protection spell on you, ok? Hey, don't get the wrong idea. This is just a simple protection spell!"])?;
                    ctx.call(Function::Nude, vec![])?;
                    ctx.next()?;
                    ctx.lines_as("The Blacksmith", args!["Bah ram y--- wait that's uh something else..."])?;
                    ctx.next()?;
                    ctx.lines_as("The Blacksmith", args!["'^ff0000Spellshield Protection^000000."])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_ABSORBSPIRITS")?])?;
                    ctx.next()?;
                    ctx.lines_as("The Blacksmith", args!["This spell protects you from any curse the keymaker might try to use. Remember the words of the spell. It won't last very long because I am just a blacksmith."])?;
                    ctx.next()?;
                    ctx.mes("[The Blacksmith]")?;
                    if ctx.call(Function::CountItem, vec![Val::from(2429)])?.is_true() {
                        ctx.call(Function::DelItem, vec![Val::from(2429), Val::from(1)])?;
                    } else if ctx.call(Function::CountItem, vec![Val::from(2430)])?.is_true() {
                        ctx.call(Function::DelItem, vec![Val::from(2430), Val::from(1)])?;
                    } else {
                        ctx.lines(args![
                            "The forest is dangerous. Be very careful in there!",
                            "Ah, and your steel boots..?"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as("The Blacksmith", args!["Where are they?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines(args![
                        "The forest is dangerous. Be very careful in there!",
                        "Ah and you shouldn't be needing those Steel Boots anymore. Good luck!"
                    ])?;
                    ctx.var("rhea_rus_main").set(Val::from(9))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "The Blacksmith",
                    args!["You need ^0000ff2 Cursed Ruby, 3 Gold, 1 Red Ring, 2 Lusalka's Hair, 10 Golden Thread^000000 to make the key."],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Red Ring?:Lusalka's Hair?:Golden Thread?")])? {
                    1 => {
                        ctx.lines_as("The Blacksmith", args!["The Red Ring has powerful enchanting powers. I had one before, but I gave it to my friend, Vassili, as a gift."])?;
                        ctx.next()?;
                        ctx.lines_as("The Blacksmith", args!["I think it would be odd to ask him to give it back to me. So uh you've got to ask him if he would give it to you."])?;
                    }
                    2 => {
                        ctx.lines_as(
                            "The Blacksmith",
                            args![
                                "It's said that a bride who drowns to death just before her wedding becomes Lusalka, the aqua fairy.",
                                "You must find Lusalka's hair."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "The Blacksmith",
                            args![
                                "I heard that a young maid has been lost recently.",
                                "It's very unfortunate but...",
                                "she might be Lusalka..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "The Blacksmith",
                            args!["Her mother is somewhere in this village asking for people to help her find her missing daughter."],
                        )?;
                    }
                    3 => {
                        ctx.lines_as("The Blacksmith", args!["Golden Thread is made from gold by using a special spinning wheel technique. I don't know exactly how it's done. But I know who does."])?;
                        ctx.next()?;
                        ctx.lines_as("The Blacksmith", args!["Find a man named Marozka. He is the only one who knows how to use the spinning wheel to make the Golden Thread."])?;
                        ctx.next()?;
                        ctx.lines_as("The Blacksmith", args!["Maria should know where he is."])?;
                    }
                    _ => {}
                }
                ctx.next()?;
                ctx.lines_as("The Blacksmith", args!["Ah, and... You still have the boots that I made right? ^ff0000Without them, you cannot get anything for the key, I swear. You must put on them.^000000"])?;
                ctx.next()?;
                ctx.lines_as(
                    "The Blacksmith",
                    args!["Well, you know what to do now. I will be waiting here."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("rhea_rus_main").get()? == 9 {
                ctx.lines_as("The Blacksmith", args!["The keymaker is at a cabin deep inside of the forest."])?;
                ctx.next()?;
                ctx.lines_as(
                    "The Blacksmith",
                    args!["This person is no ordinary person. It is said that you can be cursed by the keymaker's words."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "The Blacksmith",
                    args!["Don't forget the spell, '^ff0000Spellshield Protection.^000000'"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (ctx.var("rhea_rus_main").get()?.number()? > 9 && ctx.var("rhea_rus_main").get()?.number()? < 52) {
                ctx.lines_as(
                    "The Blacksmith",
                    args!["He is in his cabin deep inside of the forest. There, the keymaker of the Golden Key is living."],
                )?;
                ctx.next()?;
                ctx.lines_as("The Blacksmith", args!["The Golden Key is able to release Maria. Good luck!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("rhea_rus_main").get()?.number()? > 51 {
                ctx.lines_as("The Blacksmith", args!["I heard the news. You have done well."])?;
                ctx.next()?;
                ctx.lines_as(
                    "The Blacksmith",
                    args!["Ah, the person living around that cabin wants you to see her. She seems to have something to tell you."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn the_blacksmith_rus06(ctx: &Ctx) -> Script {
    the_blacksmith_rus06_body(ctx, Vec::new()).map(|_| ())
}

fn vassili_grandpapa_rus07_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 3500 {
        ctx.lines_as(
            "Vassili Grandpapa",
            args!["Why are you carring that much?", "Are you training for something?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("rhea_rus_main").get()?.number()? < 8 {
        ctx.lines_as(
            "Vassili Grandpapa",
            args!["Hmm, are you an adventurer? So, how do you like it here?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["It's not too bad.", "The weather here is great!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Vassili Grandpapa",
            args!["Even though it is fine now in Moscovia, it is not such a great place during the harsh winter season."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Vassili Grandpapa",
            args!["Eeee, I don't want to experience that coldness any more."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("rhea_rus_main").get()? == 8 {
            if ctx.var("rhea_rus_ring").get()? == 0 {
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
                ctx.lines_as(
                    "Vassili Grandpapa",
                    args!["Hmm, are you an adventurer? So, how do you like it here?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("It's so-so:I have something to ask...")],
                )?) == 1
                {
                    ctx.lines_as(
                        "Vassili Grandpapa",
                        args!["Hmm, you have traveled so much. I guess if you're tired from your trip then it's only natural, huhu."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Vassili Grandpapa", args!["Eh? What do you want to ask me about?"])?;
                ctx.next()?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                if l_input_s.clone() == "Red Ring" {
                    ctx.lines_as("Vassili Grandpapa", args!["Red Ring?! Why are you looking for that?"])?;
                    ctx.next()?;
                } else {
                    ctx.lines_as("Vassili Grandpapa", args!["Mmm, what? I don't understand you."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args!["- You tell him about the -", "- Gray Wolf and Maria -"])?;
                ctx.next()?;
                ctx.lines_as("Vassili Grandpapa", args!["Huuuh, I see."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Vassili Grandpapa",
                    args![
                        "What should I do?",
                        "I want to help but I can't.",
                        "I gave it to my youngest",
                        "daughter, Mashenka,",
                        "as a present. But... she..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Are you ok?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Vassili Grandpapa",
                    args!["It's just that Mashenka...", "..........", "....she died..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Vassili Grandpapa",
                    args![
                        "She was really pretty and smart. My first daughter, Ryubaba, is also pretty and smart but her temper is...",
                        "hotter than Ifrit's flames!",
                        "Do you know what I mean?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Vassili Grandpapa", args!["Anyway, Mashenka was such a kind and tender daughter. On cold winter nights, when I came back home late, she would wait for me with a hot bowl of my favorite stew."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Vassili Grandpapa",
                    args![
                        "How couldn't I be touched by such kindness? So, I gave the Red Ring to her as a gift.",
                        "I can still remember how happy she was when I gave it to her."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Vassili Grandpapa", args!["She disappeared soon... after Now all I have is Ryubaba. She is also pretty and smart but nothing can relieve the sadnesss in my heart from losing Mashenka..."])?;
                ctx.var("rhea_rus_ring").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("rhea_rus_ring").get()?.number()? < 8 {
                    ctx.lines_as(
                        "Vassili Grandpapa",
                        args!["I can still remember how happy she was when I gave it to her."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Vassili Grandpapa", args!["She disappeared soon... after Now all I have is Ryubaba. She is also pretty and smart but nothing can relieve the sadnesss in my heart from losing Mashenka..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("rhea_rus_ring").get()? == 8 {
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
                        ctx.lines_as(
                            "Vassili Grandpapa",
                            args!["Ehh? you are the adventurer that I met before. Something to tell me?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["...Listen to... this."],
                        )?;
                        ctx.next()?;
                        if ctx.call(Function::CountItem, vec![Val::from(7883)])?.is_true() {
                            ctx.mes("- You play the flute -")?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^ff0000Red ring that my father gave me^000000",
                                "^ff0000Red ring that my father gave me^000000",
                                "^ff0000The red ring enchanted^000000",
                                "^ff0000For the loveliest daughter^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^ff0000Red ring bringing up jealousy^000000",
                                "^ff0000Red ring bringing up jealousy^000000",
                                "^ff0000Red ring enchanted^000000",
                                "^ff0000My sister envies^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^ff0000Cold hand tightening my neck^000000",
                                "^ff0000Cold eyes tightening my heart^000000",
                                "^ff0000Cold marsh swallowing my body^000000",
                                "^ff0000Red ring taken away^000000",
                                "^ff0000Red ring enchanted^000000"
                            ])?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                            ctx.call(
                                Function::Emotion,
                                vec![
                                    ctx.constant("ET_THINK")?,
                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Vassili Grandpapa",
                                args!["...W, what is this? Ehh??", "My daughter, Mashenka, was.. by Ryubaba...!?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Vassili Grandpapa",
                                args![
                                    "No... this isn't true?",
                                    "Don't try to trick me... Why are you telling... That flute is... Mashenka..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Isn't this.. isn't this Mashenka's voice?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Vassili Grandpapa",
                                args!["Don't tell me those lies about..!! My daughters... That..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Vassili Grandpapa", args!["............................."])?;
                            ctx.next()?;
                            ctx.lines_as("Vassili Grandpapa", args!["............................."])?;
                            ctx.next()?;
                            ctx.lines_as("Vassili Grandpapa", args!["............................."])?;
                            ctx.next()?;
                            ctx.lines_as("Vassili Grandpapa", args!["............................."])?;
                            ctx.next()?;
                            ctx.lines_as("Vassili Grandpapa", args!["............................."])?;
                            ctx.next()?;
                            ctx.lines_as("Vassili Grandpapa", args!["............................."])?;
                            ctx.next()?;
                            ctx.lines_as("Vassili Grandpapa", args!["............................."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["...Uh... I'm so sorry..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Vassili Grandpapa", args!["... Can you please... play the flute again?"])?;
                            ctx.next()?;
                            ctx.lines(args!["- You play the flute -", "- with Mashenka's voice -"])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^ff0000Red ring that my father gave me^000000",
                                "^ff0000Red ring that my father gave me^000000",
                                "^ff0000The red ring enchanted^000000",
                                "^ff0000For the loveliest daughter^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^ff0000Red ring bringing up jealousy^000000",
                                "^ff0000Red ring bringing up jealousy^000000",
                                "^ff0000Red ring enchanted^000000",
                                "^ff0000My sister envies^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^ff0000Cold hand tightening my neck^000000",
                                "^ff0000Cold eyes tightening my heart^000000",
                                "^ff0000Cold marsh swallowing my body^000000",
                                "^ff0000Red ring taken away^000000",
                                "^ff0000Red ring enchanted^000000"
                            ])?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                            ctx.call(
                                Function::Emotion,
                                vec![
                                    ctx.constant("ET_THINK")?,
                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Vassili Grandpapa",
                                args![
                                    "It is.. It is true...",
                                    "When I saw this ring in Ryubaba's room, I never thought that. But, it has to be true..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Vassili Grandpapa",
                                args!["Damn this cursed ring!!! Why did my daughters have this tragedy....? ~Sob!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["......................"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Vassili Grandpapa",
                                args!["........i'm sorry. ...I know that this wasn't easy for you to tell me. Thank you for the truth."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Vassili Grandpapa",
                                args![
                                    "You said you needed this ring, right? Take it...",
                                    "It has brought me nothing but grief."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Vassili Grandpapa", args!["...It is my fault that this tragedy between my daughters happened. I will spend the rest of my life trying to make up for it..."])?;
                            ctx.call(Function::DelItem, vec![Val::from(7883), Val::from(1)])?;
                            ctx.call(Function::GetItem, vec![Val::from(7877), Val::from(1)])?;
                            ctx.var("rhea_rus_ring").set(Val::from(10))?;
                            {
                                if ctx.var("BaseLevel").get()?.number()? < 56 {
                                    ctx.call(Function::GetExperience, vec![Val::from(4700), Val::from(0)])?;
                                } else {
                                    if ctx.var("BaseLevel").get()?.number()? < 61 {
                                        ctx.call(Function::GetExperience, vec![Val::from(6150), Val::from(0)])?;
                                    } else {
                                        if ctx.var("BaseLevel").get()?.number()? < 66 {
                                            ctx.call(Function::GetExperience, vec![Val::from(10605), Val::from(0)])?;
                                        } else {
                                            if ctx.var("BaseLevel").get()?.number()? < 71 {
                                                ctx.call(Function::GetExperience, vec![Val::from(16223), Val::from(0)])?;
                                            } else {
                                                if ctx.var("BaseLevel").get()?.number()? < 76 {
                                                    ctx.call(Function::GetExperience, vec![Val::from(41227), Val::from(0)])?;
                                                } else if ctx.var("BaseLevel").get()?.number()? < 81 {
                                                    ctx.call(Function::GetExperience, vec![Val::from(69073), Val::from(0)])?;
                                                } else if ctx.var("BaseLevel").get()?.number()? < 86 {
                                                    ctx.call(Function::GetExperience, vec![Val::from(85102), Val::from(0)])?;
                                                } else if ctx.var("BaseLevel").get()?.number()? < 91 {
                                                    ctx.call(Function::GetExperience, vec![Val::from(104615), Val::from(0)])?;
                                                } else if ctx.var("BaseLevel").get()?.number()? < 99 {
                                                    ctx.call(Function::GetExperience, vec![Val::from(222035), Val::from(0)])?;
                                                } else {
                                                    ctx.call(Function::GetItem, vec![Val::from(607), Val::from(1)])?;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Vassili Grandpapa",
                            args![
                                "...............................",
                                "...............................",
                                "...What did you say?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Ehh?! This is weird. Where is the flute?!"],
                        )?;
                        ctx.call(
                            Function::Emotion,
                            vec![
                                ctx.constant("ET_HUK")?,
                                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("rhea_rus_ring").get()? == 9 {
                        ctx.lines_as(
                            "Vassili Grandpapa",
                            args!["You may think it is foolish but I cannot relieve the sadness of my lost daughter."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Vassili Grandpapa",
                            args!["It is parents that wander around the entrance of the village to wait for their children..."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
    ctx.lines_as(
        "Vassili Grandpapa",
        args![
            "..It is my fault that this tragedy between my daughters happened. I will spend the rest of my life trying to make up for it..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn vassili_grandpapa_rus07(ctx: &Ctx) -> Script {
    vassili_grandpapa_rus07_body(ctx, Vec::new()).map(|_| ())
}
