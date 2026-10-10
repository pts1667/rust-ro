use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn magatu_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = ctx.var("dmdswrd_q").get()?;
    if subject1 == 8 {
        ctx.lines_as(
            "Magatu",
            args![
                "Comodo Cheese...",
                "It really exists...!",
                "And I'm so close to",
                "having the proof! Oh...",
                "That look in your eyes...",
                "Y-you really believe me!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Magatu",
            args![
                "At long last, not only",
                "someone who believes me,",
                "but an adventurer to boot!",
                "Great, this is perfect! Now,",
                "listen, you want to learn more",
                "about Comodo Cheese, right?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Magatu",
            args![
                "Well, I happen to know",
                "someone who knows someone",
                "that might be able to give you",
                "the chance to try it for yourself! His name is ^3355FFManzi^000000 , and you can",
                "find him in Comodo's Casino."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Magatu",
            args![
                "Oh, here, before I forget,",
                "take my lucky bottle cap!",
                "Show this to Manzi, and he'll",
                "know that I sent you, and that",
                "you want to learn more about",
                "the elusive Comodo Cheese."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou have received",
            "Magatu's lucky bottle",
            "cap to present to Manzi."
        ])?;
        ctx.var("dmdswrd_q").set(Val::from(9))?;
    } else if subject1 == 9 {
        ctx.lines_as(
            "Magatu",
            args![
                "Oh, don't forget to",
                "show my lucky bottle cap",
                "to Manzi, okay? You can find",
                "him inside Comodo's Casino~",
                "Good luck, and I hope you get",
                "to try that Comodo Cheese~"
            ],
        )?;
    } else {
        ctx.lines_as(
            "Magatu",
            args![
                "It's true, it really",
                "exists... Comodo Cheese!",
                "Its flavor must be incomparably",
                "delicious if adventurers have",
                "quested to obtain it for so",
                "many generations..."
            ],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn magatu_cmd(ctx: &Ctx) -> Script {
    magatu_cmd_body(ctx, Vec::new()).map(|_| ())
}

fn manzi_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = ctx.var("dmdswrd_q").get()?;
    if subject1 == 9 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Excuse, but I'm",
                "looking for someone",
                "named Manzi. Do you",
                "know where I can find him?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manzi",
            args![
                "Hey guy, I'm right here.",
                "So what exactly do you",
                "want? I'm, um, not in",
                "trouble or anything, am I?"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou give Manzi the",
            "lucky bottle cap that you",
            "received from Magatsu.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Manzi",
            args![
                "Whoa, Magatsu gave you",
                "this? Ah, okay, so you must",
                "be looking for that Comodo",
                "Cheese he keeps talking about.",
                "Alright, I owe him a favor, so",
                "I'll tell you who to talk to..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manzi",
            args![
                "From the Casino, go north",
                "towards the center of the",
                "village, and then look to",
                "the right where you'll see",
                "the Dance Stage. You'll see",
                "this old woman right there."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manzi",
            args![
                "Don't be fooled by the",
                "way she looks--that old",
                "lady is one of Comodo's",
                "wisest elders. Ask her",
                "about the cheese, and",
                "let her know I sent you~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manzi",
            args![
                "Ah, she won't take you",
                "very seriously unless you",
                "show her this. Magatsu gave",
                "you his lucky bottle cap, so I'm gonna give you my lucky coin!",
                "The old crone'll recognize it~"
            ],
        )?;
        ctx.next()?;
        ctx.var("dmdswrd_q").set(Val::from(10))?;
        ctx.lines(args![
            "^3355FFYou received Manzi's",
            "''lucky coin.'' Strangely",
            "enough, both sides",
            "are heads."
        ])?;
    } else if subject1 == 10 {
        ctx.lines_as(
            "Manzi",
            args![
                "Look for the old crone",
                "near the Dance Stage here",
                "in Comodo and ask her about",
                "Comodo Cheese, alright? Ah,",
                "and try not to insult her, kay?",
                "It'll make me look bad~"
            ],
        )?;
    } else {
        if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.lines_as(
                "Manzi",
                args![
                    "What th...?",
                    "What's a kid like",
                    "you doing in a Casino?",
                    "Sure, it's not against the",
                    "rules, but I think you oughta",
                    "scram and play somewhere else!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Manzi",
            args![
                "Hey, have a good time in",
                "the Casino, but don't go buck",
                "wild. You wanna walk out of",
                "here with the shirt on your back, you know? Some people don't",
                "even leave here with that..."
            ],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn manzi_cmd(ctx: &Ctx) -> Script {
    manzi_cmd_body(ctx, Vec::new()).map(|_| ())
}

fn hullaris_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = ctx.var("dmdswrd_q").get()?;
    if subject1 == 10 {
        ctx.lines_as(
            "Hullaris",
            args![
                "Hula~hula~hula~",
                "Love togther, love together,",
                "we've groovin' on some more~",
                "Love togther, love together,",
                "we've living on the floor~"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Present Muzi's Coin:Um... Comodo Cheese?")],
        )?) == 1
        {
            ctx.lines(args!["^3355FFYou present Muzi's lucky", "coin to the old woman.^000000"])?;
            ctx.next()?;
            ctx.lines_as(
                "Hullaris",
                args![
                    "Love! Love!",
                    "Love together~",
                    "Love! Love!",
                    "Love togeth-hm?",
                    "Oh, that's um, Muzi's",
                    "cheat coin, isn't it?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hullaris",
                args![
                    "He always said that he",
                    "might send someone with",
                    "that coin to me as a sign",
                    "of his trust in that person.",
                    "I suppose, then, that I'm",
                    "at your service. Now..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hullaris",
                args![
                    "What exactly did you need?",
                    "I'm guessing you've come",
                    "here to ask me something",
                    "about Comodo. As one of the",
                    "oldest elders, I know more",
                    "about this village than most..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Actually, I was hoping",
                    "you can tell me about",
                    "Comodo Cheese. If it really",
                    "exists, I'd like to know",
                    "where I can get some."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hullaris",
                args![
                    "Oh... That. First of all,",
                    "Comodo Cheese does exist,",
                    "and it's as precious as the",
                    "legends say. However, it's",
                    "not a true cheese, although you may think so from its taste."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hullaris",
                args![
                    "Yes, it's a very rare, natural",
                    "substance and isn't made from",
                    "cow or goat milk or anything",
                    "like that. However, Comodo",
                    "Cheese isn't merely food.",
                    "No, it's much more..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hullaris",
                args![
                    "Those that eat Comodo",
                    "Cheese find that their",
                    "true potential is unlocked.",
                    "Comodo Cheese's true name",
                    "is the ^3355FFAwakening Stone^000000. Now,",
                    "doesn't that sound impressive?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hullaris",
                args![
                    "The Awakening Stone is",
                    "one of the keys to obtaining",
                    "some kind of forbidden power.",
                    "That's why only the bravest",
                    "adventurers can expect the",
                    "chance of ever eating it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hullaris",
                args![
                    "Do you really wish to",
                    "eat the Comodo Cheese and",
                    "see where its power may lead",
                    "you? If so, you'll have to endure great challenges to obtain it..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hullaris",
                args![
                    "When you've decided",
                    "to pursue the Awakening",
                    "Stone, seek out a man named",
                    "^3355FFNigirboran^000000. He will judge",
                    "whether you are worthy",
                    "of the Comodo Cheese..."
                ],
            )?;
            ctx.var("dmdswrd_q").set(Val::from(11))?;
            ctx.next()?;
            ctx.lines_as(
                "Hullaris",
                args![
                    "Now, you should be",
                    "able to find Nigiroban",
                    "training somewhere in one",
                    "of Comodo's Dungeon Caves.",
                    "I'll send him a message to",
                    "let him know you're coming..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Hullaris",
            args!["Love! Love!", "Love together~", "Love! Love!", "Love togeth-hm?"],
        )?;
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
            ctx.lines(args!["Boy, you're ruining", "my groove! Now beat it!"])?;
        } else {
            ctx.lines(args!["Girl, you're ruining", "my groove! Get away~"])?;
        }
    } else if subject1 == 11 {
        ctx.lines_as(
            "Hullaris",
            args![
                "Do you really wish to",
                "eat the Comodo Cheese and",
                "see where its power may lead",
                "you? If so, you'll have to endure great challenges to obtain it..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hullaris",
            args![
                "When you've decided",
                "to pursue the Awakening",
                "Stone, seek out a man named",
                "^3355FFNigirboran^000000. He will judge",
                "whether you are worthy",
                "of the Comodo Cheese..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hullaris",
            args![
                "Now, you should be",
                "able to find Nigiroban",
                "training somewhere in one",
                "of Comodo's Dungeon Caves.",
                "I'll send him a message to",
                "let him know you're coming..."
            ],
        )?;
    } else if subject1 == 12 {
        ctx.lines_as(
            "Hullaris",
            args![
                "Hmm? So you've failed",
                "Nigirboran's test, have",
                "you? Well, you better train",
                "until you can pass it. Otherwise, eating Comodo Cheese could",
                "mean your death, you know."
            ],
        )?;
    } else if subject1 == 13 {
        ctx.lines_as(
            "Hullaris",
            args![
                "Ah, you've returned.",
                "So were you able to pass",
                "Nigirboran's little test? An",
                "adventurer like you should",
                "be able to have no problem",
                "with it. I've got faith in you~"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou present the token that",
            "signifies that you passed",
            "Nigirboran's test to Hullaris.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Hullaris",
            args![
                "Ah, I was right after all.",
                "I'm glad to see that you've",
                "proven worthy of eating this",
                "Comodo Cheese, or more",
                "accurately, the Awakening",
                "Stone. Here, let me get it..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hullaris",
            args![
                "There you are...",
                "Only brave and worthy",
                "adventurers are allowed to",
                "eat this. Understand that",
                "eating Comodo Cheese is",
                "a rare and coveted honor!"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFHullaris carefully",
            "hands you a plate of",
            "Comodo Cheese. You enjoy",
            "the rich, smooth flavor of",
            "each and every morsel...",
            "It's sublimely delicious!^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFAfter you finish eating the",
            "Comodo Cheese, you feel",
            "a subtle, yet definite energy",
            "gently pulsing through your",
            "body. You feel a powerful, yet",
            "quiet confidence of being able",
            "to accomplish anything.^000000"
        ])?;
        ctx.var("dmdswrd_q").set(Val::from(14))?;
        ctx.next()?;
        ctx.lines_as(
            "Hullaris",
            args![
                "Hahahah! So what",
                "do you think? It's great,",
                "isn't it? It's unreal, how",
                "delicious it is. All other",
                "foods can never match the",
                "quality of Comodo Cheese~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hullaris",
            args![
                "Now, I know that you probably",
                "have things to do, but might",
                "I suggest that you visit the",
                "town of Al De Baran? There's",
                "a man there that can tell you",
                "about the Slate of Muriniel."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hullaris",
            args![
                "You've already eaten the",
                "Awakening Stone, so you may",
                "as well obtain the other things",
                "that you need to earn one of",
                "the three forbidden swords.",
                "It's just a thought..."
            ],
        )?;
    } else {
        ctx.lines_as(
            "Hullaris",
            args![
                "Ah... Dance.",
                "It's more than just a form",
                "of entertainment. It's art,",
                "it's seduction, it's battle,",
                "and it's love. Only a true",
                "Dancer can understand this..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hullaris",
            args![
                "You know, Comodo is famous",
                "for its Dance Academy and the",
                "Dancers that have been trained",
                "there. If you know any female",
                "Archers, why don't you suggest",
                "visiting the school to them?"
            ],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hullaris_cmd(ctx: &Ctx) -> Script {
    hullaris_cmd_body(ctx, Vec::new()).map(|_| ())
}

fn nigirboran_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = ctx.var("dmdswrd_q").get()?;
    if subject1 == 11 {
        ctx.lines_as(
            "Nigirboran",
            args![
                "You're the one that",
                "Hullaris sent? So you're",
                "here to earn the right to",
                "eat the Awakening Stone...",
                "Or Comodo Cheese, as it's",
                "more commonly known."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nigirboran",
            args![
                "Yes, it has many names...",
                "But it's more than a mere",
                "tasty treat--the Awakening",
                "Stone can help you access",
                "your true potential. But if you're not prepared... then you'll die."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nigirboran",
            args![
                "If your mind and body",
                "aren't sufficiently trained,",
                "your body will reject the",
                "Comodo Cheese and you",
                "would die a slow, painful,",
                "yet incredibly flavorful death."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nigirboran",
            args![
                "But I'm sure that won't",
                "happen to you! If Hullaris",
                "sent you to me, you must",
                "have a fighting chance, right?",
                "So come back when you're",
                "ready for the testing~"
            ],
        )?;
        ctx.var("dmdswrd_q").set(Val::from(12))?;
    } else if subject1 == 12 {
        ctx.lines_as(
            "Nigirboran",
            args![
                "Good, you have returned--",
                "I'd expect nothing less of",
                "a brave and daring adventurer.",
                "Now hold still as I gauge your",
                "body's internal energies and see if it can handle Comodo Cheese..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nigirboran",
            args!["Alright.", "Are you ready?", "Now... Brace yourself!", "Heeeeeyah! Hoooo-HAH!"],
        )?;
        ctx.next()?;
        if (ctx.var("JobLevel").get()?.number()? > 20 && ctx.var("BaseLevel").get()?.number()? > 25) {
            let subject2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
            if subject2 == 1 {
                ctx.lines(args![
                    "^3355FFA powerful current of",
                    "warmth immediately coarses",
                    "through your body from head",
                    "to toe, and you struggle to",
                    "keep yourself from writhing.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Nigirboran",
                    args![
                        "Hmm... Your body seems",
                        "well trained, but your mind",
                        "is still reeling from the test.",
                        "If you ate the Comodo Cheese",
                        "now, you'd be reduced to an",
                        "incoherent invalid..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Nigirboran",
                    args![
                        "Go and train yourself",
                        "a little more, and try",
                        "to improve your mind's",
                        "sense of clarity. When",
                        "you feel ready, come",
                        "back to me once more."
                    ],
                )?;
            } else if subject2 == 2 {
                ctx.lines(args![
                    "^3355FFA powerful current of",
                    "warmth immediately coarses",
                    "through your body from head",
                    "to toe, and you struggle to",
                    "keep yourself from writhing.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Nigirboran",
                    args![
                        "Huh. Your body is",
                        "strong, but your spirit",
                        "is weak. You need more",
                        "training. If you were to eat",
                        "the Comodo Cheese now...",
                        "You would evaporate."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Nigirboran",
                    args![
                        "But don't lose heart,",
                        "it's too early for you",
                        "to give up. Go out and",
                        "train some more, and then",
                        "come back to me when you",
                        "feel like you're ready."
                    ],
                )?;
            } else if subject2 == 3 {
                ctx.lines(args![
                    "^3355FFA powerful current of",
                    "warmth immediately coarses",
                    "through your body from head",
                    "to toe, and you struggle to",
                    "keep yourself from writhing.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Nigirboran",
                    args![
                        "Yes... Yes...",
                        "Your mind, soul, and body",
                        "seem well trained. I think you",
                        "can eat the Comodo Cheese",
                        "without any ill effect. Good,",
                        "good, I think you're ready."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Nigirboran",
                    args![
                        "Here, take this little",
                        "token to Hullaris. That",
                        "will prove to her that",
                        "you've passed my little",
                        "test. Congratulations~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFYou've received",
                    "a token button to",
                    "present to Hullaris",
                    "from Nigirboran.^000000"
                ])?;
                ctx.var("dmdswrd_q").set(Val::from(13))?;
                ctx.next()?;
                ctx.lines_as(
                    "Nigirboran",
                    args![
                        "Ah, I almost forgot",
                        "to ask you. Why do you",
                        "seek the Comodo Cheese,",
                        "or the Awakening Stone?",
                        "Are you seeking one of",
                        "the 3 forbidden swords?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Nigirboran",
                    args![
                        "I don't know all the details,",
                        "but I do know that you'll need",
                        "the ^3355FFAwakening Stone^000000, the ^3355FFBook^000000",
                        "^3355FFof the Lamb^000000 and the ^3355FFSlate of^000000",
                        "^3355FFMurniel^000000 to even have a chance of obtaining one of those swords."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Nigirboran",
                    args![
                        "Let's see... After you",
                        "speak to Hullaris, you",
                        "should go to Al De Baran",
                        "and find someone named",
                        "^3355FFMeteurengut^000000 to learn about",
                        "the Slate of Muriniel."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Nigirboran",
                    args![
                        "Anyway, I wish you",
                        "the best of luck in",
                        "accomplishing your",
                        "goals. Godspeed..."
                    ],
                )?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "^3355FFA torrent of warmth",
                "coarses through your",
                "entire body, and you",
                "immediately faint from",
                "the rush of energy.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Nigirboran",
                args![
                    "Goodness...!",
                    "Your mind and body",
                    "are far too weak to",
                    "handle the Comodo Cheese",
                    "now. You must seriously",
                    "train yourself much more..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nigirboran",
                args![
                    "After you've developed",
                    "some more strength, come",
                    "back to me. You can retake",
                    "this little test anytime~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if subject1 == 13 {
        ctx.lines_as(
            "Nigirboran",
            args![
                "I don't know all the details,",
                "but I do know that you'll need",
                "the ^3355FFAwakening Stone^000000, the ^3355FFBook^000000",
                "^3355FFof the Lamb^000000 and the ^3355FFSlate of^000000",
                "^3355FFMurniel^000000 to even have a chance",
                "of obtaining a forbidden sword."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nigirboran",
            args![
                "Let's see... After you",
                "speak to Hullaris, you",
                "should go to Al De Baran",
                "and find someone named",
                "^3355FFMeteurengut^000000 to learn about",
                "the Slate of Muriniel."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nigirboran",
            args![
                "Anyway, I wish you",
                "the best of luck in",
                "accomplishing your",
                "goals. Godspeed..."
            ],
        )?;
    } else if subject1 == 14 {
        ctx.lines_as(
            "Nigirboran",
            args![
                "Somewhere in ^3355FFAl De Baran^000000,",
                "you'll find a man named",
                "^3355FFMeteurengut^000000. He should be",
                "able to help you learn more",
                "about the Slate of Muriniel."
            ],
        )?;
    } else {
        ctx.lines_as(
            "Nigirboran",
            args![
                "Oh... Hello there.",
                "I guess you could say",
                "that I'm something of",
                "a trainer here in Comodo.",
                "My name is Nigirboran.",
                "I know it's hard to say..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nigirboran",
            args![
                "If Hullaris hasn't sent",
                "you, then I don't think",
                "I can be of any real help",
                "for you. That seems to be",
                "the case, so I'd appreciate",
                "it if you'd let me train..."
            ],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn nigirboran_cmd(ctx: &Ctx) -> Script {
    nigirboran_cmd_body(ctx, Vec::new()).map(|_| ())
}

fn meteurengut_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = ctx.var("dmdswrd_q").get()?;
    if subject1 == 14 {
        ctx.lines_as(
            "Meteurengut",
            args![
                "Ah. Your body is surrounded",
                "by the glow of one that has",
                "eaten of the Awakening Stone.",
                "That alone may prove your value",
                "as an adventurer, but are you",
                "free from your selfish desires?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Meteurengut",
            args![
                "Even just the smallest",
                "taint of greed or jealousy",
                "can prove to be a corruptive",
                "influence when power is not",
                "tempered by wisdom and a",
                "sense of true compassion."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Meteurengut",
            args![
                "Others like you have come",
                "before me in hopes of learning",
                "about the Slate of Muriniel and",
                "eventually obtaining one of the",
                "three accursed blades. You are probably no different from them..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Meteurengut",
            args![
                "The Slate of Muriniel is an",
                "alchemic artifact that can",
                "help one access unimaginable",
                "power, originally developed",
                "by a master of alchemy that was",
                "known as Rikaseh Sumarecon."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Meteurengut",
            args![
                "Sumarecon had two apprentices,",
                "and after he had passed down the secrets of the Slate of Muriniel",
                "to only one of his proteges, his other protege killed him and the",
                "other student out of jealousy."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Meteurengut",
            args![
                "Sumarecon's secrets were",
                "thought to be lost forever...",
                "Fortunately, years later, an",
                "Alchemist named Kuprite found",
                "Sumarecon's secret documents",
                "containing his knowledge."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Meteurengut",
            args![
                "Kuprite then taught these",
                "secrets to a select group of",
                "Alchemy students, one of which",
                "was my ancestor, Burukesaemu.",
                "And so, because of my lineage,",
                "I have learned those secrets."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Meteurengut",
            args![
                "If you wish for me to",
                "reproduce the Slate of",
                "Muriniel for you, then",
                "please bring me the items",
                "I require to complete the",
                "secret alchemic procedure."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Meteurengut",
            args![
                "I shall need",
                "^3355FF1 Sapphire^000000,",
                "^3355FF1 Shining Stone^000000,",
                "^3355FF1 Rough Elunium^000000,",
                "^3355FF1 Emerald^000000, and",
                "^3355FF1 Blue Gemstone^000000."
            ],
        )?;
        ctx.var("dmdswrd_q").set(Val::from(15))?;
    } else if subject1 == 15 {
        if ((((ctx.call(Function::CountItem, vec![Val::from(717)])?.number()? > 0
            && ctx.call(Function::CountItem, vec![Val::from(726)])?.number()? > 0)
            && ctx.call(Function::CountItem, vec![Val::from(721)])?.number()? > 0)
            && ctx.call(Function::CountItem, vec![Val::from(640)])?.number()? > 0)
            && ctx.call(Function::CountItem, vec![Val::from(757)])?.number()? > 0)
        {
            ctx.lines_as(
                "Meteurengut",
                args![
                    "You've already found",
                    "all of the items I require to",
                    "create the Slate of Muriniel?",
                    "Fantastic. Now I can begin work",
                    "on this. However, there is one",
                    "more thing I must ask of you."
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(717), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(726), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(721), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(640), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(757), Val::from(1)])?;
            ctx.var("dmdswrd_q").set(Val::from(16))?;
            ctx.next()?;
            ctx.lines_as(
                "Meteurengut",
                args![
                    "Would you please bring me",
                    "^3355FF1 Cobweb^000000? You can obtain",
                    "them by slaying spiders in",
                    "Muriniel Pass which is on the",
                    "way to Al De Baran from here."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Meteurengut",
                args![
                    "I'd have asked for it",
                    "sooner, but I need to prepare",
                    "all of these stones, and I need",
                    "the freshest Cobwebs I can get.",
                    "I'll make sure that these stones are ready when you return."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Meteurengut",
            args![
                "If you wish for me to",
                "reproduce the Slate of",
                "Muriniel for you, then",
                "please bring me the items",
                "I require to complete the",
                "secret alchemic procedure."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Meteurengut",
            args![
                "I shall need",
                "^3355FF1 Sapphire^000000,",
                "^3355FF1 Shining Stone^000000,",
                "^3355FF1 Rough Elunium^000000,",
                "^3355FF1 Emerald^000000, and",
                "^3355FF1 Blue Gemstone^000000."
            ],
        )?;
    } else if subject1 == 16 {
        if ctx.call(Function::CountItem, vec![Val::from(1025)])?.number()? > 0 {
            ctx.lines_as(
                "Meteurengut",
                args![
                    "I see that you've brought",
                    "me a Cobweb. It's hard to",
                    "believe, but it's integreal to",
                    "creating the Slate of Muriniel.",
                    "Now, if you'll wait a moment,",
                    "I shall complete the slate..."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::DelItem, vec![Val::from(1025), Val::from(1)])?;
            ctx.lines(args![
                "*^3355FFClang Clang Clang!*",
                "*Zaaaaaaaaaaaaaap*",
                "*Ching tink t-t-t-tap*^000000"
            ])?;
            ctx.var("dmdswrd_q").set(Val::from(17))?;
            ctx.next()?;
            ctx.lines_as(
                "Meteurengut",
                args![
                    "*Whew* It's finished.",
                    "Please take care of this",
                    "slate, and know that I am",
                    "entrusting you with an artifact",
                    "that can help you access power",
                    "beyond your imagination..."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args!["^3355FFYou have received", "the Slate of Muriniel.^000000"])?;
            ctx.next()?;
            ctx.lines_as(
                "Meteurengut",
                args![
                    "There. You have eaten of",
                    "the Awakening Stone and you",
                    "possess the Slate of Muriniel.",
                    "Now, you must try to obtain the",
                    "^3355FFBook of the Lamb^000000 if you are",
                    "questing for a doomed sword..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Meteurengut",
                args![
                    "I know very little about",
                    "that artifact, and its secrets",
                    "are as well guarded as that of",
                    "the Slate of Muriniel. However,",
                    "I do know that a man in Morocc can create the Book of the Lamb..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Meteurengut",
                args![
                    "If that man is still in",
                    "Morocc, then he will probably",
                    "recognize the subtle emanation",
                    "of the Awakening Stone and the",
                    "Slate of Muriniel from you. Best of luck to you, adventurer..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Meteurengut",
            args![
                "Ah, have you brought a",
                "Cobweb? I know it sounds",
                "strange, but I really need the",
                "unique energy found only in",
                "fresh Cobwebs to finish the",
                "Slate of Muriniel for you..."
            ],
        )?;
    } else if subject1 == 17 {
        ctx.lines_as(
            "Meteurengut",
            args![
                "You've eaten of the",
                "Awakening Stone and",
                "now possess the Slate of",
                "Muriniel. Now, all you must",
                "do is obtain the Book of the",
                "Lamb. I know little about it..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Meteurengut",
            args![
                "All I can tell you is that",
                "there should be a man in",
                "Morocc that can create it",
                "for you. A man like that can",
                "sense the Slate of Muriniel,",
                "so he may call out to you..."
            ],
        )?;
    } else {
        ctx.lines_as(
            "Meteurengut",
            args![
                "The ^3355FFSlate of Muriniel^000000",
                "is an ancient artifact",
                "that can only be created",
                "by the power of Alchemy.",
                "It is one of the best kept",
                "secrets of my family..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Meteurengut",
            args![
                "I'm responsible for guarding",
                "the secrets of its creation, but I must also share the power",
                "of the slate with those that",
                "prove themselves worthy of it."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Meteurengut",
            args![
                "After all, power is useless",
                "if it is never used. However,",
                "if power is never balanced with",
                "wisdom and compassion, then",
                "it will inevitably do more harm",
                "than good in the very end..."
            ],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn meteurengut_cmd(ctx: &Ctx) -> Script {
    meteurengut_cmd_body(ctx, Vec::new()).map(|_| ())
}

fn zaka_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = ctx.var("dmdswrd_q").get()?;
    if subject1 == 17 {
        ctx.lines_as(
            "Zaka",
            args![
                "...You there! Hold it!",
                "Yes, I can feel it from",
                "you... The power of the",
                "Awakening Stone... And...",
                "The Slate of Muriniel?",
                "Finally, you've come."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zaka",
            args![
                "I'm fully aware that only",
                "those that seek to own one",
                "of the doomed swords would",
                "trouble themselve to obtain",
                "those items. All that is left for you is the Book of the Lamb..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zaka",
            args![
                "I've been waiting for so",
                "long for the opportunity to",
                "create the Book of the Lamb.",
                "If you really want the book,",
                "I will help you so long as",
                "you will help me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zaka",
            args![
                "However, creating the book",
                "is a complicated process and",
                "requires multiple stages of",
                "preparation. Therefore, I shall",
                "ask you to bring me the items",
                "I require in separate batches."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zaka",
            args![
                "Alright...",
                "What was it now?",
                "Ah, yes. The first thing",
                "I need is ^3355FF2 Snake Scales^000000,",
                "so please bring those soon."
            ],
        )?;
        ctx.var("dmdswrd_q").set(Val::from(18))?;
    } else if subject1 == 18 {
        if ctx.call(Function::CountItem, vec![Val::from(926)])?.number()? > 1 {
            ctx.call(Function::DelItem, vec![Val::from(926), Val::from(2)])?;
            ctx.var("dmdswrd_q").set(Val::from(19))?;
            ctx.lines_as(
                "Zaka",
                args![
                    "Ah, you've brought",
                    "the Snake Scales? Good,",
                    "let me take them now and",
                    "begin work on the Book of",
                    "the Lamb. Now, I need you",
                    "to bring me ^3355FF1 Scale Shell^000000."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Zaka",
                args![
                    "I know it would be more",
                    "convenient for you if I told",
                    "you everything I needed at",
                    "once, but actually, this way",
                    "is much more convenient for",
                    "me. I'll be waiting right here!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Zaka",
            args![
                "The first thing I'll",
                "need to begin creating",
                "the Book of the Lamb is",
                "^3355FF2 Snake Scales^000000. Come back",
                "to me once you get them,",
                "alright? I'll be right here."
            ],
        )?;
    } else if subject1 == 19 {
        if ctx.call(Function::CountItem, vec![Val::from(936)])?.number()? > 0 {
            ctx.lines_as(
                "Zaka",
                args![
                    "Oh, nice! You've brought",
                    "this Scale Shell for me,",
                    "right? Great, great, this will",
                    "help enhance the book's",
                    "physical durability, but it",
                    "still needs magic durability..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Zaka",
                args![
                    "Next, you need to bring",
                    "me ^3355FF1 Shining Shell^000000. Hurry",
                    "and bring you to me before",
                    "I finish this part of the process, okay? Wait, actually, I think",
                    "you can take your time..."
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(936), Val::from(1)])?;
            ctx.var("dmdswrd_q").set(Val::from(20))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Zaka",
            args![
                "I'll be waiting right",
                "here while you go and",
                "look for a ^3355FFScale Shell^000000",
                "that you can bring to me.",
                "I'll need that in order to",
                "finish this Book of the Lamb."
            ],
        )?;
    } else if subject1 == 20 {
        if ctx.call(Function::CountItem, vec![Val::from(954)])?.number()? > 0 {
            ctx.lines_as(
                "Zaka",
                args![
                    "Ah, you're just in time!",
                    "Have you got the Shining",
                    "Scale? Perfect. Now, there",
                    "is just one more item that",
                    "I want to ask you to bring me:",
                    "^3355FF1 Stinky Scale^000000. Easy, right?"
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(954), Val::from(1)])?;
            ctx.var("dmdswrd_q").set(Val::from(21))?;
            ctx.next()?;
            ctx.lines_as(
                "Zaka",
                args![
                    "The energy of a Stinky",
                    "Scale can be used to",
                    "regulate the power of",
                    "incredibly potent artifacts.",
                    "Without that item, the Book",
                    "of the Lamb isn't much use..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Zaka",
            args![
                "If you want me to",
                "finish this Book of",
                "the Lamb, you need",
                "to come back here with",
                "^3355FF1 Shining Scale^000000. There's",
                "no way to work around it..."
            ],
        )?;
    } else if subject1 == 21 {
        if ctx.call(Function::CountItem, vec![Val::from(959)])?.number()? > 0 {
            ctx.lines_as(
                "Zaka",
                args![
                    "What's that sme--?",
                    "Oh, right. I asked you to",
                    "bring me a Stinky Scale.",
                    "Now I can finally complete",
                    "this Book of the Lamb! I've",
                    "been looking forward to this..."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFZaka completes",
                "the creation of the",
                "Book of the Lamb.^000000"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(959), Val::from(1)])?;
            ctx.var("dmdswrd_q").set(Val::from(22))?;
            ctx.next()?;
            ctx.lines_as(
                "Zaka",
                args![
                    "What...?!",
                    "That's it? Well, um,",
                    "it's done. That felt",
                    "rather anticlimatic, but",
                    "I can finally see this",
                    "thing with my own eyes!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Zaka",
                args![
                    "Alright, you've eaten of the",
                    "Awakening Stone, obtained the",
                    "Slate of Muriniel, and now have",
                    "the Book of the Lamb. You're",
                    "getting very close to owning",
                    "one of the doomed swords."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Zaka",
                args![
                    "There is man that you",
                    "must find in Comodo named",
                    "^3355FFWon^000000. He will judge you, and",
                    "then give you the proof that",
                    "shows you are qualified to",
                    "be a doomed sword bearer."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Zaka",
                args![
                    "He looks like a simple",
                    "man, but he is a true living",
                    "legend renown for his wisdom.",
                    "If you really want a doomed",
                    "sword, you need to speak to",
                    "Won and ask him to guide you."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Zaka",
            args![
                "The last thing that I need",
                "to complete the Book of the",
                "Lamb is ^3355FF1 Stinky Scale^000000. Please",
                "bring that to me as soon as",
                "you can. Thank you for being",
                "patient, young adventurer."
            ],
        )?;
    } else if subject1 == 22 {
        ctx.lines_as(
            "Zaka",
            args![
                "Now, you need to visit a",
                "man named ^3355FFWon^000000 in Comodo",
                "in order to get the proof that",
                "shows that you're qualified to",
                "possess a doomed sword.",
                "Hopefully, you'll get it..."
            ],
        )?;
    } else {
        ctx.lines_as(
            "Zaka",
            args![
                "Have you heard that swords",
                "with the power to change the",
                "world actually exist? It's true",
                "that three swords contain this",
                "immense power, but they are",
                "also bound to powerful curses."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zaka",
            args![
                "Therefore, the Mysteltainn,",
                "Ogretooth, and Executioner",
                "have all been sealed away.",
                "Only a truly great adventurer",
                "can release these doomed swords and actually wield them..."
            ],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn zaka_cmd(ctx: &Ctx) -> Script {
    zaka_cmd_body(ctx, Vec::new()).map(|_| ())
}

fn won_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = ctx.var("dmdswrd_q").get()?;
    if subject1 == 22 {
        ctx.lines_as(
            "Won",
            args![
                "Hmm...? It's been a while",
                "since someone came here to",
                "get the qualification to own a",
                "doomed sword. Yeah, I know",
                "that's why you're here..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Won",
            args![
                "To a man like me, the",
                "presenses of the Book of the",
                "Lamb, the Slate of Muriniel,",
                "and one that has eaten of the",
                "Awakening Stone are unmistakable."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Won",
            args![
                "Now, listen. I personally",
                "don't care why you want a",
                "doomed sword. If you abuse",
                "its power, you'll pay the price",
                "eventually. I'm only here to",
                "make sure you're up to snuff."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Won",
            args![
                "Of course, I also want to",
                "make sure that you don't bring",
                "great suffering to the world",
                "using a doomed sword. So I'm",
                "going to check if you're, you",
                "know, balanced and all that."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Won",
            args![
                "Alright, let me take",
                "a look into your eyes...",
                "Yeah, alright. You seem to",
                "understand the value of the",
                "Awakening Stone, Book of the Lamb, and the Slate of Muriniel."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Won",
            args![
                "Yeah, I can't sense any",
                "ill intent from you at all.",
                "That's very good. Huh, you",
                "seem pretty experienced in battle, so I'm sure you have the stamina",
                "to handle to a doomed sword..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Won",
            args![
                "Ah, but most importantly,",
                "you don't strike me as crazy",
                "at all. Alright, we're done here. Take this Stamp of Muriniel:",
                "it's the official qualification",
                "token to own a doomed sword."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args!["^3355FFYou receive the", "Stamp of Muriniel.^000000"])?;
        ctx.var("dmdswrd_q").set(Val::from(23))?;
        ctx.next()?;
        ctx.lines_as(
            "Won",
            args![
                "Mysteltainn, Executioner,",
                "and Ogretooth... These are",
                "the forbidden blades, the",
                "doomed swords. Remember",
                "that you cannot choose",
                "which weapon you'll wield..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Won",
            args![
                "These swords have a will",
                "of their own. The doomed",
                "sword that finds you most",
                "worthy of it will choose you.",
                "You can understand that, right?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Won",
            args![
                "Now that you have this",
                "qualification, I'm supposed",
                "to direct you to the Sages",
                "that can help lead you to",
                "the doomed swords."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Won",
            args![
                "The first Sage that you",
                "must visit is ^3355FFSage Yklah^000000",
                "in the city of ^3355FFJuno^000000 in the",
                "Schwarzwald Republic.",
                "Your quest to obtain a",
                "doomed sword isn't over yet..."
            ],
        )?;
    } else if subject1 == 23 {
        ctx.lines_as(
            "Won",
            args![
                "Now that you have this",
                "qualification, I'm supposed",
                "to direct you to the Sages",
                "that can help lead you to",
                "the doomed swords."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Won",
            args![
                "The first Sage that you",
                "must visit is ^3355FFSage Yklah^000000",
                "in the city of ^3355FFJuno^000000 in the",
                "Schwarzwald Republic.",
                "Your quest to obtain a",
                "doomed sword isn't over yet..."
            ],
        )?;
    } else if subject1 == 24 {
        ctx.mes("1 2 3 4 5 6 7 8 9 10 1 2 3 4 5")?;
    } else {
        ctx.lines_as(
            "Won",
            args![
                "The visitors and even the",
                "people that live here always",
                "seem to be having such a good",
                "time, just lounging in leisure.",
                "It's hard to believe the War",
                "of the Witch even happened..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Won",
            args![
                "But not everyone here in",
                "Comodo can afford to relax",
                "so easily. I, for one, have",
                "the responsibility of seeking",
                "out those that are worthy of, well, I don't know if I can say..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Won",
            args![
                "You just have to",
                "trust that I've got an",
                "incredibly important",
                "job to do. I mean, sure,",
                "it doesn't look like I'm doing",
                "anything right now, but..."
            ],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn won_cmd(ctx: &Ctx) -> Script {
    won_cmd_body(ctx, Vec::new()).map(|_| ())
}
