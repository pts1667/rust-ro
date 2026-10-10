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

fn has_advanced_job(ctx: &Ctx) -> Result<bool, Stop> {
    Ok(ctx.var("advjob").get()? != 0 || ctx.var("Upper").get()? == 1)
}

fn is_eligible(ctx: &Ctx) -> Result<bool, Stop> {
    Ok(ctx.player().base_level()? > 98
        && ctx.player().job_level()? > 49
        && ctx.player().class()? >= constants::JOB_KNIGHT
        && ctx.player().class()? <= constants::JOB_CRUSADER2)
}

pub fn valkyrie(ctx: &Ctx) -> Script {
    if has_advanced_job(ctx)? {
        ctx.lines_as("Valkyrie", args!["Welcome", "to Valhalla,", "the Hall of Honor."])?;
        ctx.next()?;
        ctx.lines_as(
            "Valkyrie",
            args![
                "Please make",
                "yourself comfortable",
                "while you are here.",
                "Honor to the warriors!"
            ],
        )?;
        return ctx.close();
    }
    if !is_eligible(ctx)? {
        ctx.lines_as("Valkyrie", args!["Welcome", "to Valhalla,", "the Hall of Honor."])?;
        ctx.next()?;
        ctx.lines_as(
            "Valkyrie",
            args!["Unfortunately, you have not yet been invited here. I ask you to leave immediately. Honor to the warriors!"],
        )?;
        ctx.close_window()?;
        ctx.warp("yuno_in02", 93, 205)?;
        return ctx.end();
    }

    ctx.lines_as("Valkyrie", args!["Welcome", "to Valhalla,", "the Hall of Honor."])?;
    ctx.next()?;
    ctx.lines_as(
        "Valkyrie",
        args![
            "You will now end",
            "your present life and",
            "begin an entirely new life.",
            "Honor to the warriors!"
        ],
    )?;
    ctx.next()?;
    if ctx.var("Weight").get()?.number()? > 0
        || ctx.player().zeny()? > 0
        || ctx.call(Function::CheckCart, vec![])? != 0
        || ctx.call(Function::CheckFalcon, vec![])? != 0
        || ctx.call(Function::CheckRiding, vec![])? != 0
    {
        ctx.lines_as(
            "Valkyrie",
            args![
                "There are a few things you must",
                "do before we start. You must",
                "first empty your mind and body.",
                "Honor comes when you abandon",
                "all your selfish desires..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Valkyrie",
            args!["You cannot take anything with you to the next life. Your items, zeny, pets and Pushcart all have to be left behind."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Valkyrie",
            args!["When you are ready", "please return to me,", "brave adventurer."],
        )?;
        ctx.close_window()?;
        ctx.warp("yuno_in02", 93, 205)?;
        return ctx.end();
    }

    ctx.lines_as(
        "Valkyrie",
        args![
            "I see you've already",
            "released yourself from",
            "all worldy attachments,",
            ctx.player().name()? + "."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Valkyrie", args!["That's an admirable attitude for an adventurer such as yourself. Honor comes when you abandon all personal desires for the sake of mankind."])?;
    ctx.next()?;
    if ctx.var("SkillPoint").get()?.is_true() {
        ctx.lines_as(
            "Valkyrie",
            args![
                "Hmm... I sense that you have",
                "some lingering attachment or",
                "unfinished business in your",
                "current life. Take care of that,",
                "and bring closure to your present life."
            ],
        )?;
        ctx.close_window()?;
        ctx.warp("yuno_in02", 93, 205)?;
        return ctx.end();
    }
    ctx.lines_as(
        "Valkyrie",
        args![
            "Now, let me remove all",
            "of your present memories...",
            "However, you will be able to",
            "remember the most honorable",
            "moments of this life."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Valkyrie",
        args![
            "With one,",
            "I will ask the",
            "goddess Urd to remove",
            "all of your present",
            "memories."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Valkyrie",
        args![
            "With two,",
            "I will ask the",
            "goddess Verdandi to keep",
            "and record the most honorable moments of your present life."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Valkyrie",
        args![
            "With three,",
            "I will ask the",
            "goddess Skuld to",
            "guide you to your",
            "next life."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Valkyrie", args!["One..."])?;
    shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
    ctx.next()?;
    ctx.lines_as("Valkyrie", args!["One...", "Two......"])?;
    ctx.next()?;
    ctx.lines_as("Valkyrie", args!["One...", "Two......", "And Three."])?;
    ctx.var("advjob")
        .set(Val::from(ctx.player().class()? + constants::JOB_NOVICE_HIGH))?;
    if ctx.var("advjob").get()? == constants::JOB_LORD_KNIGHT2 {
        ctx.var("advjob").set(Val::from(constants::JOB_LORD_KNIGHT))?;
    }
    if ctx.var("advjob").get()? == constants::JOB_PALADIN2 {
        ctx.var("advjob").set(Val::from(constants::JOB_PALADIN))?;
    }
    ctx.call(Function::JobChange, args![constants::JOB_NOVICE_HIGH])?;
    ctx.call(Function::ResetLevel, args![1])?;
    ctx.var("misc_quest")
        .set(Val::from(ctx.var("misc_quest").get()?.number()? & !1024))?;
    ctx.call(Function::Skill, args!["NV_FIRSTAID", 1, constants::SKILL_PERM])?;
    ctx.call(Function::Skill, args!["NV_TRICKDEAD", 1, constants::SKILL_PERM])?;
    ctx.quests().complete(1000)?;
    ctx.next()?;
    ctx.lines_as(
        "Valkyrie",
        args![
            "Congratulations.",
            "You are now reborn",
            "into a brand new life.",
            "Please take these small gifts",
            "in preparation for your new adventures."
        ],
    )?;
    ctx.items().give(1202, 1)?;
    ctx.items().give(2302, 1)?;
    ctx.next()?;
    ctx.lines_as("Valkyrie", args!["I wish that the release the goddess Urd has granted you proves to be a blessing. I hope that the memories Verdandi has recorded will always honor you."])?;
    ctx.next()?;
    ctx.lines_as(
        "Valkyrie",
        args!["And I pray that the new life to which the goddess Skuld will guide you will be even more honorable than your last."],
    )?;
    ctx.close_window()?;
    let (map, x, y) = match ctx.var("advjob").get()?.number()? {
        4008 | 4015 => ("izlude", 94, 103),
        4009 | 4016 => ("prontera", 273, 354),
        4010 | 4017 => ("geffen", 120, 60),
        4011 | 4019 => ("alberta", 116, 57),
        4012 | 4020 | 4021 => ("payon", 69, 100),
        4013 | 4018 => ("morocc", 154, 50),
        _ => ("yuno_in02", 93, 205),
    };
    ctx.warp(map, x, y)?;
    ctx.end()
}

pub fn metheus_sylphe_library(ctx: &Ctx) -> Script {
    if !is_eligible(ctx)? {
        ctx.lines_as(
            "Metheus Sylphe",
            args![
                "Welcome to the Library of the Schweicherbil Magic Academy.",
                "Here, we have a countless number of books. Please take your time and look around."
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("valkyrie_q").get()? != 0 {
        ctx.lines_as(
            "Metheus Sylphe",
            args![
                "Once again, thank you for your generous donation. Feel free to read a carbon copy of the 'Book of Ymir' at your leisure."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Metheus Sylphe",
        args![
            "Welcome to the Library of the Schweicherbil Magic Academy.",
            "I assume you have come here",
            "to read the 'Book of Ymir.'"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Metheus Sylphe", args!["Unfortunately, the original copy of the book has been damaged over time. We currently only allow the public to view a copy of the book."])?;
    ctx.next()?;
    ctx.lines_as("Metheus Sylphe", args!["Also, in order to preserve the original 'Book of Ymir,' we have decided to accept donations from people who wish to read the copy we have provided."])?;
    ctx.next()?;
    ctx.lines_as(
        "Metheus Sylphe",
        args!["The suggested", "donation amount is", "1,285,000 zeny."],
    )?;
    ctx.next()?;
    if ctx.menu(&["Donate.", "Cancel."])? == 0 {
        if ctx.player().zeny()? >= 1285000 {
            ctx.player().set_zeny(ctx.player().zeny()? - 1285000)?;
            ctx.var("valkyrie_q").set(Val::from(1))?;
            ctx.lines_as(
                "Metheus Sylphe",
                args![
                    "Thank you, your donation will be used for a good cause. You may",
                    "now go in and read the book."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Metheus Sylphe",
            args!["Unfortunately, you don't seem to possess enough zeny at the moment. Please check your funds and come back again."],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Metheus Sylphe", args!["Take your time, and", "enjoy your travels."])?;
    ctx.close()
}

pub fn book_of_ymir(ctx: &Ctx) -> Script {
    if has_advanced_job(ctx)? {
        ctx.lines_as("The Book of Ymir", args!["...The entrance to the Hall of Honor is open to anyone who has moved forward into their next life. It is there to help heroes decide what they want to do, and can lead them to anywhere in this world."])?;
        ctx.next()?;
        ctx.lines_as("The Book of Ymir", args!["In the Hall of Honor, everything is prepared for heroes. It is rumored that any wish that cannot be fulfilled in our reality can be realized in the Hall of Honor."])?;
        ctx.next()?;
        if ctx.menu(&["Stop reading.", "Continue reading."])? == 0 {
            ctx.lines_as("The Book of Ymir", args!["....."])?;
            return ctx.close();
        }
        ctx.lines_as("The Book of Ymir", args!["There is a forgotten path which leads to the Hall of Honor, the closest place to the heavens. The ordinary will never discover this place..."])?;
        ctx.close_window()?;
        ctx.warp("valkyrie", 48, 8)?;
        return ctx.end();
    }
    if is_eligible(ctx)? && ctx.var("valkyrie_q").get()? != 0 {
        ctx.lines_as(
            "The Book of Ymir",
            args![
                "...Therefore, ancient heroes were",
                "always in anguish, knowing that",
                "eventually, they were mortal and",
                "would pass from this realm..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "The Book of Ymir",
            args![
                "There were no documents,",
                "songs, or remaining folklore that had any information on life after death. However, I recently uncovered an old scroll",
                "about Valkyrie..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "The Book of Ymir",
            args!["Valkyrie...", "The legendary", "guardian angel.", "Angel of Ragnarok."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "The Book of Ymir",
            args![
                "Adventurers of great strength",
                "and bravery will be lead by",
                "Valkyrie to Valhalla, the Hall",
                "of Honor. There, they will be",
                "given a new life."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "The Book of Ymir",
            args![
                "Reborn, they will live again as",
                "even greater heroes that will",
                "brighten the world. Bodies that",
                "were exhausted will be filled",
                "with energy..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("The Book of Ymir", args!["And their souls will be given abilities with the heart of Ymir. However, the heart of Ymir was totally destroyed and scattered all over the world after the battle for Rune-Midgarts."])?;
        ctx.next()?;
        ctx.lines_as(
            "The Book of Ymir",
            args![
                "I have found a small amount of",
                "Ymir heart pieces over a long",
                "long period of time. But I can't",
                "confirm if the story of Valkyrie",
                "and Valhalla is true just",
                "through scientific tests."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "The Book of Ymir",
            args![
                "So, I am leaving this record in hope that someone in the future",
                "can confirm that Valkyrie and Valhalla actually exist..."
            ],
        )?;
        ctx.next()?;
        ctx.var("valkyrie_q").set(Val::from(2))?;
        if ctx.call(Function::CheckQuest, args![1000])? == -1 {
            ctx.quests().start(1000)?;
        }
        ctx.lines_as(
            "The Book of Ymir",
            args![
                "Let the heroes live new lives",
                "so they can protect the world",
                "from danger. And then..."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("The Book of Ymir", args!["..."])?;
    ctx.close()
}

pub fn heart_of_ymir(ctx: &Ctx) -> Script {
    if is_eligible(ctx)? && ctx.var("valkyrie_q").get()? == 2 {
        ctx.warp("valkyrie", 48, 8)?;
    }
    ctx.end()
}

#[derive(Clone, Copy, Debug)]
enum TeleporterStep {
    Start,
    Warp,
}

fn teleporter_run(ctx: &Ctx, mut step: TeleporterStep, mut args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TeleporterStep::Start => {
                if ctx.var("Upper").get()? != 1 {
                    if ctx.rand_range(1, 10)? > 4 {
                        ctx.lines_as("Teleporter", args!["Congratulations.", "Honor to the warriors!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Teleporter",
                        args!["Please refrain", "from touching any", "of the exhibitions.", ".........."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Teleporter", args!["Honorable one,", "which place do you wish to go?"])?;
                ctx.next()?;
                args = match ctx.menu(&[
                    "Prontera",
                    "Morocc",
                    "Payon",
                    "Geffen",
                    "Alberta",
                    "Izlude",
                    "Al De Baran",
                    "Comodo",
                    "Juno",
                ])? {
                    0 => args!["prontera", 116, 72],
                    1 => args!["morocc", 156, 46],
                    2 => args!["payon", 69, 100],
                    3 => args!["geffen", 120, 39],
                    4 => args!["alberta", 117, 56],
                    5 => args!["izlude", 94, 103],
                    6 => args!["aldebaran", 91, 105],
                    7 => args!["comodo", 209, 143],
                    _ => args!["yuno", 328, 101],
                };
                step = TeleporterStep::Warp;
                continue 'machine;
            }
            TeleporterStep::Warp => {
                ctx.lines_as("Teleporter", args!["Have a nice trip."])?;
                ctx.close_window()?;
                ctx.call(
                    Function::SavePoint,
                    args![
                        runtime::arg(&args, 0, Val::from(0)),
                        runtime::arg(&args, 1, Val::from(0)),
                        runtime::arg(&args, 2, Val::from(0)),
                        1,
                        1
                    ],
                )?;
                ctx.call(
                    Function::Warp,
                    args![
                        runtime::arg(&args, 0, Val::from(0)),
                        runtime::arg(&args, 1, Val::from(0)),
                        runtime::arg(&args, 2, Val::from(0))
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn teleporter(ctx: &Ctx) -> Script {
    teleporter_run(ctx, TeleporterStep::Start, Vec::new()).map(|_| ())
}
