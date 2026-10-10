use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn bankley_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("tu_swordman").get()?.number()? > 19 {
        ctx.lines(args!["^3355FFBankley had a pitiable", "expression on his face.^000000", "^3355FFHis body is completely lifeless. There's a long, deep wound in his chest, and a blood drenched knife is clenched in his right hand.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("tu_swordman").get()? == 19 {
        ctx.lines(args!["...", "......"])?;
        ctx.next()?;
        ctx.mes("^3355FFHe's dead!^000000")?;
        ctx.next()?;
        ctx.lines(args!["^3355FFBankley had a sad, pitiable expression on his face. Since the color is still fresh in his cheeks, he died only a little while ago.", "^3355FFThere is a long, deep wound in his chest, and a blood drenched knife is clenched in his right hand.^000000"])?;
        ctx.var("tu_swordman").set(Val::from(20))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8225), Val::from(8226)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("[Bankley]")?;
    if ctx.var("tu_swordman").get()? == 15 {
        ctx.lines(args![
            "I didn't kill anybody...!",
            "I'm innocent and don't",
            "have anything to hide!"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Bankley",
            args![
                "But look...",
                "If it helps, let me give you this weird code that some strange guy gave me. I guess it's supposed to be some kind of clue."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bankley",
            args![
                "^5D478BhekdlfiDrindkelsd^000000",
                "Do you understand what",
                "it means? I have no idea, but",
                "I told you everything I know!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bankley",
            args![
                "I feel so violated!",
                "Once you prove I'm innocent, are you going to compensate me for",
                "my mental suffering!?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bankley",
            args![
                "Stop bothering me",
                "and wasting your time.",
                "You're better off hunting",
                "for the real culprit!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("tu_swordman").get()? == 14 {
        ctx.lines(args![
            "I didn't kill anybody...!",
            "I'm innocent and don't",
            "have anything to hide!"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Bankley",
            args![
                "But look...",
                "If it helps, let me give you this weird code that some strange guy gave me. I guess it's supposed to be some kind of clue."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bankley",
            args![
                "^5D478BhekdlfiDrindkelsd^000000",
                "Do you understand what",
                "it means? I have no idea, but",
                "I told you everything I know!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bankley",
            args![
                "I feel so violated!",
                "Once you prove I'm innocent, are you going to compensate me for",
                "my mental suffering!?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bankley",
            args![
                "Stop bothering me",
                "and wasting your time.",
                "You're better off hunting",
                "for the real culprit!"
            ],
        )?;
        ctx.var("tu_swordman").set(Val::from(15))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("Even though I can't afford it now, my dream is to travel around the world and visit all of its great cities!")?;
    ctx.next()?;
    ctx.lines_as(
        "Bankley",
        args![
            "Someday, I'm sure I'll get the chance to make my dreams",
            "come true. But for now, I'll have to live as best as I can."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bankley(ctx: &Ctx) -> Script {
    bankley_body(ctx, Vec::new()).map(|_| ())
}
