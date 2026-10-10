#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants, runtime};

pub fn canell_qsk_dan01(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_DANCER && ctx.var("JobLevel").get()?.number()? > 39 {
        if ctx.var("dancer_sk").get()? == 0 {
            ctx.lines_as(
                "Canell",
                args![
                    "It's well known that we must",
                    "be beautiful to captivate those",
                    "who watch us dance. But to rely",
                    "on just our outward appearance",
                    "is a ghastly waste of our full",
                    "potential as performers."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Canell",
                args![
                    "Yes, I can tell that you",
                    "don't quite understand that",
                    "the correct frame of mind is",
                    "an essential element to true",
                    "beauty. Now tell me, are you",
                    "confident in your dancing...?"
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Of course, old crone!", "I... I'm not sure..."])? == 0 {
                ctx.lines_as(
                    "Canell",
                    args![
                        "Ho-ho~ I believe you've",
                        "confused confidence with",
                        "arrogance! The naive cannot",
                        "tell there's a vitally important distinction between the two.",
                        "Now... Prepare for punishment!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(ctx.player().name()?, args!["Ow...!", "Th-that whip!", "I-i-it huuuurts!"])?;
                ctx.call(Function::SpecialEffect, args![constants::EF_HIT2])?;
                ctx.call(Function::PercentHeal, args![-5, 0])?;
                return ctx.close();
            }
            ctx.lines_as(
                "Canell",
                args![
                    "Awareness of your own flaws",
                    "and faults is the first crucial",
                    "step towards improvement.",
                    "Yet your obvious neglect of",
                    "your gift of dancing is...",
                    "grounds for punishment!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args!["What the...? Ack!", "S-stop wh-whipping", "me! It... It stiiings!"],
            )?;
            ctx.call(Function::SpecialEffect, args![constants::EF_HIT2])?;
            ctx.call(Function::PercentHeal, args![-10, 0])?;
            ctx.next()?;
            ctx.lines_as(
                "Canell",
                args![
                    "Ho-ho~! Fortunately for",
                    "you, I will teach you what",
                    "you must learn to rise above",
                    "your current limitations. When",
                    "I'm finished with you, angels and demons won't resist your charms~"
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["No way, you're not teaching me!", "Alright, I'll see what I can learn."])? == 0 {
                ctx.lines_as(
                    "Canell",
                    args![
                        "So... You still haven't",
                        "mastered the fine art of",
                        "exuding charm and humility!",
                        "There must be consequences",
                        "for this blatant affront to our",
                        "profession! Prepare yourself!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.player().name()?,
                    args!["Nooooo--!", "Not that whip", "again! Arrgh, it's--", "It hurts so much!"],
                )?;
                ctx.call(Function::SpecialEffect, args![constants::EF_HIT2])?;
                ctx.call(Function::PercentHeal, args![-10, 0])?;
                return ctx.close();
            }
            ctx.lines_as(
                "Canell",
                args![
                    "Ho-ho~ It's good that you",
                    "recognize that you have much",
                    "to learn. Well, let's not waste",
                    "any time. Your first lesson will be on image training and self",
                    "visualization. Now, for that..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Canell",
                args![
                    "...You must bring",
                    "me ^FF00001 Crystal Mirror^000000!",
                    "This tool is imperative",
                    "to the lesson, so fetch it",
                    "and bring it to me quickly!"
                ],
            )?;
            ctx.var("dancer_sk").set(1)?;
            return ctx.close();
        } else if ctx.var("dancer_sk").get()? == 1 {
            if ctx.items().count(747)? > 0 {
                ctx.lines_as(
                    "Canell",
                    args![
                        "Finally, you've come",
                        "with the mirror. Now,",
                        "look deeply and scrutinize",
                        "your reflection. Gaze upon",
                        "each blemish and fault, each",
                        "charming trait of your face."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Canell",
                    args![
                        "No face is perfect, but",
                        "know that your face is yours",
                        "alone, and that it is what gives you a beauty that can be no",
                        "one else's. Now, repeat",
                        "these truisms after me..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Canell", args!["''I am beautiful...", "I am irresistable...''"])?;
                ctx.next()?;
                ctx.lines_as(ctx.player().name()?, args!["I am beautiful...", "I am irresistable..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Intoxicated Canell",
                    args![
                        "Louder!",
                        "More feeling!",
                        "''I am beautiful!",
                        "I am irresistable!",
                        "I am the most attractive",
                        "woman in the whole world!''"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.player().name()?,
                    args![
                        "I am beautiful!",
                        "I am irresistable!",
                        "I am the most attractive",
                        "woman in the whole world!"
                    ],
                )?;
                ctx.call(Function::SpecialEffect, args![constants::EF_FLASHER])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFYour self esteem has",
                    "sky rocketed. Fortunately,",
                    "you've managed to avoid",
                    "becoming a prima donna.^000000"
                ])?;
                ctx.items().take(747, 1)?;
                ctx.var("dancer_sk").set(2)?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "Frustrated Canell",
                    args![
                        "Did I tell you to come",
                        "back here without bringing",
                        "^FF00001 Crystal Mirror^000000!? Now go",
                        "and get it before I find",
                        "reason to punish you!"
                    ],
                )?;
                ctx.call(Function::NpcSpecialEffect, args![constants::EF_CLAYMORE])?;
                return ctx.close();
            }
        } else if ctx.var("dancer_sk").get()? == 2 {
            ctx.lines_as(
                "Canell",
                args![
                    "Yes, the first and most",
                    "important step to becoming",
                    "beautiful is to realize and",
                    "accept your gorgeousness.",
                    "Confidence first, then beauty.",
                    "Never the other way around."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Canell",
                args![
                    "Now that you've established",
                    "the proper attitude on beauty,",
                    "it's time for you to learn an",
                    "advanced technique of- shall",
                    "we say- enticement. I know an",
                    "expert that I highly recommend."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Canell",
                args![
                    "Go and find my sister",
                    "in Prontera Chruch. If you",
                    "mention me by name, she",
                    "should be willing to teach",
                    "you. I'd do it myself, but I need to enjoy my new Crystal Mirror~"
                ],
            )?;
            ctx.next()?;
            ctx.lines(args!["^3355FFIt figures that she'd", "keep your mirror.^000000"])?;
            ctx.call(
                Function::Emotion,
                args![constants::ET_ANGER, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
            )?;
            ctx.var("dancer_sk").set(3)?;
            return ctx.close();
        } else if ctx.var("dancer_sk").get()?.number()? > 2 && ctx.var("dancer_sk").get()?.number()? < 7 {
            ctx.lines_as(
                "Canell",
                args![
                    "Ah, such a glamorous",
                    "face and figure. My",
                    "beauty is incompara--",
                    "Oh! It's you. So did my",
                    "sister in Prontera Church",
                    "teach you anything yet?"
                ],
            )?;
            ctx.call(Function::Emotion, args![constants::ET_GO])?;
            return ctx.close();
        } else if ctx.var("dancer_sk").get()? == 7 {
            ctx.lines_as(
                "Canell",
                args![
                    "Ah, so were you",
                    "able to find Aelle?",
                    "Were you able to learn",
                    "anything from her?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "Actually, I only learned",
                    "about winks, but not so",
                    "much as how to do them.",
                    "She... She got drunk and",
                    "couldn't teach me more."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Canell",
                args![
                    "Oh, I should have known",
                    "this would happen. Oh,",
                    "well, I guess there's no way",
                    "around it now. But you should",
                    "know that gettng my sister",
                    "drunk... is punishable."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Canell", args!["Bam!"])?;
            ctx.call(Function::SpecialEffect, args![constants::EF_HIT2])?;
            ctx.call(Function::PercentHeal, args![-5, 0])?;
            ctx.next()?;
            ctx.lines_as(ctx.player().name()?, args!["?!", "What the", "hell was that?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Canell",
                args![
                    "Now, a true Dancer should",
                    "be able to figure out the",
                    "secret to winking on her",
                    "own. But since I'm such",
                    "a kind woman, I will deign",
                    "to explain it to you. Ho-ho~"
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFKind woman...?",
                "But she just hit you!",
                "Lightly, of course, but",
                "still, a smack is a smack.^000000"
            ])?;
            ctx.var("dancer_sk").set(8)?;
            return ctx.close();
        } else if ctx.var("dancer_sk").get()? == 8 {
            ctx.lines_as(
                "Canell",
                args![
                    "Alright, the secret to",
                    "proper winking is to allow",
                    "your eyelid to seductively",
                    "move to a natural rhythm.",
                    "Close your eye for a second, then open it slowly to this count."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Canell",
                args![
                    "Un, deux, trois~",
                    "Look at me, do it",
                    "like this. Un, deux, trois~",
                    "Now, let me see you try it."
                ],
            )?;
            ctx.call(Function::Emotion, args![constants::ET_BEST])?;
            ctx.next()?;
            if ctx.menu(&["Un deux trois~-", "Un, doux trois~", "Un, deux, trois~"])? == 2 {
                ctx.lines_as(ctx.player().name()?, args!["Un, deux, trois~"])?;
                ctx.call(
                    Function::Emotion,
                    args![constants::ET_BEST, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
                )?;
                ctx.call(Function::SpecialEffect, args![constants::EF_FLASHER])?;
                ctx.next()?;
                ctx.lines_as(
                    "Canell",
                    args![
                        "Great, that's exactly",
                        "how you do it. Now,",
                        "don't forget, the elements",
                        "of rhythm and naturalness",
                        "are absolutely essential to",
                        "this technique of enticement."
                    ],
                )?;
                ctx.call(Function::Emotion, args![constants::ET_CHUP])?;
                ctx.next()?;
                ctx.lines_as(
                    "Canell",
                    args![
                        "Now that you've mastered",
                        "the art of winking, you.",
                        "should know that you cannot",
                        "allure those who are much",
                        "stronger than you. So your",
                        "winks won't work on everyone."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Canell",
                    args![
                        "Of course, my winks don't",
                        "have that drawback. Ho-ho~",
                        "Anyway, it's time for us to part now. I hope that you grow to",
                        "become a more glamorous and charming dancer in days to come."
                    ],
                )?;
                ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
                ctx.next()?;
                ctx.lines(args!["^3355FFYou have finally learned", "the Charming Wink skill.^000000"])?;
                ctx.var("dancer_sk").set(9)?;
                ctx.call(Function::SpecialEffect, args![constants::EF_ABSORBSPIRITS])?;
                ctx.call(Function::Skill, args!["DC_WINKCHARM", 1, constants::SKILL_PERM])?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "Canell",
                    args!["No, no!", "That's wrong,", "completely wrong!", "Can't you do it right?!"],
                )?;
                ctx.next()?;
                ctx.lines_as(ctx.player().name()?, args!["O-ow!", "P-please...!", "Not the whip again!"])?;
                ctx.call(Function::SpecialEffect, args![constants::EF_HIT2])?;
                ctx.call(Function::PercentHeal, args![-5, 0])?;
                return ctx.close();
            }
        } else if ctx.var("dancer_sk").get()?.number()? > 8 && ctx.call(Function::GetSkillLv, args!["DC_WINKCHARM"])?.is_true() {
            ctx.lines_as(
                "Canell",
                args![
                    "Hm...?",
                    "Is there anything",
                    "more you wanted to",
                    "ask me? Ah, you must",
                    "be mesmerized by my",
                    "beauty, aren't you?"
                ],
            )?;
            ctx.call(Function::Emotion, args![constants::ET_HNG])?;
            ctx.next()?;
            ctx.lines_as(
                "Canell",
                args![
                    "Oh, to be so beautiful",
                    "must be a sin! I should",
                    "work in Prontera Church,",
                    "just like my sister, to",
                    "pray for forgiveness~"
                ],
            )?;
            return ctx.close();
        } else if ctx.var("dancer_sk").get()? == 9 && ctx.call(Function::GetSkillLv, args!["DC_WINKCHARM"])? == 0 {
            ctx.lines_as(
                "Canell",
                args![
                    "Hm? Oh, I remember you!",
                    "A Gypsy now, I see~ That",
                    "look suits you. Now, I'm",
                    "sure you've forgotten a few",
                    "things since transcending,",
                    "am I right? Watch this wink..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Canell",
                args![
                    "Remember now? It's all",
                    "in the natural, seductive",
                    "and rhythmic movement of",
                    "the eyelid. I hope you use",
                    "your Charming Wink to let",
                    "your inner beauty shine~"
                ],
            )?;
            ctx.next()?;
            ctx.lines(args!["^3355FFYou have learned the", "Charming Wink skill.^000000"])?;
            ctx.call(Function::SpecialEffect, args![constants::EF_ABSORBSPIRITS])?;
            ctx.call(Function::Skill, args!["DC_WINKCHARM", 1, constants::SKILL_PERM])?;
            return ctx.close();
        }
    } else if ctx.var("BaseJob").get()? == constants::JOB_DANCER && ctx.var("JobLevel").get()?.number()? < 40 {
        ctx.lines_as(
            "Canell",
            args![
                "Oh, you're such an",
                "adorable little girl! Ah,",
                "I'm sorry, but when you",
                "become a lady at, oh, I don't",
                "know, Job Level 40, then I'll",
                "be able to teach you something~"
            ],
        )?;
        return ctx.close();
    } else if ctx.var("BaseJob").get()? != constants::JOB_DANCER {
        if ctx.var("Sex").get()? == constants::SEX_FEMALE {
            ctx.lines_as(
                "Canell",
                args![
                    "Oh, what a cute",
                    "little girl~! Such",
                    "a chubby lil' belly,",
                    "so cuuuuuuuuuuute~"
                ],
            )?;
            return ctx.close();
        } else {
            ctx.lines_as("Canell", args!["Hey there,", "handsome~", "^333333*Wink~*^000000"])?;
            ctx.call(Function::Emotion, args![constants::ET_CHUP])?;
            ctx.call(Function::SpecialEffect, args![constants::EF_VALLENTINE2])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFStrangely enough,",
                "her wink has made",
                "your heart throb.^000000"
            ])?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn aelle_qsk_dan02(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_DANCER {
        if ctx.var("dancer_sk").get()? == 3 {
            ctx.lines_as(
                "Aelle",
                args![
                    "Pssst, hey! I got",
                    "some cheap, but slightly",
                    "illegal, warps to the Orc",
                    "Dungeon, Glast Heim",
                    "and the Dead Pit. So",
                    "you want in on this?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "Actually, um, I'm here",
                    "to look for somebody.",
                    "You wouldn't happen",
                    "to know Canelle's",
                    "sister, would you?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aelle",
                args![
                    "Ah, so my stuck up",
                    "sister sent you, huh?",
                    "I should have known,",
                    "judging from your clothes.",
                    "So you want to become a",
                    "more alluring performer, eh?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aelle",
                args![
                    "Awww...",
                    "That's disappointing.",
                    "So... You didn't come",
                    "here to buy these warps?",
                    "You sure you don't want any?"
                ],
            )?;
            ctx.var("dancer_sk").set(4)?;
            return ctx.close();
        } else if ctx.var("dancer_sk").get()? == 4 {
            ctx.lines_as(
                "Aelle",
                args![
                    "Okay, okay...",
                    "So you came for me to",
                    "teach you some enticement",
                    "technique or something, right?",
                    "Huh. Well, this lecture I give?",
                    "It's long and complicated."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aelle",
                args![
                    "Talking and explaining",
                    "all of the intricate details",
                    "for so long will definitely",
                    "parch my throat. So first, you",
                    "gotta bring me a refreshing",
                    "drink before we can begin..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "Oh... Okay.",
                    "Alright, I guess",
                    "I can spare a potion",
                    "or two, or maybe bring",
                    "back a bottle of juic--"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aelle",
                args![
                    "Whoa, whoa, none of",
                    "that junk! I want to",
                    "have a real drink.",
                    "You know, something",
                    "more... ^FF0000Alcohol^000000ic."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aelle",
                args![
                    "Oh, and I need some",
                    "snacks to sustain all of",
                    "my teaching energy! Yeah,",
                    "some yummy fruit on a plate",
                    "and some cookies too!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aelle",
                args![
                    "Soooo, I wanna eat",
                    "^FF00001 Apple^000000, ^FF00003 Bananas^000000 and",
                    "^FF00005 Well-Baked Cookies^000000. Ah,",
                    "make sure you bring ^FF0000China^000000",
                    "to serve them on, and don't",
                    "forget the ^FF0000Alcohol^000000, okay?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aelle",
                args![
                    "One last thing...!",
                    "Don't let anyone here",
                    "in the church know what",
                    "we're doing. I don't wanna",
                    "get in too much trouble!"
                ],
            )?;
            ctx.var("dancer_sk").set(5)?;
            return ctx.close();
        } else if ctx.var("dancer_sk").get()? == 5 {
            if ctx.items().count(970)? > 0
                && ctx.items().count(512)? > 0
                && ctx.items().count(513)? > 2
                && ctx.items().count(538)? > 4
                && ctx.items().count(736)? > 0
            {
                ctx.lines_as(
                    "Aelle",
                    args![
                        "Great, you've brought",
                        "the food! It's a good thing",
                        "I'm famished because",
                        "I'm going to finish it all!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFAelle devoured the",
                    "food like a ravenous,",
                    "hungry beast that had",
                    "been deprived for too long.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Aelle",
                    args![
                        "That was sooo good!",
                        "I haven't eaten so much",
                        "in such a long time! Then",
                        "again, you don't get many",
                        "chances to pig out in church.",
                        "Ah, that's right, the lesson!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aelle",
                    args![
                        "Now, the easiest, most",
                        "subtle and most appealing",
                        "gesture of all time is the",
                        "^FF0000wink^000000. It's saved me in times",
                        "of crisis, and mastering it",
                        "will enhance your charms."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aelle",
                    args![
                        "To the untrained eye,",
                        "the wink only looks like",
                        "a simple movement, right?",
                        "You close one eye, and then",
                        "you reopen that eye. But if",
                        "you do it right... If you..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aelle",
                    args![
                        "Oh, it's been so long",
                        "since I've had so much",
                        "to drink! Ugh, f-feeling",
                        "kinda--anyway, j-just close",
                        "and yer-- ^333333*burp*^000000 winking",
                        "is so so so so eeeeeasy~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Drunken Aelle",
                    args![
                        "Do it! Copy me as",
                        "I do it! Wink, j-just",
                        "like this, okay? Y' see?",
                        "^333333*Wiiiiiiiiiiiiiink*^000000"
                    ],
                )?;
                ctx.call(Function::Emotion, args![constants::ET_BEST])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.player().name()?,
                    args![
                        "Wh-whoa...",
                        "That was...",
                        "It's so beautiful!",
                        "And all with just",
                        "a simple wink!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Totally Drunk Aelle",
                    args![
                        "Y-yea... The...",
                        "The secret ish...",
                        "*Urp* It'sh in the--",
                        "Bwahahahaahah!",
                        "It's all spinning!"
                    ],
                )?;
                ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFIn her drunken stupor,",
                    "Aelle pummels you with",
                    "her fists and laughs",
                    "maniacally to herself.^000000"
                ])?;
                ctx.call(Function::PercentHeal, args![-10, 0])?;
                ctx.call(Function::SpecialEffect, args![constants::EF_HIT2])?;
                ctx.call(
                    Function::Emotion,
                    args![constants::ET_THINK, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
                )?;
                ctx.items().take(970, 1)?;
                ctx.items().take(512, 1)?;
                ctx.items().take(513, 3)?;
                ctx.items().take(538, 5)?;
                ctx.items().take(736, 1)?;
                ctx.var("dancer_sk").set(6)?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "Aelle",
                    args![
                        "How can you expect",
                        "me to teach you anything",
                        "if I faint in the middle of",
                        "the lesson? That's right,",
                        "I'll need some nourishing,",
                        "yet delicious, snacks to eat~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aelle",
                    args![
                        "Soooo, I wanna eat",
                        "^FF00001 Apple^000000, ^FF00003 Bananas^000000 and",
                        "^FF00005 Well-Baked Cookies^000000. Ah,",
                        "make sure you bring ^FF0000China^000000",
                        "to serve them on, and don't",
                        "forget the ^FF0000Alcohol^000000, okay?"
                    ],
                )?;
                return ctx.close();
            }
        } else if ctx.var("dancer_sk").get()? == 6 {
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "You still haven't told",
                    "me the secret to making",
                    "a simple wink have so",
                    "much provocative charm.",
                    "I really need to--"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Totally Hammered Aelle",
                args![
                    "ZzzZzz... Huh?",
                    "Yesh, I know, I'm...",
                    "I'm a geeenius, yeah...",
                    "...ZzzzZzzzZzZzzzZZZzz..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args!["W-wake up!", "I went through", "all this trouble to", "learn this technique!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Annoyed Aelle",
                args![
                    "SHADDUP!",
                    "I'M SHLEEEEEPY!",
                    "G-go away. Talk to",
                    "Canell, my sister...",
                    "...ZzzzzZzZZZzzzZZ...."
                ],
            )?;
            ctx.var("dancer_sk").set(7)?;
            ctx.close_window()?;
            ctx.warp("prontera", 156, 272)?;
            return ctx.end();
        } else if ctx.var("dancer_sk").get()?.number()? > 6 && ctx.var("dancer_sk").get()?.number()? < 8 {
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "Aelle, I really",
                    "need you to teach",
                    "me everything else",
                    "I need to know about",
                    "winking and--"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aelle",
                args![
                    "^333333*Hiccup*^000000 Zzz--wha?",
                    "I need ta teach you",
                    "to go'way. Talk to my",
                    "sister, too tired now.",
                    "Go lemme alone girl!"
                ],
            )?;
            ctx.close_window()?;
            ctx.warp("prontera", 156, 272)?;
            return ctx.end();
        } else if ctx.var("dancer_sk").get()?.number()? > 8 {
            ctx.lines_as("Sober Aelle", args!["Ohhh...", "H-headache..."])?;
            ctx.call(Function::Emotion, args![constants::ET_FRET])?;
            ctx.next()?;
            ctx.lines_as(
                "Aelle",
                args![
                    "Oh, wow, look at you!",
                    "I can tell that you've",
                    "become much more",
                    "beautiful. Yes, you're",
                    "more elegant and refined",
                    "than I can remember."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aelle",
                args![
                    "Always believe in your",
                    "beauty, wield your charm",
                    "like a weapon... And anything",
                    "you desire in this world can",
                    "be yours! Bwahahahahaha!"
                ],
            )?;
            return ctx.close();
        } else {
            ctx.lines_as(
                "Aelle",
                args![
                    "Pssst, hey! I got",
                    "some cheap, but slightly",
                    "illegal, warps to the Orc",
                    "Dungeon, Glast Heim",
                    "and the Dead Pit. So",
                    "you want in on this?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "??????",
                args![
                    "Sister Aelle...",
                    "Are you still trying",
                    "to sell illegal warps",
                    "again? Sister Aelle?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Aelle",
                args![
                    "Oh crap, it's the pastor!",
                    "Shhh, if anybody asks you,",
                    "I wasn't doing anything!"
                ],
            )?;
            ctx.call(Function::Emotion, args![constants::ET_HUK])?;
            ctx.next()?;
            ctx.lines_as(
                "Aelle",
                args![
                    "Thanks, hon...",
                    "I know you can",
                    "keep my contraband",
                    "our little secret. ^333333*Wink*^000000"
                ],
            )?;
            ctx.call(Function::Emotion, args![constants::ET_BEST])?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "Aelle",
            args![
                "Pssst, hey! I got",
                "some cheap, but slightly",
                "illegal, warps to the Orc",
                "Dungeon, Glast Heim",
                "and the Dead Pit. So",
                "you want in on this?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "??????",
            args![
                "Sister Aelle...",
                "Are you still trying",
                "to sell illegal warps",
                "again? Sister Aelle?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Aelle",
            args![
                "Oh crap, it's the pastor!",
                "Shhh, if anybody asks you,",
                "I wasn't doing anything!"
            ],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_HUK])?;
        ctx.next()?;
        ctx.lines_as(
            "Aelle",
            args![
                "Thanks, hon...",
                "I know you can",
                "keep my contraband",
                "our little secret. ^333333*Wink*^000000"
            ],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_BEST])?;
        return ctx.close();
    }
}
