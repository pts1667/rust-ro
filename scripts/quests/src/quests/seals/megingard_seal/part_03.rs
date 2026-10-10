use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn egnigem_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (runtime::op(&ctx.var("$god1").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true()
        && runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true())
    {
        if (ctx.var("god_eremes").get()?.number()? > 19 && ctx.var("god_eremes").get()?.number()? < 25) {
            if ctx.var("god_eremes").get()?.number()? > 22 {
                ctx.lines_as("Egnigem", args!["Why have you returned to", "this lonely place of darkness?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Egnigem",
                    args![
                        "Please go back to the world",
                        "of the living. To be here is to suffer, such is the nature of Niflheim."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Egnigem",
                    args![
                        "Greetings...",
                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                        "I've been watching you in the",
                        "world of the living. I see you've been busy looking for the truth about the 1st Squad, haven't you?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Egnigem", args!["I can provide you with the answers. After all, I'm the only member of the 1st Squad to retain all of my memories..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Egnigem",
                    args![
                        "Heh heh...",
                        "Let me tell you everything",
                        "about the 1st Squad that's",
                        "been forgotten..."
                    ],
                )?;
                'l1: loop {
                    if !(true) {
                        break 'l1;
                    }
                    'b1: {
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from(
                                "The Mission:Fate of the 1st Squad:Why are you in Niflheim?:Thanks, I've heard enough.",
                            )],
                        )? {
                            1 => {
                                ctx.lines_as(
                                    "Egnigem",
                                    args![
                                        "First of all, the official record of our mission is misleading.",
                                        "It contains some truth, based",
                                        "on verbal evidence, but the most important details have been left out or changed."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["Now, we were secretly assigned", "to find ^0000FFgodly artifacts^000000 by royal decree. In only three days, we succeeded and found Megingjard!"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Egnigem",
                                    args!["Now, the records say the mission was unsatisfactory, correct? That would be because of me."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["Are you aware that Megingjard", "was a belt worn by gods? It was actually one of the keys to Thor's great strength. Another name for it is the 'Girdle of Might.'"])?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["Now, if a human were to wear", "that, there's a possibility that Megingjard would give him strength comparable to a god. Naturally, you'd be afraid of such a thing falling into the wrong hands."])?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["In Rebarev Doug's mind, he", "was the only person worthy of the belt since he considered himself the most religious person alive, and thus, closest to the gods."])?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["He made it clear to us that only he would wear Megingjard. Although he was our leader, some of us felt uneasy about him wearing the belt and not giving it to his superiors, our mission objective."])?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["Naturally, I opposed him. He was clearly unbalanced, and it would be dangerous to give him control of such power. It led to something of a fist fight between us."])?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["The rest of the squad couldn't", "agree on whether or not we should simply obey our leader and let him have the belt. Some just wanted to complete the mission, while others were afraid of what might happen."])?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["But Rebarev didn't wait for us to decide. He destroyed Megingjard into pieces, intending to keep most of it and somehow reconstruct it later, and only bring one piece to the Prontera Military."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Egnigem",
                                    args![
                                        "He knew that he couldn't get",
                                        "away with keeping all of Megingjard to himself, as the entire squad could easily discredit him."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["But Rebarev knew", "that he could deceive his", "superiors by giving them just a fragment of the artifact, and say that the rest of it was destroyed."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Egnigem",
                                    args![
                                        "I discovered his intention when",
                                        "I found him taking Megingjard apart. Before I could stop him,",
                                        "I was killed. That bastard had",
                                        "a trap ready for me..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["After the mission, Rebarev Doug only gave a piece of the belt back to the higher-ups, and gave them", "a different account than what had actually happened..."])?;
                            }
                            2 => {
                                ctx.lines_as("Egnigem", args!["Once the mission was completed, Rebarev Doug reported back to the Prontera Military with the intent of keeping most of Megingjard for himself."])?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["For some reason, he has a very strong influence in the military hierarchy. Now, by this time, I'm sure the squad pretty much figured out that he killed me."])?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["But Rebarev Doug expected this, and explained to his superiors that the squad was tempted by Megingjard's power and that I led some of us in a mutiny against him."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Egnigem",
                                    args![
                                        "He told his superiors that Megingjard was damaged as we",
                                        "fought amongst each other, and that the fragment he gave them was all that remained."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["Because of the confidential nature of our mission, the 1st Squad was disbanded and we weren't given the story Rebarev fed the beaureaucrats until it was too late."])?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["The squad was punished and", "sent for disciplinary retraining for three months. During this time, specific memories were unnaturally erased from the squad members."])?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["In the end, everyone was transferred to other squads and positions. My name had to be removed from all the records since hearing or reading my name might undo the brain washing."])?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["However, they still needed an official scapegoat, so they used Royal Myst's name in the records. That's probably why he's been going through hard times right now."])?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["And now Rebarev Doug has been studying godly artifacts based on the pieces of Megingjard that we found on that mission using kingdom research funds. Someday, he might be able to reconstruct the belt."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Egnigem",
                                    args!["There are also", "a few questions that", "I've never been able", "to answer."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["I've never known why the King Tristram III ordered the research of godly artifacts. It also bothers me that Rebarev Doug is in charge of Megingjard's research. There must be some reason."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Egnigem",
                                    args!["In any case, for the sake of Midgard, it will be your responsibility to find the answers."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["There's not much a dead man", "can do. But what's worse is that no one remembers me any more. I died alone and I'm still lonely in this place for the dead."])?;
                            }
                            3 => {
                                ctx.lines_as(
                                    "Egnigem",
                                    args![
                                        "The worst part of the squad",
                                        "having their memories erased is",
                                        "that Emma Searth has absolutely",
                                        "no recollection of me."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Egnigem",
                                    args![
                                        "Do you mind listening",
                                        "to me for a while? You're",
                                        "the only person with whom",
                                        "I can talk about this..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Egnigem",
                                    args![
                                        "From here, I've seen that Emma",
                                        "is chasing her dream again, and",
                                        "I'm happy for her. But it's still",
                                        "not easy, knowing the woman you",
                                        "love has totally forgotten you."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["I remember the first time I met her. I guess it was love at first sight. But I was too cowardly to confess my love, so I despaired when she joined the Crusaders."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Egnigem",
                                    args![
                                        "It took me a while, but I made up my mind to enlist as a Crusader",
                                        "as well. I wanted to be closer to her so much."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Egnigem",
                                    args!["Now, after all our adventures and good times together, I'm in the worst possible scenario."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Egnigem",
                                    args![
                                        "I'm dead...",
                                        "But I don't even have the consolation that she is",
                                        "mourning my loss."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Egnigem",
                                    args![
                                        "I suppose that's why I'm here",
                                        "in Niflheim. I can't quite move on to heaven, Valhalla or anywhere else without her."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Egnigem", args!["Anyway, go and visit the Crusader Headquarters. You might be able to learn more about Rebarev Doug and stop any new shadiness he might be plotting. Good luck, my friend."])?;
                            }
                            4 => {
                                ctx.lines_as("Egnigem", args!["Thank you for", "listening to me..."])?;
                                if ctx.var("god_eremes").get()? == 21 {
                                    ctx.var("god_eremes").set(Val::from(23))?;
                                } else if ctx.var("god_eremes").get()? == 22 {
                                    ctx.var("god_eremes").set(Val::from(24))?;
                                }
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                }
            }
        } else {
            if ctx.var("god_eremes").get()? == 25 {
                ctx.lines_as(
                    "Egnigem",
                    args![
                        "As a Crusader...",
                        "This is the best thing",
                        "I can do in return for",
                        "what you have done for me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Egnigem",
                    args![
                        "Listen...",
                        "I'm going to",
                        "imbue you with the",
                        "^666666last vestiges of my strength^000000..."
                    ],
                )?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL5")?])?;
                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HOLYHIT")?])?;
                ctx.next()?;
                ctx.lines_as(
                    "Egnigem",
                    args![
                        "May the heavens",
                        "answer me as I call upon the",
                        "light of justice that empowers the defenders of truth, and threaten the enemies of peace."
                    ],
                )?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL6")?])?;
                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HOLYHIT")?])?;
                ctx.next()?;
                ctx.lines_as(
                    "Egnigem",
                    args![
                        "May you always shine with",
                        "the light of truth. Let the light within you eradicate the shadows",
                        "of deception, fear and malice."
                    ],
                )?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL6")?])?;
                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HOLYHIT")?])?;
                ctx.next()?;
                ctx.lines_as("Egnigem", args!["By the holy power", "invested in me, I humbly bestow upon you my remaining strength. Fight honorably, and do not lose sight of righteousness."])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LOCKON")?])?;
                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HOLYCROSS")?])?;
                ctx.var("god_eremes").set(Val::from(27))?;
                {
                    if ctx.var("BaseLevel").get()?.number()? < 56 {
                        ctx.call(Function::GetExperience, vec![Val::from(27000), Val::from(0)])?;
                    } else {
                        if (ctx.var("BaseLevel").get()?.number()? > 55 && ctx.var("BaseLevel").get()?.number()? < 61) {
                            ctx.call(Function::GetExperience, vec![Val::from(30000), Val::from(0)])?;
                        } else {
                            if (ctx.var("BaseLevel").get()?.number()? > 60 && ctx.var("BaseLevel").get()?.number()? < 66) {
                                ctx.call(Function::GetExperience, vec![Val::from(56052), Val::from(0)])?;
                            } else {
                                if (ctx.var("BaseLevel").get()?.number()? > 65 && ctx.var("BaseLevel").get()?.number()? < 71) {
                                    ctx.call(Function::GetExperience, vec![Val::from(82233), Val::from(0)])?;
                                } else if (ctx.var("BaseLevel").get()?.number()? > 70 && ctx.var("BaseLevel").get()?.number()? < 76) {
                                    ctx.call(Function::GetExperience, vec![Val::from(212271), Val::from(0)])?;
                                } else if (ctx.var("BaseLevel").get()?.number()? > 75 && ctx.var("BaseLevel").get()?.number()? < 81) {
                                    ctx.call(Function::GetExperience, vec![Val::from(390738), Val::from(0)])?;
                                } else if (ctx.var("BaseLevel").get()?.number()? > 80 && ctx.var("BaseLevel").get()?.number()? < 86) {
                                    ctx.call(Function::GetExperience, vec![Val::from(451020), Val::from(0)])?;
                                } else if (ctx.var("BaseLevel").get()?.number()? > 85 && ctx.var("BaseLevel").get()?.number()? < 91) {
                                    ctx.call(Function::GetExperience, vec![Val::from(546156), Val::from(0)])?;
                                } else {
                                    ctx.call(Function::GetExperience, vec![Val::from(1220358), Val::from(0)])?;
                                }
                            }
                        }
                    }
                }
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("god_eremes").get()? == 26 {
                    ctx.lines_as(
                        "Egnigem",
                        args![
                            "As a Crusader...",
                            "This is the best thing",
                            "I can do in return for",
                            "what you have done for me."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Egnigem",
                        args![
                            "Listen...",
                            "I'm going to",
                            "imbue you with the",
                            "^666666last vestiges of my strength^000000..."
                        ],
                    )?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL5")?])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HOLYHIT")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Egnigem",
                        args![
                            "May the heavens",
                            "answer me as I call upon the",
                            "light of justice that empowers the defenders of truth, and threaten the enemies of peace."
                        ],
                    )?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL6")?])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HOLYHIT")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Egnigem",
                        args![
                            "May you always shine with",
                            "the light of truth. Let the light within you eradicate the shadows",
                            "of deception, fear and malice."
                        ],
                    )?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL6")?])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HOLYHIT")?])?;
                    ctx.next()?;
                    ctx.lines_as("Egnigem", args!["By the holy power", "invested in me, I humbly bestow upon you my remaining strength. Fight honorably, and do not lose sight of righteousness."])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LOCKON")?])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HOLYCROSS")?])?;
                    ctx.var("god_eremes").set(Val::from(27))?;
                    {
                        if ctx.var("BaseLevel").get()?.number()? < 56 {
                            ctx.call(Function::GetExperience, vec![Val::from(27000), Val::from(0)])?;
                        } else {
                            if (ctx.var("BaseLevel").get()?.number()? > 55 && ctx.var("BaseLevel").get()?.number()? < 61) {
                                ctx.call(Function::GetExperience, vec![Val::from(30000), Val::from(0)])?;
                            } else {
                                if (ctx.var("BaseLevel").get()?.number()? > 60 && ctx.var("BaseLevel").get()?.number()? < 66) {
                                    ctx.call(Function::GetExperience, vec![Val::from(56052), Val::from(0)])?;
                                } else {
                                    if (ctx.var("BaseLevel").get()?.number()? > 65 && ctx.var("BaseLevel").get()?.number()? < 71) {
                                        ctx.call(Function::GetExperience, vec![Val::from(82233), Val::from(0)])?;
                                    } else if (ctx.var("BaseLevel").get()?.number()? > 70 && ctx.var("BaseLevel").get()?.number()? < 76) {
                                        ctx.call(Function::GetExperience, vec![Val::from(212271), Val::from(0)])?;
                                    } else if (ctx.var("BaseLevel").get()?.number()? > 75 && ctx.var("BaseLevel").get()?.number()? < 81) {
                                        ctx.call(Function::GetExperience, vec![Val::from(390738), Val::from(0)])?;
                                    } else if (ctx.var("BaseLevel").get()?.number()? > 80 && ctx.var("BaseLevel").get()?.number()? < 86) {
                                        ctx.call(Function::GetExperience, vec![Val::from(451020), Val::from(0)])?;
                                    } else if (ctx.var("BaseLevel").get()?.number()? > 85 && ctx.var("BaseLevel").get()?.number()? < 91) {
                                        ctx.call(Function::GetExperience, vec![Val::from(546156), Val::from(0)])?;
                                    } else {
                                        ctx.call(Function::GetExperience, vec![Val::from(1220358), Val::from(0)])?;
                                    }
                                }
                            }
                        }
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("god_eremes").get()?.number()? < 20 {
                    ctx.lines_as(
                        "Egnigem",
                        args![
                            "Were you betrayed by fate as well, or are you simply a wanderer that's stumbled into this land of darkness?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Egnigem",
                        args![
                            "Hmm, the living don't deserve",
                            "to be in this realm of cold and suffering. But I can only help you escape by telling you what little",
                            "I know."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Ask him about buildings.:Remove marks on mini-map.:Cancel.")])? {
                        1 => {
                            ctx.lines_as("Egnigem", args!["I see. If you better understand Niflheim's layout, you have a greater chance of surviving and escaping."])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Witch's Castle:Tool Shop:Weapon Shop:Tavern:Cancel")])? {
                                1 => {
                                    ctx.lines_as(
                                        "Egnigem",
                                        args![
                                            "The witch of Niflheim...",
                                            "You can find her castle at the ^FF3355+^000000 mark I've made on your mini-map."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Egnigem", args!["Hrrrm...", "It seems that the living who wind up in Niflheim are seeking out the witch. But I don't quite understand what's so important about her."])?;
                                    ctx.call(
                                        Function::ViewPoint,
                                        vec![Val::from(1), Val::from(253), Val::from(191), Val::from(2), Val::from(16777011)],
                                    )?;
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Egnigem",
                                        args![
                                            "The Tool shop? Here in Niflheim, they sell some unique items that",
                                            "you can't buy anywhere else."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Egnigem",
                                        args![
                                            "You might want to see their",
                                            "wares for yourself. I've drawn",
                                            "a ^CE6300+^000000 on your mini-map to mark",
                                            "its location."
                                        ],
                                    )?;
                                    ctx.call(
                                        Function::ViewPoint,
                                        vec![Val::from(1), Val::from(217), Val::from(196), Val::from(3), Val::from(16764515)],
                                    )?;
                                }
                                3 => {
                                    ctx.lines_as("Egnigem", args!["I've marked the location of the Weapon Shop at ^55FF33+^000000. There's nothing too special over there, though."])?;
                                    ctx.call(
                                        Function::ViewPoint,
                                        vec![Val::from(1), Val::from(216), Val::from(171), Val::from(4), Val::from(16733695)],
                                    )?;
                                }
                                4 => {
                                    ctx.lines_as(
                                        "Egnigem",
                                        args!["Tavern is at ^3355FF+^000000. They only sell drinks to the dead, though..."],
                                    )?;
                                    ctx.call(
                                        Function::ViewPoint,
                                        vec![Val::from(1), Val::from(189), Val::from(207), Val::from(5), Val::from(16724821)],
                                    )?;
                                }
                                5 => {
                                    ctx.lines_as("Egnigem", args!["Choose 'Remove marks on mini-map' from the menu to remove all the building location marks I've made."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.call(
                                Function::ViewPoint,
                                vec![Val::from(2), Val::from(253), Val::from(191), Val::from(2), Val::from(16711935)],
                            )?;
                            ctx.call(
                                Function::ViewPoint,
                                vec![Val::from(2), Val::from(217), Val::from(196), Val::from(3), Val::from(16711935)],
                            )?;
                            ctx.call(
                                Function::ViewPoint,
                                vec![Val::from(2), Val::from(216), Val::from(171), Val::from(4), Val::from(16711935)],
                            )?;
                            ctx.call(
                                Function::ViewPoint,
                                vec![Val::from(2), Val::from(189), Val::from(207), Val::from(5), Val::from(16711935)],
                            )?;
                            ctx.lines_as("Egnigem", args!["Alright, all the marks I've made have been removed from your mini-map. If you want to check the locations in Niflheim again, go ahead and ask me."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.lines_as("Egnigem", args!["You're gonna explore this place on your own? Pretty brave, aren't you? Just be careful: here in Niflheim, darkness reigns supreme."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.lines_as("Egnigem", args!["I really wish that", "Emma Searth could", "remember me..."])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFHis eyes seemed",
                        "to moisten with sadness.",
                        "Is it really possible for the",
                        "dead to shed tears?^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    } else {
        ctx.lines_as(
            "Egnigem",
            args!["Were you betrayed by fate as well, or are you simply a wanderer that's stumbled into this land of darkness?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Egnigem",
            args![
                "Hmm, the living don't deserve",
                "to be in this realm of cold and suffering. But I can only help you escape by telling you what little",
                "I know."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Ask him about buildings.:Remove marks on mini-map.:Cancel.")])? {
            1 => {
                ctx.lines_as(
                    "Egnigem",
                    args!["...I see. Now, let me introduce you all buildings in Niflheim!"],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Witch's Castle:Tool Shop:Weapon Shop:Tavern:Cancel")])? {
                    1 => {
                        ctx.lines_as(
                            "Egnigem",
                            args![
                                "The witch of Niflheim...",
                                "You can find her castle at the ^FF3355+^000000 mark I've made on your mini-map."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Egnigem", args!["Hrrrm...", "It seems that the living who wind up in Niflheim are seeking out the witch. But I don't quite understand what's so important about her."])?;
                        ctx.call(
                            Function::ViewPoint,
                            vec![Val::from(1), Val::from(253), Val::from(191), Val::from(6), Val::from(16777011)],
                        )?;
                    }
                    2 => {
                        ctx.lines_as(
                            "Egnigem",
                            args![
                                "The Tool shop? Here in Niflheim, they sell some unique items that",
                                "you can't buy anywhere else."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Egnigem",
                            args![
                                "You might want to see their",
                                "wares for yourself. I've drawn",
                                "a ^CE6300+^000000 on your mini-map to mark",
                                "its location."
                            ],
                        )?;
                        ctx.call(
                            Function::ViewPoint,
                            vec![Val::from(1), Val::from(217), Val::from(196), Val::from(7), Val::from(16764515)],
                        )?;
                    }
                    3 => {
                        ctx.lines_as("Egnigem", args!["I've marked the location of the Weapon Shop at ^55FF33+^000000. There's nothing too special over there, though."])?;
                        ctx.call(
                            Function::ViewPoint,
                            vec![Val::from(1), Val::from(216), Val::from(171), Val::from(8), Val::from(16733695)],
                        )?;
                    }
                    4 => {
                        ctx.lines_as(
                            "Egnigem",
                            args!["Tavern is at ^3355FF+^000000. They only sell drinks to the dead, though..."],
                        )?;
                        ctx.call(
                            Function::ViewPoint,
                            vec![Val::from(1), Val::from(189), Val::from(207), Val::from(9), Val::from(16724821)],
                        )?;
                    }
                    5 => {
                        ctx.lines_as(
                            "Egnigem",
                            args!["Choose 'Remove marks on mini-map' from the menu to remove all the building location marks I've made."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.call(
                    Function::ViewPoint,
                    vec![Val::from(2), Val::from(253), Val::from(191), Val::from(6), Val::from(16711935)],
                )?;
                ctx.call(
                    Function::ViewPoint,
                    vec![Val::from(2), Val::from(217), Val::from(196), Val::from(7), Val::from(16711935)],
                )?;
                ctx.call(
                    Function::ViewPoint,
                    vec![Val::from(2), Val::from(216), Val::from(171), Val::from(8), Val::from(16711935)],
                )?;
                ctx.call(
                    Function::ViewPoint,
                    vec![Val::from(2), Val::from(189), Val::from(207), Val::from(9), Val::from(16711935)],
                )?;
                ctx.lines_as("Egnigem", args!["Alright, all the marks I've made have been removed from your mini-map. If you want to check the locations in Niflheim again, go ahead and ask me."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as("Egnigem", args!["You're gonna explore this place on your own? Pretty brave, aren't you? Just be careful: here in Niflheim, darkness reigns supreme."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
        ctx.close_window()?;
    }
    Ok(Val::from(0))
}

pub fn egnigem(ctx: &Ctx) -> Script {
    egnigem_body(ctx, Vec::new()).map(|_| ())
}
