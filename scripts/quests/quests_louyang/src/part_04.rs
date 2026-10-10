use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn employee_poison_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    let mut l_r_o_o_f = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FF * Wait a minute! *",
            "You're carrying too many items with you right now. Please put some of your things into Kafra Storage and try again.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ch_par").get()?.number()? < 10 && ctx.var("ch_poison").get()?.number()? < 6) {
        'l1: loop {
            if !(true) {
                break 'l1;
            }
            'b1: {
                if l_r_o_o_f.clone().number()? > 5 {
                    break 'l1;
                } else if ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?.number()? > 2 {
                    l_r_o_o_f = (l_r_o_o_f.clone() + Val::from(1));
                    ctx.lines_as(
                        "Song Zhi Du",
                        args!["Let's see, there's that medicine, and then the medicine over here..."],
                    )?;
                    ctx.next()?;
                } else {
                    ctx.lines_as("Song Zhi Du", args!["...", "......"])?;
                    ctx.next()?;
                }
            }
        }
        ctx.lines_as(
            "Song Zhi Du",
            args!["I feel like you're looking at me. Is there something that you want?"],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from(
                "......:Ask him about his master.:Ask about Poison Organization.:Ask why he's working here.",
            )],
        )? {
            1 => {
                ctx.lines_as(
                    "Song Zhi Du",
                    args!["I guess there's nothing you really need from me. Well then, if you'll excuse me..."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Song Zhi Du",
                    args!["Oh, I'm sorry, but I'm kind of busy right now. You see, we're running out of medicine..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Song Zhi Du",
                    args![
                        "My master, Hua Tuo,",
                        "is also having difficulty treating all of his patients. I must get him the medicine that he needs..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as("Song Zhi Du", args!["What...?", "What did", "you just say...?"])?;
                ctx.next()?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                ctx.lines_as("Song Zhi Du", args![((Val::from("") + l_input_s.clone()) + Val::from("...?"))])?;
                ctx.next()?;
                ctx.lines_as(
                    "Song Zhi Du",
                    args!["I've never heard of that organization in my life. I'm sorry I can't help you. Now, if you'll excuse me..."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            4 => {
                ctx.lines_as("Song Zhi Du", args!["Of course I'm working here to learn about medicine. I'm grateful for the chance to work under the most famous doctor in this area."])?;
                ctx.next()?;
                ctx.lines_as("Song Zhi Du", args!["Doctor Hua Tuo is such a great person. I'm more than happy to assist him in saving the lives of others with his medical knowledge."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Song Zhi Du",
                    args!["Even though I'm just in charge of the medicine storage, I'm working hard to become his disciple someday."],
                )?;
                ctx.next()?;
                ctx.lines_as("Song Zhi Du", args!["And working with Doctor Hua Tuo is really worthwhile. Well, sometimes it's really hard to supply medicine when we have too many patients though."])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Why don't you gather medicinal herbs then?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Song Zhi Du",
                    args![
                        "My job is keeping this storage.",
                        "I'm not supposed to leave my position. I need to organize the herbs, and make sure they're in good condition."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        if (ctx.var("ch_par").get()?.number()? > 9 && ctx.var("ch_poison").get()?.number()? < 5) {
            ctx.lines_as(
                "Song Zhi Du",
                args!["Well, well. Thank you for your trouble. I didn't expect you to gather all these herbs for us..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Song Zhi Du",
                args!["I suppose I underestimated you. By the way, do you know anything about poison?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Yes, I am kind of interested in it.:No, not at all.")],
            )?) == 1
            {
                ctx.lines_as(
                    "Song Zhi Du",
                    args!["Oh, I see. Well, if you have some time, why don't you go visit my old master in the slums?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Song Zhi Du",
                    args!["If you two get along, I will tell you something important."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Song Zhi Du",
                args!["Huh. I see. Well, thank you for your trouble once again. I hope you have a good time in Luoyang."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if (ctx.var("ch_par").get()?.number()? > 9 && ctx.var("ch_poison").get()? == 5) {
                ctx.lines_as(
                    "Song Zhi Du",
                    args!["...", "So, did you talk", "to my old master?", "It seems he really", "likes you."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Song Zhi Du",
                    args![
                        "As you know, my master has been accused of something he didn't do! And they crippled his use of the martial arts!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Song Zhi Du",
                    args![
                        "I can't forgive what",
                        "they've have done to my master!",
                        "I want to grind their bones",
                        "to powder! And that's before",
                        "I kill them!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Song Zhi Du",
                    args![
                        "He's innocent and all he did",
                        "was research poison with all",
                        "of his effort! And for that,",
                        "they destroyed him!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Song Zhi Du",
                    args!["I was young", "when it happened...", "Too young to know", "any martial arts."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Song Zhi Du",
                    args![
                        "I couldn't protect him.",
                        "All I could do was hide and",
                        "watch other people get killed."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Song Zhi Du",
                    args![
                        "I hate myself",
                        "for not being strong!",
                        "But, my body isn't suited",
                        "for martial arts."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Song Zhi Du", args!["All I can do is maintain my weak body and make sure it's reasonably healthy. Even so, if it weren't for my master, I'd be dead", "in the streets..."])?;
                ctx.next()?;
                ctx.lines_as("Song Zhi Du", args!["It was my master who picked me up from the streets and cured me. So, I've decided to become a doctor and save as many lives as I can."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Song Zhi Du",
                    args!["But before", "I do that,", "I want ^FF0000revenge^000000!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Song Zhi Du",
                    args![
                        "As I told you, I am weak.",
                        "Too weak... I know that from",
                        "the bottom of my heart."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Song Zhi Du", args!["But..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Song Zhi Du",
                    args!["I am knowledgable about medicine, especially the use of poison. I can tell you that I'm one of the best."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Song Zhi Du",
                    args![
                        "Poison can kill people but it can also be used to save lifes. The people who destroyed my master",
                        "will pay the price."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Song Zhi Du", args!["Do you understand", "why I'm furious!?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("But revenge isn't good.:Yes, I fully understand.")],
                )?) == 1
                {
                    ctx.lines_as("Song Zhi Du", args!["Hmm...", "I see."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Song Zhi Du",
                        args![
                            "I suppose you couldn't",
                            "understand the way I feel.",
                            "After all, you didn't have",
                            "to go through the same",
                            "things I did."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Song Zhi Du",
                        args![
                            "I suppose I expected too much, since my master likes you. How could you know the rage and",
                            "sadness that I feel!?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Song Zhi Du",
                        args!["Fine.", "Go do whatever", "you were going to do.", "I'm just disappointed..."],
                    )?;
                    ctx.var("ch_poison").set(Val::from(6))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(11070), Val::from(11071)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Song Zhi Du", args!["Yes! Yes!", "You do understand!"])?;
                ctx.next()?;
                ctx.lines_as("Song Zhi Du", args!["I volunteered for this medicine storage position so that I can secretly study poison! Now, the time for action has come!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Song Zhi Du",
                    args![
                        "Still, I'll need some materials to complete my research. Then, when",
                        "I succeed and create a poison pill, I'll need someone to carry my revenge out for me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Song Zhi Du",
                    args!["Since my body is so frail, I can't bear the tension and rage of seeing my lifelong enemy face to face."],
                )?;
                ctx.var("ch_poison").set(Val::from(8))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(11070), Val::from(11073)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if (ctx.var("ch_par").get()?.number()? > 9 && ctx.var("ch_poison").get()? == 6) {
                    ctx.lines_as(
                        "Song Zhi Du",
                        args![
                            "I'm so disappointed.",
                            "You've even seen the",
                            "pitiable state of my",
                            "master for yourself!",
                            "How could you not",
                            "understand me?!"
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("I'm sorry for last time.:No matter what, revenge isn't good.")],
                    )?) == 1
                    {
                        ctx.lines_as(
                            "Song Zhi Du",
                            args![
                                "If you really",
                                "feel sorry for me,",
                                "then you must help me",
                                "carry out my revenge!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Song Zhi Du",
                            args![
                                "I still need to complete the",
                                "poison potion I'm creating. I hope you can bring what I need to finish it. Go and get me..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Song Zhi Du",
                            args![
                                "^0000FF4 Bee Sting,",
                                "10 Venom Canine,",
                                "10 Empty Potion,",
                                "30 Green Potion^000000."
                            ],
                        )?;
                        ctx.var("ch_poison").set(Val::from(7))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(11071), Val::from(11072)])?;
                        ctx.next()?;
                        ctx.lines_as("Song Zhi Du", args!["An apology is fine, but you must also show me that you are sorry and help me carry out my plan. Do you have any problem with this?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Song Zhi Du",
                        args![
                            "Oh, forget about it.",
                            "I don't think I can ever",
                            "make you understand how I feel."
                        ],
                    )?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_ENDURE")?])?;
                    ctx.close_window()?;
                    ctx.var("ch_poison").set(Val::from(6))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(11070), Val::from(11071)])?;
                    return Err(Stop::End);
                } else {
                    if (ctx.var("ch_par").get()?.number()? > 9 && ctx.var("ch_poison").get()? == 7) {
                        ctx.lines_as("Song Zhi Du", args!["So, did you", "gather everything", "I asked of you?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:What do you need again?")])?) == 1 {
                            if (((ctx.call(Function::CountItem, vec![Val::from(939)])?.number()? > 3
                                && ctx.call(Function::CountItem, vec![Val::from(937)])?.number()? > 9)
                                && ctx.call(Function::CountItem, vec![Val::from(1093)])?.number()? > 9)
                                && ctx.call(Function::CountItem, vec![Val::from(506)])?.number()? > 29)
                            {
                                ctx.call(Function::DelItem, vec![Val::from(939), Val::from(4)])?;
                                ctx.call(Function::DelItem, vec![Val::from(937), Val::from(10)])?;
                                ctx.call(Function::DelItem, vec![Val::from(1093), Val::from(10)])?;
                                ctx.call(Function::DelItem, vec![Val::from(506), Val::from(30)])?;
                                ctx.var("ch_poison").set(Val::from(8))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(11072), Val::from(11073)])?;
                                ctx.lines_as("Song Zhi Du", args!["Ah...", "With these, I accept your apology. Thank you for all the trouble you went through to get this stuff."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Song Zhi Du",
                                    args![
                                        "Now that you're this involved, you're in this with me all the way! I've been waiting for this",
                                        "day for years..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Song Zhi Du",
                                    args![
                                        "All that's left is to create this poison, and then to get the",
                                        "lord of Luoyang to drink it..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Song Zhi Du",
                                args![
                                    "Where are things",
                                    "I asked you to bring?",
                                    "Are you testing my",
                                    "patience or what?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Song Zhi Du", args!["You're still missing some of the items I need. I must have them all in order to finish making this poison!"])?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_ENDURE")?])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Song Zhi Du", args!["*Sigh...*", "Go and get me..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Song Zhi Du",
                            args![
                                "^0000FF4 Bee Sting,",
                                "10 Venom Canine,",
                                "10 Empty Potion,",
                                "30 Green Potion^000000."
                            ],
                        )?;
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_ENDURE")?])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if (ctx.var("ch_par").get()?.number()? > 9 && ctx.var("ch_poison").get()? == 8) {
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                            ctx.lines_as(
                                "Song Zhi Du",
                                args!["Ah, hello.", "Please give me a minute,", "I've just received a message."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Song Zhi Du",
                                args!["Ah!", "It says here", "that my delivery", "has finally arrived!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Song Zhi Du",
                                args![
                                    "Would you bring me the",
                                    "box from the firecracker",
                                    "lady at the entrance",
                                    "to Luoyang? Thanks",
                                    "in advance."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if (ctx.var("ch_par").get()?.number()? > 9 && ctx.var("ch_poison").get()? == 9) {
                                if ctx.call(Function::CountItem, vec![Val::from(7126)])?.number()? > 0 {
                                    ctx.call(
                                        Function::DelItem,
                                        vec![Val::from(7126), ctx.call(Function::CountItem, vec![Val::from(7126)])?],
                                    )?;
                                    ctx.lines_as("Song Zhi Du", args!["Ah. Thank you,", "I needed this. Now,", "shall we begin?"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Song Zhi Du",
                                        args![
                                            "First, I shall mix a poison extracted from Venom Canine",
                                            "with a foreign liquid named Karvodailnirol."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Song Zhi Du", args!["Then, I add Green Herb extract, poison extracted from Bee Sting and a Large Jellopy into the liquid! Finally, I must heat them all!"])?;
                                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL")?])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Song Zhi Du",
                                        args!["Finally, I have to carefully heat the mixture and collect it all into a Potion Bottle."],
                                    )?;
                                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_ENCHANTPOISON")?])?;
                                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Song Zhi Du",
                                        args!["Now, I've got to do this just right. This is a very delicate procedure..."],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_MAGNUMBREAK")?])?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                                    ctx.lines_as("Song Zhi Du", args!["No!! I failed again! ^666666*Sigh...*^000000 And I spent a long time preparing all of those materials..."])?;
                                    ctx.var("ch_poison").set(Val::from(10))?;
                                    ctx.call(Function::ChangeQuest, vec![Val::from(11074), Val::from(11075)])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                                ctx.lines_as(
                                    "Song Zhi Du",
                                    args!["Ah, hello.", "Please give me a minute,", "I've just received a message."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Song Zhi Du",
                                    args!["Ah!", "It says here", "that my delivery", "has finally arrived!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Song Zhi Du",
                                    args![
                                        "Would you bring me the",
                                        "box from the firecracker",
                                        "lady at the entrance",
                                        "to Luoyang? Thanks",
                                        "in advance."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if (ctx.var("ch_par").get()?.number()? > 9 && ctx.var("ch_poison").get()? == 10) {
                                    if ((((ctx.call(Function::CountItem, vec![Val::from(939)])?.number()? > 3
                                        && ctx.call(Function::CountItem, vec![Val::from(937)])?.number()? > 9)
                                        && ctx.call(Function::CountItem, vec![Val::from(1093)])?.number()? > 9)
                                        && ctx.call(Function::CountItem, vec![Val::from(506)])?.number()? > 29)
                                        && ctx.call(Function::CountItem, vec![Val::from(7126)])?.number()? > 0)
                                    {
                                        ctx.lines_as(
                                            "Song Zhi Du",
                                            args!["Oh! You brought", "me everything I need!", "I'm very impressed!"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Song Zhi Du",
                                            args![
                                                "You're the only one",
                                                "who actually understands how",
                                                "I feel. Thank you for gathering all of these ^FF0000materials^000000."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(
                                            Function::DelItem,
                                            vec![Val::from(939), ctx.call(Function::CountItem, vec![Val::from(939)])?],
                                        )?;
                                        ctx.call(
                                            Function::DelItem,
                                            vec![Val::from(937), ctx.call(Function::CountItem, vec![Val::from(937)])?],
                                        )?;
                                        ctx.call(
                                            Function::DelItem,
                                            vec![Val::from(1093), ctx.call(Function::CountItem, vec![Val::from(1093)])?],
                                        )?;
                                        ctx.call(
                                            Function::DelItem,
                                            vec![Val::from(506), ctx.call(Function::CountItem, vec![Val::from(506)])?],
                                        )?;
                                        ctx.call(
                                            Function::DelItem,
                                            vec![Val::from(7126), ctx.call(Function::CountItem, vec![Val::from(7126)])?],
                                        )?;
                                        ctx.lines_as(
                                            "Song Zhi Du",
                                            args![
                                                "Alright, now",
                                                "to create the poison.",
                                                "This is going to be tough.",
                                                "Here we go..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(1000)])?.number()? > 700 {
                                            ctx.lines_as(
                                                "Song Zhi Du",
                                                args![
                                                    "First, I shall mix a poison extracted from Venom Canine",
                                                    "with a foreign liquid named Karvodailnirol."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Song Zhi Du",
                                                args![
                                                    "Then, I add Green Herb extract,",
                                                    "a poison extracted from Bee Sting and a Large Jellopy!"
                                                ],
                                            )?;
                                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL")?])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Song Zhi Du",
                                                args![
                                                    "Finally, I have to carefully heat the mixture and collect it all into a Potion Bottle."
                                                ],
                                            )?;
                                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_ENCHANTPOISON")?])?;
                                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
                                            ctx.next()?;
                                            ctx.lines_as("Song Zhi Du", args!["*Phew...*", "Did...", "Did I make it?"])?;
                                            ctx.next()?;
                                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_PATTACK")?])?;
                                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONHIT")?])?;
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                                            ctx.lines_as(
                                                "Song Zhi Du",
                                                args!["Hahaha~!", "Success! ", "It works!", "I finally created it!", "Mwahahahahahah!"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Song Zhi Du", args!["Muhahahahaha!"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Song Zhi Du", args!["Muhahahahaha!", "Hahahahahahahahahahah!"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Song Zhi Du", args!["I made it!", "I made it!", "Now, revenge will be mine!"])?;
                                            ctx.next()?;
                                            ctx.var("ch_poison").set(Val::from(11))?;
                                            ctx.call(Function::GetItem, vec![Val::from(678), Val::from(2)])?;
                                            ctx.lines_as("Song Zhi Du", args!["Hahaha! Now, please", "take this bottle. But be careful. Even if you smell it just a little, it can cause your body to decompose, leading to death."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as(
                                            "Song Zhi Du",
                                            args![
                                                "First, I shall mix a poison extracted from Venom Canine",
                                                "with a foreign liquid named Karvodailnirol."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Song Zhi Du",
                                            args!["Then I add Green Herb extract, a poison extracted from Bee Sting and a Large Jellopy!"],
                                        )?;
                                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL")?])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Song Zhi Du",
                                            args!["Then I must carefully heat the mixture, and gather it all into a potion bottle."],
                                        )?;
                                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_ENCHANTPOISON")?])?;
                                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
                                        ctx.next()?;
                                        ctx.lines_as("Song Zhi Du", args!["*Phew...*", "Did...", "Did I make it?"])?;
                                        ctx.next()?;
                                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_MAGNUMBREAK")?])?;
                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                                        ctx.lines_as(
                                            "Song Zhi Du",
                                            args!["NO! I... I've failed again! And I spent a long time getting everything ready..."],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines_as(
                                        "Song Zhi Du",
                                        args![
                                            "I'm not sure what went wrong.",
                                            "Hmm, would you please help me again? I've used all the materials from last time."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Song Zhi Du",
                                        args![
                                            "^0000FF4 Bee Sting^000000,",
                                            "^0000FF10 Venom Canine^000000,",
                                            "^0000FF10 Empty Potion Bottle^000000,",
                                            "^0000FF30 Green Potion^000000 and",
                                            "^0000FF1 Large Jellopy^000000."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Song Zhi Du", args!["That's everything I'll need.", "I would apologize, but since you've shown that you'll help carry out my revenge, I know you'll understand."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if (ctx.var("ch_par").get()?.number()? > 9 && ctx.var("ch_poison").get()? == 11) {
                                        ctx.lines_as(
                                            "Song Zhi Du",
                                            args![
                                                "Hahahaha...!",
                                                "Now, time has come.",
                                                "With this poison, Luoyang's",
                                                "lord will be cast into hell!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Song Zhi Du",
                                            args![
                                                "If they lied about my master,",
                                                "then I'll simply make their lies into truth. Then, my master",
                                                "won't feel victimized!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Song Zhi Du", args!["Lord Bai Long!", "You will die!", "Ha HA HA HA HA!"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Song Zhi Du",
                                            args!["Muhahahaha!", "Hahaha...Haha..hahaha..haha...hahahahaha...mmmmuhahahahahahahaha!!"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Song Zhi Du", args!["Muhahahahahaha!", "..........."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Song Zhi Du",
                                            args!["Ah, forgive me.", "I was overly excited.", "Yes, I must calm down..."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Song Zhi Du",
                                            args!["Now, I have one last favor to ask of you. Please sneak into the Castle of the Dragon."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Song Zhi Du", args!["I want you to put this poison some place where Lord Bai Long might stay. But be careful, the castle has a lot of security."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Song Zhi Du", args!["Still, you're an adventurer from Midgard. You've probably had challenges like this before, so I'm sure you'll find a way."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Song Zhi Du",
                                            args![
                                                "Anyway, I'm sure my master",
                                                "will be happy to know that",
                                                "I finally created the ^0000FFpoison",
                                                "he wished to create^000000!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Song Zhi Du",
                                            args![
                                                "Hahaha...!",
                                                "Master, I did it!",
                                                "Your disciple Song Zhi Du made the world's deadliest poison for you!"
                                            ],
                                        )?;
                                        ctx.var("ch_poison").set(Val::from(12))?;
                                        ctx.call(Function::ChangeQuest, vec![Val::from(11075), Val::from(11076)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if (ctx.var("ch_par").get()?.number()? > 9 && ctx.var("ch_poison").get()? == 12) {
                                            ctx.lines_as(
                                                "Song Zhi Du",
                                                args![
                                                    "Remember...",
                                                    "The Castle of the Dragon",
                                                    "is under heavy surveillance.",
                                                    "I guess Lord Bai Long is",
                                                    "insecure. Heh heh."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Song Zhi Du", args!["Anyways, I hope you'll be", "really careful. When you go in, don't forget to use this poison somewhere the Lord Bai Long", "can ingest it."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Song Zhi Du",
                                                args![
                                                    "Anyway, I'm sure my master",
                                                    "will be happy to know that",
                                                    "I finally created the ^0000FFpoison",
                                                    "he wished to create^000000!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Song Zhi Du",
                                                args![
                                                    "Hahaha...!",
                                                    "Master, I did it!",
                                                    "Your disciple Song Zhi Du made the world's deadliest poison for you!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Song Zhi Du",
                                                args!["Leave now,", "my friend!", "For my revenge!", "Go and kill Lord Bai Long!"],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if (ctx.var("ch_par").get()?.number()? > 9 && ctx.var("ch_poison").get()? == 13) {
                                                ctx.lines_as("Song Zhi Du", args!["Ah!", "You came back!"])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Song Zhi Du",
                                                    args![
                                                        "Ah yes.",
                                                        "All I have to do now",
                                                        "is wait and hear news",
                                                        "of Lord Bai Long's death."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Song Zhi Du",
                                                    args!["Thank you so much, my friend. You've satisfied my old grudge."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Song Zhi Du",
                                                    args![
                                                        "Now, I am getting tired.",
                                                        "Let me rest... Take care, my friend, and travel in safety."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Song Zhi Du",
                                                    args![
                                                        "But before you go, let me give you some poison and a medicinal pill.",
                                                        "I made these with the leftover materials and medicine in the storage."
                                                    ],
                                                )?;
                                                ctx.var("ch_poison").set(Val::from(14))?;
                                                ctx.call(Function::GetItem, vec![Val::from(678), Val::from(3)])?;
                                                ctx.next()?;
                                                ctx.lines_as("Song Zhi Du", args!["Please take the", "medicine pill", "right away."])?;
                                                ctx.next()?;
                                                ctx.lines(args![
                                                    "^3355FFYou swallowed a strange pill",
                                                    "that shines with a gold color.",
                                                    "It tastes bitter...^000000"
                                                ])?;
                                                ctx.next()?;
                                                ctx.mes("^3355FFThe nasty taste lingers in your mouth, but then you feel a great warmth flowing throughout your body. Eventually, you pass out.^000000")?;
                                                ctx.next()?;
                                                {
                                                    if ctx.var("BaseLevel").get()?.number()? < 56 {
                                                        ctx.call(Function::GetExperience, vec![Val::from(8909), Val::from(0)])?;
                                                    } else {
                                                        if (ctx.var("BaseLevel").get()?.number()? > 55
                                                            && ctx.var("BaseLevel").get()?.number()? < 61)
                                                        {
                                                            ctx.call(Function::GetExperience, vec![Val::from(10213), Val::from(0)])?;
                                                        } else {
                                                            if (ctx.var("BaseLevel").get()?.number()? > 60
                                                                && ctx.var("BaseLevel").get()?.number()? < 66)
                                                            {
                                                                ctx.call(Function::GetExperience, vec![Val::from(17684), Val::from(0)])?;
                                                            } else {
                                                                if (ctx.var("BaseLevel").get()?.number()? > 65
                                                                    && ctx.var("BaseLevel").get()?.number()? < 71)
                                                                {
                                                                    ctx.call(
                                                                        Function::GetExperience,
                                                                        vec![Val::from(25411), Val::from(0)],
                                                                    )?;
                                                                } else if (ctx.var("BaseLevel").get()?.number()? > 70
                                                                    && ctx.var("BaseLevel").get()?.number()? < 76)
                                                                {
                                                                    ctx.call(
                                                                        Function::GetExperience,
                                                                        vec![Val::from(68757), Val::from(0)],
                                                                    )?;
                                                                } else if (ctx.var("BaseLevel").get()?.number()? > 75
                                                                    && ctx.var("BaseLevel").get()?.number()? < 81)
                                                                {
                                                                    ctx.call(
                                                                        Function::GetExperience,
                                                                        vec![Val::from(128246), Val::from(0)],
                                                                    )?;
                                                                } else if (ctx.var("BaseLevel").get()?.number()? > 80
                                                                    && ctx.var("BaseLevel").get()?.number()? < 86)
                                                                {
                                                                    ctx.call(
                                                                        Function::GetExperience,
                                                                        vec![Val::from(142340), Val::from(0)],
                                                                    )?;
                                                                } else if (ctx.var("BaseLevel").get()?.number()? > 85
                                                                    && ctx.var("BaseLevel").get()?.number()? < 91)
                                                                {
                                                                    ctx.call(
                                                                        Function::GetExperience,
                                                                        vec![Val::from(152052), Val::from(0)],
                                                                    )?;
                                                                } else {
                                                                    ctx.call(
                                                                        Function::GetExperience,
                                                                        vec![Val::from(366786), Val::from(0)],
                                                                    )?;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(270), Val::from(136)])?;
                                                return Err(Stop::End);
                                            } else if (ctx.var("ch_par").get()?.number()? > 9 && ctx.var("ch_poison").get()? == 14) {
                                                ctx.lines_as("Song Zhi Du", args!["......................"])?;
                                                ctx.next()?;
                                                ctx.mes("^3355FFSong Zhi Du looks blankly at the ceiling. With his revenge, it seems he's lost his motivation in life. Was it a good idea to help him, after all?^000000")?;
                                                ctx.next()?;
                                                ctx.mes("^3355FFBy hearsay, the poisoned drink didn't work so well, as a Thief from a foreign land actually stole the bottle.^000000")?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ((ctx.var("ch_par").get()?.number()? > 9 && ctx.var("ch_poison").get()? == 15)
                                                && ctx.var("ch_poison").get()?.number()? < 20)
                                            {
                                                ctx.lines_as(
                                                    "Song Zhi Du",
                                                    args!["What happened?", "Why isn't", "Lord Bai Long dead yet?!"],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Song Zhi Du",
                                                    args!["Hmmm...", "You don't have", "a different plot", "in mind, do you?"],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if (ctx.var("ch_par").get()?.number()? > 9 && ctx.var("ch_poison").get()? == 20) {
                                                ctx.lines_as(
                                                    "Song Zhi Du",
                                                    args!["Ah...", "Welcome.", "My master told", "me everything."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Song Zhi Du", args!["Somehow, I feel relieved, but sorry at the same time. Still, now I can forget everything that's happened in the past."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Song Zhi Du", args!["Now, I've decided to focus more on my medical studies so that I can really save as many lives as I can. I'm sorry I've been so rude to you before."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Song Zhi Du",
                                                    args!["Also, I hope you will", "take these, since I don't", "need them any longer."],
                                                )?;
                                                ctx.var("ch_poison").set(Val::from(21))?;
                                                ctx.call(Function::CompleteQuest, vec![Val::from(11083)])?;
                                                ctx.call(Function::GetItem, vec![Val::from(678), Val::from(5)])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Song Zhi Du",
                                                    args![
                                                        "Thank you",
                                                        "once again, friend.",
                                                        "Now, if you'll excuse me,",
                                                        "I have many things to do..."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ctx.var("ch_poison").get()?.number()? > 20 {
                                                ctx.mes("^3355FFSong Zhi Du is busily engaged with organizing medicinal herbs. The faint look of sadness that used to be on his face now seems drained.^000000")?;
                                                ctx.next()?;
                                                ctx.mes("^3355FFStill, from his movements and the pace at which he is working, you can tell that he loves what he is doing. It seems that Song Zhi Du has finally found his life's path.^000000")?;
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
    ctx.lines_as("Song Zhi Du", args!["Nagash Arses is also the name of poison king in a legend of Arcturus. There's good reason for my master to be proud of his name!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn employee_poison(ctx: &Ctx) -> Script {
    employee_poison_body(ctx, Vec::new()).map(|_| ())
}

fn lady_delivery_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FF * Wait a minute! *",
            "Currently, you're carrying too many items. Please put some of your things into Kafra Storage and try again.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ch_poison").get()?.number()? < 8 {
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? > 5 {
            ctx.lines_as(
                "Lady",
                args!["You're at the", "entrance of Luoyang.", "I hope you enjoy", "your stay here~"],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CHUP")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Lady", args!["You're at the entrance of Luoyang."])?;
        ctx.next()?;
        ctx.lines_as(
            "Lady",
            args![
                "Luoyang is well known",
                "for its various firecrackers.",
                "Would you like to see one?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Sure!:No thanks~")])?) == 1 {
            ctx.lines_as("Lady", args!["Alright, there you go!"])?;
            if ctx.var("Zeny").get()?.number()? > 99 {
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(100))?))?;
            }
            ctx.close_window()?;
            ctx.call(
                Function::NpcSpecialEffect,
                vec![ctx.constant("EF_BLASTMINEBOMB")?, ctx.constant("AREA")?, Val::from(" #fire")],
            )?;
            return Err(Stop::End);
        }
        ctx.lines_as("Lady", args!["Hmpf...!"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ch_poison").get()? == 8 {
        ctx.lines_as("Lady", args!["Would you", "like to see some", "firecrackers?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Sure!:No thanks~")])?) == 1 {
            ctx.lines_as("Lady", args!["Alright,", "there you go!"])?;
            if ctx.var("Zeny").get()?.number()? > 99 {
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(100))?))?;
            }
            ctx.close_window()?;
            ctx.call(Function::DoNpcEvent, vec![Val::from(" #fire::OnClaymore")])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Lady",
            args![
                "Hmm, if not firecrackers,",
                "then you must want something",
                "else. Did someone send you?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 1 {
            ctx.lines_as("Lady", args!["Who was it?", "Please tell me", "his name."])?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_input_s = input;
            if l_input_s.clone() == "Song Zhi Du" {
                ctx.lines_as("Lady", args!["Oh, I see.", "Let me give", "you the package."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Lady",
                    args!["Ah, wait! I forgot!", "You must first pay the", "delivery fee of 1,000 zeny."],
                )?;
                ctx.next()?;
                if ctx.var("Zeny").get()?.number()? > 999 {
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1000))?))?;
                    ctx.call(Function::GetItem, vec![Val::from(7126), Val::from(1)])?;
                    ctx.lines_as(
                        "Lady",
                        args![
                            "Thank you~",
                            "Now that the fee is paid,",
                            "please bring this to",
                            "Song Zhi Du.",
                            "Take care!"
                        ],
                    )?;
                    ctx.var("ch_poison").set(Val::from(9))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(11073), Val::from(11074)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Lady",
                    args![
                        "Errr...",
                        "You don't have enough zeny for the fee. I can't give you the package unless the delivery fee is paid!"
                    ],
                )?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Lady",
                args![
                    "Hmmm?",
                    "I don't know who that is.",
                    "So I definitely don't have",
                    "a package for whoever you're",
                    "talking about."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Lady", args!["No...?"])?;
        ctx.next()?;
        ctx.lines_as("Lady", args!["Are you...", "Coming on to me?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ch_poison").get()?.number()? > 8 {
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? > 5 {
            ctx.lines_as(
                "Lady",
                args!["You're at the", "entrance of Luoyang.", "I hope you have a good time~"],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CHUP")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Lady", args!["You're at the", "entrance of", "Luoyang."])?;
        ctx.next()?;
        ctx.lines_as(
            "Lady",
            args![
                "Luoyang is well known",
                "for its various firecrackers.",
                "Would you like to see one?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Sure!:No thanks~")])?) == 1 {
            ctx.lines_as("Lady", args!["Alright~", "There you go!"])?;
            if ctx.var("Zeny").get()?.number()? > 99 {
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(100))?))?;
            }
            ctx.close_window()?;
            ctx.call(Function::DoNpcEvent, vec![Val::from(" #fire::OnClaymore")])?;
            return Err(Stop::End);
        }
        ctx.lines_as("Lady", args!["Hmpf...!"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn lady_delivery(ctx: &Ctx) -> Script {
    lady_delivery_body(ctx, Vec::new()).map(|_| ())
}

fn fire_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn fire(ctx: &Ctx) -> Script {
    fire_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TrapLouIn1Step {
    Start,
    OnTouch,
}

fn trap_lou_in1_run(ctx: &Ctx, mut step: TrapLouIn1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TrapLouIn1Step::Start => {
                step = TrapLouIn1Step::OnTouch;
                continue 'machine;
            }
            TrapLouIn1Step::OnTouch => {
                if (ctx.var("ch_poison").get()? == 12 || ctx.var("ch_poison").get()? == 16) {
                    ctx.lines_as("Soldier", args!["Who goes there!"])?;
                    ctx.next()?;
                    ctx.mes("^3355FFYou ran away as quickly as you could!^000000")?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(218), Val::from(246)])?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn trap_lou_in1(ctx: &Ctx) -> Script {
    trap_lou_in1_run(ctx, TrapLouIn1Step::Start, Vec::new()).map(|_| ())
}

pub fn trap_lou_in1_ontouch(ctx: &Ctx) -> Script {
    trap_lou_in1_run(ctx, TrapLouIn1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TrapLouIn2Step {
    Start,
    OnTouch,
}

fn trap_lou_in2_run(ctx: &Ctx, mut step: TrapLouIn2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TrapLouIn2Step::Start => {
                step = TrapLouIn2Step::OnTouch;
                continue 'machine;
            }
            TrapLouIn2Step::OnTouch => {
                if (ctx.var("ch_poison").get()? == 12 || ctx.var("ch_poison").get()? == 16) {
                    ctx.lines_as("Soldier", args!["Huh...?", "What was", "that noise?"])?;
                    ctx.next()?;
                    ctx.mes("^3355FFYou bolted away from the castle as fast as your legs could carry you.^000000")?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(218), Val::from(246)])?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn trap_lou_in2(ctx: &Ctx) -> Script {
    trap_lou_in2_run(ctx, TrapLouIn2Step::Start, Vec::new()).map(|_| ())
}

pub fn trap_lou_in2_ontouch(ctx: &Ctx) -> Script {
    trap_lou_in2_run(ctx, TrapLouIn2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TrapLouIn3Step {
    Start,
    OnTouch,
}

fn trap_lou_in3_run(ctx: &Ctx, mut step: TrapLouIn3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TrapLouIn3Step::Start => {
                step = TrapLouIn3Step::OnTouch;
                continue 'machine;
            }
            TrapLouIn3Step::OnTouch => {
                if (ctx.var("ch_poison").get()? == 12 || ctx.var("ch_poison").get()? == 16) {
                    ctx.lines_as("Soldier", args!["Huh...?", "Is somebody there?"])?;
                    ctx.next()?;
                    ctx.mes("^3355FFYou escaped from the soldier's suspicious gaze.^000000")?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(218), Val::from(246)])?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn trap_lou_in3(ctx: &Ctx) -> Script {
    trap_lou_in3_run(ctx, TrapLouIn3Step::Start, Vec::new()).map(|_| ())
}

pub fn trap_lou_in3_ontouch(ctx: &Ctx) -> Script {
    trap_lou_in3_run(ctx, TrapLouIn3Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TrapLouIn4Step {
    Start,
    OnTouch,
}

fn trap_lou_in4_run(ctx: &Ctx, mut step: TrapLouIn4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TrapLouIn4Step::Start => {
                step = TrapLouIn4Step::OnTouch;
                continue 'machine;
            }
            TrapLouIn4Step::OnTouch => {
                if (ctx.var("ch_poison").get()? == 12 || ctx.var("ch_poison").get()? == 16) {
                    ctx.lines_as("Soldier", args!["Hold it right there, Midgardian!"])?;
                    ctx.next()?;
                    ctx.mes("^3355FFYou run as fast as you can before the soldiers can catch you.^000000")?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(218), Val::from(246)])?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn trap_lou_in4(ctx: &Ctx) -> Script {
    trap_lou_in4_run(ctx, TrapLouIn4Step::Start, Vec::new()).map(|_| ())
}

pub fn trap_lou_in4_ontouch(ctx: &Ctx) -> Script {
    trap_lou_in4_run(ctx, TrapLouIn4Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TrapLouIn5Step {
    Start,
    OnTouch,
}

fn trap_lou_in5_run(ctx: &Ctx, mut step: TrapLouIn5Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TrapLouIn5Step::Start => {
                step = TrapLouIn5Step::OnTouch;
                continue 'machine;
            }
            TrapLouIn5Step::OnTouch => {
                if (ctx.var("ch_poison").get()? == 12 || ctx.var("ch_poison").get()? == 16) {
                    ctx.lines_as("Soldier", args!["Huh...?", "I hear something!"])?;
                    ctx.next()?;
                    ctx.mes("^3355FFYou escape before the soldier can find you.^000000")?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(218), Val::from(246)])?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn trap_lou_in5(ctx: &Ctx) -> Script {
    trap_lou_in5_run(ctx, TrapLouIn5Step::Start, Vec::new()).map(|_| ())
}

pub fn trap_lou_in5_ontouch(ctx: &Ctx) -> Script {
    trap_lou_in5_run(ctx, TrapLouIn5Step::OnTouch, Vec::new()).map(|_| ())
}

fn lou_path_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("ch_poison").get()? == 12 || ctx.var("ch_poison").get()? == 16) {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["What's this?", "A crack in the wall?"],
        )?;
        ctx.next()?;
        ctx.mes("^3355FFYou jump to the stone wall and peep into the crack. It looks big enough for someone to squeeze through.^000000")?;
        ctx.next()?;
        ctx.mes("^3355FFYou move through the crack as quickly as you can.^000000")?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("lou_in01"), Val::from(119), Val::from(167)])?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn lou_path(ctx: &Ctx) -> Script {
    lou_path_body(ctx, Vec::new()).map(|_| ())
}

fn lou_drink1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ch_poison").get()? == 12 {
        ctx.mes("^3355FFYou found a drink bottle that's possibly owned by Bai Long, lord of Luoyang.^00000")?;
        ctx.next()?;
        ctx.mes("^3355FFYou put the deadly poison into the bottle.^000000")?;
        ctx.call(Function::DelItem, vec![Val::from(678), Val::from(1)])?;
        ctx.var("ch_poison").set(Val::from(13))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(11076), Val::from(11077)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ch_poison").get()? == 13 {
        ctx.mes("^3355FFIt would be smart to get out of this place as soon as you can.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ch_poison").get()? == 16 {
        if (ctx.call(Function::CountItem, vec![Val::from(938)])?.number()? > 0
            && ctx.call(Function::CountItem, vec![Val::from(713)])?.number()? > 0)
        {
            ctx.mes("^3355FFYou take the drink bottle and replace it with a bottle filled with Sticky Mucus. Hopefully Bai Long won't notice!^000000")?;
            ctx.call(Function::DelItem, vec![Val::from(938), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(713), Val::from(1)])?;
            ctx.var("ch_poison").set(Val::from(17))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(11079), Val::from(11080)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.mes("^3355FFIt seems that you need switch Bai Long's drinking bottle with something else. Just taking the drink bottle would arouse suspicion.^000000")?;
        ctx.next()?;
        ctx.mes("^3355FFMaybe if you found some kind of empty bottle and filled it with something, like Sticky Mucus...^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("^3355FFYou found a drink bottle. It's fancy enough that the only possible owner could be Bai Long, lord of Luoyang.^000000")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn lou_drink1(ctx: &Ctx) -> Script {
    lou_drink1_body(ctx, Vec::new()).map(|_| ())
}

fn lou_drink2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ch_poison").get()? == 12 {
        ctx.mes("^3355FFYou squeeze through the crack as quickly as you can.^000000")?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(217), Val::from(278)])?;
        return Err(Stop::End);
    } else if ctx.var("ch_poison").get()? == 13 {
        ctx.mes("^3355FFYou squeeze through the crack as quickly as you can.^000000")?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(217), Val::from(278)])?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn lou_drink2(ctx: &Ctx) -> Script {
    lou_drink2_body(ctx, Vec::new()).map(|_| ())
}

fn lord_bailong_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FF * Wait a minute! *",
            "Right now you're carrying too many items. Please put some of your things into Kafra Storage and try again.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ch_poison").get()? == 12 || ctx.var("ch_poison").get()? == 16) {
        ctx.lines_as(
            "Lord Bai Long",
            args!["Hey...!", "You're not doing", "anything suspicious", "are you?"],
        )?;
        ctx.next()?;
        ctx.lines(args!["^3355FFYou run away", "from Bai Long as", "fast as you can!^000000"])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(218), Val::from(246)])?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Lord Bai Long", args!["Hahahaha!", "Welcome to Luoyang!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Lord Bai Long",
            args![
                "Luoyang is such a great city.",
                "We've protected this land for forty years from the invasions of evil creatures!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Lord Bai Long", args!["I hope you enjoy", "your stay here,", "Midgardian!"])?;
        ctx.next()?;
        if ctx.var("ch_poison").get()? == 18 {
            ctx.lines(args![
                "^3355FFYou give Nagash Arses' Journal",
                "to Bai Long. He reads it intently, slowly turning each page.^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFHe's completely silent for about fifteen minutes, and focuses all",
                "of his attention of the journal.^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFA single tear trickles",
                "from his eye as he reads",
                "the words of Nagash Arse...^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as("Lord Bai Long", args!["Thank you..", "Adventurer..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Lord Bai Long",
                args!["Thank you so much.", "You've helped me make", "up with an old friend!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lord Bai Long",
                args![
                    "I, Bai Long, will work on giving",
                    "fair treatment to all of martial arts organizations, regardless of their methods or philosophies!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lord Bai Long",
                args![
                    "I will also forgive Song Zhi Du, even if he tried to poison me.",
                    "He is innocent..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lord Bai Long",
                args![
                    "As a matter of fact, it was reported to me that Nagash was deported to his homeland after",
                    "that incident. I had no idea",
                    "he was in jail!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Lord Bai Long", args!["Thank you so", "much for your help!"])?;
            ctx.next()?;
            ctx.next()?;
            ctx.lines_as(
                "Lord Bai Long",
                args!["Please, take this as a token of gratitute. And please deliver this letter to Nagash and Song Zhi Du for me."],
            )?;
            ctx.var("ch_poison").set(Val::from(19))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(11081), Val::from(11802)])?;
            ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
            ctx.next()?;
            ctx.mes("^3355FFYou obtained ^0000FFBai Long's letter^000000.")?;
            ctx.next()?;
            ctx.lines_as("Lord Bai Long", args!["Thank you in advance."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Lord Bai Long",
            args![
                "How could I have lost my friend from our glory days? Hmm...?",
                "Oh, I didn't realize you were still there... Listening to me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lord Bai Long",
            args!["I'm sorry, I was just talking to myself. I hope you have a good time while staying here in Luoyang."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn lord_bailong(ctx: &Ctx) -> Script {
    lord_bailong_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_bailong1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("ch_poison").get()? == 12 || ctx.var("ch_poison").get()? == 16) {
        ctx.lines_as("Soldier", args!["Hey...", "There's something", "fishy about you...! "])?;
        ctx.next()?;
        ctx.mes("^3355FFYou escaped from the castle, far away from the suspicious guard...^000000")?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(218), Val::from(246)])?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Soldier",
        args![
            "^666666*Yawn*^000000 It's so boring here nowadays. Although, I hear that",
            "a long time ago, things were",
            "much different."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Soldier", args!["Supposedly, monsters used to intrude Luoyang all the time! Heh, but that's just a rumor. Still, there are a bunch of rumors going around that I'm curious about..."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_bailong1(ctx: &Ctx) -> Script {
    soldier_bailong1_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_bailong2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("ch_poison").get()? == 12 || ctx.var("ch_poison").get()? == 16) {
        ctx.lines_as("Soldier", args!["Huh...?", "Is somebody there? "])?;
        ctx.next()?;
        ctx.lines(args!["^3355FFYou ran away", "as quickly", "as you could.^000000"])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(218), Val::from(246)])?;
        return Err(Stop::End);
    }
    ctx.lines_as("Soldier", args![".....z...Z....z..."])?;
    ctx.next()?;
    let choice = runtime::select_values(ctx, &[Val::from("Hey, wake up!")])?;
    ctx.var("@menu").set(choice)?;
    ctx.lines_as("Soldier", args!["Yeeeesss...Sir!", "I was not dozing off~ seerious...ly..."])?;
    ctx.next()?;
    ctx.lines_as("Soldier", args!["..."])?;
    ctx.next()?;
    ctx.lines_as("Soldier", args!["...", "......"])?;
    ctx.next()?;
    ctx.lines_as("Soldier", args![".....z...Z....z..."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_bailong2(ctx: &Ctx) -> Script {
    soldier_bailong2_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_bailong3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("ch_poison").get()? == 12 || ctx.var("ch_poison").get()? == 16) {
        ctx.lines_as("Soldier", args!["Huh...?", "What was that?"])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou escape the soldier's",
            "suspicious gaze as quickly",
            "as you can.^000000"
        ])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(218), Val::from(246)])?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Soldier",
        args!["People have been", "saying that there's", "a scammer at the", "entrance of town..."],
    )?;
    ctx.next()?;
    let choice = runtime::select_values(ctx, &[Val::from("...?")])?;
    ctx.var("@menu").set(choice)?;
    ctx.lines_as(
        "Soldier",
        args![
            "But I went there and I didn't",
            "find anyone suspicious. I swear, everything seemed completely normal!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_bailong3(ctx: &Ctx) -> Script {
    soldier_bailong3_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_bailong4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("ch_poison").get()? == 12 || ctx.var("ch_poison").get()? == 16) {
        ctx.lines_as("Soldier", args!["Who goes there?!"])?;
        ctx.next()?;
        ctx.lines(args!["^3355FFYou ran away", "as quickly", "as you could.^000000"])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(218), Val::from(246)])?;
        return Err(Stop::End);
    }
    ctx.lines_as("Soldier", args!["Ah, don't bother", "to listen to this", "guy beside of me."])?;
    ctx.next()?;
    ctx.lines_as(
        "Soldier",
        args!["I hope you have a good time while you're staying here in Luoyang!"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_bailong4(ctx: &Ctx) -> Script {
    soldier_bailong4_body(ctx, Vec::new()).map(|_| ())
}

fn hermit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !(ctx.var("ql_revol").get()?.is_true()) {
        ctx.lines_as(
            "Sun Mao",
            args![
                "Where there's a will,",
                "there's a way. When we",
                "work together, there's",
                "nothing we cannot do."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sun Mao",
            args![
                "Will you lend us your power, and help us build a better future? Will you join our movement for social reform in Luoyang?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Sun Mao", args!["It doesn't matter if you're a local or a foreigner. So long as we share the same goal of creating a better Luoyang, where everyone is treated as an equal."])?;
        ctx.next()?;
        ctx.lines_as(
            "Sun Mao",
            args![
                "My name...",
                "is Sun Mao.",
                "This invitation may be sudden, but I am a man of action. I see in your face that you too despise injustice."
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from(
                    "What am I supposed to do?:I will join you!:Well, I'm just another tourist...",
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
                ctx.lines_as("Sun Mao", args!["For now, we must recruit as many members as we can. When we're ready, we'll start a movement that will change Luoyang, and even the world!"])?;
                ctx.next()?;
                ctx.lines_as("Sun Mao", args!["However, we currently do not have the manpower to carry out the plans we have laid out. We've been lucky the lord hasn't found us out yet."])?;
                ctx.next()?;
                ctx.lines_as("Sun Mao", args!["I wish everyone would understand our intentions, but the people of this city are disunited. This is not the first time I have approached a foreigner for help..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Sun Mao",
                    args![
                        "I suppose you understand the urgency of our cause, and our need to gather others who see the merit of our goals!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Sun Mao",
                    args!["Oh, God must be on my side today and has sent you to me. Thank you for making the big decision..."],
                )?;
                ctx.next()?;
                ctx.lines_as("Sun Mao", args!["I shall then engrave your name", "on this bloody pledge board. But before I do so, I shall ask once more. Do you truly wish to join us, through pain and bloodshed?"])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("No, wait!:I am 100% sure.")])? {
                    1 => {
                        ctx.lines_as("Sun Mao", args!["I understand if you need time to decide. If you do decide to join us, please return. Time is on our side, after all."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.var("ql_revol").set(Val::from(1))?;
                        ctx.lines_as("Sun Mao", args![(ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from("...!")), "Your name is now engraved on this bloody pledge board. We will fight together to the death for Luoyang's future!"])?;
                        ctx.next()?;
                        runtime::party_members(ctx, ctx.call(Function::GetCharacterId, vec![Val::from(1)])?, Val::from(0))?;
                        ctx.var("@partymember").set(ctx.var("$@partymembercount").get()?)?;
                        if (ctx
                            .call(
                                Function::IsPartyLeader,
                                vec![ctx.call(Function::GetCharacterId, vec![Val::from(1)])?],
                            )?
                            .loosely_equals(&Val::from(1))
                            || !(ctx.var("@partymember").get()?.is_true()))
                        {
                            ctx.lines_as("Sun Mao", args!["Now, the most important thing for our cause is to gather more recruits and increase our numbers. Please find others who will join us in our fight."])?;
                            ctx.next()?;
                            ctx.lines_as("Sun Mao", args!["However, as to not arouse suspicion, bring only one new recruit at a time. Also, we suggest not to bring a friend in a party with you."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Sun Mao",
                                args!["Now, I will wait for you here. Go forth and find others who wish for a better future for Luoyang!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Sun Mao", args!["Please assist the people who fight behind you. As more are gathered, one of our comrades will inform you of further instructions."])?;
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
                ctx.lines_as(
                    "Sun Mao",
                    args!["Are you...? I see, you may leave now. Do not reveal what you have seen or heard in this place."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sun Mao",
                    args!["If you're a real tourist, you do not want to get involved in our business, one way or another."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if ctx
        .call(
            Function::IsPartyLeader,
            vec![ctx.call(Function::GetCharacterId, vec![Val::from(1)])?],
        )?
        .loosely_equals(&Val::from(1))
    {
        if ctx.var("ql_revol").get()? == 9 {
            ctx.lines_as(
                "Sun Mao",
                args!["Once again,", "thank you for", "your trouble.", "Go back safe."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        runtime::party_members(ctx, ctx.call(Function::GetCharacterId, vec![Val::from(1)])?, Val::from(0))?;
        ctx.var("@partymember").set(ctx.var("$@partymembercount").get()?)?;
        if ctx.var("ql_revol").get()?.number()? < 8 {
            if ctx
                .var("@partymember")
                .get()?
                .loosely_equals(&(ctx.var("ql_revol").get()? + Val::from(1)))
            {
                if ctx.var("@partymember").get()? != 8 {
                    ctx.var("ql_revol").set((ctx.var("ql_revol").get()? + Val::from(1)))?;
                    ctx.lines_as(
                        "Sun Mao",
                        args![
                            "Oh, you brought a new comrade! Welcome. Please help your friend understand our intentions before joining us."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("[Sun Mao]")?;
                    let subject3 = ctx.var("ql_revol").get()?;
                    if subject3 == 2 {
                        ctx.mes(
                            "Please bring another friend that both of you can trust. Remember, add one person at a time into your party.",
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Sun Mao",
                            args!["Otherwise, other people may learn of what we are doing, and that will lead to trouble."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if subject3 == 3 {
                        ctx.mes("The three of you may all move together for our cause. Please try to find another recruit to be your fourth member, invite him to your party and bring him to this place.")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if subject3 == 4 {
                        ctx.mes("Ah, now there are four of you. However, we must continue to strengthen our ranks at a cautious pace. Please go and find one new recruit, and only one, to add to your party.")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if subject3 == 5 {
                        ctx.mes("Now, there are five of you. Needless to say, we still require more manpower. Please go out and seek another new member for our cause. Remember, only add one more person to your party and return to me.")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if subject3 == 6 {
                        ctx.mes("There are now six of you. So far, so good. Please go forth and find just one more person to add to your party and bring him to me so that we may recruit him.")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if subject3 == 7 {
                        ctx.mes("Now, there are seven of you, but we still lack the manpower we need. Please go forth and seek out one, and only one, more person to add to your party and then return to me.")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                ctx.lines_as("Sun Mao", args!["Great work, you've brought another new comrade! Welcome! Please explain our goals and the righteousness of our cause to our new friend so that he may fully join us."])?;
                ctx.next()?;
                if ctx.var("BaseLevel").get()?.number()? < 50 {
                    ctx.lines_as("Sun Mao", args!["It is almost time to take the action..."])?;
                    ctx.next()?;
                    ctx.lines_as("Sun Mao", args!["I recognize your pledge to fight for us, but I fear that you are not yet ready for the task I have available for you."])?;
                    ctx.next()?;
                    ctx.lines_as("Sun Mao", args!["However, we still have time. Go out and train with the party members you have gathered to gain more experience."])?;
                    ctx.next()?;
                    ctx.lines_as("Sun Mao", args!["Together, all of you will be able to train faster than if you were to go train alone. Come back when you feel that you are ready for this task."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Sun Mao", args!["At long last, there are eight of you. Now, I believe there is among manpower amongst you to carry out this mission. Please listen carefully."])?;
                ctx.next()?;
                ctx.lines_as("Sun Mao", args!["The martial arts organizations of Luoyang, as you may well know, have been divided between those that fight for justice, and those that serve evil purposes."])?;
                ctx.next()?;
                ctx.lines_as("Sun Mao", args!["Recently, some of the evil organizations have conspired with corrupt government officials in the interest to expand their power."])?;
                ctx.next()?;
                ctx.lines_as("Sun Mao", args!["These corrupt government officials have been suspicious of our gunpowder research and arrested our comrade, ^3355FFHao Chenryu^000000, our gunpowder expert, under the pretext of investigation."])?;
                ctx.next()?;
                ctx.lines_as("Sun Mao", args!["Without him, we cannot continue the gunpowder research, or produce the gunpowder we'll need for our movement. We must contact him so that he may make gunpowder for us."])?;
                ctx.next()?;
                ctx.lines_as("Sun Mao", args!["Unfortunately, many of us cannot contact him, as our identities are known to the officials. However, since you are new recruits, as well as Midgardians, they will not know how you are."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Sun Mao",
                    args![
                        "Your mission will be to deliver these chemicals to our comrade Hao Chenryu who is held captive by the government."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Sun Mao", args!["Since flammable chemicals are not allowed inside government offices, you must keep them well hidden. If the chemicals are equally divided among the eight of you, we might succeed!"])?;
                ctx.next()?;
                ctx.var("ql_revol").set(Val::from(8))?;
                ctx.call(Function::GetItem, vec![Val::from(7068), Val::from(8)])?;
                ctx.call(Function::GetItem, vec![Val::from(7096), Val::from(8)])?;
                ctx.call(Function::GetItem, vec![Val::from(7004), Val::from(8)])?;
                ctx.lines_as("Sun Mao", args!["There you go. Now, divide these materials equally among the eight of you. The eight of you must fully cooperate with each other, or our plan will fail. Understood?"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if runtime::op(
                &ctx.var("@partymember").get()?,
                "<",
                &(ctx.var("ql_revol").get()? + Val::from(1)),
            )?
            .is_true()
            {
                ctx.mes("[Sun Mao]")?;
                let subject4 = ctx.var("ql_revol").get()?;
                if subject4 == 1 {
                    ctx.mes("You made a party. Now, why don't you go recruit more followers? For now, just add one more person, lest we arouse the suspicion of our enemies.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject4 == 2 {
                    ctx.lines(args!["Hmmm...", "Having trouble finding a third person to add to your party? It's important that you find someone that is trustworthy. Be careful and don't get caught!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject4 == 3 {
                    ctx.lines(args!["Hmmm...", "Having trouble finding a forth person to add to your party? I understand that it's important that you find someone you can trust. Be careful and don't get caught!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject4 == 4 {
                    ctx.mes("Having trouble finding a fifth person to add to your party? I understand that it's important that you find someone you can trust. Be careful and don't get caught!")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject4 == 5 {
                    ctx.mes("Having trouble finding a six person to add to your party? It's important that you find someone that is trustworthy. Be careful and don't get caught!")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject4 == 6 {
                    ctx.mes("Having trouble finding a seventh person to add to your party? It's important that you find someone that is trustworthy. Be careful and don't get caught!")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject4 == 7 {
                    ctx.lines(args!["Hmmm...", "Having trouble finding an eighth person to add to your party? It's important that you find someone that is trustworthy. Be careful and don't get caught!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                ctx.lines_as("Sun Mao", args!["Oh no! You've brought more than one more person to join us! We can't do any recruiting now, it will bring unnecessary attention to our activities!"])?;
                ctx.next()?;
                ctx.mes("[Sun Mao]")?;
                let subject5 = ctx.var("ql_revol").get()?;
                if subject5 == 1 {
                    ctx.mes("Please make sure that there is a total of two members in your party so that we can recruit your friend. I believe that will look natural.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject5 == 2 {
                    ctx.mes("At this stage in our plan, we cannot afford any unnecessary attention to our activities! Please make sure that there is a total of three members in your party so that we can recruit your friend.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject5 == 3 {
                    ctx.mes("Please make sure that there is a total of four members in your party so that we can recruit your friend. I believe that will look natural.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject5 == 4 {
                    ctx.mes("Please make sure that there is a total of five members in your party so that we can recruit your friend. I believe that will look natural.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject5 == 5 || subject5 == 6 {
                    ctx.mes("By this time, you should know that you should only bring one more person at a time. You know that it is crucial that we do not attract any undue attention before we can take action!")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject5 == 7 {
                    ctx.mes("At this stage in our plan, we cannot afford any unnecessary attention to our activities! Please make sure that there is a total of eight members in your party so that we can recruit your friend.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        } else if ctx.var("ql_revol").get()? == 8 {
            if ctx.var("@partymember").get()? == 8 {
                if ctx.call(Function::CountItem, vec![Val::from(7204)])?.number()? > 7 {
                    ctx.lines_as(
                        "Sun Mao",
                        args!["Welcome back! I see that the mission has been successfully accomplished! Great work, men!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Sun Mao", args!["Finally, we have a weapon to mete out severe retribution to our enemies, the corrupt government officials and the evil martial arts organizations! I appreciate your help, comrades."])?;
                    ctx.next()?;
                    ctx.call(
                        Function::DelItem,
                        vec![Val::from(7204), ctx.call(Function::CountItem, vec![Val::from(7204)])?],
                    )?;
                    ctx.var("ch_make").set(Val::from(0))?;
                    ctx.var("ql_revol").set(Val::from(9))?;
                    ctx.call(Function::GetItem, vec![Val::from(668), Val::from(8)])?;
                    ctx.lines_as("Sun Mao", args!["Please take these funds and share them with your party members. I hope that all of you will lend your power to our cause once again."])?;
                    ctx.next()?;
                    ctx.lines_as("Sun Mao", args!["Once again, I thank you for your help. Now, go in safety."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Sun Mao",
                    args![
                        "Your mission is to take the materials I have given you and your party members, and to smuggle them to Hao Chenryu."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Sun Mao", args!["Hao Chenryu is currently held captive by corrupt members of the government. Remember to divide the materials equally among your party members so that the officials do find them."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Sun Mao",
                args!["Huh, where are all your members? Come back to me with all of your recruits."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sun Mao",
                args!["Remember that you move together with your comrades. There isn't a traitor amongst you, is there?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("ch_make").get()? == 1 {
            ctx.var("ch_make").set(Val::from(0))?;
            ctx.lines_as("Sun Mao", args!["Ah...", "You have", "returned!", "Ha ha ha ha!"])?;
            ctx.next()?;
            ctx.lines_as("Sun Mao", args!["I would like to commend you and your comrades for your excellent performance in the last mission. Come back anytime if you wish to be assigned to another task for our cause."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Sun Mao", args!["Remember that the authoritative center of the party is its leader. The party members must act as one in order to accomplish their goals."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn hermit(ctx: &Ctx) -> Script {
    hermit_body(ctx, Vec::new()).map(|_| ())
}
