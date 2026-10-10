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

pub fn young_man(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Young Man",
        args![
            "Huh. So that giant",
            "air pouch can make",
            "people float in midair?",
            "Would filling my tummy",
            "with air work the same way?"
        ],
    )?;
    ctx.close()
}

pub fn emily(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Emily",
        args![
            "I feel so blessed to",
            "live in this quant, little",
            "town. It's so beautiful, and",
            "everyone here is so nice~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Emily",
        args![
            "For some reason, my older",
            "sister wants to move out of",
            "Hugel as soon as she can. She",
            "Says that she's getting crept",
            "out by the people that live here.",
            "Don't you think that sounds weird?"
        ],
    )?;
    ctx.close()
}

pub fn kayplas(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Kayplas",
        args![
            "Ooh, I really want to",
            "have that red bottle.",
            "I should ask my mom",
            "to buy me one. It doesn't",
            "look too expensive, does it?"
        ],
    )?;
    ctx.close()
}

pub fn lisa(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Lisa",
        args![
            "Hugel is a pretty",
            "small, homely village.",
            "Everyone knows everyone,",
            "everybody knows what",
            "everybody else is doing.",
            "It's so suffocating!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lisa",
        args![
            "There's no privacy in",
            "small towns. Someday,",
            "I wanna go out and",
            "live in the big city~"
        ],
    )?;
    ctx.close()
}

pub fn old_nikki(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Old Nikki",
        args![
            "You must not be from",
            "around here. Ah, you're",
            "an adventurer, right? Do",
            "you know how I could tell?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Old Nikki",
        args![
            "It's because everyone",
            "who's lived here starts",
            "to look alike after a while.",
            "And you certainly don't look",
            "as old as us. Well, have",
            "a nice day, adventurer~"
        ],
    )?;
    ctx.close()
}

pub fn marius(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Marius",
        args![
            "Yes, I'm an old man, but",
            "I can lick a whippersnapper",
            "like you any day of the week!",
            "You know, Hugel's got a longer",
            "life expectancy than all the other towns. You wanna know why?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Marius",
        args![
            "It's because the old",
            "coots in this town refuse",
            "to just lay down and die!",
            "Now, c'mon! Lemme show",
            "you how strong I am! Let's",
            "wrestle or something, kid~"
        ],
    )?;
    ctx.close()
}

pub fn chris(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Chris",
        args![
            "You know, the people don't",
            "fight harmful monsters, they",
            "just protect themselves by",
            "equipping armor. That's",
            "just the way they are."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Chris",
        args![
            "If you want to buy",
            "some nicer armors,",
            "then I suggest buying",
            "some in a bigger city."
        ],
    )?;
    ctx.close()
}

pub fn party_supplies_shop(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Shopkeeper",
        args![
            "Welcome to the party supplies",
            "shop!",
            "Why don't you enjoy some",
            "spectacular fireworks with your",
            "friends?",
            "We can provide you with 5 of them",
            "at 500 zeny."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Buy", "Cancel"])? {
        0 => {
            if ctx.player().zeny()? < 500 {
                ctx.lines_as("Shopkeeper", args!["I am sorry, but you don't have", "enough money~"])?;
                return ctx.close();
            }
            ctx.player().set_zeny(ctx.player().zeny()? - 500)?;
            ctx.items().give(12018, 5)?;
            ctx.lines_as("Shopkeeper", args!["Here you go!", "Have fun with them!"])?;
            ctx.close()
        }
        _ => {
            ctx.lines_as("Shopkeeper", args!["Thank you, please come again."])?;
            ctx.close()
        }
    }
}

pub fn a_part_timer_1(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Luda",
        args![
            "Welcome to the",
            "Shrine Expedition Office.",
            "I'm Luda, a part-time",
            "assistant. My job is to",
            "keep this office neat and",
            "clean, but look at this place!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Luda",
        args![
            "Still, I think I can",
            "handle this difficult task~",
            "This room is the office for",
            "the Schwarzwald Republic team,",
            "and the other is for the Rune-",
            "Midgarts Kingdom team."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Luda",
        args![
            "I have to clean both rooms,",
            "so they keep me pretty busy.",
            "Why don't you volunteer for",
            "their expedition? I know they",
            "can't really pay you, but it's",
            "a great chance to explore~"
        ],
    )?;
    ctx.close()
}

pub fn a_part_timer_2(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^3355FFThis part-timer is",
        "completely engrossed",
        "in his task of organizing",
        "files and books.^000000"
    ])?;
    ctx.close()
}
