use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn neha_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    if ctx.var("hg_odeng").get()? == 1 {
        if ctx.call(Function::CountItem, vec![Val::from(584)])?.is_true() {
            ctx.lines_as(
                "Neha",
                args![
                    "Oh, you have a delivery",
                    "from Cellette, hm? Hmpf!",
                    "You're late! Don't you know",
                    "this soup isn't good if it",
                    "isn't steaming hot?"
                ],
            )?;
            ctx.next()?;
            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                ctx.lines_as(
                    "Neha",
                    args![
                        "But... You're a handsome",
                        "fellow, so I'll forgive you...",
                        "On one condition. Bring me",
                        "^FF00001 Whip^000000 with which to spank ",
                        "you. You better hurry, boy, or",
                        "I won't let you off that easy."
                    ],
                )?;
            } else {
                ctx.lines_as(
                    "Neha",
                    args![
                        "Yes, yes... You should",
                        "be punished. You should be",
                        "spanked. If you want to finish",
                        "your delivery, then bring me",
                        "^FF00001 Whip^000000 so that I can spank you",
                        "for your impertinence, girl!"
                    ],
                )?;
            }
            l_i = Val::from(8064);
            'l1: loop {
                if !(l_i.clone().number()? <= 8067) {
                    break 'l1;
                }
                'b1: {
                    if (ctx.call(Function::CheckQuest, vec![l_i.clone()])?.number()? > -1
                        && ctx.call(Function::CheckQuest, vec![l_i.clone()])?.number()? < 2)
                    {
                        ctx.call(Function::CompleteQuest, vec![l_i.clone()])?;
                    }
                }
                l_i = (l_i.clone() + Val::from(1));
            }
            ctx.call(Function::SetQuest, vec![Val::from(8068)])?;
            ctx.var("hg_odeng").set(Val::from(6))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Neha",
                args![
                    "Oh ho ho ho ho ho~",
                    "I just love cute little boys...",
                    "But for some reason, it",
                    "thrills me to torture them",
                    "a little bit as well. Hoho~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("hg_odeng").get()? == 6 {
            if (ctx.call(Function::CountItem, vec![Val::from(1960)])?.is_true()
                && ctx.call(Function::CountItem, vec![Val::from(584)])?.is_true())
            {
                ctx.lines_as(
                    "Neha",
                    args![
                        "Finally... My soup!",
                        "And you brought me my Whip!",
                        "Don't worry, I'll only give you",
                        "just one hard spanking. It'll",
                        "be over before you know it~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Neha", args!["Ha!"])?;
                ctx.next()?;
                ctx.mes("^3355FF*Slap*^000000")?;
                ctx.call(Function::PercentHeal, vec![Val::from(-25), Val::from(0)])?;
                ctx.next()?;
                ctx.lines_as("Neha", args!["Heeyah!!"])?;
                ctx.next()?;
                ctx.mes("^3355FF*SLAP*^000000")?;
                ctx.call(Function::PercentHeal, vec![Val::from(-25), Val::from(0)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Neha",
                    args![
                        "Ooh, did I accidentally",
                        "whip you twice? Heh heh...",
                        "Well, I'm satisfied. I'll go",
                        "ahead and tell Celette that",
                        "you did a good job... I hope",
                        "you come deliver my soup again~"
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(1960), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(584), Val::from(1)])?;
                ctx.var("hg_odeng").set(Val::from(10))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8068), Val::from(8072)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Neha",
                    args![
                        "^FF00001 Fish Cake Soup^000000...",
                        "^FF00001 Whip^000000... Is that too",
                        "much for a girl to ask?",
                        "I'm hungry, and I want to",
                        "hit somebody! What's wrong",
                        "with that, huh? Now hurry up!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as(
                "Neha",
                args![
                    "Oh ho ho ho ho ho~",
                    "I just love cute little boys...",
                    "But for some reason, it",
                    "thrills me to torture them",
                    "a little bit as well. Hoho~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn neha(ctx: &Ctx) -> Script {
    neha_body(ctx, Vec::new()).map(|_| ())
}

fn maewan_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_odeng").get()? == 2 {
        if ctx.call(Function::CountItem, vec![Val::from(584)])?.is_true() {
            ctx.lines_as(
                "Maewan",
                args![
                    "Oh, finally, I've been",
                    "waiting for my order of",
                    "Fish Cake Soup from Cellette!",
                    "Aaaah, it smells oh so delicious~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Maewan",
                args![
                    "Argh, but you're late!",
                    "This is your fault, you",
                    "know that, right? Well,",
                    "it's not a big deal, but",
                    "I can forgive you if you",
                    "help me with my collection."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Maewan",
                args![
                    "Just bring me",
                    "1 Bookclip in Memory,",
                    "alright? Oh, and don't",
                    "eat my soup before",
                    "you deliver it to me!"
                ],
            )?;
            ctx.var("hg_odeng").set(Val::from(7))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(8065), Val::from(8069)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Maewan",
                args![
                    "I like to think of",
                    "myself as a man of",
                    "eclectic taste. I love",
                    "collecting strange and",
                    "unique items, and learning",
                    "all sorts of new things~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("hg_odeng").get()? == 7 {
            if (ctx.call(Function::CountItem, vec![Val::from(584)])?.is_true()
                && ctx.call(Function::CountItem, vec![Val::from(7015)])?.is_true())
            {
                ctx.lines_as(
                    "Maewan",
                    args![
                        "Great, you really brought",
                        "me a Bookclip in Memory!",
                        "Yes, this'll make a fine",
                        "addition to my collection~",
                        "Oh, and give me my soup.",
                        "Thanks again for delivering~"
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(584), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(7015), Val::from(1)])?;
                ctx.var("hg_odeng").set(Val::from(10))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8069), Val::from(8073)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Maewan",
                    args![
                        "I know it sounds unfair",
                        "of me to ask you to bring",
                        "^FF00001 Bookclip in Memory^000000 with",
                        "my ^FF0000Fish Cake Soup^000000, but well..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Maewan",
                    args![
                        "I'd feel left out if",
                        "I didn't do it. I know",
                        "all of Cellette's other",
                        "customers are asking the",
                        "people delivering their soup",
                        "for all sorts of crazy things."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as(
                "Maewan",
                args![
                    "I like to think of",
                    "myself as a man of",
                    "eclectic taste. I love",
                    "collecting strange and",
                    "unique items, and learning",
                    "all sorts of new things~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn maewan(ctx: &Ctx) -> Script {
    maewan_body(ctx, Vec::new()).map(|_| ())
}

fn layoma_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_odeng").get()? == 3 {
        if ctx.call(Function::CountItem, vec![Val::from(584)])?.is_true() {
            ctx.lines_as(
                "Layoma",
                args![
                    "You're here to deliver",
                    "my Fish Cake Soup?",
                    "How could you be so late?!",
                    "Don't you know that your",
                    "incompetence also reflects",
                    "on Cellette and her shop?!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Layoma",
                args![
                    "I can't let you continue",
                    "delivering for Cellette",
                    "unless you prove that you're",
                    "dependable to me. Erm, bring",
                    "me ^FF00001 Mushroom Spore^000000, and",
                    "I'll forget this whole thing."
                ],
            )?;
            ctx.var("hg_odeng").set(Val::from(8))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(8066), Val::from(8070)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Layoma",
                args![
                    "I love singing songs,",
                    "and I love eating mushrooms.",
                    "For a person living in Hugel,",
                    "that's a pretty exciting life!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("hg_odeng").get()? == 8 {
            if (ctx.call(Function::CountItem, vec![Val::from(584)])?.is_true()
                && ctx.call(Function::CountItem, vec![Val::from(921)])?.is_true())
            {
                ctx.lines_as(
                    "Layoma",
                    args![
                        "Great! You brought me",
                        "1 Mushroom Spore! This",
                        "will taste perfect with my",
                        "bowl of Fish Cake Soup~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Layoma",
                    args![
                        "Alright, maybe I can",
                        "count on you to work",
                        "for my roommate Cellette",
                        "a little while longer. Oh, and",
                        "please tell her to not to come home so late all the time, okay?"
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(921), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(584), Val::from(1)])?;
                ctx.var("hg_odeng").set(Val::from(10))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8070), Val::from(8074)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Layoma",
                    args![
                        "Hey! I thought you",
                        "wanted to prove that",
                        "you're a reliable and",
                        "trustworthy person! Go",
                        "and bring me 1 Mushroom",
                        "Spore with my Fish Cake Soup!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as(
                "Layoma",
                args![
                    "I love singing songs,",
                    "and I love eating mushrooms.",
                    "For a person living in Hugel,",
                    "that's a pretty exciting life!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn layoma(ctx: &Ctx) -> Script {
    layoma_body(ctx, Vec::new()).map(|_| ())
}

fn erjan_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_odeng").get()? == 4 {
        if ctx.call(Function::CountItem, vec![Val::from(584)])?.is_true() {
            ctx.lines_as(
                "Erjan",
                args![
                    "Well, I thought you'd",
                    "never come to deliver",
                    "my Fish Cake Soup. You're",
                    "awfully late, you know that?",
                    "Still, the smell is just so appetizing, I can barely resist..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Erjan",
                args![
                    "Hm, but as the first son",
                    "of the Franchefeschu family,",
                    "I cannot eat this soup unless",
                    "it is suitably prepared. Go!",
                    "I command you to bring me",
                    "the commoner's ^FF0000China^000000!"
                ],
            )?;
            ctx.next()?;
            ctx.var("hg_odeng").set(Val::from(9))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(8067), Val::from(8071)])?;
            ctx.lines_as(
                "Erjan",
                args![
                    "Realize that I'm compromising",
                    "with you by asking you to bring",
                    "me regular China... I always",
                    "enjoy my meals on the finest",
                    "glass and tableware available.",
                    "That's how we noblemen live~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Erjan",
                args![
                    "I am Erjan, first son of",
                    "the noble Franchefeschu",
                    "family. Commoner, if you",
                    "do not have any business",
                    "with me, then leave me be,",
                    "and do whatever it is you do."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("hg_odeng").get()? == 9 {
            if (ctx.call(Function::CountItem, vec![Val::from(584)])?.is_true()
                && ctx.call(Function::CountItem, vec![Val::from(736)])?.is_true())
            {
                ctx.lines_as(
                    "Erjan",
                    args![
                        "Ah, so you've delivered",
                        "my Fish Cake Soup with some",
                        "proper China. Yes, this pleases",
                        "me. Now, I may enjoy this meal",
                        "in a manner befitting the",
                        "Franchefeschu family."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Erjan",
                    args![
                        "Although this China is",
                        "below my standards, I also",
                        "understand that this is the",
                        "best you can do. Yes, to accept",
                        "this is my... noblesse oblige."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Erjan",
                    args![
                        "You are dismissed.",
                        "Please take your leave,",
                        "and return to Cellette",
                        "with my noble thanks."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(736), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(584), Val::from(1)])?;
                ctx.var("hg_odeng").set(Val::from(10))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8071), Val::from(8075)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Erjan",
                    args![
                        "Must I repeat myself?",
                        "Bring me my ^FF0000Fish Cake Soup^000000",
                        "with ^FF00001 China^000000 so that I may",
                        "properly enjoy my meal."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as(
                "Erjan",
                args![
                    "I am Erjan, first son of",
                    "the noble Franchefeschu",
                    "family. Commoner, if you",
                    "do not have any business",
                    "with me, then leave me be,",
                    "and do whatever it is you do."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn erjan(ctx: &Ctx) -> Script {
    erjan_body(ctx, Vec::new()).map(|_| ())
}

fn euslan_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("BaseLevel").get()?.number()? < 60 {
        ctx.lines_as(
            "Euslan",
            args!["^333333*Cough Cough*", "*Haaaaaaack*", "*C-cough* *Sob*^000000"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !(ctx.var("hg_ma1").get()?.is_true()) {
        ctx.lines_as(
            "Euslan",
            args!["^333333*Cough Cough*", "*Haaaaaaack*", "*C-cough* *Sob*^000000"],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Ignore:Speak to Euslan")])? {
            1 => {
                ctx.lines_as("Euslan", args!["^333333*Sniff*", "*Huff Huff*", "*Cough Cough*^000000"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Euslan", args!["*Huff Huff*", "*Cough Cough*^000000"])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFEusan is seized",
                    "with emotion: it's",
                    "been so long since",
                    "someone has tried",
                    "to speak to her.^000000"
                ])?;
                ctx.var("hg_ma1").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        if ctx.var("hg_ma1").get()? == 1 {
            ctx.lines_as(
                "Euslan",
                args!["^333333*Sniff* *Cough", "*Cough Cough*", "*Sob* *Sniff*^000000"],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Ignore:Speak to Euslan")])? {
                1 => {
                    ctx.lines_as("Euslan", args!["^333333*Sniff*", "*Huff Huff*", "*Cough Cough*^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as("Euslan", args!["^333333*Sniff*", "*Huff Huff*", "*Cough Cough*^000000"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Euslan",
                        args![
                            "Oh...",
                            "Hello, wh-what",
                            "^333333*Cough cough*^000000 did",
                            "you want? ^333333*Sniff*^000000"
                        ],
                    )?;
                    ctx.next()?;
                }
                _ => {}
            }
            match runtime::select_values(ctx, &[Val::from("Nothing!:Are you alright?")])? {
                1 => {
                    ctx.lines_as(
                        "Euslan",
                        args!["Oh, I'm s-sorry...", "^333333*Cough Cough*^000000 I must", "have been mistaken..."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Euslan",
                        args![
                            "Th-thank you so much",
                            "for your concern. My",
                            "^333333*Cough*^000000 name is Euslan,",
                            "and it's nice to meet you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Euslan",
                        args![
                            "I've just been waiting here",
                            "for my fiancee to return, but",
                            "I'm starting to worry since",
                            "I haven't heard from him in",
                            "days. I really want to go out",
                            "and find him on my own. ^333333*Cough*^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Euslan",
                        args![
                            "I really want to see my",
                            "Thierry. If only I wasn't",
                            "so sick, then I wouldn't have",
                            "these problems. ^333333*Sob*^000000 Where",
                            "is he? ^333333*Sniff*^000000 Where's my",
                            "Thierry? I want to see him..."
                        ],
                    )?;
                    ctx.next()?;
                }
                _ => {}
            }
            match runtime::select_values(ctx, &[Val::from("Leave her alone:Offer to find Thierry")])? {
                1 => {
                    ctx.lines_as(
                        "Euslan",
                        args![
                            "Thierry... ^333333*Sob*^000000",
                            "When are you",
                            "coming home?",
                            "I'm so worried...",
                            "^333333*Cough Cough*^000000"
                        ],
                    )?;
                    ctx.var("hg_ma1").set(Val::from(2))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Euslan",
                        args![
                            "...Are you really willing to",
                            "go out and look for Thierry",
                            "for me? Oh, thank you so",
                            "much! You don't know how",
                            "much your kind offer means",
                            "to me. Now, let's see..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Euslan",
                        args![
                            "^333333*Cough*^000000 Right, Thierry",
                            "went on a trip, traveling",
                            "on one of the Airships, to",
                            "find some medicine that the",
                            "doctors say I need for my",
                            "illness. ^333333*Cough Cough*^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Euslan",
                        args![
                            "I know it's like searching",
                            "for a needle in a haystack,",
                            "but the only way I can think",
                            "of finding Thierry is to ask",
                            "the Airship Crewmen if they",
                            "might know where my fiancee is."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Euslan",
                        args![
                            "^333333*Cough*^000000 Oh, right...",
                            "My fiancee was planning on",
                            "searching the Schwarzwald",
                            "Republic, so maybe the crewmen",
                            "on the Schwarzwald Republic domestic flights might know him."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Euslan",
                        args![
                            "If you manage to find",
                            "where Thierry might have",
                            "gone, then please let me",
                            "know as soon as you can,",
                            "okay? Thanks for cheering",
                            ((Val::from("me up, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
                        ],
                    )?;
                    ctx.var("hg_ma1").set(Val::from(3))?;
                    ctx.call(Function::SetQuest, vec![Val::from(8044)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            if ctx.var("hg_ma1").get()? == 2 {
                ctx.lines_as(
                    "Euslan",
                    args![
                        "I've just been waiting here",
                        "for my fiancee to return, but",
                        "I'm starting to worry since",
                        "I haven't heard from him in",
                        "days. I really want to go out",
                        "and find him on my own. ^333333*Cough*^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Euslan",
                    args![
                        "I really want to see my",
                        "Thierry. If only I wasn't",
                        "so sick, then I wouldn't have",
                        "these problems. ^333333*Sob*^000000 Where",
                        "is he? ^333333*Sniff*^000000 Where's my",
                        "Thierry? I want to see him..."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Leave her alone:Offer to find Thierry")])? {
                    1 => {
                        ctx.lines_as(
                            "Euslan",
                            args![
                                "Thierry... ^333333*Sob*^000000",
                                "When are you",
                                "coming home?",
                                "I'm so worried...",
                                "^333333*Cough Cough*^000000"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Euslan",
                            args![
                                "...Are you really willing to",
                                "go out and look for Thierry",
                                "for me? Oh, thank you so",
                                "much! You don't know how",
                                "much your kind offer means",
                                "to me. Now, let's see..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Euslan",
                            args![
                                "^333333*Cough*^000000 Right, Thierry",
                                "went on a trip, traveling",
                                "on one of the Airships, to",
                                "find some medicine that the",
                                "doctors say I need for my",
                                "illness. ^333333*Cough Cough*^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Euslan",
                            args![
                                "I know it's like searching",
                                "for a needle in a haystack,",
                                "but the only way I can think",
                                "of finding Thierry is to ask",
                                "the Airship Crewmen if they",
                                "might know where my fiancee is."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Euslan",
                            args![
                                "^333333*Cough*^000000 Oh, right...",
                                "My fiancee was planning on",
                                "searching the Schwarzwald",
                                "Republic, so maybe the crewmen",
                                "on the Schwarzwald Republic domestic flights might know him."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Euslan",
                            args![
                                "If you manage to find",
                                "where Thierry might have",
                                "gone, then please let me",
                                "know as soon as you can,",
                                "okay? Thanks for cheering",
                                ((Val::from("me up, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
                            ],
                        )?;
                        ctx.var("hg_ma1").set(Val::from(3))?;
                        ctx.call(Function::SetQuest, vec![Val::from(8044)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                if (ctx.var("hg_ma1").get()? == 3 || ctx.var("hg_ma1").get()? == 4) {
                    ctx.lines(args![
                        "^333333Euslan is still patiently",
                        "waiting for Thierry's return.",
                        "You should keep your promise",
                        "to her by asking the crewmen on",
                        "the Schwaltvalt Republic domestic flights about Euslan's fiancee.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("hg_ma1").get()? == 5 {
                        ctx.lines_as(
                            "Euslan",
                            args![
                                "Oh, you're back!",
                                "^333333*Cough*^000000 Maybe it's too",
                                "much to hope for, but did",
                                "you learn anything about",
                                "where my fiancee might be?"
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Well, I met Kaci on the Airship.")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Euslan",
                            args![
                                "Oh, Kaci works on the",
                                "Airship? It's been so long",
                                "since I've seen her. I guess",
                                "we've lost touch ever since",
                                "I've gotten sick... Still, I'm",
                                "glad to have heard from her."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Euslan",
                            args![
                                "Oh, how fortunate! So Kaci",
                                "knows where Thierry might be?",
                                "^333333*Cough*^000000 Ah, he went to Hugel.",
                                "I have a brother that lives",
                                "there, though it's been a while",
                                "since I've heard from him..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Euslan",
                            args![
                                "Um, if you don't mind, would",
                                "you please do another favor",
                                "for me? If you travel to Hugel,",
                                "would you find Thierry and tell",
                                "that I'm alright? He must be",
                                "so worried about he by now..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Euslan",
                            args![
                                "^333333*Cough Cough*^000000 It might",
                                "be a good idea to ask my",
                                "brother Eukran, who's working",
                                "as a Bingo game coordinator",
                                "in Hugel, since he might know",
                                "where Thierry is. ^333333*Cough*^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Euslan",
                            args![
                                "I know that it's a lot",
                                "to ask, so please don't",
                                "trouble yourself if it's out",
                                "of your way. Still, I'd really",
                                "appreciate your help if you",
                                "happen to pass through Hugel."
                            ],
                        )?;
                        ctx.var("hg_ma1").set(Val::from(6))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(8046), Val::from(8047)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("hg_ma1").get()? == 6 || ctx.var("hg_ma1").get()? == 7) {
                        ctx.lines_as(
                            "Euslan",
                            args![
                                "If you get the chance,",
                                "please visit my brother",
                                "Eukran in Hugel, and ask",
                                "him where my Thierry might be.",
                                "Thank you again for your kindness."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (((ctx.var("hg_ma1").get()? == 8 || ctx.var("hg_ma1").get()? == 9) || ctx.var("hg_ma1").get()? == 10)
                        || ctx.var("hg_ma1").get()? == 11)
                    {
                        ctx.lines(args![
                            "^3355FFEuslan is waiting for",
                            "you to bring her news from",
                            "her fiancee Thierry. You better",
                            "go back to Hugel to find him."
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("hg_ma1").get()? == 12 {
                        ctx.lines_as(
                            "Euslan",
                            args![
                                "Oh, you've returned!",
                                "^333333*Cough*^000000 Were you able to",
                                "speak to my brother and",
                                "find my fiancee Thierry?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^333333You tell Euslan that you",
                            "were able to find Thierry,",
                            "and that he has managed to",
                            "procure some medicine for",
                            "her. You give Euslan the",
                            "medicine as Thierry requested.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Euslan",
                            args![
                                "*Sniff* I don't deserve",
                                "such a wonderful man. Why",
                                "did that silly fool go through",
                                "so much trouble for a useless",
                                "woman like me. *Cough* I still don't understand it... I don't..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Euslan",
                            args![
                                "Thank you so much for",
                                "your help, adventurer, and",
                                "for bringing me this medicine.",
                                "I'll be waiting for Thierry to",
                                "come back home, and do my",
                                "best to get better. ^333333*Cough*^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Euslan",
                            args![
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                                "I'll never forget what",
                                "you've done for Thierry and",
                                "me. I'll always be praying for",
                                "your safety in your adventures.",
                                "Take care of yourself..."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.var("hg_ma1").set(Val::from(13))?;
                        ctx.call(Function::CompleteQuest, vec![Val::from(8052)])?;
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_ABSORBSPIRITS")?])?;
                        ctx.call(Function::GetExperience, vec![Val::from(900000), Val::from(600000)])?;
                        return Err(Stop::End);
                    } else if ctx.var("hg_ma1").get()? == 13 {
                        ctx.lines(args![
                            "^3355FFEuslan is still waiting",
                            "for her fiancee to return",
                            "home, but she appears",
                            "much more relieved and",
                            "relaxed than she used to be.^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn euslan(ctx: &Ctx) -> Script {
    euslan_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum HgQuestStep {
    Start,
    OnTouch,
}

fn hg_quest_run(ctx: &Ctx, mut step: HgQuestStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HgQuestStep::Start => {
                step = HgQuestStep::OnTouch;
                continue 'machine;
            }
            HgQuestStep::OnTouch => {
                if ctx.var("hg_ma1").get()? == 6 {
                    ctx.lines_as(
                        "Arcade Owner",
                        args!["Welcome to the", "Bingo Game Arcade.", "Would you like to", "play a game?"],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("I want to meet the game coordinator.")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Arcade Owner",
                        args![
                            "I'm sorry, but if he's on",
                            "duty, we can't really let you",
                            "meet him unless you come",
                            "here to play a game. Our",
                            "game coordinators tend",
                            "to be pretty busy..."
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

pub fn hg_quest(ctx: &Ctx) -> Script {
    hg_quest_run(ctx, HgQuestStep::Start, Vec::new()).map(|_| ())
}

pub fn hg_quest_ontouch(ctx: &Ctx) -> Script {
    hg_quest_run(ctx, HgQuestStep::OnTouch, Vec::new()).map(|_| ())
}

fn eukran_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_ma1").get()? == 6 {
        ctx.lines_as(
            "Eukran",
            args![
                "Oh, weren't you just",
                "in the bingo room?",
                "I hope you enjoyed",
                "the game. Now, can",
                "I help you with anything?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Talk about Euslan")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines(args![
            "^3355FFYou tell Eukran about his",
            "sister's illness, and that",
            "she has sent you to search",
            "for her fiancee Thierry.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Eukran",
            args![
                "*Sob* I had no idea",
                "that my sister Euslan",
                "was so sick! No wonder",
                "Thierry looked so depressed",
                "the last time I saw him. Why",
                "didn't he tell me anything?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Eukran",
            args![
                "You see, Thierry passed by",
                "here about a month ago, and",
                "told me that he had to visit",
                "the Odin Shrine. I wasn't",
                "sure then, but now I think",
                "he's trying to help my sister."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Eukran",
            args![
                "There must be something",
                "there that can help my",
                "sister recover. Why else",
                "would Thierry go to such a",
                "dangerous place? No one goes to the Odin Shrine for no reason."
            ],
        )?;
        ctx.close_window()?;
        ctx.var("hg_ma1").set(Val::from(8))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8047), Val::from(8048)])?;
        ctx.call(Function::Warp, vec![Val::from("que_bingo"), Val::from(37), Val::from(24)])?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn eukran(ctx: &Ctx) -> Script {
    eukran_body(ctx, Vec::new()).map(|_| ())
}

pub fn hiddenactivator_hugel(ctx: &Ctx) -> Script {
    hiddenactivator_hugel_run(ctx, HiddenactivatorHugelStep::Start, Vec::new()).map(|_| ())
}

pub fn hiddenactivator_hugel_ontouch(ctx: &Ctx) -> Script {
    hiddenactivator_hugel_run(ctx, HiddenactivatorHugelStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn hiddenactivator_hugel_ontimer180000(ctx: &Ctx) -> Script {
    hiddenactivator_hugel_run(ctx, HiddenactivatorHugelStep::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn hiddenactivator_hugel_oninit(ctx: &Ctx) -> Script {
    hiddenactivator_hugel_run(ctx, HiddenactivatorHugelStep::OnInit, Vec::new()).map(|_| ())
}

fn young_man_hu_quest_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_ma1").get()? == 8 {
        ctx.lines_as(
            "Young Man",
            args![
                "Hm? Oh, an adventurer.",
                "There's more and more",
                "of you coming to visit this",
                "Odin Shrine nowadays."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Mr. Thierry?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Thierry",
            args!["...?!", "H-how do you", "know my name?", "Do I know you", "from somewhere?"],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Tell him about Euslan")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Thierry",
            args![
                "Oh, Euslan sent you to me?",
                "I'm so sorry that you had to",
                "go through so much trouble.",
                "Still, I'm glad to hear from her. Please let her know that I'm",
                "coming home soon, would you?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("What are you doing here?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Thierry",
            args![
                "Well, um, I'm sorry.",
                "I can't really go into the",
                "details, but I'm doing this",
                "work to find a cure for",
                "Euslan's disease. It's not",
                "curable by modern medicine..."
            ],
        )?;
        ctx.close_window()?;
        ctx.var("hg_ma1").set(Val::from(9))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8048), Val::from(8049)])?;
        return Err(Stop::End);
    } else if ctx.var("hg_ma1").get()? == 9 {
        ctx.lines_as(
            "Thierry",
            args![
                "I can't tell you more",
                "than that. Please just",
                "trust me on this. In the",
                "meantime, would you tell",
                "Euslan that I should be",
                "coming home very soon?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("hg_ma1").get()? == 10 {
        ctx.lines_as(
            "Thierry",
            args![
                "Oh, I thought you",
                "already left Odin Shrine.",
                "You don't have anything ",
                "else to tell me, do you?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Warn Thierry about Suspicious Men")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Thierry",
            args![
                "What?! It's just what",
                "I thought! They were just",
                "using me this whole time...!",
                "Still, I can't leave this",
                "place just yet... I can't..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thierry",
            args![
                "I guess it all started when",
                "Euslan contracted that weird",
                "disease. None of the doctors",
                "knew what it was, none of them",
                "could do anything about it.",
                "It was totally devastating..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thierry",
            args![
                "Finally, I took her to see",
                "Mawong, who told us that he",
                "actually met someone that had",
                "that disease, and that there",
                "was a way for me to cure it.",
                "I had to take my chances..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thierry",
            args![
                "Mawong told me that the cure",
                "was somewhere near the Odin",
                "Shrine in Hugel. As you already",
                "know, this place is incredibly",
                "dangerous. But I don't care.",
                "I'll do anything for Euslan."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thierry",
            args![
                "Of course, I only told Euslan",
                "that I'd be looking around the",
                "Schwarzwald Republic... If she",
                "knew I was at the Odin Shrine,",
                "she'd worry too much about me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thierry",
            args![
                "Once I got here, I didn't",
                "know what to do, or what I was",
                "even looking for. Then, I ran",
                "into these strange men, and",
                "told them my story out of my",
                "frustration over my situation."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thierry",
            args![
                "It seemed like a miracle",
                "when they said that they knew",
                "how to cure Euslan. They told",
                "me that they were a research",
                "team working on a project",
                "related to my dilemna."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thierry",
            args![
                "I didn't even think twice",
                "about it: I immediately",
                "accepted their offer to",
                "join them. I did whatever",
                "research they asked while",
                "I searched for Euslan's cure..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thierry",
            args![
                "Later, I overheard them",
                "talking, and I learned that",
                "they were really Rekenber",
                "Corporation agents. Their",
                "project had nothing to do",
                "with medicine or my Euslan..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thierry",
            args![
                "Their true goal is to improve",
                "their guardian technology by",
                "finding the remains of some",
                "giant beneath the Odin Shrine.",
                "Still, working with them gave",
                "me access to so many resources."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thierry",
            args![
                "I've actually been able to",
                "find a cure for Euslan's",
                "illness through my research",
                "with those men, so I can't",
                "quit now. I still need to make",
                "that medicine for Euslan!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thierry",
            args![
                "So, behind their backs, I've",
                "been using their guardians to",
                "gather the ingredients needed",
                "for my fiancee's medicine. But",
                "if they're planning to get rid of me, I've just run out of time."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thierry",
            args![
                "It doesn't matter what",
                "happens to me, but I must",
                "save Euslan. I only need",
                "^FF00005 Runes of the Darkness^000000 to",
                "finish that medicine. Please",
                "help me get them, adventurer..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thierry",
            args![
                "You're the only one I can",
                "trust to save Euslan. In the",
                "meantime, I'll be hiding from",
                "those men right over here.",
                "If they get me, I'll never be",
                "able to make the medicine!"
            ],
        )?;
        ctx.close_window()?;
        ctx.var("hg_ma1").set(Val::from(11))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8050), Val::from(8051)])?;
        return Err(Stop::End);
    } else if ctx.var("hg_ma1").get()? == 11 {
        if ctx.call(Function::CountItem, vec![Val::from(7511)])?.number()? < 5 {
            ctx.lines_as(
                "Thierry",
                args![
                    "I understand that gathering",
                    "materials here in the Odin",
                    "Shrine is difficult, but try to",
                    "get me ^3355FF5 Runes of the Darkness^000000",
                    "as soon as you can. My Euslan",
                    "really needs that medicine..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Thierry",
            args![
                "Oh! You really brought me",
                "the Runes of the Darkness!",
                "I can finally make the medicine",
                "that will save my dear Euslan.",
                "I... I can't thank you enough",
                "for this. I'm so happy..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thierry",
            args![
                "Please give me a moment:",
                "I want to make this right",
                "away. Let's see, ah, here",
                "are my research notes..."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args!["......", ".........", "............"])?;
        ctx.next()?;
        ctx.lines_as(
            "Thierry",
            args![
                "It's finally done. This",
                "medicine will cure Euslan.",
                "If you'd do one last thing",
                "for me, please bring this to",
                "her. I've got to sneak away",
                "later after things calm down."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thierry",
            args![
                "Please understand that",
                "I want to see her as soon",
                "as I can, but I have to wait",
                "until those Rekenber agents",
                "stop searching for me. And let",
                "Eslan know I'll come home soon."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou received the",
            "medicine that Thierry",
            "has made to give to Euslan.^000000"
        ])?;
        ctx.close_window()?;
        ctx.call(Function::DelItem, vec![Val::from(7511), Val::from(5)])?;
        ctx.var("hg_ma1").set(Val::from(12))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8051), Val::from(8052)])?;
        return Err(Stop::End);
    } else if ctx.var("hg_ma1").get()? == 12 {
        ctx.lines_as(
            "Thierry",
            args![
                "Please give that",
                "medicine to Euslan as",
                "soon as you can: her life",
                "depends on it. Please let",
                "her know that I'm coming",
                "home as soon as I can."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Researcher",
            args![
                "I'm sorry, Euslan...",
                "Please just wait for",
                "me a little while longer.",
                "I swear I'll come home to you!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn young_man_hu_quest(ctx: &Ctx) -> Script {
    young_man_hu_quest_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum HiddenhugelStep {
    Start,
    OnTouch,
}

fn hiddenhugel_run(ctx: &Ctx, mut step: HiddenhugelStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HiddenhugelStep::Start => {
                step = HiddenhugelStep::OnTouch;
                continue 'machine;
            }
            HiddenhugelStep::OnTouch => {
                if ctx.var("hg_ma1").get()? == 9 {
                    ctx.lines(args![
                        "^3355FFYou hear the voice of",
                        "the suspicious man that",
                        "was speaking to Thierry",
                        "a little while ago. You",
                        "carefully listen to what",
                        "he is saying...^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Suspicious Man",
                        args![
                            "Thierry seems to have",
                            "caught on to what we're",
                            "really up to. I think that's why he's slowing down on his",
                            "research. We better get rid",
                            "of him before it's too late."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Very Suspicious Man",
                        args![
                            "Sure. Just let me",
                            "know when you",
                            "want him dead.",
                            "Then I'll kill him.",
                            "No problem."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFThis isn't good!",
                        "Thierry's in danger, so",
                        "you better go tell him",
                        "about this right now!^000000"
                    ])?;
                    ctx.close_window()?;
                    ctx.var("hg_ma1").set(Val::from(10))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(8049), Val::from(8050)])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn hiddenhugel(ctx: &Ctx) -> Script {
    hiddenhugel_run(ctx, HiddenhugelStep::Start, Vec::new()).map(|_| ())
}

pub fn hiddenhugel_ontouch(ctx: &Ctx) -> Script {
    hiddenhugel_run(ctx, HiddenhugelStep::OnTouch, Vec::new()).map(|_| ())
}
