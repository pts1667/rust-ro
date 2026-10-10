use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn marybell_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_a = Val::from(0);
    let mut l_b = Val::from(0);
    let mut l_c = Val::from(0);
    let mut l_input_s = Val::from("");
    if ctx.var("zdan_edq").get()? == 4 {
        ctx.lines_as("Marybell", args!["Hey. What do you want?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Password")])? {
            1 => {
                ctx.lines_as(
                    "Marybell",
                    args!["Can't hear you.", "Come a little closer,", "and talk right in my ear."],
                )?;
                ctx.next()?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                if l_input_s.clone() == "The dawn is yet to come." {
                    ctx.lines_as(
                        "Marybell",
                        args![
                            "Valdes sent you?!",
                            "I'm surprised he finally",
                            "decided to talk to someone!",
                            "Alright, if you're his friend,",
                            "then I'll do what I can to",
                            "help. What do you need?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Ask About Z Gang:Valdes says, ''Hi.''")])? {
                        1 => {
                            ctx.lines_as(
                                "Marybell",
                                args![
                                    "The Z Gang again?",
                                    "I guess he's still mad",
                                    "about capturing them.",
                                    "It's not a big deal for me",
                                    "to tell you what I know, but",
                                    "can I really trust you?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Marybell",
                                args![
                                    "Let me test you out.",
                                    "Go to Payon, find an old",
                                    "guy named Moonho Ahn. He's",
                                    "a legendary gambler better",
                                    "known as the White Meteor.",
                                    "If he trusts you, I trust you."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Marybell",
                                args![
                                    "Heh, but he'll only trust",
                                    "you if you can beat him at",
                                    "gambling. Whether you return",
                                    "or not, I'm gonna continue",
                                    "investigating the Z Gang for",
                                    "the Rogue Guild and Valdes."
                                ],
                            )?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(3122), Val::from(3123)])?;
                            ctx.var("zdan_edq").set(Val::from(5))?;
                        }
                        2 => {
                            ctx.lines_as(
                                "Marybell",
                                args![
                                    "Weird. It's not like him",
                                    "to send his regards like that.",
                                    "He's not sick or anything,",
                                    "is he? If he's still drinking",
                                    "the next time I see him,",
                                    "I'm gonna clobber that guy!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Marybell",
                                args![
                                    "Geez. Doesn't he know",
                                    "I volunteered for this",
                                    "job for his sake? Oh well,",
                                    "it's really nice to know",
                                    "that he's thinking of me.",
                                    "What? Rogues can have friends!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.lines_as("Marybell", args!["Can't tell what", "you're saying exactly."])?;
                    ctx.next()?;
                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                        ctx.lines_as(
                            "Marybell",
                            args![
                                "If this is some kind",
                                "of weird come-on, you'd",
                                "better have something",
                                "better to say than that!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Marybell",
                            args![
                                "What, are you coming",
                                "on to me? I'm down with",
                                "any gender, but, heh,",
                                "you sure about this?",
                                "Hahahahahahahahaha!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
            _ => {}
        }
    }
    if (ctx.var("zdan_edq").get()?.number()? > 4 && ctx.var("zdan_edq").get()?.number()? < 7) {
        ctx.lines_as(
            "Marybell",
            args![
                "Let me test you out.",
                "Go to Payon, find an old",
                "guy named Moonho Ahn. He's",
                "a legendary gambler better",
                "known as the White Meteor.",
                "If he trusts you, I trust you."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("zdan_edq").get()? == 7 {
        ctx.lines_as(
            "Marybell",
            args![
                "What, you're back?",
                "Aww, nuts! That signature",
                "on your arm... You really",
                "beat him? How the hell--?"
            ],
        )?;
        ctx.next()?;
        ctx.var("zdan_edq").set(Val::from(8))?;
        ctx.lines_as(
            "Marybell",
            args![
                "Wait, lemme check",
                "and make sure. Yeap.",
                "That's the one. That's",
                "the signature he used when",
                "he was the White Meteor."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marybell",
            args![
                "How did you beat him?",
                "Last time, he cleaned me",
                "out of all my money! You",
                "must be some kinda genius",
                "to beat that guy. Heh. Guess",
                "I underestimated you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marybell",
            args![
                "Hold on, before anything",
                "else, I gotta tell everyone",
                "that someone actually beat",
                "the White Meteor. They're",
                "totally not gonna believe it!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("zdan_edq").get()? == 8 {
        ctx.lines_as(
            "Marybell",
            args![
                "Well, I promised to tell",
                "you what I know about the",
                "Z Gang, so I gotta make good.",
                "What did you want to ask me?"
            ],
        )?;
        ctx.next()?;
        'l3: loop {
            if !(true) {
                break 'l3;
            }
            'b3: {
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Z Gang's Goal:Z Gang's Recent Movements:Z Gang's Stronghold:Thanks for the tip!",
                    )],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Marybell",
                            args![
                                "Actually, I don't know",
                                "what they plan to do, but",
                                "obviously it isn't something",
                                "very good. Personally, I think",
                                "they just want to harass",
                                "people as much as they can."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Marybell",
                            args![
                                "You could conquer the",
                                "world, I guess, with that",
                                "Book of Forbidden Mystery,",
                                "but I don't think they're",
                                "that... Calculating."
                            ],
                        )?;
                        ctx.next()?;
                        l_a = (l_a.clone() + Val::from(1));
                    }
                    2 => {
                        ctx.lines_as(
                            "Marybell",
                            args![
                                "Well, the Z Gang is",
                                "responsible for a series",
                                "of thefts lately. They've",
                                "only been stealing jewels",
                                "like diamonds, rubies,",
                                "emeralds. Not sure why..."
                            ],
                        )?;
                        ctx.next()?;
                        l_b = (l_b.clone() + Val::from(1));
                    }
                    3 => {
                        ctx.lines_as(
                            "Marybell",
                            args![
                                "Well, we don't know where",
                                "they're hiding out. If that",
                                "was the case, the Z Gang",
                                "would already be caught!",
                                "They even got away from",
                                "us Rogues. Crazy, huh?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Marybell",
                            args![
                                "The Rogue Guild has got",
                                "the best intel network so...",
                                "The Z Gang probably has",
                                "informers everywhere in",
                                "all the towns. It's the only",
                                "way they can escape us."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Marybell",
                            args![
                                "Hey, we can use that...",
                                "If we can catch one of",
                                "their informers, one of",
                                "them'll spill the beans",
                                "on where the Z Gang is",
                                "hiding. Worth a shot, yeah?"
                            ],
                        )?;
                        ctx.next()?;
                        l_c = (l_c.clone() + Val::from(1));
                    }
                    4 => {
                        if ((l_a.clone().number()? > 0 && l_b.clone().number()? > 0) && l_c.clone().number()? > 0) {
                            ctx.lines_as(
                                "Marybell",
                                args![
                                    "Oh, and one more thing...",
                                    "I hear that they're trying",
                                    "to secretly recruit more",
                                    "gang members in Morocc.",
                                    "The nerve of those guys...!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Marybell",
                                args![
                                    "It might be a good idea",
                                    "for you to go to Morocc",
                                    "and check it out. Good",
                                    "luck finding those Z Gang",
                                    "guys. And take care!"
                                ],
                            )?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(3125), Val::from(3126)])?;
                            ctx.var("zdan_edq").set(Val::from(9))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Marybell",
                                args![
                                    "Hey, I'm willing to",
                                    "let you in on everything",
                                    "I know. After all the trouble",
                                    "you went through, I'm sure",
                                    "you've got a ton of questions!"
                                ],
                            )?;
                            ctx.next()?;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    if (ctx.var("zdan_edq").get()?.number()? > 8 && ctx.var("zdan_edq").get()?.number()? < 11) {
        ctx.lines_as(
            "Marybell",
            args![
                "Oh, and one more thing...",
                "I hear that they're trying",
                "to secretly recruit more",
                "gang members in Morocc.",
                "The nerve of those guys...! "
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marybell",
            args![
                "I haven't heard anything",
                "else, but would you let me",
                "know if you get any new leads?",
                "The Rogue Guild wants these",
                "Z Gang guys really bad!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("zdan_edq").get()? == 11 {
        ctx.lines_as(
            "Marybell",
            args![
                "Hey, I've been looking for",
                "you! One of the Rogues",
                "dispatched to Comodo found",
                "this envelope on a Z Gang",
                "informer. Could be a clue!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marybell",
            args![
                "What's weird is that the",
                "informer was fighting so",
                "hard to hold on to blank",
                "piece of paper... But we ",
                "know that's not really",
                "the case here, is it?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marybell",
            args![
                "They must've been using",
                "some kinda secret invisible",
                "ink. Anyway, I gave the paper",
                "to a professional decoder to",
                "crack, and he oughta be",
                "done by now. Anyway..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marybell",
            args![
                "I thought maybe you'd",
                "want to go talk to him,",
                "see if he's finished with it.",
                "His name's Gooho Ahn over",
                "in Payon, one of the best",
                "decoders in this kingdom."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Gooho Ahn?",
                "Why does it feel like",
                "I should know that",
                "name? It's so familiar..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marybell",
            args![
                "I guess it's 'cause",
                "Gooho is the younger brother",
                "of Moonho, that legendary",
                "gambler you happened to beat."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marybell",
            args![
                "Eh, that's not what's",
                "important right now.",
                "Would you talk to Gooho",
                "Ahn and see what he found?"
            ],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3128), Val::from(3129)])?;
        ctx.var("zdan_edq").set(Val::from(12))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("zdan_edq").get()?.number()? > 11 && ctx.var("zdan_edq").get()?.number()? < 14) {
        ctx.lines_as(
            "Marybell",
            args![
                "Hey, why don't you talk",
                "to Gooho Ahn in Payon,",
                "and ask if he finished",
                "decoding that secret letter",
                "from the Z Gang already?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("zdan_edq").get()? == 14 {
        ctx.lines_as(
            "Marybell",
            args![
                "Oh, Gooho decoded everything?",
                "Let's see... Something about",
                "the Book of Forbidden Mystery... Ah. There we go! The location",
                "of the Z Gang's hideout!",
                "Heh heh! We got 'em!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marybell",
            args![
                "One of the Rogues followed",
                "the informer carrying this",
                "letter, but he just vanished",
                "during the chase... Like, by",
                "magic, I guess. So we don't",
                "the hideout's exact place."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marybell",
            args![
                "Even so, we're definitely",
                "sure that it's around the",
                "secret path to the South",
                "Morocc underground. Um,",
                "but we can't let you come",
                "with the Rogue Guild."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marybell",
            args![
                "Sorry, I know, it's official",
                "business and all that hooplah.",
                "Tell you what. We'll each go",
                "hunt them down separately, and",
                "whoever finds 'em first gets",
                "dibs on beating them up!"
            ],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3131), Val::from(3132)])?;
        ctx.var("zdan_edq").set(Val::from(15))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("zdan_edq").get()?.number()? > 14 && ctx.var("zdan_edq").get()?.number()? < 18) {
        ctx.lines_as(
            "Marybell",
            args![
                "We're definitely sure",
                "that the Z Gang's hideout",
                "is near the secret path",
                "to the South Morocc",
                "underground. You",
                "know where that is?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marybell",
            args![
                "Tell you what. We'll each go",
                "hunt them down separately, and",
                "whoever finds 'em first gets",
                "dibs on beating them up!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("zdan_edq").get()?.number()? > 18 {
        ctx.lines_as(
            "Marybell",
            args![
                "Nice work! You captured",
                "the Z Gang! The kingdom",
                "musta given you a really",
                "big reward, huh?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marybell",
            args![
                "Valdes really wanted to",
                "capture them himself, but",
                "I'm sure he'll be real happy",
                "to hear the news. Maybe he",
                "and I can celebrate with",
                "a little drink later."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marybell",
            args![
                "Hey, it's been a real",
                "thrill working with you,",
                "real thrill. Maybe we can",
                "team up again for a good",
                "cause some other time, yeah?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
        ctx.lines_as(
            "Marybell",
            args![
                "That Z Gang has been",
                "a pain the Rogue Guild's",
                "ass. No one runs away",
                "from us! Nobody! But...",
                "They can do it somehow."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Marybell",
            args![
                "There's not much here",
                "that might interest you,",
                "but knock yourself out",
                "if you wanna look around."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn marybell(ctx: &Ctx) -> Script {
    marybell_body(ctx, Vec::new()).map(|_| ())
}

fn moonho_ahn_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_amuro = Val::from(0);
    let mut l_input_s = Val::from("");
    let mut l_lose = Val::from(0);
    let mut l_number = Val::from(0);
    let mut l_number_false = Val::from(0);
    let mut l_number_false_2 = Val::from(0);
    let mut l_number_false_3 = Val::from(0);
    let mut l_number_right = Val::from(0);
    let mut l_number_right_2 = Val::from(0);
    let mut l_number_right_3 = Val::from(0);
    let mut l_win = Val::from(0);
    if ctx.var("zdan_edq").get()? == 5 {
        ctx.lines_as(
            "Moonho Ahn",
            args![
                "Hahaha, now what brings",
                "a youngster like you before",
                "me? You're not here for what",
                "I think you are... Are you?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Are you the White Meteor?:I'm here to challenge you.")])? {
            1 => {
                ctx.lines_as(
                    "Moonho Ahn",
                    args![
                        "It's been so long since",
                        "I've heard that name...",
                        "I'm retired now, and it",
                        "seems no one wants to",
                        "challenge me anymore."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Moonho Ahn",
                    args![
                        "Those were some good",
                        "times. Back then, only",
                        "Dalho Kwak was able to",
                        "give me a real challenge.",
                        "I wonder what he's doing now?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Moonho Ahn",
                    args![
                        "Oh, I was right...!",
                        "You're here to challenge",
                        "me! It's been a long time...",
                        "Alright. I accept. However,",
                        "I set all of the terms."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Moonho Ahn",
                    args![
                        "I'm too old to play the",
                        "complicated games I loved",
                        "in the past. Let's just",
                        "play a simple game that",
                        "I call Coin Shake."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Moonho Ahn",
                    args![
                        "This is basically",
                        "a guessing game played",
                        "in rounds, best 2 out of 3.",
                        "One of us will be the coin",
                        "shaker, and the other will",
                        "be the guesser."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Moonho Ahn",
                    args![
                        "The coin shaker begins",
                        "the round by grabbing",
                        "a random amount of coins,",
                        "shaking them in his closed",
                        "hands, and then stopping,",
                        "keeping the coins concealed."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Moonho Ahn",
                    args![
                        "The other person, the",
                        "guesser, will then declare",
                        "a guess on whether the total",
                        "value of the coins is ^0000FFOdd^000000 or",
                        "^0000FFEven^000000. Afterwards, the coin",
                        "shaker reveals the coins..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Moonho Ahn",
                    args![
                        "If the guesser is right,",
                        "then he wins the round.",
                        "However, if he guessed",
                        "wrong, then it's a victory",
                        "for the coin shaker. "
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Moohno Ahn",
                    args![
                        "We each take turns,",
                        "switching roles.",
                        "If it's your turn to",
                        "guess, remember to",
                        "call out ^0000FFOdd^000000 or ^0000FFEven^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Moonho Ahn",
                    args![
                        "Lastly, I'm the house,",
                        "so I'll charge a game",
                        "participation fee. It's",
                        "not much, just 500 zeny.",
                        "We're only playing for fun,",
                        "not to break the bank."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Moonho Ahn",
                    args![
                        "Why don't you give me",
                        "some time to prep the",
                        "game? When you return,",
                        "we'll be ready to play~"
                    ],
                )?;
                ctx.call(Function::ChangeQuest, vec![Val::from(3123), Val::from(3124)])?;
                ctx.var("zdan_edq").set(Val::from(6))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("zdan_edq").get()? == 6 {
        ctx.lines_as("Moonho Ahn", args!["Are you ready to", "play Coin Shake?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes, let's play!:How does this game work again?")])? {
            1 => {
                if ctx.var("Zeny").get()?.number()? > 500 {
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(500))?))?;
                    ctx.lines_as("Moonho Ahn", args!["Good, let's get started!", "I'll let you go first~"])?;
                    ctx.next()?;
                    ctx.lines(args!["*Shake Shake*", "Guess! Is it", "Odd or Even?"])?;
                    ctx.next()?;
                    'l3: loop {
                        if !(true) {
                            break 'l3;
                        }
                        'b3: {
                            l_number = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
                            let (input, status) = runtime::input_text(ctx, None, None)?;
                            l_input_s = input;
                            if ((l_input_s.clone() == "Odd" && l_number.clone() == 1)
                                || (l_input_s.clone() == "Even" && l_number.clone() == 2))
                            {
                                l_number_right = (l_number_right.clone() + Val::from(1));
                                ctx.lines_as(
                                    "Moonho Ahn",
                                    args![
                                        ((Val::from("^0000ff") + l_input_s.clone()) + Val::from("^000000?")),
                                        "Okay, you won."
                                    ],
                                )?;
                                ctx.next()?;
                                if (l_number_right.clone().number()? < 2 && l_number_false.clone().number()? < 2) {
                                    ctx.lines(args!["^3355FF*Shake Shake*", "Guess! Is it", "Odd or Even?^000000"])?;
                                    ctx.next()?;
                                }
                            } else if ((l_input_s.clone() == "Odd" && l_number.clone() == 2)
                                || (l_input_s.clone() == "Even" && l_number.clone() == 1))
                            {
                                l_number_false = (l_number_false.clone() + Val::from(1));
                                ctx.lines_as("Moonho Ahn", args!["Well, it's ^0000FFEven^000000.", "I won."])?;
                                ctx.next()?;
                                if (ctx.var("number_right").get()?.number()? < 2 && ctx.var("number_false").get()?.number()? < 2) {
                                    ctx.lines(args!["^3355FF*Shake Shake*", "Guess! Is it", "Odd or Even?^000000"])?;
                                    ctx.next()?;
                                }
                            }
                            if l_number_right.clone() == 2 {
                                ctx.lines_as(
                                    "Moonho Ahn",
                                    args![
                                        "Ah, you win this round.",
                                        "However, the game has",
                                        "just started. The next",
                                        "round will be mine~"
                                    ],
                                )?;
                                l_win = (l_win.clone() + Val::from(1));
                                ctx.next()?;
                                break 'l3;
                            } else if l_number_false.clone() == 2 {
                                ctx.lines_as(
                                    "Moonho Ahn",
                                    args![
                                        "It looks like I win",
                                        "this round. I guess",
                                        "my gambling skills",
                                        "haven't left me yet~"
                                    ],
                                )?;
                                l_lose = (l_lose.clone() + Val::from(1));
                                ctx.next()?;
                                break 'l3;
                            }
                            if (l_input_s.clone() != "Even" && l_input_s.clone() != "Odd") {
                                ctx.lines_as(
                                    "Moonho Ahn",
                                    args![
                                        "You can only declare",
                                        "your guess as ^0000FFOdd^000000 or ^0000FFEven^000000.",
                                        "Please try entering it again."
                                    ],
                                )?;
                                ctx.next()?;
                            }
                        }
                    }
                    ctx.lines_as(
                        "Moonho Ahn",
                        args![
                            "It's time for the",
                            "second round. This",
                            "time, I'll be the one",
                            "declaring my guess,",
                            "and you'll shake the coins."
                        ],
                    )?;
                    ctx.next()?;
                    'l4: loop {
                        if !(true) {
                            break 'l4;
                        }
                        'b4: {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["(^666666What should I guess?^000000)"],
                            )?;
                            ctx.next()?;
                            l_amuro = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
                            if Val::from(runtime::select_values(ctx, &[Val::from("Odd:Even")])?) == 1 {
                                if l_amuro.clone() == 1 {
                                    l_number_false_2 = (l_number_false_2.clone() + Val::from(1));
                                    ctx.lines_as("Moonho Ahn", args!["Let's see...", "It's ^0000FFOdd^000000, isn't it?"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Moonho Ahn", args!["Looks like I won!", "Hahahahahahaha!"])?;
                                    ctx.next()?;
                                } else if l_amuro.clone() == 2 {
                                    l_number_right_2 = (l_number_right_2.clone() + Val::from(1));
                                    ctx.lines_as("Moonho Ahn", args!["Let's see...", "It's Even, isn't it?"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Moonho Ahn", args!["I lost?", "So it's ^0000FFOdd^000000..."])?;
                                    ctx.next()?;
                                }
                                if l_number_right_2.clone() == 2 {
                                    l_win = (l_win.clone() + Val::from(1));
                                    ctx.lines_as(
                                        "Moonho Ahn",
                                        args!["Nice job. I didn't", "expect for you to", "really beat me...", "Hahahahahahah~"],
                                    )?;
                                    ctx.next()?;
                                    break 'l4;
                                } else if l_number_false_2.clone() == 2 {
                                    l_lose = (l_lose.clone() + Val::from(1));
                                    ctx.lines_as("Moonho Ahn", args!["Hahaha! I'm sorry, but", "I won. I guess I still got it!"])?;
                                    ctx.next()?;
                                    break 'l4;
                                }
                            } else {
                                if l_amuro.clone() == 1 {
                                    l_number_right_2 = (l_number_right_2.clone() + Val::from(1));
                                    ctx.lines_as("Moonho Ahn", args!["Let's see...", "It's odd, isn't it?"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Moonho Ahn", args!["I lost, huh?", "So it was ^0000ffEven^000000..."])?;
                                    ctx.next()?;
                                } else if l_amuro.clone() == 2 {
                                    l_number_false_2 = (l_number_false_2.clone() + Val::from(1));
                                    ctx.lines_as("Moonho Ahn", args!["Let's see...", "It's ^0000FFEven^000000, isn't it?"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Moonho Ahn", args!["Looks like I won!", "Hahahahahahaha!"])?;
                                    ctx.next()?;
                                }
                                if l_number_right_2.clone() == 2 {
                                    l_win = (l_win.clone() + Val::from(1));
                                    ctx.lines_as(
                                        "Moonho Ahn",
                                        args!["Nice job. I didn't", "expect for you to", "really beat me...", "Hahahahahahah~"],
                                    )?;
                                    ctx.next()?;
                                    break 'l4;
                                } else if l_number_false_2.clone() == 2 {
                                    l_lose = (l_lose.clone() + Val::from(1));
                                    ctx.lines_as("Moonho Ahn", args!["Hahaha! I'm sorry, but", "I won. I guess I still got it!"])?;
                                    ctx.next()?;
                                    break 'l4;
                                }
                            }
                        }
                    }
                    if l_win.clone() == 2 {
                        ctx.lines_as(
                            "Moonho Ahn",
                            args![
                                "You really are amazing!",
                                "It's time that the title of",
                                "White Meteor be passed",
                                "onto someone more worthy...",
                                "You are the new White Meteor!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Really? Uh, can I get", "that in writing please?", "It'd mean so much to me!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Moonho Ahn",
                            args![
                                "Ah, you want a letter",
                                "of recommendation or",
                                "my seal of approval, right?",
                                "Heh, you must be very proud.",
                                "Hold still for a second."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.mes("^3355FF*Scribble Scribble*^000000")?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["What does this", "scribble on my", "wrist mean?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Moonho Ahn",
                            args![
                                "That is the sign of",
                                "the White Meteor.",
                                "Everyone that knows",
                                "me will recognize it",
                                "and its authenticity."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Thank you so much",
                                "for the fun game!",
                                "I better get going now,",
                                "but maybe we can play",
                                "again sometime~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Moonho Ahn",
                            args![
                                "Hahahaa!",
                                "After so long, it",
                                "feels good to have",
                                "played with a worthy",
                                "opponent. Thank you~"
                            ],
                        )?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(3124), Val::from(3125)])?;
                        ctx.var("zdan_edq").set(Val::from(7))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if l_lose.clone() == 2 {
                        ctx.lines_as(
                            "Moonho Ahn",
                            args![
                                "I'm sorry, but I've won",
                                "the game. I had a really",
                                "great time, and I could",
                                "tell that it was really close.",
                                "It's been a while since I've",
                                "had this much fun gambling~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Moonho Ahn",
                            args![
                                "Oh, did you want to",
                                "play again? I can tell",
                                "that you're a bit eager to",
                                "beat me so you're welcome",
                                "to play me again anytime.",
                                "I'll be right here, hahaha~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Moonho Ahn",
                            args![
                                "Ooh, this is exciting~",
                                "We each both won a round:",
                                "the next round will decide",
                                "who wins and loses!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Moonho Ahn",
                            args!["It's my turn to shake", "the coins. Let's see", "how you fare this time..."],
                        )?;
                        ctx.next()?;
                        ctx.lines(args!["^3355FF*Shake Shake*", "Guess! Is it", "Odd or Even?^000000"])?;
                        ctx.next()?;
                        'l5: loop {
                            if !(true) {
                                break 'l5;
                            }
                            'b5: {
                                l_number = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
                                let (input, status) = runtime::input_text(ctx, None, None)?;
                                l_input_s = input;
                                if ((l_input_s.clone() == "Odd" && l_number.clone() == 1)
                                    || (l_input_s.clone() == "Even" && l_number.clone() == 2))
                                {
                                    l_number_right_3 = (l_number_right_3.clone() + Val::from(1));
                                    ctx.lines_as(
                                        "Moonho Ahn",
                                        args![
                                            ((Val::from("^0000FF") + l_input_s.clone()) + Val::from("^000000?")),
                                            "Okay, you won."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    if (l_number_right_3.clone().number()? < 2 && l_number_false_3.clone().number()? < 2) {
                                        ctx.lines(args!["^3355FF*Shake Shake*", "Guess! Is it", "Odd or Even?^000000"])?;
                                        ctx.next()?;
                                    }
                                } else if ((l_input_s.clone() == "Odd" && l_number.clone() == 2)
                                    || (l_input_s.clone() == "Even" && l_number.clone() == 1))
                                {
                                    l_number_false_3 = (l_number_false_3.clone() + Val::from(1));
                                    ctx.lines_as(
                                        "Moonho Ahn",
                                        args![
                                            ((Val::from("Well, it's ^0000FF") + l_input_s.clone()) + Val::from("^000000.")),
                                            "Looks like I won."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    if (l_number_right_3.clone().number()? < 2 && l_number_false_3.clone().number()? < 2) {
                                        ctx.lines(args!["^3355FF*Shake Shake*", "Guess! Is it", "Odd or Even?^000000"])?;
                                        ctx.next()?;
                                    }
                                }
                                if l_number_right_3.clone() == 2 {
                                    ctx.lines_as("Moonho Ahn", args!["Oh... You won..."])?;
                                    l_win = (l_win.clone() + Val::from(1));
                                    ctx.next()?;
                                    break 'l5;
                                } else if l_number_false_3.clone() == 2 {
                                    ctx.lines_as("Moonho Ahn", args!["Hahaha! I'm sorry, but", "I won. I guess I still got it!"])?;
                                    l_lose = (l_lose.clone() + Val::from(1));
                                    ctx.next()?;
                                    break 'l5;
                                }
                                if (l_input_s.clone() != "Even" && l_input_s.clone() != "Odd") {
                                    ctx.lines_as(
                                        "Moonho Ahn",
                                        args![
                                            "You can only declare",
                                            "your guess as ^0000FFOdd^000000 or ^0000FFEven^000000.",
                                            "Please try entering it again."
                                        ],
                                    )?;
                                    ctx.next()?;
                                }
                            }
                        }
                        if l_win.clone().number()? >= 2 {
                            ctx.lines_as(
                                "Moonho Ahn",
                                args![
                                    "You really are amazing!",
                                    "It's time that the title of",
                                    "White Meteor be passed",
                                    "onto someone more worthy...",
                                    "You are the new White Meteor!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Really? Uh, can I get", "that in writing please?", "It'd mean so much to me!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Moonho Ahn",
                                args![
                                    "Ah, you want a letter",
                                    "of recommendation or",
                                    "my seal of approval, right?",
                                    "Heh, you must be very proud.",
                                    "Hold still for a second."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.mes("^3355FF*Scribble Scribble*^000000")?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["What does this", "scribble on my", "wrist mean?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Moonho Ahn",
                                args![
                                    "That is the sign of",
                                    "the White Meteor.",
                                    "Everyone that knows",
                                    "me will recognize it",
                                    "and its authenticity."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Thank you so much",
                                    "for the fun game!",
                                    "I better get going now,",
                                    "but maybe we can play",
                                    "again sometime~"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Moonho Ahn",
                                args![
                                    "Hahahaa!",
                                    "After so long, it",
                                    "feels good to have",
                                    "played with a worthy",
                                    "opponent. Thank you~"
                                ],
                            )?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(3124), Val::from(3125)])?;
                            ctx.var("zdan_edq").set(Val::from(7))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if l_lose.clone() == 2 {
                            ctx.lines_as(
                                "Moonho Ahn",
                                args![
                                    "I'm sorry, but I've won",
                                    "the game. I had a really",
                                    "great time, and I could",
                                    "tell that it was really close.",
                                    "It's been a while since I've",
                                    "had this much fun gambling~"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Moonho Ahn",
                                args![
                                    "Oh, did you want to",
                                    "play again? I can tell",
                                    "that you're a bit eager to",
                                    "beat me so you're welcome",
                                    "to play me again anytime.",
                                    "I'll be right here, hahaha~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                } else {
                    ctx.lines_as(
                        "Moonho Ahn",
                        args![
                            "Hm? You don't even",
                            "have 500 zeny to play",
                            "a game? Well, whenever",
                            "you feel like playing Coin",
                            "Shake, drop on by."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines_as(
                    "Moonho Ahn",
                    args![
                        "This is basically",
                        "a guessing game played",
                        "in rounds, best 2 out of 3.",
                        "One of us will be the coin",
                        "shaker, and the other will",
                        "be the guesser."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Moonho Ahn",
                    args![
                        "The coin shaker begins",
                        "the round by grabbing",
                        "a random amount of coins,",
                        "shaking them in his closed",
                        "hands, and then stopping,",
                        "keeping the coins concealed."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Moonho Ahn",
                    args![
                        "The other person, the",
                        "guesser, will then declare",
                        "a guess on whether the total",
                        "value of the coins is ^0000FFOdd^000000 or",
                        "^0000FFEven^000000. Afterwards, the coin",
                        "shaker reveals the coins..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Moonho Ahn",
                    args![
                        "If the guesser is right,",
                        "then he wins the round.",
                        "However, if he guessed",
                        "wrong, then it's a victory",
                        "for the coin shaker. "
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Moohno Ahn",
                    args![
                        "We each take turns,",
                        "switching roles.",
                        "If it's your turn to",
                        "guess, remember to",
                        "call out ^0000FFOdd^000000 or ^0000FFEven^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Moonho Ahn",
                    args![
                        "Lastly, I'm the house,",
                        "so I'll charge a game",
                        "participation fee. It's",
                        "not much, just 500 zeny.",
                        "We're only playing for fun,",
                        "not to break the bank."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    ctx.lines_as(
        "Moonho Ahn",
        args![
            "I still remember that",
            "last legendary match with",
            "Dalho Kwak. Now that man was",
            "a worthy opponent: he could",
            "read others' hands three times",
            "faster than the normal eye!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Moonho Ahn",
        args![
            "God, I miss those",
            "days competing against",
            "him. I wonder how he's",
            "been doing lately..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn moonho_ahn(ctx: &Ctx) -> Script {
    moonho_ahn_body(ctx, Vec::new()).map(|_| ())
}

fn gooho_ahn_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("zdan_edq").get()? == 12 {
        ctx.lines_as(
            "Gooho Ahn",
            args!["Oh, hello", "adventurer.", "Um, was there", "something you wanted?"],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_QUESTION")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Rogue Guild's Decoding Request")])? {
            1 => {
                ctx.lines_as(
                    "Gooho Ahn",
                    args![
                        "Ah, Marybell sent you?",
                        "I see. I've been struggling",
                        "trying to decrypt this letter.",
                        "Did you want to take a look?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Gooho Ahn",
                    args![
                        "This looks like a piece",
                        "of blank paper, but if you",
                        "add just the right amount",
                        "of heat and light, you can",
                        "read its contents. This is",
                        "some powerful encryption!"
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_HUK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Gooho Ahn",
                    args![
                        "So far, I've only decrypted",
                        "just a portion of the letter's",
                        "content. Unfortunately, I ran",
                        "out of the materials I need to",
                        "measure how much light and heat",
                        "I need to unlock this message."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Gooho Ahn",
                    args![
                        "I'm pretty swamped over",
                        "here, but if you're working",
                        "for Marybell, I don't think",
                        "you'd mind doing a favor",
                        "for me. Would you gather",
                        "the materials I need?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Gooho Ahn",
                    args![
                        "I need",
                        "^0000FF20 Live Coals^000000,",
                        "^0000FF1 Matchstick^000000,",
                        "^0000FF2 Alcohol^000000, and",
                        "^0000FF10 Burning Hearts^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Gooho Ahn",
                    args![
                        "Once I have the things",
                        "I need, I should be able",
                        "to decrypt the rest of",
                        "this secret Z Gang letter."
                    ],
                )?;
                ctx.call(Function::ChangeQuest, vec![Val::from(3129), Val::from(3130)])?;
                ctx.var("zdan_edq").set(Val::from(13))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("zdan_edq").get()? == 13 {
        ctx.lines_as(
            "Gooho Ahn",
            args![
                "So did you bring the",
                "things I need to measure",
                "how much light and heat",
                "I need to reveal the rest",
                "of this secret Z Gang letter?"
            ],
        )?;
        ctx.next()?;
        if (((ctx.call(Function::CountItem, vec![Val::from(7098)])?.number()? > 19
            && ctx.call(Function::CountItem, vec![Val::from(970)])?.number()? > 1)
            && ctx.call(Function::CountItem, vec![Val::from(7035)])?.number()? > 0)
            && ctx.call(Function::CountItem, vec![Val::from(7097)])?.number()? > 9)
        {
            ctx.lines_as(
                "Gooho Ahn",
                args![
                    "Oh, good. Thanks a lot.",
                    "Now I should do my part,",
                    "and decrypt the rest of",
                    "this letter. Mm... Just",
                    "give me a second to",
                    "make some adjustments..."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::DelItem, vec![Val::from(7098), Val::from(20)])?;
            ctx.call(Function::DelItem, vec![Val::from(970), Val::from(2)])?;
            ctx.call(Function::DelItem, vec![Val::from(7035), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(7097), Val::from(10)])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
            ctx.lines_as(
                "Gooho Ahn",
                args![
                    "Oh, what? This language...",
                    "It's ancient! Aegye hasn't",
                    "been used in... centuries.",
                    "I've seen Aegye used in the",
                    "Book of Forbidden Mystery",
                    "before it was stolen."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Gooho Ahn",
                args![
                    "There's a legend about",
                    "an ancient civilization that",
                    "set the foundation for our",
                    "modern society. The Aegye",
                    "language was considered",
                    "a subcultural phenomenon."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Gooho Ahn",
                args![
                    "Aegye was popular among",
                    "children and rebellious",
                    "teens, but was considered",
                    "vulgar until it entered the",
                    "mainstream. Then, somehow... "
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "[Gooho Ahn] ",
                "Aegye destroyed that",
                "ancient people's lingual",
                "system and civilization",
                "once it was popularized.",
                "That is why there is strong",
                "Aegye censorship today."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "[Gooho Ahn] ",
                "In fact, the existence",
                "of Aegye is supposed to",
                "be a huge secret. That",
                "language is responsible",
                "for one of the greatest",
                "disasters of all time."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["...............................", "So what's the letter say?"],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "[Gooho Ahn] ",
                "Mm... Let me check...",
                "My knowledge in Aegye",
                "is very limited, so..."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "[Gooho Ahn] ",
                "^0000ffWeii arr prowd Z G gna^000000",
                "^0000ffAynoen hwo sspotp uys^000000",
                "^0000ffwlil eb kckide on htier ssa!^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["What...?"])?;
            ctx.next()?;
            ctx.lines(args![
                "[Gooho Ahn] ",
                "That's the nature",
                "of Aegye: it is chaotic,",
                "and barely legible!",
                "Regardless, this letter",
                "is invaluable as research",
                "material for my studies."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "[Gooho Ahn] ",
                "It will be enough",
                "for Marybell if you",
                "deliver the message to",
                "her verbatim. Listen",
                "to me carefully, and",
                "maybe write this down."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "[Gooho Ahn] ",
                "^0000ffWeii arr prowd Z G gna^000000",
                "^0000ffAynoen hwo sspotp uys^000000",
                "^0000ffwlil eb kckide on htier ssa!^000000"
            ])?;
            ctx.call(Function::ChangeQuest, vec![Val::from(3130), Val::from(3131)])?;
            ctx.var("zdan_edq").set(Val::from(14))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Gooho Ahn",
                args![
                    "Please hurry!",
                    "I'm sure Marybell is",
                    "waiting to learn the",
                    "whereabouts of the Z Gang,",
                    "and this letter might have",
                    "a clue. Oh, and remember..."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "[Gooho Ahn] ",
                "I need",
                "^0000FF20 Live Coals^000000,",
                "^0000FF1 Matchstick^000000,",
                "^0000FF2 Alcohol^000000, and",
                "^0000FF10 Burning Hearts^000000."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Gooho Ahn",
                args![
                    "Once I have the things",
                    "I need, I should be able",
                    "to decrypt the rest of",
                    "this secret Z Gang letter."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("zdan_edq").get()? == 14 {
        ctx.lines(args![
            "[Gooho Ahn] ",
            "Please tell Marybell",
            "the contents of the",
            "Z Gang's letter so that",
            "I can keep the original",
            "document for research purposes."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "[Gooho Ahn] ",
            "^0000ffWeii arr prowd Z G gna^000000",
            "^0000ffAynoen hwo sspotp uys^000000",
            "^0000ffwlil eb kckide on htier ssa!^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "[Gooho Ahn] ",
        "Encryption is the art of",
        "hiding truth in layers of",
        "secrecy. My specialty is",
        "in unraveling the truth",
        "by breaking encryption."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn gooho_ahn(ctx: &Ctx) -> Script {
    gooho_ahn_body(ctx, Vec::new()).map(|_| ())
}

pub fn suspicious_man_1(ctx: &Ctx) -> Script {
    suspicious_man_1_run(ctx, SuspiciousMan1Step::Start, Vec::new()).map(|_| ())
}

pub fn suspicious_man_1_ontimer30000(ctx: &Ctx) -> Script {
    suspicious_man_1_run(ctx, SuspiciousMan1Step::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn suspicious_man_1_oninit(ctx: &Ctx) -> Script {
    suspicious_man_1_run(ctx, SuspiciousMan1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn suspicious_man_1_ontouch(ctx: &Ctx) -> Script {
    suspicious_man_1_run(ctx, SuspiciousMan1Step::OnTouch, Vec::new()).map(|_| ())
}

fn odd_slab_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if (((ctx.var("zdan_edq").get()? == 15 || ctx.var("zdan_edq").get()? == 16) || ctx.var("zdan_edq").get()? == 17)
        && ctx.var("$@door2").get()? == 0)
    {
        ctx.lines_as("Odd Slab", args!["^FF0000*Creak Creak*", "Etner sspawrod.^000000"])?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HUK")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "This slab is speaking",
                "to me! I... I think. Now",
                "where have I heard",
                "talking like this before?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Odd Slab", args!["^FF0000*Creak Creak*", "Etner sspawrod.^000000"])?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_AHA")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Oh, right! This is that",
                "trashy language, Aegye,",
                "that Gooho Ahn told me",
                "about. It still sounds",
                "like... A poor excuse",
                "for language to me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Odd Slab", args!["^FF0000*Creak Creak*", "Etner sspawrod.^000000"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Well, I guess", "I should talk to it. Um..."],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s.clone() == "Weii arr prowd Z G gna" {
            ctx.lines_as("Odd Slab", args!["^FF0000*Creak Creak*", "Etner n2d sspawrod.^000000"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Whoa! It's saying",
                    "something different now!",
                    "I must be doing alright.",
                    "Now what do I say?"
                ],
            )?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_input_s = input;
            if l_input_s.clone() == "Aynoen hwo sspotp uys" {
                ctx.lines_as("Odd Slab", args!["^FF0000*Creak Creak*", "Etner r3d sspawrod.^000000"])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Heh! I'm on the right", "track. Now what do I say?"],
                )?;
                ctx.next()?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                if l_input_s.clone() == "wlil eb kckide on htier ssa!" {
                    ctx.lines_as("Odd Slab", args!["*Creak Creak*", "*Creak Creak*"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Um... Now what...?"],
                    )?;
                    ctx.next()?;
                    if ctx.var("$@door2").get()? == 0 {
                        ctx.var("$@door2").set(Val::from(1))?;
                        ctx.mes("Waaaah! Waaah!")?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["What th--?! What's", "happening?! I'm being", "sucked away somewhere!"],
                        )?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(3132), Val::from(3133)])?;
                        ctx.var("zdan_edq").set(Val::from(16))?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("z_agit"), Val::from(98), Val::from(40)])?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Hm? Did I do something", "wrong? It just stopped", "working all of a sudden."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Odd Slab",
                            args!["^FF0000Rrsoy, rrsoy.", "Ai cn'at elt yoo ni ofr nwo. Plzea ecmo ckba tela.^000000"],
                        )?;
                        ctx.next()?;
                        ctx.mes("^666666*Pzzzzz*^000000")?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["I think...", "I think I should", "try this again."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.lines_as("Odd Slab", args!["^666666*Pzzzzz*^000000"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Huh? This talking slab",
                            "thing isn't working now.",
                            "Hello? What happened to you?",
                            "Hey! Talk to me, will you?"
                        ],
                    )?;
                    ctx.var("$@door2").set(Val::from(0))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                ctx.lines_as("Odd Slab", args!["^666666*Pzzzzz*^000000"])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Huh? This talking slab",
                        "thing isn't working now.",
                        "Hello? What happened to you?",
                        "Hey! Talk to me, will you?"
                    ],
                )?;
                ctx.var("$@door2").set(Val::from(0))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as("Odd Slab", args!["^666666*Pzzzzz*^000000"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Huh? This talking slab",
                    "thing isn't working now.",
                    "Hello? What happened to you?",
                    "Hey! Talk to me, will you?"
                ],
            )?;
            ctx.var("$@door2").set(Val::from(0))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if (((ctx.var("zdan_edq").get()? == 15 || ctx.var("zdan_edq").get()? == 16) || ctx.var("zdan_edq").get()? == 17)
        && ctx.var("$@door2").get()?.number()? > 0)
    {
        ctx.lines_as("Odd Slab", args!["*Creak Creak*"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "This slab looks pretty",
                "strange. Is it making",
                "noises? Hmm... Maybe this",
                "is a clue to the Z Gang!",
                "I'll come back later",
                "to investigate this~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("zdan_edq").get()?.number()? > 17 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["This used to be", "where the Z Gang", "would hide out.", "Good riddance! "],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args![
            "This is a peculiar",
            "looking slab. It sounds",
            "like... What are those",
            "noises? Huh. Weird.",
            "It's like it's just",
            "talking gibberish."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn odd_slab(ctx: &Ctx) -> Script {
    odd_slab_body(ctx, Vec::new()).map(|_| ())
}

fn odd_slab_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$@zdan").set(Val::from(0))?;
    ctx.var("$@door2").set(Val::from(0))?;
    ctx.var("$@mosnter").set(Val::from(0))?;
    return Err(Stop::End);
}

pub fn odd_slab_oninit(ctx: &Ctx) -> Script {
    odd_slab_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn odd_slab_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("zdan_edq").get()? == 15 {
        ctx.mes("^FF0000*Creak Creak*^000000")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Did I just...", "Hear something", "around here?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn odd_slab_ontouch(ctx: &Ctx) -> Script {
    odd_slab_ontouch_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Entrancecheck1Step {
    Start,
    OnTouch,
}

fn entrancecheck_1_run(ctx: &Ctx, mut step: Entrancecheck1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Entrancecheck1Step::Start => {
                step = Entrancecheck1Step::OnTouch;
                continue 'machine;
            }
            Entrancecheck1Step::OnTouch => {
                if ((ctx.var("zdan_edq").get()? == 15 || ctx.var("zdan_edq").get()? == 16) && ctx.var("$@monster_zgang").get()? == 0) {
                    ctx.var("$@monster_zgang").set(Val::from(1))?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#zdan_broad::OnEnable")])?;
                } else if ((ctx.var("zdan_edq").get()? == 15 || ctx.var("zdan_edq").get()? == 16)
                    && ctx.var("$@monster_zgang").get()?.number()? > 0)
                {
                    return Err(Stop::End);
                } else if ctx.var("zdan_edq").get()? == 17 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#ZGuard::OnDisable")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Louis")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Martha")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Catfoii")])?;
                } else {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Where am I...?",
                            "Something has gone",
                            "terribly wrong, hasn't",
                            "it? Let me go baaaack~!"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.var("$@monster_zgang").set(Val::from(0))?;
                    ctx.var("$@door2").set(Val::from(0))?;
                    ctx.call(Function::Warp, vec![Val::from("moc_fild17"), Val::from(209), Val::from(235)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn entrancecheck_1(ctx: &Ctx) -> Script {
    entrancecheck_1_run(ctx, Entrancecheck1Step::Start, Vec::new()).map(|_| ())
}

pub fn entrancecheck_1_ontouch(ctx: &Ctx) -> Script {
    entrancecheck_1_run(ctx, Entrancecheck1Step::OnTouch, Vec::new()).map(|_| ())
}
