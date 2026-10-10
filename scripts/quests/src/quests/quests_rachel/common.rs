use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum ManStuckInIceCaveStep {
    Start,
    OnTouch,
    OnTouchNPC,
    OnMyMobDead,
}

pub(super) fn man_stuck_in_ice_cave_run(ctx: &Ctx, mut step: ManStuckInIceCaveStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ManStuckInIceCaveStep::Start => {
                if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(200)])? == 0 {
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
                if ctx.var("ice_necklace_q").get()? == 1 {
                    ctx.call(Function::Cutin, vec![Val::from("ra_magic3"), Val::from(2)])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                    ctx.lines_as(
                        "Man Stuck in Ice",
                        args![
                            "H-hello?",
                            "Hey! Hey, you!",
                            "Help me break this",
                            "ice! I need to get",
                            "out of here!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["How did you get", "stuck in there?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Man Stuck in Ice",
                        args!["I'll explain everything", "later! Just... Just get", "this ice off of me!."],
                    )?;
                    ctx.next()?;
                    if ctx.call(Function::GetSkillLv, vec![Val::from("MG_FIREBOLT")])?.number()? > 0 {
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FIREBALL")?])?;
                        ctx.lines(args!["^3355FFYou cast Fire Bolt at", "the ice..^000000"])?;
                    } else {
                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
                        ctx.lines(args!["^3355FFYou hammer at the", "ice with all your might.^000000"])?;
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["......", "........"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I can't...", "I can't even scratch it", "Do you have any ideas?"],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                    ctx.lines_as(
                        "Man Stuck in Ice",
                        args![
                            "This is so humiliating...",
                            "Me, the greatest mage",
                            "of our age, Maheo, stuck",
                            "in this pillar of ice."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Wait...", "You're Maheo?"],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_magic1"), Val::from(2)])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_KIK")?])?;
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "It's true. You're speaking to",
                            "Maheo, the greatest mage,",
                            "and master of arcane spells.",
                            "I know magic that even High",
                            "Wizards can never hope to",
                            "learn in their lifetimes!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_magic4"), Val::from(2)])?;
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "Despite my greatness,",
                            "I'm a humble man. See?",
                            "That's why I always wear",
                            "this Mage uniform...",
                            "To remind myself of",
                            "the value of humility."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["So how did you get", "stuck in all of this ice?"],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_magic3"), Val::from(2)])?;
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "Actually, this happened",
                            "because I was too humble",
                            "You see, I underestimated",
                            "myself, and the devastating",
                            "force of my own magic."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...Huh?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "Yes, this wouldn't have",
                            "happened if I wasn't so",
                            "humble... Humble and kind.",
                            "It all started when I thought",
                            "of this cave and how people",
                            "sometimes come here to get ice."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "I then decided that",
                            "I would exterminate these",
                            "evil monsters for the good",
                            "of the people! The citizens",
                            "would feel protected, and",
                            "I'd be recognized as a hero!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Okay... I suppose", "that sounds normal", "enough. Go on."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "I valiantly battled",
                            "the Snowiers. They were",
                            "no match for my magic!",
                            "And so, I decided to just",
                            "destroy all of them with",
                            "one cast of a magic spell."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "You know Meteor Storm?",
                            "I know another spell like",
                            "that... But it's two hundred",
                            "times more powerful! Yes...",
                            "It has the power of a million",
                            "exploding suns! But then..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "It was too powerful!",
                            "There were tremors, and",
                            "flying shards of ice, and",
                            "all the flame caused my",
                            "clothes to catch on fire!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_SPARK")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["You...", "you set fire", "to your clothes"],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_magic2"), Val::from(2)])?;
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "Yes, but not to worry.",
                            "I quickly extinguished",
                            "those flames with my",
                            "powerful Frost Diver spell!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_magic3"), Val::from(2)])?;
                    ctx.lines_as("Maheo", args!["In hindsight...", "That may have", "been a mistake..."])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_magic3"), Val::from(2)])?;
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "Enough about myself.",
                            "What noble pursuit brings",
                            "you to this place, adventurer?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "I heard that you can",
                            "polish the gems on this",
                            "necklace with your magic,",
                            "so I came here to find you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "Verily, I can shine",
                            "those gems so that they",
                            "shine as brightly as a",
                            "million exploding suns!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "That is, as soon",
                            "as I can get out",
                            "of this ice. Hmm...",
                            "But I doubt normal",
                            "magic will be able",
                            "to melt all of this."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Then how are we", "supposed to get", "you out of there?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "Fear not. I'm sure",
                            "that my master will know",
                            "of a way to free me from",
                            "this prison of ice. He can",
                            "be found near Freya's Spring:",
                            "beseech him on my behalf!"
                        ],
                    )?;
                    ctx.var("ice_necklace_q").set(Val::from(2))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2109), Val::from(2110)])?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                } else if (ctx.var("ice_necklace_q").get()? == 2 || ctx.var("ice_necklace_q").get()? == 3) {
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "My master may not have",
                            "my sheer talent, but he",
                            "is very knowledgable in",
                            "the ways of magic. Please...",
                            "Ask him for help. He should",
                            "be reading near Freya's Spring."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("ice_necklace_q").get()? == 4 {
                    if ctx.call(Function::CountItem, vec![Val::from(7569)])?.number()? > 0 {
                        if ctx.call(Function::CountItem, vec![Val::from(7572)])?.number()? > 0 {
                            ctx.lines_as(
                                "Maheo",
                                args![
                                    "Oh, you're back!",
                                    "So did my master have",
                                    "any ideas on breaking",
                                    "this cold prison of ice?"
                                ],
                            )?;
                            ctx.next()?;
                        } else {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Oh, shoot! I left the necklace in the city! I will be right back!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Maheo", args!["Hey, hey! Can't you just release me first?"])?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        ctx.lines_as(
                            "Maheo",
                            args![
                                "My master may not have",
                                "my sheer talent, but he",
                                "is very knowledgable in",
                                "the ways of magic. Please...",
                                "Ask him for help. He should",
                                "be reading near Freya's Spring."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Well, he made this",
                            "magic hammer which is",
                            "supposed to be able to",
                            "break this magic ice."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "Of course!",
                            "Why didn't I think of",
                            "that? Great, now get",
                            "me out of here please!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou tightly gripped",
                        "the Wind Hammer, and",
                        "swung it down at the ice",
                        "with all of your strength.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_TEIHIT3")?])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FREEZE")?])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_ICECRASH")?])?;
                    ctx.lines(args!["^3355FF*Pzzzzz*", "*CRASH!*^000000"])?;
                    ctx.next()?;
                    ctx.call(
                        Function::SetNpcDisplay,
                        vec![Val::from("Man Stuck in Ice#cave"), Val::from(937)],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                    ctx.lines_as("Maheo", args!["Finally...!", "After all of this", "time! I'm free!"])?;
                    ctx.next()?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL4")?])?;
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "Now, all of the monsters",
                            "in this cave will taste the",
                            "wrath of the greatest mage in",
                            "the world! I'll have my revenge,",
                            "and give those beasts double",
                            "the pain that they gave me!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("ice_dun02"),
                            Val::from(108),
                            Val::from(109),
                            Val::from("Snowier"),
                            Val::from(1775),
                            Val::from(1),
                            Val::from("Man Stuck in Ice#cave::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("ice_dun02"),
                            Val::from(114),
                            Val::from(112),
                            Val::from("Snowier"),
                            Val::from(1775),
                            Val::from(1),
                            Val::from("Man Stuck in Ice#cave::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("ice_dun02"),
                            Val::from(126),
                            Val::from(105),
                            Val::from("Snowier"),
                            Val::from(1775),
                            Val::from(1),
                            Val::from("Man Stuck in Ice#cave::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("ice_dun02"),
                            Val::from(121),
                            Val::from(99),
                            Val::from("Snowier"),
                            Val::from(1775),
                            Val::from(1),
                            Val::from("Man Stuck in Ice#cave::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LORD")?])?;
                    ctx.call(
                        Function::KillMonster,
                        vec![Val::from("ice_dun02"), Val::from("Man Stuck in Ice#cave::OnMyMobDead")],
                    )?;
                    ctx.lines_as("Maheo", args!["Muhahahahahahahaha!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maheo",
                        args!["Er, but first, I need to", "use my magic to clean", "that necklace of yours."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Here..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "Oh! This was made by",
                            "the Dwarves, wasn't it?",
                            "It looks like they've made",
                            "yet another masterpiece.",
                            "Shame that this is so",
                            "tarnished, though."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Maheo", args!["Let's see, now..."])?;
                    ctx.next()?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FROSTWEAPON")?])?;
                    ctx.mes("^3355FF*Ting*^000000")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maheo",
                        args![
                            "Well, I suppose this",
                            "is where we part ways.",
                            "Here, take this as a gift...",
                            "And please don't mention",
                            "the fact that I trapped myself",
                            "in ice to anyone else, okay?"
                        ],
                    )?;
                    ctx.var("ice_necklace_q").set(Val::from(5))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2112), Val::from(2113)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7569), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7572), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(7573), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(7574), Val::from(4)])?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    ctx.call(
                        Function::SetNpcDisplay,
                        vec![Val::from("Man Stuck in Ice#cave"), Val::from(924)],
                    )?;
                    return Err(Stop::End);
                }
                step = ManStuckInIceCaveStep::OnTouch;
                continue 'machine;
            }
            ManStuckInIceCaveStep::OnTouch => {
                return Err(Stop::End);
            }
            ManStuckInIceCaveStep::OnTouchNPC => {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                ctx.call(
                    Function::Emotion,
                    vec![ctx.constant("ET_KIK")?, ctx.call(Function::GetCharacterId, vec![Val::from(3)])?],
                )?;
                return Err(Stop::End);
            }
            ManStuckInIceCaveStep::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum CaveVosStep {
    Start,
    OnInit,
    OnTimer3600000,
    OnTimer7200000,
    OnTimer10800000,
}

pub(super) fn cave_vos_run(ctx: &Ctx, mut step: CaveVosStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            CaveVosStep::Start => {
                step = CaveVosStep::OnInit;
                continue 'machine;
            }
            CaveVosStep::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            CaveVosStep::OnTimer3600000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ice_dun02"),
                        Val::from("Go away, you animals! I'll burn you to death once I get free!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            CaveVosStep::OnTimer7200000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ice_dun02"),
                        Val::from("Hello? Can anyone hear me? I'm... I'm kind of stuck!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            CaveVosStep::OnTimer10800000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ice_dun02"),
                        Val::from("Please! I can't move! Hello? I think I might need help!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(16764416),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum IceBossBroadStep {
    Start,
    OnStop,
    OnStart,
    OnTimer2000,
    OnTimer8000,
    OnTimer10000,
    OnTimer13000,
    OnTimer16000,
    OnTimer19000,
    OnTimer21000,
}

pub(super) fn ice_boss_broad_run(ctx: &Ctx, mut step: IceBossBroadStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            IceBossBroadStep::Start => {
                step = IceBossBroadStep::OnStop;
                continue 'machine;
            }
            IceBossBroadStep::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            IceBossBroadStep::OnStart => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            IceBossBroadStep::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ice_dun03"),
                        Val::from("Someone has put out Thor's flames... Infidel!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(16737843),
                    ],
                )?;
                return Err(Stop::End);
            }
            IceBossBroadStep::OnTimer8000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ice_dun03"),
                        Val::from("I, Ktullanux, must protect and preserve Thor's fierce flames..."),
                        ctx.constant("BC_MAP")?,
                        Val::from(16737843),
                    ],
                )?;
                return Err(Stop::End);
            }
            IceBossBroadStep::OnTimer10000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ice_dun03"),
                        Val::from("As the master of this cave, I vow vengeance!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(16737843),
                    ],
                )?;
                return Err(Stop::End);
            }
            IceBossBroadStep::OnTimer13000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ice_dun03"),
                        Val::from("Only a human would be so foolhardy...."),
                        ctx.constant("BC_MAP")?,
                        Val::from(16737843),
                    ],
                )?;
                return Err(Stop::End);
            }
            IceBossBroadStep::OnTimer16000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ice_dun03"),
                        Val::from("Your curiosity will cost you, human."),
                        ctx.constant("BC_MAP")?,
                        Val::from(16737843),
                    ],
                )?;
                return Err(Stop::End);
            }
            IceBossBroadStep::OnTimer19000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ice_dun03"),
                        Val::from("Prepare yourself for a freezing realm of pain which you cannot possibly imagine!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(16737843),
                    ],
                )?;
                return Err(Stop::End);
            }
            IceBossBroadStep::OnTimer21000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ice_boss#on::OnStart")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum IceBossOnStep {
    Start,
    OnStart,
    OnStartTimer,
    OnStopTimer,
    OnMyMobDead,
    OnTimer7200000,
}

pub(super) fn ice_boss_on_run(ctx: &Ctx, mut step: IceBossOnStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            IceBossOnStep::Start => {
                step = IceBossOnStep::OnStart;
                continue 'machine;
            }
            IceBossOnStep::OnStart => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ice_dun03"),
                        Val::from(150),
                        Val::from(135),
                        Val::from("Ktullanux"),
                        Val::from(1779),
                        Val::from(1),
                        Val::from("ice_boss#on::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            IceBossOnStep::OnStartTimer => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            IceBossOnStep::OnStopTimer => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            IceBossOnStep::OnMyMobDead => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ice_dun03"),
                        Val::from("Oh, Odin! Please protect this place from Thor's fierce fire!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(16737843),
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ice_dun03"),
                        Val::from("Pzzzzz...Pzzzz..."),
                        ctx.constant("BC_MAP")?,
                        Val::from(3407871),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ice_boss#on::OnStartTimer")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#ice_sec::OnStart")])?;
                return Err(Stop::End);
            }
            IceBossOnStep::OnTimer7200000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.var("$@ktullanux_summon").set(Val::from(0))?;
                ctx.call(Function::EnableNpc, vec![Val::from("Blazing Fire#ice1")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Blazing Fire#ice2")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Blazing Fire#ice3")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Blazing Fire#ice4")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum IceSecStep {
    Start,
    OnStart,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ice4f1Step {
    Start,
    OnTouch,
    OnInit,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ice4f2Step {
    Start,
    OnTouch,
    OnInit,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ice4f3Step {
    Start,
    OnTouch,
    OnInit,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ice4f4Step {
    Start,
    OnTouch,
    OnInit,
}
