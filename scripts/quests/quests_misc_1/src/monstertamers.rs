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

pub fn monster_tamer_alb(ctx: &Ctx) -> Script {
    ctx.lines_as("Iwado", args!["This is the height of the era of Monster Taming! Don't you feel the need to gather monster items to be able to connect and to communicate with the Cute Monsters!?"])?;
    ctx.next()?;
    match ctx.menu(&[
        "^3355FF' Monster Taming '^000000 ?",
        "Order ^3355FF' Monster Juice '^000000",
        "Order ^3355FF' Singing Flower '^000000",
        "Order ^3355FF' Wild Flower '^000000",
        "Cancel",
    ])? {
        0 => {
            ctx.lines_as("Iwado", args!["These monster items, necessary to communicate with monsters, are a must have! We, the members of the Monster Tamer Guild, have a very reasonable offer."])?;
            ctx.next()?;
            ctx.lines_as(
                "Iwado",
                args![
                    "We have a system that allows you",
                    "to get your hands on these",
                    "cool items without burden!",
                    "Monster Juice !",
                    "Singing Flower !",
                    "Aaand....Wild Flower !"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Iwado",
                args![
                    "To make",
                    "Monster Juice, you need",
                    "^3355FF1 Animal Gore^000000 and",
                    "^3355FF2 Apple^000000."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Iwado",
                args!["For 1 Singing Flower,", "you will need", "^3355FF1 Singing Plant^000000."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Iwado",
                args![
                    "If you want me to make",
                    "1 Wild Flower,",
                    "just give me",
                    "^3355FF1 Fancy Flower^000000 and",
                    "^3355FF1 Clover^000000."
                ],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Iwado",
                args![
                    "This is an item that is far beyond",
                    "anything we humans can drink!",
                    "Its name? ^3355FFMonster Juice^000000 !!",
                    "The extremely sour taste",
                    "is perfect for monsters. . ."
                ],
            )?;
            ctx.next()?;
            if ctx.items().count(702)? > 0 && ctx.items().count(512)? > 1 {
                ctx.lines_as("Iwado", args!["Oh ! You have all the items to make 1 Monster Juice! Good, good! With all my pride as a monster tamer, I will make one for you right now."])?;
                ctx.next()?;
                ctx.lines_as("Iwado", args!["Got to twist these apples...", "Just right..."])?;
                ctx.next()?;
                ctx.mes("^3355FF*Splash splash swoosh swhoosh splash splash*^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "Iwado",
                    args![
                        "Okay. All done!",
                        "Please take it for the",
                        "cute monster you",
                        "are raising.",
                        "Monster Juice!"
                    ],
                )?;
                ctx.items().take(512, 2)?;
                ctx.items().take(702, 1)?;
                ctx.items().give(626, 1)?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "Iwado",
                    args![
                        "For the monster owner",
                        "that loves to use monster juice...",
                        "I have mastered the recipe for Monster Juice!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Iwado",
                    args![
                        "If you bring me",
                        "^3355FF1 Animal Gore^000000 and",
                        "^3355FF2 Apple^000000",
                        "With all my pride as a monster tamer, I will make it for you!"
                    ],
                )?;
                return ctx.close();
            }
        }
        2 => {
            ctx.lines_as(
                "Iwado",
                args![
                    "A flower with a song that is",
                    "far beyond human comprehension!",
                    "Its name? ^3355FFSinging Flower^000000!!",
                    "A scent that makes you dizzy with pleasure. A perfect item for monsters..."
                ],
            )?;
            ctx.next()?;
            if ctx.items().count(707)? > 0 {
                ctx.lines_as("Iwado", args!["Oh! You have all the items necessary to make 1 Singing Flower!! Good, good! With all my pride as a monster tamer, I will make it for you right away."])?;
                ctx.next()?;
                ctx.lines_as("Iwado", args!["Got to...", "Twist this...", "Just right..."])?;
                ctx.next()?;
                ctx.mes("^3355FF*Crumble crumble scratch scratch*^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "Iwado",
                    args![
                        "Okay. All done!",
                        "Please take it for the",
                        "cute monster you",
                        "are raising.",
                        "Singing Flower!"
                    ],
                )?;
                ctx.items().take(707, 1)?;
                ctx.items().give(629, 1)?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "Iwado",
                    args![
                        "For the monster owner",
                        "that loves to use Singing Flower...",
                        "I provide a way to make",
                        "1 Singing Flower!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Iwado",
                    args![
                        "If you bring me",
                        "^3355FF1 Singing Plant ^000000,",
                        "With all my pride as a monster tamer, I will make one for you right away!"
                    ],
                )?;
                return ctx.close();
            }
        }
        3 => {
            ctx.lines_as("Iwado", args!["This is a flower that is far beyond what humans can gaze at! Its name? ^3355FFWild Flower^000000 !! A troublesome design, but perfect for monsters!"])?;
            ctx.next()?;
            if ctx.items().count(2207)? > 0 && ctx.items().count(705)? > 0 {
                ctx.lines_as("Iwado", args!["Oh! You have all the items necessary to make 1 Wild Flower! Good! Good! With all my pride as a monster tamer, I will make one for you right away."])?;
                ctx.next()?;
                ctx.lines_as("IWado", args!["Got to twist this...", "Just right..."])?;
                ctx.next()?;
                ctx.mes("^3355FF*Crumble crumble rip rip*^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "Iwado",
                    args![
                        "Okay. All done!",
                        "Please take it for the",
                        "cute monsters you",
                        "are raising.",
                        "Wild Flower!"
                    ],
                )?;
                ctx.items().take(2207, 1)?;
                ctx.items().take(705, 1)?;
                ctx.items().give(10009, 1)?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "Iwado",
                    args![
                        "For the monster owner",
                        "that loves to use Wild Flower...",
                        "I provide a way to make",
                        "1 Wild Flower!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Iwado",
                    args![
                        "^3355FF1 Fancy Flower^000000 and",
                        "^3355FF1 Clover^000000",
                        "is all I need! If you bring those to me, I will make one for you with all my pride as a monster tamer!"
                    ],
                )?;
                return ctx.close();
            }
        }
        4 => {
            ctx.lines_as(
                "Iwado",
                args!["Ah...!", "You must have not decided which Monster you want to raise."],
            )?;
            ctx.next()?;
            ctx.lines_as("Iwado", args!["Definitely not a decision to be made carelessly! Your pet monster will accompany you throughout your life. Please take your time and return when you have decided."])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn monster_tamer_alde(ctx: &Ctx) -> Script {
    ctx.lines_as("YuU", args!["You must be looking for monster items! In making monster items, there is no monster tamer in Rune-Midgarts that's better than me."])?;
    ctx.next()?;
    ctx.lines_as(
        "YuU",
        args!["I can make all sorts of neat things for your Cute Pet monster.", "Just ask~"],
    )?;
    ctx.next()?;
    match ctx.menu(&[
        "^3355FF' Monster Taming '^000000?",
        "Order ^3355FF' Skull Helm '^000000",
        "Order ^3355FF' Monster Oxygen Mask '^000000",
        "Order ^3355FF' Silk Ribbon '^000000",
        "Order ^3355FF' Stellar Hairpin '^000000",
        "Order ^3355FF' Tiny Egg Shell '^000000",
        "Order ^3355FF' Rocker Glasses '^000000",
        "Cancel",
    ])? {
        0 => {
            ctx.lines_as(
                "YuU",
                args![
                    "The monster items that are",
                    "necessary to communicate",
                    "with monsters...",
                    "These are a must have!",
                    "We Monster Tamer guild members'",
                    "have a very reasonable offer."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "YuU",
                args![
                    "We provide a system that allows you to get your hands on these cool items with less of the hassle!",
                    "Silk Ribbon !",
                    "Monster Oxygen Mask !",
                    "Skull Helm !"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("YuU", args!["Stellar Hairpin !", "Tiny Egg Shell !!", "Rocker Glasses !!!"])?;
            ctx.next()?;
            ctx.lines_as("YuU", args!["1 Skull Helm requires", "1 ^3355FFBone Helm^000000"])?;
            ctx.next()?;
            ctx.lines_as("YuU", args!["1 Monster Oxygen Mask requires", "1 ^3355FFOxygen Mask^000000."])?;
            ctx.next()?;
            ctx.lines_as("YuU", args!["1 Silk Ribbon requires", "1 ^3355FFRibbon^000000."])?;
            ctx.next()?;
            ctx.lines_as("YuU", args!["1 Stellar Hairpin requires", "1 ^3355FFStellar^000000."])?;
            ctx.next()?;
            ctx.lines_as("YuU", args!["1 Tiny Egg Shell requires", "1 ^3355FFEgg Shell^000000."])?;
            ctx.next()?;
            ctx.lines_as(
                "YuU",
                args![
                    "1 Rocker Glassess requires",
                    "^3355FF400 Zeny^000000,",
                    "2 ^3355FFZargon^000000 and",
                    "1 ^3355FFRibbon^000000. "
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "YuU",
                args!["Try your best to find these items so that you can raise a Cute Monster! Good luck!"],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as("YuU", args!["This is an item far beyond what humans can wear! Its name? ^3355FFSkull Helm^000000! Disgusting on humans, but cute on monsters."])?;
            ctx.next()?;
            if ctx.items().count(5017)? > 0 {
                ctx.lines_as("YuU", args!["Oh! You have all the items necessary to make Skull Helm! Good, good! With all my pride as a monster tamer, I will make it for you right away."])?;
                ctx.next()?;
                ctx.lines_as("YuU", args!["Got to rub", "this Bone Helm", "just right..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "YuU",
                    args![
                        "Okay. All done!",
                        "Please take this for",
                        "the cute monster you",
                        "are raising.",
                        "Skull Helm!"
                    ],
                )?;
                ctx.items().take(5017, 1)?;
                ctx.items().give(10001, 1)?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "YuU",
                    args!["For the monster owner that loves to spoil his or her monster, we provide one way to make a Skull Helm."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "YuU",
                    args![
                        "Just bring me...",
                        "^3355FF1 Bone Helm^000000!",
                        "With all my pride as a monster tamer, I will make it for you!"
                    ],
                )?;
                return ctx.close();
            }
        }
        2 => {
            ctx.lines_as("YuU", args!["This item is far beyond what humans can wear. Its name? ^3355FFMonster Oxygen Mask^000000! Show your monster that you care with this special gift."])?;
            ctx.next()?;
            if ctx.items().count(5004)? > 0 {
                ctx.lines_as("YuU", args!["Oh! You have all the items necessary to make 1 Monster Oxygen Mask! Good, Good! With all my pride as a monster tamer, I will make it for you right away."])?;
                ctx.next()?;
                ctx.lines_as("YuU", args!["Got to pull apart", "this Oxygen Mask", "just right..."])?;
                ctx.next()?;
                ctx.mes("^3355FF*Flip flop... Zowie!*^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "YuU",
                    args![
                        "Okay. All done.",
                        "Please take it for",
                        "the cute monster you",
                        "are raising...",
                        "Monster Oxygen Mask!"
                    ],
                )?;
                ctx.items().take(5004, 1)?;
                ctx.items().give(10002, 1)?;
                return ctx.close();
            } else {
                ctx.lines_as("YuU", args!["For the monster owner that wants to pamper his or her monster with its very own breathing apparatus, we provide a way to create 1 Monster Oxygen Mask!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "YuU",
                    args![
                        "Bring me...",
                        "^3355FF1 Oxygen Mask^000000.",
                        "With all my pride as a monster tamer, I will make it for you right away!"
                    ],
                )?;
                return ctx.close();
            }
        }
        3 => {
            ctx.lines_as("YuU", args!["This is not an item that a normal human would dare to wear! Its name? ^3355FFSilk Ribbon^000000! Its perfect for monsters!"])?;
            ctx.next()?;
            if ctx.items().count(2208)? > 0 {
                ctx.lines_as("YuU", args!["Oh! You have all the items necessary to make 1 Silk Ribbon! Good, good! With all my pride as a monster tamer, I will make it for you right away!"])?;
                ctx.next()?;
                ctx.lines_as("YuU", args!["Got to unravel", "this ribbon", "just right..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "YuU",
                    args![
                        "Okay. All done.",
                        "Please take it for",
                        "the cute monster you",
                        "are raising.",
                        "Silk Ribbon!"
                    ],
                )?;
                ctx.items().take(2208, 1)?;
                ctx.items().give(10007, 1)?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "YuU",
                    args!["For the monster owner that is fond of elegantly dressing their pets, we provide a way to create 1 Silk Ribbon!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "YuU",
                    args![
                        "Just bring me...",
                        "^3355FF1 Ribbon^000000!",
                        "with all my pride as a monster tamer, I will make it for you!"
                    ],
                )?;
                return ctx.close();
            }
        }
        4 => {
            ctx.lines_as("YuU", args!["This is an item far beyond the fashion sense of humans! Its name? ^3355FFStellar Hairpin^000000! It's dangerous for humans to wear, but fashionable of monsters."])?;
            ctx.next()?;
            if ctx.items().count(2294)? > 0 {
                ctx.lines_as("YuU", args!["Oh! You have all the items necessary to make 1 Stellar! Good, good! With all my pride as a monster tamer, I will make it for you right away."])?;
                ctx.next()?;
                ctx.lines_as("YuU", args!["Now, if I can", "only peel this", "without breaking it..."])?;
                ctx.next()?;
                ctx.mes("^3355FF*Shine shine*^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "YuU",
                    args![
                        "Okay. All done.",
                        "Please take it for",
                        "the cute monster you",
                        "are raising",
                        "Stellar Hairpin!"
                    ],
                )?;
                ctx.items().take(2294, 1)?;
                ctx.items().give(10011, 1)?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "YuU",
                    args!["For the monster owner that want the best for their cute pet, we provide a wait to create 1 Stellar Hairpin!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "YuU",
                    args![
                        "Just bring me...",
                        "^3355FF 1 Stellar !^000000",
                        "with all my pride as a monster tamer, I will make it for you!"
                    ],
                )?;
                return ctx.close();
            }
        }
        5 => {
            ctx.lines_as("YuU", args!["This is far beyond what the headwear that humans can don. Its name? ^3355FFTiny Egg Shell^000000! It has the shine of marble and the glow of youth!"])?;
            ctx.next()?;
            if ctx.items().count(5015)? > 0 {
                ctx.lines_as("YuU", args!["Oh! You have all the items necessary to make 1 Tiny Egg Shell! Good, good! With all my pride as a monster tamer, I will make it for you right away!"])?;
                ctx.next()?;
                ctx.lines_as("YuU", args!["Got to", "shatter this", "just right..."])?;
                ctx.next()?;
                ctx.mes("^3355FF*Scrub scrub squeeze squeeze*^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "YuU",
                    args![
                        "Okay. All done.",
                        "Please take it for",
                        "the cute monster you",
                        "are raising",
                        "Tiny Egg Shell!"
                    ],
                )?;
                ctx.items().take(5015, 1)?;
                ctx.items().give(10012, 1)?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "YuU",
                    args![
                        "For the monster owner that is fond of eggs and fonder of egg shells, we provide a way to create 1 Tiny Egg Shell!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "YuU",
                    args![
                        "Just bring me...",
                        "^3355FF1 Tiny Egg Shell^000000!",
                        "with all my pride as a monster tamer, I will make it for you!"
                    ],
                )?;
                return ctx.close();
            }
        }
        6 => {
            ctx.lines_as("YuU", args!["Ah yes, this is eyewear far beyond the glasses of normal humans. Its name? ^3355FFRocker Glasses^000000! Suave eyewear for suave monsters~"])?;
            ctx.next()?;
            if ctx.items().count(912)? > 1 && ctx.items().count(2208)? > 0 && ctx.player().zeny()? > 399 {
                ctx.lines_as("YuU", args!["Oh! You have all the items necessary to make 1 Rocker Glasses! Good, good! With all my pride as a monster tamer, I will make it for you right away!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "YuU",
                    args!["Okay, now all I gotta do is take this Zargon and, um, make glasses lenses out of them. Somehow."],
                )?;
                ctx.next()?;
                ctx.mes("^3355FF*Squeak squeak crush snap*^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "YuU",
                    args![
                        "Whew! Lucky!",
                        "Somehow I did it.",
                        "Please take this for",
                        "the cute monster you",
                        "are raising.",
                        "Rocker Glasses!"
                    ],
                )?;
                ctx.items().take(912, 2)?;
                ctx.items().take(2208, 1)?;
                ctx.player().set_zeny(ctx.player().zeny()? - 400)?;
                ctx.items().give(10014, 1)?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "YuU",
                    args!["For the monster owner that likes monster glasses, we provide a way to make Rocker Glasses!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "YuU",
                    args![
                        "Just bring me...",
                        "^3355FF2 Zargon^000000",
                        "^3355FF1 Ribbon^000000 and",
                        "^3355FF400 zeny^000000. ",
                        "I will make it for you, with all my pride as a monster tamer!"
                    ],
                )?;
                return ctx.close();
            }
        }
        7 => {
            ctx.lines_as(
                "YuU",
                args!["Ah...!", "You must have not decided which Monster you want to raise."],
            )?;
            ctx.next()?;
            ctx.lines_as("YuU", args!["Definitely not a decision to be made carelessly! Your pet monster will accompany you throughout your life. Please take your time and return when you have decided."])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn monstertamer_izlude(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Shogo",
        args![
            "Are you pre-occupied with",
            "gathering items for monsters",
            "and want a break? ",
            "We will gather items",
            "for your Cute Pets."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&[
        "^3355FF' Monster Taming '^000000?",
        "Order ^3355FF' Book of Devil '^000000",
        "Order ^3355FF' No Recipient '^000000",
        "Order ^3355FF' Orc Trophy '^000000",
        "Cancel",
    ])? {
        0 => {
            ctx.lines_as("Shogo", args!["The monster taming items are necessary to communicate with the monster of your choice! We Monster Tamer guild members have a very reasonable offer..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Shogo",
                args![
                    "We provide a system that allows you to get your hands on these cool items with less of the hassle!",
                    "Book of Devil!",
                    "No Recipient!",
                    "Orc Trophy!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Shogo",
                args![
                    "For 1 Book of Devil, you can exchange:",
                    "^3355FF1 Old Magic Book^000000",
                    "^3355FF2 Horrendous Mouth^000000."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Shogo",
                args!["For 1 No Recipient, you can exchange ^3355FF1 Old Portrait^000000."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Shogo",
                args![
                    "Lastly, you can exchange",
                    "^3355FF1 Chivalry Emblem^000000 and",
                    "^3355FF1 Scorpion Tail^000000",
                    "for 1 Orc Trophy.",
                    "Try these fantastic items!"
                ],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as("Shogo", args!["This is an item far beyond what humans can read! Its name? ^3355FFBook of Devil^000000! A very mysterious item that can supposedly summon demons..."])?;
            ctx.next()?;
            if ctx.items().count(1006)? > 0 && ctx.items().count(958)? > 1 {
                ctx.lines_as("Shogo", args!["Oh! You have all the items necessary to make 1 Book of Devil! Good, good! With all my pride as a monster tamer, I will make one for you right away."])?;
                ctx.next()?;
                ctx.lines_as("Shogo", args!["Let's see...", "Hmm, this is going to be hard."])?;
                ctx.next()?;
                ctx.mes("^3355FF*Knock knock scrape scrape*^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "Shogo",
                    args![
                        "Okay. All done.",
                        "Please take it for",
                        "the cute monster you",
                        "are raising.",
                        "Book of Devil!"
                    ],
                )?;
                ctx.items().take(958, 2)?;
                ctx.items().take(1006, 1)?;
                ctx.items().give(642, 1)?;
                return ctx.close();
            } else {
                ctx.lines_as("Shogo", args!["For the owner that wishes to raise a monster that can be summoned using the Book of Devil, we provide a way to create 1 Book of Devil!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shogo",
                    args![
                        "Just bring me...",
                        "^3355FF1 Old Magicbook^000000 and",
                        "^3355FF2 Horrendous Mouth^000000!",
                        "If you bring me these items, with all my pride as a monster tamer, I will make it for you!"
                    ],
                )?;
                return ctx.close();
            }
        }
        2 => {
            ctx.lines_as("Shogo", args!["This is an item which expresses feelings beyond what a human can feel. Its name? ^3355FFNo Recipient^000000! Who sent this letter, and who was supposed to receive it?"])?;
            ctx.next()?;
            if ctx.items().count(7014)? > 0 {
                ctx.lines_as("Shogo", args!["Oh! You have all the items necessary to make No Recipient! Good, good! With all my pride as a monster tamer, I will make one for you right away."])?;
                ctx.next()?;
                ctx.lines_as("Shogo", args!["Got to fold this Old Portrait just right..."])?;
                ctx.next()?;
                ctx.mes("^3355FF*Scrape scrape brush brush*^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "Shogo",
                    args![
                        "Okay. All done.",
                        "Please take this item",
                        "for the cute monster",
                        "you are raising.",
                        "No Recipient!"
                    ],
                )?;
                ctx.items().take(7014, 1)?;
                ctx.items().give(636, 1)?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "Shogo",
                    args!["For the monster owner that is fond of No Recipient, we provide a way to create to special item!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shogo",
                    args![
                        "Just bring me...",
                        "^3355FF1 Old Portrait^000000!",
                        "If you bring this to me, with all my pride as a monster tamer, I will make it for you!"
                    ],
                )?;
                return ctx.close();
            }
        }
        3 => {
            ctx.lines_as(
                "Shogo",
                args![
                    "This is an item far beyond the pride of humans!",
                    "Its name?",
                    "^3355FFOrc Trophy^000000!"
                ],
            )?;
            ctx.next()?;
            if ctx.items().count(1004)? > 0 && ctx.items().count(904)? > 0 {
                ctx.lines_as("Shogo", args!["Oh! You have all the items necessary to make an Orc Trophy! Good, good! With all my pride as a monster tamer, let me make one for you right away!"])?;
                ctx.next()?;
                ctx.lines_as("Shogo", args!["Got to...", "Fit this Scorpion Tail...", "Just right..."])?;
                ctx.next()?;
                ctx.mes("^3355FF*Clang clang Boong*^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "Shogo",
                    args![
                        "Okay. All done.",
                        "Please take this for",
                        "the cute monster",
                        "you are raising...",
                        "Orc Trophy!"
                    ],
                )?;
                ctx.items().take(904, 1)?;
                ctx.items().take(1004, 1)?;
                ctx.items().give(635, 1)?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "Shogo",
                    args![
                        "For the monster owner",
                        "who is fond of Horror of Tribe...",
                        "One way to make Horror of Tribe !",
                        "^3355FF 1 Chivalry Emblem^000000!",
                        "^3355FF 1 Scorpion Tail^000000!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shogo",
                    args![
                        "If you bring these items to me,",
                        "with all my pride as a monster",
                        "tamer, I will make it for you!"
                    ],
                )?;
                return ctx.close();
            }
        }
        4 => {
            ctx.lines_as(
                "Shogo",
                args!["Ah...!", "You must have not decided which Monster you want to raise."],
            )?;
            ctx.next()?;
            ctx.lines_as("Shogo", args!["Definitely not a decision to be made carelessly! Your pet monster will accompany you throughout your life. Please take your time and return when you have decided."])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn munak_s_grandma(ctx: &Ctx) -> Script {
    if ctx.items().count(1558)? > 0 {
        ctx.lines_as("Munak's grandma", args!["Oh my...", "Have you seen my granddaughter,"])?;
        if ctx.var("Sex").get()? == constants::SEX_MALE {
            ctx.mes("boy? My poor granddaughter")?;
        } else {
            ctx.mes("young lady? My poor granddaughter")?;
        }
        ctx.mes("has been missing...")?;
        ctx.next()?;
        ctx.lines_as(
            "Munak's Grandma",
            args![
                "I can't remember exactly when it",
                "was, but when I lived in Payon, I",
                "had a cute granddaughter. She",
                "was really happy when I made her",
                "hair like ^000077Danggie^000000..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Munak's Grandma",
            args![
                "She was alwaying working with the",
                "village chief... She was such a sweet",
                "girl, and always got along with",
                "chief's son..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Munak's Grandma",
            args![
                "But one day our deity became angry",
                "and cursed the chief's son with a",
                "sickness! The village had to offer",
                "my granddaughter to him as a",
                "companion to lift the curse..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Munak's Grandma",
            args![
                "The chief's son regained his",
                "health, but I lost my",
                "granddaughter! I can't look at him",
                "and not think of her, so I tried to",
                "leave my misery behind and came",
                "here to Comodo..."
            ],
        )?;
        ctx.next()?;
        match ctx.menu(&["Oh come on! Cheer up!", "Um, is this diary...?"])? {
            0 => {
                ctx.lines_as(
                    "Munak's Grandma",
                    args![
                        "It seems my granddaughter haunts my",
                        "dreams every night. I believe I've",
                        "been trying to cheer up for years",
                        "now..."
                    ],
                )?;
                return ctx.close();
            }
            1 => {
                if ctx.items().count(901)? > 0 {
                    ctx.lines_as(
                        "Munak's Grandma",
                        args!["Oh god!", "It's my granddaughter's diary!", "Th-This is her writing! Oh my...!"],
                    )?;
                    ctx.next()?;
                    ctx.mes("[Munak's Grandma]")?;
                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                        ctx.mes("Young man... I'll read this to you if")?;
                    } else {
                        ctx.mes("Young lady... I'll read this to you if")?;
                    }
                    ctx.lines(args![
                        "you give it to me with a Danggie,",
                        "please. I no longer have anything",
                        "that belonged to her now..."
                    ])?;
                    ctx.next()?;
                    match ctx.menu(&["No way.", "Ok, I will."])? {
                        0 => {
                            ctx.lines_as("Munak's Grandma", args!["Oh...?"])?;
                            if ctx.var("Sex").get()? == constants::SEX_MALE {
                                ctx.lines(args!["Alright, young man.", "Thank you anyway."])?;
                            } else {
                                ctx.lines(args!["Thank you anyway,", "young lady"])?;
                            }
                            ctx.next()?;
                            ctx.lines_as(
                                "Munak's Grandma",
                                args!["It's alright...", "I can only hope that the deity is", "taking good care of her!"],
                            )?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.items().take(1558, 1)?;
                            ctx.items().take(901, 1)?;
                            ctx.mes("[Munak's Grandma]")?;
                            if ctx.var("Sex").get()? == constants::SEX_MALE {
                                ctx.lines(args!["Oh!", "Thank you,", "young man~!"])?;
                            } else {
                                ctx.lines(args!["Goodness!", "Thank you,", "young lady..."])?;
                            }
                            ctx.next()?;
                            ctx.lines_as("Munak's Grandma", args!["Alright...", "I'll read this.", "Let me see..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Munak's Grandma",
                                args![
                                    "It seems that my granddaughter was",
                                    "treated by the chief like his own",
                                    "child! There are so many happy",
                                    "memories in this book!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.items().give(659, 1)?;
                            ctx.lines_as(
                                "Munak's Grandma",
                                args![
                                    "Oh, thank you. I now have a good",
                                    "keepsake of my granddaughter. Thank",
                                    "you so much! May God-Poing bless",
                                    "you!"
                                ],
                            )?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                } else {
                    ctx.lines_as(
                        "Munak's Grandma",
                        args!["What? A diary? What's that diary...?", "I don't... I can't remember, oh my..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Munak's Grandma",
                        args![
                            "I really wish I could make her hair",
                            "into a ^000077Danggie^000000 again. I really",
                            "wish... Oh, oh my granddaughter..."
                        ],
                    )?;
                    return ctx.close();
                }
            }
            _ => {}
        }
    } else {
        ctx.lines_as(
            "Munak's Grandma",
            args!["My own granddaughter...", "Why did she have to leave...?"],
        )?;
        return ctx.close();
    }
    Ok(())
}
