use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn event_switch_pc_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn event_switch_pc(ctx: &Ctx) -> Script {
    event_switch_pc_body(ctx, Vec::new()).map(|_| ())
}

fn event_switch_pc_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((((ctx.var("prt_curse").get()? == 36 || ctx.var("prt_curse").get()? == 45) || ctx.var("prt_curse").get()? == 56)
        || ctx.var("prt_curse").get()? == 61)
        && ctx.var("aru_monas").get()?.number()? < 1)
    {
        ctx.call(Function::EnableNpc, vec![Val::from("Agent#pc1")])?;
        ctx.lines_as(
            "????",
            args![
                ((Val::from("Are you ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
                "I've been waiting for you.",
                "^6B8E23Priest Bamph^000000 is expecting",
                "your arrival at ^6B8E23Prontera",
                "Church^000000. Please visit him",
                "as soon as you can."
            ],
        )?;
        ctx.var("aru_monas").set(Val::from(1))?;
        ctx.close_window()?;
        ctx.call(Function::DisableNpc, vec![Val::from("Agent#pc1")])?;
    }
    return Err(Stop::End);
}

pub fn event_switch_pc_ontouch(ctx: &Ctx) -> Script {
    event_switch_pc_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn agent_pc1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn agent_pc1(ctx: &Ctx) -> Script {
    agent_pc1_body(ctx, Vec::new()).map(|_| ())
}

fn agent_pc1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Agent#pc1")])?;
    return Err(Stop::End);
}

pub fn agent_pc1_oninit(ctx: &Ctx) -> Script {
    agent_pc1_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn ordinary_man_pc1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()?.number()? < 2 {
        ctx.lines_as(
            "Larjes",
            args![
                "Damn it, I lost again!",
                "This game is rigged!",
                "No matter what I try,",
                "I can never seem to win!",
                "But Comodo Hold 'Em",
                "is so... It's so addicting!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_monas").get()? == 2 {
        ctx.lines_as(
            "Larjes",
            args![
                "Argh, I lost at this game",
                "so many times that I've",
                "completely lost track!",
                "I've got to win sometime..."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Excuse me.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Larjes", args!["Oh! Uh... Hmm...", "How may I help you?"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("^6B8E23Priest Bamph^000000 sent me.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Larjes",
            args![
                "Oh, I see. I apologize",
                "if my screaming and yelling",
                "made you a bit uncomfortable.",
                "I'm just here to kill some",
                "time... I now see firsthand",
                "that gambling is truly evil!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Larjes",
            args![
                "Er, would you come",
                "over this way? I don't",
                "want anyone else to",
                "overhear us talking."
            ],
        )?;
        ctx.close_window()?;
        ctx.var("aru_monas").set(Val::from(3))?;
        ctx.call(Function::Warp, vec![Val::from("cmd_in02"), Val::from(110), Val::from(53)])?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Larjes",
            args!["What?! I lost again?", "How does... How does", "this game even work?!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn ordinary_man_pc1(ctx: &Ctx) -> Script {
    ordinary_man_pc1_body(ctx, Vec::new()).map(|_| ())
}

fn ordinary_man_pc2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()?.number()? < 3 {
        ctx.lines_as(
            "Larjes",
            args!["^666666*Sigh...*^000000", "This isn't good at all.", "I don't know what to do..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_monas").get()? == 3 {
        ctx.lines_as(
            "Larjes",
            args![
                "Good, no one should be",
                "able to hear us from here.",
                "After I sent a message to",
                "the place, I found some",
                "new information that should",
                "really help my investigation."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Larjes",
            args![
                "I learned that two details",
                "about the night when that",
                "high ranking man vanished.",
                "Firstly, he left the guards for",
                "a bit to meet a woman. It's",
                "always a woman, isn't it?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Larjes",
            args![
                "Secondly, after he left to",
                "meet that woman, some other",
                "people came in, and then they",
                "carried some really large",
                "baggage with them when",
                "they left to the west."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Larjes",
            args![
                "Those bags were so big,",
                "it's suspicious. It seems",
                "like some organization has",
                "kidnapped him, and wants to",
                "bring him west for some reason."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Why to the west?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Larjes",
            args![
                "This is just a guess, but",
                "I think whoever kidnapped",
                "him are from ^2F4F2FArunafeltz^000000.",
                "I'm prohibited from entering",
                "Arunafeltz so I wasn't able",
                "to investigate any further."
            ],
        )?;
        ctx.var("aru_monas").set(Val::from(4))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Larjes",
            args!["I gambled away", "this much money?", "How am I gonna pay", "off all of this debt?!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn ordinary_man_pc2(ctx: &Ctx) -> Script {
    ordinary_man_pc2_body(ctx, Vec::new()).map(|_| ())
}

fn waiter_pc_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()?.number()? < 7 {
        ctx.lines_as(
            "Waiter",
            args![
                "Welcome to",
                "Rachel's Palate, one",
                "of Rachel's finest and",
                "most exquisite restraunts."
            ],
        )?;
        if ctx.var("aru_monas").get()?.number()? < 6 {
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.var("aru_monas").get()? == 6 {
            ctx.next()?;
            ctx.lines_as(
                "Waiter",
                args![
                    "What would you like",
                    "to order? Or... If I'm",
                    "not mistaken, you're",
                    "here for something else?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("I'd like to order.:Yes, I've come for another reason.")],
            )?) == 1
            {
                ctx.lines_as("Waiter", args!["Sure, no problem.", "Did you come alone?"])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Yes.")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as("Waiter", args!["Right this way."])?;
                ctx.close_window()?;
                ctx.var("aru_monas").set(Val::from(7))?;
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                if subject1 == 1 {
                    ctx.call(Function::Warp, vec![Val::from("ra_in01"), Val::from(303), Val::from(43)])?;
                    return Err(Stop::End);
                } else if subject1 == 2 {
                    ctx.call(Function::Warp, vec![Val::from("ra_in01"), Val::from(304), Val::from(43)])?;
                    return Err(Stop::End);
                } else if subject1 == 3 {
                    ctx.call(Function::Warp, vec![Val::from("ra_in01"), Val::from(304), Val::from(39)])?;
                    return Err(Stop::End);
                } else if subject1 == 4 {
                    ctx.call(Function::Warp, vec![Val::from("ra_in01"), Val::from(303), Val::from(39)])?;
                    return Err(Stop::End);
                }
            }
            ctx.lines_as(
                "Waiter",
                args![
                    "Hm, okay. Well, whatever",
                    "it is, I hope you enjoy your",
                    "stay at Rachel's Palate."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as(
        "Waiter",
        args![
            "The food in this",
            "restaurant is the",
            "best in Rachel--no,",
            "it is the best in all",
            "of Arunafeltz!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn waiter_pc(ctx: &Ctx) -> Script {
    waiter_pc_body(ctx, Vec::new()).map(|_| ())
}

fn normal_looking_man_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Normal-Looking Man",
        args![
            "Mmm-Mmm!",
            "I love the food in",
            "this restaurant!",
            "They don't serve",
            "anything this this",
            "good anywhere else!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn normal_looking_man(ctx: &Ctx) -> Script {
    normal_looking_man_body(ctx, Vec::new()).map(|_| ())
}

fn common_looking_man_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Common-Looking Man",
        args![
            "The food here isn't bad,",
            "but my favorite restaurants",
            "are over in Hugel. The food",
            "over there is definitely best."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn common_looking_man(ctx: &Ctx) -> Script {
    common_looking_man_body(ctx, Vec::new()).map(|_| ())
}

fn suspicious_looking_man_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Suspicious-Looking Man",
        args![
            "Each dish has its own",
            "unique taste that you must",
            "learn to relish. Of course,",
            "unhealthy junk food is the",
            "exception to this rule."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn suspicious_looking_man(ctx: &Ctx) -> Script {
    suspicious_looking_man_body(ctx, Vec::new()).map(|_| ())
}

fn mealconversation_trigger_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mealconversation_trigger(ctx: &Ctx) -> Script {
    mealconversation_trigger_body(ctx, Vec::new()).map(|_| ())
}

fn mealconversation_trigger_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(200)])? == 0 {
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
    if ctx.var("aru_monas").get()? == 7 {
        ctx.lines_as(
            "Waiter",
            args![
                "What would you like to order?",
                "Our special today is Fried",
                "Veins Stripe Stickleback",
                "since we received some",
                "high quality fish from Veins",
                "just this morning."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("...Yes, that sounds good.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Waiter",
            args![
                "Excellent choice.",
                "Just to let you know,",
                "that dish is ^9932CD3,000 zeny^000000.",
                "I'll be right back."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThe waiter took your",
            "order, and left to go to",
            "the kitchen. You decide to",
            "eavesdrop on the people",
            "talking at the next table",
            "while you're waiting.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Husky Male Voice",
            args![
                "...............................",
                "I really saw it, the greatest",
                "fish in the world! Last time",
                "I went to Veins, I glimpsed",
                "the Veins Golden Stripe",
                "Stickleback! It's true!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gloomy Male Voice",
            args!["So were you able to taste", "that Veins Golden Stripe", "Stickleback juice?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Average Male Voice",
            args![
                "Aw man, if I did, that",
                "would've been a once",
                "in a lifetime experience!",
                "Veins Golden Stripe",
                "Sticklebacks are only",
                "caught once a decade!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Husky Male Voice",
            args![
                "Oh yeah, that juice is",
                "incredible. My voice even",
                "changed after I drank it!",
                "It was just... So profound."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Average Male Voice", args!["Oh! So that's why", "you sound different."])?;
        ctx.next()?;
        ctx.lines_as(
            "Gloomy Male Voice",
            args![
                "How did you get to drink",
                "such rare juice? I mean,",
                "there must be tons of people",
                "in line with reservations,",
                "just waiting to drink it!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Husky Male Voice",
            args![
                "I got damn lucky: I went",
                "to this tavern in Veins, and",
                "overheard that ^DB7093a group of",
                "smugglers were arrested",
                "by the guards at the",
                "southeast beach^000000."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Husky Male Voice",
            args![
                "Then I started thinking:",
                "maybe the smugglers were",
                "fishing. So I checked the",
                "beach to see if I could",
                "find their fishing boat."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Average Male Voice",
            args![
                "Man, you must be obsessed",
                "with food. I mean, how else",
                "would you figure out that the",
                "smugglers were illegally",
                "fishing so quickly?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Husky Male Voice",
            args![
                "Ha, I guess you're right!",
                "Anyway, there was only one",
                "fishing boat, and the guards",
                "were inspecting it. I was about",
                "to turn back in disappointment",
                "but then... I saw a fisherman!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Husky Male Voice",
            args![
                "Out of the corner of my eye,",
                "I saw him bringing in his",
                "catch for the day, Veins",
                "Golden Strip Stickleback!",
                "I bought all of his fish",
                "right then and there."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gloomy Male Voice",
            args![
                "Boy, were you lucky.",
                "And you never hesitate",
                "to splurge when it comes",
                "to food, do you? Heh heh~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Husky Male Voice",
            args![
                "Oh, that wasn't even a",
                "splurge. I'm sure you guys",
                "would do the same thing",
                "if you were in my situation.",
                "I'm just grateful that I was",
                "able to taste such rare fish."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FF...............................",
            "Then, the men started",
            "to compliment each other.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Waiter",
            args!["Ah, here's your Fried", "Veins Stripe Stickleback.", "Please enjoy your meal."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "^ff0000Veins^000000, huh? I might want",
                "to check that place one",
                "of these days to see if",
                "I can find more of this fish."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Whoa, this fish tastes...",
                "I've never tasted anything",
                "like it! It's almost like",
                "a dessert. God, it's good!"
            ],
        )?;
        ctx.var("aru_monas").set(Val::from(8))?;
        if ctx.var("Zeny").get()?.number()? > 3000 {
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(3000))?))?;
            ctx.call(Function::GetItem, vec![Val::from(12052), Val::from(4)])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn mealconversation_trigger_ontouch(ctx: &Ctx) -> Script {
    mealconversation_trigger_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn magistrate_aru_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()?.number()? < 8 {
        ctx.lines_as(
            "Al Hamad",
            args![
                "Argh! Why must this",
                "happen to me, especially",
                "just when I'm about to",
                "retire! A-ah! My ulcer...",
                "Just all of a sudden, it--!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("aru_monas").get()? == 8 {
            ctx.lines_as(
                "Al Hamad",
                args![
                    "Man, this is probably",
                    "the biggest hassle of my",
                    "career. Why does this have",
                    "to happen now? Retirement",
                    "is so close, yet so far."
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Are you in trouble?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Al Hamad",
                args![
                    "Huh? You don't look like",
                    "you're from around here.",
                    "Listen, I appreciate your",
                    "concern, but I'm really not",
                    "comfortable talking to complete",
                    "strangers about my problems."
                ],
            )?;
            ctx.var("aru_monas").set(Val::from(9))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("aru_monas").get()? == 9 {
            ctx.lines_as(
                "Al Hamad",
                args![
                    "Unless you've got special",
                    "business here, you're not",
                    "allowed to enter this place.",
                    "We're here to keep the public",
                    "peace, and letting anyone",
                    "in here will cause trouble."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("aru_monas").get()? == 10 {
            ctx.lines_as(
                "Al Hamad",
                args![
                    "I thought I told you",
                    "that you weren't allowed",
                    "to come in here. Why do",
                    "you keep coming back?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("I'm sorry.:I want to know about the smugglers.")],
            )?) == 1
            {
                ctx.lines_as(
                    "Al Hamad",
                    args![
                        "It's fine. Just leave,",
                        "and don't come back",
                        "unless you've got a",
                        "really good reason."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Al Hamad",
                args![
                    "Hey, how'd you find out",
                    "about the smugglers? Did",
                    "word spread around town?",
                    "Well, I'm sorry, but I can't",
                    "disclose any details about",
                    "an ongoing investigation."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Al Hamad",
                args![
                    "^9370DBIf you insist on learning",
                    "more, then you'd need to",
                    "bring me a written order",
                    "from a higher ranking officer.^000000",
                    "Oh, and you're prohibited",
                    "asking anyone else, so behave."
                ],
            )?;
            ctx.var("aru_monas").set(Val::from(11))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("aru_monas").get()?.number()? < 13 {
            ctx.lines_as(
                "Al Hamad",
                args![
                    "I don't blame you for",
                    "being curious, but getting",
                    "involved when you're not",
                    "supposed to will just cause",
                    "trouble. Be careful, got it?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("aru_monas").get()? == 13 {
            ctx.lines_as(
                "Al Hamad",
                args![
                    "Huh, that's weird. You're not",
                    "even a citizen of Arunafeltz,",
                    "but somehow you know such",
                    "a high ranking priest. Alright,",
                    "I guess I can tell you more",
                    "about our investigation."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Al Hamad",
                args![
                    "I should let you know that",
                    "what I tell you might not be",
                    "worth all that trouble. This",
                    "information probably isn't",
                    "as valuable as you think."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Al Hamad",
                args![
                    "The smugglers we caught are",
                    "the worst kind of scoundrels:",
                    "they deal in human trafficking, but we caught them this time when they",
                    "were crossing the ocean. Their background info is pretty typical."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Al Hamad",
                args![
                    "Let's see... They kidnapped",
                    "some man from Comodo.",
                    "That's a place in the southwest",
                    "part of Rune-Midgarts. Anyway,",
                    "the smugglers thought he'd be",
                    "wealthy, so they set a trap."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Al Hamad",
                args![
                    "They lured him away from his",
                    "guards by sending a woman,",
                    "but when they captured him,",
                    "they found that he was not only",
                    "rich, he was also a high ranked",
                    "official of Rune-Midgarts."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Al Hamad",
                args![
                    "That's when they got sloppy:",
                    "they were so scared that the",
                    "kingdom's troops might pursue",
                    "them that they crossed the",
                    "ocean that we patrol. That's",
                    "where we caught them."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Al Hamad",
                args![
                    "What I don't understand is",
                    "why they were so scared.",
                    "I mean, they mainly deal",
                    "in human trafficking so...",
                    "Kidnapping an official isn't",
                    "that big a difference, is it?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Al Hamad",
                args![
                    "We contacted the temple,",
                    "and reported the smugglers,",
                    "our investigation, and the",
                    "high ranking man from the",
                    "Rune-Midgarts kingdom."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Al Hamad",
                args![
                    "Then, some people from",
                    "the temple interrupted our",
                    "investigation, took away the",
                    "smugglers before we finished",
                    "our interrogations, and we",
                    "haven't seen them since."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Al Hamad",
                args![
                    "I asked an acquaintance that",
                    "works at the temple, but he",
                    "doesn't know what's going",
                    "on either. The ^32CD32Rune-Midgarts",
                    "official^000000 has gone missing too.",
                    "It's all very frustrating."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Al Hamad",
                args![
                    "That happens to be all",
                    "I know. Does this mean",
                    "we have to capture the",
                    "smugglers again? I don't",
                    "know what's going on..."
                ],
            )?;
            ctx.var("aru_monas").set(Val::from(14))?;
            ctx.next()?;
            ctx.lines_as(
                "Al Hamad",
                args![
                    "Even if I could make headway",
                    "into the investigation, I've",
                    "been ordered to stop by upper",
                    "management. It looks like",
                    "this case is more trouble",
                    "than it's worth to them?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Al Hamad",
                args![
                    "I don't want to be involved",
                    "in this anymore. I should be",
                    "retired already! If you want to",
                    "learn more, your only choice",
                    "would be to find out on your",
                    "own. Fat chance, right?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Al Hamad",
                args!["I've suffered enough", "already! Why can't this", "case just be done?!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn magistrate_aru(ctx: &Ctx) -> Script {
    magistrate_aru_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_aru_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()?.number()? < 9 {
        ctx.lines_as(
            "Himus",
            args![
                "Huh. The magistrate",
                "still seems pretty upset",
                "over the case that happened",
                "at least a few months ago..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_monas").get()? == 9 {
        ctx.lines_as(
            "Himus",
            args![
                "Hm? Oh, the magistrate",
                "gave you the brush-off?",
                "Well, he hasn't really been",
                "himself ever since we arrested",
                "those ^32CD32smugglers coming from",
                "the south^000000 a few months ago."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Himus",
            args![
                "Once we reported the arrest",
                "to the temple, the pope's",
                "soldiers came and took the",
                "smugglers away. We have no",
                "clue what happened to them",
                "or where they even are now."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Himus",
            args![
                "It might not seem like",
                "a big deal to you, but",
                "the magistrate is pretty",
                "meticulous, and he hates",
                "it when people interfere",
                "with his job, you know?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Who are the smugglers?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Himus",
            args![
                "Well, I don't really know.",
                "Maybe the magistrate would",
                "know if he was able to finish",
                "interrogating the smugglers.",
                "I'm not sure if he did since the smugglers were taken away."
            ],
        )?;
        ctx.var("aru_monas").set(Val::from(10))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_monas").get()? == 10 {
        ctx.lines_as(
            "Himus",
            args![
                "The magistrate is about",
                "to retire, so I hope he can",
                "finally quit work and just",
                "forget about this whole",
                "business. He does nothing",
                "but stress out about it!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Himus", args!["Nice day, isn't it?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn soldier_aru(ctx: &Ctx) -> Script {
    soldier_aru_body(ctx, Vec::new()).map(|_| ())
}

fn drunkard_aru_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()?.number()? < 14 {
        ctx.lines_as("Drunkard", args!["Man, I feel great! Hah hah!", "Hey, gimme one more drink!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("aru_monas").get()? == 14 {
        if ctx.var("rachel_camel").get()? == 25 {
            ctx.lines_as(
                "Drunkard",
                args![
                    "So--*Hic* What I was...",
                    "Oog, dizzy... I say sayin',",
                    "that guy that ^32CD32snuck out to",
                    "sea on a boat^000000? You know",
                    "*Hic* why he came back",
                    "so early? You won't believe it!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Drunkard",
                args![
                    "Hah hah! He--*Hic*",
                    "He thought he saw a ^DBDB70ghost^000000!",
                    "Bwah hah hah! Oooog... Hey..."
                ],
            )?;
            ctx.var("aru_monas").set(Val::from(15))?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "That's right!",
                    "Maybe I should check",
                    "on ^32CD32Kurdi's father since",
                    "he's a fisherman^000000"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Drunkard", args!["Zzzz...", "Umm... Zzz..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Drunkard",
            args![
                "Why?! Why can't I take",
                "my boat out to sea? I hafta",
                "catch fish to make a living!",
                "What, they expect me to starve",
                "to death or something?! Huh?!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn drunkard_aru(ctx: &Ctx) -> Script {
    drunkard_aru_body(ctx, Vec::new()).map(|_| ())
}

fn drunkard_aru1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Drunkard", args!["Hohohoho~", "Oh~ Hohohoho~"])?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["She's kind of strange..."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn drunkard_aru1(ctx: &Ctx) -> Script {
    drunkard_aru1_body(ctx, Vec::new()).map(|_| ())
}

fn boat_aru_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()?.number()? < 16 {
        ctx.lines(args![
            "^3355FFThis boat seems to ",
            "be in decent condition.",
            "Who could its owner be?^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_monas").get()? == 16 {
        ctx.lines(args![
            "^3355FFThis must be the boat that",
            "Karyn was talking about.",
            "It seems to be in pretty",
            "good shape. Now, the",
            "monastery should be",
            "southwest from here...^000000"
        ])?;
        ctx.next()?;
        ctx.call(Function::EnableNpc, vec![Val::from("Secret Agent#Aru")])?;
        ctx.next()?;
        ctx.lines_as(
            "Larjes",
            args![
                "Long time no see,",
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Larjes!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Larjes",
            args![
                "I had a tough time following",
                "you, you know? I actually",
                "almost lost you once. You",
                "must be wondering why",
                "I'm here. If you'll listen,",
                "I'll explain everything."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Larjes",
            args![
                "According to our investigation, Arunafeltz is definitely involved",
                "in this case. We've been watching you since you're somehow related",
                "to that high ranked official from Arunafeltz. I apologize for that."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Larjes",
            args![
                "This is an international",
                "incident, so to prevent any",
                "intel leaks to Arunafeltz,",
                "we've had to be much stricter",
                "with securing and protecting",
                "all information for this case."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Larjes",
            args![
                "Anyway, I've decided to",
                "reveal myself to you since",
                "I'm the only one that can help",
                "you right now. You're thinking",
                "of using that boat, aren't you?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Larjes",
            args![
                "If you don't have the skills,",
                "knowledge, and preparation",
                "to sail those waters ahead,",
                "you'll probably wreck the",
                "boat. You'll make it if you're lucky but... That's a lot of luck."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Larjes",
            args![
                "I'll help you sail this",
                "boat to get where you're",
                "going. But if you fail, there's",
                "a chance you might have to",
                "sail this boat on your own if I'm not here. So, you ready to go?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes, I'm ready.:Give me more time.")])? {
            1 => {
                ctx.lines_as(
                    "Larjes",
                    args![
                        "Alright, let's go.",
                        "Hold on to something:",
                        "this will probably be",
                        "a pretty rocky ride..."
                    ],
                )?;
                ctx.var("aru_monas").set(Val::from(17))?;
                ctx.close_window()?;
                ctx.call(Function::DisableNpc, vec![Val::from("Secret Agent#Aru")])?;
                ctx.call(Function::Warp, vec![Val::from("nameless_i"), Val::from(257), Val::from(217)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Larjes", args!["Alright. Just let me know", "whenever you're ready."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("aru_monas").get()?.number()? < 20 {
        ctx.lines(args![
            "^3355FFThis is the boat that",
            "can take you to the",
            "monastery.^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Go to Monastery:Cancel")])? {
            1 => {
                ctx.lines(args!["^3355FFYou set sail for", "the monastery...^000000"])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("nameless_i"), Val::from(257), Val::from(217)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args!["^3355FFYou decide to", "stay ashore.^000000"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines(args![
            "^3355FFThis is the boat that",
            "can take you to the",
            "monastery.^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Go to Monastery:Cancel")])? {
            1 => {
                ctx.lines(args!["^3355FFYou set sail for", "the monastery...^000000"])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("nameless_n"), Val::from(257), Val::from(217)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args!["^3355FFYou decide to", "stay ashore.^000000"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn boat_aru(ctx: &Ctx) -> Script {
    boat_aru_body(ctx, Vec::new()).map(|_| ())
}

fn secret_agent_aru_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()? == 16 {
        ctx.lines_as(
            "Larjes",
            args![
                "Are you ready to go",
                "aboard? I'll help you",
                "sail to that monastery,",
                "but if you fail while you're",
                "there, I might not be here",
                "when you come back."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes, I'm ready.:Give me more time.")])? {
            1 => {
                ctx.lines_as(
                    "Larjes",
                    args![
                        "Alright, let's go.",
                        "Hold on to something:",
                        "this will probably be",
                        "a pretty rocky ride..."
                    ],
                )?;
                ctx.var("aru_monas").set(Val::from(17))?;
                ctx.close_window()?;
                ctx.call(Function::DisableNpc, vec![Val::from("Secret Agent#Aru")])?;
                ctx.call(Function::Warp, vec![Val::from("nameless_i"), Val::from(257), Val::from(217)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Larjes", args!["Alright. Just let me know", "whenever you're ready."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    return Err(Stop::End);
}

pub fn secret_agent_aru(ctx: &Ctx) -> Script {
    secret_agent_aru_body(ctx, Vec::new()).map(|_| ())
}

fn secret_agent_aru_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Secret Agent#Aru")])?;
    return Err(Stop::End);
}

pub fn secret_agent_aru_oninit(ctx: &Ctx) -> Script {
    secret_agent_aru_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn outside_island_aru_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn outside_island_aru(ctx: &Ctx) -> Script {
    outside_island_aru_body(ctx, Vec::new()).map(|_| ())
}

fn outside_island_aru_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()? == 17 {
        ctx.lines(args![
            "^3355FFThe village is totally",
            "silent, as if all life had",
            "abandoned it. You look",
            "around and see that something",
            "was here a few hours ago, but",
            "it's somewhere else now.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFWhatever it was,",
            "it definitely wasn't",
            "human. What kind of",
            "creature could it be?^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn outside_island_aru_ontouch(ctx: &Ctx) -> Script {
    outside_island_aru_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn inside_island_aru_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn inside_island_aru(ctx: &Ctx) -> Script {
    inside_island_aru_body(ctx, Vec::new()).map(|_| ())
}

fn inside_island_aru_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()? == 17 {
        ctx.lines(args![
            "^3355FFThere are traces of",
            "humans around here,",
            "along with some kind",
            "of creature that you",
            "can't clearly identify.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn inside_island_aru_ontouch(ctx: &Ctx) -> Script {
    inside_island_aru_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn grass_behind_aru_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn grass_behind_aru(ctx: &Ctx) -> Script {
    grass_behind_aru_body(ctx, Vec::new()).map(|_| ())
}

fn grass_behind_aru_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()? == 17 {
        ctx.lines(args![
            "^3355FFA strange scent strikes",
            "you as you enter this field of",
            "grass. A few ^32CD32animal corpses^3355FF",
            "are strewn around the ground.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn grass_behind_aru_ontouch(ctx: &Ctx) -> Script {
    grass_behind_aru_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn dead_crow_aru_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()?.number()? < 17 {
        ctx.mes("^3355FFThere is a dead crow on the ground.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_monas").get()? == 17 {
        ctx.lines(args![
            "^3355FFThe sight of this ^32CD32dead",
            "crow^3355FF makes you feel",
            "uneasy for some reason.^000000"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Ignore:Investigate")])?) == 1 {
            ctx.lines(args!["^3355FFYou decide not to touch", "the animal's carcass.^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FFYou notice some grass sap",
            "on the crow's beak, so it",
            "must have been nibbling on",
            "some grass. Some feathers",
            "are missing, revealing",
            "scaly, snake-like skin.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Wait...", "Did this grass...?"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThis grass must be the",
            "main ingredient of the",
            "poison used to kill the",
            "Geoborg family princes.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Looks like I just hit", "the jackpot. Huh?", "Someone behi--"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFA sharp, throbbing pain",
            "assails the back of your",
            "head as you fall into",
            "unconsciousness...^000000"
        ])?;
        ctx.var("aru_monas").set(Val::from(18))?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("nameless_in"), Val::from(15), Val::from(60)])?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn dead_crow_aru(ctx: &Ctx) -> Script {
    dead_crow_aru_body(ctx, Vec::new()).map(|_| ())
}

fn pass_out_aru_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn pass_out_aru(ctx: &Ctx) -> Script {
    pass_out_aru_body(ctx, Vec::new()).map(|_| ())
}

fn pass_out_aru_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()? == 18 {
        ctx.call(Function::DisableNpc, vec![Val::from("Out_from_Monastery")])?;
        ctx.call(
            Function::StartStatus,
            vec![ctx.constant("SC_BLIND")?, Val::from(600000), Val::from(0), Val::from(10000)],
        )?;
        ctx.lines(args![
            "^3355FFThe pressure on your",
            "stomach and the blood",
            "rushing to your head tells",
            "you someone is carrying you",
            "over his shoulder. He stops,",
            "and you hear a faint voice.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThe faint voice steadily",
            "grows stronger and more",
            "distinct--someone's calling",
            "your name. You feel cold water",
            "trickle through your lips, and",
            "then you regain your senses.^000000"
        ])?;
        ctx.next()?;
        ctx.call(Function::EndStatus, vec![ctx.constant("SC_BLIND")?])?;
        ctx.call(Function::EnableNpc, vec![Val::from("Larjes#Monastery")])?;
        ctx.next()?;
        ctx.lines(args!["That was too close...", "For a second there,", "I thought I lost you."])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Larjes...? What...", "What happened?", "Ugh, my head..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Larjes",
            args![
                "Try not to move for a while.",
                "I had a bad feeling waiting",
                "for you on the boat. Lucky",
                "thing. When I found you,",
                "these strange creatures",
                "were savagely attacking you!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["You saved me?", "Thank you. Do you", "happen to know what", "those creatures were?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Larjes",
            args![
                "No clue. They looked",
                "like humans, but... They",
                "definitely weren't. Once",
                "I killed them, they all",
                "turned into sand."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::EnableNpc, vec![Val::from("Creature#Monas")])?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HUK")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HUK")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Larjes#Monastery")])?,
            ],
        )?;
        ctx.lines_as("Larjes", args!["?!?!?!!!!!"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["!!!!?!?!!!!!!", "Isn't... Isn't that...?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Larjes",
            args![
                "It seems it's just like the",
                "creatures that kidnapped",
                "you, but... I wonder why",
                "he's not attacking us."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I guess you'll have to go", "right up to him and ask."],
        )?;
        ctx.var("aru_monas").set(Val::from(19))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn pass_out_aru_ontouch(ctx: &Ctx) -> Script {
    pass_out_aru_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn larjes_monastery_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Larjes", args!["Be careful. That guy", "looks pretty dangerous."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn larjes_monastery(ctx: &Ctx) -> Script {
    larjes_monastery_body(ctx, Vec::new()).map(|_| ())
}

fn larjes_monastery_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Larjes#Monastery")])?;
    return Err(Stop::End);
}

pub fn larjes_monastery_oninit(ctx: &Ctx) -> Script {
    larjes_monastery_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn creature_monas_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn creature_monas(ctx: &Ctx) -> Script {
    creature_monas_body(ctx, Vec::new()).map(|_| ())
}

fn creature_monas_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Creature#Monas")])?;
    return Err(Stop::End);
}

pub fn creature_monas_oninit(ctx: &Ctx) -> Script {
    creature_monas_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn creature_monas_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("aru_monas").get()? == 19
        && ctx
            .call(
                Function::MobCount,
                vec![Val::from("nameless_in"), Val::from("Creature#Monas::OnMyMobDead")],
            )?
            .number()?
            < 1)
        && !(ctx.var("@aru_monas_kill").get()?.is_true()))
    {
        ctx.lines_as("???????", args!["Grrr~!!!"])?;
        ctx.close_window()?;
        ctx.call(
            Function::Monster,
            vec![
                Val::from("nameless_in"),
                Val::from(13),
                Val::from(53),
                Val::from("Zombie"),
                Val::from(1864),
                Val::from(1),
                Val::from("Creature#Monas::OnMyMobDead"),
            ],
        )?;
        ctx.call(Function::DisableNpc, vec![Val::from("Creature#Monas")])?;
    }
    return Err(Stop::End);
}

pub fn creature_monas_ontouch(ctx: &Ctx) -> Script {
    creature_monas_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn creature_monas_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("@aru_monas_kill").set(Val::from(1))?;
    ctx.call(Function::EnableNpc, vec![Val::from("Out_from_Monastery")])?;
    return Err(Stop::End);
}

pub fn creature_monas_onmymobdead(ctx: &Ctx) -> Script {
    creature_monas_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn out_from_monastery_run(ctx: &Ctx, mut step: OutFromMonasteryStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            OutFromMonasteryStep::Start => {
                return Err(Stop::End);
            }
            OutFromMonasteryStep::OnTouch => {
                if ctx.var("aru_monas").get()? == 19 {
                    ctx.var("aru_monas").set(Val::from(20))?;
                    ctx.call(Function::Warp, vec![Val::from("nameless_n"), Val::from(168), Val::from(252)])?;
                    return Err(Stop::End);
                }
                step = OutFromMonasteryStep::OnInit;
                continue 'machine;
            }
            OutFromMonasteryStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Out_from_Monastery")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn out_from_monastery(ctx: &Ctx) -> Script {
    out_from_monastery_run(ctx, OutFromMonasteryStep::Start, Vec::new()).map(|_| ())
}

pub fn out_from_monastery_ontouch(ctx: &Ctx) -> Script {
    out_from_monastery_run(ctx, OutFromMonasteryStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn out_from_monastery_oninit(ctx: &Ctx) -> Script {
    out_from_monastery_run(ctx, OutFromMonasteryStep::OnInit, Vec::new()).map(|_| ())
}

fn outtoin_01_mo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn outtoin_01_mo(ctx: &Ctx) -> Script {
    outtoin_01_mo_body(ctx, Vec::new()).map(|_| ())
}

fn outtoin_01_mo_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("aru_monas").get()? == 18 || ctx.var("aru_monas").get()? == 19) {
        ctx.call(Function::Warp, vec![Val::from("nameless_in"), Val::from(12), Val::from(41)])?;
        return Err(Stop::End);
    }
    ctx.lines(args!["^3355FFThe door won't budge:", "you can't go through.^000000"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn outtoin_01_mo_ontouch(ctx: &Ctx) -> Script {
    outtoin_01_mo_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn night_aru2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn night_aru2(ctx: &Ctx) -> Script {
    night_aru2_body(ctx, Vec::new()).map(|_| ())
}

fn night_aru2_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()? == 20 {
        ctx.call(Function::EnableNpc, vec![Val::from("Larjes#Monastery2")])?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["What the hell...!?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Larjes", args!["So... This is the", "island's true nature."])?;
        ctx.next()?;
        ctx.lines_as(
            "Larjes",
            args![
                "Someone like me won't",
                "survive long in a place",
                "like this. I'm going to wait",
                "for you in the boat. Learn",
                ((Val::from("what you need to learn, and then come back safe, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from("."))
            ],
        )?;
        ctx.var("aru_monas").set(Val::from(21))?;
        ctx.close_window()?;
        ctx.call(Function::DisableNpc, vec![Val::from("Larjes#Monastery2")])?;
    }
    return Err(Stop::End);
}

pub fn night_aru2_ontouch(ctx: &Ctx) -> Script {
    night_aru2_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn larjes_monastery2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn larjes_monastery2(ctx: &Ctx) -> Script {
    larjes_monastery2_body(ctx, Vec::new()).map(|_| ())
}

fn larjes_monastery2_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Larjes#Monastery2")])?;
    return Err(Stop::End);
}

pub fn larjes_monastery2_oninit(ctx: &Ctx) -> Script {
    larjes_monastery2_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn larjes_aru_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()?.number()? < 18 {
        ctx.lines_as(
            "Larjes",
            args![
                "I don't like this place.",
                "You'd be better be careful",
                "around here with those weird",
                "creatures running around."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_monas").get()?.number()? < 24 {
        ctx.lines_as(
            "Larjes",
            args![
                "Good, you're back",
                "I know that there's still",
                "things on this island that",
                "you want to investigate,",
                "but did you want to leave",
                "this place for a while?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 1 {
            ctx.lines_as("Larjes", args!["Alight, let's go."])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("ve_fild07"), Val::from(130), Val::from(130)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Larjes",
            args!["Alright, but becareful.", "Those creatures almost", "got you once, you know."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Larjes", args!["Did you still want to", "investigate the island?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("No:Yes")])?) == 1 {
            ctx.lines_as("Larjes", args!["Alright, let's go."])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("ve_fild07"), Val::from(130), Val::from(130)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Larjes",
            args!["Alright, but be careful.", "Those creatures almost", "got you once, you know."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn larjes_aru(ctx: &Ctx) -> Script {
    larjes_aru_body(ctx, Vec::new()).map(|_| ())
}

fn larjes_boat1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_monas").get()? == 24 {
        if ctx.call(Function::CountItem, vec![Val::from(7726)])?.number()? < 1 {
            ctx.lines_as(
                "Larjes",
                args![
                    "Good, you're back.",
                    "I know that there's still",
                    "things on this island that",
                    "you want to investigate,",
                    "but did you want to leave",
                    "this place for a while?"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
                1 => {
                    ctx.lines_as("Larjes", args!["Alright, let's go."])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("ve_fild07"), Val::from(130), Val::from(130)])?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Larjes",
                        args!["Alright, but becareful.", "Those creatures almost", "got you once, you know."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.lines_as(
                "Larjes",
                args![
                    "I don't believe it...",
                    "That guy was Tristam III?!",
                    "This explains a lot, I suppose.",
                    "I'll report this along with the",
                    "voucher. So Arunafeltz was",
                    "behind that poison grass too..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Larjes",
                args![
                    "I'm shocked over this",
                    "whole debacle. I'm sure",
                    "the Rune-Midgarts royal family",
                    "will be in an uproar over this.",
                    "To think that we're involved",
                    "in something this huge..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Larjes", args!["Anyway, we better get", "going... This is huge!"])?;
            ctx.call(Function::DelItem, vec![Val::from(7726), Val::from(1)])?;
            ctx.var("aru_monas").set(Val::from(25))?;
            ctx.call(Function::GetExperience, vec![Val::from(1000000), Val::from(0)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as("Larjes", args!["Did you still want to", "investigate the island?"])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("No:Yes")])? {
        1 => {
            ctx.lines_as("Larjes", args!["Alright, let's go."])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("ve_fild07"), Val::from(130), Val::from(130)])?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Larjes",
                args!["Alright, but becareful.", "Those creatures almost", "got you once, you know."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn larjes_boat1(ctx: &Ctx) -> Script {
    larjes_boat1_body(ctx, Vec::new()).map(|_| ())
}

fn books_mona1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7766), Val::from(1)])? == 0 {
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
    if ctx.var("aru_monas").get()?.number()? < 21 {
        ctx.lines(args!["^3355FFIt's just a bunch", "of old, moldy books.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_monas").get()? == 21 {
        ctx.lines(args![
            "^3355FFThere's a book stained",
            "with blood amongst all",
            "these old, moldy books.^000000"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Examine Book:Ignore")])?) == 1 {
            ctx.var("aru_monas").set(Val::from(22))?;
            ctx.call(Function::GetItem, vec![Val::from(7755), Val::from(1)])?;
            ctx.call(Function::ReadBook, vec![Val::from(7755), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args!["^3355FFThat book was probably", "worthless to you anyway.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args!["^3355FFThere are old books", "scattered all over here.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn books_mona1(ctx: &Ctx) -> Script {
    books_mona1_body(ctx, Vec::new()).map(|_| ())
}
