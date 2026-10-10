use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum OutFromMonasteryStep {
    Start,
    OnTouch,
    OnInit,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum EmEndStep {
    Start,
    OnInit,
    OnTouch,
    OnTimer4000,
    OnTimer7000,
    OnTimer10000,
    OnTimer15000,
    OnTimer19000,
    OnTimer23000,
    OnTimer30000,
    OnTimer35000,
    OnTimer43000,
    OnTimer47000,
    OnTimer53000,
}

pub(super) fn em_end_run(ctx: &Ctx, mut step: EmEndStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            EmEndStep::Start => {
                step = EmEndStep::OnInit;
                continue 'machine;
            }
            EmEndStep::OnInit => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            EmEndStep::OnTouch => {
                if ctx.var("aru_em").get()? == 22 {
                    ctx.call(Function::InitNpcTimer, vec![])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("#em_end")])?;
                }
                return Err(Stop::End);
            }
            EmEndStep::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("rachel"),
                        Val::from("Pope: Citizens of Arunafeltz. High Priests and Priestesses."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFCE00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            EmEndStep::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("rachel"),
                        Val::from("Pope: I appreciate you all for coming to the Sky Garden as I've asked."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFCE00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            EmEndStep::OnTimer10000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("rachel"),
                        Val::from(
                            "Pope: As the chosen vessel of Goddess Freya, I hereby announce the words I received from her yesterday.",
                        ),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFCE00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            EmEndStep::OnTimer15000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("rachel"),
                        Val::from("Pope: I have observed all that is happening in Arunafeltz."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x66CCCC"),
                    ],
                )?;
                return Err(Stop::End);
            }
            EmEndStep::OnTimer19000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("rachel"), Val::from("Pope: I am aware of the factioning between the High Priests, their selfish fighting, their failure to achieve solidarity."), ctx.constant("BC_MAP")?, Val::from("0x66CCCC")])?;
                return Err(Stop::End);
            }
            EmEndStep::OnTimer23000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("rachel"),
                        Val::from("Pope: However, I did not intervene for the sake of those that still pray for peace in Arunafeltz."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x66CCCC"),
                    ],
                )?;
                return Err(Stop::End);
            }
            EmEndStep::OnTimer30000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("rachel"), Val::from("Pope: High Priest Zhed and High Priestess Niren, both of you must do your best to assist my vessel until the day of my arrival."), ctx.constant("BC_MAP")?, Val::from("0x66CCCC")])?;
                return Err(Stop::End);
            }
            EmEndStep::OnTimer35000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("rachel"), Val::from("Pope: As for the other high priests, you have forgotten your duty to serve me, to enlighten my followers with my teachings. For pursuing your selfish desires, you will be all under Zhed and Niren's command."), ctx.constant("BC_MAP")?, Val::from("0x66CCCC")])?;
                return Err(Stop::End);
            }
            EmEndStep::OnTimer43000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("rachel"),
                        Val::from("Pope: My followers in Arunafeltz, keep your faith in me, and keep your country strong."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x66CCCC"),
                    ],
                )?;
                return Err(Stop::End);
            }
            EmEndStep::OnTimer47000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("rachel"),
                        Val::from("Pope: Then, paradise will surely be yours."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x66CCCC"),
                    ],
                )?;
                return Err(Stop::End);
            }
            EmEndStep::OnTimer53000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("rachel"),
                        Val::from("Citizens: Hail Freya! Hail to the Pope!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x99CC00"),
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("#em_end")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum SuspiciousMan1Step {
    Start,
    OnTimer30000,
    OnInit,
    OnTouch,
    SQuest,
}

pub(super) fn suspicious_man_1_run(ctx: &Ctx, mut step: SuspiciousMan1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SuspiciousMan1Step::Start => {
                if ctx.var("zdan_edq").get()? == 9 {
                    if ctx.var("$@zdan").get()? == 0 {
                        suspicious_man_1_run(ctx, SuspiciousMan1Step::SQuest, vec![])?;
                    } else {
                        ctx.mes("[Suspicious Man]")?;
                        if ctx.call(Function::StrNpcInfo, vec![Val::from(2)])? == "1" {
                            ctx.lines(args![
                                "...............................",
                                "...............................",
                                "............................... "
                            ])?;
                        } else {
                            ctx.mes("... ...")?;
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Something's not quite",
                                "right. I should come back",
                                "and investigate this area",
                                "later when there are fewer",
                                "people watching..."
                            ],
                        )?;
                    }
                } else {
                    if ctx.var("zdan_edq").get()? == 10 {
                        if ctx.var("$@zdan").get()?.number()? > 0 {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Nuts! I was supposed",
                                    "to try to do this secretly!",
                                    "I better try to investigate",
                                    "this area again when no",
                                    "one is around here."
                                ],
                            )?;
                        } else {
                            ctx.var("$@zdan").set(Val::from(1))?;
                            ctx.call(Function::InitNpcTimer, vec![])?;
                            ctx.lines_as("????", args!["Eeek...!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Haha! Got you!", "You're an informer for", "the Z Gang, aren't you?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "????",
                                args!["I... I... d-don't", "know what you're", "talking about!", "I'm innocent!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Then you're telling",
                                    "me this note didn't",
                                    "just fall out of your",
                                    "pocket? What's this",
                                    "about trying to kill me?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("????", args!["Th-that's...", "I'm not--That...!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "You better confess, or",
                                    "I'll drag you over to the",
                                    "Prontera Knightage or the",
                                    "Rogue Guild to take care",
                                    "of you. In fact, let's just",
                                    "head over to Prontera..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Z Gang Informer",
                                args![
                                    "N-no! I'll tell you",
                                    "everything! Please!",
                                    "My mother's old! I've",
                                    "got kids to feed!",
                                    "I... I can't go to jail!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Alright.", "Let's start by you", "telling me where", "I can find the Z Gang."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Z Gang Informer",
                                args![
                                    "I... I really don't",
                                    "know where to find them.",
                                    "I'm at the bottom of the",
                                    "food chain, I just follow",
                                    "their written instructions."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "You know what?",
                                    "Never mind. I'm won't",
                                    "take you to be jailed by",
                                    "the Prontera Knights.",
                                    "I'll drop you off",
                                    "at the Rogues."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Z Gang Informer",
                                args![
                                    "...............................",
                                    "Their secret hideout is in",
                                    "South Morocc, and you can't",
                                    "enter the place without the",
                                    "secret password."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Nice. Now, you better",
                                    "stop running with the",
                                    "Z Gang. Otherwise, I'm",
                                    "not going to be so merciful",
                                    "the next time I see you."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Z Gang Informer", args!["Anything you want!", "J-just let me liiive!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "I should head back to",
                                    "Marybell, and see if she's",
                                    "learned any new information."
                                ],
                            )?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(3127), Val::from(3128)])?;
                            ctx.var("zdan_edq").set(Val::from(11))?;
                            ctx.var("$@zdan").set(Val::from(0))?;
                            ctx.call(Function::StopNpcTimer, vec![])?;
                        }
                    } else {
                        if ctx.var("zdan_edq").get()?.number()? < 9 {
                            if ctx.call(Function::StrNpcInfo, vec![Val::from(2)])? == "1" {
                                ctx.mes("[Suspicious Man]")?;
                            } else {
                                ctx.mes("[Thug]")?;
                            }
                            ctx.lines(args![
                                "What? Get lost!",
                                "Listen, you don't",
                                "want to mess with",
                                "me. Just. Don't."
                            ])?;
                        } else if (ctx.var("zdan_edq").get()?.number()? > 10 && ctx.var("zdan_edq").get()?.number()? < 15) {
                            ctx.lines_as(
                                "Z Gang Informer",
                                args![
                                    "Whoa, leave me alone!",
                                    "I'm just standing here",
                                    "on the road, I didn't",
                                    "do anything wrong!"
                                ],
                            )?;
                        } else if ctx.var("zdan_edq").get()?.number()? > 14 {
                            ctx.lines_as(
                                "Former Z Gang Informer",
                                args![
                                    "You don't have to",
                                    "worry about me anymore.",
                                    "I've turned over a new",
                                    "leaf, got a real job,",
                                    "that sort of deal."
                                ],
                            )?;
                        } else {
                            if ctx.call(Function::StrNpcInfo, vec![Val::from(2)])? == "1" {
                                ctx.mes("[Suspicious Man]")?;
                            } else {
                                ctx.mes("[Thug]")?;
                            }
                            ctx.lines(args![
                                "What? Get lost!",
                                "Listen, you don't",
                                "want to mess with",
                                "me. Just. Don't."
                            ])?;
                        }
                    }
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            SuspiciousMan1Step::OnTimer30000 => {
                ctx.var("$@zdan").set(Val::from(0))?;
                return Err(Stop::End);
            }
            SuspiciousMan1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Suspicious Man#2")])?;
                return Err(Stop::End);
            }
            SuspiciousMan1Step::OnTouch => {
                if (ctx.var("zdan_edq").get()? == 9 && ctx.var("$@zdan").get()? == 0) {
                    suspicious_man_1_run(ctx, SuspiciousMan1Step::SQuest, vec![])?;
                }
                return Err(Stop::End);
            }
            SuspiciousMan1Step::SQuest => {
                ctx.var("$@zdan").set(Val::from(1))?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.lines_as("????", args!["I know you've", "been pursuing us!", "Grrrr... DIE NOW!"])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Wh-who are you?"])?;
                ctx.next()?;
                ctx.lines_as("????", args!["Y-you're stronger", "than I thought!", "Run awaaaaay!"])?;
                if ctx.call(Function::StrNpcInfo, vec![Val::from(2)])? == "1" {
                    ctx.call(Function::DisableNpc, vec![Val::from("Suspicious Man#1")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Suspicious Man#2")])?;
                } else {
                    ctx.call(Function::DisableNpc, vec![Val::from("Suspicious Man#2")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Suspicious Man#1")])?;
                }
                ctx.var("$@zdan").set(Val::from(0))?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.next()?;
                ctx.lines(args![
                    ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("] ")),
                    "Huh? That man must",
                    "have dropped this",
                    "note in his haste",
                    "to get away from here.",
                    "Let's see what it says..."
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    ((Val::from("^666666Kill ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", meow.")),
                    "That arrogant do-gooder",
                    "is looking into us too much.",
                    "Fail to kill him, and death",
                    "will be too good for you, meow.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "That must have been",
                        "an informer for the Z Gang.",
                        "He can't have gotten too far:",
                        "I have a chance to catch him!"
                    ],
                )?;
                ctx.call(Function::ChangeQuest, vec![Val::from(3126), Val::from(3127)])?;
                ctx.var("zdan_edq").set(Val::from(10))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum ZdanBroadStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnTimer3000,
    OnTimer5000,
    OnTimer7000,
    OnTimer9000,
    OnTimer11000,
    OnTimer13000,
    OnTimer15000,
    OnTimer18000,
    OnTimer21000,
    OnTimer300000,
    OnTimer350000,
}

pub(super) fn zdan_broad_run(ctx: &Ctx, mut step: ZdanBroadStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ZdanBroadStep::Start => {
                step = ZdanBroadStep::OnInit;
                continue 'machine;
            }
            ZdanBroadStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#zdan_broad")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ZdanBroadStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("#zdan_broad")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Louis")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Martha")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Catfoii")])?;
                return Err(Stop::End);
            }
            ZdanBroadStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("#zdan_broad")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ZdanBroadStep::OnTimer3000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("z_agit"), Val::from("#ZGuard::OnMyMobDead")],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("z_agit"),
                        Val::from("Catfoii: Err? I heard something, meow! We must be under attack, meow!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ZdanBroadStep::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("z_agit"),
                        Val::from("Louis: Hey, Martha! Are you the one who just got in?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ZdanBroadStep::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("z_agit"),
                        Val::from("Martha: Louis, are you blind? I've been next to you this whole time! We'd better hide first."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ZdanBroadStep::OnTimer9000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("z_agit"),
                        Val::from("Louis: We have an intruder! Hey, Catfoii, what happened? Did you leave the entrance open?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ZdanBroadStep::OnTimer11000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("z_agit"),
                        Val::from("Catfoii: No, impossible, meow~! I have a photographic memory!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ZdanBroadStep::OnTimer13000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("z_agit"), Val::from("Martha: Shut up, both of you! I don't know who you are, but you must have a lot of guts to mess with the Z Gang!"), ctx.constant("BC_MAP")?, Val::from("0xFFFF00")])?;
                return Err(Stop::End);
            }
            ZdanBroadStep::OnTimer15000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("z_agit"),
                        Val::from("Louis: Catfoii, summon the soldiers! This is an emergency! Stop the intruder!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ZdanBroadStep::OnTimer18000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("z_agit"),
                        Val::from("Catfoii: I haven't seen how strong they are, but... Meowkay. Guys, go out and fight!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ZdanBroadStep::OnTimer21000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#ZGuard::OnEnable")])?;
                return Err(Stop::End);
            }
            ZdanBroadStep::OnTimer300000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("z_agit"), Val::from("#ZGuard::OnMyMobDead")],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#zdan_broad::OnDisable")])?;
                ctx.var("$@monster_zgang").set(Val::from(0))?;
                ctx.var("$@door2").set(Val::from(0))?;
                return Err(Stop::End);
            }
            ZdanBroadStep::OnTimer350000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("z_agit"), Val::from("moc_fild17"), Val::from(209), Val::from(235)],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("Louis")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Martha")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Catfoii")])?;
                ctx.var("$@monster_zgang").set(Val::from(0))?;
                ctx.var("$@door2").set(Val::from(0))?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum ZguardStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnReset,
    OnMyMobDead,
    OnTimer300000,
}

pub(super) fn zguard_run(ctx: &Ctx, mut step: ZguardStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ZguardStep::Start => {
                step = ZguardStep::OnInit;
                continue 'machine;
            }
            ZguardStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#ZGuard")])?;
                return Err(Stop::End);
            }
            ZguardStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("#ZGuard")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("z_agit"),
                        Val::from(97),
                        Val::from(78),
                        Val::from("Catfoii's Guard"),
                        Val::from(1479),
                        Val::from(1),
                        Val::from("#ZGuard::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("z_agit"),
                        Val::from(98),
                        Val::from(79),
                        Val::from("Catfoii's Guard"),
                        Val::from(1479),
                        Val::from(1),
                        Val::from("#ZGuard::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("z_agit"),
                        Val::from(96),
                        Val::from(81),
                        Val::from("Catfoii's Guard"),
                        Val::from(1523),
                        Val::from(1),
                        Val::from("#ZGuard::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ZguardStep::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("z_agit"), Val::from("#ZGuard::OnMyMobDead")],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ZguardStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("z_agit"), Val::from("#ZGuard::OnMyMobDead")],
                )?;
                ctx.var("$@monster_zgang").set(Val::from(0))?;
                return Err(Stop::End);
            }
            ZguardStep::OnMyMobDead => {
                if ctx
                    .call(Function::MobCount, vec![Val::from("z_agit"), Val::from("#ZGuard::OnMyMobDead")])?
                    .number()?
                    < 1
                {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "You cowardly Z Gang!",
                            "Come out and surrender!",
                            "I've defeated your monster",
                            "soldiers already!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Martha", args!["What should we do?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Catfoii",
                        args!["This is our greatest", "crisis ever! I don't", "know what to do, meow!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Louis",
                        args![
                            "The Z Gang's not going",
                            "to surrender yet! Come",
                            "forth, my loyal servants",
                            "of the darkness!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Catfoii", args!["No-no-no-meow!", "Not that button!"])?;
                    ctx.next()?;
                    ctx.lines_as("Louis", args!["Huh? Why...?"])?;
                    ctx.next()?;
                    ctx.var("zdan_edq").set(Val::from(17))?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#ZGuard::OnDisable")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Louis")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Martha")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Catfoii")])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
            ZguardStep::OnTimer300000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("z_agit"), Val::from("moc_fild17"), Val::from(209), Val::from(235)],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#zdan_broad::OnDisable")])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("z_agit"), Val::from("#ZGuard::OnMyMobDead")],
                )?;
                ctx.var("$@monster_zgang").set(Val::from(0))?;
                ctx.var("$@door2").set(Val::from(0))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#ZGuard::OnDisable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}
