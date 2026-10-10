use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn maku_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    ctx.mes("[Maku]")?;
    if ctx.var("friendship").get()?.number()? > 14 {
        ctx.call(Function::Cutin, vec![Val::from("lhz_macu07"), Val::from(2)])?;
        ctx.lines(args!["Why is this guy so", "late? Once he shows", "up, I swear, I'm gonna...!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Maku",
            args![
                "Eh, he might have",
                "some kinda reason for",
                "being late, but if he don't,",
                "I've been saving a whole",
                "six pack of kickass to open,",
                "just for him. Heh heh heh~"
            ],
        )?;
    } else {
        if ctx.var("friendship").get()? == 14 {
            ctx.call(Function::Cutin, vec![Val::from("lhz_macu05"), Val::from(2)])?;
            ctx.lines(args![
                "Why is Digotz",
                "so late? This isn't",
                "like him at all. Maybe",
                "something's wrong?"
            ])?;
        } else {
            if (ctx.var("friendship").get()? == 13 && ctx.call(Function::CountItem, vec![Val::from(7351)])?.number()? > 0) {
                ctx.call(Function::Cutin, vec![Val::from("lhz_macu06"), Val::from(2)])?;
                ctx.lines(args![
                    "Hey, what is that? You want",
                    "I should read this journal?",
                    "Er, okay, but I'm none too",
                    "comfortable going through",
                    "somebody's diary. It's just",
                    "kinda... creepy, you know?"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Maku",
                    args![
                        "Hey, this thing is",
                        "Benkaistein's. I haven't",
                        "seen that guy in a long while.",
                        "Ah, so he gave it to you for me",
                        "to read? Alright, I owe him a",
                        "favor or two, so I oughta..."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("lhz_macu06"), Val::from(255)])?;
                ctx.lines_as(
                    "Benkaistein's Journal",
                    args![
                        "^856363Today, me, Digotz and",
                        "Maku played this crazy flying",
                        "game. Basically, we make",
                        "these wings out of wood and",
                        "paper, jump off these hills",
                        "and try to fly. Dumb, I know.^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Benkaistein's Journal",
                    args![
                        "^856363Today it was my turn to",
                        "jump and flap my arms with",
                        "these fake, badly made wings.",
                        "It's not really a fun game when",
                        "I think about it. Boy, I hope",
                        "we don't do that again.^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("lhz_macu03"), Val::from(2)])?;
                ctx.lines_as(
                    "Maku",
                    args![
                        "What is he talking about?!",
                        "That game was real fun!",
                        "Yeah, I usually wore the",
                        "wings and Digotz always",
                        "wanted to wear them too."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                ctx.lines_as(
                    "Benkaistein's Journal",
                    args![
                        "^856363Maku, Digotz and me went",
                        "outside of town. Of course,",
                        "we didn't tell anyone or else",
                        "we'd get in trouble. It was",
                        "a really exciting day. But",
                        "then, we ran into a monster!^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Benkaistein's Journal",
                    args![
                        "^856363I wanted to run away but Maku",
                        "and Digotz wanted to beat it so",
                        "that we could become heroes.",
                        "Of course, we got hurt pretty",
                        "bad and the monster got away.",
                        "Boy, mom was not happy...^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("lhz_macu03"), Val::from(2)])?;
                ctx.lines_as(
                    "Maku",
                    args![
                        "That's right! Back then,",
                        "the three of us weren't",
                        "afraid of anything! Of course,",
                        "Digotz got beat up the most.",
                        "But I gotta say, he was also",
                        "the most fearless of us."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                ctx.lines_as(
                    "Benkaistein's Journal",
                    args![
                        "^856363Digot's been sick for three",
                        "days now. It's just a normal",
                        "cold and Maku keeps saying",
                        "it's Digotz's fault he got sick.^FFFFFF ^856363 But he's always asking me to",
                        "go visit him and see if he's okay.^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("lhz_macu04"), Val::from(2)])?;
                ctx.lines_as(
                    "Maku",
                    args![
                        "Wh-what?! No, I wasn't",
                        "worried at all! That must",
                        "have been the time Digot",
                        "caught Clymonia. You know,",
                        "that, uh, horrible disease. No",
                        "one should have that one!"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                ctx.lines_as(
                    "Benkaistein's Journal",
                    args![
                        "^856363Mom and dad keep telling",
                        "me not to hang out with Maku",
                        "anymore. Their reason is really",
                        "dumb, and I don't care if he is",
                        "poor. He's one of the best guys",
                        "that I'll ever know.^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("lhz_macu01"), Val::from(2)])?;
                ctx.lines_as(
                    "Benkaistein's Journal",
                    args![
                        "^856363Digotz's family is really",
                        "rich and they don't want him",
                        "to see Maku anymore either.",
                        "But Digotz doesn't care.",
                        "I know he likes Maku a lot.^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Maku", args!["...", "......"])?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                ctx.lines_as(
                    "Benkaistein's Journal",
                    args![
                        "^856363Today, the three of us",
                        "made an oath of brotherhood,",
                        "just like we read in the comic",
                        "book. We swore we'd always",
                        "be friends no matter what.",
                        "For always and for always.^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("lhz_macu06"), Val::from(2)])?;
                ctx.lines_as(
                    "Maku",
                    args![
                        "Well, that's true,",
                        "I guess, but people",
                        "change! Besides, we got",
                        "that idea from a comic book!",
                        "Well, if he apologizes first,",
                        "I guess I better forgive him."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Good...", "Because Digotz said", "that he'll be coming", "by in a few days."],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("lhz_macu07"), Val::from(2)])?;
                ctx.lines_as(
                    "Maku",
                    args![
                        "What?! He's really coming",
                        "here? What for? It's too late",
                        "to patch things up! Still, I'd be",
                        "a real prick if I didn't see him. Alright, fine! I'll teach that guy",
                        "a lesson once he's here!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Maku",
                    args![
                        "And, um, gimme that",
                        "journal! I'm gonna read",
                        "more of it so I can make",
                        "fun of Digotz. Bwahahaha!",
                        "But yeah, um, thanks. Not",
                        "that I'm grateful or anything."
                    ],
                )?;
                ctx.call(Function::Cutin, vec![Val::from("lhz_macu07"), Val::from(255)])?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(7351), Val::from(1)])?;
                ctx.var("friendship").set(Val::from(14))?;
                {
                    if ctx.var("BaseLevel").get()?.number()? > 90 {
                        ctx.call(Function::GetExperience, vec![Val::from(700000), Val::from(0)])?;
                    } else if ctx.var("BaseLevel").get()?.number()? > 75 {
                        ctx.call(Function::GetExperience, vec![Val::from(400000), Val::from(0)])?;
                    } else {
                        ctx.call(Function::GetExperience, vec![Val::from(200000), Val::from(0)])?;
                    }
                }
                ctx.call(Function::Cutin, vec![Val::from("lhz_macu04"), Val::from(2)])?;
                ctx.lines_as(
                    "Maku",
                    args![
                        "So, uh, I guess",
                        "I'll see you later.",
                        "Um, now I gotta get",
                        "ready for something.",
                        "^333333(But not to see Digotz!)^000000"
                    ],
                )?;
            } else {
                if (ctx.var("friendship").get()?.number()? > 5 && ctx.var("friendship").get()?.number()? < 13) {
                    ctx.call(Function::Cutin, vec![Val::from("lhz_macu06"), Val::from(2)])?;
                    ctx.lines(args![
                        "Arrrrgh! Whenever I hear",
                        "about that Digotz, I get so",
                        "peeved! Is that guy giving",
                        "me the brushoff just because",
                        "I'm not a rich guy like he is?!"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maku",
                        args![
                            "I can't...",
                            "I can't even beat him up",
                            "all properly because of",
                            "all those freakin' guards!",
                            "Arrrrrrrgh! Man, where's",
                            "Benkaistein when I need him?"
                        ],
                    )?;
                } else if ctx.var("friendship").get()? == 5 {
                    ctx.lines(args!["RrrrRrrrr....", "RrrrrRRRrrRR....", "GGGGGRRRRR..."])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFUh oh...",
                        "It looks like Maku",
                        "is starting to rage",
                        "just a bit too much.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args!["...", "......", "........."])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("lhz_macu06"), Val::from(2)])?;
                    ctx.lines_as("Maku", args!["Gggrrrr..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maku",
                        args![
                            "GRAAAAAAAAH~!",
                            "Who the hell does he",
                            "think he is, telling me",
                            "all sorts of crap!? Digotz,",
                            "you're not getting away",
                            "with this! Gonna wreck you!!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFMaku's seething quickly",
                        "explodes into pure, violent",
                        "rage. You manage to calm",
                        "him down after a while, but",
                        "barely keep yourself from",
                        "getting killed in this outburst.^000000"
                    ])?;
                    ctx.call(Function::PercentHeal, vec![Val::from(-50), Val::from(0)])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maku",
                        args![
                            "^333333*Pant pant* *Whew~*^000000",
                            "D-don't worry, I've got",
                            "a grip on myself now.",
                            "Thanks for not letting me",
                            "get too crazy. Times like",
                            "this, I really miss ^FF0000Benkaistein^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maku",
                        args![
                            "Benkaistein would always",
                            "make sure that I'd stay out",
                            "of fights. I really miss that",
                            "guy. Still, he ain't around..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("friendship").set(Val::from(6))?;
                    ctx.lines_as(
                        "Maku",
                        args![
                            "Damn those guards!",
                            "If they weren't there,",
                            "I could just go over and",
                            "kick Digotz's ass! I swear,",
                            "if it weren't for them...!"
                        ],
                    )?;
                } else if ctx.var("friendship").get()? == 4 {
                    ctx.call(Function::Cutin, vec![Val::from("lhz_macu05"), Val::from(2)])?;
                    ctx.lines(args![
                        "What the hell are you",
                        "still doing around here?",
                        "You must have better things",
                        "to do than talk to a ruffian",
                        "like me or that snobby and",
                        "totally prickish Digotz."
                    ])?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Speaking of which...")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Speaking of which...",
                            "I spoke to Digotz again.",
                            "He told me to give you a",
                            "message, but I'm not sure th--"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maku",
                        args![
                            "That no-good bastard",
                            "has a message for me?!",
                            "Oh, I'm soooo honored~",
                            "Tell me what that fink",
                            "has to say, line by line!"
                        ],
                    )?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    if l_input_s.clone() == "Hopeless bastard!" {
                        ctx.lines_as(
                            "Maku",
                            args![
                                "''Hopeless bastard?!''",
                                "Well, at least he had",
                                "the stomach to say that.",
                                "Through somebody else",
                                "anyway! What else'd he say?!"
                            ],
                        )?;
                        ctx.next()?;
                        let (input, status) = runtime::input_text(ctx, None, None)?;
                        l_input_s = input;
                        if l_input_s.clone() == "You're still a stubborn jerk!" {
                            ctx.lines_as(
                                "Maku",
                                args![
                                    "''Stubborn jerk?!''",
                                    "Takes one to know one,",
                                    "bastard! Why I oughta--",
                                    "Grrr! What'd he say next?!"
                                ],
                            )?;
                            ctx.next()?;
                            let (input, status) = runtime::input_text(ctx, None, None)?;
                            l_input_s = input;
                            if l_input_s.clone() == "You owe me at least 3 lunches!" {
                                ctx.lines_as(
                                    "Maku",
                                    args![
                                        "Three lunches?!",
                                        "I treated that guy to",
                                        "lunch, like, fifteen times!",
                                        "I tell you, the guy does not",
                                        "know the meaning of friendship!",
                                        "What else did that moron say?!"
                                    ],
                                )?;
                                ctx.next()?;
                                let (input, status) = runtime::input_text(ctx, None, None)?;
                                l_input_s = input;
                                if l_input_s.clone() == "Not to mention an apology!" {
                                    ctx.call(Function::Cutin, vec![Val::from("lhz_macu05"), Val::from(255)])?;
                                    ctx.lines_as(
                                        "Maku",
                                        args![
                                            "Me, apologize?!",
                                            "He should be on his hands",
                                            "and knees begging for my",
                                            "frickin' forgiveness! That...",
                                            "That selfish no-good stupid...",
                                            "W-what else did he tell you?!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    let (input, status) = runtime::input_text(ctx, None, None)?;
                                    l_input_s = input;
                                    if l_input_s.clone() == "But who cares what you think?!" {
                                        ctx.lines_as(
                                            "Maku",
                                            args![
                                                "''Who cares what I think?!''",
                                                "GRRRAAAH~!! Who cares",
                                                "what he thinks!! ^333333*Pant Pant*^000000",
                                                "I'm gonna murderlize that",
                                                "dumb creep! He can't possibly",
                                                "make me angrier than I am now!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        let (input, status) = runtime::input_text(ctx, None, None)?;
                                        l_input_s = input;
                                        if l_input_s.clone() == "I'm so goddamn happy without you!" {
                                            ctx.var("friendship").set(Val::from(5))?;
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_macu06"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Maku",
                                                args![
                                                    "That's it.",
                                                    "It's decided. The",
                                                    "next time I see Digotz,",
                                                    "I'm gonna plaster his",
                                                    "face all over the floor."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_macu06"), Val::from(255)])?;
                                            return Err(Stop::End);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ctx.lines_as(
                        "Maku",
                        args![
                            "Wha...? I dunno if Digotz",
                            "would say something like",
                            "that. You sure you heard",
                            "him carefully enough? It's",
                            "been a while, but I know",
                            "how Digotz talks, man."
                        ],
                    )?;
                } else if ctx.var("friendship").get()? == 3 {
                    ctx.call(Function::Cutin, vec![Val::from("lhz_macu03"), Val::from(2)])?;
                    ctx.lines(args![
                        "Man, I need to blow off some",
                        "steam! Sure, me and Digotz",
                        "were buds before and maybe",
                        "we might seem like friends now,",
                        "but not anymore, though we used",
                        "to be closer than this. Argh!"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maku",
                        args![
                            "Don't get me wrong, I don't",
                            "miss the guy or anything like",
                            "that and I don't feel sorry about",
                            "what happened. But if he ever",
                            "came to apologize to me, I'd",
                            "probably accept, you know."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maku",
                        args![
                            "Well, after thinking",
                            "about it, of course.",
                            "I mean, I'm not the one",
                            "holding a grudge. It's all",
                            "that guy's fault! Sheeeesh!"
                        ],
                    )?;
                } else if ctx.var("friendship").get()? == 2 {
                    ctx.lines(args![
                        "Hey, what are you",
                        "doing back over here?",
                        "I thought I recommended",
                        "going over to check out",
                        "Uptown Lighthalzen. This",
                        "place is pretty run-down..."
                    ])?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("I actually met Digotz and...")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.call(Function::Cutin, vec![Val::from("lhz_macu05"), Val::from(2)])?;
                    ctx.lines_as(
                        "Maku",
                        args![
                            "You what...?!",
                            "You saw my old pal,",
                            "Digotz?! Er, I mean,",
                            "Mister Alexander Digotz,",
                            "who usedta be my buddy,",
                            "but obviously not anymore."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("lhz_macu02"), Val::from(2)])?;
                    ctx.lines_as(
                        "Maku",
                        args![
                            "Sure, we were real close",
                            "at one time, but that was",
                            "too long ago. It's been a",
                            "long time since we hung",
                            "out and he probably hates",
                            "my penniless guts and..."
                        ],
                    )?;
                    ctx.var("friendship").set(Val::from(3))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(12001), Val::from(12002)])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maku",
                        args![
                            "Crud, just listen to",
                            "me, I sound like a wuss.",
                            "I don't miss Digotz! In fact,",
                            "I hate the guy, one hundred",
                            "percent! The next time I see",
                            "him, I'll beat him to a pulp!"
                        ],
                    )?;
                } else {
                    ctx.call(Function::Cutin, vec![Val::from("lhz_macu01"), Val::from(2)])?;
                    ctx.lines(args![
                        "Hey, you're one of",
                        "those adventurers, eh?",
                        "Welcome to the ghetto.",
                        "Nothing too adventurous",
                        "here, but hey, you can",
                        "explore all you want."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maku",
                        args![
                            "I don't know if you know,",
                            "but actually, the people who",
                            "live here ain't allowed to",
                            "explore this whole city. It's",
                            "kind of taboo to talk about,",
                            "but what do I care, right?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maku",
                        args![
                            "Yeah, basically the rich",
                            "people here are too afraid",
                            "of the poor people comin' to",
                            "see them, so the security in",
                            "this city is pretty tight! Those",
                            "upper class guys are trash..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maku",
                        args![
                            "I didn't used to think",
                            "this way. I actually used",
                            "to have a pretty rich friend",
                            "till I found out he's not all",
                            "I thought he was. That",
                            "moron! Why's he like that?!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maku",
                        args![
                            "Eh, forget about it.",
                            "Why am I even talking",
                            "about my personal life",
                            "to someone I just met",
                            "anyway? Sure, we all",
                            "do it, but still..."
                        ],
                    )?;
                    ctx.var("friendship").set(Val::from(1))?;
                    if ctx.call(Function::IsBeginQuest, vec![Val::from(12000)])? == 0 {
                        ctx.call(Function::SetQuest, vec![Val::from(12000)])?;
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        "Maku",
                        args![
                            "Well, when you get",
                            "bored of the ghetto,",
                            "you really oughta check",
                            "out the rich section of town.",
                            "I'm bitter, but I'll also admit",
                            "it's way nicer than this place."
                        ],
                    )?;
                }
            }
        }
    }
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn maku(ctx: &Ctx) -> Script {
    maku_body(ctx, Vec::new()).map(|_| ())
}

fn student_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("friendship").get()? == 7 || ctx.var("friendship").get()? == 8) {
        ctx.lines_as(
            "Joey Choryee",
            args![
                "This is a study area where",
                "you're not supposed to speak,",
                "walk or even breathe loudly.",
                "Still, students like Benkaistein can tune out the whole world",
                "when they study hard enough..."
            ],
        )?;
        ctx.next()?;
        ctx.var("friendship").set(Val::from(8))?;
        ctx.lines_as(
            "Joey Choryee",
            args![
                "Benkaistein...?",
                "He's in the north part",
                "of this room. He's a real",
                "nice guy, but a little anal.",
                "Well, he's too organized",
                "and he labels everything!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Joey Choryee",
        args![
            "Property damage. Huh.",
            "It has to do with lightning",
            "and fire and water and stuff",
            "like that? Here I thought it",
            "meant, I dunno, buildings",
            "getting wrecked or something."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn student(ctx: &Ctx) -> Script {
    student_body(ctx, Vec::new()).map(|_| ())
}

fn passionate_student_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 300
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("friendship").get()? == 15 {
        ctx.call(Function::Cutin, vec![Val::from("lhz_benkaistin01.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Benkaistein",
            args![
                "Were you able to bring",
                "my journal to Digotz and",
                "Maku? I'm pretty sure it'd",
                "remind them of all the good",
                "times we had. I know they",
                "sure can be stubborn..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Benkaistein",
            args![
                "Anyway, I really",
                "appreciate all your",
                "help. When I go back",
                "home, I look forward to",
                "seeing the two of them again."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Benkaistein",
            args![
                "Until then, I need to research,",
                "finish my thesis and accomplish",
                "my academic goals. Oh, please",
                "take this pass which will let you",
                "go back and forth between the",
                "rich and poor areas as my thanks."
            ],
        )?;
        ctx.next()?;
        ctx.var("friendship").set(Val::from(16))?;
        ctx.call(Function::CompleteQuest, vec![Val::from(12006)])?;
        {
            if ctx.var("BaseLevel").get()?.number()? > 90 {
                ctx.call(Function::GetExperience, vec![Val::from(700000), Val::from(0)])?;
            } else if ctx.var("BaseLevel").get()?.number()? > 75 {
                ctx.call(Function::GetExperience, vec![Val::from(400000), Val::from(0)])?;
            } else {
                ctx.call(Function::GetExperience, vec![Val::from(200000), Val::from(0)])?;
            }
        }
        ctx.call(Function::GetItem, vec![Val::from(7350), Val::from(1)])?;
        ctx.lines_as(
            "Benkaistein",
            args![
                "Anyway, I wish you",
                "safety in your travels,",
                "adventurer. When the three",
                "of us get together, I'll be",
                "sure to let you know~"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    }
    if (ctx.var("friendship").get()? == 11 && ctx.call(Function::CountItem, vec![Val::from(7351)])?.number()? > 0) {
        ctx.lines_as(
            "Benkaistein",
            args![
                "Aw nuts, this is",
                "taking much longer",
                "than I had expected.",
                "Now where did I put",
                "that thing? Hmmmm..."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_benkaistin02.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Benkaistein",
            args![
                "Oh, is that it?",
                "Did you find my",
                "journal? Quick, let",
                "me check. Yes, yes...",
                "This is it! Thank you",
                "for finding this for me!"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        ctx.lines_as(
            "Benkaistein",
            args![
                "Would you mind doing",
                "a favor for me? It'd be",
                "better if I talk to them",
                "myself, but I'm too busy",
                "working on this thesis..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Benkaistein",
            args![
                "Would you please give this",
                "journal to Digotz and Maku?",
                "I wrote in it when we were",
                "really young, so it should",
                "remind them of all the good",
                "times we used to share."
            ],
        )?;
        ctx.next()?;
        ctx.var("friendship").set(Val::from(12))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(12004), Val::from(12005)])?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_benkaistin04.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Benkaistein",
            args![
                "Anyway, this should at",
                "least help them realize",
                "how stupid they've been",
                "acting. Thanks in advance,",
                "and please take care of",
                "Maku and Digotz for me."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    }
    if (ctx.var("friendship").get()? == 10 || ctx.var("friendship").get()? == 11) {
        ctx.lines_as(
            "Benkaistein",
            args![
                "Aw nuts, this is",
                "taking much longer",
                "than I had expected.",
                "Now where did I put",
                "that thing? Hmmmm..."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFPerhaps it would",
            "be best if you help",
            "Benkaistein look for",
            "he is searching for.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("friendship").get()? == 9 {
        ctx.call(Function::Cutin, vec![Val::from("lhz_benkaistin03.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Passionate Student",
            args![
                "Oh, you startled me!",
                "Still, I'm aware that it's",
                "hard to get my attention",
                "once I immerse myself",
                "in a book. So, how can",
                "I help you, adventurer?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Tell him about Maku and Digotz.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_benkaistin02.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Benkaistein",
            args![
                "Oh, how are my friends",
                "doing? Oh, what? They're",
                "having a huge fight just",
                "because one's rich and",
                "the other one's poor?",
                "That's pretty childish!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Benkaistein",
            args![
                "But then again, that's just",
                "like them. *Sigh* I really want",
                "to go back home and get those",
                "two to make up, but I also need",
                "to finish this thesis. Let's see... What can I possibly do from here?"
            ],
        )?;
        ctx.next()?;
        ctx.var("friendship").set(Val::from(10))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(12003), Val::from(12004)])?;
        ctx.lines_as(
            "Benkaistein",
            args![
                "Oh, I know what I can do!",
                "Wait, but where did I put it?",
                "Oh, how could I lose something",
                "so important? Wait! Would you",
                "please wait a second while",
                "I look for something?"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    }
    if ctx.var("friendship").get()? == 8 {
        ctx.call(Function::Cutin, vec![Val::from("lhz_benkaistin04.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Passionate Student",
            args![
                "Let's see, now.",
                "Wind Magic, Black Magic,",
                "Porings, ah, there it is.",
                "Monster race properties.",
                "Hopefully this contains",
                "the information I need..."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFUpon briefly glancing at",
            "this student's belongings,",
            "you notice that the name",
            "''Benkaistein'' is printed",
            "on them. This is the friend",
            "mentioned by Maku and Digotz!^000000"
        ])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Excuse me...")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Excuse me...", "Benkaistein?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Passionate Student",
            args![
                "...The world of humans",
                "and the world of demons,",
                "yes, yes... No, what I'm",
                "looking for is a reference",
                "to the heavens or Asgard.",
                "Hmm, this here might help..."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Hey...")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Hey...", "Over here.", "Benkaistein!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Passionate Student",
            args![
                "...Oh, now that's a very",
                "interesting observation.",
                "If I can incorporate that",
                "into my thesis without too",
                "much trouble, my standpoint",
                "would look much more solid..."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("HEY YOU...!")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["HEY YOU...!", "BENKAISTEIN~!"],
        )?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_benkaistin02.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Passionate Student",
            args![
                "Oh, good heavens!",
                "C-can't you keep",
                "your voice down?",
                "I-I'm trying to study!",
                "No, wait. Have you been",
                "calling me all this time?"
            ],
        )?;
        ctx.var("friendship").set(Val::from(9))?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Passionate Student",
        args![
            "Let's see, now.",
            "Wind Magic, Black Magic,",
            "Porings, ah, there it is.",
            "Monster race properties.",
            "Hopefully this contains",
            "the information I need..."
        ],
    )?;
    ctx.next()?;
    ctx.lines(args![
        "^3355FFThis student seems to",
        "be dilligently conducting",
        "intensive research on some",
        "academic subject. For now,",
        "it would be best to leave him",
        "alone so that he can study.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn passionate_student(ctx: &Ctx) -> Script {
    passionate_student_body(ctx, Vec::new()).map(|_| ())
}

fn book_lhz_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("friendship").get()? == 11 {
        ctx.lines(args![
            "^3355FFThere's nothing",
            "over here that you",
            "really need anymore.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("friendship").get()? == 10 {
        ctx.var("friendship").set(Val::from(11))?;
        ctx.call(Function::GetItem, vec![Val::from(7351), Val::from(1)])?;
        ctx.lines(args![
            "^3355FFThis book is labeled,",
            "''Benkaistein's Journal",
            "Vol. 6.'' This is probably",
            "what Benkaistein was trying",
            "to find, so it might be best to",
            "bring this and show it to him.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFThere's nothing of",
        "any real interest",
        "over here for now.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn book_lhz(ctx: &Ctx) -> Script {
    book_lhz_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Kiz011Step {
    Start,
    OnTouch,
}

fn kiz01_1_run(ctx: &Ctx, mut step: Kiz011Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Kiz011Step::Start => {
                step = Kiz011Step::OnTouch;
                continue 'machine;
            }
            Kiz011Step::OnTouch => {
                if ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 0 {
                    if ctx.var("lhz_curse").get()? == 0 {
                        ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
                        ctx.var("lhz_curse").set(Val::from(1))?;
                        ctx.call(Function::SetQuest, vec![Val::from(2086)])?;
                    } else if ctx.var("lhz_curse").get()?.number()? < 26 {
                        ctx.var("@lhz_ghost")
                            .set(ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?)?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(1000), Val::from(0)],
                        )?;
                        ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
                        if ctx.var("@lhz_ghost").get()? == 1 {
                            ctx.lines_as("??????", args!["..................."])?;
                            ctx.next()?;
                            ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
                            ctx.lines_as("??????", args!["...elp....help..."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("@lhz_ghost").get()? == 4 {
                            ctx.lines_as("??????", args!["..................."])?;
                            ctx.next()?;
                            ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
                            ctx.lines_as("??????", args!["I...", "I despise the living."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn kiz01_1(ctx: &Ctx) -> Script {
    kiz01_1_run(ctx, Kiz011Step::Start, Vec::new()).map(|_| ())
}

pub fn kiz01_1_ontouch(ctx: &Ctx) -> Script {
    kiz01_1_run(ctx, Kiz011Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Kiz021Step {
    Start,
    OnTouch,
}

fn kiz02_1_run(ctx: &Ctx, mut step: Kiz021Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Kiz021Step::Start => {
                step = Kiz021Step::OnTouch;
                continue 'machine;
            }
            Kiz021Step::OnTouch => {
                if ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 0 {
                    if ctx.var("lhz_curse").get()? == 0 {
                        ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                        ctx.var("lhz_curse").set(Val::from(1))?;
                        ctx.call(Function::SetQuest, vec![Val::from(2086)])?;
                    } else if ctx.var("lhz_curse").get()?.number()? < 26 {
                        ctx.var("@lhz_ghost")
                            .set(ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?)?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(1000), Val::from(0)],
                        )?;
                        ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                        if ctx.var("@lhz_ghost").get()? == 1 {
                            ctx.lines_as("??????", args!["..................."])?;
                            ctx.next()?;
                            ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                            ctx.lines_as("??????", args!["...elp....help..."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("@lhz_ghost").get()? == 4 {
                            ctx.lines_as("??????", args!["..................."])?;
                            ctx.next()?;
                            ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                            ctx.lines_as("??????", args!["I...", "I despise the living."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn kiz02_1(ctx: &Ctx) -> Script {
    kiz02_1_run(ctx, Kiz021Step::Start, Vec::new()).map(|_| ())
}

pub fn kiz02_1_ontouch(ctx: &Ctx) -> Script {
    kiz02_1_run(ctx, Kiz021Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Kiz031Step {
    Start,
    OnTouch,
}

fn kiz03_1_run(ctx: &Ctx, mut step: Kiz031Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Kiz031Step::Start => {
                step = Kiz031Step::OnTouch;
                continue 'machine;
            }
            Kiz031Step::OnTouch => {
                if ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 0 {
                    if ctx.var("lhz_curse").get()? == 0 {
                        ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                        ctx.var("lhz_curse").set(Val::from(1))?;
                        ctx.call(Function::SetQuest, vec![Val::from(2086)])?;
                    } else if ctx.var("lhz_curse").get()?.number()? < 26 {
                        ctx.var("@lhz_ghost")
                            .set(ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?)?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(1000), Val::from(0)],
                        )?;
                        ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                        if ctx.var("@lhz_ghost").get()? == 1 {
                            ctx.lines_as("??????", args!["..................."])?;
                            ctx.next()?;
                            ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                            ctx.lines_as("??????", args!["...elp....help..."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("@lhz_ghost").get()? == 4 {
                            ctx.lines_as("??????", args!["..................."])?;
                            ctx.next()?;
                            ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                            ctx.lines_as("??????", args!["I...", "I despise the living."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn kiz03_1(ctx: &Ctx) -> Script {
    kiz03_1_run(ctx, Kiz031Step::Start, Vec::new()).map(|_| ())
}

pub fn kiz03_1_ontouch(ctx: &Ctx) -> Script {
    kiz03_1_run(ctx, Kiz031Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Kiz03Step {
    Start,
    OnTouch,
}

fn kiz03_run(ctx: &Ctx, mut step: Kiz03Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Kiz03Step::Start => {
                step = Kiz03Step::OnTouch;
                continue 'machine;
            }
            Kiz03Step::OnTouch => {
                if ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 0 {
                    if ctx.var("lhz_curse").get()? == 0 {
                        ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(1000), Val::from(0)],
                        )?;
                    } else if (ctx.var("lhz_curse").get()?.number()? > 0 && ctx.var("lhz_curse").get()?.number()? < 26) {
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_BLIND")?, Val::from(60000), Val::from(0)],
                        )?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "?????",
                            args![
                                "Honey, I'm sorry,",
                                "but I... We know how",
                                "hungry you are, but we",
                                "have nothing to feed you."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "?????",
                            args![
                                "*Sniff* I know, I know,",
                                "but somehow, that tragic",
                                "truth eats away at me just",
                                "a little bit more everyday..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "?????",
                            args![
                                "D-damn it...!",
                                "Why do we have",
                                "to live like this?!",
                                "It's like we're less",
                                "than animals. I hate this!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.call(
                            Function::Emotion,
                            vec![
                                ctx.constant("ET_QUESTION")?,
                                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                            ],
                        )?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Those voices weren't", "just in my head, were they?", "Hello...? Anybody there...?"],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::EndStatus, vec![ctx.constant("SC_BLIND")?])?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(5000), Val::from(0)],
                        )?;
                        if !(ctx.var("lhz_spi01").get()?.is_true()) {
                            ctx.var("lhz_spi01").set(Val::from(1))?;
                        }
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn kiz03(ctx: &Ctx) -> Script {
    kiz03_run(ctx, Kiz03Step::Start, Vec::new()).map(|_| ())
}

pub fn kiz03_ontouch(ctx: &Ctx) -> Script {
    kiz03_run(ctx, Kiz03Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Kiz04Step {
    Start,
    OnTouch,
}

fn kiz04_run(ctx: &Ctx, mut step: Kiz04Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Kiz04Step::Start => {
                step = Kiz04Step::OnTouch;
                continue 'machine;
            }
            Kiz04Step::OnTouch => {
                if ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 0 {
                    if ctx.var("lhz_curse").get()? == 0 {
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(1000), Val::from(0)],
                        )?;
                        ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                    } else if (ctx.var("lhz_curse").get()?.number()? > 0 && ctx.var("lhz_curse").get()?.number()? < 26) {
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_BLIND")?, Val::from(60000), Val::from(0)],
                        )?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.lines_as("?????", args!["Hey, you won't", "believe it! Rekenber", "decided to hire us!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "???????",
                            args![
                                "You sure that's so great?",
                                "Those big corporations",
                                "always take advantage",
                                "of the little guy. We're",
                                "probably gonna end up",
                                "slavin' away for our wages..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "?????",
                            args![
                                "No, that's not the",
                                "case at all. I mean,",
                                "sure, we won't start off",
                                "with much responsibility,",
                                "but they'll pay us well!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "?????",
                            args![
                                "Well, we do need to eat.",
                                "If they're true to their word,",
                                "that'll be even better!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.lines_as("???", args!["Waaaah!", "Waaaaaaah~!"])?;
                        ctx.next()?;
                        ctx.lines_as("????", args!["Woman, shut this", "baby up! Shut up, kid!"])?;
                        ctx.next()?;
                        ctx.lines_as("?????", args!["Honey, please...", "He's just a baby!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "????",
                            args!["I don't care how", "crummy this house is!", "It's mine and I want quiet!"],
                        )?;
                        ctx.next()?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.call(
                            Function::Emotion,
                            vec![
                                ctx.constant("ET_QUESTION")?,
                                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                            ],
                        )?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["I'm hearing things", "again! Where are all of", "these voices coming from?"],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                        ctx.call(Function::EndStatus, vec![ctx.constant("SC_BLIND")?])?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(5000), Val::from(0)],
                        )?;
                        if !(ctx.var("lhz_spi02").get()?.is_true()) {
                            ctx.var("lhz_spi02").set(Val::from(1))?;
                        }
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn kiz04(ctx: &Ctx) -> Script {
    kiz04_run(ctx, Kiz04Step::Start, Vec::new()).map(|_| ())
}

pub fn kiz04_ontouch(ctx: &Ctx) -> Script {
    kiz04_run(ctx, Kiz04Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Kiz05Step {
    Start,
    OnTouch,
}

fn kiz05_run(ctx: &Ctx, mut step: Kiz05Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Kiz05Step::Start => {
                step = Kiz05Step::OnTouch;
                continue 'machine;
            }
            Kiz05Step::OnTouch => {
                if ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 0 {
                    if ctx.var("lhz_curse").get()? == 0 {
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(1000), Val::from(0)],
                        )?;
                    } else if (ctx.var("lhz_curse").get()?.number()? > 0 && ctx.var("lhz_curse").get()?.number()? < 26) {
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_BLIND")?, Val::from(60000), Val::from(0)],
                        )?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "?????",
                            args![
                                "Mommy, why don't those",
                                "dirty people get new clothes?",
                                "Don't they know it's gross?",
                                "They're scaring me..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("???????", args!["Honey, don't look at", "them and hurry up!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "????",
                            args!["Please, do you have", "any spare change? I...", "I need something to eat..."],
                        )?;
                        ctx.next()?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["This is...", "This is insane!", "I must be hallucinating!"],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
                        ctx.call(Function::EndStatus, vec![ctx.constant("SC_BLIND")?])?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(3000), Val::from(0)],
                        )?;
                        if !(ctx.var("lhz_spi03").get()?.is_true()) {
                            ctx.var("lhz_spi03").set(Val::from(1))?;
                        }
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn kiz05(ctx: &Ctx) -> Script {
    kiz05_run(ctx, Kiz05Step::Start, Vec::new()).map(|_| ())
}

pub fn kiz05_ontouch(ctx: &Ctx) -> Script {
    kiz05_run(ctx, Kiz05Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Kiz06Step {
    Start,
    OnTouch,
}

fn kiz06_run(ctx: &Ctx, mut step: Kiz06Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Kiz06Step::Start => {
                step = Kiz06Step::OnTouch;
                continue 'machine;
            }
            Kiz06Step::OnTouch => {
                if ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 0 {
                    if ctx.var("lhz_curse").get()? == 0 {
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(1000), Val::from(0)],
                        )?;
                    } else if (ctx.var("lhz_curse").get()?.number()? > 0 && ctx.var("lhz_curse").get()?.number()? < 26) {
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_BLIND")?, Val::from(60000), Val::from(0)],
                        )?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "?????",
                            args![
                                "Listen, I know you're",
                                "the newest hire, but you've",
                                "shown us a lot of potential.",
                                "I think you'd be a perfect",
                                "fit for this new position."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("???????", args!["Are you really serious?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "?????",
                            args![
                                "I'll tell it to you straight.",
                                "This new position will require",
                                "you to be away from home once",
                                "in a while, but that comes with",
                                "the new responsibilities."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "?????",
                            args![
                                "But you'll receive",
                                "many new benefits that",
                                "I'm sure your family would",
                                "appreciate and you'll be",
                                "generously compensated."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("???????", args!["............."])?;
                        ctx.next()?;
                        ctx.lines_as("???????", args!["...Alright, I'm in."])?;
                        ctx.next()?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["............"])?;
                        ctx.close_window()?;
                        ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                        ctx.call(Function::EndStatus, vec![ctx.constant("SC_BLIND")?])?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(5000), Val::from(0)],
                        )?;
                        if !(ctx.var("lhz_spi04").get()?.is_true()) {
                            ctx.var("lhz_spi04").set(Val::from(1))?;
                        }
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn kiz06(ctx: &Ctx) -> Script {
    kiz06_run(ctx, Kiz06Step::Start, Vec::new()).map(|_| ())
}

pub fn kiz06_ontouch(ctx: &Ctx) -> Script {
    kiz06_run(ctx, Kiz06Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Kiz07Step {
    Start,
    OnTouch,
}

fn kiz07_run(ctx: &Ctx, mut step: Kiz07Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Kiz07Step::Start => {
                step = Kiz07Step::OnTouch;
                continue 'machine;
            }
            Kiz07Step::OnTouch => {
                if ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 0 {
                    if ctx.var("lhz_curse").get()? == 0 {
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(1000), Val::from(0)],
                        )?;
                    } else if (ctx.var("lhz_curse").get()?.number()? > 0 && ctx.var("lhz_curse").get()?.number()? < 26) {
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_BLIND")?, Val::from(60000), Val::from(0)],
                        )?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "?????",
                            args![
                                "So, what exactly did you",
                                "want me to do? I'm sorry,",
                                "I wasn't able to gather my",
                                "actual job function from",
                                "the presentation..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "???",
                            args![
                                "Hee hee hee~",
                                "Don't worry about it.",
                                "Just relax and stay",
                                "right where you are."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "?????",
                            args![
                                "Mmm. Alright.",
                                "I just thought that I was",
                                "supposed to go somewhere,",
                                "that's all. Um, shouldn't",
                                "I be getting ready...?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "???",
                            args![
                                "Oh, don't worry about",
                                "that, either. All of the",
                                "arrangements will be",
                                "taken care of. You'll",
                                "be taken to that far",
                                "off place very soon..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.mes("............")?;
                        ctx.close_window()?;
                        ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                        ctx.call(Function::EndStatus, vec![ctx.constant("SC_BLIND")?])?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(3000), Val::from(0)],
                        )?;
                        if ctx.var("lhz_curse").get()? == 6 {
                            ctx.var("lhz_curse").set(Val::from(7))?;
                        }
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn kiz07(ctx: &Ctx) -> Script {
    kiz07_run(ctx, Kiz07Step::Start, Vec::new()).map(|_| ())
}

pub fn kiz07_ontouch(ctx: &Ctx) -> Script {
    kiz07_run(ctx, Kiz07Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Kiz08Step {
    Start,
    OnTouch,
}

fn kiz08_run(ctx: &Ctx, mut step: Kiz08Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Kiz08Step::Start => {
                step = Kiz08Step::OnTouch;
                continue 'machine;
            }
            Kiz08Step::OnTouch => {
                if ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 0 {
                    if ctx.var("lhz_curse").get()? == 0 {
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(1000), Val::from(0)],
                        )?;
                    } else if (ctx.var("lhz_curse").get()?.number()? > 0 && ctx.var("lhz_curse").get()?.number()? < 26) {
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_BLIND")?, Val::from(60000), Val::from(0)],
                        )?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "????",
                            args![
                                "Oh, I'm so sorry to",
                                "hear that you want to",
                                "quit. Are you sure there",
                                "isn't anything we can do",
                                "to change your mind?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "???",
                            args![
                                "Well, I'm thinking of",
                                "retiring early and just",
                                "spending more time",
                                "with my family. But I won't",
                                "ever talk about the company",
                                "or any of its, well, secrets."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "????",
                            args![
                                "I really appreciate",
                                "your honesty and sincerity.",
                                "You've really done a great",
                                "job for us and I can't thank",
                                "you enough for your years of",
                                "dedication to this company."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("???", args!["Thank you, sir."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "????",
                            args![
                                "It would be a great loss",
                                "for this company to let you",
                                "go now. But I guess we have",
                                "no choice, since you've had",
                                "that horrible accident."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "???",
                            args!["Sir...?", "Accident...?", "I don't understand", "what you're talking ab--"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "???",
                            args![
                                "N-no! Please...!",
                                "I didn't, I swear on",
                                "my life that I won't ever",
                                "say anything about this",
                                "place! I'm begging you,",
                                "for the love of god!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "????",
                            args!["Your life...?", "Sorry, not good enough.", "You should have known."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "???",
                            args![
                                "Ssstop! I have chil--",
                                "MY LEEEEGS!! OH MY GOD",
                                "WHAT HAVE YOU DONE TO",
                                "MY LEGS?! HELP ME, OH MY G--"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.mes("............")?;
                        ctx.close_window()?;
                        ctx.call(Function::SoundEffect, vec![Val::from("tao_gunka_stand.wav"), Val::from(0)])?;
                        ctx.call(Function::EndStatus, vec![ctx.constant("SC_BLIND")?])?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(3000), Val::from(0)],
                        )?;
                        if ctx.var("lhz_curse").get()? == 11 {
                            ctx.var("lhz_curse").set(Val::from(12))?;
                        }
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn kiz08(ctx: &Ctx) -> Script {
    kiz08_run(ctx, Kiz08Step::Start, Vec::new()).map(|_| ())
}

pub fn kiz08_ontouch(ctx: &Ctx) -> Script {
    kiz08_run(ctx, Kiz08Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Kiz09Step {
    Start,
    OnTouch,
}

fn kiz09_run(ctx: &Ctx, mut step: Kiz09Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Kiz09Step::Start => {
                step = Kiz09Step::OnTouch;
                continue 'machine;
            }
            Kiz09Step::OnTouch => {
                if ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 0 {
                    if ctx.var("lhz_curse").get()? == 12 {
                        ctx.mes("............")?;
                        ctx.next()?;
                        ctx.mes("............")?;
                        ctx.call(Function::SoundEffect, vec![Val::from("loli_ruri_stand.wav"), Val::from(0)])?;
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(1000), Val::from(0)],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("lhz_que01"), Val::from(26), Val::from(27)])?;
                    } else if ((ctx.var("lhz_curse").get()?.number()? > 0 && ctx.var("lhz_curse").get()?.number()? < 12)
                        && (ctx.var("lhz_curse").get()?.number()? > 12 && ctx.var("lhz_curse").get()?.number()? < 26))
                    {
                        ctx.call(
                            Function::StartStatus,
                            vec![ctx.constant("SC_CURSE")?, Val::from(1000), Val::from(0)],
                        )?;
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn kiz09(ctx: &Ctx) -> Script {
    kiz09_run(ctx, Kiz09Step::Start, Vec::new()).map(|_| ())
}

pub fn kiz09_ontouch(ctx: &Ctx) -> Script {
    kiz09_run(ctx, Kiz09Step::OnTouch, Vec::new()).map(|_| ())
}
