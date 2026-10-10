use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn crewman_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_comment_s = Val::from("");
    ctx.mes("[Kain Himere]")?;
    if ctx.var("kain_ticket").get()? == 4 {
        ctx.lines(args!["Ah...", "Here it is!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Kain Himere",
            args![
                "A few days ago, a customer",
                "from Einbroch left this on the",
                "Airship. He called and let us",
                "know that he can't come back",
                "to get it, but he is staying",
                "over at the Einbroch Hotel."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kain Himere",
            args![
                "I know it's a strange",
                "favor to ask, but would",
                "you deliver this to Defru",
                "Ark at the Einbroch Hotel?",
                "Of course, I'll repay you",
                "upon your return."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::SetQuest, vec![Val::from(2079)])?;
        ctx.var("kain_ticket").set(Val::from(5))?;
        ctx.mes("^3355FFKain Himere has given you a small box.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "Welcome to the Airship~",
        "If you have questions or",
        "need any assistance, please",
        "don't hesitate to ask me or",
        "any one of the other crewmen."
    ])?;
    ctx.next()?;
    if (ctx.var("kain_ticket").get()? == 0 || ctx.var("kain_ticket").get()? == 1) {
        match runtime::select_values(ctx, &[Val::from("About the Airship..."), Val::from("Leave a Comment")])? {
            1 => {
                ctx.mes("[Kain Himere]")?;
                ctx.var("kain_ticket").set((ctx.var("kain_ticket").get()? + Val::from(1)))?;
                if ctx.var("kain_ticket").get()? == 1 {
                    ctx.lines(args![
                        "Is this your first time flying?",
                        "I understand if you feel nervous. Before I worked here, I used to feel the same way. Still, this",
                        "Airship is pretty amazing. It's incredible what science can do..."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kain Himere",
                        args![
                            "I hear from the scholars",
                            "who developed the Airship",
                            "technology that just a little",
                            "piece of the heart of Ymir",
                            "generates the power for",
                            "this ship to fly. Incredible..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kain Himere",
                        args![
                            "You know, scientific research",
                            "has made living easier and more",
                            "comfortable in the Schwarzwald",
                            "Republic, much in the way magic",
                            "research has changed life in the Rune-Midgarts Kingdom."
                        ],
                    )?;
                } else {
                    ctx.lines(args![
                        "You want to hear more",
                        "about the Airship? Hmm,",
                        "there's not too much that",
                        "I know, but let me see..."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kain Himere",
                        args![
                            "Well, it's rumored that this",
                            "really big corporation runs",
                            "this Airship. Supposedly,",
                            "they've got their hands in",
                            "all sorts of enterprises."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kain Himere",
                        args![
                            "Since the Airships are our",
                            "form of national transportation,",
                            "the higher-ups must be making",
                            "a ton of money. It's pretty crazy."
                        ],
                    )?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Kain Himere",
                    args![
                        "You wish to leave",
                        "a comment about our",
                        "service? Tell me your",
                        "message and I'll report",
                        "it to the higher ups.",
                        "To cancel, press '0'."
                    ],
                )?;
                ctx.next()?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_comment_s = input;
                if l_comment_s.clone() == "kafra" {
                    ctx.lines_as(
                        "Kain Himere",
                        args!["K-Kafra...?", "Hmm, maybe I better", "not send this up after all..."],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                } else if l_comment_s.clone() == "0" {
                    ctx.lines_as(
                        "Kain Himere",
                        args![
                            "Ah, well, if you have any",
                            "helpful criticism about our",
                            "service, feel free to leave",
                            "me a comment at any time."
                        ],
                    )?;
                } else {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![(l_comment_s.clone() + Val::from("."))],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kain Himere",
                        args![
                            "Hmmmm...",
                            "I see. Well, I'll",
                            "send your message",
                            "to my superiors as",
                            "soon as possible.",
                            "Thank you very much."
                        ],
                    )?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        if (ctx.var("kain_ticket").get()? == 2 || ctx.var("kain_ticket").get()? == 3) {
            if ctx.var("kain_ticket").get()? == 3 {
                ctx.lines_as(
                    "Kain Himere",
                    args![
                        "Oh, how are you?",
                        "If you don't mind me",
                        "asking, where are you",
                        "headed this time?"
                    ],
                )?;
            } else {
                let choice = runtime::select_values(ctx, &[Val::from("About the Airship...")])?;
                ctx.var("@menu").set(choice)?;
                ctx.next()?;
                ctx.lines_as("Kain Himere", args!["You must really want to know all about the Airship, don't you? I'm sorry, but I don't know much more than what I've already told you."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kain Himere",
                    args![
                        "I guess if you'd want to know more, you should study to become a Sage in Juno and do your own Airship research..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kain Himere",
                    args!["^666666*Sob...*^000000", "E-excuse me...", "^666666*Sniff*^000000"],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("What the hell...?:What's wrong?")])? {
                    1 => {
                        ctx.lines_as(
                            "Kain Himere",
                            args![
                                "I-I'm sorry...",
                                "That was so",
                                "unprofessional.",
                                "H-have a safe trip",
                                "and th-thank you for",
                                "using the Airship! ^333333*Sob*^000000"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Kain Himere",
                            args!["I'm sorry, but it's", "a long story. Plus,", "you wouldn't understand..."],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("I have time for a long story.")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Kain Himere",
                            args![
                                "Alright, I might as well get",
                                "this off my chest. I guess it",
                                "all started when I was a young",
                                "man living in Einbech."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kain Himere",
                            args![
                                "Those days, I was pretty hot",
                                "headed and short tempered,",
                                "but working as a miner used to",
                                "always bring me a sense of peace. Staking my life with my friends, mining pick in hand. Yeah..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kain Himere",
                            args![
                                "It might have been dangerous,",
                                "but it was good, hard and honest work. You know, the mines that connected to the Einbech lode",
                                "used to be everything to our little town. But then, things changed..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kain Himere",
                            args![
                                "The factories started popping",
                                "up and all the ores were moved",
                                "to where the smokestacks never",
                                "stopped churning. More miners",
                                "started working in the factories and the mines became lonelier..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kain Himere",
                            args![
                                "But I loved working on the",
                                "land so much, I kept at it",
                                "until the accident happened.",
                                "Now I can't use my right arm",
                                "the way I used to, so I had to",
                                "quit my job as a miner..."
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("I'm sorry to hear that.")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kain Himere",
                            args![
                                "Yeah...",
                                "I was really devastated",
                                "when it happened. Those",
                                "were probably the worst",
                                "years of my entire life."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kain Himere",
                            args![
                                "I couldn't think about",
                                "life without mining and",
                                "I just lost all enthusiasm",
                                "for life. I developed a drinking problem and started shutting out my family. I was so stupid..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kain Himere",
                            args![
                                "I still can't believe how",
                                "supportive my wife was during",
                                "those days. Even when I was at",
                                "my lowest, she tried to sooth my sorrow and even took on a job",
                                "to support me and our daughter."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kain Himere",
                            args![
                                "^666666*Sob*^000000 It was only after she",
                                "died that I realized how much",
                                "she must have suffered. To make",
                                "matters worse, I had to leave my daughter behind with a neighbor",
                                "while I searched for work..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kain Himere",
                            args![
                                "I spent years wandering",
                                "from town to town, living",
                                "a pretty wild life until I could pick myself up again. And finally, I became an Airship crewman."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kain Himere",
                            args![
                                "It's been twenty years",
                                "since I've lost my family.",
                                "My daughter is somewhere",
                                "out there, but I can't even",
                                "remember her name..."
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Have you tried looking for her?")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Kain Himere",
                            args![
                                "I tried going to the house",
                                "where my neighbor had lived,",
                                "but they moved out years ago.",
                                "I really have no clue where she",
                                "could be. But I don't deserve to see her. I was a horrible father!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kain Himere",
                            args![
                                "^666666*Sob*^000000 I'm sorry you",
                                "had to hear all of that,",
                                "but I really appreciate that",
                                "you've listened to me. Now",
                                "tell me, where are you headed?"
                            ],
                        )?;
                    }
                    _ => {}
                }
            }
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Einbroch:Juno")])? {
                1 => {
                    ctx.var("kain_ticket").set(Val::from(4))?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I'm heading", "to Einbroch."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kain Himere",
                        args![
                            "Perfect...!",
                            "If you don't mind,",
                            "would you do a favor",
                            "for me? First, let me",
                            "find that package..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.var("kain_ticket").set(Val::from(3))?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I'm heading", "to Juno."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kain Himere",
                        args![
                            "Ah, I see...",
                            "There was something",
                            "I needed sent to Einbroch.",
                            "Anyway, have a good trip.",
                            "Oh, and thanks for listening."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            if (ctx.var("kain_ticket").get()?.number()? > 4 && ctx.var("kain_ticket").get()?.number()? < 10) {
                ctx.lines_as(
                    "Kain Himere",
                    args![
                        "Oh hello!",
                        "Things are a little",
                        "hectic right now, but",
                        "did you delive-- Oh!",
                        "Wait, excuse me! Sir--!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3355FFKain seems too", "busy to speak to", "you right now...^000000"])?;
            } else if ctx.var("kain_ticket").get()? == 10 {
                ctx.lines_as(
                    "Kain Himere",
                    args!["Welcome back to the Airship. So did you deliver that little box safely?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["No, unfortunately. He was away when I got there. Let me give this box back to you."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kain Himere",
                    args![
                        "Oh...",
                        "What should I do now? Oh well, sorry for putting you through so much trouble."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("Casually mention Miner's Song"), Val::from("Suavely mention Tarsha")],
                )? {
                    1 => {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "While we're on the topic",
                                "of boxes and deliveries,",
                                "have you ever heard of",
                                "the ''Miner's Song?''"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kain Himere",
                            args![
                                "''Miner's Song...''",
                                "I miss singing that",
                                "while working the mines.",
                                "I used to sing it all the time",
                                "when I was younger, actually."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kain Himere",
                            args![
                                "I think I sang it so",
                                "much, I even got my little",
                                "daughter to sing it with me~",
                                "(But what was her name?!)"
                            ],
                        )?;
                    }
                    2 => {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["^333333*Cough cough*", "*Cou--TARSHA--Cough*^000000"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kain Himere",
                            args![
                                "What the...?",
                                "Wait. T-Tarsha...",
                                "Why does it feel like that",
                                "name means something?"
                            ],
                        )?;
                    }
                    _ => {}
                }
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "You know...",
                        "I met a little girl in Einbroch",
                        "who was singing the ''Miner's",
                        "Song.'' It seems her mother",
                        "Tarsha had taught it to her."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kain Himere",
                    args![
                        "Interesting. But usually,",
                        "only miners would know ",
                        "that song. Does this have",
                        "anything to do with me?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Well, this little girl's mother, Tarsha, learned the song from",
                        "her father who might have been",
                        "a miner. But he mysteriously",
                        "disappeared years ago..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kain Himere",
                    args!["Poor girl...!", "Her father just", "left her?! Why,", "she's just like..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kain Himere",
                    args![
                        "Wait...",
                        "Sweet Christ...",
                        "The miner who taught",
                        "Tarsha that song might",
                        "have been... ^0000FFme^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Well, for now, we can't",
                        "be too sure. Do you have",
                        "anything that might prove",
                        "you're Tarsha's father?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kain Himere",
                    args![
                        "Ehhmmm...",
                        "Before I left own, I buried",
                        "some of my stuff underneath",
                        "a tree in Einbech. I can't exactly remember where, but my wife's",
                        "journal should be there."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kain Himere",
                    args![
                        "Since my wife kept track",
                        "of everything in her journal,",
                        "she should have written enough",
                        "about our daughter for us to know whether or not this Tarsha could be my long lost daughter."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kain Himere",
                    args![
                        "Now, I can't really leave",
                        "the Airship, so would you help",
                        "me by finding that journal and",
                        "seeing if Tarsha is really related to me somehow? I'd be grateful",
                        "if you could do that for me..."
                    ],
                )?;
                ctx.call(Function::ChangeQuest, vec![Val::from(2081), Val::from(2082)])?;
                ctx.var("kain_ticket").set(Val::from(11))?;
            } else if (ctx.var("kain_ticket").get()? == 11 || ctx.var("kain_ticket").get()? == 12) {
                ctx.lines_as(
                    "Kain Himere",
                    args![
                        "Would you find my wife's",
                        "journal and see if Tarsha",
                        "is really my daughter? The",
                        "journal should be buried",
                        "underneath a tree in Einbech."
                    ],
                )?;
            } else if ctx.var("kain_ticket").get()? == 13 {
                ctx.lines(args![
                    "^3355FFYou better fulfill Kain's",
                    "request by finding his wife's",
                    "journal and confronting Tarsha",
                    "before you speak to him again.^000000"
                ])?;
            } else if ctx.var("kain_ticket").get()? == 14 {
                if ctx.call(Function::CountItem, vec![Val::from(7276)])? == 1 {
                    ctx.lines(args![
                        "^3355FFYou give his wife's",
                        "journal to Kain and tell",
                        "him that you learn that Tarsha",
                        "is really his daughter and that",
                        "she is happily married and is",
                        "an inventor in Einbroch.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFKain's eyes well",
                        "with tears and his",
                        "entire body trembles",
                        "with unrestrainable joy."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kain Himere",
                        args!["She must have", "had a hard time...", "But she doesn't", "hate me at all."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kain Himere",
                        args![
                            "Rashelle?",
                            "Can you see me?",
                            "Can I really be this",
                            "happy? I'm so sorry,",
                            "my love... ^666666*Sob...*^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kain Himere",
                        args![
                            "Thank you for all of",
                            "your help, youngster.",
                            "And please, take this",
                            "as a token of my gratitude."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kain Himere",
                        args![
                            "I know this isn't",
                            "much, but please",
                            "understand that it's",
                            "all I have to give you.",
                            "God bless you for all",
                            "you've done for me~"
                        ],
                    )?;
                    ctx.call(Function::CompleteQuest, vec![Val::from(2085)])?;
                    ctx.var("kain_ticket").set(Val::from(15))?;
                    ctx.call(Function::DelItem, vec![Val::from(7276), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(7311), Val::from(4)])?;
                } else {
                    ctx.lines(args![
                        "^3355FFHmmm...",
                        "It would be better",
                        "if you approached",
                        "Kain with the Doodled",
                        "Letter that you received.^000000"
                    ])?;
                }
            } else {
                ctx.lines_as(
                    "Kain Himere",
                    args![
                        "Oh, how are you",
                        "lately, my friend?",
                        "I've been doing great,",
                        "especially since you've",
                        "helped me find my daughter.",
                        "Enjoy your travels~"
                    ],
                )?;
            }
        }
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn crewman(ctx: &Ctx) -> Script {
    crewman_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DoorEinStep {
    Start,
    OnTouch,
}

fn door_ein_run(ctx: &Ctx, mut step: DoorEinStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DoorEinStep::Start => {
                step = DoorEinStep::OnTouch;
                continue 'machine;
            }
            DoorEinStep::OnTouch => {
                if ctx.var("kain_ticket").get()? == 5 {
                    ctx.var("kain_ticket").set(Val::from(6))?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Excuse me...?"])?;
                    ctx.next()?;
                    ctx.mes("^3355FFNo one's here!^000000")?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "This must be the room...",
                            "But where the hell is he?",
                            "I can't just leave this stuff",
                            "here if he won't come back."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Oh well...", "I guess I can", "just give this", "back to Kain", "on the Airship."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "While I'm in Einbroch,",
                            "I may as well take a look",
                            "around here and see if I can",
                            "find anything interesting. Still, I can't help but think of Kain",
                            "Himere's long lost daughter... "
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Maybe...",
                            "Just maybe...",
                            "I can find some",
                            "sort of clue about",
                            "where she is. She might",
                            "even live here in Einbroch."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "I think I'll do that.",
                            "Maybe even right this",
                            "minute. Treat it like",
                            "sort of quest or something.",
                            "Yeah, that could work."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn door_ein(ctx: &Ctx) -> Script {
    door_ein_run(ctx, DoorEinStep::Start, Vec::new()).map(|_| ())
}

pub fn door_ein_ontouch(ctx: &Ctx) -> Script {
    door_ein_run(ctx, DoorEinStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ElleChernoStep {
    Start,
    OnTouch,
}

fn elle_cherno_run(ctx: &Ctx, mut step: ElleChernoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ElleChernoStep::Start => {
                step = ElleChernoStep::OnTouch;
                continue 'machine;
            }
            ElleChernoStep::OnTouch => {
                ctx.lines_as(
                    "Elle Cherno",
                    args![
                        "Let's get to work",
                        "fear-less comraaades~!",
                        "Do our best! Nothing",
                        "can stop us, lads~!"
                    ],
                )?;
                if ctx.var("kain_ticket").get()? == 6 {
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Hello!:What are you singing?")])? {
                        1 => {
                            ctx.lines_as(
                                "Elle Cherno",
                                args![
                                    "Let's get to work",
                                    "fear-less comraaades~!",
                                    "Do our best! Nothing",
                                    "can stop us, lads~!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Elle Cherno",
                                args!["I like this song lots!", "Especially when I yell out", "''comrades!'' COMRADES!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.call(Function::ChangeQuest, vec![Val::from(2079), Val::from(2080)])?;
                            ctx.var("kain_ticket").set(Val::from(7))?;
                            ctx.lines_as(
                                "Elle Cherno",
                                args![
                                    "This...?",
                                    "It's the ''Miner's Song!''",
                                    "My mommy taught it to me",
                                    "and she sings it a lot. Daddy",
                                    "says it's too noisy, though."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Elle Cherno", args!["Mmm, when my mommy sings", "this song, sometimes she looks", "a little sad. Jus' today she sang it a little and looked like she was gonna cry. ^666666*Gasp!*^000000 Wait, do you", "think maybe Mommy's sick?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Elle Cherno",
                                args![
                                    "Oh no, oh no...",
                                    "Hey, will you go see",
                                    "if my mommy's okay for me?",
                                    "Please? Our house is behind",
                                    "the Hotel, so tell me if she's",
                                    "not feeling good, promise?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("kain_ticket").get()? == 7 {
                    ctx.next()?;
                    ctx.lines_as(
                        "Elle Cherno",
                        args![
                            "Our house is right",
                            "behind of the Hotel.",
                            "Will you go there and",
                            "see my mommy for me,",
                            "please? I'm worried..."
                        ],
                    )?;
                } else if ctx.var("kain_ticket").get()? == 13 {
                    ctx.next()?;
                    if ctx.call(Function::CheckWeight, vec![Val::from(7276), Val::from(1)])? == 0 {
                        ctx.lines_as(
                            "Elle Cherno",
                            args![
                                "Hey! I have something",
                                "to give you, but you have",
                                "tooooo much stuff! Hey,",
                                "come back later so I can",
                                "give it to you, okay?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Elle Cherno",
                        args![
                            "You saw my grandpa?",
                            "You're his friend, right?",
                            "A-are you gonna see",
                            "him later? 'Cuz... cuz..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elle Cherno",
                        args![
                            "I made this for him.",
                            "I don't know how to write,",
                            "but I made Grampa a letter",
                            "as best as I can. Will you",
                            "promise to give this to him",
                            "for me, pleeeeeease?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args!["^3355FFElle put a big", "doodled message", "into your hands.^000000"])?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2084), Val::from(2085)])?;
                    ctx.var("kain_ticket").set(Val::from(14))?;
                    ctx.call(Function::GetItem, vec![Val::from(7276), Val::from(1)])?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn elle_cherno(ctx: &Ctx) -> Script {
    elle_cherno_run(ctx, ElleChernoStep::Start, Vec::new()).map(|_| ())
}

pub fn elle_cherno_ontouch(ctx: &Ctx) -> Script {
    elle_cherno_run(ctx, ElleChernoStep::OnTouch, Vec::new()).map(|_| ())
}

fn theo_cherno_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("kain_ticket").get()? == 7 {
        shared::quests_quests_airship::f_cherno(ctx, vec![])?;
        ctx.var("kain_ticket").set(Val::from(8))?;
    } else if (ctx.var("kain_ticket").get()?.number()? > 7 && ctx.var("kain_ticket").get()?.number()? < 10) {
        ctx.lines_as(
            "Theo Cherno",
            args![
                "You're very kind,",
                "adventurer. There",
                "should be more people",
                "in the world like you."
            ],
        )?;
    } else if (ctx.var("kain_ticket").get()?.number()? > 9 && ctx.var("kain_ticket").get()?.number()? < 13) {
        ctx.lines_as(
            "Theo Cherno",
            args![
                "Are you sure",
                "that you can find",
                "Tarsha's father? Oh,",
                "you must be a godsend!"
            ],
        )?;
    } else if ctx.var("kain_ticket").get()?.number()? > 12 {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
        ctx.lines_as(
            "Theo Cherno",
            args!["I'm so happy", "for my wife. I....", "I don't know how", "I can thank you..."],
        )?;
    } else {
        ctx.lines_as(
            "Theo Cherno",
            args![
                "Hmm...",
                "Can we talk later?",
                "I'm pretty busy trying",
                "to fix this thing at",
                "the moment."
            ],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn theo_cherno(ctx: &Ctx) -> Script {
    theo_cherno_body(ctx, Vec::new()).map(|_| ())
}

fn tarsha_cherno_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("kain_ticket").get()? == 7 {
        shared::quests_quests_airship::f_cherno(ctx, vec![])?;
        ctx.var("kain_ticket").set(Val::from(8))?;
    } else {
        if ctx.var("kain_ticket").get()? == 8 {
            ctx.lines_as(
                "Tarsha Cherno",
                args![
                    "Although most of these",
                    "contraptions aren't worth",
                    "showing off, we've put a lot",
                    "of work in inventing these",
                    "machines. So, of course,",
                    "they give us pride~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tarsha Cherno",
                args![
                    "Maybe all these little",
                    "projects might not be",
                    "that impressive, but I like",
                    "to think of them as stepping",
                    "stones to future achievements."
                ],
            )?;
        } else if ctx.var("kain_ticket").get()? == 9 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Excuse me, ma'am,",
                    "but you have a scar",
                    "on your shoulder. Is",
                    "that from an accident?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tarsha Cherno",
                args![
                    "Oh, that's an old",
                    "injury that happened",
                    "years ago. I suppose",
                    "I was quite the trouble",
                    "maker to the people",
                    "who raised me..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tarsha Cherno",
                args![
                    "You see, I lost my parents",
                    "when I was very young. It",
                    "was only later when I realized",
                    "I was raised by foster parents.",
                    "I learned that my real mother",
                    "died a long time ago..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tarsha Cherno",
                args![
                    "As for my father, no",
                    "one knows if he's dead",
                    "or alive. I tried my best",
                    "to my foster parents happy,",
                    "but I ended up being too rebellious and giving them grief."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tarsha Cherno",
                args![
                    "I finally left home and",
                    "studied mechanics. I met",
                    "my husband in school and",
                    "it was the greatest thing that",
                    "ever happened to me~"
                ],
            )?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_THROB")?,
                    ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Theo Cherno")])?,
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "By the way, I heard",
                    "your daughter singing",
                    "some sort of Miner's Song.",
                    "Did you teach her that?"
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_HUK")?,
                    ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Theo Cherno")])?,
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Theo Cherno",
                args![
                    "I don't understand",
                    "why little Elle likes",
                    "such a rowdy, man song.",
                    "This is all your fault, Tarsha!"
                ],
            )?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_ANGER")?,
                    ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Theo Cherno")])?,
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
            ctx.lines_as(
                "Tarsha Cherno",
                args![
                    "Oh~hohohoho~",
                    "I was too young to",
                    "remember it clearly,",
                    "but I'm sure my real",
                    "father loved that song."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tarsha Cherno",
                args![
                    "I barely recall that",
                    "he used to sing it while",
                    "cleaning our old house.",
                    "Yes, I don't remember what",
                    "he looked like, but he must",
                    "have been a miner at one time."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "A miner...?",
                    "(Wait, that crewman,",
                    "Kain Himere! He was",
                    "a miner who lost his",
                    "daughter! Just maybe...)"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["...!", "Tarsha...", "Your father", "may still be alive!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Tarsha Cherno", args!["...What?"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "I've got to go check",
                    "something now, but",
                    "hopefully I'll be back",
                    "soon with good news!"
                ],
            )?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2080), Val::from(2081)])?;
            ctx.var("kain_ticket").set(Val::from(10))?;
        } else if (ctx.var("kain_ticket").get()? == 10 || ctx.var("kain_ticket").get()? == 11) {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Tarsha, I know someone",
                    "who might be your father!",
                    "Just wait for me and hopefully",
                    "I'll bring back good news soon!"
                ],
            )?;
        } else if ctx.var("kain_ticket").get()? == 12 {
            ctx.lines_as(
                "Tarsha Cherno",
                args!["Oh, welcome back~", "So were you able to", "bring back good news?"],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Why, yes.", "Speaking of which,", "allow me to... check", "something first..."],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from("Check her neck.:Check her hands.:Check her legs.:Check her forehead.")],
            )? {
                1 => {
                    ctx.lines(args!["^3355FFUh oh...", "Nothing's there!^000000"])?;
                    ctx.next()?;
                    ctx.lines_as("Theo Cherno", args!["H-how rude!", "Touching another", "man's wife...?!"])?;
                    ctx.next()?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_PIERCE")?])?;
                    ctx.call(Function::PercentHeal, vec![Val::from(-10), Val::from(0)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines(args!["^3355FFYou found", "a burn mark on", "her hand.^000000"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "That's it!",
                            "Tarsha, your",
                            "father's name is",
                            "Kain Himere and he's",
                            "working on the Airship",
                            "as one of the crewmen."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["He misses you", "a lot and hopes", "that you can forgive", "him... Someday."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Tarsha Cherno",
                        args![
                            "Are...",
                            "Are you serious?",
                            "My father's alive!",
                            "Oh thank god! Thank",
                            "you so much, adventurer!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Tarsha Cherno",
                        args![
                            "...........",
                            "So that means, my",
                            "real name would be",
                            "^3131FFTarsha Himere Cherno^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Tarsha Cherno",
                        args![
                            "Oh, you're a godsend!",
                            "I'll never forget what you've",
                            "done for me! I must repay",
                            "you somehow! But all I know",
                            "is mechanical engineering..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Tarsha Cherno",
                        args![
                            "Oh, I know~",
                            "If you have any",
                            "^3131FFbroken equipment^000000,",
                            "I can fix it with this",
                            "Expert Repairman. It's",
                            "one of our best inventions!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Tarsha Cherno",
                        args![
                            "I must go and see",
                            "my real father soon!",
                            "I've missed him for years!",
                            "Thank you again for all",
                            "you've done for my family~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Tarsha Cherno",
                        args![
                            "Oh, and before you",
                            "go, would you go see",
                            "Elle again? She wanted to",
                            "talk to you for some reason."
                        ],
                    )?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2083), Val::from(2084)])?;
                    ctx.var("kain_ticket").set(Val::from(13))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines(args!["^3355FFUh oh...", "Nothing's there!^000000"])?;
                    ctx.next()?;
                    ctx.lines_as("Theo Cherno", args!["H-how rude!", "Touching another", "man's wife...?!"])?;
                    ctx.next()?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_PIERCE")?])?;
                    ctx.call(Function::PercentHeal, vec![Val::from(-20), Val::from(0)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.lines(args!["^3355FFUh oh...", "Nothing's there!^000000"])?;
                    ctx.next()?;
                    ctx.lines_as("Theo Cherno", args!["H-how rude!", "Touching another", "man's wife...?!"])?;
                    ctx.next()?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_PIERCE")?])?;
                    ctx.call(Function::PercentHeal, vec![Val::from(-10), Val::from(0)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("kain_ticket").get()?.number()? > 12 {
            ctx.lines_as(
                "Tarsha Cherno",
                args![
                    "I've been feeling great",
                    "after all you've done for",
                    "us. Once again, I'd like to",
                    "thank you, kind adventurer."
                ],
            )?;
        } else {
            ctx.lines_as(
                "Tarsha Cherno",
                args!["I'm sorry, but", "we're pretty busy.", "Please excuse us~"],
            )?;
        }
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn tarsha_cherno(ctx: &Ctx) -> Script {
    tarsha_cherno_body(ctx, Vec::new()).map(|_| ())
}

fn exp_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn exp_ein(ctx: &Ctx) -> Script {
    exp_ein_body(ctx, Vec::new()).map(|_| ())
}

fn unidentified_machine_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFIt's...",
        "It's a really",
        "strange looking",
        "machine. Does it",
        "even do anything?^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn unidentified_machine_ein(ctx: &Ctx) -> Script {
    unidentified_machine_ein_body(ctx, Vec::new()).map(|_| ())
}

fn mirror_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_r = Val::from(0);
    if ctx.var("kain_ticket").get()? == 8 {
        ctx.lines(args![
            "^3355FFIt's a mirror.",
            "So, of course the",
            "first thing you'll",
            "see is yourself."
        ])?;
        ctx.next()?;
        l_r = ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?;
        if l_r.clone() == 5 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Oh wow...",
                    "I look pretty good.",
                    "No wait. Really, really",
                    "good. So this must be",
                    "why I get such great",
                    "service at restaurants..."
                ],
            )?;
        } else if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
            let subject1 = l_r.clone();
            if subject1 == 1 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Man...", "I didn't know", "I was so good", "looking! Ooh yah~"],
                )?;
            } else if subject1 == 2 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Whoa...",
                        "So that's why the",
                        "ladies keep looking",
                        "at me. I'm a walking",
                        "free gun show!"
                    ],
                )?;
            } else if subject1 == 3 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "It's a shame I can't job",
                        "change to Male Model.",
                        "Clearly, I'd be like, Job",
                        "Level 87 or something.",
                        "Man, I'm beautiful..."
                    ],
                )?;
            } else if subject1 == 4 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "This is my reflection?!",
                        "No one can be this good",
                        "looking, not without special",
                        "effects! I mean, it's not",
                        "fair to everyone else..."
                    ],
                )?;
            }
        } else {
            let subject2 = l_r.clone();
            if subject2 == 1 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Oh. Wow.",
                        "I never realized...",
                        "Everything is in",
                        "perfect proportion!",
                        "No wonder people",
                        "want to party with me~"
                    ],
                )?;
            } else if subject2 == 2 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Oh my gosh~",
                        "How can I look this",
                        "good without any makeup?",
                        "I-It isn't fair to all the other",
                        "girls... Wow, is this really me?"
                    ],
                )?;
            } else if subject2 == 3 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Am I going crazy?",
                        "Is that girl in the",
                        "mirror really me...?",
                        "How did I not realize",
                        "how gorgeous I look?"
                    ],
                )?;
            } else if subject2 == 4 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "So...",
                        "What did you do with",
                        "this mirror? Because I'm",
                        "looking at my reflection",
                        "and I can't seem to find",
                        "any flaws with my figure..."
                    ],
                )?;
                ctx.next()?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_PROFUSELY_SWEAT")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Tarsha Cherno")])?,
                    ],
                )?;
                ctx.lines_as(
                    "Tarsha Cherno",
                    args![
                        "Actually...",
                        "That's not one",
                        "of our inventions.",
                        "It's just a normal mirror..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Well now...!",
                        "Are you sure?",
                        "Because my face looks",
                        "freakin' immaculate! And",
                        "it's like, I'm almost too sexy!"
                    ],
                )?;
            }
        }
        ctx.next()?;
        ctx.lines(args![
            "^3355FFAfter enjoying that",
            "little epiphany, you",
            "see a reflection of",
            "Tarsha's neck as you",
            "set the mirror back down.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFAs you take a closer",
            "look, you find that there's",
            "a strange mark around her",
            "neck. What ever could it mean?^000000"
        ])?;
        ctx.var("kain_ticket").set(Val::from(9))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn mirror_ein(ctx: &Ctx) -> Script {
    mirror_ein_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TreeT11Step {
    Start,
    LDiary,
}

fn tree_t1_1_run(ctx: &Ctx, mut step: TreeT11Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_find_diary = Val::from(0);
    'machine: loop {
        match step {
            TreeT11Step::Start => {
                if ctx.var("kain_ticket").get()? == 11 {
                    ctx.lines(args![
                        "^3355FFYou crouch down",
                        "under the tree and",
                        "begin digging into",
                        "the ground.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args!["...", "......"])?;
                    ctx.next()?;
                    ctx.lines(args!["...", "......", "........."])?;
                    ctx.next()?;
                    l_find_diary = ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?;
                    if l_find_diary.clone().number()? > 10 {
                        ctx.lines(args!["^3355ffUnfortunately,", "you weren't able", "to find anything.^000000"])?;
                    } else {
                        ctx.lines(args![
                            "^3355FFYou have found a journal",
                            "among some other articles",
                            "that have been buried by Kain.",
                            "You open the journal and begin",
                            "to read what's written inside.^000000"
                        ])?;
                        tree_t1_1_run(ctx, TreeT11Step::LDiary, vec![])?;
                        ctx.next()?;
                        ctx.lines(args!["...", "......"])?;
                        ctx.next()?;
                        ctx.mes("^3355FFThat was the last page of the journal. You picked it up so that you can bring it over to ^3131FFTarsha.^000000")?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(2082), Val::from(2083)])?;
                        ctx.var("kain_ticket").set(Val::from(12))?;
                    }
                } else if ctx.var("kain_ticket").get()? == 12 {
                    ctx.lines(args!["^3355FFYou open", "the journal", "and begin to read...^000000"])?;
                    tree_t1_1_run(ctx, TreeT11Step::LDiary, vec![])?;
                } else {
                    return Err(Stop::End);
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            TreeT11Step::LDiary => {
                ctx.next()?;
                ctx.lines(args!["...", "......"])?;
                ctx.next()?;
                ctx.lines(args![
                    "<Date : OX OX>",
                    "^856363I'm sooo happy",
                    "to be with Kain.",
                    "He's such an honest,",
                    "sincere man, even if",
                    "he is quiet sometimes."
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^856363Although he's a very",
                    "good husband to me,",
                    "I'm not so good at being",
                    "a housewife. But I'll do",
                    "my very best for us both."
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "<Date : OX OX>",
                    "^856363I fell asleep while",
                    "fixing dinner and burned",
                    "all the food. How can I be",
                    "so careless? But Kain ate",
                    "every bite, even if he had",
                    "to pretend that he liked it.^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "<Date : OX OX>",
                    "^856363Kain and I will be having",
                    "a child soon! I'm so happy,",
                    "but I'm also a little worried",
                    "sometimes. Kain is all smiles",
                    "though, and he gives me relief.^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "<Date : OX OX>",
                    "^856363More people have been",
                    "leaving the mines to work",
                    "for the new factories, many",
                    "of them Kain's friends. I think",
                    "Kain's pride was pretty hurt.",
                    "I wonder how I can help him?^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "<Date : OX OX>",
                    "^856363Kain and I are now the",
                    "proud parents of a beautiful",
                    "baby girl. We named her ^000000Tarsha^856363",
                    "and she has her father's eyes.",
                    "I'm going to be the best mother",
                    "that I can possibly be for her."
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "<Date : OX OX>",
                    "^856363Tarsha called me",
                    "''Mommy'' for the first",
                    "time! It's a miracle!",
                    "I want nothing else in",
                    "the world but for her to",
                    "be happy and healthy.^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "<Date : OX OX>",
                    "^856363Kain got into an accident",
                    "at the mine. While they were",
                    "digging ores, toxic gas was",
                    "released somehow. It wasn't",
                    "lethal, but Kain's arm has",
                    "been partly paralyzed...^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^856363When the doctor said that",
                    "it may even affect his memories",
                    "later, he became so depressed.",
                    "I tried to make him feel better",
                    "by making his favorite soup.",
                    "He smiled, but I could tell...^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^856363I could tell he's",
                    "not alright with this.",
                    "I hope he feels better",
                    "soon. His despair is",
                    "at least as great as his",
                    "passion for his job.^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args!["...", "......"])?;
                ctx.next()?;
                ctx.lines(args![
                    "<Date : OX OX>",
                    "^856363Kain's drinking is",
                    "getting worse. He comes",
                    "home too often screaming",
                    "and yelling. There's too",
                    "much anger in him and he's",
                    "not the same anymore..."
                ])?;
                ctx.next()?;
                ctx.lines(args!["...", "......"])?;
                ctx.next()?;
                ctx.lines(args![
                    "<Date : OX OX>",
                    "^856363Tarsha got her hand",
                    "scalded from boiling",
                    "water while playing in",
                    "the kitchen. Although she",
                    "was treated, she'll always",
                    "have that ^000000burn mark^856363.^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^856363After we got home from",
                    "the doctor's, Tarsha laughed",
                    "and played with her doll as if",
                    "nothing ever happened. But for",
                    "some reason, I couldn't stop",
                    "myself from crying...^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args!["...", "......"])?;
                ctx.next()?;
                ctx.lines(args![
                    "<Date : OX OX>",
                    "^856363Kain comes home less",
                    "nowadays. I've had to",
                    "start working in the store",
                    "after we spent all of our",
                    "savings. It's tough working",
                    "and taking care of the family."
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^856363Still, it's all worth",
                    "it to see Tarsha's smile.",
                    "But I can never stop worrying",
                    "about my poor, dear Kain..."
                ])?;
                ctx.next()?;
                ctx.lines(args!["...", "......"])?;
                ctx.next()?;
                ctx.lines(args![
                    "<Date : OX OX>",
                    "^856363Everyday, I feel weaker",
                    "and weaker, as if I die",
                    "just a little more each day.",
                    "I wanted to tell Kain about",
                    "what the doctor said, but he's",
                    "not ready for this news yet...^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^856363Lately, Tarsha cries",
                    "a lot. My poor baby loves",
                    "her father, but he's always",
                    "so distant. But sometimes,",
                    "I see him sadly smile for",
                    "Tarsha, if only for a moment.^000000"
                ])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn tree_t1_1(ctx: &Ctx) -> Script {
    tree_t1_1_run(ctx, TreeT11Step::Start, Vec::new()).map(|_| ())
}

fn unidentified_machine_as_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_total = Val::from(0);
    let mut l_total2 = Val::from(0);
    if ctx.var("kain_ticket").get()?.number()? > 12 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "This is the",
                "^FF0000Expert Repairman^000000?!",
                "It looks like it",
                "needs repairs itself..."
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Fix an Item"), Val::from("Cancel.")])?) == 1 {
            if ctx.call(Function::GetBrokenId, vec![Val::from(1)])? == 0 {
                ctx.lines(args!["*Beep-*", "You have", "nothing to fix."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            runtime::inventory_list(ctx)?;
            l_i = Val::from(0);
            'l1: loop {
                if !(runtime::op(&l_i.clone(), "<", &ctx.var("@inventorylist_count").get()?)?.is_true()) {
                    break 'l1;
                }
                'b1: {
                    l_total = (l_total.clone() + ctx.var("@inventorylist_attribute").get_at(runtime::index(&l_i.clone())?)?);
                }
                l_i = (l_i.clone() + Val::from(1));
            }
            ctx.lines(args![
                "*Beep-*",
                "You have a total",
                ((Val::from("of ") + l_total.clone()) + Val::from(" damaged items.")),
                "Shall I repair them?"
            ])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Yes"), Val::from("No")])?) == 1 {
                runtime::inventory_list(ctx)?;
                l_i = Val::from(0);
                'l2: loop {
                    if !(runtime::op(&l_i.clone(), "<", &ctx.var("@inventorylist_count").get()?)?.is_true()) {
                        break 'l2;
                    }
                    'b2: {
                        l_total2 = (l_total2.clone() + ctx.var("@inventorylist_attribute").get_at(runtime::index(&l_i.clone())?)?);
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                if l_total.clone().loosely_equals(&l_total2.clone()) {
                    ctx.call(Function::RepairAll, vec![])?;
                    ctx.lines(args!["*Beep-*", "Repair complete."])?;
                } else {
                    ctx.lines(args!["*Beep-*", "Please check", "your items again."])?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        ctx.mes("You have chosen to stop.")?;
    } else {
        ctx.lines(args![
            "^3355FFIt's some sort of",
            "strange machine.",
            "Its function and purpose",
            "is completely undiscernable.^000000"
        ])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn unidentified_machine_as(ctx: &Ctx) -> Script {
    unidentified_machine_as_body(ctx, Vec::new()).map(|_| ())
}

pub fn airship_airplane02(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::Start, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_onenable(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer25000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer25000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer30000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer34000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer34000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer38000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer38000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer48000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer48000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer48010(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer48010, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer53000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer53000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer58000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer58000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer63000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer63000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer68000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer68000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer73000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer73000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer73500(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer73500, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer74000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer74000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer74500(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer74500, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer75000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer75000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer75500(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer75500, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer76000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer76000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer76500(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer76500, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer77000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer77000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer77500(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer77500, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer78000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer78000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer79000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer79000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer80000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer80000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer81000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer81000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer82000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer82000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer83000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer83000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer84000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer84000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer85000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer85000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer86000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer86000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer87000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer87000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer88000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer88000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer93000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer93000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer98000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer98000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer103000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer103000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer103500(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer103500, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer104000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer104000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer104500(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer104500, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer105000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer105000, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer105500(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer105500, Vec::new()).map(|_| ())
}

pub fn airship_airplane02_ontimer106000(ctx: &Ctx) -> Script {
    airship_airplane02_run(ctx, AirshipAirplane02Step::OnTimer106000, Vec::new()).map(|_| ())
}
