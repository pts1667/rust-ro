#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

pub fn mr_claus(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Mr. Claus",
        args!["Ho Ho Ho~", "Merry Christmas!!", "I wish all of you joy", "and Christmas cheer!"],
    )?;
    ctx.next()?;
    let subject1 = ctx.menu(&["Info about Lutie", "Move to 'Lutie'", "Cancel"])?;
    if subject1 == 0 {
        ctx.lines_as("Mr. Claus", args!["^3355FFLutie^000000, the fantastic Christmas Town! Always filled with the spirit of giving, Lutie is filled with appetizing cakes, tiny toy soldiers, and all sorts of wonderful things~!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Mr. Claus",
            args![
                "Ho Ho Ho~",
                "It's an amazing land blessed with the beauty of winter, and a year round atmosphere of festivity!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mr. Claus",
            args![
                "I'm a Santa that will guide people to Lutie, the Christmas Town. Ask me at any time, and I'll magically send you there~"
            ],
        )?;
        return ctx.close();
    } else if subject1 == 1 {
        ctx.lines_as(
            "Mr. Claus",
            args!["Ho Ho Ho~", "The only way to get to Lutie is here in Al de Baran!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Mr. Claus", args!["I keep this place and personally transport people who want to visit Lutie. Please ask Santa Claus over there if you want to leave town. He will let you know the way out of Lutie."])?;
        ctx.next()?;
        ctx.lines_as(
            "Mr. Claus",
            args!["Ho Ho Ho~", "Well, are you ready?", "Have a nice trip!", "Meeeeerry Christmas!"],
        )?;
        ctx.close_window()?;
        ctx.warp("xmas_fild01", 78, 68)?;
        return ctx.end();
    } else if subject1 == 2 {
        ctx.lines_as(
            "Mr. Claus",
            args![
                "Ho Ho Ho~",
                "Whenever you want to visit Lutie, be my guest. Just let me know when you want to leave."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Mr. Claus", args!["Ho ho hooooo!!", "Haaaaappy Holidays!"])?;
        return ctx.close();
    }
    Ok(())
}

pub fn santa_claus(ctx: &Ctx) -> Script {
    ctx.lines_as("Santa Claus", args!["Ho Ho Ho~", "Meeeerry Christmas !!"])?;
    ctx.next()?;
    ctx.lines(args!["^3355FFIt's...^000000", "^3355FFIt's the original Santa Claus!^000000"])?;
    ctx.next()?;
    ctx.lines_as(
        "Santa Claus",
        args![
            "Ho Ho Ho~",
            "I'm Santa Claus, and I bring gifts to every good boy and girl on Christmas!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Santa Claus", args!["If you want to leave Lutie, go outside town and head south to the first field that you see. You'll be able to find a magical warp that will take you to Al de Baran."])?;
    ctx.next()?;
    ctx.lines_as("Santa Claus", args!["Ho ho ho~", "Meeeeeeerry Christmas!"])?;
    ctx.close()
}

pub fn duffle(ctx: &Ctx) -> Script {
    if ctx.var("xmas_npc").get()? == 1 {
        ctx.lines_as("Duffle", args!["Merry Christmas!", "Welcome to Lutie!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Duffle",
            args!["You got a present", "from Santa Claus?!", "Ha ha, you must", "be really excited!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Duffle",
            args!["Hey, have you heard that here in Lutie, we have an attraction that's equally as famous as Santa himself?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Duffle",
            args!["It's ^3355FFSnowysnow^000000,", "the magical", "talking snowman!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Duffle", args!["Before you leave, you really should meet and talk to Snowysnow, even if it's only once. He's really a nice guy and fun to talk to."])?;
        ctx.next()?;
        ctx.lines_as("Duffle", args!["Well then...", "Merry Christmas!!"])?;
        ctx.var("xmas_npc").set(Val::from(2))?;
        return ctx.close();
    } else if ctx.var("xmas_npc").get()?.number()? > 1 {
        ctx.lines_as(
            "Duffle",
            args!["Have you ever talked to the snowman in front of this town? The lonely snowman who stands in solitude..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Duffle",
            args![
                "But he's so warm hearted~! Sometimes, I talk to Snowysnow the snowman. For some weird reason, he can talk just like us!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Duffle",
            args![
                "When I talk to Snowysnow, I get to wondering how he came to be. I guess if you talk to him too, you'll feel the same way."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Duffle",
            args!["How he was created, and how he thinks and talks like a human is such a mystery..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Duffle",
            args!["Where did he come from and what kind of place was it? And how did he come to Lutie without any legs...?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Duffle",
            args!["Lately, it seems more and more people are coming to this town to see Snowysnow."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Duffle",
            args![
                "I guess you should talk to the other people living in Lutie if you want to learn more about the mystery of Snowysnow..."
            ],
        )?;
        return ctx.close();
    } else {
        ctx.lines_as(
            "Duffle",
            args![
                "Oh...!",
                "While you're here, don't forget to visit the original Santa Claus here in Lutie."
            ],
        )?;
        return ctx.close();
    }
}

pub fn lenient_aunt(ctx: &Ctx) -> Script {
    let subject1 = ctx.var("xmas_npc").get()?;
    if subject1 == 5 {
        ctx.lines_as(
            "Thachentze",
            args!["Hmm? The Hairy guy", "spoke well of me,", "did he? Well well..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Thachentze", args!["I know what he thinks... Ho ho ho~! He intends to make me feel happy so that I'll give him some free jars of pickles! Oh well~!"])?;
        ctx.next()?;
        ctx.lines_as("Thachentze", args!["He knows me too well. I almost can't stop from giving that man some pickles. Yes, even I know my pickles are the best in town!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Thachentze",
            args![
                "Hmmm~?",
                "You want to know",
                "about ^3355FFSnowysnow^000000?",
                "Oh. Yes, yes, I see..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thachentze",
            args!["Well, I can't just let anyone know something so important about Snowysnow. Hmmm..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Thachentze", args!["Snowysnow has been holding something for me as a favor, the ^3355FFroughest salt in the world^000000 which I use to pickle cabbages."])?;
        ctx.next()?;
        ctx.lines_as("Thachentze", args!["I suppose if you're really Snowysnow's friend, he will trust you enough to give it to you so you can deliver it to me. And in any case, I'll need more of it soon."])?;
        ctx.next()?;
        ctx.lines_as("Thachentze", args!["Now be a dear", "and hurry up.", "Come back quickly~"])?;
        ctx.var("xmas_npc").set(Val::from(6))?;
        return ctx.close();
    } else if subject1 == 6 {
        ctx.lines_as("Thachentze", args!["Hohohoho~", "You're back!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Thachentze",
            args!["Did you bring it?", "Oh goodness...!", "My cabbages will", "get sour soon!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Thachentze", args!["You...", "Don't have it?"])?;
        ctx.next()?;
        ctx.lines_as("Thachentze", args!["*Sigh*", "You are really a scatter-brained person, my dear. Now hurry over to Snowysnow and bring me the ^3355FFroughest salt in the world^000000."])?;
        ctx.next()?;
        ctx.lines_as("Thachentze", args!["Hurry now, dear,", "Chop Chop~!"])?;
        return ctx.close();
    } else if subject1 == 7 {
        ctx.lines_as(
            "Thachentze",
            args!["Hohohohoh hohohohoho !", "Let's see, let's see...", "Thank you dear,Thank you."],
        )?;
        ctx.next()?;
        ctx.var("xmas_npc").set(Val::from(8))?;
        ctx.lines(args!["^3355FFYou gave her the", "roughest salt in the world.^000000"])?;
        ctx.next()?;
        ctx.lines_as(
            "Thachentze",
            args!["Now I am able to pickle my cabbages properly. Thank you, my dear. Thank you..."],
        )?;
        ctx.next()?;
        ctx.mes("...")?;
        ctx.next()?;
        ctx.lines(args!["...", "......"])?;
        ctx.next()?;
        ctx.lines_as(
            "Thachentze",
            args!["Oh yes, I'm sorry. I almost forgot what I promised you. You asked me about Snowysnow's magical gift bag, didn't you?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Thachentze", args!["If you already met Uncle Cantata, you must know by now that Snowysnow has been made out of some mysterious snow that covered a thick field of magical flowers."])?;
        ctx.next()?;
        ctx.lines_as("Thachentze", args!["I can't tell you how, but when Snowysnow was revived, there was a reaction between the Alchemist's materials and the energies of Snowysnow's snow."])?;
        ctx.next()?;
        ctx.lines_as(
            "Thachentze",
            args!["For some reason, Snowysnow's gift bag can now create as many presents as Snowysnow wants, just like Santa Claus."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thachentze",
            args!["Of course, if this power were to fall into the hands of evil, we would all be in trouble."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thachentze",
            args![
                "However, everyone knows that Snowysnow is kind and loving towards others. So we're never worried about Snowysnow's powers."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Thachentze", args!["Oh, and I've just heard some shocking news from ^3355FFHashokii^000000 the clown. It's quite an interesting story, actually. Why don't you ask him more about it?"])?;
        return ctx.close();
    } else {
        ctx.lines_as("Thachentze", args!["Merry Christmas~", "Ho! Ho! Ho!"])?;
        ctx.next()?;
        ctx.lines_as("Thachentze", args!["I can feel the Christmas spirit all around me! It's even in the eyes of the young travelers who've come out here all the way to Lutie. Hoho, I wish you a Merry Christmas!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Thachentze",
            args![
                "We have a dungeon named",
                "'Christmas dungeon' around here. Well, I guess I don't need to tell you much if you've heard about it already.",
                "Oh well..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Thachentze", args!["I figured something out a few days ago. In the Christmas dungeon, you'll run into creatures that are similar to those outside of town."])?;
        ctx.next()?;
        ctx.lines_as("Thachentze", args!["I'm guessing that monsters wandered here from outside of town, and were changed by the cold weather here. So monsters adapted to live in this environment."])?;
        ctx.next()?;
        ctx.lines_as("Thachentze", args!["...", "Okay, now I think I better be ready to pickle some cabbages. If you didn't know already, I make the best pickles around! Why don't you visit me later and try some?"])?;
        return ctx.close();
    }
}

pub fn poze(ctx: &Ctx) -> Script {
    if ctx.var("xmas_npc").get()? == 3 || ctx.var("xmas_npc").get()? == 4 {
        ctx.lines_as(
            "Poze",
            args!["You've gone to", "^3355FFSnowysnow^000000 and he", "mentioned me?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Poze", args!["Oh I see...", "He's a snowman that doesn't have any legs. No wonder he hasn't come to visit me. What a shame, what a shame. I guess I better go visit him instead."])?;
        ctx.next()?;
        ctx.lines_as("Poze", args!["Oh, there is someone who knows how Snowysnow came to be able to speak. That person would be ^3355FFUncle Hairy Cantata^000000..."])?;
        ctx.next()?;
        ctx.lines_as("Poze", args!["One day when apprentice of the great alchemist visited Lutie, I came to listen in on a conversation between him and Uncle Hairy."])?;
        ctx.next()?;
        ctx.lines_as("Poze", args!["Long ago, a great alchemist came by Snowysnow's hometown and happened to meet Snowysnow dying, melting down into water. However, Snowysnow was miraculously revived by that Alchemist."])?;
        ctx.next()?;
        ctx.lines_as(
            "Poze",
            args!["But that's pretty much all I know. For the actual details, you should ask ^3355FFUncle Hairy Cantata^000000."],
        )?;
        ctx.var("xmas_npc").set(Val::from(4))?;
        return ctx.close();
    } else {
        ctx.lines_as(
            "Poze",
            args![
                "Welcome to Lutie,",
                "the town which blesses",
                "all of its visitors with",
                "the spirit of Christmas!",
                "Merry Christmas !"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Poze",
            args![
                "Here in this magical land of fun and fancy, you can enjoy the spirit of Christmas all year round~! Isn't that wonderful?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Poze", args!["Lutie isn't merely just a simple attraction. We have convenient facilities like the other towns, but in a festive environment."])?;
        ctx.next()?;
        ctx.lines_as(
            "Poze",
            args!["So if you decide to stay here for a while, you should have all the comforts that you need. Merry Christmas~"],
        )?;
        return ctx.close();
    }
}

pub fn uncle_hairy(ctx: &Ctx) -> Script {
    if ctx.var("xmas_npc").get()? == 4 {
        if ctx.items().count(1024)? > 0 && ctx.items().count(938)? > 0 {
            ctx.lines_as("Cantata", args!["Oh? Y-y-you've got the stuff? Goooooooood. It's been so long since I've been able to have some of this... G-give it to me!"])?;
            ctx.next()?;
            ctx.items().take(1024, 1)?;
            ctx.items().take(938, 1)?;
            ctx.mes("^3355FFYou quickly handed him the Squid Ink and Sticky Mucus and watched with a little disgust as he relished the flavor.^000000")?;
            ctx.next()?;
            ctx.lines_as("Cantata", args!["*Burrrrpppp~*", "Well, now it's the time for my story. Keep in mind that this is the whole story from what I know. I'm not sure how much you've already heard though..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Cantata",
                args![
                    "A long time ago,",
                    "there was a great",
                    "Alchemist living",
                    "in the far north.",
                    "His name was",
                    "^3355FFPhilip Varsez^000000!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Cantata", args!["He was always foremost in the research of alchemy and needed rare materials to conduct his studies. Because of that, he had to travel the world in search of materials containing magical energies..."])?;
            ctx.next()?;
            ctx.lines_as("Cantata", args!["One day, his travels brought him to a northern village known for its freezing weather. But when he arrived, he was welcomed by a smouldering town that had recently been destroyed."])?;
            ctx.next()?;
            ctx.lines_as("Cantata", args!["It was a grim sight: People were lying at the roadside, groaning in agony. As Varsez walked by, each villager would beg, 'K-Kill me...' and plead for him to put them out of their misery."])?;
            ctx.next()?;
            ctx.lines_as("Cantata", args!["Amidst the woeful cries of despair, the wails of two infants reached the ears of Philip Varsez. He rushed to investigate and found two babies cushioned in the bosom of a melting snowman."])?;
            ctx.next()?;
            ctx.lines_as("Cantata", args!["That snowman...", "was ^3355FFSnowysnow^000000."])?;
            ctx.next()?;
            ctx.lines_as("Cantata", args!["Being the wise Alchemist that he is, Varsez deduced that Snowysnow sacrificed himself to protect those two babies from the great disaster that had destroyed the village."])?;
            ctx.next()?;
            ctx.lines_as("Cantata", args!["Varsez was touched, and was determined to save the life of this snowman with his alchemy. He would then transport him here to Lutie, the safest place in the world."])?;
            ctx.next()?;
            ctx.lines_as("Cantata", args!["Of course, there was another rumor that, in addition to the mercy from that Alchemist, Snowysnow was able to survive due to the special properties of his snow."])?;
            ctx.next()?;
            ctx.lines_as("Cantata", args!["It's believed that Snowysnow's snow used to cover a mysterious field that would be filled with the bloom of magical flowers."])?;
            ctx.next()?;
            ctx.lines_as(
                "Cantata",
                args![
                    "Muhahaha~",
                    "Well, that's pretty much all I know. I hope you were able to learn what you wanted from my story! Haw haw haw!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Cantata", args!["Well...", "Now that I think about it..."])?;
            ctx.next()?;
            ctx.lines_as("Cantata", args!["Snowysnow can not only speak, but he also seems to be able to create an endless supply of Christmas presents. Or at least, that's what I hear."])?;
            ctx.next()?;
            ctx.lines_as("Cantata", args!["^3355FFThachentze^000000, that lovely pickle maker, knows more about it. So if you're curious, you should go talk to her. Alrighty then, Merry Christmas!"])?;
            ctx.var("xmas_npc").set(Val::from(5))?;
            return ctx.close();
        } else {
            ctx.lines_as("Cantata", args!["Oh yeah? Heard about me from Poze, did you? Haw haw haw! Yeah, I know a little bit about Snowysnow. In fact, I may even be his weak point, since..."])?;
            ctx.next()?;
            ctx.lines_as("Cantata", args!["I know", "the secret of", "^3355FFSnowysnow's birth^000000!!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Cantata",
                args![
                    "Are you curious?",
                    "Heh heh heh~ Well, don't think I'll let you know unless you give me something in return!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cantata",
                args![
                    "Living in this",
                    "town doesn't give me",
                    "much of a chance to enjoy",
                    "a man's drink. Hmmm, bring me..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cantata",
                args!["^3355FF1 Squid Ink^000000 and", "^3355FF1 Sticky Muscus^000000!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cantata",
                args!["Wahhahaha! What the hell is that look for?! Never you mind my gourmet sense of taste! Now get to work~!"],
            )?;
            return ctx.close();
        }
    } else {
        ctx.lines_as("Cantata", args!["Merry Christmas!", "Welcome to Lutie!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Cantata",
            args!["It looks like the cold has brought a rosiness to your cheeks.", "Haw haw haw!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cantata",
            args!["Be careful, it wouldn't be good for you to catch the Lutie Flu.", "..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cantata",
            args![
                "*Sigh* That reminds me...",
                "My little boy had a terrible case of the Lutie Flu a while ago."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cantata",
            args![
                "It was during the night, and there was no place I could get any medicine. It seemed I could do nothing for my little boy."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Cantata", args!["I thought the least I could do for my son was get something cold to bring down his fever, but the snow of Lutie kept on melting after being placed on his forehead. He was burning up, and I was failing to relieve him."])?;
        ctx.next()?;
        ctx.lines_as("Cantata", args!["It was then that", "I knew I needed", "some magic ice."])?;
        ctx.next()?;
        ctx.lines_as("Cantata", args!["Eventually, I found myself in the Christmas dungeon without any goal in mind. Inside I found, thank God, a certain creature made entirely of ice!"])?;
        ctx.next()?;
        ctx.lines_as("Cantata", args!["It was an ^3355FFIceporing^000000! The local people used to call it 'Icepantzering.' Anyway, I was able to save my boy's life with it. Thank goodness something like that was around in this town."])?;
        ctx.next()?;
        ctx.lines_as("Cantata", args!["Oops, I think I've talked a bit much. Merry Christmas~!"])?;
        return ctx.close();
    }
}

pub fn snowman(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
        ctx.fx().cutin("rutie_snownow01.bmp", 2)?;
        ctx.lines(args![
            "- Wait a minute !! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again -",
            "- after you lose some weight. -"
        ])?;
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return ctx.end();
    }
    ctx.fx().cutin("rutie_snownow03.bmp", 2)?;
    ctx.lines_as("Snowysnow", args!["I'm...", "I'm so lonely!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Snowysnow",
        args![
            "Always stuck here...",
            "On the same spot...",
            "Day after day after day after day after day after day after day after day after day after day..."
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "^3355FFSnowysnow?^000000:Info about the Christmas dungeon:Quit this conversation",
            )],
        )?);
        let mut matched1 = false;
        let no_case1 =
            !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2)) && !subject1.loosely_equals(&Val::from(3));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            if ctx.var("xmas_npc").get()?.number()? < 2 {
                ctx.fx().cutin("rutie_snownow01.bmp", 2)?;
                ctx.lines_as("Snowysnow", args!["I was born in an area to the north where it snowed all the time, and was much colder than Lutie, if you can believe that."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Snowysnow",
                    args![
                        "I was made with love by a human, and I was really happy there. Life was simple, but it was full of quiet bliss."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Snowysnow",
                    args!["I thought I could live the rest of my life in that kind of contentment. But... It was not to be."],
                )?;
                ctx.next()?;
                ctx.fx().cutin("rutie_snownow01.bmp", 2)?;
                ctx.lines_as("Snowysnow", args!["One fateful day, some ugly old woman came to our town. People say her name was 'Merlophechum,' and that she was from some strange cave town where the weather was always hot."])?;
                ctx.next()?;
                ctx.lines_as("Snowysnow", args!["On the third night she was there, she set the town on fire with frightening magic. Everyone was running in panic amongst the fear and chaos. And somehow, I was knocked out."])?;
                ctx.next()?;
                ctx.fx().cutin("rutie_snownow02.bmp", 2)?;
                ctx.lines_as("Snowysnow", args!["I didn't notice how much time passed after that, but when I woke up, I was here. Well, I can say that this place, Lutie, is pretty much like heaven."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Snowysnow",
                    args!["Everyone here is never worried, and I'm always hearing Christmas carols and stuff."],
                )?;
                ctx.next()?;
                ctx.fx().cutin("rutie_snownow03.bmp", 2)?;
                ctx.lines_as(
                    "Snowysnow",
                    args![
                        "But still...",
                        "Sometimes all that Christmas joy somehow doesn't cure the dark loneliness that wells deep inside of me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Snowysnow",
                    args!["So will you be my friend? If you do, I'll be your friend too~"],
                )?;
                ctx.close_window()?;
                ctx.fx().cutin("", 255)?;
                return ctx.end();
            } else {
                let subject2 = ctx.var("xmas_npc").get()?;
                if subject2 == 2 {
                    ctx.fx().cutin("rutie_snownow01.bmp", 2)?;
                    ctx.lines_as(
                        "Snowysnow",
                        args![
                            "Oh...?",
                            "So you've met Duffle?",
                            "Yeah, sometimes she",
                            "stops by and says 'hi...'"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Snowysnow", args!["It's weird that the people of Lutie call me a mysterious, magical snowman. I mean, inside, aren't I just the same as regular people?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snowysnow",
                        args![
                            "*Sniff* S-sometimes,",
                            "I don't even know what I am. But even so, the people of Lutie try to accept me no matter what."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Snowysnow", args!["^3355FFPoze^000000 gave me his glamour photo with his address on it, and told me to visit him whenever I'm feeling blue. I was so happy to hear that..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snowysnow",
                        args!["But then fate played its cruel joke on me once again, and I realized that I had no legs to visit him with."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Snowysnow", args!["How did I ever get to this town? And how in the world am I able to talk?! I-It's not natural, is it? Does... Does that make me a monster?"])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFSnowysnow is immersed in his deep thoughts, and seems^000000",
                        "^3355FFfixated on Poze's memento.^000000"
                    ])?;
                    ctx.var("xmas_npc").set(Val::from(3))?;
                    ctx.close_window()?;
                    ctx.fx().cutin("", 255)?;
                    return ctx.end();
                } else if subject2 == 3 {
                    ctx.fx().cutin("rutie_snownow01.bmp", 2)?;
                    ctx.lines_as("Snowysnow", args!["..."])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFSnowysnow is immersed in his deep thoughts, and seems^000000",
                        "^3355FFfixated on Poze's memento.^000000"
                    ])?;
                    ctx.close_window()?;
                    ctx.fx().cutin("", 255)?;
                    return ctx.end();
                } else if subject2 == 4 {
                    ctx.fx().cutin("rutie_snownow01.bmp", 2)?;
                    ctx.lines_as("Snowysnow", args!["Oh goody!", "You've met Poze!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snowysnow",
                        args!["He's such an honest, good hearted guy. I hope he and Duffle get together someday."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Snowysnow", args!["...Oh no~!", "Did I say that out loud? That was supposed to stay in my head! I'm soooo sorry! Boy, I can be a real dum-dum head, huh?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snowysnow",
                        args![
                            "Yeah...",
                            "Poze is in love with Duffle. And she's kind to everybody, except for Poze."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snowysnow",
                        args!["But I know that's because she likes him a whole whole lot! Hee hee hee!"],
                    )?;
                    ctx.close_window()?;
                    ctx.fx().cutin("", 255)?;
                    return ctx.end();
                } else if subject2 == 5 {
                    ctx.fx().cutin("rutie_snownow01.bmp", 2)?;
                    ctx.lines_as("Snowysnow", args!["Oh...?", "You've met", "^3355FFUncle Hairy Cantata^000000?"])?;
                    ctx.next()?;
                    ctx.fx().cutin("rutie_snownow02.bmp", 2)?;
                    ctx.lines_as("Snowysnow", args!["Sure, he has a loud voice, doesn't take showers and smells like rotting food. But he's a funny guy with a warm heart. Everybody loves him!"])?;
                    ctx.next()?;
                    ctx.lines_as("Snowysnow", args!["Of course, he still enjoys drinking strange things. They say it's a miracle that he doesn't have a tummyache. Hee hee~! Oh, I love that guy!"])?;
                    ctx.close_window()?;
                    ctx.fx().cutin("", 255)?;
                    return ctx.end();
                } else if subject2 == 6 {
                    ctx.fx().cutin("rutie_snownow01.bmp", 2)?;
                    ctx.lines_as("Snowysnow", args!["Oh...?", "You've met", "^3355FFAunt Thachentze^000000?"])?;
                    ctx.next()?;
                    ctx.lines_as("Snowysnow", args!["Yeah, she's a pickle expert, alright. Oh right, would you give this to her? I've been keeping the roughest salt in the world for her as a bit of a favor."])?;
                    ctx.next()?;
                    ctx.var("xmas_npc").set(Val::from(7))?;
                    ctx.mes("^3355FFSnowysnow gave you the roughest salt in the world^000000.")?;
                    ctx.next()?;
                    ctx.lines_as("Snowysnow", args!["I like her cooking because it's soooo delicious! Sometimes, she gives me grape syrup on snow flakes. Anyway, please deliver that salt for me, buddy~!"])?;
                    ctx.close_window()?;
                    ctx.fx().cutin("", 255)?;
                    return ctx.end();
                } else if subject2 == 7 {
                    ctx.fx().cutin("rutie_snownow01.bmp", 2)?;
                    ctx.lines_as(
                        "Snowysnow",
                        args![
                            "'^3355FFThe roughest",
                            "salt in the world^000000...'",
                            "Wow. Now, that's rough! Aunt Tachentze is always making pickles, so she sure could use it soon!"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.fx().cutin("", 255)?;
                    return ctx.end();
                } else if subject2 == 8 {
                    ctx.fx().cutin("rutie_snownow01.bmp", 2)?;
                    ctx.lines_as("Snowysnow", args!["Hashokii, the boring clown? At first, he seems kind of dumb and not really that funny. But deep inside, he cares a lot about other people."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snowysnow",
                        args!["He's always trying his best to make those two orphans laugh and forget their troubles..."],
                    )?;
                    ctx.close_window()?;
                    ctx.fx().cutin("", 255)?;
                    return ctx.end();
                } else if subject2 == 9 {
                    ctx.fx().cutin("rutie_snownow01.bmp", 2)?;
                    ctx.lines_as("Snowysnow", args!["Ah...", "So you've met Charu Charu? That boy is so full of optimism and always looking forward. When he grows up, he's going to be a big shot!"])?;
                    ctx.next()?;
                    ctx.lines_as("Snowysnow", args!["I'm sure of it!", "Hee hee hee~!"])?;
                    ctx.close_window()?;
                    ctx.fx().cutin("", 255)?;
                    return ctx.end();
                } else if subject2 == 10 {
                    ctx.fx().cutin("rutie_snownow01.bmp", 2)?;
                    ctx.lines_as("Snowysnow", args!["Thank you for listening to me so far. I really appreciate that you try to understand me, even though you're a stranger here."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Snowysnow",
                        args!["Now, you know me better than anyone else in this town. So, in return, I want to give you a small present."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Snowysnow", args!["Tah dah!", "Pick anything", "you want in here~"])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou gingerly stir",
                        "your hand around in",
                        "Snowysnow's magical gift bag^000000."
                    ])?;
                    ctx.next()?;
                    let subject3 = ctx.rand_range(1, 8)?;
                    if subject3 == 1 {
                        ctx.var("xmas_npc").set(Val::from(11))?;
                        ctx.items().give(529, 5)?;
                        ctx.fx().cutin("rutie_snownow02.bmp", 2)?;
                        ctx.lines_as("Snowysnow", args!["Wow~!", "^3355FF5 Candy^000000!", "Congratulations!"])?;
                    } else if subject3 == 2 {
                        ctx.var("xmas_npc").set(Val::from(11))?;
                        ctx.items().give(529, 10)?;
                        ctx.fx().cutin("rutie_snownow02.bmp", 2)?;
                        ctx.lines_as("Snowysnow", args!["Ooh~!", "^3355FF10 Candy^000000!"])?;
                    } else if subject3 == 3 {
                        ctx.var("xmas_npc").set(Val::from(11))?;
                        ctx.items().give(530, 5)?;
                        ctx.fx().cutin("rutie_snownow02.bmp", 2)?;
                        ctx.lines_as("Snowysnow", args!["Hoooraaaay~!", "^3355FF5 Candy Cane^000000!"])?;
                    } else if subject3 == 4 {
                        ctx.var("xmas_npc").set(Val::from(11))?;
                        ctx.items().give(530, 10)?;
                        ctx.fx().cutin("rutie_snownow02.bmp", 2)?;
                        ctx.lines_as("Snowysnow", args!["Wow, that's so great!", "^3355FF10 Candy Cane^000000!"])?;
                    } else if subject3 == 5 {
                        ctx.var("xmas_npc").set(Val::from(11))?;
                        ctx.items().give(539, 1)?;
                        ctx.fx().cutin("rutie_snownow02.bmp", 2)?;
                        ctx.lines_as("Snowysnow", args!["Aren't you lucky!", "^3355FF1 Piece Of Cake^000000!"])?;
                    } else if subject3 == 6 {
                        ctx.var("xmas_npc").set(Val::from(11))?;
                        ctx.items().give(539, 2)?;
                        ctx.fx().cutin("rutie_snownow02.bmp", 2)?;
                        ctx.lines_as("Snowysnow", args!["Now, isn't that nice?", "^3355FF2 Piece Of Cake^000000!"])?;
                    } else if subject3 == 7 {
                        ctx.var("xmas_npc").set(Val::from(11))?;
                        ctx.items().give(538, 5)?;
                        ctx.fx().cutin("rutie_snownow02.bmp", 2)?;
                        ctx.lines_as("Snowysnow", args!["Oh woooooow~!", "^3355FF5 Cookie^000000!"])?;
                    } else if subject3 == 8 {
                        ctx.var("xmas_npc").set(Val::from(11))?;
                        ctx.items().give(538, 10)?;
                        ctx.fx().cutin("rutie_snownow02.bmp", 2)?;
                        ctx.lines_as("Snowysnow", args!["*Gasp!* Ooh~", "^3355FF10 Cookie^000000!"])?;
                    }
                    ctx.next()?;
                    ctx.lines_as("Snowysnow", args!["My dear friend, please visit me from time to time, so that we can chitchat, okay? See you soon! And Merry Christmas!"])?;
                    ctx.close_window()?;
                    ctx.fx().cutin("", 255)?;
                    return ctx.end();
                } else if subject2 == 11 {
                    ctx.fx().cutin("rutie_snownow02.bmp", 2)?;
                    ctx.lines_as("Snowysnow", args!["Hello hello!"])?;
                    ctx.next()?;
                    ctx.lines_as("Snowysnow", args!["You're always welcome in Lutie, especially by me, Snowysnow! Happy Kwanza, Happy Hannukah and Merry Christmas~!"])?;
                    ctx.close_window()?;
                    ctx.fx().cutin("", 255)?;
                    return ctx.end();
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            ctx.fx().cutin("rutie_snownow01.bmp", 2)?;
            ctx.lines_as("Snowysnow", args!["Around this wonderful town, eternally blessed with Christmas, there is a horrible dungeon, eternally cursed with Christmas."])?;
            ctx.next()?;
            ctx.lines_as("Snowysnow", args!["I've heard that it's well decorated and looks just like a Toy Factory inside, where everything is so cute and pretty. They are Toy Soldiers and Gift Boxes as far as the eye can see!"])?;
            ctx.next()?;
            ctx.fx().cutin("rutie_snownow01.bmp", 2)?;
            ctx.lines_as("Snowysnow", args!["Isn't that soooo exciting?! *Sigh* Even if it is a dungeon, I would like to go inside just to look. If only I was a real boy, or even had legs..."])?;
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
            return ctx.end();
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            ctx.fx().cutin("rutie_snownow03.bmp", 2)?;
            ctx.lines_as(
                "Snowysnow",
                args![
                    "Bye bye, friend~!",
                    "Thank you for listening to me~",
                    "I'll see you again, someday! You'll always be in my heart~"
                ],
            )?;
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
            return ctx.end();
        }
    }
    Ok(())
}

pub fn hashokii(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Hashokii",
        args!["Meeee~RrrrrYYYY Christmas~!", "La La La~!", "Dum di Dum di Dum!"],
    )?;
    ctx.next()?;
    let subject1 = ctx.menu(&["Yo Clown boy, what's up?", "About Snowysnow", "Quit conversation"])?;
    if subject1 == 0 {
        ctx.lines_as(
            "Hashokii",
            args![
                "La La La~!",
                "Dum di Dum di Dum!",
                "Ooh, I'm trying to think of a good show to put on for Charu Charu and Marcell!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Hashokii", args!["They are getting smarter and wittier everyday, and now it seems that they don't laugh at my best jokes anymore. How did they get to be so clever?"])?;
        ctx.next()?;
        ctx.lines_as("Hashokii", args!["Well, if I work hard enough, they can't help but laugh at my hilarious jokes! So... I better start inventing better jokes. Like, pronto."])?;
        ctx.next()?;
        ctx.lines_as("Hashokii", args!["La La La~!", "Dum di Dum di Dum", "Merry Christmas!"])?;
        return ctx.close();
    } else if subject1 == 1 {
        if ctx.var("xmas_npc").get()? == 8 {
            ctx.lines_as(
                "Hashokii",
                args!["Dum di Dum di Dum", "Ah ha! So you wanna learn more about Snowyshow! Let's see..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hashokii",
                args![
                    "Well, there are two naughty kids,",
                    "^3355FF' Charu Charu '^000000 and",
                    "^3355FF' Marcell '^000000. They attend my show regularly. I'm guessing you've heard the story from Cantata?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Hashokii", args!["Anyway, the two babies that were protected in Snowysnow's bosom? Yup, that's them. But Charu Charu and Marcell don't seem to know that Snowysnow saved them."])?;
            ctx.next()?;
            ctx.lines_as("Hashokii", args!["Snowysnow told me the story of how he let his body fly into the air to block the giant fire ball that was about to hit them when they were babies. For their sake, he was willing to sacrifice himself."])?;
            ctx.next()?;
            ctx.lines_as("Hashokii", args!["Why don't you go meet those 2 children? They might tell you the story we've never got the chance to hear. Okay then, good luck~! Bye bye!"])?;
            ctx.var("xmas_npc").set(Val::from(9))?;
            return ctx.close();
        } else {
            ctx.lines_as("Hashokii", args!["Ah... ^3355FFSnowysnow^000000?", "Of course I know him! Anyone who doesn't know Snowysnow is a total stranger around here! Sometimes, he and I share a nice chat..."])?;
            ctx.next()?;
            ctx.lines_as("Hashokii", args!["He makes such a good audience for my show. But to be honest, I'm not sure if he really likes it or not. Most people don't seem to care for my jokes."])?;
            ctx.next()?;
            ctx.lines_as(
                "Hashokii",
                args!["It totally baffles me! How could they not like the best jokes in the world?! Sheeeeesh~"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hashokii",
                args![
                    "Hmmm, sorry!",
                    "Anyway, Snowysnow",
                    "is a great guy!",
                    "La La La~!",
                    "Dum di Dum di Dum",
                    "Merry Christmas- !!"
                ],
            )?;
            return ctx.close();
        }
    } else if subject1 == 2 {
        ctx.lines_as("Hashokii", args!["La La La~!", "Dum di Dum di Dum", "Merry Christmas~!"])?;
        return ctx.close();
    }
    Ok(())
}

pub fn little_boy(ctx: &Ctx) -> Script {
    if ctx.var("xmas_npc").get()? == 9 {
        ctx.lines_as("Charu Charu", args!["Errrm?", "Snowysnow?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Charu Charu",
            args![
                "Hmmm, well...",
                "He's a nice snowman!",
                "You want to know more about Snowysnow? Ummm, I'm not that smart! Ask Marcell!"
            ],
        )?;
        return ctx.close();
    } else {
        ctx.lines_as("Charu Charu", args!["Merry Merry Christmas!", "Heheheheheh~!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Charu Charu",
            args!["Did you talk to that clown guy over there? Isn't he soooooo booooring? (-.-)"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Charu Charu",
            args!["When Marcell and I watch his show, we feel like we're getting dumber and dumber~"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marcell",
            args!["Charu Charu!! Watch your mouth! How dare you say that about poor Hashokii?! He's always trying hard to make us happy!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Charu Charu",
            args![
                "Yeah, yeah.",
                "Whatever~",
                "I already know that!",
                "But he's not funny at all!",
                "I'd rather stay with ^3355FFSnowysnow^000000~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Charu Charu",
            args!["Oh well, if you didn't visit Snowysnow yet, you should see him at least once. He's funny!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Charu Charu", args!["Merry Christmas!", "Enjoy your Holiday in Lutie~!"])?;
        return ctx.close();
    }
}

pub fn little_girl(ctx: &Ctx) -> Script {
    if ctx.var("xmas_npc").get()? == 9 || ctx.var("xmas_npc").get()? == 10 {
        let subject1 = ctx.var("xmas_npc").get()?;
        if subject1 == 9 {
            ctx.lines_as("Marcell", args!["You mean Snowysnow?", "Of course I know him!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Marcell",
                args![
                    "He's a nice and funny guy!",
                    "And as Charu Charu always insists, he's funnier than Hashokii~ (But please don't let Hashokii know!)"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Marcell", args!["Well, Charu Charu and I are orphans, and don't remember our parents at all. We've been brought up by the people here in Lutie."])?;
            ctx.next()?;
            ctx.lines_as("Marcell", args!["Uncle Cantata and Auntie Thachentze treated us like their own children, and Poze and Duffle have been like a brother and sister to us!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Marcell",
                args!["They're all nice and generous, and we always appreciate what they've done to take care of us."],
            )?;
            ctx.next()?;
            ctx.lines_as("Marcell", args!["I also heard Snowysnow doesn't have a mommy or daddy too. And I also heard Snowysnow and us weren't born here, but somewhere else."])?;
            ctx.next()?;
            ctx.lines_as("Marcell", args!["I've heard that Snowysnow and us actually come from the same place, although I'm not sure yet. But I know that Snowysnow and me have the same kind of burns on our body."])?;
            ctx.next()?;
            ctx.lines_as("Marcell", args!["Charu Charu and I have these old burns on our backs, and Snowysnow has a dark smudge on his tummy. So I think we got burned all at the same time..."])?;
            ctx.next()?;
            ctx.lines_as("Marcell", args!["Oh, now I see . . . . .", "You wanna learn all about Snowysnow because you want to become his friend! He'll be so happy to know that! Ooh! Maybe he'll give you a present! Good luck!"])?;
            ctx.var("xmas_npc").set(Val::from(10))?;
            return ctx.close();
        } else if subject1 == 10 {
            ctx.lines_as("Marcell", args!["More than anybody else, you know the most about Snowysnow! Please talk to Mr.Snowysnow, he'll be happy to know you care about him. Merry Christmas!"])?;
            return ctx.close();
        }
    } else {
        ctx.lines_as("Marcell", args!["Merry Christmas~!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Marcell",
            args!["It's freezing out here...! And Charu Charu makes me colder with his unbearable jokes. And the wind's blowing so hard!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marcell",
            args!["You know what? Snowysnow has a special power. He can make as many presents as Santa Claus! Isn't that great?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Marcell", args!["Huh? What's that look on your face for? Snowysnow has a big gift bag inside of his body, and gives gifts whenever he feels like it. What's so hard to believe about that?"])?;
        return ctx.close();
    }
    Ok(())
}
