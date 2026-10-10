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

pub fn kunlun_envoy_gonryun(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Wa Bai Hu",
        args![
            "Good day~",
            "Let me invite you all",
            "to my homeland, Kunlun.",
            "It is my honor to guide",
            "such distinguished quests from",
            "the Rune-Midgarts kingdom."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["About Kunlun", "Visit Kunlun", "Cancel"])? {
        0 => {
            ctx.lines_as(
                "Wa Bai Hu",
                args![
                    "Kunlun is a beautiful place,",
                    "rich with history, and its",
                    "own traditions. I also think",
                    "Kunlun is the best place for",
                    "sightseeing."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Wa Bai Hu",
                args![
                    "When you arrive at the harbor of",
                    "Kunlun, you'll be able to see",
                    "miniature replicas of",
                    "buildings found in Alberta",
                    "and Prontera."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Wa Bai Hu",
                args![
                    "After enjoying a nice, leisurely",
                    "stroll, step into the beautiful",
                    "column of light that will take",
                    "you up into the clouds to",
                    "Kunlun Village."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Wa Bai Hu",
                args![
                    "I've heard that on the",
                    "Rune-Midgarts continent,",
                    "there is another city that is",
                    "is kept aloft in the sky by",
                    "an ancient, mysterious power..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Wa Bai Hu",
                args![
                    "Well, my Kunlun also floats",
                    "in the air, but without any",
                    "so called technology or",
                    "power supply. We consider",
                    "our land especially blessed..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Wa Bai Hu",
                args![
                    "When you're in Kunlun, don't",
                    "forget to try our specialties",
                    "such as the giant dumpling or the heaven peach."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Wa Bai Hu",
                args![
                    "You'd better prepare yourself",
                    "if you are planning to visit",
                    "the Kunlun dungeon. I must",
                    "say, that is not a safe place to go for fun."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Wa Bai Hu",
                args![
                    "If you are interested in visiting",
                    "Kunlun, do not hesitate to let",
                    "me know. It's my great pleasure",
                    "to serve you, honorable guest."
                ],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Wa Bai Hu",
                args![
                    "Excellent choice, I am glad",
                    "to have you as our guest~",
                    "However, a small fee is required",
                    "to board the ship to Kunlun."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Wa Bai Hu",
                args![
                    "We ask that you pay 10,000 zeny",
                    "prior to departure. That fee also",
                    "covers the cost of returning",
                    "to Alberta. I am ready to guide",
                    "you to Kunlun at any time."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Wa Bai Hu", args!["Would you like to board?"])?;
            ctx.next()?;
            if ctx.menu(&["To Kunlun~!", "No."])? == 0 {
                if ctx.player().zeny()? > 9999 {
                    ctx.lines_as("Wa Bai Hu", args!["Thank you, let me guide you there immediately."])?;
                    ctx.close_window()?;
                    ctx.player().set_zeny(ctx.player().zeny()? - 10000)?;
                    ctx.warp("gon_fild01", 258, 82)?;
                    return ctx.end();
                }
                ctx.lines_as(
                    "Wa Bai Hu",
                    args![
                        "I am sorry, but you must have",
                        "10,000 zeny to travel to Kunlun.",
                        "Please make sure you have enough",
                        "zeny with you. Thank you, and",
                        "please come again."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Wa Bai Hu",
                args![
                    "I see. However, whenever you",
                    "change your mind, please let",
                    "me know. It would be a great",
                    "please to serve you, most",
                    "honorable guest."
                ],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Wa Bai Hu",
                args![
                    "I see. However, whenever you",
                    "change your mind, please let me",
                    "know. It would be a great pleasure to serve you, most honorable guest."
                ],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn kunlun_envoy_gon2(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Wa Bai Hu",
        args![
            "So, did you enjoy your trip?",
            "I guess it's the time for you to",
            "go home. The ship to Rune-Midgarts is ready to depart at any time."
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Go back to Alberta", "Cancel"])? == 0 {
        ctx.lines_as(
            "Wa Bai Hu",
            args![
                "Please come again.",
                "I hope you will let your friends",
                "know about Kunlun when you get",
                "back. Now, let me guide you",
                "back to Alberta."
            ],
        )?;
        ctx.close_window()?;
        ctx.warp("alberta", 243, 67)?;
        return ctx.end();
    }
    ctx.lines_as(
        "Wa Bai Hu",
        args!["Take your time, my guest.", "There should be many places", "you may have missed."],
    )?;
    return ctx.close();
}

pub fn kunlun_envoy_gon3(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Zhang Quing Long",
        args![
            "Please head north to enter Kunlun.",
            "I hope you will have a great time",
            "while staying in Kunlun."
        ],
    )?;
    return ctx.close();
}

pub fn kunlun_envoy_gon4(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Zhang Quing Long",
        args![
            "Please make yourself comfortable.",
            "If you want to go back, I will",
            "be more than happy to guide you",
            "to the ship to Alberta."
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Go back to the harbor", "Cancel"])? == 0 {
        ctx.lines_as(
            "Zhang Quing Long",
            args!["I hope you enjoyed your trip.", "Now, let me guide you back", "to the harbor."],
        )?;
        ctx.close_window()?;
        ctx.warp("gon_fild01", 258, 82)?;
        return ctx.end();
    }
    ctx.lines_as(
        "Zhang Quing Long",
        args!["Take your time, my guest.", "There should be many places", "you may have missed."],
    )?;
    return ctx.close();
}

pub fn jian_chung_xun_gon(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Jian Chung Xun",
        args![
            "I simply adore festivals.",
            "That's why I love this town.",
            "This town makes me feel like I am",
            "in the middle of a festival all year round."
        ],
    )?;
    return ctx.close();
}

pub fn liang_zhun_bu_gon(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Liang Zhun Bu",
        args![
            "We are proud to be an independent",
            "nation, and have been fighting",
            "against the evil invaders who've",
            "wanted to conquer this blessed land for many years..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Liang Zhun Bu",
        args![
            "But we have victoriously fended",
            "off every invasion! As long",
            "as we believe in ourselves,",
            "we shall never forget the",
            "Triumphal Song that has helped us in our struggles."
        ],
    )?;
    return ctx.close();
}

pub fn qian_yuen_shuang_gon(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Qian Yuen Shuang",
        args![
            "The chief of this town is a man",
            "who opens his heart to others.",
            "However, I have heard that there",
            "are some people who don't like his personality..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Qian Yuen Shuang",
        args![
            "Well, I like my town. The Chief's",
            "efforts have made our town safer.",
            "I just hope other people feel the",
            "same way about what he has done."
        ],
    )?;
    return ctx.close();
}

pub fn jing_wen_zhen_gon(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Jing Wen Zhen",
        args![
            "The men in our town, Kunlun, are",
            "all brave and courageous.",
            "But, they are unable to get",
            "married. It's quite a shame really..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jing Wen Zhen",
        args![
            "It's all because there are",
            "more men than women.",
            "I am not even sure whether",
            "or not my son will be able to",
            "find me a daughter in law."
        ],
    )?;
    return ctx.close();
}

pub fn gatekeeper_gon(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Kunlun Guard",
        args!["Welcome.", "This is the residence of Shi Yan Wen, the chief of Kunlun."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kunlun Guard",
        args![
            "You better behave yourself while",
            "you are here. If we see anything",
            "suspicious, we'll arrest you in a heartbeat."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kunlun Guard",
        args![
            "However, rest assured, you seem",
            "like a trustworthy person.",
            "I'm sure nothing will happen. Enjoy your visit."
        ],
    )?;
    return ctx.close();
}

pub fn gatekeeper_gon2(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Kunlun Guard",
        args!["Welcome.", "This is the residence of Shi Yan Wen, the chief of Kunlun."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kunlun Guard",
        args![
            "You better behave yourself while",
            "you are here. If we see anything",
            "suspicious, we'll arrest you in a heartbeat."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kunlun Guard",
        args![
            "However, rest assured, you seem",
            "like a trustworthy person.",
            "I'm sure nothing will happen. Enjoy your visit."
        ],
    )?;
    return ctx.close();
}

pub fn ji_chung_zhe_gon(ctx: &Ctx) -> Script {
    if ctx.var("nakha").get()?.number()? >= 0 && ctx.var("nakha").get()?.number()? <= 2 {
        ctx.lines_as("Ji Chung Zhe", args!["............"])?;
        ctx.next()?;
        ctx.lines_as("Ji Chung Zhe", args!["puuuuu....This sure is", "something to worry about."])?;
        return ctx.close();
    }
    if ctx.var("nakha").get()?.number()? == 3 {
        ctx.lines_as(
            "Ji Chung Zhe",
            args![
                "I am Ji Chung Zhe, a renown brewer",
                "of teas. Everyday, I put all my",
                "efforts in making scrumptious, delicious tea."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ji Chung Zhe",
            args![
                "*Sigh* But lately, the tea I've",
                "been making hasn't been that",
                "great... If I only had some special ingredients..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ji Chung Zhe",
            args![
                "I've been told that if you use",
                "a snake, you can concoct a truly",
                "extraordinary beverage~",
                "But...where can I find one",
                "and how can I catch one?",
                "Hmm..."
            ],
        )?;
        return ctx.close();
    }
    Ok(())
}

pub fn yu_jiu_xia_gon(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Yu Jiu Xia",
        args![
            "Geez, just as I thought.",
            "They won't sell alcohol to me.",
            "Maybe its cuz I'm too young...",
            "Hmmm...I wonder how it tastes..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Yu Jiu Xia",
        args![
            "However, I know they're making",
            "some tasty tea that even kids",
            "like me can enjoy.",
            "It makes my mouth water just",
            "thinking about this new tea."
        ],
    )?;
    return ctx.close();
}

pub fn soldier_gon(ctx: &Ctx) -> Script {
    if ctx.var("b_sword").get()?.number()? < 7 {
        ctx.lines_as(
            "Wa Qiu Wu",
            args![
                "Let me tell you something",
                "interesting about this place~",
                "Long ago, this entire area used to be a shrine."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wa Qiu Wu",
            args![
                "In those days, Taoist hermits",
                "used to gather here in order to",
                "reach the Sky Kingdom. However,",
                "they failed miserably...slowly the monsters began to come..."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("b_sword").get()?.number()? > 6 && ctx.var("b_sword").get()?.number()? < 10 {
        ctx.lines_as(
            "Wa Qiu Wu",
            args![
                "Don't you think it was quite noisy",
                "last night? It was all because",
                "of that thief. He made quite",
                "a scene...It was so loud that",
                "I couldn't sleep at all..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wa Qiu Wu",
            args![
                "Ahh~~~~!",
                "In the middle of all that",
                "commotion, I saw",
                "something running straight",
                "into the shrine."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wa Qiu Wu",
            args![
                "It was moving so fast that",
                "I couldn't even tell what it was.",
                "From what I could recognize, it",
                "looked human. I wonder",
                "what it was..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wa Qiu Wu",
            args![
                "It might have been the",
                "thief, but it moved",
                "so fast, it seemed like",
                "just a blur."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Wa Qiu Wu",
        args!["Let me tell you something", "interesting~ This entire area", "used to be a shrine."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Wa Qiu Wu",
        args![
            "A long time ago, Taoist hermits",
            "used to gather here in order to",
            "reach the Sky Kingdom. However,",
            "they failed miserably...slowly the monsters began to come."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Wa Qiu Wu",
        args![
            "The town is getting ready for the",
            "Festival, but something is delaying",
            "it. This has never happened before..."
        ],
    )?;
    return ctx.close();
}

pub fn guidev_gon(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Li Xi Jiao",
        args![
            "Welcome to Kunlun!",
            "Did you enjoy all the incredible",
            "scenery on your way here?",
            "The buildings may be small, but we",
            "all worked hard to build this city."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Li Xi Jiao",
        args![
            "I have some miniatures of",
            "the Rune-Midgarts Kingdom.",
            "You can view all of Prontera in a",
            "single glance. The craftsmanship",
            "on these masterpieces is quite stunning!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Li Xi Jiao",
        args![
            "If you look around carefully,",
            "You'll find all sorts of beautiful",
            "sights throughout the town."
        ],
    )?;
    return ctx.close();
}
