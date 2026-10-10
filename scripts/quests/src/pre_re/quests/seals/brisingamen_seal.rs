#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

pub fn personnel_record_book1(ctx: &Ctx) -> Script {
    if runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
        ctx.lines_as("Librarian", args!["What are you doing?", "Don't touch anything!"])?;
        return ctx.close();
    }
    if runtime::op(&ctx.var("$god3").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
        ctx.lines_as("Librarian", args!["What are you doing?", "Don't touch anything!"])?;
        return ctx.close();
    }
    if ctx.var("god_brising").get()? == 26 {
        read_book_intro(ctx)?;
        let roll = ctx.call(Function::Rand, args![1, 3])?;
        if roll == 1 {
            read_lowen_record(ctx)?;
            ctx.var("god_brising").set(30)?;
            return leave_castle(ctx);
        }
        wake_librarian(ctx)?;
        return leave_castle(ctx);
    } else if ctx.var("god_brising").get()? == 25 {
        ctx.mes("^3355FFYou took the book from the shelf while the librarian was dozing. According to the Crusader Personnel Records, there's only one recruit that was named Lowen Ellenen.")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args!["Lowen Ellenen...", "Member of the", "2nd squad...", "Age 22...", "Female..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args!["Became a fugitive?", "This isn't the same", "information the", "Librarian told me!"],
        )?;
        ctx.next()?;
        let roll = ctx.call(Function::Rand, args![1, 3])?;
        if roll == 1 {
            read_book_intro(ctx)?;
            read_lowen_record(ctx)?;
            ctx.var("god_brising").set(30)?;
            return leave_castle(ctx);
        }
        wake_librarian(ctx)?;
        ctx.var("god_brising").set(26)?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["(Nuts...!", "I didn't get", "to finish reading!)"])?;
        return leave_castle(ctx);
    } else {
        ctx.lines_as("Librarian", args!["What are you doing here?", "Don't touch anything!"])?;
        return ctx.close();
    }
}

fn read_book_intro(ctx: &Ctx) -> Result<(), Stop> {
    ctx.lines_as(
        ctx.player().name()?,
        args!["I should read this", "from the point where", "I left off..."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.player().name()?,
        args!["Huh...?", "What's this mark", "here at the bottom?", "Some kind of secret?"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.player().name()?,
        args![
            "No wonder the librarian",
            "seemed to hesitate before",
            "saying anything. Huh.",
            "Now let's see..."
        ],
    )?;
    ctx.next()
}

fn read_lowen_record(ctx: &Ctx) -> Result<(), Stop> {
    ctx.lines_as("Personnel Record", args!["^663300Lowen, of the 2nd squad, returned to her detachment 3 days after the incident. A trial was held, and she was judged guilty of fleeing when ordered to fight.^000000"])?;
    ctx.next()?;
    ctx.lines_as("Personnel Record", args!["^663300Despite the severity of this offense, she was only discharged from the Crusaders and the holy power granted to her was forcibly removed...^000000"])?;
    ctx.next()?;
    ctx.lines_as(
        ctx.player().name()?,
        args!["She was forced to leave the Crusaders?! That's ridiculous!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.player().name()?,
        args!["Did they want to sentence her to death or what? That's almost too cruel. She must have been humiliated..."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Personnel Record",
        args![
            "^6633002 years later after the incident, an expedition team found rusty armor shards, a broken sword",
            "and some effects belonging to",
            "Lowen Ellenen.^000000"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Personnel Record",
        args![
            "^663300However, this information",
            "is highly classified. Officially, Lowen Ellenen died during the mission.^000000"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.player().name()?,
        args![
            "They thought she brought",
            "disgrace to the Crusaders?",
            "This sounds pretty messy.",
            "Should I try to meet her again?"
        ],
    )?;
    ctx.next()?;
    wake_librarian(ctx)
}

fn wake_librarian(ctx: &Ctx) -> Result<(), Stop> {
    ctx.lines_as(
        "Librarian",
        args![
            "^666666*Yawn...*^000000",
            "What the...?",
            "who are you!",
            "Get the hell",
            "out of here!"
        ],
    )
}

fn leave_castle(ctx: &Ctx) -> Script {
    ctx.close_window()?;
    ctx.warp("prt_castle", 94, 37)?;
    ctx.end()
}
