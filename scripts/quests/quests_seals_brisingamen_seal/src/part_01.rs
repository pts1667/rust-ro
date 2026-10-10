use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn bard_brising_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("god_nelluad01"), Val::from(2)])?;
    if (runtime::op(&ctx.var("$god2").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true()
        && runtime::op(&ctx.var("$god3").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true())
    {
        if ctx.var("god_brising").get()?.number()? > 49 {
            ctx.lines_as(
                "Nelliorde",
                args!["Oh, I guess all is going well for you. So how has everything else been?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Nelliorde", args!["I may be being overly sensitive, but it feels like something might happen soon. Aaah, don't mind me. It's no big deal. Maybe I'm just being weird."])?;
            ctx.next()?;
            ctx.lines_as(
                "Nelliorde",
                args!["Right, right, let me play a song for you. Would you care to listen?"],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Sure, why not~:How about some news?:No thanks.")])? {
                1 => {
                    ctx.lines_as(
                        "Nelliorde",
                        args!["So, which song", "would you like to hear?", "Go ahead, pick one~"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "Bragi's Poem:Eternal Chaos:Assassin in the Sunset:Der Ring des Nibelungen",
                        )],
                    )? {
                        1 => {
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad02"), Val::from(2)])?;
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "Bragi's Poem!",
                                    "What a good choice!",
                                    "I must say this song",
                                    "represents the",
                                    "heart of a poet!"
                                ],
                            )?;
                            ctx.call(Function::SoundEffect, vec![Val::from("bragis_poem.wav"), Val::from(1)])?;
                            ctx.close_window()?;
                        }
                        2 => {
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad02"), Val::from(2)])?;
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "Eternal Chaos...",
                                    "Sounds too chaotic.",
                                    "By the way, I wonder",
                                    "what the title really",
                                    "means. Is it the chaos",
                                    "of hell or our own reality?"
                                ],
                            )?;
                            ctx.call(Function::SoundEffect, vec![Val::from("chaos_of_eternity.wav"), Val::from(1)])?;
                            ctx.close_window()?;
                        }
                        3 => {
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad02"), Val::from(2)])?;
                            ctx.lines_as("Nelliorde", args!["Assassin in the Sunset!", "Yes, no one looks cooler than an Assassin, the bringer of death, standing alone in the sunset! Heh heh~"])?;
                            ctx.call(Function::SoundEffect, vec![Val::from("assassin_of_sunset.wav"), Val::from(1)])?;
                            ctx.close_window()?;
                        }
                        4 => {
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad02"), Val::from(2)])?;
                            ctx.lines_as(
                                "Nelliorde",
                                args!["Der Ring des Nibelungen.....", "Okay, are you ready to listen?", "Hum hum hum...."],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::SoundEffect, vec![Val::from("ring_of_nibelungen.wav"), Val::from(1)])?;
                            ctx.lines(args![
                                "^4D4DFFRheingold...",
                                "Hidden in the Rhein river.",
                                "If made into a ring,",
                                "Could rule the world~^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^4D4DFFProtected by a spell",
                                "Cursing its thief",
                                "To never find love.",
                                "Alberich had known",
                                "Nonetheless stole it.",
                                "Love was forsaken for power.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^4D4DFFGiants built beautiful Valhalla",
                                "Matrimony with Freya",
                                "Goddess of beauty",
                                "Their supposed payment.",
                                "In not receiving it",
                                "They forcefully took her.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^4D4DFFThe gods would",
                                "give Alberich's treasure",
                                "To the Giants for Freya's return.",
                                "Loki tricked Alberich,",
                                "Stealing his ring or power.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^4D4DFFBut Alberich cursed his ring",
                                "Before it was returned to him,",
                                "Envy and death would",
                                "befall its wearers.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^4D4DFFThe ring was given to the giants",
                                "Freya was returned to the gods",
                                "The giants killed themselves",
                                "Fighting over the rheingold,",
                                "Victims of Alberich's curse~^000000"
                            ])?;
                            ctx.close_window()?;
                        }
                        _ => {}
                    }
                }
                2 => {
                    ctx.lines_as(
                        "Nelliorde",
                        args!["Some news you say..", "What kind of news do", "you wish to hear about?"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "About Prontera!:About coastal areas.:Is the desert still hot?:How about the borderlands?",
                        )],
                    )? {
                        1 => {
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "What?! Prontera?!",
                                    "Can you not see that the front",
                                    "gate of Prontera is right before you? You should at least know where you are, adventurer."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nelliorde",
                                args!["Anyways, I have one interesting news about Prontera. But, it's a secret. Shh! Are you... Single?"],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad04"), Val::from(2)])?;
                            ctx.lines_as("Nelliorde", args!["If so, go visit the tavern. I met a guy there who took to me a nice place. Oh yes. He took me to 'the place' where singles are forbidden~!"])?;
                            ctx.close_window()?;
                        }
                        2 => {
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "Coastal areas?",
                                    "That's a funny question. I mean, there's Alberta and a few other places where you can see the sea."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "To the southwest there's",
                                    "Kokomo Beach. But, I suppose it's too dangerous just to go there to take a stroll."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.mes("[Nelliorde]")?;
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad03"), Val::from(2)])?;
                            ctx.lines(args![
                                "Kokomo Beach is a large",
                                "expanse of sand. The nearby water is filled with sea animals, so you can enjoy catching Shellfishes."
                            ])?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["However, there are always bullies in that kind of place. Once, I met a threatening looking hoodlum with red spiky hair."])?;
                            ctx.next()?;
                            ctx.mes("[Nelliorde]")?;
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad04"), Val::from(2)])?;
                            ctx.mes("He was chewing on something and spat it on the ground. The big buffoon managed to intimidate me into giving him all of my Zeny!")?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "^333333*Sigh...*^000000",
                                    "This world is too",
                                    "dangerous for a young",
                                    "and fragile Bard."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["By the way, Alberta doesn't seem to have changed. The Sunken Ship looks the same as well, with all of its Skeleton Pirates and whatnot. You know how it is."])?;
                            ctx.next()?;
                            ctx.mes("[Nelliorde]")?;
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad01"), Val::from(2)])?;
                            ctx.mes("The event agency in charge of expeditions into the Sunken Ship is still making tons of money. If you have nothing to do, why don't you hunt for monsters down there?")?;
                            ctx.close_window()?;
                        }
                        3 => {
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "Oh, hell yes!",
                                    "When is it not?",
                                    "The desert is hot, has",
                                    "been hot and probably",
                                    "always will be...!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["Giant worms still jump out of nowhere, freaking out innocent travellers like myself, and the Sandmen still kick people in the, well... You know where."])?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["Even if the desert is filled with hideous monsters, the view of the ocean there is so magnificent. Hah, it makes me want to play a song."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nelliorde",
                                args!["I must say there is", "one place you may wish", "to see in the desert."],
                            )?;
                            ctx.next()?;
                            ctx.mes("[Nelliorde]")?;
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad02"), Val::from(2)])?;
                            ctx.mes(
                                "I'm talking about the Assassin Guild. Supposedly, it's hidden behind a sandstorm in the Morocc Desert!",
                            )?;
                            if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                                || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                            {
                                ctx.mes("But you should know what I am talking about! So when are you gonna take me to the guild, eh?")?;
                            } else {
                                ctx.mes("Have you ever been there before?")?;
                            }
                            ctx.close_window()?;
                        }
                        4 => {
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "Well, to the north is the Schwaltzwald Republic.",
                                    "There, you'll find Juno,",
                                    "the 'City of Sages.'"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.mes("[Nelliorde]")?;
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad03"), Val::from(2)])?;
                            ctx.lines(args![
                                "The people there say that there is one major nuisance, a man known as",
                                "the 'Mad Scientist.' He's antisocial and creates a lot of public disturbances with his experiments."
                            ])?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["It seems everyone in Juno", "hates him with a passion. Even one of the Tool Shop owners begged a passing adventurer to stop this Mad Scientist's research!"])?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["But I have a different opinion about him. I believe it's not a good idea to interrupt the work of any scientist."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "After all, he might just discover something important, even if his experiments are a little hazardous."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nelliorde",
                                args!["Speaking of discoveries, you know how Juno floats in the air, right?"],
                            )?;
                            ctx.next()?;
                            ctx.mes("[Nelliorde]")?;
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad01"), Val::from(2)])?;
                            ctx.lines(args![
                                "It's said that a piece of Ymir's Heart generates enough power",
                                "to keep the entire city aloft in the sky. Isn't that amazing?"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["Even more recent is the news about the discovery of Ymir's Book. You can read it in Juno's Sage Castle. I hear that it contains the secret to meeting Valkyrie..."])?;
                            ctx.close_window()?;
                        }
                        _ => {}
                    }
                }
                3 => {
                    ctx.lines_as(
                        "Nelliorde",
                        args!["Huh? Okay then, talk to you later. May Bragi bless you on your journeys."],
                    )?;
                    ctx.close_window()?;
                }
                _ => {}
            }
        } else if (ctx.var("god_brising").get()?.number()? > 0 && ctx.var("god_brising").get()?.number()? < 50) {
            ctx.lines_as(
                "Nelliorde",
                args![
                    "So, have you",
                    "met Mr. Kaili?",
                    "Please do your best.",
                    "After all, I especially",
                    "recommended you~"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Tell me a story.:Sing a song for me.")])? {
                1 => {
                    ctx.call(Function::Cutin, vec![Val::from("god_nelluad02"), Val::from(2)])?;
                    ctx.lines_as("Nelliorde", args!["Hmm. What would", "be a good story for", "you this time...? Ah yes, I've got it. Let me tell you about one of my own adventures. It was a pretty amazing experience."])?;
                    ctx.next()?;
                    ctx.lines_as("Nelliorde", args!["I went to Morocc, the city of the desert. On my way there, I became exhausted from the heat. Luckily, I found a hole beneath the shadows and climbed in to rest."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Nelliorde",
                        args![
                            "Oh my God...",
                            "If I had known",
                            "better, I would have",
                            "never gone inside that place..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Nelliorde",
                        args!["I actually crawled into the infamous ^FF0000Ant Hell^000000! I don't sound like a coward to you, do I?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Nelliorde", args!["Well, the ants aren't the only ones living there. I wasn't afraid of them at all. Except, of course, when they attacked in groups."])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("god_nelluad03"), Val::from(2)])?;
                    ctx.lines_as(
                        "Nelliorde",
                        args!["What I was really afraid of was this type of strawberry colored... tongue thing. Phreeoni!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Nelliorde",
                        args!["Fortunately, I survived the whole incident, although I still have nightmares of that man-sized tongue."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Nelliorde", args!["Once I got out of there, I craved human companionship to restore my sense of normalcy. The closest place that I knew was Paros Lighthouse."])?;
                    ctx.next()?;
                    ctx.lines_as("Nelliorde", args!["Yet again, I decided to traverse the desert, despite the fact that the Kafra Ladies provide a very convenient teleport service there."])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("god_nelluad01"), Val::from(2)])?;
                    ctx.lines_as("Nelliorde", args!["Anyway, the fortress of Sandaruman is on the way to Paros Lighthouse. I remember Serutero, a guy I met in south Morocc, telling me about its scenic beauty."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Nelliorde",
                        args![
                            "I was so excited",
                            "about seeing it",
                            "for myself. After all, doesn't 'Sandaruman' sound like the",
                            "name of a beautiful place?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Nelliorde", args!["But I immediately regretted my decision once I arrived. Of course, the view was breathtaking: the sandy hills near the ocean, as", "well as the ruins."])?;
                    ctx.next()?;
                    ctx.lines_as("Nelliorde", args!["And of course I forgot Serutero's warning about all the monsters. Argh! It was a nightmare! Instead of enjoying the scenary, I ended up running for my life."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Nelliorde",
                        args![
                            "I was still trying to escape those monsters when I stumbled upon",
                            "an old house, West of Sandaruman, that seemed to be in a state of decay."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Nelliorde", args!["I recalled that Serutero said that no one was living in that area, so I began to worry a bit. Still, I was desperate and needed refuge."])?;
                    ctx.next()?;
                    ctx.lines_as("Nelliorde", args!["In my panic, I violently knocked on the door and screamed for someone to let me inside. Then do you know what I heard?"])?;
                    ctx.next()?;
                    ctx.mes("[Nelliorde]")?;
                    ctx.call(Function::Cutin, vec![Val::from("god_nelluad03"), Val::from(2)])?;
                    ctx.lines(args![
                        "^FF0000Who's there?!",
                        "Who would dare to intrude",
                        "upon my territory?^000000",
                        "I was freaked out! Whoever owned that house must have been some",
                        "kind of lunatic!"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as("Nelliorde", args!["Th-then, with the specter of Death nipping at my heels, he told me to get lost! Luckily, I was saved by a mysterious, wandering SuperNovice, but that's another story."])?;
                    ctx.next()?;
                    ctx.lines_as("Nelliorde", args!["In any case, I finally arrived at Paros Lighthouse. There, I learned that the old house was a secret camp for the Rogue Guild. "])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Nelliorde",
                        args![
                            "It's amazing, how they can live in such dangerous areas. Only Rogues would be able to survive there, you know?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("[Nelliorde]")?;
                    ctx.call(Function::Cutin, vec![Val::from("god_nelluad01"), Val::from(2)])?;
                    ctx.lines(args![
                        "Ah...",
                        "I'm so tired from talking so much. But if I happen to have another adventure, I shall share that tale with you."
                    ])?;
                    ctx.close_window()?;
                }
                2 => {
                    ctx.call(Function::Cutin, vec![Val::from("god_nelluad02"), Val::from(2)])?;
                    ctx.lines_as(
                        "Nelliorde",
                        args!["Ah, you wish", "to hear a song?", "Okay! I shall then", "perform my favorite."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Nelliorde",
                        args![
                            "I chased after fame.",
                            "It eluded me.",
                            "I ran after happiness",
                            "But never caught it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Nelliorde",
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
                    ctx.call(Function::Cutin, vec![Val::from("god_nelluad04"), Val::from(2)])?;
                    ctx.lines_as(
                        "Nelliorde",
                        args![
                            "Cheer up! Life goes on.",
                            "As surely as the Water",
                            "Mill turns, tomorrow",
                            "Will come."
                        ],
                    )?;
                    ctx.close_window()?;
                }
                _ => {}
            }
        } else {
            ctx.lines_as(
                "Nelliorde",
                args!["Say...", "Have we met before?", "Hmm? Never? Well...", "That's rather odd."],
            )?;
            ctx.next()?;
            ctx.lines_as("Nelliorde", args!["I seem to recall that you've asked me to give you some information if I ever have any. Well, now I have and shall share it with you~"])?;
            ctx.next()?;
            ctx.lines_as("Nelliorde", args!["The usefulness of this information all depends on you. Now, have you heard of the ^3333CCMonster Research Organization^000000?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Nelliorde",
                args!["I happened to get this information from its headquarters in Juno. So how does that sound?"],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Wee~ I want to hear!:Booooring~")])? {
                1 => {
                    ctx.lines_as(
                        "Nelliorde",
                        args![
                            "Well...!",
                            "You seem to be",
                            "very excited about",
                            "this new tidbit of",
                            "knowledge I have~"
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.var("BaseLevel").get()?.number()? < 70 {
                        ctx.call(Function::Cutin, vec![Val::from("god_nelluad03"), Val::from(2)])?;
                        ctx.lines_as(
                            "Nelliorde",
                            args!["Alas, this information doesn't seem to be very valuable to someone such as yourself."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Nelliorde", args!["I'm so very sorry to have excited you, but only people that are strong enough to handle grueling work can benefit from this."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nelliorde",
                            args![
                                "Why don't you travel around and gain more experiences? I will let you know when you're ready, you know~"
                            ],
                        )?;
                        ctx.close_window()?;
                    } else {
                        ctx.call(Function::Cutin, vec![Val::from("god_nelluad02"), Val::from(2)])?;
                        ctx.lines_as(
                            "Nelliorde",
                            args![
                                "Ooh...",
                                "And you look",
                                "like you can handle",
                                "this kind of information.",
                                "I'll share everything I know!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Nelliorde", args!["One day, an adventurer discovered a strange, mysterious object. Unable to figure out what it could do, he visited the Monster Organization."])?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("god_nelluad01"), Val::from(2)])?;
                        ctx.lines_as("Nelliorde", args!["There, it was entrusted to a scholar by the name of Mr. Kaili. He still hasn't uncovered the truth about that item and wishes for assistance in his investigation."])?;
                        ctx.next()?;
                        ctx.lines_as("Nelliorde", args!["Mr. Kaili suspects that the item might be part of an undiscovered ancient relic. Fortunately, he is also researching ancient relics as a project assigned by the royal court."])?;
                        ctx.next()?;
                        ctx.lines_as("Nelliorde", args!["So what do you think? Isn't that interesting? Of course, you'd better speak to Mr. Kaili if you wish to learn more. If you'd like, I shall write you a letter or recommendation."])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Sure! Sounds good.:Sorry, I am not that interested.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Nelliorde",
                                    args![
                                        "Excellent...!",
                                        "So your name is...",
                                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?"))
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Nelliorde", args!["Alright, I shall contact him right away! Oh, and you can find the Monster Organization west of Juno's central plaza. It shouldn't be hard to find. Good luck!"])?;
                                ctx.var("god_brising").set(Val::from(1))?;
                                ctx.close_window()?;
                            }
                            2 => {
                                ctx.call(Function::Cutin, vec![Val::from("god_nelluad02"), Val::from(2)])?;
                                ctx.lines_as("Nelliorde", args!["Ah, it's disappointing to hear that. I thought that you'd be perfect to help out Mr. Kaili. Oh well, talk to you later~"])?;
                                ctx.close_window()?;
                            }
                            _ => {}
                        }
                    }
                }
                2 => {
                    ctx.lines_as(
                        "Nelliorde",
                        args![
                            "Boring, you say?",
                            "Perhaps, but not as",
                            "boring as an adventurer",
                            "that passes up a chance",
                            "for an adventure, yes?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Tell me a story.:Sing a song for me.")])? {
                        1 => {
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad02"), Val::from(2)])?;
                            ctx.lines_as("Nelliorde", args!["Hmm. What would", "be a good story for", "you this time...? Ah yes, I've got it. Let me tell you about one of my own adventures. It was a pretty amazing experience."])?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["I went to Morocc, the city of the desert. On my way there, I became exhausted from the heat. Luckily, I found a hole beneath the shadows and climbed in to rest."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "Oh my God...",
                                    "If I had known",
                                    "better, I never",
                                    "would have gone",
                                    "inside that place..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "I actually crawled into the infamous ^FF0000Ant Hell^000000! I don't sound like a coward to you, do I?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["Well, the ants aren't the only ones living there. I wasn't afraid of them at all. Except, of course, when they attacked in groups."])?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad03"), Val::from(2)])?;
                            ctx.lines_as(
                                "Nelliorde",
                                args!["What I was really afraid of was this type of strawberry colored... tongue thing. Phreeoni!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "Fortunately, I survived the whole incident, although I still have nightmares of that man-sized tongue."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["Once I got out of there, I craved human companionship to restore my sense of normalcy. The closest place that I knew was Paros Lighthouse."])?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["Yet again, I decided to traverse the desert, despite the fact that the Kafra Ladies provide a very convenient teleport service there."])?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad01"), Val::from(2)])?;
                            ctx.lines_as("Nelliorde", args!["Anyway, the fortress of Sandaruman is on the way to Paros Lighthouse. I remember Serutero, a guy I met in south Morocc, telling me about its scenic beauty."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "I was so excited",
                                    "about seeing it",
                                    "for myself. After all, doesn't 'Sandaruman' sound like the",
                                    "name of a beautiful place?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["But I immediately regretted my decision once I arrived. Of course, the view was breathtaking: the sandy hills near the ocean, as", "well as the ruins."])?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["And of course I forgot Serutero's warning about all the monsters. Argh! It was a nightmare! Instead of enjoying the scenary, I ended up running for my life."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "I was still trying to escape those monsters when I stumbled upon",
                                    "an old house, West of Sandaruman, that seemed to be in a state of decay."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["I recalled that Serutero said that no one was living in that area, so I began to worry a bit. Still, I was desperate and needed refuge."])?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["In my panic, I violently knocked on the door and screamed for someone to let me inside. Then do you know what I heard?"])?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad04"), Val::from(2)])?;
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "^FF0000Who's there?!",
                                    "Who would dare to intrude",
                                    "upon my territory?^000000",
                                    "I was freaked out! Whoever owned that house must have been some",
                                    "kind of lunatic!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["Th-then, with the specter of Death nipping at my heels, he told me to get lost! Luckily, I was saved by a mysterious, wandering SuperNovice, but that's another story."])?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["In any case, I finally arrived at Paros Lighthouse. There, I learned that the old house was a secret camp for the Rogue Guild. "])?;
                            ctx.next()?;
                            ctx.lines_as("Nelliorde", args!["It's amazing, how they can live in such dangerous areas. Only Rogues would be able to survive there, you know?"])?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad01"), Val::from(2)])?;
                            ctx.lines_as("Nelliorde", args!["Ah...", "I'm so tired from talking so much. But if I happen to have another adventure, I shall share that tale with you."])?;
                            ctx.close_window()?;
                        }
                        2 => {
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad02"), Val::from(2)])?;
                            ctx.lines_as(
                                "Nelliorde",
                                args!["Ah, you wish", "to hear a song?", "Okay! I shall then", "perform my favorite."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "I chased after fame.",
                                    "It eluded me.",
                                    "I ran after happiness",
                                    "But never caught it."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nelliorde",
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
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad04"), Val::from(2)])?;
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "Cheer up! Life goes on.",
                                    "As surely as the Water",
                                    "Mill turns, tomorrow",
                                    "Will come."
                                ],
                            )?;
                            ctx.close_window()?;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    } else {
        ctx.lines_as(
            "Nelliorde",
            args!["Hello there!", "Isn't it such", "a glorious day?", "You know of me,", "do you not?"],
        )?;
        ctx.next()?;
        ctx.mes("[Nelliorde]")?;
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
            ctx.lines(args![
                "Hahahaha, forgive my rudeness,",
                "but I fail to remember your name, although your face does seem rather familiar."
            ])?;
        } else {
            ctx.lines(args![
                "Oh well well...",
                "How could I ever",
                "forget the lovely",
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?"))
            ])?;
        }
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("god_nelluad01"), Val::from(2)])?;
        ctx.lines_as("Nelliorde", args!["So, aren't you curious about me? Feel free to ask me whatever you like! I'm the vagabond that will wander the earth till the end of his days, your friend who loves to sing under the moonlight~"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Who are you?:What's new?:Can you sing?")])? {
            1 => {
                ctx.lines_as(
                    "Nelliorde",
                    args!["Why, I am Nelliorde, the Bard.", "In truth, my real name is Elliorde."],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("god_nelluad03"), Val::from(2)])?;
                ctx.lines_as("Nelliorde", args!["However, while I was singing at a Tavern, as per usual, a man with long blonde hair angrily demanded that I change my name!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Nelliorde",
                    args!["Never before have I seen such a flailing and a fuss over someone coincidentally sharing the same name!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Nelliorde",
                    args!["And so, that little altercation", "was settled with me being named Nelliorde."],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("god_nelluad04"), Val::from(2)])?;
                ctx.lines_as("Nelliorde", args!["I don't sound like a coward to you, do I? Do you believe I should have fought for the right to keep my original name?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Nelliorde",
                    args![
                        "If so, you're wrong! I just don't believe that violence can solve anything, as the romantic Bard",
                        "that cherishes peace..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFSeeing as it's not worth",
                    "listening to Nelliorde's lies, you decide to simply tune him out.^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args!["^3355FF...", "......^000000"])?;
                ctx.next()?;
                ctx.lines(args!["^3355FF...", "......", ".........^000000"])?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("god_nelluad03"), Val::from(2)])?;
                ctx.lines_as(
                    "Nelliorde",
                    args![
                        "And that's how I convinced wise",
                        "and benevolent King Tristram III to build a paradise where married lovers could enjo--Hm? Are you even listening?!"
                    ],
                )?;
                ctx.close_window()?;
            }
            2 => {
                ctx.lines_as(
                    "Nelliorde",
                    args![
                        "You wish to",
                        "hear of some news?",
                        "What kind of news do",
                        "you wish to know about?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "About Prontera!:About seaside areas.:Is the desert still hot?:How about the borderlands?",
                    )],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Nelliorde",
                            args![
                                "What?! Prontera?!",
                                "Can you not see that the front",
                                "gate of Prontera is right before you? You should at least know where you are, adventurer."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nelliorde",
                            args!["Anyways, I know one interesting bit of news about Prontera. But it's a secret. Shh! Are you... Single?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Nelliorde", args!["If so, go visit the tavern. I met a guy there who took me a nice place. Oh yes. He took me the place where singles are forbidden~!"])?;
                        ctx.close_window()?;
                    }
                    2 => {
                        ctx.lines_as(
                            "Nelliorde",
                            args![
                                "Seaside areas?",
                                "That's a funny question. I mean, there's Alberta and a few other places where you can see the sea."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nelliorde",
                            args![
                                "To the southwest there's",
                                "Kokomo Beach. But, I suppose it's too dangerous just to go there to take a stroll."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nelliorde",
                            args![
                                "Kokomo Beach is a large",
                                "expanse of sand. The nearby water is filled with sea animals, so you can enjoy catching Shellfishes."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Nelliorde", args!["However, there are always bullies in that kind of place. Once, I met a threatening looking hoodlum with red spiky hair."])?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("god_nelluad04"), Val::from(2)])?;
                        ctx.lines_as("Nelliorde", args!["He was chewing on something and spat on the ground. The big buffoon managed to intimidate me into giving him all of my Zeny!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nelliorde",
                            args![
                                "^333333*Sigh...*^000000",
                                "This world is too",
                                "dangerous for a young",
                                "and fragile Bard."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Nelliorde", args!["By the way, Alberta doesn't seem to have changed. The Sunken Ship looks the same as well, with all of its Skeleton Pirates and whatnot. You know how it is."])?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("god_nelluad01"), Val::from(2)])?;
                        ctx.lines_as("Nelliorde", args!["The event agency in charge of expeditions into the Sunken Ship is still making tons of money. If you have nothing to do, why don't you hunt for monsters down there?"])?;
                        ctx.close_window()?;
                    }
                    3 => {
                        ctx.lines_as(
                            "Nelliorde",
                            args![
                                "Oh yes!",
                                "When is it not?",
                                "The desert is hot, has",
                                "been hot and probably",
                                "always will be...!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Nelliorde", args!["Giant worms still jump out of nowhere, freaking out innocent travellers like myself, and the Sandmen still kick people in the, well... You know where."])?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("god_nelluad02"), Val::from(2)])?;
                        ctx.lines_as("Nelliorde", args!["Even if the desert is filled with hideous monsters, the view of the ocean there is so magnificent. Hah, it makes me want to play a song."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nelliorde",
                            args!["I must say there is", "one place you may wish", "to see in the desert."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nelliorde",
                            args!["I'm talking about the Assassin Guild. Supposedly, it's hidden behind a sandstorm in the Morocc"],
                        )?;
                        if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                        {
                            ctx.mes(
                                "Desert! But you should know what I am talking about! So when are you gonna take me to the guild, eh?",
                            )?;
                        } else {
                            ctx.mes("Desert! Have you ever been there before?")?;
                        }
                        ctx.close_window()?;
                    }
                    4 => {
                        ctx.lines_as(
                            "Nelliorde",
                            args![
                                "Well, to the north is the Schwaltzwald Republic.",
                                "There, you'll find Juno,",
                                "the 'City of Sages.'"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nelliorde",
                            args![
                                "The people there say that there is one major nuisance, a man known",
                                "as the 'Mad Scientist.' He's antisocial and creates a lot of public disturbances with his experiments."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("god_nelluad03"), Val::from(2)])?;
                        ctx.lines_as("Nelliorde", args!["It seems everyone in Juno hates", "him with a passion. Even one of the Tool Shop owners begged a passing adventurer to stop this Mad Scientist's research!"])?;
                        ctx.next()?;
                        ctx.lines_as("Nelliorde", args!["But I have a different opinion about him. I believe it's not a good idea to interrupt the work of any scientist."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nelliorde",
                            args!["After all, he might just discover something important, even if his experiments are a little hazardous."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nelliorde",
                            args!["Speaking of discoveries, you know how Juno floats in the air, right?"],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("god_nelluad01"), Val::from(2)])?;
                        ctx.lines_as(
                            "Nelliorde",
                            args![
                                "It's said that a piece of Ymir's Heart generates enough power",
                                "to keep the entire city aloft in the sky. Isn't that amazing?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Nelliorde", args!["Even more recent is the news about the discovery of Ymir's Book. You can read it in Juno's Sage Castle. I hear that it contains the secret to meeting Valkyrie..."])?;
                        ctx.close_window()?;
                    }
                    _ => {}
                }
            }
            3 => {
                ctx.lines_as(
                    "Nelliorde",
                    args![
                        "Can I sing, you ask?",
                        "Certainly, and if I may say so, pleasureably. Which song would",
                        "you care to hear?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Bragi's Poem:Eternal Chaos:Assassin in the Sunset")])? {
                    1 => {
                        if ctx.var("Zeny").get()?.number()? > 499 {
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad02"), Val::from(2)])?;
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "Bragi's Poem!",
                                    "What a good choice!",
                                    "I must say this song",
                                    "represents the",
                                    "heart of a poet!"
                                ],
                            )?;
                            ctx.call(Function::SoundEffect, vec![Val::from("bragis_poem.wav"), Val::from(1)])?;
                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(500))?))?;
                            ctx.close_window()?;
                        } else {
                            ctx.lines_as("Nelliorde", args!["Please forgive me, but I cannot play a song for free. The world is harsh, and a frail Bard such as myself must have some means", "to survive~"])?;
                            ctx.close_window()?;
                        }
                    }
                    2 => {
                        if ctx.var("Zeny").get()?.number()? > 499 {
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad02"), Val::from(2)])?;
                            ctx.lines_as(
                                "Nelliorde",
                                args!["You must like robust", "and energetic music, eh?", "As you wish~"],
                            )?;
                            ctx.call(Function::SoundEffect, vec![Val::from("chaos_of_eternity.wav"), Val::from(1)])?;
                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(500))?))?;
                            ctx.close_window()?;
                        } else {
                            ctx.lines_as("Nelliorde", args!["Please forgive me, but I cannot play a song for free. The world is harsh, and a frail Bard such as myself must have some means", "to survive~"])?;
                            ctx.close_window()?;
                        }
                    }
                    3 => {
                        if ctx.var("Zeny").get()?.number()? > 499 {
                            ctx.call(Function::Cutin, vec![Val::from("god_nelluad02"), Val::from(2)])?;
                            ctx.lines_as(
                                "Nelliorde",
                                args![
                                    "Excellent choice!",
                                    "Somehow, the image of",
                                    "the setting sun fits",
                                    "well with Assassins.",
                                    "Don't you agree?"
                                ],
                            )?;
                            ctx.call(Function::SoundEffect, vec![Val::from("assassin_of_sunset.wav"), Val::from(1)])?;
                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(500))?))?;
                            ctx.close_window()?;
                        } else {
                            ctx.lines_as("Nelliorde", args!["Please forgive me, but I cannot play a song for free. The world is harsh, and a frail Bard such as myself must have some means", "to survive~"])?;
                            ctx.close_window()?;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn bard_brising(ctx: &Ctx) -> Script {
    bard_brising_body(ctx, Vec::new()).map(|_| ())
}

fn studying_scholar_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(200)])? == 0 {
        ctx.mes("^3355FFWait a second! Right now, you're carrying too many items with you. Please come back after putting some of your things into Kafra Storage.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("$god3").get()?.loosely_equals(&ctx.var("$@god_check2").get()?) {
        ctx.lines_as("Studying Scholar", args!["Hmmm...", "The highest quality Red Potion ever..."])?;
        ctx.next()?;
        ctx.lines_as("Studying Scholar", args!["If only I could figure a way to balance the consistency of the ground Red Herbs mixed with the water. That part is most crucial..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if runtime::op(&ctx.var("$god2").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true() {
        if ctx.var("god_brising").get()? == 50 {
            ctx.lines_as(
                "Enrico Kaili",
                args![
                    ((Val::from("Ah, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                    "you've returned."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Enrico Kaili", args!["Learning about Brisingamen was truly fascinating. But since we've hit a plateau for now, I've started a new research project."])?;
            ctx.next()?;
            ctx.lines_as("Enrico Kaili", args!["Hahahaha~", "I'm sorry, but I cannot tell you about it for now, since it's confidential. But once I get a chance, I'll ask you for help."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("god_brising").get()? == 49 {
                ctx.lines_as(
                    "Enrico Kaili",
                    args![
                        "Ah, I see. So it's true that Berling sealed himself in the ripple where the goddess' tear",
                        "fell into the waters."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Enrico Kaili",
                    args!["Oh! I'm so excited! I don't know what to do first! Should I report to His Majesty or publish my findings or..."],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("^333333*Ahem!*^000000 My reward!")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as("Enrico Kaili", args!["Oh, right.", "Thank you for all the trouble you have endured for my sake. You are truly one of the best researchers I've ever met."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Enrico Kaili",
                    args![
                        "Then as promised...",
                        "I will give you something from my precious collection! Now, let me see what we have here..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Enrico Kaili", args!["Please...", "Take this."])?;
                ctx.var("god_brising").set(Val::from(50))?;
                ctx.call(Function::GetItem, vec![Val::from(616), Val::from(1)])?;
                ctx.call(Function::GetExperience, vec![Val::from(600000), Val::from(0)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Enrico Kaili",
                    args![
                        "Please accept this as my way of thanking you for assisting in my",
                        "research. If the opportunity arises, I would like to ask you for your help once again. Now please, take care."
                    ],
                )?;
                if runtime::op(&ctx.var("$god3").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true() {
                    ctx.var("$god3").set((ctx.var("$god3").get()? + Val::from(1)))?;
                }
                if ctx.var("$god3").get()?.loosely_equals(&ctx.var("$@god_check1").get()?) {
                    ctx.call(
                        Function::Announce,
                        vec![Val::from("The 3rd Seal of [Brisingamen] has appeared."), ctx.constant("BC_ALL")?],
                    )?;
                } else if ctx.var("$god3").get()?.loosely_equals(&ctx.var("$@god_check2").get()?) {
                    if (((ctx.var("$god1").get()?.loosely_equals(&ctx.var("$@god_check2").get()?)
                        && ctx.var("$god2").get()?.loosely_equals(&ctx.var("$@god_check2").get()?))
                        && ctx.var("$god3").get()?.loosely_equals(&ctx.var("$@god_check2").get()?))
                        && ctx.var("$god4").get()?.loosely_equals(&ctx.var("$@god_check2").get()?))
                    {
                        ctx.call(
                            Function::Announce,
                            vec![
                                Val::from("Four seals have been released at the same time with the seal of [Brisingamen]."),
                                ctx.constant("BC_ALL")?,
                            ],
                        )?;
                    } else {
                        ctx.call(
                            Function::Announce,
                            vec![
                                Val::from("The 3rd seal of [Brisingamen] has been released."),
                                ctx.constant("BC_ALL")?,
                            ],
                        )?;
                    }
                }
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("god_brising").get()? == 48 {
                    ctx.lines_as(
                        "Enrico Kaili",
                        args!["Why are you still here?", "I asked you to go find Alfrik.", "Ah~ I cannot wait..."],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SPARK")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("god_brising").get()? == 47 {
                        ctx.lines_as(
                            "Enrico Kaili",
                            args![
                                "I guess it's true that",
                                "the four Dwarves have been",
                                "awakened. Does this mean that Brisingamen can now be created?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Enrico Kaili", args!["If such a thing is possible, what do we need in order to create it? Let's review the information you've gathered..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Enrico Kaili",
                            args![
                                "There's Freya's Tear Drop Crystal, Freya's Jewel, the Never-Melting Snow Crystal found where Alfrik",
                                "used to stay, the Drifting Air from the path where Dvalin slept. Hmm."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Enrico Kaili",
                            args![
                                "Perhaps we could find something from the pond of Freya's Gold Tears where Berling is sleeping,",
                                "though I am unsure."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Enrico Kaili",
                            args![
                                "I also assume that the splendid Silver Ornament constructed by",
                                "Gher is also needed to create Brisingamen."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Enrico Kaili", args!["So what", "do you think?"])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("I'm not sure...:I suppose you're right.")])? {
                            1 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["I'm not sure...", "I mean, maybe", "Alfrik would", "probably know, bu--"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Enrico Kaili", args!["Of course!", "That's a great idea! I'll wait here while you go and ask Alfrik. You're my only hope in completing this research!"])?;
                                ctx.next()?;
                                ctx.call(
                                    Function::Emotion,
                                    vec![
                                        ctx.constant("ET_SCRATCH")?,
                                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                    ],
                                )?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["(^333333I shouldn't have said anything...^000000)"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Enrico Kaili",
                                    args!["Well, what are", "waiting for? Please", "come back as soon", "as possible."],
                                )?;
                                ctx.var("god_brising").set(Val::from(48))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["I suppose", "you're right."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Still, who knows what exactly is in the Pond of Freya's Golden Tears."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Enrico Kaili",
                                    args!["Yes.", "Ah! Right.", "Hm, perhaps it would be", "best to ask Alfrik first?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Enrico Kaili",
                                    args!["Hurry and seek him", "out! And I thank you", "in advance."],
                                )?;
                                ctx.var("god_brising").set(Val::from(48))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else {
                        if ctx.var("god_brising").get()? == 46 {
                            ctx.lines_as("Enrico Kaili", args!["I guess he finally sang for you?", "By the way, why didn't you speak to Grer the Dwarf? Oh right, the mine is heavily populated with monsters."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Enrico Kaili",
                                args![
                                    "I know it won't be easy,",
                                    "but please seek him out",
                                    "and get more information.",
                                    "Thank you in advance."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("god_brising").get()? == 45 {
                                ctx.lines_as("Enrico Kaili", args!["Hm...?", "Did you forget the words to the song? Well, I didn't write them down either. Hahahaha! I guess there's only one thing you can do~"])?;
                                ctx.next()?;
                                ctx.lines_as("Enrico Kaili", args!["You've got to go back to Berling and ask him for the lyrics again. Good luck, and don't forget to memorize them this time."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("god_brising").get()? == 44 {
                                    ctx.lines_as(
                                        "Enrico Kaili",
                                        args!["It looks like you were able to answer that riddle. Good work!"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Enrico Kaili", args!["That strange, unearthly feeling of dampness... Ah, you've brought Freya's Tear Drops! That means Grer must have awakened on his own now."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Enrico Kaili", args!["I suppose your next course of action should be to enter the mine. I'm sorry for the trouble, but I grow more excited with each new thing we learn."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Enrico Kaili", args!["It's only with your help that I've been able to make such progress with my research. Anyway, I shall share my conclusions with you later."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Enrico Kaili",
                                        args!["So are you heading for the mine now? Be careful around those monsters."],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("god_brising").get()? == 43 {
                                        ctx.lines_as(
                                            "Enrico Kaili",
                                            args![
                                                "Dvalin's riddle?",
                                                "He must have been",
                                                "talking about the",
                                                "endless war that",
                                                "Freya had to cause."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Enrico Kaili", args!["There were two kings that", "each had twenty knights. The kings and these knights fought and killed each other, and would be brought back to life the next morning by Freya until thousands of Valkyries filled Valhalla."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Enrico Kaili",
                                            args![
                                                "All forty-two",
                                                "of them did this...",
                                                "Everyday. It's an old",
                                                "story that everyone knows."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Enrico Kaili",
                                            args![
                                                "In any case, it's about time that you investigated Berling. There's",
                                                "a stream that begins northwest of Prontera and passes through a mine in Mount Mjolnir."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Enrico Kaili",
                                            args![
                                                "It may be wise to",
                                                "start your search there and investigate any streams and",
                                                "puddles that you can find."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.var("god_brising").get()? == 42 {
                                            ctx.lines_as(
                                                "Enrico Kaili",
                                                args![
                                                    "Are you sure?!",
                                                    "Those items were",
                                                    "intended to create",
                                                    "Freya's necklace,",
                                                    "Brisingamen?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Enrico Kaili",
                                                args![
                                                    "If the four Dwarves",
                                                    "awaken, Brisingamen may",
                                                    "appear before human eyes?",
                                                    "Fascinating. Just fascinating."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Enrico Kaili",
                                                args![
                                                    "Let's see....",
                                                    "The place where",
                                                    "Dvalin is sleeping?",
                                                    "I assume it may be near",
                                                    "the place where the Monks",
                                                    "train, but I am unsure."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Enrico Kaili",
                                                args![
                                                    "In any case, it would",
                                                    "to no harm to investigate",
                                                    "any leads yes? The monastery,",
                                                    "I believe, is to the east",
                                                    "of Prontera."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.var("god_brising").get()? == 41 {
                                                ctx.lines_as(
                                                    "Enrico Kaili",
                                                    args![
                                                        "What...?",
                                                        "You actually",
                                                        "met Alfrik...?",
                                                        "I can't believe",
                                                        "the legendary Alfrick",
                                                        "is still alive..."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Enrico Kaili", args!["Alfrik is one of the four Dwarves who created Brisingamen, the world's most beautiful necklace, for the goddess Freya."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Enrico Kaili", args!["Now I realize...", "We've been gathering information related to an item containing the power of the gods!"])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Enrico Kaili",
                                                    args![
                                                        "Let's see...",
                                                        "There's another Dwarf, Dvalin.",
                                                        "Did Alfrik tell you anything about where Dvalin may be sleeping? Hmm..."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                if ctx.var("god_brising").get()? == 40 {
                                                    ctx.lines_as(
                                                        "Enrico Kaili",
                                                        args!["For now, I suppose", "it would be best to", "investigate Lutie."],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Enrico Kaili", args!["Somewhere there, I'm", "sure you can find a remaining trace of the goddess Freya. If you were able to meet Valkyrie, I'm sure you'll be able to do this."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Enrico Kaili",
                                                        args![
                                                            "I've been thinking...",
                                                            "About those words",
                                                            "Valkyrie told you",
                                                            "to remember..."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Enrico Kaili",
                                                        args![
                                                            "^4d4dffThe beauty of the stars",
                                                            "Wanes in comparison.",
                                                            "We lost our hearts",
                                                            "To that golden hair",
                                                            "And those dazzling eyes.^000000"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Enrico Kaili",
                                                        args!["What could that mean?", "I'm beginning to doubt", "what I think I know..."],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    if ctx.var("god_brising").get()? == 35 {
                                                        ctx.lines_as(
                                                            "Enrico Kaili",
                                                            args![
                                                                "I can tell you've",
                                                                "gone through a lot...",
                                                                "So what did you hear",
                                                                "from Hermite?"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines(args![
                                                            "^3355FFYou tell Enrico Kaili",
                                                            "what you were told by Hermite,",
                                                            "as well as your experiences",
                                                            "with Valkyrie.^000000"
                                                        ])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Enrico Kaili", args!["I can scarcely believe this!", "But if what you say is true, it may be possible for humans to possess godly power."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Enrico Kaili", args!["Hm? There's a reaction coming", "from this thing that was sent from the royal court. Hm, come closer and take a look."])?;
                                                        ctx.next()?;
                                                        ctx.mes("^3355FFEnrico opened a locked chest, revealing an incredibly clear, shimmering jewel.^000000")?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Enrico Kaili", args!["Isn't it amazing?", "If what Valkyrie said is true, this jewel must be a crystal from Freya's teardrop."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Enrico Kaili", args!["I don't understand why all of this is happening so suddenly. For now, I'll have to think about the poem you heard from Valkyrie."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Enrico Kaili",
                                                            args![
                                                                "A cold snowy path on",
                                                                "which the goddess tread.",
                                                                "A blue river the goddess crossed.",
                                                                "A narrow path the goddess passed.",
                                                                "Four Dwarves that the goddess met..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Enrico Kaili", args!["I'm sure the four Dwarves are the famous craftsmen I've studied, but I can't determine what the rest of the poem means."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Enrico Kaili",
                                                            args!["Now...", "I'll need your", "help once again!"],
                                                        )?;
                                                        ctx.next()?;
                                                        match runtime::select_values(
                                                            ctx,
                                                            &[Val::from("Wah? It's not over?!:What is it this time?")],
                                                        )? {
                                                            1 => {
                                                                ctx.lines_as(
                                                                    "Enrico Kaili",
                                                                    args![
                                                                        "Hah...!",
                                                                        "Our work has just begun!",
                                                                        "Now, head over to Lutie",
                                                                        "and find some clues."
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Enrico Kaili", args!["The poem mentions a snowy path, and this jewel was found there, so Lutie is a pretty well educated guess as the location of our next clue."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Enrico Kaili",
                                                                    args![
                                                                        "I hope we'll be",
                                                                        "able to learn what",
                                                                        "the goddess Freya has",
                                                                        "left behind in Midgard..."
                                                                    ],
                                                                )?;
                                                                ctx.var("god_brising").set(Val::from(40))?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            }
                                                            2 => {
                                                                ctx.lines_as(
                                                                    "Enrico Kaili",
                                                                    args![
                                                                        "Hahahaha...!",
                                                                        "I like your attitute!",
                                                                        "Once again, Nelliorde has",
                                                                        "proven to be an excellent",
                                                                        "judge of character."
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Enrico Kaili", args!["The poem mentions", "a snowy path, and this jewel was found in Lutie. So Lutie would be a pretty good place to search for the next clue."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Enrico Kaili",
                                                                    args![
                                                                        "I hope we'll be",
                                                                        "able to learn what",
                                                                        "the goddess Freya has",
                                                                        "left behind in Midgard..."
                                                                    ],
                                                                )?;
                                                                ctx.var("god_brising").set(Val::from(40))?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            }
                                                            _ => {}
                                                        }
                                                    } else {
                                                        if (ctx.var("god_brising").get()?.number()? > 9
                                                            && ctx.var("god_brising").get()?.number()? < 35)
                                                        {
                                                            ctx.lines_as(
                                                                "Enrico Kaili",
                                                                args![
                                                                    "Hmm...?",
                                                                    "I see...",
                                                                    "I know the work",
                                                                    "I'm having you do might",
                                                                    "be quite troublesome..."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Enrico Kaili",
                                                                args![
                                                                    "But rest assured,",
                                                                    "I will reward you",
                                                                    "once our work is completed.",
                                                                    "Good help such as the kind",
                                                                    "you provide is hard to find."
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else if (ctx.var("god_brising").get()?.number()? > 3
                                                            && ctx.var("god_brising").get()?.number()? < 10)
                                                        {
                                                            ctx.lines_as(
                                                                "Enrico Kaili",
                                                                args![
                                                                    "So have you",
                                                                    "met Hermite? Hm?",
                                                                    "He actually asked",
                                                                    "you to do that?"
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Enrico Kaili", args!["For now, it looks like you might have no alternative. It would probably be wise to do what he wants. Knowing Charles, it's probably won't be too bad."])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else if ctx.var("god_brising").get()? == 3 {
                                                            ctx.lines_as(
                                                                "Enrico Kaili",
                                                                args![
                                                                    "Hermite...?",
                                                                    "Do you need more",
                                                                    "information about him?",
                                                                    "Let's see..."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Enrico Kaili", args!["Oh, that's right!", "He's a registered member of that Monster Research Organization. Why don't you ask the lady over there for more information on him?"])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Enrico Kaili", args!["If she'll let you look at her records, you can probably find out where he lives. Now why don't you talk to that lady towards the left part of this room?"])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else if ctx.var("god_brising").get()? == 2 {
                                                            ctx.lines_as(
                                                                "Enrico Kaili",
                                                                args![
                                                                    "So, have you",
                                                                    "considered my proposal?",
                                                                    "Work for me, and I shall",
                                                                    "surely repay you."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            match runtime::select_values(ctx, &[Val::from("No thanks.:Sure, why not.")])? {
                                                                1 => {
                                                                    ctx.lines_as(
                                                                        "Enrico Kaili",
                                                                        args![
                                                                            "What...?!",
                                                                            "Then why bother",
                                                                            "to speak to me?",
                                                                            "If you just intend to",
                                                                            "bother me, then please",
                                                                            "leave right away."
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                                2 => {
                                                                    ctx.lines_as("Enrico Kaili", args!["Really?", "If you could help me, that would be great! Before we start, would you take a look at this?"])?;
                                                                    ctx.next()?;
                                                                    ctx.lines(args!["^3355FFEnrico Kaili showed you a small white crystallization inside a box. It looked very fragile, but emanated an intensely", "cold aura.^000000"])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Enrico Kaili",
                                                                        args![
                                                                            "I call this the Snow Crystal.",
                                                                            "It looks just like a snowflake, doesn't it? Strangely enough,",
                                                                            "it's never melted."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Enrico Kaili", args!["A few years ago,", "a young adventurer", "gave this to me. I need to find this person so that I can find out exactly what this is, as well as where it came from."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Enrico Kaili",
                                                                        args![
                                                                            "Now, the person",
                                                                            "who brought this to",
                                                                            "me is, let's see, ah...",
                                                                            "Hermite Charles."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Enrico Kaili",
                                                                        args!["When you find him,", "please give him this letter."],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines(args![
                                                                        "^3355FFEnrico gave you a small",
                                                                        "letter addressed to Hermite",
                                                                        "Charles that is sealed with",
                                                                        "red wax.^000000"
                                                                    ])?;
                                                                    ctx.var("god_brising").set(Val::from(3))?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                                _ => {}
                                                            }
                                                        } else if ctx.var("god_brising").get()? == 1 {
                                                            ctx.lines_as(
                                                                "Enrico Kaili",
                                                                args![
                                                                    "Ah, you've arrived!",
                                                                    ((Val::from("")
                                                                        + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                        + Val::from(", right?")),
                                                                    "Yes, I was told by Nelliorde that you'd come. He always manages",
                                                                    "to find me good, reliable help."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Enrico Kaili", args!["As Nelliorde probably mentioned,", "I need some help in completing my research. Since you adventurers are always traveling, I was hoping you'd help me find someone."])?;
                                                            ctx.next()?;
                                                            match runtime::select_values(
                                                                ctx,
                                                                &[Val::from("I'm no good at finding people.:I can do that!")],
                                                            )? {
                                                                1 => {
                                                                    ctx.lines_as("Enrico Kaili", args!["Oh... Really?", "I was really hoping that you'd be able to help me. But I understand if it's not within your capacity."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Enrico Kaili", args!["I suppose I can try to find another adventurer to help me. But if you change your mind later, please do not hesitate to lend me your assistance."])?;
                                                                    ctx.var("god_brising").set(Val::from(2))?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                                2 => {
                                                                    ctx.lines_as("Enrico Kaili", args!["Really?", "If you could help me, that would be great! Before we start, would you take a look at this?"])?;
                                                                    ctx.next()?;
                                                                    ctx.lines(args!["^3355FFEnrico Kaili showed you a small white crystallization inside a box. It looked very fragile, but emanated an intensely", "cold aura.^000000"])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Enrico Kaili",
                                                                        args![
                                                                            "I call this the Snow Crystal.",
                                                                            "It looks just like a snowflake, doesn't it? Strangely enough,",
                                                                            "it's never melted."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Enrico Kaili", args!["A few years ago,", "a young adventurer", "gave this to me. I need to find this person so that I can find out exactly what this is, as well as where it came from."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Enrico Kaili",
                                                                        args![
                                                                            "Now, the person",
                                                                            "who brought this to",
                                                                            "me is, let's see, ah...",
                                                                            "Hermite Charles."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Enrico Kaili",
                                                                        args!["When you find him,", "please give him this letter."],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines(args![
                                                                        "^3355FFEnrico gave you a small",
                                                                        "letter addressed to Hermite",
                                                                        "Charles that is sealed with",
                                                                        "red wax.^000000"
                                                                    ])?;
                                                                    ctx.var("god_brising").set(Val::from(3))?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                                _ => {}
                                                            }
                                                        } else {
                                                            ctx.lines_as("Enrico Kaili", args!["I'm not exactly sure why you've come to me, but I apologize for the fact that I'm unable to help you."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Enrico Kaili",
                                                                args![
                                                                    "Right now, I'm far",
                                                                    "too busy trying to",
                                                                    "complete my research.",
                                                                    "Please leave me alone",
                                                                    "to do my work."
                                                                ],
                                                            )?;
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
    } else {
        ctx.lines_as(
            "Studying Scholar",
            args![
                "Do you know how",
                "to make Red Potions?",
                "You grind a lot of Red Herbs and carefully shake the liquid until you achieve the desire consistency."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Studying Scholar",
            args![
                "If I could find a way",
                "to produce them much faster,",
                "I'd become rich in no time..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn studying_scholar_1(ctx: &Ctx) -> Script {
    studying_scholar_1_body(ctx, Vec::new()).map(|_| ())
}
