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

pub fn young_man_bard_q1(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_BARD {
        ctx.call(Function::Emotion, args![constants::ET_HUK])?;
        ctx.lines_as("Timid Young Man", args!["Eh? Wwwaaaah--!", "Y-you're--it's-it's--"])?;
        if ctx.var("Upper").get()? != 1 {
            ctx.mes("It's a freakin' Bard!")?;
        } else {
            ctx.mes("It's a freakin' Minstrel!")?;
        }
        ctx.lines(args!["D-don't come any closer!", "I... I don't like you guys!"])?;
        ctx.next()?;
        if ctx.menu(&["Wha--? Why the heck not?", "Hey, take it easy, man."])? == 0 {
            ctx.lines_as(
                "Timid Young Man",
                args![
                    "N-no! Don't look at me!",
                    "I know what you're trying",
                    "to do! Please, I haven't",
                    "done anything to you!",
                    "J-just s-stay away!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args!["Um...", "I'm not really trying", "to do anything. Why", "don't you relax, and--"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Timid Young Man",
                args![
                    "Relax?! Nobody believes me",
                    "when I tell them how dangerous",
                    "you guys are. You think you're",
                    "so smug with your funny jokes",
                    "and lovely songs, but I know",
                    "what kind of powers you have!"
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Timid Young Man",
            args![
                "N-no! Don't look at me!",
                "I know what you're trying",
                "to do! Please, I haven't",
                "done anything to you!",
                "J-just s-stay away!"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Offer him a drink.", "Reassure him that you're safe."])? == 0 {
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "Look, I don't know what",
                    "you've got against me, but",
                    "you really need to relax.",
                    "Here, have a drink on me."
                ],
            )?;
            ctx.next()?;
            if ctx.items().count(12112)? > 0 {
                ctx.lines_as(
                    "Timid Young Man",
                    args![
                        "Oh~! Isn't that",
                        "a Tropical Sograt?",
                        "That's my favorite",
                        "drink in all the world!"
                    ],
                )?;
                if ctx.var("qskill_bard").get()? == 9 {
                    ctx.mes("Thanks so--waitaminute.")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Timid Young Man",
                        args![
                            "This is some sort",
                            "of weird trick, isn't it?",
                            "And to think I almost",
                            "f-f-fell for it! P-please",
                            "j-just leave me alone!"
                        ],
                    )?;
                    return ctx.close();
                }
                if ctx.var("qskill_bard").get()?.number()? > 0 {
                    ctx.next()?;
                    ctx.lines_as(
                        "Timid Young Man",
                        args![
                            "Wait, I've seen you before.",
                            "And you brought me a drink",
                            "just like this one. You...",
                            "You d-didn't learn th-that",
                            "w-w-weird skill, d-did you?",
                            "Wait, no. You couldn't have..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.player().name()?,
                        args!["Oh, please~", "You know, I think", "that maybe you", "worry too much."],
                    )?;
                    return ctx.close();
                }
                ctx.next()?;
                ctx.lines_as(
                    "Timid Young Man",
                    args![
                        "^333333*Gulp Gulp*^000000",
                        "Ahhhh~ Oh, you don't",
                        "understand how long I've",
                        "been wanting this drink!",
                        "It tastes so good, and",
                        "now I feel sooo relaxed..."
                    ],
                )?;
                ctx.next()?;
                match ctx.menu(&["See? I'm not dangerous at all~", "So what makes someone like me so scary?"])? {
                    0 => {
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "Hmmm... Maybe.",
                                "Maybe all of you Bards",
                                "and Minstrels aren't that",
                                "bad. But I can never forget",
                                "what that Bard did to me..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "It all started when I was",
                                "traveling through Umbala and",
                                "met a strange Bard who was",
                                "studying under the tutelage",
                                "of Puchuchartan, the Utan",
                                "Shaman of the village."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "That Bard and I got along fairly well until he took me to Umbala's",
                                "Bungee Jump. He insisted that I jump at least once for the ''full",
                                "Umbala experience.'' I refused, seeing as they don't use bungees."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "The Bard seemed offended",
                                "and claimed it was perfectly",
                                "safe, and that only a few people",
                                "have died by jumping. Then, he",
                                "just... He gave me this intense look."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "His eyes seemed so full",
                                "of rage! I remember him",
                                "mumbling something, and",
                                "all of a sudden, I lost control",
                                "of my body! My arms and legs",
                                "were just moving on their own!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "Before long, I found myself",
                                "struggling to keep myself from",
                                "leaping off that Bungee Jump.",
                                "But the more I resisted, the",
                                "more violently I'd flail toward",
                                "the edge. It was horrible!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "That was the most terrifying",
                                "experience of my life! It was",
                                "bad enough that I risked my",
                                "life, but that feeling of not",
                                "having any control over your",
                                "body is so overwhelming!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.player().name()?,
                            args![
                                "Wait, you're saying",
                                "a Bard did this to you?",
                                "I've never heard of a song",
                                "or skill with that sort of effect before. That's really strange..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "Well, I've never heard",
                                "of that sort of power up",
                                "until I had to experience",
                                "it for myself. Oh, I can still",
                                "see that evil smile of his",
                                "in my nightmares..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "Anyway, thanks for that",
                                "drink, it really helped me",
                                "settle my nerves. But I must",
                                "warn you not to look for that",
                                "strange Bard. I'm sure he's really some sort of demon or something..."
                            ],
                        )?;
                        ctx.next()?;
                        if ctx.var("JobLevel").get()?.number()? > 39 {
                            ctx.lines_as(
                                "Timid Young Man",
                                args![
                                    "But... If you really",
                                    "want to attain that sort",
                                    "of power, I can't really",
                                    "stop you. Thankfully, nobody",
                                    "has any idea of where he is~"
                                ],
                            )?;
                            ctx.items().take(12112, 1)?;
                            ctx.var("qskill_bard").set(Val::from(1))?;
                        } else {
                            ctx.lines_as(
                                "Timid Young Man",
                                args![
                                    "Even if you could find that",
                                    "Bard to get him to teach you",
                                    "how he did that to me, I'm sure",
                                    "he mentioned something about",
                                    "being at least ^660000Job Level 40^000000 to",
                                    "be able to handle that power..."
                                ],
                            )?;
                            ctx.items().take(12112, 1)?;
                        }
                        return ctx.close();
                    }
                    1 => {
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "^333333*Sigh*^000000 Well, maybe all Bards",
                                "and Minstrels aren't terrifying. But any Bard will remind me",
                                "of the one that I met during my",
                                "travels. Just thinking about",
                                "that time gives me goosebumps."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "It all started when I was",
                                "traveling through Umbala and",
                                "met a strange Bard who was",
                                "studying under the tutelage",
                                "of Puchuchartan, the Utan",
                                "Shaman of the village."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "That Bard and I got along fairly well until he took me to Umbala's",
                                "Bungee Jump. He insisted that I jump at least once for the ''full",
                                "Umbala experience.'' I refused, seeing as they don't use bungees."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "The Bard seemed offended",
                                "and claimed it was perfectly",
                                "that a few people have died",
                                "by jumping. Then, he just...",
                                "He gave me this intense look."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "His eyes seemed so full",
                                "of rage! I remember him",
                                "mumbling something, and",
                                "all of a sudden, I lost control",
                                "of my body! My arms and legs",
                                "were just moving on their own!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "Before long, I found myself",
                                "struggling to keep myself from",
                                "leaping off that Bungee Jump.",
                                "But the more I resisted, the",
                                "more violently I'd flail toward",
                                "the edge. It was horrible!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "That was the most terrifying",
                                "experience of my life! It was",
                                "bad enough that I risked my",
                                "life, but that feeling of not",
                                "having any control over your",
                                "body is so overwhelming!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.player().name()?,
                            args![
                                "Wait, you're saying",
                                "a Bard did this to you?",
                                "I've never heard of a song",
                                "or skill with that sort of effect before. That's really strange..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "Well, I've never heard",
                                "of that sort of power up",
                                "until I had to experience",
                                "it for myself. Oh, I can still",
                                "see that evil smile of his",
                                "in my nightmares..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Timid Young Man",
                            args![
                                "Anyway, thanks for that",
                                "drink, it really helped me",
                                "settle my nerves. But I must",
                                "warn you not to look for that",
                                "strange Bard. I'm sure he's really some sort of demon or something..."
                            ],
                        )?;
                        ctx.next()?;
                        if ctx.var("JobLevel").get()?.number()? > 39 {
                            ctx.lines_as(
                                "Timid Young Man",
                                args![
                                    "But... If you really",
                                    "want to attain that sort",
                                    "of power, I can't really",
                                    "stop you. Thankfully, nobody",
                                    "has any idea of where he is~"
                                ],
                            )?;
                            ctx.items().take(12112, 1)?;
                            ctx.var("qskill_bard").set(Val::from(1))?;
                        } else {
                            ctx.lines_as(
                                "Timid Young Man",
                                args![
                                    "Even if you could find that",
                                    "Bard to get him to teach you",
                                    "how he did that to me, I'm sure",
                                    "he mentioned something about",
                                    "being at least ^660000Job Level 40^000000 to",
                                    "be able to handle that power..."
                                ],
                            )?;
                            ctx.items().take(12112, 1)?;
                        }
                        return ctx.close();
                    }
                    _ => {}
                }
            } else {
                ctx.lines_as(
                    "Timid Young Man",
                    args![
                        "Eh...?! Um, th-that's",
                        "nice of y-you to offer,",
                        "but I'm p-pretty picky",
                        "about what I d-drink.",
                        "P-plus, I don't k-know",
                        "if I can t-trust you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Timid Young Man",
                    args![
                        "I don't think there's",
                        "much that could get m-me",
                        "to ch-change my m-mind!",
                        "Well... Maybe if you brought",
                        "my favorite drink, Tropical",
                        "Sograt, I would reconsider..."
                    ],
                )?;
                return ctx.close();
            }
        }
        ctx.lines_as(ctx.player().name()?, args!["Don't worry,", "I won't hurt you."])?;
        ctx.next()?;
        ctx.lines_as(
            "Timid Young Man",
            args![
                "Th-that's what th-they",
                "all say, right before they",
                "get into your mind and then",
                "twist it as hard as they can!"
            ],
        )?;
        if ctx.var("qskill_bard").get()? == 9 {
            ctx.lines(args!["J-just don't t-touch me!", "...Ack! And stay away!"])?;
        } else {
            ctx.lines(args!["E-even if you d-don't have", "that p-power, l-leave me alone!"])?;
        }
        return ctx.close();
    }
    ctx.lines_as(
        "Timid Young Man",
        args![
            "Oh... Oh goodness.",
            "Was that a Bard just",
            "over there? Oh, I'm so",
            "afraid of those guys!",
            "And those Minstrels",
            "are even worse!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Timid Young Man",
        args![
            "Don't get me wrong, I love",
            "songs and entertainment,",
            "but you've got to understand!",
            "Some of them have powers",
            "that you wouldn't believe!",
            "I... I've seen them myself!"
        ],
    )?;
    ctx.close()
}

pub fn young_man_bard_q1_ontouch(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_BARD {
        ctx.call(Function::Emotion, args![constants::ET_HUK])?;
    }
    ctx.end()
}

pub fn spiteful_looking_bard_bs(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_BARD {
        if ctx.var("Class").get()? == constants::JOB_CLOWN && ctx.var("qskill_bard").get()? == 9 {
            if ctx.call(Function::GetSkillLv, args!["BA_PANGVOICE"])? != 0 {
                ctx.lines_as(
                    "Riott",
                    args![
                        "Geh heh heh~",
                        "Been making good use of",
                        "what I taught you? Just be",
                        "careful and don't use that skill recklessly. Otherwise, people",
                        "will hate you as they hate me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "Your enemies, and the",
                        "occasional drunkard, on",
                        "the other hand, are different",
                        "matters entirely! Bwah hah hah!"
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Riott",
                args![
                    "Impossible! You forgot",
                    "everything I've taught you?",
                    "How can that be? Oh well, it's",
                    "no trouble for me to teach that",
                    "to you again if you'd like."
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["No, thanks.", "Thanks, I'd appreciate that."])? == 0 {
                ctx.lines_as(
                    "Riott",
                    args![
                        "What...?",
                        "You really don't",
                        "want to learn it?",
                        "I assure you there's",
                        "no strings attached.",
                        "If you change your mind..."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Riott",
                args![
                    "First, you must stare",
                    "fiercely into the eyes of",
                    "your target, and focus on",
                    "thoughts of dominance. This",
                    "is the basis for mesmerization. Now listen to this incantation..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Riott",
                args![
                    "Uuuummm Baaalaaaa",
                    "Uuuummmm Baaalaaa~",
                    "Kkkkuuurrirrreeee",
                    "Kkkkuuurrirrreeee",
                    "Oooumm guandlejdl",
                    "Woooo Ei ei ei ei......"
                ],
            )?;
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_TALK_FROSTJOKE])?;
            ctx.next()?;
            ctx.lines_as(
                "Riott",
                args![
                    "Pang's Voice is used to",
                    "confuse people and disrupt",
                    "control of their bodies. It's not a fatal skill, but it is effective",
                    "in mentally upsetting your enemy. Make very wise use of this skill."
                ],
            )?;
            ctx.call(Function::Skill, args!["BA_PANGVOICE", 1, constants::SKILL_PERM])?;
            return ctx.close();
        }
        if ctx.var("qskill_bard").get()?.number()? > 8 {
            ctx.lines_as(
                "Riott",
                args![
                    "Geh heh heh~",
                    "Been making good use of",
                    "what I taught you? Just be",
                    "careful and don't use that skill recklessly. Otherwise, people",
                    "will hate you as they hate me."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Riott",
                args![
                    "Your enemies, and the",
                    "occasional drunkard, on",
                    "the other hand, are different",
                    "matters entirely! Bwah hah hah!"
                ],
            )?;
            return ctx.close();
        }
        if ctx.var("qskill_bard").get()? == 8 {
            if ctx.items().count(7277)? > 0 {
                ctx.lines_as(
                    "Riott",
                    args![
                        "Ah, you've brought me",
                        "a Munak Doll made by",
                        "Yao Jun, just like you said",
                        "you would. Ah yes, this is her",
                        "craftsmanship, impeccable",
                        "as always. You've done well~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "To fulfill my part of this",
                        "bargain, I shall now teach",
                        "you my special skill. Now,",
                        "I developed this by listening",
                        "to incantations by the Utan",
                        "Shaman in Umbala."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "First, you must stare",
                        "fiercely into the eyes of",
                        "your target, and focus on",
                        "thoughts of dominance. This",
                        "is the basis for mesmerization. Now listen to this incantation..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "''Toad's leg, Verit's heart,",
                        "spinning stars, spilling zeny,",
                        "hands and feet tied. Is this",
                        "voices yours, is this voice",
                        "mine. Head spinning, head",
                        "spinning, head spinning...!''"
                    ],
                )?;
                ctx.call(Function::SpecialEffect, args![constants::EF_TALK_FROSTJOKE])?;
                ctx.items().take(7277, 1)?;
                ctx.call(Function::Skill, args!["BA_PANGVOICE", 1, constants::SKILL_PERM])?;
                ctx.var("qskill_bard").set(Val::from(9))?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "Remember, it doesn't matter",
                        "what you say, but how you say",
                        "it. Hypnotically induce your",
                        "target with a forbiddenly",
                        "seductive rhythm and your",
                        "grasp will be inescapable!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "Ah, and use sleight of",
                        "hand to distract your target",
                        "from your true motive! I find",
                        "that casting Unbarring Octave",
                        "with this skill works best. This skill's name is ''Pang Voice!''"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "''Pang Voice'' will mentally",
                        "shock your target and disrupt",
                        "control of his own body for",
                        "a while. You can't exert control",
                        "over victims with this skill, but they usually assume otherwise..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "You need to be judicious in",
                        "your use of this skill! Don't",
                        "use it recklessly, or people",
                        "will come to hate you as they",
                        "hate me. But ''Pang Voice'' can be welcome is certain situations."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "Subject your enemies to",
                        "Pang Voice as much as you",
                        "like, and no one will blame",
                        "you for it. And you can get away with casting Pang Voice on bullies",
                        "and drunkards occasionally..."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Riott",
                args![
                    "Hmpf. Weren't able",
                    "to find me a Munak Doll",
                    "yet? Well, don't worry, I'm",
                    "a patient man. Just try to get",
                    "one for me as soon as you can."
                ],
            )?;
            return ctx.close();
        }
        if ctx.var("qskill_bard").get()? == 7 {
            if ctx.items().count(574)? > 4 {
                ctx.lines_as(
                    "Riott",
                    args![
                        "Ah, you've brought me",
                        "some fresh eggs laid by",
                        "Yhelle, just like I asked.",
                        "I'm sure it was dangerous",
                        "going to Niflheim, but the flavor of these eggs is worth it."
                    ],
                )?;
                ctx.next()?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "To fulfill my part of this",
                        "bargain, I shall now teach",
                        "you my special skill. Now,",
                        "I developed this by listening",
                        "to incantations by the Utan",
                        "Shaman in Umbala."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "First, you must stare",
                        "fiercely into the eyes of",
                        "your target, and focus on",
                        "thoughts of dominance. This",
                        "is the basis for mesmerization. Now listen to this incantation..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "''Toad's leg, Verit's heart,",
                        "spinning stars, spilling zeny,",
                        "hands and feet tied. Is this",
                        "voices yours, is this voice",
                        "mine. Head spinning, head",
                        "spinning, head spinning...!''"
                    ],
                )?;
                ctx.call(Function::SpecialEffect, args![constants::EF_TALK_FROSTJOKE])?;
                ctx.items().take(574, 5)?;
                ctx.call(Function::Skill, args!["BA_PANGVOICE", 1, constants::SKILL_PERM])?;
                ctx.var("qskill_bard").set(Val::from(9))?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "Remember, it doesn't matter",
                        "what you say, but how you say",
                        "it. Hypnotically induce your",
                        "target with a forbiddenly",
                        "seductive rhythm and your",
                        "grasp will be inescapable!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "Ah, and use sleight of",
                        "hand to distract your target",
                        "from your true motive! I find",
                        "that casting Unbarring Octave",
                        "with this skill works best. This skill's name is ''Pang Voice!''"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "''Pang Voice'' will mentally",
                        "shock your target and disrupt",
                        "control of his own body for",
                        "a while. You can't exert control",
                        "over victims with this skill, but they usually assume otherwise..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "You need to be judicious in",
                        "your use of this skill! Don't",
                        "use it recklessly, or people",
                        "will come to hate you as they",
                        "hate me. But ''Pang Voice'' can be welcome is certain situations."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "Subject your enemies to",
                        "Pang Voice as much as you",
                        "like, and no one will blame",
                        "you for it. And you can get away with casting Pang Voice on bullies",
                        "and drunkards occasionally..."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Riott",
                args![
                    "Hmm... You didn't",
                    "bring enough Eggs...",
                    "This will not do. It'll",
                    "be a while until Yhelle",
                    "will be able to lay more",
                    "eggs. Yes, this isn't enough..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Riott",
                args![
                    "Well, to make up for the",
                    "missing eggs, go and get ",
                    "me ^4D4DFF1 Munak Doll^000000. I know that",
                    "Yao Jun's Munak Dolls are",
                    "masterpieces, and I am an",
                    "an avid collector of her work."
                ],
            )?;
            ctx.next()?;
            ctx.mes("[Riott]")?;
            if ctx.items().count(574)? > 0 {
                ctx.lines(args![
                    "In the meanwhile,",
                    "I'll enjoy the few",
                    "eggs that you do have!",
                    "Bweh heh heh heh heh!"
                ])?;
                ctx.call(Function::DelItem, args![574, ctx.call(Function::CountItem, args![574])?])?;
            }
            ctx.var("qskill_bard").set(Val::from(8))?;
            return ctx.close();
        }
        if ctx.var("qskill_bard").get()?.number()? > 1 && ctx.var("qskill_bard").get()?.number()? < 7 {
            ctx.lines_as(
                "Riott",
                args![
                    "So have you been",
                    "having trouble gathering",
                    "eggs from Yhelle? I know",
                    "she can be one fast running",
                    "chicken. But to survive where",
                    "she roosts, she has to be."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as("Riott", args!["Hmmrmpf!", "Eh heh heh heh!"])?;
        ctx.call(Function::Emotion, args![constants::ET_KIK])?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["Um...", "What's so funny?"])?;
        ctx.next()?;
        if ctx.var("gef_bard_q").get()? == 30 || ctx.var("gef_bard_q").get()? == 31 {
            ctx.lines_as("Riott", args!["Hm? Ah! That's one of the"])?;
            if ctx.var("gef_bard_q").get()? == 30 {
                ctx.lines(args![
                    "Black Seals that can only",
                    "be given by Kino Kitty. You",
                    "must be a person of great",
                    "emotional depth if he favors",
                    "you enough to give you that."
                ])?;
            } else {
                ctx.lines(args![
                    "Silver Seals that can only",
                    "be given by Errende. You",
                    "must be truly kind at heart",
                    "if he has offered to be your",
                    "friend. How about that?"
                ])?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Riott",
                args!["Geh heh heh~", "Hey, take a look", "at those two drunks", "all the way over there."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args!["Yeah, I can see them.", "But what's so special", "about those two guys?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Riott", args!["Just...", "Keep watching."])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFRiott stared intensely",
                "at one of the drunken men",
                "and began to harshly murmur",
                "some indistinct words in a",
                "low, hoarse voice. One of the",
                "men starts slightly convulsing.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Little Bit Drunken Guy",
                args![
                    "H-hey...! ^333333*Hiccup!*^000000",
                    "What are you doing?!",
                    "K-keep your hands to",
                    "yourself! Do I look",
                    "like a woman to you?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "More Drunken Guy",
                args![
                    "What are you talking",
                    "about? Wh-what?! Why",
                    "are my arms all wrapped",
                    "around you? S-sorry, I was",
                    "trying to just go that w--",
                    "I wasn't trying to hug you!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Little Bit Drunken Guy",
                args![
                    "Bumping into me,",
                    "I understand. But a full",
                    "blown hug? Come on, now!",
                    "That was totally on purpose!",
                    "Wh-what? My h-hand! It's...",
                    "It's moving my itself?!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "More Drunken Guy",
                args!["Ack! Wh-what are", "you doing! S-stop", "touching my butt!"],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFThe two men continued",
                "to gesticulate and move",
                "wildly without direction."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args!["Those two...", "Those two probably", "had way too much to drink."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Riott",
                args![
                    "Nah, they just lost",
                    "control of their bodies",
                    "for a bit. It's the result",
                    "of my skill which sort of",
                    "scrambles their minds."
                ],
            )?;
            ctx.next()?;
            if ctx.var("qskill_bard").get()? == 1 {
                ctx.lines_as(
                    ctx.player().name()?,
                    args![
                        "Mind scrambling?",
                        "Wait, are you the same",
                        "Bard who made someone",
                        "jump off Umbala's Bungee",
                        "Jump against his will?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Riott",
                    args![
                        "Huh? How did you",
                        "learn about that?",
                        "I'm not proud of that,",
                        "(even though it was",
                        "hilarious at the time)",
                        "but yeah, that was me."
                    ],
                )?;
                ctx.next()?;
                if ctx.menu(&["Please teach me that skill!", "Oh, alright. Just checking."])? == 0 {
                    ctx.lines_as(
                        "Riott",
                        args![
                            "Hm? You want to learn",
                            "how to scramble minds",
                            "like I did just now? Well,",
                            "I invented this skill, though",
                            "I did have a lot of help from",
                            "the Utan Shaman. Let's see..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Riott",
                        args![
                            "Alright. If you want me",
                            "to teach you, then bring me",
                            "5 Eggs from a chicken named",
                            "Yhelle. Yhelle lays the highest",
                            "quality eggs: they're delicious",
                            "and great for a Bard's voice~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Riott",
                        args![
                            "However, this chicken roosts",
                            "in a strange, dangerous place.",
                            "You'll need to explore this huge, mysterious tree in Umbala in order",
                            "to get there. Last time I went,",
                            "I pretty much almost died."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Riott",
                        args![
                            "Alright...",
                            "So don't forget",
                            "to come back here",
                            "and bring me back",
                            "^4D4DFF5 Yhelle's Eggs^000000, alright?"
                        ],
                    )?;
                    ctx.var("qskill_bard").set(Val::from(2))?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Riott",
                    args![
                        "Boy, you're a curious one.",
                        "But if you know about that",
                        "and you bumped into me, you",
                        "must certainly travel around",
                        "a lot. I can respect a good,",
                        "seasoned adventurer like you."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "Wha...?",
                    "I can't believe you.",
                    "Mind scrambling? That",
                    "doesn't make any sense!",
                    "They're just really drunk..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Riott",
                args![
                    "How dare you question my",
                    "power? Oh well, I suppose",
                    "I really can't be too angry.",
                    "Most people who do believe",
                    "me usually claim that I'm",
                    "an axis of evil about now..."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Riott",
            args![
                "Eh, nothing much. Say,",
                "you don't have any Bard",
                "Seals? That's a sure sign",
                "that you haven't been really",
                "connecting with the Bard",
                "community. That's a shame..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Riott",
            args![
                "*Sigh* Alright, I know",
                "how you can meet more Bards.",
                "Why don't you try making friends with Errende? Look for a Bard",
                "dressed in green in Geffen and",
                "you should be able to find him."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Riott", args!["Hmmrmpf!", "Eh heh heh heh!"])?;
    ctx.call(Function::Emotion, args![constants::ET_KIK])?;
    ctx.next()?;
    ctx.lines_as(ctx.player().name()?, args!["Um...", "What's so funny?"])?;
    ctx.next()?;
    if ctx.var("gef_bard_q").get()? == 30 || ctx.var("gef_bard_q").get()? == 31 {
        ctx.lines_as("Riott", args!["Hm? Ah! That's one of the"])?;
        if ctx.var("gef_bard_q").get()? == 30 {
            ctx.lines(args![
                "Black Seals that can only",
                "be given by Kino Kitty. You",
                "must be a person of great",
                "emotional depth if he favors",
                "you enough to give you that."
            ])?;
        } else {
            ctx.lines(args![
                "Silver Seals that can only",
                "be given by Errende. You",
                "must be truly kind at heart",
                "if he has offered to be your",
                "friend. How about that?"
            ])?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Riott",
            args!["Geh heh heh~", "Hey, take a look", "at those two drunks", "all the way over there."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args!["Yeah, I can see", "together. But what", "about them?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Riott", args!["Just...", "Keep watching."])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Riott",
        args![
            "Eh, nothing much. Say,",
            "I notice you don't have",
            "any Bard seals. That tells",
            "me that you don't really",
            "meet that many Bards. You",
            "should really change that."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Riott",
        args![
            "Bards and Minstrels can",
            "make some pretty handy",
            "friends if you give them",
            "a chance. Here, why don't",
            "you find Errende in Geffen?",
            "He's a pretty popular guy..."
        ],
    )?;
    ctx.close()
}

pub fn spiteful_looking_bard_bs_ontouch(ctx: &Ctx) -> Script {
    ctx.call(Function::Emotion, args![constants::ET_KIK])?;
    ctx.end()
}

pub fn yhelle_bard_chick1(ctx: &Ctx) -> Script {
    ctx.call(
        Function::Emotion,
        args![constants::ET_HUK, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
    )?;
    ctx.call(Function::Emotion, args![constants::ET_HUK])?;
    ctx.lines_as("Hen Yhelle", args!["Cluck-Cluuuck?", "Cluck cluck cluck!"])?;
    ctx.next()?;
    ctx.set_npc_visible("Yhelle#bard_chick1", false)?;
    ctx.npc().do_event("Yhelle#bard_chick2::OnEnable")?;
    ctx.lines(args![
        "^3355FFUpon sensing your",
        "presense, the hen",
        "quickly ran away.^000000"
    ])?;
    ctx.close()
}

pub fn yhelle_bard_chick1_oninit(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Yhelle#bard_chick5", false)?;
    ctx.set_npc_visible("Yhelle#bard_chick4", false)?;
    ctx.set_npc_visible("Yhelle#bard_chick3", false)?;
    ctx.set_npc_visible("Yhelle#bard_chick2", false)?;
    ctx.end()
}

pub fn yhelle_bard_chick1_onenable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Yhelle#bard_chick1", true)?;
    ctx.end()
}

pub fn yhelle_bard_chick1_ondisable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Yhelle#bard_chick1", false)?;
    ctx.end()
}

pub fn yhelle_bard_chick1_ontouch(ctx: &Ctx) -> Script {
    shared::quests_skills_bard_skills::f_bardskillyhelle(ctx, args![1, 2])?;
    ctx.close()
}

pub fn yhelle_bard_chick2(ctx: &Ctx) -> Script {
    ctx.call(
        Function::Emotion,
        args![constants::ET_HUK, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
    )?;
    ctx.call(Function::Emotion, args![constants::ET_HUK])?;
    ctx.lines_as("Hen Yhelle", args!["Cluck-Cluuuck?", "Cluck cluck cluck!"])?;
    ctx.next()?;
    ctx.set_npc_visible("Yhelle#bard_chick2", false)?;
    ctx.npc().do_event("Yhelle#bard_chick3::OnEnable")?;
    ctx.lines(args![
        "^3355FFUpon sensing your",
        "presense, the hen",
        "quickly ran away.^000000"
    ])?;
    ctx.close()
}

pub fn yhelle_bard_chick2_onenable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Yhelle#bard_chick2", true)?;
    ctx.end()
}

pub fn yhelle_bard_chick2_ondisable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Yhelle#bard_chick2", false)?;
    ctx.end()
}

pub fn yhelle_bard_chick2_ontouch(ctx: &Ctx) -> Script {
    shared::quests_skills_bard_skills::f_bardskillyhelle(ctx, args![2, 3])?;
    ctx.close()
}

pub fn yhelle_bard_chick3(ctx: &Ctx) -> Script {
    ctx.call(
        Function::Emotion,
        args![constants::ET_HUK, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
    )?;
    ctx.call(Function::Emotion, args![constants::ET_HUK])?;
    ctx.lines_as("Hen Yhelle", args!["Cluck-Cluuuck?", "Cluck cluck cluck!"])?;
    ctx.next()?;
    ctx.set_npc_visible("Yhelle#bard_chick3", false)?;
    ctx.npc().do_event("Yhelle#bard_chick4::OnEnable")?;
    ctx.lines(args![
        "^3355FFUpon sensing your",
        "presense, the hen",
        "quickly ran away.^000000"
    ])?;
    ctx.close()
}

pub fn yhelle_bard_chick3_onenable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Yhelle#bard_chick3", true)?;
    ctx.end()
}

pub fn yhelle_bard_chick3_ondisable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Yhelle#bard_chick3", false)?;
    ctx.end()
}

pub fn yhelle_bard_chick3_ontouch(ctx: &Ctx) -> Script {
    shared::quests_skills_bard_skills::f_bardskillyhelle(ctx, args![3, 4])?;
    ctx.close()
}

pub fn yhelle_bard_chick4(ctx: &Ctx) -> Script {
    ctx.call(
        Function::Emotion,
        args![constants::ET_HUK, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
    )?;
    ctx.call(Function::Emotion, args![constants::ET_HUK])?;
    ctx.lines_as("Hen Yhelle", args!["Cluck-Cluuuck?", "Cluck cluck cluck!"])?;
    ctx.next()?;
    ctx.set_npc_visible("Yhelle#bard_chick4", false)?;
    ctx.npc().do_event("Yhelle#bard_chick5::OnEnable")?;
    ctx.lines(args![
        "^3355FFUpon sensing your",
        "presense, the hen",
        "quickly ran away.^000000"
    ])?;
    ctx.close()
}

pub fn yhelle_bard_chick4_onenable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Yhelle#bard_chick4", true)?;
    ctx.end()
}

pub fn yhelle_bard_chick4_ondisable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Yhelle#bard_chick4", false)?;
    ctx.end()
}

pub fn yhelle_bard_chick4_ontouch(ctx: &Ctx) -> Script {
    shared::quests_skills_bard_skills::f_bardskillyhelle(ctx, args![4, 5])?;
    ctx.close()
}

pub fn yhelle_bard_chick5(ctx: &Ctx) -> Script {
    ctx.call(
        Function::Emotion,
        args![constants::ET_HUK, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
    )?;
    ctx.call(Function::Emotion, args![constants::ET_HUK])?;
    ctx.lines_as("Hen Yhelle", args!["Cluck-Cluuuck?", "Cluck cluck cluck!"])?;
    ctx.next()?;
    ctx.set_npc_visible("Yhelle#bard_chick5", false)?;
    ctx.npc().do_event("Yhelle#bard_chick1::OnEnable")?;
    ctx.lines(args![
        "^3355FFUpon sensing your",
        "presense, the hen",
        "quickly ran away.^000000"
    ])?;
    ctx.close()
}

pub fn yhelle_bard_chick5_onenable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Yhelle#bard_chick5", true)?;
    ctx.end()
}

pub fn yhelle_bard_chick5_ondisable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Yhelle#bard_chick5", false)?;
    ctx.end()
}

pub fn yhelle_bard_chick5_ontouch(ctx: &Ctx) -> Script {
    shared::quests_skills_bard_skills::f_bardskillyhelle(ctx, args![5, 1])?;
    ctx.close()
}

pub fn customer_bard_skill01(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Little Bit Drunken Guy",
        args![
            "What do you think",
            "is the best drink in",
            "all the world? I think",
            "the Tri-- Tristan? What",
            "was it called again?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "More Drunken Guy",
        args![
            "Oh! 13 Year Old Tristan?",
            "That's a great drink, sure,",
            "but it's way too expensive for",
            "anything less than a special",
            "occasion. ^333333*Hiccup*^000000 Personally,",
            "I really like Ver... Uh, Ver..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Little Bit Drunken Guy",
        args![
            "Vermilion on the Beach?",
            "Yeah, that's really good,",
            "you'll pass out after just",
            "having one shot! Yeah...",
            "Tro... Tropical! People say",
            "that's good too. Wait, what?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Little Bit Drunken Guy",
        args![
            "H-hey...! ^333333*Hiccup!*^000000",
            "What are you doing?!",
            "K-keep your hands to",
            "yourself! Do I look",
            "like a woman to you?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "More Drunken Guy",
        args![
            "What are you talking",
            "about? Wh-what?! Why",
            "are my arms all wrapped",
            "around you? S-sorry, I was",
            "trying to just go that w--",
            "I wasn't trying to hug you!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Little Bit Drunken Guy",
        args![
            "Bumping into me,",
            "I understand. But a full",
            "blown hug? Come on, now!",
            "That was totally on purpose!",
            "Wh-what? My h-hand! It's...",
            "It's moving my itself?!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "More Drunken Guy",
        args!["Ack! Wh-what are", "you doing! S-stop", "touching my butt!"],
    )?;
    ctx.next()?;
    ctx.lines(args![
        "^3355FFA Bard in the room",
        "watches the two drunk",
        "men intently and giggles",
        "at their stupor. Remember:",
        "drinking too much isn't good!^000000"
    ])?;
    ctx.close()
}

pub fn customer_bard_skill02(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Little Bit Drunken Guy",
        args![
            "What do you think",
            "is the best drink in",
            "all the world? I think",
            "the Tri-- Tristan? What",
            "was it called again?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "More Drunken Guy",
        args![
            "Oh! 13 Year Old Tristan?",
            "That's a great drink, sure,",
            "but it's way too expensive for",
            "anything less than a special",
            "occasion. ^333333*Hiccup*^000000 Personally,",
            "I really like Ver... Uh, Ver..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Little Bit Drunken Guy",
        args![
            "Vermilion on the Beach?",
            "Yeah, that's really good,",
            "you'll pass out after just",
            "having one shot! Yeah...",
            "Tro... Tropical! People say",
            "that's good too. Wait, what?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Little Bit Drunken Guy",
        args![
            "H-hey...! ^333333*Hiccup!*^000000",
            "What are you doing?!",
            "K-keep your hands to",
            "yourself! Do I look",
            "like a woman to you?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "More Drunken Guy",
        args![
            "What are you talking",
            "about? Wh-what?! Why",
            "are my arms all wrapped",
            "around you? S-sorry, I was",
            "trying to just go that w--",
            "I wasn't trying to hug you!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Little Bit Drunken Guy",
        args![
            "Bumping into me,",
            "I understand. But a full",
            "blown hug? Come on, now!",
            "That was totally on purpose!",
            "Wh-what? My h-hand! It's...",
            "It's moving my itself?!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "More Drunken Guy",
        args!["Ack! Wh-what are", "you doing! S-stop", "touching my butt!"],
    )?;
    ctx.next()?;
    ctx.lines(args![
        "^3355FFA Bard in the room",
        "watches the two drunk",
        "men intently and giggles",
        "at their stupor. Remember:",
        "drinking too much isn't good!^000000"
    ])?;
    ctx.close()
}

pub fn bartender_bard_qskill(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
        ctx.lines(args!["^3355FFHold it right there!", "You're carrying too many items and don't have enough inventory space to receive any rewards. Please make more inventory space available and come back to take this challenge."])?;
        return ctx.close();
    }
    ctx.lines_as("Bartender", args!["So what would", "you like to order?"])?;
    ctx.next()?;
    match ctx.menu(&["Tropical Sograt", "Vermilion on the Beach", "Nothing, thanks."])? {
        0 => {
            if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 2000 {
                ctx.lines_as(
                    "Bartender",
                    args![
                        "You sure you can carry",
                        "any more stuff with you?",
                        "Damn, it doesn't look that",
                        "way to me. Take a load off,",
                        "and put some stuff with in",
                        "your Kafra Storage, alright?"
                    ],
                )?;
                return ctx.close();
            }
            if ctx.var("Zeny").get()?.number()? < 1000 {
                ctx.lines_as(
                    "Bartender",
                    args![
                        " You sure you can afford",
                        "this now? This drink's",
                        "1,000 zeny, so you better",
                        "check the cash you have",
                        "onhand. Eh, just come back",
                        "later when you have the money."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Bartender",
                args![
                    "Here you are,",
                    "1 Tropical Sograt.",
                    "It tastes sweet and",
                    "mild, but if you're not",
                    "careful, you'll pass out",
                    "in no time flat. Take it easy."
                ],
            )?;
            ctx.player().set_zeny(ctx.player().zeny()? - 1000)?;
            ctx.items().give(12112, 1)?;
            return ctx.close();
        }
        1 => {
            if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 2000 {
                ctx.lines_as(
                    "Bartender",
                    args![
                        "You sure you can carry",
                        "any more stuff with you?",
                        "Damn, it doesn't look that",
                        "way to me. Take a load off,",
                        "and put some stuff with in",
                        "your Kafra Storage, alright?"
                    ],
                )?;
                return ctx.close();
            }
            if ctx.var("Zeny").get()?.number()? < 1000 {
                ctx.lines_as(
                    "Bartender",
                    args![
                        "You sure you can afford",
                        "this now? This drink's",
                        "1,000 zeny, so you better",
                        "check the cash you have",
                        "onhand. Eh, just come back",
                        "later when you have the money."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Bartender",
                args!["Here you go.", "Be sure that you", "enjoy your drinking", "without going crazy."],
            )?;
            ctx.player().set_zeny(ctx.player().zeny()? - 1000)?;
            ctx.items().give(12113, 1)?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as("Bartender", args!["Alright, then.", "I'll see you around."])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}
