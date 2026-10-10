#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants, runtime};

pub fn louise_kim_designer(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Designer Louise Kim",
        args![
            "Cone shaped red Santa hat is too ordinary.",
            "It's old fashioned.",
            "Maybe in 1980's?!",
            "Haha~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Designer Louise Kim",
        args![
            "If you leave it on me,",
            "I'll change it to lastest model.",
            "You know what I mean~!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Designer Louise Kim",
        args![
            "You know Antonio's hat,right?",
            "Guess who made it?",
            "As you know, Antonio is hard to catch,",
            "that's because I blowed some power in the hat. "
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Designer Louise Kim",
        args![
            "If you don't like your hat,",
            "bring it to me.",
            "I'll change it to brand new one.",
            "Stylish Louise's hat."
        ],
    )?;
    ctx.call(Function::Emotion, args![constants::ET_THROB])?;
    ctx.next()?;
    if ctx.items().count(2236)? > 0 {
        if ctx.menu(&["Here.", "It's ok."])? == 0 {
            ctx.lines_as(
                "Designer Louise Kim",
                args![
                    "Nice choice!!",
                    "If I do it like this ...",
                    "and this and...",
                    "finally it'll turn into fantastic hat.",
                    "But before that,I need some materials to make with."
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["What are the materials?", "So what? I don't want to know."])? == 0 {
                ctx.lines_as(
                    "Designer Louise Kim",
                    args![
                        "Well, nothing special.",
                        "Basically, you need Santa's hat of course.",
                        "and with a touch of my magical fingers,",
                        "it'll just turn into very special thing.",
                        "Well,just little bit prettier and",
                        "little bit more practical. Haha..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Designer Louise Kim",
                    args![
                        "Anyway,to sum up,required materials are....",
                        "Basically ^0000FFSanta's Hat^000000 and",
                        " ^0000FF 1 Cactus Needle ^000000 for sewing, ",
                        "^0000FF 10 Holy Water ^000000 for blessing, ",
                        "^0000FF 1 Rosary ^000000 for luckiness.",
                        "It's pretty enough to make Louise Hat."
                    ],
                )?;
                ctx.next()?;
                if ctx.items().count(952)? > 0 && ctx.items().count(523)? > 9 && ctx.items().count(2608)? > 0 {
                    let choice = runtime::select_values(ctx, &[Val::from("Here you are.....")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Designer Louise Kim",
                        args![
                            "Wow~~!! So fast!!",
                            "I like your style~.",
                            "Ok!! If everything is ready, no need to hesitate.",
                            "I'll show you what Designer Louise Kim's power is."
                        ],
                    )?;
                    ctx.call(Function::Emotion, args![constants::ET_BEST])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "-She puts the hat in a bucket -",
                        "-filled with Holy Water.-",
                        "-She rapidly takes it out and starts mending the hat-",
                        "-humming a tune.-"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Designer Louise Kim",
                        args!["~With the number one designer, Louise Kim,~", "~you are the most blessed soul.~"],
                    )?;
                    ctx.call(Function::Emotion, args![constants::ET_DELIGHT])?;
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_BLESSING])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "-Immediately, she puts Rosary in an-",
                        "-unknown liquid and dissolves it.-",
                        "-And with a brush,-",
                        "-neatly coats the liquid on -",
                        "-a thread of the hat.-"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Designer Louise Kim",
                        args!["~With the number one designer, Louise Kim,~", "~you are the luckiest soul.~"],
                    )?;
                    ctx.call(Function::Emotion, args![constants::ET_DELIGHT])?;
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_GLORIA])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "-She starts sewing the hat with -",
                        "-a Cactus Needle and a thread.-",
                        " "
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Designer Louise Kim",
                        args![
                            "~This is called the Louise's miracle.~",
                            "~The most talented disigner,~",
                            "~L_O_U_I_S_E K_I_M~"
                        ],
                    )?;
                    ctx.call(Function::Emotion, args![constants::ET_THROB])?;
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_BENEDICTIO])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Designer Louise Kim",
                        args![
                            "Here!! All done~~",
                            "How do you like it?",
                            "Isn't it so wonderful?",
                            "Take it!! It's a gift."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Designer Louise Kim",
                        args![
                            "I should have called high price for it",
                            "but as you know it's Christmas!",
                            "It's a gift for you!",
                            "I won't charge anything.",
                            "Just tell many people how good it is."
                        ],
                    )?;
                    ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Designer Louise Kim",
                        args![
                            "Wherever you go,",
                            "whatever you do,",
                            "never take off the hat.",
                            "You won't have a chance to buy it",
                            "no matter how much you pay."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Designer Louise Kim",
                        args![
                            "Alright~Go ahead~",
                            "Go brag yourself!",
                            "~Who would be happier than~",
                            "~being with Louise.~"
                        ],
                    )?;
                    ctx.call(Function::Emotion, args![constants::ET_DELIGHT])?;
                    ctx.items().take(2236, 1)?;
                    ctx.items().take(952, 1)?;
                    ctx.items().take(523, 10)?;
                    ctx.items().take(2608, 1)?;
                    ctx.items().give(5136, 1)?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Designer Louise Kim",
                    args![
                        "Come on~If you just get me the materials,",
                        "I won't charge anything,",
                        "Call me if you change your mind."
                    ],
                )?;
                ctx.call(Function::Emotion, args![constants::ET_THROB])?;
                return ctx.close();
            }
            ctx.lines_as("Designer Louise Kim", args!["You'll regret!", "Think again!"])?;
            ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
            return ctx.close();
        }
        ctx.lines_as(
            "Designer Louise Kim",
            args!["Ok~ whatever~", "It's not me,", "who's going to lose whose own luck."],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Designer Louise Kim",
        args![
            "Perhaps you get the chance to acheive Santa's Hat later some time,",
            "think about it carefully.",
            "You can get the better designed hat,",
            "and I can show off my talent."
        ],
    )?;
    ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
    ctx.close()
}

#[derive(Clone, Copy, Debug)]
enum EnjoyEnjoyStep {
    Start,
    OnMyMobDead,
    OnCommandGo,
    OnCommandStop,
    OnTimer3000,
    OnTimer5000,
    OnTimer7000,
    OnTimer9000,
    OnTimer11000,
    OnTimer13000,
    OnTimer180000,
}

const ENJOY_SINGLE_MOBS: [(i32, i32, i32, &str); 25] = [
    (155, 300, 1062, "'s anguish"),
    (156, 300, 1062, "'s jealousy"),
    (157, 300, 1062, "'s despair"),
    (158, 300, 1062, "'s frustration"),
    (154, 300, 1062, "'s bombing"),
    (158, 299, 1246, "'s grudge"),
    (157, 299, 1246, "'s curse"),
    (156, 299, 1246, "'s anger"),
    (155, 299, 1246, "'s grief"),
    (154, 299, 1246, "'s hatred"),
    (158, 298, 1245, "'s a bitter taste of solo"),
    (157, 298, 1245, "'s couple punisher"),
    (156, 298, 1245, "'s loneliness"),
    (155, 298, 1245, "'s sobbing"),
    (154, 298, 1245, "'s darkness"),
    (158, 297, 1244, "'s depression"),
    (157, 297, 1244, "'s estrangement"),
    (156, 297, 1244, "'s nightmare"),
    (155, 297, 1244, "'s wail"),
    (154, 297, 1244, "'s whisper"),
    (158, 296, 1588, "'s regret"),
    (157, 296, 1588, "'s shadow"),
    (156, 296, 1588, "'s couplebreaker"),
    (155, 296, 1588, "'s sadness"),
    (154, 296, 1588, "'s symbol of brokenheart"),
];

fn enjoy_enjoy_run(ctx: &Ctx, mut step: EnjoyEnjoyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            EnjoyEnjoyStep::Start => {
                if !ctx.var("christ_solo05").get()?.is_true() {
                    ctx.lines_as(
                        "Enjoy",
                        args![
                            "Oh~~~",
                            "It's already winter again~~!",
                            "This chilling weather makes",
                            "my body freeze~",
                            "And also makes my heart freeze.",
                            "Who said that christmas is only for lovers~",
                            "Oh~~I'm so lonely~!!!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Enjoy",
                        args![
                            "Pitiless sister!!",
                            "How can she leave me alone on a christmas day~",
                            "'Spend your days with family on a chirstmas day'",
                            "is our family precept",
                            "Hm...I need to get some rest.",
                            "I'm so nervous these days~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Enjoy",
                        args![
                            "Lets make a joyful christmas for ",
                            "lonely singles.",
                            "Who's with me?!!",
                            "!!!!!!!!!!!",
                            "!!!!!!!!!!!!!"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Wow!", "Hm...I'm not interested.", "I have ~"])? {
                        0 => {
                            ctx.lines_as(
                                "Enjoy",
                                args![
                                    "Alright~~!!!",
                                    "Let's rock and roll!!",
                                    "Here's my plan!!",
                                    "Let's punish those couples",
                                    "who are so excited about christmas.",
                                    "I just don't want to see them happy."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Enjoy",
                                args![
                                    "To put in action,",
                                    "we need to gather many phalanges.",
                                    "Ok!!Bring our phalanges in every town on a way back here. ",
                                    "Alright?Let's go!!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Enjoy",
                                args![
                                    "Each should have one person's phone number.",
                                    "We must keep it secret before we put in action.",
                                    "So we must be very careful.",
                                    "Well...",
                                    "I know ^0000FFHappymerry^000000's phone number."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Enjoy",
                                args![
                                    "First of all, go look for ^0000FFHappymerry^000000",
                                    "and tell him about our plan.",
                                    "On a way back, bring as many phalanges as you can. "
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(ctx.player().name()?, args!["Alright,sir!!!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Enjoy",
                                args![
                                    "Oh,and don't forget to bring",
                                    "5 branch of dead trees!!!",
                                    "Must bring item to attack town~",
                                    "hahahaha~~"
                                ],
                            )?;
                            ctx.call(Function::Emotion, args![constants::ET_KIK])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Enjoy",
                                args![
                                    "Get it?!!!!",
                                    "Let's go punish!!!",
                                    "We are not being jealous,",
                                    "It's just not right leading a loose life!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(ctx.player().name()?, args!["Let's punish!!!!!!!!!!!!!!!!!!!!!!!!!!!"])?;
                            ctx.var("christ_solo05").set(1)?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        1 => {
                            ctx.lines_as("Enjoy", args!["If you are not with me, get away~!!", "Get out of my sight!!!!"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as("Enjoy", args!["......"])?;
                            ctx.call(Function::NpcSpecialEffect, args![constants::EF_BLESSING])?;
                            ctx.next()?;
                            ctx.lines_as("Enjoy", args!["............."])?;
                            ctx.call(Function::NpcSpecialEffect, args![constants::EF_INCAGIDEX])?;
                            ctx.next()?;
                            ctx.lines_as("Enjoy", args!["........................"])?;
                            ctx.call(Function::NpcSpecialEffect, args![constants::EF_STEELBODY])?;
                            ctx.next()?;
                            ctx.lines_as("Enjoy", args!["........................", "Get lost,you devil!!!!!!"])?;
                            ctx.call(Function::NpcSpecialEffect, args![constants::EF_BEGINASURA])?;
                            ctx.call(Function::SpecialEffect, args![constants::EF_HIT2])?;
                            ctx.var("Hp").set(ctx.var("Hp").get()?.number()? / 2)?;
                            ctx.var("Hp").set(ctx.var("Hp").get()?.number()? / 2)?;
                            ctx.close_window()?;
                            ctx.warp("prontera", 155, 230)?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("christ_solo05").get()?.number()? > 0 && ctx.var("christ_solo05").get()?.number()? < 5 {
                    ctx.lines_as(
                        "Enjoy",
                        args![
                            "Each should have one person's phone number.",
                            "We must keep it secret before we put in action.",
                            "So we must be very careful.",
                            "Well...",
                            "I know ^0000FFHappymerry^000000's phone number."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Enjoy",
                        args![
                            "First of all, go look for ^0000FFHappymerry^000000",
                            "and tell him about our plan.",
                            "On a way back, bring as many phalanges as you can."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Enjoy",
                        args![
                            "Oh,and don't forget to bring",
                            "5 branch of dead trees!!!",
                            "Let's go punish couples!!!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("christ_solo05").get()? == 5 {
                    if ctx.items().count(604)? > 4 {
                        ctx.lines_as(
                            "Enjoy",
                            args![
                                "Did you do as I told you to do?!!!",
                                "Did you bring branch of dead trees?Let me see~~!!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Enjoy", args!["Fine!!", "You are all ready!!!"])?;
                        ctx.next()?;
                        ctx.lines_as(ctx.player().name()?, args!["Wait!!", "Where's other people??"])?;
                        ctx.next()?;
                        ctx.lines_as("Enjoy", args!["What are you talking about?!", "They are already here......."])?;
                        ctx.next()?;
                        ctx.lines_as("Enjoy", args!["Can't you see?!", "Please~!!!!Are you ok?!!!"])?;
                        ctx.npc().do_event("Happymerry#happymerry02::OnCommandOn")?;
                        ctx.npc().do_event("Christ#christ02::OnCommandOn")?;
                        ctx.npc().do_event("Mas#mas02::OnCommandOn")?;
                        ctx.npc().do_event("Event#event02::OnCommandOn")?;
                        ctx.next()?;
                        ctx.lines_as(ctx.player().name()?, args!["Oh.. Yup!!!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Enjoy",
                            args![
                                "Welcome!! Welcome,my phalanges!!!",
                                "Being a single is not a sin.",
                                "Why do we have to hide ourselves from ",
                                "their sight!",
                                "I hate couples!!",
                                "Poor single!!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Enjoy",
                            args![
                                "For those of who have friend who just met his/her mate,",
                                "or who had to turn his/her back from kissing couples!!",
                                "What are you waiting for!!",
                                "Why do we have to be the victim!!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Enjoy", args!["It's christmas season again!!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Enjoy",
                            args![
                                "Are we the soldiers",
                                "who have beaten up monsters with our bare hand.",
                                "Don't you remember the days?!!We have jumped down from Air ship!!",
                                "We are well trained singles!!Haha~!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Enjoy",
                            args![
                                "We don't have to wipe our tears",
                                "looking at party players anymore.",
                                "No need to envy!!",
                                "This christmas is for singles!! ",
                                "Yahoo~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "All",
                            args![
                                "Christmas for singles!!!",
                                "No more envy!!No more sorrow!!No more anger!!!",
                                "Christmas for singles!!!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args!["-Enjoy takes away branches of dead trees.", "-Grabs them tight.-"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Enjoy",
                            args![
                                "Let's punish couples,",
                                "those of who leading a loose life!!",
                                " ",
                                "[All]",
                                "Let's punish!!!!!"
                            ],
                        )?;
                        ctx.call(Function::Emotion, args![constants::ET_GO])?;
                        ctx.call(
                            Function::Emotion,
                            args![constants::ET_GO, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
                        )?;
                        ctx.npc().do_event("Happymerry#happymerry02::OnCommandEmotion")?;
                        ctx.npc().do_event("Christ#christ02::OnCommandEmotion")?;
                        ctx.npc().do_event("Mas#mas02::OnCommandEmotion")?;
                        ctx.npc().do_event("Event#event02::OnCommandEmotion")?;
                        ctx.call(Function::NpcSpecialEffect, args![constants::EF_HITLINE2])?;
                        ctx.items().take(604, 5)?;
                        ctx.var("christ_solo05").set(6)?;
                        ctx.npc().do_event("Happymerry#happymerry02::OnCommandOff")?;
                        ctx.npc().do_event("Christ#christ02::OnCommandOff")?;
                        ctx.npc().do_event("Mas#mas02::OnCommandOff")?;
                        ctx.npc().do_event("Event#event02::OnCommandOff")?;
                        ctx.close_window()?;
                        ctx.call(
                            Function::MapAnnounce,
                            args![
                                "prontera",
                                Val::from("Single soldiers ") + ctx.player().name()? + "'s sorrow spread all over the town.",
                                constants::BC_MAP,
                                6750156
                            ],
                        )?;
                        for (x, y, mob, suffix) in ENJOY_SINGLE_MOBS {
                            ctx.call(
                                Function::Monster,
                                args!["prontera", x, y, ctx.player().name()? + suffix, mob, 1, "Enjoy#enjoy::OnMyMobDead"],
                            )?;
                        }
                        ctx.npc().do_event("Enjoy#enjoy::OnCommandGo")?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Enjoy",
                            args![
                                "There's no much time left!!!",
                                "No time to hesitate!!",
                                "Couples will enjoy their christmas day",
                                "so happily.",
                                "Are you going to leave them like that!!!!!",
                                "Let's go let's go!!",
                                "Go get ^0000FF 5 branch of dead tree^000000s!!!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.lines_as(
                        "Enjoy",
                        args![
                            "Hm.... ",
                            "It's no use just blaming oneself!",
                            "We lonely fellows can build our own hopeful future.",
                            "Let's go!!",
                            "Let's go phalanges!!!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Enjoy",
                        args![
                            "For the day we all get happy~!!",
                            "Let's go for it!!!",
                            "Cheer up everybody!!!",
                            "Let's rock till you get happy~!"
                        ],
                    )?;
                    ctx.var("christ_solo05").set(0)?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = EnjoyEnjoyStep::OnMyMobDead;
                continue 'machine;
            }
            EnjoyEnjoyStep::OnMyMobDead => {
                return Err(Stop::End);
            }
            EnjoyEnjoyStep::OnCommandGo => {
                ctx.set_npc_visible("Enjoy#enjoy", false)?;
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            EnjoyEnjoyStep::OnCommandStop => {
                ctx.set_npc_visible("Enjoy#enjoy", true)?;
                ctx.call(Function::KillMonster, args!["prontera", "Enjoy#enjoy::OnMyMobDead"])?;
                ctx.call(Function::StopNpcTimer, args![])?;
                return Err(Stop::End);
            }
            EnjoyEnjoyStep::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args!["prontera", "You must refine by yourself to satisfy!!!!", constants::BC_MAP, 6750156],
                )?;
                return Err(Stop::End);
            }
            EnjoyEnjoyStep::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "prontera",
                        "It's a waste to organize party at dungeon!!!",
                        constants::BC_MAP,
                        6750156
                    ],
                )?;
                return Err(Stop::End);
            }
            EnjoyEnjoyStep::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args!["prontera", "There is a NPC flirting me!!!", constants::BC_MAP, 6750156],
                )?;
                return Err(Stop::End);
            }
            EnjoyEnjoyStep::OnTimer9000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "prontera",
                        "I was always alone from the day I was born!!",
                        constants::BC_MAP,
                        6750156
                    ],
                )?;
                return Err(Stop::End);
            }
            EnjoyEnjoyStep::OnTimer11000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args!["prontera", "We dig herbs even on a christmas day!!", constants::BC_MAP, 6750156],
                )?;
                return Err(Stop::End);
            }
            EnjoyEnjoyStep::OnTimer13000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "prontera",
                        "...We are the insuperable single soldiers!!!",
                        constants::BC_MAP,
                        6750156
                    ],
                )?;
                return Err(Stop::End);
            }
            EnjoyEnjoyStep::OnTimer180000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "prontera",
                        "Wish every single soldiers have a merry christmas!!",
                        constants::BC_MAP,
                        6750156
                    ],
                )?;
                ctx.npc().do_event("Enjoy#enjoy::OnCommandStop")?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn enjoy_enjoy(ctx: &Ctx) -> Script {
    enjoy_enjoy_run(ctx, EnjoyEnjoyStep::Start, Vec::new()).map(|_| ())
}

pub fn enjoy_enjoy_onmymobdead(ctx: &Ctx) -> Script {
    enjoy_enjoy_run(ctx, EnjoyEnjoyStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn enjoy_enjoy_oncommandgo(ctx: &Ctx) -> Script {
    enjoy_enjoy_run(ctx, EnjoyEnjoyStep::OnCommandGo, Vec::new()).map(|_| ())
}

pub fn enjoy_enjoy_oncommandstop(ctx: &Ctx) -> Script {
    enjoy_enjoy_run(ctx, EnjoyEnjoyStep::OnCommandStop, Vec::new()).map(|_| ())
}

pub fn enjoy_enjoy_ontimer3000(ctx: &Ctx) -> Script {
    enjoy_enjoy_run(ctx, EnjoyEnjoyStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn enjoy_enjoy_ontimer5000(ctx: &Ctx) -> Script {
    enjoy_enjoy_run(ctx, EnjoyEnjoyStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn enjoy_enjoy_ontimer7000(ctx: &Ctx) -> Script {
    enjoy_enjoy_run(ctx, EnjoyEnjoyStep::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn enjoy_enjoy_ontimer9000(ctx: &Ctx) -> Script {
    enjoy_enjoy_run(ctx, EnjoyEnjoyStep::OnTimer9000, Vec::new()).map(|_| ())
}

pub fn enjoy_enjoy_ontimer11000(ctx: &Ctx) -> Script {
    enjoy_enjoy_run(ctx, EnjoyEnjoyStep::OnTimer11000, Vec::new()).map(|_| ())
}

pub fn enjoy_enjoy_ontimer13000(ctx: &Ctx) -> Script {
    enjoy_enjoy_run(ctx, EnjoyEnjoyStep::OnTimer13000, Vec::new()).map(|_| ())
}

pub fn enjoy_enjoy_ontimer180000(ctx: &Ctx) -> Script {
    enjoy_enjoy_run(ctx, EnjoyEnjoyStep::OnTimer180000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum HappymerryHappymerry02Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandEmotion,
    OnCommandOff,
    OnTimer60000,
}

fn happymerry_happymerry02_run(ctx: &Ctx, mut step: HappymerryHappymerry02Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HappymerryHappymerry02Step::Start => {
                return Err(Stop::End);
            }
            HappymerryHappymerry02Step::OnInit => {
                ctx.set_npc_visible("Happymerry#happymerry02", false)?;
                ctx.call(Function::StopNpcTimer, args![])?;
                return Err(Stop::End);
            }
            HappymerryHappymerry02Step::OnCommandOn => {
                ctx.call(Function::InitNpcTimer, args![])?;
                ctx.set_npc_visible("Happymerry#happymerry02", true)?;
                step = HappymerryHappymerry02Step::OnCommandEmotion;
                continue 'machine;
            }
            HappymerryHappymerry02Step::OnCommandEmotion => {
                ctx.call(Function::Emotion, args![constants::ET_GO])?;
                return Err(Stop::End);
            }
            HappymerryHappymerry02Step::OnCommandOff => {
                ctx.set_npc_visible("Happymerry#happymerry02", false)?;
                ctx.call(Function::StopNpcTimer, args![])?;
                return Err(Stop::End);
            }
            HappymerryHappymerry02Step::OnTimer60000 => {
                ctx.npc().do_event("Happymerry#happymerry02::OnCommandOff")?;
                ctx.npc().do_event("Christ#christ02::OnCommandOff")?;
                ctx.npc().do_event("Mas#mas02::OnCommandOff")?;
                ctx.npc().do_event("Event#event02::OnCommandOff")?;
                ctx.call(Function::StopNpcTimer, args![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn happymerry_happymerry02(ctx: &Ctx) -> Script {
    happymerry_happymerry02_run(ctx, HappymerryHappymerry02Step::Start, Vec::new()).map(|_| ())
}

pub fn happymerry_happymerry02_oninit(ctx: &Ctx) -> Script {
    happymerry_happymerry02_run(ctx, HappymerryHappymerry02Step::OnInit, Vec::new()).map(|_| ())
}

pub fn happymerry_happymerry02_oncommandon(ctx: &Ctx) -> Script {
    happymerry_happymerry02_run(ctx, HappymerryHappymerry02Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn happymerry_happymerry02_oncommandemotion(ctx: &Ctx) -> Script {
    happymerry_happymerry02_run(ctx, HappymerryHappymerry02Step::OnCommandEmotion, Vec::new()).map(|_| ())
}

pub fn happymerry_happymerry02_oncommandoff(ctx: &Ctx) -> Script {
    happymerry_happymerry02_run(ctx, HappymerryHappymerry02Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn happymerry_happymerry02_ontimer60000(ctx: &Ctx) -> Script {
    happymerry_happymerry02_run(ctx, HappymerryHappymerry02Step::OnTimer60000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ChristChrist02Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandEmotion,
    OnCommandOff,
}

fn christ_christ02_run(ctx: &Ctx, mut step: ChristChrist02Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ChristChrist02Step::Start => {
                return Err(Stop::End);
            }
            ChristChrist02Step::OnInit => {
                ctx.set_npc_visible("Christ#christ02", false)?;
                return Err(Stop::End);
            }
            ChristChrist02Step::OnCommandOn => {
                ctx.set_npc_visible("Christ#christ02", true)?;
                step = ChristChrist02Step::OnCommandEmotion;
                continue 'machine;
            }
            ChristChrist02Step::OnCommandEmotion => {
                ctx.call(Function::Emotion, args![constants::ET_GO])?;
                return Err(Stop::End);
            }
            ChristChrist02Step::OnCommandOff => {
                ctx.set_npc_visible("Christ#christ02", false)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn christ_christ02(ctx: &Ctx) -> Script {
    christ_christ02_run(ctx, ChristChrist02Step::Start, Vec::new()).map(|_| ())
}

pub fn christ_christ02_oninit(ctx: &Ctx) -> Script {
    christ_christ02_run(ctx, ChristChrist02Step::OnInit, Vec::new()).map(|_| ())
}

pub fn christ_christ02_oncommandon(ctx: &Ctx) -> Script {
    christ_christ02_run(ctx, ChristChrist02Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn christ_christ02_oncommandemotion(ctx: &Ctx) -> Script {
    christ_christ02_run(ctx, ChristChrist02Step::OnCommandEmotion, Vec::new()).map(|_| ())
}

pub fn christ_christ02_oncommandoff(ctx: &Ctx) -> Script {
    christ_christ02_run(ctx, ChristChrist02Step::OnCommandOff, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MasMas02Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandEmotion,
    OnCommandOff,
}

fn mas_mas02_run(ctx: &Ctx, mut step: MasMas02Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MasMas02Step::Start => {
                return Err(Stop::End);
            }
            MasMas02Step::OnInit => {
                ctx.set_npc_visible("Mas#mas02", false)?;
                return Err(Stop::End);
            }
            MasMas02Step::OnCommandOn => {
                ctx.set_npc_visible("Mas#mas02", true)?;
                step = MasMas02Step::OnCommandEmotion;
                continue 'machine;
            }
            MasMas02Step::OnCommandEmotion => {
                ctx.call(Function::Emotion, args![constants::ET_GO])?;
                return Err(Stop::End);
            }
            MasMas02Step::OnCommandOff => {
                ctx.set_npc_visible("Mas#mas02", false)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mas_mas02(ctx: &Ctx) -> Script {
    mas_mas02_run(ctx, MasMas02Step::Start, Vec::new()).map(|_| ())
}

pub fn mas_mas02_oninit(ctx: &Ctx) -> Script {
    mas_mas02_run(ctx, MasMas02Step::OnInit, Vec::new()).map(|_| ())
}

pub fn mas_mas02_oncommandon(ctx: &Ctx) -> Script {
    mas_mas02_run(ctx, MasMas02Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn mas_mas02_oncommandemotion(ctx: &Ctx) -> Script {
    mas_mas02_run(ctx, MasMas02Step::OnCommandEmotion, Vec::new()).map(|_| ())
}

pub fn mas_mas02_oncommandoff(ctx: &Ctx) -> Script {
    mas_mas02_run(ctx, MasMas02Step::OnCommandOff, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum EventEvent02Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandEmotion,
    OnCommandOff,
}

fn event_event02_run(ctx: &Ctx, mut step: EventEvent02Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            EventEvent02Step::Start => {
                return Err(Stop::End);
            }
            EventEvent02Step::OnInit => {
                ctx.set_npc_visible("Event#event02", false)?;
                return Err(Stop::End);
            }
            EventEvent02Step::OnCommandOn => {
                ctx.set_npc_visible("Event#event02", true)?;
                step = EventEvent02Step::OnCommandEmotion;
                continue 'machine;
            }
            EventEvent02Step::OnCommandEmotion => {
                ctx.call(Function::Emotion, args![constants::ET_GO])?;
                return Err(Stop::End);
            }
            EventEvent02Step::OnCommandOff => {
                ctx.set_npc_visible("Event#event02", false)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn event_event02(ctx: &Ctx) -> Script {
    event_event02_run(ctx, EventEvent02Step::Start, Vec::new()).map(|_| ())
}

pub fn event_event02_oninit(ctx: &Ctx) -> Script {
    event_event02_run(ctx, EventEvent02Step::OnInit, Vec::new()).map(|_| ())
}

pub fn event_event02_oncommandon(ctx: &Ctx) -> Script {
    event_event02_run(ctx, EventEvent02Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn event_event02_oncommandemotion(ctx: &Ctx) -> Script {
    event_event02_run(ctx, EventEvent02Step::OnCommandEmotion, Vec::new()).map(|_| ())
}

pub fn event_event02_oncommandoff(ctx: &Ctx) -> Script {
    event_event02_run(ctx, EventEvent02Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn happymerry_happymerry(ctx: &Ctx) -> Script {
    if ctx.var("christ_solo05").get()? == 1 {
        ctx.lines_as(
            "Happymerry",
            args![
                "Holgren~~!!",
                "I've never expected you betraying me! Don't wanna get refined~!!!",
                "I hate christmas~!"
            ],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_CRY])?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["Are...you...?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Happymerry",
            args![
                "What are you laughing at? huh~!",
                "At least, I never borrowed a hand",
                "to get my equips refined!",
                "I was always brave!!",
                "Blessing? Gloria~~?!",
                "Couples~~duh~!!!!!!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args!["You seem to be the right one!!!", "Enjoy is waiting for you.", "let's go!!!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Happymerry",
            args![
                "...!",
                "The day has come?",
                "He help me last christmas,",
                "when I failed refining my equips.",
                "Oh, holy Enjoy~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Happymerry",
            args![
                "Alright!I've been waiting for a year!!",
                "I'm ready to mess up christmas day~!!",
                "So,where is Enjoy?",
                "Where is he?!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args!["He's waiting for you!", "Go ahead~", "I'll follow you after contacting others."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Happymerry",
            args![
                "Alright!",
                "I was supposed to call ^0000FFChrist^000000!",
                "Call him for me!",
                "I'll go ahead with my bags packed up.",
                "See ya!"
            ],
        )?;
        ctx.var("christ_solo05").set(2)?;
        return ctx.close();
    }
    if ctx.var("christ_solo05").get()?.number()? > 1 {
        ctx.lines_as(
            "Happymerry",
            args![
                "Hm...There's more things to pack up than I thought.",
                "Well,it's been a year.....",
                "Anyway,",
                "Don't for get to call ^0000FFChrist^000000!",
                "See ya!"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Happymerry",
        args![
            "Holgren!!",
            "I've never expected you betraying me! Don't wanna get refined~!!!",
            "I hate christmas~!"
        ],
    )?;
    ctx.call(Function::Emotion, args![constants::ET_CRY])?;
    ctx.next()?;
    ctx.lines_as(
        "Happymerry",
        args![
            "What are you laughing at? huh~!",
            "At least, I never borrowed a hand",
            "to get my equips refined!",
            "I was always brave!!",
            "Blessing? Gloria~~?!",
            "Couples~~duh~!!!!!!"
        ],
    )?;
    ctx.close()
}

pub fn christ_christ(ctx: &Ctx) -> Script {
    if ctx.var("christ_solo05").get()? == 2 {
        ctx.lines_as(
            "Christ",
            args![
                "Now~finally!!!",
                "I get to ride Pecopeco~!",
                "Why do need to organize a party with priest?!",
                "I don't need all that.",
                "Only thing I need is this chubby Pecopeco~!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["Um..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Christ",
            args![
                "Who are you!",
                "Don't ever think to get around my Peco~!",
                "Oh~my sweat Peco~~Weren't you scared? It's ok darling.",
                "Enjoy was all alone lonely from the day he were born~",
                "But me?!! Nope!!",
                "I have my sweat peco with me!!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Christ",
            args![
                "I'm going to held party with my peco.",
                "We'll share christmas cake together and.....",
                "I'm not gonna be lonely~",
                "No I won't!!!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["Actually Enjoy told me..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Christ",
            args![
                "Huh? What did you say?",
                "Enjoy? You know him? Then,you must be the one whom Happymerry sent!?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["Yes~ Happymerry sent me..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Christ",
            args![
                "Finally, the day has come!!",
                "Did you hear it? Peco~",
                "...We have an amazing plan!",
                "This christmas is gonna be fantastic!!",
                "No need to envy couples!!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Christ",
            args![
                "Alright!!",
                "I'll run to Enjoy with my peco~!",
                "Go tell ^0000FFMas^000000",
                "about this!!!",
                "See ya~~!"
            ],
        )?;
        ctx.var("christ_solo05").set(3)?;
        return ctx.close();
    }
    if ctx.var("christ_solo05").get()?.number()? > 2 {
        ctx.lines_as(
            "Christ",
            args![
                "Than,see you there!!",
                "I'll go meet Enjoy!",
                "Never forget to tell ^0000FFMas^000000",
                "about this!!!",
                "See ya~~!"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Christ",
        args![
            "Now~finally!!!",
            "I get to ride Pecopeco~!",
            "Why do need to organize a party with priest?!",
            "I don't need all that.",
            "Only thing I need is this chubby Pecopeco~!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Christ",
        args![
            "Who are you!",
            "Don't ever think to get around my Peco~!",
            "Oh~my sweat Peco~~Weren't you scared?It's ok darling.",
            "Enjoy was all alone lonely from the day he were born~",
            "But me?!!Nope!!",
            "I have my sweat peco with me!!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Christ",
        args![
            "I'm going to held party with my peco.",
            "We'll share christmas cake together and.....",
            "I'm not gonna be lonely~",
            "No I won't!!!"
        ],
    )?;
    ctx.close()
}

pub fn mas_mas(ctx: &Ctx) -> Script {
    if ctx.var("christ_solo05").get()? == 3 {
        ctx.lines_as(
            "Mas",
            args![
                "Herds!!!!!How long does it take!",
                "Somebody know the regenerating time of Herb?!!",
                "I'll dig herbs and make potions and sell it to singles!!",
                "Hahahaha!!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mas",
            args![
                "Ah~~",
                "How come I feel so empty~.",
                "although I have herds fill in a storage. ",
                "No~~!!!!!!",
                "No time to waste~~",
                "Let's dig herbs......."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["Hey~are you ok? Are you Mas?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Mas",
            args![
                "Who...who are you?!!!",
                "Well, it's been so long since I spoke to a stranger.",
                "Hm...",
                "I feel something warm inside my heart....... ",
                "Never mind!! What am I thinking?!!",
                "Get away~I have dig herbs~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["Mas!!", "Are you ok?!", "Christ sent me."])?;
        ctx.next()?;
        ctx.lines_as(
            "Mas",
            args![
                "What!!!Already!!",
                "Yeah~right!",
                "I don't need to spend times digging herbs!!",
                "If Enjoy made an order!?!",
                "I'll be there right away~!!!!!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Mas", args!["...Are going with me?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args!["No~~", "I have something left to do.", "I have to tell others about this."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mas",
            args![
                "Oh yeah right!!!",
                "Go look for ^0000FFEvent^000000.",
                "Well,bye~.",
                "I'll meet you there!!...",
                "Herbs~?! Couples?! Whatever~~"
            ],
        )?;
        ctx.var("christ_solo05").set(4)?;
        return ctx.close();
    }
    if ctx.var("christ_solo05").get()?.number()? > 3 {
        ctx.lines_as(
            "Mas",
            args![
                "If you excuse me, I'll go ahead and meat Enjoy.",
                "And don't forget to tell ^0000FFEvent^000000 about this.",
                "Herbs~couples~~Whatever~~",
                "This christmas is gonna be fantastic!!",
                "Hahahaha~"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Mas",
        args![
            "Herds!!!!!How long does it take!",
            "Somebody know the regenerating time of Herb?!!",
            "I'll dig herbs and make potions and sell it to singles!!",
            "Hahahaha!!"
        ],
    )?;
    ctx.next()?;
    ctx.lines(args![
        "Ah~~",
        "How come I feel so empty~.",
        "although I have herds fill in a storage. ",
        "No~~!!!!!!",
        "No time to waste~~",
        "Let's dig herbs......."
    ])?;
    ctx.close()
}

pub fn event_event(ctx: &Ctx) -> Script {
    if ctx.var("christ_solo05").get()? == 4 {
        ctx.lines_as("Event", args!["........................"])?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["...Are... you...?"])?;
        ctx.next()?;
        ctx.lines_as("Event", args!["...I love you too~!!!"])?;
        ctx.call(
            Function::Emotion,
            args![constants::ET_HUK, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["(Oh,my..)"])?;
        ctx.next()?;
        ctx.lines_as(
            "Event",
            args![
                "It's ok, Tinybee. I'm not lonely at all.",
                "I have Ms.Bathory and Ms.Orclady with me.",
                "Hahahaha~~~"
            ],
        )?;
        ctx.next()?;
        ctx.mes("-He laughed talking to his right hand.-")?;
        ctx.call(Function::Emotion, args![constants::ET_CHUP])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args!["Mas sent me here.", "You know about Enjoy's plan, right?", "...Are you listening?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Event",
            args!["...Did you hear?? Tinybee?", "The day has come!!", "Hahaha~~~~."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Event",
            args![
                "Wait for me miss Kafra~~~",
                "Don't be so lonely~.",
                "I'll make your christmas unforgettably fantastic.",
                "Let's go Tinybee."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "-He kept talking to his right hand-",
            "-and packed his stuff and bowed to Kafra.-"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args![
                "Finally!!! Done telling everyone!!",
                "Now I should get my ^0000FFBranch of Dead Tree^000000s packed up",
                "and go punish singles!!!",
                "Hahahaha.."
            ],
        )?;
        ctx.var("christ_solo05").set(5)?;
        return ctx.close();
    }
    if ctx.var("christ_solo05").get()?.number()? > 4 {
        ctx.lines_as(
            "Event",
            args![
                "Wait for me miss Kafra~~~",
                "Don't be so lonely~.",
                "I'll make your christmas unforgettably fantastic.",
                "Let's go Tinybee."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "-He  talked to his right hand-",
            "-and packed his stuff and bowed to Kafra.-"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args![
                "Finally!!! Done telling everyone!!",
                "Now I should get my things packed up",
                "and go punish singles!!!",
                "Hahahaha.."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Event", args!["........................"])?;
    ctx.next()?;
    ctx.lines_as("Event", args!["...I love you too~!!!"])?;
    ctx.call(
        Function::Emotion,
        args![constants::ET_HUK, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
    )?;
    ctx.next()?;
    ctx.lines_as(ctx.player().name()?, args!["(Oh.my~)"])?;
    ctx.next()?;
    ctx.lines_as(
        "Event",
        args![
            "It's ok Tinybee.I'm not lonely at all.",
            "I have Ms.Bathory and Ms.Orclady with me.",
            "Hahahaha~~~"
        ],
    )?;
    ctx.next()?;
    ctx.mes("-He laughed talking to his right hand.-")?;
    ctx.call(Function::Emotion, args![constants::ET_CHUP])?;
    ctx.close()
}

pub fn oholy_pron(ctx: &Ctx) -> Script {
    if !ctx.var("christ_carol05").get()?.is_true() {
        ctx.lines_as("Oholy", args!["Joy to the world!", "The Lord has come."])?;
        ctx.call(Function::NpcSpecialEffect, args![constants::EF_GLORIA])?;
        ctx.next()?;
        if ctx.var("Sex").get()? == constants::SEX_MALE {
            ctx.lines_as(
                "Oholy",
                args![
                    "Merry Christmas!",
                    "Dear brother, what comes in",
                    "your mind when you think of Christmas?"
                ],
            )?;
            ctx.next()?;
        } else {
            ctx.lines_as(
                "Oholy",
                args![
                    "Merry Christmas!",
                    "Dear sister, what comes in",
                    "your mind when you think of Christmas?"
                ],
            )?;
            ctx.next()?;
        }
        match ctx.menu(&[
            "Santa Claus",
            "Christmas Gifts",
            "Christmas Carols",
            "Santa Hat",
            "I don't like couples",
        ])? {
            0 => {
                ctx.lines_as(
                    "Oholy",
                    args!["Santa Claus!", "You still have childish", "innocence, kid!!!", "Hohoho."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Oholy",
                    args![
                        "There is a rumor that Santa Claus",
                        "in the town where Christmas ",
                        "never ends. This is just",
                        "between you and me, okay?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Oholy", args!["The latest headline by Oholy", "Isn't it amazing?"])?;
                ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
                return ctx.close();
            }
            1 => {
                ctx.lines_as(
                    "Oholy",
                    args![
                        "Gifts! That's nice!",
                        "How exciting it is!!!",
                        "You wake up and find",
                        "christmas gifts next to your pillow!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Oholy",
                    args![
                        "Have you heard that",
                        "bad santa who makes a suprise",
                        "attack in every christmas, has",
                        "taken Santa Claus's gifts to",
                        "good kids!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Oholy",
                    args![
                        "So, Santa Claus in Christmas",
                        "town has offered a reward for",
                        "capturing phony Santa, Antonio."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Oholy", args!["The latest headline by Oholy", "Isn't it amazing?"])?;
                ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
                return ctx.close();
            }
            2 => {
                ctx.lines_as(
                    "Oholy",
                    args![
                        "That's right!",
                        "Carol is the essential for",
                        "Christmas! When I was",
                        "young, my mind used to be",
                        "fluttered by carols during",
                        "Christmas."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Oholy",
                    args![
                        "But, in these days, not many",
                        "people sing Christmas carols",
                        "so it is hard to feel that",
                        "Christmas is coming closer",
                        " "
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Oholy",
                    args![
                        "Therefore, I decided to wish",
                        "a merry christmas to everyone",
                        "by singing Christmas carols and",
                        "giving gifts to kids from door",
                        "to door, but, unfortunately,",
                        "wicked devil has torn off my carol music book!!!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Oholy",
                    args![
                        "I have many houses to visit.",
                        "I feel so sad for disappointed",
                        "kids who didn't hear the carols."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select(ctx, &["...can I help you?"])?;
                ctx.var("@menu").set(choice)?;
                match choice {
                    1 => {}
                    _ => return Err(Stop::Error("Invalid menu selection".into())),
                }
                ctx.lines_as(
                    "Oholy",
                    args![
                        "Good gracious! Are you for real?",
                        "Oh? Shee... Can you hear it?",
                        "Every kids appreciate your kindness."
                    ],
                )?;
                ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
                ctx.lines(args!["Don't be afraid. I will not", "ask you to make a new christmas carol."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Oholy",
                    args![
                        "If you have a will,",
                        "we got no time to waste.",
                        "Let's move on to give hope to kids!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Oholy",
                    args![
                        "Well, please bring me back",
                        Val::from("Christmas carol music book, ") + ctx.player().name()? + ".",
                        "That little devil will be still",
                        "in the town because it only happened a few minutes ago."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Oholy",
                    args![
                        "Please be careful because",
                        "you are dealing with devil.",
                        "Well then, hope you a good luck!!!",
                        " "
                    ],
                )?;
                ctx.call(Function::Emotion, args![constants::ET_BEST])?;
                ctx.var("christ_carol05").set(1)?;
                return ctx.close();
            }
            3 => {
                ctx.lines_as(
                    "Oholy",
                    args![
                        "A Santa Hat!!",
                        "Did you know that the real",
                        "Santa Hat is totally different",
                        "from the one that monsters",
                        "are wearing?! I heard a rumor",
                        "that an anonymous designer",
                        "in Lutie, made all of those santa hats."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Oholy",
                    args![
                        "Also, there is another rumor",
                        "about phony Santa, Antonio.",
                        "He has been chased by many ",
                        "adventurers but never been",
                        "caught because of his Santa Costume."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Oholy",
                    args![
                        "Maybe his hat and clothes have",
                        "special functions within...",
                        "Maybe that anonymous designer",
                        "still lives in Lutie. Why don't",
                        "you go visit him and ask to",
                        "make you a new Santa Hat?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Oholy", args!["The latest headline by Oholy", "Isn't it amazing?"])?;
                ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
                return ctx.close();
            }
            4 => {
                ctx.lines_as(
                    "Oholy",
                    args![
                        "Oh, dear. I was thinking of",
                        "the sa.m..e... Oops, ho..hoho.",
                        "Oh well, it's not only me. Many",
                        "people think of the same in this Christmas."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Oholy",
                    args![
                        "I heard a strange rumor that",
                        "those people are plotting",
                        "something in this Christmas.",
                        "...hope it goes well(*murmur*)"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Oholy", args!["The latest headline by Oholy", "Isn't it amazing?"])?;
                ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
                return ctx.close();
            }
            _ => {}
        }
    } else if ctx.var("christ_carol05").get()? == 1 {
        ctx.lines_as(
            "Oholy",
            args![
                "He should not be able to escape",
                "from the town. Please find the",
                "devil and bring me back my",
                "Christmas Carol Music Book.",
                "Punish the wicked devil who is ruining Christmas!!!"
            ],
        )?;
        return ctx.close();
    } else if ctx.var("christ_carol05").get()? == 2 {
        if !ctx.call(Function::CheckWeight, args![1201, 1])?.is_true() {
            ctx.lines(args![
                "^3355FFWait a second!",
                "Right now, you're carrying",
                "too many things with you.",
                "Please come back after",
                "using the Kafra Service",
                "to store some of your items.^000000"
            ])?;
            return ctx.close();
        }
        if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 2000 {
            ctx.lines(args![
                "^3355FFWait a second!",
                "Right now, you're carrying",
                "too many things with you.",
                "Please come back after",
                "using the Kafra Service",
                "to store some of your items.^000000"
            ])?;
            return ctx.close();
        }
        if ctx.items().count(1097)? > 0 {
            ctx.lines_as(
                "Oholy",
                args![
                    "Oh, my gracious! ",
                    "You have brought me the book!",
                    "Didn't the devil trouble you?",
                    "I'm glad you have return safely."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Oholy", args!["In return, I'm going to sing", "a carol from the music book."])?;
            ctx.next()?;
            ctx.lines_as(
                "Oholy",
                args![
                    "Hum! Huum!!",
                    "~Sleep well, little children,~",
                    "~wherever you are;~",
                    "~Tomorrow is Christmas~",
                    "~beneath every star.~"
                ],
            )?;
            ctx.call(Function::NpcSpecialEffect, args![constants::EF_GLORIA])?;
            ctx.next()?;
            ctx.lines(args![
                "-Your mind is overwhelmed by her singing-",
                "-You started humming then,-",
                "-began to sing the next phase-"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "~Soon the snowflackes will fall~",
                    "~and tomorrow you'll see~",
                    "~Every wish, one and all,~",
                    "~waiting under the tree.~"
                ],
            )?;
            ctx.call(Function::SpecialEffect, args![constants::EF_GLORIA])?;
            ctx.next()?;
            ctx.lines_as(
                "Oholy",
                args![
                    "Oh, my. You have a wonderful",
                    "voice!! Alright!!!",
                    "I was going to give these",
                    "to kids, but, since you found",
                    "my music book and sang a carol to me!"
                ],
            )?;
            ctx.next()?;
            ctx.lines(args!["-She brought a big sack-", "-and opened it in front of you-"])?;
            ctx.next()?;
            ctx.lines_as(
                "Oholy",
                args!["OK! Don't look inside.", "Just put your hands", "grab what you want."],
            )?;
            ctx.next()?;
            let (item, amount) = match ctx.rand_range(1, 15)? {
                1 => {
                    ctx.lines_as(
                        "Oholy",
                        args![
                            "A Cookie Bag!",
                            "I wrapped those indivisually.",
                            "There are many sweets in them.",
                            "Merry Christmas!"
                        ],
                    )?;
                    (12130, 7)
                }
                2 => {
                    ctx.lines_as(
                        "Oholy",
                        args![
                            "Candies!",
                            "These were made by",
                            "Chief noun.",
                            "Very sweet and delicious.",
                            "Merry Christmas!"
                        ],
                    )?;
                    (529, 20)
                }
                3 => {
                    ctx.lines_as(
                        "Oholy",
                        args![
                            "Candy Canes!",
                            "These were made by",
                            "Chief noun.",
                            "Very sweet and delicious.",
                            "Merry Christmas!"
                        ],
                    )?;
                    (530, 15)
                }
                4 => {
                    ctx.lines_as(
                        "Oholy",
                        args![
                            "A Piece Of Cake!",
                            "These were baked by",
                            "Chief Acolyte.",
                            "Very soft and delicious.",
                            "Merry Christmas!"
                        ],
                    )?;
                    (539, 5)
                }
                5 => {
                    ctx.lines_as(
                        "Oholy",
                        args![
                            "Cookies!",
                            "These were baked by",
                            "Chief Acolyte.",
                            "Very crispy and delicious.",
                            "Merry Christmas!"
                        ],
                    )?;
                    (538, 10)
                }
                6 => {
                    ctx.lines_as(
                        "Oholy",
                        args![
                            "A Spore Doll!",
                            "It's made elaborately by",
                            "Bishop, Tomas.",
                            "Very cute.",
                            "Merry Christmas!"
                        ],
                    )?;
                    (743, 1)
                }
                7 => {
                    ctx.lines_as(
                        "Oholy",
                        args![
                            "A Baphomet Doll!",
                            "..........?!..........",
                            "How did it get in here..?!",
                            "Oops, oh well.",
                            "Merry Christmas!"
                        ],
                    )?;
                    (750, 1)
                }
                8 => {
                    ctx.lines_as(
                        "Oholy",
                        args![
                            "A Osiris Doll!",
                            "..........?!..........",
                            "How did it get in here..?!",
                            "Oops, oh well.",
                            "Merry Christmas!"
                        ],
                    )?;
                    (751, 1)
                }
                9 => {
                    ctx.lines_as(
                        "Oholy",
                        args![
                            "A Rocker Doll!",
                            "This was donated by",
                            "a knight, Lighten.",
                            "Very kind of him.",
                            "Merry Christmas!"
                        ],
                    )?;
                    (752, 1)
                }
                10 => {
                    ctx.lines_as(
                        "Oholy",
                        args![
                            "A Yoyo Doll!",
                            "This was donated by",
                            "an assassin, Marzia.",
                            "Very kind of him.",
                            "Merry Christmas!"
                        ],
                    )?;
                    (753, 1)
                }
                11 => {
                    ctx.lines_as(
                        "Oholy",
                        args![
                            "A Racoon Doll!",
                            "This was donated by",
                            "a hunter, Raiden Kurs.",
                            "Very kind of him.",
                            "Merry Christmas!"
                        ],
                    )?;
                    (754, 1)
                }
                12 => {
                    ctx.lines_as(
                        "Oholy",
                        args![
                            "A Black Cat Doll!",
                            "Sister Magareta found the item",
                            "from the monster, Loli Ruri.",
                            "Very kind of her.",
                            "Hope she is doing okay.",
                            "Merry Christmas!"
                        ],
                    )?;
                    (7206, 1)
                }
                13 => {
                    ctx.lines_as(
                        "Oholy",
                        args!["A Hung Doll!", "I made this doll.", "Isn't it adorable?!", "Merry Christmas!"],
                    )?;
                    (7212, 1)
                }
                14 => {
                    ctx.lines_as(
                        "Oholy",
                        args![
                            "A Munak Doll!",
                            "That is from some country",
                            "across the ocean.",
                            "An artisan made this doll",
                            "with his passion.",
                            "Merry Christmas!"
                        ],
                    )?;
                    (7277, 1)
                }
                15 => {
                    ctx.lines_as(
                        "Oholy",
                        args![
                            "A Santa Hat!",
                            "This is only produced",
                            "during Christmas season.",
                            "It is not a common hat.",
                            "Merry Christmas!"
                        ],
                    )?;
                    (2236, 1)
                }
                _ => return Ok(()),
            };
            ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
            ctx.items().take(1097, 1)?;
            ctx.var("christ_carol05").set(3)?;
            ctx.items().give(item, amount)?;
            return ctx.close();
        }
        ctx.lines_as(
            "Oholy",
            args![
                "Welcome back!! You look good.",
                "2 arms and 2 legs, you look great.",
                "But, where is my music book?!"
            ],
        )?;
        return ctx.close();
    } else {
        ctx.lines_as(
            "Oholy",
            args![
                "Thank you very much.",
                "People and even Devils are",
                "all excited on Christmas day,",
                "so nobody knows what would",
                "happen. Will you help me",
                "then, won't you? Please~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Oholy",
            args!["Let's think about the neighbors", "and do a good deed during Christmas!"],
        )?;
        ctx.var("christ_carol05").set(0)?;
        return ctx.close();
    }
    Ok(())
}

pub fn deviruchi_pron_01(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi2(ctx, args!["prontera", 155, 230])?;
    ctx.end()
}

pub fn deviruchi_pron_01_ontouch(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi(ctx, args!["Deviruchi#pron_01", "Deviruchi#pron_02", "prontera", 155, 230])?;
    ctx.end()
}

pub fn deviruchi_pron_02(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi2(ctx, args!["prontera", 155, 230])?;
    ctx.end()
}

pub fn deviruchi_pron_02_ontouch(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi(ctx, args!["Deviruchi#pron_02", "Deviruchi#pron_03", "prontera", 155, 230])?;
    ctx.end()
}

pub fn deviruchi_pron_03(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi2(ctx, args!["prontera", 155, 230])?;
    ctx.end()
}

pub fn deviruchi_pron_03_ontouch(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi(ctx, args!["Deviruchi#pron_03", "Deviruchi#pron_01", "prontera", 155, 230])?;
    ctx.end()
}

pub fn deviruchi_payon_01(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi2(ctx, args!["payon", 166, 60])?;
    ctx.end()
}

pub fn deviruchi_payon_01_ontouch(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi(ctx, args!["Deviruchi#payon_01", "Deviruchi#payon_02", "payon", 166, 60])?;
    ctx.end()
}

pub fn deviruchi_payon_02(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi2(ctx, args!["payon", 166, 60])?;
    ctx.end()
}

pub fn deviruchi_payon_02_ontouch(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi(ctx, args!["Deviruchi#payon_02", "Deviruchi#payon_03", "payon", 166, 60])?;
    ctx.end()
}

pub fn deviruchi_payon_03(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi2(ctx, args!["payon", 166, 60])?;
    ctx.end()
}

pub fn deviruchi_payon_03_ontouch(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi(ctx, args!["Deviruchi#payon_03", "Deviruchi#payon_01", "payon", 166, 60])?;
    ctx.end()
}

pub fn deviruchi_morocc_01(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi2(ctx, args!["morocc", 160, 51])?;
    ctx.end()
}

pub fn deviruchi_morocc_01_ontouch(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi(ctx, args!["Deviruchi#morocc_01", "Deviruchi#morocc_02", "morocc", 160, 51])?;
    ctx.end()
}

pub fn deviruchi_morocc_02(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi2(ctx, args!["morocc", 160, 51])?;
    ctx.end()
}

pub fn deviruchi_morocc_02_ontouch(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi(ctx, args!["Deviruchi#morocc_02", "Deviruchi#morocc_03", "morocc", 160, 51])?;
    ctx.end()
}

pub fn deviruchi_morocc_03(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi2(ctx, args!["morocc", 160, 51])?;
    ctx.end()
}

pub fn deviruchi_morocc_03_ontouch(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi(ctx, args!["Deviruchi#morocc_03", "Deviruchi#morocc_01", "morocc", 160, 51])?;
    ctx.end()
}

pub fn deviruchi_geffen_01(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi2(ctx, args!["geffen", 120, 34])?;
    ctx.end()
}

pub fn deviruchi_geffen_01_ontouch(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi(ctx, args!["Deviruchi#geffen_01", "Deviruchi#geffen_02", "geffen", 120, 34])?;
    ctx.end()
}

pub fn deviruchi_geffen_02(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi2(ctx, args!["geffen", 120, 34])?;
    ctx.end()
}

pub fn deviruchi_geffen_02_ontouch(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi(ctx, args!["Deviruchi#geffen_02", "Deviruchi#geffen_03", "geffen", 120, 34])?;
    ctx.end()
}

pub fn deviruchi_geffen_03(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi2(ctx, args!["geffen", 120, 34])?;
    ctx.end()
}

pub fn deviruchi_geffen_03_ontouch(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi(ctx, args!["Deviruchi#geffen_03", "Deviruchi#geffen_01", "geffen", 120, 34])?;
    ctx.end()
}

pub fn deviruchi_alberta_01(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi2(ctx, args!["alberta", 28, 235])?;
    ctx.end()
}

pub fn deviruchi_alberta_01_ontouch(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi(ctx, args!["Deviruchi#alberta_01", "Deviruchi#alberta_02", "alberta", 28, 235])?;
    ctx.end()
}

pub fn deviruchi_alberta_02(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi2(ctx, args!["alberta", 28, 235])?;
    ctx.end()
}

pub fn deviruchi_alberta_02_ontouch(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi(ctx, args!["Deviruchi#alberta_02", "Deviruchi#alberta_03", "alberta", 28, 235])?;
    ctx.end()
}

pub fn deviruchi_alberta_03(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi2(ctx, args!["alberta", 28, 235])?;
    ctx.end()
}

pub fn deviruchi_alberta_03_ontouch(ctx: &Ctx) -> Script {
    shared::events_christmas_2005::f_carol_devi(ctx, args!["Deviruchi#alberta_03", "Deviruchi#alberta_01", "alberta", 28, 235])?;
    ctx.end()
}
