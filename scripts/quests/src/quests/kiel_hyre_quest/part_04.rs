use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn kiel_hyre_kh_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khqread = Val::from(0);
    let mut l_khstitle_s = Val::from("");
    let mut l_khtitle_s = Val::from("");
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(200)])? == 0 {
        ctx.lines(args![
            "^3355FFJust a second...",
            "You're carrying too",
            "many items with you",
            "right now, so you'll",
            "need to free up more",
            "inventory space first...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(Function::Cutin, vec![Val::from("kh_kyel01"), Val::from(2)])?;
    if ctx.var("kielhyrequest").get()?.number()? < 46 {
        ctx.lines_as("Kiel Hyre", args![".........", ".........", "............"])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    } else {
        if ctx.var("kielhyrequest").get()? == 46 {
            ctx.call(Function::Cutin, vec![Val::from("kh_kyel01"), Val::from(2)])?;
            ctx.lines_as(
                "Kiel Hyre",
                args![
                    ((Val::from("Ah, you must be ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                    "I'd like to thank you for saving",
                    "my life. You must have many",
                    "questions to ask me, so I'll",
                    "do my best to give you answers."
                ],
            )?;
            ctx.next()?;
            'l1: loop {
                if !(true) {
                    break 'l1;
                }
                'b1: {
                    match runtime::select_values(ctx, &[Val::from("Robots?:^3355FFKiehl^000000?:^FF0000Elly^000000's button?")])? {
                        1 => {
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "I've been researching",
                                    "robotics for thirty-two",
                                    "years now. I'm proud to",
                                    "say that I've succeeded",
                                    "where the great Sage",
                                    "Varmundt did not."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "It's been my dream to",
                                    "develop humanoid robots",
                                    "from humans. Those Guardians",
                                    "might be robots too, but they",
                                    "don't operate using free will."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "If you'd like to know more",
                                    "of the specifics concerning",
                                    "robotics, why don't you speak",
                                    "with ^3355FFAllysia^000000? She can explain",
                                    "everything much more succinctly",
                                    "than I can. I tend to ramble..."
                                ],
                            )?;
                            l_khqread = (l_khqread.clone() + Val::from(1));
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "^3355FFKiehl^000000 is my only son,",
                                    "but the love of my life",
                                    "died after giving birth to",
                                    "him. I'll admit that he's",
                                    "a genius in mechanical",
                                    "design and development."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "He's largely responsible",
                                    "for the creation of Third",
                                    "Generation robots like Elly.",
                                    "Unfortunately, he's trying to",
                                    "modify his creations for",
                                    "some sinister purpose."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "I tried to stop him,",
                                    "but I ended up getting",
                                    "locked inside the factory.",
                                    "I don't know why he wants",
                                    "to do this. I still have",
                                    "absolutely no clue..."
                                ],
                            )?;
                            l_khqread = (l_khqread.clone() + Val::from(1));
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "That button Elly was",
                                    "holding has ^3355FFKiehl's emblem^000000",
                                    "engraved on it. Ah, and that",
                                    "man in black menacing the",
                                    "students? That was probably",
                                    "^3355FFKaiser^000000, Kiehl's bodyguard."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Kaiser...",
                                    "I don't know",
                                    "anything about him.",
                                    "I've never even seen",
                                    "his face! Kiehl just hired",
                                    "him without letting me know..."
                                ],
                            )?;
                            l_khqread = (l_khqread.clone() + Val::from(1));
                            ctx.next()?;
                        }
                        _ => {}
                    }
                    if l_khqread.clone() == 3 {
                        ctx.lines_as(
                            "Kiel Hyre",
                            args![
                                "If you don't have",
                                "anymore questions for me,",
                                "then would you please",
                                "let me rest?? I'm still not",
                                "feeling well from the time",
                                "I was locked up in the factory."
                            ],
                        )?;
                        ctx.call(Function::DelItem, vec![Val::from(7493), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(7494), Val::from(1)])?;
                        ctx.var("kielhyrequest").set(Val::from(48))?;
                        break 'l1;
                    }
                }
            }
        } else {
            if ctx.var("kielhyrequest").get()? == 48 {
                ctx.lines_as(
                    "Kiel Hyre",
                    args![
                        "Ah, I almost forgot.",
                        "Please, take this as",
                        "a little reward for",
                        "saving my life."
                    ],
                )?;
                ctx.call(Function::GetItem, vec![Val::from(12105), Val::from(1)])?;
                ctx.call(Function::GetExperience, vec![Val::from(700000), Val::from(0)])?;
                ctx.var("kielhyrequest").set(Val::from(50))?;
            } else {
                if (ctx.var("kielhyrequest").get()?.number()? >= 50 && ctx.var("kielhyrequest").get()?.number()? < 64) {
                    ctx.lines_as(
                        "Kiel Hyre",
                        args![
                            "If you don't have",
                            "anymore questions for me,",
                            "then would you please",
                            "let me rest?? I'm still not",
                            "feeling well from the time",
                            "I was locked up in the factory."
                        ],
                    )?;
                } else {
                    if ctx.var("kielhyrequest").get()? == 64 {
                        ctx.call(Function::Cutin, vec![Val::from("kh_kyel01"), Val::from(2)])?;
                        ctx.lines_as(
                            "Kiel Hyre",
                            args!["Hm? Did you", "have something", "that you wanted", "to ask me?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "I don't have enough",
                                "concrete evidence yet,",
                                "but I might have some",
                                "questions soon enough."
                            ],
                        )?;
                    } else {
                        if ctx.var("kielhyrequest").get()? == 68 {
                            ctx.call(Function::Cutin, vec![Val::from("kh_kyel02"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    ((Val::from("Ah, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                                    "It's you. So how can",
                                    "I help you today?"
                                ],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("About ^3355FFAllysia^000000...")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "You know, I was looking",
                                    "through this deserted house",
                                    "in Juno, and discovered",
                                    "an old portrait of a woman",
                                    "that looks just like Allysia."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_kyel02"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args!["Oh...", "Is that all?", "I thought you had", "a robotics question."],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_Kyel03"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Anyway, that's a",
                                    "strange coincidence.",
                                    "Well, I suppose it's",
                                    "not so strange to find",
                                    "look-a-likes for other people..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "I don't think it's just",
                                    "a coincidence. The woman",
                                    "in that thirty year old portrait",
                                    "was also named Allysia, and she",
                                    "worked at Orsimier street",
                                    "in Juno. Does that ring a bell?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_kyel02"), Val::from(2)])?;
                            ctx.lines_as("Kiel Hyre", args!["......", ".........", "............"])?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_kyel01"), Val::from(2)])?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Then I thought that this",
                                    "Allysia must have been the",
                                    "woman that you loved, and",
                                    "that you based your robot's",
                                    "apperance on her."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_kyel02"), Val::from(2)])?;
                            ctx.lines_as("Kiel Hyre", args!["............"])?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_kyel01"), Val::from(2)])?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "I think this is what happened:",
                                    "when you were a young, poor",
                                    "man, you fell in love with",
                                    "Allysia. However, she was",
                                    "in love with Rosimier, who",
                                    "was rich and powerful."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "However, Rosimier was",
                                    "betrothed to some other",
                                    "woman, and he ended up",
                                    "marrying his fiancee, thus",
                                    "breaking Allysia's heart."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Feeling betrayed, her",
                                    "heart broken, Allysia jumped",
                                    "into a river. Then, you decided",
                                    "to get revenge on Rosimier, so",
                                    "you ended up joining",
                                    "Rekenber Corporation!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_Kyel03"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Hahahahahahaha!",
                                    "Oh, what an imagination~",
                                    "That's very ridiculous...",
                                    "Though, I admit, maybe",
                                    "I did design Allysia after",
                                    "seeing that woman long ago."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "I'd almost forgotten",
                                    "about her! I think we",
                                    "were friends... Though,",
                                    "where did you get the idea",
                                    "that I might have",
                                    "been in love with her?"
                                ],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("Reveal Kiel's Portrait from Hut")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_kyel02"), Val::from(2)])?;
                            ctx.lines_as("Kiel Hyre", args!["Wh-what...", "How did...", "Where did you...?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Kiel Hyre, I found this",
                                    "portrait of you as a young",
                                    "man from the house of the",
                                    "man that bought Allysia's ring.",
                                    "I even spoke to the fisherman",
                                    "that discovered Allysia's body."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "You paid an awful lot",
                                    "of money to buy Allysia's",
                                    "ring. How can you not tell",
                                    "me that you didn't love her?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_Kyel03"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "*Sigh...*",
                                    "You got me, you got me.",
                                    "I didn't want you to learn",
                                    "the truth. You are correct.",
                                    "I loved Allysia, and designed",
                                    "my robot to look just like her."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "I could never forget her.",
                                    "Ever. But I would never",
                                    "do anything to harm the",
                                    "Rosimiers! I'm a scientist!",
                                    "I hated him when I was young,",
                                    "but things are different now!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "I shed no tears when the",
                                    "Rosimiers fell, but I wasn't",
                                    "responsible. Besides, I didn't",
                                    "have the resources or the",
                                    "capability to cause it..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_kyel02"), Val::from(2)])?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["I'm afraid that the", "evidence shows otherwise."],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("Reveal Portrait of Rosimiers")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Take a good look",
                                    "at this portrait that",
                                    "I found at the Rosimiers'",
                                    "old house. Do you see",
                                    "anything... incriminating?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_Kyel03"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Aside from that",
                                    "James Rosimier, you",
                                    "mean? No! I don't see",
                                    "anything wrong with",
                                    "this picture at all."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Take a good look",
                                    "at the pocketwatch",
                                    "in the portrait. That's",
                                    "the pocketwatch you",
                                    "wear today, isn't it?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_kyel02"), Val::from(2)])?;
                            ctx.lines_as("Kiel Hyre", args!["...!!!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "You might not have caused",
                                    "the downfall of the Rosimiers",
                                    "yourself, but with the aid of",
                                    "the Rekenber Corporation, I'd",
                                    "say it was entirely possible!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_Kyel03"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Well played, adventurer.",
                                    "Well played. I don't regret",
                                    "what I did: they killed my",
                                    "Allysia! If James didn't betray",
                                    "her, if only he didn't drive",
                                    "her to commit suicide..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "That's where you're wrong!",
                                    "Allysia was killed, she didn't",
                                    "commit suicide. Take a good",
                                    "look at this note right here!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Kiel Hyre", args!["What?!"])?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("Show James's Note")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "What does this prove?",
                                    "This doesn't show that",
                                    "James didn't betray Allysia.",
                                    "How does this change anything?",
                                    "She's dead, nothing I can do",
                                    "will bring her back to me!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "I never said James didn't",
                                    "betray her. Look at the date",
                                    "on the note. James made plans",
                                    "to run away with her on August",
                                    "20th. However, her body was",
                                    "found on the same day."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Ergo, Allysia must have",
                                    "died on August 19th. If she",
                                    "was planning to run away with",
                                    "her love on the next day, then",
                                    "she had no reason to kill herself!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "No, that's not right!",
                                    "She probably couldn't",
                                    "trust me! She must have",
                                    "realized she was nothing",
                                    "but another toy to him!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Oh yeah? I say she jumped",
                                    "into the river because you",
                                    "met her on that day. Now,",
                                    "take a good look at this!"
                                ],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("Show K.H.'s note")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "This note was  written by",
                                    "a man with your initials,",
                                    "K.H. These initials were also",
                                    "signed on her portrait. You",
                                    "must have written this note:",
                                    "there's too many coincidences!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "According to this note,",
                                    "you told Allysia that you",
                                    "wanted to see her again",
                                    "at the place you first met.",
                                    "I think you did see her again...",
                                    "on August 19th, the day she died!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "I'm assuming the place you",
                                    "two first met was near the",
                                    "river. No more of your lies:",
                                    "Tell me what really happened!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_kyel02"), Val::from(2)])?;
                            ctx.lines_as("Kiel Hyre", args!["............", ".........", "......"])?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_Kyel03"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Hah... Ha ha ha...",
                                    "Yes... That's right...",
                                    "That horrible night.",
                                    "I remember it well...",
                                    "....................."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_kyel02"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "That night, when she came",
                                    "to the river to meet me as",
                                    "I had asked, I begged her to",
                                    "run away with me, instead",
                                    "of waiting for that James."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "She insisted that James",
                                    "never betrayed her, and he",
                                    "promised to take her away",
                                    "with him the next day. Can",
                                    "you imagine how that made",
                                    "me feel? I was nothing to her."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "My feelings didn't matter to",
                                    "her at all! She kept fidgeting",
                                    "with that ring...I lost control",
                                    "and tried to take that damned",
                                    "thing away from her, and",
                                    "throw it into the river..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "But you know what?",
                                    "She struggled, she actually",
                                    "fought me! It was just a small",
                                    "fight, but then, before I knew",
                                    "it, the ground underneath us",
                                    "collapsed and... the rains..."
                                ],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from(".........")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "I'm not sure what it",
                                    "was. The rain weakened",
                                    "the ground, something went",
                                    "wrong... and she just... just...",
                                    "The river swallowed her...",
                                    "I felt empty. She was gone. "
                                ],
                            )?;
                            ctx.call(Function::DelItem, vec![Val::from(7499), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(7500), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(7501), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(7502), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(7503), Val::from(1)])?;
                            ctx.var("kielhyrequest").set(Val::from(70))?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        } else if ctx.var("kielhyrequest").get()?.number()? <= 70 {
                            ctx.call(Function::Cutin, vec![Val::from("kh_kyel01"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "You already know that",
                                    "she was found dead the",
                                    "next day. But what really",
                                    "broke my heart was that",
                                    "she held that ring so tightly",
                                    "in her hand, even in death..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_kyel03"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "His family had everything",
                                    "while I had nothing. And",
                                    "he had the audacity to take",
                                    "Allysia away from me?!",
                                    "How could that be right?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Even though she had passed",
                                    "away, I still wanted to prove",
                                    "to Allysia what kind of ugly",
                                    "person James really was.",
                                    "That was when I joined the",
                                    "Rekenber Corporation."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "I designed the very first",
                                    "First Generation Robot, which",
                                    "I named Allysia, and sold the",
                                    "designs to Rekenber. I gave them",
                                    "robots, and they gave me money,",
                                    "power, obedient subordinates."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Of course I knew they'd use",
                                    "my robots for spying and killing!",
                                    "But you know what? It didn't",
                                    "matter so long as they gave me",
                                    "the means to my revenge. It was",
                                    "the perfect partnership, really."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "You've got me right",
                                    "where you want me.",
                                    "Who are you working",
                                    "for, and what exactly",
                                    "do you want? My designs?",
                                    "My death? Everything...?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_kyel01"), Val::from(2)])?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Actually, I just want",
                                    "to ask about the nature",
                                    "of your professional",
                                    "relationship with the",
                                    "Rekenber Corporation.",
                                    "And about Kiehl."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Kiehl, eh? After hearing",
                                    "my crazy story, I'm guessing",
                                    "that you already suspect the",
                                    "truth about him... He's also",
                                    "a robot, specifically the first",
                                    "of the Second Generation models."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "His mind was developed using.",
                                    "an experimental, and unstable,",
                                    "form of the Condensed Magic",
                                    "Spell Scrolls. He was the only",
                                    "Second Generation robot that",
                                    "I was allowed to keep."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "I've become very attached",
                                    "to Kiehl. It's not surprising,",
                                    "seeing that robotics have",
                                    "become my life. I even raised",
                                    "him as my own son, and taught",
                                    "him everything about robotics"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Kiehl is now a genius,",
                                    "and has even developed the",
                                    "Third Generation of robots.",
                                    "Unfortunately, I failed to",
                                    "properly raise him with",
                                    "human morals and ethics."
                                ],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("......")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "He's been transforming",
                                    "the Third Generation robots",
                                    "into killing machines. That's",
                                    "why I tried to put them all",
                                    "into the academy, so they",
                                    "could learn human behavior."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Although the academy",
                                    "has delayed Kiehl's plans,",
                                    "he has succeeded into",
                                    "converting all of the robots",
                                    "into uncontrollable engines",
                                    "of mass destruction."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Yes, he's been working",
                                    "closely with Rekenber.",
                                    "Their true objective is to",
                                    "create killing machines",
                                    "for Rekenber's use."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "You know all the secrets",
                                    "of my past now. I'm not",
                                    "upset with you or anything,",
                                    "but I do have something",
                                    "that I want to ask of you."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Please! Stop Kiehl!",
                                    "I don't want his madness",
                                    "to destroy any more robots!",
                                    "I see each and every one",
                                    "of them as one of my children!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "I know that I must take",
                                    "full responsibility for all",
                                    "that has happened. I promise",
                                    "to take any punishment for",
                                    "my actions once everything",
                                    "has been resolved."
                                ],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("Accept:Okay:Nod")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Thank you so much!",
                                    "You can find Kiehl",
                                    "in the underground",
                                    "level in this mansion."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "He stays in the old room",
                                    "where he was created, but",
                                    "he reconstructed it as some",
                                    "kind of cave to keep everyone",
                                    "out, including me. Yes, he",
                                    "doesn't trust anyone anymore..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "When you find him, I want",
                                    "you to take Allysia's ring",
                                    "out of his heart. If you",
                                    "remove it, that should stop",
                                    "him from going berserk."
                                ],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("Allysia's Ring?")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Yes... Her ring is the",
                                    "beginning of everything",
                                    "I put that in his heart so",
                                    "that I'd never forget what",
                                    "the Rosimiers did to me."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "I think Kiehl's grown",
                                    "so powerful that normal",
                                    "weapons might not work",
                                    "on him anymore. Use this",
                                    "device that will cause his",
                                    "power supply to fluctuate."
                                ],
                            )?;
                            ctx.call(Function::GetItem, vec![Val::from(7504), Val::from(1)])?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_kyel03"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "If you successfully attach",
                                    "this to Kiehl's body, then",
                                    "he won't be able to use his",
                                    "body's full power. While he's",
                                    "weakened, open up his chest",
                                    "and get the ring from his heart."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args!["Let me know once", "you're ready. Then, I'll", "guide you Kiehl's room."],
                            )?;
                            ctx.var("kielhyrequest").set(Val::from(74))?;
                        } else if (ctx.var("kielhyrequest").get()?.number()? >= 74 && ctx.var("kielhyrequest").get()?.number()? <= 104) {
                            ctx.lines_as("Kiel Hyre", args!["Are you ready", "to confront", "Kiehl now?"])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
                                1 => {
                                    ctx.lines_as(
                                        "Kiel Hyre",
                                        args![
                                            "There... the secret",
                                            "passage is open now.",
                                            "Just go to the right of",
                                            "me, but be careful. Kiehl",
                                            "is extremely dangerous."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    ctx.call(Function::EnableNpc, vec![Val::from("Kiehl_Room_Warp")])?;
                                    ctx.call(Function::DoNpcEvent, vec![Val::from("Kiehl_Room_Warp::OnEnable")])?;
                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Kiel Hyre",
                                        args!["Please take your", "time. I imagine that", "it won't be easy."],
                                    )?;
                                }
                                _ => {}
                            }
                        } else if (ctx.var("kielhyrequest").get()?.number()? >= 74 && ctx.var("kielhyrequest").get()?.number()? <= 106) {
                            ctx.call(Function::Cutin, vec![Val::from("kh_kyel01"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "You're back...!",
                                    "So were you able",
                                    "to retrieve Allysia's",
                                    "Ring from Kiehl's heart?"
                                ],
                            )?;
                            ctx.next()?;
                            if ctx.call(Function::CountItem, vec![Val::from(7508)])?.number()? < 1 {
                                ctx.lines_as(
                                    "Kiel Hyre",
                                    args![
                                        "You mean... You don't have it?",
                                        "Please, retrieve Allysia's Ring",
                                        "from Kiehl's heart!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::EnableNpc, vec![Val::from("Kiehl_Room_Warp")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("Kiehl_Room_Warp::OnEnable")])?;
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Wh-what happened...?",
                                    "Kiehl developed a new",
                                    "body for himself? Th-that",
                                    "would make him a Fourth",
                                    "Generation robot. I had",
                                    "no idea he was this smart."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Wait, now that I think about",
                                    "it, I did see robot bodies that",
                                    "looked like Kiehl when I was",
                                    "locked in the factory. So he",
                                    "was using those copies to",
                                    "develop personal upgrades."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Here, please take this",
                                    "Keycard which will let you",
                                    "enter and investigate the",
                                    "deepest levels of the factory.",
                                    "I'll investigate Kiehl's room."
                                ],
                            )?;
                            ctx.call(Function::GetItem, vec![Val::from(7509), Val::from(1)])?;
                            ctx.var("kielhyrequest").set(Val::from(108))?;
                            ctx.next()?;
                            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
                                if ctx.call(Function::GetPartnerId, vec![])? == 0 {
                                    l_khtitle_s = Val::from("Miss");
                                } else {
                                    l_khstitle_s = Val::from("Mrs");
                                }
                            } else {
                                l_khtitle_s = Val::from("Mr");
                            }
                            ctx.lines_as(
                                "Kiel Hyre",
                                args![
                                    "Kiehl is my responsibility...",
                                    "No matter what the cost may",
                                    "be, I've got to stop him! Oh,",
                                    "and here, please take this",
                                    "with my thanks for all of",
                                    ((((Val::from("your help, ") + l_khtitle_s.clone()) + Val::from(" "))
                                        + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                        + Val::from("."))
                                ],
                            )?;
                            ctx.call(Function::GetItem, vec![Val::from(616), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(7508), Val::from(1)])?;
                            ctx.call(Function::GetExperience, vec![Val::from(1000000), Val::from(0)])?;
                        } else if ctx.var("kielhyrequest").get()?.number()? >= 108 {
                            ctx.lines_as("Kiel Hyre", args!["......", ".........", "............"])?;
                        }
                    }
                }
            }
        }
    }
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn kiel_hyre_kh(ctx: &Ctx) -> Script {
    kiel_hyre_kh_body(ctx, Vec::new()).map(|_| ())
}

fn allysia_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("kielhyrequest").get()?.number()? < 46 {
        ctx.lines_as("Allysia", args!["Who are you?", "How did you get here?", "Go away"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(Function::Cutin, vec![Val::from("kh_ellisia"), Val::from(2)])?;
    if ctx.var("kielhyrequest").get()?.number()? < 70 {
        ctx.lines_as(
            "Allysia",
            args![
                "You must be surprised",
                "by everything that's been",
                "happening. Maybe everything",
                "would be easier to understand",
                "if I explained about robots?"
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Allysia",
                    args![
                        "The robots you've been",
                        "encountering are automated",
                        "mechanical puppets that can",
                        "independantly think and operate.",
                        "Many sages have tried to develop",
                        "their own robots, but have failed."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Allysia",
                    args![
                        "My master, Kiel Hyre, has",
                        "been studying robotics since",
                        "he was twenty years old, and",
                        "has developed three different",
                        "generations of robots, the first,",
                        "second, and third generations."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("First Generation:Second Generation:Third Generation")])? {
                    1 => {
                        ctx.lines_as(
                            "Allysia",
                            args![
                                "I am a good example of one",
                                "of Kiel Hyre's First Generation",
                                "robots. I was constructed using",
                                "a heavy mechanical framework,",
                                "a robotic heart, and chemically",
                                "synthesized skin covering."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Allysia",
                            args![
                                "My central processing unit,",
                                "equivalent to your brain, is",
                                "essentially a Memory Scroll",
                                "based on the design of the",
                                "Magic Spell Scrolls that you",
                                "adventurers use in battle."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Allysia",
                            args![
                                "I'm the oldest type of",
                                "humanoid robot, so I weigh",
                                "a lot, and my mind can only",
                                "process a limited amount of",
                                "data. Therefore, I can't express",
                                "emotion similarly to a human."
                            ],
                        )?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as(
                            "Allysia",
                            args![
                                "The First Generation robots",
                                "were developed from mostly",
                                "mechanical parts, but the",
                                "Second Generation robots",
                                "incorporated Homunculus",
                                "science and technology."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Allysia",
                            args![
                                "Second Generation robots",
                                "are more life-like since they",
                                "have artifically created skin",
                                "and flesh, although they still",
                                "are constructed from a heavy",
                                "mechanical framework."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Allysia",
                            args![
                                "Sage elemental scroll",
                                "technology was also used",
                                "to develop the Condensed",
                                "Memory Scroll, a central",
                                "processing unit superior to that",
                                "used in First Generation robots."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Allysia",
                            args![
                                "Although Condensed Memory",
                                "Scrolls were 100,000 times",
                                "more powerful than ordinary",
                                "Memory Scrolls, they were",
                                "problematic and were prone",
                                "to too many error problems."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Allysia",
                            args![
                                "Second Generation robots",
                                "were capable of expressing",
                                "human-like emotion, but their",
                                "production halted after six",
                                "years because they were",
                                "considered faulty."
                            ],
                        )?;
                        ctx.next()?;
                    }
                    3 => {
                        ctx.lines_as(
                            "Allysia",
                            args![
                                "Third Generation robots",
                                "were mostly designed by",
                                "Kiel Hyre's son, Kiehl,",
                                "and don't use a mechanical",
                                "framework at all: the entire",
                                "body is basically a homunculus."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Allysia",
                            args![
                                "With their organic bodies",
                                "and advanced artificial hearts",
                                "made from imitation Ymir Heart",
                                "Pieces, they can experience",
                                "physiologic phenomena",
                                "just like ordinary humans."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Allysia",
                            args![
                                "Kiehl was able to develop",
                                "a more stable form of the",
                                "Condensed Memory Scroll",
                                "which does not suffer from",
                                "critical errors, and can be",
                                "cheaply mass processed."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Allysia",
                            args![
                                "Elly is actually a Third",
                                "Generation prototype. Once",
                                "we optimize the prototypes,",
                                "we will begin mass production.",
                                "In fact, the academy is our",
                                "prototype testing ground."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Allysia",
                            args![
                                "The fact that out prototype",
                                "robots can interact just like",
                                "real humans is proof of our",
                                "success in robotics."
                            ],
                        )?;
                        ctx.next()?;
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Allysia",
                    args!["Please let me know if", "you'd like to learn more", "about Kiel Hyre's robots."],
                )?;
                break 'b1;
            }
        }
    } else {
        ctx.lines_as("Allysia", args!["......", ".........", "............"])?;
    }
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn allysia(ctx: &Ctx) -> Script {
    allysia_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AbductionTriggerStep {
    Start,
    OnTouch,
}

fn abduction_trigger_run(ctx: &Ctx, mut step: AbductionTriggerStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AbductionTriggerStep::Start => {
                step = AbductionTriggerStep::OnTouch;
                continue 'machine;
            }
            AbductionTriggerStep::OnTouch => {
                if ctx.var("kielhyrequest").get()? == 50 {
                    ctx.lines(args![
                        "^3355FFAs you walked out of",
                        "the mansion, something",
                        "smashed the top of your",
                        "head, and you instantly",
                        "lose consciousness..."
                    ])?;
                    ctx.close_window()?;
                    ctx.call(Function::PercentHeal, vec![Val::from(-99), Val::from(0)])?;
                    ctx.call(Function::Warp, vec![Val::from("kh_mansion"), Val::from(30), Val::from(75)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn abduction_trigger(ctx: &Ctx) -> Script {
    abduction_trigger_run(ctx, AbductionTriggerStep::Start, Vec::new()).map(|_| ())
}

pub fn abduction_trigger_ontouch(ctx: &Ctx) -> Script {
    abduction_trigger_run(ctx, AbductionTriggerStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MysteriousWomanKhStep {
    Start,
    OnTouch,
}

fn mysterious_woman_kh_run(ctx: &Ctx, mut step: MysteriousWomanKhStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MysteriousWomanKhStep::Start => {
                step = MysteriousWomanKhStep::OnTouch;
                continue 'machine;
            }
            MysteriousWomanKhStep::OnTouch => {
                if ctx.var("kielhyrequest").get()? == 50 {
                    ctx.lines(args![
                        "^3355FFYou awaken with your",
                        "head painfully throbbing,",
                        "and a mysterious woman",
                        "standing in front of you.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "??????",
                        args![
                            "Hm? Oh, you're awake",
                            "earlier than I thought.",
                            "You must feel confused,",
                            "but listen carefully. If you",
                            "don't, then I can't guarantee",
                            "your safety, okay? Alright."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "??????",
                        args![
                            "All you need to know it",
                            "that I'm a secret agent for",
                            "the Schwarzwald Republic",
                            "government. We're investigating",
                            "abnormal activity between Kiel",
                            "Hyre and the Rekenber Corporation."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "??????",
                        args![
                            "We saw you enter the",
                            "mansion and speak to",
                            "Kiel Hyre, so basically",
                            "you're here for questioning.",
                            "Now tell me the truth. How",
                            "do you know Kiel Hyre?"
                        ],
                    )?;
                    ctx.next()?;
                    'b1: {
                        let subject1 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("I'll tell you everything!:I don't know nuthin'!")],
                        )?);
                        let mut matched1 = false;
                        let no_case1 = !subject1.loosely_equals(&Val::from(2)) && !subject1.loosely_equals(&Val::from(1));
                        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.lines_as(
                                "??????",
                                args![
                                    "Don't...know...nuthin'?",
                                    "You sure about that? Only",
                                    "a select few can even speak",
                                    "with Kiel Hyre in person. You",
                                    "must have some connection",
                                    "to him. I'm right, aren't it?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["......", ".........", "............"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "??????",
                                args![
                                    "Are you trying to protect",
                                    "him? I think that you might",
                                    "not understand what kind of",
                                    "person you're really dealing",
                                    "with here. I'll tell you what",
                                    "I've learned about him..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "??????",
                                args![
                                    "Kiel Hyre. CEO of the",
                                    "Kiel Hyre Foundation,",
                                    "manufacturer of various",
                                    "machinery. His company started",
                                    "as a small Einbroch store whose",
                                    "technology slowly grew famous."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "??????",
                                args![
                                    "Rekenber Corperation offered",
                                    "a merger with the Kiel Hyre",
                                    "Foundation. We're still not",
                                    "sure why they wanted Kiel",
                                    "Hyre in particular to repair",
                                    "and develop their Guardians..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "??????",
                                args![
                                    "We're also not sure why",
                                    "they wanted to suddenly",
                                    "focus more on Guardian",
                                    "development. Then, all",
                                    "of a sudden, Kiel Hyre's son",
                                    "appears from out of nowehre."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "??????",
                                args![
                                    "It's very suspicious.",
                                    "There's no records of his",
                                    "birth or anything. Still, maybe",
                                    "Kiel Hyre really did have him",
                                    "with his secretary, Allysia.",
                                    "Well, no one is really sure."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "??????",
                                args![
                                    "In any case, Kiel Hyre's",
                                    "son and heir, Kiehl, helped",
                                    "his father establish this",
                                    "special academy as their",
                                    "way of giving back to society."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "??????",
                                args![
                                    "Almost every corporation and",
                                    "organization tried to get their",
                                    "spies to enter this academy,",
                                    "but all of them were rejected.",
                                    "it's strange. At least one of",
                                    "them should have made it in."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "??????",
                                args![
                                    "And recently, Kiehl Hyre",
                                    "has held a secret meeting with",
                                    "Rekenber executives to announce",
                                    "his new project. He intends to",
                                    "create advanced humanoid robots",
                                    "that will replace guardians!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "??????",
                                args![
                                    "Our spies reported that",
                                    "Kiel Hyre was nowhere to",
                                    "be seen at that meeting,",
                                    "as well as his trusted",
                                    "secretary, Allysia. He...",
                                    "He just disappeared!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "??????",
                                args![
                                    "Then, Kiel Hyre pops back",
                                    "in his mansion after all",
                                    "this time as if nothing",
                                    "happened! At the same time,",
                                    "Kiehl disappears, under the",
                                    "excuse of conducting research."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "??????",
                                args![
                                    "Rekenber is sponsoring",
                                    "both Kiehl and Kiel, but",
                                    "there's some kind of conflict",
                                    "going on between father and",
                                    "son, I just know it! Now tell",
                                    "me, what's going on?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "??????",
                                args![
                                    "I'm not sure how well you",
                                    "know this country, but the",
                                    "fact that Rekenber is invoved",
                                    "must tell you that these aren't",
                                    "good people. Tell me what",
                                    "you know about them!"
                                ],
                            )?;
                            ctx.next()?;
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.lines(args![
                                "^3355FFYou tell the woman^000000",
                                "^3355FFeverything you know^000000",
                                "^3355FFabout Kiel Hyre. Your^000000",
                                "^3355FFvoice quivers with sadness^000000",
                                "^3355FFwhenever you mention Elly.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "??????",
                                args![
                                    "I see, I see...",
                                    "That poor girl... So...",
                                    "Your involvement in this",
                                    "is a coincidence? In that",
                                    "case, I want your help",
                                    "in our investigation."
                                ],
                            )?;
                            ctx.next()?;
                            'b2: {
                                let subject2 = Val::from(runtime::select_values(ctx, &[Val::from("Okay:......")])?);
                                let mut matched2 = false;
                                let no_case2 = !subject2.loosely_equals(&Val::from(2)) && !subject2.loosely_equals(&Val::from(1));
                                if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                                    matched2 = true;
                                }
                                if matched2 {
                                    ctx.lines_as(
                                        "??????",
                                        args!["If you don't cooperate,", "then I can't guarantee", "your safety, adventurer"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["Huh? What...?", "What does that mean?", "Are you threatening me?"],
                                    )?;
                                    ctx.next()?;
                                    let choice = runtime::select_values(ctx, &[Val::from("Just do what she says.")])?;
                                    ctx.var("@menu").set(choice)?;
                                }
                                if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                                    matched2 = true;
                                }
                                if matched2 {
                                    ctx.lines_as(
                                        "??????",
                                        args![
                                            "Great.",
                                            "I'm Agent Mitchell Layla.",
                                            "From here on, you're working",
                                            "for the Schwarzwald Republic!"
                                        ],
                                    )?;
                                    ctx.var("kielhyrequest").set(Val::from(52))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                        }
                    }
                } else if ctx.var("kielhyrequest").get()? == 52 {
                    ctx.lines_as(
                        "Mitchell",
                        args![
                            "I've got some new",
                            "information for you.",
                            "There's an old lady in",
                            "Juno that knew a woman",
                            "named Allysia 30 years ago."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mitchell",
                        args![
                            "The clincher is that this",
                            "Allysia from 30 years ago",
                            "commited suicide, and is",
                            "identical to Kiel Hyre's",
                            "secretary, who is also",
                            "named Allysia."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mitchell",
                        args![
                            "This is too much of a",
                            "coincidence. I want you to",
                            "go to Juno and investigate.",
                            "When you're done, talk to",
                            "Kiel Hyre's steward, and",
                            "he'll send you over to me."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mitchell",
                        args![
                            "Yeah, I know.",
                            "That guy actually",
                            "works for me. Anyway,",
                            "when you're ready to go",
                            "to Juno, let me know, and you",
                            "can board the federal airship."
                        ],
                    )?;
                    ctx.var("kielhyrequest").set(Val::from(54))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("kielhyrequest").get()? == 54 {
                    ctx.lines_as(
                        "Mitchell",
                        args![
                            "Are you ready?",
                            "I'll let you board",
                            "the federal Airship so",
                            "you can get to Juno, and",
                            "finish your mission quickly."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
                        1 => {
                            ctx.lines_as(
                                "Mitchell",
                                args![
                                    "Good luck. Once you",
                                    "complete your mission,",
                                    "make sure that you report",
                                    "to Kiel Hyre's steward so",
                                    "that he can send you to me."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("yuno"), Val::from(54), Val::from(209)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Mitchell",
                                args!["Take your time...", "But keep in mind that", "I'm not a patient woman!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("kielhyrequest").get()? == 64 {
                    ctx.lines_as(
                        "Mitchell",
                        args!["Ah, you're back.", "What do you have", "to report from your", "investigation?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mitchell",
                        args![
                            ".....................",
                            "Ah, I see. Good work.",
                            "Why don't you go speak to Kiel",
                            "Hyre and confront him with",
                            "what you've learned about",
                            "his past? Yeah, grill him."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mitchell",
                        args![
                            "Wear this hidden mic,",
                            "so we can send help if",
                            "you're endangered. I want",
                            "you to find out who Kiel",
                            "really is, and what's his",
                            "relationship to Rekenber."
                        ],
                    )?;
                    ctx.var("kielhyrequest").set(Val::from(68))?;
                    ctx.next()?;
                    ctx.call(Function::Warp, vec![Val::from("kh_mansion"), Val::from(83), Val::from(50)])?;
                    return Err(Stop::End);
                } else if ctx.var("kielhyrequest").get()?.number()? >= 68 {
                    ctx.lines_as("Mitchell", args!["Shouldn't you be", "leaving about now?"])?;
                    ctx.next()?;
                    ctx.call(Function::Warp, vec![Val::from("kh_mansion"), Val::from(83), Val::from(50)])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn mysterious_woman_kh(ctx: &Ctx) -> Script {
    mysterious_woman_kh_run(ctx, MysteriousWomanKhStep::Start, Vec::new()).map(|_| ())
}

pub fn mysterious_woman_kh_ontouch(ctx: &Ctx) -> Script {
    mysterious_woman_kh_run(ctx, MysteriousWomanKhStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn kiehl_room_warp(ctx: &Ctx) -> Script {
    kiehl_room_warp_run(ctx, KiehlRoomWarpStep::Start, Vec::new()).map(|_| ())
}

pub fn kiehl_room_warp_ontouch(ctx: &Ctx) -> Script {
    kiehl_room_warp_run(ctx, KiehlRoomWarpStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn kiehl_room_warp_onenable(ctx: &Ctx) -> Script {
    kiehl_room_warp_run(ctx, KiehlRoomWarpStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn kiehl_room_warp_ontimer30000(ctx: &Ctx) -> Script {
    kiehl_room_warp_run(ctx, KiehlRoomWarpStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn kiehl_room_warp_oninit(ctx: &Ctx) -> Script {
    kiehl_room_warp_run(ctx, KiehlRoomWarpStep::OnInit, Vec::new()).map(|_| ())
}

fn odd_grandma_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("kielhyrequest").get()?.number()? < 54 {
        ctx.lines_as(
            "Grandma",
            args!["Where did you go,", "my darling? Where", "are you, my little dear?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()? == 54 {
        ctx.lines_as(
            "Grandma",
            args!["Lullabye...", "Say goodnight...", "Hush little baby...", "Go to sleeeeep~"],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("What are you doing?:Um, I don't see a baby...")])? {
            1 => {
                ctx.lines_as(
                    "Grandma",
                    args![
                        "Oh? My baby won't stop",
                        "crying and can't seem",
                        "to sleep. She needs to",
                        "rest, so I can go to work.",
                        "The house is so messy,",
                        "and the boss is unhappy..."
                    ],
                )?;
                ctx.var("kielhyrequest").set(Val::from(56))?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Um, I don't see a baby...")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Grandma",
                    args![
                        "What, she's right--",
                        "Well. Darling, what",
                        "are you doing? Don't",
                        "misbehave in front of",
                        "our friend! Shhh, be",
                        "good, my little girl."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Grandma",
                    args![
                        "What are you...?",
                        "Oh, look at that.",
                        "You made my little",
                        "darling cry! Shhh,",
                        "hush, little ^0000FFAllysia^000000.",
                        "Go to sleeeeeeep~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if (ctx.var("kielhyrequest").get()?.number()? >= 56 && ctx.var("kielhyrequest").get()?.number()? < 60) {
        ctx.lines_as(
            "Grandma",
            args![
                "Allysia...?!",
                "Allysia, where did",
                "you go? You were",
                "supposed to come",
                "home a while ago!"
            ],
        )?;
        if ctx.call(Function::CountItem, vec![Val::from(7500)])?.number()? < 1 {
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Allysia? Isn't she...")])? {
                1 => {
                    ctx.call(Function::Cutin, vec![Val::from("kh_ellisia_port"), Val::from(1)])?;
                    ctx.lines(args!["^3355FFYou show Allysia's", "portrait to the old woman.^000000"])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    ctx.lines_as(
                        "Grandma",
                        args![
                            "Oh, do you know",
                            "Allysia? She's been",
                            "missing! She left home",
                            "yesterday and hasn't",
                            "come back! C-can you",
                            "tell me where she is?!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
    } else if ctx.var("kielhyrequest").get()?.number()? >= 60 {
        ctx.lines_as(
            "Grandma",
            args![
                "Don't worry, Allysia...",
                "Mommy will always be",
                "here for you. There's no",
                "need to be sad..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn odd_grandma(ctx: &Ctx) -> Script {
    odd_grandma_body(ctx, Vec::new()).map(|_| ())
}
