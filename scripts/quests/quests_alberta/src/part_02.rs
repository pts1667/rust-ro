use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn a_pile_of_turtle_crystal_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("turtle").get()? == 7 {
        ctx.lines(args![
            "^3355FFAmong the Turtle Crystals,^000000",
            "^3355FFyou find a strange key hole.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args!["^3355FFYou use the", "Turtle crystal key", "in the key hole.^000000"])?;
        ctx.next()?;
        ctx.mes("^3355FF*Click! Click!*^000000")?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFBetween the crystals,000000",
            "^3355FFa small crystal plate^000000",
            "^3355FFin which a message^000000",
            "^3355FFin engraved.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Crystal Plate",
            args![
                "^3355AAI, the swordsman,^000000",
                "^3355AA'One,' has succeeded.^000000",
                "^3355AAI've cut the turtle's shell.^000000",
                "^3355AANow I shall try to^000000",
                "^3355AAcut turtle crystal.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Crystal Plate",
            args![
                "^3355AAThe surface of turtle^000000",
                "^3355AAcrystal is so rough and^000000",
                "^3355AAirregular. I'll need^000000",
                "^3355AAto sharpen my skills.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Crystal Plate",
            args!["^3355AAIt's going to be tough,^000000", "^3355AAbut I will succeed...^000000"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Crystal plate",
            args![
                "^3355AAGo 5 steps east,^000000",
                "^3355AA30 steps south^000000",
                "^3355AAand 5 steps around^000000",
                "^3355AAthe turtle pillar.^000000",
                "^3355AAThere you will find^000000",
                "^3355AAthe end of sword art.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFAt the edge of the crystal^000000",
            "^3355FFplate, is a long key.^000000"
        ])?;
        ctx.var("turtle").set(Val::from(8))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(11033), Val::from(11034)])?;
        ctx.lines(args!["^3355FFYou've gained the^000000", "^3355FFTurtle Pillar key.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFThere are so many Turtle^000000",
        "^3355FFCrystals that can only be^000000",
        "^3355FFfound on Turtle island.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_pile_of_turtle_crystal(ctx: &Ctx) -> Script {
    a_pile_of_turtle_crystal_body(ctx, Vec::new()).map(|_| ())
}

fn turtle_pillar_tur_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("turtle").get()? == 8 {
        ctx.lines(args![
            "^3355FFAt the base of the pillar^000000",
            "^3355FFis a tiny key hole with^000000",
            "^3355FFa turtle shaped mark.^000000",
            "^3355FFYou used the",
            "Turtle Pillar key",
            "in that key hole.^000000"
        ])?;
        ctx.next()?;
        ctx.mes("^3355FF*Click! Crack!*^000000")?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFAt the base of the pillar,^000000",
            "^3355FFa little door appears, and^000000",
            "^3355FFfrom that door an engraved^000000",
            "^3355FFstone bead rolls out.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Turtle Stone Bead",
            args!["This turtle pillar is made of the strongest material on this island."],
        )?;
        ctx.next()?;
        ctx.lines_as("Turtle Stone Bead", args!["Cutting it is impossible for even someone with legendary skills. Even if I managed to cut it, all of Turtle Island would collapse upon me."])?;
        ctx.next()?;
        ctx.lines_as("Turtle Stone Bead", args!["I've developed my mental skills, and trained using materials on this island. Sharpening my mind, I now know that I can cut this pillar in two."])?;
        ctx.next()?;
        ctx.lines_as(
            "Turtle Stone Bead",
            args!["Can this be the pinnacle of swordsmanship? Of the sword arts?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Turtle Stone Bead",
            args!["For this final accomplishment, I leave behind an offering."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Turtle Stone Bead",
            args![
                "I intend to offer my gratitude to the blood of the monsters I have killed, and the materials I have used in my training."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args!["^3355FFThere are many items^000000", "^3355FFunder the pillar...^000000"])?;
        ctx.var("misc_quest")
            .set(runtime::op(&ctx.var("misc_quest").get()?, "|", &Val::from(65536))?)?;
        ctx.var("turtle").set(Val::from(0))?;
        ctx.call(Function::CompleteQuest, vec![Val::from(11034)])?;
        let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?;
        if subject1 == 1 {
            ctx.call(Function::GetItem, vec![Val::from(702), Val::from(1)])?;
            ctx.lines(args!["^3355FFYou've gained ^000000", "^3355FFAnimal Gore.^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 2 {
            ctx.call(Function::GetItem, vec![Val::from(716), Val::from(1)])?;
            ctx.lines(args!["^3355FFYou've gained^000000", "^3355FFa Red Gemstone.^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 3 {
            ctx.call(Function::GetItem, vec![Val::from(734), Val::from(1)])?;
            ctx.lines(args!["^3355FFYou've gained^000000", "^3355FFa Red Frame^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 4 {
            ctx.call(Function::GetItem, vec![Val::from(10019), Val::from(1)])?;
            ctx.lines(args!["^3355FFYou've gained^000000", "^3355FFRed Scarf.^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 5 {
            ctx.call(Function::GetItem, vec![Val::from(725), Val::from(1)])?;
            ctx.lines(args!["^3355FFYou've gained^000000", "^3355FFa Sardonyx.^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 6 {
            ctx.call(Function::GetItem, vec![Val::from(716), Val::from(1)])?;
            ctx.lines(args!["^3355FFYou've gained^000000", "^3355FFa Red Gemstone.^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 7 {
            ctx.call(Function::GetItem, vec![Val::from(716), Val::from(1)])?;
            ctx.lines(args!["^3355FFYou've gained^000000", "^3355FFa Red Gemstone.^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 8 {
            ctx.call(Function::GetItem, vec![Val::from(716), Val::from(1)])?;
            ctx.lines(args!["^3355FFYou've gained^000000", "^3355FFa Red Gemstone.^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 9 {
            ctx.call(Function::GetItem, vec![Val::from(716), Val::from(1)])?;
            ctx.lines(args!["^3355FFYou've gained^000000", "^3355FFa Red Gemstone.^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 10 {
            ctx.call(Function::GetItem, vec![Val::from(725), Val::from(1)])?;
            ctx.lines(args!["^3355FFYou've gained^000000", "^3355FFa Sardonyx.^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines(args![
        "^3355FFThis Turtle Pillar^000000",
        "^3355FFis just one of^000000",
        "^3355FFmany pillars.^000000"
    ])?;
    ctx.next()?;
    ctx.mes("^3355FFOr is it...?^000000")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn turtle_pillar_tur(ctx: &Ctx) -> Script {
    turtle_pillar_tur_body(ctx, Vec::new()).map(|_| ())
}

fn turtle_stone_tur_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("turtle").get()? == 4 {
        ctx.lines(args![
            "^3355FFOn top of the stone",
            "is a small key hole",
            "with the turtle mark.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args!["^3355FFYou used the", "Security Key", "in that hole.^000000"])?;
        ctx.next()?;
        ctx.mes("^3355FF*Tat tat tat tat*^000000")?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThe back of the stone",
            "split into two parts",
            "revealing some hidden",
            "words engraved inside.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Turtle Stone",
            args!["The one who dreams to be a star of serenity must first hold the comet and three rays of light as one."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Turtle Stone",
            args!["You already possess the comet, so I grant you the first light..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Turtle Stone",
            args!["- Light of the second stage -", "^FF3355B2 , X : 75 , Y : 249^000000"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFAt the bottom of the stone",
            "beneath the line of words,",
            "rests a brilliant red bead.^000000"
        ])?;
        ctx.next()?;
        ctx.var("turtle").set(Val::from(10))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(11032), Val::from(11035)])?;
        ctx.lines(args![
            "^3355FFYou set the red bead to the",
            "Security key, fitting it",
            "within one of three tiny holes.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFIt is just a",
        "normal turtle",
        "stone. But",
        "on the top",
        "is written",
        "the words,",
        "'1st stage.'^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn turtle_stone_tur(ctx: &Ctx) -> Script {
    turtle_stone_tur_body(ctx, Vec::new()).map(|_| ())
}

fn turtle_stone_tur2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("turtle").get()? == 10 {
        ctx.lines(args![
            "^3355FFOn top of the stone",
            "is a small keyhole",
            "with the turtle mark.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args!["^3355FFYou used the", "Security Key", "in the keyhole.^000000"])?;
        ctx.next()?;
        ctx.mes("^3355FF*Click! Click!*^000000")?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThe back of the stone",
            "split into two parts",
            "revealing some hidden",
            "words along the inside.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Turtle Stone",
            args!["The one who dreams to be a star of serenity must first hold the comet and three rays of light as one."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Turtle Stone",
            args!["You already possess the comet and the first light, so I now grant you the second light."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Turtle stone",
            args!["- Light of the third stage -", "^FF3355B3 . X : 118 . Y : 233^000000"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "At the bottom of the stone",
            "beneath the line of words,",
            "rests a brilliant yellow bead."
        ])?;
        ctx.next()?;
        ctx.var("turtle").set(Val::from(11))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(11035), Val::from(11036)])?;
        ctx.lines(args![
            "^3355FFYou set the yellow bead",
            "into the Security key, fitting it",
            "within one of three tiny holes.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFIt is just a",
        "normal turtle",
        "stone. But",
        "on the top,",
        "the words",
        "'2nd stage'",
        "are written.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn turtle_stone_tur2(ctx: &Ctx) -> Script {
    turtle_stone_tur2_body(ctx, Vec::new()).map(|_| ())
}

fn turtle_stone_tur3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("turtle").get()? == 11 {
        ctx.lines(args![
            "^3355FFOn top of the stone",
            "is a small key hole",
            "with the turtle mark.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args!["^3355FFYou used the", "Security Key", "in the keyhole.^000000"])?;
        ctx.next()?;
        ctx.mes("^3355FF*Click! Click!*^000000")?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThe back of the stone",
            "split into two parts",
            "revealing some hidden",
            "words along the inside.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Turtle Stone",
            args!["The one who dreams to be a star of serenity must first hold the comet and three rays of light as one."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Turtle Stone",
            args!["You already possess the comet, and the first two lights. The final light is now yours."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Turtle Stone",
            args!["- Star of Serenity -", "^FF3355B4 . X : 113 . Y : 178^000000"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFAt the bottom of the stone",
            "beneath the line of words,",
            "rests a brilliant blue bead.^000000"
        ])?;
        ctx.next()?;
        ctx.var("turtle").set(Val::from(12))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(11036), Val::from(11037)])?;
        ctx.lines(args![
            "^3355FFYou set the blue bead to the",
            "into the Security Key, fitting it",
            "within one of three tiny holes.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFIt is just a",
        "normal turtle",
        "stone. But",
        "on the top,",
        "the words,",
        "'3rd stage,'",
        "are written.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn turtle_stone_tur3(ctx: &Ctx) -> Script {
    turtle_stone_tur3_body(ctx, Vec::new()).map(|_| ())
}

fn turtle_statue_tur_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("turtle").get()? == 12 {
        ctx.lines(args![
            "^3355FFOn top of the statue",
            "is a small key hole",
            "with the turtle mark.^000000",
            "^3355FFYou used the",
            "Security Key",
            "in the keyhole.^000000"
        ])?;
        ctx.next()?;
        ctx.mes("^3355FF*Click! Click!*^000000")?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThe statue split into",
            "two parts revealing a",
            "metal plate underneath.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Metal Plate",
            args![
                "To the one who dreams to be a star of serenity among the people: You know hold the comet and three rays of light as one."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou find a small lever",
            "at the bottom of the",
            "metal plate. You pull",
            "it open like a drawer.^000000"
        ])?;
        ctx.next()?;
        ctx.var("misc_quest")
            .set(runtime::op(&ctx.var("misc_quest").get()?, "|", &Val::from(65536))?)?;
        ctx.var("turtle").set(Val::from(0))?;
        ctx.call(Function::CompleteQuest, vec![Val::from(11037)])?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(20)])? == 7 {
            let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
            if subject1 == 1 {
                ctx.call(Function::GetItem, vec![Val::from(644), Val::from(1)])?;
                ctx.lines(args![
                    "^3355FFInside the drawer,",
                    "is a Gift Box. This",
                    "Gift Box is now yours.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject1 == 2 {
                ctx.call(Function::GetItem, vec![Val::from(616), Val::from(1)])?;
                ctx.lines(args![
                    "^3355FFInside the drawer,",
                    "is an Old Card Album.",
                    "It is now yours to keep.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject1 == 3 {
                ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
                ctx.lines(args![
                    "^3355FFInside the drawer,",
                    "is an Old Purple Box.",
                    "It is now yours to keep.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject1 == 4 {
                ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
                ctx.lines(args![
                    "^3355FFInside the drawer,",
                    "is an Old Purple Box.",
                    "It is now yours to keep.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        ctx.call(Function::GetItem, vec![Val::from(604), Val::from(1)])?;
        ctx.lines(args![
            "^3355FFInside the drawer,",
            "is a Dead Branch.",
            "It is now yours to keep.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFIt is a turtle statue",
        "made of ordinary turtle",
        "stone. On top of it is",
        "the word 'Security.'^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn turtle_statue_tur(ctx: &Ctx) -> Script {
    turtle_statue_tur_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum KnightLeaderTurStep {
    Start,
    OnTouch,
}

fn knight_leader_tur_run(ctx: &Ctx, mut step: KnightLeaderTurStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            KnightLeaderTurStep::Start => {
                ctx.lines_as("Takuyaka", args!["Where did all my men go?!", "This is horrible~!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Takuyaka",
                    args!["Oh...", "*Whew!*", "Finally, another person.", "Tell me who you are!"],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[
                        ((Val::from("Who are you to ask?:") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                            + Val::from("!:What are you doing here?:First, tell me who you are.")),
                    ],
                )? {
                    1 => {
                        ctx.lines_as("Takuyaka", args!["How dare you speak to me in that tone! Don't you know who I am?! I'm Takuyaka, leader of the Security Knights of Alberta! Kobolds and Porings quiver in fear when they hear my name..."])?;
                        ctx.next()?;
                        ctx.lines_as("Takuyaka", args!["So tell me the truth! You came here for treasure, didn't you?! You want it all for yourself! You bastard! Now get off this island! Go, shoo!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Takuyaka",
                            args![
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
                                "Well, your name sounds better than mine. And you have more hair on your head as well. Hmmm..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Takuyaka", args!["Do you have anything to give me? You know, some food or zeny? It's not like I really need it, it's just... Oh bother. Never mind. Perhaps we'll meet later."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.lines_as("Takuyaka", args!["I've heard that many treasures can be found on Turtle Island. If I can take enough home, I'll be rich! Well, we're supposed to rescue lost adventurers too, but whatever. As leader, I can do whatever I want!"])?;
                        ctx.next()?;
                        ctx.lines_as("Takuyaka", args!["However, we're totally lost. Meaning we can't find any treasure or even rescue any people. Grrr...! Where are my useless soldiers! I've probably spoiled them with too much food!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Takuyaka",
                            args!["If you see any of them, tell them they must assemble here! Alright?!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    4 => {
                        ctx.lines_as("Takuyaka", args!["Me? I'm Takuyaka. A tall, dark, handsome, and dashing leader, as well as a fearsome knight! I have come to save the foolish people of Alberta that have come seeking treasure on Turtle Island."])?;
                        ctx.next()?;
                        ctx.lines_as("Mudasamu", args!["Really, now?"])?;
                        ctx.next()?;
                        ctx.lines_as("Takuyaka", args!["Quiet!"])?;
                        ctx.next()?;
                        ctx.lines_as("Takuyaka", args!["Anyway, we came here with the purest of intents. But right when we arrived, we were attacked by monsters! Because of the confusion from all the fighting, my soldiers were scattered."])?;
                        ctx.next()?;
                        ctx.lines_as("Takuyaka", args!["Mudasamu!", "Why is this", "happening to me!!!!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mudasamu",
                            args!["Huh...!", "Why should you", "be surprised,", "you chubby dwarf?"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                step = KnightLeaderTurStep::OnTouch;
                continue 'machine;
            }
            KnightLeaderTurStep::OnTouch => {
                ctx.lines_as("Scared Voice", args!["Wh-wh-whaat?!"])?;
                ctx.next()?;
                ctx.lines_as("Terrified Voice", args!["What's that noise?!", "M-M-Mudasamu!", "Protect me!"])?;
                ctx.next()?;
                ctx.lines_as("Mudasamu", args!["No.", "And shut up."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn knight_leader_tur(ctx: &Ctx) -> Script {
    knight_leader_tur_run(ctx, KnightLeaderTurStep::Start, Vec::new()).map(|_| ())
}

pub fn knight_leader_tur_ontouch(ctx: &Ctx) -> Script {
    knight_leader_tur_run(ctx, KnightLeaderTurStep::OnTouch, Vec::new()).map(|_| ())
}

fn mudasamu_tur_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Mudasamu",
        args!["My name is Mudasamu...", "Second in command of the", "Alberta Security Knights."],
    )?;
    ctx.next()?;
    ctx.lines_as("Mudasamu", args!["We've come here to rescue treasure hunters that might be in trouble. However, our leader is this porky specimen right next to me. So I guess I foresaw that this mission would fail."])?;
    ctx.next()?;
    ctx.lines_as("Mudasamu", args!["Basically, our leader was so distracted by the idea of finding treasure himself that we've lost our men. It's bad enough that he's a fat slob, but he's a greedy slob too!"])?;
    ctx.next()?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
    ctx.lines_as("Takuyaka", args!["Hey, Mudasamu!", "Are you talking", "about me?!"])?;
    ctx.next()?;
    ctx.lines_as("Mudasamu", args!["No, fat one,", "we're not", "talking about you."])?;
    ctx.next()?;
    ctx.lines_as(
        "Mudasamu",
        args!["*Sigh...*", "I better be paid", "overtime for", "putting up with this."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mudasamu_tur(ctx: &Ctx) -> Script {
    mudasamu_tur_body(ctx, Vec::new()).map(|_| ())
}

fn knight_tur_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "?",
        args![
            "Where are they?",
            "Oh hey! You're an adventurer, aren't you? Do you know where my comrades are?"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(
        ctx,
        &[Val::from(
            "Who are you?:Sorry, I don't.:Why did you come here?:How did you get here?",
        )],
    )? {
        1 => {
            ctx.lines_as("Passats", args!["My name is Passats, a proud Security Knight of Alberta. We've obtained some information that some people came here to seek treasure."])?;
            ctx.next()?;
            ctx.lines_as("Passats", args!["Our task here is to save people, but when we arrived on Turtle Island, we were attacked by a mob of monsters! So we all got scattered."])?;
            ctx.next()?;
            ctx.lines_as(
                "Passats",
                args!["Now I'm all alone. If you find anyone else in my unit, would you let them know?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Passats",
                args![
                    "Ah well...",
                    "Well, if you happen to find anyone else in my unit, would you let them know where I am?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Passat",
                args!["Oh, and if you find our leader, kick his butt, will you? It's so fat, there's no way you can miss."],
            )?;
            ctx.next()?;
            if ctx.var("BaseLevel").get()?.number()? <= 10 {
                ctx.call(Function::PercentHeal, vec![Val::from(40), Val::from(0)])?;
            }
            ctx.lines_as("Passats", args!["Oh...", "And, um...", "Cheer up!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            ctx.lines_as("Passats", args!["There are a lot of stories about the amazing wealth that can be found on this island. However, everyone assumed that it was impossible to find Turtle Island."])?;
            ctx.next()?;
            ctx.lines_as("Passats", args!["But then, the problem began when some navigator actually knew the way to Turtle Island. I think he'll take just about anybody for 10,000 zeny!"])?;
            ctx.next()?;
            ctx.lines_as("Passats", args!["So anyway, some of the people of Alberta have been foolishly coming here, hoping to find treasure. But of course, they can't fight monsters."])?;
            ctx.next()?;
            ctx.lines_as("Passats", args!["So now we're here in order to seek out towners in trouble, and send them safely back home. Of course, now it seems that we also need rescuing..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        4 => {
            ctx.lines_as("Passats", args!["Hmmm..."])?;
            ctx.next()?;
            ctx.lines_as("Passats", args!["No clue."])?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("How can that be?!")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as("Passats", args!["Oh, don't be angry. I just drank too much vodka! Before we arrived I was still unconscious and when I finally sobered up, I was here. I'm so embarrassed~"])?;
            ctx.next()?;
            ctx.lines_as("Passats", args!["I...", "I'm a victim too~"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn knight_tur(ctx: &Ctx) -> Script {
    knight_tur_body(ctx, Vec::new()).map(|_| ())
}

fn knight_tur2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Jayprocat",
        args!["Oh? Are you a civilian? Nice to meet you. I'm a mighty Security Knight of Alberta, Jayprocat."],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(
        ctx,
        &[Val::from("What is that, exactly?:I'm not impressed.")],
    )?) == 1
    {
        ctx.lines_as(
            "Jayprocat",
            args![
                "Well...",
                "We're supposed to protect the people of Alberta. That's our job as proud Security Knights."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jayprocat",
            args![
                "So when a bunch of them caught treasure fever and started coming here, we came to rescue them. Anyway, that was the plan."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Jayprocat", args!["It turns out that by some joke by fate, our leader is this pudgy guy who attracted a mob on monsters to us by noisily digging up treasure, instead of doing what we were supposed to do."])?;
        ctx.next()?;
        ctx.lines_as(
            "Jayprocat",
            args![
                "*Sigh...*",
                "Why wasn't Mudasamu in charge? With him as leader, I would've been home for dinner..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Jayprocat", args!["Well...", "It's true that we pretty much messed up our current mission to save anyone here seeking for treasure, since we need rescuing now as well."])?;
    ctx.next()?;
    ctx.lines_as(
        "Jayprocat",
        args![
            "*Sniff*",
            "Why did we have to be attacked by so many monsters on my first mission? I'm... I'm just a new recruit..."
        ],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(
        ctx,
        &[Val::from("Why did you run away?:I don't care.")],
    )?) == 1
    {
        ctx.lines_as("Jayprocat", args!["No, I didn't just run away! The monsters were too stong and there were so many of them! Plus, it's our damn leader's fault."])?;
        ctx.next()?;
        ctx.lines_as(
            "Jayprocat",
            args![
                "The monsters are all highly evolved turtles. And this island even looks like a turtle! But I guess you knew that already."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jayprocat",
            args!["I had no idea that this island's monsters were so much stronger than the ones that live near Alberta."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jayprocat",
            args![
                "Their shells are so tough, and we can't seem to find any sort of weak point! I have no idea how to fight these turtles!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Jayprocat", args!["You're so mean...", "I mean, I'm really new at this!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn knight_tur2(ctx: &Ctx) -> Script {
    knight_tur2_body(ctx, Vec::new()).map(|_| ())
}

fn knight_tur3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Squall",
        args!["Am I the only one left? Where are all my comrades?! I... I'm about to go crazy!"],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(
        ctx,
        &[Val::from("Maybe they ran away?:What happened?")],
    )?) == 1
    {
        ctx.lines_as("Squall", args!["No, we would never run away from the face of danger."])?;
        ctx.next()?;
        ctx.lines_as("Squall", args!["Well, we sort of did, but that was because it was a direct order from our leader! If a knight is loyal and proud, he will accept his leader's orders without question!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Squall",
            args!["Even if that leader is a little greedy, with hair that might be thinning..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Squall", args!["^FF0000And has the body", "of a pregnant whale^000000."])?;
        ctx.next()?;
        ctx.lines_as(
            "Squall",
            args!["Oh wait!", "I shouldn't", "have said that!", "What I meant", "to say was..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Squall",
            args!["^CC0000And has the", "body of a whale", "pregnant with twins^000000."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Squall",
            args!["Grrrr...!", "I'm so angry!", "Why can't Mudasamu", "be our leader?!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Squall", args!["We had good plans for this mission, made by our second in command, Mudasamu. However, they were executely poorly by the first in command."])?;
    ctx.next()?;
    ctx.lines_as(
        "Squall",
        args!["Whenever Takuyaka leads a mission, he always manages to mess it up! Why do the leaders in Alberta like him anyway?!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Squall", args!["*Sniff*", "I sweat and I bleed for the Alberta Security Knights, and still I'm tortured by the fact that Sir Porky commands us. I hate it!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Squall",
        args!["Our weapon and armor budget is always gone after Takuyaka visits a buffet... Why do we have to be so miserable?!"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn knight_tur3(ctx: &Ctx) -> Script {
    knight_tur3_body(ctx, Vec::new()).map(|_| ())
}

fn knight_tur4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Nysurea", args!["Hm? I'm surprised that someone around here is still alive."])?;
    ctx.next()?;
    ctx.lines_as(
        "Nysurea",
        args!["I'm beginning to think all my comrades have been killed by the monsters here..."],
    )?;
    ctx.next()?;
    if ctx.var("BaseLevel").get()?.number()? <= 40 {
        ctx.lines_as("Nysurea", args!["Having a", "tough time?"])?;
        ctx.next()?;
        if ctx.var("Zeny").get()?.number()? <= 3000 {
            ctx.lines_as("Nysurea", args!["Even though you're pretty strong, sooner or later you might need a reprieve. I can send you back to Alberta right now if you want."])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Yes, Help me.:No, thanks.")])?) == 1 {
                ctx.lines_as(
                    "Nysurea",
                    args!["Okay then, let me activate this teleport thingee donated to us by Kafra Corporation..."],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("alberta"), Val::from(241), Val::from(115)])?;
                return Err(Stop::End);
            }
            ctx.lines_as("Nysurea", args!["Ah, that's the spirit! You should be careful though. And if you see any other of my comrades, let them know I'm here, would you?"])?;
            ctx.next()?;
            ctx.lines_as("Nysurea", args!["Well, good luck!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Nysurea",
            args!["You look lost! If you stick around here, well, the monsters will probably get the best of you sooner or later."],
        )?;
        ctx.next()?;
        ctx.lines_as("Nyusurea", args!["Anyway...", "Good luck!", "You'll need it!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Nysurea",
        args![
            "Huh...",
            "You look calm and collected. If all our soldiers were as strong as you, we probably would not have been scattered."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Nysurea",
        args![
            "I have dreams of becoming a great knight, but since our 'leader' is all plump and no brains, we're stuck in this predicament."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Nysurea", args!["Anyway, be careful in this third level of Turtle Island. A lot of our men were annihilated when they went to explore the East and North sides, so be careful!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn knight_tur4(ctx: &Ctx) -> Script {
    knight_tur4_body(ctx, Vec::new()).map(|_| ())
}

fn iromo_ep3_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_cooltime = Val::from(0);
    let mut l_goyang = Val::from(0);
    if ctx.var("ep13_2_hiki").get()? == 13 {
        ctx.lines_as(
            "Iromo",
            args![
                "But... this world is...",
                "What a big world... Not just humans...",
                "Other lifeforms just like us...",
                "Where could they be?..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Iromo",
            args!["...Their lives.. and...", "Their kingdom... and land...", "Where could they be..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Iromo",
            args![
                "If they exist, that'd be great...",
                "No... they must exist...",
                "I wish one day... one day I can go there..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_2_hiki").get()? == 12 {
        ctx.lines_as("Iromo", args!["If it's too painful...", "It's better if I don't..."])?;
        ctx.next()?;
        ctx.mes("-Silent pause-")?;
        ctx.next()?;
        ctx.lines(args![
            ((Val::from("-") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(" starts to speak-")),
            "-And tells him of the journeys-",
            "-Adventures, joy, sorrow, and loss-",
            "-One by one to the boy-"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "-Fighting monsters, and winning-",
            "-Coming across many people, and-",
            "-leaving them, and tells of many other things-",
            "-As much as possible, to the boy.-"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "-Much has happened, and-",
            "-There were many dangers, but in the end-",
            "-Happy things outweighs the bad,-",
            "-What I have received is more than what I have lost.-",
            "-I try hard to convey this message to Iromo.-"
        ])?;
        ctx.next()?;
        ctx.lines_as("Iromo", args!["..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Iromo",
            args!["...So in the end...", "You just want me... to go outside...", "Am I right..."],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("It's true, you'll get more than you pay.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Iromo", args!["..."])?;
        ctx.next()?;
        ctx.lines_as("Iromo", args!["...But now... I am not going outside.", "I am still afraid..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Iromo",
            args![
                "Even if I don't go out right now,",
                "I will still... make an effort to do so.",
                "One day, I will surely become...",
                "..An adventurer... and do many, many things."
            ],
        )?;
        ctx.next()?;
        ctx.var("ep13_2_hiki").set(Val::from(13))?;
        ctx.call(Function::CompleteQuest, vec![Val::from(10089)])?;
        ctx.call(Function::GetExperience, vec![Val::from(80000), Val::from(0)])?;
        ctx.lines_as(
            "Iromo",
            args![
                "The world is big, there are many things...",
                "I have not seen yet. Yes... I want to...",
                "...see them all."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_2_hiki").get()? == 11 && ctx.var("friendship").get()?.number()? > 14) {
        ctx.lines_as(
            "Iromo",
            args![
                "To go on an adventure with my friends,",
                "I would still rather stay here.",
                "I would still rather stay here quietly and safely."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Iromo",
            args!["Alright...go away. I know...", "I know what you want to say...what an annoyance..."],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "-You don't give up, and continue to-",
            "-tell of the stories-",
            "-shared with friends.-",
            "-You tell of the truth.-"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "-Someone met long ago,-",
            "-Suddenly leaves,-",
            "-and finally reunited, but-",
            "-because of the cruel fate-",
            "-he dies in this tragedy.-",
            "-In the end, it's all a tragedy...-"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "-But you emphasize -",
            "-Even though the ending is tragic,-",
            "-A beautiful friendship-",
            "-Will not disappear because of that.-"
        ])?;
        ctx.next()?;
        ctx.lines_as("Iromo", args!["..."])?;
        ctx.next()?;
        ctx.lines_as("Iromo", args!["...Thank you for telling me...", "They are great friends..."])?;
        ctx.next()?;
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
            ctx.lines_as(
                "Iromo",
                args![
                    "...And? ...What do you...",
                    "Want to say to me this time...",
                    "You've told me many stories...",
                    "...How about one of your own?"
                ],
            )?;
            ctx.next()?;
        } else {
            ctx.lines_as(
                "Iromo",
                args![
                    "...And? ...What do you...",
                    "Want to say to me this time...",
                    "You've told me many stories...",
                    "...How about one of your own?"
                ],
            )?;
            ctx.next()?;
        }
        let choice = runtime::select_values(ctx, &[Val::from("Friendship cannot be forgotten.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Iromo", args!["..."])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("To have that kind of friends and that kind of friendship...")])?;
        ctx.var("@menu").set(choice)?;
        let choice = runtime::select_values(ctx, &[Val::from("You need to get out of your house and see the world.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Iromo",
            args!["Though you changed a story... but...", "You're still talking about this..."],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("To be honest, the other kids miss you.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Iromo",
            args!["...Him? ...Miss me? That's...", "Something from a long time ago..."],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("There are many things you don't know yet.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Iromo", args!["..."])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Don't you think it's a waste to stay home?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Iromo", args!["..."])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("In order to find safety, you would give up fun adventures?")])?;
        ctx.var("@menu").set(choice)?;
        let choice = runtime::select_values(ctx, &[Val::from("That's too bad. Change is a very fun thing.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Iromo", args!["...Change...", "Is fun...?"])?;
        ctx.next()?;
        let choice = runtime::select_values(
            ctx,
            &[Val::from("Outside this city and this kingdom, there is a bigger world.")],
        )?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Iromo", args!["..."])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Since it's so big, there are many fun things.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Iromo",
            args!["Really...? Other than Alberta, and Rune-Midgarts...?", "There are other places...?"],
        )?;
        ctx.next()?;
        ctx.var("ep13_2_hiki").set(Val::from(12))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10088), Val::from(10089)])?;
        ctx.lines_as(
            "Iromo",
            args![
                "But... there will also be many troubles...",
                "Isn't that true... I can't relax...",
                "I... don't like those..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_2_hiki").get()? == 11 {
        ctx.lines_as(
            "Iromo",
            args![
                "To go on an adventure with my friends,",
                "I would still rather stay here.",
                "I would still rather stay here quietly and safely."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Iromo",
            args![
                "Alright... go away. I know...",
                "I know what you want to say... what an annoyance..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_2_hiki").get()? == 10 {
        l_cooltime = ctx.call(Function::CheckQuest, vec![Val::from(10087), ctx.constant("PLAYTIME")?])?;
        if l_cooltime.clone() == 2 {
            ctx.lines_as(
                "Iromo",
                args!["...You really are annoying...", "What exactly do you want from me...?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Iromo",
                args![
                    "A long time ago... I had a friend just like you.",
                    "I met him when I went out to play...",
                    "But...",
                    "He must've forgotten all about me..."
                ],
            )?;
            ctx.next()?;
            ctx.var("ep13_2_hiki").set(Val::from(11))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(10087), Val::from(10088)])?;
            ctx.call(Function::GetExperience, vec![Val::from(67000), Val::from(0)])?;
            ctx.lines_as(
                "Iromo",
                args![
                    "Nevermind... it's not unexpected...",
                    "To go on an adventure with my friends,",
                    "I would still rather stay here.",
                    "I would still rather stay here quietly and safely."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Iromo",
            args!["...What... Right now, I...", "I don't want to... hear anything...", "Go away..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_2_hiki").get()? == 9 && ctx.var("lhz_rekenber").get()?.number()? > 21) {
        ctx.lines_as("Iromo", args!["...What is it... this time...?"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("A story about a pair of siblings.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines(args![
            "-Tell him about Kazien and Lyozien.-",
            "-Because of ill fate,-",
            "-They were born to do illegal tasks-",
            "-But the innocent Lyozien-",
            "-Trusted his brother...-"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "-And even though Kazien has become-",
            "-a criminal, but as an older brother-",
            "-He hopes to take good care of his younger brother.-"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "-This is a story from far away,-",
            "-But because you have seen it with your own eyes,-",
            "-You can clearly express-",
            "-Your feelings.-"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "-This story must be able to-",
            "-Stir up the interest of-",
            "-Iromo, who has always wanted a brother.-"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Iromo",
            args![
                "I see...They were great siblings...",
                "Tragic...but also beautiful.",
                "If only...I can be the protagonist...",
                "..in such a story..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Iromo",
            args!["...But...I'm an only child...", "...So there's nothing I can do..."],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("You don't need to be related by blood to be brothers.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Iromo", args!["..."])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("You can find make of these brothers outside.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Iromo", args!["...Outside?..."])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("But if you stay at home, nothing will change.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Iromo", args!["..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Iromo",
            args![
                "...What... So it's all my fault...",
                "What you want to say...",
                "...is always the same...",
                "Get out... I'm not in the mood..."
            ],
        )?;
        ctx.next()?;
        ctx.var("ep13_2_hiki").set(Val::from(10))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10086), Val::from(10087)])?;
        ctx.call(Function::GetExperience, vec![Val::from(57000), Val::from(0)])?;
        ctx.lines_as("Iromo", args!["...I'm not in a good mood, get out..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_2_hiki").get()? == 8 || ctx.var("ep13_2_hiki").get()? == 9) {
        l_cooltime = ctx.call(Function::CheckQuest, vec![Val::from(10085), ctx.constant("PLAYTIME")?])?;
        if l_cooltime.clone() == 2 {
            ctx.lines_as("Iromo", args!["You never give up, don't you?", "What are you trying to say?"])?;
            ctx.next()?;
            ctx.lines_as("Iromo", args!["It is frustrating. But...", "Why do you care about me so much?"])?;
            ctx.next()?;
            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
                ctx.lines_as(
                    "Iromo",
                    args!["Really, its not that bad...", "I wish I could have a sister like you."],
                )?;
                ctx.next()?;
            } else {
                ctx.lines_as(
                    "Iromo",
                    args!["Really, its not that bad...", "I wish I could have a brother like you."],
                )?;
                ctx.next()?;
            }
            ctx.lines_as("Iromo", args!["Do you have any stories about siblings?"])?;
            ctx.next()?;
            ctx.var("ep13_2_hiki").set(Val::from(9))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(10085), Val::from(10086)])?;
            ctx.lines_as("Iromo", args!["What? Anything to say?", "Seriously!", "Just leave me alone."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Iromo", args!["Thank you for caring...", "But that is ok...No thanks..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_2_hiki").get()? == 7 {
        l_goyang = ctx.call(Function::CheckQuest, vec![Val::from(10084), ctx.constant("HUNTING")?])?;
        if l_goyang.clone() == 2 {
            ctx.lines_as(
                "Iromo",
                args![
                    "Oh, you made the furious cat",
                    "go away from the village?",
                    "...Oh, you did..Thanks!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Iromo",
                args!["But, I am still scared of", "being outside.", "I would rather stay at home."],
            )?;
            ctx.next()?;
            ctx.var("ep13_2_hiki").set(Val::from(8))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(10084), Val::from(10085)])?;
            ctx.call(Function::GetExperience, vec![Val::from(47000), Val::from(0)])?;
            ctx.lines_as("Iromo", args!["Thank you for being helpful.", "But, no thanks."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Iromo", args!["If I go out, I will see the scary cat."])?;
        ctx.next()?;
        ctx.lines_as(
            "Iromo",
            args!["He will bite me..and", "scratch me again.", "I should stay at home."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.var("ep13_2_hiki").get()? == 6 && ctx.call(Function::CountItem, vec![Val::from(528)])?.number()? > 0)
        && ctx.call(Function::CountItem, vec![Val::from(501)])?.number()? > 0)
    {
        ctx.lines_as(
            "Iromo",
            args!["...Huh? This smell...", "This is the one I like the most.", "Ha Ha!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Iromo", args!["But..my mom is beside me.", "(Wink, Wink)"])?;
        ctx.next()?;
        ctx.lines_as("Iromo", args!["...Thanks! I need to hide this.", "Anyway, thank you!"])?;
        ctx.next()?;
        ctx.lines_as("Iromo", args!["It smells so good."])?;
        ctx.next()?;
        ctx.lines_as("Iromo", args!["Huh?? If I go out,", "then can I get these things?"])?;
        ctx.next()?;
        ctx.lines_as("Iromo", args!["......"])?;
        ctx.next()?;
        ctx.lines_as("Iromo", args!["But... I'm scared..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Iromo",
            args!["I am scared. If I go out...", "No, I don't want to get hurt again."],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Again?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Iromo",
            args![
                "...there is a weird cat that can walk on two feet.",
                "I like cats so I approached him.",
                "Then suddenly he scratched and bit me."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(528), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(501), Val::from(1)])?;
        ctx.var("ep13_2_hiki").set(Val::from(7))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10083), Val::from(10084)])?;
        ctx.call(Function::GetExperience, vec![Val::from(37500), Val::from(0)])?;
        ctx.lines_as(
            "Iromo",
            args!["I don't want to see the cat again.", "If I go out, I will see him. It is scary."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.var("ep13_2_hiki").get()? == 3 || ctx.var("ep13_2_hiki").get()? == 4) || ctx.var("ep13_2_hiki").get()? == 5)
        || ctx.var("ep13_2_hiki").get()? == 5)
    {
        ctx.lines_as(
            "Iromo",
            args![
                "... I don't like being outside of the village.",
                "Being inside of the house is the best."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_2_hiki").get()? == 1 || ctx.var("ep13_2_hiki").get()? == 2) {
        ctx.lines_as(
            "Iromo",
            args![
                "Mother told me that",
                "I should go out and play",
                "with friends. But I don't",
                "want to go out with them."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Iromo",
            args![
                "It is little bit inconvenient that",
                "I can't get delicious food",
                "that grows outside of the village.",
                "I can stand it."
            ],
        )?;
        ctx.next()?;
        if ctx.var("ep13_2_hiki").get()? == 1 {
            ctx.var("ep13_2_hiki").set(Val::from(2))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(10079), Val::from(10080)])?;
        }
        ctx.lines_as(
            "Iromo",
            args![
                "But, I can have other food",
                "instead of the outside food.",
                "That way I don't have to go out."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Iromo", args!["......"])?;
    ctx.next()?;
    ctx.lines_as("Iromo", args!["I like my house and my room.", "I don't want to go out."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn iromo_ep3_2(ctx: &Ctx) -> Script {
    iromo_ep3_2_body(ctx, Vec::new()).map(|_| ())
}

fn iromo_s_mother_ep3_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_2_hiki").get()?.number()? >= 8 {
        ctx.lines_as(
            "Mother",
            args![
                "Thank you for helping my son.",
                "But I think we can't do anything",
                "about him anymore."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mother",
            args![
                "Iromo was really active and",
                "curious about the world.",
                "He wanted to see all the",
                "sights of the world."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mother",
            args![
                "But look at him now.",
                "He doesn't go out anymore.",
                "He is just stuck inside of his room."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mother",
            args![
                "He became a timid person.",
                "If he has an argument with me,",
                "he doesn't talk to me for a week."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mother",
            args!["You might have", "a hard time getting", "Iromo to talk to you."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_2_hiki").get()? == 7 {
        ctx.lines_as(
            "Mother",
            args![
                "Oh, I remember",
                "what happened last time...",
                "One day, Iromo came home",
                "with tears in his eyes",
                "And he seemed hurt."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mother",
            args![
                "It was not a big injury.",
                "I was worried about him so much.",
                "But, he never tells me what happened that day."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mother",
            args!["After that day, he hasn't talked to people much and doesn't go out at all."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mother",
            args!["I hope he tells me what happened that day.", "Iromo was such a good boy..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.var("ep13_2_hiki").get()? == 4 || ctx.var("ep13_2_hiki").get()? == 5) || ctx.var("ep13_2_hiki").get()? == 6) {
        ctx.lines_as(
            "Mother",
            args![
                "Hum.. Iromo's favorite?",
                "Let me think...",
                "Actually he likes all kinds of food.",
                "He is not picky about food."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mother",
            args![
                "When he went out with friends,",
                "he spent all his pocket money,",
                "but I don't know what he has been eating."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mother",
            args![
                "Oh, why don't you ask Iromo's friend?",
                "I think he would know about his favorite food."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_2_hiki").get()? == 2 || ctx.var("ep13_2_hiki").get()? == 3) {
        ctx.lines_as(
            "Mother",
            args![
                "Hum.. Iromo's favorite?",
                "Let me think...",
                "Actually he likes all kinds of food.",
                "He is not picky about food."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mother",
            args![
                "When he went out with friends,",
                "he spent all his pocket money,",
                "but I don't know what he has been eating."
            ],
        )?;
        ctx.next()?;
        ctx.var("ep13_2_hiki").set(Val::from(3))?;
        ctx.lines_as(
            "Mother",
            args![
                "Oh, why don't you ask Iromo's friend?",
                "I think he would know about his favorite food."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Mother", args!["This little boy is my son, Iromo."])?;
    ctx.next()?;
    ctx.lines_as(
        "Mother",
        args!["He used to go out all the time.", "He loved playing outside with friends."],
    )?;
    ctx.next()?;
    ctx.lines_as("Mother", args!["But, somehow...", "he doesn't go out anymore."])?;
    ctx.next()?;
    if (ctx.var("BaseLevel").get()?.number()? > 40 && ctx.call(Function::CheckQuest, vec![Val::from(10079)])? == -1) {
        ctx.var("ep13_2_hiki").set(Val::from(1))?;
        ctx.call(Function::SetQuest, vec![Val::from(10079)])?;
    }
    ctx.lines_as("Mother", args!["I am so worried about him...", "What happened to him..?"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn iromo_s_mother_ep3_2(ctx: &Ctx) -> Script {
    iromo_s_mother_ep3_2_body(ctx, Vec::new()).map(|_| ())
}

fn little_boy_ep3_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_2_hiki").get()? == 6 {
        ctx.lines_as(
            "Little Boy",
            args!["Iromo? He used to like ^FF0000Monster's Feed^000000 and ^FF0000Red Potion^000000."],
        )?;
        ctx.next()?;
        ctx.lines_as("Little Boy", args!["Oh ya and one more thing~!", "I like Bananas. Haha."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_2_hiki").get()? == 5 {
        ctx.lines_as("Little Boy", args!["......"])?;
        ctx.next()?;
        ctx.lines_as(
            "Little Boy",
            args!["Yummy! Banana is always delicious!", "Bananas are the best fruits ever!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Little Boy",
            args!["Thanks for the Banana...", "Now I can think about", "what Iromo likes to eat."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Little Boy",
            args![
                "He used to like junk food.",
                "You know, cheap and weird tasting snacks...",
                "So he had to hide when he had them."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Little Boy",
            args![
                "His mother wouldn't let him",
                "have those kind of snacks...",
                "So he hid them from her.",
                "I don't know why he liked",
                "those junk foods anyways."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Little Boy",
            args![
                "He usually enjoyed having",
                "Monster's Feed.",
                "He said it is good",
                "with Red Potion soup.",
                "He always had a Red Potion and Monster's Feed for his lunch."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Little Boy",
            args![
                "He's got a weird appetite.",
                "I can't understand why he liked",
                "those foods. He was odd..."
            ],
        )?;
        ctx.next()?;
        ctx.var("ep13_2_hiki").set(Val::from(6))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10082), Val::from(10083)])?;
        ctx.lines_as(
            "Little Boy",
            args![
                "Anyway, I already told you",
                "what I know about him.",
                "So, I am done now.",
                "Let me know if you have more Bananas. Haha.",
                "I love Bananas~~!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_2_hiki").get()? == 4 && ctx.call(Function::CountItem, vec![Val::from(513)])?.number()? > 0) {
        ctx.lines_as(
            "Little Boy",
            args!["Huh? What a delicious smell!", "You brought me Bananas!", "Oh, thanks so much!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Little Boy", args!["Let me have them first."])?;
        ctx.next()?;
        ctx.lines_as("Little Boy", args!["Chew, chew..."])?;
        ctx.next()?;
        ctx.lines_as("Little Boy", args!["Yum Yum..."])?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(513), Val::from(1)])?;
        ctx.var("ep13_2_hiki").set(Val::from(5))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10081), Val::from(10082)])?;
        ctx.lines_as("Little Boy", args!["Wait. Wait...", "***Gulp***", "just a second...", "Wait."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_2_hiki").get()? == 4 {
        ctx.lines_as("Little Boy", args!["Banana~banana~", "I love bananas~"])?;
        ctx.next()?;
        ctx.lines_as(
            "Little Boy",
            args!["They don't sell bananas in this village...", "I am eager to have bananas."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Little Boy",
            args!["Oh~ you promised!", "Bring bananas for me~!! I will wait for you!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_2_hiki").get()? == 3 {
        ctx.lines_as(
            "Little Boy",
            args!["I am hungry. We don't have", "much snack bar in this village."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Little Boy",
            args![
                "Huh? Iromo?",
                "Oh, I haven't seen him for a long time...",
                "Hum.. Where is he?",
                "Did he move out?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Little Boy", args!["Huh? His favorite food?.", "Hum.. What was it..?"])?;
        ctx.next()?;
        ctx.lines_as("Little Boy", args!["If I can remember it can you buy me a banana?"])?;
        ctx.next()?;
        ctx.lines_as("Little Boy", args!["Banana~banana~", "I love bananas~"])?;
        ctx.next()?;
        ctx.lines_as(
            "Little Boy",
            args!["They don't sell bananas in this village...", "I am eager to have bananas."],
        )?;
        ctx.next()?;
        ctx.var("ep13_2_hiki").set(Val::from(4))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10080), Val::from(10081)])?;
        ctx.lines_as(
            "Little Boy",
            args![
                "Oh, you will buy me a banana?",
                "I'll try to remember what Iromo's favorite food is for sure if you bring it to me."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Little Boy",
        args!["I am hungry. We don't have", "much snack bar in this village."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Little Boy",
        args!["I hope they sell bananas", "in this village.", "Like other nice villages."],
    )?;
    ctx.next()?;
    ctx.lines_as("Little Boy", args!["This village is so boring...", "What a small village."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn little_boy_ep3_2(ctx: &Ctx) -> Script {
    little_boy_ep3_2_body(ctx, Vec::new()).map(|_| ())
}
