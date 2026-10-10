use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn bard_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_random = Val::from(0);
    ctx.var("@name$").set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
    if ctx.var("gef_bard_q").get()? == 31 {
        ctx.lines_as(
            "Kino Kitty",
            args![
                "Everything will be",
                "fine in the end, right?",
                "I'll be okay, Jorti will be okay, and she'll be... Oh."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kino Kitty",
            args![
                "I'm sorry, I was just",
                "muttering to myself.",
                "Is there anything",
                "you want from me?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("Tell me a story...:Your voice is...:No thanks, I appreciate it though.")],
        )? {
            1 => {
                l_random = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                if l_random.clone() == 1 {
                    ctx.lines_as("Kino Kitty", args!["Many legends have been passed down as songs. Stories of Gods and tales of brave warriors have all been written as songs."])?;
                    ctx.next()?;
                    ctx.lines_as("Kino Kitty", args!["Many people love to listen to those songs because they dream of the romantic past. To sing these kinds of songs is the reason people like me exist."])?;
                    ctx.next()?;
                    ctx.mes("^3355FFKino Kitty adjusted the strings on his guitar, and began to sing in a low voice.^000000")?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^D43D1ARheingold...",
                        "Hidden in the Rhein river.",
                        "If made into a ring,",
                        "Could rule the world~^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^D43D1AProtected by a spell",
                        "Cursing its thief",
                        "To never find love.",
                        "Alberich had known",
                        "Nonetheless stole it.",
                        "Love was forsaken for power.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^D43D1AGiants built beautiful Valhalla",
                        "Matrimony with Freya",
                        "Goddess of beauty",
                        "Their supposed payment.",
                        "In not receiving it",
                        "They forcefully took her.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^D43D1AThe gods would",
                        "give Alberich's treasure",
                        "To the Giants for Freya's return.",
                        "Loki tricked Alberich,",
                        "Stealing his ring or power.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^D43D1ABut Alberich cursed his ring",
                        "Before it was returned to him,",
                        "Envy and death would",
                        "befall its wearers.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^D43D1AThe ring was given to the giants",
                        "Freya was returned to the gods",
                        "The giants killed themselves",
                        "Fighting over the rheingold,",
                        "Victims of",
                        "Alberich's curse~^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kino Kitty",
                        args![
                            "Hmm...",
                            "^333333*Cough Cough*^000000",
                            "This old song",
                            "is about the ring",
                            "of the Nibelungs."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kino Kitty",
                        args![
                            "^333333*Cough Cough*^000000",
                            "Hah... I can't sing more than one song nowadays. But did you like it? ^333333*Cough Cough*^000000"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if l_random.clone() == 2 {
                    ctx.lines_as(
                        "Kino Kitty",
                        args![
                            "Ah...",
                            "I do feel like singing a song. You know, every song has its own story. ^333333*Cough Cough*^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("^3355FFKino Kitty coughed a few times, wiping his mouth with his sleeve. As he adjusts the guitar strings, you notice small stains of blood on his sleeve. Then, he began to sing.^000000")?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^D43D1ABrave hero Siegfried ",
                        "Vanquished a mighty dragon\t",
                        "Its blood coated his skin",
                        "Making it impenetrable",
                        "Save for one tender spot",
                        "Blocked by a single leaf",
                        "From a linden tree.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^D43D1ASiegfried was powerful",
                        "Clearly invincible, save for",
                        "those who knew of his secret.",
                        "In the end he was killed, by",
                        "A spear flung into his back",
                        "set into motion by the wrath",
                        "And jealousy of a woman.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jorti",
                        args![
                            "Bravo, Bravo~!",
                            "Uncle Kino is the best Bard in the world~! Jorti likes Uncle Kino's singing!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Kino Kitty", args!["Thank you, Jorti.", "Anytime for my", "little princess."])?;
                    ctx.next()?;
                    ctx.lines_as("Kino Kitty", args!["This song is about Sigfried, who was invincible, except for a single spot on his back. Just singing this reminds me of the influence women have over the world."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Kino Kitty", args!["You want to", "listen to a story?", "Mmm, let me think..."])?;
                    ctx.next()?;
                    ctx.lines_as("Kino Kitty", args!["When you go venture Southwest from Morocc, you will arrive at Fortress Sandarman. Did you know that the fortress is built on a sand hill, which serves as a natural defense?"])?;
                    ctx.next()?;
                    ctx.lines_as("Kino Kitty", args!["When you go South of Sandarman, you will see Paros Lighthouse. East from the Lighthouse, you will see Kokomo Beach. North of the beach is Papuchica Forest."])?;
                    ctx.next()?;
                    ctx.lines_as("Kino Kitty", args!["I'm not boring you, am I? I just wanted to tell you about the village of Umbala, which is above that last place I was telling you about. Have you ever been there?"])?;
                    ctx.next()?;
                    ctx.lines_as("Kino Kitty", args!["Umbala itself is a pretty interesting place. But I'm really interested in this giant tree in Umbala. I've heard that it leads to Niffheim..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kino Kitty",
                        args![
                            "I wonder...",
                            "Could that tree be Yggdrasil? Is it possible that I could meet her in the Niffheim, realm of the dead?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines_as("Kino Kitty", args!["Oh... Well, I have a chronic disease so I can't speak out loud. I, um, even cough up a little blood. But aside from that, I'm perfectly healthy."])?;
                ctx.next()?;
                ctx.mes("^3355FFA look of bitterness momentarily flashed across Kino Kitty's face. He then adjusted his guitar strings and began to play, humming a low tune.^000000")?;
                ctx.call(Function::SoundEffect, vec![Val::from("humming.wav"), Val::from(0)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as("Kino Kitty", args!["Hmm...?", "I wish everyone in the world were as kind and honest as you. If that were the case, there'd be no selfish fights. Hahahahaha~"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        if ctx.var("gef_bard_q").get()? == 30 {
            ctx.lines_as(
                "Kino Kitty",
                args![
                    "Everything will be",
                    "fine in the end, right?",
                    "I'll be okay, Jorti will be okay, and she'll be... Oh."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kino Kitty",
                args![
                    "I'm sorry, I was just",
                    "muttering to myself.",
                    "Is there anything",
                    "you want from me?"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Tell me a story, or sing something~:Your voice is...:No thanks. I appreciate it, though.",
                )],
            )? {
                1 => {
                    l_random = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                    if l_random.clone() == 1 {
                        ctx.lines_as(
                            "Kino Kitty",
                            args![
                                "Ah...",
                                "I do feel like singing a song. You know, every song has its own story. ^333333*Cough Cough*^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.mes("^3355FFKino Kitty coughed a few times, wiping his mouth with his sleeve. As he adjusts the guitar strings, you notice small stains of blood on his sleeve. Then, he began to sing.^000000")?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^D43D1ABrave hero Siegfried ",
                            "Vanquished a mighty dragon",
                            "Its blood coated his skin",
                            "Making it impenetrable",
                            "Save for one tender spot",
                            "Blocked by a single leaf",
                            "From a linden tree.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^D43D1ASiegfried was powerful",
                            "Clearly invincible, save for",
                            "those who knew of his secret.",
                            "In the end he was killed, by",
                            "A spear flung into his back",
                            "set into motion by the wrath",
                            "And jealousy of a woman.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Jorti",
                            args![
                                "Bravo, Bravo~!",
                                "Uncle Kino is the best Bard in the world~! Jorti likes Uncle Kino's singing!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Kino Kitty", args!["Thank you, Jorti.", "Anytime for my", "little princess."])?;
                        ctx.next()?;
                        ctx.lines_as("Kino Kitty", args!["This song is about Sigfried, who was invincible, except for a single spot on his back. Just singing this reminds me of the influence women have over the world."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if l_random.clone() == 2 {
                        ctx.lines_as(
                            "Kino Kitty",
                            args![
                                "Ah...",
                                "I do feel like singing a song. You know, every song has its own story. ^333333*Cough Cough*^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.mes("^3355FFKino Kitty coughed a few times, wiping his mouth with his sleeve. As he adjusts the guitar strings, you notice small stains of blood on his sleeve. Then, he began to sing.^000000")?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^D43D1AI still remember feeling",
                            "Your shoulder's warmth",
                            "As I leaned on it.",
                            "You held my hands",
                            "until the final moment.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^D43D1ACrying on the inside,",
                            "I saw you off with a smile.",
                            "Things would never be the same,",
                            "But I'd remember every single",
                            "Moment we shared.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^D43D1AI still remember the clear chimes",
                            "Of the Prontera Church Bells",
                            "The ice cream we shared in Morocc,",
                            "Being chased by bats",
                            "In Payon dungeon.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^D43D1AGazing together at",
                            "Comodo's fireworks",
                            "Snuggles under the",
                            "Gently falling Lutie",
                            "Snowflakes...^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^D43D1AYour beautiful soul",
                            "Will be with Odin.",
                            "When the day",
                            "Of the dusk comes,",
                            "A brand new place",
                            "Will be before your eyes.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^D43D1AI will remember you forever.",
                            "Forget me not, call my name.",
                            "I'll hear you, and look to the skies.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Jorti",
                            args!["Ah, Uncle Kino, isn't this the song my mom always sang? Sing more songs, please! Please~!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Kino Kitty", args!["You're right...", "She always used to sing this song... ^333333*Cough*^000000 There are always people waiting for their beloved to return..."])?;
                        ctx.next()?;
                        ctx.lines_as("Kino Kitty", args!["If you have someone who you've left behind, someone that is waiting for you, make sure you come back to that person. You'll regret it if you don't."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Kino Kitty", args!["You want to", "listen to a story?", "Mmm, let me think..."])?;
                        ctx.next()?;
                        ctx.lines_as("Kino Kitty", args!["When you go venture Southwest from Morocc, you will arrive at Fortress Sandarman. Did you know that the fortress is built on a sand hill, which serves as a natural defense?"])?;
                        ctx.next()?;
                        ctx.lines_as("Kino Kitty", args!["When you go South of Sandarman, you will see Paros Lighthouse. East from the Lighthouse, you will see Kokomo Beach. North of the beach is Papuchica Forest."])?;
                        ctx.next()?;
                        ctx.lines_as("Kino Kitty", args!["I'm not boring you, am I? I just wanted to tell you about the village of Umbala, which is above that last place I was telling you about. Have you ever been there?"])?;
                        ctx.next()?;
                        ctx.lines_as("Kino Kitty", args!["Umbala itself is a pretty interesting place. But I'm really interested in this giant tree in Umbala. I've heard that it leads to Niffheim..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kino Kitty",
                            args![
                                "I wonder...",
                                "Could that tree be Yggdrasil? Is it possible that I could meet her in the Niffheim, realm of the dead?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                2 => {
                    ctx.lines_as("Kino Kitty", args!["Oh... Well, I have a chronic disease so I can't speak out loud. I, um, even cough up a little blood. But aside from that, I'm perfectly healthy."])?;
                    ctx.next()?;
                    ctx.mes("^3355FFA look of bitterness momentarily flashed across Kino Kitty's face. He then adjusted his guitar strings and began to play, humming a low tune.^000000")?;
                    ctx.call(Function::SoundEffect, vec![Val::from("humming.wav"), Val::from(0)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as("Kino Kitty", args!["Hmm...?", "I wish everyone in the world were as kind and honest as you. If that were the case, there'd be no selfish fights. Hahahahaha~"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            if (ctx.var("gef_bard_q").get()?.number()? > 12 && ctx.var("gef_bard_q").get()?.number()? < 16) {
                ctx.lines_as("Kino Kitty", args!["Errende must be waiting for you. Although others would be in anguish if they were made to wait, and people like Errende and me wouldn't mind, it's still more polite to hurry when you can."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("gef_bard_q").get()? == 12 {
                    ctx.lines_as("Kino Kitty", args!["I guess you've met Mr. Sketzi.", "By the way, why do you have that look on your face? Didn't find what you were looking for? Haha, would you show me your left hand?"])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Here you go.:Um... No!")])?) == 1 {
                        ctx.lines_as(
                            "Kino Kitty",
                            args![
                                "Well, while you were gone,",
                                "I remembered the original lyrics to the song. I guess they really are important."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Kino Kitty", args!["Here, let me give you a letter with the lyrics to Errende. I should also give you my seal so that Errende will know it's me. You don't mind, do you?"])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Yes, I mind!:No, I don't mind.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Kino Kitty",
                                    args![
                                        "Ah, right.",
                                        "You already have a seal marked by Errende. Oh well, if you don't want it, what can I do?"
                                    ],
                                )?;
                                ctx.var("gef_bard_q").set(Val::from(13))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Kino Kitty",
                                    args!["Now, let's see...", "How is it supposed to go?", "...Lu lu lu...", "...La la la..."],
                                )?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^D43D1AAt One, I fall in love.",
                                    "At Two, you give me your smile.",
                                    "At Three, I adore your touch.",
                                    "At Four, a tender kiss.",
                                    "At Five, we change our minds.",
                                    "A petal scatters through the air.^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^D43D1AAt Six, I fall in love~",
                                    "At Seven, you fall in love~",
                                    "At Eight, we're both in love.",
                                    "At Nine, you know my heart.",
                                    "At Ten, I know you've",
                                    "been waiting for me.^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^D43D1AAt Eleven, a precious",
                                    "Whisper: 'Will you marry me?'",
                                    "At Twelve, our two hearts",
                                    "Are one. 12 petals, our",
                                    "Love finally blossoms.^000000"
                                ])?;
                                ctx.next()?;
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_POISONREACT2")?])?;
                                ctx.mes("^3355FFAfter the song, Kino smiles at you. You feel a sharp pain on your wrist. On the spot where the silver crescent was, you see a tiny black cross.^000000")?;
                                ctx.next()?;
                                ctx.lines_as("Kino Kitty", args!["So, how do you like it? Well, I'm certain that Errende will love it for sure. Hahahahaha~ ^333333*Cough Cough*^000000"])?;
                                ctx.next()?;
                                ctx.lines_as("Kino Kitty", args!["I want to apologize.", "I didn't mean to make you wait, but I just remembered the old lyrics to the song. Now, hurry to Errende, I'm sure he's waiting."])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFKino Kitty hands you a letter that is labeled 'Dear Errende'",
                                    "on the front.^000000"
                                ])?;
                                ctx.var("gef_bard_q").set(Val::from(14))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else {
                        ctx.lines_as(
                            "Kino Kitty",
                            args!["Hmmm...?", "There's no reason to be scared.", "I mean, I won't bite or anything."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("gef_bard_q").get()? == 10 {
                        ctx.lines_as(
                            "Kino Kitty",
                            args![
                                "Didn't you leave already?",
                                "I'm sure you can find the lyrics in a bookstore in Juno."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Kino Kitty", args!["Well, I'm sure Errende knows more about bookstores, so he can probably tell you where to look in Juno. Why don't you ask Errende?"])?;
                        ctx.next()?;
                        ctx.lines_as("Kino Kitty", args!["If you don't mind, please come again and listen to this poor Bard's songs. I'll prepare a song just for you."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kino Kitty",
                            args!["Don't worry about my health. I'm a Bard at heart, and I can't help but sing for my audience."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kino Kitty",
                            args!["Also, I'd like to thank you for helping Errende. He may be a little whiny, but he's a good person."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("gef_bard_q").get()? == 11 {
                            ctx.lines_as(
                                "Kino Kitty",
                                args![
                                    "A silver seal...?",
                                    "Well, welcome to the wanderer's club. I must say, the silver crescent suits you."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kino Kitty",
                                args!["If you have that seal, Mr. Sketzi will permit you to read his books full of Norse songs."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if (ctx.var("gef_bard_q").get()?.number()? > 19 && ctx.var("gef_bard_q").get()?.number()? < 28) {
                                ctx.lines_as("Kino Kitty", args!["What business", "do you have with me,", "my friend?"])?;
                                ctx.next()?;
                                if ctx.var("gef_bard_q").get()? == 24 {
                                    ctx.mes("^3355FFYou gave candy to the crying little girl, and relate the problem with the lyrics to Kino Kitty.^000000")?;
                                    ctx.next()?;
                                    ctx.lines_as("Kino Kitty", args!["Hmm...", "I'm insulted that Errende does not like the words I wrote for 'At One, I Fall in Love.' But, I suppose he is a romantic at heart."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.var("@name$").get()?,
                                        args!["So...", "Do you know the", "original words", "for the song?"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Kino Kitty", args!["The original lyrics were horrible. There's not enough room in my mind to remember so romantic nonsense. ^333333*Cries*...^000000"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Kino Kitty",
                                        args![
                                            "Romance is for foolish dreamers!",
                                            "I refuse to sing or even remember such vapid lyrics!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Jorti",
                                        args![
                                            "U-uncle Kino",
                                            "You're scaring me.",
                                            "Please don't yell!",
                                            "It makes me want",
                                            "to cry..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Kino Kitty",
                                        args![
                                            "Oh...",
                                            "I'm sorry, princess.",
                                            "It won't happen again.",
                                            "Your Unclde Kino will",
                                            "try to be good",
                                            "from now on."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Kino Kitty", args!["*Sigh...*", "Alright, listen."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Kino Kitty", args!["You'll need an old book of edda lyrics for the original words to the song. All the new books no longer contain the original version."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Kino Kitty", args!["I wonder...", "How did Errende happen to know the original lyrics? In any case, I'm sorry about all this. I suppose I'm a little jaded."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Kino Kitty", args!["You know, maybe you should go to Juno. There's a small book store on the book street, and you can probably find the song in that store."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Kino Kitty", args!["^333333^333333*Cough Cough*^000000^000000", "Wh-why do I have to suffer? Are my final days as a Bard on this earth soon approaching? I feel so pathetic..."])?;
                                    ctx.var("gef_bard_q").set(Val::from(10))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    match runtime::select_values(ctx, &[Val::from("Tell me a story.:Sing a song.:Nothing.")])? {
                                        1 => {
                                            ctx.lines_as(
                                                "Kino Kitty",
                                                args![
                                                    "A story? Mmmm...",
                                                    "My throat hurts,",
                                                    "but if you listen,",
                                                    "I will do my best",
                                                    "to relate a tale."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Kino Kitty", args!["Do you know how Baldur, son of Odin, died? There is a story involving his obsessive mother", "and the wicked god, Loki."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Kino Kitty", args!["One night, Baldur had a nightmare where he was cast into the pit of Hell. Upon hearing of this, Baldur's mother Frigg was stricken with panic."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Kino Kitty", args!["To prevent tragedy from befalling her son, she made every single object on earth to agree to an oath where they would not harm Baldur."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Kino Kitty", args!["^333333*Cough Cough*^000000 And they all got together to ^333333*Wheeeeze*^000000 celebr--^333333^333333*Cough*^000000^000000..."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Jorti", args!["Uncle Kino...?", "Are you okay?....?"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Kino Kitty",
                                                args![
                                                    "...Hah...",
                                                    "...Hah...",
                                                    "I'm sorry, but I don't think I can speak for much longer."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as("Kino Kitty", args!["Sorry, but I don't feel like singing at the moment. I hope you understand. I want to sing the last song of my life for Jorti..."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        3 => {
                                            ctx.lines_as("Kino Kitty", args!["You have no", "business with me?", "What a shame."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                            } else {
                                ctx.lines(args![
                                    "^D43D1AEven in the sandy wind,",
                                    "Even in the pouring rain,",
                                    "Even in the falling snow,",
                                    "I know it's you~^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines_as("Mysterious Bard", args!["..."])?;
                                ctx.next()?;
                                ctx.lines_as("Mysterious Bard", args!["...", "......"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Mysterious Bard",
                                    args![
                                        "My apologies,",
                                        "^333333*Cough*^000000 but this is",
                                        "a private performance",
                                        "for my little",
                                        "princess."
                                    ],
                                )?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from("Um, was that a love song?:I'm sorry for interrupting you.")],
                                )?) == 1
                                {
                                    ctx.lines_as("Mysterious Bard", args!["A...", "Love song?!"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Mysterious Bard", args!["Love songs are for fools who have never known true love, or never had their hearts broken. Happy songs are fine, but I refuse to sing nonsense!"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Jorti",
                                        args!["Waaaah~!", "Uncle Kino,", "I'm scared!", "I wanna see", "my mommy...!", "*Cries*"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Mysterious Bard",
                                        args!["There, there,", "princess. Don't cry.", "Everything's okay."],
                                    )?;
                                    ctx.next()?;
                                    ctx.mes("[Mysterious Bard]")?;
                                    if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
                                        ctx.mes(
                                            "There's no reason to be scared. The scary people with swords only use them on monsters, okay?",
                                        )?;
                                    } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                                        ctx.mes("You don't need to be scared, that person is a servant of God, okay?")?;
                                    } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?)
                                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?))
                                    {
                                        ctx.mes("There's no reason to be afraid of this riffraff, your Uncle Kino is here, okay?")?;
                                    } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?) {
                                        ctx.mes("There's no reason to be afraid. I know that person looks scary, but you're a good girl, so you'll be okay.")?;
                                    } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_BLACKSMITH")?) {
                                        ctx.mes("There's no reason to be scared, honey. It's just a Blacksmith.")?;
                                    } else {
                                        ctx.mes("There's no reason to be scared. See...? That person won't hurt you.")?;
                                    }
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.var("@name$").get()?,
                                        args!["Um...", "^333333(Wasn't the little girl", "scared by your outburst...?)^000000"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.var("gef_bard_q").get()? == 2 {
                                    ctx.lines_as("Mysterious Bard", args!["You're such a kind, young person. I will remember your name. I would much appreciate it if you would also remember mine."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Mysterious Bard",
                                        args![
                                            "I am known as Kino Kitty. I am a wandering poet who wishes to rediscover wishes and dreams."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Kino Kitty", args!["When next we meet, I will tell you what I have heard and experienced. That is, I am willing to spend my time with you."])?;
                                    ctx.var("gef_bard_q").set(Val::from(22))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.var("gef_bard_q").get()? == 3 {
                                    ctx.lines_as("Mysterious Bard", args!["You're such a kind, young person. I will remember your name. I would much appreciate it if you would also remember mine."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Mysterious Bard",
                                        args![
                                            "I am known as Kino Kitty. I am a wandering poet who wishes to rediscover wishes and dreams."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Kino Kitty", args!["When next we meet, I will tell you what I have heard and experienced. That is, I am willing to spend my time with you."])?;
                                    ctx.var("gef_bard_q").set(Val::from(23))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.var("gef_bard_q").get()? == 4 {
                                    ctx.lines_as(
                                        "Mysterious Bard",
                                        args!["Oh, you're most admirable. You truly do respect Bards, don't you?"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Mysterious Bard", args!["Now that I think about it, are you look for anything, or is there a reason you wish to speak to me?"])?;
                                    ctx.next()?;
                                    if Val::from(runtime::select_values(
                                        ctx,
                                        &[Val::from("I'm looking for Kino Kitty...:Nothing, really.")],
                                    )?) == 1
                                    {
                                        ctx.lines_as(
                                            "Kino Kitty",
                                            args![
                                                "How do you",
                                                "know my name?",
                                                "Ah, I'm flattered",
                                                "that my reputation",
                                                "precedes me."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.mes("^3355FFYou give some candy to the crying girl and relate your tale regarding the lyrics for the song 'At One, I Fall in Love.'^000000")?;
                                        ctx.next()?;
                                        ctx.lines_as("Kino Kitty", args!["Hmm...", "I'm insulted that Errende does not like the words I wrote for 'At One, I Fall in Love.' But, I suppose he is a romantic at heart."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.var("@name$").get()?,
                                            args!["So...", "Do you know the", "original words", "for the song?"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Kino Kitty", args!["The original lyrics were horrible. There's not enough room in my mind to remember so romantic nonsense. ^333333*Cries*...^000000"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Kino Kitty",
                                            args!["Romance is for foolish dreamers! I refuse to sing or even remember such vapid lyrics!"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Jorti",
                                            args![
                                                "U-uncle Kino",
                                                "You're scaring me.",
                                                "Please don't yell!",
                                                "It makes me want",
                                                "to cry..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Kino Kitty", args!["Oh...", "I'm sorry, princess. It won't happen again. Your Unclde Kino will try to be good from now on."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Kino Kitty", args!["*Sigh...*", "Alright, listen."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Kino Kitty", args!["You'll need an old book of edda lyrics for the original words to the song. All the new books no longer contain the original version."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Kino Kitty", args!["I wonder...", "How did Errende happen to know the original lyrics? In any case, I'm sorry about all this. I supposed I'm a little jaded."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Kino Kitty", args!["You know, maybe you should go to Juno. There's a small book store on the book street, and you can probably find the song in that store."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Kino Kitty", args!["^333333^333333*Cough Cough*^000000^000000", "Wh-why do I have to suffer? Are my final days as a Bard on this earth soon approaching? I feel so pathetic..."])?;
                                        ctx.var("gef_bard_q").set(Val::from(10))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as("Mysterious Bard", args!["Really now?", "That's strange.", "You adventurers are always one some kind of adventure, aren't you? I mean, that's the very definition of the word."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                } else if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                    ctx.lines_as("Kino Kitty", args!["You're such a nice young man. I will remember your name. I would much appreciate it if you would also remember mine."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Mysterious Bard",
                                        args![
                                            "I am known as Kino Kitty. I am a wandering poet who wishes to rediscover wishes and dreams."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Kino Kitty", args!["When next we meet, I will tell you what I have heard and experienced. That is, I am willing to spend my time with you."])?;
                                    ctx.var("gef_bard_q").set(Val::from(20))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as("Mysterious Bard", args!["Ah, it is a privilege to meet a noble woman such as yourself. By your leave, I shall give you my name."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Mysterious Bard", args!["My lady, fair as wisteria, whose beauty rivals that of the goddess Freya, let me introduce myself as the poor poet who wanders the earth, Kino Kitty. I hope you remember me."])?;
                                    ctx.var("gef_bard_q").set(Val::from(20))?;
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
    Ok(Val::from(0))
}

pub fn bard_3(ctx: &Ctx) -> Script {
    bard_3_body(ctx, Vec::new()).map(|_| ())
}

fn little_girl_jorti_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("gef_bard_q").get()?.number()? > 9 && ctx.var("gef_bard_q").get()?.number()? < 30) {
        ctx.lines_as(
            "Jorti",
            args!["Jorti tries not to cry anymore because it hurts Uncle Kino and then he coughs up more blood."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jorti",
            args!["I miss my mommy a lot, but I'm worried, it makes Uncle Kino worry a lot too."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jorti",
            args!["I don't see my mommy and daddy aren't here anymore, and I don't want Uncle Jorti to go away either."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jorti",
            args!["Um, I don't remember my daddy, but mommy lives far away, somewhere near the sky. That's what Uncle Kino says."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jorti",
            args!["U-Uncle Kino says we can't go see her yet because there are no flying boats yet..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("gef_bard_q").get()? == 30 {
        ctx.lines_as(
            "Jorti",
            args![
                "La la la...",
                "La la la...",
                "Jorti doesn't cry anymore!",
                "Jorti is going to enjoy Uncle Kino's songs and stories for as long as she can!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jorti",
            args!["The songs Uncle Kino sings are ones that my mommy used to sing. She's not here, but maybe we can visit her someday!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jorti",
            args!["Hey... The shiny black cross on your hand means you're a friend of Uncle Kino's. So that means, you're my friend too!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("gef_bard_q").get()?.number()? > 30 {
        ctx.lines_as(
            "Jorti",
            args![
                "Jorti tries not to cry anymore because it hurts Uncle Kino and then he coughs up more blood.",
                "I miss my mommy a lot, but when I'm worried, it makes Uncle Kino worry a lot too."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jorti",
            args!["My mommy and daddy aren't here anymore, and I don't want Uncle Jorti to go away either."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jorti",
            args!["Um, I don't remember my daddy, but mommy lives far away, somewhere near the sky. That's what Uncle Kino says."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jorti",
            args!["U-Uncle Kino says we can't go see her yet because there are no flying boats yet..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Mysterious Bard",
            args![
                "Even in the sandy wind,",
                "Even in the pouring rain,",
                "Even in the falling snow,",
                "I know it's--",
                "........"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Jorti", args!["Uncle...?", "Are you okay?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Mysterious Bard",
            args![
                "Hmmm...",
                "I'm sorry, but this is a private performance. This song is only intended for my little Jorti."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn little_girl_jorti(ctx: &Ctx) -> Script {
    little_girl_jorti_body(ctx, Vec::new()).map(|_| ())
}

fn old_man_bq1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("gef_bard_q").get()?.number()? > 11 && ctx.var("gef_bard_q").get()?.number()? < 20) {
        ctx.lines_as("Sketzi Bundin", args!["Well, did you find what you were seeking for? Although all we have are old, dusty books, I hope you come by to visit. And please give my regards to your Bard friends."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("gef_bard_q").get()? == 30 {
        ctx.lines_as("Sketzi Bundin", args!["Interesting...", "You have a black cross seal on your hand. Is Kitty still around? I'm glad to see he's still alive. If you're a friend of his, then you are most welcome here."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("gef_bard_q").get()? == 31 {
        ctx.lines_as("Sketzi Bundin", args!["Well, well, well. You're here again. So, what kind of books are you seeking today? Of course, all we have are old, dusty tomes full of eddas. Hahahaha~"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("gef_bard_q").get()? == 11 {
        ctx.lines_as("Sketzi Bundin", args!["Welcome.", "You must be looking for something special. Well, we have almost every Norse poem, or 'edda.' This is the only place where you can find those kinds of old songs."])?;
        ctx.next()?;
        ctx.lines_as("Sketzi Bundin", args!["However, I cannot show these fragile books to just anybody. For the sake of preservation, I can only show these works to preferred customers."])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Show him your left hand.:Show him your right hand.")],
        )?) == 1
        {
            ctx.lines_as(
                "Sketzi Bundin",
                args!["Ah~! You must be the friend of a high ranking Bard! I see, I see. You must be a friend of Minty Errende."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sketzi Bundin",
                args!["So, what is it that you're looking for? Well, I suppose I don't really need to ask that. Hahahaha~"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sketzi Bundin",
                args!["Please...", "Take your time.", "I hope you find", "what you're", "looking for."],
            )?;
            ctx.var("gef_bard_q").set(Val::from(12))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Sketzi Bundin", args!["Let's see... Okay.", "Well, your heartbeat is a little faster than normal. You might want to look into that. You know, for the sake of your health?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Sketzi Bundin",
                args!["You're not looking for any medical or health related books, are you? I'm sorry, but we don't carry any of those."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Sketzi Bundin",
            args![
                "Welcome.",
                "You must be looking for something special and rare. But we only carry one kind of book around here."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sketzi Bundin",
            args![
                "If you're looking for monster information, why don't you check the Pronrera Library or the Monster Museum here in Juno?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn old_man_bq1(ctx: &Ctx) -> Script {
    old_man_bq1_body(ctx, Vec::new()).map(|_| ())
}

fn old_book_bq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("@name$").set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
    if (ctx.var("gef_bard_q").get()?.number()? > 11 && ctx.var("gef_bard_q").get()?.number()? < 14) {
        if !(ctx.call(Function::Rand, vec![Val::from(5)])?.is_true()) {
            ctx.mes("^3355FFYou opened the book. There's a crisp brittleness to the pages, and the letters are faded and barely readable. You can't even identify the author's name.^000000")?;
            ctx.next()?;
            ctx.lines_as(
                "Collection of Eddas",
                args!["This is a love song. Everyone suffers from unrequited love at least once in their lifetime."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Collection of Eddas",
                args![
                    "At One, I fall in love.",
                    "At Two, you give me your smile.",
                    "At Three, I adore your touch.",
                    "At Four, a tender kiss..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Collection of Eddas",
                args![
                    "At Five, we change our minds.",
                    "A petal scatters through the air.",
                    "At Six, I fall in love~",
                    "At Seven, you fall in love..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.var("@name$").get()?,
                args!["I think this is it...!", "I better write it down."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Collection of Eddas",
                args![
                    "At Eight, we're both in love.",
                    "At Nine, you know my heart.",
                    "At Ten, I know you've",
                    "been waiting for me."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Collection of Eddas",
                args![
                    "At Eleven, a precious",
                    "Whisper: 'Will you marry me?'",
                    "At Twelve, our two hearts",
                    "Are one. 12 petals, our",
                    "Love finally blossoms."
                ],
            )?;
            ctx.next()?;
            ctx.mes("^3355FFYou copy down the final lines, and keep them in a note inside your pocket.")?;
            ctx.var("gef_bard_q").set(Val::from(15))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                ctx.var("@name$").get()?,
                args!["*Sigh* I can barely read this book. Maybe I should try flipping through another one."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Sketzi Bundin",
            args!["I'm sorry, but that book is too fragile for handling. Only special customers can peruse these books."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn old_book_bq(ctx: &Ctx) -> Script {
    old_book_bq_body(ctx, Vec::new()).map(|_| ())
}

fn luke_s_songs_vol_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("gef_bard_q").get()?.number()? > 11 {
        ctx.lines_as(
            "Preface",
            args![
                "I, Luke of Izlude, greatest of Bards in my time, leave the lyrics of my essential songs for posterity.",
                " ",
                "Contents"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Wedding Song:Life is a Water Mill:We")])? {
            1 => {
                ctx.lines_as(
                    "Wedding Song",
                    args![
                        "Prontera Sanctuary",
                        "Is where all are heading",
                        "To celebrate your union",
                        "At your most happy wedding."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Wedding Song",
                    args![
                        "With joy I strum my Lute",
                        "As that is my trade",
                        "To bring cheerful song",
                        "To your wedding parade."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Wedding Song",
                    args![
                        "For my songs, pay me",
                        "Not Zeny or Gold,",
                        "But the smile of the",
                        "Bride, if I may",
                        "be so bold."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Wedding Song",
                    args![
                        "The glistening eyes",
                        "Of a beautiful lass",
                        "Those lustrous locks",
                        "And that firm, supple--"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Wedding Song",
                    args![
                        "Forgive me, groom!",
                        "But she's just so beautiful!",
                        "May your union be blessed!",
                        "Er, may your wife be dutiful."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Luke's Note", args!["This song is about a certain Bard who was invited to a wedding ceremony and could not resist the bride's beauty. He ended up singing a song of seduction."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Luke's Note",
                    args!["The lyrics, of course, are fictional, and are in no way anecdotal."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Life is a Water Mill",
                    args![
                        "I chased after fame.",
                        "It eluded me.",
                        "I ran after happiness",
                        "But never caught it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Life is a Water Mill",
                    args![
                        "But tomorrow will still",
                        "Be there, I'm sure.",
                        "Like the Water Mill",
                        "In Al De Baran",
                        "Which turns as",
                        "Life goes on."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Life is a Water Mill",
                    args![
                        "Cheer up! Life goes on.",
                        "As surely as the Water",
                        "Mill turns, tomorrow",
                        "Will come."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Luke's Note",
                    args!["This song was", "made to give comfort", "to people in despair."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    "We",
                    args![
                        "A good Bard sings",
                        "To please his listener.",
                        "So do not expect a sad song",
                        "That deepens your anguish."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "We",
                    args![
                        "A good Dancer dances",
                        "To please her audience.",
                        "Shall we dance together?",
                        "Just hold my hands.",
                        "La la la~ La la la~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "We",
                    args![
                        "Towner: 'Then why the hell do you guys make discords sometimes?!'",
                        "Bard: 'Well... Nobody's perfect!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Luke's Note", args!["This song is good to be sung during festivals, as the intentional discord encourages the audience to participate. Aside from that, this song is pretty meaningless."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines_as(
            "Sketzi Bundin",
            args!["I'm sorry, but that book is not available for public exhibition."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn luke_s_songs_vol_1(ctx: &Ctx) -> Script {
    luke_s_songs_vol_1_body(ctx, Vec::new()).map(|_| ())
}

fn battle_songs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("gef_bard_q").get()?.number()? > 11 {
        ctx.lines_as(
            "Drumming in the battlefield",
            args!["This song was written to give courage to soldiers on the battlefield."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drumming in the battlefield",
            args!["The sounds of galloping", "Echo in the distance.", "A cloud of hazy dust"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drumming in the battlefield",
            args![
                "Fills the setting sun.",
                "Thousands of eyes open",
                "Torches on the castle",
                "Flare like thousands of Ifrits."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drumming in the battlefield",
            args![
                "Hear the throbbing of my heart,",
                "The blood flowing in my veins.",
                "Feeling the heaviness of my armor.",
                "The enemy has appeared before us."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drumming in the battlefield",
            args![
                "Beat the drums hard, harder!",
                "Courage, soldiers, march forward!",
                "Shout loud, soldiers, louder!",
                "Today will never come back!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drumming in the battlefield",
            args![
                "Stun the sky",
                "Provoke the earth,",
                "I feel my heartbeat again.",
                "Blow the bugle to",
                "Sway the fortress.",
                "Today will never come back!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Sketzi Bundin",
            args!["I'm sorry, but that book is not available for public exhibition."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn battle_songs(ctx: &Ctx) -> Script {
    battle_songs_body(ctx, Vec::new()).map(|_| ())
}

fn apple_of_idun_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("gef_bard_q").get()?.number()? > 11 {
        ctx.lines_as("Apple of Idun", args!["This song praises the golden apples of the goddess Idun. These were the source of the God's power, as it prevented them from growing old."])?;
        ctx.next()?;
        ctx.lines_as(
            "Apple of Idun",
            args![
                "Every god never grows old",
                "Because of beautiful",
                "Goddess, Idun.",
                "Keeper of the apples of youth",
                "Goddess of immortality."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Apple of Idun",
            args![
                "Every god never grows old.",
                "Idun, the wife of Bragi,",
                "Idun, Odin's daughter in law~",
                "The apples she keeps",
                "In her basket."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Apple of Idun",
            args!["Without Idun,", "Every god would", "have succumbed to age."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Apple of Idun",
            args![
                "Even Thor, the strongest of gods,",
                "would grow frail, Megingjard would",
                "slip from his waist, and Mjolnir",
                "would never fly again."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Apple of Idun",
            args![
                "Without Idun,",
                "Every god would",
                "have succumbed to age.",
                "Loki was careless once,",
                "and made her lost to the gods.",
                "Loki was forced to get her back."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Apple of Idun",
            args![
                "My goddess stands",
                "In the field of Asgard",
                "She hands me fruit from heaven.",
                "You will be loved by every god...",
                "You will be blessed",
                "By every god..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Apple of Idun",
            args![
                "If you share the",
                "Apple of youth with me",
                "Even a bite of it with",
                "This poor poet."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Apple of Idun",
            args!["You will be loved by every god...", "You will be blessed", "By every god..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Sketzi Bundin",
            args!["I'm sorry, but that book is not available for public exhibition."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn apple_of_idun(ctx: &Ctx) -> Script {
    apple_of_idun_body(ctx, Vec::new()).map(|_| ())
}

fn bard_4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_inputstr_s = Val::from("");
    ctx.var("@name$").set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
    if (ctx.var("gef_bard_q").get()? == 2 || ctx.var("gef_bard_q").get()? == 22) {
        ctx.lines_as("Gunther Doubleharmony", args!["Hahaha~!", "Listen, listen!"])?;
        ctx.next()?;
        ctx.lines_as("Gunther Doubleharmony", args!["I was told this song from one of my friends about this Merchant who lives in Payon and everyone loves this song, especially because I'm singing it and you know that..."])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.var("@name$").get()?,
            args!["Excuse me.", "Are you...", "Gunther Doubleharmony?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gunther Doubleharmony",
            args!["...the Merchant was so poor he didn't even have--eh? Oh yeah, right. That's me. What's up?"],
        )?;
        ctx.next()?;
        ctx.mes("^3355FFYou explain that you have come to him, seeking lost song lyrics.^000000")?;
        ctx.next()?;
        ctx.lines_as(
            "Gunther Doubleharmony",
            args!["Ahhhhhhh, I see.", "Now, what was the", "name of the song again?"],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_inputstr_s = input;
        if l_inputstr_s.clone() == "At One, I Fall in Love" {
            ctx.lines_as(
                "Gunther Doubleharmony",
                args!["Ah~ that song...?", "By the way, who asked you", "to find out about the song?"],
            )?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_inputstr_s = input;
            if l_inputstr_s.clone() == "Minty Errende" {
                ctx.lines_as("Gunther Doubleharmony", args!["Yes, that's my friend! Minty Errende! We used to so close to each other, so I'll write every word of the song for my friend Minty Errende, so turn around please!"])?;
                ctx.next()?;
                ctx.mes("^3355FFGunther furiously scribbled something upon your back.^000000")?;
                ctx.next()?;
                ctx.lines_as("Gunther Doubleharmony", args!["There you go! Now you can go back to Minty Errende and show him your back and he will see what I wrote and then remember the lyrics!"])?;
                if ctx.var("gef_bard_q").get()? == 2 {
                    ctx.var("gef_bard_q").set(Val::from(3))?;
                }
                if ctx.var("gef_bard_q").get()? == 22 {
                    ctx.var("gef_bard_q").set(Val::from(23))?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as("Gunther Doubleharmony", args!["No no no, I don't know anyone with that name, so you better go and check the name of the person asking again, okay?"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as("Gunther Doubleharmony", args!["No no no no, I don't know any song with that title, it might not even exist, so you should go and check the name of the song again and tell me, okay?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if (ctx.var("gef_bard_q").get()? == 6 || ctx.var("gef_bard_q").get()? == 26) {
        ctx.lines_as(
            "Gunther Doubleharmony",
            args![
                "You came back again!",
                "Huh, the song I wrote on your back? Hold on, hold on, let me think let me--ah, right, I got it!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Gunther Doubleharmony", args!["The words I wrote down on your back were written by Mr. Kitty, my idol, my hero! That song is the art of ^228B22Kino Kitty^000000!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Gunther Doubleharmony",
            args!["I wish that someday I could be as great a Bard as him!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Gunther Doubleharmony", args!["Hahaha~!", "Listen, listen!"])?;
        ctx.next()?;
        ctx.lines_as("Gunther Doubleharmony", args!["I was told this song from one of my friends about this Merchant who lives in Payon and everyone loves this song, especially because I'm singing it and you know that..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Gunther Doubleharmony",
            args![
                "Oh, right...!",
                "Do you wanna listen",
                "to my song or a story?",
                "I know you want to!",
                "Right, right?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Show some interest.:Ignore him.")])?) == 1 {
            ctx.lines_as(
                "Gunther Doubleharmony",
                args!["Yay~! I knew it!", "So you wanna hear", "a song or a story?"],
            )?;
            ctx.next()?;
            'b1: {
                let subject1 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("A song:A story:Maybe some other time")],
                )?);
                let mut matched1 = false;
                let no_case1 = !subject1.loosely_equals(&Val::from(1))
                    && !subject1.loosely_equals(&Val::from(2))
                    && !subject1.loosely_equals(&Val::from(3));
                if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                    matched1 = true;
                }
                if matched1 {
                    let subject2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                    if subject2 == 1 {
                        ctx.lines_as(
                            "Gunther Doubleharmony",
                            args![
                                "Gunther sings!",
                                "Gunther dances!",
                                "The tile of this song is~",
                                "'The Rich Mr. Kim~!'"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^228B22Merchant of Payon",
                            "So poooooooor~",
                            "No money for armor",
                            "No money to make."
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^228B22Sold the",
                            "Cotton Shirt",
                            "Off his back",
                            "No pity he'll take.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^228B22First he only sold Red Pots",
                            "At first, he only sold red pots.",
                            "Then he moved up to Carrots, whoohoo~",
                            "He could afford new armor",
                            "and even wear it,",
                            "whoohoo~^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^228B22But one day, he was scammed",
                            "Scammed by a wicked guild.",
                            "Made poor once again.",
                            "He decided to go to Ant Hell",
                            "Right there",
                            "And right then."
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^228B22Bats, Dwarves, Eggs, Ants!",
                            "He battled them all~",
                            "Worm Peelings, Jellopy!",
                            "He gathered loot great and small."
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^228B22Then the glorious day came",
                            "When he found a valuable card",
                            "That'd bring great wealth to his naaaame~^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^228B22But he kept it dear to him",
                            "To remember his times of",
                            "working so hard.",
                            "He never sold it, never sold",
                            "his precious card~^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as("Gunther Doubleharmony", args!["That's a very old story about rich Mr. Kim, and his rise from rags to riches to rags to riches. Is it true or is it fiction? Oh, please don't ask me! I've no clue!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if subject2 == 2 {
                        ctx.lines_as("Gunther Doubleharmony", args!["*Ahem*", "Gunther sings ", "of Yggdrasil~"])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^228B22Evergreen Yggdrasil~",
                            "Giant ashen tree",
                            "reaching for the sky.",
                            "Crystal, morning dew",
                            "From its leaves",
                            "Formed Urd's Pond.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^228B22Three wise girls.",
                            "Seated beneath its boughs.",
                            "Urd of the past,",
                            "Belldandy of the present",
                            "Skuld the future.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^228B22Spinning, weaving",
                            "Threads of destiny.",
                            "Evergreen Yggdrasil~",
                            "Giant ashen tree",
                            "reaching for the sky.",
                            "Its roots soaked with tears.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^228B22Death in Hvergelmir.",
                            "An evil dragon",
                            "Burning its roots",
                            "With eternal flame.",
                            "The evil dragon Nidhogg",
                            "Living between Yggdrasil",
                            "and Niffheim.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^228B22Evergreen Yggdrasil~",
                            "Giant ashen tree",
                            "reaching for the sky.",
                            "Wisdom in its roots",
                            "Roots reaching",
                            "Mimir's pond.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^228B22Guarded by a wise giant.",
                            "Odin sacrificed one",
                            "of his eyes for the wisdom.",
                            "Heimdall's horn hidden",
                            "In Yggdrasil's roots",
                            "Will sound one last time",
                            "Signaling Ragnarok.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Gunther Doubleharmony",
                            args![
                                "This is a very old story...",
                                "Is it truth or fiction? But please don't ask me, I have no idea~!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if subject2 == 3 {
                        ctx.lines_as("Gunther Doubleharmony", args!["I will sing one of Luke's songs, you know, Luke, one of the greatest Bards of his time? But I changed the words a little bit."])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^228B22I found it in a drawer.",
                            "Old, worn letters",
                            "Forming elaborate words.",
                            "Sincere reflection",
                            "Of a sincere mind.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^228B22I found it in a drawer.",
                            "Was I really like that once?",
                            "Was I really that childish?",
                            "My memories are tarnished."
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^228B22I found it in a drawer.",
                            "Love I had forgotten.",
                            "She never got this letter.",
                            "But both of us were too shy."
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^228B22I found it in a drawer.",
                            "Love I had forgotten.",
                            "I never gave her this letter.",
                            "But both of us were too proud."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as("Gunther Doubleharmony", args!["Do you have anyone in mind? Do you? If you ever write a love letter, you must send it and express yourself."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Gunther Doubleharmony",
                            args!["If you've written love letters that you'll never send, throw them away. Throw your goddamn pride away."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                    matched1 = true;
                }
                if matched1 {
                    let subject3 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                    if subject3 == 1 {
                        ctx.lines_as("Gunther Doubleharmony", args!["Um, have you ever", "tasted Comodo cheese?"])?;
                        ctx.next()?;
                        ctx.lines_as("Gunther Doubleharmony", args!["You can only taste it in Comodo, but you need to have a good strong stomach to digest it. Oh! And the cheese has a secret!"])?;
                        ctx.next()?;
                        ctx.lines_as("Gunther Doubleharmony", args!["You ^228B22might^000000 be invulnerable to the power of the doomed swords, which come from the other world, if you eat it!"])?;
                        ctx.next()?;
                        ctx.lines_as("Gunther Doubleharmony", args!["Why don't you go taste it if you haven't yet? I tried to taste it once. It was kind of yummy, but then I fainted. Hahahaha~!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if subject3 == 2 {
                        ctx.lines_as(
                            "Gunther Doubleharmony",
                            args!["I was passing Prontera the other day at the place where it used to be the Swordman training ground."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Gunther Doubleharmony", args!["I saw some kid training really really hard and he didn't notice me watching him, so I guess he was really really serious!"])?;
                        ctx.next()?;
                        ctx.lines_as("Gunther Doubleharmony", args!["He looked like he wanted to be a professional Swordman, but he was also giving his gear away to other Novices."])?;
                        ctx.next()?;
                        ctx.lines_as("Gunther Doubleharmony", args!["I got bored watching him do the same thing over and over and over again, but I think the Monster Research Organization would like him if I introduced him."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if subject3 == 3 {
                        ctx.lines_as(
                            "Gunther Doubleharmony",
                            args!["Have you ever been in Lutie,", "land of year round snow?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Gunther Doubleharmony", args!["There is a snowman named", "SnowySnow and if you met him, you'd know all sorts of things about him like he can talk! It's so strange and mysterious~!"])?;
                        ctx.next()?;
                        ctx.lines_as("Gunther Doubleharmony", args!["He has a mysterious bag where endless gifts come out, and he's got a mysterious past involving some colder town and something about a nasty witch."])?;
                        ctx.next()?;
                        ctx.lines_as("Gunther Doubleharmony", args!["But it's okay because he was rescued by some Alchemist and came back to life, but you should go to Lutie if you wanna know more about him, okay?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as("Gunther Doubleharmony", args!["You're gonna leave right now and not even listen to me a little bit? Okay, I'm cool, but promise you'll come back and listen to just one of my songs, okay?"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        } else {
            ctx.lines_as(ctx.var("@name$").get()?, args!["..."])?;
            ctx.next()?;
            ctx.lines_as("Gunther Doubleharmony", args!["Wow, you're ignoring me, huh? Alright, that's fine by me! Unless you have some kind of problem where you can't talk, then I'm really really sorry."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn bard_4(ctx: &Ctx) -> Script {
    bard_4_body(ctx, Vec::new()).map(|_| ())
}

fn representative_bq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_inputstr_s = Val::from("");
    ctx.mes("[Marlin Putiur]")?;
    if ctx.var("god_brising").get()? == 3 {
        ctx.lines(args!["Welcome to the", "Monster Research", "Organization."])?;
        ctx.next()?;
        ctx.lines_as("Marlin Putiur", args!["We are researching monsters based upon information from adventurers in order to efficiently cope with monsters out in the wild."])?;
        ctx.next()?;
        ctx.lines_as(
            "Marlin Putiur",
            args![
                "We're accepting any kind of information related to monsters,",
                "so if you have any news or info, please don't hesitate to submit it to me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marlin Putiur",
            args!["You can use the report documentation form in", "this room at your convenience."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marlin Putiur",
            args!["Eh...?", "Did you just say", "you're looking for", "someone? Well..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marlin Putiur",
            args!["Well, if he's a registered", "member of this organization,", "I can help you."],
        )?;
        ctx.next()?;
        ctx.lines_as("Marlin Putiur", args!["What was the name?", "Hermite Charles...?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Marlin Putiur",
            args![
                "Oh, I found him.",
                "Errr, but he didn't submit all of the required information when he applied for membership..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marlin Putiur",
            args![
                "Oh. He's a Rogue.",
                "Well, that explains everything. Anyway, did you want to see all",
                "the information we have?"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args!["^3355FFMarlin shows you", "the membership", "application card.^000000"])?;
        ctx.next()?;
        ctx.lines_as(
            "Membership Card",
            args![
                "Name: Hermite Charles",
                "Job: Rogue",
                "Sex: Rogue",
                "Address: Yo Mama Street, Prontera"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marlin Putiur",
            args!["Unfortunately, we don't have any more information. But I hope this will be helpful to you in some way."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marlin Putiur",
            args![
                "Still, it may be a good idea to visit the Rogue Guild. Perhaps",
                "they can help you."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args!["Welcome to the", "Monster Research", "Organization."])?;
    ctx.next()?;
    ctx.lines_as(
        "Marlin Putiur",
        args![
            "We are researching monsters based on information from adventurers in order to efficiently cope with monsters out in the wild."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Marlin Putiur", args!["We're accepting any kind of information related to monsters, so if you have any news or info, please don't hesitate to submit it to me."])?;
    ctx.next()?;
    ctx.lines_as(
        "Marlin Putiur",
        args!["You can use the report documentation form in this room at your convenience."],
    )?;
    if (ctx.var("gef_bard_q").get()? == 4 || ctx.var("gef_bard_q").get()? == 24) {
        ctx.next()?;
        ctx.lines_as(
            "Marlin Putiur",
            args!["Hmmm?", "Adventurers of this organization? Ah, you say you're looking for a Bard?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marlin Putuir",
            args!["Yes, we have a few Bard members who regularly send us information related to monsters."],
        )?;
        ctx.next()?;
        ctx.lines_as("Marlin Putiur", args!["I also hear the Bards have been helping scholars instill bulletin boards in fields which indicate the location for new adventurers. Would you let me know the full name of the person you're looking for?"])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_inputstr_s = input;
        if l_inputstr_s.clone() == "Minty Errende" {
            ctx.lines_as(
                "Marlin Putiur",
                args![
                    "Minty Errende...",
                    "Oh yes, I remember him. He told me a while ago that he's heading South, and that he's staying in Geffen now."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Marlin Putiur", args!["He's a very kind, friendly person. Errende's always doing his best to provide us with the information we need. When you get a chance, would you please give him my regards?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if l_inputstr_s.clone() == "Kino Kitty" {
            ctx.lines_as("Marlin Putiur", args!["Kino Kitty, Kino Kitty... Oh, here we are. He sent us a letter that says, 'I will stay in the desert until I find my real self.'"])?;
            ctx.next()?;
            ctx.lines_as("Marlin Putiur", args!["He doesn't seem healthy, but I guess he's still traveling. Trying seeking him out in Morocc, and give my regards to him if you get the chance."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if l_inputstr_s.clone() == "Gunther Doubleharmony" {
            ctx.lines_as(
                "Marlin Putiur",
                args![
                    "Ah, are you a friend of Gunther's? Haha, he's a very funny guy, if a little excitable. Let's see, Gunther, Gunther..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Marlin Putiur", args!["Ah, it says here that he wanted to look around Payon and Alberta. So he'll be at one of those places. I'm sorry I can't be more specific."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Marlin Putiur",
                args![
                    ((Val::from(" ") + l_inputstr_s.clone()) + Val::from("...?")),
                    "Ummm hmm...",
                    "I'm sorry, but we don't have any records for that person."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.next()?;
        ctx.lines_as(
            "Marlin Putiur",
            args![
                "We are endeavoring to research monsters in this world with the intent to aid each and every adventurer on their journeys."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn representative_bq(ctx: &Ctx) -> Script {
    representative_bq_body(ctx, Vec::new()).map(|_| ())
}

fn adventurer_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_inputstr_s = Val::from("");
    ctx.mes("...")?;
    ctx.next()?;
    ctx.lines(args!["...", "......."])?;
    ctx.next()?;
    ctx.mes("^3355FF*Scribble Scribble*^000000")?;
    ctx.next()?;
    ctx.lines_as("Energetic Young Man", args!["Err...?", "What is it?"])?;
    ctx.next()?;
    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
        ctx.lines_as("Energetic Young Man", args!["Oh...!", "A beautiful,", "young lady~!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Energetic Young Man",
            args!["Hello there~", "My name is Pane.", "May I ask yours?"],
        )?;
        ctx.var("@name$").set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_inputstr_s = input;
        ctx.next()?;
        ctx.lines_as(
            "Energetic Young Man",
            args!["Oh...", ((Val::from("") + l_inputstr_s.clone()) + Val::from("..."))],
        )?;
        if ctx.var("@name$").get()?.loosely_equals(&l_inputstr_s.clone()) {
            ctx.next()?;
            ctx.lines_as(
                "Energetic Young Man",
                args![
                    ((Val::from("^FF6699") + l_inputstr_s.clone()) + Val::from("!")),
                    "Such a wonderful name!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Energetic Young Man", args!["I shall remember your name, my lady. Oh, but I'm so sorry. I'm kind of busy right now. Would you come back later? I'll do my best to please you next time."])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CHUP")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.mes("That's your name?")?;
            ctx.next()?;
            ctx.lines_as("Energetic Young Man", args!["Eh, whatever. Oh, but I'm so sorry. I'm kind of busy right now. Would you come back later? I'll do my best to please you next time."])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Energetic Young Man",
            args!["What, man...!", "Leave me alone.", "Can't you see I'm busy?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn adventurer_1(ctx: &Ctx) -> Script {
    adventurer_1_body(ctx, Vec::new()).map(|_| ())
}
