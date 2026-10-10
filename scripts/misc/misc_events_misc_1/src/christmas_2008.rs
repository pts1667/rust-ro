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

pub fn caroller_iroxmas08(ctx: &Ctx) -> Script {
    if ctx.var("iroxmas08carol").get()?.number()? < 1 || ctx.var("iroxmas08carol").get()? == 3 {
        if ctx.var("iroxmas08carol").get()? == 3 {
            ctx.lines_as(
                "Caroller",
                args![
                    "Jingle Bells! Jingle Bells! Jingle all the way!",
                    "O' what fun it is to ride in a one-horse open sleeeigh, Hey!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Caroller",
                args!["Merry Christmas!", "Hey! You! What comes to mind when you think about Christmas?"],
            )?;
            ctx.next()?;
        } else {
            ctx.lines_as("Caroller", args!["Jingle Bells! Jingle Bells! Jingle all the way!"])?;
            ctx.next()?;
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_GLORIA])?;
            ctx.lines_as("Caroller", args!["O' what fun it is to ride in a one-horse open sleigh!"])?;
            ctx.next()?;
            ctx.lines_as("Caroller", args!["Merry Christmas!"])?;
            if ctx.var("Sex").get()? == constants::SEX_MALE {
                ctx.mes("Hey, boy! What comes to mind when")?;
            } else {
                ctx.mes("Hey, girl! What comes to mind when")?;
            }
            ctx.mes("you think about Christmas?")?;
            ctx.next()?;
        }
        match ctx.menu(&["Santa Claus", "Gift Boxes", "Carols", "Santa Costumes", "Fake Santa Antonio"])? {
            0 => {
                ctx.lines_as("Caroller", args!["Santa Claus!", "You're so innocent!!", "Ah!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Caroller",
                    args![
                        "Here's a secret!",
                        "There's a rumor that Santa Claus lives in a certain village all throughout the year."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Caroller",
                    args![
                        "But now!!!",
                        "In this Christmas season!!",
                        "You guys can meet Santa on either of the five possible villages throughout Rune-Midgerts!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Caroller",
                    args!["If you win over Santa, you can get a gift. Would you go for it?"],
                )?;
                ctx.next()?;
                ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
                ctx.lines_as("Caroller", args!["Caroller's hot news! Ha!", "Isn't that big news?"])?;
                return ctx.close();
            }
            1 => {
                ctx.lines_as(
                    "Caroller",
                    args!["Gift boxes?! All right!", "Isn't it thrilling to open gifts over your head!!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Caroller",
                    args![
                        "Anyway, did you know...",
                        "Some villain, a fake Santa robbed some gifts from the good Santa!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Caroller",
                    args!["Furthermore, he has put bad magic on the gifts so that they become monsters!"],
                )?;
                ctx.next()?;
                ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
                ctx.lines_as("Caroller", args!["Caroller's hot news! Ha!", "Isn't it amazing?"])?;
                return ctx.close();
            }
            2 => {
                if ctx.var("iroxmas08carol").get()? == 3 {
                    ctx.lines_as("Caroller", args!["Ah, a music box is useful."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Caroller",
                        args!["Though we can't all play it around the village as we planned, it's cool that you carry it."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Caroller",
                        args!["We want to enjoy carols all together... I hope to get Crystal Pieces!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Caroller",
                        args!["If you can get ^0000FFSinging Crystal Pieces^000000, give them to me please?"],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Why not? Sure, I can give you some.")])?;
                    ctx.var("@menu").set(choice)?;
                    if ctx.items().count(6092)? < 6 {
                        ctx.lines_as("Caroller", args!["Yes, please."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Caroller",
                            args!["If you can get ^0000FFSinging Crystal Pieces^000000, give them to me please?"],
                        )?;
                        return ctx.close();
                    } else {
                        ctx.lines_as("Caroller", args!["Wow, you have them."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Caroller",
                            args!["I can bake you a cake, and I can carve your name on the cake, if you want!"],
                        )?;
                        ctx.next()?;
                        if ctx.menu(&["No, thanks.", "Please name it for me."])? == 1 {
                            ctx.lines_as("Caroller", args!["Thank for your help!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Caroller",
                                args![
                                    "Many people live in the giant world!",
                                    "So many people hope to hear Caroller, yet I always lack Crystal pieces."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Caroller",
                                args!["If you can get ^0000FFSinging Crystal Pieces^000000, give them to me please?"],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
                            ctx.items().take(6092, 6)?;
                            ctx.call(Function::GetNamedItem, args![12354, ctx.player().name()?])?;
                            ctx.lines_as(
                                "Caroller",
                                args!["Let's care about others around you on this Christmas season!"],
                            )?;
                            return ctx.close();
                        } else {
                            ctx.lines_as("Caroller", args!["Oh!", "Shyness!"])?;
                            ctx.next()?;
                            ctx.lines_as("Caroller", args!["Anyway, thanks a lot for your help."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Caroller",
                                args![
                                    "Many people live in the giant world!",
                                    "So many people hope to hear Caroller, yet I always lack Crystal pieces."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Caroller",
                                args!["If you can get ^0000FFSinging Crystal Pieces^000000, give them to me please?"],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
                            ctx.items().take(6092, 6)?;
                            ctx.items().give(12354, 1)?;
                            ctx.lines_as(
                                "Caroller",
                                args!["Let's care about others around you on this Christmas season!"],
                            )?;
                            return ctx.close();
                        }
                    }
                }
                ctx.lines_as(
                    "Caroller",
                    args![
                        "You know about Christmas!",
                        "Talking about Christmas...",
                        "...it's carols!!!",
                        "I've been waiting for this for when",
                        "Christmas comes around!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Caroller",
                    args!["But there's been no caroling here and there like before, so we can't feel the Christmas spirit."],
                )?;
                ctx.next()?;
                ctx.lines_as("Caroller", args!["So, I installed a Singing Crystal in Prontera..."])?;
                ctx.next()?;
                ctx.lines_as("Caroller", args!["but that jerk Antonio broke the crystal!"])?;
                ctx.next()?;
                ctx.lines_as("Caroller", args!["The gift boxes have been changing into monsters since Antonio placed some magic on them, so the monsters ate up the crystal fragments!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Caroller",
                    args!["I am so devastated, since the Singing Crystal has been the hope of many children around the villages."],
                )?;
                ctx.next()?;
                if ctx.menu(&["There's no other way to carol?", "You can hear carols in Lutie."])? == 1 {
                    ctx.lines_as("Caroller", args!["And just stay there, every Christmas, for your whole life?"])?;
                    ctx.next()?;
                    ctx.lines_as(ctx.player().name()?, args!["What? What do you mean?"])?;
                    ctx.next()?;
                    ctx.lines_as("Caroller", args!["Oh, nevermind.", "Ah..."])?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Caroller",
                    args!["Maybe, it's quite hard to make a jukebox for the villages, but a music box might be possible."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Caroller",
                    args!["We need the ^0000FFSinging Crystal Pieces^000000 that the monsters ate up."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Caroller",
                    args!["Please bring me ^0000FF6 Singing Crystal Pieces^000000, and you will be rewarded with presents!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Caroller", args!["Sounds cool, huh!!"])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("........................")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as("Caroller", args!["...Why are you staring at me like that?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Caroller",
                    args!["You're thinking that we are always getting our plans spoiled, aren't you?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Caroller",
                    args!["I know that we are not good at controlling stuff, but our rewards are good, right?"],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("........................")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as("Caroller", args!["Hey, we treat you good..."])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("........................")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as("Caroller", args!["You are so cruel to me!", "Bad! Bad!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Caroller",
                    args![
                        "Please, for our children's hope!!!?",
                        "Please bring me 6 Singing Crystal Pieces from ^0000FFViolent Gift Boxes^000000!"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Emotion, args![constants::ET_OK])?;
                ctx.lines_as("Caroller", args!["They're definitely as harsh as their name.", "Go on please!"])?;
                ctx.var("iroxmas08carol").set(Val::from(1))?;
                return ctx.close();
            }
            3 => {
                ctx.lines_as(
                    "Caroller",
                    args![
                        "Santa costumes!!",
                        "You know, the santa costumes that the monsters are wearing now aren't genuine!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Caroller",
                    args!["The rumor 'round here is, Lutie's designer made these costumes."],
                )?;
                ctx.next()?;
                ctx.lines_as("Caroller", args!["Every year, adventurers challenge to attack Antonio the fake Santa, but there's no way to catch up to him due to his hat and costume!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Caroller",
                    args![
                        "Are there some special abilities within them?",
                        "There's still the designer in the Christmas village... How about asking her to make that costume?"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
                ctx.lines_as("Caroller", args!["Caroller's hot news! Ha!", "It's hot, huh?"])?;
                return ctx.close();
            }
            4 => {
                ctx.lines_as(
                    "Caroller",
                    args!["A-N-T-O-N-I-O!!!", "As I heard, this Antonio is quite different from before."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Caroller",
                    args!["He seems to be quite resolved with himself since he ran away from people before."],
                )?;
                ctx.next()?;
                ctx.lines_as("Caroller", args!["And he isn't alone anymore... is what I heard..."])?;
                ctx.next()?;
                ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
                ctx.lines_as("Caroller", args!["Caroller's hot news! Ha!", "Hotness, right?"])?;
                return ctx.close();
            }
            _ => {}
        }
    }
    if ctx.var("iroxmas08carol").get()? == 1 {
        if ctx.items().count(6092)? < 6 {
            ctx.lines_as(
                "Caroller",
                args!["Maybe, it's quite hard to make a jukebox for the villages, but a music box might be possible."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Caroller",
                args!["We need the ^0000FFSinging Crystal Pieces^000000 that the monsters ate up."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Caroller",
                args!["Please bring me ^0000FF6 Singing Crystal Pieces^000000, and you will be rewarded with presents!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Caroller",
                args![
                    "For all of the children of this world!!!",
                    "Please bring me ^0000FFSinging Crystal Pieces^000000 from ^0000FFViolent Gift Boxes^000000!"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, args![constants::ET_OK])?;
            ctx.lines_as(
                "Caroller",
                args!["Be careful!", "They're definitely as harsh as their name.", "Take care!!!"],
            )?;
            return ctx.close();
        } else {
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_GLORIA])?;
            ctx.lines_as(
                "Caroller",
                args!["Don't cry, don't cry!", "Santa won't give you a gift if you're crying."],
            )?;
            ctx.next()?;
            ctx.mes("[Caroller]")?;
            let choice = runtime::select_values(ctx, &[Val::from("I got them!")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Caroller",
                args![
                    "Wow, you got them!",
                    "They are so cruel, aren't they?",
                    "I'm happy to see you again."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Caroller", args!["Let's count together!", "... ...", "Six!!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Caroller",
                args!["All right! We can start to make our music box with crystal fragments."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Caroller",
                args!["We would amplify the sounds of the crystal fragments to sound through the music box."],
            )?;
            ctx.next()?;
            ctx.lines_as("Caroller", args!["You know, a music box that sounds like a jukebox!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Caroller",
                args![
                    "It's handy. You can carry it, as well as listen to sweet carols anywhere.",
                    "Doesn't that sound cool!?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Caroller",
                args![
                    "So, we need more of some materials.",
                    "^0000FF10 Trunk, 1 Hammer Of Blacksmith, 1 Jubilee, 10 Sticky Mucus, 3carat Diamond^000000!"
                ],
            )?;
            ctx.next()?;
            ctx.items().take(6092, 6)?;
            ctx.call(Function::Emotion, args![constants::ET_OK])?;
            ctx.var("iroxmas08carol").set(Val::from(2))?;
            ctx.lines_as("Caroller", args!["Those are all needed.", "Isn't that easy?"])?;
            return ctx.close();
        }
    }
    if ctx.var("iroxmas08carol").get()? == 2 {
        if ctx.items().count(1019)? < 10
            || ctx.items().count(1005)? < 1
            || ctx.items().count(7312)? < 1
            || ctx.items().count(938)? < 10
            || ctx.items().count(732)? < 1
        {
            ctx.lines_as(
                "Caroller",
                args![
                    "We can make a music box with Singing Crystal Pieces.",
                    "It's handy, you can hear carols anywhere. Sounds cool?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Caroller",
                args![
                    "So, we need more of some materials.",
                    "^0000FF10 Trunk, 1 Hammer Of Blacksmith, 1 Jubilee, 10 Sticky Mucus, 3carat Diamond^000000!"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, args![constants::ET_OK])?;
            ctx.lines_as("Caroller", args!["That's all we need.", "Isn't that easy?"])?;
            return ctx.close();
        } else {
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_GLORIA])?;
            ctx.lines_as(
                "Caroller",
                args![
                    "Jingle bells, jingle bells,",
                    "jingle all the way!",
                    "O what fun it is to ride, in a",
                    "one... horse... o-pen... sleigh!!!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Caroller", args!["Wow!", "You came back!", "Did you bring all the materials?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Caroller",
                args!["Good!", "No need to hesitate! Let's get started to make our music box."],
            )?;
            ctx.next()?;
            ctx.mes("[Caroller]")?;
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_REPAIRWEAPON])?;
            ctx.mes("Blacksmith hammers on trunk... and we shape the frame.")?;
            ctx.next()?;
            ctx.lines_as(
                "Caroller",
                args!["Please use the Singing Crystal Pieces for a column, the Diamond as a prop, and the Sticky Mucus as glue."],
            )?;
            ctx.next()?;
            ctx.mes("[Caroller]")?;
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_HIT2])?;
            ctx.lines(args!["And now...", "we decorate with a Jubilee..."])?;
            ctx.next()?;
            ctx.mes("[Caroller]")?;
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_SUFFRAGIUM])?;
            ctx.lines(args!["The last step...!", "Breating life into it!"])?;
            ctx.next()?;
            ctx.lines_as("Caroller", args!["It's done now!!!"])?;
            ctx.next()?;
            ctx.lines_as("Caroller", args!["It's so cool! Isn't it cute!!?!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Caroller",
                args!["You did as I requested, so I will give you gifts, as promised."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Caroller",
                args!["One, is this music box.", "Please play this music box all over the villages!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Caroller",
                args![
                    "Another present is a Christmas cake especially shaped like your name!",
                    "I made this cake shaped like your name!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Caroller", args!["Don't worry, it tastes good."])?;
            ctx.next()?;
            ctx.call(Function::Emotion, args![constants::ET_OK])?;
            ctx.items().give(2784, 1)?;
            ctx.call(Function::GetNamedItem, args![12354, ctx.player().name()?])?;
            ctx.items().take(1019, 10)?;
            ctx.items().take(1005, 1)?;
            ctx.items().take(7312, 1)?;
            ctx.items().take(938, 10)?;
            ctx.items().take(732, 1)?;
            ctx.var("iroxmas08carol").set(Val::from(3))?;
            ctx.lines_as(
                "Caroller",
                args!["Thanks a lot.", "Merry Christmas!", "Have a good holiday season!"],
            )?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn santa_claus_iroxmas08(ctx: &Ctx) -> Script {
    let mut l_santacardnpc = Val::from(0);
    let mut l_santacardp = Val::from(0);
    let mut l_santacardprize = Val::from(0);
    let mut l_santacardturn = Val::from(0);
    let mut l_santacardwins = Val::from(0);
    ctx.lines_as(
        "Santa Claus",
        args![
            "Wow! Were you naughty or nice this year?",
            "All right, what comes to your mind when you think about Christmas?"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(
        ctx,
        &[Val::from("Carolling:Santa Claus:Gift Boxes:Santa Costume:Not much really...")],
    )? {
        1 => {
            ctx.lines_as(
                "Santa Claus",
                args!["Carolling! That's good!", "A sweet carol always makes Christmas more happy!!!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Santa Claus", args!["Yes indeed Carolling spreads Joy throughout the world?"])?;
            return ctx.close();
        }
        2 => {
            ctx.mes("[Santa Claus]")?;
            if ctx.var("Sex").get()? == constants::SEX_MALE {
                ctx.mes("Ho ho ho! What a good boy!")?;
            } else {
                ctx.mes("Ho ho ho! What a good girl!")?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Santa Claus",
                args![
                    "But you should be wary of a fake Santa romaing around.",
                    "Have you heard of Antonio, who invades villages every Christmas?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Santa Claus",
                args!["He has stolen my gifts, as well as attacked adventurers around Toy and Lutie field."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Santa Claus",
                args![
                    "A bad Santa ruins us good Santa's reputations!",
                    "The World Santa Organization is considering this a grave situation."
                ],
            )?;
            return ctx.close();
        }
        3 => {
            ctx.lines_as(
                "Santa Claus",
                args![
                    "A gift box! That sounds good!",
                    "It's so exciting to open gift boxes when you wake up on Christmas morning!!!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Santa Claus",
                args!["But we have very little gifts now, since Santa Antonio has stolen my gift bag."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Santa Claus",
                args!["Many adventurers try to catch up to him, but he is not easy to catch."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Santa Claus",
                args!["Anyway let those adventurers try to catch him, I have a small game for you."],
            )?;
            ctx.next()?;
            ctx.lines_as("Santa Claus", args!["I will give you a small gift if you beat me!"])?;
            ctx.next()?;
            ctx.lines_as("Santa Claus", args!["Do you want to play a game with me?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("No, thanks.:Yes, I would.")])?) == 1 {
                ctx.lines_as(
                    "Santa Claus",
                    args![
                        "Aww, don't be afraid.",
                        "If you've been a little naughty this year I won't stuff your stockings with coal."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Santa Claus",
                    args!["I will stay here throughout the Christmas season, just visit me when you change your mind."],
                )?;
                return ctx.close();
            }
            if runtime::op(
                &ctx.call(Function::GetTimeTick, args![2])?,
                "<",
                &ctx.var("santacardtime").get()?,
            )?
            .is_true()
            {
                ctx.lines_as("Santa Claus", args!["Um... You've played the game recently haven't you?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Santa Claus",
                    args![
                        "You can try the game once a hour.",
                        "Please visit me after the one hour has passed."
                    ],
                )?;
                return ctx.close();
            }
            l_santacardturn = Val::from(0);
            l_santacardwins = Val::from(0);
            ctx.lines_as("Santa Claus", args!["Wow! You're so cool!"])?;
            ctx.next()?;
            ctx.lines_as("Santa Claus", args!["Let me explain how to play this game."])?;
            ctx.next()?;
            ctx.lines_as(
                "Santa Claus",
                args![
                    "It's quite simple.",
                    "I will pick one of three cards: Poring Card, Ghostring Card, and Angeling Card.",
                    "Guess which card I pick and you're a winner!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Santa Claus",
                args!["If you guess right 3 times out of 5, I will give you a gift.", "Let's get started!"],
            )?;
            ctx.next()?;
            'l2: loop {
                if l_santacardturn == 5 {
                    break 'l2;
                }
                'b2: {
                    ctx.call(Function::Emotion, args![constants::ET_BLABLA])?;
                    ctx.lines_as("Santa Claus", args!["First let me shuffle up these cards... Ok!!!"])?;
                    ctx.next()?;
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_HIT1])?;
                    ctx.lines_as("Santa Claus", args!["One!"])?;
                    ctx.next()?;
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_HIT2])?;
                    ctx.lines_as("Santa Claus", args!["Two!"])?;
                    ctx.next()?;
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_HIT3])?;
                    ctx.lines_as("Santa Claus", args!["Three!"])?;
                    ctx.next()?;
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_STEAL])?;
                    ctx.lines_as("Santa Claus", args!["I'm picking up only one!"])?;
                    ctx.next()?;
                    ctx.fx().cutin("sorry", 4)?;
                    ctx.lines_as("Santa Claus", args!["I'm picking up only one!", "Please guess what is is."])?;
                    ctx.next()?;
                    'b3: {
                        let subject3 = Val::from(runtime::select_values(ctx, &[Val::from("Poring:Angeling:Ghostring")])?);
                        let mut matched3 = false;
                        if !matched3 && subject3.loosely_equals(&Val::from(1)) {
                            matched3 = true;
                        }
                        if matched3 {
                            ctx.lines_as(ctx.player().name()?, args!["Um...I choose Poring!"])?;
                            l_santacardp = Val::from(1);
                            ctx.next()?;
                            break 'b3;
                        }
                        if !matched3 && subject3.loosely_equals(&Val::from(2)) {
                            matched3 = true;
                        }
                        if matched3 {
                            ctx.lines_as(ctx.player().name()?, args!["Um...I choose Angeling!"])?;
                            l_santacardp = Val::from(2);
                            ctx.next()?;
                            break 'b3;
                        }
                        if !matched3 && subject3.loosely_equals(&Val::from(3)) {
                            matched3 = true;
                        }
                        if matched3 {
                            ctx.lines_as(ctx.player().name()?, args!["Um...I choose Ghostring!"])?;
                            l_santacardp = Val::from(3);
                            ctx.next()?;
                        }
                    }
                    ctx.lines_as("Santa Claus", args!["Let's see!!", "One! Two! Three!"])?;
                    l_santacardnpc = ctx.call(Function::Rand, args![1, 3])?;
                    ctx.next()?;
                    ctx.lines_as("Santa Claus", args!["Let's see!!", "One! Two! Three!"])?;
                    if l_santacardnpc == 1 {
                        ctx.fx().cutin("����ī��", 4)?;
                    } else if l_santacardnpc == 2 {
                        ctx.fx().cutin("������ī��", 4)?;
                    } else if l_santacardnpc == 3 {
                        ctx.fx().cutin("����Ʈ��ī��", 4)?;
                    }
                    ctx.next()?;
                    l_santacardturn = l_santacardturn.clone() + Val::from(1);
                    if l_santacardp.loosely_equals(&l_santacardnpc) {
                        l_santacardwins = l_santacardwins.clone() + Val::from(1);
                        ctx.call(Function::Emotion, args![constants::ET_SURPRISE])?;
                        ctx.call(
                            Function::Emotion,
                            args![constants::ET_AHA, ctx.call(Function::GetCharacterId, args![0])?.is_true(),],
                        )?;
                        ctx.fx().cutin("", 255)?;
                        ctx.lines_as(
                            "Santa Claus",
                            args!["You're lucky.", "Can you guess the right card the next time around?"],
                        )?;
                        ctx.next()?;
                    } else {
                        ctx.call(Function::Emotion, args![constants::ET_SURPRISE])?;
                        ctx.call(
                            Function::Emotion,
                            args![constants::ET_HUK, ctx.call(Function::GetCharacterId, args![0])?.is_true(),],
                        )?;
                        ctx.fx().cutin("", 255)?;
                        ctx.lines_as("Santa Claus", args!["Aww maybe next time..."])?;
                        ctx.next()?;
                    }
                }
            }
            if l_santacardwins.clone().number()? < 3 {
                ctx.lines_as(ctx.player().name()?, args!["This is just luck.", "Let me try again!!"])?;
                ctx.next()?;
                ctx.lines_as("Santa Claus", args!["Whenever you want."])?;
                return ctx.close();
            } else {
                ctx.lines_as("Santa Claus", args!["You're so good!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Santa Claus",
                    args!["Now this gift is for you.", "Put your hand into the bag and pick only one."],
                )?;
                l_santacardprize = ctx.call(Function::Rand, args![1, 12])?;
                ctx.next()?;
                if l_santacardprize == 1 {
                    ctx.items().give(12354, 2)?;
                } else if l_santacardprize == 2 {
                    ctx.items().give(595, 3)?;
                } else if l_santacardprize == 3 {
                    ctx.items().give(593, 3)?;
                } else if l_santacardprize == 4 {
                    ctx.items().give(12236, 3)?;
                } else if l_santacardprize == 5 {
                    ctx.items().give(538, 10)?;
                } else if l_santacardprize == 6 {
                    ctx.items().give(14546, 10)?;
                } else if l_santacardprize == 7 {
                    ctx.items().give(5136, 1)?;
                } else if l_santacardprize == 8 {
                    ctx.items().give(603, 1)?;
                } else if l_santacardprize == 9 {
                    ctx.items().give(12130, 1)?;
                } else if l_santacardprize == 10 {
                    ctx.items().give(14550, 10)?;
                } else if l_santacardprize == 11 {
                    ctx.items().give(12132, 3)?;
                } else if l_santacardprize == 12 {
                    ctx.items().give(594, 3)?;
                }
                ctx.var("santacardtime")
                    .set(ctx.call(Function::GetTimeTick, args![2])? + Val::from(3600))?;
                ctx.lines_as(
                    "Santa Claus",
                    args!["Good job! Thanks for playing the card game with me!", "Merry Christmas!"],
                )?;
                return ctx.close();
            }
        }
        4 => {
            ctx.lines_as(
                "Santa Claus",
                args![
                    "Do you mean that ever so fashionable costume for Santa's and youngsters!?!",
                    "Light-weight, fashionable, and keeps you warm!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Santa Claus",
                args![
                    "There used to be designer who visited my house to make my Santa costumes.",
                    "Do you have any ideas?"
                ],
            )?;
            return ctx.close();
        }
        5 => {
            ctx.lines_as("Santa Claus", args!["........................"])?;
            ctx.next()?;
            ctx.lines_as("Santa Claus", args!["........................"])?;
            if ctx.var("Sex").get()? == constants::SEX_MALE {
                ctx.mes("Oh! Poor boy...")?;
            } else {
                ctx.mes("Oh! Poor girl...")?;
            }
            ctx.next()?;
            ctx.lines_as("Santa Claus", args!["........................"])?;
            if ctx.var("Sex").get()? == constants::SEX_MALE {
                ctx.mes("Oh! Poor boy...")?;
            } else {
                ctx.mes("Oh! Poor girl...")?;
            }
            ctx.mes("...")?;
            ctx.next()?;
            ctx.lines_as(
                "Santa Claus",
                args![
                    "You should open your heart to the spirit of Christmas!",
                    "Once you do, I know you'll be able to think of something."
                ],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn louise_kim_iroxmas08(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Louise Kim",
        args![
            "I always thought about how boring Santa Claus is wearing a too boring costume.",
            "Too boring!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Louise Kim", args!["I could make a glamorous style for him!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Louise Kim",
        args![
            "I once made a costume for the notorious figure Antonio!",
            "The reason catching up to him is so hard, is that his clothes have been given strong power!",
            "I blessed them with good luck!"
        ],
    )?;
    ctx.next()?;
    ctx.call(Function::Emotion, args![constants::ET_THROB])?;
    ctx.lines_as(
        "Louise Kim",
        args!["Preta Porter!!", "Which is quite luxurious but sold at good price ~"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Louise Kim",
        args![
            "Hey loosers!",
            "You can share my sense of fashion and wear my look if you run a light mission for me."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Louise Kim", args!["Are you interested?"])?;
    ctx.next()?;
    if ctx.menu(&["Nope.", "Yes, please."])? == 0 {
        ctx.lines_as("Louise Kim", args!["Ah!", "You're silly! You lost your big chance!"])?;
        ctx.next()?;
        ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
        ctx.lines_as(
            "Louise Kim",
            args![
                "Come to me later if you want to get the mission.",
                "I, Louise Kim, am generous enough to accept you next time."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Louise Kim", args!["You're so cool."])?;
    ctx.next()?;
    ctx.lines_as(
        "Louise Kim",
        args![
            "Don't worry about this mission.",
            "It's not that difficult.",
            "I'm only in need of some materials. Things that are beyond my ability..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Louise Kim", args!["Anyway, can you bring me some stuff?"])?;
    ctx.next()?;
    if ctx.menu(&["Why not? What do you need?", "Sorry, no time."])? == 1 {
        ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
        ctx.lines_as(
            "Louise Kim",
            args!["What?", "You will definitely regret it.", "No more chances later."],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Louise Kim",
        args![
            "All right, you're cool!!",
            "You need to bring me: ^0000FFCotton Shirt, 3 Red Potion, Holy Water, 4 Wrapping Paper, Wrapping Lace^000000."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Louise Kim",
        args!["If you bring me all that stuff, I can make you a glamorous Santa costume."],
    )?;
    ctx.next()?;
    if ctx.items().count(2301)? < 1
        || ctx.items().count(501)? < 3
        || ctx.items().count(523)? < 1
        || ctx.items().count(7175)? < 4
        || ctx.items().count(7174)? < 1
    {
        ctx.lines_as(
            "Louise Kim",
            args!["If you were to bring me all the stuff, I would make you a wonderful costume, for free..."],
        )?;
        ctx.next()?;
        ctx.call(Function::Emotion, args![constants::ET_THROB])?;
        ctx.lines_as("Louise Kim", args!["Please see me again if you are interested."])?;
        return ctx.close();
    } else {
        let choice = runtime::select_values(ctx, &[Val::from("Here you are.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.call(Function::Emotion, args![constants::ET_OK])?;
        ctx.lines_as(
            "Louise Kim",
            args!["Oh! Good!", "Let's not delay.", "I will show you my limitless ability."],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "-She pours everything in a pot-",
            "-even the shirt goes in!-",
            "-She takes it out with skill-",
            "-and many blessings she sings.-"
        ])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, args![constants::EF_BLESSING])?;
        ctx.call(Function::Emotion, args![constants::ET_DELIGHT])?;
        ctx.lines_as(
            "Louise Kim",
            args!["By artist, Louise Kim!", "All over the world will be blessed tonight!"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "-Pour red potion in another pot-",
            "-then stir all of its parts.-",
            "-Put the cotton shirts in-",
            "-and dye it for grateful hearts.-",
            "-Thread by thread-",
            "-String by string-",
            "-Count your blessings and sing!-"
        ])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, args![constants::EF_GLORIA])?;
        ctx.call(Function::Emotion, args![constants::ET_DELIGHT])?;
        ctx.lines_as(
            "Louise Kim",
            args!["By artist, Louise Kim!", "All over the world will be blessed tonight!"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "-She is knitting clothes-",
            "-with a hooked needle-",
            "-with her great skill.-",
            "-Spread your blessings,-",
            "-cheer and goodwill!-"
        ])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, args![constants::EF_BENEDICTIO])?;
        ctx.call(Function::Emotion, args![constants::ET_THROB])?;
        ctx.lines_as(
            "Louise Kim",
            args!["This is miraculous!", "I am a genius of the world.", "Artist, Louise Kim!!!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Louise Kim",
            args![
                "All right! Isn't it wonderful?",
                "You can call it what you wish.",
                "My fashion is radiant.",
                "I ain't envious of Designer Pierre."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
        ctx.lines_as(
            "Louise Kim",
            args!["I am supposed to get paid well, however I will just let it be free, since this is Christmas!"],
        )?;
        ctx.next()?;
        ctx.items().take(2301, 1)?;
        ctx.items().take(501, 3)?;
        ctx.items().take(523, 1)?;
        ctx.items().take(7175, 4)?;
        ctx.items().take(7174, 1)?;
        ctx.items().give(12132, 1)?;
        ctx.lines_as(
            "Louise Kim",
            args!["Go brag about these wonderful clothes. There wouldn't be any without me, Louise Kim."],
        )?;
        return ctx.close();
    }
}
