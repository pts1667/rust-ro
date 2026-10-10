use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn lin_2_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mao_request").get()? == 103 {
        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Lin",
            args![
                "What...?",
                "I thought we're",
                "only supposed to",
                "protect you? What",
                "exactly is going on?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "R.",
            args!["I'm sorry, but it is", "imperative that I leave", "for the Juno Library..."],
        )?;
        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Excuse me, but", "what are you guys", "talking about...?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lin",
            args![
                ((Val::from("Ah, this is ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                "my partner for this assignment."
            ],
        )?;
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
            ctx.mes("Don't worry, you can trust him.")?;
        } else {
            ctx.mes("Don't worry, you can trust her.")?;
        }
        ctx.next()?;
        ctx.lines_as(
            "R.",
            args![
                "Ah, I see. Please,",
                "for the sake of anonymity,",
                "call me Mr. R. If it will help",
                "you in your task, you may want",
                "to ask Lin to explain everything that I just told her. Excuse me..."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFMr. R pulled out a pendant",
            "from his necklace, placed it",
            "against his forehead, and then",
            "quickly mumbled something",
            "before raising his head.^000000"
        ])?;
        ctx.var("mao_request").set(Val::from(104))?;
        ctx.close_window()?;
    } else {
        if ctx.var("mao_request").get()? == 104 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Um...",
                    "What was that?",
                    "Mr. R just took out",
                    "that strange pendant",
                    "and starting praying",
                    "or... something."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
            ctx.lines_as(
                "Lin",
                args![
                    "I guess he was praying...",
                    "Though whatever religion",
                    "it is, I've never heard of it.",
                    "Anyway, it's not a big deal.",
                    "Let me explain our current",
                    "objective for this assignment."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lin",
                args![
                    "Our client, Mr. R, was attacked",
                    "while researching important",
                    "documents in the Juno Library.",
                    "He escaped with his life, but",
                    "he wasn't able to bring all of",
                    "the documents along with him."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "R.",
                args![
                    "The information in those",
                    "documents is vital for my",
                    "research, and may provide",
                    "clues to finding those children",
                    "that have been missing from",
                    "Morocc. Please retrieve them!"
                ],
            )?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
            ctx.next()?;
            ctx.lines_as(
                "Lin",
                args![
                    "Well, I guess that's that.",
                    "I suppose that if we retrieve",
                    "those documents, we'll be one",
                    "step closer to finding those",
                    "lost kids. So let's do this."
                ],
            )?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Wait!"), Val::from("Won't they attack again?")])? {
                1 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Wait! It's too risky!",
                            "What if Mr. R is attacked",
                            "again? I mean, isn't that",
                            "why we're here? To protect",
                            "him from those attackers?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lin",
                        args![
                            "Good point. I guess",
                            "we should split up.",
                            "One of us will stay here",
                            "to guard Mr. R, and the other",
                            "will go to the Juno Library to",
                            "find his research documents."
                        ],
                    )?;
                }
                2 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Won't they attack", "again? Shouldn't both", "of us guard Mr. R here?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lin",
                        args![
                            "I agree with you, but",
                            "I think one of us should",
                            "be enough to ensure Mr. R's",
                            "safety. Besides, we should be",
                            "doing all that we can to save",
                            "those missing children."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "I think you're right.",
                            "But I think we should",
                            "see if that's alright",
                            "with Mr. R first."
                        ],
                    )?;
                }
                _ => {}
            }
            ctx.next()?;
            ctx.lines_as(
                "Lin",
                args![
                    "Okay, okay...",
                    "So what do you",
                    "think about that, Mr. R?",
                    "That alright with you?"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
            ctx.lines_as(
                "R.",
                args![
                    "Of course, I'd like my",
                    "safety to be guaranteed,",
                    "but the information in those",
                    "documents is more important",
                    "than my life. Do whatever it",
                    "takes--please get them back."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
            ctx.lines_as(
                "Lin",
                args![
                    "Well then. Mr. R, you'll",
                    "stay here where I can protect",
                    "you. This place is also shielded",
                    "with security magic, so you'll",
                    "be just fine. Now, as for you,",
                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("..."))
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lin",
                args![
                    "Your priority is to go",
                    "to the Juno Library and",
                    "bring Mr. R's documents",
                    "back over here. Hey, Mr. R,",
                    "how can we tell which are",
                    "your documents, anyway?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "R.",
                args![
                    "They're, um...",
                    "They're labeled with",
                    "the name, ''Moore.''",
                    "Just... another pseudonym."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lin",
                args![
                    "Once you're in the",
                    "Juno Library, find a",
                    "girl named ^4D4DFFYunia^000000. She'll",
                    "help make your job a lot",
                    "easier. Okay then, good luck~",
                    "Good luck!"
                ],
            )?;
            ctx.var("mao_request").set(Val::from(105))?;
            ctx.close_window()?;
        } else {
            if ctx.var("mao_request").get()? == 105 {
                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(0)])?;
                ctx.lines_as(
                    "Lin",
                    args![
                        "Head over to Juno",
                        "Library and speak to",
                        "a girl there named Yunia.",
                        "If anyone gets in your way,",
                        "just dispose of 'em. Punks",
                        "deserve whatever you give 'em!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lin",
                    args![
                        "Er, but that's completely",
                        "up to you. Just wanted to let",
                        "you know that you wouldn't get",
                        "in trouble for assassinating when you're working with Assassins",
                        "is all I'm saying. Okay, seeya~"
                    ],
                )?;
                ctx.close_window()?;
            } else {
                if ctx.var("mao_request").get()? == 106 {
                    ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(0)])?;
                    ctx.lines_as(
                        "Lin",
                        args!["Ah, were you able to", "find Yunia? Wait a sec,", "did you bring the documents?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Uh oh...", "I guess I better", "go back and talk", "to Yunia once again!"],
                    )?;
                    ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                    ctx.close_window()?;
                } else {
                    if ctx.var("mao_request").get()? == 107 {
                        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                        ctx.lines_as(
                            "Lin",
                            args![
                                "Yunia gave you the",
                                "documents that Mr. R",
                                "wanted? Great, now go",
                                "bring them to him and",
                                "see what he wants us",
                                "to do for him next."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(108))?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                        ctx.lines_as(
                            "R. Moore",
                            args![
                                "Ah yes, these are exactly",
                                "what I wanted! Thank you so",
                                "much for your hard work. Wow,",
                                "these are so well organized..."
                            ],
                        )?;
                        ctx.close_window()?;
                    } else {
                        if ctx.var("mao_request").get()? == 108 {
                            ctx.lines_as(
                                "Lin",
                                args![
                                    "Huh. Now that we brought",
                                    "him these documents, he's",
                                    "probably gonna ask us to",
                                    "do something else for him.",
                                    "That's how it always works..."
                                ],
                            )?;
                            ctx.close_window()?;
                        } else {
                            if ctx.var("mao_request").get()? == 109 {
                                ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                ctx.lines_as(
                                    "Lin",
                                    args![
                                        "I guess we should do as",
                                        "Mr. R asks. Go ahead and",
                                        "check the field west of Morocc",
                                        "for one of those artifacts or",
                                        "crests, and I'll remain here",
                                        "to guard Mr. R, just in case."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Lin",
                                    args![
                                        "I'm sorry that ^A9A9A9WAIT^000000 you've",
                                        "got to do ^A9A9A9FOR^000000 this ^A9A9A9ME^000000 alone,",
                                        "^A9A9A9AT^000000 but this situation can't",
                                        "be ^A9A9A9THE^000000 helped. ^A9A9A9STAIRS^000000"
                                    ],
                                )?;
                                ctx.close_window()?;
                            } else {
                                if (ctx.var("mao_request").get()?.number()? > 109 && ctx.var("mao_request").get()?.number()? < 115) {
                                    ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Lin",
                                        args![
                                            "I don't really know",
                                            "what's going on, but",
                                            "it looks like we have",
                                            "no choice but to do as",
                                            "Mr. R. Moore asks for now..."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                } else {
                                    if ctx.var("mao_request").get()? == 115 {
                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "Lin",
                                            args![
                                                "Hey... I need to talk to",
                                                "you alone. This R. Moore guy...",
                                                "I don't like him! Something's",
                                                "not adding up quite right!",
                                                "I've been going through his",
                                                "research notes, you know?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Lin",
                                            args![
                                                "There's all this mention",
                                                "of Morocc, Morocc Satan,",
                                                "and Thanatos Tower... How",
                                                "is this all connected to the",
                                                "missing children? If you ask",
                                                "me, it's pretty morbid stuff."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Lin",
                                            args![
                                                "Hey, I want you to find out",
                                                "what you can from our local",
                                                "historian in Morocc about Morocc Satan and all that. There's",
                                                "too much we don't know, and",
                                                "I don't wanna take chances."
                                            ],
                                        )?;
                                        ctx.var("mao_request").set(Val::from(116))?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                        ctx.lines_as(
                                            "R. Moore",
                                            args!["Hm...?", "What were you", "talking about so", "excitedly over there?"],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "Lin",
                                            args![
                                                "Oh, we were just",
                                                "talking about the those",
                                                "elemental crests and how",
                                                "they're now taken care of."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                        ctx.lines_as(
                                            "R. Moore",
                                            args![
                                                ((Val::from("Say, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                    + Val::from(",")),
                                                "were you about to leave?",
                                                "I had another reques--"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "Lin",
                                            args![
                                                ((Val::from("I'm sending ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                    + Val::from("")),
                                                "on a snack errand and it won't"
                                            ],
                                        )?;
                                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                            ctx.mes("take too long. Did you want him")?;
                                        } else {
                                            ctx.mes("take too long. Did you want her")?;
                                        }
                                        ctx.mes("to bring you back anything?")?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                        ctx.lines_as(
                                            "R. Moore",
                                            args![
                                                "Oh, yes. On your way",
                                                "back, would you bring",
                                                "me a Tropical Sograt?",
                                                "I'm very fond of those."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "Lin",
                                            args![
                                                ((Val::from("Well, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                    + Val::from(",")),
                                                "you heard the ^A9A9A9GO^000000 man.",
                                                "^A9A9A9TO^000000 Come back ^A9A9A9THE^000000 as soon",
                                                "as ^A9A9A9HISTORIAN^000000 can, ^A9A9A9NOW^000000 alright?"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                    } else {
                                        if ctx.var("mao_request").get()? == 116 {
                                            ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                            ctx.lines_as("Lin", args!["Back so soon?", "Did you forget what", "I asked you to do?"])?;
                                            ctx.next()?;
                                            ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                            ctx.lines_as("R. Moore", args!["You were going", "on a snack errand,", "weren't you?"])?;
                                            ctx.next()?;
                                            ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Lin",
                                                args![
                                                    "Right. I ^A9A9A9GO^000000 guess ^A9A9A9FIND^000000 you",
                                                    "^A9A9A9THE^000000 came back because you",
                                                    "^A9A9A9HISTORIAN^000000 forgot something. ",
                                                    "Now hurry ^A9A9A9IN^000000 up, we're going to",
                                                    "need ^A9A9A9Morocc^000000 snacks ^A9A9A9NOW^000000 soon."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                        } else {
                                            if ctx.var("mao_request").get()? == 117 {
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args!["Well, I'm back..."],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Lin",
                                                    args![
                                                        "I'm sorry, ^A9A9A9MEET^000000 partner,",
                                                        "but I forgot to ask you to",
                                                        "bring me a snack ^A9A9A9ME^000000 too.",
                                                        "Do you think you ^A9A9A9AT^000000 can",
                                                        "^A9A9A9THE^000000 get me a glass of, um...",
                                                        "^A9A9A9STAIRS^000000 Vermilion on the Beach?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args![
                                                        "Snack? Um... That's",
                                                        "more of a drink, isn't it?",
                                                        "But if that's what you want,",
                                                        "then I'll be back soon.",
                                                        "(^333333This is so cloak and dagger!",
                                                        "How does she talk like that?^000000)"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                            } else if ctx.var("mao_request").get()? == 118 {
                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                                ctx.lines_as("Lin", args!["Oh, good.", "You're finally back~"])?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                                ctx.lines_as(
                                                    "R. Moore",
                                                    args![
                                                        "Ah! You've finally returned.",
                                                        "I have another request that",
                                                        "I must ask of you that pertains",
                                                        "to the missing children."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Lin",
                                                    args![
                                                        "Yeah, alright.",
                                                        "Depending on what",
                                                        "it is, maybe we'll do it.",
                                                        "But if it's some crazy",
                                                        "nonsense errand, we won't."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                                ctx.lines_as(
                                                    "R. Moore",
                                                    args![
                                                        "For the sake of my",
                                                        "research, I want you to",
                                                        "investigate Thanatos Tower.",
                                                        "Learn more about its origin",
                                                        "through any means possible. I didn't want to say this, but..."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "R. Moore",
                                                    args![
                                                        "I think it's possible that the",
                                                        "missing children were kidnapped",
                                                        "to revive an ancient evil, Satan Morocc. I believe we can learn",
                                                        "more about the kidnappers by",
                                                        "learning about Satan Morocc."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "R. Moore",
                                                    args![
                                                        "I know this all sounds",
                                                        "crazy, but please trust me.",
                                                        "Thanatos Tower is somehow",
                                                        "related to Satan Morocc, so",
                                                        "if you could tell me what",
                                                        "you can learn from there..."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Lin",
                                                    args![
                                                        "Thanatos Tower, huh?",
                                                        "Alright, it's not like we don't",
                                                        "believe you, but we need some",
                                                        "time to consider your request",
                                                        "before we can go ahead with the investigation you're asking for..."
                                                    ],
                                                )?;
                                                ctx.var("mao_request").set(Val::from(119))?;
                                                ctx.close_window()?;
                                            } else if ctx.var("mao_request").get()? == 119 {
                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Lin",
                                                    args![
                                                        "Alright... Before",
                                                        "we go and do this, we",
                                                        "need to make absolutely",
                                                        "sure of a few things."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                                ctx.lines_as("R. Moore", args!["Of course.", "What is it that", "you want to ask me?"])?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Lin",
                                                    args![
                                                        "First, those men that",
                                                        "attacked you. Are they",
                                                        "trying to revive Satan",
                                                        "Morocc? And those crests...",
                                                        "Were they built to break Satan",
                                                        "Morocc's seal or protect it?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                                                ctx.lines_as(
                                                    "R. Moore",
                                                    args![
                                                        "Oh, my attackers. I have no",
                                                        "idea what they could want.",
                                                        "As for the elemental crests,",
                                                        "they were originally built by",
                                                        "a madman to revive Satan Morocc. "
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "R. Moore",
                                                    args![
                                                        "However, once that crazed",
                                                        "man's plot was discovered,",
                                                        "the crests were modified to",
                                                        "further shield the seal that",
                                                        "contains Satan Morocc."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "R. Moore",
                                                    args![
                                                        "But as a historian...",
                                                        "These stories about Satan",
                                                        "Morocc and seals are just",
                                                        "conjecture. I need concrete",
                                                        "proof that the threat of",
                                                        "Satan Morocc truly exists."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "R. Moore",
                                                    args![
                                                        "If I can obtain that,",
                                                        "people would be able to",
                                                        "take Satan Morocc's threat",
                                                        "more seriously. And Thanatos Tower wouldn't be a tourist area."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "R. Moore",
                                                    args![
                                                        "I'm begging you, please",
                                                        "go to Thanatos Tower and",
                                                        "try to find some sort of solid",
                                                        "historical record that proves",
                                                        "that Satan Morocc really",
                                                        "existed in our world."
                                                    ],
                                                )?;
                                                ctx.var("mao_request").set(Val::from(120))?;
                                                ctx.close_window()?;
                                            } else if ctx.var("mao_request").get()? == 120 {
                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Lin",
                                                    args![
                                                        "I'm not sure what to",
                                                        "believe, but investigating",
                                                        "Thanatos Tower seems to be",
                                                        "our best course of action now."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Lin",
                                                    args![
                                                        "While you work on that,",
                                                        "I will be trying to piece",
                                                        "together the information that",
                                                        "we've collected so far, as well",
                                                        "as keep an eye on Mr. R. Moore.",
                                                        "Good luck, and be careful."
                                                    ],
                                                )?;
                                                ctx.var("mao_request").set(Val::from(121))?;
                                                ctx.close_window()?;
                                            } else if ctx.var("mao_request").get()? == 121 {
                                                ctx.lines(args![
                                                    "^3355FFLin seems to be lost",
                                                    "in thought, carefully",
                                                    "weighing your mission's",
                                                    "options. For now, you",
                                                    "better do your part and",
                                                    "investigate Thanatos Tower.^000000"
                                                ])?;
                                                ctx.close_window()?;
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
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn lin_2_1(ctx: &Ctx) -> Script {
    lin_2_1_body(ctx, Vec::new()).map(|_| ())
}

fn r_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mao_request").get()? == 103 {
        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
        ctx.lines_as(
            "R.",
            args!["Right now, my life isn't", "so important! Please let", "me finish my research!"],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
        ctx.lines_as(
            "Lin",
            args![
                "What...?",
                "I thought we're",
                "only supposed to",
                "protect you? What",
                "exactly is going on?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "R.",
            args!["I'm sorry, but it is", "imperative that I leave", "for the Juno Library..."],
        )?;
        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Excuse me, but", "what are you guys", "talking about...?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lin",
            args![
                ((Val::from("Ah, this is ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                "my partner for this assignment."
            ],
        )?;
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
            ctx.mes("Don't worry, you can trust him.")?;
        } else {
            ctx.mes("Don't worry, you can trust her.")?;
        }
        ctx.next()?;
        ctx.lines_as(
            "R.",
            args![
                "Ah, I see. Please, for",
                "for the sake of anonymity,",
                "call me Mr. R. If it will help",
                "you in your task, you may want",
                "to ask Lin to explain everything that I just told her. Excuse me..."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFMr. R pulled out a pendant",
            "from his necklace, placed it",
            "against his forehead, and then",
            "quickly mumbled something",
            "before raising his head.^000000"
        ])?;
        ctx.var("mao_request").set(Val::from(104))?;
        ctx.close_window()?;
    } else {
        if ctx.var("mao_request").get()? == 104 {
            ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
            ctx.lines_as(
                "R.",
                args![
                    "Your partner should be",
                    "able to relate everything",
                    "that I've already told her,",
                    "as well as any details that",
                    "you might need to know for",
                    "your job. Please excuse me..."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFMr. R pulled out a pendant",
                "from his necklace, placed it",
                "against his forehead, and then",
                "quickly mumbled something",
                "before raising his head.^000000"
            ])?;
            ctx.close_window()?;
        } else {
            if ctx.var("mao_request").get()? == 105 {
                ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                ctx.lines_as(
                    "R.",
                    args![
                        "The Juno Library is",
                        "divided into 3 sections:",
                        "2 sections are used to store",
                        "books, and the other section",
                        "is dedicated for use as a",
                        "reading room or study hall."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "R.",
                    args![
                        "You should be able to",
                        "find Yunia in that study",
                        "hall. Hopefully, by that",
                        "time, she'll be finished",
                        "organizing my documents..."
                    ],
                )?;
                ctx.close_window()?;
            } else {
                if ctx.var("mao_request").get()? == 106 {
                    ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                    ctx.lines_as(
                        "R. Moore",
                        args![
                            "So were you able to find",
                            "Yunia and get my research",
                            "documents from her? If you've",
                            "forgotten, then please go back",
                            "to Juno Library and bring them",
                            "back here... Thanks again."
                        ],
                    )?;
                    ctx.close_window()?;
                } else {
                    if ctx.var("mao_request").get()? == 107 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["These are the documents", "that I received from Yunia.", "Is this what you needed?"],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                        ctx.lines_as(
                            "R. Moore",
                            args![
                                "Ah yes, these are exactly",
                                "what I wanted! Thank you so",
                                "much for your hard work. Wow,",
                                "these are so well organized..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                        ctx.lines_as(
                            "Lin",
                            args![
                                "Alright. You need to tell",
                                "us how your research is",
                                "related to our next job...",
                                "We need to protect you, but",
                                "we still want to help those",
                                "missing children if we can."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(108))?;
                        ctx.close_window()?;
                    } else {
                        if ctx.var("mao_request").get()? == 108 {
                            ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                            ctx.lines_as(
                                "R. Moore",
                                args![
                                    "The information in these",
                                    "documents is related to my",
                                    "next request for you. As your",
                                    "client, I hope you will carry",
                                    "out this task and consider it",
                                    "part of your assignment."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                            ctx.lines_as(
                                "Lin",
                                args![
                                    "Yeah, yeah, you don't",
                                    "have to remind me. Besides,",
                                    "we Assassins don't like leaving",
                                    "any job unfinished. So what do",
                                    "you need us to do this time?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                            ctx.lines_as(
                                "R. Moore",
                                args![
                                    "There are four crests",
                                    "hidden in four different",
                                    "directions around Morocc",
                                    "which regulate the power",
                                    "of the Water, Wind, Earth",
                                    "and Fire elements in the area."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "R. Moore",
                                args![
                                    "The stability of these",
                                    "elements in Morocc is in",
                                    "currently in question, so",
                                    "I want you to ^4D4DFFuse enchanted",
                                    "stones^000000 to strengthen the power",
                                    "of these elemental crests."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                            ctx.lines_as(
                                "Lin",
                                args![
                                    "Wait... We can honor",
                                    "requests regarding your",
                                    "safety, or anything that",
                                    "will help us find those",
                                    "missing kids, but we're your",
                                    "bodyguards, not your servants."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                            ctx.lines_as(
                                "R. Moore",
                                args![
                                    "I know, I know...",
                                    "But whatever force has",
                                    "been tampering with these",
                                    "artifacts is probably responsible for kidnapping those children.",
                                    "It's worth investigating..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "R. Moore",
                                args![
                                    "Regardless, those men",
                                    "attacked me because they",
                                    "did not want me to repair",
                                    "Morocc's elemental equilibrium.",
                                    "At least, that's what I believe."
                                ],
                            )?;
                            ctx.var("mao_request").set(Val::from(109))?;
                            ctx.next()?;
                            ctx.lines_as(
                                "R. Moore",
                                args![
                                    "You can't possibly",
                                    "understand now, but",
                                    "trust me... You don't want",
                                    "those crests to weaken."
                                ],
                            )?;
                            ctx.close_window()?;
                        } else {
                            if ctx.var("mao_request").get()? == 109 {
                                ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                ctx.lines_as(
                                    "R. Moore",
                                    args![
                                        "Why don't you search West",
                                        "Morocc first for one of the",
                                        "elemental artifacts since it",
                                        "is closer? It should be where",
                                        "the pyramids are located."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Lin",
                                    args![
                                        "Please go ahead",
                                        "and do as he says.",
                                        "In the meantime, I'll",
                                        "stay here and make sure",
                                        "no one attacks Mr. R again."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lin",
                                    args![
                                        "I'm sorry that ^A9A9A9WAIT^000000 you've",
                                        "got to do ^A9A9A9FOR^000000 this ^A9A9A9ME^000000 alone,",
                                        "^A9A9A9AT^000000 but this situation can't",
                                        "be ^A9A9A9THE^000000 helped. ^A9A9A9STAIRS^000000"
                                    ],
                                )?;
                                ctx.close_window()?;
                            } else {
                                if (ctx.var("mao_request").get()?.number()? > 109 && ctx.var("mao_request").get()?.number()? < 112) {
                                    ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                                    ctx.lines_as(
                                        "R. Moore",
                                        args![
                                            "You should be able to find",
                                            "the element regulation device",
                                            "behind one of the pyramids in",
                                            "the field west of Morocc. Please",
                                            "use an enchanted stone to",
                                            "boost the crest's power."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                } else {
                                    if ctx.var("mao_request").get()? == 112 {
                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                                        ctx.lines_as(
                                            "R. Moore",
                                            args![
                                                "Alright, now, I don't know",
                                                "the locations of the other",
                                                "crests. I wish I had the",
                                                "chance to search for them",
                                                "before I was attacked. I'm",
                                                "sorry I can't be of more help."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "R. Moore",
                                            args![
                                                "I didn't know asking for",
                                                "protection would require",
                                                "me to live like a prisoner...",
                                                "I feel... I feel trapped."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "Lin",
                                            args![
                                                "Hey, what's your problem,",
                                                "Mr. R?! You're snug as a rug",
                                                "in here, next to freakin' pub",
                                                "for goodness sake, while we're",
                                                "running around doing all of",
                                                "your dirty work for you!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                        ctx.lines_as("R. Moore", args!["This much is", "true, I suppose..."])?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(255)])?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args![
                                                "Well, I guess I'll go out",
                                                "and try to activate those",
                                                "other elemental crests",
                                                "just outside of Morocc."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                    } else {
                                        if (ctx.var("mao_request").get()?.number()? > 112 && ctx.var("mao_request").get()?.number()? < 115)
                                        {
                                            ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                                            ctx.lines_as(
                                                "R. Moore",
                                                args![
                                                    "Thank you so much",
                                                    "for all of your hard",
                                                    "work. Future generations",
                                                    "will praise what you have",
                                                    "done. Rest assured..."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Lin", args!["What...?", "What did you", "say just now?"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "R. Moore",
                                                args![
                                                    "Oh nothing...",
                                                    ((Val::from("Just giving ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                        + Val::from("")),
                                                    "due credit. That's all."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                        } else {
                                            if ctx.var("mao_request").get()? == 115 {
                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                                ctx.lines_as(
                                                    "R. Moore",
                                                    args![
                                                        "Good, you've arrived.",
                                                        "Now, to ensure the safety",
                                                        "of all of those poor children,",
                                                        "I need you to complete ano--"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Lin",
                                                    args![
                                                        "Excuse me, I don't mean",
                                                        "to butt in, but I need to",
                                                        "talk to you alone. It's",
                                                        "a very urgent issue that",
                                                        "I need to discuss here",
                                                        ((Val::from("with ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                            + Val::from("."))
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                                ctx.lines_as(
                                                    "R. Moore",
                                                    args!["...Very well.", "Then I shall next;", "until you return."],
                                                )?;
                                                ctx.close_window()?;
                                            } else {
                                                if ctx.var("mao_request").get()? == 116 {
                                                    ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                                    ctx.lines_as(
                                                        "R. Moore",
                                                        args![
                                                            "Hmmm...?",
                                                            "Weren't you going",
                                                            "out on an errand to",
                                                            "pick up some snacks",
                                                            "and drinks for us?"
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                } else {
                                                    if ctx.var("mao_request").get()? == 117 {
                                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                                        ctx.lines_as(
                                                            "R. Moore",
                                                            args![
                                                                "Oh good, you've",
                                                                "returned. Never mind",
                                                                "the snacks, I want to",
                                                                "talk to you about my",
                                                                "next request if you wo--"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                                        ctx.lines_as(
                                                            "Lin",
                                                            args![
                                                                "I'm sorry, ^A9A9A9MEET^000000 partner,",
                                                                "but I forgot to ask you to",
                                                                "bring me a snack ^A9A9A9ME^000000 too.",
                                                                "Do you think you ^A9A9A9AT^000000 can",
                                                                "^A9A9A9THE^000000 get me a glass of, um...",
                                                                "^A9A9A9STAIRS^000000 Vermilion on the Beach?"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args![
                                                                "Snack? Um... That's",
                                                                "more of a drink, isn't it?",
                                                                "But if that's what you want,",
                                                                "then I'll be back soon.",
                                                                "(^333333This is so cloak and dagger!",
                                                                "How does she talk like that?^000000)"
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                    } else if ctx.var("mao_request").get()? == 118 {
                                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                                        ctx.lines_as(
                                                            "R. Moore",
                                                            args![
                                                                "Ah, you've finally returned.",
                                                                "I have another request that",
                                                                "I must ask of you that pertains",
                                                                "to the missing children."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                                        ctx.lines_as(
                                                            "Lin",
                                                            args![
                                                                "Yeah, alright.",
                                                                "Depending on what",
                                                                "it is, maybe we'll do it.",
                                                                "But if it's some crazy",
                                                                "nonsense errand, we won't."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(0)])?;
                                                        ctx.lines_as(
                                                            "R. Moore",
                                                            args![
                                                                "For the sake of my",
                                                                "research, I want you to",
                                                                "investigate Thanatos Tower.",
                                                                "Learn more about its origin",
                                                                "through any means possible. I didn't want to say this, but..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "R. Moore",
                                                            args![
                                                                "I think it's possible that the",
                                                                "missing children were kidnapped",
                                                                "to revive an ancient evil, Satan Morocc. I believe we can learn",
                                                                "more about the kidnappers by",
                                                                "learning about Satan Morocc."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "R. Moore",
                                                            args![
                                                                "I know this all sounds",
                                                                "crazy, but please trust me.",
                                                                "Thanatos Tower is somehow",
                                                                "related to Satan Morocc, so",
                                                                "if you could tell me what",
                                                                "you can learn from there..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                                                        ctx.lines_as(
                                                            "Lin",
                                                            args![
                                                                "Thanatos Tower, huh?",
                                                                "Alright, it's not like we don't",
                                                                "believe you, but we need some",
                                                                "time to consider your request",
                                                                "before we can go ahead with the investigation you're asking for..."
                                                            ],
                                                        )?;
                                                        ctx.var("mao_request").set(Val::from(119))?;
                                                        ctx.close_window()?;
                                                    } else if ctx.var("mao_request").get()? == 119 {
                                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                                                        ctx.lines_as(
                                                            "R. Moore",
                                                            args![
                                                                "Lin has something to ask",
                                                                "me, so I should probably",
                                                                "answer her questions before",
                                                                "requesting you to do anything."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                    } else if ctx.var("mao_request").get()? == 120 {
                                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                                                        ctx.lines_as(
                                                            "R. Moore",
                                                            args![
                                                                "I'm begging you, please",
                                                                "go to Thanatos Tower and",
                                                                "try to find some sort of solid",
                                                                "historical record that proves",
                                                                "that Satan Morocc really",
                                                                "existed in our world."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                    } else if ctx.var("mao_request").get()? == 121 {
                                                        ctx.call(Function::Cutin, vec![Val::from("mocseal_dan01.bmp"), Val::from(1)])?;
                                                        ctx.lines_as(
                                                            "R. Moore",
                                                            args![
                                                                "You'll be investigating",
                                                                "Thanatos Tower, then?",
                                                                "Good, good, I'll soon be",
                                                                "able to complete my work."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines(args![
                                                            "^3355FFMr. R pulled out a pendant",
                                                            "from his necklace, placed it",
                                                            "against his forehead, and then",
                                                            "quickly mumbled something",
                                                            "before raising his head.^000000"
                                                        ])?;
                                                        ctx.close_window()?;
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
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn r(ctx: &Ctx) -> Script {
    r_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Lin3Step {
    Start,
    OnInit,
    OnEnter,
}

fn lin_3_run(ctx: &Ctx, mut step: Lin3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Lin3Step::Start => {
                step = Lin3Step::OnInit;
                continue 'machine;
            }
            Lin3Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Lin#3")])?;
                return Err(Stop::End);
            }
            Lin3Step::OnEnter => {
                ctx.call(Function::DisableNpc, vec![Val::from("Lin#3")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn lin_3(ctx: &Ctx) -> Script {
    lin_3_run(ctx, Lin3Step::Start, Vec::new()).map(|_| ())
}

pub fn lin_3_oninit(ctx: &Ctx) -> Script {
    lin_3_run(ctx, Lin3Step::OnInit, Vec::new()).map(|_| ())
}

pub fn lin_3_onenter(ctx: &Ctx) -> Script {
    lin_3_run(ctx, Lin3Step::OnEnter, Vec::new()).map(|_| ())
}

fn kidd_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
    if ctx.var("mao_request").get()?.number()? < 3 {
        ctx.lines_as("Kidd", args!["...", "......", "Um. Hey."])?;
        ctx.close_window()?;
    } else if (ctx.var("mao_request").get()?.number()? > 102 && ctx.var("mao_request").get()?.number()? < 126) {
        ctx.lines_as(
            "Kidd",
            args![
                "Oh hey, you're the one who's",
                "working with Lin, right? We're",
                "all glad you came on board for",
                "that mission. I mean, we can't do everything ourselves, you know?"
            ],
        )?;
        ctx.close_window()?;
    } else if ctx.var("mao_request").get()? == 3 {
        ctx.lines_as(
            "Kidd",
            args![
                "The representative from",
                "the Dandelion organization",
                "is waiting for us in the hall,",
                "so you better hustle over to",
                "him. Don't worry, I'll meet you",
                "as soon as you get there."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::DisableNpc, vec![Val::from("Kidd#1")])?;
    } else if (ctx.var("mao_request").get()? == 28 || ctx.var("mao_request").get()? == 29) {
        ctx.lines_as("Kidd", args!["Oh, hey...", "You're here,", "you really came back."])?;
        ctx.next()?;
        ctx.lines_as("Valdes", args!["Excuse me...", "But can we", "talk for a second?"])?;
        ctx.close_window()?;
    } else if (ctx.var("mao_request").get()? == 126 || ctx.var("mao_request").get()? == 127) {
        ctx.lines_as("Kidd", args!["Oh, hey...", "You're here,", "you really came back."])?;
        ctx.next()?;
        ctx.lines_as("Valdes", args!["Excuse me...", "But can we", "talk for a second?"])?;
        ctx.close_window()?;
    }
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn kidd_1(ctx: &Ctx) -> Script {
    kidd_1_body(ctx, Vec::new()).map(|_| ())
}

fn kidd_1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Kidd#1")])?;
    return Err(Stop::End);
}

pub fn kidd_1_oninit(ctx: &Ctx) -> Script {
    kidd_1_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn kidd_1_onenter_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Kidd#1")])?;
    return Err(Stop::End);
}

pub fn kidd_1_onenter(ctx: &Ctx) -> Script {
    kidd_1_onenter_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Kidd2Step {
    Start,
    OnInit,
    OnEnter,
}

fn kidd_2_run(ctx: &Ctx, mut step: Kidd2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Kidd2Step::Start => {
                step = Kidd2Step::OnInit;
                continue 'machine;
            }
            Kidd2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Kidd#2")])?;
                return Err(Stop::End);
            }
            Kidd2Step::OnEnter => {
                ctx.call(Function::DisableNpc, vec![Val::from("Kidd#2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn kidd_2(ctx: &Ctx) -> Script {
    kidd_2_run(ctx, Kidd2Step::Start, Vec::new()).map(|_| ())
}

pub fn kidd_2_oninit(ctx: &Ctx) -> Script {
    kidd_2_run(ctx, Kidd2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn kidd_2_onenter(ctx: &Ctx) -> Script {
    kidd_2_run(ctx, Kidd2Step::OnEnter, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Kidd3Step {
    Start,
    OnInit,
    OnEnter,
}

fn kidd_3_run(ctx: &Ctx, mut step: Kidd3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Kidd3Step::Start => {
                step = Kidd3Step::OnInit;
                continue 'machine;
            }
            Kidd3Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Kidd#3")])?;
                return Err(Stop::End);
            }
            Kidd3Step::OnEnter => {
                ctx.call(Function::DisableNpc, vec![Val::from("Kidd#3")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn kidd_3(ctx: &Ctx) -> Script {
    kidd_3_run(ctx, Kidd3Step::Start, Vec::new()).map(|_| ())
}

pub fn kidd_3_oninit(ctx: &Ctx) -> Script {
    kidd_3_run(ctx, Kidd3Step::OnInit, Vec::new()).map(|_| ())
}

pub fn kidd_3_onenter(ctx: &Ctx) -> Script {
    kidd_3_run(ctx, Kidd3Step::OnEnter, Vec::new()).map(|_| ())
}
