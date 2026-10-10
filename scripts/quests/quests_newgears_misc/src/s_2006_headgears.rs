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

pub fn script(ctx: &Ctx) -> Script {
    if ctx.items().count(983)? > 0 && ctx.items().count(7111)? > 99 && ctx.items().count(938)? > 98 && ctx.player().zeny()? > 99999 {
        ctx.lines_as(
            "?",
            args![
                "Giggle giggle...isn't it my partner, eh?",
                "So, did you bring everything that I asked?",
                "Great, now I can make the item which will help you",
                "to cover your identity! Giggle giggle..."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "- Bzzz Bzzz Click Click -",
            "- Fumble Fumble Fumble Fumble -",
            "- Bzzz Bzzz Click Click -",
            "- Fumble Fumble Fumble Fumble -"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "?",
            args![
                "...Hey, don't look over my shoulder.",
                "I don't want to share",
                "my business secret with you, you know?"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "- Bzzz Bzzz Click Click -",
            "- Fumble Fumble Fumble Fumble -",
            "- Bzzz Bzzz Click Click -",
            "- Fumble Fumble Fumble Fumble -"
        ])?;
        ctx.next()?;
        ctx.items().take(983, 1)?;
        ctx.items().take(7111, 100)?;
        ctx.items().take(938, 99)?;
        ctx.player().set_zeny(ctx.player().zeny()? - 100000)?;
        ctx.items().give(5175, 1)?;
        ctx.lines_as(
            "?",
            args![
                "Phew...it's done. Well, it was not that difficult to make, but...",
                "Giggle giggle, what is important is that",
                "now you can hide your identity. Now, take this.",
                "Hopefully, this mask will help you",
                "to avoid encountering your enemies. Ahahaha!"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "?",
        args![
            "...No way! Don't you dare to find out about me!",
            "Don't you even speak to me!",
            "Shushhh! Don't let my enemy know where I am!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "?",
        args![
            "Err? You are a my kind person.",
            "So, you are running away from something,",
            "and you want to hide your identity, am I right?",
            "Yeah...I guess that I am right... Giggle giggle."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "?",
        args![
            "Well, I should be kind to my comrade.",
            "Although I can't reveal my identity to you,",
            "I can help you to safely hide from your enemies."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "?",
        args![
            "Bring me ^FF00001 Black Dyestuffs^000000,",
            "^FF0000100 Slick Paper^000000, ^FF000099 Sticky Mucus^000000,",
            "and ^FF0000100,000 zeny^000000.",
            "Then I will help you, giggle giggle."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "?",
        args![
            "You may leave now.",
            "Somehow we can be so helpful to each other.",
            "Once you finish gathering all the material,",
            "come back without anyone knowning. Giggle giggle."
        ],
    )?;
    ctx.close()
}

pub fn sakjul(ctx: &Ctx) -> Script {
    if ctx.items().count(5172)? > 0 && ctx.items().count(7063)? > 99 && ctx.items().count(982)? > 0 {
        ctx.lines_as(
            "Sakjul",
            args![
                "Great, you have brought everything",
                "to make ^FF0000Feather Beret^000000!",
                "To reward for your labor,",
                "I shall personally proceed with the hat creation."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sakjul",
            args![
                "Stand next to me, and watch the creation process solemnly.",
                "Keep your integrity by standing straight,",
                "and looking straight forward!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sakjul",
            args![
                "If you do not do what I just said,",
                "^FF0000this hat creation could result in failure!",
                "And, if you fail to create the hat,",
                "you will lose all the materials,",
                "and I am not going to take the responsibility for your mistake!^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args!["- THUD THUD -", "- THUD THUD -", "- THUD THUD -", "- BOOM -"])?;
        ctx.next()?;
        ctx.lines(args!["- THUD THUD -", "- THUD THUD -", "- THUD THUD -", "- BOOM -"])?;
        ctx.next()?;
        let roll = ctx.rand_range(1, 10)?;
        if roll == 4 {
            ctx.items().take(5172, 1)?;
            ctx.items().take(7063, 100)?;
            ctx.items().take(982, 1)?;
            ctx.lines_as(
                "Sakjul",
                args![
                    "I did succeed in making the hat,",
                    "but I cannot let it fall into someone else's hand!",
                    "The essential of Feather Beret is",
                    "the precise angle of the beret and the feather,",
                    "but this hat's angle has become slightly crooked,",
                    "and the quality has become too poor to be my artwork!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sakjul",
                args![
                    "I understand that you feel quite unfortunate",
                    "with this result. However, I urge you to not to be",
                    "so disappointed, and try again!",
                    "There is no impossibility in the world, so you can do it!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sakjul",
                args![
                    "Now, brace yourself up!",
                    "You have done this already,",
                    "so you can easily do it again.",
                    "If you understood, now, go, go gather the materials again!"
                ],
            )?;
            return ctx.close();
        }
        ctx.items().take(5172, 1)?;
        ctx.items().take(7063, 100)?;
        ctx.items().take(982, 1)?;
        ctx.items().give(5170, 1)?;
        ctx.lines_as(
            "Sakjul",
            args![
                "Great, I have made it! Look at this beautiful coordination",
                "between the feather and the beret,",
                "and you can feel moderation in the coordination.",
                "I must say that this hat must be",
                "one of the needful things for young men in nowadays."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sakjul",
            args![
                "Since I have created this hat with your materials,",
                "it belongs to you.",
                "Keep this hat with care, and be a great, confident person."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Sakjul",
        args![
            "Straighten yourself, and keep your tension!",
            "Keep yourself under control, and move with integrity!",
            "That's how a respectable man carry himself!",
            "Young men in nowadays are too weak and tender.",
            "Don't you agree with me, young adventurer?"
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Yes, sir!", "No."])? == 0 {
        ctx.lines_as(
            "Sakjul",
            args![
                "Ah! I like your answer!",
                "Lately, I found it very hard to see a diciplined young man like you.",
                "Unfortunately, even you are not yet diciplined as well as I expect!",
                "Hmm....Oh, yes, probably ^FF0000Feather Beret^000000",
                "might help you to look more diciplined."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sakjul",
            args![
                "I like to compliment your attitude.",
                "Thus, I am willing to create Feather Beret for you",
                "only if you bring me the materials to me. Understand?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sakjul",
            args![
                "Materials are ^FF00001 Beret^000000, ^FF0000100 Soft Feather^000000,",
                "and ^FF00001 White Dyestuffs^000000.",
                "Make sure that you will remember all of them, and bring me",
                "the exact amount of materials!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sakjul",
            args![
                "Thank me for the offer",
                "because the hat will finish your look",
                "to be more majestic and elegant!",
                "If you understood, go,",
                "go gather the material as soon as you can!"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Sakjul",
        args![
            "I can't hear you! I don't feel any confidence from your voice!",
            "How can you live this tough world with that weak attitude?",
            "Put yourself together, right now!"
        ],
    )?;
    ctx.close()
}

pub fn ghenirhemin(ctx: &Ctx) -> Script {
    if ctx.var("moza_valkylie").get()? == 5 {
        let roll = ctx.rand_range(1, 10)?;
        if roll == 3 {
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_LIGHTSPHERE])?;
            ctx.lines_as("Ghenirhemin", args!["The materials are still being fused."])?;
            ctx.next()?;
            ctx.lines_as("Ghenirhemin", args!["Umm?! Oh...oh?! Isn't this...", "Isn't this...?!"])?;
            ctx.next()?;
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_FORESTLIGHT2])?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "Ah...ahahaha, we've made it! We've made it!",
                    "Valkyre's Helm...We've recreated Valkyre's Helm,",
                    "a glorious godly armor!",
                    "Ah...does this mean...?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args!["We are writing new history!", "Yes, we just have started writing new history."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "Thank you so much for giving me a chance to participate in this.",
                    "I am sure that my ancestors in heaven will be glad",
                    "to see this successful recreation of Valkyre's Helm.",
                    "Hahahahaha!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "Please take this helm.",
                    "As I promised, this is yours.",
                    "Please take pride in yourself, and",
                    "do not disgrace this Helm through evil doings."
                ],
            )?;
            ctx.next()?;
            ctx.var("moza_valkylie").set(0)?;
            ctx.items().give(5171, 1)?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "May God bless this adventurer",
                    "who has written new history.",
                    "I, Gheirhemin pray to god for his safe journey",
                    "who is now heading toward the new history."
                ],
            )?;
            return ctx.close();
        }
        ctx.call(Function::NpcSpecialEffect, args![constants::EF_LIGHTSPHERE])?;
        ctx.lines_as("Ghenirhemin", args!["The materials are still being fused."])?;
        return ctx.close();
    } else if ctx.var("moza_valkylie").get()? == 4 {
        let roll = ctx.rand_range(1, 10)?;
        if roll == 7 {
            ctx.lines_as("Ghenirhemin", args!["............"])?;
            ctx.next()?;
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_LIGHTSPHERE])?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "Oh...oh? They....they just have started being fused!",
                    "Look at this shining light!",
                    "I can't...I can't believe that",
                    "this is what happens in Valhala!"
                ],
            )?;
            ctx.next()?;
            ctx.var("moza_valkylie").set(5)?;
            ctx.lines_as("Ghenirhemin", args!["We are now witnessess of the God's grace...ah...."])?;
            return ctx.close();
        }
        ctx.lines_as("Ghenirhemin", args!["....Let's wait a little longer."])?;
        return ctx.close();
    } else if ctx.var("moza_valkylie").get()? == 3 {
        ctx.var("moza_valkylie").set(4)?;
        ctx.lines_as(
            "Ghenirhemin",
            args![
                "Will you stop being anxious?",
                "Just relax and wait, will you?",
                "In a sense, we are holding a holy rite,",
                "and it is not going to happen faster,",
                "only because you, a human wants it to happen faster."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("moza_valkylie").get()? == 2
        && ctx.player().zeny()? > 9999999
        && ctx.items().count(4219)? > 0
        && ctx.items().count(4114)? > 0
        && ctx.items().count(4177)? > 0
        && ctx.items().count(4259)? > 0
        && ctx.items().count(4212)? > 0
        && ctx.items().count(4073)? > 0
        && ctx.items().count(4112)? > 0
        && ctx.items().count(4081)? > 0
        && ctx.items().count(4251)? > 0
        && ctx.items().count(4166)? > 0
        && ctx.items().count(7511)? > 999
        && ctx.items().count(7563)? > 999
    {
        ctx.lines_as(
            "Ghenirhemin",
            args![
                "Hmm...I am impressed that you have brought everything",
                "despite of the difficulty in gathering them. It was difficult, wasn't it?",
                "Haha, I can tell just by looking at your face.",
                "Good job, my friend, you did a good job."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ghenirhemin",
            args![
                "Now, please hand them to me.",
                "If I place them together in one place,",
                "I am pretty sure that they will start",
                "being fused into the helm with their own mysterious powers."
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Give him the items.", "Don't give him the items."])? == 0 {
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "Excellent. Now, all we have to do",
                    "is just waiting until they finish being fused into one."
                ],
            )?;
            ctx.next()?;
            ctx.items().take(4219, 1)?;
            ctx.items().take(4114, 1)?;
            ctx.items().take(4177, 1)?;
            ctx.items().take(4259, 1)?;
            ctx.items().take(4212, 1)?;
            ctx.items().take(4073, 1)?;
            ctx.items().take(4112, 1)?;
            ctx.items().take(4081, 1)?;
            ctx.items().take(4251, 1)?;
            ctx.items().take(4166, 1)?;
            ctx.items().take(7511, 1000)?;
            ctx.items().take(7563, 1000)?;
            ctx.player().set_zeny(ctx.player().zeny()? - 10000000)?;
            ctx.var("moza_valkylie").set(3)?;
            ctx.lines_as(
                "Ghenirhemin",
                args!["Now I am all nervous", "in anticipation of a good result. Hahahaha."],
            )?;
            return ctx.close();
        }
        ctx.lines_as("Ghenirhemin", args!["?"])?;
        return ctx.close();
    } else if ctx.var("moza_valkylie").get()? == 2 {
        ctx.lines_as(
            "Ghenirhemin",
            args![
                "Please bring me",
                "^FF00001 Sage Worm Card, 1Argiope Card,",
                "^FF00001 Dryad Card, 1 Wooden Golem Card,",
                "^FF00001 Bongun Card, 1 Pirate Skeleton Card,",
                "^FF00001 Marduk Card, 1 Hode Card,",
                "^FF00001 Elder Card, 1 Nightmare Terror Card,^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ghenirhemin",
            args![
                "^FF00001,000 Rune of the Darkness,",
                "^FF00001,000 Bloody Rune,",
                "^FF0000and the helm price, 10,000,000 zeny^000000."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ghenirhemin",
            args![
                "It surely is a reasonable price considering that ",
                "you are about to obtain a godly power, don't you think?"
            ],
        )?;
        ctx.next()?;
        ctx.var("moza_valkylie").set(2)?;
        ctx.lines_as(
            "Ghenirhemin",
            args![
                "Let's discuss it later",
                "once you prepare the money and the items.",
                "Now I am so exhausted",
                "as it has been a while since the last time that I talked this much.",
                "Excuse me."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("moza_valkylie").get()? == 1 {
        ctx.lines_as(
            "Ghenirhemin",
            args![
                "You must be pretty surprised by now.",
                "Yes, it is surely understandable.",
                "If you have travelled many places,",
                "and heard many stories, you would know what this is.",
                "Yes, it is ^FF0000Valkyre's Helm^000000."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ghenirhemin",
            args![
                "A legendary helm only allowed to men",
                "who have transcended their limit.",
                "A godly armor that is blessed with an incredible power.",
                "Can you see now that my story is true?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ghenirhemin",
            args![
                "It has passed so many years, and thus",
                "its power has become weakened,",
                "but you can still feel something holy about the helm.",
                "Don't you think?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ghenirhemin",
            args![
                "Well...I am not allowed to wear this helm.",
                "Thus, if I find someone who is just perfect for this helm,",
                "I am going to give this to him without any regret."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ghenirhemin",
            args![
                "Of course, I can't give away",
                "such a valuable thing for nothing in return.",
                "That's how it goes, you know?",
                "I maybe think about giving it to you",
                "because you seem to be wealthier than others,",
                "and I can sense special aura from you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ghenirhemin",
            args![
                "So, what do you say?",
                "I think that I have an eye for right men.",
                "I feel that you will be able to fully recreate this Valkyre's Helm",
                "as well as use it to its full potential."
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Sure.", "No, thanks."])? == 0 {
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "You are an ambitious young man as I expected.",
                    "It is surely a great advantage.",
                    "Okay, I will tell you what I know from now on.",
                    "So, listen carefully."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "First, we have to fully restore the helm's power.",
                    "To do so, we must find monster cards",
                    "that possess mysterious powers."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "That's not all. You know,",
                    "we can't just glue those monster cards",
                    "on the helm. Thus, we need a power that",
                    "enables us to fuse the cards and the helm into one."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "Ancient runes are perfect to supply the power,",
                    "especially we need Rune of the Darkness, and Bloody Rune.",
                    "Perhaps, they were chosen because of their sealed dark power",
                    "and bloody sticky power? That's just my guess, hahahaha!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "Once we successfully restore the helm's power,",
                    "you are expected to purchase the helm from me.",
                    "As I said earlier, I am not going to",
                    "give away such a valuable thing with nothing in return.",
                    "It will also demonstrate your qualification to become the helm's owner."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "Basically, you have to bring me...",
                    "Ah, there are so many things that you need to bring me.",
                    "I suggest you to write down, and memorize them."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "Please bring me",
                "^FF00001 Sage Worm Card, 1Argiope Card,",
                "1 Dryad Card, 1 Wooden Golem Card,",
                "1 Bongun Card, 1 Pirate Skeleton Card,",
                "1 Marduk Card, 1 Hode Card,",
                "1 Elder Card, 1 Nightmare Terror Card,^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "^FF00001,000 Rune of the Darkness,",
                    "1,000 Bloody Rune,",
                    "and the helm price, 10,000,000 zeny^000000."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "It surely is a reasonable price considering that ",
                    "you are about to obtain a godly power, don't you think?"
                ],
            )?;
            ctx.next()?;
            ctx.var("moza_valkylie").set(2)?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "Let's discuss it later",
                    "once you prepare the money and the items.",
                    "Now I am so exhausted",
                    "as it has been a while since the last time that I talked this much.",
                    "Excuse me."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Ghenirhemin",
            args![
                "Haha, I understand that",
                "it is not an easy choice to make.",
                "Take your time, my friend.",
                "If you still don't want the helm, I respect your decision."
            ],
        )?;
        return ctx.close();
    } else if ctx.player().zeny()? > 9999999 {
        ctx.lines_as(
            "Ghenirhemin",
            args![
                "Hmm...you look like a poor-looking wanderer,",
                "but you seem to have an enormous amount of money with you.",
                "I guess that the old saying was right:",
                "''Don't judge a book by its cover.''"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ghenirhemin",
            args![
                "You maybe understand my story.",
                "Would you like to listen to my long story?",
                "It may or may not be interesting to you,",
                "but I gurantee that it will never bore you in either way."
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Listen.", "Don't listen."])? == 0 {
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "Good, let me introduce myself.",
                    "My name is Ghenirhemin.",
                    "I am no different than others,",
                    "if I may speak of my only advantage,",
                    "I have little more money than them. Hahaha."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "However, I did not earn my money,",
                    "but I inherited the fortune from my ancestors,",
                    "who were all gifted businessmen.",
                    "I thank to him for handing over such great fortune."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "In fact, I am ashamed to tell you that",
                    "I do not know what exactly my ancestors did",
                    "to make this much fortune. Perhaps some would have held business,",
                    "some others would have been great artists..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "Our family is just blessed enough to have",
                    "such great ancestors, who brought wealth to their family.",
                    "It would take me at least a week to list",
                    "every single successful ancestor of mine."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "However, I like to tell you about one of my amazing ancestors,",
                    "who has become our family hero."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "Well, actually I do not know exactly",
                    "what he did, but I know that",
                    "he brought glory and fame",
                    "to our family for the first time.",
                    "However, since it has been so long, we do not have",
                    "any document about him, so..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "I clearly remember one thing about him though.",
                    "He was the only human who was able to",
                    "become close to the gods, and he demonstrated",
                    "his godly power to bring light to us, humans."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "I don't know how he obtained the godly power,",
                    "or how he could become close to the gods.",
                    "I just remember these things because I was repeatedly told",
                    "about his amazing story ever since I was born..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "I can show you that I am not making things up.",
                    "I told you that we do not have any document about him, right?",
                    "However, there is something that I can show you.",
                    "Now, take a look at this."
                ],
            )?;
            ctx.next()?;
            ctx.var("moza_valkylie").set(1)?;
            ctx.lines_as(
                "Ghenirhemin",
                args![
                    "See? This surely tells you that",
                    "I am not making things up!",
                    "This is what my ancestor used to use!"
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Ghenirhemin",
            args![
                "What a shame! However, I don't want to force you to",
                "do something that you don't want to do. Hahaha.",
                "Just remember, you just have made another choice for your life."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Ghenirhemin",
        args![
            "Everything has meaning and reason to exist in this world.",
            "You and I, we are destined to meet with each other today."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ghenirhemin",
        args![
            "Well...although I said that everything has meaning,",
            "unfortunately you are not interesting to me at all.",
            "Call me a money monger,",
            "but I like the rich."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ghenirhemin",
        args![
            "If you prove me that you are wealthy,",
            "I maybe change my mind, and become interested in you.",
            "But, I will decide whether or not I will be interested in you",
            "when you show me your money. Hahahaha!"
        ],
    )?;
    ctx.close()
}

#[derive(Clone, Copy, Debug)]
enum ChungwolmangStep {
    Start,
    SMakeMask,
}

fn chungwolmang_run(ctx: &Ctx, step: ChungwolmangStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ChungwolmangStep::Start => {
                if ctx.var("moza_tal").get()? == 2 {
                    if ctx.items().count(7015)? > 19 && ctx.items().count(952)? > 99 && ctx.items().count(1028)? > 99 {
                        chungwolmang_run(
                            ctx,
                            ChungwolmangStep::SMakeMask,
                            args![5176, 7015, 20, 952, 100, 1028, 100, 0, 0],
                        )?;
                    }
                    if ctx.items().count(1048)? > 499 && ctx.items().count(1053)? > 1 && ctx.items().count(980)? > 0 {
                        chungwolmang_run(ctx, ChungwolmangStep::SMakeMask, args![5177, 1048, 500, 1053, 2, 980, 1, 0, 0])?;
                    }
                    if ctx.items().count(1049)? > 19
                        && ctx.items().count(1059)? > 499
                        && ctx.items().count(1054)? > 1
                        && ctx.items().count(1024)? > 99
                    {
                        chungwolmang_run(
                            ctx,
                            ChungwolmangStep::SMakeMask,
                            args![5169, 1049, 20, 1059, 500, 1054, 2, 1024, 100],
                        )?;
                    }
                    ctx.lines_as(
                        "Chungwolmang",
                        args![
                            "Well well, have you not gathered the materials yet?",
                            "You'd better hurry because I can't wait so long!",
                            "Wait, what mask did you ask me to make anyways?"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Hahoe Mask", "Bride Mask", "Mythical Lion Mask"])? {
                        0 => {
                            ctx.lines_as(
                                "Chungwolmang",
                                args![
                                    "Oh, yes, you want Hahoe Mask.",
                                    "Then you need to bring... ^FF000020 Bookclip in Memory^000000,",
                                    "^FF0000100 Cactus Needle^000000, and",
                                    "^FF0000100 Mane^000000."
                                ],
                            )?;
                        }
                        1 => {
                            ctx.lines_as(
                                "Chungwolmang",
                                args![
                                    "Oh, yes, you want Bride Mask.",
                                    "Then you need to bring... ^FF000020 Skirt of Virgin^000000,",
                                    "^FF0000500 Fabric^000000, ^FF00002 Ancient Lips^000000,",
                                    "and ^FF0000100 Squid Ink^000000."
                                ],
                            )?;
                        }
                        2 => {
                            ctx.lines_as(
                                "Chungwolmang",
                                args![
                                    "Oh, yes, you want Mythical Lion Mask.",
                                    "Then you need to bring... ^FF0000500 Horrendous Hair^000000,",
                                    "^FF00002 Ancient Tooth^000000, and",
                                    "^FF00001 Orange Dyestuffs^000000."
                                ],
                            )?;
                        }
                        _ => {}
                    }
                    ctx.mes("Bring the exact amount of materials, then I will give you the mask right away.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.items().count(7201)? > 499 && ctx.items().count(7200)? > 9 && ctx.var("moza_tal").get()? == 1 {
                    ctx.lines_as(
                        "Chungwolmang",
                        args![
                            "Ah, Four Leaf Clover must bring me luck",
                            "because you came back faster than I thought! Wow...",
                            "Thank you so much, now I think that",
                            "I have fortune on my side. Hahaha!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chungwolmang",
                        args![
                            "Alright! I will return your favor as I promised.",
                            "I will tell you everything. In fact,",
                            "I am a traditional mask craftsman.",
                            "I am proud to tell you that no one can beat me",
                            "in crafting traditional masks! Ahem!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chungwolmang",
                        args![
                            "I had experienced emotional instability",
                            "as well as had bad luck with everything due to various reasons.",
                            "But, that's history now! I am back, hahahaha!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chungwolmang",
                        args![
                            "I feel good, and I have fortune back to my side.",
                            "I really appreciate you for helping me to feel better.",
                            "In return, I am going to make you a traditional mask! Hahaha!",
                            "You should be thankful for that I am offering you a great gift."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chungwolmang",
                        args![
                            "You know why? Because there is no one who can offer you",
                            "such a valuable item except me.",
                            "Anyhow, I can make 3 different masks:",
                            "Hahoe Mask, Bride Mask and Mythical Lion Mask.",
                            "Which one whould you like to have? Pick one."
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Hahoe Mask", "Bride Mask", "Mythical Lion Mask"])? {
                        0 => {
                            ctx.lines_as(
                                "Chungwolmang",
                                args![
                                    "Great, you want Hahoe Mask, eh? That's easy!",
                                    "Then you need to bring... ^FF000020 Bookclip in Memory^000000,",
                                    "^FF0000100 Cactus Needle^000000, and",
                                    "^FF0000100 Mane^000000.",
                                    "Bring the exact amount of materials, then I will give you the mask right away."
                                ],
                            )?;
                        }
                        1 => {
                            ctx.lines_as(
                                "Chungwolmang",
                                args![
                                    "Great, you want Bride Mask, eh? That's easy!",
                                    "Then you need to bring... ^FF000020 Skirt of Virgin^000000,",
                                    "^FF0000500 Fabric^000000, ^FF00002 Ancient Lips^000000,",
                                    "and ^FF0000100 Squid Ink^000000.",
                                    "Bring the exact amount of materials, then I will give you the mask right away."
                                ],
                            )?;
                        }
                        2 => {
                            ctx.lines_as(
                                "Chungwolmang",
                                args![
                                    "Great, you want Mythical Lion Mask, eh? That's easy!",
                                    "Then you need to bring... ^FF0000500 Horrendous Hair^000000,",
                                    "^FF00002 Ancient Tooth^000000, and",
                                    "^FF00001 Orange Dyestuffs^000000.",
                                    "Bring the exact amount of materials, then I will give you the mask right away."
                                ],
                            )?;
                        }
                        _ => {}
                    }
                    ctx.items().take(7201, 500)?;
                    ctx.items().take(7200, 10)?;
                    ctx.var("moza_tal").set(2)?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chungwolmang",
                        args![
                            "...Why are you eyeing at me?",
                            "What, did you expect me to make the mask for free?",
                            "If you did, you must think of it this way.",
                            "My term of payback is to use my skills,",
                            "and let you have my precious mask,",
                            "not making the mask for you at free of charge, understood?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chungwolmang",
                        args![
                            "At least I am not charging you with service fee, am I?",
                            "Just bring me the materials, I will do my best",
                            "to make an incredible mask for you!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chungwolmang",
                        args![
                            "In the meantime, I am going to prepare",
                            "the work by using these Log and Elastic Band.",
                            "So come back as soon as you can, okay?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("moza_tal").get()? == 1 {
                    ctx.lines_as(
                        "Chungwolmang",
                        args![
                            "Umm...the things that I've asked you were",
                            "^FF0000500 Log^000000 and ^FF000010 Elastic Band^000000.",
                            "Thanks in advance."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.items().count(706)? > 0 {
                    ctx.lines_as(
                        "Chungwolmang",
                        args![
                            "Ah...! Hey! I can tell something unique about you.",
                            "Do you have a Four Leaf Clover by any chance?",
                            "Do you mind if I ask you to give me the clover?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chungwolmang",
                        args![
                            "I am not asking you to give it to me for free.",
                            "Although I can't gurantee it,",
                            "I will be able to pay back your favor sooner or later.",
                            "So, can I take it?"
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.menu(&["Sure.", "No, you can't."])? == 0 {
                        ctx.lines_as(
                            "Chungwolmang",
                            args![
                                "Oh! Thank you so much!",
                                "Muhahahahahaha! Now I will become a lucky guy!",
                                "With the clover, fortune will be on my side again!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Chungwolmang",
                            args![
                                "Oh, right! I almost forgot.",
                                "I promised you to pay back your favor, right?",
                                "I know that this might sound selfish,",
                                "but, hey, can you do me one more favor?",
                                "Since you did me a favor already,",
                                "I don't think that it would be",
                                "that hard for you to help me one more time?",
                                "Please, show me your generosity~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Chungwolmang",
                            args![
                                "I need... ^FF0000500 Log^000000 and",
                                "^FF000010 Elastic Band^000000.",
                                "Will you be so kind to bring them to me?",
                                "In fact, I can't pay you back unless I have them."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.items().take(706, 1)?;
                        ctx.var("moza_tal").set(1)?;
                        ctx.lines_as(
                            "Chungwolmang",
                            args!["You want me to pay you back, don't you?", "So, please bring them to me, please~"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Chungwolmang",
                        args![
                            "I see... Well, it is not easy to give away the lucky charm",
                            "to a stranger without a second thought.",
                            "...Haha, but that doesn't make me stop being upset.",
                            "How dare you to refuse my request? Hah!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Chungwolmang", args!["Arrrgghhh... I neeed... I need..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Chungwolmang",
                    args![
                        "I want to be lucky! Someone, please bring me luck!",
                        "I am not asking too much! I need one thing!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Chungwolmang",
                    args![
                        "I need a ^FF0000Four Leaf Clover^000000!",
                        "Somebody, please bring me a Four Leaf Clover!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ChungwolmangStep::SMakeMask => {
                let item_id = runtime::arg(&args, 0, Val::from(0));
                ctx.lines_as(
                    "Chungwolmang",
                    args![
                        "Oh, you have brought everything",
                        Val::from("to make a ") + ctx.call(Function::GetItemName, args![item_id.clone()])? + Val::from(". Excellent."),
                        "Please give me the materials. I will make the mask right away."
                    ],
                )?;
                ctx.next()?;
                if ctx.menu(&["Give him the items.", "Don't give him the items."])? == 0 {
                    ctx.lines_as(
                        "Chungwolmang",
                        args![
                            "Great, I like your unhesitating attitude!",
                            "Alright then, I will make the mask as quickly as I can!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chungwolmang",
                        args![
                            "- Fumble Fumble Fumble -",
                            "- Fumble Fumble Fumble -",
                            "- Fumble Fumble Fumble -",
                            "- Thud Thud Thud Thud -"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chungwolmang",
                        args![
                            "- Fumble Fumble Fumble -",
                            "- Fumble Fumble Fumble -",
                            "- Fumble Fumble Fumble -",
                            "- Thud Thud Thud Thud -"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(
                        Function::DelItem,
                        args![runtime::arg(&args, 1, Val::from(0)), runtime::arg(&args, 2, Val::from(0))],
                    )?;
                    ctx.call(
                        Function::DelItem,
                        args![runtime::arg(&args, 3, Val::from(0)), runtime::arg(&args, 4, Val::from(0))],
                    )?;
                    ctx.call(
                        Function::DelItem,
                        args![runtime::arg(&args, 5, Val::from(0)), runtime::arg(&args, 6, Val::from(0))],
                    )?;
                    if item_id == 5169 {
                        ctx.call(
                            Function::DelItem,
                            args![runtime::arg(&args, 7, Val::from(0)), runtime::arg(&args, 8, Val::from(0))],
                        )?;
                    }
                    ctx.call(Function::GetItem, args![item_id.clone(), 1])?;
                    ctx.var("moza_tal").set(0)?;
                    ctx.lines_as(
                        "Chungwolmang",
                        args![
                            "Wow, it was a piece of cake!",
                            "You know, I am not an artisan only in title.",
                            Val::from("Hahaha, here, take your ")
                                + ctx.call(Function::GetItemName, args![item_id.clone()])?
                                + Val::from("."),
                            "I hope that you will wear it with pride, hahaha!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Chungwolmang",
                        args!["Aright, I need to take a rest", "until I have next customer. Hahaha, bye!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Chungwolmang",
                    args![
                        Val::from("Err? Don't you want ") + ctx.call(Function::GetItemName, args![item_id.clone()])? + Val::from("?"),
                        "Alright then...coward."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn chungwolmang(ctx: &Ctx) -> Script {
    chungwolmang_run(ctx, ChungwolmangStep::Start, Vec::new()).map(|_| ())
}

pub fn han_garam(ctx: &Ctx) -> Script {
    if ctx.var("moza_korea").get()? == 2
        && ctx.items().count(954)? > 299
        && ctx.items().count(733)? > 4
        && ctx.items().count(975)? > 0
        && ctx.items().count(7166)? > 49
    {
        ctx.lines_as(
            "Han Garam",
            args![
                "You came back faster than I expected.",
                "Great, I am also highly motivated for the fact that",
                "my Ayam will belong to someone like you,",
                "who are competent and trustworthy."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Han Garam",
            args![
                "Okay then, I will make the hat as quickly as I can.",
                "Now, hand me all the materials."
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Give him the items.", "Don't give him the items."])? == 0 {
            ctx.lines_as(
                "Han Garam",
                args![
                    "Excellent! I don't have to be mediumized",
                    "by the Dragon God for this work",
                    "because I can perfectly do it on my own.",
                    "So, hold it right there, okay?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Han Garam", args!["Yes...right...umm hmm."])?;
            ctx.next()?;
            ctx.lines_as("Han Garam", args!["................."])?;
            ctx.next()?;
            ctx.lines(args![
                "- Han Garam started hammering, -",
                "- and assembling the materials without a word. -"
            ])?;
            ctx.next()?;
            ctx.lines_as("Han Garam", args!["................."])?;
            ctx.next()?;
            ctx.lines_as(
                "Han Garam",
                args!["Phew, it was quite difficult,", "as I had not done this for quite a while."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Han Garam",
                args![
                    "I still feel good that I had a chance",
                    "to exercise my skills, you know. It was fun, too.",
                    "Please take this Ayam.",
                    "As I promised, this is my gift for you."
                ],
            )?;
            ctx.next()?;
            ctx.items().take(954, 300)?;
            ctx.items().take(733, 5)?;
            ctx.items().take(975, 1)?;
            ctx.items().take(7166, 50)?;
            ctx.var("moza_korea").set(0)?;
            ctx.items().give(5174, 1)?;
            ctx.lines_as(
                "Han Garam",
                args![
                    "Now I need to meet with the Dragon God again,",
                    "if I want to make another hat...umm...",
                    "Oh well, somehow I was able to make one this time with your help,",
                    "so I guess that it will happen when the time is right."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Han Garam",
                args![
                    "Thank you so much! Please take my Ayam with care,",
                    "and be proud that you are the owner of Ayam!"
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Han Garam",
            args!["...? What? Do you need more time?", "Can't you just give them to me already?"],
        )?;
        return ctx.close();
    } else if ctx.var("moza_korea").get()? == 2
        && ctx.items().count(954)? > 299
        && ctx.items().count(733)? > 4
        && ctx.items().count(983)? > 0
        && ctx.items().count(2221)? > 0
    {
        ctx.lines_as(
            "Han Garam",
            args![
                "You came back faster than I expected.",
                "Great, I am also highly motivated for the fact that",
                "my Magistrate Hat will belong to someone like you,",
                "who are competent and trustworthy."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Han Garam",
            args![
                "Okay then, I will make the hat as quickly as I can.",
                "Now, hand me all the materials."
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Give him the items.", "Don't give him the items."])? == 0 {
            ctx.lines_as(
                "Han Garam",
                args![
                    "Excellent! I don't have to be mediumized",
                    "by the Dragon God for this work",
                    "because I can perfectly do it on my own.",
                    "So, hold it right there, okay?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Han Garam", args!["Yes...right...umm hmm."])?;
            ctx.next()?;
            ctx.lines_as("Han Garam", args!["................."])?;
            ctx.next()?;
            ctx.lines(args![
                "- Han Garam started hammering, -",
                "- and assembling the materials without a word. -"
            ])?;
            ctx.next()?;
            ctx.lines_as("Han Garam", args!["................."])?;
            ctx.next()?;
            ctx.lines_as(
                "Han Garam",
                args!["Phew, it was quite difficult,", "as I had not done this for quite a while."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Han Garam",
                args![
                    "I still feel good that I had a chance",
                    "to exercise my skills, you know. It was fun, too.",
                    "Please take this Magistrate Hat.",
                    "As I promised, this is my gift for you."
                ],
            )?;
            ctx.next()?;
            ctx.items().take(954, 300)?;
            ctx.items().take(733, 5)?;
            ctx.items().take(983, 1)?;
            ctx.items().take(2221, 1)?;
            ctx.var("moza_korea").set(0)?;
            ctx.items().give(5173, 1)?;
            ctx.lines_as(
                "Han Garam",
                args![
                    "Now I need to meet with the Dragon God again,",
                    "if I want to make another hat...umm...",
                    "Oh well, somehow I was able to make one this time with your help,",
                    "so I guess that it will happen when the time is right."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Han Garam",
                args![
                    "Thank you so much! Please take my Magistrate Hat with care,",
                    "and be proud that you are the owner of Magistrate Hat!"
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Han Garam",
            args!["...? What? Do you need more time?", "Can't you just give them to me already?"],
        )?;
        return ctx.close();
    } else if ctx.var("moza_korea").get()? == 2 {
        ctx.lines_as(
            "Han Garam",
            args![
                "To make Ayam, I need",
                "^FF0000300 Shining Scale^000000, ^FF00005 Cracked Diamond^000000,",
                "^FF00001 Scarlet Dyestuffs^000000, and",
                "^FF000050 Soft Silk^000000!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Han Garam",
            args![
                "To make Magistrate Hat, I need",
                "^FF0000300 Shining Scale^000000, ^FF00005 Cracked Diamond^000000,",
                "^FF00001 Black Dyestuffs^000000, and ^FF00001 Slotted Hat^000000!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Han Garam",
            args![
                "...That's what I said.",
                "So, choose a hat, and bring me",
                "its materials without missing anything."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Han Garam",
            args![
                "Ah~ it's been a while since the last time that I felt the Dragon God in me.",
                "I felt like that I have learned something unworldly."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("moza_korea").get()? == 1 {
        ctx.lines_as("Han Garam", args!["............."])?;
        ctx.next()?;
        ctx.lines_as("Han Garam", args!["............."])?;
        ctx.next()?;
        ctx.lines_as("Han Garam", args!["............."])?;
        ctx.next()?;
        ctx.lines_as(
            "Han Garam",
            args![
                "...*Tremble Tremble*...He...He's here...He's here!",
                "*Tremble Tremble* I can feel",
                "the Dragon God inside of me! Waaah!",
                "He is waving his tail, and showing me future!",
                "Aaaaaahhhhhhh!"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, args![constants::EF_LORD])?;
        ctx.lines_as("Han Garam", args!["Waaaahhhh!"])?;
        ctx.next()?;
        ctx.lines_as("Han Garam", args!["............."])?;
        ctx.next()?;
        ctx.lines_as("Han Garam", args!["............."])?;
        ctx.next()?;
        ctx.lines_as(
            "Han Garam",
            args![
                "Yes... yes, I can! I can make ^FF0000Ayam^000000",
                "and ^FF0000Magistrate Hat^000000 now!",
                "Oh, I can see how to make them,",
                "I know what I need to make them! Everything is in my vision!",
                "I feel like that I am alreadying making one!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Han Garam",
            args![
                "To make Ayam, I need",
                "^FF0000300 Shining Scale^000000, ^FF00005 Cracked Diamond^000000,",
                "^FF00001 Scarlet Dyestuffs^000000, and",
                "^FF000050 Soft Silk^000000!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Han Garam",
            args![
                "To make Magistrate Hat, I need",
                "^FF0000300 Shining Scale^000000, ^FF00005 Cracked Diamond^000000,",
                "^FF00001 Black Dyestuffs^000000, and ^FF00001 Slotted Hat^000000!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Han Garam",
            args!["Anyone who brings these, I will create hat for him!", "Ahhh~ Dragon God!"],
        )?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, args![constants::EF_EXIT])?;
        ctx.lines_as("Han Garam", args!["............."])?;
        ctx.next()?;
        ctx.lines_as("Han Garam", args!["............."])?;
        ctx.next()?;
        ctx.lines_as(
            "Han Garam",
            args![
                "Phew... Dragon God has gone now.",
                "However, I still clearly remember",
                "what I need, and how to make the hats.",
                "So, you don't have to worry about that.",
                "You know...are they Ayam and Magistrate Hat?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Han Garam",
            args![
                "You heard what materials I need, don't you?",
                "Choose a hat, and bring me its materials without missing anything."
            ],
        )?;
        ctx.next()?;
        ctx.var("moza_korea").set(2)?;
        ctx.lines_as("Han Garam", args!["Okay then, I will be waiting you to come back."])?;
        return ctx.close();
    }
    if ctx.items().count(7446)? > 6 && ctx.items().count(7448)? > 6 && ctx.items().count(7445)? > 6 && ctx.items().count(7447)? > 6 {
        ctx.lines_as(
            "Han Garam",
            args![
                "My name is Han Garam,",
                "and I am a proud heir of a renown family... eh?",
                "Wait, I feel something mysteriously familiar from you...",
                "Hey, do you have 7 Bijous for each of the 4 Bijou colors?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Han Garam",
            args![
                "...Wow! This is crazy! You do have them, don't you?",
                "Wow, how did you gather all of them? They are so hard to find..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Han Garam", args!["........Umm."])?;
        ctx.next()?;
        ctx.lines_as(
            "Han Garam",
            args![
                "Hey, if you don't mind, can I have them?",
                "In fact, I am the only traditional hat craftsman",
                "in this Rune-Midgarts Kingdom.",
                "However, I have recently gotten into trouble,",
                "and have not been able to focus on my business."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Han Garam",
            args![
                "The trouble is that I have ran out of those Bijous.",
                "You know, those colorful beads that you have.",
                "I could restart my business again,",
                "only if I have them...*Sigh*"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Han Garam",
            args![
                "Can you please let me have them?",
                "I will pay you back with one of my proud traditional hats.",
                "Well, since I am the only one who can make them,",
                "in fact, you are not doing a losing business with me, don't you think?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Han Garam", args!["Please? I assure you that I can make traditional hats."])?;
        ctx.next()?;
        if ctx.menu(&["Give him the items.", "Don't give him the items."])? == 0 {
            ctx.lines_as(
                "Han Garam",
                args![
                    "Wow, wow, thank you so much!",
                    "As I promised, I will make you a hat in return.",
                    "But, can you wait for a while?",
                    "To make hat, I have to contact the Dragon God first."
                ],
            )?;
            ctx.next()?;
            ctx.items().take(7446, 7)?;
            ctx.items().take(7448, 7)?;
            ctx.items().take(7445, 7)?;
            ctx.items().take(7447, 7)?;
            ctx.var("moza_korea").set(1)?;
            ctx.lines_as("Han Garam", args!["...Okay, give me some time to focus."])?;
            return ctx.close();
        }
        ctx.lines_as(
            "Han Garam",
            args![
                "Bah, that's your choice.",
                "If you change your mind, tell me immediately though.",
                "You should know that",
                "those things belong to someone else, not you."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Han Garam",
        args![
            "I am Han Garam, and a heir of a formerly renown family.",
            "Do you want to know why I say ''formerly renown''?",
            "It is because my grandfather lost my family's wealth,",
            "and now we are just like other ordinary families, hahahaha!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Han Garam",
        args![
            "I have inherited nothing but this traditional hat making skill.",
            "Even then, it is a quite extraordinary inheritance,",
            "since I am the only one in the Rune-Midgarts Kingdom",
            "who can make traditional hats."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Han Garam",
        args![
            "Unfortunately, I haven't been able to",
            "exercise my skill because I haven't met with the Dragon God...",
            "Eh? What do I mean, you ask?",
            "You know what mediums do, right?",
            "They receive spirits into their bodies,",
            "and communicate with them."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Han Garam",
        args![
            "So, in my case, a Dragon God comes into my body,",
            "and gives me strength and wisdom.",
            "When my family was wealthy,",
            "we had enough invocation materials",
            "to summon the Dragon God,",
            "but now, as you see,",
            "we cannot afford such expensive things."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Han Garam",
        args![
            "Basically, I may have inherited my family's heirloom,",
            "but I can't use it",
            "because I cannot afford buying the invocation materials."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Han Garam",
        args![
            "To summon the Dragon God,",
            "I must have ^FF00007 Bijous for each of the 4 Bijou colors^000000.",
            "^FF0000I need 7 Bijous for each of the Blue, ",
            "^FF0000Yellow, Green, and Red colors^000000.",
            "Basically I need total 28 Bijous...umm."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Han Garam",
        args![
            "As I said earlier, not only they are expensive,",
            "but also they are hard to find...",
            "*Sigh* I am afraid that my family's heirloom skill",
            "might be discontinued at my generation..."
        ],
    )?;
    ctx.close()
}
