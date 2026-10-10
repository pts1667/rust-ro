use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Bard2Step {
    Start,
    SStorySong,
}

fn bard_2_run(ctx: &Ctx, mut step: Bard2Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_inputstr_s = Val::from("");
    let mut l_num = Val::from(0);
    let mut l_random = Val::from(0);
    'machine: loop {
        match step {
            Bard2Step::Start => {
                if ctx.var("bard_q").get()?.number()? > 5 {
                    ctx.var("gef_bard_q").set(ctx.var("bard_q").get()?)?;
                }
                ctx.var("@name$").set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
                if ctx.var("gef_bard_q").get()?.number()? > 29 {
                    ctx.call(Function::Cutin, vec![Val::from("bard_eland02"), Val::from(2)])?;
                    ctx.lines_as(
                        "Errende",
                        args!["Why, hello there~", "Isn't the weather", "especially pleasant today?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Errende", args!["Ah~ There are only two things that can make this picturesque moment even more beautiful: story and song. Now then, which would you like to hear?"])?;
                    bard_2_run(ctx, Bard2Step::SStorySong, vec![Val::from(1)])?;
                } else {
                    if (ctx.var("gef_bard_q").get()? == 14 || ctx.var("gef_bard_q").get()? == 15) {
                        if ctx.var("gef_bard_q").get()? == 14 {
                            ctx.call(Function::Cutin, vec![Val::from("bard_eland04"), Val::from(2)])?;
                            ctx.lines_as("Errende", args!["Hmmm?", "Is that the seal of black...? Huh. Kino Kitty removed the one he gave me without a trace, so I surmise that he likes you a lot. Otherwise..."])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^3355FFYou give Errende the letter",
                                "you have received from Kino Kitty.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("bard_eland01"), Val::from(2)])?;
                            ctx.lines_as("Errende", args!["Er, he knows everything already? From this spot of blood, I think he still has health problems. He should stop torturing himself..."])?;
                            ctx.next()?;
                            ctx.lines_as("Errende", args!["Thank you, I really appreciate what you've done for me. But, what would be an appropriate way to express my gratitude for an adventurer like yourself?"])?;
                        } else {
                            ctx.call(Function::Cutin, vec![Val::from("bard_eland01"), Val::from(2)])?;
                            ctx.lines_as(
                                "Errende",
                                args![
                                    "Ah, so how did it go?",
                                    "Wait, you already found it?",
                                    "Yes, this is it! Great!",
                                    "This is so amazing!!",
                                    "Ah, yes, right.",
                                    "Right."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Errende",
                                args![
                                    "I would like to express my gratitute. But, what would be appropriate for an adventurer",
                                    "like yourself?"
                                ],
                            )?;
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            "Errende",
                            args!["Oh~! I have just the thing. Alright, please make yourself comfortable, and listen to my song."],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("bard_eland03"), Val::from(2)])?;
                        ctx.lines(args![
                            "^483D8BEvery god never grows old",
                            "Because of beautiful",
                            "Goddess, Idun.",
                            "Keeper of the apples of youth",
                            "Goddess of immortality.^000000"
                        ])?;
                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^483D8BEvery god never grows old.",
                            "Idun, the wife of Bragi,",
                            "Idun, Odin's daughter in law~",
                            "The apples she keeps",
                            "In her basket.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^483D8BWithout Idun,",
                            "Every god would",
                            "have succumbed to age.",
                            "Even Thor, the strongest of gods,",
                            "would grow frail, Megingjard would",
                            "slip from his waist, and Mjolnir",
                            "would never fly again.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^483D8BWithout Idun,",
                            "Every god would",
                            "have succumbed to age.",
                            "Loki was careless once,",
                            "and made her lost to the gods.",
                            "He was forced to get her back.^000000"
                        ])?;
                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^483D8BMy goddess stands",
                            "In the field of Asgard",
                            "She hands me fruit from heaven.",
                            "You will be loved by every god...",
                            "You will be blessed",
                            "By every god...^000000"
                        ])?;
                        ctx.next()?;
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL6")?])?;
                        ctx.lines(args![
                            "^483D8BIf you share the",
                            "Apple of youth with me",
                            "Even a bite of it with",
                            "This poor poet.",
                            "You will be loved by every god...",
                            "You will be blessed",
                            "By every god...^000000"
                        ])?;
                        ctx.next()?;
                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_RESURRECTION")?])?;
                        ctx.mes("^3355FFWhile listening to his song, you feel at ease, and your thoughts become clearer. You believe that you see the vision of an angel, and you gain some experience points.^000000")?;
                        ctx.var("gef_bard_q").set((ctx.var("gef_bard_q").get()? + Val::from(16)))?;
                        {
                            if ctx.var("BaseLevel").get()?.number()? < 56 {
                                ctx.call(Function::GetExperience, vec![Val::from(4500), Val::from(0)])?;
                            } else {
                                if (ctx.var("BaseLevel").get()?.number()? > 55 && ctx.var("BaseLevel").get()?.number()? < 61) {
                                    ctx.call(Function::GetExperience, vec![Val::from(5500), Val::from(0)])?;
                                } else {
                                    if (ctx.var("BaseLevel").get()?.number()? > 60 && ctx.var("BaseLevel").get()?.number()? < 66) {
                                        ctx.call(Function::GetExperience, vec![Val::from(9684), Val::from(0)])?;
                                    } else {
                                        if (ctx.var("BaseLevel").get()?.number()? > 65 && ctx.var("BaseLevel").get()?.number()? < 71) {
                                            ctx.call(Function::GetExperience, vec![Val::from(13411), Val::from(0)])?;
                                        } else if (ctx.var("BaseLevel").get()?.number()? > 70 && ctx.var("BaseLevel").get()?.number()? < 76)
                                        {
                                            ctx.call(Function::GetExperience, vec![Val::from(35757), Val::from(0)])?;
                                        } else if (ctx.var("BaseLevel").get()?.number()? > 75 && ctx.var("BaseLevel").get()?.number()? < 81)
                                        {
                                            ctx.call(Function::GetExperience, vec![Val::from(60246), Val::from(0)])?;
                                        } else if (ctx.var("BaseLevel").get()?.number()? > 80 && ctx.var("BaseLevel").get()?.number()? < 86)
                                        {
                                            ctx.call(Function::GetExperience, vec![Val::from(70340), Val::from(0)])?;
                                        } else if (ctx.var("BaseLevel").get()?.number()? > 85 && ctx.var("BaseLevel").get()?.number()? < 91)
                                        {
                                            ctx.call(Function::GetExperience, vec![Val::from(92052), Val::from(0)])?;
                                        } else {
                                            ctx.call(Function::GetExperience, vec![Val::from(156786), Val::from(0)])?;
                                        }
                                    }
                                }
                            }
                        }
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("bard_eland02"), Val::from(2)])?;
                        ctx.lines_as("Errende", args!["So, how do you feel now? I hope my song has refreshed you. I'm afraid it may not be enough to repay you, but please understand that this is the best way for me to express my gratitude."])?;
                        ctx.next()?;
                        ctx.lines_as("Errende", args!["Besides, I've been thinking of you now as my friend, with whom I may candidly speak without any worry. And I think you have a beautiful smile. Am I wrong~? Hahaha~"])?;
                        ctx.next()?;
                        ctx.lines_as("Errende", args!["I hope that you'll always remain honest and respectful towards other people, and that you continue to ignore greed for fortune or power."])?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    } else {
                        if (ctx.var("gef_bard_q").get()?.number()? > 11 && ctx.var("gef_bard_q").get()?.number()? < 14) {
                            ctx.call(Function::Cutin, vec![Val::from("bard_eland02"), Val::from(2)])?;
                            ctx.lines_as("Errende", args!["It seems you haven't found it yet. Well, take your time, I can wait as long as you want. It doesn't really bore me, since waiting seems to be a part of my profession."])?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("gef_bard_q").get()? == 11 {
                                ctx.call(Function::Cutin, vec![Val::from("bard_eland01"), Val::from(2)])?;
                                ctx.lines_as("Errende", args!["Mr. Skezti has a small book store on the book street at the right side of Mineta in Juno. If you show him the seal, he'll help you out."])?;
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("gef_bard_q").get()? == 10 {
                                    ctx.call(Function::Cutin, vec![Val::from("bard_eland01"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Errende",
                                        args![
                                            "So, have you met Mr. Kitty?",
                                            "Ah, in Morocc? He has a chronic disease, but obviously doesn't",
                                            "seem to care about it.",
                                            "So what did he say?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Errende",
                                        args![
                                            "Hm? In Juno?",
                                            "Then that must be it.",
                                            "It won't be easy if you want to go visit Mr. Sketzi's book store. Here, let me mark you with",
                                            "^483D8BThe Seal of Friendship^000000."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("bard_eland03"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Errende",
                                        args!["Give me your left hand.", "Now, let me cast a spell...", "*Mumble mumble...*"],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CONE")?])?;
                                    ctx.mes("^3355FFOn your left wrist, a crescent shaped mark glowing with a silver light appeared. It's only noticeable when you concentrate on finding it, but it might clearly appear under the moonlight.^000000")?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Errende",
                                        args![
                                            "There you go. Now, you may go to Mr. Sketzi. Remember, you must",
                                            "show him the Seal of Friendship.",
                                            "Good luck, now~"
                                        ],
                                    )?;
                                    ctx.var("gef_bard_q").set(Val::from(11))?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("gef_bard_q").get()? == 20 {
                                        ctx.call(Function::Cutin, vec![Val::from("bard_eland03"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "Bard",
                                            args![
                                                "^483D8BWhat day is",
                                                "best for drinking?",
                                                "La la la~",
                                                "It's the day of",
                                                "the earth, the sun",
                                                "And the moon~",
                                                "La la la~^000000"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines(args![
                                            "^483D8BLa la la~",
                                            "I'll only",
                                            "drink on one day~",
                                            "So if you'll tell me",
                                            "when you'll drink",
                                            "I'll tell you when",
                                            "I'll drink with you~^000000"
                                        ])?;
                                        ctx.next()?;
                                        ctx.lines(args!["^483D8BLet's get together", "Yea yea ye-^000000 Hmmmmm...?"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Bard",
                                            args!["Why, hello there. Oh, have you come to listen to my song and forget your worries?"],
                                        )?;
                                        ctx.next()?;
                                        if Val::from(runtime::select_values(ctx, &[Val::from("Who are you?:Ignore him.")])?) == 1 {
                                            ctx.call(Function::Cutin, vec![Val::from("bard_eland04"), Val::from(2)])?;
                                            ctx.lines_as(
                                                ctx.var("@name$").get()?,
                                                args!["You seem to be", "new around here...", "Who are you?"],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::Cutin, vec![Val::from("bard_eland02"), Val::from(2)])?;
                                            ctx.lines_as("Errende", args!["Mm? Ah yes. I am merely another wandering poet who goes where the wind takes him. Please call me ^483D8BErrende^000000, the Bard who wishes to please you."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Errende",
                                                args![
                                                    "If you will let me, I will tell you of my travels. By your leave,",
                                                    "I will play a song that will help you forget your troubles."
                                                ],
                                            )?;
                                            ctx.var("gef_bard_q").set(Val::from(21))?;
                                            bard_2_run(ctx, Bard2Step::SStorySong, vec![Val::from(2)])?;
                                        } else {
                                            ctx.call(Function::Cutin, vec![Val::from("bard_eland01"), Val::from(2)])?;
                                            ctx.lines_as("Errende", args!["Waaah, wah~", "You can't just ignore me like that! Where's your sense of merriment, your sense of romance?"])?;
                                            ctx.close_window()?;
                                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                            return Err(Stop::End);
                                        }
                                    } else {
                                        if ctx.var("gef_bard_q").get()? == 27 {
                                            ctx.call(Function::Cutin, vec![Val::from("bard_eland01"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Errende",
                                                args![
                                                    "How could I forget the song?",
                                                    "I'm worthless as a Bard~!",
                                                    "Please... Just...",
                                                    "Please just leave",
                                                    "me alone."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            if Val::from(runtime::select_values(
                                                ctx,
                                                &[Val::from("No problem~:Is there any way I can help you?")],
                                            )?) == 1
                                            {
                                                ctx.call(Function::Cutin, vec![Val::from("bard_eland04"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Errende",
                                                    args![
                                                        "Waaaah~! You're so mean!",
                                                        "You're supposed to say, 'Errende, what's wrong? Maybe I can help?'"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Errende",
                                                    args![
                                                        "I mean, we're supposed",
                                                        "to know each other better",
                                                        "than that. But...",
                                                        "Maybe I was wrong!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Errende",
                                                    args![
                                                        "*Sob...*",
                                                        "Who made this poor Bard cry by neglecting him? *Sniff* I do believe it was you!"
                                                    ],
                                                )?;
                                            } else {
                                                ctx.lines_as("Errende", args!["Help me?", "Hmmm...", "That's it!", "Gunther!"])?;
                                                ctx.next()?;
                                                ctx.lines_as("Errende", args!["Let me ask you a favor.", "If perchance you happen to meet ^483D8BGunther Doubleharmony^000000, please inform him of my dilemna."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Errende", args!["Tell him that ^483D8BMinty Errende^000000 happened to forget a line of the song, ^483D8BAt One, I Fall in Love^000000. The line is called ^483D8B8th love^000000."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Errende", args!["I beseech you, if you meet him, please ask him of the 8th love and inform me of that lyric immediately~"])?;
                                                ctx.var("gef_bard_q").set(Val::from(22))?;
                                            }
                                            ctx.close_window()?;
                                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.var("gef_bard_q").get()? == 26 {
                                                ctx.call(Function::Cutin, vec![Val::from("bard_eland01"), Val::from(2)])?;
                                                ctx.lines_as("Errende", args!["Argh...!", "Who changed the words of this song anyway? It's really difficult to understand. I wish I could ask the person who changed the words."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Errende",
                                                    args!["This is so frustrating!", "Now, I've lost the passion", "to sing or play..."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    ctx.var("@name$").get()?,
                                                    args!["Don't you think...", "The person would be..."],
                                                )?;
                                                let (input, status) = runtime::input_text(ctx, None, None)?;
                                                l_inputstr_s = input;
                                                ctx.lines(args![(l_inputstr_s.clone() + Val::from("?"))])?;
                                                ctx.next()?;
                                                if l_inputstr_s.clone() == "Kino Kitty" {
                                                    ctx.call(Function::Cutin, vec![Val::from("bard_eland04"), Val::from(2)])?;
                                                    ctx.lines_as(
                                                        "Errende",
                                                        args!["Ah! Of course!", "I think you're right!", "How could I not think of that?"],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Errende", args!["It all makes sense now. After all, he used to be a member of the Invincible Single Army. His changes might have been a little mean, since this song used to be about a happy couple..."])?;
                                                    ctx.next()?;
                                                    ctx.call(Function::Cutin, vec![Val::from("bard_eland01"), Val::from(2)])?;
                                                    ctx.lines_as("Errende", args!["Ummm...", "I'm sorry to ask a favor of you again, but in your travels, do you think you could find the original lyrics for this song? I can wait for it..."])?;
                                                    ctx.next()?;
                                                    if Val::from(runtime::select_values(
                                                        ctx,
                                                        &[Val::from("No, thanks.:I can, so stop crying.")],
                                                    )?) == 1
                                                    {
                                                        ctx.lines_as("Errende", args!["Ah, I guess it was too much to ask of you. My apologies. Don't worry about it, I'll find out some other way."])?;
                                                        ctx.var("gef_bard_q").set(Val::from(25))?;
                                                    } else {
                                                        ctx.lines_as(
                                                            "Errende",
                                                            args![
                                                                "Are you serious?",
                                                                "Oh, thank you so much!",
                                                                "You must be an angel!",
                                                                "An angel that truly understands the heart of a poet!"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Errende",
                                                            args![
                                                                "I'll pay you back somehow!",
                                                                "Thank you for your trouble",
                                                                "in advance~"
                                                            ],
                                                        )?;
                                                        ctx.var("gef_bard_q").set(Val::from(24))?;
                                                    }
                                                    ctx.close_window()?;
                                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                    return Err(Stop::End);
                                                } else if l_inputstr_s.clone() == "Gunther" {
                                                    ctx.lines_as("Errende", args!["Gunther? I don't think he would do this. He always puts lines in his songs like 'doubleharmony for you.' Plus, he's too silly for that."])?;
                                                    ctx.close_window()?;
                                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                    return Err(Stop::End);
                                                } else if l_inputstr_s.clone() == "Gunther Doubleharmony" {
                                                    ctx.lines_as("Errende", args!["Gunther? I don't think he would do this. He always puts lines in his songs like 'doubleharmony for you.' Plus, he's too silly for that."])?;
                                                    ctx.close_window()?;
                                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                    return Err(Stop::End);
                                                } else if l_inputstr_s.clone() == "Errende" {
                                                    ctx.call(Function::Cutin, vec![Val::from("bard_eland04"), Val::from(2)])?;
                                                    ctx.lines_as(
                                                        "Errende",
                                                        args![
                                                            "Surely you jest!",
                                                            "If I did, why would",
                                                            "I not know what",
                                                            "this song is about?"
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                    return Err(Stop::End);
                                                } else {
                                                    ctx.lines_as(
                                                        "Errende",
                                                        args![
                                                            ((Val::from("") + l_inputstr_s.clone()) + Val::from("...?")),
                                                            "I don't think I know that person. Maybe you misunderstood",
                                                            "something? *Sigh...*"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Errende",
                                                        args!["What was the line...?", "How could I forget", "the 8th love?"],
                                                    )?;
                                                    ctx.var("gef_bard_q").set(Val::from(26))?;
                                                    ctx.close_window()?;
                                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                    return Err(Stop::End);
                                                }
                                            } else {
                                                if ctx.var("gef_bard_q").get()? == 25 {
                                                    ctx.call(Function::Cutin, vec![Val::from("bard_eland04"), Val::from(2)])?;
                                                    ctx.lines_as(
                                                        "Errende",
                                                        args![
                                                            "Oh...!",
                                                            "These tears just won't stop!",
                                                            "Wh-why must people always treat Bards like this?"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^483D8BWho made",
                                                        "this poor Bard cry?",
                                                        "Who broke his",
                                                        "tender heart of glass?",
                                                        "Shattering his dreams...^000000"
                                                    ])?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^483D8BNo dreams,",
                                                        "No heart,",
                                                        "No love,",
                                                        "No hope",
                                                        "No....^000000"
                                                    ])?;
                                                    l_random = ctx.call(Function::Rand, vec![Val::from(1), Val::from(50)])?;
                                                    ctx.next()?;
                                                    if (l_random.clone().number()? > 27 && l_random.clone().number()? < 37) {
                                                        ctx.mes("^3355FFErrende continues to sing about his personal despair. He seems to be disappointed in your refusal to help him. Of course, you begin to feel sorry for him.^000000")?;
                                                        ctx.next()?;
                                                        if Val::from(runtime::select_values(
                                                            ctx,
                                                            &[Val::from("Well, I should help him then...:Ignore him anyway.")],
                                                        )?) == 1
                                                        {
                                                            ctx.lines_as(ctx.var("@name$").get()?, args!["Hey. Hey, Errende. Stop singing this song. It's embarassing, okay? Alright, I'll go find the original song for you."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Errende",
                                                                args![
                                                                    "*Gasp!* Really! Are you sure? Thank you! Thank you so much!",
                                                                    "You really do care about",
                                                                    "the sadness of a Bard!"
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Errende", args!["I promise to pay you back as best as I can! I'll wait for you here until you return!"])?;
                                                            ctx.var("gef_bard_q").set(Val::from(24))?;
                                                        } else {
                                                            ctx.call(Function::Cutin, vec![Val::from("bard_eland04"), Val::from(2)])?;
                                                            ctx.lines(args![
                                                                "^3355FFYou ignore his",
                                                                "heart wrenching song.",
                                                                "But at what cost to your soul?^000000"
                                                            ])?;
                                                        }
                                                        ctx.close_window()?;
                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                        return Err(Stop::End);
                                                    } else {
                                                        ctx.call(Function::Cutin, vec![Val::from("bard_eland01"), Val::from(2)])?;
                                                        ctx.lines_as("Errende", args!["You don't have anything you want to ask me, do you? If not, would you like to listen to my song...?"])?;
                                                        ctx.next()?;
                                                        ctx.lines(args![
                                                            "^483D8BMiiiiserable...",
                                                            "Noboooody looooves meee",
                                                            "Friends foooooor never...",
                                                            "Ooooooooh wah!^000000"
                                                        ])?;
                                                        ctx.close_window()?;
                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                        return Err(Stop::End);
                                                    }
                                                } else {
                                                    if ctx.var("gef_bard_q").get()? == 24 {
                                                        ctx.call(Function::Cutin, vec![Val::from("bard_eland02"), Val::from(2)])?;
                                                        ctx.lines_as("Errende", args!["Umm...", "I am not sure exactly where Mr. Kitty is. Maybe you might be able to find out at the Monster Museum in Juno? I think he's a member of the Monster Research Organization."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Errende", args!["If you're experienced in exploration, you'd know the Monster Research Organization. Recently, many scholars have been wounded as a result of researching monsters."])?;
                                                        ctx.next()?;
                                                        ctx.call(Function::Cutin, vec![Val::from("bard_eland01"), Val::from(2)])?;
                                                        ctx.lines_as("Errende", args!["Because of the dangers of monster research, adventurers are needed to gather the information for the researchers."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Errende", args!["For wanderers like myself, it's a great way to earn money. We give them the information they need, and they give us financial support to live the way we please."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Errende", args!["Anyway, you should visit the Monster Museum in Juno to find out about Mr. Kitty. It's pretty far from here, but when you get to Juno, the Museum is West of the central plaza."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Errende",
                                                            args!["Once again,", "thank you so much", "for your help."],
                                                        )?;
                                                        ctx.close_window()?;
                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                        return Err(Stop::End);
                                                    } else {
                                                        if ctx.var("gef_bard_q").get()? == 22 {
                                                            ctx.lines_as(
                                                                "Errende",
                                                                args![
                                                                    "*Sigh...*",
                                                                    "Where can",
                                                                    "I find the 8th love...?",
                                                                    "Have you met Gunther?",
                                                                    "Ah, perhaps not yet."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.call(Function::Cutin, vec![Val::from("bard_eland04"), Val::from(2)])?;
                                                            ctx.lines_as("Errende", args!["*Sigh* I've lost the heart to sing ever since I've forgotten that line to the song. I just... can't. My spirit is unmoved. There's no inspiration."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Errende", args!["Please ask ^483D8BGunther^000000 about the ^483D8B8th love^000000 in ^483D8BAt One, I Fall in Love^000000. Thank you in advance."])?;
                                                            ctx.close_window()?;
                                                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                            return Err(Stop::End);
                                                        } else {
                                                            if ctx.var("gef_bard_q").get()? == 23 {
                                                                ctx.call(Function::Cutin, vec![Val::from("bard_eland02"), Val::from(2)])?;
                                                                ctx.lines_as("Errende", args!["So...", "Have you", "seen Gunther?"])?;
                                                                ctx.next()?;
                                                                ctx.lines(args![
                                                                    "^3355FFYou turn around",
                                                                    "to show him your back.^000000"
                                                                ])?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Errende", args!["Huh...?!", "Isn't that?!", "Is that the line of the song written on your back? Wait, don't move! The 8th love is...", "Now I see!"])?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Errende",
                                                                    args![
                                                                        "At One, I fall in love.",
                                                                        "At Two, you give me your smile.",
                                                                        "At Three, I adore your touch.",
                                                                        "At Four, a tender kiss."
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Errende",
                                                                    args![
                                                                        "At Five, we change our minds.",
                                                                        "A petal scatters through the air.",
                                                                        "At Six, I fall in love~",
                                                                        "At Seven, you fall in love~"
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Errende",
                                                                    args![
                                                                        "At Eight we turn away...",
                                                                        "At Nine, love is reborn.",
                                                                        "At Ten, my Love is gone.",
                                                                        "At Eleven I find out why."
                                                                    ],
                                                                )?;
                                                                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                                                    ctx.mes("At Twelve I see his new girlfriend?")?;
                                                                } else {
                                                                    ctx.mes("At Twelve I see her new boyfriend?")?;
                                                                }
                                                                ctx.next()?;
                                                                ctx.lines_as("Errende", args!["..."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Errende", args!["...", "......"])?;
                                                                ctx.next()?;
                                                                ctx.call(Function::Cutin, vec![Val::from("bard_eland04"), Val::from(2)])?;
                                                                ctx.lines_as(
                                                                    "Errende",
                                                                    args![
                                                                        "This...",
                                                                        "This cannot be.",
                                                                        "This song is supposed to be about love, not a romantic travesty!"
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Errende", args!["The lyrics. They must have been changed. Did Gunther say anything about this?! Hmmm, but who would change the lyrics...?"])?;
                                                                ctx.next()?;
                                                                ctx.call(Function::Cutin, vec![Val::from("bard_eland01"), Val::from(2)])?;
                                                                let (input, status) = runtime::input_text(ctx, None, None)?;
                                                                l_inputstr_s = input;
                                                                if l_inputstr_s.clone() == "Kino Kitty" {
                                                                    ctx.lines_as(
                                                                        "Errende",
                                                                        args![
                                                                            "Ah! Of course!",
                                                                            "I think you're right!",
                                                                            "How could I not think of that?"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Errende", args!["It all makes sense now. After all, he used to be a member of the Invincible Single Army. His changes might have been a little mean, since this song used to be about a happy couple..."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Errende", args!["Ummm...", "I'm sorry to ask a favor of you again, but in your travels, do you think you could find the original lyrics for this song? I can wait for it..."])?;
                                                                    ctx.next()?;
                                                                    if Val::from(runtime::select_values(
                                                                        ctx,
                                                                        &[Val::from("No, thanks.:I can, so stop crying.")],
                                                                    )?) == 1
                                                                    {
                                                                        ctx.lines_as("Errende", args!["Ah, I guess it was too much to ask of you. My apologies. Don't worry about it, I'll find out some other way."])?;
                                                                        ctx.var("gef_bard_q").set(Val::from(25))?;
                                                                    } else {
                                                                        ctx.lines_as(
                                                                            "Errende",
                                                                            args![
                                                                                "Are you serious?",
                                                                                "Oh, thank you so much!",
                                                                                "You must be an angel!",
                                                                                "An angel that truly understands the heart of a poet!"
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Errende",
                                                                            args![
                                                                                "I'll pay you back somehow!",
                                                                                "Thank you for your trouble",
                                                                                "in advance~"
                                                                            ],
                                                                        )?;
                                                                        ctx.var("gef_bard_q").set(Val::from(24))?;
                                                                    }
                                                                    ctx.close_window()?;
                                                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                    return Err(Stop::End);
                                                                } else if l_inputstr_s.clone() == "Gunther" {
                                                                    ctx.lines_as("Errende", args!["Gunther? I don't think he would do this. He always puts lines in his songs like 'doubleharmony for you.' Plus, he's too silly for that."])?;
                                                                    ctx.var("gef_bard_q").set(Val::from(26))?;
                                                                    ctx.close_window()?;
                                                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                    return Err(Stop::End);
                                                                } else if l_inputstr_s.clone() == "Gunther Doubleharmony" {
                                                                    ctx.lines_as("Errende", args!["Gunther? I don't think he would do this. He always puts lines in his songs like 'doubleharmony for you.' Plus, he's too silly for that."])?;
                                                                    ctx.var("gef_bard_q").set(Val::from(26))?;
                                                                    ctx.close_window()?;
                                                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                    return Err(Stop::End);
                                                                } else if l_inputstr_s.clone() == "Errende" {
                                                                    ctx.call(
                                                                        Function::Cutin,
                                                                        vec![Val::from("bard_eland04"), Val::from(2)],
                                                                    )?;
                                                                    ctx.lines_as(
                                                                        "Errende",
                                                                        args![
                                                                            "Surely you jest!",
                                                                            "If I did, why would",
                                                                            "I not know what this",
                                                                            "song is about?"
                                                                        ],
                                                                    )?;
                                                                    ctx.var("gef_bard_q").set(Val::from(26))?;
                                                                    ctx.close_window()?;
                                                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                    return Err(Stop::End);
                                                                } else {
                                                                    ctx.lines_as(
                                                                        "Errende",
                                                                        args![
                                                                            ((Val::from("") + l_inputstr_s.clone()) + Val::from("...?")),
                                                                            "I don't think I know that person. Maybe you misunderstood",
                                                                            "something? *Sigh...*"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Errende",
                                                                        args![
                                                                            "What was the line...?",
                                                                            "How could I forget",
                                                                            "the 8th love?"
                                                                        ],
                                                                    )?;
                                                                    ctx.var("gef_bard_q").set(Val::from(26))?;
                                                                    ctx.close_window()?;
                                                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                    return Err(Stop::End);
                                                                }
                                                            } else {
                                                                if ctx.var("gef_bard_q").get()? == 21 {
                                                                    ctx.call(
                                                                        Function::Cutin,
                                                                        vec![Val::from("bard_eland01"), Val::from(2)],
                                                                    )?;
                                                                    ctx.lines_as(
                                                                        "Errende",
                                                                        args![
                                                                            "Welcome back,",
                                                                            ((Val::from("") + ctx.var("@name$").get()?) + Val::from("~")),
                                                                            "What would you like",
                                                                            "me to do for you?",
                                                                            "Would you like to hear",
                                                                            "a tale or listen to a song?"
                                                                        ],
                                                                    )?;
                                                                    bard_2_run(ctx, Bard2Step::SStorySong, vec![Val::from(3)])?;
                                                                } else {
                                                                    if ctx.var("gef_bard_q").get()? == 7 {
                                                                        ctx.call(
                                                                            Function::Cutin,
                                                                            vec![Val::from("bard_eland04"), Val::from(2)],
                                                                        )?;
                                                                        ctx.lines_as("Errende", args!["How could I forget the song?", "I'm worthless as a Bard~! Please... Just... Please just leave me alone."])?;
                                                                        ctx.next()?;
                                                                        if Val::from(runtime::select_values(
                                                                            ctx,
                                                                            &[Val::from("No problem.:Is there any way I can help you?")],
                                                                        )?) == 1
                                                                        {
                                                                            ctx.lines_as("Errende", args!["Waaaah~! You're so mean!", "You're supposed to say, 'Errende, what's wrong? Maybe I can help?'"])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Errende", args!["I mean, we're supposed to know each other better than that. But, but maybe I was wrong!"])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Errende", args!["*Sob...*", "Who made this poor Bard cry by neglecting him? *Sniff* I do believe it was you!"])?;
                                                                        } else {
                                                                            ctx.call(
                                                                                Function::Cutin,
                                                                                vec![Val::from("bard_eland01"), Val::from(2)],
                                                                            )?;
                                                                            ctx.lines_as(
                                                                                "Errende",
                                                                                args!["Help me?", "Hmmmmmmm...", "That's it!", "Gunther!"],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Errende", args!["Let me ask you a favor. If perchance you happen to meet ^3355FFGunther Doubleharmony^000000, please inform him of my dilemna."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Errende", args!["Tell him that ^483D8BMinty Errende^000000 happened to forget a line of the song, ^483D8BAt One, I Fall in Love^000000. The line is called ^483D8B8th love^000000."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Errende", args!["I beseech you, if you meet him, please ask him of the 8th love and inform me of that lyric immediately~"])?;
                                                                            ctx.var("gef_bard_q").set(Val::from(2))?;
                                                                        }
                                                                        ctx.close_window()?;
                                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                        return Err(Stop::End);
                                                                    } else {
                                                                        if ctx.var("gef_bard_q").get()? == 6 {
                                                                            ctx.call(
                                                                                Function::Cutin,
                                                                                vec![Val::from("bard_eland04"), Val::from(2)],
                                                                            )?;
                                                                            ctx.lines_as("Errende", args!["Who could have changed the lyrics to the song? I can't understand why such a thing would happen. I really wish I could ask the person who changed the words..."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Errende", args!["*Sigh* I'm so frustrated I can't even sing. I just... can't. My spirit is unmoved. There's no inspiration."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as(
                                                                                ctx.var("@name$").get()?,
                                                                                args![
                                                                                    "Could it be that",
                                                                                    "the person who",
                                                                                    "changed the song is..."
                                                                                ],
                                                                            )?;
                                                                            let (input, status) = runtime::input_text(ctx, None, None)?;
                                                                            l_inputstr_s = input;
                                                                            ctx.lines(args![
                                                                                ((Val::from("") + l_inputstr_s.clone()) + Val::from("?"))
                                                                            ])?;
                                                                            ctx.next()?;
                                                                            if l_inputstr_s.clone() == "Kino Kitty" {
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("bard_eland01"), Val::from(2)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Errende",
                                                                                    args![
                                                                                        "Ah! Of course!",
                                                                                        "I think you're right!",
                                                                                        "How could I not think of that?"
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Errende", args!["It all makes sense now. After all, he used to be a member of the Invincible Single Army. His changes might have been a little mean, since this song used to be about a happy couple..."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Errende", args!["Ummm...", "I'm sorry to ask a favor of you again, but in your travels, do you think you could find the original lyrics for this song? I can wait for it..."])?;
                                                                                ctx.next()?;
                                                                                if Val::from(runtime::select_values(
                                                                                    ctx,
                                                                                    &[Val::from("No, thanks.:I can, so stop crying.")],
                                                                                )?) == 1
                                                                                {
                                                                                    ctx.lines_as("Errende", args!["Ah, I guess it was too much to ask of you. My apologies. Don't worry about it, I'll find out some other way."])?;
                                                                                    ctx.var("gef_bard_q").set(Val::from(5))?;
                                                                                } else {
                                                                                    ctx.lines_as("Errende", args!["Are you serious? Oh, thank you so much! You must be an angel! An angel that truly understands the heart of a poet!"])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as("Errende", args!["I'll pay you back somehow! Thank you for your trouble in advance~"])?;
                                                                                    ctx.var("gef_bard_q").set(Val::from(4))?;
                                                                                }
                                                                                ctx.close_window()?;
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from(""), Val::from(255)],
                                                                                )?;
                                                                                return Err(Stop::End);
                                                                            } else if l_inputstr_s.clone() == "Gunther" {
                                                                                ctx.lines_as("Errende", args!["Gunther? I don't think he would do this. He always puts lines in his songs like 'doubleharmony for you.' Plus, he's too silly for that."])?;
                                                                                ctx.close_window()?;
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from(""), Val::from(255)],
                                                                                )?;
                                                                                return Err(Stop::End);
                                                                            } else if l_inputstr_s.clone() == "Gunther Doubleharmony" {
                                                                                ctx.lines_as("Errende", args!["Gunther? I don't think he would do this. He always puts lines in his songs like 'doubleharmony for you.' Plus, he's too silly for that."])?;
                                                                                ctx.close_window()?;
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from(""), Val::from(255)],
                                                                                )?;
                                                                                return Err(Stop::End);
                                                                            } else if l_inputstr_s.clone() == "Errende" {
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("bard_eland04"), Val::from(2)],
                                                                                )?;
                                                                                ctx.lines_as("Errende", args!["Surely you jest! If I did, why would I not know what this song is about?"])?;
                                                                                ctx.close_window()?;
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from(""), Val::from(255)],
                                                                                )?;
                                                                                return Err(Stop::End);
                                                                            } else {
                                                                                ctx.lines_as("Errende", args![((Val::from("") + l_inputstr_s.clone()) + Val::from("...?")), "I don't think I know that person. Maybe you misunderstood something? *Sigh...*"])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as(
                                                                                    "Errende",
                                                                                    args![
                                                                                        "What was the line...?",
                                                                                        "How could I forget",
                                                                                        "the 8th love?"
                                                                                    ],
                                                                                )?;
                                                                                ctx.var("gef_bard_q").set(Val::from(6))?;
                                                                                ctx.close_window()?;
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from(""), Val::from(255)],
                                                                                )?;
                                                                                return Err(Stop::End);
                                                                            }
                                                                        } else {
                                                                            if ctx.var("gef_bard_q").get()? == 5 {
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("bard_eland04"), Val::from(2)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Errende",
                                                                                    args![
                                                                                        "Oh...!",
                                                                                        "These tears just won't stop!",
                                                                                        "Wh-why must people always treat Bards like this?"
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines(args![
                                                                                    "^483D8BWho made this poor Bard cry?",
                                                                                    "Who broke his tender heart of glass?",
                                                                                    "Shattering his dreams...",
                                                                                    "No dreams, no heart, no love, no hope, no...^000000"
                                                                                ])?;
                                                                                ctx.next()?;
                                                                                l_random = ctx.call(
                                                                                    Function::Rand,
                                                                                    vec![Val::from(1), Val::from(50)],
                                                                                )?;
                                                                                if (l_random.clone().number()? > 27
                                                                                    && l_random.clone().number()? < 37)
                                                                                {
                                                                                    ctx.mes("^3355FFErrende continues to sing about his personal despair. He seems to be disappointed in your refusal to help him. Of course, you begin to feel sorry for him.^000000")?;
                                                                                    ctx.next()?;
                                                                                    if Val::from(runtime::select_values(
                                                                                        ctx,
                                                                                        &[Val::from(
                                                                                            "Well, I should help him then...:Ignore him anyway.",
                                                                                        )],
                                                                                    )?) == 1
                                                                                    {
                                                                                        ctx.lines_as(ctx.var("@name$").get()?, args!["Hey. Hey, Errende. Stop singing this song. It's embarassing, okay? Alright, I'll go find the original song for you."])?;
                                                                                        ctx.next()?;
                                                                                        ctx.call(
                                                                                            Function::Cutin,
                                                                                            vec![Val::from("bard_eland01"), Val::from(2)],
                                                                                        )?;
                                                                                        ctx.lines_as("Errende", args!["*Gasp!* Really! Are you sure? Thank you! Thank you so much! You really do care about the sadness of a Bard!"])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as("Errende", args!["I promise to pay you back as best as I can! I'll wait for you here until you return!"])?;
                                                                                        ctx.var("gef_bard_q").set(Val::from(4))?;
                                                                                    } else {
                                                                                        ctx.lines(args![
                                                                                            "^3355FFYou ignore his",
                                                                                            "heart wrenching song.",
                                                                                            "But at what cost to your soul?^000000"
                                                                                        ])?;
                                                                                    }
                                                                                    ctx.close_window()?;
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from(""), Val::from(255)],
                                                                                    )?;
                                                                                    return Err(Stop::End);
                                                                                } else {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("bard_eland01"), Val::from(2)],
                                                                                    )?;
                                                                                    ctx.lines_as("Errende", args!["You don't have anything you want to ask me, do you? If not, would you like to listen to my song...?"])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines(args![
                                                                                        "^483D8BMiiiiserable...",
                                                                                        "Noboooody looooves meee",
                                                                                        "Friends foooooor never...",
                                                                                        "Ooooooooooh wah!^000000"
                                                                                    ])?;
                                                                                    ctx.close_window()?;
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from(""), Val::from(255)],
                                                                                    )?;
                                                                                    return Err(Stop::End);
                                                                                }
                                                                            } else {
                                                                                if ctx.var("gef_bard_q").get()? == 4 {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("bard_eland01"), Val::from(2)],
                                                                                    )?;
                                                                                    ctx.lines_as("Errende", args!["Umm...", "I am not sure exactly where Mr. Kitty is. Maybe you might be able to find out at the Monster Museum in Juno? I think he's a member of the Monster Research Organization."])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as("Errende", args!["If you're experienced in exploration, you'd know the Monster Research Organization. Recently, many scholars have been wounded as a result of researching monsters."])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as("Errende", args!["Because of the dangers of monster research, adventurers are needed to gather the information for the researchers."])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as("Errende", args!["For wanderers like myself, it's a great way to earn money. We give them the information they need, and they give us financial support to live the way we please."])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as("Errende", args!["Anyway, you should visit the Monster Museum in Juno to find out about Mr. Kitty. It's pretty far from here, but when you get to Juno, the Museum is West of the central plaza."])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as(
                                                                                        "Errende",
                                                                                        args![
                                                                                            "Once again,",
                                                                                            "thank you so much",
                                                                                            "for your help."
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.close_window()?;
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from(""), Val::from(255)],
                                                                                    )?;
                                                                                    return Err(Stop::End);
                                                                                } else {
                                                                                    if ctx.var("gef_bard_q").get()? == 2 {
                                                                                        ctx.call(
                                                                                            Function::Cutin,
                                                                                            vec![Val::from("bard_eland04"), Val::from(2)],
                                                                                        )?;
                                                                                        ctx.lines_as("Errende", args!["*Sigh...* Where can I find the 8th love...? Have you met Gunther?", "Ah, perhaps not yet."])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as("Errende", args!["*Sigh* I've lost the heart to sing ever since I forgotten that line to the song. I just... can't. My spirit is unmoved. There's no inspiration."])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as("Errende", args!["Please ask ^483D8BGunther^000000 about the ^483D8B8th love^000000 in ^483D8BAt One, I Fall in Love^000000. Thank you in advance."])?;
                                                                                        ctx.close_window()?;
                                                                                        ctx.call(
                                                                                            Function::Cutin,
                                                                                            vec![Val::from(""), Val::from(255)],
                                                                                        )?;
                                                                                        return Err(Stop::End);
                                                                                    } else if ctx.var("gef_bard_q").get()? == 3 {
                                                                                        ctx.call(
                                                                                            Function::Cutin,
                                                                                            vec![Val::from("bard_eland02"), Val::from(2)],
                                                                                        )?;
                                                                                        ctx.lines_as(
                                                                                            "Errende",
                                                                                            args!["So...", "Have you", "seen Gunther?"],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines(args![
                                                                                            "^3355FFYou turn around",
                                                                                            "to show him your back.^000000"
                                                                                        ])?;
                                                                                        ctx.next()?;
                                                                                        ctx.call(
                                                                                            Function::Cutin,
                                                                                            vec![Val::from("bard_eland03"), Val::from(2)],
                                                                                        )?;
                                                                                        ctx.lines_as("Errende", args!["Huh...?!", "Isn't that?!", "Is that the line of the song written on your back? Wait, don't move! The 8th love is...", "Now I see!"])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "Errende",
                                                                                            args![
                                                                                                "At One, I fall in love.",
                                                                                                "At Two, you give me your smile.",
                                                                                                "At Three, I adore your touch.",
                                                                                                "At Four, a tender kiss."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "Errende",
                                                                                            args![
                                                                                                "At Five, we change our minds.",
                                                                                                "A petal scatters through the air.",
                                                                                                "At Six, I fall in love~",
                                                                                                "At Seven, you fall in love~"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "Errende",
                                                                                            args![
                                                                                                "At Eight we turn away...",
                                                                                                "At Nine, love is reborn.",
                                                                                                "At Ten, my Love is gone.",
                                                                                                "At Eleven I find out why."
                                                                                            ],
                                                                                        )?;
                                                                                        if ctx
                                                                                            .var("Sex")
                                                                                            .get()?
                                                                                            .loosely_equals(&ctx.constant("SEX_MALE")?)
                                                                                        {
                                                                                            ctx.mes("At Twelve I see her new boyfriend?")?;
                                                                                        } else {
                                                                                            ctx.mes("At Twelve I see his new girlfriend?")?;
                                                                                        }
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as("Errende", args!["..."])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as("Errende", args!["...", "......"])?;
                                                                                        ctx.next()?;
                                                                                        ctx.call(
                                                                                            Function::Cutin,
                                                                                            vec![Val::from("bard_eland04"), Val::from(2)],
                                                                                        )?;
                                                                                        ctx.lines_as("Errende", args!["This...", "This cannot be.", "This song is supposed to be about love, not a romantic travesty!"])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as("Errende", args!["The lyrics. They must have been changed. Did Gunther say anything about this?! Hmmm, but who would change the lyrics...?"])?;
                                                                                        ctx.next()?;
                                                                                        let (input, status) =
                                                                                            runtime::input_text(ctx, None, None)?;
                                                                                        l_inputstr_s = input;
                                                                                        if l_inputstr_s.clone() == "Kino Kitty" {
                                                                                            ctx.call(
                                                                                                Function::Cutin,
                                                                                                vec![
                                                                                                    Val::from("bard_eland01"),
                                                                                                    Val::from(2),
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.lines_as(
                                                                                                "Errende",
                                                                                                args![
                                                                                                    "Ah! Of course!",
                                                                                                    "I think you're right!",
                                                                                                    "How could I not think of that?"
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.next()?;
                                                                                            ctx.lines_as("Errende", args!["It all makes sense now. After all, he used to be a member of the Invincible Single Army. His changes might have been a little mean, since this song used to be about a happy couple..."])?;
                                                                                            ctx.next()?;
                                                                                            ctx.lines_as("Errende", args!["Ummm...", "I'm sorry to ask a favor of you again, but in your travels, do you think you could find the original lyrics for this song? I can wait for it..."])?;
                                                                                            ctx.next()?;
                                                                                            if Val::from(runtime::select_values(
                                                                                                ctx,
                                                                                                &[Val::from(
                                                                                                    "No, thanks.:I can, so stop crying.",
                                                                                                )],
                                                                                            )?) == 1
                                                                                            {
                                                                                                ctx.lines_as("Errende", args!["Ah, I guess it was too much to ask of you. My apologies. Don't worry about it, I'll find out some other way."])?;
                                                                                                ctx.var("gef_bard_q").set(Val::from(5))?;
                                                                                            } else {
                                                                                                ctx.lines_as("Errende", args!["Are you serious? Oh, thank you so much! You must be an angel! An angel that truly understands the heart of a poet!"])?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines_as("Errende", args!["I'll pay you back somehow! Thank you for your trouble in advance~"])?;
                                                                                                ctx.var("gef_bard_q").set(Val::from(4))?;
                                                                                            }
                                                                                            ctx.close_window()?;
                                                                                            ctx.call(
                                                                                                Function::Cutin,
                                                                                                vec![Val::from(""), Val::from(255)],
                                                                                            )?;
                                                                                            return Err(Stop::End);
                                                                                        } else if l_inputstr_s.clone() == "Gunther" {
                                                                                            ctx.lines_as("Errende", args!["Gunther? I don't think he would do this. He always puts lines in his songs like 'doubleharmony for you.' Plus, he's too silly for that."])?;
                                                                                            ctx.var("gef_bard_q").set(Val::from(6))?;
                                                                                            ctx.close_window()?;
                                                                                            ctx.call(
                                                                                                Function::Cutin,
                                                                                                vec![Val::from(""), Val::from(255)],
                                                                                            )?;
                                                                                            return Err(Stop::End);
                                                                                        } else if l_inputstr_s.clone()
                                                                                            == "Gunther Doubleharmony"
                                                                                        {
                                                                                            ctx.lines_as("Errende", args!["Gunther? I don't think he would do this. He always puts lines in his songs like 'doubleharmony for you.' Plus, he's too silly for that."])?;
                                                                                            ctx.var("gef_bard_q").set(Val::from(6))?;
                                                                                            ctx.close_window()?;
                                                                                            ctx.call(
                                                                                                Function::Cutin,
                                                                                                vec![Val::from(""), Val::from(255)],
                                                                                            )?;
                                                                                            return Err(Stop::End);
                                                                                        } else if l_inputstr_s.clone() == "Errende" {
                                                                                            ctx.call(
                                                                                                Function::Cutin,
                                                                                                vec![
                                                                                                    Val::from("bard_eland04"),
                                                                                                    Val::from(2),
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.lines_as("Errende", args!["Surely you jest! If I did, why would I not know what this song is about?"])?;
                                                                                            ctx.var("gef_bard_q").set(Val::from(6))?;
                                                                                            ctx.close_window()?;
                                                                                            ctx.call(
                                                                                                Function::Cutin,
                                                                                                vec![Val::from(""), Val::from(255)],
                                                                                            )?;
                                                                                            return Err(Stop::End);
                                                                                        } else {
                                                                                            ctx.lines_as("Errende", args![((Val::from("") + l_inputstr_s.clone()) + Val::from("...?")), "I don't think I know that person. Maybe you misunderstood something? *Sigh...*"])?;
                                                                                            ctx.next()?;
                                                                                            ctx.lines_as(
                                                                                                "Errende",
                                                                                                args![
                                                                                                    "What was the line...?",
                                                                                                    "How could I forget",
                                                                                                    "the 8th love?"
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.var("gef_bard_q").set(Val::from(6))?;
                                                                                            ctx.close_window()?;
                                                                                            ctx.call(
                                                                                                Function::Cutin,
                                                                                                vec![Val::from(""), Val::from(255)],
                                                                                            )?;
                                                                                            return Err(Stop::End);
                                                                                        }
                                                                                    } else if ctx.var("gef_bard_q").get()? == 1 {
                                                                                        ctx.call(
                                                                                            Function::Cutin,
                                                                                            vec![Val::from("bard_eland01"), Val::from(2)],
                                                                                        )?;
                                                                                        ctx.lines_as(
                                                                                            "Errende",
                                                                                            args![
                                                                                                "Welcome back,",
                                                                                                ((Val::from("")
                                                                                                    + ctx.var("@name$").get()?)
                                                                                                    + Val::from("~")),
                                                                                                "What would you like",
                                                                                                "me to do for you?",
                                                                                                "Would you like to hear",
                                                                                                "a tale or listen to a song?"
                                                                                            ],
                                                                                        )?;
                                                                                        bard_2_run(
                                                                                            ctx,
                                                                                            Bard2Step::SStorySong,
                                                                                            vec![Val::from(4)],
                                                                                        )?;
                                                                                    } else {
                                                                                        ctx.call(
                                                                                            Function::Cutin,
                                                                                            vec![Val::from("bard_eland03"), Val::from(2)],
                                                                                        )?;
                                                                                        ctx.lines(args![
                                                                                            "^483D8BWhat day is",
                                                                                            "best for drinking?",
                                                                                            "La la la~",
                                                                                            "It's the day of",
                                                                                            "the earth, the sun",
                                                                                            "And the moon~",
                                                                                            "La la la~^000000"
                                                                                        ])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines(args![
                                                                                            "^483D8BLa la la~",
                                                                                            "I'll only",
                                                                                            "drink on one day~",
                                                                                            "So if you'll tell me",
                                                                                            "when you'll drink",
                                                                                            "I'll tell you when",
                                                                                            "I'll drink with you~^000000"
                                                                                        ])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines(args![
                                                                                            "^483D8BLet's get together",
                                                                                            "Yea yea ye-^000000 Hmmmmm...?"
                                                                                        ])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as("Bard", args!["Why, hello there. Oh, have you come to listen to my song and forget your worries?"])?;
                                                                                        ctx.next()?;
                                                                                        if Val::from(runtime::select_values(
                                                                                            ctx,
                                                                                            &[Val::from("Who are you?:Ignore him.")],
                                                                                        )?) == 1
                                                                                        {
                                                                                            ctx.call(
                                                                                                Function::Cutin,
                                                                                                vec![
                                                                                                    Val::from("bard_eland04"),
                                                                                                    Val::from(2),
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.lines_as(
                                                                                                ctx.var("@name$").get()?,
                                                                                                args![
                                                                                                    "You seem to be",
                                                                                                    "new around here...",
                                                                                                    "Who are you?"
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.next()?;
                                                                                            ctx.call(
                                                                                                Function::Cutin,
                                                                                                vec![
                                                                                                    Val::from("bard_eland02"),
                                                                                                    Val::from(2),
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.lines_as("Errende", args!["Mm? Ah yes. I am merely another wandering poet who goes where the wind takes him. Please call me ^483D8BErrende^000000, the Bard who wishes to please you."])?;
                                                                                            ctx.next()?;
                                                                                            ctx.lines_as("Errende", args!["If you will let me, I will tell you of my travels. By your leave,", "I will play a song that will help you forget your troubles."])?;
                                                                                            ctx.var("gef_bard_q").set(Val::from(1))?;
                                                                                            bard_2_run(
                                                                                                ctx,
                                                                                                Bard2Step::SStorySong,
                                                                                                vec![Val::from(5)],
                                                                                            )?;
                                                                                        } else {
                                                                                            ctx.lines_as("Errende", args!["Waaah, wah~", "You can't just ignore me like that! Where's your sense of merriment, your sense of romance?"])?;
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
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
            Bard2Step::SStorySong => {
                ctx.next()?;
                'b1: {
                    let subject1 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Tell me a story.:Would you play a song?:Eh, maybe later.")],
                    )?);
                    let mut matched1 = false;
                    let no_case1 = !subject1.loosely_equals(&Val::from(1))
                        && !subject1.loosely_equals(&Val::from(2))
                        && !subject1.loosely_equals(&Val::from(3));
                    if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as("Errende", args!["You like stories, huh? What kind of story would you like me to tell? I'll you whatever I know... Just for you, of course. *Grins*"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("News and rumors~:Cancel.")])?) == 1 {
                            ctx.call(Function::Cutin, vec![Val::from("bard_eland01"), Val::from(2)])?;
                            ctx.mes("[Errende]")?;
                            l_random = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                            if l_random.clone() == 1 {
                                ctx.mes(
                                    "Hmmm. Then shall we talk about this town, Geffen? Have you ever been to the Pub or the Inn here?",
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["I'd be lying if I didn't say that the sisters that work the Pub and Inn are... ^FF6699sensuously captivating^000000."])?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["The elder one is gorgeous, graceful and carries with her an air of refinement. And the younger one is so cheerful, energetic and so... ^FF6699nubile^000000."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Errende",
                                    args![
                                        "But I digress.",
                                        "Sometimes though,",
                                        "just sometimes mind you,",
                                        "they're not as sexy as usual."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["It seems the spirit of their late father, God bless his soul for siring such hot women, possesses them from time to time."])?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("bard_eland02"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Errende",
                                    args![
                                        "I guess when someone acts even",
                                        "a little flirtacious towards those girls, their father's spirit takes over. I should know."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Errende",
                                    args!["I guess a father's love endures forever, even into the afterlife. Of course, I wouldn't know."],
                                )?;
                                ctx.next()?;
                                ctx.mes("[Errende]")?;
                                ctx.call(Function::Cutin, vec![Val::from("bard_eland01"), Val::from(2)])?;
                                ctx.lines(args![
                                    "Let's see...",
                                    "Look out for this really shady Merchant that hangs out behind",
                                    "the Pub in Geffen."
                                ])?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["Judging from his clothes, he must be from Morocc, which is a lot warmer. That's why he's got a Hood, even though it has no drawstrings, for the cooler weather over here."])?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["I must warn you, don't let him talk you into buying that Hood of his since he's selling it for an outrageous price. He's definitely not trustworthy."])?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["Lastly, I want to mention the lost city of Geffenia, hidden under the Geffen Tower. I hear some kind of condition is required to enter Geffenia, but I'm not sure."])?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["Geffenia is related to the two attractive ladies I mentioned earlier. It seems their father, William, entered Geffen Tower with a party to exterminate monsters, but he never returned."])?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["It's tragic that he left his family behind in that way. But perhaps, it is more tragic that his spirit scares away anyone interested in those girls."])?;
                            } else if l_random.clone() == 2 {
                                ctx.mes("Okay, let me tell you a story about Morocc, city of the desert. Adventurers worth their salt are expected to have explored the city and its surrounding desert.")?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["The Sphinx and the Pyramids are especially popular areas of exploration for adventurers. Have you been there before? I haven't yet, but perhaps someday I'll go."])?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["While I was in Morocc, I found a Merchant that sells Sword Maces to Priests and Priestess. Aside from those, Priests are prohibited from using any kind of weapon with blades."])?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["I also saw an energetic little boy who kept begging his father to tame a Munak for him. I heard the father speak a bit, and it seems that one of his friends from Morocc is lost in Alberta."])?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("bard_eland02"), Val::from(2)])?;
                                ctx.lines_as("Errende", args!["In any case, I helped the father with an errand, and he gave me a mysterious box in return. When I opened that box, I found a strange feather. I'm unaware of how it works, though..."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Errende",
                                    args!["I wonder if that man's friend, Pandger Mayer, ever found", "his way back home..."],
                                )?;
                            } else if l_random.clone() == 3 {
                                ctx.mes("Why don't we talk about Alberta? There is a sunken ship developed by an event agency as a place where adventurers may go on expeditions. It seems they're making a lot of money.")?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["When the sunken ship first drifted near Alberta, it was immediately found by one of the Alberta Security Knights."])?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["That Security Knight ventured inside and found an infant deep inside one of the rooms. Next to the baby was a music box."])?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["This baby was the one and only survivor from the sunken ship. He was brought up in Alberta, although he was treated with contempt when he was a child. He's fine now, but I digress."])?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["As for the sunken ship, I'm unsure of whether or not it was a pirate ship. It is said that skeleton monsters wearing pirate costumes roam its remains."])?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["One of Alberta's other mysteries is the elusive Turtle Island. It seems that one seafarer has found a dependable route to that place and is giving passage to adventurers."])?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["Still, it seems Turtle Island is not quite safe. Two squads of Alberta Security Knights have already traveled there, but have not yet returned. Verily, this is cause for concern."])?;
                                ctx.next()?;
                                ctx.lines_as("Errende", args!["Still, that didn't stop me from visiting that place. By sheer accident, I found the journal of an adventurer who had already been there. It seems he's a famous scholar now."])?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("bard_eland02"), Val::from(2)])?;
                                ctx.lines_as("Errende", args!["In that journal are details about an ultimate swordsman, and the exploration of Turtle Island. I'm sure that anyone seeking treasure in that place will find exciting adventure."])?;
                            }
                        } else {
                            ctx.call(Function::Cutin, vec![Val::from("bard_eland01"), Val::from(2)])?;
                            ctx.lines_as("Errende", args!["Oh, how disappointing. But promise me that you will drop by later, so that we can share stories and merriment."])?;
                        }
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                        matched1 = true;
                    }
                    if matched1 {
                        l_num = runtime::arg(&args, 0, Val::from(0));
                        ctx.mes("[Errende]")?;
                        if l_num.clone().number()? < 3 {
                            ctx.mes("You recognize my talent, so you deserve to listen to my songs! Now, what would you like to hear? I can play anything you want, you know.")?;
                        } else {
                            ctx.mes("At last, I've met someone who recognizes my talent! You deserve to listen to my songs! Now, what would you like to hear? I can play anything you want, you know.")?;
                        }
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Hmm, any song will do.:Play an upbeat song~!:Never mind...")])? {
                            1 => {
                                ctx.mes("[Errende]")?;
                                if ctx.var("Zeny").get()?.number()? > 499 {
                                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(500))?))?;
                                    ctx.call(Function::Cutin, vec![Val::from("bard_eland03"), Val::from(2)])?;
                                    ctx.lines(args!["Alright.", "Here we go~"])?;
                                    l_random = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                    if l_random.clone() == 1 {
                                        ctx.call(Function::SoundEffect, vec![Val::from("ring_of_nibelungen.wav"), Val::from(0)])?;
                                    } else if l_random.clone() == 2 {
                                        ctx.call(Function::SoundEffect, vec![Val::from("dont_forget_me_not.wav"), Val::from(0)])?;
                                    } else {
                                        ctx.call(Function::SoundEffect, vec![Val::from("in_to_the_abyss.wav"), Val::from(0)])?;
                                    }
                                } else {
                                    ctx.call(Function::Cutin, vec![Val::from("bard_eland04"), Val::from(2)])?;
                                    ctx.lines(args![
                                        "Ahahaha~",
                                        "My apologies,",
                                        "But I cannot offer my services for free. Even a Bard needs zeny to live, wouldn't you agree?"
                                    ])?;
                                }
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.call(Function::Cutin, vec![Val::from("bard_eland03"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Errende",
                                    args![
                                        "An upbeat song...?",
                                        "Hmmm... Let's see.",
                                        "An upbeat song,",
                                        "An upbeat song...",
                                        "Okay, here we go~"
                                    ],
                                )?;
                                ctx.next()?;
                                if (l_num.clone() == 3 || l_num.clone() == 4) {
                                    l_random = ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?;
                                } else {
                                    l_random = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                }
                                if l_random.clone() == 1 {
                                    ctx.lines(args![
                                        "^483D8BValhalla dazzles in gold",
                                        "The fifth as we know",
                                        "Is old Glast Heim!",
                                        "Glorious warriors answer",
                                        "The summons of Odin.^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^483D8BPalace of the dead",
                                        "With a silver roof~",
                                        "The third as we know...",
                                        "Valaskjalf!",
                                        "Glorious warriors answer",
                                        "The summons of Odin.^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^483D8BFive hundred forty doors",
                                        "In the grand halls of Valhalla",
                                        "Fling open to the heart of a hero.",
                                        "Eight hundred warriors gather",
                                        "Under the will of God, and",
                                        "Charge as one out through",
                                        "Those doors.^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^483D8BGodly warriors",
                                        "Step into Yggdrassil",
                                        "Towards their fate.",
                                        "With pride and honor,",
                                        "They accept Valyrie's",
                                        "Welcome.^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^483D8BWarriors fallen in battle",
                                        "In glorious clashes of red",
                                        "Death may have come,",
                                        "But your fame lives on",
                                        "and your spirit will be",
                                        "led to Valhalla.^000000"
                                    ])?;
                                    if l_num.clone() == 2 {
                                        ctx.next()?;
                                        ctx.lines_as("Errende", args!["..."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Errende", args!["...", "......"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Errende", args!["You know this song is about a legend regarding Valhalla don't you? Supposedly, more than one place was used as the sacred Hall of Honor."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Errende", args!["But only the gods will know if this was true or not. Was a place like Glast Heim really the fifth Valhalla? We humans", "may never know", "with certainty."])?;
                                    }
                                    ctx.close_window()?;
                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                    return Err(Stop::End);
                                } else if l_random.clone() == 2 {
                                    ctx.lines(args![
                                        "^483D8BThe sounds of galloping",
                                        "Echo in the distance.",
                                        "A cloud of hazy dust",
                                        "Fills the setting sun.",
                                        "Thousands of eyes open",
                                        "Torches on the castle",
                                        "Flare like thousands of Ifrits."
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^483D8BHear the throbbing of my heart,",
                                        "The blood flowing in my veins.",
                                        "Feeling the heaviness of my armor.",
                                        "The enemy has appeared before us.^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^483D8BBeat the drums hard, harder!",
                                        "Courage, soldiers, march forward!",
                                        "Shout loud, soldiers, louder!",
                                        "Today will never come back!^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^483D8BStun the sky",
                                        "Provoke the earth.",
                                        "I feel my heartbeat again.",
                                        "Blow the bugle to",
                                        "Sway the fortress.",
                                        "Today will never come back!^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines_as("Errende", args!["Ah, this is called 'Drumming in the Battlefield,' which was written by Mr. Iolo. Yes, I rather like this song."])?;
                                } else {
                                    if (l_num.clone() == 3 || l_num.clone() == 4) {
                                        ctx.mes("[Errende]")?;
                                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                            ctx.mes("Heroic warrior,")?;
                                        } else {
                                            ctx.mes("My fair lady,")?;
                                        }
                                        ctx.mes("Please listen to my song. If you have a flower in hand and are in love, let's count the flower petals as we go along.")?;
                                        ctx.next()?;
                                        ctx.lines(args![
                                            "^483D8BAt One, I fall in love.",
                                            "At Two, you give me your smile.",
                                            "At Three, I adore your touch.",
                                            "At Four, a tender kiss.",
                                            "At Five, we change our minds.",
                                            "A petal scatters through the air.^000000"
                                        ])?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("bard_eland04"), Val::from(2)])?;
                                        ctx.lines(args![
                                            "^483D8BAt Six, I fall in love~",
                                            "At Seven, you fall in love~",
                                            "At Eight~^000000"
                                        ])?;
                                        ctx.next()?;
                                        ctx.lines_as("Errende", args!["At eight~", "At... Eight...", "What was next...?"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Errende",
                                            args![
                                                "Oh my...!",
                                                "What was the next part?!",
                                                "What was the 8th love?!",
                                                "How shameful for a Bard",
                                                "to forget the words",
                                                "to a song!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Errende",
                                            args![
                                                "I...",
                                                "I can't bear the humiliation!",
                                                "Or the suspense of what",
                                                "happens next...!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Errende", args!["You're an adventurer, aren't you? So you must travel quite a bit? It's embarassing for me to ask,", "but I have a favor to ask...!"])?;
                                        ctx.next()?;
                                        if Val::from(runtime::select_values(
                                            ctx,
                                            &[Val::from("Sure, no problem.:I ain't gonna help you.")],
                                        )?) == 1
                                        {
                                            ctx.call(Function::Cutin, vec![Val::from("bard_eland01"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Errende",
                                                args!["Thank you, so much!", "Let's see, who would know?", "I've got it! Gunther!"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Errende", args!["If perchance you happen to meet ^483D8BGunther Doubleharmony^000000, please inform him of my dilemna."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Errende", args!["Tell him that ^483D8BMinty Errende^000000 happened to forget a line of the song, ^483D8BAt One, I Fall in Love^000000. The line is called ^483D8B8th love^000000."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Errende", args!["I beseech you, if you meet him, please ask him of the 8th love and inform me of that lyric immediately~"])?;
                                            if ctx.var("gef_bard_q").get()? == 1 {
                                                ctx.var("gef_bard_q").set(Val::from(2))?;
                                            }
                                            if ctx.var("gef_bard_q").get()? == 21 {
                                                ctx.var("gef_bard_q").set(Val::from(22))?;
                                            }
                                        } else {
                                            ctx.lines_as("Errende", args!["*Sigh...*", "I can't remember the 8th part of this song if my life depended on it. And it does~! *Wahhhh~*"])?;
                                            if ctx.var("gef_bard_q").get()? == 1 {
                                                ctx.var("gef_bard_q").set(Val::from(7))?;
                                            }
                                            if ctx.var("gef_bard_q").get()? == 21 {
                                                ctx.var("gef_bard_q").set(Val::from(27))?;
                                            }
                                        }
                                        ctx.close_window()?;
                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "^483D8BA good Bard sings",
                                        "To please his listener.",
                                        "So do not expect a sad song",
                                        "That deepens your anguish.^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^483D8BA good Dancer dances",
                                        "To please her audience.",
                                        "Shall we dance together?",
                                        "Just hold my hands.",
                                        "La la la~ La la la~^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.var("@name$").get()?,
                                        args![
                                            "By the way...",
                                            "Why do you guys play",
                                            "discords sometimes?",
                                            "It sounds weird",
                                            "when you do that."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Errende", args![((Val::from("H-how can you say such a thing, ") + ctx.var("@name$").get()?) + Val::from("? Have you ever been a Bard before? It's difficult to come up with fresh, original melodies!"))])?;
                                }
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                return Err(Stop::End);
                            }
                            3 => {
                                ctx.call(Function::Cutin, vec![Val::from("bard_eland01"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Errende",
                                    args![
                                        "But of course.",
                                        "Merriment is best",
                                        "enjoyed when you",
                                        "are in the mood for it.",
                                        "Please come again."
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.call(Function::Cutin, vec![Val::from("bard_eland01"), Val::from(2)])?;
                        ctx.lines_as("Errende", args!["Hmm~?", "Well, alright. Though, listening to a good story or cheerful song can really do you some good. Alright then, see you later."])?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    }
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn bard_2(ctx: &Ctx) -> Script {
    bard_2_run(ctx, Bard2Step::Start, Vec::new()).map(|_| ())
}
