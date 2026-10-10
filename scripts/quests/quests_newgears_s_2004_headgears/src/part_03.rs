use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum OrcWarrior1Step {
    Start,
    OnTouch,
}

fn orc_warrior_1_run(ctx: &Ctx, mut step: OrcWarrior1Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_total_vouchers = Val::from(0);
    'machine: loop {
        match step {
            OrcWarrior1Step::Start => {
                step = OrcWarrior1Step::OnTouch;
                continue 'machine;
            }
            OrcWarrior1Step::OnTouch => {
                if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
                    ctx.lines(args![
                        "- Wait a minute !! -",
                        "- Currently you're carrying -",
                        "- too many items with you. -",
                        "- Please try again -",
                        "- after you lose some weight. -"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("orcs_hero_hat").get()?.number()? < 1 {
                    ctx.lines_as(
                        "Orc Warrior",
                        args![".......", ".........", "Human, what are you doing in my home?!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Orc Warrior",
                        args![
                            "Hurrumph.",
                            "I guess you decided to come inside, thinking no one was around, to rest after killing others of my kind."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Orc Warrior",
                        args![
                            "Well! We Orcs are a proud race!",
                            "So don't expect me to talk to you.",
                            "Though, I must admit that you have courage to enter an Orc's home."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Orc Warrior", args!["I'm too tired to kill you today, but keep silent! If you're wounded, you can bleed quietly on the floor and die with some honor I suppose."])?;
                    if (ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])? == 5 && ctx.var("BaseLevel").get()?.number()? >= 55) {
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from(".....:Come on, let's talk.")])?) == 1 {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Orc Warrior", args!["You dare invite an Orc to conversation?! We Orcs are a savage race! Born out suffering, we only live to fight and do battle!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Orc Warrior",
                            args![
                                "Talk with humans?!",
                                "Hah! The only way I will suffer such humiliation would be if you too, lost your dignity."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Orc Warrior",
                            args![
                                "You can bleed in my house, but I will only talk with you if...",
                                "You bring ^FF00001000 Jellopy^000000!"
                            ],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("I challenge you to a duel, Orc!:1000 Jellopies it is.")],
                        )?) == 1
                        {
                            ctx.lines_as(
                                "Orc Warrior",
                                args![
                                    "Humans... such annoying bastards!",
                                    "If you want to fight, go outside!",
                                    "So get out of my house!"
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("gef_fild10"), Val::from(100), Val::from(100)])?;
                            return Err(Stop::End);
                        }
                        ctx.var("orcs_hero_hat").set(Val::from(1))?;
                        ctx.lines_as(
                            "Orc Warrior",
                            args![
                                "Hah! It's a promise then!",
                                "Bwahahahahhahahahahahaha!",
                                "That's 1000 Jellopies!",
                                "Are all you humans so foolish?!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("orcs_hero_hat").get()? == 1 {
                        if ctx.call(Function::CountItem, vec![Val::from(909)])?.number()? > 999 {
                            ctx.lines_as("Orc Warrior", args!["WHAT?!", "That's so much Jellopy!!"])?;
                            ctx.next()?;
                            ctx.lines_as("Orc Warrior", args!["I do not like this joke you have played on me human. Still, I refuse to believe that you have really brought ^FF00001000 Jellopy^000000."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Orc Warrior",
                                args!["We Orcs are a proud race and do not like to admit when we are wrong."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Orc Warrior", args!["One..."])?;
                            ctx.next()?;
                            ctx.lines_as("Orc Warrior", args!["One... T-two..."])?;
                            ctx.next()?;
                            ctx.lines_as("Orc Warrior", args!["One... T-two...", "Three..."])?;
                            ctx.next()?;
                            ctx.lines_as("Orc Warrior", args!["One... T-two...", "Three...", "....F-f-fooooour..."])?;
                            ctx.next()?;
                            ctx.lines_as("Orc Warrior", args!["......."])?;
                            ctx.next()?;
                            ctx.lines_as("Orc Warrior", args![".......", "........."])?;
                            ctx.next()?;
                            ctx.lines_as("Orc Warrior", args![".......", ".........", "............"])?;
                            ctx.next()?;
                            ctx.lines_as("Orc Warrior", args!["Nine hundred ninety-eight..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Orc Warrior",
                                args!["Nine hundred ninety-eight...", "Nine hundred ninety-nine..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Orc Warrior",
                                args!["Nine hundred ninety-eight...", "Nine hundred ninety-nine...", "One..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Orc Warrior",
                                args![
                                    "Nine hundred ninety-eight...",
                                    "Nine hundred ninety-nine...",
                                    "One... One thousand."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Orc Warrior", args!["Curse you, human.", "You have shall your 'conversation.'"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Orc Warrior",
                                args!["I hate humans! In fact, all Orcs hate humans! Hate you hate you hate you with a passion!"],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::DelItem, vec![Val::from(909), Val::from(1000)])?;
                            ctx.var("orcs_hero_hat").set(Val::from(2))?;
                            ctx.lines_as("Orc Warrior", args!["So...", "Still in the mood to talk?!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Orc Warrior",
                                args!["Tough!", "If you want to", "hear more, bring...", "^FF00001000 Jellopy^000000!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Orc Warrior",
                                args!["Now get out of here, you horrible stinking, poor excuse for an animal!!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Orc Warrior",
                            args![
                                "Did you already forget?!",
                                "^FF00001000 Jellopy^000000!",
                                "Show me your respect",
                                "if you want to talk."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("orcs_hero_hat").get()? == 2 {
                            if ctx.call(Function::CountItem, vec![Val::from(909)])?.number()? > 999 {
                                ctx.lines_as(
                                    "Orc Warrior",
                                    args!["No.... Not again!!", "What are you doing with so much Jellopy?!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Orc Warrior",
                                    args![
                                        "Damn...!",
                                        "I refuse to speak to you until I've finished counting all of this Jellopy."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Orc Warrior", args!["..."])?;
                                ctx.next()?;
                                ctx.lines_as("Orc Warrior", args!["...", "....."])?;
                                ctx.next()?;
                                ctx.lines_as("Orc Warrior", args!["...", ".....", "ONE THOUSAND."])?;
                                ctx.next()?;
                                ctx.lines_as("Orc Warrior", args!["Once again, human, you've managed to infuriate me. But we Orcs are a proud race. I shall fulfill my part of this pathetic deal."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Orc Warrior",
                                    args!["Now, where was I?", "Oh right.", "How much I hate humans."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Orc Warrior", args!["Humans are always invading our village, intruding on our land! They kill our warriors not out of honor, but for selfish human reasons!!"])?;
                                ctx.next()?;
                                ctx.lines_as("Orc Warrior", args!["Bah! Humans have built their own towns and cities. Do they keep their senseless killing amongst each other?! NO! Humans have to rampage and ravage everything!!"])?;
                                ctx.next()?;
                                ctx.call(Function::DelItem, vec![Val::from(909), Val::from(1000)])?;
                                ctx.var("orcs_hero_hat").set(Val::from(3))?;
                                ctx.lines_as("Orc Warrior", args!["Now, if you want", "to listen to more,", "bring me..."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Orc Warrior",
                                    args![
                                        "^FF00001000 Jellopy^000000 again!",
                                        "I do not think that you can endure much more of this torture.",
                                        "Bwahahahahahha~!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Orc Warrior",
                                args![
                                    "Did you already forget!?",
                                    "^FF00001000 Jellopy^000000!",
                                    "Show me your respect",
                                    "if you want to talk."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("orcs_hero_hat").get()? == 3 {
                                if ctx.call(Function::CountItem, vec![Val::from(909)])?.number()? > 999 {
                                    ctx.lines_as("Orc Warrior", args![".....", "Again, human, you confound me with your stubborness. I see that humans are masochists. Or just like to annoy Orcs by making them count a thousand Jellopies."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Orc Warrior", args!["*sigh* Let me count..."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Orc Warrior", args!["..."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Orc Warrior", args!["...", "......."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Orc Warrior", args!["Nine hundred ninety-nine..."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Orc Warrior", args!["Nine hundred ninety-nine...", "...A thousand."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Orc Warrior",
                                        args!["Argh~!", "Now where was I?", "Oh right, why humans are scum."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Orc Warrior",
                                        args![
                                            "How dare humans think they're so great?! We Orcs have also made contributions to the world!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Orc Warrior", args!["Humans may have built cities and towns, but we Orcs have also created our own cave complexes and canals!"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Orc Warrior", args!["We Orcs have invented many useful things! Of course, now we've had to keep these things to ourselves. Why? Because of the stupidity and greed of humans!!"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Orc Warrior", args!["We Orcs have once constructed a tower for a human, but were then trapped and enslaved there. Needless to say, humans are deserving of our wrath and fury."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Orc Warrior",
                                        args![
                                            "*Sigh*",
                                            "I don't understand why I am sharing this story about my tribe.",
                                            "That's enough for today!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::DelItem, vec![Val::from(909), Val::from(1000)])?;
                                    ctx.var("orcs_hero_hat").set(Val::from(4))?;
                                    ctx.lines_as(
                                        "Orc Warrior",
                                        args!["Grrr~! If you want to listen longer go bring me ^FF00001000 Jellopy^000000 again."],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Orc Warrior",
                                    args![
                                        "Did you already forget!?",
                                        "^FF00001000 Jellopy^000000!",
                                        "Show me your respect if",
                                        "you wish to talk!!",
                                        "I don't take the 2,000 jellopies you have given to me seriously!!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("orcs_hero_hat").get()? == 4 {
                                    if ctx.call(Function::CountItem, vec![Val::from(909)])?.number()? > 999 {
                                        ctx.lines_as("Orc Warrior", args!["Oh... it's you again.", "I can see the Jellopy in your hands, and I'm pretty sure you've brought a thousand again, so let's get this over with."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Orc Warrior", args!["*Sigh* I suppose it's time for our conversation."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Orc Warrior", args!["Once upon a time, Orcs and humans were allies. I... I miss those times. But it's no use to long for old comraderies. It's impossible to clear up such misunderstandings from ages ago..."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Orc Warrior", args!["There are too many wicked humans that have pursued wealth and fame at any cost. They don't understand ideals like honor and loyalty, and use any means to get whatever they want."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Orc Warrior", args!["Those hateful humans started to get rid of every obstacle that stood between them and what they wanted, not caring if friend or foe stood in their way."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Orc Warrior",
                                            args!["...I've talked too much already.", "That's the end of the conversation for today."],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::DelItem, vec![Val::from(909), Val::from(1000)])?;
                                        ctx.var("orcs_hero_hat").set(Val::from(5))?;
                                        ctx.lines_as(
                                            "Orc Warrior",
                                            args![
                                                "Now, if you want to listen some more, go bring me",
                                                "^FF00001000 Jellopy^000000 again tomorrow."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines_as(
                                        "Orc Warrior",
                                        args![
                                            "Did you already forget?",
                                            "^FF00001000 Jellopy^000000!",
                                            "Show me your respect",
                                            "if you want to talk.",
                                            "I don't take the 3,000 jellopies you have given to me seriously."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("orcs_hero_hat").get()? == 5 {
                                        if ctx.call(Function::CountItem, vec![Val::from(909)])?.number()? > 999 {
                                            ctx.lines_as(
                                                "Orc Warrior",
                                                args![
                                                    "*Sigh...*",
                                                    "You're too tough to get rid of.",
                                                    "Do you really want to listen to me so much...?",
                                                    "Okay... I will tell you a little bit more then."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Orc Warrior", args!["For us Orcs, victory in battle is the greatest honor.", "We fight boldly and fearlessly with our opponent, giving our all in hopes of a fair victory. We would die for victory in battle."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Orc Warrior", args!["Even after death, we never lose the spirit of combat. It is said that some Orcs that have died unfairly will stand from their graves to seek even more battle."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Orc Warrior", args!["The difference between you humans and us is that we would never tolerate foul play in battle. But more importantly, Orcs never, ever retreat. An Orc will always fight till the last breath..."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Orc Warrior", args!["Orcs never restore health during battle using your silly medicines. Hmpf. Humans can be so weak, drinking bottles of Potion to keep up in a fight. Oh well, though, I'm not sure if you're the same."])?;
                                            ctx.next()?;
                                            ctx.call(Function::DelItem, vec![Val::from(909), Val::from(1000)])?;
                                            ctx.var("orcs_hero_hat").set(Val::from(6))?;
                                            ctx.lines_as(
                                                "Orc Warrior",
                                                args![
                                                    "Now, if you want",
                                                    "to listen to me",
                                                    "some more, go and",
                                                    "bring me ^FF00001000 Jellopy^000000",
                                                    "again tomorrow."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as(
                                            "Orc Warrior",
                                            args![
                                                "Did you already forget?",
                                                "^FF00001000 Jellopy^000000!",
                                                "Show me your respect",
                                                "if you want to talk.",
                                                "I don't take the 4,000 jellopies you have given to me seriously."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.var("orcs_hero_hat").get()? == 6 {
                                            if ctx.call(Function::CountItem, vec![Val::from(909)])?.number()? > 999 {
                                                ctx.lines_as("Orc Warrior", args!["What the...! Don't you know when to stop? I do not believe a human would like to hear about Orcs so much. But...", "We Orcs are a proud race, and I shall oblige."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Orc Warrior", args!["We only select the strongest of our tribe to become warriors. When Orcs reach adulthood, they are sent to the forest to survive alone in its environment."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Orc Warrior", args!["Those Orcs that come back alive are recognized as true members of the tribe. The strongest males are chosen to become Orc Warriors, and the strongest females are chosen as Orc Ladies."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Orc Warrior", args!["Orcs that have proven to have good sight and dexterity are given bows and arrows and become Orc Archers."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Orc Warrior", args!["We Orcs have a special training course called 'The Trial of Fire.' If any Orc Warrior passes this course, he will be protected by our fire god..."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Orc Warrior", args!["That Orc will have stronger physical strength and abilities than normal Orc Warriors. These warriors are known as High Orcs."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Orc Warrior", args!["Hm, well to you humans, we may all look the same, but we Orcs can tell each other apart. How about you, human? Do I look the same as every Warrior outside to you?"])?;
                                                ctx.next()?;
                                                ctx.call(Function::DelItem, vec![Val::from(909), Val::from(1000)])?;
                                                ctx.var("orcs_hero_hat").set(Val::from(7))?;
                                                ctx.lines_as(
                                                    "Orc Warrior",
                                                    args![
                                                        "Now...",
                                                        "If you want to",
                                                        "listen longer,",
                                                        "go bring me",
                                                        "^FF00001000 Jellopy^000000",
                                                        "again tomorrow."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            ctx.lines_as(
                                                "Orc Warrior",
                                                args![
                                                    "Did you already forget?",
                                                    "^FF00001000 Jellopy^000000!",
                                                    "Show me your respect",
                                                    "if you want to talk.",
                                                    "I don't take the 5,000 jellopies you have given to me seriously."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.var("orcs_hero_hat").get()? == 7 {
                                                if ctx.call(Function::CountItem, vec![Val::from(909)])?.number()? > 999 {
                                                    ctx.lines_as("Orc Warrior", args!["I expected you to quit by now. But I was wrong, because you're here once again. We made a deal and I will keep my promise."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Orc Warrior",
                                                        args!["Last time, we were", "talking about our", "warriors..."],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Orc Warrior", args!["Sometimes, a few Orcs are born among us, possessing phenomenal strength and talent. It is rumored that they could compete with 10,000 warriors at a time..."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Orc Warrior", args!["They are the best warriors of the Orc tribe, and are known to you as Orc Heroes. Not only are they the strongest ones among us, but they also our leaders in battle."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Orc Warrior", args!["As proof of their status, Orc Heroes are given Vouchers of Orcish Hero, as well as special swords that are only allowed to be used by our heroes."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Orc Warrior", args!["That sword can tear the sky, sever the earth and make a waterfall flow backwards."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Orc Warrior", args!["The special helmet that is only worn by Orc Heroes represents the power and honor of the Orc Warriors."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Orc Warrior", args!["Well, humans rarely get the chance to see this helmet unless they're in battle with an Orc Hero themselves..."])?;
                                                    ctx.next()?;
                                                    ctx.call(Function::DelItem, vec![Val::from(909), Val::from(1000)])?;
                                                    ctx.var("orcs_hero_hat").set(Val::from(8))?;
                                                    ctx.lines_as(
                                                        "Orc Warrior",
                                                        args![
                                                            "Now, if you want",
                                                            "to listen longer,",
                                                            "go and bring me",
                                                            "^FF00001000 Jellopy^000000",
                                                            "again tomorrow."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                ctx.lines_as(
                                                    "Orc Warrior",
                                                    args![
                                                        "Did you already forget?",
                                                        "^FF00001000 Jellopy^000000!",
                                                        "Show me your respect",
                                                        "if you want to talk."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Orc Warrior",
                                                    args!["I don't take the 6,000 jellopies you have given to me that seriously."],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                if ctx.var("orcs_hero_hat").get()? == 8 {
                                                    if ctx.call(Function::CountItem, vec![Val::from(909)])?.number()? > 999 {
                                                        ctx.lines_as(
                                                            "Orc Warrior",
                                                            args![
                                                                "Oh, hello. You're back.",
                                                                "Go ahead and put the 1000 Jellopies over there on the table by the skulls."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Orc Warrior", args!["Ah yes, we were talking about Orc Heroes. Now, unlike Orc Heroes who are blessed with might from the day they were born, there are older, seasoned warriors that have managed to survive many years of combat."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Orc Warrior", args!["To Orc Warriors, we do not see age as weakness. We look up to old warriors as our trainers due to their experience in battle and their fighting skills."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Orc Warrior", args!["Those old warriors have shown outstanding fighting abilities, even without the use of weapons."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Orc Warrior", args!["They could endure in battle for at least few days without axes. When you get to a certain fighting skill level, weapons can actually become a burden..."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Orc Warrior", args!["Among those well-experienced warriors, the greatest one of all is given the title of Orc Lord, and is given our respect as the leader of our tribe."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Orc Warrior", args!["Every Orc can recognize him by his distinguished helm with the three horns."])?;
                                                        ctx.next()?;
                                                        ctx.call(Function::DelItem, vec![Val::from(909), Val::from(1000)])?;
                                                        ctx.var("orcs_hero_hat").set(Val::from(9))?;
                                                        ctx.lines_as(
                                                            "Orc Warrior",
                                                            args![
                                                                "Now...",
                                                                "If you want to",
                                                                "listen longer,",
                                                                "go bring me",
                                                                "^FF00001000 Jellopy^000000",
                                                                "again tomorrow."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    ctx.lines_as(
                                                        "Orc Warrior",
                                                        args![
                                                            "Did you already forget?",
                                                            "^FF00001000 Jellopy^000000!",
                                                            "Show me your respect",
                                                            "if you want to talk.",
                                                            "I don't take the 7,000 jellopies you have given to me seriously."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    if ctx.var("orcs_hero_hat").get()? == 9 {
                                                        if ctx.call(Function::CountItem, vec![Val::from(909)])?.number()? > 999 {
                                                            ctx.lines_as("Orc Warrior", args!["It's been a while I've seen such a determined human such as yourself. Just leave the Jellopy near the bed."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Orc Warrior",
                                                                args!["Now...", "Let me tell you", "more about the", "Orc Tribe."],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Orc Warrior", args!["Although I don't like to admit it, there are cowardly Orcs that run away from battle, or fight using dishonorable methods.", "Needless to say, they have brought shame and disgrace to our tribe."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Orc Warrior", args!["A curse is placed on those dishonorable Orcs so that they can be singled out by the tribe and banished from the village."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Orc Warrior", args!["You may recognize those cursed Orcs by their shrunken bodies and long teeth."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Orc Warrior", args!["We call these cursed Orcs Zenorc. They are forced to live inside of caves or dungeons, away from from honorable Orcs. It's difficult to believe that Zenorcs were once one of us..."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Orc Warrior", args!["I don't think there is a curse which transforms humans into a totally ugly creature depending on their deeds. So it is difficult to tell which humans are honorable..."])?;
                                                            ctx.next()?;
                                                            ctx.call(Function::DelItem, vec![Val::from(909), Val::from(1000)])?;
                                                            ctx.var("orcs_hero_hat").set(Val::from(10))?;
                                                            ctx.lines_as(
                                                                "Orc Warrior",
                                                                args![
                                                                    "Now...",
                                                                    "If you want",
                                                                    "to hear more,",
                                                                    "go bring me",
                                                                    "^FF00001000 Jellopy^000000",
                                                                    "again tomorrow."
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                        ctx.lines_as(
                                                            "Orc Warrior",
                                                            args![
                                                                "Did you already forget?",
                                                                "^FF00001000 Jellopy^000000!",
                                                                "Show me your respect",
                                                                "if you want to talk.",
                                                                "I don't take the 8,000 jellopies you have given to me seriously."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else {
                                                        if ctx.var("orcs_hero_hat").get()? == 10 {
                                                            if ctx.call(Function::CountItem, vec![Val::from(909)])?.number()? > 999 {
                                                                ctx.lines_as("Orc Warrior", args!["With these...", "You have brought me 10,000 Jellopies. I must say that I am impressed. Since you have shown me your respect, I will repay you with my honesty."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Orc Warrior", args!["Considering the stories about my tribe, it's enough for me if you just listen. However, merely listening is totally different from having the experience as your own."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Orc Warrior", args!["I suggest you that you experience more battles with the many different Orcs out there. That's the best way of understanding and learning about the Orc tribe."])?;
                                                                ctx.next()?;
                                                                ctx.call(Function::DelItem, vec![Val::from(909), Val::from(1000)])?;
                                                                ctx.var("orcs_hero_hat").set(Val::from(11))?;
                                                                ctx.call(Function::GetItem, vec![Val::from(1304), Val::from(1)])?;
                                                                ctx.lines_as("Orc Warrior", args!["This is a small token of my gratitue. This axe used to aid me in many battles and is very precious to me. I hope you will take care of this."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Orc Warrior", args!["You have made a lot of effort to show me respect, despite being a human. Now, if you want to learn more about Orcs, feel free to come back anytime, my friend."])?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            }
                                                            ctx.lines_as(
                                                                "Orc Warrior",
                                                                args![
                                                                    "Did you already forget?",
                                                                    "^FF00001000 Jellopy^000000!",
                                                                    "Show me your respect",
                                                                    "if you want to talk.",
                                                                    "I don't take 9,000 jellopies you have given to me seriously."
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else if ctx.var("orcs_hero_hat").get()? == 11 {
                                                            if ctx.call(Function::CountItem, vec![Val::from(1304)])?.number()? > 0 {
                                                                ctx.lines_as("Orc Warrior", args!["You have made a lot of effort to show me your respect. Now, if you want to learn more about Orcs, feel free to come back anytime, my friend."])?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else if ctx.call(Function::CountItem, vec![Val::from(931)])?.number()? > 99 {
                                                                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])? == 5 {
                                                                    ctx.lines_as("Orc Warrior", args!["Oh, I see you've taken my advice and have been meeting many other Orcs through battle.", "You must have learned much about the Orc's spirit of battle by seeing it for yourself firsthand."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Orc Warrior", args!["These Orchish Vouchers you have been collecting are the true token of Orc warriors. We recognize an Orc who won over 1,000 battles as a true Orc Warrior."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Orc Warrior", args!["Still...", "You're a human, so I don't give we can give you that honor so easily..."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Orc Warrior",
                                                                        args!["Alright...", "Go win over 10,000 battles."],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Orc Warrior", args!["Then, I'm sure you can be granted status as a real Orc warrior. But, make sure you are fighting for honorable purposes, and that you have no other motive."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Orc Warrior", args!["You only achieve honor by winning battles. Even among Orcs, I'm sure you can prove your battle prowess by defeating 10,000 Orc Warriors."])?;
                                                                    ctx.next()?;
                                                                    if Val::from(runtime::select_values(
                                                                        ctx,
                                                                        &[Val::from(
                                                                            "I don't think that is necessary.:I will win 10,000 battles!",
                                                                        )],
                                                                    )?) == 1
                                                                    {
                                                                        ctx.lines_as("Orc Warrior", args!["I see. I will not force you to choose what I wish to see you do. I respect your decision."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Orc Warrior", args!["Although...", "I wished for other Orcs to see and understand that not all humans are wicked and selfish."])?;
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    }
                                                                    ctx.lines_as("Orc Warrior", args!["Ah, spoken like a true warrior. Alright then, go forth and do battle with others of my tribe, and bring me 10,000 Orcish Vouchers."])?;
                                                                    ctx.next()?;
                                                                    ctx.call(Function::DelItem, vec![Val::from(931), Val::from(100)])?;
                                                                    ctx.var("orcs_hero_hat").set(Val::from(13))?;
                                                                    ctx.var("orcs_hero_hat2").set(Val::from(100))?;
                                                                    ctx.lines_as("Orc Warrior", args!["I took 100 Orcish Voucher from you now. Now, fight and fight until you have defeated 10,000 more Orc Warriors so that the whole tribe will have to recognize you!"])?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                                ctx.lines_as("Orc Warrior", args!["Hm? You don't seem to understand our way of life completely. It's strange that you are so curious about other races, human."])?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else {
                                                                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])? == 5 {
                                                                    ctx.lines_as("Orc Warrior", args!["The best way of understanding the Orc tribe is to do battle with them. By competing in power and skills, eventually you will understand and respect your opponent."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Orc Warrior", args!["So, go outside and fight.", "Although you may be learning theory by talking with me, you must experience our way of life for yourself."])?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                                ctx.lines_as("Orc Warrior", args!["Hm? You don't seem to understand our way of life completely. It's strange that you are so curious about other races, human."])?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            }
                                                        } else {
                                                            if (ctx.var("orcs_hero_hat2").get()?.number()? > 99
                                                                && ctx.var("orcs_hero_hat2").get()?.number()? < 10000)
                                                            {
                                                                if ctx.call(Function::CountItem, vec![Val::from(931)])?.number()? > 0 {
                                                                    if ctx.var("orcs_hero_hat2").get()?.number()? > 9999 {
                                                                        ctx.var("orcs_hero_hat2").set(Val::from(10000))?;
                                                                    }
                                                                    ctx.lines_as("Orc Warrior", args!["You've come back...", "It doesn't seem that you've accomplished your goal yet, but I do not expect you to defeat 10,000 Orc Warriors so easily."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Orc Warrrior", args![((Val::from("If you wish, you may rest here. ") + ctx.var("orcs_hero_hat2").get()?) + Val::from(" victories over Orc Warriors are recognized by our tribe."))])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Orc Warrior",
                                                                        args!["Do you wish to", "record your current", "victory with me?"],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    if Val::from(runtime::select_values(
                                                                        ctx,
                                                                        &[Val::from("Yes, I do.:I will do it later.")],
                                                                    )?) == 1
                                                                    {
                                                                        l_total_vouchers = (ctx.var("orcs_hero_hat2").get()?
                                                                            + ctx.call(Function::CountItem, vec![Val::from(931)])?);
                                                                        if l_total_vouchers.clone().number()? < 10000 {
                                                                            ctx.call(
                                                                                Function::DelItem,
                                                                                vec![
                                                                                    Val::from(931),
                                                                                    ctx.call(Function::CountItem, vec![Val::from(931)])?,
                                                                                ],
                                                                            )?;
                                                                            ctx.var("orcs_hero_hat2").set(l_total_vouchers.clone())?;
                                                                            ctx.lines_as(
                                                                                "Orc Warrior",
                                                                                args![
                                                                                    "I hope you will",
                                                                                    "continue your efforts",
                                                                                    "in understanding the Orc",
                                                                                    "way of life through battle.",
                                                                                    "Don't give up."
                                                                                ],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Orc Warrior", args![((Val::from("There are ") + (Val::from(10000).try_sub(l_total_vouchers.clone())?)) + Val::from(" battles ahead of you before you reach your goal. I understand the difficulty of this challenge, but I hope you make it."))])?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        } else {
                                                                            ctx.lines_as(
                                                                                "Orc Warrior",
                                                                                args![
                                                                                    "Now...",
                                                                                    "You won over",
                                                                                    "10,000 battles",
                                                                                    "with Orcs."
                                                                                ],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Orc Warrior", args!["For a human, accomplishing such a feat is truly astonishing. Through all of that fighting, I'm sure that you have learned both good and bad things about my tribe."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as(
                                                                                "Orc Warrior",
                                                                                args![
                                                                                    "Although you're a",
                                                                                    "human, you have",
                                                                                    "demonstrated incredible",
                                                                                    "bravery and honor...",
                                                                                    "I shall grant you true",
                                                                                    "Orc Warrior status!!"
                                                                                ],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Orc Warrior", args!["You are now..."])?;
                                                                            if ctx
                                                                                .var("Sex")
                                                                                .get()?
                                                                                .loosely_equals(&ctx.constant("SEX_MALE")?)
                                                                            {
                                                                                ctx.mes("an Orc Warrior!!")?;
                                                                            } else {
                                                                                ctx.mes("an Orc Lady!!")?;
                                                                            }
                                                                            ctx.next()?;
                                                                            ctx.call(
                                                                                Function::DelItem,
                                                                                vec![
                                                                                    Val::from(931),
                                                                                    ctx.call(Function::CountItem, vec![Val::from(931)])?,
                                                                                ],
                                                                            )?;
                                                                            ctx.var("orcs_hero_hat").set(Val::from(14))?;
                                                                            ctx.var("orcs_hero_hat2").set(Val::from(10000))?;
                                                                            ctx.call(
                                                                                Function::GetItem,
                                                                                vec![Val::from(2299), Val::from(1)],
                                                                            )?;
                                                                            ctx.lines_as("Orc Warrior", args!["This is a present for you. I am not sure if it will fit to your head or not, but try it. If your head is too big to wear this, I suggest that you carry this with you."])?;
                                                                            ctx.next()?;
                                                                            ctx.call(
                                                                                Function::GetItem,
                                                                                vec![Val::from(931), Val::from(1)],
                                                                            )?;
                                                                            ctx.lines_as(
                                                                                "Orc Warrior",
                                                                                args![
                                                                                    "As an Orc Warrior,",
                                                                                    "I will now give you",
                                                                                    "an Orcish Voucher",
                                                                                    "of your very own."
                                                                                ],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Orc Warrior", args!["Although it doesn't look different from the others, please keep this with care. You can only be recognized as an Orc warrior with this."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Orc Warrior", args!["Now, you may go back to where you have come from. Come drop by whenever my tribe comes to your mind."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Orc Warrior", args!["As you are now one who feels the true intent of the opponent after endless battles, I will now return you to the battleground."])?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        }
                                                                    }
                                                                    ctx.lines_as("Orc Warrior", args!["I see, do as you wish.", "I will be really disappointed in you if you quit in a challenge you have undertaken."])?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                                ctx.lines_as(
                                                                    "Orc Warrior",
                                                                    args![
                                                                        "Come take a rest here.",
                                                                        ((Val::from("You have won ") + ctx.var("orcs_hero_hat2").get()?)
                                                                            + Val::from(" victories over Orc Warriors."))
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else if ctx.var("orcs_hero_hat").get()? == 14 {
                                                                if (ctx.call(Function::CountItem, vec![Val::from(931)])? == 1
                                                                    && (ctx.call(Function::CountItem, vec![Val::from(2299)])?.number()?
                                                                        > 0
                                                                        || ctx
                                                                            .call(Function::IsEquipped, vec![Val::from(2299)])?
                                                                            .is_true()))
                                                                {
                                                                    ctx.lines_as("Orc Warrior", args!["Hm? You don't think you cannot come back to where you originally came, just because now you're an Orc Warrior, do you? Hahahahaha!"])?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else if (ctx
                                                                    .call(Function::CountItem, vec![Val::from(2299)])?
                                                                    .number()?
                                                                    > 0
                                                                    || ctx.call(Function::IsEquipped, vec![Val::from(2299)])?.is_true())
                                                                {
                                                                    ctx.lines_as(
                                                                        "Orc Warrior",
                                                                        args![
                                                                            "Warrior...",
                                                                            "May a fresh",
                                                                            "light be with",
                                                                            "you in battle."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Orc Warrior", args!["Show your opponents your indomitable spirit. Take pride in being a human that has been given the title of Orc Warrior!"])?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else {
                                                                    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])? == 1 {
                                                                        ctx.lines_as(
                                                                            "Orc Warrior",
                                                                            args!["My my...", "Are you still", "hungry for blood...?"],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Orc Warrior", args!["You don't seem to be satisfied even after being recognized as a warrior. But... I suppose the path of the warrior is to seek new and more difficult challenges."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Orc Warrior", args!["Okay, would you like to go through another test? It may reduce all your effort in coming here to nothing. Even seasoned Orc Warriors fear taking this test."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Orc Warrior", args!["But, since you're an exceptional case, it might be possible for you to accomplish..."])?;
                                                                        ctx.next()?;
                                                                        if Val::from(runtime::select_values(
                                                                            ctx,
                                                                            &[Val::from(
                                                                                "I am already satisfied.:...I will take this challenge.",
                                                                            )],
                                                                        )?) == 1
                                                                        {
                                                                            ctx.lines_as(
                                                                                "Orc Warrior",
                                                                                args![
                                                                                    "Yes, that's a good",
                                                                                    "attitude. Sometimes,",
                                                                                    "you must accept your",
                                                                                    "limitations, or that",
                                                                                    "you cannot beat",
                                                                                    "certain opponents."
                                                                                ],
                                                                            )?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        }
                                                                        ctx.lines_as("Orc Warrior", args!["Excellent, I admire your enthusiasm. This is a very rare chance to do battle with the strongest Orc Warriors."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Orc Warrior", args!["Go fight with 100 Orc Heroes and bring me the tokens of the battles you have endured. Even if you're an excellent warrior, I assure you this test will be the hardest one you'll ever get."])?;
                                                                        ctx.next()?;
                                                                        ctx.var("orcs_hero_hat").set(Val::from(15))?;
                                                                        ctx.lines_as("Orc Warrior", args!["Now, go, warrior! I believe that you have a chance to succeed. Fight with 100 Orc Heroes, and I will grant you, a human, recognition as an Orc Hero!"])?;
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    }
                                                                    ctx.lines_as("Orc Warrior", args!["Warrior, may a fresh light be with you in battle. Show your opponents your indomitable spirit. Take pride in being a human that has been given the title of Orc Warrior!"])?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                            } else {
                                                                if ctx.var("orcs_hero_hat").get()? == 15 {
                                                                    if ctx.call(Function::CountItem, vec![Val::from(968)])?.number()? > 99 {
                                                                        ctx.lines_as(
                                                                            "Orc Warrior",
                                                                            args!["Ah...", "I knew you would succeed!"],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Orc Warrior", args!["The Orc Heroes who were defeated by you also must have recognized your courage. Thank you, human, for showing us what is true strength."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Orc Warrior",
                                                                            args![
                                                                                "On behalf of",
                                                                                "the Orc tribe,",
                                                                                "let me pay",
                                                                                "you homage."
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Orc Warrior", args!["Now, make one last choice. What would you do with those Heroic Emblems? They might not be valuable to you humans, but for Orcs, they are treasured by those who own them."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Orc Warrior", args!["Since you are the victor of those battles, you have every right to keep them. But you could also return them to the Orc Heroes, meaning that you wish to meet them in battle once again."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Orc Warrior",
                                                                            args![
                                                                                "Now, what would",
                                                                                "you want to do",
                                                                                "with the vouchers?"
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        match runtime::select_values(
                                                                            ctx,
                                                                            &[Val::from(
                                                                                "I want to keep them.:I shall return them to the Orc Heroes.",
                                                                            )],
                                                                        )? {
                                                                            1 => {
                                                                                ctx.lines_as(
                                                                                    "Orc Warrior",
                                                                                    args!["Yes, that's", "not bad...", "Not bad at all."],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Orc Warrior", args!["Nobody has the right to tell you what you can or cannot do. Please keep them as a memory of your victory."])?;
                                                                                ctx.close_window()?;
                                                                                return Err(Stop::End);
                                                                            }
                                                                            2 => {
                                                                                ctx.lines_as("Orc Warrior", args!["What a great attitude for a warrior...! It will be a model to other Orcs to show respect to opponents that you have defeated!"])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Orc Warrior", args!["You deserve to be an Orc Hero, the most powerful Orc warrior! As of today, you are now an Orc Hero!"])?;
                                                                                ctx.next()?;
                                                                                ctx.call(
                                                                                    Function::DelItem,
                                                                                    vec![Val::from(968), Val::from(100)],
                                                                                )?;
                                                                                ctx.var("orcs_hero_hat").set(Val::from(16))?;
                                                                                ctx.call(
                                                                                    Function::GetItem,
                                                                                    vec![Val::from(1124), Val::from(1)],
                                                                                )?;
                                                                                ctx.lines_as("Orc Warrior", args!["This is a sword only given to our heroes. I am not sure if you can use this or not, but as an Orc Hero, you're obligated to carry this with you always."])?;
                                                                                ctx.next()?;
                                                                                ctx.call(
                                                                                    Function::GetItem,
                                                                                    vec![Val::from(968), Val::from(1)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Orc Warrior",
                                                                                    args![
                                                                                        "Now that you",
                                                                                        "are an Orc Hero,",
                                                                                        "let me give you",
                                                                                        "your own Heroic",
                                                                                        "Emblem."
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Orc Warrior", args!["Although it doesn't look different from the others, I hope you will keep it with care. You can only be recognized as an Orc Hero with this token."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Orc Warrior", args!["Then, you may leave now....", "A human who has a better understanding than other humans. One who showed others power regardless of the race, I leave you to the future of Chaos."])?;
                                                                                ctx.close_window()?;
                                                                                return Err(Stop::End);
                                                                            }
                                                                            _ => {}
                                                                        }
                                                                    }
                                                                    ctx.lines_as("Orc Warrior", args!["Hm...", "Struggling, are you?"])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Orc Warrior", args!["Well, Orc Hero is held in high regard by other Orcs for their great power. As a human, you will probably have difficulty in dealing with him."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Orc Warrior", args!["However, you made the decision to go through with this test! Still, there's no need to rush it. Life is an endless series of battles, so the combat will come."])?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else if ctx.var("orcs_hero_hat").get()?.number()? > 15 {
                                                                    if ctx.call(Function::CountItem, vec![Val::from(968)])?.number()? < 1 {
                                                                        ctx.lines_as("Orc Warrior", args!["Warrior, where did you leave your Heroic Emblem? Without the token, I cannot recognize you as an Orc Hero. Please find it and keep it with you anywhere you go."])?;
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    } else if (ctx
                                                                        .call(Function::CountItem, vec![Val::from(1124)])?
                                                                        .number()?
                                                                        < 1
                                                                        && ctx.call(Function::IsEquipped, vec![Val::from(1124)])? == 0)
                                                                    {
                                                                        ctx.lines_as("Orc Warrior", args!["Warrior, where did you leave your sword? Without the sword, I cannot recognize you as an Orc Hero. Please find it and keep it with you anywhere you go."])?;
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    }
                                                                    ctx.lines_as(
                                                                        "Orc Warrior",
                                                                        args![
                                                                            "The most",
                                                                            "powerful warrior...",
                                                                            "Orc Hero!",
                                                                            "That's who you are!",
                                                                            "May God be with you in battle!"
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else {
                                                                    ctx.lines_as("Orc Warrior", args!["...................."])?;
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
                    }
                }
            }
        }
    }
}

pub fn orc_warrior_1(ctx: &Ctx) -> Script {
    orc_warrior_1_run(ctx, OrcWarrior1Step::Start, Vec::new()).map(|_| ())
}

pub fn orc_warrior_1_ontouch(ctx: &Ctx) -> Script {
    orc_warrior_1_run(ctx, OrcWarrior1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum OrcHero1Step {
    Start,
    OnTouch,
}

fn orc_hero_1_run(ctx: &Ctx, mut step: OrcHero1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            OrcHero1Step::Start => {
                step = OrcHero1Step::OnTouch;
                continue 'machine;
            }
            OrcHero1Step::OnTouch => {
                if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
                    ctx.lines(args![
                        "- Wait a minute !! -",
                        "- Currently you're carrying -",
                        "- too many items with you. -",
                        "- Please try again -",
                        "- after you lose some weight. -"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("orcs_hero_hat").get()?.number()? < 16 {
                    ctx.lines_as("Orc Hero", args!["Stop bugging me", "and get outta here!"])?;
                    ctx.next()?;
                    ctx.lines_as("Orc Hero", args!["If you wish to challenge me, wait inside the forest at the west. I'm not in the mood to deal with humans right now."])?;
                    ctx.next()?;
                    ctx.lines_as("Orc Hero", args!["Now, hurry", "up and scram!!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("orcs_hero_hat").get()? == 16 {
                    if (ctx.call(Function::CountItem, vec![Val::from(968)])? == 1
                        && (ctx.call(Function::CountItem, vec![Val::from(1124)])?.number()? > 0
                            || ctx.call(Function::IsEquipped, vec![Val::from(1124)])? == 1))
                    {
                        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])? == 1 {
                            ctx.lines_as(
                                "Orc Hero",
                                args!["Hm, are you the human who was granted status as an Orc Hero?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Orc Hero", args!["I hope you know the meaning of returning my Emblem. I expect that we will meet again on the battlefield."])?;
                            if ctx.call(Function::CountItem, vec![Val::from(2299)])?.number()? > 0 {
                                ctx.next()?;
                                ctx.lines_as("Orc Hero", args!["Wait...", "Isn't that an Orc Warrior's Helm...?"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Orc Hero",
                                    args![
                                        "Wait a second...",
                                        "It's not! Ha...!",
                                        "Interesting!",
                                        "That's very",
                                        "interesting."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Orc Hero",
                                    args!["I guess you don't know what's so special about this particular helm..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Orc Hero", args!["It actually belonged to one of our Orc Lords who was defeated by a human. He was so furious about losing that he broke the middle horn and threw it away!"])?;
                                ctx.next()?;
                                ctx.lines_as("Orc Hero", args!["I was told that it somehow ended in the hands of a human, but I didn't know it was you, the human Orc Hero."])?;
                                ctx.next()?;
                                ctx.lines_as("Orc Hero", args!["Alright, that belongs to the Orc Lord. I mean, even though it's now yours, it was given to you by mistake."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Orc Hero",
                                    args!["Would you mind giving the item back to its owner, my human Orc Hero?"],
                                )?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(ctx, &[Val::from("Yes, I mind.:No, I don't mind.")])?) == 1 {
                                    ctx.lines_as("Orc Hero", args!["Wow, you're so stubborn! Just treat the helm with care, and wear it with respect. Recognize that it has a long history..."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as("Orc Hero", args!["Good, that's a good decision. I will give this back to him. Muhahahahaha... I didn't expect to see this thing again."])?;
                                ctx.next()?;
                                ctx.lines_as("Orc Hero", args!["Okay...", "Let me give you", "something useful!"])?;
                                ctx.next()?;
                                ctx.lines_as("Orc Hero", args!["Although you're a human, you're an Orc Hero amongst us now, so you should have a helm suitable for your position..."])?;
                                ctx.next()?;
                                ctx.call(Function::DelItem, vec![Val::from(2299), Val::from(1)])?;
                                ctx.var("orcs_hero_hat").set(Val::from(17))?;
                                ctx.call(
                                    Function::GetNamedItem,
                                    vec![Val::from(5094), ctx.call(Function::StrCharInfo, vec![Val::from(0)])?],
                                )?;
                                ctx.lines_as("Orc Hero", args!["There you go. I marked a small indication on it. So wear this helm from now on. Do you understand? My human Orc Hero."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Orc Hero",
                            args!["Hm, are you the human who was granted status as an Orc Hero?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Orc Hero",
                            args![
                                " I hope you know the meaning of returning my Emblem. I expect that we will meet again on the battlefield."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Orc Hero",
                        args!["Hm, are you the human that was granted status as an Orc Hero?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Orc Hero",
                        args![
                            "Hmpf, I'm not convinced. Are you really the one who gave us our Emblems back? I don't think I can trust you..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Orc Hero", args!["Yeh, you would think I'd remember getting whupped by an ugly human, but I tend not to remember faces when I'm too busy getting whomped on."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Orc Hero",
                        args![
                            "I refuse to",
                            "acknowledge someone",
                            "like you who forgets",
                            "our most basic customs!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Orc Hero",
                        args![
                            "Grrrr...",
                            "Come on!",
                            "Let's meet outside west of the forest, and I'll test your strength again!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("orcs_hero_hat").get()? == 17 {
                    ctx.lines_as(
                        "Orc Hero",
                        args!["Muhahahaha~", "You're the", "strangest human", "I've ever met.", "Hahahahaha..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Orc Hero", args!["I like you,", "human, I like you."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Orc Hero", args!["...................."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn orc_hero_1(ctx: &Ctx) -> Script {
    orc_hero_1_run(ctx, OrcHero1Step::Start, Vec::new()).map(|_| ())
}

pub fn orc_hero_1_ontouch(ctx: &Ctx) -> Script {
    orc_hero_1_run(ctx, OrcHero1Step::OnTouch, Vec::new()).map(|_| ())
}
