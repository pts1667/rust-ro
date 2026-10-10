use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn silk_sand_camel_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rachel_camel").get()? == 11 {
        ctx.lines(args![
            "^3355FFThis camel's leg is",
            "wounded. Although it",
            "seems hurt, its nostrils",
            "flared as soon as it saw",
            "the camel appetite stimulant,",
            "and it smacked its lips.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as("Camel", args!["^333333*Chew Chew~*^000000", "^333333*Smacks lips*^000000"])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThe camel started nibbling",
            "the stimulant, but its eating",
            "became quicker as it ate",
            "more of the feed until it",
            "was completely consumed.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "This must be the",
                "Silk Sand Camel...",
                "I guess all I need to",
                "do is collect some of",
                "that precious camel dung."
            ],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3070), Val::from(3071)])?;
        ctx.var("rachel_camel").set(Val::from(12))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if (ctx.var("rachel_camel").get()?.number()? > 11 && ctx.var("rachel_camel").get()?.number()? < 17) {
            if (((ctx.call(Function::CountItem, vec![Val::from(511)])?.number()? > 19
                && ctx.call(Function::CountItem, vec![Val::from(909)])?.number()? > 9)
                && ctx.call(Function::CountItem, vec![Val::from(519)])?.number()? > 1)
                && ctx.call(Function::CountItem, vec![Val::from(713)])?.number()? > 0)
            {
                ctx.lines(args![
                    "^3355FFThe camel can smell",
                    "that you have food for",
                    "it, and started salivating.",
                    "You may as well just feed it.^000000"
                ])?;
                ctx.next()?;
                let subject1 = ctx.call(Function::Rand, vec![Val::from(3)])?;
                if subject1 == 1 {
                    if ctx.var("rachel_camel").get()? == 12 {
                        ctx.lines_as(
                            "Silk Sand Camel",
                            args!["^333333*Chew Chew~*^000000", "^333333*Smacks lips*^000000"],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFThe camel ate everything,",
                            "but it doesn't seem like",
                            "it'll go through any bowel",
                            "movements anytime soon.^000000"
                        ])?;
                    } else {
                        ctx.lines(args![
                            "^3355FFThe camel grimaced",
                            "as if it were suffering",
                            "from a stomachache...",
                            "And... Out pops 2 Sweet",
                            "Potatoes. They're probably",
                            "safe to eat... Hopefully.^000000"
                        ])?;
                        ctx.call(Function::GetItem, vec![Val::from(516), Val::from(2)])?;
                    }
                    ctx.call(Function::DelItem, vec![Val::from(519), Val::from(2)])?;
                    ctx.call(Function::DelItem, vec![Val::from(511), Val::from(20)])?;
                    ctx.call(Function::DelItem, vec![Val::from(909), Val::from(10)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject1 == 2 {
                    ctx.lines_as(
                        "Silk Sand Camel",
                        args!["^333333*Chew Chew~*^000000", "^333333*Smacks lips*^000000"],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFThe camel grimaced",
                        "as if it were suffering",
                        "from a stomachache...",
                        "Huzzah! You got a lump",
                        "of steaming camel dung!",
                        "This is cause for celebration!^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
                    ])?;
                    if ctx.var("rachel_camel").get()? == 12 {
                        ctx.lines(args!["Now all I need is", "just 4 more lumps", "of this nasty old dung."])?;
                        ctx.var("rachel_camel").set(Val::from(13))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(3071), Val::from(3072)])?;
                    } else if ctx.var("rachel_camel").get()? == 13 {
                        ctx.lines(args!["Awesome! I got", "2 glorious camel dung", "lumps! Only 3 more to go!"])?;
                        ctx.var("rachel_camel").set(Val::from(14))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(3072), Val::from(3073)])?;
                    } else if ctx.var("rachel_camel").get()? == 14 {
                        ctx.lines(args![
                            "Yes! Now I have",
                            "3 camel dung lumps.",
                            "Just 2 more... I'm more",
                            "than halfway done!"
                        ])?;
                        ctx.var("rachel_camel").set(Val::from(15))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(3073), Val::from(3074)])?;
                    } else if ctx.var("rachel_camel").get()? == 15 {
                        ctx.lines(args![
                            "4 lumps of camel dung...",
                            "Heh heh! This is going",
                            "better than I thought!",
                            "Only 1 more to go!"
                        ])?;
                        ctx.var("rachel_camel").set(Val::from(16))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(3074), Val::from(3075)])?;
                    } else if ctx.var("rachel_camel").get()? == 16 {
                        ctx.lines(args![
                            "In my hands...",
                            "I am holding",
                            "5 lumps of camel dung.",
                            "This is my finest moment."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Never, in all my years",
                                "of adventuring, saving the",
                                "oppressed, protecting the",
                                "innocent, did I dare dream",
                                "that I'd accomplish such",
                                "a magnificent feat."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "I am so happy--nay--",
                                "^4D4DFFproud^000000 that my strength, my",
                                "valor, and my determination",
                                "was up to this task. May the",
                                "annals of history never forget",
                                ((Val::from("this day! Long live ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                    + Val::from("!"))
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args!["^3355FFIt's time for you to", "return to Mr. Saraman.^000000"])?;
                        ctx.var("rachel_camel").set(Val::from(17))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(3075), Val::from(3076)])?;
                    }
                    ctx.call(Function::DelItem, vec![Val::from(519), Val::from(2)])?;
                    ctx.call(Function::DelItem, vec![Val::from(511), Val::from(20)])?;
                    ctx.call(Function::DelItem, vec![Val::from(909), Val::from(10)])?;
                    ctx.call(Function::DelItem, vec![Val::from(713), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Silk Sand Camel",
                        args!["^333333*Chew Chew~*^000000", "^333333*Smacks lips*^000000"],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFThe camel grimaced",
                        "as if it were suffering",
                        "from a stomachache...",
                        "And... Out pops a Sweet",
                        "Potato. It's probably",
                        "safe to eat... Maybe.^000000"
                    ])?;
                    ctx.call(Function::DelItem, vec![Val::from(519), Val::from(2)])?;
                    ctx.call(Function::DelItem, vec![Val::from(511), Val::from(20)])?;
                    ctx.call(Function::DelItem, vec![Val::from(909), Val::from(10)])?;
                    ctx.call(Function::GetItem, vec![Val::from(516), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                ctx.lines(args![
                    ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
                ])?;
                if ctx.var("rachel_camel").get()? == 12 {
                    ctx.lines(args![
                        "I need to feed this camel if",
                        "I ever want to get any dung",
                        "from it. Let's see, Mr. Saraman",
                        "mentioned that the items I got",
                        "for Ms. Ivory were actually",
                        "camel feed. I need to have..."
                    ])?;
                } else {
                    ctx.lines(args![
                        "Did I run out of feed",
                        "already? Nuts, if I want",
                        "to feed this camel again,",
                        "then I need to bring it..."
                    ])?;
                }
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "^4D4DFF2 Milk^000000,",
                        "^4D4DFF20 Green Herbs^000000,",
                        "^4D4DFF10 Jellopies^000000, and",
                        "^4D4DFF1 Empty Bottles^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Silk Sand Camel", args!["*Chew Chew*", "*Neigh Neigh*~"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("rachel_camel").get()? == 17 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "I managed to get the soap",
                        "ingredients: 5 of those",
                        "camel dung lumps. I should",
                        "head back to Mr. Saraman to",
                        "tell him where his camel is,",
                        "and then go to Ms. Ivory."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as("Silk Sand Camel", args!["*Neigh Neigh*~"])?;
                ctx.next()?;
                ctx.mes("^3355FFSilly camel.^000000")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn silk_sand_camel(ctx: &Ctx) -> Script {
    silk_sand_camel_body(ctx, Vec::new()).map(|_| ())
}

fn young_town_native_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rachel_camel").get()? == 3 {
        ctx.lines_as(
            "Native Young Man",
            args![
                "My name is Toby.",
                "I was born and raised here,",
                "and no one knows more about",
                "this town than me. Feel free",
                "to ask if you need to find",
                "your way around here."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Excuse me, but do", "you know where I can", "find a locksmith?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Toby",
            args![
                "Of course, I do!",
                "Mr. Lockenlock is a famous",
                "locksmith, and he makes almost",
                "all the keys and locks in Veins",
                "and even in Rachel."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Mr. Lockenlock, eh?", "So where can I find him?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Toby",
            args![
                "Oh, he's always sitting",
                "somewhere in the market",
                "street. He drinks a lot,",
                "though, so he doesn't really",
                "work when he's hung over."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Toby",
            args![
                "Ah, but you know what'll",
                "shock him back to sobriety?",
                "A Yellow Potion! It never",
                "fails with that guy!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I see.", "Thanks for", "the advice."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Toby",
            args![
                "You're so very",
                "welcome! It's just...",
                "After all these years...",
                "I'm finally useful to someone!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...Ha...?"])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFFind Mr. Lockenlock",
            "in the market street, and",
            "bring him a Yellow Potion.^000000"
        ])?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3062), Val::from(3063)])?;
        ctx.var("rachel_camel").set(Val::from(4))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rachel_camel").get()? == 4 {
        ctx.lines_as("Toby", args!["After all these years...", "I'm finally useful to someone!"])?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...Ha...?"])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFFind Mr. Lockenlock",
            "in the market street, and",
            "bring him a Yellow Potion.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Native Young Man",
            args![
                "My name is Toby.",
                "I was born and raised here,",
                "and no one knows more about",
                "this town than me. Feel free",
                "to ask if you need to find",
                "your way around here."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["No, thanks."])?;
        ctx.next()?;
        ctx.lines_as(
            "Native Young Man",
            args!["You don't...", "^333333*Sob*^000000 You don't", "need me at all?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn young_town_native(ctx: &Ctx) -> Script {
    young_town_native_body(ctx, Vec::new()).map(|_| ())
}

fn rachel_guard_vol1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.var("aru_vol").get()? == 2 {
        ctx.lines_as(
            "Guard Karlum",
            args![
                "High Priest Vildt isn't",
                "here right now. Please",
                "come back later if you",
                "wish to see him."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Think of a Distraction")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "This guy's not going to",
                "let me pass. Let's see...",
                "Is there some way I could",
                "get him to leave? What, or",
                "even ^FF0000who^000000, could distract him?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Wait a second...", "Of course! I should", "talk to him about..."],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Wait a second...",
                "Of course! I should",
                ((Val::from("talk to him about ^FF0000") + l_input_s.clone()) + Val::from("^000000 !!"))
            ],
        )?;
        ctx.next()?;
        if l_input_s.clone() != "Lamir" {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["What the...?", "Where did I think of that?", "That doesn't make any sense..."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "That's right! I talked",
                "to Lamir a while ago.",
                "If she's right, then this",
                "guy must be Karlum, the guy",
                "who's totally in love with her.",
                "Hmm... I know what I'll say..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "^333333*Ahem*^000000 Excuse me,",
                "but are you Karlum?",
                "I've got a message for you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guard Karlum",
            args![
                "A message for me?",
                "Is that why you're still",
                "loitering around? Well,",
                "spit it out. I can't waste",
                "too much time on the job..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["You know ^3131FFLamir^000000, right?"],
        )?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
        ctx.lines_as(
            "Guard Karlum",
            args!["Lamir? Oh... My.", "Oh no! Did something", "bad happen to her?", "Quick, tell me!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "No, nothing like that.",
                "She just told me that she",
                "had something important to",
                "tell you, and that you had to",
                "come see her when you're free."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "I tried to ask her",
                "more, but she just kept",
                "blushing and turning away.",
                "Is there something going",
                "on between you too?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guard Karlum",
            args![
                "...I don't believe it.",
                "Finally. After all these",
                "years. She feels the same",
                "way I feel for her! My midnight",
                "serenade a few days ago",
                "must've touched her heart."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guard Karlum",
            args![
                "Screw this stupid job!",
                "I've made my choice, and",
                "I choose true love! I can't",
                "keep Lamir waiting any longer!"
            ],
        )?;
        ctx.var("aru_vol").set(Val::from(3))?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("vol_time::OnEnable")])?;
        ctx.close_window()?;
        ctx.call(Function::DisableNpc, vec![Val::from("Rachel Guard#vol1")])?;
        return Err(Stop::End);
    } else if (ctx.var("aru_vol").get()?.number()? > 2 && ctx.var("aru_vol").get()?.number()? < 5) {
        ctx.lines_as(
            "Guard Karlum",
            args![
                "Hey! Lamir told me that",
                "she didn't want to see",
                "me at all! What's your",
                "game, huh? Do you think",
                "I'm that easy to trick?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "What? Is that what",
                "happened? I could've",
                "sworn th--Oooh. I get it now.",
                "She must be playing hard to",
                "get. That must mean that",
                "she's reeeeally into you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guard Karlum",
            args![
                "Ah! That makes perfect",
                "sense! No wonder she treated",
                "me that way! Hahaha! I should",
                "have figured it out sooner!",
                "Well then, I should go see",
                "her and play hard to get too!"
            ],
        )?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("vol_time::OnEnable")])?;
        ctx.close_window()?;
        ctx.call(Function::DisableNpc, vec![Val::from("Rachel Guard#vol1")])?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Guard Karlum",
        args![
            "High Priest Vildt isn't",
            "here right now. Please",
            "come back later if you",
            "wish to see him."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn rachel_guard_vol1(ctx: &Ctx) -> Script {
    rachel_guard_vol1_body(ctx, Vec::new()).map(|_| ())
}

fn rachel_guard_vol1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Rachel Guard#vol1")])?;
    return Err(Stop::End);
}

pub fn rachel_guard_vol1_oninit(ctx: &Ctx) -> Script {
    rachel_guard_vol1_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn rachel_guard_vol1_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Warp, vec![Val::from("ra_temin"), Val::from(85), Val::from(137)])?;
    return Err(Stop::End);
}

pub fn rachel_guard_vol1_ontouch(ctx: &Ctx) -> Script {
    rachel_guard_vol1_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn rachel_guard_vol2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Guard Krodger",
        args![
            "High Priest Vildt isn't",
            "here right now. Please",
            "come back later if you",
            "wish to see him."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn rachel_guard_vol2(ctx: &Ctx) -> Script {
    rachel_guard_vol2_body(ctx, Vec::new()).map(|_| ())
}

fn rachel_guard_vol2_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Rachel Guard#vol2")])?;
    return Err(Stop::End);
}

pub fn rachel_guard_vol2_oninit(ctx: &Ctx) -> Script {
    rachel_guard_vol2_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn rachel_guard_vol2_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Warp, vec![Val::from("ra_temin"), Val::from(85), Val::from(137)])?;
    return Err(Stop::End);
}

pub fn rachel_guard_vol2_ontouch(ctx: &Ctx) -> Script {
    rachel_guard_vol2_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn flower_vase_vol_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("aru_vol").get()?.number()? > 2 && ctx.var("aru_vol").get()?.number()? < 5) {
        ctx.lines(args![
            "^3355FFYou find a giant",
            "vase full of beautiful",
            "flowers that look freshly",
            "picked from a garden.^000000"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Destroy Vase:Don't Destroy Vase")])?) == 1 {
            ctx.lines(args![
                "^3355FFYou grasp the flower",
                "vase with both hands, and",
                "then hurl it to the ground.^000000"
            ])?;
            ctx.next()?;
            ctx.mes("^3355FF*Crash!*^000000")?;
            ctx.next()?;
            ctx.lines_as("Guard Krodger", args!["Who's there?!"])?;
            ctx.call(Function::DisableNpc, vec![Val::from("Rachel Guard#vol2")])?;
            ctx.call(Function::EnableNpc, vec![Val::from("Rachel Guard#vol2_1")])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("vol_time2::OnEnable")])?;
            ctx.var("aru_vol").set(Val::from(4))?;
            ctx.call(Function::DisableNpc, vec![Val::from("Flower Vase#vol")])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "A lot of loving care",
                "was put into arranging",
                "these flowers.. I can't",
                "bear to disturb their beauty."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFAnd so you just",
            "stood there, looking",
            "a bit pitiable, but not",
            "really all that pathetic.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFYou find a giant",
        "vase full of beautiful",
        "flowers that look freshly",
        "picked from a garden.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn flower_vase_vol(ctx: &Ctx) -> Script {
    flower_vase_vol_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum PathVol1Step {
    Start,
    OnTouch,
}

fn path_vol1_run(ctx: &Ctx, mut step: PathVol1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PathVol1Step::Start => {
                step = PathVol1Step::OnTouch;
                continue 'machine;
            }
            PathVol1Step::OnTouch => {
                if (ctx.var("aru_vol").get()? != 3 && ctx.var("aru_vol").get()? != 4) {
                    ctx.call(Function::Warp, vec![Val::from("ra_temin"), Val::from(85), Val::from(137)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn path_vol1(ctx: &Ctx) -> Script {
    path_vol1_run(ctx, PathVol1Step::Start, Vec::new()).map(|_| ())
}

pub fn path_vol1_ontouch(ctx: &Ctx) -> Script {
    path_vol1_run(ctx, PathVol1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum PathVol12Step {
    Start,
    OnTouch,
}

fn path_vol1_2_run(ctx: &Ctx, mut step: PathVol12Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PathVol12Step::Start => {
                step = PathVol12Step::OnTouch;
                continue 'machine;
            }
            PathVol12Step::OnTouch => {
                if ctx.var("aru_vol").get()? != 4 {
                    ctx.call(Function::Warp, vec![Val::from("ra_temin"), Val::from(85), Val::from(137)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn path_vol1_2(ctx: &Ctx) -> Script {
    path_vol1_2_run(ctx, PathVol12Step::Start, Vec::new()).map(|_| ())
}

pub fn path_vol1_2_ontouch(ctx: &Ctx) -> Script {
    path_vol1_2_run(ctx, PathVol12Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum PathVol13Step {
    Start,
    OnTouch,
}

fn path_vol1_3_run(ctx: &Ctx, mut step: PathVol13Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PathVol13Step::Start => {
                step = PathVol13Step::OnTouch;
                continue 'machine;
            }
            PathVol13Step::OnTouch => {
                if ctx.var("aru_vol").get()? == 5 {
                    ctx.call(Function::Warp, vec![Val::from("ra_temin"), Val::from(84), Val::from(124)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn path_vol1_3(ctx: &Ctx) -> Script {
    path_vol1_3_run(ctx, PathVol13Step::Start, Vec::new()).map(|_| ())
}

pub fn path_vol1_3_ontouch(ctx: &Ctx) -> Script {
    path_vol1_3_run(ctx, PathVol13Step::OnTouch, Vec::new()).map(|_| ())
}

fn female_follower_vol_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Lamir",
        args![
            "^333333*Sigh*^000000 High Priest Vildt",
            "left over so much food after",
            "eating. Didn't he learn to",
            "finish all of his food?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lamir",
        args![
            "You know, my mother used",
            "to threaten that she'd force",
            "me to marry Karlum if I didn't",
            "finish all my food when I was",
            "a kid. I learned never to",
            "leave any leftovers that way~"
        ],
    )?;
    ctx.next()?;
    let choice = runtime::select_values(ctx, &[Val::from("Who's Karlum?")])?;
    ctx.var("@menu").set(choice)?;
    ctx.lines_as(
        "Lamir",
        args![
            "Karlum? Oh, he's been",
            "chasing me ever since we",
            "were kids, declaring his",
            "love and all that. Even",
            "after we grew up, he's still",
            "stubborn about that point."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lamir",
        args![
            "Ugh! Even today, he",
            "still gets on my nerves!",
            "I mean, it's great that he's",
            "a guard at High Priest Vildt's",
            "office, but come on! Why can't",
            "he bother another girl?"
        ],
    )?;
    if ctx.var("aru_vol").get()? == 1 {
        ctx.var("aru_vol").set(Val::from(2))?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn female_follower_vol(ctx: &Ctx) -> Script {
    female_follower_vol_body(ctx, Vec::new()).map(|_| ())
}

fn rachel_guard_vol1_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn rachel_guard_vol1_1(ctx: &Ctx) -> Script {
    rachel_guard_vol1_1_body(ctx, Vec::new()).map(|_| ())
}

fn rachel_guard_vol1_1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Rachel Guard#vol1_1")])?;
    return Err(Stop::End);
}

pub fn rachel_guard_vol1_1_oninit(ctx: &Ctx) -> Script {
    rachel_guard_vol1_1_oninit_body(ctx, Vec::new()).map(|_| ())
}

pub fn vol_time(ctx: &Ctx) -> Script {
    vol_time_run(ctx, VolTimeStep::Start, Vec::new()).map(|_| ())
}

pub fn vol_time_oninit(ctx: &Ctx) -> Script {
    vol_time_run(ctx, VolTimeStep::OnInit, Vec::new()).map(|_| ())
}

pub fn vol_time_onenable(ctx: &Ctx) -> Script {
    vol_time_run(ctx, VolTimeStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn vol_time_ontimer10000(ctx: &Ctx) -> Script {
    vol_time_run(ctx, VolTimeStep::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn vol_time_ontimer15000(ctx: &Ctx) -> Script {
    vol_time_run(ctx, VolTimeStep::OnTimer15000, Vec::new()).map(|_| ())
}

pub fn vol_time_ontimer20000(ctx: &Ctx) -> Script {
    vol_time_run(ctx, VolTimeStep::OnTimer20000, Vec::new()).map(|_| ())
}

pub fn vol_time_ontimer30000(ctx: &Ctx) -> Script {
    vol_time_run(ctx, VolTimeStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn vol_time_ontimer35000(ctx: &Ctx) -> Script {
    vol_time_run(ctx, VolTimeStep::OnTimer35000, Vec::new()).map(|_| ())
}

pub fn vol_time_ontimer40000(ctx: &Ctx) -> Script {
    vol_time_run(ctx, VolTimeStep::OnTimer40000, Vec::new()).map(|_| ())
}

pub fn vol_time_ontimer45000(ctx: &Ctx) -> Script {
    vol_time_run(ctx, VolTimeStep::OnTimer45000, Vec::new()).map(|_| ())
}

pub fn vol_time_ontimer50000(ctx: &Ctx) -> Script {
    vol_time_run(ctx, VolTimeStep::OnTimer50000, Vec::new()).map(|_| ())
}

pub fn vol_time_ontimer55000(ctx: &Ctx) -> Script {
    vol_time_run(ctx, VolTimeStep::OnTimer55000, Vec::new()).map(|_| ())
}

fn rachel_guard_vol2_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Guard Krodger",
        args![
            "What's with this vase?",
            "They always send me out",
            "here to clean up this mess!",
            "I mean, it happens so often,",
            "I don't think it's accidental.",
            "You think it's vandals?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn rachel_guard_vol2_1(ctx: &Ctx) -> Script {
    rachel_guard_vol2_1_body(ctx, Vec::new()).map(|_| ())
}

fn rachel_guard_vol2_1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Rachel Guard#vol2_1")])?;
    return Err(Stop::End);
}

pub fn rachel_guard_vol2_1_oninit(ctx: &Ctx) -> Script {
    rachel_guard_vol2_1_oninit_body(ctx, Vec::new()).map(|_| ())
}

pub fn vol_time2(ctx: &Ctx) -> Script {
    vol_time2_run(ctx, VolTime2Step::Start, Vec::new()).map(|_| ())
}

pub fn vol_time2_oninit(ctx: &Ctx) -> Script {
    vol_time2_run(ctx, VolTime2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn vol_time2_onenable(ctx: &Ctx) -> Script {
    vol_time2_run(ctx, VolTime2Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn vol_time2_ontimer30000(ctx: &Ctx) -> Script {
    vol_time2_run(ctx, VolTime2Step::OnTimer30000, Vec::new()).map(|_| ())
}

fn drawer_vol1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFThere are some neatly",
        "printed and organized",
        "documents inside",
        "these drawers.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn drawer_vol1(ctx: &Ctx) -> Script {
    drawer_vol1_body(ctx, Vec::new()).map(|_| ())
}

fn drawer_vol3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
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
    if ctx.var("aru_vol").get()? == 4 {
        ctx.lines(args![
            "^3355FFYou find a thick pile",
            "of reports submitted",
            "to the high priest",
            "inside this drawer.^000000"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Check the Reports:Cancel")])?) == 1 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Well, it might not to",
                    "the most moral thing,",
                    "but I get the feeling",
                    "that I should at least",
                    "check some of these out."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFYou started shuffling",
                "through the documents,",
                "glancing at a few that",
                "catch your interest.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Ooh...", "This might be", "what I'm looking for."],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFYou take the thick report",
                "labeled ''Veins Geological",
                "Research Institute'' on",
                "the cover, and then you",
                "close the drawer.^000000"
            ])?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2114), Val::from(2115)])?;
            ctx.var("aru_vol").set(Val::from(5))?;
            ctx.call(Function::GetItem, vec![Val::from(7342), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Forget it.", "I didn't get permission", "to look through these files."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_vol").get()? == 5 {
        if ctx.call(Function::CountItem, vec![Val::from(7342)])?.number()? < 1 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Oh! Here's another", "copy of that report", "I wanted! Pretty lucky~"],
            )?;
            ctx.call(Function::GetItem, vec![Val::from(7342), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FFYou find a thick pile",
            "of reports submitted",
            "to the high priest",
            "inside this drawer.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFYou find a thick pile",
        "of reports submitted",
        "to the high priest",
        "inside this drawer.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn drawer_vol3(ctx: &Ctx) -> Script {
    drawer_vol3_body(ctx, Vec::new()).map(|_| ())
}

fn goddess_statue_vol1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFIt's a statue of Freya,",
        "a goddess revered for her",
        "clemency and wisdom.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn goddess_statue_vol1(ctx: &Ctx) -> Script {
    goddess_statue_vol1_body(ctx, Vec::new()).map(|_| ())
}

fn ladder_vol1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_vol").get()? == 5 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Wait, I can use this",
                "ladder to sneak out of",
                "here! I snuck inside so",
                "I'd get caught if I just",
                "passed the guards..."
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Climb Ladder:Cancel")])?) == 1 {
            ctx.lines(args!["^3355FFYou climbed the", "ladder over the", "wall and snuck out.^000000"])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("ra_temin"), Val::from(74), Val::from(136)])?;
            return Err(Stop::End);
        }
        ctx.lines(args!["^3355FFYou decided not to climb", "up the ladder for now.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn ladder_vol1(ctx: &Ctx) -> Script {
    ladder_vol1_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum VolroomStep {
    Start,
    OnTouch,
}

fn volroom_run(ctx: &Ctx, mut step: VolroomStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            VolroomStep::Start => {
                step = VolroomStep::OnTouch;
                continue 'machine;
            }
            VolroomStep::OnTouch => {
                if ctx.var("aru_vol").get()? == 6 {
                    ctx.lines(args![
                        "^3355FFThis house looks like",
                        "it's been abandoned for",
                        "a while: the floor is thickly",
                        "covered with dust and many",
                        "pieces of discarded paper.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFOne particular piece",
                        "of paper catches your",
                        "attention. You pick it",
                        "up and give it a read.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Paper",
                        args![
                            "''^333333The regularly scheduled",
                            "geological survey had been",
                            "postponed for over a week.",
                            "Please submit your report",
                            "to us as soon as possible.^000000''"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Although the sender's",
                            "name isn't on this letter,",
                            "I can guess who wrote it. ",
                            "Speaking of which...",
                            "Where's the geologist?"
                        ],
                    )?;
                    ctx.var("aru_vol").set(Val::from(7))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("aru_vol").get()?.number()? < 6 {
                    ctx.lines(args![
                        "^3355FFThis house looks like",
                        "it's been abandoned for",
                        "a while: the floor is thickly",
                        "covered with dust and many",
                        "pieces of discarded paper.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn volroom(ctx: &Ctx) -> Script {
    volroom_run(ctx, VolroomStep::Start, Vec::new()).map(|_| ())
}

pub fn volroom_ontouch(ctx: &Ctx) -> Script {
    volroom_run(ctx, VolroomStep::OnTouch, Vec::new()).map(|_| ())
}

fn towner_vol_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Towner",
        args![
            "The small office on the",
            "2nd floor of this weapon",
            "shop is occupied by a",
            "geologist. At least, he's",
            "supposed to be one..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Towner",
        args![
            "The guy might be a quack:",
            "all he does is drink and",
            "flirt with skanky women",
            "all day. I thought scholars",
            "are supposed to read and study",
            "and discover things, you know?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn towner_vol(ctx: &Ctx) -> Script {
    towner_vol_body(ctx, Vec::new()).map(|_| ())
}

fn drunken_man_vol_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
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
    if ctx.var("aru_vol").get()?.number()? < 7 {
        ctx.lines_as("Drunken Man", args!["So... ^333333*Urp*^000000", "So then I said..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args![
                "''^3131FFHey, buddy! A man",
                "uses his back to talk,",
                "not his fists! You wanna",
                "piece of me? Bring it on!^000000''"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args![
                "Then he got all",
                "scared, and ran away!",
                "Hahahaha! Guess I look",
                "pretty tough, don't I?"
            ],
        )?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_THROB")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Drunken Lady#1")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_THROB")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Drunken Lady#2")])?,
            ],
        )?;
        ctx.lines_as("Ladies", args!["Oh, my God!", "You're so cool~!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args![
                "Well... Anyone would",
                "have done it. I was just",
                "being a gentleman.",
                "Hahahah, that's right!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_vol").get()? == 7 {
        ctx.lines_as("Drunken Man", args!["So... ^333333*Urp*^000000", "So then I said..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args![
                "''^3131FFHey, buddy! A man",
                "uses his back to talk,",
                "not his fists! You wanna",
                "piece of me? Bring it on!^000000''"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args![
                "Then he got all",
                "scared, and ran away!",
                "Hahahaha! Guess I look",
                "pretty tough, don't I?"
            ],
        )?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_THROB")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Drunken Lady#1")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_THROB")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Drunken Lady#2")])?,
            ],
        )?;
        ctx.lines_as("Ladies", args!["Oh, my God!", "You're so cool~!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args![
                "Well... Anyone would",
                "have done it. I was just",
                "being a gentleman.",
                "Hahahah, that's right!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Excuse me, but are", "you the executive director", "of the Veins Geological Team?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args![
                "Yeah, sure! Executive",
                "director, deputy director,",
                "director, researcher, CEO,",
                "no... No, wait, that last one",
                "doesn't sound right. Hah!",
                "I'm all of those~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args![
                "I'm the executive director...",
                "I'm the only one that works",
                "at the institute, really.",
                "Why, what do you want?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Uhh... There are some",
                "official notices for you",
                "at your office. I guess you",
                "need to get some surveys",
                "done? They sound like",
                "they're pretty important."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Maybe...", "Maybe even ^FF0000urgent^000000."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args![
                "Wha--? Hey, what day",
                "is it today? Damn it!",
                "Fine, fine, time to get",
                "to work. Just when I was",
                "really enjoying myself too!",
                "Argh, I never wanna be sober!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args![
                "But... Working is the",
                "only way for me to afford",
                "all this drinking... Such",
                "is life. Such is life."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ladies",
            args![
                "Where are you going?",
                "Can't you stay a bit",
                "longer and talk with",
                "us? Pleeeeeease?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Drunken Man", args!["Sorry, ladies,", "but duty calls.", "Hahahahahahha~"])?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_THROB")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Drunken Lady#1")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_THROB")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Drunken Lady#2")])?,
            ],
        )?;
        ctx.lines_as("Ladies", args!["Please don't go~"])?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args![
                "Ahem!",
                "Let's see now.",
                "What'd be best...?",
                "..............................",
                "..............................",
                ".............................."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args![
                "..............................",
                "..............................",
                "........................Right!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Drunken Man", args!["Hey, you."])?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Yes?"])?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
        ctx.lines_as(
            "Drunken Man",
            args![
                "I hereby promote you as",
                "chief researcher of the",
                "Veins Geological Research",
                "Institute. Congratulations!",
                "Welcome to the team, friend!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Huh...?", "I don't understand", "what you're talking about!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args![
                "Heh! You should be grateful",
                "that I'm accepting you as my",
                "student! Everyone'd be proud",
                "to study under me, Gio, the",
                "world's greatest geologist!",
                "(Well, maybe.)"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Hey, I never--"])?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args![
                "Ah-ah! Now that you're",
                "my student, I expect you",
                "to work hard if you're going",
                "to learn anything. First thing",
                "first--go to my office and",
                "clear up my belated business."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args![
                "Here, take this ^FF0000reference",
                "guide^000000 with you to my office.",
                "When you check my desk, you'll",
                "find a ^FF0000pyrometer^000000 and a ^FF0000report",
                "form^000000. You'll need to bring all",
                "that stuff to Thor Volcano."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args![
                "When you get to Thor Volcano,",
                "use the pryometer to check the",
                "volcano's temperature, and",
                "fill out the report form."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args![
                "Take the filled report",
                "form to the geology camp",
                "that's deep inside the volcano",
                "so that they can stamp their",
                "confirmation on it. That's",
                "not so hard now, is it?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args![
                "Ahh, I've also decided",
                "to take these lovely ladies",
                "on as my students as well~",
                "I should stay here and",
                "entreat them to a lecture."
            ],
        )?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_THROB")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Drunken Lady#1")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_THROB")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Drunken Lady#2")])?,
            ],
        )?;
        ctx.lines_as("Ladies", args!["Oh~! You'll really", "teach us geology?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args![
                "Oh, right! When they",
                "ask you about the volcano's",
                "temperature when you submit",
                "the report at the geological",
                "camp, make something up.",
                "Make sure it sounds bad!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args![
                "Words like ''explosion,''",
                "''disaster,'' and ''collatoral",
                "damage'' would be perfect.",
                "Just do your part, and I'll",
                "take care of the rest. Okay~",
                "Come back soon, my pupil!"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFWell, this isn't what",
            "you expected, but you get",
            "the feeling that this will",
            "all turn out in your favor.",
            "You know that feeling, right?^000000"
        ])?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2116), Val::from(2117)])?;
        ctx.var("aru_vol").set(Val::from(8))?;
        ctx.call(Function::GetItem, vec![Val::from(7705), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("aru_vol").get()?.number()? > 7 && ctx.var("aru_vol").get()?.number()? < 24) {
        ctx.lines_as(
            "Geologist Gio",
            args![
                "Hey, you'd better hurry",
                "it up. I mean, you're the",
                "one that found that notice",
                "in my office, didn't you?",
                "You know how important",
                "this work is to us!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args![
                "Get the pyrometer,",
                "and go to Thor Volcano",
                "to fill out the report",
                "form and submit it to",
                "the geological camp!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_vol").get()? == 24 {
        ctx.lines_as(
            "Geologist Gio",
            args!["Well, those are", "nice legs, but they're", "not the best pair I've see--"],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I'm back."])?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args![
                "*Ahem* And that's how",
                "erosion... Works. Tomorrow,",
                "I'll teach you ladies all",
                "about rocks. All of them."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args!["Welcome back! So,", "how was the volcanic", "temperature report?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Well, I did what you",
                "told me. Hey, are you",
                "sure you wanted me to",
                "exaggerate the temperature?",
                "What about the camp?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args!["Oh, don't worry about", "the temperature. It's", "supposed to go up."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "What are you...?",
                "Actually, I thought that",
                "maybe the pryometer",
                "might be broken."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Geologist Gio", args!["Heh! You're right~", "I broke it on purpose."])?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["What?!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args![
                "You know, it's hard for",
                "scholars like me to make",
                "a decent living. Hell, I was",
                "lucky enough to get that",
                "temperature measuring job",
                "from the geological camp."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args![
                "Those guys've been trying",
                "to fire me ever since they",
                "realized the volcano became",
                "dormant. But... They can't",
                "fire me if there's proof that",
                "it might go off anytime!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args![
                "Then, when they're all",
                "panicked, I calmly and",
                "suavely offer a solution",
                "that looks like it works.",
                "Of course, there's never",
                "a problem to begin with..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["So...", "You're a con man."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args![
                "Awww, don't look at",
                "me like that. I'm a real",
                "scientist. Come on...",
                "Oh, come on..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args![
                "Look, why don't you head",
                "back to my institute and",
                "check out my bookshelf?",
                "I keep a small box there",
                "where I keep all sorts",
                "of nifty little goodies."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args![
                "You can have the very",
                "first thing that pops out",
                "of that box. I wonder if the",
                "goddess will grace you",
                "with good fortune. You",
                "might get something good~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Geologist Gio",
            args![
                "Consider it your",
                "payment for a job",
                "well done. Good work!",
                "I expected nothing less",
                "from my star pupil!"
            ],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(60211), Val::from(60212)])?;
        ctx.var("aru_vol").set(Val::from(25))?;
        ctx.call(Function::DelItem, vec![Val::from(7342), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(7704), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(7705), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Drunken Man", args!["So... ^333333*Urp*^000000", "So then I said..."])?;
    ctx.next()?;
    ctx.lines_as(
        "Drunken Man",
        args![
            "''^3131FFHey, buddy! A man",
            "uses his back to talk,",
            "not his fists! You wanna",
            "piece of me? Bring it on!^000000''"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Drunken Man",
        args![
            "Then he got all",
            "scared, and ran away!",
            "Hahahaha! Guess I look",
            "pretty tough, don't I?"
        ],
    )?;
    ctx.next()?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_THROB")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Drunken Lady#1")])?,
        ],
    )?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_THROB")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Drunken Lady#2")])?,
        ],
    )?;
    ctx.lines_as("Ladies", args!["Oh, my God!", "You're so cool~!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Drunken Man",
        args![
            "Well... Anyone would",
            "have done it. I was just",
            "being a gentleman.",
            "Hahahah, that's right!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn drunken_man_vol(ctx: &Ctx) -> Script {
    drunken_man_vol_body(ctx, Vec::new()).map(|_| ())
}

fn drunken_lady_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Drunken Lady",
        args![
            "This guys' actually",
            "pretty boring, but...",
            "I get free drinks if",
            "I can put up with him~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn drunken_lady_1(ctx: &Ctx) -> Script {
    drunken_lady_1_body(ctx, Vec::new()).map(|_| ())
}

fn drunken_lady_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Drunken Lady",
        args![
            "This tavern might look",
            "luxurious and gorgeous,",
            "but the drinks here stink!",
            "I can mix better drinks",
            "at home, no sweat at all~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn drunken_lady_2(ctx: &Ctx) -> Script {
    drunken_lady_2_body(ctx, Vec::new()).map(|_| ())
}

fn wall_closet_vol_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
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
    if ctx.var("aru_vol").get()? == 8 {
        if ctx.call(Function::CountItem, vec![Val::from(7704)])? == 0 {
            ctx.lines(args!["^3355FFYou found the", "pyrometer inside", "the closet.^000000"])?;
            ctx.call(Function::GetItem, vec![Val::from(7704), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args!["^3355FFThere's so much junk", "crammed in here!^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args!["^3355FFThere's so much junk", "crammed in here!^000000"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn wall_closet_vol(ctx: &Ctx) -> Script {
    wall_closet_vol_body(ctx, Vec::new()).map(|_| ())
}

fn bookshelf_vol_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_box_box = Val::from(0);
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
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
    if ctx.var("aru_vol").get()? == 8 {
        if ctx.call(Function::CountItem, vec![Val::from(7342)])? == 0 {
            ctx.lines(args![
                "^3355FFYou find a bundle",
                "of reports carelessly",
                "stuck between some",
                "books on this bookshelf.^000000"
            ])?;
            ctx.call(Function::GetItem, vec![Val::from(7342), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FFThere's a lot of",
            "scattered books and",
            "notebooks lying on",
            "this bookshelf.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("aru_vol").get()?.number()? > 8 && ctx.var("aru_vol").get()?.number()? < 25) {
        ctx.lines(args![
            "^3355FFThere's a lot of",
            "scattered books and",
            "notebooks lying on",
            "this bookshelf.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_vol").get()? == 25 {
        ctx.lines(args![
            "^3355FFThere's a lot of",
            "scattered books and",
            "notebooks lying on",
            "this bookshelf.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFAfter a quick look,",
            "you notice the grayish",
            "purple box that Gio was",
            "talking about it. You close",
            "your eyes, and reach inside",
            "Gio's purple box of goodies.^000000"
        ])?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(60212), Val::from(60213)])?;
        ctx.var("aru_vol").set(Val::from(26))?;
        l_box_box = ctx.call(Function::Rand, vec![Val::from(1), Val::from(20)])?;
        if l_box_box.clone().number()? < 7 {
            ctx.call(Function::GetItem, vec![Val::from(12104), Val::from(1)])?;
        } else if (l_box_box.clone().number()? > 6 && l_box_box.clone().number()? < 9) {
            ctx.call(Function::GetItem, vec![Val::from(661), Val::from(1)])?;
        } else if (l_box_box.clone().number()? > 8 && l_box_box.clone().number()? < 20) {
            ctx.call(Function::GetItem, vec![Val::from(12027), Val::from(5)])?;
        } else {
            ctx.call(Function::GetItem, vec![Val::from(12103), Val::from(1)])?;
        }
        ctx.call(Function::GetExperience, vec![Val::from(800000), Val::from(0)])?;
        ctx.lines(args![
            "^3355FFWell, you've done all",
            "that you could here.",
            "Now would be a good time",
            "to return to High Priest Zhed.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFThere's a lot of",
        "scattered books and",
        "notebooks lying on",
        "this bookshelf.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bookshelf_vol(ctx: &Ctx) -> Script {
    bookshelf_vol_body(ctx, Vec::new()).map(|_| ())
}
