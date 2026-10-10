use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum PrtcurseStep {
    Start,
    OnTouch,
}

fn prtcurse_run(ctx: &Ctx, mut step: PrtcurseStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PrtcurseStep::Start => {
                step = PrtcurseStep::OnTouch;
                continue 'machine;
            }
            PrtcurseStep::OnTouch => {
                if ctx.var("prt_curse").get()? == 25 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Hm. I'd better review",
                            "the facts I've learned so",
                            "that I can better focus on",
                            "this investigation. Let's see~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Long ago, the giant serpent",
                            "Jormungand threatened mankind.",
                            "7 warriors defeated Jormungand, led by Tristram III of the Geoborg",
                            "family, but Jormungand cursed the Geoborg bloodline in its defeat."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Ever since, the curse kills",
                            "the first born prince of the",
                            "Geoborg family at an early age.",
                            "However, all of the princes of",
                            "this generation were killed."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "According to what I've",
                            "learned from that assassin,",
                            "the first prince died from the",
                            "curse, and the other two may",
                            "have died from poisoning."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "It's most likely that if",
                            "poison was used, then the",
                            "person who used it was an",
                            "assassin from outside of the",
                            "Rune-Midgarts Kingdom. Yes,",
                            "that's about everything I know."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Now, that historian Rodafrian",
                            "has been waiting for me to tell",
                            "her the lyrics of that song, but Father Bamph is also waiting",
                            "for the info I've learned from the Assassin Guild. What should I do?"
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Go to Rodafrian:Go to Father Bamph")])?) == 1 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Well, it's easier for me",
                                "to visit Rodafrian now.",
                                "She's much closer than",
                                "Father Bamph, so I guess",
                                "that I'll go talk to her first."
                            ],
                        )?;
                        ctx.var("prt_curse").set(Val::from(30))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Well, it's more important",
                            "that I go see Father Bamph",
                            "and investigate the princes'",
                            "bodies. I better head over",
                            "to Prontera right away."
                        ],
                    )?;
                    ctx.var("prt_curse").set(Val::from(50))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn prtcurse(ctx: &Ctx) -> Script {
    prtcurse_run(ctx, PrtcurseStep::Start, Vec::new()).map(|_| ())
}

pub fn prtcurse_ontouch(ctx: &Ctx) -> Script {
    prtcurse_run(ctx, PrtcurseStep::OnTouch, Vec::new()).map(|_| ())
}

fn librarian_curse_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("prt_curse").get()? == 3 {
        if ctx.call(Function::CountItem, vec![Val::from(7431)])?.number()? < 1 {
            ctx.lines_as(
                "Librarian",
                args![
                    "Please make sure to return",
                    "library books to the correct",
                    "place after you use them.",
                    "We don't have enough staff",
                    "to organize all these books..."
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Report the loss of the books:End Conversation")],
            )?) == 1
            {
                ctx.lines_as(
                    "Librarian",
                    args![
                        "Oh, you lost some books?",
                        "Please write down the book",
                        "titles and pay the 700 zeny",
                        "penalty charge. Aftewards,",
                        "we will provide you with",
                        "replacement copies."
                    ],
                )?;
                ctx.next()?;
                if ctx.var("Zeny").get()?.number()? > 699 {
                    ctx.lines_as(
                        "Librarian",
                        args![
                            "Ah, here you are.",
                            "Please take these",
                            "replacement copies,",
                            "and try not to lose",
                            "them again. Thank you."
                        ],
                    )?;
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(700))?))?;
                    ctx.call(Function::GetItem, vec![Val::from(7431), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Librarian",
                    args![
                        "Hmm...",
                        "Come back as soon",
                        "as you can with the",
                        "700 zeny to pay the",
                        "lost book penalty charge."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Librarian",
                args!["Please keep silent", "while inside the library.", "Thank you for cooperating."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Librarian",
                args![
                    "Being a librarian can",
                    "be pretty rough. People",
                    "just leave the books all",
                    "scattered, but expect them",
                    "to be organized. Ooh, books",
                    "are always getting lost too..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Librarian",
                args![
                    "When you have",
                    "to clean up their",
                    "messes, you get to",
                    "realize how sloppy",
                    "people can really be."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Librarian",
            args![
                "Being a librarian can",
                "be pretty rough. People",
                "just leave the books all",
                "scattered, but expect them",
                "to be organized. Ooh, books",
                "are always getting lost too..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Librarian",
            args![
                "When you have",
                "to clean up their",
                "messes, you get to",
                "realize how sloppy",
                "people can really be."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn librarian_curse(ctx: &Ctx) -> Script {
    librarian_curse_body(ctx, Vec::new()).map(|_| ())
}
