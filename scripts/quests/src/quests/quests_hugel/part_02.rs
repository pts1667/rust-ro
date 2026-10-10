use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn siria_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_bio").get()? == 1 {
        ctx.lines_as(
            "Siria",
            args![
                "Oh, hello there.",
                "Can I help you",
                "with anything?",
                "It looks like you want",
                "to ask me something..."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args!["^3355FFHow can a woman this", "young be Morriphen's wife?^000000"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Yes, um, Morriphen sent",
                "me here so that I can bring",
                "him his medicine. I was wo--"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Siria",
            args![
                "Oh my God! Is he alright?",
                "Please, oh dear Lord, don't",
                "let him be dead! No! D-did",
                "anything bad happen to him?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "(She is being panicky, and doesn't seem to know what she should do.",
                "She absent-mindedly steps forward and then stumbles on the floor.)"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Are you okay?!",
                "(Her eyes look blank. She seems to have a hard time to see things.)"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Siria",
            args![
                "Oh, I am fine, thank you.",
                "I am just...I am just having a vision problem.",
                "So...what happened to him?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "He's just not feeling",
                "well, that's all. All he",
                "told me was that he ran",
                "out of his medicine, and",
                "needs some more."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Siria",
            args![
                "Aww, I just ran out of",
                "the medicine too. Find",
                "Morriphen's friend, and get",
                "it from him. He's supposed to",
                "be in Lighthalzen, somewhere..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Siria",
            args![
                "^333333*Cough cough!*^000000",
                "Ugh, excuse me.",
                "Let me sit down for",
                "a moment. *Cough!*"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFSiria's face grows",
            "pale, and she starts",
            "to sweat profusely.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Um...", "Are you feeling", "alright? You're not", "sick too, are you?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Siria",
            args![
                "Oh, no, no... I'm just",
                "so worried about Morriphen.",
                "*Sob* I can't live without him,",
                "he's everything in the world",
                "to me. I owe him everything,",
                "I have to go see Dono, I hav--"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "You know what? You better",
                "relax and rest here, and I'll",
                "go visit this Dono person in",
                "Lighthalzen. You don't sound",
                "well enough to be traveling."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Siria",
            args![
                "I'm... I'm sorry to",
                "trouble you. But if",
                "Morriphen can trust you",
                "to come here, then I'll",
                "trust you too. ^333333*Cough*^000000",
                "Thank you so much..."
            ],
        )?;
        ctx.var("hg_bio").set(Val::from(2))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(11009), Val::from(11010)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("hg_bio").get()? == 2 {
            ctx.lines_as(
                "Siria",
                args![
                    "^333333*Sniff*^000000 Please",
                    "go to Lighthalzen and",
                    "get ^333333*Cough*^000000 Morriphen's",
                    "medicine from his friend,",
                    "Dono, as soon as you can."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("hg_bio").get()? == 3 {
                ctx.lines_as(
                    "Siria",
                    args![
                        "Oh, have you met Dono?",
                        "I'm so grateful for all of",
                        "his help. He really is a",
                        "good friend to Morriphen,",
                        "no matter what he says."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("hg_bio").get()? == 4 {
                    ctx.lines_as(
                        "Siria",
                        args![
                            "Oh, have you met Dono?",
                            "I'm so grateful for all of",
                            "his help. He really is a",
                            "good friend to Morriphen,",
                            "no matter what he says."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("hg_bio").get()? == 5 {
                        ctx.lines_as(
                            "Siria",
                            args![
                                "Oh, have you met Dono?",
                                "I'm so grateful for all of",
                                "his help. He really is a",
                                "good friend to Morriphen,",
                                "no matter what he says."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("hg_bio").get()? == 6 {
                        ctx.lines_as(
                            "Siria",
                            args![
                                "*Cough* I-I'm fine,",
                                "so please don't worry",
                                "about me. Just bring the",
                                "medicine to Morriphen as",
                                "fast as you can for me."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("hg_bio").get()? == 7 {
                        ctx.lines(args![
                            "^3355FFYou gently administered",
                            "the medicine to Siria, who",
                            "slow regained consciousness.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Hey, are you alright?",
                                "How are you feeling?",
                                "Morriphen asked me to",
                                "come here and give you",
                                "this medicine right away."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Siria",
                            args![
                                "...Ah... I can't...",
                                "I don't know... I...",
                                "Where is...? Oh, it's",
                                "you. I think... I think",
                                "you just saved my life.",
                                "I'm sorry for being a burden..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Siria",
                            args![
                                "All I've been is a nuisance",
                                "to both Morriphen and Dono.",
                                "You don't understand how I've",
                                "destroyed their lives. I just--",
                                "I just... I just need to relax.",
                                "Thank you for everything..."
                            ],
                        )?;
                        ctx.var("hg_bio").set(Val::from(8))?;
                        ctx.call(Function::GetExperience, vec![Val::from(500000), Val::from(0)])?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(11015), Val::from(11016)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("hg_bio").get()? == 8 {
                        ctx.lines_as(
                            "Siria",
                            args![
                                "Oh, please go back",
                                "to Morriphen and let",
                                "him know that I'm much",
                                "better. I don't want him",
                                "to worry about me!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("hg_bio").get()? == 9 {
                        ctx.lines_as(
                            "Siria",
                            args![
                                "Oh, welcome~",
                                "Thank you so much",
                                "for helping me and",
                                "Morriphen. You've been",
                                "a real godsend, you know?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Siria", args!["Oh, hello there.", "Can I help you", "with anything?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
}

pub fn siria(ctx: &Ctx) -> Script {
    siria_body(ctx, Vec::new()).map(|_| ())
}

fn dono_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_bio").get()? == 2 {
        ctx.lines_as("Dono", args!["What do you want?", "Spit it out, I don't", "have all day."])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Excuse me, but", "I'm looking for a", "man named Dono."],
        )?;
        ctx.next()?;
        ctx.lines_as("Dono", args!["Yeah, that's me,", "but who the heck", "are you, anyway?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Oh, I'm just here",
                "to get some medicine",
                "for Morriphen. Um, his",
                "wife Siria said that",
                "you would have it?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dono",
            args![
                "Wife? Hurmph! Not likely.",
                "Alright, I see. He must've",
                "run out of medicine. Jesus,",
                "they're both still alive? It's",
                "a miracle that they've both",
                "lasted this long. Eh, oh well."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dono",
            args![
                "It's his own fault his body",
                "got messed up. Now let's",
                "see how much medicine we",
                "got here... Hmm... Dang. This",
                "isn't good. Hey, you said you",
                "work for Morriphen, that right?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dono",
            args![
                "I have the medicine, but it's",
                "not enough for one person, much",
                "less the two of them. I need you to help me make some more.",
                "Now, listen carefully, these",
                "are the ingredients I need..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Wait, wait, I thought I was",
                "only getting medicine for",
                "Morriphen. Are you saying",
                "that his wife Siria needs",
                "the medicine too?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dono",
            args![
                "Why do you keep calling",
                "Siria his wife? Yeah, she's",
                "sick too. If you met her, you'd",
                "see for yourself, right? Well,",
                "I'm getting pretty worried about them, so you better hurry this up."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dono",
            args![
                "Go get",
                "^3355FF5 Coal^000000,",
                "^3355FF5 Brigan^000000,",
                "^3355FF5 Cyfar^000000,",
                "^3355FF1 Unripe Apple^000000,",
                "^3355FF3 Detrimindexta^000000..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dono",
            args![
                "...^3355FF20 Shells^000000,",
                "^3355FF1 Red Herb^000000,",
                "^3355FF1 Blue Herb^000000,",
                "^3355FF1 Green Herb^000000,",
                "^3355FF1 White Herb^000000, and",
                "^3355FF1 Yellow Herb^000000."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dono",
            args![
                "I know this is sudden,",
                "I know I'm pushing you",
                "hard, but I've got no choice.",
                "If you don't hurry, then",
                "those two will die, simple as that. I'll wait for you here."
            ],
        )?;
        ctx.var("hg_bio").set(Val::from(3))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(11010), Val::from(11011)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("hg_bio").get()? == 1 {
            ctx.lines_as(
                "Dono",
                args![
                    "Store's closed.",
                    "Get outta here, and",
                    "come back later to",
                    "buy something, got it?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("hg_bio").get()? == 3 {
                if ((((((((((ctx.call(Function::CountItem, vec![Val::from(971)])?.number()? > 2
                    && ctx.call(Function::CountItem, vec![Val::from(1003)])?.number()? > 4)
                    && ctx.call(Function::CountItem, vec![Val::from(619)])?.is_true())
                    && ctx.call(Function::CountItem, vec![Val::from(507)])?.is_true())
                    && ctx.call(Function::CountItem, vec![Val::from(508)])?.is_true())
                    && ctx.call(Function::CountItem, vec![Val::from(511)])?.is_true())
                    && ctx.call(Function::CountItem, vec![Val::from(509)])?.is_true())
                    && ctx.call(Function::CountItem, vec![Val::from(510)])?.is_true())
                    && ctx.call(Function::CountItem, vec![Val::from(7053)])?.number()? > 4)
                    && ctx.call(Function::CountItem, vec![Val::from(7054)])?.number()? > 4)
                    && ctx.call(Function::CountItem, vec![Val::from(935)])?.number()? > 19)
                {
                    ctx.call(Function::DelItem, vec![Val::from(971), Val::from(3)])?;
                    ctx.call(Function::DelItem, vec![Val::from(1003), Val::from(5)])?;
                    ctx.call(Function::DelItem, vec![Val::from(619), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(507), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(508), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(511), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(509), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(510), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7053), Val::from(5)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7054), Val::from(5)])?;
                    ctx.call(Function::DelItem, vec![Val::from(935), Val::from(20)])?;
                    ctx.var("hg_bio").set(Val::from(4))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(11011), Val::from(11012)])?;
                    ctx.lines_as(
                        "Dono",
                        args![
                            "Good, good, you've brought",
                            "everything I've asked. Give",
                            "me a moment to process these",
                            "materials and make the medicine. ^333333*Sigh*^000000 I used to have assistants",
                            "do this kind of work for me..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dono",
                        args![
                            "Back when I used to work for",
                            "the Rekenber Corporation, I was",
                            "a department head! Oh well, it",
                            "can't be helped now. Ah, it's",
                            "done. Now, let me test this",
                            "out before I give it to you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFDono reveals that he has",
                        "grinded the materials into",
                        "a fine powder. He sprinkles",
                        "a small amount into a glass",
                        "of water, and the liquid turns",
                        "blue after he mixes it.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dono",
                        args![
                            "Damn! I used the right",
                            "proportions! Why isn't it",
                            "working?! Morriphen, I hate",
                            "you! Grrr...! Hey, kid, I need",
                            "you to help me again!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dono",
                        args![
                            "Go to the Lighthalzen",
                            "Makkie. Tell him I sent you",
                            "to get some Red Plant Stem Powder for Morriphen's medicine."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dono",
                        args![
                            "Look, I know it's a pain,",
                            "but hurry and get it for me",
                            "as fast as you can. I need",
                            "that stuff to correct some",
                            "chemical impurities in this",
                            "medicine. Go, go now!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Dono",
                        args![
                            "If you don't hurry and",
                            "gather the materials for",
                            "the medicine, Morriphen",
                            "and Siria are gonna be",
                            "in big trouble. Listen",
                            "up, this is what I need..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dono",
                        args![
                            "Go get",
                            "^3355FF5 Coal^000000,",
                            "^3355FF5 Brigan^000000,",
                            "^3355FF5 Cyfar^000000,",
                            "^3355FF1 Unripe Apple^000000,",
                            "^3355FF3 Detrimindexta^000000..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dono",
                        args![
                            "...^3355FF20 Shells^000000,",
                            "^3355FF1 Red Herb^000000,",
                            "^3355FF1 Blue Herb^000000,",
                            "^3355FF1 Green Herb^000000,",
                            "^3355FF1 White Herb^000000, and",
                            "^3355FF1 Yellow Herb^000000."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("hg_bio").get()? == 4 {
                    ctx.lines_as(
                        "Dono",
                        args![
                            "Hey, you should be able",
                            "to find Makkie inside the",
                            "pub across the street. Hurry",
                            "up and bring me some Red",
                            "Plant Stem Power from him. Come on, Morriphen's in trouble!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("hg_bio").get()? == 5 {
                        ctx.lines_as(
                            "Dono",
                            args![
                                "Where is it?!",
                                "Okay, good, you brought",
                                "it. Huh. This isn't a lot of",
                                "Red Plant Stem Powder,",
                                "but this should do. Alright,",
                                "let's see if it works now."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFDono sprinkles the",
                            "red powder into the",
                            "glass of blue water.",
                            "The liquid immediately",
                            "becomes red.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dono",
                            args![
                                "Great, the medicine is",
                                "finally ready! By the way,",
                                "how is Makkie doing? I know",
                                "he's got a big mouth, so did",
                                "he tell you anything stupid?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Er, he did mention that",
                                "both you and Morriphen used",
                                "to work together for Rekenber."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dono",
                            args![
                                "He did, huh? Yeah, that's",
                                "true. Morriphen and I used",
                                "to be researchers... those",
                                "were the best damn days",
                                "of my life. Afterwards, it",
                                "pretty much went downhill."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["What do you mean?", "Did something happen?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dono",
                            args![
                                "Oh, hell yeah. That dumb",
                                "romanticist Morriphen caused",
                                "huge trouble for the company.",
                                "It's pretty much his fault that",
                                "I got fired from Rekenber.",
                                "Grr! Let's not talk about it!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dono",
                            args![
                                "Just thinking about what",
                                "happened makes me so angry!",
                                "I swear, I'd kill him if he wasn't my best friend! Ah, speaking",
                                "of which, you better get this",
                                "medicine to Morriphen now!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dono",
                            args![
                                "Right, and don't",
                                "mention anything",
                                "I said to him, willya?",
                                "If he gets upset, and",
                                "stresses himself to death,",
                                "my work will be for nothing."
                            ],
                        )?;
                        ctx.var("hg_bio").set(Val::from(6))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(11013), Val::from(11014)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("hg_bio").get()? == 6 {
                        ctx.lines_as(
                            "Dono",
                            args![
                                "Whoa, what are you still",
                                "doing here?! Hurry and",
                                "get that medicine over to",
                                "Morriphen, or he'll die soon!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("hg_bio").get()? == 7 {
                        ctx.lines_as(
                            "Dono",
                            args![
                                "Oh, hey, do you know",
                                "how Morriphen's doing?",
                                "Tell that jerk to come",
                                "by more often. He's the",
                                "only guy that can offer me",
                                "a decent challenge at chess..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("hg_bio").get()? == 8 {
                        ctx.lines_as(
                            "Dono",
                            args![
                                "Oh, hey, do you know",
                                "how Morriphen's doing?",
                                "Tell that jerk to come",
                                "by more often. He's the",
                                "only guy that can offer me",
                                "a decent challenge at chess..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("hg_bio").get()? == 9 {
                        ctx.lines_as(
                            "Dono",
                            args![
                                "Oh, hey, do you know",
                                "how Morriphen's doing?",
                                "Tell that jerk to come",
                                "by more often. He's the",
                                "only guy that can offer me",
                                "a decent challenge at chess..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Dono",
                            args![
                                "Hey, store's closed!",
                                "I'll open when I feel",
                                "like it, so until then,",
                                "I'm not selling you",
                                "anything! Now, scram!"
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

pub fn dono(ctx: &Ctx) -> Script {
    dono_body(ctx, Vec::new()).map(|_| ())
}

fn makkie_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_bio").get()? == 4 {
        ctx.lines_as("Makkie", args!["Hello, how may", "I help you today?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Hi, I'm here to get",
                "some Red Plant Stem",
                "Powder for Dono. It's",
                "pretty important, actually."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Makkie",
            args![
                "Ah, I see. He must need",
                "it to prepare medicine for",
                "Morriphen and Siria, am",
                "I right? Okay, just give me",
                "a moment. While you're waiting, go ahead and relax, look around..."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFMakkie sifted the powder",
            "into a bag for you, you see",
            "a picture of Makkie, Dono,",
            "and Morriphen wearing white",
            "lab coats on the table.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Makkie",
            args![
                "Oh, that? Yeah, that was",
                "taken when Dono, Morriphen",
                "and I worked for the",
                "Rekenber Corporation. I know",
                "they look eccentric, but they're really good guys, trust me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Makkie",
            args![
                "Yeah, I used to work under",
                "Dono, actually. He was head",
                "of the Genetic Engineering",
                "Department, and Morriphen",
                "was doing other research.",
                "Those were some good times."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Makkie",
            args![
                "Suddenly, me, Dono, and",
                "everyone else working under",
                "him was just fired! I was new",
                "at the time, so I was only able",
                "to learn why we were fired much",
                "later than everyone else."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Makkie",
            args![
                "It turns out that there",
                "was a fire, possibly arson,",
                "in the Rekenber genetic labs.",
                "Dono assumed full responsibility, though everyone who worked for",
                "him just knows he didn't do it."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Makkie",
            args![
                "So that's how we ended",
                "up at our current jobs.",
                "Strangely enough, Morriphen",
                "disappeared at the same time.",
                "I don't know what happened to him since our days at Rekenber..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Makkie",
            args![
                "Anyway, when Morriphen",
                "showed up again, he ended up",
                "needing the medicine we're",
                "making. I sure wish I had a",
                "best friend like Dono or",
                "Morriphen. Those two are close."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Makkie",
            args![
                "Yeah, Dono sure is a nice guy.",
                "He even helped me get this job~",
                "Anyway, if you see him, or if",
                "you visit Morriphen, please",
                "give them my regards. Here you",
                "are: Red Plant Stem Powder."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args!["^3355FFYou receive a small bag", "of Red Plant Stem Powder.^000000"])?;
        ctx.var("hg_bio").set(Val::from(5))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(11012), Val::from(11013)])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Oh! Thanks so much",
                "for the powder. Now,",
                "I better hurry and give",
                "this to Dono. Goodbye~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("hg_bio").get()? == 1 {
            ctx.lines_as(
                "Makkie",
                args!["Gosh, I'm so bored~", "I wouldn't even mind", "having some work to do."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("hg_bio").get()? == 2 {
                ctx.lines_as(
                    "Makkie",
                    args!["Gosh, I'm so bored~", "I wouldn't even mind", "having some work to do."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("hg_bio").get()? == 3 {
                    ctx.lines_as(
                        "Makkie",
                        args!["Gosh, I'm so bored~", "I wouldn't even mind", "having some work to do."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("hg_bio").get()? == 5 {
                        ctx.lines_as(
                            "Makkie",
                            args![
                                "Hm? Shouldn't you hurry",
                                "and get that bag of Red",
                                "Plant Stem Powder to",
                                "Dono as soon as you can?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("hg_bio").get()? == 6 {
                        ctx.lines_as(
                            "Makkie",
                            args![
                                "Hey, have you seen",
                                "Morriphen and Dono",
                                "lately? I hope they're",
                                "doing alright. Maybe",
                                "I should go see them",
                                "soon, for old time's sake."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("hg_bio").get()? == 7 {
                        ctx.lines_as(
                            "Makkie",
                            args![
                                "Hey, have you seen",
                                "Morriphen and Dono",
                                "lately? I hope they're",
                                "doing alright. Maybe",
                                "I should go see them",
                                "soon, for old time's sake."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("hg_bio").get()? == 8 {
                        ctx.lines_as(
                            "Makkie",
                            args![
                                "Hey, have you seen",
                                "Morriphen and Dono",
                                "lately? I hope they're",
                                "doing alright. Maybe",
                                "I should go see them",
                                "soon, for old time's sake."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("hg_bio").get()? == 9 {
                        ctx.lines_as(
                            "Makkie",
                            args![
                                "Hey, have you seen",
                                "Morriphen and Dono",
                                "lately? I hope they're",
                                "doing alright. Maybe",
                                "I should go see them",
                                "soon, for old time's sake."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Makkie",
                            args!["Hey, welcome to the", "Lighthalzen Pub. Relax,", "and make yourself at home~"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
}

pub fn makkie(ctx: &Ctx) -> Script {
    makkie_body(ctx, Vec::new()).map(|_| ())
}

fn shede_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("siide1.bmp"), Val::from(2)])?;
    if ctx.var("hg_tre").get()? == 0 {
        ctx.lines_as(
            "Shede",
            args![
                "Yay~ are you talking to me? Yay!",
                "Hey, are you from a different city? Wow, nice to see you!"
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
        ctx.next()?;
        ctx.lines_as(
            "Shede",
            args![
                "Ever since the airport was built, Hugel has been so busy to welcome tourists and adventurers.",
                "I am kind of surprised to see Hugel being so crowded like this,",
                "but I think that this is a good change for this town."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Shede",
            args![
                "At the same time, I have so many customers to serve...",
                "In fact, I don't even have a time to cht-chat, hohoho."
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("Do you want me to help you?:I will leave you alone")],
            )?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Shede",
                    args!["Oh~ how kind of you! Thank you so much~", "Then can I ask you a favor?"],
                )?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
                ctx.next()?;
                ctx.mes("[Shede]")?;
                let subject2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                if subject2 == 1 {
                    ctx.lines(args![
                        "Uncle Hollun's birthday is coming,",
                        "so I want to make \"^3131FFMushroom Flavor Cookies^000000\" for him.",
                        "Unfortunately, I am short of the most important ingredient, the Mushrooms."
                    ])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("siide2.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Shede",
                        args![
                            "Ordinary mushrooms don't smell that good, but",
                            "\"^3131FFMoks Mushrooms^000000\" which only grow in Hugel have such appetizing scent.",
                            "So, when they are added to cookies, they enhance the taste of cookies so much better."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shede",
                        args![
                            "Can you please go gather \"^3131FF5 Moks Mushroom Solution^000000\" from \"^3131FFMoks Mushrooms^000000\"?",
                            "You will find the mushrooms in fields near Hugel.",
                            "I hope that you will bring their solution as quickly as you can!"
                        ],
                    )?;
                    ctx.var("hg_tre").set(Val::from(10))?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                } else if subject2 == 2 {
                    ctx.mes("In fact, Agette and I am competing with each other for the Hugel's best cookie master position.")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shede",
                        args![
                            "Becoming the best cookie mater is not an easy thing to achieve.",
                            "Everyone can have the best ingredients and the best cooking utensils,",
                            "but that doesn't mean that they can become the best cookie masters."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shede",
                        args![
                            "You need to know how to harmonize your skills and your senses",
                            "to make the most delicious cookies."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("siide2.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Shede",
                        args![
                            "I dare to say that my cookies are the best.",
                            "However, Agette is also a professional cookie baker."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shede",
                        args![
                            "If I hear her name while baking,",
                            "it makes me shake my hands in anxiety",
                            "so that I make a mistake to measure the right amount of ingredients.",
                            "That explains how good she is at baking cookies."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shede",
                        args![
                            "Still, I am a proud cookie baker, and",
                            "I can hear a voice from deep inside of me saying",
                            "that I cannot be beat by her!",
                            "Can you hear that?",
                            "Can you hear my inner fighting spirit as a cookie baker?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shede",
                        args![
                            "I am going to compete with her",
                            "for the Hugel's best cookie mater position after a while.",
                            "So, I have been doing my best in practicing my skills to win her!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shede",
                        args![
                            "There were many obstacles in my way to become the best cookie baker.",
                            "But, see? I am still here, alive and kicking! I will never give up! In that sense..",
                            "I am going to challenge on baking ^FF0000Clam Cookies^000000 today!",
                            "I am going to make them taste tangy and unique. I am sure that they will be a great success!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shede",
                        args![
                            "When you go to the beach, ",
                            "you will find clams near the ferry.",
                            "Please gather ^3131FF5 Clam Flesh^000000 from the clams.",
                            "Please hurry!"
                        ],
                    )?;
                    ctx.var("hg_tre").set(Val::from(20))?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                } else if subject2 == 3 {
                    ctx.lines(args![
                        "In fact, I gave my heart to a gentleman.",
                        "I think that the time has come to let him know",
                        "how I feel about him."
                    ])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("siide2.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Shede",
                        args![
                            "So, I decided to bake special cookies called...",
                            "...... ^3131FFDevil's Cookies^000000!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Devil's Cookies?"])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("siide1.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Shede",
                        args![
                            "Yes, Devil's Cookies.",
                            "When you go outside of Hugel, you will find \"^3131FFMoks Bugs^000000\"",
                            "hiding in the bushes. They breed in this season, so they are plump with eggs."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shede",
                        args![
                            "When you stew their shells and use the broth to kneed flour,",
                            "you can make fresh and crunch cookies."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("siide2.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Shede",
                        args![
                            "Devil's Cookies are so delicious, so they are rumored to dazzle the eaters with",
                            "the mysteriously crunch texture and the sweetness, and to allure them to hell without them knowing.",
                            "Ah~ I hope that he will like them..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("siide1.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Shede",
                        args!["Would you please help me to bake the cookies? I need 5 Moks Bug Shells~"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["'......I am confused...Does she love him or hate him?'"],
                    )?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_THINK")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.var("hg_tre").set(Val::from(30))?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as("Shede", args!["No, it is fine. Somehow I needed a break, hohoh!"])?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
        }
    } else {
        if (ctx.var("hg_tre").get()?.number()? > 9 && ctx.var("hg_tre").get()?.number()? < 15) {
            ctx.lines_as(
                "Shede",
                args![
                    "Can you please go gather \"^3131FF5 Moks Mushroom Solution^000000\" from \"^3131FFMoks Mushrooms^000000\"?",
                    "You will find the mushrooms in fields near Hugel.",
                    "I hope that you will bring their solution as quickly as you can!"
                ],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        } else {
            if (ctx.var("hg_tre").get()?.number()? > 19 && ctx.var("hg_tre").get()?.number()? < 25) {
                ctx.lines_as(
                    "Shede",
                    args![
                        "Please pick up \"^3131FF5 Clams^000000\" from a fish trap in the ferry.",
                        "I hope that you will bring their solution as quickly as you can!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            } else {
                if (ctx.var("hg_tre").get()?.number()? > 29 && ctx.var("hg_tre").get()?.number()? < 35) {
                    ctx.lines_as(
                        "Shede",
                        args![
                            "Please bring \"^3131FF5 Moks Bug Shells^000000\" from outside of Hugel.",
                            "I hope that you will bring their solution as quickly as you can!"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                } else if ctx.var("hg_tre").get()? == 15 {
                    ctx.lines_as(
                        "Shede",
                        args![
                            "Oh! You came back earlier than I thought.",
                            "Now, I can bake the Mushroom Flavor Cookies for uncle Hollun.",
                            "Thank you so much."
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                    ctx.var("hg_tre").set(Val::from(40))?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                } else if ctx.var("hg_tre").get()? == 25 {
                    ctx.lines_as(
                        "Shede",
                        args![
                            "Oh! You came back earlier than I thought.",
                            "Now, I can bake the best cookies of mine and compete with Agette!",
                            "Thank you so much."
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                    ctx.var("hg_tre").set(Val::from(40))?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                } else if ctx.var("hg_tre").get()? == 35 {
                    ctx.lines_as(
                        "Shede",
                        args![
                            "Oh! You came back earlier than I thought.",
                            "Now, I can bake Devil's Cookies to confess my love to the gentleman.",
                            "Thank you so much."
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "(If he knew what ingredients she used for the cookies,)",
                            "(I am pretty sure that at least he will remember her face to avoid next time...)'"
                        ],
                    )?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_THINK")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.var("hg_tre").set(Val::from(40))?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                } else if ctx.var("hg_tre").get()? == 40 {
                    if (ctx.var("lhz_boss").get()? == 44 && ctx.var("lhz_curse").get()?.number()? > 30) {
                        ctx.lines_as(
                            "Shede",
                            args!["Have you seen Mr. Herico?", "He is a lonesome old man who is staying at Hugel Inn."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Shede",
                            args![
                                "If you have some time, why don't you go spend some time with him?",
                                "I know that he will appreciate it."
                            ],
                        )?;
                        ctx.var("hg_tre").set(Val::from(41))?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Shede",
                            args!["So, how do you like Hugel?", "It is a peaceful village, isn't it?"],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("hg_tre").get()? == 41 {
                        ctx.lines_as(
                            "Shede",
                            args![
                                "Mr. Herico has an impediment in movement, so he usually stays in his room.",
                                "Please go spend some time with him. I know that he will appreciate it."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Shede",
                            args!["So, how do you like Hugel?", "It is a peaceful village, isn't it?"],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn shede(ctx: &Ctx) -> Script {
    shede_body(ctx, Vec::new()).map(|_| ())
}
